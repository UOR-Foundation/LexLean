//! CL-22 (SPEC.md §17.17, §24.6): a program that embeds the library ends the
//! children it started through it with `lexlean::verify::end_live_children`.
//!
//! The shutdown state the function enters belongs to the whole process, and
//! this case runs in the conformance binary beside every other case, which
//! start children of their own. Each scenario therefore runs in a host of its
//! own: this test again, in a separate process, with its role named by
//! `HOST_ROLE`. The host checks what only it can see (its run's diagnostic,
//! the report, its own identity and dispositions) and writes what it saw to a
//! report file; the parent reads that file, the host's exit status, and the
//! process table.

use crate::support;

/// The role of a host process; absent in the conformance run itself.
const HOST_ROLE: &str = "LEXLEAN_CL22_HOST";
/// The file a host writes what it saw to, one `key value` line each.
const REPORT: &str = "LEXLEAN_CL22_REPORT";
/// What a host needs from the parent: a directory to work in.
const WORK: &str = "LEXLEAN_CL22_WORK";

pub(crate) fn run() {
    if let Ok(role) = std::env::var(HOST_ROLE) {
        host(&role);
        return;
    }
    // Children started concurrently with the call, and the deterministic
    // cases of ending, refusing, and allowing again, on every host.
    let stress = launch("stress");
    assert_report(&stress, "stress");
    eprintln!(
        "CL-22 stress: {}",
        stress.value("rounds").unwrap_or_default()
    );
    #[cfg(unix)]
    unix::parent();
    one_kill_path();
}

