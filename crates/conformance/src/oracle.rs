//! The differential validation oracle (SPEC.md §17.19): `lean4-prod`, an
//! independent Lean 4 to Rust generator, acquired at a pinned commit and run
//! against the Lean LexLean generates.
//!
//! The oracle is validation evidence only. Everything here is test
//! infrastructure of the conformance crate: the shipped crate never names it,
//! no identity LexLean computes takes it as an input, and the gate that
//! checks that (`audit-oracle-isolation`) reads this crate's pin file, not
//! the other way round.
//!
//! The reason this module is careful about *failure* is the whole point of
//! the integration. An oracle that could not be fetched, did not build,
//! crashed, was killed, or produced nothing has said nothing, and a harness
//! that read that silence as agreement would report a validation that never
//! happened. So there are exactly two kinds of answer from the tool,
//! [`Verdict::Accepted`] and [`Verdict::Refused`] (the oracle's own, structured
//! refusal of a program), and everything else is [`Unavailable`], which no
//! caller can turn into a validated result.

use std::collections::BTreeMap;
use std::fmt;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::support::repo_root;

/// The role of the oracle in every statement this repository makes about it.
pub const ROLE: &str = "validation-evidence";

/// The domain of the canonical frame of the oracle's files.
const TREE_DOMAIN: &[u8] = b"lexlean-oracle-tree-v1\0";

/// The marker the oracle's exporter writes in front of the reason it
/// refuses a program. Only this marker makes a failed run a refusal.
const EXPORT_REFUSAL: &str = "prod-export failed: ";

/// The marker the oracle's code generator writes in front of the reason it
/// refuses a program.
const CODEGEN_REFUSAL: &str = "Code generation refused: ";

/// The pin of the oracle (`oracle/lean4-prod.toml`).
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Pin {
    /// The schema tag, `lexlean/oracle-pin/1`.
    pub spec: String,
    /// The oracle's name.
    pub name: String,
    /// The role: always [`ROLE`].
    pub role: String,
    /// Where the oracle is fetched from.
    pub remote: String,
    /// The exact commit.
    pub commit: String,
    /// The commit's git tree, for a reader with git.
    pub git_tree: String,
    /// The path prefixes of the files the integration uses.
    pub used: Vec<String>,
    /// How many files those are.
    pub tree_files: usize,
    /// The SHA-256 of their canonical frame.
    pub tree_sha256: String,
    /// The Lean toolchain the oracle is built with.
    pub lean_toolchain: String,
    /// The Rust toolchain the oracle is built with.
    pub rust_toolchain: String,
    /// The Lake target that is built.
    pub lean_target: String,
    /// The Cargo package that is built.
    pub cargo_package: String,
    /// The bound on one invocation, in seconds.
    pub timeout_seconds: u64,
    /// The licence finding.
    pub licence: Licence,
}

/// What the pin records about the oracle's licence.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Licence {
    /// What the oracle's manifests declare.
    pub declared: String,
    /// Whether the oracle's repository ships a licence text.
    pub text_shipped: bool,
    /// Whether any of the oracle is vendored here.
    pub vendored: bool,
    /// Why.
    pub reason: String,
}

