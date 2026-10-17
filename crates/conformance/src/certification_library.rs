//! The certification library's environment (SPEC.md §17.18): the native
//! LexLean modules under `compiler/proofs/`, the Lean the compiler generates
//! from them compiled with the pinned Lean beside the compiler project's
//! generated modules and the shipped calculus modules, replayed through
//! `leanchecker`, and audited declaration by declaration against the axioms
//! the native rows pin.
//!
//! The library is repository evidence, not a shipped file: it quantifies over
//! the evaluator and the certificate checker, which LexLean's proof language
//! cannot state. What it proves is therefore checked here, against the
//! generated modules the compiler project verifies (their text is the
//! project's rendered build, so a drifted module is the library's problem too).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Deserialize;

use crate::preservation::{joined, pinned_with, shipped};
use crate::support::{self, repo_root, P};

/// The library's own project, relative to the repository root.
pub const PROJECT_DIR: &str = "compiler/proofs";

/// The module list of the library, which names no declaration.
pub const REGISTRY: &str = "compiler/proofs/library.toml";

/// The registry's tag.
pub const REGISTRY_SPEC: &str = "lexlean/certification-library/2";

/// The module prefix of the library project.
pub const PREFIX: &str = "LexLeanCertification";

/// The library's modules, in dependency order.
pub const MODULES: [&str; 13] = [
    "Trials",
    "Tail",
    "Factor",
    "Schemas",
    "Facts",
    "PremiseSound",
    "Positions",
    "Lower",
    "Frontier",
    "Representation",
    "Normality",
    "Check",
    "Soundness",
];

/// The generated modules the library is stated against, with the target
/// modules they import, in dependency order.
pub const GENERATED: [&str; 6] = [
    "LexLeanTarget.TargetSyntax",
    "LexLeanTarget.TargetSemantics",
    "LexLeanTarget.Gnaf",
    "LexLeanTarget.GnafFixtures",
    "LexLeanTarget.Certification",
    "LexLeanTarget.GnafCertificates",
];

/// The axioms every library declaration may reach.
pub const AXIOMS: [&str; 3] = ["Classical.choice", "Quot.sound", "propext"];

/// The registry: the module list the library project ships, and every
/// declaration of the native modules with its exact axioms.
#[derive(Debug, Clone)]
pub struct Registry {
    /// The registry's tag.
    pub spec: String,
    /// The pinned toolchain the axioms were measured under.
    pub toolchain: String,
    /// The library's modules, in dependency order.
    pub modules: Vec<String>,
    /// The declarations the native modules state that a Lean command can name.
    pub declaration: Vec<Declaration>,
}

/// The module list as `library.toml` states it.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ModuleList {
    spec: String,
    toolchain: String,
    modules: Vec<String>,
}

/// One declaration of a native module: its module, name, and exact axioms.
#[derive(Debug, Clone)]
pub struct Declaration {
    /// The module declaring it.
    pub module: String,
    /// Its qualified name.
    pub name: String,
    /// The exact axioms its row pins.
    pub axioms: Vec<String>,
}

/// A module of the library by its name.
#[must_use]
pub fn module_name(stem: &str) -> String {
    format!("{PREFIX}.{stem}")
}

/// The project-relative path of the native source of the module `stem`.
#[must_use]
pub fn source_path(stem: &str) -> String {
    format!("{PROJECT_DIR}/src/{stem}.lex.tex")
}

/// The Lean modules a library module may import besides itself and `Init`:
/// the preservation core and the calculus and generated modules the library is
/// stated against.
pub const ALLOWED_IMPORTS: [&str; 7] = [
    "LexLeanPreservation.Core",
    "LexLeanTarget.Gnaf",
    "LexLeanTarget.Certification",
    "LexLeanTarget.GnafCertificates",
    "LexLeanTarget.GnafFixtures",
    "LexLeanTarget.TargetSemantics",
    "LexLeanTarget.TargetSyntax",
];

