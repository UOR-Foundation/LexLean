//! The `mathlib` suite: ML-01..ML-04, an external Lean library pinned in the
//! configuration and the lock, acquired only by `lexlean deps fetch`
//! (SPEC.md §10.4, §11.4, §23.4).

use lexlean::artifact::content_id::Sha256Digest;
use lexlean::{
    BuildRequest, CheckRequest, Engine, FormatRequest, LexLeanError, LockRequest, Selection,
    VerifyRequest,
};

use crate::support::{self, P};

const URL: &str = "https://github.com/leanprover-community/mathlib4";
const REVISION: &str = "520045ab14e26149ee970e2e617ca04b09bde5d6";
const OTHER_REVISION: &str = "81a5d257c8e410db227a6665ed08f64fea08e997";

/// Run the case for one `ML` ID.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub fn run(id: &str) {
    match id {
        "ML-01" => ml_01(),
        "ML-02" => ml_02(),
        "ML-03" => ml_03(),
        "ML-04" => ml_04(),
        "ML-05" => ml_05(),
        _ => panic!("no mathlib case is wired for {id}"),
    }
}

fn table(name: &str, url: &str, revision: &str) -> String {
    format!("[[lean_library]]\nname = \"{name}\"\nurl = \"{url}\"\nrevision = \"{revision}\"\n\n")
}

/// The language-1.2 example with `libraries` declared, required by its Lake
/// workspace, and (when `fetched`) present in the form `lake exe cache get`
/// leaves it. No lock has been written.
fn library_project(libraries: &[&str], fetched: bool) -> P {
    let project = P::language_1_2_example();
    for name in libraries {
        declare(&project, name, URL, REVISION);
    }
    require(&project, libraries, REVISION);
    if fetched {
        for name in libraries {
            present(&project, name, REVISION);
        }
    }
    project
}

fn declare(project: &P, name: &str, url: &str, revision: &str) {
    project.edit(
        "lexlean.toml",
        "[limits]",
        &format!("{}[limits]", table(name, url, revision)),
    );
}

/// Write the Lake workspace manifest the way Lake records a git requirement.
fn require(project: &P, names: &[&str], revision: &str) {
    let packages: Vec<String> = names
        .iter()
        .map(|name| {
            format!(
                "{{\"url\": \"{URL}\", \"type\": \"git\", \"subDir\": null, \"scope\": \"leanprover-community\", \"rev\": \"{revision}\", \"name\": \"{name}\", \"manifestFile\": \"lake-manifest.json\", \"inputRev\": \"{revision}\", \"inherited\": false, \"configFile\": \"lakefile.lean\"}}"
            )
        })
        .collect();
    project.write_raw(
        "lake-manifest.json",
        &format!(
            "{{\"version\": \"1.2.0\",\n \"packagesDir\": \".lake/packages\",\n \"packages\": [{}],\n \"name\": \"language_twelve_host\",\n \"lakeDir\": \".lake\",\n \"fixedToolchain\": false}}\n",
            packages.join(", ")
        ),
    );
}

/// A library as Lake and `cache get` leave it: a checkout at `revision`, its
/// own manifest, and the built artifacts directory.
fn present(project: &P, name: &str, revision: &str) {
    let base = format!(".lake/packages/{name}");
    project.write_raw(&format!("{base}/.git/HEAD"), &format!("{revision}\n"));
    project.write_raw(
        &format!("{base}/lake-manifest.json"),
        &format!("{{\"version\": \"1.2.0\", \"packagesDir\": \".lake/packages\", \"packages\": [], \"name\": \"{name}\", \"lakeDir\": \".lake\"}}\n"),
    );
    project.write_raw(
        &format!("{base}/.lake/build/lib/lean/{name}.olean"),
        "olean",
    );
}

fn lock_request(check_only: bool) -> LockRequest {
    LockRequest {
        check_only,
        allow_network: false,
    }
}