fn is_hex(text: &str, length: usize) -> bool {
    text.len() == length
        && text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

impl Pin {
    /// Read the committed pin.
    ///
    /// # Panics
    ///
    /// Panics when the committed pin is not valid: that is a defect of the
    /// repository, not a condition of the host.
    #[must_use]
    pub fn committed() -> Self {
        let text =
            std::fs::read_to_string(repo_root().join("oracle/lean4-prod.toml").as_std_path())
                .expect("oracle/lean4-prod.toml");
        Self::parse(&text).unwrap_or_else(|reason| panic!("oracle/lean4-prod.toml: {reason}"))
    }

    /// Parse and validate a pin.
    ///
    /// # Errors
    ///
    /// Returns the first way the text is not a complete pin: an unknown or
    /// missing field, a role other than [`ROLE`], a commit or digest that is
    /// not lowercase hexadecimal of the right length, or an unusable list of
    /// used paths.
    pub fn parse(text: &str) -> Result<Self, String> {
        let pin: Self = toml::from_str(text).map_err(|error| error.to_string())?;
        if pin.spec != "lexlean/oracle-pin/1" {
            return Err(format!("unknown pin schema `{}`", pin.spec));
        }
        if pin.role != ROLE {
            return Err(format!(
                "the oracle's role is `{}`; it is `{ROLE}` and nothing else",
                pin.role
            ));
        }
        if !is_hex(&pin.commit, 40) || !is_hex(&pin.git_tree, 40) {
            return Err("the commit and its git tree are 40 lowercase hexadecimal digits".into());
        }
        if !is_hex(&pin.tree_sha256, 64) {
            return Err("tree_sha256 is 64 lowercase hexadecimal digits".into());
        }
        if pin.used.is_empty()
            || pin
                .used
                .iter()
                .any(|path| path.is_empty() || path.starts_with('/') || path.contains(".."))
        {
            return Err("`used` names relative path prefixes of the checkout".into());
        }
        if pin.tree_files == 0 || pin.timeout_seconds == 0 {
            return Err("tree_files and timeout_seconds are positive".into());
        }
        if pin.licence.vendored {
            return Err(
                "nothing of the oracle is vendored: the repository ships no licence text".into(),
            );
        }
        Ok(pin)
    }

    fn uses(&self, relative: &str) -> bool {
        self.used.iter().any(|prefix| {
            relative == prefix.trim_end_matches('/') || relative.starts_with(prefix.as_str())
        })
    }
}

/// The SHA-256 of the canonical frame of the files of `root` the pin uses,
/// and how many there are.
///
/// # Errors
///
/// Returns the I/O error that kept a file from being read.
pub fn tree_digest(root: &Path, pin: &Pin) -> Result<(String, usize), String> {
    let mut files: Vec<(String, PathBuf)> = Vec::new();
    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), ".git" | ".lake" | "target")
                    || crate::support::holds_cache_tag(entry)))
        })
    {
        let entry = entry.map_err(|error| error.to_string())?;
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(root)
            .map_err(|error| error.to_string())?
            .components()
            .map(|part| part.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/");
        if pin.uses(&relative) {
            files.push((relative, entry.path().to_path_buf()));
        }
    }
    files.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    let mut hasher = Sha256::new();
    hasher.update(TREE_DOMAIN);
    for (relative, path) in &files {
        let bytes = std::fs::read(path).map_err(|error| format!("{relative}: {error}"))?;
        hasher.update((relative.len() as u64).to_be_bytes());
        hasher.update(relative.as_bytes());
        hasher.update((bytes.len() as u64).to_be_bytes());
        hasher.update(Sha256::digest(&bytes));
    }
    let hex = hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    Ok((hex, files.len()))
}

/// Why the oracle could not say anything. None of these is a validation, and
/// none is a disagreement: the gate that meets one fails on the normative
/// host and reports "not validated" elsewhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unavailable {
    /// No checkout, and none could be fetched.
    NotAcquired(String),
    /// A checkout exists but is not the pinned one.
    PinMismatch(String),
    /// The pinned oracle could not be built with the pinned toolchains.
    BuildFailed {
        /// The step.
        step: String,
        /// The tool's output.
        detail: String,
    },
    /// A tool of the oracle failed in a way that is not its structured
    /// refusal of a program: a crash, a signal, a timeout, an unreadable or
    /// missing output.
    ToolFailed {
        /// The step.
        step: String,
        /// What happened.
        detail: String,
    },
    /// The run had nothing to validate: a comparison over zero cases says
    /// nothing, and is never read as agreement.
    Vacuous(String),
    /// The pin names toolchains other than LexLean's own, or the pinned
    /// oracle needs another Lean: a repository inconsistency to be fixed by
    /// repinning, not a condition of the host or of the network.
    ToolchainDrift(String),
}

impl fmt::Display for Unavailable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAcquired(detail) => write!(f, "the oracle was not acquired: {detail}"),
            Self::PinMismatch(detail) => {
                write!(f, "the checkout is not the pinned oracle: {detail}")
            }
            Self::BuildFailed { step, detail } => {
                write!(f, "the oracle did not build at `{step}`: {detail}")
            }
            Self::ToolFailed { step, detail } => {
                write!(
                    f,
                    "the oracle failed at `{step}` without refusing the program: {detail}"
                )
            }
            Self::Vacuous(detail) => write!(f, "nothing was validated: {detail}"),
            Self::ToolchainDrift(detail) => write!(
                f,
                "the oracle's pin and LexLean's toolchains disagree (repin: SPEC.md §17.19): {detail}"
            ),
        }
    }
}

impl std::error::Error for Unavailable {}

/// Where an oracle refusal arose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    /// The Lean exporter (`prod-export`) refused the root.
    Export,
    /// The Rust code generator (`prod`) refused the exported module.
    Codegen,
}

