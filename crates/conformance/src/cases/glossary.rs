//! The generated external-Lean glossary: GL-19, GL-20, and CL-25
//! (SPEC.md §13.12, §23.4).
//!
//! The planner is exercised on `tests/glossary/extraction.json`, the committed
//! output of the Lean extractor over `tests/glossary/GlossaryFixtures.lean`.
//! When the pinned toolchain is available each case also re-runs the extractor
//! and requires the committed file to equal its output, so the file cannot
//! drift from the metaprogram.

use camino::{Utf8Path, Utf8PathBuf};
use lexlean::glossary::extract::{self, Extraction, Selection};
use lexlean::glossary::plan::{self, PackageSpec, Plan};
use lexlean::glossary::{self, Request};
use lexlean::lexicon::entry::Denotation;
use lexlean::lexicon::package::{load_package, LoadContext};

use crate::support;

fn fixtures_dir() -> Utf8PathBuf {
    support::repo_root().join("tests/glossary")
}

fn committed() -> Extraction {
    let bytes = std::fs::read(fixtures_dir().join("extraction.json").as_std_path())
        .expect("tests/glossary/extraction.json");
    Extraction::parse(&bytes).expect("the committed extraction parses")
}

fn spec() -> PackageSpec {
    PackageSpec {
        package: "fix.glossary".to_owned(),
        version: "1.0.0".to_owned(),
        language: "1.0".to_owned(),
    }
}

fn planned(extraction: &Extraction) -> Plan {
    plan::plan(extraction, &spec()).expect("the planner accepts the extraction")
}

fn entry_text<'a>(plan: &'a Plan, id: &str) -> &'a str {
    let path = lexlean::lexicon::package::entry_path(id);
    let (_, bytes) = plan
        .files
        .iter()
        .find(|(file, _)| *file == path)
        .unwrap_or_else(|| panic!("the package has no entry file {path}"));
    std::str::from_utf8(bytes).expect("UTF-8")
}

/// The committed extraction equals a fresh one, when Lean is available.
fn extraction_is_current(id: &str) {
    if !support::lean_backed(id) {
        return;
    }
    let scratch = tempfile::tempdir().expect("tempdir");
    let dir = Utf8Path::from_path(scratch.path()).expect("utf8");
    std::fs::copy(
        fixtures_dir().join("GlossaryFixtures.lean").as_std_path(),
        dir.join("GlossaryFixtures.lean").as_std_path(),
    )
    .expect("copy the fixture module");
    let lean = lexlean::verify::toolchain::toolchain_root()
        .expect("toolchain root")
        .join("bin/lean");
    let status = std::process::Command::new(lean.as_std_path())
        .args(["-o", "GlossaryFixtures.olean", "GlossaryFixtures.lean"])
        .current_dir(dir.as_std_path())
        .status()
        .expect("lean runs");
    assert!(status.success(), "{id}: the fixture module compiles");
    let fresh = extract::run_with(
        &Selection::Modules(vec!["GlossaryFixtures".to_owned()]),
        None,
        Some(dir),
    )
    .expect("the extractor runs");
    assert_eq!(
        fresh,
        committed(),
        "{id}: tests/glossary/extraction.json is stale; run tests/glossary/regenerate.sh"
    );
}

