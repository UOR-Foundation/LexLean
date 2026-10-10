//! An interrupt of `lexlean` ends the children it started (SPEC.md §17.17,
//! §22.3). Each child leads a process group of its own so that a timeout can
//! end what it started, which takes it out of the group the terminal's Ctrl-C
//! signals; the real binary is interrupted here, while a child is running,
//! and what the child started must be gone afterwards. A signal the process
//! was started ignoring (what `nohup` does to SIGHUP) stays ignored.

mod support;

/// SIGINT and SIGTERM everywhere; SIGQUIT (Ctrl-\) and SIGHUP too. On a host
/// without `/proc` the ignored signals are read from `ps`, which is not
/// available to this test to the same degree, so SIGHUP (which is left alone
/// when the host cannot say what is ignored) is exercised where `/proc` is.
#[test]
fn an_interrupt_leaves_no_child_running() {
    #[cfg(unix)]
    {
        let mut signals = vec![("INT", 1000), ("TERM", 2000), ("QUIT", 3000)];
        if std::path::Path::new("/proc/self").exists() {
            signals.push(("HUP", 4000));
        }
        for (signal, seconds) in signals {
            // `lexlean` keeps a signal ignored that its parent ignored, as it
            // must, and these runs start it from this process: a test run in
            // the background of a non-interactive shell ignores SIGINT and
            // SIGQUIT, and the run then proves nothing about handling them.
            if ignored_here(signal) {
                eprintln!("SIG{signal} is ignored by the test process; its case did not run");
                continue;
            }
            scenario(signal, seconds, None);
        }
    }
}

/// A signal that was ignored when `lexlean` began stays ignored: the run is
/// not ended by it and its child keeps running, and the signals that were not
/// ignored still end both.
#[test]
fn an_ignored_signal_stays_ignored() {
    #[cfg(unix)]
    {
        // `nohup` ignores SIGHUP, and every host reads that one.
        scenario("TERM", 5000, Some("HUP"));
        if std::path::Path::new("/proc/self").exists() {
            // The background job of a non-interactive shell ignores these.
            scenario("TERM", 6000, Some("INT"));
            scenario("TERM", 7000, Some("QUIT"));
        }
    }
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

/// Whether this process was started ignoring the signal (`/proc` only: where
/// there is none, nothing is said to be ignored).
#[cfg(unix)]
fn ignored_here(name: &str) -> bool {
    let number = match name {
        "INT" => 2,
        "QUIT" => 3,
        "HUP" => 1,
        _ => 15,
    };
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|status| {
            status
                .lines()
                .find_map(|line| u64::from_str_radix(line.strip_prefix("SigIgn:")?.trim(), 16).ok())
        })
        .is_some_and(|mask| mask & (1_u64 << (number - 1)) != 0)
}

#[cfg(unix)]
fn signal_of(name: &str) -> (rustix::process::Signal, i32) {
    match name {
        "INT" => (rustix::process::Signal::INT, 2),
        "QUIT" => (rustix::process::Signal::QUIT, 3),
        "HUP" => (rustix::process::Signal::HUP, 1),
        _ => (rustix::process::Signal::TERM, 15),
    }
}

/// Run `lexlean lock --allow-network` with a `git` that starts a long-running
/// child of its own, as `lake` starts `lean`, and never finishes; send the
/// process `which` once the child runs. With `ignoring`, the process is
/// started with that signal ignored, as `nohup` does, and is sent it first:
/// it must neither end nor lose its child.
#[cfg(unix)]
fn scenario(which: &str, seconds: u32, ignoring: Option<&str>) {
    use std::os::unix::process::ExitStatusExt;
    use std::time::{Duration, Instant};

    let Some(root) = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .map(std::path::Path::to_path_buf)
        .filter(|root| root.join("examples/nat-add-zero/lexlean.toml").is_file())
    else {
        eprintln!("the crate is being tested outside its repository; the interrupt did not run");
        return;
    };
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
            &format!(
                "\n[[lexicon_source]]\npackage = \"test.remote\"\nkind = \"git\"\nurl = \"https://example.invalid/repo.git\"\nrevision = \"{}\"\nsubdirectory = \"lexicon\"\n\n[limits]",
                "0123456789abcdef0123456789abcdef01234567"
            ),
        ),
    )
    .expect("write");
    let bin = directory.path().join("bin");
    std::fs::create_dir(&bin).expect("mkdir");
    // The decimal part is this process, so no other run is mistaken for it.
    let marker = format!("sleep {seconds}.{}", std::process::id());
    support::script(
        &bin.join("git"),
        &format!(
            "#!/bin/sh\nsleep {seconds}.{} &\nwait\n",
            std::process::id()
        ),
    );
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let lexlean_bin = env!("CARGO_BIN_EXE_lexlean");
    let mut command = match ignoring {
        // `sh` ignores the signal and replaces itself with `lexlean`, which
        // inherits the disposition, exactly as `nohup lexlean ...` does.
        Some(ignored) => {
            let mut command = std::process::Command::new("/bin/sh");
            command.args([
                "-c",
                &format!("trap '' {ignored}; exec \"$0\" \"$@\""),
                lexlean_bin,
                "lock",
                "--allow-network",
            ]);
            command
        }
        None => {
            let mut command = std::process::Command::new(lexlean_bin);
            command.args(["lock", "--allow-network"]);
            command
        }
    };
    let mut lexlean = command
        .current_dir(&project)
        .env("PATH", path)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("lexlean starts");
    let give_up = Instant::now() + Duration::from_secs(60);
    while support::running(&marker).is_empty() {
        assert!(
            Instant::now() < give_up && lexlean.try_wait().expect("wait").is_none(),
            "lexlean never started the child"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    let pid = rustix::process::Pid::from_raw(i32::try_from(lexlean.id()).expect("pid"))
        .expect("a process id");
    if let Some(ignored) = ignoring {
        // Give the signal time to do harm if it is going to.
        rustix::process::kill_process(pid, signal_of(ignored).0).expect("signal lexlean");
        std::thread::sleep(Duration::from_millis(1500));
        let ended = lexlean.try_wait().expect("wait");
        let left = support::running(&marker);
        if ended.is_some() || left.is_empty() {
            let _ = lexlean.kill();
            let _ = lexlean.wait();
            support::end_all(&left);
            panic!("SIG{ignored} was ignored at start, yet lexlean ended ({ended:?}) or lost its child ({left:?})");
        }
    }
    // The child is in a group of its own, which the terminal would not signal.
    let (signal, number) = signal_of(which);
    rustix::process::kill_process(pid, signal).expect("signal lexlean");
    let give_up = Instant::now() + Duration::from_secs(20);
    let status = loop {
        if let Some(status) = lexlean.try_wait().expect("wait") {
            break status;
        }
        if Instant::now() >= give_up {
            let _ = lexlean.kill();
            let _ = lexlean.wait();
            support::end_all(&support::running(&marker));
            panic!("lexlean did not end after SIG{which}");
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let give_up = Instant::now() + Duration::from_secs(10);
    let mut left = support::running(&marker);
    while !left.is_empty() && Instant::now() < give_up {
        std::thread::sleep(Duration::from_millis(20));
        left = support::running(&marker);
    }
    support::end_all(&left);
    assert_eq!(
        status.signal(),
        Some(number),
        "SIG{which} ends lexlean by the signal itself: {status:?}"
    );
    assert!(left.is_empty(), "SIG{which} left running: {left:?}");
}
