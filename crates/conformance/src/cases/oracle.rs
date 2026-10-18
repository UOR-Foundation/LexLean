//! Conformance cases for the differential validation oracle (SPEC.md §17.19).
//!
//! Each case has a host-independent half, which runs everywhere and decides
//! what the pin, the tables, the comparison, and the isolation audit are,
//! and an oracle-backed half, which needs the pinned toolchain and the
//! acquired oracle and which says that it did not run where it could not.

use std::collections::BTreeSet;
use std::path::Path;

use crate::oracle::{self, Pin, Unavailable};
use crate::oracle_differential as differential;
use crate::support::{self, repo_root};

fn read(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative).as_std_path())
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn write(root: &Path, relative: &str, text: &str) {
    let path = root.join(relative);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("directory");
    std::fs::write(path, text).expect("file");
}

/// A pin that selects the files under `lean/` and `rust/`, for trees the
/// cases build themselves.
fn synthetic_pin() -> Pin {
    let text = read("oracle/lean4-prod.toml");
    Pin::parse(&text).expect("the committed pin")
}

fn pin_tree(dir: &Path) {
    write(dir, "lean/a.txt", "alpha");
    write(dir, "rust/b/c.rs", "beta");
    write(dir, "other/x", "gamma");
}

/// The digest of the tree [`pin_tree`] builds, computed independently of
/// the implementation (a Python transcription of the frame the pin states).
const SYNTHETIC_DIGEST: &str = "f85d0b3ee2fe54cb7fa069126812bd692d665b5c4e27a7dd40437edf7e5f1382";

/// The pin and the tree digest it names (OR-01), host-independent.
fn pin_and_digest() {
    let pin = Pin::committed();
    assert_eq!(
        pin.role,
        oracle::ROLE,
        "the oracle's role is validation evidence"
    );
    assert_eq!(pin.name, "lean4-prod");
    assert_eq!(pin.remote, "https://github.com/auser/lean4-prod");
    assert!(!pin.licence.vendored && !pin.licence.text_shipped);
    assert_eq!(pin.used, ["lean/", "rust/", "rust-toolchain.toml"]);
    // Pin validation: each of these edits is a pin that is refused.
    let text = read("oracle/lean4-prod.toml");
    let refused = |from: &str, to: &str| -> String {
        let edited = text.replacen(from, to, 1);
        assert_ne!(edited, text, "the edit `{from}` applied");
        Pin::parse(&edited).expect_err(&format!("`{from}` -> `{to}` is refused"))
    };
    assert!(
        refused("role = \"validation-evidence\"", "role = \"authority\"")
            .contains("validation-evidence")
    );
    assert!(
        refused("role = \"validation-evidence\"", "role = \"proof\"")
            .contains("validation-evidence")
    );
    assert!(!refused(&format!("commit = \"{}\"\n", pin.commit), "").is_empty());
    assert!(!refused("name = \"lean4-prod\"\n", "").is_empty());
    assert!(!refused("timeout_seconds = 300", "timeout_seconds = 300\nextra = 1").is_empty());
    assert!(refused("vendored = false", "vendored = true").contains("vendored"));
    assert!(!refused(&pin.commit, &pin.commit[..39]).is_empty());
    assert!(!refused(&pin.tree_sha256, &pin.tree_sha256.to_uppercase()).is_empty());
    assert!(!refused(
        "used = [\"lean/\", \"rust/\", \"rust-toolchain.toml\"]",
        "used = []"
    )
    .is_empty());
    assert!(!refused(
        "used = [\"lean/\", \"rust/\", \"rust-toolchain.toml\"]",
        "used = [\"/etc/\"]"
    )
    .is_empty());
    // The digest: the frame is the one the pin states.
    let dir = tempfile::tempdir().expect("tempdir");
    pin_tree(dir.path());
    let (digest, files) = oracle::tree_digest(dir.path(), &synthetic_pin()).expect("digest");
    assert_eq!((digest.as_str(), files), (SYNTHETIC_DIGEST, 2));
    // A file outside the used prefixes, a `.git` or `.lake` or `target`
    // directory, and an empty directory are not part of it ...
    write(dir.path(), "docs/readme", "ignored");
    write(dir.path(), "lean/.lake/build/x", "ignored");
    write(dir.path(), "rust/target/debug/y", "ignored");
    write(dir.path(), ".git/HEAD", "ignored");
    std::fs::create_dir_all(dir.path().join("lean/empty")).expect("dir");
    assert_eq!(
        oracle::tree_digest(dir.path(), &synthetic_pin())
            .expect("digest")
            .0,
        SYNTHETIC_DIGEST
    );
    // ... and a byte changed, a file added, a file removed, and a file
    // renamed each change it.
    let changed = |edit: &dyn Fn(&Path)| -> String {
        let copy = tempfile::tempdir().expect("tempdir");
        pin_tree(copy.path());
        edit(copy.path());
        oracle::tree_digest(copy.path(), &synthetic_pin())
            .expect("digest")
            .0
    };
    let mut seen = BTreeSet::from([SYNTHETIC_DIGEST.to_owned()]);
    for (name, digest) in [
        (
            "a byte changed",
            changed(&|d| write(d, "lean/a.txt", "alphb")),
        ),
        ("a file added", changed(&|d| write(d, "lean/new.lean", ""))),
        (
            "a file removed",
            changed(&|d| std::fs::remove_file(d.join("rust/b/c.rs")).expect("rm")),
        ),
        (
            "a file renamed",
            changed(&|d| {
                std::fs::rename(d.join("lean/a.txt"), d.join("lean/z.txt")).expect("mv");
            }),
        ),
    ] {
        assert!(seen.insert(digest), "{name} changes the digest");
    }
    // The oracle is no authority and no ledger claim.
    let identifiers = repo_model::isolation::identifiers(repo_root().as_std_path()).expect("pin");
    for file in ["model/authorities.toml", "model/ledger.toml"] {
        assert_eq!(
            repo_model::isolation::mention(&read(file), &identifiers),
            None,
            "{file} must not cite the oracle: it is validation evidence, not an authority (§27.4)"
        );
    }
}

// ---- OR-07: the oracle's silence is never agreement --------------------------

/// A fake tool: a POSIX shell script with the given body.
#[cfg(unix)]
fn fake_tool(dir: &Path, name: &str, body: &str) -> std::path::PathBuf {
    use std::os::unix::fs::PermissionsExt as _;
    let path = dir.join(name);
    std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("script");
    let mut mode = std::fs::metadata(&path).expect("metadata").permissions();
    mode.set_mode(0o755);
    std::fs::set_permissions(&path, mode).expect("chmod");
    path
}

/// The exporter that writes an IR defining `body` to its `--out`.
#[cfg(unix)]
const WRITES_IR: &str = "out=''\nwhile [ $# -gt 0 ]; do if [ \"$1\" = '--out' ]; then out=\"$2\"; fi; shift; done\nmkdir -p \"$out\"\nprintf '(module X\\n  (def f () Nat 1)\\n)\\n' > \"$out/kernel.ir\"";

