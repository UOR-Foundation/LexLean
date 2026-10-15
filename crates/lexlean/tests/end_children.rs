//! A host that embeds the library ends the children the library started
//! (SPEC.md §17.17, §24.6): one public function ends every live child
//! process group, reports each as ended, already gone, or failing, installs
//! no handler, never signals the host, and refuses a later child start until
//! the host re-enables it. The case is its own test binary because it
//! replaces `PATH`, takes real signals, and ends the process-wide live list,
//! so its cases share one lock and restore the shutdown state.

mod support;

/// The tests share the process-wide child state (the live list and the
/// shutdown state), so they run one at a time.
fn serial() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    LOCK.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Re-enable starting children whatever way the case ends: the shutdown
/// state is process-wide and the next case starts children.
struct Reenable;

impl Drop for Reenable {
    fn drop(&mut self) {
        lexlean::verify::allow_children();
    }
}

/// Restore the `PATH` a case replaced.
#[cfg(unix)]
struct RestorePath(std::ffi::OsString);

#[cfg(unix)]
impl Drop for RestorePath {
    fn drop(&mut self) {
        std::env::set_var("PATH", &self.0);
    }
}

/// A `sleep` that leads a process group of its own, as every child of the
/// library does, registered the way `verify::child::run` registers one.
#[cfg(unix)]
fn spawn_sleep(seconds: u32) -> (std::process::Child, lexlean::verify::lifeline::Guard) {
    use std::os::unix::process::CommandExt;
    let mut command = std::process::Command::new("sleep");
    command
        .arg(format!("{seconds}.{}", std::process::id()))
        .process_group(0)
        .stdin(std::process::Stdio::null())
        // Not the host's pipes: a child that outlives a failed host must not
        // keep the case's reader waiting for an end of file that never comes.
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    lexlean::verify::lifeline::spawn(&mut command).expect("a child starts")
}

/// This process's own process-group number: `/proc` where the host has one
/// and `ps` where it does not (macOS), so the case is not vacuous there.
#[cfg(unix)]
fn own_pgid() -> i32 {
    if let Ok(stat) = std::fs::read_to_string("/proc/self/stat") {
        let mut fields = stat.rsplit_once(')').expect("stat").1.split_whitespace();
        let _state = fields.next();
        let _parent = fields.next();
        return fields.next().expect("pgrp").parse().expect("a number");
    }
    let ps = ["/bin/ps", "/usr/bin/ps"]
        .into_iter()
        .find(|path| std::path::Path::new(path).is_file())
        .expect("the host has neither /proc nor ps, so the process table cannot be read");
    let output = std::process::Command::new(ps)
        .args(["-o", "pgid=", "-p", &std::process::id().to_string()])
        .output()
        .expect("ps runs");
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .expect("a group number")
}

/// The kernel's record of which signals this process catches and which it
/// ignores, as the two masks (bit `n - 1` for signal `n`): `SigCgt` and
/// `SigIgn` of `/proc/self/status` where the host has `/proc`, the `caught`
/// and `ignored` columns of `ps` where the host's `ps` has them. `None` is a
/// host that cannot say (the `ps` of macOS has neither column): there the
/// comparison of masks is not made, and what shows that the library installed
/// no handler is [`a_signal_the_host_did_not_handle_ends_it_by_default`],
/// which needs no mask. A host with `/proc` can always say, and the cases
/// that use this require it, so the comparison is not vacuous where it can be
/// made.
#[cfg(unix)]
fn signal_dispositions() -> Option<(u64, u64)> {
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        let mask = |key: &str| {
            status
                .lines()
                .find_map(|line| u64::from_str_radix(line.strip_prefix(key)?.trim(), 16).ok())
                .unwrap_or_else(|| panic!("/proc/self/status has no {key}"))
        };
        return Some((mask("SigCgt:"), mask("SigIgn:")));
    }
    let ps = ["/bin/ps", "/usr/bin/ps"]
        .into_iter()
        .find(|path| std::path::Path::new(path).is_file())
        .expect("the host has neither /proc nor ps, so the process table cannot be read");
    let column = |keyword: &str| {
        let output = std::process::Command::new(ps)
            .args([
                "-o",
                &format!("{keyword}="),
                "-p",
                &std::process::id().to_string(),
            ])
            .output()
            .expect("ps runs");
        let text = String::from_utf8_lossy(&output.stdout);
        let digits = text.trim();
        u64::from_str_radix(digits.strip_prefix("0x").unwrap_or(digits), 16).ok()
    };
    Some((column("caught")?, column("ignored")?))
}