/// GL-19: the generated package is a package.
pub(crate) fn package() {
    let extraction = committed();
    let first = planned(&extraction);
    let second = planned(&extraction);
    assert_eq!(first.files, second.files, "two plans, one package");
    assert_eq!(
        first.report.to_bytes(),
        second.report.to_bytes(),
        "two plans, one report"
    );

    // The package loader accepts the files exactly as written.
    let bootstrap = lexlean::lexicon::load_bootstrap_for("1.0").expect("bootstrap");
    let loaded = load_package(
        "fix.glossary",
        &first.files,
        None,
        &LoadContext {
            language: "1.0",
            forbidden_controls: &bootstrap.structural.forbidden_controls,
            max_scope_depth: 1024,
        },
    )
    .unwrap_or_else(|diagnostics| panic!("the loader rejects the package: {diagnostics:?}"));
    assert!(!loaded.entries.is_empty());
    for (id, entry) in &loaded.entries {
        let Denotation::Lean { module, name } = &entry.denotation else {
            panic!("{id} does not denote a Lean constant");
        };
        // The requested module's own declarations name it; support entries
        // (`HAdd.hAdd`, `Nat`) name the module of the core library that
        // defines them.
        if name.starts_with("Fix.") {
            assert_eq!(module, "GlossaryFixtures", "{id}");
        } else {
            assert!(module.starts_with("Init."), "{id}: {module}");
        }
    }

    // A theorem about a class: the instance argument is a binder of the
    // signature, and the application mentions explicit arguments only.
    let scale_zero = entry_text(&first, "fix.scale-zero");
    assert!(
        scale_zero.contains("category = \"proof-constant\""),
        "{scale_zero}"
    );
    assert!(scale_zero.contains("(instance "), "{scale_zero}");
    assert!(
        scale_zero.contains("(app (const fix.glossary::fix.scale.scale (u)) (nat 0) (local a1))"),
        "{scale_zero}"
    );
    assert!(
        scale_zero.contains("surface = \"fixScaleZero\""),
        "{scale_zero}"
    );
    assert!(scale_zero.contains("universes = [\"u\"]"), "{scale_zero}");
    assert!(
        scale_zero.contains("math = \"(seq (operator-name fixScaleZero) (paren (seq (slot 0) (token comma) (space) (slot 1))))\""),
        "{scale_zero}"
    );

    // Categories follow the shape of the type.
    for (id, category) in [
        ("fix.tree", "type-noun"),
        ("fix.always", "predicate-constant"),
        ("fix.small", "adjective-predicate"),
        ("fix.between", "transitive-predicate"),
        ("fix.tree.leaf", "term-constant"),
        ("fix.tree.size", "function"),
        ("fix.scale", "function"),
    ] {
        assert!(
            entry_text(&first, id).contains(&format!("category = \"{category}\"")),
            "{id} is a {category}"
        );
    }
    assert!(entry_text(&first, "fix.small").contains("surface = \"small\""));

    // An inductive type carries its eliminator over the constructor entries.
    let tree = entry_text(&first, "fix.tree");
    for expected in [
        "cases_lean_name = \"Fix.Tree.casesOn\"",
        "induction_lean_name = \"Fix.Tree.rec\"",
        "entry = \"fix.glossary::fix.tree.leaf\"",
        "entry = \"fix.glossary::fix.tree.node\"",
        "fields = [\"left\", \"right\", \"label\"]",
        "induction_hypotheses = [\"leftih\", \"rightih\"]",
    ] {
        assert!(tree.contains(expected), "{expected} in {tree}");
    }
    let tree_entry = &loaded.entries["fix.tree"];
    let eliminator = tree_entry.eliminator.as_ref().expect("an eliminator");
    assert_eq!(eliminator.constructors.len(), 2);
    for constructor in &eliminator.constructors {
        assert!(
            loaded.entries.contains_key(&constructor.entry.entry),
            "the descriptor's constructor entry exists"
        );
    }

    // The same bytes through the public writer, twice.
    let one = tempfile::tempdir().expect("tempdir");
    let two = tempfile::tempdir().expect("tempdir");
    for directory in [&one, &two] {
        let root = Utf8Path::from_path(directory.path()).expect("utf8");
        glossary::generate_from(
            &Request {
                package: spec(),
                selection: Selection::Modules(vec!["GlossaryFixtures".to_owned()]),
                workspace: None,
                out: root.join("package"),
                report: Some(root.join("report.json")),
                strict: false,
            },
            &extraction,
        )
        .expect("the package is written");
    }
    let read = |directory: &tempfile::TempDir, relative: &str| {
        std::fs::read(directory.path().join(relative)).expect("written file")
    };
    assert_eq!(read(&one, "report.json"), read(&two, "report.json"));
    for (path, bytes) in &first.files {
        assert_eq!(&read(&one, &format!("package/{path}")), bytes, "{path}");
        assert_eq!(&read(&two, &format!("package/{path}")), bytes, "{path}");
    }
    extraction_is_current("GL-19");
}