/// What each failure of the oracle's tools is reported as.
#[cfg(unix)]
fn failures_are_unavailable() {
    use oracle::{Oracle, Stage, Verdict};
    let dir = tempfile::tempdir().expect("tempdir");
    let scratch = dir.path().join("scratch");
    std::fs::create_dir_all(&scratch).expect("scratch");
    let library = dir.path().join("lib");
    std::fs::create_dir_all(&library).expect("lib");
    // The time every tool is allowed. It is generous, because a loaded host
    // runs these among hundreds of tests and a reference that ran out of
    // time would be a flake, not a verdict; only the tool that is meant to
    // overrun is given a tiny one (`ask_within`).
    const GENEROUS: u64 = 120;
    let exporter_ok = fake_tool(dir.path(), "export-ok", WRITES_IR);
    let prod_ok_body = "out=''\nwhile [ $# -gt 0 ]; do if [ \"$1\" = '--output' ]; then out=\"$2\"; fi; shift; done\nmkdir -p \"$out/src\"\nprintf '[package]\\nname = \"x\"\\n' > \"$out/Cargo.toml\"\nprintf 'pub fn f() -> u64 { 1 }\\n' > \"$out/src/lib.rs\"";
    let prod_ok = fake_tool(dir.path(), "prod-ok", prod_ok_body);
    let ask_within = |seconds: u64, exporter: &Path, prod: &Path| {
        let mut pin = Pin::committed();
        pin.timeout_seconds = seconds;
        Oracle::with_tools(
            &pin,
            exporter.to_path_buf(),
            prod.to_path_buf(),
            scratch.clone(),
        )
        .verdict(&library, "M", "M.f", "crate-x")
    };
    let ask = |exporter: &Path, prod: &Path| ask_within(GENEROUS, exporter, prod);
    // The reference: working tools are an acceptance.
    match ask(&exporter_ok, &prod_ok) {
        Ok(Verdict::Accepted { package, .. }) => assert!(package.contains_key("src/lib.rs")),
        other => panic!("working tools accept: {other:?}"),
    }
    // The oracle's own refusals are answers ...
    let refuses_export = fake_tool(
        dir.path(),
        "export-refuses",
        "echo 'uncaught exception: prod-export failed: unsupported local function in M.f: escaping' >&2\nexit 1",
    );
    match ask(&refuses_export, &prod_ok) {
        Ok(Verdict::Refused {
            stage: Stage::Export,
            message,
        }) => {
            assert!(
                message.starts_with("unsupported local function"),
                "{message}"
            );
        }
        other => panic!("a structured export refusal is a refusal: {other:?}"),
    }
    let refuses_gen = fake_tool(
        dir.path(),
        "prod-refuses",
        "echo 'Code generation refused: UnresolvedCall(\"Nat.succ\"): no' >&2\nexit 1",
    );
    match ask(&exporter_ok, &refuses_gen) {
        Ok(Verdict::Refused {
            stage: Stage::Codegen,
            message,
        }) => {
            assert!(message.starts_with("UnresolvedCall"), "{message}");
        }
        other => panic!("a structured code generation refusal is a refusal: {other:?}"),
    }
    // ... and everything else is not: a crash, a signal, a missing output,
    // an output defining nothing, a timeout, and a failure of the generator
    // that does not state a refusal.
    let crashes = fake_tool(
        dir.path(),
        "crashes",
        "echo \"thread 'main' panicked at x\" >&2\nexit 101",
    );
    let killed = fake_tool(dir.path(), "killed", "kill -9 $$");
    let silent = fake_tool(dir.path(), "silent", "exit 0");
    let empty = fake_tool(
        dir.path(),
        "empty",
        "out=''\nwhile [ $# -gt 0 ]; do if [ \"$1\" = '--out' ]; then out=\"$2\"; fi; shift; done\nmkdir -p \"$out\"\nprintf '(module X\\n)\\n' > \"$out/kernel.ir\"",
    );
    let slow = fake_tool(dir.path(), "slow", "sleep 30");
    let missing = dir.path().join("does-not-exist");
    // A tool that overruns is ended with everything it started, not left
    // running beside the suite: its child is dead when the verdict returns.
    #[cfg(target_os = "linux")]
    {
        let pidfile = dir.path().join("grandchild.pid");
        let spawns = fake_tool(
            dir.path(),
            "spawns",
            &format!("sleep 120 &\necho $! > {}\nwait", pidfile.display()),
        );
        let started = std::time::Instant::now();
        assert!(
            matches!(
                ask_within(1, &spawns, &prod_ok),
                Err(Unavailable::ToolFailed { .. })
            ),
            "an exporter that starts a child and overruns is unavailable"
        );
        assert!(
            started.elapsed() < std::time::Duration::from_secs(30),
            "the overrun is ended at its bound, not at its child's"
        );
        let pid = std::fs::read_to_string(&pidfile)
            .expect("the tool wrote its child's pid")
            .trim()
            .to_owned();
        // A killed child is a zombie until its parent's parent reaps it, which
        // a container's first process may not do: a zombie is dead.
        let alive = || {
            std::fs::read_to_string(format!("/proc/{pid}/stat")).is_ok_and(|stat| {
                !stat
                    .rsplit(')')
                    .next()
                    .unwrap_or("")
                    .trim_start()
                    .starts_with('Z')
            })
        };
        for _ in 0..100 {
            if !alive() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        assert!(
            !alive(),
            "the child {pid} of the overrunning tool is ended with it"
        );
    }
    for (name, result) in [
        ("an exporter that crashes", ask(&crashes, &prod_ok)),
        ("an exporter killed by a signal", ask(&killed, &prod_ok)),
        ("an exporter that writes nothing", ask(&silent, &prod_ok)),
        (
            "an exporter whose IR defines nothing",
            ask(&empty, &prod_ok),
        ),
        (
            "an exporter that overruns its time",
            ask_within(1, &slow, &prod_ok),
        ),
        ("an exporter that is not there", ask(&missing, &prod_ok)),
        ("a generator that crashes", ask(&exporter_ok, &crashes)),
        (
            "a generator that writes no crate",
            ask(&exporter_ok, &silent),
        ),
        ("a generator that is not there", ask(&exporter_ok, &missing)),
    ] {
        match result {
            Err(Unavailable::ToolFailed { .. }) => {}
            other => panic!("{name} must be unavailable, not an answer: {other:?}"),
        }
    }
}

#[cfg(not(unix))]
fn failures_are_unavailable() {
    support::unix_only("OR-07", "POSIX shell for the oracle's failing stand-ins");
}

/// Acquisition fails closed: a checkout that is missing, that is another
/// oracle, or that does not build is unavailable.
fn acquisition_fails_closed() {
    let pin = Pin::committed();
    let cache = tempfile::tempdir().expect("tempdir");
    // A source that is not there.
    match oracle::Oracle::acquire_in(&pin, cache.path(), Some(cache.path().join("nowhere"))) {
        Err(Unavailable::NotAcquired(_)) => {}
        other => panic!(
            "a missing checkout is not acquired: {:?}",
            other.map(|_| ())
        ),
    }
    // A source that holds nothing of the oracle.
    let empty = tempfile::tempdir().expect("tempdir");
    match oracle::verify_checkout(&pin, empty.path()) {
        Err(Unavailable::PinMismatch(_)) => {}
        other => panic!("an empty checkout is not the pin: {other:?}"),
    }
}

/// A run with nothing to say says nothing: validated only when cases were
/// compared against an oracle that generated Rust.
fn vacuous_runs_are_unavailable() {
    let table = differential::KnownDifferences::committed();
    let empty = differential::Run {
        roots: Vec::new(),
        executed: std::collections::BTreeMap::new(),
        pin: Pin::committed(),
        timings: std::collections::BTreeMap::new(),
    };
    match differential::validate(&empty, &table) {
        differential::Validation::Unavailable(Unavailable::Vacuous(_)) => {}
        other => panic!(
            "a run over no root is unavailable, not validated: {}",
            other.report()
        ),
    }
    assert!(!differential::validate(&empty, &table).validated());
}

/// What a gate does with an oracle that did not run.
fn gates_decide() {
    let unavailable: Result<differential::Run, Unavailable> =
        Err(Unavailable::NotAcquired("planted".into()));
    match differential::decide(true, &unavailable) {
        differential::Gate::Fail(reason) => assert!(reason.contains("not a validation")),
        other => panic!("the normative host fails on an unavailable oracle: {other:?}"),
    }
    match differential::decide(false, &unavailable) {
        differential::Gate::NotValidated(reason) => assert!(reason.contains("planted")),
        other => panic!("another host reports not validated: {other:?}"),
    }
}

// ---- toolchains --------------------------------------------------------------

/// The pin's toolchains are the repository's own, and a pin or a checkout that
/// names others is refused with a message that names both.
fn toolchains_are_the_repositorys() {
    let pin = Pin::committed();
    oracle::check_toolchains(&pin, repo_root().as_std_path())
        .expect("the committed pin names the repository's toolchains");
    let repo = tempfile::tempdir().expect("tempdir");
    let state = |channel: &str, lean: &str| {
        write(
            repo.path(),
            "rust-toolchain.toml",
            &format!("[toolchain]\nchannel = \"{channel}\"\n"),
        );
        write(repo.path(), "lean-toolchain", &format!("{lean}\n"));
        oracle::check_toolchains(&pin, repo.path())
    };
    state(&pin.rust_toolchain, &pin.lean_toolchain).expect("agreeing toolchains");
    for (name, result, needles) in [
        (
            "another Rust",
            state("9.99.9", &pin.lean_toolchain),
            vec!["rust_toolchain", "9.99.9", pin.rust_toolchain.as_str()],
        ),
        (
            "another Lean",
            state(&pin.rust_toolchain, "leanprover/lean4:v0.0.1"),
            vec!["lean_toolchain", "v0.0.1", pin.lean_toolchain.as_str()],
        ),
    ] {
        match result {
            Err(Unavailable::ToolchainDrift(detail)) => {
                for needle in needles {
                    assert!(detail.contains(needle), "{name}: {detail}");
                }
            }
            other => panic!("{name} is a drift: {other:?}"),
        }
    }
    std::fs::remove_file(repo.path().join("lean-toolchain")).expect("rm");
    assert!(matches!(
        oracle::check_toolchains(&pin, repo.path()),
        Err(Unavailable::ToolchainDrift(_))
    ));
    // The oracle's own Lean, which the pin hashes, is LexLean's.
    let source = tempfile::tempdir().expect("tempdir");
    write(
        source.path(),
        "lean/lean-toolchain",
        &format!("{}\n", pin.lean_toolchain),
    );
    oracle::check_source_toolchain(&pin, source.path()).expect("the oracle needs the pinned Lean");
    write(
        source.path(),
        "lean/lean-toolchain",
        "leanprover/lean4:v0.0.1\n",
    );
    match oracle::check_source_toolchain(&pin, source.path()) {
        Err(Unavailable::ToolchainDrift(detail)) => {
            assert!(detail.contains("v0.0.1") && detail.contains(&pin.lean_toolchain));
        }
        other => panic!("an oracle that needs another Lean is a drift: {other:?}"),
    }
}

// ---- acquisition ---------------------------------------------------------------

/// A local git repository holding [`pin_tree`], and the pin that names its
/// commit and the digest of its files.
fn origin() -> (tempfile::TempDir, Pin) {
    let dir = tempfile::tempdir().expect("tempdir");
    pin_tree(dir.path());
    let git = |arguments: &[&str]| -> String {
        let output = std::process::Command::new("git")
            .args([
                "-c",
                "user.name=lexlean",
                "-c",
                "user.email=lexlean@example.invalid",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(arguments)
            .current_dir(dir.path())
            .output()
            .expect("git runs");
        assert!(output.status.success(), "git {arguments:?}");
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    };
    git(&["init", "--quiet"]);
    git(&["add", "-A"]);
    git(&["commit", "--quiet", "-m", "origin"]);
    let mut pin = synthetic_pin();
    pin.commit = git(&["rev-parse", "HEAD"]);
    pin.remote = dir.path().display().to_string();
    pin.timeout_seconds = 120;
    let (digest, files) = oracle::tree_digest(dir.path(), &pin).expect("digest");
    pin.tree_sha256 = digest;
    pin.tree_files = files;
    (dir, pin)
}

/// What a cache directory holds, by name.
fn entries(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .expect("cache")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    names
}

/// A fetch is atomic and a bad cache heals: a fetch that fails leaves nothing
/// a later run could take for a checkout, a cached checkout that is not the
/// pin is discarded and fetched again, and files that are not the pin are
/// refused whatever the remote says.
fn acquisition_is_atomic_and_heals() {
    use std::time::Duration;
    let quick = [Duration::from_millis(5), Duration::from_millis(5)];
    let (origin, pin) = origin();
    // The reference: a fetch of the pin puts the pinned files in place and
    // nothing else beside them.
    let cache = tempfile::tempdir().expect("tempdir");
    let source = oracle::cached_source(&pin, cache.path(), &quick).expect("a fetch of the pin");
    assert_eq!(source, cache.path().join("source"));
    oracle::verify_checkout(&pin, &source).expect("the fetched files are the pin");
    assert_eq!(entries(cache.path()), ["source"]);
    // A cache that is the pin is used as it is: no fetch (the remote is gone).
    let mut gone = pin.clone();
    gone.remote = cache.path().join("no-remote").display().to_string();
    assert_eq!(
        oracle::cached_source(&gone, cache.path(), &quick).expect("the cache is the pin"),
        source
    );
    // A fetch that fails leaves nothing behind. A remote that does not exist is
    // permanent and ends the tries at once ...
    let failed = tempfile::tempdir().expect("tempdir");
    match oracle::cached_source(&gone, failed.path(), &quick) {
        Err(Unavailable::NotAcquired(detail)) => {
            assert!(
                detail.contains("no longer has the pinned commit") && detail.contains("§17.19"),
                "the repin procedure is named: {detail}"
            );
        }
        other => panic!("a missing remote is not acquired: {other:?}"),
    }
    assert!(
        entries(failed.path()).is_empty(),
        "{:?}",
        entries(failed.path())
    );
    // ... and one that cannot be reached is tried again, and still leaves
    // nothing for a later run to take for a checkout.
    let mut unreachable = pin.clone();
    unreachable.remote = "git://127.0.0.1:1/lean4-prod.git".to_owned();
    match oracle::cached_source(&unreachable, failed.path(), &quick) {
        Err(Unavailable::NotAcquired(detail)) => {
            assert!(detail.contains("after 3 tries"), "{detail}");
        }
        other => panic!("an unreachable remote is not acquired: {other:?}"),
    }
    assert!(
        entries(failed.path()).is_empty(),
        "{:?}",
        entries(failed.path())
    );
    // A cache that is not the pin (an interrupted run's empty repository, a
    // changed file) is discarded and fetched once more.
    let damaged = tempfile::tempdir().expect("tempdir");
    write(damaged.path(), "source/.git/HEAD", "ref: refs/heads/none\n");
    write(damaged.path(), "source/lean/a.txt", "poisoned");
    let healed = oracle::cached_source(&pin, damaged.path(), &quick).expect("a cache that heals");
    oracle::verify_checkout(&pin, &healed).expect("the healed cache is the pin");
    assert_eq!(
        std::fs::read_to_string(healed.join("lean/a.txt")).expect("file"),
        "alpha"
    );
    assert_eq!(entries(damaged.path()), ["source"]);
    // A remote that serves files that are not the pin is a mismatch, once,
    // and leaves nothing.
    let mut wrong = pin.clone();
    wrong.tree_sha256 = "0".repeat(64);
    let refused = tempfile::tempdir().expect("tempdir");
    match oracle::cached_source(&wrong, refused.path(), &quick) {
        Err(Unavailable::PinMismatch(_)) => {}
        other => panic!("files that are not the pin are a mismatch: {other:?}"),
    }
    assert!(entries(refused.path()).is_empty());
    drop(origin);
}

/// The retry: transient failures are tried again, up to the delays, and a
/// permanent one ends the tries.
fn retries_are_bounded() {
    use oracle::Failure;
    use std::time::{Duration, Instant};
    let quick = [Duration::from_millis(20), Duration::from_millis(20)];
    let mut tries = 0;
    let got = oracle::retry(&quick, |at| {
        tries += 1;
        if at < 2 {
            Err(Failure::Transient(format!("blip {at}")))
        } else {
            Ok("done")
        }
    });
    assert_eq!((got, tries), (Ok("done"), 3), "two blips are survived");
    let mut tries = 0;
    let started = Instant::now();
    let got: Result<(), Failure> = oracle::retry(&quick, |_| {
        tries += 1;
        Err(Failure::Transient(format!("down {tries}")))
    });
    assert_eq!(got, Err(Failure::Transient("down 3".into())));
    assert_eq!(tries, 3, "the delays bound the tries");
    assert!(
        started.elapsed() >= Duration::from_millis(40),
        "the delays are waited"
    );
    let mut tries = 0;
    let got: Result<(), Failure> = oracle::retry(&quick, |_| {
        tries += 1;
        Err(Failure::Permanent("gone".into()))
    });
    assert_eq!((got, tries), (Err(Failure::Permanent("gone".into())), 1));
    // The real delays are about a minute in all.
    let total: u64 = oracle::RETRY_DELAYS.iter().map(Duration::as_secs).sum();
    assert!((30..=120).contains(&total), "{total} s");
}

// ---- OR-08: evidence only ---------------------------------------------------

/// A tree with one small file under each root the isolation audit reads, and
/// the pin.
fn isolation_tree() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    write(
        dir.path(),
        "oracle/lean4-prod.toml",
        &read("oracle/lean4-prod.toml"),
    );
    write(
        dir.path(),
        "crates/lexlean/src/lib.rs",
        "pub fn compile() {}\n",
    );
    write(
        dir.path(),
        "language/semantics-1.2.toml",
        "version = \"1.2\"\n",
    );
    write(dir.path(), "schemas/attestation.schema.json", "{}\n");
    write(
        dir.path(),
        "compiler/lexlean.lock",
        "spec = \"lexlean/lock/1\"\n",
    );
    write(
        dir.path(),
        "examples/demo/expected/build/manifest.json",
        "{}\n",
    );
    write(
        dir.path(),
        "model/authorities.toml",
        "spec = \"lexlean/authorities/1\"\n",
    );
    write(
        dir.path(),
        "model/ledger.toml",
        "spec = \"lexlean/ledger/1\"\n",
    );
    dir
}

fn isolation_audit() {
    // The real tree: nothing the identities are computed from names the
    // oracle, and the audit read every root it is meant to.
    let scanned =
        repo_model::isolation::scan(repo_root().as_std_path()).expect("the tree is isolated");
    assert!(scanned.files > 200, "{scanned:?}");
    assert_eq!(scanned.per_root.len(), repo_model::isolation::SCANNED.len());
    assert!(
        scanned.per_root.iter().all(|(_, count)| *count > 0),
        "{scanned:?}"
    );
    // The compiler-semantics ID is the digest of exactly the embedded files,
    // and none of them names the oracle.
    let identifiers = repo_model::isolation::identifiers(repo_root().as_std_path()).expect("pin");
    assert!(lexlean::embedded::FILES.len() > 100);
    for (path, bytes) in lexlean::embedded::FILES {
        let text = String::from_utf8_lossy(bytes);
        assert_eq!(
            repo_model::isolation::mention(&text, &identifiers),
            None,
            "{path} is an input of the compiler-semantics ID and names the oracle"
        );
    }
    // A synthetic tree is clean, and the audit fails on each way of planting
    // the oracle in each place an identity or a record is read from.
    let pin = Pin::committed();
    let plants = [
        ("its name", "lean4-prod".to_owned()),
        ("its Lake target", "prod-export".to_owned()),
        ("its pinned commit", pin.commit.clone()),
        ("the digest of its files", pin.tree_sha256.clone()),
        ("its git tree", pin.git_tree.clone()),
        ("its pin's path", "oracle/lean4-prod.toml".to_owned()),
        ("its pin's schema", "lexlean/oracle-pin/1".to_owned()),
    ];
    let places = [
        "crates/lexlean/src/lib.rs",
        "language/semantics-1.2.toml",
        "schemas/attestation.schema.json",
        "compiler/lexlean.lock",
        "examples/demo/expected/build/manifest.json",
        "model/authorities.toml",
        "model/ledger.toml",
    ];
    let clean = isolation_tree();
    repo_model::isolation::scan(clean.path()).expect("a clean tree passes");
    for place in places {
        for (what, plant) in &plants {
            let tree = isolation_tree();
            let path = tree.path().join(place);
            let mut text = std::fs::read_to_string(&path).expect("file");
            text.push_str(&format!("\n{plant}\n"));
            std::fs::write(&path, text).expect("plant");
            let error = repo_model::isolation::scan(tree.path())
                .expect_err(&format!("planting {what} in {place} must fail the audit"));
            // The audit joins path segments with the host separator, so on
            // Windows the message reads `crates/lexlean\src\lib.rs`.
            assert!(error.replace('\\', "/").contains(place), "{error}");
        }
    }
    // An audit that read nothing has not passed.
    let hollow = isolation_tree();
    std::fs::remove_file(hollow.path().join("model/ledger.toml")).expect("rm");
    assert!(repo_model::isolation::scan(hollow.path())
        .expect_err("a scanned root with no file")
        .contains("holds no file"));
}

/// A planted `oracle` field in each record an identity or an attestation is
/// read back from fails the record's schema, which is closed.
fn records_are_closed() {
    use serde_json::{json, Value};
    let load = |name: &str| -> Value {
        serde_json::from_str(&read(&format!("schemas/{name}"))).expect("schema")
    };
    let plant = |instance: &Value| -> Value {
        let mut planted = instance.clone();
        planted["oracle"] = json!({"commit": Pin::committed().commit, "verdict": "validated"});
        planted
    };
    let found = |schema: &Value, instance: &Value| -> bool {
        crate::schema::validate(schema, instance)
            .iter()
            .any(|violation| violation.to_string().starts_with("/oracle"))
    };
    // Committed records: valid as they are, refused with the field.
    let manifest: Value =
        serde_json::from_str(&read("examples/production/expected/build/manifest.json"))
            .expect("manifest");
    let preservation: Value = serde_json::from_str(&read(
        "examples/production/expected/verify/preserve/preservation.json",
    ))
    .expect("preservation");
    let fixture: Value =
        serde_json::from_str(&read("compiler/certificates/bridge-attained.json")).expect("fixture");
    let certificate = fixture["certificate"].clone();
    for (schema_name, instance) in [
        ("build-manifest-v2.schema.json", &manifest),
        ("preservation.schema.json", &preservation),
        ("gnaf-certificate.schema.json", &certificate),
    ] {
        let schema = load(schema_name);
        assert!(
            crate::schema::validate(&schema, instance).is_empty(),
            "{schema_name}: the committed record is valid"
        );
        assert!(
            found(&schema, &plant(instance)),
            "{schema_name}: an oracle field in the record is refused"
        );
    }
    // Records generated only by a verification run: their schemas admit no
    // oracle field either.
    for schema_name in [
        "attestation-v2.schema.json",
        "attestation.schema.json",
        "semantic-snapshot-v2.schema.json",
        "compiler-input.schema.json",
    ] {
        let schema = load(schema_name);
        assert_eq!(
            schema["additionalProperties"],
            json!(false),
            "{schema_name} is closed"
        );
        assert!(
            found(&schema, &json!({"oracle": {"verdict": "validated"}})),
            "{schema_name}: an oracle field is refused"
        );
    }
}

/// Nothing outside the oracle's own modules names the oracle's module in the
/// conformance crate: the other cases are what they are with the oracle gone.
fn oracle_is_only_its_own() {
    let mut users = Vec::new();
    for entry in std::fs::read_dir(repo_root().join("crates/conformance/src").as_std_path())
        .expect("sources")
        .flatten()
    {
        let path = entry.path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let text = std::fs::read_to_string(&path).expect("source");
        if text.contains("crate::oracle") && !name.starts_with("oracle") {
            users.push(name);
        }
    }
    assert!(
        users.is_empty(),
        "only the oracle's modules and its cases use the oracle: {users:?}"
    );
}

// ---- shared material ----------------------------------------------------------

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde_json::{json, Value as Json};

use crate::oracle_corpus as corpus;

/// The constructs of every eligible root of the example and probe projects,
/// as LexLean's eligibility states them: no Lean, no oracle.
fn lexlean_constructs() -> &'static BTreeMap<String, BTreeSet<String>> {
    static CONSTRUCTS: OnceLock<BTreeMap<String, BTreeSet<String>>> = OnceLock::new();
    CONSTRUCTS.get_or_init(|| {
        let mut out = BTreeMap::new();
        for name in corpus::EXAMPLES {
            out.extend(differential::lexlean_constructs(&support::P::copy_example(
                name,
            )));
        }
        out.extend(differential::lexlean_constructs(&corpus::probe_project(
            &corpus::probes(),
        )));
        out
    })
}

/// A root made of a committed fixture with one case, and the oracle's
/// outcome and allocation count for it.
fn synthetic(outcome: &Json, allocations: u64) -> (differential::RootRun, differential::Executed) {
    let root = differential::fixture_root("binding-and-shapes", "");
    let executed = differential::Executed {
        oracle: vec![(outcome.clone(), allocations)],
        lexlean: None,
    };
    (root, executed)
}

fn expected_outcome(root: &differential::RootRun) -> Json {
    let differential::LexLeanSide::Eligible(accepted) = &root.lexlean else {
        panic!("an eligible root");
    };
    lexlean::calculus::rust::observable(&accepted.cases[0].outcome).expect("observable")
}

fn diffs_of(
    root: &differential::RootRun,
    executed: &differential::Executed,
) -> Vec<differential::Diff> {
    differential::differences(root, Some(&Ok(executed.clone())))
}

// ---- OR-02: the corpus and the overlap ---------------------------------------

fn malformed_non_overlap_tables_are_refused() {
    let text = read("oracle/known-non-overlap.toml");
    let refused = |from: &str, to: &str| {
        let edited = text.replacen(from, to, 1);
        assert_ne!(edited, text, "the edit `{from}` applied");
        differential::NonOverlapTable::parse(&edited)
            .map(|_| ())
            .expect_err(&format!("`{from}` -> `{to}` is refused"));
    };
    refused(
        "spec = \"lexlean/oracle-overlap/1\"",
        "spec = \"lexlean/oracle-overlap/2\"",
    );
    let mut value: toml::Value = text.parse().expect("the table is TOML");
    let version = value["version"].as_integer().expect("version");
    refused(&format!("version = {version}"), "version = 0");
    refused("kind = \"not-runtime\"", "kind = \"vendored\"");
    // A row with a blank reason.
    value["construct"][0]["reason"] = toml::Value::String("  ".to_owned());
    differential::NonOverlapTable::parse(&value.to_string())
        .expect_err("a blank reason is refused");
    refused(
        "[[construct]]\nkey = \"type.type\"",
        "[[construct]]\nunknown = 1\nkey = \"type.type\"",
    );
    refused("disposition = \"formal-only\"\n", "");
    // A construct twice.
    let edited = format!("{text}\n[[construct]]\nkey = \"type.type\"\nkind = \"not-runtime\"\ndisposition = \"formal-only\"\nreason = \"again\"\n");
    differential::NonOverlapTable::parse(&edited).expect_err("a repeated construct is refused");
    // A refusal row without its probe.
    let edited = format!(
        "{text}\n[[construct]]\nkey = \"term.add\"\nkind = \"oracle-refuses\"\nreason = \"x\"\n"
    );
    differential::NonOverlapTable::parse(&edited).expect_err("a refusal row needs its probe");
}

/// LexLean's side of the corpus: every runtime construct is realized by a
/// corpus root or is a row of the table, the table's rows are what the
/// registry and the corpus say they are, and every probe realizes what it is
/// for. Nothing here needs Lean or the oracle.
fn lexlean_side_of_the_corpus() {
    let registry = differential::registry();
    let table = differential::NonOverlapTable::committed();
    let constructs = lexlean_constructs();
    let realized: BTreeSet<&String> = constructs.values().flatten().collect();
    // The not-runtime rows are exactly the registry's constructs no value
    // realizes.
    let not_runtime: BTreeSet<&String> = registry
        .iter()
        .filter(|(_, disposition)| *disposition != "runtime")
        .map(|(key, _)| key)
        .collect();
    let listed: BTreeSet<&String> = table
        .construct
        .iter()
        .filter(|row| row.kind == "not-runtime")
        .map(|row| &row.key)
        .collect();
    assert_eq!(
        listed, not_runtime,
        "the formal-only and erased constructs are the not-runtime rows"
    );
    for row in &table.construct {
        assert!(
            registry.contains_key(&row.key),
            "`{}` is a construct of the registry",
            row.key
        );
        if row.kind == "not-runtime" {
            continue;
        }
        let probe = row.probe.as_deref().expect("a probe");
        let held = constructs.get(probe).unwrap_or_else(|| {
            panic!(
                "`{}`: the probe `{probe}` is an eligible corpus root",
                row.key
            )
        });
        if row.kind == "instantiated" {
            assert!(
                !realized.contains(&row.key),
                "`{}` is realized by no closure",
                row.key
            );
        } else {
            assert!(
                held.contains(&row.key),
                "`{}`: `{probe}` realizes it",
                row.key
            );
        }
        let in_probe_project = probe.starts_with("OracleProbe.");
        match row.scope.as_deref() {
            Some("construct") => {
                assert!(
                    in_probe_project,
                    "`{}`: a construct-scope probe is a probe root",
                    row.key
                );
                let found = corpus::probes().into_iter().any(|candidate| {
                    format!("OracleProbe.{}.{}", candidate.module, candidate.root) == probe
                        && (row.kind == "instantiated"
                            || candidate.keys.contains(&row.key.as_str()))
                });
                assert!(found, "`{}`: the probe `{probe}` is built for it", row.key);
            }
            Some("closure") => assert!(
                !in_probe_project,
                "`{}`: a closure-scope probe is a root of a certified example",
                row.key
            ),
            other => panic!("`{}`: scope {other:?}", row.key),
        }
    }
    // Every runtime construct is realized by some corpus root, or is the one
    // construct no closure holds.
    for (key, disposition) in &registry {
        if disposition == "runtime" {
            let instantiated = table
                .construct
                .iter()
                .any(|row| &row.key == key && row.kind == "instantiated");
            assert!(
                realized.contains(key) || instantiated,
                "`{key}` is realized by no root of the corpus"
            );
        }
    }
    // Every probe realizes the constructs it is for.
    for probe in corpus::probes() {
        let root = format!("OracleProbe.{}.{}", probe.module, probe.root);
        let held = constructs
            .get(&root)
            .unwrap_or_else(|| panic!("{root} is eligible"));
        for key in probe.keys {
            assert!(held.contains(*key), "{root} realizes `{key}`");
        }
    }
    // The rejection fixtures: each is a committed fixture LexLean refuses as
    // a production root, and stays a program when the marking is removed.
    for fixture in corpus::REJECTIONS {
        let (code, message) = corpus::recorded_rejection(fixture);
        assert!(code == "LLT4005" || code == "LLT4001", "{fixture}: {code}");
        assert!(!message.is_empty());
        let (project, roots) = corpus::rejection_project(fixture);
        assert!(
            !roots.is_empty(),
            "{fixture} has a root that carried a marking"
        );
        support::checked_project(&project);
    }
}

fn overlap() {
    malformed_non_overlap_tables_are_refused();
    lexlean_side_of_the_corpus();
    let Some(run) = differential::gate("OR-02") else {
        return;
    };
    let registry = differential::registry();
    let table = differential::NonOverlapTable::committed();
    let coverage = differential::coverage(&registry, run, &table);
    assert!(coverage.problems.is_empty(), "{:#?}", coverage.problems);
    let runtime = registry.values().filter(|d| *d == "runtime").count();
    eprintln!(
        "OR-02: {} of {runtime} runtime constructs are realized in the closure of a compiled root; {} rows list the rest",
        coverage.overlapping.len(),
        table.construct.len()
    );
    // Each way the table can be wrong is refused.
    let problems = |edit: &dyn Fn(&mut differential::NonOverlapTable)| -> Vec<String> {
        let mut edited = table.clone();
        edit(&mut edited);
        differential::coverage(&registry, run, &edited).problems
    };
    let row = |table: &differential::NonOverlapTable, key: &str| {
        table
            .construct
            .iter()
            .position(|row| row.key == key)
            .expect("a row")
    };
    // A construct in neither the accepted roots nor the table.
    assert!(problems(&|t| {
        let at = row(t, "primitive.map_insert");
        t.construct.remove(at);
    })
    .iter()
    .any(|p| p.contains("primitive.map_insert") && p.contains("no row")));
    // A row for a construct a root the oracle accepts and compares realizes.
    assert!(problems(&|t| {
        let mut extra = t.construct[row(t, "primitive.map_insert")].clone();
        extra.key = "term.add".to_owned();
        t.construct.push(extra);
    })
    .iter()
    .any(|p| p.contains("term.add") && p.contains("outside the overlap")));
    // A row whose probe the oracle refuses for another reason.
    assert!(problems(&|t| {
        let at = row(t, "primitive.map_insert");
        t.construct[at].refusal = Some("opaque-lowering".to_owned());
    })
    .iter()
    .any(|p| p.contains("primitive.map_insert") && p.contains("opaque-lowering")));
    // A row whose probe the oracle accepts.
    assert!(problems(&|t| {
        let at = row(t, "primitive.map_insert");
        t.construct[at].probe = Some("OracleProbe.ProbeUnit.unitRoot".to_owned());
    })
    .iter()
    .any(|p| p.contains("primitive.map_insert")));
    // A row for a name that is no construct.
    assert!(problems(&|t| {
        let mut extra = t.construct[0].clone();
        extra.key = "term.imaginary".to_owned();
        t.construct.push(extra);
    })
    .iter()
    .any(|p| p.contains("term.imaginary")));
    // A runtime construct listed as not runtime, and a formal-only one listed
    // as refused by the oracle.
    assert!(problems(&|t| {
        let at = row(t, "primitive.map_insert");
        t.construct[at].kind = "not-runtime".to_owned();
        t.construct[at].disposition = Some("formal-only".to_owned());
    })
    .iter()
    .any(|p| p.contains("primitive.map_insert")));
    assert!(problems(&|t| {
        let at = row(t, "type.type");
        t.construct[at].kind = "oracle-refuses".to_owned();
        t.construct[at].probe = Some("OracleProbe.ProbeUnit.unitRoot".to_owned());
        t.construct[at].refusal = Some("x".to_owned());
        t.construct[at].scope = Some("construct".to_owned());
    })
    .iter()
    .any(|p| p.contains("type.type")));
}

// ---- OR-03: the corpus matches ----------------------------------------------

/// The comparison itself, on a root and a stand-in oracle: it agrees when
/// the outcomes, the contract, and the resources agree, and each way they do
/// not is a difference with both sides' evidence.
fn comparison_is_semantic() {
    let (root, _) = synthetic(&json!(null), 0);
    let expected = expected_outcome(&root);
    // Agreement: no difference, whatever the oracle's crate says in bytes.
    let (_, agreeing) = synthetic(&expected, 0);
    assert!(
        diffs_of(&root, &agreeing).is_empty(),
        "equal outcomes, contracts, and resources agree"
    );
    // Disagreement in the outcome is a behavior difference naming both.
    let wrong = json!({"kind": "nat", "value": "12345"});
    let (_, differing) = synthetic(&wrong, 0);
    let found = diffs_of(&root, &differing);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found[0].kind, differential::DiffKind::Behavior);
    assert!(found[0].lexlean.to_string().contains(&expected.to_string()));
    assert!(found[0].oracle.to_string().contains("12345"));
    // An overflow is an overflow whichever error the oracle names; where
    // LexLean has a value it is a difference.
    let (_, named) = synthetic(&json!({"kind": "overflow", "detail": "AddOverflow"}), 0);
    let found = diffs_of(&root, &named);
    assert_eq!(found.len(), 1);
    assert!(found[0].key.starts_with("outcome:"), "{}", found[0].key);
    // A panic is an outcome of its own.
    let (_, panicked) = synthetic(&json!({"kind": "panic"}), 0);
    assert!(diffs_of(&root, &panicked)
        .iter()
        .any(|d| d.key.ends_with("/panic")));
    // An allocation by a root eligible for rust-core is a resource difference.
    let (_, allocating) = synthetic(&expected, 3);
    let found = diffs_of(&root, &allocating);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].kind, differential::DiffKind::Resource);
    // A crate that does not compile is a difference, never an omission.
    let broken = differential::differences(&root, Some(&Err("does not compile".to_owned())));
    assert!(broken.iter().any(|d| d.key == "oracle-crate-does-not-run"));
}

