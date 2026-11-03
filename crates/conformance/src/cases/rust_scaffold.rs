//! Conformance cases for the package scaffolding of the Rust backend
//! (SPEC.md §17.16): RB-10..RB-13. The scaffolding is data, a language-1.2
//! module the compiler embeds, so these cases read the module, the
//! specification, the production registry, and the renderer's sources as the
//! bytes they are and compare them with what the renderer prints.

use std::collections::BTreeSet;

use lexlean::calculus::rust::package::{self, cargo_toml_of, Manifest};
use lexlean::calculus::rust::scaffold::{self, ClippyGroup, Level, Scaffold};
use lexlean::calculus::rust::{self, Profile};
use serde_json::Value as Json;

use crate::rust_packages;
use crate::support::{self, repo_root};

const MODULE: &str = "language/package-1.2/library/src/RustPackage.lex.tex";

fn module_text() -> String {
    std::fs::read_to_string(repo_root().join(MODULE).as_std_path()).expect("the scaffold module")
}

/// `text` with the one occurrence of `from` replaced; a plant that matches
/// nothing would test nothing.
fn plant(text: &str, from: &str, to: &str) -> String {
    assert_eq!(
        text.matches(from).count(),
        1,
        "the plant `{from}` is unique"
    );
    text.replace(from, to)
}

fn schema(name: &str) -> Json {
    serde_json::from_slice(
        &std::fs::read(repo_root().join("schemas").join(name).as_std_path()).expect("schema"),
    )
    .expect("schema JSON")
}

/// The source of every file that prints Rust: each `.rs` file of
/// `calculus/rust/`, the renderer's (`RENDERER_FILES`) and the runtime's
/// (`rt.rs`, `rt_decode.rs`, `rt_validate.rs`) alike, so that a table parked
/// in a file the renderer digest does not name is read as well.
fn renderer_sources() -> Vec<(String, String)> {
    let directory = repo_root().join("crates/lexlean/src/calculus/rust");
    let mut names: Vec<String> = std::fs::read_dir(directory.as_std_path())
        .expect("the renderer directory")
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.ends_with(".rs"))
        .collect();
    names.sort();
    for named in rust::RENDERER_FILES {
        assert!(
            names.iter().any(|name| name == named),
            "`{named}` is a file of the directory"
        );
    }
    names
        .into_iter()
        .map(|name| {
            let text = std::fs::read_to_string(directory.join(&name).as_std_path())
                .expect("renderer source");
            (name, text)
        })
        .collect()
}

fn audited(sources: &[(String, String)], scaffold: &Scaffold) -> Result<(), String> {
    let borrowed: Vec<(&str, &str)> = sources
        .iter()
        .map(|(name, text)| (name.as_str(), text.as_str()))
        .collect();
    scaffold::audit(scaffold, &borrowed)
}

/// The Cargo manifest that `scaffold` gives a package, written here from
/// Cargo's grammar and not by the renderer's printer.
fn expected_cargo(scaffold: &Scaffold, name: &str, version: &str) -> String {
    let mut lines = vec![
        "[package]".to_owned(),
        format!("name = \"{name}\""),
        format!("version = \"{version}\""),
        format!("edition = \"{}\"", scaffold.edition),
        format!("publish = {}", scaffold.publish),
        String::new(),
        "[lib]".to_owned(),
        format!("path = \"{}\"", scaffold.library),
        String::new(),
        "[lints.rust]".to_owned(),
    ];
    let gate = |gate: &scaffold::Gate| match gate.priority {
        0 => format!("{} = \"{}\"", gate.lint, gate.level.as_str()),
        priority => format!(
            "{} = {{ level = \"{}\", priority = {priority} }}",
            gate.lint,
            gate.level.as_str()
        ),
    };
    lines.extend(scaffold.rustc.iter().map(gate));
    lines.push(String::new());
    lines.push("[lints.clippy]".to_owned());
    lines.extend(scaffold.clippy.iter().map(gate));
    lines.extend(
        scaffold
            .allowed
            .iter()
            .map(|row| format!("{} = \"{}\"", row.lint, Level::Allow.as_str())),
    );
    lines.join("\n") + "\n"
}

