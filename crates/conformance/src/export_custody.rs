//! The bootstrap production export carried between the jobs of one CI run
//! (SPEC.md §9.4). The `bootstrap-export` job exports the bootstrap project
//! once, requires its packages to equal the committed packages, and uploads
//! the bundle with a custody record that binds it to the repository, the
//! run and its attempt, the commit and its tree, the toolchain pins and the
//! toolchains that ran, and the bundle's bytes. A conformance case of the same
//! run that needs the export takes it from there instead of exporting the
//! project again: the harness names the directory it waits in through
//! [`VARIABLE`], checks the record against its own run and checkout, replays
//! the bundle, and only then hands it to the case. Unset, as in `just vv`,
//! every process that needs the export runs it as before.

use std::collections::BTreeMap;
use std::process::Command;
use std::time::{Duration, Instant};

use camino::{Utf8Path, Utf8PathBuf};
use lexlean::api::production::{self, ProductionOutput, ProductionResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::production_export::{self as harness, Exported, BOOTSTRAP};
use crate::support::{self, P};

/// The variable naming the directory a job takes the run's export in.
pub const VARIABLE: &str = "LEXLEAN_BOOTSTRAP_EXPORT";

/// The custody record's format.
pub const SPEC: &str = "lexlean/ci-bootstrap-export/1";

/// The custody record, beside the bundle in the uploaded directory.
pub const CUSTODY_FILE: &str = "custody.json";

/// The exported bundle, beside the custody record.
pub const BUNDLE_DIR: &str = "bundle";

/// Written by the harness when a case first needs the export.
pub const WANTED: &str = "wanted";

/// Written by the job's fetcher once the artifact is in [`ARTIFACT_DIR`].
pub const READY: &str = "ready";

/// Written by the job's fetcher, with the reason, when the run has no export.
pub const FAILED: &str = "failed";

/// Where the fetcher unpacks the uploaded directory.
pub const ARTIFACT_DIR: &str = "artifact";

/// The repository files whose bytes pin the toolchains and the dependencies.
pub const PINS: [&str; 3] = ["rust-toolchain.toml", "lean-toolchain", "Cargo.lock"];

/// The longest a case waits for the run's export: the export job's own
/// limit, after which the job has failed and the fetcher says so.
const WAIT: Duration = Duration::from_secs(6 * 60 * 60);

/// What binds an uploaded export to the run and the tree that made it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Custody {
    /// Exactly [`SPEC`].
    pub spec: String,
    /// `owner/name` of the repository whose run exported it.
    pub repository: String,
    /// The run that exported it.
    pub run_id: String,
    /// The attempt of that run.
    pub run_attempt: String,
    /// The commit checked out.
    pub commit: String,
    /// The tree of that commit.
    pub tree: String,
    /// The SHA-256 of each file of [`PINS`].
    pub pins: BTreeMap<String, String>,
    /// What the toolchains report: `rustc -vV` and `lean --version`.
    pub toolchains: BTreeMap<String, String>,
    /// The project-relative bundle directory of the export.
    pub bundle_path: String,
    /// [`bundle_digest`] of the bundle.
    pub bundle_sha256: String,
    /// The number of files in the bundle.
    pub bundle_files: u64,
}