/// The native modules read through `read`, in the registry's order, with the
/// registry they state: each parsed as language-1.2 core data, its imports
/// closed over `Init`, [`ALLOWED_IMPORTS`], and the earlier modules, every
/// declaration it states pinning its axioms exactly to the expected three,
/// and no name stated twice.
///
/// # Errors
///
/// Returns the first reason the library is refused.
pub fn check_sources(
    read: &dyn Fn(&str) -> Result<String, String>,
) -> Result<(Registry, Vec<(String, lexlean::ir::core::CoreModule)>), String> {
    let text = read(REGISTRY)?;
    let list: ModuleList = toml::from_str(&text).map_err(|error| format!("{REGISTRY}: {error}"))?;
    if list.spec != REGISTRY_SPEC {
        return Err(format!(
            "{REGISTRY}: spec `{}` is not `{REGISTRY_SPEC}`",
            list.spec
        ));
    }
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut names: BTreeSet<String> = BTreeSet::new();
    let mut declaration = Vec::new();
    let mut modules = Vec::new();
    for module in &list.modules {
        let stem = module
            .strip_prefix(&format!("{PREFIX}."))
            .ok_or_else(|| format!("{REGISTRY}: `{module}` is outside `{PREFIX}`"))?;
        let path = source_path(stem);
        let core = lexlean::ir::core::CoreModule::from_source(&read(&path)?, "1.2")
            .map_err(|reason| format!("{path}: {reason}"))?;
        for import in &core.imports {
            let allowed = import == "Init"
                || ALLOWED_IMPORTS.contains(&import.as_str())
                || seen.contains(import);
            if !allowed {
                return Err(format!(
                    "{path}: imports `{import}`, which is neither `Init`, a module the library is stated against, nor an earlier library module"
                ));
            }
        }
        for row in &core.declarations {
            if row.generated {
                continue;
            }
            if !names.insert(row.name.clone()) {
                return Err(format!("{path}: `{}` is declared twice", row.name));
            }
            if !matches!(row.policy, lexlean::ir::core::CoreAxiomPolicy::Exact(_)) {
                return Err(format!(
                    "{path}: `{}` does not pin its axioms exactly",
                    row.name
                ));
            }
            // A name Lean prints as written is audited by name; a private or
            // escaped auxiliary is covered by every declaration using it.
            if lexlean::ir::core::is_closed_lean_name(&row.name) {
                let mut axioms = row.policy.axioms().to_vec();
                axioms.sort();
                declaration.push(Declaration {
                    module: module.clone(),
                    name: row.name.clone(),
                    axioms,
                });
            }
        }
        seen.insert(module.clone());
        modules.push((stem.to_owned(), core));
    }
    Ok((
        Registry {
            spec: list.spec,
            toolchain: list.toolchain,
            modules: list.modules,
            declaration,
        },
        modules,
    ))
}

fn on_disk(path: &str) -> Result<String, String> {
    std::fs::read_to_string(repo_root().join(path).as_std_path())
        .map_err(|error| format!("{path}: {error}"))
}

/// The native modules and the registry of the repository's library.
///
/// # Panics
///
/// Panics when the library is refused.
#[must_use]
pub fn native() -> &'static (Registry, Vec<(String, lexlean::ir::core::CoreModule)>) {
    static NATIVE: OnceLock<(Registry, Vec<(String, lexlean::ir::core::CoreModule)>)> =
        OnceLock::new();
    NATIVE.get_or_init(|| check_sources(&on_disk).unwrap_or_else(|reason| panic!("{reason}")))
}

/// The registry.
///
/// # Panics
///
/// Panics when the library is refused.
#[must_use]
pub fn registry() -> Registry {
    native().0.clone()
}

/// The Lean the compiler generates from the native module `stem`.
///
/// # Panics
///
/// Panics when the module is unknown or cannot be rendered.
#[must_use]
pub fn library_text(stem: &str) -> String {
    let core = native()
        .1
        .iter()
        .find(|(candidate, _)| candidate == stem)
        .unwrap_or_else(|| panic!("the library has no module `{stem}`"));
    lexlean::backend::core::lean_text(&core.1)
        .unwrap_or_else(|diagnostic| panic!("{stem}: {}", diagnostic.message))
}

/// One Lean source to compile.
pub struct Source {
    /// The Lean module name.
    pub module: String,
    /// Its text.
    pub text: String,
}

fn path_of(module: &str) -> String {
    format!("{}.lean", module.replace('.', "/"))
}