fn matching() {
    comparison_is_semantic();
    rejected_roots_state_their_refusal();
    boundary_inputs();
    let Some(run) = differential::gate("OR-03") else {
        return;
    };
    let table = differential::KnownDifferences::committed();
    let validation = differential::validate(run, &table);
    differential::require("OR-03", &validation);
    let differential::Validation::Validated(summary) = &validation else {
        unreachable!("require returns only for a validated run")
    };
    // The run is not vacuous, and the specification states what it covered.
    let compiled = run
        .executed
        .values()
        .filter(|result| matches!(result, Ok(done) if !done.oracle.is_empty()))
        .count();
    assert!(
        summary.generated > 20 && summary.refused > 50 && summary.cases > 200,
        "{summary:?}"
    );
    let broken = run
        .executed
        .values()
        .filter(|result| result.is_err())
        .count();
    assert!(
        broken > 0,
        "the oracle's crates that rustc rejects are in the run"
    );
    let spec = support::spec_text()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let stated = format!(
        "The corpus has {} roots; the oracle generates Rust for {} of them and refuses {}; {} of the crates it generates for eligible roots compile and are run on {} inputs",
        summary.roots, summary.generated, summary.refused, compiled, summary.cases
    );
    assert!(spec.contains(&stated), "SPEC.md states: {stated}");
    // Every root the oracle's crate was run for agrees with the interpreter and
    // with LexLean's own rendering, on every input.
    for root in &run.roots {
        let Some(Ok(done)) = run.executed.get(&root.root) else {
            continue;
        };
        if done.oracle.is_empty() {
            continue;
        }
        let differential::LexLeanSide::Eligible(accepted) = &root.lexlean else {
            panic!("{} was run without being eligible", root.root);
        };
        assert_eq!(
            done.oracle.len(),
            accepted.cases.len(),
            "{}: one outcome per input",
            root.root
        );
        let own = done
            .lexlean
            .as_ref()
            .unwrap_or_else(|| panic!("{}: LexLean's rendering ran", root.root));
        for (at, case) in accepted.cases.iter().enumerate() {
            let expected = lexlean::calculus::rust::observable(&case.outcome).expect("observable");
            assert_eq!(
                own[at], expected,
                "{}: LexLean's rendering on {:?}",
                root.root, case.arguments
            );
        }
        if accepted.targets.iter().any(|t| t == "rust-core") {
            assert!(
                done.oracle.iter().all(|(_, n)| *n == 0),
                "{}: no allocation in a rust-core root",
                root.root
            );
        }
        assert!(
            done.oracle
                .iter()
                .all(|(outcome, _)| outcome["kind"] != "panic"),
            "{}: no panic",
            root.root
        );
    }
}

