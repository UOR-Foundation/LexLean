//! Source packages (SPEC.md §10.5): the workspace rule, the resolution of the
//! packages a language-1.2 project declares from member projects of its
//! workspace, and their pins in the lock.
//!
//! A source package is a project. Nothing is fetched: a package is read from a
//! member directory of the workspace, confined to its manifest and its source
//! roots, and hashed; the pin is the digest, and every command recomputes it.

use std::collections::{BTreeMap, BTreeSet};

use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;

use crate::artifact::content_id::{tree_digest, Sha256Digest};
use crate::code;
use crate::config::{
    is_module_segment, is_project_relative, parse_project, LexiconSource, Limits, ProjectConfig,
};
use crate::diagnostic::Diagnostic;
use crate::lexicon::package::toml_comment_at;
use crate::lock::{Lock, LockSourcePackage};
use crate::project::Project;

/// The workspace file name (§10.5), at the root of the workspace.
pub const WORKSPACE_FILE: &str = "lexlean-workspace.toml";

/// The workspace schema.
pub const WORKSPACE_SPEC: &str = "lexlean/workspace/1";

fn workspace_error(message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code!("LLC0108"), message)
}

fn security(message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code!("LLS8001"), message)
}

fn config_error(message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code!("LLC0101"), message)
}

/// The workspace a project belongs to (§10.5): the directory holding
/// [`WORKSPACE_FILE`] and the member projects it lists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    /// The canonical workspace root.
    pub root: Utf8PathBuf,
    /// The member project directories, workspace-relative, sorted.
    pub members: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWorkspace {
    spec: String,
    members: Vec<String>,
}

impl Workspace {
    /// Read the workspace file of `directory`, when it holds one. A symlinked
    /// or unreadable file, or one that is not a `lexlean/workspace/1`
    /// document, is refused.
    pub fn read(directory: &Utf8Path) -> Result<Option<Self>, Diagnostic> {
        let file = directory.join(WORKSPACE_FILE);
        let metadata = match std::fs::symlink_metadata(file.as_std_path()) {
            Ok(metadata) => metadata,
            Err(io_error) if io_error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(io_error) => return Err(security(format!("{WORKSPACE_FILE}: {io_error}"))),
        };
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(security(format!(
                "{WORKSPACE_FILE}: the workspace file must be a regular file"
            )));
        }
        let bytes = std::fs::read(file.as_std_path())
            .map_err(|io_error| workspace_error(format!("{WORKSPACE_FILE}: {io_error}")))?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| workspace_error(format!("{WORKSPACE_FILE} is not UTF-8")))?;
        if toml_comment_at(text).is_some() {
            return Err(workspace_error(format!(
                "{WORKSPACE_FILE}: comments are forbidden"
            )));
        }
        let raw: RawWorkspace = toml::from_str(text)
            .map_err(|error| workspace_error(format!("{WORKSPACE_FILE}: {error}")))?;
        if raw.spec != WORKSPACE_SPEC {
            return Err(workspace_error(format!(
                "{WORKSPACE_FILE}: unsupported workspace schema `{}`",
                raw.spec
            )));
        }
        for member in &raw.members {
            if !is_project_relative(member) {
                return Err(workspace_error(format!(
                    "{WORKSPACE_FILE}: member `{member}` is not a relative directory of the workspace"
                )));
            }
        }
        if !raw.members.windows(2).all(|pair| pair[0] < pair[1]) {
            return Err(workspace_error(format!(
                "{WORKSPACE_FILE}: members must be unique and sorted"
            )));
        }
        Ok(Some(Self {
            root: directory.to_path_buf(),
            members: raw.members,
        }))
    }

    /// The workspace of a project: the nearest ancestor of its canonical root
    /// that holds a workspace file, which must list the project's own
    /// directory. A project is not a member of the workspace of its own
    /// directory, and a stray file higher up grants nothing to a project it
    /// does not list.
    pub fn of_project(root: &Utf8Path) -> Result<Self, Diagnostic> {
        let mut current = root.parent();
        while let Some(directory) = current {
            if let Some(workspace) = Self::read(directory)? {
                if workspace.member_of(root).is_none() {
                    return Err(workspace_error(format!(
                        "the project is not a member of its workspace: {WORKSPACE_FILE} does not list its directory"
                    )));
                }
                return Ok(workspace);
            }
            current = directory.parent();
        }
        Err(workspace_error(format!(
            "a project outside any workspace cannot declare a source package: no ancestor directory holds a {WORKSPACE_FILE}"
        )))
    }

    /// The member directory (workspace-relative) of a project root beneath
    /// this workspace, when it is a listed member.
    #[must_use]
    pub fn member_of(&self, root: &Utf8Path) -> Option<String> {
        let relative =
            crate::project::project_relative(root.strip_prefix(&self.root).ok()?.as_str());
        self.members.contains(&relative).then_some(relative)
    }
}