/// GL-20: refusals are coded, complete, and never weakened.
pub(crate) fn refusals() {
    let extraction = committed();
    let plan = planned(&extraction);
    let report = &plan.report;

    let refusal = |name: &str| {
        report
            .refusals
            .iter()
            .find(|r| r.name == name)
            .unwrap_or_else(|| panic!("{name} is not refused"))
    };
    for (name, code, reason) in [
        ("Fix.polyId", "LLT4101", "sort-polymorphic"),
        ("Fix.withStrict", "LLT4102", "strict-implicit-binder"),
        ("Fix.projAxiom", "LLT4103", "primitive-projection"),
        ("Fix.strLit", "LLT4103", "string-literal"),
        ("Fix.proofEq", "LLT4104", "core-level-mismatch"),
        ("Fix.add_comm'", "LLT4105", "name-not-conservative"),
    ] {
        let found = refusal(name);
        assert_eq!(found.code, code, "{name}");
        assert_eq!(found.reason, reason, "{name}");
        assert!(found.requested && found.root.is_none(), "{name}");
    }
    let dependent = refusal("Fix.usesPoly");
    assert_eq!(dependent.code, "LLT4106");
    assert_eq!(dependent.reason, "dependency");
    assert_eq!(dependent.root.as_deref(), Some("Fix.polyId"));
    assert_eq!(dependent.root_reason.as_deref(), Some("sort-polymorphic"));
    assert!(
        dependent.detail.contains("Fix.polyId"),
        "{}",
        dependent.detail
    );

    // Never emitted: no entry file denotes a refused declaration.
    let refused: Vec<&str> = report.refusals.iter().map(|r| r.name.as_str()).collect();
    for (path, bytes) in &plan.files {
        let text = std::str::from_utf8(bytes).expect("UTF-8");
        for name in &refused {
            let denotation = format!("name = \"{name}\"");
            assert!(
                !text.lines().any(|line| line == denotation),
                "{path} denotes the refused {name}"
            );
        }
    }

    // The counts add up, and each count is what the refusals say.
    assert_eq!(
        report.requested,
        report.translated + report.refused + report.excluded
    );
    let requested_refused = report.refusals.iter().filter(|r| r.requested).count() as u64;
    assert_eq!(requested_refused, report.refused);
    let by_reason: u64 = report.refused_by_reason.values().map(|c| c.count).sum();
    let by_dependency: u64 = report.refused_by_dependency.values().sum();
    assert_eq!(by_reason + by_dependency, report.refused);
    assert_eq!(report.refused_by_reason["sort-polymorphic"].code, "LLT4101");
    let inherited = report
        .refusals
        .iter()
        .filter(|r| r.requested && r.root_reason.as_deref() == Some("sort-polymorphic"))
        .count() as u64;
    assert!(inherited >= 1);
    assert_eq!(report.refused_by_dependency["sort-polymorphic"], inherited);

    // LLT4107: the planner checks its own entries instead of trusting the
    // extraction. This extraction applies a one-argument predicate to two
    // arguments, which the Lean metaprogram would never produce.
    let mut bad = committed();
    let small = bad
        .declarations
        .iter()
        .position(|d| d.name == "Fix.small_two")
        .expect("Fix.small_two");
    bad.declarations[small].signature = Some("(app (const @Fix.Small) (nat 1) (nat 2))".to_owned());
    let checked = planned(&bad);
    let entry_check = checked
        .report
        .refusals
        .iter()
        .find(|r| r.name == "Fix.small_two")
        .expect("refused");
    assert_eq!(entry_check.code, "LLT4107");
    assert_eq!(entry_check.reason, "entry-check");
    assert!(
        !checked
            .files
            .iter()
            .any(|(path, _)| path.ends_with("small-two.toml")),
        "an entry that fails its own check is not written"
    );

    // Strict mode turns the first refusal into the error of its class, and
    // writes nothing.
    let scratch = tempfile::tempdir().expect("tempdir");
    let root = Utf8Path::from_path(scratch.path()).expect("utf8");
    let error = glossary::generate_from(
        &Request {
            package: spec(),
            selection: Selection::Modules(vec!["GlossaryFixtures".to_owned()]),
            workspace: None,
            out: root.join("package"),
            report: None,
            strict: true,
        },
        &extraction,
    )
    .expect_err("strict mode refuses");
    assert_eq!(error.class, lexlean::ErrorClass::Language);
    let codes: Vec<&str> = error.diagnostics.iter().map(|d| d.code.as_str()).collect();
    for code in [
        "LLT4101", "LLT4102", "LLT4103", "LLT4104", "LLT4105", "LLT4106",
    ] {
        assert!(codes.contains(&code), "{code} among {codes:?}");
    }
    assert!(!root.join("package").as_std_path().exists());
    extraction_is_current("GL-20");
}