// ---- OR-03: the boundary inputs ----------------------------------------------------

/// The inputs of the roots include the edges of their types and literals, and
/// the audit that says so can fail (no Lean, no oracle).
fn boundary_inputs() {
    use lexlean::calculus::Value;
    type Roots = std::sync::Arc<Vec<(String, String, differential::Accepted)>>;
    // The eligible roots of the examples and the probes: stated once for the
    // process, by the corpus run or by this case, whichever asks first.
    let stated: Vec<(String, Roots)> = std::thread::scope(|scope| {
        let handles: Vec<_> = corpus::corpora()
            .into_iter()
            .filter(|c| matches!(c.kind, corpus::Kind::Example | corpus::Kind::Probes))
            .map(|c| {
                scope.spawn(move || {
                    (
                        c.name.clone(),
                        differential::shared_roots(&c.name, &c.project),
                    )
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().expect("the roots are stated"))
            .collect()
    });
    let production_coverage = support::P::copy_example("production-coverage");
    let inputs = |root: &str| -> Vec<Vec<Value>> {
        stated
            .iter()
            .flat_map(|(_, roots)| roots.iter())
            .find(|(_, name, _)| name == root)
            .unwrap_or_else(|| panic!("{root} is eligible"))
            .2
            .cases
            .iter()
            .map(|case| case.values.clone())
            .collect()
    };
    let int8 = |value: i8| Value::I8 {
        value: value.to_string(),
    };
    // MIN / -1 of a signed width: where a checked division and a wrapping one
    // part, and no seeded sample reaches.
    assert!(
        inputs("Coverage.Prims.fixedSmall").contains(&vec![int8(i8::MIN), int8(-1)]),
        "i8: -128 and -1 occur together"
    );
    let int64 = |value: i64| Value::I64 {
        value: value.to_string(),
    };
    assert!(
        inputs("Coverage.Prims.fixedWide")
            .iter()
            .any(|input| input[..2] == [int64(i64::MIN), int64(-1)]),
        "i64: MIN and -1 occur together"
    );
    // Every arm of a match over an option and a result: the one no sample built.
    assert!(
        inputs("Coverage.Main.options").contains(&vec![
            Value::None,
            Value::Error {
                value: Box::new(Value::Bool { value: false })
            }
        ]),
        "none and error false occur together"
    );
    // A split into exactly as many fields as the limit allows, and one fewer
    // and one more.
    let fields: BTreeSet<usize> = inputs("Coverage.Prims.text")
        .iter()
        .filter_map(|input| match (&input[0], &input[1]) {
            (Value::String { value: text }, Value::String { value: delimiter })
                if !delimiter.is_empty() =>
            {
                Some(text.split(delimiter.as_str()).count())
            }
            _ => None,
        })
        .collect();
    for count in [3, 4, 5] {
        assert!(
            fields.contains(&count),
            "a split into {count} fields: {fields:?}"
        );
    }
    // Every constructor of every parameter type of every root of the examples
    // and the probes is built by some input.
    let mut roots = 0;
    for corpus in corpus::corpora() {
        if !matches!(corpus.kind, corpus::Kind::Example | corpus::Kind::Probes) {
            continue;
        }
        let (_, eligible) = stated
            .iter()
            .find(|(name, _)| *name == corpus.name)
            .expect("the corpus was stated");
        for audit in differential::seed_audit(&corpus.project, eligible.as_slice(), &|_, _| true) {
            roots += 1;
            assert!(
                audit.uncovered.is_empty(),
                "{}: no input builds {:?}",
                audit.root,
                audit.uncovered
            );
            if !audit.parameters.is_empty() {
                assert!(audit.boundary > 0, "{}: boundary inputs", audit.root);
            }
        }
    }
    assert_eq!(
        roots,
        stated.iter().map(|(_, roots)| roots.len()).sum::<usize>(),
        "every eligible root was audited"
    );
    assert!(roots > 50, "{roots} roots audited");
    // The audit can fail: without the inputs that build `none`, it says so.
    let (_, eligible) = stated
        .iter()
        .find(|(name, _)| name == "production-coverage")
        .expect("the corpus was stated");
    let audit =
        differential::seed_audit(&production_coverage, eligible.as_slice(), &|root, input| {
            !(root == "Coverage.Main.options" && input[0] == Value::None)
        });
    let options = audit
        .iter()
        .find(|audit| audit.root == "Coverage.Main.options")
        .expect("the root");
    assert!(
        options.uncovered.iter().any(|arm| arm.ends_with(": none")),
        "the audit names the arm no input builds: {:?}",
        options.uncovered
    );
}

/// A root LexLean rejects and the oracle refuses is an agreement on the
/// verdict whose reason the table records, and a refusal for another reason is
/// a disagreement (no Lean, no oracle).
fn rejected_roots_state_their_refusal() {
    use oracle::Stage;
    let rejected = |root: &str, kind: &str| differential::RootRun {
        corpus: "production-polymorphic-root".to_owned(),
        root: root.to_owned(),
        lean_module: "LanguageTwelve.Main".to_owned(),
        lexlean: differential::LexLeanSide::Rejected {
            code: "LLT4005".to_owned(),
            message: "a type parameter".to_owned(),
        },
        oracle: differential::OracleSide::Refused {
            stage: Stage::Export,
            kind: kind.to_owned(),
            message: "the oracle's words".to_owned(),
        },
    };
    let table = differential::KnownDifferences::committed();
    let named = differential::differences(
        &rejected("LanguageTwelve.Main.identity", "opaque-lowering"),
        None,
    );
    assert_eq!(named.len(), 1, "a refusal of a rejected root is stated");
    assert_eq!(named[0].key, "both-refuse:opaque-lowering");
    carries_both_sides_of_a_verdict(&named[0]);
    assert!(
        differential::judge(&named, &table).unlisted.is_empty(),
        "the recorded reason is accepted"
    );
    let other = differential::differences(
        &rejected("LanguageTwelve.Main.identity", "codegen-OpaqueType"),
        None,
    );
    let judged = differential::judge(&other, &table);
    assert_eq!(
        judged.unlisted.len(),
        1,
        "a refusal for another reason is a disagreement"
    );
    let elsewhere = differential::differences(
        &rejected("LanguageTwelve.Main.unrecorded", "opaque-lowering"),
        None,
    );
    assert_eq!(
        differential::judge(&elsewhere, &table).unlisted.len(),
        1,
        "a rejected root the table does not name is a disagreement"
    );
}

fn carries_both_sides_of_a_verdict(diff: &differential::Diff) {
    assert_eq!(diff.lexlean["verdict"], "rejected");
    assert_eq!(diff.oracle["verdict"], "refused");
    assert!(diff.lexlean["code"] == "LLT4005" && !diff.oracle["message"].is_null());
}

// ---- OR-05: the versions of the tables ---------------------------------------------

/// A table's version names its rows: a row that changes without a new version,
/// and a version raised without a change or without its digest recorded, are
/// each refused (no Lean, no oracle).
fn tables_are_versioned() {
    let history = differential::TableVersions::committed();
    let differences_text = read("oracle/known-differences.toml");
    let overlap_text = read("oracle/known-non-overlap.toml");
    let differences = differential::KnownDifferences::parse(&differences_text).expect("table");
    let overlap = differential::NonOverlapTable::parse(&overlap_text).expect("table");
    history
        .check(
            "known-differences.toml",
            differences.version,
            &differences.digest(),
        )
        .expect("the committed differences are their version");
    history
        .check("known-non-overlap.toml", overlap.version, &overlap.digest())
        .expect("the committed overlap table is its version");
    // A layout that is not a row changes nothing ...
    let commented = differential::KnownDifferences::parse(&format!(
        "# a remark\n\n{}\n\n# another\n",
        differences_text.replace("\n\n", "\n\n\n")
    ))
    .expect("table");
    assert_eq!(commented.digest(), differences.digest());
    // ... and a word of a row changes it, and the version no longer names it.
    let edited = differential::KnownDifferences::parse(&differences_text.replacen(
        "Same verdict, the oracle's own reason.",
        "Same verdict, the oracle's reason.",
        1,
    ))
    .expect("table");
    assert_ne!(edited.digest(), differences.digest(), "the edit applied");
    let error = history
        .check("known-differences.toml", edited.version, &edited.digest())
        .expect_err("a row changed without a new version is refused");
    assert!(
        error.contains("rows changed") && error.contains("raise `version`"),
        "{error}"
    );
    let removed = differential::NonOverlapTable::parse(&overlap_text.replacen(
        "[[construct]]\nkey = \"type.type\"",
        "[[construct]]\nkey = \"type.type-planted\"",
        1,
    ))
    .expect("table");
    assert!(history
        .check("known-non-overlap.toml", removed.version, &removed.digest())
        .is_err());
    // A version raised with the rows as they were is not recorded.
    let raised = differential::KnownDifferences::parse(&differences_text.replacen(
        &format!("version = {}", differences.version),
        &format!("version = {}", differences.version + 1),
        1,
    ))
    .expect("table");
    let error = history
        .check("known-differences.toml", raised.version, &raised.digest())
        .expect_err("a version raised without a record is refused");
    assert!(error.contains("records it up to version"), "{error}");
    assert!(history.check("unknown.toml", 1, "").is_err());
    // The history itself is closed.
    let text = read("oracle/table-versions.toml");
    let refused = |from: &str, to: &str| {
        let edited = text.replacen(from, to, 1);
        assert_ne!(edited, text, "the edit `{from}` applied");
        differential::TableVersions::parse(&edited)
            .map(|_| ())
            .expect_err(&format!("`{from}` -> `{to}` is refused"))
    };
    refused("{ version = 2,", "{ version = 3,");
    refused(
        "lexlean/oracle-table-versions/1",
        "lexlean/oracle-table-versions/2",
    );
    refused(
        "file = \"known-non-overlap.toml\"",
        "file = \"known-differences.toml\"",
    );
    refused(
        "{ version = 1, rows_sha256 = \"e46b",
        "{ version = 1, rows_sha256 = \"E46b",
    );
    let same = text.replacen(
        "2b09c7bbd4a744100b620c3b327ace1a38389e7654b55d2a97b96530c249921f",
        "e46beeaee24c5346bd67e6b3e025deb9dc97ed08346b5e2cf474250139aafb2d",
        1,
    );
    assert!(differential::TableVersions::parse(&same)
        .expect_err("a version of the rows of the one before is refused")
        .contains("changed nothing"));
}

// ---- OR-04: planted discrepancies -----------------------------------------------

/// A planted edit of the oracle's crate for a root, and what the differential
/// must say of it.
struct Plant {
    name: &'static str,
    root: &'static str,
    edit: fn(&str) -> Option<String>,
    /// The same site rewritten so that it means what it meant: the plant's
    /// control. A detector that says this is a discrepancy is not looking at
    /// the behaviour, and a result that stood for another plant's would show
    /// here as a discrepancy where there is none.
    null: fn(&str) -> Option<String>,
    kind: differential::DiffKind,
    keys: &'static [&'static str],
}

/// An edit that changes the text and not the meaning (an equivalent mutant):
/// the differential must not call it a discrepancy, however the inputs are
/// chosen.
struct Equivalent {
    name: &'static str,
    root: &'static str,
    edit: fn(&str) -> Option<String>,
}

fn replace_all(source: &str, from: &str, to: &str) -> Option<String> {
    source.contains(from).then(|| source.replace(from, to))
}

fn replace_once(source: &str, from: &str, to: &str) -> Option<String> {
    source.contains(from).then(|| source.replacen(from, to, 1))
}

fn plants() -> Vec<Plant> {
    use differential::DiffKind::{Behavior, Contract, Resource};
    vec![
        Plant {
            name: "a checked multiplication made wrapping",
            root: "Production.Main.shapeArea",
            edit: |s| {
                replace_once(
                    s,
                    ".checked_mul(x_16).ok_or(crate::ComputeError::MulOverflow)?",
                    ".wrapping_mul(x_16)",
                )
            },
            null: |s| replace_once(s, ".checked_mul(x_16)", ".checked_mul(x_16 * 1)"),
            kind: Behavior,
            keys: &["outcome:overflow/value"],
        },
        Plant {
            name: "an addition replaced by a multiplication",
            root: "Production.Main.sumAll",
            edit: |s| replace_once(s, ".checked_add(_x_32)", ".checked_mul(_x_32)"),
            null: |s| replace_once(s, ".checked_add(_x_32)", ".checked_add(_x_32 + 0)"),
            kind: Behavior,
            keys: &["outcome:value/value", "outcome:overflow/value"],
        },
        Plant {
            name: "a literal off by one",
            root: "Coverage.Main.shapes",
            edit: |s| replace_once(s, "let _x_33 = 3;", "let _x_33 = 4;"),
            null: |s| replace_once(s, "let _x_33 = 3;", "let _x_33 = 2 + 1;"),
            kind: Behavior,
            keys: &["outcome:value/value"],
        },
        Plant {
            name: "swapped branches",
            root: "Models.Main.step",
            edit: |s| {
                let swapped = s
                    .replace("false => {", "\u{1}")
                    .replace("true => {", "false => {")
                    .replace('\u{1}', "true => {");
                (swapped != s).then_some(swapped)
            },
            null: |s| replace_once(s, "true => {", "true => { let _null = 0;"),
            kind: Behavior,
            keys: &[
                "outcome:value/value",
                "outcome:value/overflow",
                "outcome:overflow/value",
            ],
        },
        Plant {
            name: "an inverted comparison",
            root: "Models.Main.step",
            edit: |s| replace_once(s, "(cost <= _x_1)", "(cost > _x_1)"),
            null: |s| replace_once(s, "(cost <= _x_1)", "(cost <= (_x_1 + 0))"),
            kind: Behavior,
            keys: &[
                "outcome:value/value",
                "outcome:value/overflow",
                "outcome:overflow/value",
            ],
        },
        Plant {
            name: "a dropped recursive arm",
            root: "Production.Main.sumAll",
            edit: |s| replace_once(s, "total(&(tail_17))?", "0"),
            null: |s| replace_once(s, "total(&(tail_17))?", "(total(&(tail_17))? + 0)"),
            kind: Behavior,
            keys: &["outcome:value/value", "outcome:overflow/value"],
        },
        Plant {
            name: "swapped parameters",
            root: "Coverage.Main.natOps",
            edit: |s| {
                replace_once(
                    s,
                    "pub fn natOps(a: u64, b: u64)",
                    "pub fn natOps(b: u64, a: u64)",
                )
            },
            null: |s| {
                replace_once(
                    s,
                    "pub fn natOps(a: u64, b: u64)",
                    "pub fn natOps(a: u64, b: u64,)",
                )
            },
            kind: Behavior,
            keys: &["outcome:value/value"],
        },
        Plant {
            name: "a changed parameter width",
            root: "Coverage.Prims.bits",
            edit: |s| {
                replace_once(
                    s,
                    "pub fn bits(a: u16, b: u16, s: u32)",
                    "pub fn bits(a: u16, b: u16, s: u64)",
                )
            },
            null: |s| {
                replace_once(
                    s,
                    "pub fn bits(a: u16, b: u16, s: u32)",
                    "pub fn bits(a: u16, b: u16, s: u32,)",
                )
            },
            kind: Contract,
            keys: &["signature-drift"],
        },
        Plant {
            name: "a dropped failure typing",
            root: "Production.Main.halvings",
            edit: |s| {
                replace_once(s, "pub fn halvings(", "fn halvings_inner(").map(|edited| {
                    format!("{edited}\npub fn halvings(number: u64) -> u64 {{ halvings_inner(number).unwrap_or(0) }}\n")
                })
            },
            null: |s| replace_once(s, "pub fn halvings(", "#[inline]\npub fn halvings("),
            kind: Contract,
            keys: &["failure-typing:only-lexlean"],
        },
        // The edges a seeded sample does not reach: each of these was missed
        // when the inputs were the production differential's alone.
        Plant {
            name: "a checked signed division made wrapping (MIN / -1)",
            root: "Coverage.Prims.fixedSmall",
            edit: |s| {
                replace_once(
                    s,
                    "(a).checked_div(b)",
                    "(if b == 0 { None } else { Some((a).wrapping_div(b)) })",
                )
            },
            null: |s| {
                replace_once(
                    s,
                    "(a).checked_div(b)",
                    "(if b == 0 { None } else { (a).checked_div(b) })",
                )
            },
            kind: Behavior,
            keys: &["outcome:value/value"],
        },
        Plant {
            name: "a checked negation made wrapping (MIN)",
            root: "Coverage.Prims.fixedSmall",
            edit: |s| replace_once(s, "(a).checked_neg()", "Some((a).wrapping_neg())"),
            null: |s| replace_once(s, "(a).checked_neg()", "(a).checked_neg().map(|x| x)"),
            kind: Behavior,
            keys: &["outcome:value/value"],
        },
        Plant {
            name: "a truncating division made Euclidean",
            root: "Coverage.Prims.fixedWide",
            edit: |s| replace_once(s, "(a).checked_div(b)", "(a).checked_div_euclid(b)"),
            null: |s| replace_once(s, "(a).checked_div(b)", "(a).checked_div(b).map(|x| x)"),
            kind: Behavior,
            keys: &["outcome:value/value"],
        },
        Plant {
            name: "a checked addition made wrapping (MAX + 1)",
            root: "Coverage.Prims.fixedWide",
            edit: |s| replace_once(s, "(c).checked_add(_x_17)", "Some((c).wrapping_add(_x_17))"),
            null: |s| replace_once(s, "(c).checked_add(_x_17)", "(c).checked_add(_x_17 + 0)"),
            kind: Behavior,
            keys: &["outcome:value/value"],
        },
        Plant {
            name: "a shift made to wrap at the width",
            root: "Coverage.Prims.bits",
            edit: |s| replace_once(s, "(a).checked_shl(s)", "Some((a).wrapping_shl(s))"),
            null: |s| replace_once(s, "(a).checked_shl(s)", "(a).checked_shl(s).map(|x| x)"),
            kind: Behavior,
            keys: &["outcome:value/value"],
        },
        Plant {
            name: "a match arm's constant changed",
            root: "Coverage.Main.options",
            edit: |s| replace_once(s, "let _x_94 = 2;", "let _x_94 = 3;"),
            null: |s| replace_once(s, "let _x_94 = 2;", "let _x_94 = 1 + 1;"),
            kind: Behavior,
            keys: &["outcome:value/value"],
        },
        Plant {
            name: "a split limit one too small",
            root: "Coverage.Prims.text",
            edit: |s| replace_once(s, "let _x_67 = 4;", "let _x_67 = 3;"),
            null: |s| replace_once(s, "let _x_67 = 4;", "let _x_67 = 2 + 2;"),
            kind: Behavior,
            keys: &["outcome:value/value"],
        },
        Plant {
            name: "characters counted as bytes",
            root: "Coverage.Prims.text",
            edit: |s| replace_once(s, "(s).chars().count() as u64", "(s).len() as u64"),
            null: |s| {
                replace_once(
                    s,
                    "(s).chars().count() as u64",
                    "((s).chars().count() as u64 + 0)",
                )
            },
            kind: Behavior,
            keys: &["outcome:value/value"],
        },
        Plant {
            name: "a saturating subtraction made wrapping",
            root: "Coverage.Main.natOps",
            edit: |s| {
                replace_all(
                    s,
                    "identity::<u64>(a).saturating_sub(b)",
                    "identity::<u64>(a).wrapping_sub(b)",
                )
            },
            null: |s| {
                replace_all(
                    s,
                    "identity::<u64>(a).saturating_sub(b)",
                    "identity::<u64>(a).saturating_sub(b).max(0)",
                )
            },
            kind: Behavior,
            keys: &["outcome:value/value", "outcome:value/overflow"],
        },
        Plant {
            name: "a checked addition made a panic on overflow",
            root: "Production.Main.checkedSum",
            edit: |s| replace_once(s, "(left).checked_add(right)", "Some((left) + (right))"),
            null: |s| {
                replace_once(
                    s,
                    "(left).checked_add(right)",
                    "(left).checked_add(right).map(|x| x)",
                )
            },
            kind: Behavior,
            keys: &["outcome:value/panic"],
        },
        Plant {
            name: "an append in the other order",
            root: "Coverage.Prims.sequences",
            edit: |s| {
                replace_once(
                    s,
                    "__append.0.extend(__append.1); __append.0",
                    "__append.1.extend(__append.0); __append.1",
                )
            },
            null: |s| {
                replace_once(
                    s,
                    "__append.0.extend(__append.1); __append.0",
                    "__append.0.extend(__append.1.into_iter()); __append.0",
                )
            },
            kind: Behavior,
            keys: &["outcome:value/value"],
        },
        Plant {
            name: "an allocation in a rust-core root",
            root: "Production.Main.halvings",
            edit: |s| {
                replace_once(
                    s,
                    "let _x_43 = 1;",
                    "let _x_43 = { let _v = alloc::vec![1u8]; _v[0] as u64 };",
                )
            },
            null: |s| replace_once(s, "let _x_43 = 1;", "let _x_43 = 0 + 1;"),
            kind: Resource,
            keys: &["allocation:rust-core-root"],
        },
    ]
}

/// Edits that change the text and not the meaning of the oracle's crate.
fn equivalents() -> Vec<Equivalent> {
    vec![
        Equivalent {
            name: "the quotient by zero, which the product that follows ignores",
            root: "Coverage.Main.natOps",
            edit: |s| replace_once(s, "let _x_2 = 7;", "let _x_2 = 8;"),
        },
        Equivalent {
            // The root's last component is `s == d` conjoined with a
            // comparison that is always false, so it is false whatever the
            // equality says.
            name: "an equality made an inequality under a conjunction with false",
            root: "Coverage.Prims.text",
            edit: |s| replace_once(s, "let _x_43 = (s == d);", "let _x_43 = (s != d);"),
        },
    ]
}

/// What a disagreement carries: the root, the kind, a name, and the evidence
/// of both sides.
fn carries_both_sides(diff: &differential::Diff) {
    assert!(!diff.root.is_empty() && !diff.key.is_empty());
    assert!(!diff.lexlean.is_null(), "{}: LexLean's evidence", diff.key);
    assert!(
        !diff.oracle.is_null(),
        "{}: the oracle's evidence",
        diff.key
    );
    if diff.kind == differential::DiffKind::Behavior {
        let lexlean = diff.lexlean.to_string();
        let oracle = diff.oracle.to_string();
        assert!(
            lexlean.contains("arguments") && oracle.contains("arguments"),
            "the inputs are recorded"
        );
        assert!(
            lexlean.contains("outcome") && oracle.contains("outcome"),
            "the outcomes are recorded"
        );
    } else if diff.kind == differential::DiffKind::Resource {
        assert!(
            diff.lexlean.to_string().contains("targets"),
            "LexLean's targets"
        );
        assert!(
            diff.oracle.to_string().contains("allocations"),
            "the oracle's allocations"
        );
    } else {
        assert!(
            diff.lexlean.to_string().contains("contract"),
            "LexLean's contract"
        );
        assert!(
            diff.oracle.to_string().contains("contract"),
            "the oracle's contract"
        );
    }
}

fn planted_discrepancies() {
    // The same detection on a stand-in, which needs no oracle: a changed
    // result, a changed contract, and the evidence of both sides.
    let (root, _) = synthetic(&json!(null), 0);
    let expected = expected_outcome(&root);
    let (_, wrong) = synthetic(&json!({"kind": "nat", "value": "7"}), 0);
    for diff in diffs_of(&root, &wrong) {
        carries_both_sides(&diff);
    }
    let mut abi = root.clone();
    if let differential::OracleSide::Generated { contract, .. } = &mut abi.oracle {
        contract.parameters.push(differential::Shape::Bool);
        contract.failure = !contract.failure;
        contract.borrowed.push(0);
    }
    let (_, agreeing) = synthetic(&expected, 0);
    let found = diffs_of(&abi, &agreeing);
    assert!(found.iter().any(|d| d.key == "arity"), "{found:?}");
    assert!(
        found.iter().any(|d| d.key.starts_with("failure-typing")),
        "{found:?}"
    );
    for diff in &found {
        carries_both_sides(diff);
    }
    // Runs under one key would let one result stand for another's; the
    // executor refuses them before it builds anything.
    let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let slot = differential::Slot {
            key: "same",
            run: &root,
        };
        let _ = differential::execute(&[slot, slot], false);
    }));
    assert!(refused.is_err(), "two runs under one key are refused");
    let Some(run) = differential::gate("OR-04") else {
        return;
    };
    let table = differential::KnownDifferences::committed();
    let find = |name: &str| -> &differential::RootRun {
        run.roots
            .iter()
            .find(|candidate| candidate.root == name)
            .unwrap_or_else(|| panic!("{name} is a root of the corpus"))
    };
    let all = plants();
    assert!(
        all.iter()
            .any(|a| all.iter().filter(|b| b.root == a.root).count() > 1),
        "some root has more than one plant, which is the case the keys exist for"
    );
    let make = |edit: &dyn Fn(&str) -> Option<String>, plant: &str, root: &str| {
        differential::planted(find(root), edit).unwrap_or_else(|reason| panic!("{plant}: {reason}"))
    };
    // One evaluation holds every control, plant, null edit, and equivalent
    // mutant; each has a crate and a result of its own, so a root with several
    // of them is judged by each, not by the last.
    let mut runs: Vec<differential::RootRun> = Vec::new();
    let control_roots: BTreeSet<&str> = all
        .iter()
        .map(|plant| plant.root)
        .chain(equivalents().iter().map(|e| e.root))
        .collect();
    for name in &control_roots {
        runs.push(find(name).clone());
    }
    let controls = 0..runs.len();
    let planted_at = runs.len();
    for plant in &all {
        runs.push(make(&plant.edit, plant.name, plant.root));
    }
    let nulls_at = runs.len();
    for plant in &all {
        runs.push(make(&plant.null, plant.name, plant.root));
    }
    let equivalents_at = runs.len();
    let equivalent_list = equivalents();
    for equivalent in &equivalent_list {
        runs.push(make(&equivalent.edit, equivalent.name, equivalent.root));
    }
    let found = differential::evaluate(&runs);
    let unlisted = |at: usize| differential::judge(&found[at], &table).unlisted;
    // The controls: the unplanted roots, built and run the same way, agree.
    for at in controls {
        assert!(
            unlisted(at).is_empty(),
            "the unplanted {} agrees: {:?}",
            runs[at].root,
            unlisted(at)
        );
    }
    // The null edits and the equivalent mutants are not discrepancies.
    for (position, plant) in all.iter().enumerate() {
        let diffs = unlisted(nulls_at + position);
        assert!(
            diffs.is_empty(),
            "{}: its null edit (the same site, the same meaning) is not a discrepancy: {:?}",
            plant.name,
            diffs
                .iter()
                .map(|d| (d.kind, d.key.clone()))
                .collect::<Vec<_>>()
        );
    }
    for (position, equivalent) in equivalent_list.iter().enumerate() {
        let diffs = unlisted(equivalents_at + position);
        assert!(
            diffs.is_empty(),
            "{}: an equivalent mutant is not a discrepancy: {:?}",
            equivalent.name,
            diffs
                .iter()
                .map(|d| (d.kind, d.key.clone()))
                .collect::<Vec<_>>()
        );
    }
    // Each plant is a disagreement, however many others share its root.
    let mut by_plant: Vec<BTreeSet<(differential::DiffKind, String)>> = Vec::new();
    let mut missed: Vec<String> = Vec::new();
    for (position, plant) in all.iter().enumerate() {
        let at = planted_at + position;
        let run_of_plant = &runs[at];
        let judgement = differential::judge(&found[at], &table);
        let hits: Vec<&differential::Diff> = judgement
            .unlisted
            .iter()
            .filter(|diff| diff.kind == plant.kind && plant.keys.contains(&diff.key.as_str()))
            .collect();
        by_plant.push(
            judgement
                .unlisted
                .iter()
                .map(|d| (d.kind, d.key.clone()))
                .collect(),
        );
        if hits.is_empty() {
            missed.push(format!(
                "{} in {} is detected as {:?} {:?}: found {:?}",
                plant.name, run_of_plant.root, plant.kind, plant.keys, by_plant[position]
            ));
            continue;
        }
        for diff in hits {
            carries_both_sides(diff);
        }
        // The run containing it is a disagreement, recorded with both sides.
        let mut diffs_of_run: Vec<differential::Diff> = Vec::new();
        for other in &run.roots {
            if other.root != run_of_plant.root {
                diffs_of_run.extend(differential::differences(
                    other,
                    run.executed.get(&other.root),
                ));
            }
        }
        diffs_of_run.extend(found[at].clone());
        let judged = differential::judge(&diffs_of_run, &table);
        let verdict = differential::Validation::Disagreement(judged.unlisted, judged.stale);
        assert!(!verdict.validated());
        assert!(verdict.report().contains(&run_of_plant.root));
    }
    assert!(
        missed.is_empty(),
        "plants the differential did not detect:\n{}",
        missed.join("\n")
    );
    // The same plants, run each alone, are found as they are in company: what
    // is found does not depend on what else is built beside it.
    for name in [
        "an addition replaced by a multiplication",
        "swapped branches",
        "a checked signed division made wrapping (MIN / -1)",
    ] {
        let position = all
            .iter()
            .position(|plant| plant.name == name)
            .unwrap_or_else(|| panic!("{name} is a plant"));
        let alone = differential::evaluate(std::slice::from_ref(&runs[planted_at + position]));
        let alone: BTreeSet<(differential::DiffKind, String)> =
            differential::judge(&alone[0], &table)
                .unlisted
                .iter()
                .map(|d| (d.kind, d.key.clone()))
                .collect();
        assert_eq!(alone, by_plant[position], "{name}: alone and in company");
    }
}