/// One source package read from its member directory and verified against
/// the rules of §10.5.
#[derive(Debug, Clone)]
pub struct ResolvedSourcePackage {
    /// The identifier the declaration gave it.
    pub id: String,
    /// The declared version.
    pub version: String,
    /// The workspace-relative member directory.
    pub member: String,
    /// Its module prefix, the first thing its modules are named under.
    pub prefix: String,
    /// Its parsed manifest.
    pub config: ProjectConfig,
    /// The tree digest over its manifest and source files.
    pub tree_sha256: Sha256Digest,
    /// The digest of its manifest bytes.
    pub manifest_sha256: Sha256Digest,
    /// The identifiers of the packages its manifest declares, sorted.
    pub dependencies: Vec<String>,
    /// Every file the tree digest covers beneath the member, by
    /// member-relative path.
    pub files: BTreeMap<String, Vec<u8>>,
    /// Its modules by name relative to its source roots, with the
    /// member-relative path of the file that declares each.
    pub modules: BTreeMap<String, String>,
}

impl ResolvedSourcePackage {
    /// The key of one of its modules in a project that imports it: the Lean
    /// name the module has, its prefix and its relative name (§10.5).
    #[must_use]
    pub fn key(&self, relative_name: &str) -> String {
        format!("{}.{relative_name}", self.prefix)
    }

    /// The display path of one of its files in diagnostics and records.
    #[must_use]
    pub fn display_path(&self, member_relative: &str) -> String {
        format!("{}:{member_relative}", self.id)
    }
}

/// Every source package a project's closure reaches, by identifier.
#[derive(Debug, Clone, Default)]
pub struct SourcePackages {
    /// The packages, by identifier.
    pub packages: BTreeMap<String, ResolvedSourcePackage>,
    /// The identifiers the project declares itself, sorted.
    pub declared: Vec<String>,
}

impl SourcePackages {
    /// The lock rows of the closure, sorted by identifier (§11.1).
    #[must_use]
    pub fn rows(&self) -> Vec<LockSourcePackage> {
        self.packages
            .values()
            .map(|package| LockSourcePackage {
                id: package.id.clone(),
                version: package.version.clone(),
                kind: "path".to_owned(),
                source: package.member.clone(),
                module_prefix: package.prefix.clone(),
                tree_sha256: package.tree_sha256,
                manifest_sha256: package.manifest_sha256,
                imports: package
                    .dependencies
                    .iter()
                    .map(|dependency| format!("{dependency}@{}", self.packages[dependency].version))
                    .collect(),
            })
            .collect()
    }

    /// Whether the project reaches no source package.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }

    /// The package that owns a module key of the importing project, with the
    /// module's name relative to the package's source roots.
    #[must_use]
    pub fn owner_of_key<'a>(
        &'a self,
        key: &'a str,
    ) -> Option<(&'a ResolvedSourcePackage, &'a str)> {
        self.packages.values().find_map(|package| {
            let relative = key
                .strip_prefix(package.prefix.as_str())?
                .strip_prefix('.')?;
            package
                .modules
                .contains_key(relative)
                .then_some((package, relative))
        })
    }
}

/// Is `prefix` equal to `other` or a dotted extension of it, either way?
fn prefixes_overlap(prefix: &str, other: &str) -> bool {
    prefix == other
        || prefix
            .strip_prefix(other)
            .is_some_and(|rest| rest.starts_with('.'))
        || other
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('.'))
}