/// The attributes the production registry gives a profile, read from the
/// registry's own file.
fn registry_allocation(profile: Profile) -> bool {
    let registry: toml::Value = std::fs::read_to_string(
        repo_root()
            .join("language/production-1.2.toml")
            .as_std_path(),
    )
    .expect("the registry")
    .parse()
    .expect("TOML");
    registry["target"]
        .as_array()
        .expect("targets")
        .iter()
        .find(|row| row["id"].as_str() == Some(profile.target()))
        .expect("the target row")["allocation"]
        .as_bool()
        .expect("allocation")
}

fn refused(result: Result<Scaffold, String>, expected: &str) {
    let error = result.expect_err(expected);
    assert!(error.contains(expected), "{error}");
}

pub fn run(id: &str) {
    match id {
        "RB-10" => rb_10(),
        "RB-11" => rb_11(),
        "RB-12" => rb_12(),
        "RB-13" => rb_13(),
        other => panic!("`{other}` is not a scaffold case"),
    }
}

/// §17.16: the scaffolding is data.
fn rb_10() {
    let text = module_text();
    let record = scaffold::decode(&text).expect("the module decodes");
    assert_eq!(
        &record,
        scaffold::get().expect("the embedded module"),
        "the module on disk is the one the compiler embeds"
    );
    // The library path is the module's: a package keeps its library where the
    // module says, and no renderer file holds the path as text (planted below).
    assert_eq!(record.library, "src/lib.rs");
    // Armed: an empty module states nothing for the audit to protect.
    assert!(
        record.allowed.len() > 1 && record.content().len() > record.allowed.len(),
        "the module states content"
    );

    // No file of the renderer holds a string of the module.
    let sources = renderer_sources();
    audited(&sources, &record).expect("no renderer file holds the scaffolding as text");

    // A manifest constant, a lint table, a header constant, and the reader's
    // own text are each refused, in the file they are planted in.
    for (file, planted, named) in [
        (
            "package.rs",
            format!(
                "\nconst MANIFEST: &str = \"[package]\\nedition = \\\"{}\\\"\\n\";\n",
                record.edition
            ),
            record.edition.clone(),
        ),
        (
            "package.rs",
            format!(
                "\npub const ALLOWED: &[&str] = &[\"{}\"];\n",
                record.allowed[0].lint
            ),
            record.allowed[0].lint.clone(),
        ),
        (
            "ast.rs",
            format!(
                "\nconst HEADER: &str = \"#![forbid({})]\\n\";\n",
                record.rustc[0].lint
            ),
            record.rustc[0].lint.clone(),
        ),
        (
            "scaffold.rs",
            format!("\nconst REASON: &str = \"{}\";\n", record.allowed[1].reason),
            record.allowed[1].reason.clone(),
        ),
        (
            "mod.rs",
            format!("\nconst GATE: &str = \"{}\";\n", record.clippy[0].lint),
            record.clippy[0].lint.clone(),
        ),
        (
            "lower.rs",
            "\nconst KEY: &str = \"publish = false\";\n".to_owned(),
            "publish = false".to_owned(),
        ),
        (
            "moves.rs",
            "\nconst KEY: &str = \"all = { level = 1 }\";\n".to_owned(),
            record.clippy[0].lint.clone(),
        ),
        (
            "validate.rs",
            "\nconst KEY: &str = \"[lints] priority = -1\";\n".to_owned(),
            "priority = -1".to_owned(),
        ),
        (
            "package.rs",
            format!("\nconst LIBRARY: &str = \"{}\";\n", record.library),
            record.library.clone(),
        ),
        (
            "rt.rs",
            format!(
                "\nconst HEADER: &str = \"#![forbid({})]\\n\";\n",
                record.rustc[0].lint
            ),
            record.rustc[0].lint.clone(),
        ),
        (
            "rt_decode.rs",
            format!(
                "\nconst KEY: &str = \"edition = \\\"{}\\\"\";\n",
                record.edition
            ),
            record.edition.clone(),
        ),
        (
            "rt_validate.rs",
            format!("\nconst LINT: &str = \"{}\";\n", record.clippy[0].lint),
            record.clippy[0].lint.clone(),
        ),
    ] {
        let mut planted_sources = sources.clone();
        planted_sources
            .iter_mut()
            .find(|(name, _)| name == file)
            .expect("the file")
            .1
            .push_str(&planted);
        let error = audited(&planted_sources, &record).expect_err("a planted constant");
        assert!(
            error.contains(file) && error.contains(&named),
            "{file}: {error}"
        );
    }

    // `Cargo.toml` is what the module gives, and is the committed one: every
    // committed package's manifest equals the module's, written here
    // independently of the printer.
    let mut seen = BTreeSet::new();
    for committed in rust_packages::packages() {
        let rendered = package::package(&committed.manifest).expect("a package");
        let cargo = String::from_utf8(rendered.files["Cargo.toml"].clone()).expect("UTF-8");
        assert_eq!(
            cargo,
            expected_cargo(
                &record,
                &committed.manifest.name,
                &committed.manifest.version
            ),
            "{}",
            committed.directory
        );
        // The crate header is the registry's `allocation = false` and the
        // module's gates at `forbid`, read from the registry file.
        let library = String::from_utf8(rendered.files["src/lib.rs"].clone()).expect("UTF-8");
        let expected_header = format!(
            "{}#![forbid({})]\n",
            if registry_allocation(committed.profile) {
                ""
            } else {
                "#![no_std]\n"
            },
            record
                .rustc
                .iter()
                .find(|gate| gate.level == Level::Forbid)
                .expect("a forbidden lint")
                .lint
        );
        assert!(
            library.starts_with(&expected_header),
            "{}: {}",
            committed.directory,
            &library[..library.len().min(120)]
        );
        seen.insert(committed.profile);
    }
    assert_eq!(seen.len(), 2, "both profiles are rendered");
    // The manifest and the crate state the one fact once: the lint the crate
    // forbids is a gate of the manifest at `forbid`.
    assert!(record
        .rustc
        .iter()
        .any(|gate| gate.lint == "unsafe_code" && gate.level == Level::Forbid));

    // The header follows its two sources: no allocation prints `no_std`,
    // another forbidden gate is restated, and a gate below `forbid` is not.
    assert!(record
        .attributes_with(false)
        .expect("attributes")
        .starts_with("#![no_std]\n"));
    assert!(!record
        .attributes_with(true)
        .expect("attributes")
        .contains("no_std"));
    let mut widened = record.clone();
    widened.rustc.push(scaffold::Gate {
        lint: "missing_docs".to_owned(),
        level: Level::Forbid,
        priority: 0,
    });
    assert!(widened
        .attributes_with(true)
        .expect("attributes")
        .ends_with("#![forbid(missing_docs)]\n"));
    widened.rustc.last_mut().expect("a gate").level = Level::Deny;
    assert_eq!(
        widened.attributes_with(true).expect("attributes"),
        record.attributes_with(true).expect("attributes")
    );
    // A gate with no lint name is refused by the validator of the
    // attribute nodes the header is printed from.
    let mut unnamed = record.clone();
    unnamed.rustc.push(scaffold::Gate {
        lint: String::new(),
        level: Level::Forbid,
        priority: 0,
    });
    let error = unnamed
        .attributes_with(true)
        .expect_err("an empty lint name");
    assert!(error.contains("attribute-args"), "{error}");

    // A row dropped from the module is a row dropped from `Cargo.toml`, and
    // the committed manifests, which state it, no longer equal the module.
    let mut dropped = record.clone();
    let row = dropped.allowed.remove(3);
    let committed = rust_packages::packages().remove(0);
    let gate_text = cargo_toml_of(&dropped, "n", "1.0.0");
    assert!(!gate_text.contains(&format!("{} =", row.lint)));
    assert_ne!(
        gate_text,
        cargo_toml_of(&record, "n", "1.0.0"),
        "{}",
        committed.directory
    );

    // A module the reader does not accept is refused, not read. Each plant
    // is one edit of the module's text.
    refused(
        scaffold::decode(&plant(
            &text,
            "name: clippyGates,",
            "name: clippyGatesElsewhere,",
        )),
        "clippyGatesElsewhere",
    );
    refused(
        scaffold::decode(&plant(
            &text,
            "{name: forbid, fields: []}",
            "{name: ban, fields: []}",
        )),
        "`Level` has the constructors",
    );
    refused(
        scaffold::decode(&plant(
            &text,
            "{name: priority, type: Int}",
            "{name: weight, type: Int}",
        )),
        "`Gate` has the fields",
    );
    refused(
        scaffold::decode(&plant(
            &text,
            "body: (record Scaffold {edition := \"2021\", publish := false, library := \"src/lib.rs\"})",
            "body: (if true then (record Scaffold {edition := \"2021\", publish := false, library := \"src/lib.rs\"}) else (record Scaffold {edition := \"2021\", publish := false, library := \"src/lib.rs\"}))",
        )),
        "is not a record",
    );
    refused(
        scaffold::decode(&plant(&text, ",\n    {name: library, type: String}", "")),
        "`Scaffold` has the fields",
    );
    refused(
        scaffold::decode(&plant(&text, "edition := \"2021\"", "edition := \"20 21\"")),
        "is not a plain name",
    );
    refused(
        scaffold::decode(&plant(
            &text,
            "lint := \"too_many_arguments\"",
            "lint := \"type_complexity\"",
        )),
        "allows `type_complexity` twice",
    );
    refused(scaffold::decode("not a module"), "holds no module");
}