/// The text of the `[[lean_library]]` row of `name`.
fn lock_row(lock: &str, name: &str) -> String {
    lock.split("[[lean_library]]")
        .skip(1)
        .map(|row| row.split("\n[").next().unwrap_or(row))
        .find(|row| row.contains(&format!("name = \"{name}\"\n")))
        .unwrap_or_else(|| panic!("the lock has a row for `{name}`: {lock}"))
        .to_owned()
}

fn field(row: &str, key: &str) -> String {
    let prefix = format!("{key} = \"");
    row.lines()
        .find_map(|line| line.strip_prefix(prefix.as_str()))
        .unwrap_or_else(|| panic!("no `{key}` in {row}"))
        .trim_end_matches('"')
        .to_owned()
}

fn empty_tree() -> String {
    Sha256Digest::of(b"lexlean-tree-v1\0").to_hex()
}

/// The configuration parsed from `text`, expecting its refusal.
fn config_refused(text: &str, why: &str) -> String {
    let diagnostics = lexlean::config::parse_project("lexlean.toml", text.as_bytes())
        .err()
        .unwrap_or_else(|| panic!("{why}: the configuration is refused"));
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code.as_str() == "LLC0109"),
        "{why}: expected LLC0109, found {diagnostics:?}"
    );
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.clone())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Every command that must stay offline, each expected to fail with `code`
/// before any backend runs.
fn every_command_refuses(project: &P, code: &str) {
    let engine = project.engine();
    let selection = || Selection::Entrypoints;
    let errors: Vec<(&str, LexLeanError)> = vec![
        (
            "lock",
            engine.lock(lock_request(false)).err().expect("lock fails"),
        ),
        (
            "lock --check",
            engine
                .lock(lock_request(true))
                .err()
                .expect("lock --check fails"),
        ),
        (
            "check",
            engine
                .check(CheckRequest {
                    selection: selection(),
                })
                .err()
                .expect("check fails"),
        ),
        (
            "build",
            engine
                .build(BuildRequest {
                    selection: selection(),
                })
                .err()
                .expect("build fails"),
        ),
        (
            "fmt",
            engine
                .format(FormatRequest {
                    selection: selection(),
                    check_only: true,
                })
                .err()
                .expect("fmt fails"),
        ),
        (
            "verify",
            engine
                .verify(VerifyRequest {
                    selection: selection(),
                })
                .err()
                .expect("verify fails"),
        ),
    ];
    for (command, error) in &errors {
        assert!(
            error
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code.as_str() == code),
            "{command}: expected {code}, found {error}"
        );
    }
    for output in [".lexlean/build", ".lexlean/verified"] {
        assert!(
            !project.root.join(output).as_std_path().exists(),
            "{code}: `{output}` was published"
        );
    }
}