/// Walk `member` beneath the workspace root with `symlink_metadata`, refusing
/// any symlink component, and require the last component to be a directory.
fn confined_member(workspace: &Workspace, member: &str) -> Result<Utf8PathBuf, Diagnostic> {
    if !is_project_relative(member) {
        return Err(security(format!("`{member}` escapes the workspace")));
    }
    let mut current = workspace.root.clone();
    let mut walked = String::new();
    for segment in member.split('/') {
        current.push(segment);
        if !walked.is_empty() {
            walked.push('/');
        }
        walked.push_str(segment);
        let metadata = std::fs::symlink_metadata(current.as_std_path())
            .map_err(|io_error| workspace_error(format!("`{walked}`: {io_error}")))?;
        if metadata.file_type().is_symlink() {
            return Err(security(format!(
                "`{walked}`: symlinks are rejected in the path of a source package"
            )));
        }
    }
    let metadata = std::fs::symlink_metadata(current.as_std_path())
        .map_err(|io_error| workspace_error(format!("`{member}`: {io_error}")))?;
    if !metadata.is_dir() {
        return Err(workspace_error(format!(
            "`{member}` is not a directory of the workspace"
        )));
    }
    Ok(current)
}

/// Read one regular file of a package, bounded by `max_file_bytes`.
fn read_file(
    absolute: &Utf8Path,
    shown: &str,
    limits: &Limits,
) -> Result<Vec<u8>, Vec<Diagnostic>> {
    let metadata = std::fs::symlink_metadata(absolute.as_std_path())
        .map_err(|io_error| vec![workspace_error(format!("{shown}: {io_error}"))])?;
    if metadata.file_type().is_symlink() {
        return Err(vec![security(format!(
            "`{shown}`: symlinks are rejected in a source package"
        ))]);
    }
    if !metadata.is_file() {
        return Err(vec![security(format!(
            "`{shown}`: special files are rejected in a source package"
        ))]);
    }
    if metadata.len() > limits.max_file_bytes {
        return Err(vec![Diagnostic::new(
            code!("LLS8002"),
            format!(
                "max_file_bytes exceeded: configured {}, observed {} in {shown}",
                limits.max_file_bytes,
                metadata.len()
            ),
        )]);
    }
    std::fs::read(absolute.as_std_path())
        .map_err(|io_error| vec![workspace_error(format!("{shown}: {io_error}"))])
}

/// The modules of a package by name relative to its source roots (§10.5),
/// with the same duplicate and case-fold rules as a project's own (§23.3).
fn package_modules(
    id: &str,
    config: &ProjectConfig,
    files: &BTreeMap<String, Vec<u8>>,
) -> Result<BTreeMap<String, String>, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    let mut modules: BTreeMap<String, String> = BTreeMap::new();
    for root in &config.source_roots {
        let prefix = format!("{root}/");
        for path in files.keys() {
            let Some(inside) = path.strip_prefix(&prefix) else {
                continue;
            };
            let Some(stem) = inside.strip_suffix(".lex.tex") else {
                continue;
            };
            let name = stem.replace('/', ".");
            if !name.split('.').all(is_module_segment) {
                diagnostics.push(config_error(format!(
                    "source package `{id}`: `{path}` is not a valid module path"
                )));
                continue;
            }
            if let Some(existing) = modules.get(&name) {
                diagnostics.push(Diagnostic::new(
                    code!("LLC0104"),
                    format!(
                        "source package `{id}`: `{path}` and `{existing}` declare the same module `{name}`"
                    ),
                ));
                continue;
            }
            modules.insert(name, path.clone());
        }
    }
    let mut folded: BTreeMap<String, &String> = BTreeMap::new();
    for module in modules.keys() {
        if let Some(existing) = folded.insert(module.to_lowercase(), module) {
            diagnostics.push(Diagnostic::new(
                code!("LLC0104"),
                format!(
                    "source package `{id}`: modules `{existing}` and `{module}` collide under case folding"
                ),
            ));
        }
    }
    if diagnostics.is_empty() {
        Ok(modules)
    } else {
        Err(diagnostics)
    }
}

