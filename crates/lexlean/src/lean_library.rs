//! External Lean libraries (SPEC.md §10.4): the offline resolution of a
//! declared library in the Lake workspace, the rows it contributes to the
//! lock, the comparison of those rows with the lock, and `deps fetch`, the one
//! operation that acquires a library.
//!
//! Resolution never touches the network and never runs a process: a library
//! that is not there is `LLV7019`, and the way to make it be there is
//! `lexlean deps fetch`, which is the only code in this module that starts a
//! child.

use crate::artifact::content_id::Sha256Digest;
use crate::code;
use crate::config::{is_project_relative, LeanLibrary};
use crate::diagnostic::Diagnostic;
use crate::lexicon::package::LexiconPackage;
use crate::lock::LockLeanLibrary;
use crate::project::Project;

const LAKE_MANIFEST: &str = "lake-manifest.json";
const DEFAULT_PACKAGES_DIR: &str = ".lake/packages";
/// Where Lake keeps the built `.olean` files of a package, below the package.
const BUILT_ARTIFACTS: &str = ".lake/build/lib/lean";

/// The digest of a glossary that is not there: the tree digest (§11.5) of no
/// files. A project that declares no `lean.<name>` lexicon package has an
/// empty glossary for the library, and the lock says so with this value
/// rather than leaving the field out, so adding the generated glossary later
/// is a visible change to the row.
fn empty_tree_digest() -> Sha256Digest {
    Sha256Digest::of(b"lexlean-tree-v1\0")
}

fn workspace_relative(project: &Project, name: &str) -> String {
    if project.config.lean_workspace == "." {
        name.to_owned()
    } else {
        format!("{}/{name}", project.config.lean_workspace)
    }
}

fn mismatch(message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code!("LLC0110"), message)
}

fn not_fetched(library: &LeanLibrary, detail: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(
        code!("LLV7019"),
        format!(
            "Lean library `{}` ({} at {}) is not available: {detail}; no command but `lexlean deps fetch` acquires a library, so run `lexlean deps fetch`",
            library.name, library.url, library.revision
        ),
    )
}

/// A copy that cannot be written is an environment failure, not a failed fetch.
fn host_failure(detail: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(code!("LLV7010"), detail.to_string())
}

fn fetch_failure(detail: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(
        code!("LLV7020"),
        format!("`lexlean deps fetch` failed: {detail}"),
    )
}

/// Does `relative` exist under the project root, without following a link?
fn exists(project: &Project, relative: &str) -> bool {
    std::fs::symlink_metadata(project.root.join(relative).as_std_path()).is_ok()
}

/// The project-relative directory the Lake workspace puts `library` in, once
/// its `lake-manifest.json` requires the library at the declared URL and
/// revision (§10.4). The declaration is the single statement of the pin; the
/// workspace manifest must agree with it, and disagreement is `LLC0110`
/// naming both.
fn required_dir(project: &Project, library: &LeanLibrary) -> Result<String, Diagnostic> {
    let manifest = workspace_relative(project, LAKE_MANIFEST);
    let path = project.confined_file_or_missing(&manifest, || {
        mismatch(format!(
            "the Lake workspace has no `{manifest}`, so it does not require Lean library `{}`; lexlean.toml declares {} at {}",
            library.name, library.url, library.revision
        ))
    })?;
    let bytes = std::fs::read(path.as_std_path())
        .map_err(|io_error| mismatch(format!("{manifest}: {io_error}")))?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|parse_error| mismatch(format!("{manifest}: not JSON: {parse_error}")))?;
    let packages_dir = value
        .get("packagesDir")
        .and_then(serde_json::Value::as_str)
        .unwrap_or(DEFAULT_PACKAGES_DIR);
    if !is_project_relative(packages_dir) {
        return Err(mismatch(format!(
            "{manifest}: packagesDir `{packages_dir}` is not a relative directory of the Lake workspace"
        )));
    }
    let entry = value
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .and_then(|packages| {
            packages.iter().find(|package| {
                package.get("name").and_then(serde_json::Value::as_str) == Some(&library.name)
            })
        });
    let Some(entry) = entry else {
        return Err(mismatch(format!(
            "the Lake workspace `{manifest}` does not require `{}`; lexlean.toml declares {} at {}",
            library.name, library.url, library.revision
        )));
    };
    let found = |key: &str| {
        entry
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
    };
    if found("url") != library.url || found("rev") != library.revision {
        return Err(mismatch(format!(
            "the Lake workspace `{manifest}` requires `{}` at {} {}, but lexlean.toml declares {} at {}",
            library.name,
            found("url"),
            found("rev"),
            library.url,
            library.revision
        )));
    }
    Ok(workspace_relative(
        project,
        &format!("{packages_dir}/{}", library.name),
    ))
}