/// ML-01: the declaration and its refusals.
fn ml_01() {
    let example = P::language_1_2_example();
    let base = example.read_raw("lexlean.toml");
    let with = |tables: &str| base.replacen("[limits]", &format!("{tables}[limits]"), 1);

    // The declaration reads back to the bytes written.
    let text = with(&table("mathlib", URL, REVISION));
    let config = lexlean::config::parse_project("lexlean.toml", text.as_bytes())
        .expect("the declaration parses");
    assert_eq!(config.lean_libraries.len(), 1);
    assert_eq!(config.lean_libraries[0].name, "mathlib");
    assert_eq!(config.lean_libraries[0].url, URL);
    assert_eq!(config.lean_libraries[0].revision, REVISION);
    assert_eq!(config.canonical_toml(), text, "canonical serialization");
    let two = with(&format!(
        "{}{}",
        table("aesop", URL, REVISION),
        table("mathlib", URL, REVISION)
    ));
    let config = lexlean::config::parse_project("lexlean.toml", two.as_bytes()).expect("parses");
    assert_eq!(config.canonical_toml(), two);

    // A project without one keeps its bytes.
    let plain = lexlean::config::parse_project("lexlean.toml", base.as_bytes()).expect("parses");
    assert!(plain.lean_libraries.is_empty());
    assert_eq!(plain.canonical_toml(), base);
    assert!(!base.contains("lean_library"));

    // Each refusal is LLC0109.
    let cases: Vec<(&str, String)> = vec![
        ("an http URL", with(&table("mathlib", "http://github.com/leanprover-community/mathlib4", REVISION))),
        ("an ssh URL", with(&table("mathlib", "git@github.com:leanprover-community/mathlib4", REVISION))),
        ("a short revision", with(&table("mathlib", URL, &REVISION[..39]))),
        ("an uppercase revision", with(&table("mathlib", URL, &REVISION.to_uppercase()))),
        ("a branch as the revision", with(&table("mathlib", URL, "master"))),
        ("a tag as the revision", with(&table("mathlib", URL, "v4.32.1"))),
        (
            "a branch field",
            with(&format!("{}
branch = \"master\"\n\n", table("mathlib", URL, REVISION).trim_end()
                )),
        ),
        (
            "a tag field",
            with(&format!("{}
tag = \"v4.32.1\"\n\n", table("mathlib", URL, REVISION).trim_end()
                )),
        ),
        (
            "an unknown field",
            with(&format!("{}
path = \"x\"\n\n", table("mathlib", URL, REVISION).trim_end()
                )),
        ),
        ("a name that is not an identifier", with(&table("Mathlib", URL, REVISION))),
        ("a missing revision", with("[[lean_library]]\nname = \"mathlib\"\nurl = \"https://github.com/leanprover-community/mathlib4\"\n\n")),
        (
            "a repeated name",
            with(&format!("{}{}", table("mathlib", URL, REVISION), table("mathlib", URL, REVISION))),
        ),
        (
            "tables out of order",
            with(&format!("{}{}", table("mathlib", URL, REVISION), table("aesop", URL, REVISION))),
        ),
    ];
    for (why, text) in &cases {
        config_refused(text, why);
    }
    // The branch and the tag are named, not merely rejected as unknown.
    let message = config_refused(&cases[6].1, "a branch field");
    assert!(message.contains("branch"), "{message}");
    let message = config_refused(&cases[7].1, "a tag field");
    assert!(message.contains("tag"), "{message}");

    // Another language has no Lean library.
    let old = P::example();
    let old_text = old.read_raw("lexlean.toml").replacen(
        "[limits]",
        &format!("{}[limits]", table("mathlib", URL, REVISION)),
        1,
    );
    let message = config_refused(&old_text, "language 1.0");
    assert!(message.contains("language"), "{message}");
}