/// Read one member as a source package: its manifest, its source roots, and
/// nothing else (§25.1).
fn load_member(
    workspace: &Workspace,
    id: &str,
    version: &str,
    member: &str,
    limits: &Limits,
) -> Result<ResolvedSourcePackage, Vec<Diagnostic>> {
    if !workspace.members.iter().any(|listed| listed == member) {
        return Err(vec![workspace_error(format!(
            "source package `{id}`: `{member}` is not a member of the workspace; {WORKSPACE_FILE} lists the directories a source package may name"
        ))]);
    }
    let directory = confined_member(workspace, member).map_err(|d| vec![d])?;
    let manifest_name = format!("{member}/lexlean.toml");
    let manifest_bytes = read_file(&directory.join("lexlean.toml"), &manifest_name, limits)?;
    let config = parse_project(&manifest_name, &manifest_bytes)?;
    let mut policy = Vec::new();
    if config.language != crate::LANGUAGE_1_2 {
        policy.push(config_error(format!(
            "source package `{id}`: {manifest_name} is language {}, and a source package is a language-1.2 project",
            config.language
        )));
    }
    if !config.artifact_sources.is_empty() {
        policy.push(config_error(format!(
            "source package `{id}`: {manifest_name} declares artifact_source, which a package may not: its files would lie outside the pin"
        )));
    }
    if config.pdf.is_some() {
        policy.push(config_error(format!(
            "source package `{id}`: {manifest_name} declares a pdf provider, which a package may not"
        )));
    }
    if config
        .lexicon_sources
        .iter()
        .any(|source| !matches!(source, LexiconSource::Builtin { .. }))
    {
        policy.push(config_error(format!(
            "source package `{id}`: {manifest_name} declares a lexicon source other than a builtin one; its glossaries are those of the importing project"
        )));
    }
    if !policy.is_empty() {
        return Err(policy);
    }
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    files.insert("lexlean.toml".to_owned(), manifest_bytes.clone());
    for root in &config.source_roots {
        let absolute_root = confined_member(
            &Workspace {
                root: workspace.root.clone(),
                members: Vec::new(),
            },
            &format!("{member}/{root}"),
        )
        .map_err(|d| vec![d])?;
        for entry in walkdir::WalkDir::new(absolute_root.as_std_path())
            .follow_links(false)
            .sort_by_file_name()
        {
            let entry = entry
                .map_err(|walk_error| vec![security(format!("`{member}/{root}`: {walk_error}"))])?;
            let file_type = entry.file_type();
            if file_type.is_dir() {
                continue;
            }
            let absolute = crate::project::utf8_path(entry.path()).map_err(|d| vec![d])?;
            let relative = absolute
                .strip_prefix(&directory)
                .map(|inside| crate::project::project_relative(inside.as_str()))
                .map_err(|_| vec![security(format!("`{member}/{root}` escapes the package"))])?;
            let bytes = read_file(&absolute, &format!("{member}/{relative}"), limits)?;
            files.insert(relative, bytes);
        }
    }
    let rows: Vec<(&str, &[u8])> = files
        .iter()
        .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
        .collect();
    let mut sorted = rows;
    sorted.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    let tree_sha256 = tree_digest(&sorted);
    let modules = package_modules(id, &config, &files)?;
    Ok(ResolvedSourcePackage {
        id: id.to_owned(),
        version: version.to_owned(),
        member: member.to_owned(),
        prefix: config.module_prefix.clone(),
        manifest_sha256: Sha256Digest::of(&manifest_bytes),
        dependencies: config
            .source_packages
            .iter()
            .map(|package| package.package.clone())
            .collect(),
        config,
        tree_sha256,
        files,
        modules,
    })
}