/// The library as Lake and `lake exe cache get` leave it, or `LLV7019`: a
/// checkout whose `HEAD` is the declared commit, its own `lake-manifest.json`,
/// and its built artifacts. Returns the digest of the library's manifest.
fn present(
    project: &Project,
    library: &LeanLibrary,
    directory: &str,
) -> Result<Sha256Digest, Diagnostic> {
    if !exists(project, directory) {
        return Err(not_fetched(
            library,
            format!("`{directory}` does not exist"),
        ));
    }
    let head = format!("{directory}/.git/HEAD");
    if !exists(project, &head) {
        return Err(not_fetched(
            library,
            format!("`{directory}` is not a Git checkout"),
        ));
    }
    let head_bytes = std::fs::read(project.confined_file(&head)?.as_std_path())
        .map_err(|io_error| not_fetched(library, format!("{head}: {io_error}")))?;
    let at = String::from_utf8_lossy(&head_bytes).trim().to_owned();
    if at != library.revision {
        return Err(not_fetched(
            library,
            format!("`{directory}` is at `{at}`, not the declared revision"),
        ));
    }
    let built = format!("{directory}/{BUILT_ARTIFACTS}");
    if !exists(project, &built) {
        return Err(not_fetched(
            library,
            format!("`{built}` does not exist, so there are no built `.olean` files"),
        ));
    }
    project.confined_dir(&built)?;
    let manifest = format!("{directory}/{LAKE_MANIFEST}");
    if !exists(project, &manifest) {
        return Err(not_fetched(library, format!("`{manifest}` does not exist")));
    }
    let bytes = std::fs::read(project.confined_file(&manifest)?.as_std_path())
        .map_err(|io_error| not_fetched(library, format!("{manifest}: {io_error}")))?;
    Ok(Sha256Digest::of(&bytes))
}

/// The lock rows of the project's declared libraries (§11.1), read offline
/// from the Lake workspace. `packages` are the project's loaded lexicon
/// packages: the glossary of a library is the one named `lean.<name>`.
///
/// # Errors
///
/// `LLC0110` for a Lake workspace that does not require a library as declared
/// and `LLV7019` for a library that is not available.
pub fn resolve(
    project: &Project,
    packages: &[LexiconPackage],
) -> Result<Vec<LockLeanLibrary>, Vec<Diagnostic>> {
    let mut rows = Vec::new();
    let mut diagnostics = Vec::new();
    for library in &project.config.lean_libraries {
        let outcome = required_dir(project, library)
            .and_then(|directory| present(project, library, &directory));
        match outcome {
            Ok(lake_manifest_sha256) => {
                let glossary = format!("lean.{}", library.name);
                let glossary_sha256 = packages
                    .iter()
                    .find(|package| package.id == glossary)
                    .map_or_else(empty_tree_digest, |package| package.tree_sha256);
                rows.push(LockLeanLibrary {
                    name: library.name.clone(),
                    url: library.url.clone(),
                    revision: library.revision.clone(),
                    lake_manifest_sha256,
                    glossary_sha256,
                });
            }
            Err(diagnostic) => diagnostics.push(diagnostic),
        }
    }
    if diagnostics.is_empty() {
        Ok(rows)
    } else {
        Err(diagnostics)
    }
}