// ---- OR-05: known deliberate differences ----------------------------------------

fn malformed_difference_tables_are_refused() {
    let text = read("oracle/known-differences.toml");
    let refused = |edit: &dyn Fn(&mut toml::Value)| {
        let mut value: toml::Value = text.parse().expect("the table is TOML");
        edit(&mut value);
        differential::KnownDifferences::parse(&value.to_string())
            .map(|_| ())
            .expect_err("a malformed table is refused");
    };
    refused(&|v| v["spec"] = toml::Value::String("lexlean/oracle-differences/2".into()));
    refused(&|v| v["version"] = toml::Value::Integer(0));
    refused(&|v| v["difference"][0]["reason"] = toml::Value::String(" ".into()));
    refused(&|v| v["difference"][0]["roots"] = toml::Value::Array(Vec::new()));
    refused(&|v| v["difference"][0]["kind"] = toml::Value::String("bytes".into()));
    refused(&|v| v["difference"][1]["id"] = v["difference"][0]["id"].clone());
    refused(&|v| {
        let first = v["difference"][0]["roots"][0].clone();
        v["difference"][0]["roots"]
            .as_array_mut()
            .expect("roots")
            .push(first);
    });
    refused(&|v| {
        v["difference"][0]
            .as_table_mut()
            .expect("row")
            .insert("note".into(), toml::Value::Integer(1));
    });
}