/// Resolve every source package a project declares, with the packages theirs
/// declare in turn, from the member directories of its workspace (§10.5).
/// Nothing here reads outside a package's manifest and source roots, and
/// nothing is fetched.
///
/// # Errors
///
/// `LLC0108` for a workspace rule, `LLS8001` for a symlink or an escaping
/// path, `LLC0106` for an identifier with two sources, `LLR3003` for a
/// package that reaches itself, `LLC0107` for a module prefix two sources
/// claim, and the configuration errors of a manifest.
pub fn resolve(project: &Project) -> Result<SourcePackages, Vec<Diagnostic>> {
    if project.config.source_packages.is_empty() {
        return Ok(SourcePackages::default());
    }
    let Some(workspace) = &project.workspace else {
        return Err(vec![workspace_error(
            "a project outside any workspace cannot declare a source package",
        )]);
    };
    let own_member = project.workspace_member.clone();
    let limits = &project.config.limits;
    let mut found: BTreeMap<String, ResolvedSourcePackage> = BTreeMap::new();
    let mut work: Vec<(String, String, String)> = project
        .config
        .source_packages
        .iter()
        .map(|package| {
            (
                package.package.clone(),
                package.version.clone(),
                package.path.clone(),
            )
        })
        .collect();
    let mut diagnostics = Vec::new();
    while let Some((id, version, member)) = work.pop() {
        if let Some(existing) = found.get(&id) {
            if existing.member != member || existing.version != version {
                diagnostics.push(Diagnostic::new(
                    code!("LLC0106"),
                    format!(
                        "source package `{id}` is declared with two sources: `{}` at {} and `{member}` at {version}",
                        existing.member, existing.version
                    ),
                ));
            }
            continue;
        }
        if own_member.as_deref() == Some(member.as_str()) {
            diagnostics.push(Diagnostic::new(
                code!("LLR3003"),
                format!("source package `{id}` names the importing project itself (`{member}`)"),
            ));
            continue;
        }
        match load_member(workspace, &id, &version, &member, limits) {
            Ok(package) => {
                for dependency in &package.config.source_packages {
                    work.push((
                        dependency.package.clone(),
                        dependency.version.clone(),
                        dependency.path.clone(),
                    ));
                }
                found.insert(id, package);
            }
            Err(mut errors) => diagnostics.append(&mut errors),
        }
    }
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    let declared: Vec<String> = project
        .config
        .source_packages
        .iter()
        .map(|package| package.package.clone())
        .collect();
    let packages = SourcePackages {
        packages: found,
        declared,
    };
    check_acyclic(&packages)?;
    check_prefixes(project, &packages)?;
    Ok(packages)
}

/// A package that reaches itself through the packages it declares is an
/// import cycle (§15.1).
fn check_acyclic(packages: &SourcePackages) -> Result<(), Vec<Diagnostic>> {
    fn visit<'a>(
        id: &'a str,
        packages: &'a SourcePackages,
        state: &mut BTreeMap<&'a str, u8>,
    ) -> Result<(), String> {
        match state.get(id) {
            Some(1) => return Err(id.to_owned()),
            Some(2) => return Ok(()),
            _ => {}
        }
        state.insert(id, 1);
        if let Some(package) = packages.packages.get(id) {
            for dependency in &package.dependencies {
                visit(dependency, packages, state)?;
            }
        }
        state.insert(id, 2);
        Ok(())
    }
    let mut state = BTreeMap::new();
    for id in packages.packages.keys() {
        visit(id, packages, &mut state).map_err(|cycle| {
            vec![Diagnostic::new(
                code!("LLR3003"),
                format!("source packages depend on each other through `{cycle}`"),
            )]
        })?;
    }
    Ok(())
}