/// The rows of the §17.16 lint table of `spec`: lint, group, reason.
fn specified_rows(spec: &str) -> Vec<(String, String, String)> {
    let table = spec
        .split("| Lint | Clippy group | Reason |\n| --- | --- | --- |\n")
        .nth(1)
        .expect("the §17.16 lint table");
    table
        .lines()
        .take_while(|line| line.starts_with('|'))
        .map(|line| {
            let cells: Vec<&str> = line.trim_matches('|').split(" | ").collect();
            let [lint, group, reason] = cells[..] else {
                panic!("a table row of three cells: {line}");
            };
            (
                lint.trim().trim_matches('`').to_owned(),
                group.trim().to_owned(),
                reason.trim().to_owned(),
            )
        })
        .collect()
}

fn table_matches(spec: &str, record: &Scaffold) -> Result<(), String> {
    let specified = specified_rows(spec);
    let module: Vec<(String, String, String)> = record
        .allowed
        .iter()
        .map(|row| {
            (
                row.lint.clone(),
                row.group.as_str().to_owned(),
                row.reason.clone(),
            )
        })
        .collect();
    if specified == module {
        return Ok(());
    }
    let mut broken = Vec::new();
    for row in &specified {
        if !module.contains(row) {
            broken.push(format!(
                "the specification's `{}` is not the module's",
                row.0
            ));
        }
    }
    for row in &module {
        if !specified.contains(row) {
            broken.push(format!(
                "the module's `{}` is not the specification's",
                row.0
            ));
        }
    }
    if broken.is_empty() {
        broken.push("the rows are in another order".to_owned());
    }
    Err(broken.join("; "))
}

