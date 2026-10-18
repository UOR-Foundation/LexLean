//! The `packages` suite: PK-01..PK-04, source packages (SPEC.md §10.5): a
//! language-1.2 project imports the modules of a locked, content-addressed
//! package of its workspace.

use lexlean::{Engine, LexLeanError, LockRequest};

use crate::production_consumer::{AppRoot, Application};
use crate::support::{self, P};

/// Run the case for one `PK` ID.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub fn run(id: &str) {
    match id {
        "PK-01" => pk_01(),
        "PK-02" => pk_02(),
        "PK-03" => pk_03(),
        "PK-04" => pk_04(),
        "PK-05" => pk_05(),
        _ => panic!("no packages case is wired for {id}"),
    }
}

/// Edit one occurrence in a file of the copy by its own bytes: `P::edit` reads
/// a `.lex.tex` file in its data form, which would rewrite the notation.
fn edit_raw(project: &P, relative: &str, from: &str, to: &str) {
    let path = project.root.join(relative);
    let text = std::fs::read_to_string(path.as_std_path()).expect("read");
    assert!(text.contains(from), "{relative} does not contain {from:?}");
    std::fs::write(path.as_std_path(), text.replacen(from, to, 1)).expect("write");
}

fn lock_request(check_only: bool) -> LockRequest {
    LockRequest {
        check_only,
        allow_network: false,
    }
}

/// The first failure of loading the project, then `lock --check`, then
/// `check`: the stage a refusal surfaces in is not what these cases pin, the
/// code is.
fn first_error(project: &P) -> LexLeanError {
    let engine = match Engine::load(&project.root.join("lexlean.toml")) {
        Ok(engine) => engine,
        Err(error) => return error,
    };
    if let Err(error) = engine.lock(lock_request(true)) {
        return error;
    }
    match engine.check(lexlean::CheckRequest {
        selection: lexlean::Selection::Entrypoints,
    }) {
        Ok(_) => panic!("the project is refused"),
        Err(error) => error,
    }
}

fn refused(project: &P, code: &str) -> LexLeanError {
    let error = first_error(project);
    support::expect_code(&error, code);
    // Refused before any backend runs: nothing is built or verified.
    for output in [".lexlean/build", ".lexlean/verified"] {
        assert!(
            !project.root.join(output).as_std_path().exists(),
            "{code}: `{output}` was published"
        );
    }
    error
}

fn lock_text(project: &P) -> String {
    project.read_raw("lexlean.lock")
}

/// The digest a lock row of `id` carries in `field`.
fn locked(lock: &str, id: &str, field: &str) -> String {
    let row = lock
        .split("[[source_package]]")
        .skip(1)
        .find(|row| row.contains(&format!("id = \"{id}\"\n")))
        .unwrap_or_else(|| panic!("the lock has a row for `{id}`"));
    let prefix = format!("{field} = \"");
    let value = row
        .lines()
        .find_map(|line| line.strip_prefix(prefix.as_str()))
        .unwrap_or_else(|| panic!("the row of `{id}` has `{field}`"));
    value.trim_end_matches('"').to_owned()
}

fn modules_of(project: &P) -> Vec<String> {
    support::checked_project(project)
        .modules
        .keys()
        .cloned()
        .collect()
}

