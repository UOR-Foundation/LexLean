//! A timeout ends what the child started whether or not the host has a
//! `kill` executable: the process group is signalled directly, not by
//! running a tool found on `PATH` (SPEC.md §17.17, §22.3). The case is its
//! own test binary because it replaces `PATH` for the whole process.

mod support;

#[test]
fn a_timeout_ends_the_group_on_a_host_without_kill() {
    #[cfg(unix)]
    scenario();
}

#[cfg(unix)]
fn scenario() {
    use camino::Utf8Path;
    use lexlean::verify::child::{resolve_on_path, run, ChildHome, ChildSpec, Normalizer};

    let directory = tempfile::tempdir().expect("tempdir");
    let bin = directory.path().join("bin");
    std::fs::create_dir(&bin).expect("mkdir");
    // `sleep` is all the script needs of the host; `kill` is not provided.
    let sleep = resolve_on_path("sleep").expect("sleep");
    std::os::unix::fs::symlink(sleep.as_std_path(), bin.join("sleep")).expect("symlink");
    let marker = format!("sleep 1000.{}", std::process::id());
    let script = directory.path().join("lake");
    support::script(
        &script,
        &format!("#!/bin/sh\nsleep 1000.{} &\nwait\n", std::process::id()),
    );
    std::env::set_var("PATH", &bin);
    // The case is vacuous if the host can find a `kill` anyway.
    assert!(
        resolve_on_path("kill").is_err(),
        "the test PATH must not provide kill"
    );

    let program = Utf8Path::from_path(&script).expect("utf-8");
    let cwd = Utf8Path::from_path(directory.path()).expect("utf-8");
    let spec = ChildSpec {
        tool: "lake",
        module: Some("Probe".to_owned()),
        program,
        executable_sha256: lexlean::Sha256Digest::of(b"script"),
        argv: Vec::new(),
        cwd,
        extra_env: Vec::new(),
        home: ChildHome::Isolated { home: cwd },
    };
    let error =
        run(&spec, &limits(700), &Normalizer::default()).expect_err("the child outlives 700 ms");
    let left = support::running(&marker);
    support::end_all(&left);
    assert_eq!(error.code.as_str(), "LLS8002", "{error:?}");
    assert!(
        error.message.contains("child_timeout_ms exceeded"),
        "{error:?}"
    );
    assert!(
        !error.message.contains("may still be running"),
        "the group was ended, so the report does not claim otherwise: {error:?}"
    );
    assert!(left.is_empty(), "a timeout left running: {left:?}");
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
