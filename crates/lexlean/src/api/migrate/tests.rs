//! The publication and the proof-to-lock window of the migration (§17.12).

use std::path::Path;

use camino::Utf8PathBuf;

use super::{publish, rewrites, rollback, stage, write, Staged};
use crate::api::Engine;

fn utf8(path: &Path) -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(path.to_path_buf()).expect("a UTF-8 temporary path")
}

/// Copy `from` to `to`, leaving out the build root and the expected oracles.
fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("a directory");
    for entry in std::fs::read_dir(from).expect("a readable directory") {
        let entry = entry.expect("a directory entry");
        let name = entry.file_name();
        if name == ".lexlean" || name == "expected" {
            continue;
        }
        let target = to.join(&name);
        if entry.file_type().expect("a file type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("a copied file");
        }
    }
}

/// A fresh copy of the committed language-1.1 example, loaded.
fn example() -> (tempfile::TempDir, Engine) {
    let temp = tempfile::tempdir().expect("a temporary directory");
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/semantic-1.1");
    copy_tree(&source, temp.path());
    let engine = Engine::load(&utf8(temp.path()).join("lexlean.toml")).expect("the example loads");
    (temp, engine)
}

/// A staged replacement of `target` with `bytes`, whose temporary lives in
/// `target`'s directory when it has one.
fn staged(target: &Utf8PathBuf, bytes: &[u8], original: Option<&[u8]>) -> Staged {
    let mut temp = tempfile::NamedTempFile::new_in(
        target
            .parent()
            .filter(|parent| parent.is_dir())
            .expect("a parent")
            .as_std_path(),
    )
    .expect("a temporary file");
    std::io::Write::write_all(&mut temp, bytes).expect("written");
    Staged {
        target: target.clone(),
        shown: target.file_name().expect("a file name").to_owned(),
        temp,
        original: original.map(<[u8]>::to_vec),
    }
}

#[test]
fn a_failed_publish_restores_every_file_already_published() {
    let dir = tempfile::tempdir().expect("a temporary directory");
    let root = utf8(dir.path());
    let first = root.join("first.txt");
    let second = root.join("second.txt");
    let created = root.join("created.txt");
    std::fs::write(&first, b"first before").expect("written");
    std::fs::write(&second, b"second before").expect("written");
    // The third target is beneath a directory that does not exist, so its
    // rename fails after the first three were published.
    let unreachable = root.join("missing").join("third.txt");
    let entries = vec![
        staged(&first, b"first after", Some(b"first before")),
        staged(&second, b"second after", Some(b"second before")),
        staged(&created, b"created after", None),
        Staged {
            target: unreachable.clone(),
            shown: "missing/third.txt".to_owned(),
            temp: staged(&root.join("holder.txt"), b"third after", None).temp,
            original: None,
        },
    ];
    let failure = publish(entries)
        .expect_err("the last publish fails")
        .to_string();
    assert!(failure.contains("missing/third.txt: publish"), "{failure}");
    assert!(!failure.contains("partly migrated"), "{failure}");
    assert_eq!(std::fs::read(&first).expect("read"), b"first before");
    assert_eq!(std::fs::read(&second).expect("read"), b"second before");
    assert!(!created.exists(), "a file the migration created is removed");
}

#[test]
fn every_restore_is_attempted_and_every_failure_reported() {
    let dir = tempfile::tempdir().expect("a temporary directory");
    let root = utf8(dir.path());
    let restorable = root.join("restorable.txt");
    std::fs::write(&restorable, b"migrated").expect("written");
    // Listed in the order published; restored last first. The newest and the
    // oldest cannot be restored (the directory they lie in is gone), and the
    // one between them can: a failure must not stop it.
    let gone = root.join("gone");
    let published = vec![
        (gone.join("oldest.txt"), Some(b"before".to_vec())),
        (restorable.clone(), Some(b"original".to_vec())),
        (gone.join("newest.txt"), None),
    ];
    let failures = rollback(published);
    assert_eq!(failures.len(), 2, "{failures:?}");
    assert!(failures[0].contains("newest.txt"), "{failures:?}");
    assert!(failures[1].contains("oldest.txt"), "{failures:?}");
    assert_eq!(
        std::fs::read(&restorable).expect("read"),
        b"original",
        "the restore between two failures was still made"
    );
}