/// PK-01: configuration, lock, workspace, and the refusals.
#[allow(clippy::too_many_lines)]
fn pk_01() {
    // The configuration and the lock read back to the bytes written.
    let app = P::package_workspace("app");
    app.relock();
    let lock = lock_text(&app);
    assert!(lock.starts_with("spec = \"lexlean/lock/2\"\n"));
    let base_row = "[[source_package]]\nid = \"base\"\nversion = \"1.0.0\"\nkind = \"path\"\nsource = \"base\"\nmodule_prefix = \"PkgBase\"\n";
    let lib_row = "[[source_package]]\nid = \"lib\"\nversion = \"1.0.0\"\nkind = \"path\"\nsource = \"lib\"\nmodule_prefix = \"PkgLib\"\n";
    assert!(lock.contains(base_row), "{lock}");
    assert!(lock.contains(lib_row), "{lock}");
    assert!(
        lock.find(base_row) < lock.find(lib_row),
        "rows are sorted by id"
    );
    assert!(lock.contains("imports = []\n"));
    assert!(lock.contains("imports = [\"base@1.0.0\"]\n"));
    for id in ["base", "lib"] {
        for field in ["tree_sha256", "manifest_sha256"] {
            assert_eq!(locked(&lock, id, field).len(), 64, "{id} {field}");
        }
    }
    let engine = app.engine();
    engine
        .lock(lock_request(true))
        .expect("lock --check accepts the lock it wrote");
    engine.lock(lock_request(false)).expect("lock again");
    assert_eq!(lock, lock_text(&app), "locking again writes the same bytes");
    let config =
        lexlean::config::parse_project("lexlean.toml", app.read_raw("lexlean.toml").as_bytes())
            .expect("the configuration parses");
    assert_eq!(config.source_packages.len(), 1);
    assert_eq!(
        config.canonical_toml(),
        app.read_raw("lexlean.toml"),
        "the configuration is its own canonical serialization"
    );
    let tree = locked(&lock, "lib", "tree_sha256");
    // The pin is of the package's bytes alone: the same package in another
    // workspace copy has the same digest.
    let other = P::package_workspace("app");
    other.relock();
    assert_eq!(tree, locked(&lock_text(&other), "lib", "tree_sha256"));

    // A project without a source package keeps its bytes (§10.1).
    let example = P::example();
    assert!(!lock_text(&example).contains("source_package"));
    assert_eq!(
        example.read_raw("lexlean.lock"),
        std::fs::read_to_string(
            support::repo_root()
                .join("examples/nat-add-zero/lexlean.lock")
                .as_std_path()
        )
        .expect("committed lock"),
        "a language-1.0 project locks to the bytes it locked to before"
    );
    example.relock();
    assert!(!lock_text(&example).contains("source_package"));

    // A changed byte of the package: the locked and the observed digests are
    // both named.
    let tampered = P::package_workspace("app");
    tampered.relock();
    let before = lock_text(&tampered);
    edit_raw(
        &tampered,
        "../base/src/Core.lex.tex",
        "name: twice_two,",
        "name: twice_twoo,",
    );
    let error = refused(&tampered, "LLC0105");
    let message = error.to_string();
    let locked_digest = locked(&before, "base", "tree_sha256");
    assert!(message.contains("`base`"), "{message}");
    assert!(message.contains(&locked_digest), "{message}");
    tampered.relock();
    let observed = locked(&lock_text(&tampered), "base", "tree_sha256");
    assert_ne!(observed, locked_digest);
    assert!(message.contains(&observed), "{message}");

    // Every command that links reads the package again: a lock read before the
    // package changed is not the lock of what would be linked.
    let direct = P::package_workspace("app");
    direct.relock();
    let loaded = lexlean::project::Project::load(&direct.root.join("lexlean.toml")).expect("loads");
    let (read, resolved) = lexlean::lock::read_current_lock(&loaded).expect("the lock is current");
    edit_raw(
        &direct,
        "../base/src/Core.lex.tex",
        "name: twice_two,",
        "name: twice_twoo,",
    );
    let stale =
        lexlean::link::check_project(&loaded, &lexlean::Selection::Entrypoints, &read, resolved)
            .err()
            .expect("linking reads the package again");
    support::expect_code(&stale, "LLC0105");

    // The same bytes for a changed manifest of the package.
    let manifest = P::package_workspace("app");
    manifest.relock();
    edit_raw(
        &manifest,
        "../base/lexlean.toml",
        "name = \"base\"",
        "name = \"basis\"",
    );
    refused(&manifest, "LLC0105");

    // A changed version, in the declaration.
    let version = P::package_workspace("app");
    version.relock();
    edit_raw(
        &version,
        "lexlean.toml",
        "version = \"1.0.0\"",
        "version = \"1.0.1\"",
    );
    let error = refused(&version, "LLC0105");
    assert!(error.to_string().contains("1.0.1"), "{error}");

    // An undeclared package: the lock names one the configuration does not.
    let undeclared = P::package_workspace("app");
    undeclared.relock();
    edit_raw(
        &undeclared,
        "lexlean.toml",
        "[[source_package]]\npackage = \"lib\"\nversion = \"1.0.0\"\nkind = \"path\"\npath = \"lib\"\n\n",
        "",
    );
    refused(&undeclared, "LLC0106");

    // One identifier with two sources.
    let split = P::package_workspace("app");
    edit_raw(
        &split,
        "../lib/lexlean.toml",
        "path = \"base\"",
        "path = \"lib\"",
    );
    edit_raw(
        &split,
        "lexlean.toml",
        "package = \"lib\"",
        "package = \"lib\"",
    );
    let split_error = Engine::load(&split.root.join("lexlean.toml"))
        .ok()
        .and_then(|engine| engine.lock(lock_request(false)).err())
        .expect("the lock is refused");
    assert!(
        split_error
            .diagnostics
            .iter()
            .any(|d| d.code.as_str() == "LLC0106" || d.code.as_str() == "LLR3003"),
        "{split_error}"
    );

    // A module prefix claimed by two sources.
    let prefix = P::package_workspace("app");
    edit_raw(
        &prefix,
        "../base/lexlean.toml",
        "module_prefix = \"PkgBase\"",
        "module_prefix = \"PkgLib\"",
    );
    let error = Engine::load(&prefix.root.join("lexlean.toml"))
        .expect("loads")
        .lock(lock_request(false))
        .err()
        .expect("two sources claim PkgLib");
    support::expect_code(&error, "LLC0107");
    let own = P::package_workspace("app");
    edit_raw(
        &own,
        "lexlean.toml",
        "module_prefix = \"PkgApp\"",
        "module_prefix = \"PkgLib.Sub\"",
    );
    let error = Engine::load(&own.root.join("lexlean.toml"))
        .expect("loads")
        .lock(lock_request(false))
        .err()
        .expect("the project's prefix lies below a package's");
    support::expect_code(&error, "LLC0107");

    // A path that is not a member, a project that is not a member, and no
    // workspace at all.
    let stranger = P::package_workspace("app");
    edit_raw(
        &stranger,
        "lexlean.toml",
        "path = \"lib\"",
        "path = \"other\"",
    );
    refused(&stranger, "LLC0108");
    let not_member = P::package_workspace("app");
    std::fs::write(
        not_member
            .workspace_dir()
            .join("lexlean-workspace.toml")
            .as_std_path(),
        "spec = \"lexlean/workspace/1\"\nmembers = [\"base\", \"lib\"]\n",
    )
    .expect("write");
    refused(&not_member, "LLC0108");
    let outside = P::package_workspace("app");
    std::fs::remove_file(
        outside
            .workspace_dir()
            .join("lexlean-workspace.toml")
            .as_std_path(),
    )
    .expect("remove");
    refused(&outside, "LLC0108");
    let malformed = P::package_workspace("app");
    std::fs::write(
        malformed
            .workspace_dir()
            .join("lexlean-workspace.toml")
            .as_std_path(),
        "spec = \"lexlean/workspace/1\"\nmembers = [\"lib\", \"base\"]\n",
    )
    .expect("write");
    refused(&malformed, "LLC0108");

    // A path that escapes the workspace, and a symlink.
    let escaping = P::package_workspace("app");
    edit_raw(
        &escaping,
        "lexlean.toml",
        "path = \"lib\"",
        "path = \"../lib\"",
    );
    refused(&escaping, "LLS8001");
    let absolute = P::package_workspace("app");
    edit_raw(
        &absolute,
        "lexlean.toml",
        "path = \"lib\"",
        "path = \"/lib\"",
    );
    refused(&absolute, "LLS8001");
    if cfg!(unix) {
        let linked = P::package_workspace("app");
        let workspace = linked.workspace_dir();
        std::fs::rename(
            workspace.join("lib").as_std_path(),
            workspace.join("lib-real").as_std_path(),
        )
        .expect("rename");
        support::symlink_any(workspace.join("lib-real"), workspace.join("lib"));
        refused(&linked, "LLS8001");
        let inner = P::package_workspace("app");
        let source = inner.workspace_dir().join("base/src");
        std::fs::rename(
            source.as_std_path(),
            inner.workspace_dir().join("base/src-real").as_std_path(),
        )
        .expect("rename");
        support::symlink_any(inner.workspace_dir().join("base/src-real"), source);
        refused(&inner, "LLS8001");
    }

    // Other kinds and other languages are refused, not accepted.
    let git = P::package_workspace("app");
    edit_raw(
        &git,
        "lexlean.toml",
        "kind = \"path\"\npath = \"lib\"",
        "kind = \"git\"\npath = \"lib\"",
    );
    refused(&git, "LLC0101");
    let language = P::package_workspace("app");
    edit_raw(
        &language,
        "lexlean.toml",
        "language = \"1.2\"",
        "language = \"1.1\"",
    );
    refused(&language, "LLC0101");
    let unsorted = P::package_workspace("app");
    edit_raw(
        &unsorted,
        "lexlean.toml",
        "\n[limits]",
        "\n[[source_package]]\npackage = \"base\"\nversion = \"1.0.0\"\nkind = \"path\"\npath = \"base\"\n\n[limits]",
    );
    refused(&unsorted, "LLC0101");
    // A package may declare no model artifact, no PDF provider, and no
    // lexicon source but a builtin one: those would read outside what the
    // pin covers.
    let artifact = P::package_workspace("app");
    edit_raw(
        &artifact,
        "../base/lexlean.toml",
        "\n[limits]",
        "\n[[artifact_source]]\nsha256 = \"0000000000000000000000000000000000000000000000000000000000000000\"\npath = \"artifact.bin\"\n\n[limits]",
    );
    refused(&artifact, "LLC0101");
}