/// The masks read before must be the masks read now, where the host can read
/// them, and a host with `/proc` must be able to.
#[cfg(unix)]
fn assert_dispositions_unchanged(before: Option<(u64, u64)>, what: &str) {
    if std::path::Path::new("/proc/self/status").is_file() {
        assert!(
            before.is_some(),
            "a host with /proc reports its signal dispositions"
        );
    }
    if let Some(before) = before {
        assert_eq!(
            Some(before),
            signal_dispositions(),
            "{what} changed a signal disposition of the host"
        );
    }
}

/// An embedding host runs the compiler on a thread, ends the live children
/// while a grandchild runs, and every group is reported ended, nothing of
/// the group is left, and the run returns a diagnostic rather than hanging.
#[test]
fn an_embedding_host_ends_every_live_group() {
    let _serial = serial();
    #[cfg(unix)]
    scenario_embedding();
}

#[cfg(unix)]
fn scenario_embedding() {
    use std::time::{Duration, Instant};

    let Some(root) = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .map(std::path::Path::to_path_buf)
        .filter(|root| root.join("examples/nat-add-zero/lexlean.toml").is_file())
    else {
        eprintln!("the crate is being tested outside its repository; the host did not run");
        return;
    };
    let _reenable = Reenable;
    let directory = tempfile::tempdir().expect("tempdir");
    let project = directory.path().join("project");
    copy_tree(&root.join("examples/nat-add-zero"), &project);
    // A git source that must be acquired: `lock --allow-network` runs `git`.
    let manifest = project.join("lexlean.toml");
    let text = std::fs::read_to_string(&manifest).expect("read");
    std::fs::write(
        &manifest,
        text.replace(
            "\n[limits]",
            "\n[[lexicon_source]]\npackage = \"test.remote\"\nkind = \"git\"\nurl = \"https://example.invalid/repo.git\"\nrevision = \"0123456789abcdef0123456789abcdef01234567\"\nsubdirectory = \"lexicon\"\n\n[limits]",
        ),
    )
    .expect("write");
    let bin = directory.path().join("bin");
    std::fs::create_dir(&bin).expect("mkdir");
    // The `git` starts a long-running grandchild, as `lake` starts `lean`,
    // and never finishes. The decimal part is this process, so no other run
    // is mistaken for it.
    let marker = format!("sleep 10000.{}", std::process::id());
    support::script(
        &bin.join("git"),
        &format!("#!/bin/sh\nsleep 10000.{} &\nwait\n", std::process::id()),
    );
    let _path = RestorePath(std::env::var_os("PATH").unwrap_or_default());
    std::env::set_var(
        "PATH",
        format!(
            "{}:{}",
            bin.display(),
            std::env::var("PATH").unwrap_or_default()
        ),
    );

    let arguments: Vec<String> = ["lexlean", "lock", "--allow-network"]
        .into_iter()
        .map(str::to_owned)
        .collect();
    let working = camino::Utf8PathBuf::from_path_buf(project).expect("utf-8");
    let (code_tx, code_rx) = std::sync::mpsc::channel();
    let run = std::thread::spawn(move || {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = lexlean::cli::run(&arguments, &working, &mut out, &mut err);
        let _ = code_tx.send(code);
    });
    let give_up = Instant::now() + Duration::from_secs(60);
    while support::running(&marker).is_empty() {
        assert!(
            Instant::now() < give_up,
            "the run never started the grandchild"
        );
        std::thread::sleep(Duration::from_millis(20));
    }

    let (pid_before, pgid_before) = (std::process::id(), own_pgid());
    let outcome = lexlean::verify::end_live_children();
    assert!(!outcome.is_empty(), "a group was live");
    assert!(
        outcome
            .iter()
            .all(|row| row.outcome == lexlean::verify::GroupEnd::Ended),
        "every live group is reported ended: {outcome:?}"
    );
    assert!(
        outcome
            .iter()
            .all(|row| row.group != u32::try_from(pgid_before).unwrap_or(0)),
        "the host's own group is never among the ended: {outcome:?}"
    );
    let give_up = Instant::now() + Duration::from_secs(10);
    let mut left = support::running(&marker);
    while !left.is_empty() && Instant::now() < give_up {
        std::thread::sleep(Duration::from_millis(20));
        left = support::running(&marker);
    }
    support::end_all(&left);
    assert!(left.is_empty(), "the end left running: {left:?}");
    let code = code_rx
        .recv_timeout(Duration::from_secs(60))
        .expect("the run returns rather than hanging once its children are gone");
    run.join().expect("the run thread");
    assert_ne!(code, 0, "the run returns a diagnostic, not a success");
    assert_eq!(pid_before, std::process::id(), "the host is untouched");
    assert_eq!(pgid_before, own_pgid(), "the host's group is untouched");
}