/// The shipped crate ends a process group or a process tree in exactly one
/// place, `end_groups`, which `end_live_children` and a child's timeout
/// reach; the executable's interrupt thread reaches it only through
/// `end_live_children`. A second kill path (a thread of its own that signals
/// the groups, a timeout that ends its child by other means) fails here.
fn one_kill_path() {
    let source = support::repo_root().join("crates/lexlean/src");
    let mut group_signals = Vec::new();
    let mut tree_ends = Vec::new();
    let mut single_signals = Vec::new();
    let mut end_callers = Vec::new();
    let mut interrupt_thread = None;
    for entry in walkdir::WalkDir::new(source.as_std_path())
        .sort_by_file_name()
        .into_iter()
        .flatten()
    {
        if entry
            .path()
            .extension()
            .is_none_or(|extension| extension != "rs")
        {
            continue;
        }
        // `/`-separated, so the comparison is the same on Windows.
        let relative = entry
            .path()
            .strip_prefix(source.as_std_path())
            .expect("under src")
            .components()
            .map(|part| part.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/");
        // A Windows checkout may end its lines with CR LF.
        let text = std::fs::read_to_string(entry.path())
            .expect("source")
            .replace("\r\n", "\n");
        // The code the crate ships: a file's tests are its last module.
        let shipped = text
            .split("\n#[cfg(test)]\nmod ")
            .next()
            .unwrap_or_default();
        for (index, _) in shipped.match_indices("kill_process_group(") {
            if !shipped[..index].ends_with("test_") {
                group_signals.push((relative.clone(), enclosing_fn(shipped, index)));
            }
        }
        for (index, _) in shipped.match_indices("Command::new(\"taskkill\")") {
            tree_ends.push((relative.clone(), enclosing_fn(shipped, index)));
        }
        for (index, _) in shipped.match_indices("kill_process(") {
            single_signals.push((relative.clone(), enclosing_fn(shipped, index)));
        }
        for (index, _) in shipped.match_indices("end_groups(") {
            if !shipped[..index].ends_with("fn ") {
                end_callers.push((relative.clone(), enclosing_fn(shipped, index)));
            }
        }
        // The thread the executable's `main` arms.
        if relative == "verify/lifeline.rs" {
            interrupt_thread = fn_body(shipped, "fn install(").map(str::to_owned);
        }
    }
    let at = |file: &str, function: &str| (file.to_owned(), function.to_owned());
    assert_eq!(
        group_signals,
        [at("verify/lifeline.rs", "signal_group")],
        "a process group is signalled in exactly one place"
    );
    assert_eq!(
        tree_ends,
        [at("verify/lifeline.rs", "end_tree")],
        "a process tree is ended in exactly one place"
    );
    assert_eq!(
        single_signals,
        Vec::<(String, String)>::new(),
        "no shipped code signals a single process by ID"
    );
    assert_eq!(
        end_callers,
        [
            at("verify/child.rs", "stop"),
            at("verify/lifeline.rs", "end_live_children")
        ],
        "the timeout and the public function are the only ways in"
    );
    let body = interrupt_thread.expect("the executable's interrupt thread is installed");
    assert!(
        body.contains("end_live_children()") && !body.contains("kill"),
        "the interrupt thread ends the children through the public function and nothing else: {body}"
    );
}

/// The name of the function whose body holds `index`.
fn enclosing_fn(text: &str, index: usize) -> String {
    let start = text[..index].rfind("fn ").map_or(0, |at| at + 3);
    text[start..]
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// The text of the function that `signature` begins, through its closing
/// brace.
fn fn_body<'a>(text: &'a str, signature: &str) -> Option<&'a str> {
    let start = text.find(signature)?;
    let open = start + text[start..].find('{')?;
    let mut depth = 0_usize;
    for (offset, c) in text[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&text[start..=open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

/// What a host wrote, and how it ended.
struct Hosted {
    lines: Vec<(String, String)>,
    status: std::process::ExitStatus,
    output: String,
    _work: tempfile::TempDir,
}

impl Hosted {
    fn value(&self, key: &str) -> Option<&str> {
        self.lines
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.as_str())
    }
}

/// This test, run again as a host with `role`.
fn host_command(role: &str, work: &std::path::Path) -> std::process::Command {
    let mut command = std::process::Command::new(std::env::current_exe().expect("this test"));
    command
        .args([
            "--exact",
            "conformance_cl_22",
            "--test-threads=1",
            "--nocapture",
        ])
        .env(HOST_ROLE, role)
        .env(REPORT, work.join("report"))
        .env(WORK, work)
        // The host's run of this case is not a case of its own.
        .env_remove(lexlean::verify::operational::TELEMETRY_VARIABLE)
        .stdin(std::process::Stdio::null())
        // Files, not pipes: a process the host leaves behind must not keep
        // the parent waiting for the end of its output.
        .stdout(std::fs::File::create(work.join("stdout")).expect("stdout"))
        .stderr(std::fs::File::create(work.join("stderr")).expect("stderr"));
    // The host's fake tools come first.
    let mut path = vec![work.join("bin")];
    path.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    command.env("PATH", std::env::join_paths(path).expect("PATH"));
    command
}

/// Run a host to its end; on unix it leads a process group of its own.
fn launch(role: &str) -> Hosted {
    let work = tempfile::Builder::new()
        .prefix("lexlean-cl22-")
        .tempdir()
        .expect("tempdir");
    let mut command = host_command(role, work.path());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let status = command.status().expect("the host runs");
    hosted(work, status)
}

fn hosted(work: tempfile::TempDir, status: std::process::ExitStatus) -> Hosted {
    let read = |name: &str| std::fs::read_to_string(work.path().join(name)).unwrap_or_default();
    let lines = std::fs::read_to_string(work.path().join("report"))
        .unwrap_or_default()
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(' ').unwrap_or((line, ""));
            (key.to_owned(), value.to_owned())
        })
        .collect();
    Hosted {
        lines,
        status,
        output: format!("{}{}", read("stdout"), read("stderr")),
        _work: work,
    }
}

/// The host ran to its own end, said so, and left nothing running.
fn assert_report(hosted: &Hosted, role: &str) {
    assert!(
        hosted.status.success() && hosted.value("done") == Some(role),
        "the {role} host failed ({:?}): {:?}\n{}",
        hosted.status,
        hosted.lines,
        hosted.output
    );
}