/// The judgement of differences against a table, on stand-in differences:
/// listed ones are accepted, unlisted ones are disagreements, and a row that
/// is not matched is stale.
fn judgement_is_exact() {
    let table = differential::KnownDifferences::parse(
        "spec = \"lexlean/oracle-differences/1\"\nversion = 1\n\n[[difference]]\nid = \"KD-1\"\nkind = \"acceptance\"\nkey = \"oracle-refuses:opaque-lowering\"\nreason = \"a stand-in\"\nroots = [\"A.b\"]\n",
    )
    .expect("a table");
    let diff = |root: &str, kind: differential::DiffKind, key: &str| differential::Diff {
        corpus: "c".into(),
        root: root.into(),
        kind,
        key: key.into(),
        lexlean: json!({"verdict": "eligible"}),
        oracle: json!({"verdict": "refused"}),
    };
    use differential::DiffKind::{Acceptance, Behavior};
    let known = diff("A.b", Acceptance, "oracle-refuses:opaque-lowering");
    // Listed and present: accepted.
    let ok = differential::judge(std::slice::from_ref(&known), &table);
    assert!(ok.unlisted.is_empty() && ok.stale.is_empty() && ok.accepted == 1);
    // Present on another root, of another kind, or under another name: not
    // the listed one.
    for other in [
        diff("A.c", Acceptance, "oracle-refuses:opaque-lowering"),
        diff("A.b", Behavior, "oracle-refuses:opaque-lowering"),
        diff("A.b", Acceptance, "oracle-refuses:escaping-local-function"),
    ] {
        let judged = differential::judge(&[known.clone(), other.clone()], &table);
        assert_eq!(judged.unlisted.len(), 1, "{other:?} is a disagreement");
        assert!(judged.stale.is_empty());
    }
    // Absent: the row is stale.
    let judged = differential::judge(&[], &table);
    assert_eq!(judged.stale, [("KD-1".to_owned(), "A.b".to_owned())]);
    // Planted and then listed: a difference that was a disagreement is
    // accepted once a row names it, and is re-asserted by that row.
    let planted = diff("A.b", Behavior, "outcome:value/value");
    let judged = differential::judge(&[known.clone(), planted.clone()], &table);
    assert_eq!(judged.unlisted.len(), 1);
    let text = "spec = \"lexlean/oracle-differences/1\"\nversion = 2\n\n[[difference]]\nid = \"KD-1\"\nkind = \"acceptance\"\nkey = \"oracle-refuses:opaque-lowering\"\nreason = \"a stand-in\"\nroots = [\"A.b\"]\n\n[[difference]]\nid = \"KD-2\"\nkind = \"behavior\"\nkey = \"outcome:value/value\"\nreason = \"a stand-in\"\nroots = [\"A.b\"]\n";
    let listed = differential::KnownDifferences::parse(text).expect("a table");
    let judged = differential::judge(&[known.clone(), planted], &listed);
    assert!(judged.unlisted.is_empty() && judged.stale.is_empty());
    assert!(
        differential::judge(&[known], &listed).stale.len() == 1,
        "the vanished difference is stale"
    );
}