#[cfg(unix)]
fn copy_tree(from: &std::path::Path, to: &std::path::Path) {
    std::fs::create_dir_all(to).expect("mkdir");
    for entry in std::fs::read_dir(from).expect("read_dir") {
        let entry = entry.expect("entry");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("file type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("copy");
        }
    }
}

/// A host process: this test binary, run again for one case, as the leader
/// of its own process group the way a service manager starts one. A fresh
/// process holds none of the library's process-wide state from earlier
/// cases, so what it reads of its signal dispositions and what ends its
/// children is the library's doing alone, whatever the order of the cases; the
/// signals come from outside as a service manager's would; and the exit
/// status is the host's.
#[cfg(unix)]
struct Host {
    child: std::process::Child,
    pid: u32,
    lines: std::sync::mpsc::Receiver<String>,
    reader: std::thread::JoinHandle<()>,
    errors: std::thread::JoinHandle<String>,
}

#[cfg(unix)]
impl Host {
    fn start(case: &str) -> Self {
        use std::io::{BufRead, BufReader};
        use std::os::unix::process::CommandExt;

        let mut child =
            std::process::Command::new(std::env::current_exe().expect("the test binary"))
                .args(["--exact", case, "--nocapture", "--test-threads=1"])
                .env(HOST_ENV, "1")
                .process_group(0)
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .expect("the host starts");
        let pid = child.id();
        let stdout = child.stdout.take().expect("piped");
        let stderr = child.stderr.take().expect("piped");
        let (tx, lines) = std::sync::mpsc::channel::<String>();
        let reader = std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                let _ = tx.send(line);
            }
        });
        let errors = std::thread::spawn(move || {
            BufReader::new(stderr)
                .lines()
                .map_while(Result::ok)
                .collect::<Vec<_>>()
                .join("\n")
        });
        Host {
            child,
            pid,
            lines,
            reader,
            errors,
        }
    }

    /// The next event the host says, or `None` when it says nothing more
    /// within `seconds` (it ended, or it is stuck). libtest writes
    /// `test <name> ... ` without a newline before a case that prints, so the
    /// first of the host's lines does not start the line it is on.
    fn next_event(&self, deadline: std::time::Instant) -> Option<String> {
        loop {
            let remaining = deadline.saturating_duration_since(std::time::Instant::now());
            let line = self.lines.recv_timeout(remaining).ok()?;
            if let Some((_, event)) = line.split_once("HOST: ") {
                return Some(event.to_owned());
            }
        }
    }

    fn signal(&self, signal: rustix::process::Signal) {
        rustix::process::kill_process(
            rustix::process::Pid::from_raw(i32::try_from(self.pid).expect("a process id"))
                .expect("a process id"),
            signal,
        )
        .expect("the host is signalled");
    }

    /// End the host if it is still running (a host that said nothing more is
    /// stuck: it is ended, not waited for), reap it, and end what it started
    /// before the readers wait for its pipes to close. Returns how it ended,
    /// what it printed to stderr, and what of its children was still alive
    /// when it ended.
    fn finish(mut self, stuck: bool) -> (std::process::ExitStatus, String, Vec<(i32, String)>) {
        if stuck {
            let _ = self.child.kill();
        }
        let status = self.child.wait().expect("the host exits");
        let alive = support::running(&format!("sleep 10000.{}", self.pid));
        support::end_all(&alive);
        self.reader.join().expect("the stdout reader");
        let errors = self.errors.join().expect("the stderr reader");
        (status, errors, alive)
    }
}

/// The environment variable that makes the host tests the hosts of the signal
/// cases; without it they do nothing.
#[cfg(unix)]
const HOST_ENV: &str = "LEXLEAN_END_CHILDREN_HOST";