/// The oracle's answer about one root: it generated Rust, or it refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// The oracle generated this package: its files by path.
    Accepted {
        /// The exported intermediate representation.
        ir: String,
        /// The generated crate's files.
        package: BTreeMap<String, Vec<u8>>,
    },
    /// The oracle refused the program, in its own structured words.
    Refused {
        /// Which stage refused.
        stage: Stage,
        /// The reason, as the oracle printed it (first line).
        message: String,
    },
}

/// An acquired and built oracle.
#[derive(Debug)]
pub struct Oracle {
    pin: Pin,
    source: PathBuf,
    exporter: PathBuf,
    prod: PathBuf,
    scratch: PathBuf,
    bin: PathBuf,
}

/// Where the oracle is cached: a directory of the build, or the environment's.
fn cache_root() -> PathBuf {
    if let Some(dir) = std::env::var_os("LEXLEAN_ORACLE_CACHE") {
        return PathBuf::from(dir);
    }
    let target = std::env::var_os("CARGO_TARGET_DIR").map_or_else(
        || repo_root().join("target").into_std_path_buf(),
        PathBuf::from,
    );
    target.join("lexlean-oracle")
}

/// A command of the pinned toolchain's own `bin`, so a tool that spawns
/// another (Lake spawns `lean`) finds the pinned one whatever an elan proxy
/// would choose.
fn toolchain_command(program: &str, cwd: &Path) -> Command {
    let bin = crate::preservation::toolchain_bin();
    let path = std::env::join_paths(std::iter::once(bin.clone()).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("a search path");
    let mut command = Command::new(bin.join(program));
    command.current_dir(cwd).env("PATH", path);
    command
}

/// What a finished process left.
pub(crate) struct Ran {
    pub(crate) status: Option<i32>,
    pub(crate) stdout: String,
    pub(crate) stderr: String,
}

/// End a child and everything it started: the process group it leads (unix)
/// or its process tree (windows), then the child itself.
fn stop_tree(child: &mut std::process::Child) {
    #[cfg(unix)]
    {
        use rustix::process::{kill_process_group, Pid, Signal};
        if let Some(group) = i32::try_from(child.id()).ok().and_then(Pid::from_raw) {
            let _ = kill_process_group(group, Signal::KILL);
        }
    }
    #[cfg(windows)]
    {
        let _ = Command::new("taskkill")
            .args(["/PID", &child.id().to_string(), "/T", "/F"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let _ = child.kill();
    let _ = child.wait();
}

/// Run `command` to completion within `limit`, or end it and everything it
/// started, and say so.
pub(crate) fn run(mut command: Command, limit: Duration) -> Result<Ran, String> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    // The child leads a group of its own, so that a timeout ends what it
    // started (`lake` starts `lean`, `cargo` starts `rustc`).
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let mut child = command
        .spawn()
        .map_err(|error| format!("cannot start: {error}"))?;
    let mut out = child.stdout.take().ok_or("no stdout")?;
    let mut err = child.stderr.take().ok_or("no stderr")?;
    let stdout = std::thread::spawn(move || {
        let mut text = Vec::new();
        let _ = out.read_to_end(&mut text);
        text
    });
    let stderr = std::thread::spawn(move || {
        let mut text = Vec::new();
        let _ = err.read_to_end(&mut text);
        text
    });
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => {
                if start.elapsed() > limit {
                    stop_tree(&mut child);
                    return Err(format!("ended after {} s", limit.as_secs()));
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(error) => {
                stop_tree(&mut child);
                return Err(format!("cannot wait: {error}"));
            }
        }
    };
    let text = |bytes: Vec<u8>| String::from_utf8_lossy(&bytes).into_owned();
    Ok(Ran {
        status: status.code(),
        stdout: text(stdout.join().unwrap_or_default()),
        stderr: text(stderr.join().unwrap_or_default()),
    })
}

fn last_lines(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    lines[lines.len().saturating_sub(12)..].join("\n")
}

fn copy_tree(from: &Path, to: &Path, pin: &Pin, root: &Path) -> Result<(), String> {
    for entry in walkdir::WalkDir::new(from)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), ".git" | ".lake" | "target")
                    || crate::support::holds_cache_tag(entry)))
        })
    {
        let entry = entry.map_err(|error| error.to_string())?;
        let relative = entry.path().strip_prefix(root).map_err(|e| e.to_string())?;
        let text = relative
            .components()
            .map(|part| part.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/");
        if !entry.file_type().is_file() || !pin.uses(&text) {
            continue;
        }
        let destination = to.join(relative);
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        std::fs::copy(entry.path(), &destination).map_err(|error| error.to_string())?;
    }
    Ok(())
}