fn known_differences() {
    malformed_difference_tables_are_refused();
    tables_are_versioned();
    judgement_is_exact();
    let table = differential::KnownDifferences::committed();
    assert!(table.version >= 1 && table.difference.len() >= 5);
    let Some(run) = differential::gate("OR-05") else {
        return;
    };
    let mut diffs = Vec::new();
    for root in &run.roots {
        diffs.extend(differential::differences(
            root,
            run.executed.get(&root.root),
        ));
    }
    assert!(!diffs.is_empty());
    // The committed table names every difference of the run, and each of its
    // rows is matched on each of its roots.
    let judgement = differential::judge(&diffs, &table);
    assert!(judgement.unlisted.is_empty(), "{:?}", judgement.unlisted);
    assert!(judgement.stale.is_empty(), "{:?}", judgement.stale);
    // Remove a row: every difference it named is a disagreement.
    let mut without = table.clone();
    let removed = without.difference.remove(0);
    let judged = differential::judge(&diffs, &without);
    assert_eq!(
        judged.unlisted.len(),
        removed.roots.len(),
        "the roots of {}",
        removed.id
    );
    // Move a row's key: it names nothing, and what it named is unlisted.
    let mut renamed = table.clone();
    renamed.difference[1].key.push_str("-planted");
    let judged = differential::judge(&diffs, &renamed);
    assert!(!judged.unlisted.is_empty() && !judged.stale.is_empty());
    // A listed root the run does not show.
    let mut extended = table.clone();
    extended.difference[1]
        .roots
        .push("Production.Main.checkedSum".to_owned());
    let judged = differential::judge(&diffs, &extended);
    assert_eq!(
        judged.stale,
        [(
            extended.difference[1].id.clone(),
            "Production.Main.checkedSum".to_owned()
        )]
    );
}

// ---- OR-06: meaning, not bytes -------------------------------------------------

/// A rendering that keeps LexLean's runtime and replaces its functions.
fn with_functions(
    lexlean: &BTreeMap<String, Vec<u8>>,
    functions: &str,
) -> BTreeMap<String, Vec<u8>> {
    let source = String::from_utf8_lossy(&lexlean["src/lib.rs"]).into_owned();
    let at = source
        .find("\npub fn f0(")
        .expect("the rendering's functions follow its runtime");
    let mut files = lexlean.clone();
    files.insert(
        "src/lib.rs".to_owned(),
        format!("{}\n{functions}", &source[..at]).into_bytes(),
    );
    files
}