/// ML-02: the lock rows and the refusals that name both values.
#[allow(clippy::too_many_lines)]
fn ml_02() {
    let project = library_project(&["aesop", "mathlib"], true);
    project.relock();
    let lock = project.read_raw("lexlean.lock");
    assert!(lock.starts_with("spec = \"lexlean/lock/2\"\n"));
    let manifest = |name: &str| {
        Sha256Digest::of(
            project
                .read_raw(&format!(".lake/packages/{name}/lake-manifest.json"))
                .as_bytes(),
        )
        .to_hex()
    };
    for name in ["aesop", "mathlib"] {
        let row = format!(
            "[[lean_library]]\nname = \"{name}\"\nurl = \"{URL}\"\nrevision = \"{REVISION}\"\nlake_manifest_sha256 = \"{}\"\nglossary_sha256 = \"{}\"\n",
            manifest(name),
            empty_tree()
        );
        assert!(lock.contains(&row), "{row}\n{lock}");
    }
    assert!(
        lock.find("name = \"aesop\"") < lock.find("name = \"mathlib\"")
            || lock.find("[[lean_library]]\nname = \"aesop\"")
                < lock.find("[[lean_library]]\nname = \"mathlib\""),
        "rows sort by name"
    );
    let engine = project.engine();
    engine
        .lock(lock_request(true))
        .expect("lock --check accepts what lock wrote");
    engine.lock(lock_request(false)).expect("lock again");
    assert_eq!(
        lock,
        project.read_raw("lexlean.lock"),
        "the same bytes again"
    );

    // A project without a library keeps its lock bytes.
    let plain = P::language_1_2_example();
    plain.relock();
    assert!(!plain.read_raw("lexlean.lock").contains("lean_library"));
    assert_eq!(
        plain.read_raw("lexlean.lock"),
        std::fs::read_to_string(
            support::repo_root()
                .join("examples/language-1.2/lexlean.lock")
                .as_std_path()
        )
        .expect("committed lock"),
    );

    // The glossary digest is the tree digest of the project's `lean.<name>`
    // lexicon package.
    let glossed = library_project(&["mathlib"], true);
    glossed.write_raw(
        "glossary/lean.mathlib/lexicon.toml",
        "spec = \"lexlean/lexicon/1\"
package = \"lean.mathlib\"
version = \"1.0.0\"
language = \"1.2\"
imports = []
",
    );
    glossed.edit(
        "lexlean.toml",
        "[[lexicon_source]]",
        "[[lexicon_source]]
package = \"lean.mathlib\"
kind = \"path\"
path = \"glossary/lean.mathlib\"

[[lexicon_source]]",
    );
    glossed.relock();
    let lock = glossed.read_raw("lexlean.lock");
    let package_row = lock
        .split("[[package]]")
        .skip(1)
        .find(|row| row.contains("id = \"lean.mathlib\"\n"))
        .expect("the glossary package is locked");
    let glossary = field(&lock_row(&lock, "mathlib"), "glossary_sha256");
    assert_eq!(glossary, field(package_row, "tree_sha256"));
    assert_ne!(glossary, empty_tree());

    // A library whose bytes are not the pinned ones: both digests are named.
    let before = project.read_raw("lexlean.lock");
    let locked_manifest = field(&lock_row(&before, "mathlib"), "lake_manifest_sha256");
    project.write_raw(
        ".lake/packages/mathlib/lake-manifest.json",
        "{\"version\": \"1.2.0\", \"packagesDir\": \".lake/packages\", \"packages\": [], \"name\": \"mathlib\", \"lakeDir\": \".lake\", \"changed\": true}\n",
    );
    let error = project
        .engine()
        .lock(lock_request(true))
        .err()
        .expect("a changed library manifest is refused");
    support::expect_code(&error, "LLC0110");
    let found = manifest("mathlib");
    let message = error.to_string();
    assert!(message.contains("`mathlib`"), "{message}");
    assert!(message.contains(&locked_manifest), "{message}");
    assert!(message.contains(&found), "{message}");
    project.check_fails_with("LLC0110");

    // A changed glossary is refused by name too.
    let locked_glossary = glossary;
    glossed.edit("glossary/lean.mathlib/lexicon.toml", "1.0.0", "1.0.1");
    let error = glossed
        .engine()
        .lock(lock_request(true))
        .err()
        .expect("a changed glossary is refused");
    support::expect_code(&error, "LLC0110");
    assert!(error.to_string().contains(&locked_glossary), "{error}");

    // The Lake workspace must require the library at the declared place.
    for (why, manifest_url, manifest_revision) in [
        ("another revision", URL, OTHER_REVISION),
        (
            "another URL",
            "https://github.com/example/mathlib4",
            REVISION,
        ),
    ] {
        let mismatch = library_project(&["mathlib"], true);
        mismatch.write_raw(
            "lake-manifest.json",
            &format!("{{\"version\": \"1.2.0\", \"packagesDir\": \".lake/packages\", \"packages\": [{{\"url\": \"{manifest_url}\", \"type\": \"git\", \"rev\": \"{manifest_revision}\", \"name\": \"mathlib\", \"inputRev\": \"{manifest_revision}\"}}], \"name\": \"language_twelve_host\", \"lakeDir\": \".lake\"}}\n"),
        );
        let error = mismatch
            .engine()
            .lock(lock_request(false))
            .err()
            .unwrap_or_else(|| panic!("{why} is refused"));
        support::expect_code(&error, "LLC0110");
        let message = error.to_string();
        assert!(
            message.contains(manifest_revision) && message.contains(REVISION),
            "{why}: {message}"
        );
    }
    let unrequired = library_project(&["mathlib"], true);
    unrequired.write_raw(
        "lake-manifest.json",
        "{\"version\": \"1.2.0\", \"packagesDir\": \".lake/packages\", \"packages\": [], \"name\": \"language_twelve_host\", \"lakeDir\": \".lake\"}\n",
    );
    let error = unrequired
        .engine()
        .lock(lock_request(false))
        .err()
        .expect("an unrequired library is refused");
    support::expect_code(&error, "LLC0110");
}

