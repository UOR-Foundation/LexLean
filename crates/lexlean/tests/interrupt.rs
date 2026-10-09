//! An interrupt of `lexlean` ends the children it started (SPEC.md §17.17,
//! §22.3). Each child leads a process group of its own so that a timeout can
//! end what it started, which takes it out of the group the terminal's Ctrl-C
//! signals; the real binary is interrupted here, while a child is running,
//! and what the child started must be gone afterwards.

mod support;

#[test]
fn an_interrupt_leaves_no_child_running() {
    #[cfg(unix)]
    for (signal, seconds) in [("SIGINT", 1000), ("SIGTERM", 2000)] {
        scenario(signal, seconds);
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

#[cfg(unix)]
fn scenario(which: &str, seconds: u32) {
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
    // A git source that must be acquired: `lock --allow-network` runs `git`,
    // which here starts a long-running child of its own, as `lake` starts
    // `lean`, and never finishes.
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
    let mut lexlean = std::process::Command::new(env!("CARGO_BIN_EXE_lexlean"))
        .args(["lock", "--allow-network"])
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
    // The child is in a group of its own, which the terminal would not signal.
    let pid = rustix::process::Pid::from_raw(i32::try_from(lexlean.id()).expect("pid"))
        .expect("a process id");
    let signal = match which {
        "SIGINT" => rustix::process::Signal::INT,
        _ => rustix::process::Signal::TERM,
    };
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
            panic!("lexlean did not end after {which}");
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
    let expected = match which {
        "SIGINT" => 2,
        _ => 15,
    };
    assert_eq!(
        status.signal(),
        Some(expected),
        "{which} ends lexlean by the signal itself: {status:?}"
    );
    assert!(left.is_empty(), "{which} left running: {left:?}");
}