/// Append one line to the host's report.
fn note(key: &str, value: &str) {
    use std::io::Write;
    let path = std::env::var_os(REPORT).expect("a host has a report file");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("report");
    writeln!(file, "{key} {value}").expect("report line");
}

fn work() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var_os(WORK).expect("a host has a work directory"))
}

fn host(role: &str) {
    match role {
        "stress" => stress(),
        // A child the stress host starts, and what that child starts.
        "tree" => tree(),
        "leaf" => leaf(),
        #[cfg(unix)]
        "embedded" => unix::embedded(),
        #[cfg(unix)]
        "INT" | "TERM" => unix::signalled(role),
        other => panic!("no CL-22 host role {other}"),
    }
}

/// Wait until `ready` holds, for a minute at most: a bound on a condition, so
/// that a defect fails the case instead of hanging it, and not an assumption
/// about how long anything takes.
fn wait_until(what: &str, mut ready: impl FnMut() -> bool) {
    let give_up = std::time::Instant::now() + std::time::Duration::from_secs(60);
    while !ready() {
        assert!(std::time::Instant::now() < give_up, "{what}");
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

/// Run `call` on a thread of its own and return what it returns. A call that
/// is still running after 20 seconds started a child that nothing ended: it
/// is ended, so that the case fails instead of hanging, and is the failure.
fn bounded<T: Send>(what: &str, call: impl FnOnce() -> T + Send) -> T {
    std::thread::scope(|scope| {
        let handle = scope.spawn(call);
        let give_up = std::time::Instant::now() + std::time::Duration::from_secs(20);
        while !handle.is_finished() {
            if std::time::Instant::now() >= give_up {
                let _ = lexlean::verify::end_live_children();
                panic!("{what}");
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        handle.join().expect("the call returns")
    })
}

/// The limits of an ordinary project, with a child timeout long enough that
/// no child here reaches it unless it survived the call.
fn limits() -> lexlean::config::Limits {
    lexlean::config::Limits {
        max_file_bytes: 4_194_304,
        max_total_source_bytes: 67_108_864,
        max_primitive_atoms: 2_000_000,
        max_token_lattice_edges: 4_000_000,
        max_parse_states: 4_000_000,
        max_ir_nodes: 2_000_000,
        max_scope_depth: 1024,
        max_import_depth: 128,
        max_diagnostics: 256,
        max_child_output_bytes: 16_777_216,
        child_timeout_ms: 300_000,
    }
}

/// A child that starts a long-running grandchild in its group and waits for
/// it, as `lake` starts `lean`: a shell script on unix, and on Windows this
/// test again (a tree whose leaf records its process ID).
struct Sleeper {
    program: camino::Utf8PathBuf,
    argv: Vec<String>,
    extra_env: Vec<(String, String)>,
    cwd: camino::Utf8PathBuf,
    /// unix: the command line of the grandchild.
    #[cfg(unix)]
    marker: String,
    /// windows: where each leaf writes a file named by its process ID.
    #[cfg(windows)]
    leaves: std::path::PathBuf,
}

impl Sleeper {
    fn new() -> Self {
        let cwd = camino::Utf8PathBuf::from_path_buf(work()).expect("utf-8");
        #[cfg(unix)]
        {
            let marker = format!("sleep 1000.{}", std::process::id());
            let script = cwd.join("lake");
            script_file(
                script.as_std_path(),
                &format!("#!/bin/sh\n{marker} &\nwait\n"),
            );
            Self {
                program: script,
                argv: Vec::new(),
                extra_env: Vec::new(),
                cwd,
                marker,
            }
        }
        #[cfg(windows)]
        {
            let leaves = cwd.join("leaves");
            std::fs::create_dir_all(&leaves).expect("leaves");
            let program =
                camino::Utf8PathBuf::from_path_buf(std::env::current_exe().expect("this test"))
                    .expect("utf-8");
            Self {
                program,
                argv: [
                    "--exact",
                    "conformance_cl_22",
                    "--test-threads=1",
                    "--quiet",
                ]
                .map(str::to_owned)
                .to_vec(),
                extra_env: vec![
                    (HOST_ROLE.to_owned(), "tree".to_owned()),
                    (WORK.to_owned(), leaves.to_string()),
                ],
                cwd,
                leaves: leaves.into_std_path_buf(),
            }
        }
    }

    /// Start one and wait for it to end: the run's own result.
    fn run(&self) -> Result<lexlean::verify::child::ChildRecord, lexlean::Diagnostic> {
        let spec = lexlean::verify::child::ChildSpec {
            tool: "lake",
            module: Some("Probe".to_owned()),
            program: &self.program,
            executable_sha256: lexlean::Sha256Digest::of(b"sleeper"),
            argv: self.argv.clone(),
            cwd: &self.cwd,
            extra_env: self.extra_env.clone(),
            home: lexlean::verify::child::ChildHome::Isolated { home: &self.cwd },
        };
        lexlean::verify::child::run(
            &spec,
            &limits(),
            &lexlean::verify::child::Normalizer::default(),
        )
    }

    /// The grandchildren that have started.
    fn started(&self) -> usize {
        #[cfg(unix)]
        {
            running(&self.marker).len()
        }
        #[cfg(windows)]
        {
            std::fs::read_dir(&self.leaves)
                .expect("leaves")
                .flatten()
                .filter(|entry| entry.path().extension().is_none())
                .count()
        }
    }

    /// The grandchildren that are still running.
    fn left(&self) -> Vec<String> {
        #[cfg(unix)]
        {
            running(&self.marker)
                .into_iter()
                .map(|(pid, command)| format!("{pid} {command}"))
                .collect()
        }
        // `taskkill /F` begins the termination of each process of the tree
        // and returns; the system takes them off its table soon after, which
        // is waited for (bounded, like every wait here).
        #[cfg(windows)]
        {
            let give_up = std::time::Instant::now() + std::time::Duration::from_secs(10);
            loop {
                let listed = windows::listed();
                let left: Vec<String> = std::fs::read_dir(&self.leaves)
                    .expect("leaves")
                    .flatten()
                    .filter_map(|entry| entry.file_name().to_str()?.parse::<u32>().ok())
                    .filter(|pid| listed.contains(pid))
                    .map(|pid| pid.to_string())
                    .collect();
                if left.is_empty() || std::time::Instant::now() >= give_up {
                    return left;
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }
}

/// The deterministic cases, then rounds in which several threads start
/// children at once while the host calls the function.
fn stress() {
    use lexlean::verify::{allow_new_children, end_live_children, GroupOutcome};
    let sleeper = Sleeper::new();
    let identity = Identity::now();

    // No child is live: nothing is signalled.
    let report = end_live_children();
    assert!(report.groups.is_empty(), "{report:?}");
    // The shutdown state refuses a start, and says why.
    let refused =
        bounded("a child started in the shutdown state", || sleeper.run()).expect_err("refused");
    assert_refused(&refused);
    allow_new_children();

    // One child, running: ended, and its run says so instead of hanging.
    std::thread::scope(|scope| {
        let run = scope.spawn(|| sleeper.run());
        wait_until("the child started its grandchild", || {
            sleeper.started() == 1
        });
        let report = end_live_children();
        assert_eq!(report.groups.len(), 1, "{report:?}");
        assert!(
            matches!(report.groups[0].outcome, GroupOutcome::Ended),
            "{report:?}"
        );
        assert!(report.all_ended());
        let ended = run.join().expect("the run returns").expect_err("ended");
        assert_ended(&ended);
        // Idempotent: what the first call ended is gone, or forgotten.
        let again = end_live_children();
        assert!(
            again
                .groups
                .iter()
                .all(|group| matches!(group.outcome, GroupOutcome::AlreadyGone)),
            "{again:?}"
        );
    });
    assert_eq!(sleeper.left(), Vec::<String>::new());
    allow_new_children();

    let (rounds, threads) = if cfg!(windows) { (3, 4) } else { (12, 8) };
    let (mut ended, mut refused) = (0, 0);
    for round in 0..rounds {
        let barrier = std::sync::Barrier::new(threads + 1);
        std::thread::scope(|scope| {
            let runs: Vec<_> = (0..threads)
                .map(|_| {
                    scope.spawn(|| {
                        barrier.wait();
                        sleeper.run()
                    })
                })
                .collect();
            barrier.wait();
            let report = end_live_children();
            // A child that was started concurrently with the call and
            // survived it keeps its run waiting.
            let give_up = std::time::Instant::now() + std::time::Duration::from_secs(20);
            while !runs.iter().all(std::thread::ScopedJoinHandle::is_finished) {
                if std::time::Instant::now() >= give_up {
                    let left = sleeper.left();
                    let _ = end_live_children();
                    panic!("round {round}: a child started concurrently with the call survived it: {left:?}");
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            assert!(report.all_ended(), "round {round}: {report:?}");
            ended += report.groups.len();
            for run in runs {
                let error = run
                    .join()
                    .expect("the run returns")
                    .expect_err("ended or refused");
                assert_eq!(error.code.as_str(), "LLV7018", "round {round}: {error:?}");
                if error.message.contains("was not started") {
                    refused += 1;
                }
            }
        });
        let left = sleeper.left();
        assert!(left.is_empty(), "round {round} left running: {left:?}");
        allow_new_children();
    }
    assert_eq!(identity, Identity::now(), "the host is untouched");
    note(
        "rounds",
        &format!("{rounds} ended {ended} refused {refused}"),
    );
    note("done", "stress");
}

fn assert_refused(error: &lexlean::Diagnostic) {
    assert_eq!(error.code.as_str(), "LLV7018", "{error:?}");
    assert!(error.message.contains("was not started"), "{error:?}");
}

fn assert_ended(error: &lexlean::Diagnostic) {
    assert_eq!(error.code.as_str(), "LLV7018", "{error:?}");
    assert!(error.message.contains("was ended by"), "{error:?}");
}

/// A child of the stress host: start a leaf and wait for it.
fn tree() {
    let mut leaf = std::process::Command::new(std::env::current_exe().expect("this test"))
        .args([
            "--exact",
            "conformance_cl_22",
            "--test-threads=1",
            "--quiet",
        ])
        .env(HOST_ROLE, "leaf")
        .stdin(std::process::Stdio::null())
        .spawn()
        .expect("the leaf starts");
    let _ = leaf.wait();
}

/// A grandchild of the stress host: say who it is, and run until ended.
fn leaf() {
    let leaves = work();
    let pid = std::process::id().to_string();
    // Written whole, then named: a reader never sees half a file.
    std::fs::write(leaves.join(format!("{pid}.part")), &pid).expect("leaf");
    std::fs::rename(leaves.join(format!("{pid}.part")), leaves.join(&pid)).expect("leaf");
    std::thread::sleep(std::time::Duration::from_secs(1000));
}

/// What the host is, which the function must leave as it was: its process,
/// its group, and the signals it catches and ignores.
#[derive(Debug, PartialEq, Eq)]
struct Identity {
    pid: u32,
    #[cfg(unix)]
    group: i32,
    #[cfg(unix)]
    dispositions: (u64, u64),
}

impl Identity {
    fn now() -> Self {
        Self {
            pid: std::process::id(),
            #[cfg(unix)]
            group: rustix::process::getpgrp().as_raw_pid(),
            #[cfg(unix)]
            dispositions: unix::dispositions(),
        }
    }
}

#[cfg(unix)]
fn script_file(path: &std::path::Path, body: &str) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::write(path, body).expect("write");
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).expect("chmod");
}

/// The processes that are running and whose command line holds `marker`. A
/// zombie is not running. The table is `/proc` where the host has one and
/// `ps` by its absolute path otherwise (macOS), so the case is not vacuous on
/// a host without `/proc`; a host with neither is an error, because an empty
/// answer there would read as "nothing is left".
#[cfg(unix)]
fn running(marker: &str) -> Vec<(i32, String)> {
    if std::path::Path::new("/proc/self").exists() {
        return std::fs::read_dir("/proc")
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|entry| {
                let pid = entry.file_name().to_str()?.parse::<i32>().ok()?;
                let command = std::fs::read(format!("/proc/{pid}/cmdline")).ok()?;
                let command = String::from_utf8_lossy(&command).replace('\0', " ");
                let status = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
                let zombie = status.rsplit(") ").next()?.starts_with('Z');
                (command.contains(marker) && !zombie).then_some((pid, command))
            })
            .collect();
    }
    let ps = ["/bin/ps", "/usr/bin/ps"]
        .into_iter()
        .find(|path| std::path::Path::new(path).is_file())
        .expect("the host has neither /proc nor ps, so the process table cannot be read");
    let output = std::process::Command::new(ps)
        .args(["-axo", "pid=,stat=,command="])
        .output()
        .expect("ps runs");
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let pid = fields.next()?.parse::<i32>().ok()?;
            let state = fields.next()?;
            let command = fields.collect::<Vec<_>>().join(" ");
            (command.contains(marker) && !state.starts_with('Z')).then_some((pid, command))
        })
        .collect()
}

#[cfg(windows)]
mod windows {
    /// The process IDs `tasklist` lists.
    pub(super) fn listed() -> std::collections::BTreeSet<u32> {
        let output = std::process::Command::new("tasklist")
            .args(["/FO", "CSV", "/NH"])
            .stdin(std::process::Stdio::null())
            .output()
            .expect("tasklist runs");
        assert!(output.status.success(), "tasklist: {:?}", output.status);
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter_map(|line| line.split(',').nth(1)?.trim_matches('"').parse().ok())
            .collect()
    }
}

#[cfg(unix)]
mod unix {
    use super::{
        assert_report, bounded, hosted, launch, note, running, wait_until, work, Identity,
    };
    use lexlean::verify::{allow_new_children, end_live_children, GroupOutcome};

    /// The embedded run, and a real SIGINT and SIGTERM to a host that handles
    /// them itself, with the host leading its group and not.
    pub(super) fn parent() {
        let embedded = launch("embedded");
        assert_report(&embedded, "embedded");
        signalled_from_outside("INT", false);
        signalled_from_outside("TERM", true);
    }

    /// The signals this process catches and ignores, as the kernel's masks:
    /// `SigCgt` and `SigIgn` of `/proc/self/status`, or the `caught` and
    /// `ignored` columns of `ps` where there is no `/proc` (macOS). A host
    /// that can say neither is an error, not a pass.
    pub(super) fn dispositions() -> (u64, u64) {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            let mask = |name: &str| {
                status
                    .lines()
                    .find_map(|line| u64::from_str_radix(line.strip_prefix(name)?.trim(), 16).ok())
                    .expect("/proc/self/status states the mask")
            };
            return (mask("SigCgt:"), mask("SigIgn:"));
        }
        let ps = ["/bin/ps", "/usr/bin/ps"]
            .into_iter()
            .find(|path| std::path::Path::new(path).is_file())
            .expect("the host has neither /proc nor ps, so the dispositions cannot be read");
        let read = |keyword: &str| {
            let output = std::process::Command::new(ps)
                .args([
                    "-o",
                    &format!("{keyword}="),
                    "-p",
                    &std::process::id().to_string(),
                ])
                .output()
                .ok()?;
            let text = String::from_utf8_lossy(&output.stdout);
            let digits = text.trim();
            u64::from_str_radix(digits.strip_prefix("0x").unwrap_or(digits), 16).ok()
        };
        [("caught", "ignored"), ("sigcatch", "sigignore")]
            .into_iter()
            .find_map(|(caught, ignored)| Some((read(caught)?, read(ignored)?)))
            .expect("ps states the caught and ignored signals")
    }

    /// A project whose lock acquisition runs a `git` that starts a
    /// long-running grandchild in its group and waits for it; the marker is
    /// the grandchild's command line.
    fn project() -> (crate::support::P, String) {
        let project = crate::support::P::copy_example("nat-add-zero");
        let manifest = project.root.join("lexlean.toml");
        let text = std::fs::read_to_string(&manifest).expect("read");
        std::fs::write(
            &manifest,
            text.replace(
                "\n[limits]",
                "\n[[lexicon_source]]\npackage = \"test.remote\"\nkind = \"git\"\nurl = \"https://example.invalid/repo.git\"\nrevision = \"0123456789abcdef0123456789abcdef01234567\"\nsubdirectory = \"lexicon\"\n\n[limits]",
            ),
        )
        .expect("write");
        let marker = format!("sleep 2000.{}", std::process::id());
        let bin = work().join("bin");
        std::fs::create_dir_all(&bin).expect("bin");
        super::script_file(&bin.join("git"), &format!("#!/bin/sh\n{marker} &\nwait\n"));
        (project, marker)
    }

    /// `lexlean lock --allow-network` through the library: its exit code and
    /// stderr. The `git` it runs is first on `PATH`, which the parent set.
    fn cli_run(project: &camino::Utf8Path) -> (i32, String) {
        let arguments: Vec<String> = ["lexlean", "lock", "--allow-network"]
            .into_iter()
            .map(str::to_owned)
            .collect();
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = lexlean::cli::run(&arguments, project, &mut out, &mut err);
        (code, String::from_utf8_lossy(&err).into_owned())
    }

    /// A process in this process's own group, which nothing here may end;
    /// ended when dropped, so that a failed assertion leaves nothing behind.
    struct Sibling(std::process::Child);

    impl Drop for Sibling {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }

    fn sibling() -> Sibling {
        Sibling(
            std::process::Command::new("/bin/sh")
                .args(["-c", &format!("exec sleep 3000.{}", std::process::id())])
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("the sibling starts"),
        )
    }

    fn assert_sibling_alive(mut sibling: Sibling) {
        let alive = sibling.0.try_wait().expect("wait").is_none();
        assert!(alive, "the host's own group was signalled");
    }

    /// An embedded run, ended from an ordinary thread of the host.
    pub(super) fn embedded() {
        let (project, marker) = project();
        let identity = Identity::now();
        let sibling = sibling();
        std::thread::scope(|scope| {
            let run = scope.spawn(|| cli_run(&project.root));
            wait_until("the run started its child", || !running(&marker).is_empty());
            let report = end_live_children();
            assert_eq!(report.groups.len(), 1, "{report:?}");
            assert!(
                matches!(report.groups[0].outcome, GroupOutcome::Ended),
                "{report:?}"
            );
            let (code, stderr) = run.join().expect("the run returns");
            assert_eq!(code, 3, "{stderr}");
            assert!(
                stderr.contains("LLV7018") && stderr.contains("was ended by"),
                "{stderr}"
            );
        });
        assert_eq!(running(&marker), Vec::new(), "the grandchild is gone");
        // The shutdown state: the run is refused before its child starts.
        let (code, stderr) = bounded("a child started in the shutdown state", || {
            cli_run(&project.root)
        });
        assert_eq!(code, 3, "{stderr}");
        assert!(
            stderr.contains("LLV7018") && stderr.contains("was not started"),
            "{stderr}"
        );
        assert_eq!(running(&marker), Vec::new());
        // Allowed again, the library starts children again.
        allow_new_children();
        std::thread::scope(|scope| {
            let run = scope.spawn(|| cli_run(&project.root));
            wait_until("the run started its child again", || {
                !running(&marker).is_empty()
            });
            assert!(end_live_children().all_ended());
            let (_, stderr) = run.join().expect("the run returns");
            assert!(stderr.contains("LLV7018"), "{stderr}");
        });
        assert_eq!(running(&marker), Vec::new());
        assert_sibling_alive(sibling);
        assert_eq!(identity, Identity::now(), "the host is untouched");
        note("done", "embedded");
    }

    /// A host with its own handling of SIGINT and SIGTERM, which ends the
    /// children from the thread its handler wakes, then cleans up and exits
    /// with its own status.
    pub(super) fn signalled(which: &str) {
        use signal_hook::consts::{SIGINT, SIGTERM};
        let (project, marker) = project();
        let mut signals =
            signal_hook::iterator::Signals::new([SIGINT, SIGTERM]).expect("the host's handler");
        let identity = Identity::now();
        let sibling = sibling();
        std::thread::scope(|scope| {
            let run = scope.spawn(|| cli_run(&project.root));
            let handler = scope.spawn(move || {
                let signal = signals.forever().next().expect("a signal");
                (signal, end_live_children())
            });
            wait_until("the run started its child", || !running(&marker).is_empty());
            note("ready", &std::process::id().to_string());
            let (signal, report) = handler.join().expect("the handler's thread");
            let expected = if which == "INT" { SIGINT } else { SIGTERM };
            assert_eq!(signal, expected);
            assert_eq!(report.groups.len(), 1, "{report:?}");
            assert!(
                matches!(report.groups[0].outcome, GroupOutcome::Ended),
                "{report:?}"
            );
            let (code, stderr) = run.join().expect("the run returns");
            assert_eq!(code, 3, "{stderr}");
            assert!(stderr.contains("LLV7018"), "{stderr}");
        });
        assert_eq!(running(&marker), Vec::new(), "the grandchild is gone");
        assert_sibling_alive(sibling);
        // The host's own handler is still the only one, as it installed it.
        assert_eq!(identity, Identity::now(), "the host is untouched");
        let (code, stderr) = bounded("a child started in the shutdown state", || {
            cli_run(&project.root)
        });
        assert!(code == 3 && stderr.contains("was not started"), "{stderr}");
        note("cleanup", "ran");
        note("done", which);
    }

    /// Start a host, wait for it to be ready, signal it, and read what it
    /// did. Without `leads`, the host is a member of a group a shell leads,
    /// as under a job-control shell; with it, it leads its own, as a service
    /// manager starts one.
    fn signalled_from_outside(which: &str, leads: bool) {
        use std::os::unix::process::CommandExt;
        let work = tempfile::Builder::new()
            .prefix("lexlean-cl22-")
            .tempdir()
            .expect("tempdir");
        let mut command = if leads {
            let mut command = super::host_command(which, work.path());
            command.process_group(0);
            command
        } else {
            // The shell stays (it has more to do after the host), so it, not
            // the host, leads the group.
            let host = super::host_command(which, work.path());
            let mut command = std::process::Command::new("/bin/sh");
            command
                .args(["-c", "\"$0\" \"$@\"; status=$?; exit $status"])
                .arg(host.get_program())
                .args(host.get_args());
            for (key, value) in host.get_envs() {
                match value {
                    Some(value) => command.env(key, value),
                    None => command.env_remove(key),
                };
            }
            command
                .process_group(0)
                .stdin(std::process::Stdio::null())
                .stdout(std::fs::File::create(work.path().join("stdout")).expect("stdout"))
                .stderr(std::fs::File::create(work.path().join("stderr")).expect("stderr"));
            command
        };
        let mut child = command.spawn().expect("the host runs");
        let report = work.path().join("report");
        let mut pid = None;
        wait_until("the host became ready", || {
            assert!(
                child.try_wait().expect("wait").is_none(),
                "the host ended before it was ready"
            );
            pid = std::fs::read_to_string(&report).ok().and_then(|text| {
                text.lines()
                    .find_map(|line| line.strip_prefix("ready ")?.parse::<i32>().ok())
            });
            pid.is_some()
        });
        let pid = rustix::process::Pid::from_raw(pid.expect("ready")).expect("a process ID");
        let signal = if which == "INT" {
            rustix::process::Signal::INT
        } else {
            rustix::process::Signal::TERM
        };
        rustix::process::kill_process(pid, signal).expect("signal the host");
        let status = child.wait().expect("the host ends");
        let hosted = hosted(work, status);
        assert_eq!(
            hosted.value("cleanup"),
            Some("ran"),
            "the host survived SIG{which} to clean up: {:?}\n{}",
            hosted.lines,
            hosted.output
        );
        assert_report(&hosted, which);
    }
}