/// CL-25: the command.
pub(crate) fn cli() {
    let scratch = tempfile::tempdir().expect("tempdir");
    let root = Utf8Path::from_path(scratch.path()).expect("utf8");
    let out = root.join("package");
    let nothing_written = || {
        assert!(
            !out.as_std_path().exists(),
            "a refused request writes nothing"
        );
    };

    // The selection and the identity are checked before any Lean runs.
    let (exit, _, stderr) = support::cli_in(
        root,
        &[
            "deps",
            "glossary",
            "--package",
            "t.pkg",
            "--out",
            out.as_str(),
        ],
    );
    assert_eq!(exit, 2, "{stderr}");
    assert!(stderr.contains("LLC0002"), "no selection: {stderr}");
    let (exit, _, stderr) = support::cli_in(
        root,
        &[
            "deps",
            "glossary",
            "--package",
            "t.pkg",
            "--out",
            out.as_str(),
            "--modules",
            "Init.Core",
            "--names",
            "Nat.add_comm",
        ],
    );
    assert_eq!(exit, 2, "{stderr}");
    assert!(stderr.contains("LLC0002"), "both selections: {stderr}");
    let (exit, _, stderr) = support::cli_in(
        root,
        &[
            "deps",
            "glossary",
            "--package",
            "t.pkg",
            "--out",
            out.as_str(),
            "--names",
            "Nat.add_comm",
            "Nat.add_comm",
        ],
    );
    assert_eq!(exit, 2, "{stderr}");
    assert!(stderr.contains("LLC0002"), "a repeated name: {stderr}");
    let (exit, _, stderr) = support::cli_in(
        root,
        &[
            "deps",
            "glossary",
            "--package",
            "Bad_Package",
            "--out",
            out.as_str(),
            "--names",
            "Nat.add_comm",
        ],
    );
    assert_eq!(exit, 2, "{stderr}");
    assert!(
        stderr.contains("LLC0001"),
        "a malformed package ID: {stderr}"
    );
    let (exit, _, stderr) = support::cli_in(
        root,
        &[
            "deps",
            "glossary",
            "--package",
            "t.pkg",
            "--out",
            out.as_str(),
            "--names",
            "Nat.add-comm",
        ],
    );
    assert_eq!(exit, 2, "{stderr}");
    assert!(stderr.contains("LLC0001"), "a malformed name: {stderr}");
    let (exit, _, stderr) = support::cli_in(root, &["deps", "glossary"]);
    assert_eq!(exit, 2, "{stderr}");
    assert!(
        stderr.contains("LLC0001"),
        "missing required options: {stderr}"
    );
    nothing_written();

    if !support::lean_backed("CL-25") {
        return;
    }

    // A Lean core request writes the package and the report, identically
    // twice.
    let run = |name: &str| {
        let package = root.join(format!("{name}-package"));
        let report = root.join(format!("{name}-report.json"));
        let (exit, stdout, stderr) = support::cli_in(
            root,
            &[
                "deps",
                "glossary",
                "--package",
                "core.nat",
                "--out",
                package.as_str(),
                "--report",
                report.as_str(),
                "--names",
                "Nat.add_comm",
                "Nat.le_antisymm",
                "Nat.succ_le_of_lt",
                "List.append_assoc",
                "Nat",
                "Nat.le",
                "id",
            ],
        );
        assert_eq!(exit, 0, "{stderr}");
        assert!(stdout.contains("core.nat@1.0.0: 7 requested"), "{stdout}");
        (package, report)
    };
    let (package_a, report_a) = run("a");
    let (package_b, report_b) = run("b");
    assert_eq!(
        std::fs::read(report_a.as_std_path()).expect("report"),
        std::fs::read(report_b.as_std_path()).expect("report"),
    );
    let files = |package: &Utf8Path| {
        let mut out: Vec<(String, Vec<u8>)> = walkdir::WalkDir::new(package.as_std_path())
            .into_iter()
            .flatten()
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| {
                (
                    entry
                        .path()
                        .strip_prefix(package.as_std_path())
                        .expect("under the package")
                        .to_string_lossy()
                        .into_owned(),
                    std::fs::read(entry.path()).expect("read"),
                )
            })
            .collect();
        out.sort();
        out
    };
    let written = files(&package_a);
    assert_eq!(written, files(&package_b), "two runs, one package");
    let bootstrap = lexlean::lexicon::load_bootstrap_for("1.0").expect("bootstrap");
    load_package(
        "core.nat",
        &written,
        None,
        &LoadContext {
            language: "1.0",
            forbidden_controls: &bootstrap.structural.forbidden_controls,
            max_scope_depth: 1024,
        },
    )
    .unwrap_or_else(|diagnostics| panic!("the loader rejects the package: {diagnostics:?}"));
    let report: serde_json::Value =
        serde_json::from_slice(&std::fs::read(report_a.as_std_path()).expect("report"))
            .expect("the report is JSON");
    assert_eq!(report["spec"], "lexlean/glossary-report/1");
    assert_eq!(report["requested"], 7);
    assert_eq!(
        report["refused_by_reason"]["sort-polymorphic"]["code"],
        "LLT4101"
    );

    // --strict exits with the class of the first refusal's code, and writes
    // nothing.
    let strict_out = root.join("strict-package");
    let (exit, _, stderr) = support::cli_in(
        root,
        &[
            "deps",
            "glossary",
            "--package",
            "core.nat",
            "--out",
            strict_out.as_str(),
            "--strict",
            "--names",
            "id",
        ],
    );
    assert_eq!(exit, 1, "{stderr}");
    assert!(stderr.contains("LLT4101"), "{stderr}");
    assert!(!strict_out.as_std_path().exists());
    // The JSON mode carries the same diagnostics in the command result.
    let (exit, stdout, _) = support::cli_in(
        root,
        &[
            "--diagnostic-format",
            "json",
            "deps",
            "glossary",
            "--package",
            "core.nat",
            "--out",
            strict_out.as_str(),
            "--strict",
            "--names",
            "id",
        ],
    );
    assert_eq!(exit, 1);
    assert!(
        stdout.contains("\"command\":\"deps\"") && stdout.contains("LLT4101"),
        "{stdout}"
    );
}