/// A real SIGINT and a real SIGTERM reach a host process that registered its
/// own handling for them: the host ends the children from that handling's
/// thread, survives to run its own cleanup and to exit with its own status,
/// and the library installed and changed nothing.
#[test]
fn the_host_ends_children_from_its_own_signal_thread() {
    let _serial = serial();
    #[cfg(unix)]
    scenario_signals();
}

#[cfg(unix)]
fn scenario_signals() {
    use std::time::{Duration, Instant};

    let host = Host::start("the_host_process");
    let give_up = Instant::now() + Duration::from_secs(120);
    let mut rounds = 0;
    let outcome = loop {
        match host.next_event(give_up).as_deref() {
            Some(event) if event.starts_with("ready ") => {
                rounds += 1;
                host.signal(if rounds == 1 {
                    rustix::process::Signal::INT
                } else {
                    rustix::process::Signal::TERM
                });
            }
            Some("done") => break "done",
            Some(_) => {}
            None => break "the host said nothing more",
        }
    };
    let (status, errors, left) = host.finish(outcome != "done");
    assert!(
        outcome == "done" && status.success(),
        "the host must survive both signals, run its own cleanup, and exit by its own status, not by a signal (it said {outcome:?} and ended with {status:?})\n{errors}"
    );
    assert_eq!(rounds, 2, "one SIGINT and one SIGTERM");
    assert!(left.is_empty(), "the host's children survived: {left:?}");
}

/// The library installs nothing, on every host: a signal that the host did not
/// handle ends the host by its default action, and does not end the child
/// the library started. A handler that the library had installed for the
/// signals it watches in the executable (`SIGINT`, `SIGTERM`, `SIGHUP`,
/// `SIGQUIT`) would end the child first, and the host's death would then
/// be the same one, so what is asserted is the child: it is alive when the host
/// has died. This is what shows it where the host cannot report the signal
/// dispositions it would otherwise be compared by (macOS).
#[test]
fn a_signal_the_host_did_not_handle_ends_it_by_default() {
    let _serial = serial();
    #[cfg(unix)]
    scenario_unhandled();
}

#[cfg(unix)]
fn scenario_unhandled() {
    use std::os::unix::process::ExitStatusExt;
    use std::time::{Duration, Instant};

    let host = Host::start("the_unhandled_host_process");
    let give_up = Instant::now() + Duration::from_secs(120);
    let ready = host.next_event(give_up);
    let started = ready
        .as_deref()
        .is_some_and(|event| event.starts_with("ready "));
    if started {
        host.signal(rustix::process::Signal::QUIT);
    }
    let (status, errors, left) = host.finish(!started);
    assert!(
        started,
        "the host did not start its child: {ready:?}\n{errors}"
    );
    assert_eq!(
        status.signal(),
        Some(signal_hook::consts::SIGQUIT),
        "a signal the host did not handle ends it by the signal's default action: {status:?}\n{errors}"
    );
    assert_eq!(
        left.len(),
        1,
        "the library installed no handler, so it did not end the child when the host died of the signal: {left:?}"
    );
}

/// The host of [`the_host_ends_children_from_its_own_signal_thread`]: it
/// registers its own handling of SIGINT and SIGTERM, and for each signal that
/// reaches it ends the children from that handling's thread, then does its own
/// cleanup. It reads its signal dispositions before it starts a child and
/// after it has started one and after it has ended one, and they must be the
/// same: the library installs no handler and changes no disposition.
#[test]
fn the_host_process() {
    #[cfg(unix)]
    if std::env::var_os(HOST_ENV).is_some() {
        host_process();
    }
}