/// PK-02: the import closure at link.
fn pk_02() {
    let app = P::package_workspace("app");
    app.relock();
    app.check_ok();
    let modules = modules_of(&app);
    assert_eq!(
        modules,
        ["Main", "PkgBase.Core", "PkgLib.Shapes", "PkgLib.Util"],
        "package modules are modules of the importing project under their own prefix"
    );
    let checked = support::checked_project(&app);
    for (name, lean) in [
        ("Main", "PkgApp.Main"),
        ("PkgBase.Core", "PkgBase.Core"),
        ("PkgLib.Shapes", "PkgLib.Shapes"),
        ("PkgLib.Util", "PkgLib.Util"),
    ] {
        assert_eq!(checked.modules[name].document.lean_module, lean);
    }
    // Inside a package a module keeps its relative import of a sibling and
    // names a module of a package its manifest declares by that package's
    // prefix.
    assert_eq!(
        checked.modules["PkgLib.Util"].document.imports,
        ["PkgLib.Shapes"]
    );
    assert_eq!(
        checked.modules["PkgLib.Shapes"].document.imports,
        ["PkgBase.Core"]
    );

    // A module of a package the project does not declare, though the
    // closure holds it.
    let transitive = P::package_workspace("app");
    edit_raw(
        &transitive,
        "src/Main.lex.tex",
        "\\importmodule{PkgLib.Util}\n",
        "\\importmodule{PkgBase.Core}\n\\importmodule{PkgLib.Util}\n",
    );
    transitive.relock();
    transitive.check_fails_with("LLR3009");

    // The same for a module of a package: `base` does not declare `lib`.
    let inward = P::package_workspace("app");
    edit_raw(
        &inward,
        "../base/src/Core.lex.tex",
        "\\title{Natural number addition}",
        "\\importmodule{PkgLib.Util}\n\\title{Natural number addition}",
    );
    inward.relock();
    inward.check_fails_with("LLR3009");

    // A module no source provides keeps the existing refusal.
    let unknown = P::package_workspace("app");
    edit_raw(
        &unknown,
        "src/Main.lex.tex",
        "\\importmodule{PkgLib.Util}\n",
        "\\importmodule{PkgLib.Util}\n\\importmodule{PkgLib.Missing}\n",
    );
    unknown.relock();
    unknown.check_fails_with("LLR3005");

    // A name both the project and a package provide.
    let both = P::package_workspace("app");
    both.write(
        "src/PkgLib/Util.lex.tex",
        &std::fs::read_to_string(
            both.workspace_dir()
                .join("lib/src/Util.lex.tex")
                .as_std_path(),
        )
        .expect("read")
        .replace("\\begin{lexlean}{Util}", "\\begin{lexlean}{PkgLib.Util}")
        .replace("\\importmodule{Shapes}\n", ""),
    );
    both.relock();
    both.check_fails_with("LLC0107");

    // A cycle through a package.
    let cycle = P::package_workspace("app");
    edit_raw(
        &cycle,
        "../base/lexlean.toml",
        "\n[limits]",
        "\n[[source_package]]\npackage = \"lib\"\nversion = \"1.0.0\"\nkind = \"path\"\npath = \"lib\"\n\n[limits]",
    );
    let error = Engine::load(&cycle.root.join("lexlean.toml"))
        .expect("loads")
        .lock(lock_request(false))
        .err()
        .expect("the packages depend on each other");
    support::expect_code(&error, "LLR3003");

    // A package is read through its own configuration: a change of its
    // entrypoints or of a module no import reaches changes the pin, but only
    // the modules imported are linked.
    let spare = P::package_workspace("app");
    spare.write(
        "../lib/src/Spare.lex.tex",
        "\\begin{lexlean}{Spare}\n\\title{Spare}\n\\end{lexlean}\n",
    );
    spare.relock();
    assert_eq!(modules_of(&spare), modules);
}

