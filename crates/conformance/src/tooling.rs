//! The committed tooling fixtures (SPEC.md §17.20, §28.3, §28.4).
//!
//! A tooling module is accepted only in a project the compiler itself embeds,
//! so the published-build gates that drive the command line over `examples/`
//! can never reach one. This module is their counterpart for
//! `tests/tooling/<fixture>/`: it loads each project with tooling enabled,
//! renders the complete platform-independent build tree, and compares it with
//! `expected/build/` byte for byte. Golden bytes change only through
//! `cargo xtask check-golden --write`.

use std::path::{Path, PathBuf};

use camino::Utf8PathBuf;

/// The files of one rendered build: relative path and bytes.
pub type Files = Vec<(String, Vec<u8>)>;

/// The directory holding the fixtures.
pub const DIRECTORY: &str = "tests/tooling";

/// Every fixture directory, sorted.
///
/// # Errors
///
/// Returns an error when the directory cannot be read or holds no fixture:
/// a gate over nothing would read as evidence.
pub fn fixtures(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut dirs = Vec::new();
    let read =
        std::fs::read_dir(root.join(DIRECTORY)).map_err(|error| format!("{DIRECTORY}: {error}"))?;
    for entry in read {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            dirs.push(path);
        }
    }
    dirs.sort();
    if dirs.is_empty() {
        return Err(format!(
            "TL-11: {DIRECTORY} holds no fixture; the gate has nothing to check"
        ));
    }
    Ok(dirs)
}

fn copy_project(from: &Path, to: &Path) -> Result<(), String> {
    for entry in walkdir::WalkDir::new(from).into_iter().flatten() {
        let relative = entry
            .path()
            .strip_prefix(from)
            .map_err(|error| error.to_string())?;
        let text = relative.to_string_lossy();
        if text.is_empty() || text.starts_with(".lexlean") || text.starts_with("expected") {
            continue;
        }
        let destination = to.join(relative);
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&destination).map_err(|error| error.to_string())?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = destination.parent() {
                std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            std::fs::copy(entry.path(), &destination).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

/// The project at `config`, as the compiler embeds it: tooling enabled.
///
/// # Errors
///
/// Returns the project's own load error.
pub fn embedded(config: &Utf8PathBuf) -> Result<lexlean::project::Project, String> {
    Ok(lexlean::project::Project::load(config)
        .map_err(|error| format!("{config}: {error}"))?
        .with_tooling())
}

/// The checked and rendered build of the project whose configuration is
/// `config`, with the lock computed in memory (a tooling fixture commits none,
/// so that no committed lock can drift from the compiler-semantics ID).
///
/// # Errors
///
/// Returns the first refusal, with its diagnostic codes.
pub fn render(config: &Utf8PathBuf) -> Result<lexlean::api::RenderedBuild, String> {
    let project = embedded(config)?;
    let (lock, packages) = lexlean::lock::compute_lock(&project, false)
        .map_err(|diagnostics| format!("lock: {diagnostics:?}"))?;
    let checked = lexlean::link::check_project(
        &project,
        &lexlean::project::Selection::Entrypoints,
        &lock,
        packages,
    )
    .map_err(|error| format!("check: {error}"))?;
    lexlean::api::render_build(&project, &checked).map_err(|error| format!("render: {error}"))
}

/// The build tree of a fixture, rendered from a fresh copy in a temporary
/// directory. The directory is returned so that the caller can show that two
/// calls used distinct ones.
///
/// # Errors
///
/// Returns the first failure, and refuses a file that embeds the temporary
/// project root (AR-07).
pub fn files(fixture: &Path) -> Result<(tempfile::TempDir, Files), String> {
    let temp = tempfile::Builder::new()
        .prefix("lexlean-tooling-")
        .tempdir()
        .map_err(|error| error.to_string())?;
    copy_project(fixture, temp.path())?;
    let config = Utf8PathBuf::from_path_buf(temp.path().join("lexlean.toml"))
        .map_err(|bad| format!("non-UTF-8 path {}", bad.display()))?;
    let build = render(&config)?;
    let root_text = temp.path().to_string_lossy().into_owned();
    let mut out: Files = Vec::new();
    for (relative, bytes) in build.files {
        if String::from_utf8_lossy(&bytes).contains(&root_text) {
            return Err(format!(
                "AR-07: {relative} embeds the absolute project path {root_text}"
            ));
        }
        out.push((relative, bytes));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok((temp, out))
}

fn name_of(fixture: &Path) -> String {
    fixture
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

fn read_tree(base: &Path) -> Result<Files, String> {
    let mut out: Files = Vec::new();
    for entry in walkdir::WalkDir::new(base).into_iter().flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(base)
            .map_err(|error| error.to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = std::fs::read(entry.path()).map_err(|error| format!("{relative}: {error}"))?;
        out.push((relative, bytes));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// §28.3: the published build tree of every tooling fixture equals its
/// committed golden byte for byte; `write` replaces the goldens.
///
/// # Errors
///
/// Returns the first fixture whose tree differs, naming the file.
pub fn check_golden(root: &Path, write: bool) -> Result<usize, String> {
    let mut count = 0;
    for fixture in fixtures(root)? {
        let name = name_of(&fixture);
        let (_temp, files) = files(&fixture)?;
        let expected = fixture.join("expected").join("build");
        if write {
            if expected.exists() {
                std::fs::remove_dir_all(&expected).map_err(|error| error.to_string())?;
            }
            for (relative, bytes) in &files {
                let destination = expected.join(relative);
                if let Some(parent) = destination.parent() {
                    std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
                }
                std::fs::write(destination, bytes).map_err(|error| error.to_string())?;
            }
        } else {
            let committed = read_tree(&expected)?;
            if committed.iter().map(|(path, _)| path).collect::<Vec<_>>()
                != files.iter().map(|(path, _)| path).collect::<Vec<_>>()
            {
                return Err(format!(
                    "R10: {name}: the golden file set differs from the published one; run `cargo xtask check-golden --write`"
                ));
            }
            for ((path, bytes), (_, golden)) in files.iter().zip(&committed) {
                if bytes != golden {
                    return Err(format!(
                        "R10: {name}: {path} differs from its golden; run `cargo xtask check-golden --write`"
                    ));
                }
            }
        }
        count += files.len();
    }
    Ok(count)
}

/// §28.4: two clean builds of every tooling fixture in distinct directories
/// publish byte-identical trees.
///
/// # Errors
///
/// Returns the first fixture whose two trees differ, naming the file.
pub fn check_reproducibility(root: &Path) -> Result<usize, String> {
    let mut count = 0;
    for fixture in fixtures(root)? {
        let name = name_of(&fixture);
        let (first, one) = files(&fixture)?;
        let (second, two) = files(&fixture)?;
        if first.path() == second.path() {
            return Err("the two build directories must be distinct".to_owned());
        }
        if one != two {
            let differing = one
                .iter()
                .zip(&two)
                .find(|(a, b)| a != b)
                .map_or_else(|| "the file sets".to_owned(), |(a, _)| a.0.clone());
            return Err(format!(
                "R10: {name}: {differing} differs between two clean builds in distinct directories"
            ));
        }
        count += one.len();
    }
    Ok(count)
}