/// Require the rows a lock holds to be the libraries as they now are, naming
/// both values of the first field that differs (§10.4).
///
/// # Errors
///
/// `LLC0110` for each row that is not the library as it now is.
pub fn check_pins(
    locked: &[LockLeanLibrary],
    expected: &[LockLeanLibrary],
) -> Result<(), Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    for row in locked {
        match expected.iter().find(|candidate| candidate.name == row.name) {
            None => diagnostics.push(mismatch(format!(
                "the lock pins Lean library `{}`, which the project does not declare",
                row.name
            ))),
            Some(current) if current != row => {
                let mut differences = Vec::new();
                if row.url != current.url {
                    differences.push(format!("url: locked {}, found {}", row.url, current.url));
                }
                if row.revision != current.revision {
                    differences.push(format!(
                        "revision: locked {}, found {}",
                        row.revision, current.revision
                    ));
                }
                if row.lake_manifest_sha256 != current.lake_manifest_sha256 {
                    differences.push(format!(
                        "lake_manifest_sha256: locked {}, found {}",
                        row.lake_manifest_sha256.to_hex(),
                        current.lake_manifest_sha256.to_hex()
                    ));
                }
                if row.glossary_sha256 != current.glossary_sha256 {
                    differences.push(format!(
                        "glossary_sha256: locked {}, found {}",
                        row.glossary_sha256.to_hex(),
                        current.glossary_sha256.to_hex()
                    ));
                }
                diagnostics.push(mismatch(format!(
                    "Lean library `{}` is not the library the lock pins: {}",
                    row.name,
                    differences.join("; ")
                )));
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

/// Give a scratch copy of the project the three things `present` reads of each
/// declared library (the checkout's `HEAD`, its own manifest, and the
/// directory of built artifacts), not the library. `lexlean fmt` re-checks the
/// project in such a copy (§23.5); the copy never runs Lean, so it needs the
/// library to be recognised as fetched and not its seven gigabytes.
///
/// # Errors
///
/// `LLC0110` for a Lake workspace that does not require a library as declared,
/// `LLV7019` for a library that is not available, and an environment failure
/// when the copy cannot be written.
pub fn stub_for_scratch(
    project: &Project,
    scratch: &camino::Utf8Path,
) -> Result<(), Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    for library in &project.config.lean_libraries {
        let outcome = required_dir(project, library).and_then(|directory| {
            present(project, library, &directory)?;
            let copy = |relative: String| -> Result<(), Diagnostic> {
                let source = project.confined_file(&relative)?;
                let destination = scratch.join(&relative);
                if let Some(parent) = destination.parent() {
                    std::fs::create_dir_all(parent.as_std_path())
                        .map_err(|io_error| host_failure(format!("{relative}: {io_error}")))?;
                }
                std::fs::copy(source.as_std_path(), destination.as_std_path())
                    .map_err(|io_error| host_failure(format!("{relative}: {io_error}")))?;
                Ok(())
            };
            copy(format!("{directory}/.git/HEAD"))?;
            copy(format!("{directory}/{LAKE_MANIFEST}"))?;
            let built = scratch.join(format!("{directory}/{BUILT_ARTIFACTS}"));
            std::fs::create_dir_all(built.as_std_path())
                .map_err(|io_error| host_failure(format!("{built}: {io_error}")))
        });
        if let Err(diagnostic) = outcome {
            diagnostics.push(diagnostic);
        }
    }
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

/// The libraries `deps fetch` acquired, by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fetched {
    /// The declared libraries, now present, sorted by name.
    pub libraries: Vec<String>,
}

/// Acquire every declared library with `lake exe cache get` in the Lake
/// workspace (§23.4): the library's sources are cloned by Lake at the
/// revision the workspace manifest pins, and Mathlib's cache supplies its
/// `.olean` files so that nothing is rebuilt. This is the only function that
/// starts a process for a library or reaches the network.
///
/// # Errors
///
/// `LLC0110` when the Lake workspace does not require a library as declared,
/// `LLV7020` when `lake` cannot run or fails or the library is not present
/// afterwards, and `LLS8002` when the cache download exceeds
/// `child_timeout_ms`.
pub fn fetch(project: &Project) -> Result<Fetched, Vec<Diagnostic>> {
    use crate::verify::child::{resolve_on_path, ChildHome, ChildSpec, Normalizer};

    let libraries = &project.config.lean_libraries;
    let mut directories = Vec::new();
    let mut diagnostics = Vec::new();
    for library in libraries {
        match required_dir(project, library) {
            Ok(directory) => directories.push(directory),
            Err(diagnostic) => diagnostics.push(diagnostic),
        }
    }
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    if libraries.is_empty() {
        return Ok(Fetched {
            libraries: Vec::new(),
        });
    }

    let workspace = if project.config.lean_workspace == "." {
        project.root.clone()
    } else {
        project
            .confined_dir(&project.config.lean_workspace)
            .map_err(|d| vec![d])?
    };
    let lake = resolve_on_path("lake").map_err(|_| {
        vec![fetch_failure(
            "no `lake` executable is available on PATH; install the pinned toolchain with elan",
        )]
    })?;
    let lake_sha256 = Sha256Digest::of(
        &std::fs::read(lake.as_std_path())
            .map_err(|io_error| vec![fetch_failure(format!("{lake}: {io_error}"))])?,
    );
    let lake_bin = lake
        .parent()
        .map_or_else(|| project.root.clone(), camino::Utf8Path::to_path_buf);
    let normalizer = Normalizer::new(&workspace, &project.root, &workspace, &lake_bin);
    let record = crate::verify::child::run(
        &ChildSpec {
            tool: "lake",
            module: Some("deps:fetch".to_owned()),
            program: &lake,
            executable_sha256: lake_sha256,
            argv: ["exe", "cache", "get"].map(str::to_owned).to_vec(),
            cwd: &workspace,
            extra_env: Vec::new(),
            home: ChildHome::Toolchain {
                toolchain_bin: &lake_bin,
            },
        },
        &project.config.limits,
        &normalizer,
    )
    .map_err(|diagnostic| {
        // A timeout is a resource-limit violation and keeps its own code.
        if diagnostic.code.as_str() == "LLS8002" {
            vec![diagnostic]
        } else {
            vec![fetch_failure(diagnostic.message)]
        }
    })?;
    if record.exit_code != 0 {
        return Err(vec![fetch_failure(format!(
            "`lake exe cache get` exited {}: {}",
            record.exit_code,
            record.stderr.trim_end()
        ))]);
    }
    let mut names = Vec::new();
    for (library, directory) in libraries.iter().zip(&directories) {
        present(project, library, directory).map_err(|diagnostic| {
            vec![fetch_failure(format!(
                "`lake exe cache get` succeeded but {}",
                diagnostic.message
            ))]
        })?;
        names.push(library.name.clone());
    }
    Ok(Fetched { libraries: names })
}
