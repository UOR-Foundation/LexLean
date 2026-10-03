//! Building rendered Rust packages with their harnesses under cargo
//! (SPEC.md §17.16): the workspace the Rust backend's and the preservation
//! suite's rustc runs share.

use std::collections::BTreeMap;
use std::path::Path;

fn cargo() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
}

/// A package to build: its crate name, its files, and the harness that
/// calls its export.
pub struct Built {
    /// The crate name.
    pub name: String,
    /// The package files by path.
    pub files: BTreeMap<String, Vec<u8>>,
    /// The source of the harness binary.
    pub harness: String,
}

/// Write `packages` and one harness crate each into a workspace at `dir`.
///
/// # Panics
///
/// Panics when a file cannot be written.
pub fn workspace(dir: &Path, packages: &[Built]) {
    let mut members = Vec::new();
    for Built {
        name,
        files,
        harness,
    } in packages
    {
        let package = dir.join("p").join(name);
        for (path, bytes) in files {
            let file = package.join(path);
            std::fs::create_dir_all(file.parent().expect("parent")).expect("package directory");
            std::fs::write(file, bytes).expect("package file");
        }
        let runner = dir.join("h").join(format!("run_{name}"));
        std::fs::create_dir_all(runner.join("src")).expect("harness directory");
        std::fs::write(
            runner.join("Cargo.toml"),
            format!(
                "[package]\nname = \"run_{name}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{name} = {{ path = \"../../p/{name}\" }}\n"
            ),
        )
        .expect("harness manifest");
        std::fs::write(runner.join("src/main.rs"), harness).expect("harness source");
        members.push(format!("\"p/{name}\""));
        members.push(format!("\"h/run_{name}\""));
    }
    std::fs::write(
        dir.join("Cargo.toml"),
        format!(
            "[workspace]\nresolver = \"2\"\nmembers = [{}]\n",
            members.join(", ")
        ),
    )
    .expect("workspace manifest");
}

/// Run `cargo` with `arguments` in `dir`, with `dir`'s own target directory
/// and no inherited rustc flags.
///
/// # Panics
///
/// Panics when cargo cannot be started.
pub fn cargo_in(dir: &Path, arguments: &[&str]) -> std::process::Output {
    std::process::Command::new(cargo())
        .args(arguments)
        .current_dir(dir)
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .output()
        .expect("cargo runs")
}