/// §17.16: every allowed lint states its Clippy group, and none is
/// `correctness`.
fn rb_11() {
    let text = module_text();
    let record = scaffold::decode(&text).expect("the module decodes");
    assert!(
        record.allowed.len() > 1,
        "an empty list has no group to state"
    );
    assert!(scaffold::outside_correctness(&record.allowed));
    assert!(record
        .allowed
        .iter()
        .all(|row| row.group != ClippyGroup::Correctness));
    let spec = support::spec_text();
    table_matches(spec, &record).expect("the table is the module's list");

    // The module is a project of its own, and its linked IR checks (the
    // kernel's replay of the theorem is `verify`'s, which this case does not
    // run); a term the checker refuses is refused there.
    let project = support::P::package_project();
    project.relock();
    project.check_ok();
    let ill_typed = support::P::package_project();
    let notation = ill_typed.read_raw("src/RustPackage.lex.tex");
    ill_typed.write(
        "src/RustPackage.lex.tex",
        &plant(
            &notation,
            "(outsideCorrectness (allowedLints)) = true",
            "(outsideCorrectness (allowedLints)) = 3",
        ),
    );
    ill_typed.relock();
    assert!(
        !ill_typed.check_err_or_ok(),
        "an ill-typed theorem is refused by the project check"
    );

    // Each row of the table, moved, is refused naming the lint.
    let rows = specified_rows(spec);
    let first = &rows[0];
    let line = format!("| `{}` | {} | {} |", first.0, first.1, first.2);
    for (planted, expected) in [
        (
            spec.replace(&line, &format!("| `{}` | perf | {} |", first.0, first.2)),
            first.0.clone(),
        ),
        (spec.replace(&format!("{line}\n"), ""), first.0.clone()),
        (
            spec.replace(
                &line,
                &format!("{line}\n| `needless_return` | style | asks for no `return` |"),
            ),
            "needless_return".to_owned(),
        ),
        (
            spec.replace(&first.2, "asks for something else"),
            first.0.clone(),
        ),
    ] {
        assert_ne!(planted, spec, "the plant changes the specification");
        let error = table_matches(&planted, &record).expect_err("a planted table");
        assert!(error.contains(&expected), "{error}");
    }
    // A row dropped from the module is a row the specification has.
    let mut dropped = record.clone();
    let row = dropped.allowed.remove(0);
    let error = table_matches(spec, &dropped).expect_err("a dropped row");
    assert!(error.contains(&row.lint), "{error}");

    // The module refuses `correctness`, and a claim that says less.
    refused(
        scaffold::decode(&plant(
            &text,
            "lint := \"type_complexity\", group := (ctor ClippyGroup.complexity)",
            "lint := \"type_complexity\", group := (ctor ClippyGroup.correctness)",
        )),
        "`correctness` group",
    );
    refused(
        scaffold::decode(&plant(
            &text,
            "name: allowedLintsOutsideCorrectness,",
            "name: allowedLintsAreFine,",
        )),
        "allowedLintsAreFine",
    );
    refused(
        scaffold::decode(&plant(
            &text,
            "statement: ((outsideCorrectness (allowedLints)) = true)",
            "statement: (true = true)",
        )),
        "does not state `outsideCorrectness allowedLints = true`",
    );
    refused(
        scaffold::decode(&plant(
            &text,
            "| ClippyGroup.correctness => false",
            "| ClippyGroup.correctness => (outsideCorrectness rest)",
        )),
        "is not false exactly on `correctness`",
    );
}