#[cfg(unix)]
fn host_process() {
    use std::io::Write;
    use std::time::{Duration, Instant};

    let say = |line: String| {
        let mut out = std::io::stdout().lock();
        writeln!(out, "HOST: {line}").expect("stdout");
        out.flush().expect("stdout");
    };
    let pid = std::process::id();
    let pgid = own_pgid();
    assert_eq!(
        pgid,
        i32::try_from(pid).expect("a process id"),
        "the host leads its own process group, so it can be signalled by mistake"
    );
    let marker = format!("sleep 10000.{pid}");
    // The host's own handling of SIGINT and SIGTERM: an ordinary thread fed
    // by the host's own registration, as a service handles shutdown.
    let mut signals = signal_hook::iterator::Signals::new([
        signal_hook::consts::SIGINT,
        signal_hook::consts::SIGTERM,
    ])
    .expect("the host registers its handler");
    let (outcomes_tx, outcomes_rx) = std::sync::mpsc::channel();
    let handler = std::thread::spawn(move || {
        for signal in signals.forever().take(2) {
            let _ = outcomes_tx.send((signal, lexlean::verify::end_live_children()));
        }
    });
    // What the host has set; the library may not change it.
    let dispositions = signal_dispositions();
    for round in 1..=2 {
        // The host's own cleanup between the two: re-enable starting.
        lexlean::verify::allow_children();
        let (mut child, guard) = spawn_sleep(10000);
        let child_pid = child.id();
        assert_dispositions_unchanged(dispositions, "starting a child");
        say(format!("ready {round} {pid} {child_pid}"));
        let (signal, outcome) = outcomes_rx
            .recv_timeout(Duration::from_secs(60))
            .expect("the signal reached the host's own handling");
        let expected = if round == 1 {
            signal_hook::consts::SIGINT
        } else {
            signal_hook::consts::SIGTERM
        };
        assert_eq!(signal, expected, "round {round}");
        assert!(
            outcome
                .iter()
                .all(|row| row.outcome == lexlean::verify::GroupEnd::Ended),
            "round {round}: every live group ended: {outcome:?}"
        );
        assert!(
            outcome.iter().any(|row| row.group == child_pid),
            "round {round}: the child was ended and reported: {outcome:?}"
        );
        let give_up = Instant::now() + Duration::from_secs(10);
        let mut left = support::running(&marker);
        while !left.is_empty() && Instant::now() < give_up {
            std::thread::sleep(Duration::from_millis(20));
            left = support::running(&marker);
        }
        support::end_all(&left);
        assert!(left.is_empty(), "round {round}: left running: {left:?}");
        let _ = child.wait();
        drop(guard);
        assert_dispositions_unchanged(dispositions, "ending the children");
        assert_eq!(pid, std::process::id(), "the host is untouched");
        assert_eq!(pgid, own_pgid(), "the host's group is untouched");
    }
    handler.join().expect("the handling thread");
    say("done".to_owned());
}

/// The host of [`a_signal_the_host_did_not_handle_ends_it_by_default`]: it
/// handles no signal, starts a child through the library, and waits to be
/// ended by the signal the case sends it. It lowers its core-file limit so
/// that `SIGQUIT`, whose default action dumps core, leaves no file.
#[test]
fn the_unhandled_host_process() {
    #[cfg(unix)]
    if std::env::var_os(HOST_ENV).is_some() {
        unhandled_host_process();
    }
}

#[cfg(unix)]
fn unhandled_host_process() {
    use std::io::Write;

    rustix::process::setrlimit(
        rustix::process::Resource::Core,
        rustix::process::Rlimit {
            current: Some(0),
            maximum: Some(0),
        },
    )
    .expect("the core-file limit is lowered");
    let pid = std::process::id();
    let (child, _guard) = spawn_sleep(10000);
    {
        let mut out = std::io::stdout().lock();
        writeln!(out, "HOST: ready 1 {pid} {}", child.id()).expect("stdout");
        out.flush().expect("stdout");
    }
    std::thread::sleep(std::time::Duration::from_secs(120));
    panic!("the signal the case sent did not end the host");
}

/// A call with no live child is a no-op, and a second call is idempotent.
#[test]
fn nothing_live_is_a_no_op_and_a_second_call_is_idempotent() {
    let _serial = serial();
    let _reenable = Reenable;
    let first = lexlean::verify::end_live_children();
    assert!(first.is_empty(), "nothing live is nothing ended: {first:?}");
    let second = lexlean::verify::end_live_children();
    assert!(
        second.is_empty(),
        "the second call ends nothing either: {second:?}"
    );
}

/// A child that already exited is reported as already gone, not as a
/// failure: it is waited and reaped, so its group has nothing left to
/// signal.
#[test]
fn an_exited_child_is_reported_gone_not_failed() {
    let _serial = serial();
    #[cfg(unix)]
    scenario_gone();
}