/// A `lake` that behaves as `lake exe cache get` does, in a directory of its
/// own, and the `PATH` that finds it first.
struct StubLake {
    directory: tempfile::TempDir,
}

impl StubLake {
    fn new(body: &str) -> Self {
        let directory = tempfile::Builder::new()
            .prefix("lexlean-stub-lake-")
            .tempdir()
            .expect("tempdir");
        let program = directory.path().join("lake");
        std::fs::write(&program, format!("#!/bin/sh\n{body}\n")).expect("write stub");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755))
                .expect("chmod");
        }
        Self { directory }
    }

    /// Succeeds like `cache get`: leaves `name` present at `revision`.
    fn fetching(name: &str, revision: &str) -> Self {
        Self::new(&format!(
            "mkdir -p .lake\necho \"$*\" >> .lake/stub.log\n[ \"$1 $2 $3\" = \"exe cache get\" ] || exit 2\nmkdir -p .lake/packages/{name}/.git .lake/packages/{name}/.lake/build/lib/lean\nprintf '%s\\n' {revision} > .lake/packages/{name}/.git/HEAD\nprintf '{{\"packages\": [], \"name\": \"{name}\"}}\\n' > .lake/packages/{name}/lake-manifest.json\necho olean > .lake/packages/{name}/.lake/build/lib/lean/{name}.olean"
        ))
    }

    fn with_path<T>(&self, body: impl FnOnce() -> T) -> T {
        let path = std::env::var("PATH").unwrap_or_default();
        let combined = format!("{}:{path}", self.directory.path().display());
        support::with_env(&[("PATH", Some(&combined))], body)
    }
}

/// ML-03: `deps fetch` and nothing else acquires the library.
fn ml_03() {
    // The library is absent; the declaration and the Lake requirement hold.
    let project = library_project(&["mathlib"], false);
    let before = support::file_set(&project.root);
    let stub = StubLake::fetching("mathlib", REVISION);
    let (exit, stdout, stderr) = stub.with_path(|| project.cli(&["deps", "fetch"]));
    assert_eq!(exit, 0, "{stdout}{stderr}");
    let log = project.read_raw(".lake/stub.log");
    assert_eq!(log.trim(), "exe cache get", "{log}");
    let after = support::file_set(&project.root);
    let written: Vec<&String> = after
        .difference(&before)
        .filter(|path| !path.starts_with(".lake/"))
        .collect();
    assert!(
        written.is_empty(),
        "written outside the Lake workspace: {written:?}"
    );
    // Now the project locks and checks offline.
    project.relock();
    project
        .engine()
        .lock(lock_request(true))
        .expect("lock --check");
    project.check_ok();

    // With no library declared nothing is fetched and the command succeeds.
    let none = P::language_1_2_example();
    let (exit, stdout, stderr) = stub.with_path(|| none.cli(&["deps", "fetch"]));
    assert_eq!(exit, 0, "{stdout}{stderr}");
    assert!(!none.root.join(".lake/stub.log").as_std_path().exists());

    // A failing `lake`, a missing `lake`, and a `lake` that leaves another
    // revision are each LLV7020, exit 3.
    let failing = StubLake::new("echo boom >&2\nexit 1");
    let leaves_other = StubLake::fetching("mathlib", OTHER_REVISION);
    let leaves_nothing = StubLake::new("exit 0");
    for (why, lake) in [
        ("a failing lake", &failing),
        ("another revision", &leaves_other),
        ("nothing", &leaves_nothing),
    ] {
        let project = library_project(&["mathlib"], false);
        let (exit, _, stderr) = lake.with_path(|| project.cli(&["deps", "fetch"]));
        assert_eq!(exit, 3, "{why}: {stderr}");
        assert!(stderr.contains("LLV7020"), "{why}: {stderr}");
    }
    let project = library_project(&["mathlib"], false);
    let (exit, _, stderr) = support::with_env(&[("PATH", Some("/nonexistent"))], || {
        project.cli(&["deps", "fetch"])
    });
    assert_eq!(exit, 3, "{stderr}");
    assert!(stderr.contains("LLV7020"), "{stderr}");

    // A declaration the Lake workspace does not require is not fetched.
    let unrequired = P::language_1_2_example();
    declare(&unrequired, "mathlib", URL, REVISION);
    let (exit, _, stderr) = stub.with_path(|| unrequired.cli(&["deps", "fetch"]));
    assert_eq!(exit, 2, "{stderr}");
    assert!(stderr.contains("LLC0110"), "{stderr}");
    assert!(!unrequired
        .root
        .join(".lake/stub.log")
        .as_std_path()
        .exists());
}