/// §21.2: the language-1.2 ID covers the scaffold module.
fn rb_12() {
    let rows = support::normative_rows();
    let identities = |rows: &[(String, Vec<u8>)]| {
        let borrowed: Vec<(&str, &[u8])> = rows
            .iter()
            .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
            .collect();
        support::semantics_ids(&borrowed)
    };
    let (ten, eleven, twelve) = identities(&rows);
    // The harness is calibrated on the unchanged tree.
    assert_eq!(ten, lexlean::compiler_semantics_id_for("1.0"));
    assert_eq!(eleven, lexlean::compiler_semantics_id_for("1.1"));
    assert_eq!(twelve, lexlean::compiler_semantics_id_for("1.2"));
    let (only_1_2, _) = support::introduced_later();
    assert!(
        only_1_2.iter().any(|item| item == "language/package-1.2/"),
        "§21.2 lists the scaffold's directory among the files introduced for 1.2"
    );

    let edited = |edit: &dyn Fn(&str) -> String| {
        let mut out = rows.clone();
        let row = out
            .iter_mut()
            .find(|(path, _)| path == MODULE)
            .expect("the module is in the embedded tree");
        row.1 = edit(&String::from_utf8(row.1.clone()).expect("UTF-8")).into_bytes();
        out
    };
    // What the module says is part of LexLean's identity.
    for (what, copy) in [
        (
            "a lint row",
            edited(&|text| {
                plant(
                    text,
                    "lint := \"single_match\"",
                    "lint := \"single_match_else\"",
                )
            }),
        ),
        (
            "the edition",
            edited(&|text| plant(text, "edition := \"2021\"", "edition := \"2024\"")),
        ),
        (
            "a gate",
            edited(&|text| {
                plant(
                    text,
                    "lint := \"warnings\", level := (ctor Level.deny)",
                    "lint := \"warnings\", level := (ctor Level.warn)",
                )
            }),
        ),
    ] {
        let (ten_after, eleven_after, twelve_after) = identities(&copy);
        assert_ne!(twelve_after, twelve, "{what} changes the language-1.2 ID");
        assert_eq!(
            ten_after, ten,
            "{what}: language 1.0 does not hash the module"
        );
        assert_eq!(
            eleven_after, eleven,
            "{what}: language 1.1 does not hash the module"
        );
    }
    // The form it is written in is not.
    let data = {
        let mut out = rows.clone();
        let row = out
            .iter_mut()
            .find(|(path, _)| path == MODULE)
            .expect("the module");
        row.1 = lexlean::identity_bytes(MODULE, &row.1).into_owned();
        assert_ne!(
            row.1,
            rows.iter()
                .find(|(path, _)| path == MODULE)
                .expect("module")
                .1,
            "the data form is other bytes"
        );
        out
    };
    let spaced = edited(&|text| text.replace("\n\ndefinition{", "\n\n\n\ndefinition{"));
    for (what, copy) in [("its data form", data), ("reformatted notation", spaced)] {
        assert_eq!(identities(&copy).2, twelve, "{what} is the same identity");
    }
}