/// How a step failed: for a moment, or for good.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Failure {
    /// The network or the host failed in a way a later try may not.
    Transient(String),
    /// Trying again cannot help: the upstream no longer has what the pin names.
    Permanent(String),
}

/// The delays between the tries of a network step: three tries over about
/// a minute.
pub const RETRY_DELAYS: [Duration; 2] = [Duration::from_secs(10), Duration::from_secs(45)];

/// Try `attempt` once, and again after each of `delays` while it fails
/// transiently; a permanent failure ends the tries.
///
/// # Errors
///
/// The failure of the last try.
pub fn retry<T>(
    delays: &[Duration],
    mut attempt: impl FnMut(usize) -> Result<T, Failure>,
) -> Result<T, Failure> {
    let mut tries = 0;
    loop {
        match attempt(tries) {
            Ok(value) => return Ok(value),
            Err(failure @ Failure::Permanent(_)) => return Err(failure),
            Err(failure) => match delays.get(tries) {
                Some(delay) => std::thread::sleep(*delay),
                None => return Err(failure),
            },
        }
        tries += 1;
    }
}

/// Whether a failed `git fetch` says the upstream no longer has the commit.
fn upstream_is_gone(stderr: &str) -> bool {
    let text = stderr.to_ascii_lowercase();
    [
        "repository not found",
        "couldn't find remote ref",
        "not our ref",
        "unadvertised object",
        "does not appear to be a git repository",
        "no such file or directory",
        "not found",
        "invalid refspec",
    ]
    .iter()
    .any(|marker| text.contains(marker))
}