#[test]
fn a_file_changed_after_the_proof_is_refused_and_nothing_is_written() {
    let (temp, engine) = example();
    let root = utf8(temp.path());
    let plan = rewrites(&engine).expect("the example's migration is derived");
    let module = plan.modules.first().expect("a module");
    let relative = engine
        .project()
        .all_modules()
        .expect("modules")
        .into_iter()
        .find_map(|(name, path)| (&name == module).then_some(path))
        .expect("the module's file");
    let edited = format!("{}\n% edited after the proof\n", {
        std::fs::read_to_string(root.join(&relative)).expect("the module")
    });
    std::fs::write(root.join(&relative), &edited).expect("written");
    let before: Vec<(String, Vec<u8>)> = plan
        .files
        .keys()
        .map(|file| (file.clone(), std::fs::read(root.join(file)).expect("read")))
        .collect();
    let build_root = root.join(&engine.project().config.build_root);
    assert!(
        !build_root.exists(),
        "the example starts without a build root"
    );
    let failure = write(&engine, &plan.files, &plan.inputs)
        .expect_err("the stale proof is refused")
        .to_string();
    assert!(failure.contains("LLV7010"), "{failure}");
    assert!(
        !build_root.exists(),
        "a refusal leaves no build root it created"
    );
    assert!(
        failure.contains(&format!(
            "{relative}: changed since the migration was proven"
        )),
        "{failure}"
    );
    for (file, bytes) in before {
        assert_eq!(
            std::fs::read(root.join(&file)).expect("read"),
            bytes,
            "`{file}` was left as it was"
        );
    }
    assert_eq!(
        std::fs::read_to_string(root.join(&relative)).expect("read"),
        edited,
        "the edit made after the proof is still there"
    );
}

#[test]
fn a_project_unchanged_since_the_proof_is_written() {
    let (temp, engine) = example();
    let root = utf8(temp.path());
    let plan = rewrites(&engine).expect("the example's migration is derived");
    write(&engine, &plan.files, &plan.inputs).expect("the proven files are published");
    for (file, bytes) in &plan.files {
        assert_eq!(
            &std::fs::read(root.join(file)).expect("read"),
            bytes,
            "{file}"
        );
    }
}

#[test]
fn a_staged_file_whose_original_vanished_is_refused() {
    let (temp, engine) = example();
    let root = utf8(temp.path());
    let plan = rewrites(&engine).expect("the example's migration is derived");
    let (relative, digest) = plan.inputs.iter().next().expect("an input");
    std::fs::remove_file(root.join(relative)).expect("removed");
    let failure = stage(&engine, relative, b"new", Some(digest))
        .err()
        .expect("a vanished original is a change")
        .to_string();
    assert!(
        failure.contains("changed since the migration was proven"),
        "{failure}"
    );
}

#[test]
fn a_refusal_keeps_a_build_root_that_was_already_there() {
    let (temp, engine) = example();
    let root = utf8(temp.path());
    let build_root = root.join(&engine.project().config.build_root);
    std::fs::create_dir_all(&build_root).expect("a build root");
    std::fs::write(build_root.join("kept"), b"kept").expect("written");
    let plan = rewrites(&engine).expect("the example's migration is derived");
    let relative = plan.inputs.keys().next().expect("a proven file").clone();
    std::fs::write(root.join(&relative), b"changed after the proof").expect("written");
    write(&engine, &plan.files, &plan.inputs).expect_err("the stale proof is refused");
    assert_eq!(
        std::fs::read(build_root.join("kept")).expect("read"),
        b"kept",
        "a build root the migration did not create is left alone"
    );
}