/// The run and checkout a job is in: what a custody record must name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    /// `GITHUB_REPOSITORY`.
    pub repository: String,
    /// `GITHUB_RUN_ID`.
    pub run_id: String,
    /// `GITHUB_RUN_ATTEMPT`.
    pub run_attempt: String,
    /// `HEAD`.
    pub commit: String,
    /// `HEAD^{tree}`.
    pub tree: String,
    /// The SHA-256 of each file of [`PINS`].
    pub pins: BTreeMap<String, String>,
    /// What the toolchains report.
    pub toolchains: BTreeMap<String, String>,
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn run(root: &Utf8Path, program: &str, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(root.as_std_path())
        .output()
        .map_err(|error| format!("`{program} {}`: {error}", arguments.join(" ")))?;
    if !output.status.success() {
        return Err(format!(
            "`{program} {}` failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_owned())
        .map_err(|_| format!("`{program} {}` printed no UTF-8", arguments.join(" ")))
}

/// The run and checkout this process is in.
///
/// # Errors
///
/// When the run is not named by GitHub's variables, the checkout's tracked
/// files differ from its commit, or a toolchain cannot be asked its version.
pub fn identity(root: &Utf8Path) -> Result<Identity, String> {
    let variable = |name: &str| -> Result<String, String> {
        std::env::var(name)
            .ok()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| format!("`{name}` is unset: the export crosses jobs of a CI run only"))
    };
    let repository = variable("GITHUB_REPOSITORY")?;
    let run_id = variable("GITHUB_RUN_ID")?;
    let run_attempt = variable("GITHUB_RUN_ATTEMPT")?;
    for (name, value) in [
        ("GITHUB_RUN_ID", &run_id),
        ("GITHUB_RUN_ATTEMPT", &run_attempt),
    ] {
        if !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(format!("`{name}` is not a number: {value}"));
        }
    }
    let changed = run(
        root,
        "git",
        &["status", "--porcelain", "--untracked-files=no"],
    )?;
    if !changed.is_empty() {
        return Err(format!(
            "the checkout's tracked files differ from its commit:\n{changed}"
        ));
    }
    let mut pins = BTreeMap::new();
    for name in PINS {
        let bytes = std::fs::read(root.join(name).as_std_path())
            .map_err(|error| format!("{name}: {error}"))?;
        pins.insert(name.to_owned(), sha256_hex(&bytes));
    }
    let toolchains = BTreeMap::from([
        ("lean".to_owned(), run(root, "lean", &["--version"])?),
        ("rustc".to_owned(), run(root, "rustc", &["-vV"])?),
    ]);
    Ok(Identity {
        repository,
        run_id,
        run_attempt,
        commit: run(root, "git", &["rev-parse", "HEAD"])?,
        tree: run(root, "git", &["rev-parse", "HEAD^{tree}"])?,
        pins,
        toolchains,
    })
}

/// The digest of a bundle directory: the SHA-256 of one line
/// `<sha256 of the file>  <relative path>` per file, in path order, and the
/// number of files.
#[must_use]
pub fn bundle_digest(bundle: &Utf8Path) -> (String, u64) {
    let files = harness::tree(bundle);
    let mut listing = String::new();
    for (relative, bytes) in &files {
        listing.push_str(&sha256_hex(bytes));
        listing.push_str("  ");
        listing.push_str(relative);
        listing.push('\n');
    }
    (sha256_hex(listing.as_bytes()), files.len() as u64)
}

/// Whether `custody` names the run and checkout `current` is in and the
/// bundle whose digest is `bundle`. Any attempt of the same run may take
/// an export an attempt before it made: a rerun of failed jobs reruns the
/// consumers and keeps the export that passed, on the same commit.
///
/// # Errors
///
/// Every field that does not match, one per line.
pub fn check(custody: &Custody, current: &Identity, bundle: &(String, u64)) -> Result<(), String> {
    let mut unmet = Vec::new();
    let mut same =
        |what: &str, recorded: &dyn std::fmt::Debug, here: &dyn std::fmt::Debug, equal: bool| {
            if !equal {
                unmet.push(format!(
                    "{what}: the record names {recorded:?}, this job has {here:?}"
                ));
            }
        };
    same("spec", &custody.spec, &SPEC, custody.spec == SPEC);
    same(
        "repository",
        &custody.repository,
        &current.repository,
        custody.repository == current.repository,
    );
    same(
        "run",
        &custody.run_id,
        &current.run_id,
        custody.run_id == current.run_id,
    );
    let attempt = |value: &str| value.parse::<u64>().ok();
    same(
        "attempt (at most this job's)",
        &custody.run_attempt,
        &current.run_attempt,
        matches!(
            (attempt(&custody.run_attempt), attempt(&current.run_attempt)),
            (Some(recorded), Some(here)) if recorded <= here
        ),
    );
    same(
        "commit",
        &custody.commit,
        &current.commit,
        custody.commit == current.commit,
    );
    same(
        "tree",
        &custody.tree,
        &current.tree,
        custody.tree == current.tree,
    );
    same(
        "pins",
        &custody.pins,
        &current.pins,
        custody.pins == current.pins,
    );
    same(
        "toolchains",
        &custody.toolchains,
        &current.toolchains,
        custody.toolchains == current.toolchains,
    );
    same(
        "bundle",
        &(&custody.bundle_sha256, custody.bundle_files),
        &(&bundle.0, bundle.1),
        custody.bundle_sha256 == bundle.0 && custody.bundle_files == bundle.1,
    );
    let path = Utf8Path::new(&custody.bundle_path);
    same(
        "bundle path (relative, below the project)",
        &custody.bundle_path,
        &"a relative path",
        !custody.bundle_path.is_empty()
            && path
                .components()
                .all(|part| matches!(part, camino::Utf8Component::Normal(_))),
    );
    if unmet.is_empty() {
        Ok(())
    } else {
        Err(unmet.join("\n"))
    }
}

fn copy_files(from: &Utf8Path, to: &Utf8Path) -> Result<(), String> {
    for (relative, bytes) in harness::tree(from) {
        let path = to.join(&relative);
        std::fs::create_dir_all(path.parent().expect("a file has a parent").as_std_path())
            .map_err(|error| format!("{path}: {error}"))?;
        std::fs::write(path.as_std_path(), bytes).map_err(|error| format!("{path}: {error}"))?;
    }
    Ok(())
}

/// Export the bootstrap project once for the run, require its packages to
/// equal the committed packages, and write the bundle and its custody record
/// to `output`. Returns the record.
///
/// # Errors
///
/// When the run or checkout cannot be named, the export fails, a package
/// differs from the committed one, or `output` cannot be written.
pub fn publish(output: &Utf8Path) -> Result<Custody, String> {
    let root = support::repo_root();
    let before = identity(&root)?;
    let exported = harness::export(BOOTSTRAP)?;
    let unmet = harness::differences(BOOTSTRAP, &exported);
    if !unmet.is_empty() {
        return Err(unmet.join("\n"));
    }
    // The export ran from this checkout: it changed no tracked file.
    let after = identity(&root)?;
    if after != before {
        return Err("the checkout changed while the bootstrap project exported".to_owned());
    }
    if output.as_std_path().exists() {
        return Err(format!("{output} exists: the export is written once"));
    }
    let bundle = output.join(BUNDLE_DIR);
    copy_files(&exported.bundle(), &bundle)?;
    let (bundle_sha256, bundle_files) = bundle_digest(&bundle);
    let custody = Custody {
        spec: SPEC.to_owned(),
        repository: before.repository,
        run_id: before.run_id,
        run_attempt: before.run_attempt,
        commit: before.commit,
        tree: before.tree,
        pins: before.pins,
        toolchains: before.toolchains,
        bundle_path: exported.output.bundle.to_string(),
        bundle_sha256,
        bundle_files,
    };
    std::fs::write(
        output.join(CUSTODY_FILE).as_std_path(),
        harness::canonical(&custody),
    )
    .map_err(|error| format!("{output}/{CUSTODY_FILE}: {error}"))?;
    Ok(custody)
}

/// Wait in `directory` for the run's export, check its custody record
/// against this job's run and checkout, place the bundle in a fresh copy of
/// the bootstrap project, and replay it.
///
/// # Errors
///
/// When the run has no export, the record names another run, commit, tree,
/// pin, toolchain, or bundle, or the bundle does not replay.
pub fn take(directory: &Utf8Path) -> Result<Exported, String> {
    std::fs::create_dir_all(directory.as_std_path())
        .and_then(|()| std::fs::write(directory.join(WANTED).as_std_path(), b""))
        .map_err(|error| format!("{directory}: {error}"))?;
    let started = Instant::now();
    eprintln!("bootstrap export: waiting in {directory} for the run's export");
    loop {
        if directory.join(READY).as_std_path().exists() {
            break;
        }
        if let Ok(reason) = std::fs::read_to_string(directory.join(FAILED).as_std_path()) {
            return Err(format!(
                "the run has no bootstrap export: {}",
                reason.trim()
            ));
        }
        if started.elapsed() > WAIT {
            return Err(format!("no export reached {directory} in {WAIT:?}"));
        }
        std::thread::sleep(Duration::from_secs(5));
    }
    let artifact = directory.join(ARTIFACT_DIR);
    let custody: Custody = serde_json::from_slice(
        &std::fs::read(artifact.join(CUSTODY_FILE).as_std_path())
            .map_err(|error| format!("{CUSTODY_FILE}: {error}"))?,
    )
    .map_err(|error| format!("{CUSTODY_FILE}: {error}"))?;
    let source = artifact.join(BUNDLE_DIR);
    let digest = bundle_digest(&source);
    let root = support::repo_root();
    check(&custody, &identity(&root)?, &digest)?;
    let project = P::bootstrap();
    let bundle = project.root.join(&custody.bundle_path);
    copy_files(&source, &bundle)?;
    let result: ProductionResult = serde_json::from_slice(
        &std::fs::read(bundle.join("result.json").as_std_path())
            .map_err(|error| format!("result.json: {error}"))?,
    )
    .map_err(|error| format!("result.json: {error}"))?;
    let report = production::verify_artifact(&bundle)
        .map_err(|error| format!("the bundle does not replay: {:?}", error.diagnostics))?;
    if report.result_id != result.result_id {
        return Err(format!(
            "the replay names result {}, result.json {}",
            report.result_id, result.result_id
        ));
    }
    eprintln!(
        "bootstrap export: taken from run {} attempt {} after {:?} (commit {}, bundle sha256 {}, {} files), replayed",
        custody.run_id,
        custody.run_attempt,
        started.elapsed(),
        custody.commit,
        custody.bundle_sha256,
        custody.bundle_files
    );
    Ok(Exported {
        project,
        output: ProductionOutput {
            result,
            bundle: Utf8PathBuf::from(&custody.bundle_path),
        },
        report,
        gnaf: None,
    })
}