#[cfg(unix)]
fn scenario_gone() {
    use std::os::unix::process::CommandExt;

    let _reenable = Reenable;
    let mut command = std::process::Command::new("true");
    command.process_group(0).stdin(std::process::Stdio::null());
    let (mut child, guard) = lexlean::verify::lifeline::spawn(&mut command).expect("true starts");
    let pid = child.id();
    let status = child.wait().expect("the child is waited and reaped");
    assert!(status.success());
    let outcome = lexlean::verify::end_live_children();
    assert_eq!(
        outcome,
        vec![lexlean::verify::ChildGroupEnd {
            group: pid,
            outcome: lexlean::verify::GroupEnd::AlreadyGone,
        }],
        "an exited child is gone, not a failure"
    );
    drop(guard);
}

/// After the end, starting a child is refused with `LLV7018` until the host
/// re-enables it; then a child starts again.
#[test]
fn a_child_started_after_the_end_is_refused() {
    let _serial = serial();
    #[cfg(unix)]
    scenario_refused();
}

#[cfg(unix)]
fn scenario_refused() {
    let _reenable = Reenable;
    let _ = lexlean::verify::end_live_children();
    match spawn_sleep_or_refusal() {
        Err(lexlean::verify::lifeline::CannotStart::Shutdown) => {}
        Err(other) => panic!("the refusal is the shutdown state: {other:?}"),
        Ok(_) => panic!("a child started after the end must be refused"),
    }
    // Through the child pipeline the refusal is the registered diagnostic.
    let directory = tempfile::tempdir().expect("tempdir");
    let script = directory.path().join("lake");
    support::script(&script, "#!/bin/sh\nexit 0\n");
    let program = camino::Utf8Path::from_path(&script).expect("utf-8");
    let cwd = camino::Utf8Path::from_path(directory.path()).expect("utf-8");
    let spec = lexlean::verify::child::ChildSpec {
        tool: "lake",
        module: None,
        program,
        executable_sha256: lexlean::Sha256Digest::of(b"script"),
        argv: Vec::new(),
        cwd,
        extra_env: Vec::new(),
        home: lexlean::verify::child::ChildHome::Isolated { home: cwd },
    };
    let error = lexlean::verify::child::run(
        &spec,
        &limits(1000),
        &lexlean::verify::child::Normalizer::default(),
    )
    .expect_err("a child started after the end is refused");
    assert_eq!(error.code.as_str(), "LLV7018", "{error:?}");
    // The host re-enables explicitly, and a child starts again.
    lexlean::verify::allow_children();
    let mut command = std::process::Command::new("true");
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0).stdin(std::process::Stdio::null());
    }
    let (mut child, guard) =
        lexlean::verify::lifeline::spawn(&mut command).expect("a child starts once re-enabled");
    let _ = child.wait();
    drop(guard);
}

/// A `sleep` or the refusal to start it.
#[cfg(unix)]
fn spawn_sleep_or_refusal() -> Result<
    (std::process::Child, lexlean::verify::lifeline::Guard),
    lexlean::verify::lifeline::CannotStart,
> {
    use std::os::unix::process::CommandExt;
    let mut command = std::process::Command::new("sleep");
    command
        .arg(format!("10000.{}", std::process::id()))
        .process_group(0)
        .stdin(std::process::Stdio::null());
    lexlean::verify::lifeline::spawn(&mut command)
}

/// Children started from several threads while another thread ends the live
/// children again and again: each child is ended or refused, none survives.
///
/// The correct function passes every cycle deterministically, because the
/// shutdown state is set under the same lock that registers a child: when a
/// call returns, every child that was registered is ended and every later
/// start is refused, so the process table holds none of them. The cycles
/// exist for the defect, not for the function: a registration that releases
/// the lock between reading the shutdown state and registering the child
/// leaves a started, unregistered child alive at the instant a call returns,
/// and each cycle is an independent chance for the call to land in that
/// window (the defect is caught or missed per cycle, and over `CYCLES`
/// cycles the chance that it is missed every time is nil for practical
/// purposes), which is what "a planted removal of the registration lock
/// makes it fail" needs without a timing assumption for the passing case.
#[test]
fn concurrent_starts_are_ended_or_refused() {
    let _serial = serial();
    #[cfg(unix)]
    scenario_concurrent();
}