/// Compile `sources` in order, the olean of each found by those after it, in
/// a directory below the build directory named by the hash of the sources and
/// of the search directories' names, so that an unchanged environment is
/// compiled once per machine and never twice in a run. Publication is by
/// renaming a finished directory, as for the shipped environment.
fn cached(label: &str, sources: &[Source], upstream: &[PathBuf]) -> PathBuf {
    let mut digest = String::new();
    for source in sources {
        digest.push_str(&format!("{}\n{}\n", source.module, source.text));
    }
    for directory in upstream {
        digest.push_str(&format!("{}\n", directory.display()));
    }
    digest.push_str(lexlean::LEAN_TOOLCHAIN);
    let key = lexlean::artifact::content_id::Sha256Digest::of(digest.as_bytes()).to_hex();
    let base = std::env::current_exe()
        .ok()
        .and_then(|exe| {
            exe.ancestors()
                .nth(2)
                .map(|debug| debug.join("lexlean-certification-build"))
        })
        .unwrap_or_else(|| std::env::temp_dir().join("lexlean-certification-build"));
    let finished = base.join(format!("{label}-{key}"));
    if finished.join("ready").is_file() {
        return finished.join("build");
    }
    std::fs::create_dir_all(&base).expect("the build's directory");
    let building = base.join(format!("{label}-{key}.{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&building);
    /// Removes the unfinished directory when a module is rejected.
    struct Unfinished(PathBuf);
    impl Drop for Unfinished {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let _unfinished = Unfinished(building.clone());
    let mut search = vec![building.join("build")];
    search.extend(upstream.iter().cloned());
    for source in sources {
        let path = path_of(&source.module);
        let file = building.join("src").join(&path);
        let olean = building
            .join("build")
            .join(path.trim_end_matches(".lean"))
            .with_extension("olean");
        for target in [&file, &olean] {
            std::fs::create_dir_all(target.parent().expect("a parent")).expect("mkdir");
        }
        std::fs::write(&file, &source.text).expect("write");
        let output = pinned_with("lean", &building, &search)
            .arg("-o")
            .arg(&olean)
            .arg(&file)
            .output()
            .expect("lean runs");
        assert!(
            output.status.success(),
            "the module `{}` was rejected by Lean:\n{}",
            source.module,
            joined(&output)
        );
    }
    std::fs::write(building.join("ready"), &key).expect("write");
    if std::fs::rename(&building, &finished).is_err() {
        std::fs::remove_dir_all(&building).expect("remove the duplicate build");
    }
    finished.join("build")
}

/// The compiler project's generated modules this library is stated against,
/// from its rendered build.
fn generated() -> Vec<Source> {
    let project = P::compiler();
    let rendered = support::rendered(&project);
    GENERATED
        .iter()
        .map(|module| {
            let text = rendered
                .modules
                .iter()
                .find(|candidate| candidate.lean_module == *module)
                .unwrap_or_else(|| panic!("the compiler renders {module}"))
                .lean_text
                .clone();
            Source {
                module: (*module).to_owned(),
                text,
            }
        })
        .collect()
}

/// The build directory with the compiler project's generated `Gnaf`,
/// `GnafFixtures`, `Certification`, and `GnafCertificates` compiled over the
/// shipped calculus modules.
///
/// # Panics
///
/// Panics when Lean rejects a generated module.
#[must_use]
pub fn environment() -> &'static PathBuf {
    static ENVIRONMENT: OnceLock<PathBuf> = OnceLock::new();
    ENVIRONMENT.get_or_init(|| {
        let shipped = shipped().build.clone();
        cached("generated", &generated(), &[shipped])
    })
}

/// The build directory with the library compiled over [`environment`].
///
/// # Panics
///
/// Panics when Lean rejects a library module.
#[must_use]
pub fn library() -> &'static PathBuf {
    static LIBRARY: OnceLock<PathBuf> = OnceLock::new();
    LIBRARY.get_or_init(|| {
        let sources: Vec<Source> = MODULES
            .iter()
            .map(|stem| Source {
                module: module_name(stem),
                text: library_text(stem),
            })
            .collect();
        cached(
            "library",
            &sources,
            &[environment().clone(), shipped().build.clone()],
        )
    })
}

/// The directories Lean searches for the library and what it imports.
#[must_use]
pub fn search() -> Vec<PathBuf> {
    vec![
        library().clone(),
        environment().clone(),
        shipped().build.clone(),
    ]
}

/// Compile one extra module over `search`, in a directory of its own below
/// `root`; the module's name is `module` and its output is Lean's.
#[must_use]
pub fn compile(root: &Path, module: &str, text: &str, search: &[PathBuf]) -> std::process::Output {
    let path = path_of(module);
    let file = root.join("src").join(&path);
    let olean = root
        .join("build")
        .join(path.trim_end_matches(".lean"))
        .with_extension("olean");
    for target in [&file, &olean] {
        std::fs::create_dir_all(target.parent().expect("a parent")).expect("mkdir");
    }
    std::fs::write(&file, text).expect("write");
    let mut directories = vec![root.join("build")];
    directories.extend(search.iter().cloned());
    pinned_with("lean", root, &directories)
        .arg("-o")
        .arg(&olean)
        .arg(&file)
        .output()
        .expect("lean runs")
}

/// Replay `module` through `leanchecker`, over the directories of `search`
/// and `root`'s own build.
#[must_use]
pub fn replay(root: &Path, module: &str, search: &[PathBuf]) -> std::process::Output {
    let mut directories = vec![root.join("build")];
    directories.extend(search.iter().cloned());
    pinned_with("leanchecker", root, &directories)
        .arg(module)
        .output()
        .expect("leanchecker runs")
}

/// The axioms a Lean `#print axioms` listing reports, by declaration.
#[must_use]
pub fn printed_axioms(output: &str) -> BTreeMap<String, Vec<String>> {
    let mut out = BTreeMap::new();
    for line in output.lines() {
        let Some(rest) = line.strip_prefix('\'') else {
            continue;
        };
        let Some((name, tail)) = rest.split_once('\'') else {
            continue;
        };
        let tail = tail.trim();
        let axioms = if tail == "does not depend on any axioms" {
            Vec::new()
        } else if let Some(list) = tail.strip_prefix("depends on axioms: [") {
            let mut axioms: Vec<String> = list
                .trim_end_matches(']')
                .split(',')
                .map(|axiom| axiom.trim().to_owned())
                .filter(|axiom| !axiom.is_empty())
                .collect();
            axioms.sort();
            axioms
        } else {
            continue;
        };
        out.insert(name.to_owned(), axioms);
    }
    out
}