/// A module prefix is claimed by one source (§10.5): the project's own and
/// every package's must not be equal, and none may lie beneath another.
fn check_prefixes(project: &Project, packages: &SourcePackages) -> Result<(), Vec<Diagnostic>> {
    let mut claims: Vec<(String, String)> = vec![(
        project.config.module_prefix.clone(),
        "the importing project".to_owned(),
    )];
    claims.extend(packages.packages.values().map(|package| {
        (
            package.prefix.clone(),
            format!("source package `{}`", package.id),
        )
    }));
    let mut diagnostics = Vec::new();
    for (index, (prefix, owner)) in claims.iter().enumerate() {
        for (other_prefix, other_owner) in &claims[index + 1..] {
            if prefixes_overlap(prefix, other_prefix) {
                diagnostics.push(Diagnostic::new(
                    code!("LLC0107"),
                    format!(
                        "the module prefix `{prefix}` of {owner} and the module prefix `{other_prefix}` of {other_owner} overlap: a module prefix is claimed by one source"
                    ),
                ));
            }
        }
    }
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

/// Compare the pins of a lock with the packages as they are now (§11.1): a
/// package whose version, source, prefix, or digests differ is `LLC0105`, naming
/// the package and both digests; a package the lock pins that the project's
/// closure does not reach is `LLC0106`. A package the closure reaches that the
/// lock lacks is left to the staleness of the lock as a whole.
///
/// # Errors
///
/// The diagnostics above.
pub fn check_pins(
    locked: &[LockSourcePackage],
    expected: &[LockSourcePackage],
) -> Result<(), Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    for row in locked {
        match expected.iter().find(|candidate| candidate.id == row.id) {
            None => diagnostics.push(Diagnostic::new(
                code!("LLC0106"),
                format!(
                    "the lock pins source package `{}`, which the project does not declare",
                    row.id
                ),
            )),
            Some(current) if current != row => {
                let mut differences = Vec::new();
                if row.version != current.version {
                    differences.push(format!(
                        "version: locked {}, found {}",
                        row.version, current.version
                    ));
                }
                if row.source != current.source {
                    differences.push(format!(
                        "source: locked {}, found {}",
                        row.source, current.source
                    ));
                }
                if row.module_prefix != current.module_prefix {
                    differences.push(format!(
                        "module_prefix: locked {}, found {}",
                        row.module_prefix, current.module_prefix
                    ));
                }
                if row.tree_sha256 != current.tree_sha256 {
                    differences.push(format!(
                        "tree_sha256: locked {}, found {}",
                        row.tree_sha256.to_hex(),
                        current.tree_sha256.to_hex()
                    ));
                }
                if row.manifest_sha256 != current.manifest_sha256 {
                    differences.push(format!(
                        "manifest_sha256: locked {}, found {}",
                        row.manifest_sha256.to_hex(),
                        current.manifest_sha256.to_hex()
                    ));
                }
                if row.imports != current.imports {
                    differences.push(format!(
                        "imports: locked [{}], found [{}]",
                        row.imports.join(", "),
                        current.imports.join(", ")
                    ));
                }
                diagnostics.push(Diagnostic::new(
                    code!("LLC0105"),
                    format!(
                        "source package `{}` is not the package the lock pins: {}",
                        row.id,
                        differences.join("; ")
                    ),
                ));
            }
            Some(_) => {}
        }
    }
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

/// Resolve the source packages and require them to be the ones `lock` pins:
/// every command that links recomputes the digests (§11.1).
///
/// # Errors
///
/// Those of [`resolve`] and [`check_pins`], and `LLC0102` for a package the
/// closure reaches that the lock does not pin.
pub fn load_locked(project: &Project, lock: &Lock) -> Result<SourcePackages, Vec<Diagnostic>> {
    let packages = resolve(project)?;
    let expected = packages.rows();
    check_pins(&lock.source_packages, &expected)?;
    let pinned: BTreeSet<&str> = lock
        .source_packages
        .iter()
        .map(|row| row.id.as_str())
        .collect();
    if let Some(missing) = expected
        .iter()
        .find(|row| !pinned.contains(row.id.as_str()))
    {
        return Err(vec![Diagnostic::new(
            code!("LLC0102"),
            format!(
                "{} does not pin source package `{}`; run `lexlean lock`",
                project.config.lockfile, missing.id
            ),
        )]);
    }
    Ok(packages)
}

#[cfg(test)]
mod tests {
    use super::prefixes_overlap;
    use crate::config::is_package_version;

    /// §10.5: a module prefix is claimed by one source, and a prefix beneath
    /// another is a claim on it, but a longer segment is not.
    #[test]
    fn a_prefix_beneath_another_overlaps_it_and_a_longer_segment_does_not() {
        assert!(prefixes_overlap("PkgLib", "PkgLib"));
        assert!(prefixes_overlap("PkgLib.Sub", "PkgLib"));
        assert!(prefixes_overlap("PkgLib", "PkgLib.Sub"));
        assert!(!prefixes_overlap("PkgLib", "PkgLibrary"));
        assert!(!prefixes_overlap("PkgLib.A", "PkgLib.B"));
    }

    /// §10.1: a version is a reader's string of at most 64 ASCII letters,
    /// digits, `.`, `+`, and `-` that begins with a letter or digit.
    #[test]
    fn a_version_is_a_short_ascii_string() {
        for good in ["1.0.0", "2026.10", "1.0.0-rc.1+build5", "v1"] {
            assert!(is_package_version(good), "{good}");
        }
        for bad in ["", "-1", ".1", "1 0", "1/0", "é", &"1".repeat(65)] {
            assert!(!is_package_version(bad), "{bad:?}");
        }
    }
}