#[cfg(unix)]
fn scenario_concurrent() {
    use std::collections::BTreeSet;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    const CYCLES: usize = 60;
    const SPAWNERS: usize = 4;

    let _reenable = Reenable;
    let marker = format!("sleep 300.{}", std::process::id());
    let (registered_tx, registered_rx) = std::sync::mpsc::channel::<u32>();
    let stop = Arc::new(AtomicBool::new(false));
    let mut joins = Vec::new();
    for _ in 0..SPAWNERS {
        let registered_tx = registered_tx.clone();
        let stop = Arc::clone(&stop);
        joins.push(std::thread::spawn(move || {
            let mut mine: Vec<(std::process::Child, lexlean::verify::lifeline::Guard)> = Vec::new();
            let mut started: Vec<u32> = Vec::new();
            while !stop.load(Ordering::SeqCst) {
                match spawn_sleep_or_refusal_300() {
                    Ok((child, guard)) => {
                        let _ = registered_tx.send(child.id());
                        started.push(child.id());
                        mine.push((child, guard));
                    }
                    // Refused: the host has ended the children and not yet
                    // re-enabled new ones.
                    Err(lexlean::verify::lifeline::CannotStart::Shutdown) => {
                        std::thread::sleep(Duration::from_micros(100));
                    }
                    Err(other) => panic!("the spawn works or is refused: {other:?}"),
                }
                // A child the host has ended stays in the process table as a
                // zombie until its parent waits for it, and this loop starts
                // children as fast as the machine lets it: unreaped, the
                // zombies of sixty cycles exhaust a runner's process limit
                // (`EAGAIN` on macOS) and fail the case for a reason that
                // is not the function's. Reap the ones that have exited; the
                // ids stay in `started`, which is what the case checks.
                mine.retain_mut(|(child, _)| matches!(child.try_wait(), Ok(None)));
            }
            (mine, started)
        }));
    }

    let mut ended: BTreeSet<u32> = BTreeSet::new();
    for cycle in 0..CYCLES {
        // Starts are on again; wait until one has begun, so that the others
        // are in flight when the call lands.
        lexlean::verify::allow_children();
        registered_rx
            .recv_timeout(Duration::from_secs(30))
            .expect("a child starts once starting is re-enabled");
        let outcome = lexlean::verify::end_live_children();
        assert!(
            outcome
                .iter()
                .all(|row| row.outcome == lexlean::verify::GroupEnd::Ended),
            "cycle {cycle}: every registered group was live and is ended: {outcome:?}"
        );
        ended.extend(outcome.iter().map(|row| row.group));
        // The call has returned and starting is refused: no child that is
        // not a zombie may be left, registered or not.
        let left = support::running(&marker);
        support::end_all(&left);
        assert!(
            left.is_empty(),
            "cycle {cycle}: a child started concurrently with the call survived it: {left:?}"
        );
    }

    stop.store(true, Ordering::SeqCst);
    let mut spawned: Vec<(std::process::Child, lexlean::verify::lifeline::Guard)> = Vec::new();
    let mut started: Vec<u32> = Vec::new();
    for join in joins {
        let (alive, ids) = join.join().expect("the spawn thread");
        spawned.extend(alive);
        started.extend(ids);
    }
    // What was registered after the last call is ended by one more.
    let sweep = lexlean::verify::end_live_children();
    assert!(
        sweep
            .iter()
            .all(|row| row.outcome == lexlean::verify::GroupEnd::Ended),
        "every group of the sweep is ended: {sweep:?}"
    );
    ended.extend(sweep.iter().map(|row| row.group));
    for id in &started {
        assert!(
            ended.contains(id),
            "every started child is ended or was refused: {id} is neither"
        );
    }
    for (mut child, guard) in spawned {
        let _ = child.wait();
        drop(guard);
    }
    let left = support::running(&marker);
    support::end_all(&left);
    assert!(left.is_empty(), "a concurrent child survived: {left:?}");
}

/// A `sleep 300` or the refusal to start it.
#[cfg(unix)]
fn spawn_sleep_or_refusal_300() -> Result<
    (std::process::Child, lexlean::verify::lifeline::Guard),
    lexlean::verify::lifeline::CannotStart,
> {
    use std::os::unix::process::CommandExt;
    let mut command = std::process::Command::new("sleep");
    command
        .arg(format!("300.{}", std::process::id()))
        .process_group(0)
        .stdin(std::process::Stdio::null());
    lexlean::verify::lifeline::spawn(&mut command)
}

/// The limits of an ordinary project with the given child timeout.
#[cfg(unix)]
fn limits(timeout_ms: u64) -> lexlean::config::Limits {
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
        child_timeout_ms: timeout_ms,
    }
}