/// Fetch the pinned commit of the oracle, verify that it is the pin, and only
/// then put it at `dir`.
///
/// The fetch happens in a directory beside `dir` that is removed on every
/// failure, so a fetch that fails leaves nothing that a later run could take
/// for a checkout. The fetch is tried again after a transient failure (see
/// [`RETRY_DELAYS`]); a commit that the remote no longer has, and files that do
/// not hash to the pin, end it at once.
fn fetch(pin: &Pin, dir: &Path, delays: &[Duration]) -> Result<(), Unavailable> {
    let fail = |detail: String| Unavailable::NotAcquired(detail);
    let staging = dir.with_extension(format!("fetching-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&staging);
    std::fs::create_dir_all(&staging).map_err(|error| fail(error.to_string()))?;
    let outcome = fetch_into(pin, &staging, delays).and_then(|()| {
        verify_checkout(pin, &staging)?;
        std::fs::rename(&staging, dir).map_err(|error| fail(format!("{}: {error}", dir.display())))
    });
    if outcome.is_err() {
        let _ = std::fs::remove_dir_all(&staging);
    }
    outcome
}

fn fetch_into(pin: &Pin, staging: &Path, delays: &[Duration]) -> Result<(), Unavailable> {
    let git = |arguments: &[&str]| -> Result<Ran, Failure> {
        let mut command = Command::new("git");
        command.args(arguments).current_dir(staging);
        let ran = run(command, Duration::from_secs(pin.timeout_seconds))
            .map_err(|detail| Failure::Transient(format!("git {}: {detail}", arguments[0])))?;
        if ran.status == Some(0) {
            Ok(ran)
        } else {
            let detail = format!(
                "git {} exited {:?}: {}",
                arguments[0],
                ran.status,
                last_lines(&ran.stderr)
            );
            Err(if upstream_is_gone(&ran.stderr) {
                Failure::Permanent(detail)
            } else {
                Failure::Transient(detail)
            })
        }
    };
    let finished = git(&["init", "--quiet"])
        .and_then(|_| {
            retry(delays, |_| {
                git(&[
                    "fetch",
                    "--quiet",
                    "--depth",
                    "1",
                    pin.remote.as_str(),
                    pin.commit.as_str(),
                ])
            })
        })
        .and_then(|_| git(&["checkout", "--quiet", "--detach", "FETCH_HEAD"]));
    match finished {
        Ok(_) => Ok(()),
        Err(Failure::Permanent(detail)) => Err(Unavailable::NotAcquired(format!(
            "{detail}; the upstream no longer has the pinned commit, so the pin has to be moved to a commit or a mirror that does (SPEC.md §17.19)"
        ))),
        Err(Failure::Transient(detail)) => Err(Unavailable::NotAcquired(format!(
            "{detail} (after {} tries)",
            delays.len() + 1
        ))),
    }
}

/// The cached checkout of the pinned oracle in `cache`: one that is there and
/// is the pin is used; one that is there and is not (an interrupted run, a
/// damaged cache) is discarded and fetched again once; none is fetched.
///
/// # Errors
///
/// [`Unavailable::NotAcquired`] when the fetch fails (after the tries `delays`
/// allow) and [`Unavailable::PinMismatch`] when what the remote serves is not
/// the pinned oracle.
pub fn cached_source(pin: &Pin, cache: &Path, delays: &[Duration]) -> Result<PathBuf, Unavailable> {
    let dir = cache.join("source");
    if dir.exists() {
        if verify_checkout(pin, &dir).is_ok() {
            return Ok(dir);
        }
        std::fs::remove_dir_all(&dir).map_err(|error| {
            Unavailable::NotAcquired(format!(
                "{} is not the pinned oracle and cannot be removed: {error}",
                dir.display()
            ))
        })?;
    }
    fetch(pin, &dir, delays)?;
    Ok(dir)
}

/// The channel `rust-toolchain.toml` of `repo` pins.
fn rust_channel(repo: &Path) -> Result<String, String> {
    let text = std::fs::read_to_string(repo.join("rust-toolchain.toml"))
        .map_err(|error| format!("rust-toolchain.toml: {error}"))?;
    let value: toml::Value = text
        .parse()
        .map_err(|error| format!("rust-toolchain.toml: {error}"))?;
    value["toolchain"]["channel"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| "rust-toolchain.toml states no channel".to_owned())
}

/// Refuse a pin whose toolchains are not LexLean's own. The oracle is built
/// with the toolchains the repository pins (`rust-toolchain.toml`,
/// `lean-toolchain`), and the pin states which they are so that a bump of
/// either shows up here, as this error, rather than as a build that fails
/// somewhere else.
///
/// # Errors
///
/// [`Unavailable::ToolchainDrift`] naming both values.
pub fn check_toolchains(pin: &Pin, repo: &Path) -> Result<(), Unavailable> {
    let drift = Unavailable::ToolchainDrift;
    let rust = rust_channel(repo).map_err(drift)?;
    if pin.rust_toolchain != rust {
        return Err(drift(format!(
            "oracle/lean4-prod.toml states rust_toolchain `{}` and rust-toolchain.toml pins `{rust}`",
            pin.rust_toolchain
        )));
    }
    let lean = std::fs::read_to_string(repo.join("lean-toolchain"))
        .map_err(|error| drift(format!("lean-toolchain: {error}")))?;
    if pin.lean_toolchain != lean.trim() {
        return Err(drift(format!(
            "oracle/lean4-prod.toml states lean_toolchain `{}` and lean-toolchain pins `{}`",
            pin.lean_toolchain,
            lean.trim()
        )));
    }
    Ok(())
}

/// Refuse a checkout of the oracle that needs another Lean than the pin
/// states: its own `lean/lean-toolchain` is part of the tree the pin hashes,
/// and it must be LexLean's.
///
/// # Errors
///
/// [`Unavailable::ToolchainDrift`] naming both values.
pub fn check_source_toolchain(pin: &Pin, source: &Path) -> Result<(), Unavailable> {
    let needed = std::fs::read_to_string(source.join("lean/lean-toolchain"))
        .map_err(|error| Unavailable::ToolchainDrift(format!("lean/lean-toolchain: {error}")))?;
    if needed.trim() == pin.lean_toolchain {
        Ok(())
    } else {
        Err(Unavailable::ToolchainDrift(format!(
            "the oracle's lean/lean-toolchain needs `{}` and the pin builds it with `{}`",
            needed.trim(),
            pin.lean_toolchain
        )))
    }
}

/// Refuse a checkout that is not the pinned oracle: the files the pin names
/// must hash to its digest and, in a git checkout, `HEAD` must be its commit.
///
/// # Errors
///
/// [`Unavailable::NotAcquired`] for a missing or unreadable checkout and
/// [`Unavailable::PinMismatch`] for one that is another oracle.
pub fn verify_checkout(pin: &Pin, source: &Path) -> Result<(), Unavailable> {
    if !source.is_dir() {
        return Err(Unavailable::NotAcquired(format!(
            "{} is not a directory",
            source.display()
        )));
    }
    let (digest, files) = tree_digest(source, pin).map_err(Unavailable::NotAcquired)?;
    if digest != pin.tree_sha256 || files != pin.tree_files {
        return Err(Unavailable::PinMismatch(format!(
            "{files} files hash to {digest}; the pin names {} files hashing to {}",
            pin.tree_files, pin.tree_sha256
        )));
    }
    if source.join(".git").exists() {
        let mut command = Command::new("git");
        command.args(["rev-parse", "HEAD"]).current_dir(source);
        let head = run(command, Duration::from_secs(60))
            .ok()
            .filter(|ran| ran.status == Some(0))
            .map(|ran| ran.stdout.trim().to_owned());
        if head.as_deref() != Some(pin.commit.as_str()) {
            return Err(Unavailable::PinMismatch(format!(
                "HEAD is {head:?}; the pin names {}",
                pin.commit
            )));
        }
    }
    Ok(())
}

/// Acquire the pinned oracle and build it, once per process.
///
/// The checkout is `$LEXLEAN_ORACLE_DIR` when set and a fetch of the pinned
/// commit into the cache otherwise. Either is refused unless the files the pin
/// names hash to its `tree_sha256` and, when it is a git checkout, `HEAD` is
/// the pinned commit.
///
/// # Errors
///
/// [`Unavailable`] in every case where the oracle cannot be used: the caller
/// decides what that means for its gate, and none of it is agreement.
pub fn acquire() -> Result<&'static Oracle, Unavailable> {
    static ORACLE: OnceLock<Result<Oracle, Unavailable>> = OnceLock::new();
    ORACLE
        .get_or_init(|| Oracle::acquire_from(&Pin::committed()))
        .as_ref()
        .map_err(Clone::clone)
}

impl Oracle {
    /// Acquire and build the oracle `pin` names.
    ///
    /// # Errors
    ///
    /// As [`acquire`].
    pub fn acquire_from(pin: &Pin) -> Result<Self, Unavailable> {
        Self::acquire_in(
            pin,
            &cache_root(),
            std::env::var_os("LEXLEAN_ORACLE_DIR").map(PathBuf::from),
        )
    }

    /// Acquire and build the oracle `pin` names, caching in `cache`, from
    /// the checkout `source` or, when there is none, from a fetch of the
    /// pinned commit.
    ///
    /// # Errors
    ///
    /// As [`acquire`].
    pub fn acquire_in(
        pin: &Pin,
        cache: &Path,
        source: Option<PathBuf>,
    ) -> Result<Self, Unavailable> {
        std::fs::create_dir_all(cache)
            .map_err(|error| Unavailable::NotAcquired(format!("{}: {error}", cache.display())))?;
        // One process builds; the others wait for the finished build.
        let lock = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(cache.join("lock"))
            .map_err(|error| Unavailable::NotAcquired(error.to_string()))?;
        lock.lock()
            .map_err(|error| Unavailable::NotAcquired(error.to_string()))?;
        check_toolchains(pin, repo_root().as_std_path())?;
        let source = match source {
            Some(dir) => dir,
            None => cached_source(pin, cache, &RETRY_DELAYS)?,
        };
        verify_checkout(pin, &source)?;
        check_source_toolchain(pin, &source)?;
        let build = cache.join(format!("build-{}", &pin.tree_sha256[..16]));
        let exporter = build.join("lean/.lake/build/bin").join(format!(
            "{}{}",
            pin.lean_target,
            std::env::consts::EXE_SUFFIX
        ));
        let prod = build
            .join("target/release")
            .join(format!("prod{}", std::env::consts::EXE_SUFFIX));
        if !(exporter.is_file() && prod.is_file()) {
            let _ = std::fs::remove_dir_all(&build);
            std::fs::create_dir_all(&build)
                .map_err(|error| Unavailable::NotAcquired(error.to_string()))?;
            copy_tree(&source, &build, pin, &source).map_err(Unavailable::NotAcquired)?;
            Self::build(pin, &build)?;
        }
        let scratch = cache.join("scratch");
        std::fs::create_dir_all(&scratch)
            .map_err(|error| Unavailable::NotAcquired(error.to_string()))?;
        Ok(Self {
            pin: pin.clone(),
            source,
            exporter,
            prod,
            scratch,
            bin: crate::preservation::toolchain_bin(),
        })
    }

    /// An oracle made of the given tools, for the tests that give the
    /// harness an oracle that fails: no acquisition, no pin check.
    #[must_use]
    pub fn with_tools(pin: &Pin, exporter: PathBuf, prod: PathBuf, scratch: PathBuf) -> Self {
        Self {
            pin: pin.clone(),
            source: PathBuf::new(),
            exporter,
            prod,
            scratch,
            bin: crate::preservation::toolchain_bin(),
        }
    }

    /// The verified checkout this oracle was built from.
    #[must_use]
    pub fn source(&self) -> &Path {
        &self.source
    }

    fn build(pin: &Pin, build: &Path) -> Result<(), Unavailable> {
        let limit = Duration::from_secs(pin.timeout_seconds * 4);
        let failed = |step: &str, detail: String| Unavailable::BuildFailed {
            step: step.to_owned(),
            detail,
        };
        let mut lake = toolchain_command("lake", &build.join("lean"));
        lake.args(["build", pin.lean_target.as_str()]);
        let ran = run(lake, limit).map_err(|detail| failed("lake build", detail))?;
        if ran.status != Some(0) {
            return Err(failed(
                "lake build",
                format!("{:?}: {}", ran.status, last_lines(&ran.stderr)),
            ));
        }
        // The oracle's own crates.io dependencies are fetched exactly as its
        // Cargo.lock names them (`--locked`; the lock file is part of the tree
        // the pin hashes) into a Cargo home of their own, so that the build
        // reads nothing the host happened to have cached, and then built
        // without the network. Only the fetch can fail for a moment, and it is
        // tried again.
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
        let home = build.parent().map_or_else(
            || build.join("cargo-home"),
            |cache| cache.join("cargo-home"),
        );
        let rust = rust_channel(repo_root().as_std_path()).map_err(Unavailable::ToolchainDrift)?;
        let cargo_command = |arguments: &[&str]| {
            let mut command = Command::new(&cargo);
            command
                .args(arguments)
                .current_dir(build.join("rust"))
                .env("CARGO_HOME", &home)
                .env("CARGO_TARGET_DIR", build.join("target"))
                .env("RUSTUP_TOOLCHAIN", &rust)
                .env("CARGO_INCREMENTAL", "0")
                .env_remove("RUSTFLAGS")
                .env_remove("CARGO_ENCODED_RUSTFLAGS");
            command
        };
        retry(&RETRY_DELAYS, |_| {
            let ran =
                run(cargo_command(&["fetch", "--locked"]), limit).map_err(Failure::Transient)?;
            if ran.status == Some(0) {
                return Ok(());
            }
            let detail = format!("{:?}: {}", ran.status, last_lines(&ran.stderr));
            let text = ran.stderr.to_ascii_lowercase();
            let network = [
                "spurious network error",
                "failed to download",
                "could not resolve",
                "timed out",
                "connection",
                "ssl",
                "failed to query replaced source",
                "unable to update",
                "network",
            ]
            .iter()
            .any(|marker| text.contains(marker));
            Err(if network {
                Failure::Transient(detail)
            } else {
                Failure::Permanent(detail)
            })
        })
        .map_err(|failure| match failure {
            Failure::Transient(detail) | Failure::Permanent(detail) => {
                failed("cargo fetch", detail)
            }
        })?;
        let ran = run(
            cargo_command(&[
                "build",
                "--release",
                "--locked",
                "--offline",
                "-p",
                pin.cargo_package.as_str(),
            ]),
            limit,
        )
        .map_err(|detail| failed("cargo build", detail))?;
        if ran.status == Some(0) {
            Ok(())
        } else {
            Err(failed(
                "cargo build",
                format!("{:?}: {}", ran.status, last_lines(&ran.stderr)),
            ))
        }
    }

    /// The pin this oracle was acquired by.
    #[must_use]
    pub fn pin(&self) -> &Pin {
        &self.pin
    }

    fn limit(&self) -> Duration {
        Duration::from_secs(self.pin.timeout_seconds)
    }

    /// Export the closure of `root` of `module` from the compiled Lean
    /// modules in `library`, and generate its crate named `crate_name`.
    ///
    /// # Errors
    ///
    /// [`Unavailable::ToolFailed`] when a tool fails other than by refusing
    /// the program. A refusal is an answer, [`Verdict::Refused`], and carries
    /// the oracle's own words.
    pub fn verdict(
        &self,
        library: &Path,
        module: &str,
        root: &str,
        crate_name: &str,
    ) -> Result<Verdict, Unavailable> {
        let work = tempfile::Builder::new()
            .prefix("oracle-")
            .tempdir_in(&self.scratch)
            .map_err(|error| Unavailable::ToolFailed {
                step: "scratch".into(),
                detail: error.to_string(),
            })?;
        let tool = |step: &str, detail: String| Unavailable::ToolFailed {
            step: step.to_owned(),
            detail,
        };
        let search =
            std::env::join_paths([library]).map_err(|e| tool("LEAN_PATH", e.to_string()))?;
        let mut export = Command::new(&self.exporter);
        export
            .args([
                "--module",
                module,
                "--root",
                root,
                "--ir-module",
                "Oracle",
                "--out",
            ])
            .arg(work.path().join("export"))
            .env("LEAN_PATH", &search)
            .env("PATH", self.path_with_toolchain());
        let ran = run(export, self.limit()).map_err(|detail| tool("prod-export", detail))?;
        if ran.status != Some(0) {
            let all = format!("{}\n{}", ran.stderr, ran.stdout);
            return match refusal(&all, EXPORT_REFUSAL) {
                Some(message) => Ok(Verdict::Refused {
                    stage: Stage::Export,
                    message,
                }),
                None => Err(tool(
                    "prod-export",
                    format!("exit {:?}: {}", ran.status, last_lines(&all)),
                )),
            };
        }
        let ir_path = work.path().join("export/kernel.ir");
        let ir = std::fs::read_to_string(&ir_path)
            .map_err(|error| tool("prod-export", format!("kernel.ir: {error}")))?;
        if !ir.contains("(def ") {
            return Err(tool(
                "prod-export",
                "the exported module defines nothing".to_owned(),
            ));
        }
        std::fs::write(work.path().join("README.md"), "oracle differential\n")
            .map_err(|error| tool("scratch", error.to_string()))?;
        let licence = work.path().join("LICENSE");
        std::fs::write(&licence, "none\n").map_err(|error| tool("scratch", error.to_string()))?;
        let package_dir = work.path().join("package");
        let mut generate = Command::new(&self.prod);
        generate
            .arg("cargo")
            .arg(&ir_path)
            .arg("--output")
            .arg(&package_dir)
            .args([
                "--name",
                crate_name,
                "--version",
                "0.1.0",
                "--description",
                "oracle",
            ])
            .args(["--repository", "https://example.invalid/oracle"])
            .args(["--homepage", "https://example.invalid/oracle"])
            .arg("--readme")
            .arg(work.path().join("README.md"))
            .arg("--license-mit")
            .arg(&licence)
            .arg("--license-apache")
            .arg(&licence)
            .env("RUSTC_WRAPPER", "");
        let ran = run(generate, self.limit()).map_err(|detail| tool("prod cargo", detail))?;
        if ran.status != Some(0) {
            let all = format!("{}\n{}", ran.stderr, ran.stdout);
            return match refusal(&all, CODEGEN_REFUSAL) {
                Some(message) => Ok(Verdict::Refused {
                    stage: Stage::Codegen,
                    message,
                }),
                None => Err(tool(
                    "prod cargo",
                    format!("exit {:?}: {}", ran.status, last_lines(&all)),
                )),
            };
        }
        let mut package = BTreeMap::new();
        for entry in walkdir::WalkDir::new(&package_dir) {
            let entry = entry.map_err(|error| tool("prod cargo", error.to_string()))?;
            if entry.file_type().is_file() {
                let relative = entry
                    .path()
                    .strip_prefix(&package_dir)
                    .map_err(|error| tool("prod cargo", error.to_string()))?
                    .components()
                    .map(|part| part.as_os_str().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("/");
                let bytes = std::fs::read(entry.path())
                    .map_err(|error| tool("prod cargo", error.to_string()))?;
                package.insert(relative, bytes);
            }
        }
        if !package.contains_key("src/lib.rs") || !package.contains_key("Cargo.toml") {
            return Err(tool(
                "prod cargo",
                "the generated package has no src/lib.rs and Cargo.toml".to_owned(),
            ));
        }
        Ok(Verdict::Accepted { ir, package })
    }

    fn path_with_toolchain(&self) -> std::ffi::OsString {
        std::env::join_paths(
            std::iter::once(self.bin.clone()).chain(std::env::split_paths(
                &std::env::var_os("PATH").unwrap_or_default(),
            )),
        )
        .expect("a search path")
    }
}

/// The oracle's stated reason, when `output` holds its refusal marker.
fn refusal(output: &str, marker: &str) -> Option<String> {
    output.lines().find_map(|line| {
        line.find(marker).map(|at| {
            line[at + marker.len()..]
                .trim()
                .chars()
                .take(400)
                .collect::<String>()
        })
    })
}

/// The kind of an oracle refusal, from its words: the part of the message that
/// does not name a program, so that two programs refused for one reason have
/// one kind.
#[must_use]
pub fn refusal_kind(stage: Stage, message: &str) -> String {
    let kind = match stage {
        Stage::Export => {
            if message.contains("LocalFunctionError.escaping") {
                "escaping-local-function"
            } else if message.contains("opaque lowering") {
                "opaque-lowering"
            } else if message.contains("unresolved external call") {
                "unresolved-external-call"
            } else if message.contains("LocalFunctionError") {
                "unsupported-local-function"
            } else {
                "export-refused"
            }
        }
        Stage::Codegen => {
            let head = message.split([':', '(']).next().unwrap_or_default().trim();
            return format!("codegen-{head}");
        }
    };
    kind.to_owned()
}