/// PK-03: an importing project builds, verifies, and records the origin.
fn pk_03() {
    let app = P::package_workspace("app");
    // A package file that is not canonical stays as its package wrote it:
    // formatting the importing project never rewrites it.
    edit_raw(
        &app,
        "../lib/src/Util.lex.tex",
        "  name: quadPlusOne,",
        "      name: quadPlusOne,",
    );
    app.relock();
    let before = package_bytes(&app);
    let formatted = app
        .engine()
        .format(lexlean::FormatRequest {
            selection: lexlean::Selection::Entrypoints,
            check_only: false,
        })
        .expect("fmt");
    assert_eq!(
        formatted.units.keys().cloned().collect::<Vec<_>>(),
        ["Main"],
        "only the project's own modules are formatted"
    );
    assert_eq!(before, package_bytes(&app), "fmt wrote no package file");
    app.fmt_check_ok();
    let built = app.build_ok();
    let source_id = built.source_id;
    let build_dir = app.build_dir(&built.build_id.expect("a build ID"));
    for module in [
        "PkgApp/Main",
        "PkgBase/Core",
        "PkgLib/Shapes",
        "PkgLib/Util",
    ] {
        assert!(
            build_dir.join(format!("modules/{module}.lean")).is_file(),
            "{module}"
        );
    }
    let manifest =
        std::fs::read_to_string(build_dir.join("manifest.json").as_std_path()).expect("manifest");
    assert!(
        manifest.contains("\"origin\":\"lib\""),
        "the manifest names the origin of a package module"
    );
    assert!(
        manifest.contains("\"origin\":\"base\""),
        "the manifest names the origin of a package module"
    );
    assert_eq!(manifest.matches("\"origin\"").count(), 3);

    // The source ID follows the pins: a byte of the package moves it once
    // the lock is rewritten, and no run reads the changed package before.
    let moved = P::package_workspace("app");
    moved.relock();
    edit_raw(
        &moved,
        "../base/src/Core.lex.tex",
        "name: twice_two,",
        "name: twice_twoo,",
    );
    moved.verify_fails_with("LLC0105");
    moved.assert_no_backend_output_with("LLC0105");
    moved.relock();
    let relocked = moved.check_ok();
    assert_ne!(
        relocked.source_id, source_id,
        "the source ID follows the pins"
    );

    if !support::lean_backed("PK-03") {
        return;
    }
    let _environment = support::env_shared();
    let verified = support::verify_ok(&app);
    assert_eq!(
        verified.units.keys().cloned().collect::<Vec<_>>(),
        ["Main", "PkgBase.Core", "PkgLib.Shapes", "PkgLib.Util"]
    );
    assert_eq!(verified.units["PkgLib.Util"].lean_module, "PkgLib.Util");
    // Every module is replayed and audited, the package modules under their
    // own prefix and with their origin in the records.
    let verified_dir = app
        .root
        .join(".lexlean/verified")
        .join(verified.attestation_id.to_hex());
    let attestation: serde_json::Value = serde_json::from_slice(
        &std::fs::read(verified_dir.join("attestation.json").as_std_path()).expect("attestation"),
    )
    .expect("the attestation is JSON");
    let processes = attestation["processes"].as_array().expect("processes");
    for module in [
        "PkgApp.Main",
        "PkgBase.Core",
        "PkgLib.Shapes",
        "PkgLib.Util",
    ] {
        for tool in ["lean", "leanchecker"] {
            assert!(
                processes.iter().any(|process| {
                    process["module"] == module
                        && process["argv"].as_array().is_some_and(|argv| {
                            argv.iter().any(|a| {
                                a.as_str()
                                    .is_some_and(|a| a == tool || a.ends_with(&format!("/{tool}")))
                            })
                        })
                }),
                "`{module}` is run through `{tool}`"
            );
        }
    }
    let declarations: Vec<&str> = attestation["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .filter_map(|row| row["name"].as_str())
        .collect();
    for name in [
        "PkgBase.Core.twice",
        "PkgBase.Core.twice_two",
        "PkgLib.Shapes.quad",
        "PkgLib.Util.quadPlusOne",
        "PkgApp.Main.appValue",
    ] {
        assert!(declarations.contains(&name), "`{name}` is audited");
    }
    assert_eq!(
        attestation["origins"],
        serde_json::json!({
            "PkgBase.Core": "base",
            "PkgLib.Shapes": "lib",
            "PkgLib.Util": "lib"
        }),
        "the modules of a package carry the package as their origin"
    );
    assert_eq!(before, package_bytes(&app), "verify wrote no package file");
}

/// The bytes of every file of the packages beside the project.
fn package_bytes(project: &P) -> Vec<(String, Vec<u8>)> {
    let mut files = Vec::new();
    for member in ["base", "lib"] {
        for entry in walkdir::WalkDir::new(project.workspace_dir().join(member).as_std_path())
            .sort_by_file_name()
            .into_iter()
            .flatten()
            .filter(|entry| entry.file_type().is_file())
        {
            files.push((
                entry.path().to_string_lossy().into_owned(),
                std::fs::read(entry.path()).expect("read"),
            ));
        }
    }
    files
}

/// PK-04: a production root reaches a package definition.
fn pk_04() {
    let app = P::package_workspace("app");
    app.relock();
    let checked = support::checked_project(&app);
    for (name, module) in &checked.modules {
        assert_eq!(
            module.production.is_some(),
            name == "Main",
            "`{name}`: only the importing project's own roots are analysed"
        );
    }
    let roots = lexlean::production::lower::roots(&checked).expect("roots");
    assert_eq!(
        roots
            .iter()
            .map(|root| root.report.root.clone())
            .collect::<Vec<_>>(),
        ["PkgApp.Main.appValue"],
        "a package's own roots are not roots of the importing project"
    );
    // The closure of the root includes the definitions of both packages.
    let program = crate::production_export::lowered_program(&app, "PkgApp.Main.appValue");
    let rendered = format!("{program:?}");
    assert!(!rendered.is_empty());

    if !support::lean_backed("PK-04") {
        return;
    }
    let _environment = support::env_shared();
    let application = Application {
        roots: vec![AppRoot {
            root: "PkgApp.Main.appValue".to_owned(),
            name: "app_value".to_owned(),
            version: "1.0.0".to_owned(),
            targets: vec!["rust-std".to_owned()],
        }],
    };
    let (output, report) =
        crate::production_consumer::export(&app.root.join("lexlean.toml"), &application, &[])
            .expect("the root certifies and exports");
    let _ = report;
    assert_eq!(
        output
            .result
            .roots
            .iter()
            .map(|root| root.root.clone())
            .collect::<Vec<_>>(),
        ["PkgApp.Main.appValue"],
        "the export holds the importing project's root alone"
    );
    let bundle = app.root.join(&output.bundle);
    let files = crate::production_export::tree(&bundle);
    assert!(
        files
            .keys()
            .all(|path| !path.contains("PkgBase") && !path.contains("PkgLib")),
        "no certificate of the package's own roots is published: {:?}",
        files.keys().collect::<Vec<_>>()
    );
}

/// PK-05: the bootstrap pilot imports the standard library's `Text`.
fn pk_05() {
    let root = support::repo_root();
    let read = |path: &str| {
        std::fs::read_to_string(root.join(path).as_std_path())
            .unwrap_or_else(|error| panic!("{path}: {error}"))
    };
    let config =
        lexlean::config::parse_project("lexlean.toml", read("bootstrap/lexlean.toml").as_bytes())
            .expect("the bootstrap configuration parses");
    assert_eq!(
        config
            .source_packages
            .iter()
            .map(|package| (package.package.as_str(), package.path.as_str()))
            .collect::<Vec<_>>(),
        [("std", "stdlib")],
        "the pilot declares the standard library as its one source package"
    );
    let workspace = lexlean::source_package::Workspace::read(&root)
        .expect("the workspace file reads")
        .expect("the repository has a workspace file");
    assert_eq!(workspace.members, ["bootstrap", "stdlib"]);

    let project = P::bootstrap();
    assert!(
        !project.root.join("src/Text.lex.tex").as_std_path().exists(),
        "the pilot holds no module Text of its own"
    );
    let checked = support::checked_project(&project);
    assert!(
        checked.modules.contains_key("AxiomOutput")
            && checked.modules.contains_key("LexLeanStd.Text"),
        "AxiomOutput and LexLeanStd.Text are linked modules of the project"
    );
    assert_eq!(
        checked.modules["LexLeanStd.Text"]
            .document
            .origin
            .as_deref(),
        Some("std"),
        "the standard library's Text is a module of the project, from the package"
    );
    let roots = lexlean::production::lower::roots(&checked).expect("roots");
    assert!(
        roots
            .iter()
            .all(|found| found.report.root.starts_with("LexLeanBoot.")),
        "the standard library's production roots are not roots of the project"
    );
    let mut axiom_roots: Vec<String> = roots
        .iter()
        .filter(|found| found.module == "AxiomOutput")
        .map(|found| found.report.root.clone())
        .collect();
    axiom_roots.sort();
    assert_eq!(
        axiom_roots,
        [
            "LexLeanBoot.AxiomOutput.isClosedLeanName",
            "LexLeanBoot.AxiomOutput.parseAuditOutput"
        ],
        "AxiomOutput's production roots are unchanged"
    );

    // The copy and the assertion that held it equal to the original are gone,
    // and no text still calls the copy temporary.
    let case = read("crates/conformance/src/cases/bootstrap.rs");
    assert!(
        !case.contains(&["vendored", "_text"].concat()),
        "BS-01 keeps no equivalence assertion"
    );
    assert!(
        !read("bootstrap/README.md").contains("temporary"),
        "bootstrap/README.md calls nothing temporary"
    );
    let spec = read("SPEC.md");
    let section = spec
        .split("## 17.23")
        .nth(1)
        .or_else(|| spec.split("### 17.23").nth(1))
        .expect("SPEC.md has the section 17.23")
        .split("\n### ")
        .next()
        .expect("a section");
    assert!(
        !section.contains("is therefore present in `bootstrap/src/` as a"),
        "SPEC.md §17.23 states no copy"
    );
}