fn effect_ids(manifest: &Manifest) -> Vec<&str> {
    manifest.effects.iter().map(String::as_str).collect()
}

/// §17.13, §17.16: a package states its effects.
fn rb_13() {
    let package_schema = schema("rust-package.schema.json");
    let provenance_schema = schema("rust-provenance.schema.json");
    let mut shown = BTreeSet::new();
    let mut recursive = None;
    let mut overflowing = None;
    let mut plain = None;
    let mut plain_core = None;
    for committed in rust_packages::packages() {
        let manifest = committed.manifest.clone();
        let profile = committed.profile;
        let name = committed.directory.as_str();
        let effects = effect_ids(&manifest);
        let rendered = package::package(&manifest).expect("a package");
        let provenance: Json =
            serde_json::from_slice(&rendered.files["provenance.json"]).expect("provenance");
        assert_eq!(
            provenance["effects"],
            serde_json::json!(manifest.effects),
            "{name}"
        );
        let violations = crate::schema::validate(&provenance_schema, &provenance);
        assert!(violations.is_empty(), "{name}: {violations:?}");
        let manifest_json: Json =
            serde_json::from_slice(&manifest.to_file_bytes().expect("canonical")).expect("JSON");
        let violations = crate::schema::validate(&package_schema, &manifest_json);
        assert!(violations.is_empty(), "{name}: {violations:?}");
        shown.extend(manifest.effects.iter().cloned());
        if effects.contains(&"recursion") && recursive.is_none() {
            recursive = Some((manifest.clone(), profile));
        } else if effects == ["overflow"] && overflowing.is_none() {
            overflowing = Some((manifest.clone(), profile));
        } else if effects.is_empty() {
            if profile == Profile::Core && plain_core.is_none() {
                plain_core = Some(manifest.clone());
            }
            if plain.is_none() {
                plain = Some((manifest.clone(), profile));
            }
        }
    }
    let ((recursive, _), (_, _), (plain, _)) = (
        recursive.expect("a recursive fixture"),
        overflowing.expect("an overflowing fixture"),
        plain.expect("a plain fixture"),
    );
    assert!(
        shown.contains("recursion") && shown.contains("overflow"),
        "the fixtures show both effects the renderer can see: {shown:?}"
    );
    // The schemas require the member, and a package states it.
    let mut without =
        serde_json::from_slice::<Json>(&plain.to_file_bytes().expect("canonical")).expect("JSON");
    without.as_object_mut().expect("object").remove("effects");
    assert!(
        !crate::schema::validate(&package_schema, &without).is_empty(),
        "a manifest without effects is invalid"
    );

    let refused = |manifest: &Manifest, expected: &str| {
        let error = package::package(manifest).expect_err(expected);
        assert!(error.contains(expected), "{error}");
    };
    // The list is the root's, not derived from the program: a list that
    // states less than the program shows renders, because eligibility, and
    // not the renderer, decides a root's effects (the generated argument
    // check of a recursive type recurses, the graph templates' `nat_add` fuel
    // bound can overflow).
    let mut stated = recursive.clone();
    stated.effects.clear();
    package::package(&stated).expect("the effects are the root's to state");
    // Unregistered, unordered, and allocation where there is none.
    let mut unknown = plain.clone();
    unknown.effects = vec!["speed".to_owned()];
    refused(&unknown, "`speed` is not a registered effect");
    let mut unordered = recursive.clone();
    unordered.effects = vec!["recursion".to_owned(), "overflow".to_owned()];
    refused(&unordered, "effects are strictly ascending");
    let mut core = plain_core.expect("a plain rust-core fixture");
    package::package(&core).expect("the plain package renders in rust-core");
    core.effects = vec!["allocation".to_owned()];
    refused(&core, "hidden allocation");

    // The registry says what exhausting the stack does.
    let registry = lexlean::production::registry().expect("the registry");
    let statement = &registry.effects["recursion"].statement;
    assert!(
        statement.contains("aborts the process") && !statement.contains("explicit failure"),
        "{statement}"
    );
}
