//! The `cli-api` suite, migration: CL-24 (SPEC.md §17.12, §23.4).

use std::collections::BTreeMap;

use camino::Utf8Path;

use crate::support::{self, P};

/// Every file under `root` with its bytes, keyed by its project-relative path.
fn snapshot(root: &Utf8Path) -> BTreeMap<String, Vec<u8>> {
    support::file_set(root)
        .into_iter()
        .map(|relative| {
            let bytes = std::fs::read(root.join(&relative).as_std_path()).expect("a project file");
            (relative, bytes)
        })
        .collect()
}

/// The project as `migrate` leaves it when it refuses: every file as it was
/// and no build root created beside them.
fn assert_untouched(project: &P, before: &BTreeMap<String, Vec<u8>>, context: &str) {
    let after = snapshot(&project.root);
    assert_eq!(
        before.keys().collect::<Vec<_>>(),
        after.keys().collect::<Vec<_>>(),
        "{context}: no file was created or removed"
    );
    for (relative, bytes) in before {
        assert_eq!(
            Some(bytes),
            after.get(relative),
            "{context}: `{relative}` was changed"
        );
    }
}

/// A language-1.1 path lexicon package with one entry, registered in the
/// project: the package a migration must move to language 1.2 with the project.
fn add_path_lexicon(project: &P) {
    project.write_raw(
        "lexicons/migrated/lexicon.toml",
        "spec = \"lexlean/lexicon/1\"\npackage = \"test.migrated\"\nversion = \"1.0.0\"\nlanguage = \"1.1\"\nimports = [\"lexlean.core@1.1.0\", \"lexlean.std.nat@1.1.0\"]\n",
    );
    project.write_raw(
        "lexicons/migrated/entries/word.toml",
        &support::label_word_entry("word", "migrated word"),
    );
    project.add_lexicon_source("test.migrated", "lexicons/migrated");
    project.relock();
}

fn assert_refused(project: &P, arguments: &[&str], exit: i32, code: &str, context: &str) {
    let before = snapshot(&project.root);
    let (status, stdout, stderr) = project.cli(arguments);
    assert_eq!(status, exit, "{context}: stdout {stdout}\nstderr {stderr}");
    assert!(
        stderr.contains(&format!("[{code}]")),
        "{context}: expected {code} in {stderr}"
    );
    assert_untouched(project, &before, context);
}

pub(crate) fn cli() {
    let project = P::semantic_example();
    add_path_lexicon(&project);
    let sources: Vec<String> = support::file_set(&project.root)
        .into_iter()
        .filter(|relative| relative.ends_with(".lex.tex"))
        .collect();
    assert_eq!(sources.len(), 11, "the example's modules");
    let old: BTreeMap<String, serde_json::Value> = sources
        .iter()
        .map(|relative| {
            let module = lexlean::sdk::read_module(&project.read_raw(relative))
                .expect("a language-1.1 module reads");
            (relative.clone(), module)
        })
        .collect();

    // `--check` proves the migration and writes nothing.
    let before = snapshot(&project.root);
    let (status, stdout, stderr) = project.cli(&["migrate", "--to", "1.2", "--check"]);
    assert_eq!(status, 0, "stdout {stdout}\nstderr {stderr}");
    assert_eq!(
        stdout, "11 modules and 1 lexicon package migrate to language 1.2; nothing was written\n",
        "--check reports what it would migrate"
    );
    assert_untouched(&project, &before, "migrate --check");

    // One command migrates the whole project.
    let (status, stdout, stderr) = project.cli(&["migrate", "--to", "1.2"]);
    assert_eq!(status, 0, "stdout {stdout}\nstderr {stderr}");
    assert_eq!(
        stdout,
        "migrated 11 modules and 1 lexicon package to language 1.2; wrote lexlean.lock\n"
    );
    let config = project.read_raw("lexlean.toml");
    assert!(config.contains("language = \"1.2\"\n"), "{config}");
    let lock = project.read_raw("lexlean.lock");
    assert!(
        lock.starts_with("spec = \"lexlean/lock/2\"\nlanguage = \"1.2\"\n"),
        "{lock}"
    );
    let manifest = project.read_raw("lexicons/migrated/lexicon.toml");
    assert_eq!(
        manifest,
        "spec = \"lexlean/lexicon/1\"\npackage = \"test.migrated\"\nversion = \"1.0.0\"\nlanguage = \"1.2\"\nimports = [\"lexlean.core@1.2.0\", \"lexlean.std.nat@1.2.0\"]\n"
    );
    for relative in &sources {
        let text = project.read_raw(relative);
        assert!(
            !text.contains("\\semanticdata") && !text.contains("@1.1.0"),
            "`{relative}` is written in the notation against the 1.2.0 glossaries"
        );
        assert!(
            text.contains("\\useglossary{lexlean.std.nat@1.2.0}"),
            "`{relative}` keeps its glossary, moved to 1.2.0"
        );
        // The same declarations, names, statements, and axiom policies: the
        // notation reads to the language-1.1 module under the 1.2 tag.
        let mut expected = old[relative].clone();
        expected["spec"] = serde_json::json!("lexlean/semantic-module/2");
        assert_eq!(
            lexlean::sdk::read_module(&text).expect("the notation reads"),
            expected,
            "`{relative}` reads to the module it was"
        );
    }

    // The result is a language-1.2 project: it checks, its lock is current,
    // and its source is canonical.
    project.check_ok();
    for arguments in [
        &["lock", "--check"][..],
        &["fmt", "--check", "--all"][..],
        &["check", "--all"][..],
    ] {
        let (status, stdout, stderr) = project.cli(arguments);
        assert_eq!(status, 0, "{arguments:?}: stdout {stdout}\nstderr {stderr}");
    }

    // A project that is not language 1.1 is refused, nothing written.
    assert_refused(
        &project,
        &["migrate", "--to", "1.2"],
        2,
        "LLC0103",
        "a migrated project",
    );
    assert_refused(
        &P::language_1_2_example(),
        &["migrate", "--to", "1.2"],
        2,
        "LLC0103",
        "the language-1.2 example",
    );

    // A module the notation cannot express is refused with a named
    // diagnostic, and nothing is written.
    assert_refused(
        &P::negative("migrate-module-without-notation"),
        &["migrate", "--to", "1.2"],
        1,
        "LLP2003",
        "a module the notation cannot express",
    );
    assert_refused(
        &P::semantic_example(),
        &["migrate", "--to", "1.1"],
        2,
        "LLC0001",
        "a target other than 1.2",
    );
}