fn equivalent_renderings() {
    let project = support::P::copy_example("production");
    let roots = differential::eligible_roots(&project);
    let find = |name: &str| -> differential::Accepted {
        roots
            .iter()
            .find(|(_, root, _)| root == name)
            .map(|(_, _, accepted)| accepted.clone())
            .unwrap_or_else(|| panic!("{name} is a root of examples/production"))
    };
    let halvings = find("Production.Main.halvings");
    let sum_all = find("Production.Main.sumAll");
    // The baselines: LexLean's own renderings, as the Rust backend renders them.
    let own_halvings = differential::lexlean_package(&halvings, "rust-core");
    let own_sum = differential::lexlean_package(&sum_all, "rust-std");
    let base_halvings = differential::render_candidate(&halvings, "rust-core", &own_halvings)
        .expect("LexLean's rendering builds");
    let base_sum = differential::render_candidate(&sum_all, "rust-std", &own_sum)
        .expect("LexLean's rendering builds");
    differential::accept_rendering(&halvings, &base_halvings, &base_halvings)
        .expect("a rendering is itself");
    differential::accept_rendering(&sum_all, &base_sum, &base_sum).expect("a rendering is itself");
    // Deliberately better, and equivalent: a closed form for the recursion that
    // counts down by twos, and a loop for the right-nested sum (fixtures).
    let fixture = |name: &str| read(&format!("tests/oracle/renderings/{name}"));
    let closed_form = with_functions(&own_halvings, &fixture("halvings.closed-form.rs"));
    let looped = with_functions(&own_sum, &fixture("sum-all.loop.rs"));
    for (name, accepted, profile, better, own, base) in [
        (
            "a closed form for the recursion",
            &halvings,
            "rust-core",
            &closed_form,
            &own_halvings,
            &base_halvings,
        ),
        (
            "a loop for the recursion",
            &sum_all,
            "rust-std",
            &looped,
            &own_sum,
            &base_sum,
        ),
    ] {
        assert_ne!(
            better["src/lib.rs"], own["src/lib.rs"],
            "{name} differs from LexLean's bytes"
        );
        let rendered = differential::render_candidate(accepted, profile, better)
            .unwrap_or_else(|reason| panic!("{name} builds against the interface: {reason}"));
        differential::accept_rendering(accepted, &rendered, base)
            .unwrap_or_else(|reason| panic!("{name} is accepted: {reason}"));
    }
    // Wrong in one place is refused. One result on one input: the closed form
    // is wrong at one odd input of the differential; the loop wraps where the
    // sum overflows, which one input of the differential reaches; another
    // does more work; another takes a narrower parameter.
    let Some(lexlean::calculus::Value::Nat { value: odd }) = halvings
        .cases
        .iter()
        .map(|case| case.values[0].clone())
        .rfind(|value| {
            matches!(value, lexlean::calculus::Value::Nat { value }
                if value.parse::<u64>().is_ok_and(|n| n % 2 == 1))
        })
    else {
        panic!("the inputs of halvings include an odd one");
    };
    let one_input = with_functions(
        &own_halvings,
        &fixture("halvings.wrong-on-one-input.rs").replace("@INPUT@", &odd),
    );
    let wrapping = with_functions(&own_sum, &fixture("sum-all.wrapping.rs"));
    let busy = with_functions(&own_sum, &fixture("sum-all.extra-work.rs"));
    let narrow = with_functions(&own_halvings, &fixture("halvings.narrow-parameter.rs"));
    let one_input_result =
        differential::render_candidate(&halvings, "rust-core", &one_input).expect("it builds");
    let error = differential::accept_rendering(&halvings, &one_input_result, &base_halvings)
        .expect_err("a rendering wrong on one input is refused");
    assert!(error.contains("on ["), "the input is named: {error}");
    let wrapping_result =
        differential::render_candidate(&sum_all, "rust-std", &wrapping).expect("it builds");
    let error = differential::accept_rendering(&sum_all, &wrapping_result, &base_sum)
        .expect_err("a rendering that wraps where the sum overflows is refused");
    assert!(error.contains("overflow"), "{error}");
    let busy_result =
        differential::render_candidate(&sum_all, "rust-std", &busy).expect("it builds");
    assert_eq!(
        busy_result.outcomes, base_sum.outcomes,
        "the busy rendering prints the same outcomes"
    );
    let error = differential::accept_rendering(&sum_all, &busy_result, &base_sum)
        .expect_err("a rendering that does more work is refused");
    assert!(error.contains("work"), "{error}");
    // The work is what the rendering's own runtime counts, so a rendering that
    // does more and counts none of it is not caught by it. The allocations are
    // counted by an allocator the harness installs, which the rendering cannot
    // reach, and they are the measure that does not depend on the rendering.
    // The limit is stated, not hidden: a rendering that does extra work
    // without allocating and without counting is not caught by either (SPEC.md
    // §17.19).
    let silent = with_functions(&own_sum, &fixture("sum-all.silent-extra.rs"));
    let silent_result =
        differential::render_candidate(&sum_all, "rust-std", &silent).expect("it builds");
    assert_eq!(
        silent_result.outcomes, base_sum.outcomes,
        "the silent rendering prints the same outcomes"
    );
    assert!(
        silent_result.work <= base_sum.work,
        "it counts no more work ({} against {})",
        silent_result.work,
        base_sum.work
    );
    let error = differential::accept_rendering(&sum_all, &silent_result, &base_sum)
        .expect_err("a rendering that does more and counts none of it is refused");
    assert!(error.contains("allocates"), "{error}");
    assert!(
        differential::render_candidate(&halvings, "rust-core", &narrow).is_err(),
        "a rendering with a narrower parameter does not provide the interface"
    );
    // Against the oracle's crates, when it ran: the better rendering differs in
    // bytes from the oracle's too.
    if let Some(run) = differential::gate("OR-06") {
        for (name, better) in [
            ("Production.Main.halvings", &closed_form),
            ("Production.Main.sumAll", &looped),
        ] {
            let root = run.roots.iter().find(|r| r.root == name).expect("a root");
            let differential::OracleSide::Generated { package, .. } = &root.oracle else {
                panic!("{name} is generated by the oracle");
            };
            assert_ne!(package["src/lib.rs"], better["src/lib.rs"]);
        }
    }
}

// ---- OR-01 / OR-07: the checkout and what a failing oracle is --------------------

/// The pinned files of a checkout, copied to `to`.
fn copy_used(from: &Path, to: &Path, pin: &Pin) {
    for entry in walkdir::WalkDir::new(from)
        .into_iter()
        .filter_entry(|entry| !support::holds_cache_tag(entry))
        .flatten()
    {
        let relative = entry.path().strip_prefix(from).expect("under root");
        let text = relative
            .components()
            .map(|part| part.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/");
        if !entry.file_type().is_file()
            || text
                .split('/')
                .any(|part| matches!(part, ".git" | ".lake" | "target"))
            || !pin.used.iter().any(|prefix| {
                text == prefix.trim_end_matches('/') || text.starts_with(prefix.as_str())
            })
        {
            continue;
        }
        let destination = to.join(relative);
        std::fs::create_dir_all(destination.parent().expect("parent")).expect("dir");
        std::fs::copy(entry.path(), destination).expect("copy");
    }
}

/// A checkout that is not the pin is refused: the acquired checkout, copied
/// and changed in each way.
fn checkouts_that_are_not_the_pin() {
    let Some(oracle) = differential::oracle_gate("OR-01") else {
        return;
    };
    let pin = Pin::committed();
    let pristine = tempfile::tempdir().expect("tempdir");
    copy_used(oracle.source(), pristine.path(), &pin);
    oracle::verify_checkout(&pin, pristine.path()).expect("a copy of the pinned files is the pin");
    let changed = |edit: &dyn Fn(&Path)| -> Result<(), Unavailable> {
        let copy = tempfile::tempdir().expect("tempdir");
        copy_used(pristine.path(), copy.path(), &pin);
        edit(copy.path());
        oracle::verify_checkout(&pin, copy.path())
    };
    let file = "lean/Prod/Lower.lean";
    for (name, result) in [
        (
            "a byte changed",
            changed(&|d| {
                let mut bytes = std::fs::read(d.join(file)).expect("file");
                bytes[0] ^= 1;
                std::fs::write(d.join(file), bytes).expect("write");
            }),
        ),
        (
            "a file added",
            changed(&|d| write(d, "rust/prod-ir/src/added.rs", "")),
        ),
        (
            "a file removed",
            changed(&|d| std::fs::remove_file(d.join("rust/prod-ir/Cargo.toml")).expect("rm")),
        ),
        (
            "a file renamed",
            changed(&|d| {
                std::fs::rename(d.join(file), d.join("lean/Prod/Lowered.lean")).expect("mv")
            }),
        ),
        (
            "an empty checkout",
            changed(&|d| {
                std::fs::remove_dir_all(d.join("lean"))
                    .and_then(|()| std::fs::remove_dir_all(d.join("rust")))
                    .expect("rm")
            }),
        ),
    ] {
        assert!(
            matches!(result, Err(Unavailable::PinMismatch(_))),
            "{name} is not the pin: {result:?}"
        );
    }
    // The same files at another commit: the digest holds and HEAD does not.
    let repository = tempfile::tempdir().expect("tempdir");
    copy_used(pristine.path(), repository.path(), &pin);
    let git = |arguments: &[&str]| {
        std::process::Command::new("git")
            .args([
                "-c",
                "user.name=lexlean",
                "-c",
                "user.email=lexlean@example.invalid",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(arguments)
            .current_dir(repository.path())
            .output()
    };
    match git(&["init", "--quiet"])
        .and_then(|_| git(&["add", "-A"]))
        .and_then(|_| git(&["commit", "--quiet", "-m", "elsewhere"]))
    {
        Ok(output) if output.status.success() => {
            match oracle::verify_checkout(&pin, repository.path()) {
                Err(Unavailable::PinMismatch(detail)) => {
                    assert!(detail.contains("HEAD"), "{detail}")
                }
                other => panic!("another commit of the same files is not the pin: {other:?}"),
            }
        }
        _ => eprintln!("OR-01: this host has no usable git; the other-commit refusal was not run"),
    }
}

/// An oracle that does not build is unavailable: the pinned files with a Lake
/// target that is not there.
fn a_build_that_fails_is_unavailable() {
    let Some(oracle) = differential::oracle_gate("OR-07") else {
        return;
    };
    let mut pin = Pin::committed();
    pin.lean_target = "no-such-target".to_owned();
    let cache = tempfile::tempdir().expect("tempdir");
    match oracle::Oracle::acquire_in(&pin, cache.path(), Some(oracle.source().to_path_buf())) {
        Err(Unavailable::BuildFailed { step, .. }) => assert_eq!(step, "lake build"),
        other => panic!(
            "an oracle that does not build is unavailable: {:?}",
            other.map(|_| ())
        ),
    }
}

/// A disagreement is recorded, with both sides' evidence, and fails the gate.
fn disagreement_is_recorded() {
    let (root, executed) = synthetic(&json!({"kind": "nat", "value": "99"}), 0);
    let mut run = differential::Run {
        roots: vec![root.clone()],
        executed: BTreeMap::new(),
        pin: Pin::committed(),
        timings: BTreeMap::new(),
    };
    run.executed.insert(root.root.clone(), Ok(executed));
    let table = differential::KnownDifferences::committed();
    // The stand-in has a row in no table, so its difference is a disagreement.
    let validation = differential::validate(&run, &table);
    let differential::Validation::Disagreement(unlisted, _) = &validation else {
        panic!(
            "a differing outcome is a disagreement: {}",
            validation.report()
        );
    };
    assert!(!unlisted.is_empty());
    let report = validation.report();
    assert!(
        report.contains("lexlean:") && report.contains("oracle:") && report.contains("99"),
        "{report}"
    );
    let directory = tempfile::tempdir().expect("tempdir");
    let path = validation
        .record(directory.path())
        .expect("the record is written")
        .expect("a disagreement is recorded");
    let record: Json = serde_json::from_slice(&std::fs::read(path).expect("record")).expect("JSON");
    assert_eq!(record["role"], json!("validation-evidence"));
    let first = &record["disagreements"][0];
    assert!(
        !first["lexlean"].is_null() && !first["oracle"].is_null(),
        "both sides are recorded"
    );
    assert!(first["oracle"].to_string().contains("99"));
    // A validated or an unavailable verdict records nothing.
    let unavailable = differential::Validation::Unavailable(Unavailable::NotAcquired("x".into()));
    assert_eq!(unavailable.record(directory.path()).expect("io"), None);
    // `require` fails closed on each.
    for verdict in [&validation, &unavailable] {
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            differential::require("OR-07", verdict);
        }));
        assert!(
            caught.is_err(),
            "a verdict that is not validated fails the gate"
        );
    }
}

pub fn run(id: &str) {
    let start = std::time::Instant::now();
    run_case(id);
    eprintln!("{id}: {:.1} s", start.elapsed().as_secs_f64());
}

fn run_case(id: &str) {
    match id {
        "OR-01" => {
            pin_and_digest();
            toolchains_are_the_repositorys();
            checkouts_that_are_not_the_pin();
        }
        "OR-02" => overlap(),
        "OR-03" => matching(),
        "OR-04" => planted_discrepancies(),
        "OR-05" => known_differences(),
        "OR-06" => equivalent_renderings(),
        "OR-07" => {
            failures_are_unavailable();
            acquisition_fails_closed();
            acquisition_is_atomic_and_heals();
            retries_are_bounded();
            vacuous_runs_are_unavailable();
            gates_decide();
            disagreement_is_recorded();
            a_build_that_fails_is_unavailable();
        }
        "OR-08" => {
            isolation_audit();
            records_are_closed();
            oracle_is_only_its_own();
        }
        other => panic!("no oracle case is wired for {other}"),
    }
}
