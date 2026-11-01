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

/// Whether the project has a build root: what `--check` and a refusal must not
/// create.
fn has_build_root(project: &P) -> bool {
    project.root.join(".lexlean").exists()
}

/// The project as `migrate` leaves it when it refuses: every file as it was
/// and no build root created beside them.
fn assert_untouched(
    project: &P,
    before: &BTreeMap<String, Vec<u8>>,
    had_root: bool,
    context: &str,
) {
    assert_eq!(
        had_root,
        has_build_root(project),
        "{context}: the build root was {}",
        if had_root { "removed" } else { "created" }
    );
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

/// A path lexicon package with one entry, registered in the project but not
/// locked, whose manifest is `manifest`.
fn register_path_lexicon(project: &P, manifest: &str) {
    project.write_raw("lexicons/migrated/lexicon.toml", manifest);
    project.write_raw(
        "lexicons/migrated/entries/word.toml",
        &support::label_word_entry("word", "migrated word"),
    );
    project.add_lexicon_source("test.migrated", "lexicons/migrated");
}

const MANIFEST_1_1: &str = "spec = \"lexlean/lexicon/1\"\npackage = \"test.migrated\"\nversion = \"1.0.0\"\nlanguage = \"1.1\"\nimports = [\"lexlean.core@1.1.0\", \"lexlean.std.nat@1.1.0\"]\n";

/// A language-1.1 path lexicon package with one entry, registered and locked:
/// the package a migration must move to language 1.2 with the project. The
/// relock creates the build root, which is removed, so that what `--check` and
/// a refusal must not create is absent before them.
fn add_path_lexicon(project: &P) {
    register_path_lexicon(project, MANIFEST_1_1);
    project.relock();
    std::fs::remove_dir_all(project.root.join(".lexlean").as_std_path())
        .expect("the build root the relock created");
}

fn assert_refused(project: &P, arguments: &[&str], exit: i32, code: &str, context: &str) {
    let before = snapshot(&project.root);
    let had_root = has_build_root(project);
    let (status, stdout, stderr) = project.cli(arguments);
    assert_eq!(status, exit, "{context}: stdout {stdout}\nstderr {stderr}");
    assert!(
        stderr.contains(&format!("[{code}]")),
        "{context}: expected {code} in {stderr}"
    );
    assert_untouched(project, &before, had_root, context);
}

/// The project-relative paths whose bytes differ between two snapshots, or
/// that exist in one only.
fn changed(before: &BTreeMap<String, Vec<u8>>, after: &BTreeMap<String, Vec<u8>>) -> Vec<String> {
    before
        .keys()
        .chain(after.keys())
        .filter(|relative| !relative.starts_with(".lexlean/"))
        .filter(|relative| before.get(*relative) != after.get(*relative))
        .cloned()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
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
    assert!(!has_build_root(&project), "the example has no build root");
    let (status, stdout, stderr) = project.cli(&["migrate", "--to", "1.2", "--check"]);
    assert_eq!(status, 0, "stdout {stdout}\nstderr {stderr}");
    assert_eq!(
        stdout, "11 modules and 1 lexicon package migrate to language 1.2; nothing was written\n",
        "--check reports what it would migrate"
    );
    assert_untouched(&project, &before, false, "migrate --check");

    // One command migrates the whole project, and its files are replaced
    // together: every module, the manifest, the configuration, and the lock
    // are rewritten, no other file is touched, and no staging file is left.
    let (status, stdout, stderr) = project.cli(&["migrate", "--to", "1.2"]);
    assert_eq!(status, 0, "stdout {stdout}\nstderr {stderr}");
    let mut expected_changes: Vec<String> = sources.clone();
    expected_changes.extend(
        [
            "lexicons/migrated/lexicon.toml",
            "lexlean.toml",
            "lexlean.lock",
        ]
        .map(str::to_owned),
    );
    expected_changes.sort();
    assert_eq!(
        changed(&before, &snapshot(&project.root)),
        expected_changes,
        "the migration rewrote exactly the modules, the manifest, the configuration, and the lock"
    );
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

    // A module that is not a language-1.1 module (the fixture has a top-level
    // member the schema forbids, which the notation cannot spell) is refused
    // with a named diagnostic, and nothing is written.
    assert_refused(
        &P::negative("migrate-module-without-notation"),
        &["migrate", "--to", "1.2"],
        1,
        "LLP2003",
        "a module that is not a language-1.1 module",
    );
    // So is a module that is not a semantic one.
    let tooling = P::semantic_example();
    tooling.write_raw(
        "src/Support.lex.tex",
        "\\begin{lexlean}{Support}\n\\begin{toolingmodule}\n\\toolingdata{{\"declarations\":[],\"spec\":\"lexlean/tooling-module/1\"}}\n\\end{toolingmodule}\n\\end{lexlean}\n",
    );
    assert_refused(
        &tooling,
        &["migrate", "--to", "1.2"],
        1,
        "LLP2003",
        "a module that is not semantic",
    );
    // A data form that language 1.1 `check` refuses as not canonical (the same
    // module, pretty-printed, which 1.1 reads past, with one member name
    // written as a `\u` escape, which it does not) is refused too, and nothing
    // is written.
    let pretty = P::semantic_example();
    let source = "src/KernelReductionDefinitions.lex.tex";
    let text = pretty.read_raw(source);
    let opening = "\\semanticdata{";
    let closing = "}\n\\end{semanticmodule}";
    let start = text.find(opening).expect("the data form") + opening.len();
    let end = text.find(closing).expect("the end of the data form");
    let module = lexlean::sdk::read_module(&text).expect("a language-1.1 module reads");
    pretty.write_raw(
        source,
        &format!(
            "{}{}{}",
            &text[..start],
            serde_json::to_string_pretty(&module)
                .expect("the module prints")
                .replacen("\"name\"", "\"\\u006eame\"", 1),
            &text[end..]
        ),
    );
    let (status, _, stderr) = pretty.cli(&["check", "--all"]);
    assert_eq!(status, 1, "1.1 check refuses it: {stderr}");
    assert!(stderr.contains("[LLT4001]"), "{stderr}");
    for arguments in [
        &["migrate", "--to", "1.2"][..],
        &["migrate", "--to", "1.2", "--check"][..],
    ] {
        assert_refused(
            &pretty,
            arguments,
            1,
            "LLP2003",
            "a data form that is not canonical JSON",
        );
    }
    // A path lexicon manifest that is not language 1.1 is refused with
    // LLC0103, in the migration and in its `--check`.
    let manifest = P::semantic_example();
    register_path_lexicon(
        &manifest,
        &MANIFEST_1_1.replace("language = \"1.1\"", "language = \"1.2\""),
    );
    for arguments in [
        &["migrate", "--to", "1.2"][..],
        &["migrate", "--to", "1.2", "--check"][..],
    ] {
        assert_refused(
            &manifest,
            arguments,
            2,
            "LLC0103",
            "a language-1.2 path lexicon manifest",
        );
    }
    // A migration whose scratch copy does not check as language 1.2 is refused
    // with the diagnostics of that check: the manifest imports a package that
    // does not exist, which migration leaves as it is and the scratch lock
    // cannot resolve.
    let unresolved = P::semantic_example();
    register_path_lexicon(
        &unresolved,
        &MANIFEST_1_1.replace("lexlean.std.nat@1.1.0", "lexlean.std.nat@9.9.9"),
    );
    for arguments in [
        &["migrate", "--to", "1.2"][..],
        &["migrate", "--to", "1.2", "--check"][..],
    ] {
        assert_refused(
            &unresolved,
            arguments,
            1,
            "LLR3001",
            "a scratch copy that fails its 1.2 check",
        );
    }
    assert_refused(
        &P::semantic_example(),
        &["migrate", "--to", "1.1"],
        2,
        "LLC0001",
        "a target other than 1.2",
    );
}