/// ML-04: every other command stays offline and says what to run.
/// ML-05: `fmt` of a project that declares a library is a rewrite, not a fetch.
fn ml_05() {
    let project = library_project(&["mathlib"], true);
    project.relock();
    let canonical = project.read_raw("src/Main.lex.tex");
    project.write_raw("src/Main.lex.tex", &canonical.replace('\n', "\r\n"));
    let format = |check_only| FormatRequest {
        selection: Selection::Entrypoints,
        check_only,
    };
    assert!(
        project.engine().format(format(true)).is_err(),
        "a CRLF entrypoint is not canonical"
    );
    project
        .engine()
        .format(format(false))
        .expect("fmt rewrites a project that declares a library");
    assert_eq!(
        project.read_raw("src/Main.lex.tex"),
        canonical,
        "the entrypoint is rewritten to its canonical form"
    );
    project
        .engine()
        .format(format(true))
        .expect("and it is canonical now");
}

fn ml_04() {
    // Declared and required, never fetched.
    let unfetched = library_project(&["mathlib"], false);
    every_command_refuses(&unfetched, "LLV7019");
    let error = unfetched
        .engine()
        .lock(lock_request(false))
        .err()
        .expect("fails");
    let message = error.to_string();
    assert!(message.contains("mathlib"), "{message}");
    assert!(message.contains("lexlean deps fetch"), "{message}");
    let (exit, _, stderr) = unfetched.cli(&["verify", "--all"]);
    assert_eq!(exit, 3, "{stderr}");
    assert!(stderr.contains("LLV7019"), "{stderr}");

    // Fetched, locked, then the library goes away.
    let project = library_project(&["mathlib"], true);
    project.relock();
    project.check_ok();
    std::fs::remove_dir_all(project.root.join(".lake/packages/mathlib").as_std_path())
        .expect("remove");
    every_command_refuses(&project, "LLV7019");

    // Present at another revision, and present without its built artifacts.
    for (why, edit) in [
        ("another revision", 0),
        ("no built artifacts", 1),
        ("no manifest", 2),
    ] {
        let project = library_project(&["mathlib"], true);
        match edit {
            0 => project.write_raw(
                ".lake/packages/mathlib/.git/HEAD",
                &format!("{OTHER_REVISION}\n"),
            ),
            1 => std::fs::remove_dir_all(
                project
                    .root
                    .join(".lake/packages/mathlib/.lake/build")
                    .as_std_path(),
            )
            .expect("remove"),
            _ => std::fs::remove_file(
                project
                    .root
                    .join(".lake/packages/mathlib/lake-manifest.json")
                    .as_std_path(),
            )
            .expect("remove"),
        }
        let error = project
            .engine()
            .lock(lock_request(false))
            .err()
            .unwrap_or_else(|| panic!("{why}: refused"));
        support::expect_code(&error, "LLV7019");
    }

    // No command ran the stub `lake`: nothing fetches.
    let stub = StubLake::fetching("mathlib", REVISION);
    let project = library_project(&["mathlib"], false);
    stub.with_path(|| every_command_refuses(&project, "LLV7019"));
    assert!(!project.root.join(".lake/stub.log").as_std_path().exists());
    let _ = Engine::load(&project.root.join("lexlean.toml")).expect("the project loads");
}
