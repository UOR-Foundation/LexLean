//! A host that embeds the library keeps its own signals (SPEC.md §17.17). The
//! compiler ends its children when it is interrupted only as the executable;
//! `lexlean::cli::run` called by another program starts children without
//! watching for any signal, so the host's `SIGTERM` handling, installed
//! beside it, is the only one and the host is not ended by this crate. The
//! case is its own test binary because it replaces `PATH` and takes `SIGTERM`.

mod support;

#[test]
fn an_embedded_run_leaves_the_hosts_signals_alone() {
    #[cfg(unix)]
    scenario();
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
fn scenario() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
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
    // The host's own handling of SIGTERM: a flag, as a service sets one to
    // shut down in order.
    let terminated = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&terminated))
        .expect("the host registers its handler");

    let directory = tempfile::tempdir().expect("tempdir");
    let project = directory.path().join("project");
    copy_tree(&root.join("examples/nat-add-zero"), &project);
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
    // A `git` that records that it ran, takes a moment, and fails: the run
    // starts a child, and has ended by the time the host is signalled.
    let bin = directory.path().join("bin");
    std::fs::create_dir(&bin).expect("mkdir");
    let ran = directory.path().join("git-ran");
    support::script(
        &bin.join("git"),
        &format!("#!/bin/sh\ntouch '{}'\nsleep 0.3\nexit 1\n", ran.display()),
    );
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
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let _ = lexlean::cli::run(&arguments, &working, &mut out, &mut err);
    assert!(
        ran.is_file(),
        "the run started a child, so the case is not vacuous"
    );

    // No child is alive. A SIGTERM now reaches the host's handler and nothing
    // else: if this crate had installed one, the process would end here.
    rustix::process::kill_process(rustix::process::getpid(), rustix::process::Signal::TERM)
        .expect("signal this process");
    let give_up = Instant::now() + Duration::from_secs(10);
    while !terminated.load(Ordering::SeqCst) {
        assert!(
            Instant::now() < give_up,
            "the host's own handler never saw SIGTERM"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    // Still here, and still able to run the library.
    std::thread::sleep(Duration::from_millis(300));
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let _ = lexlean::cli::run(&arguments, &working, &mut out, &mut err);
}
