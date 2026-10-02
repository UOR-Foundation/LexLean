//! Conformance cases for semantic preservation (SPEC.md §17.17).

use std::collections::BTreeSet;
use std::sync::OnceLock;

use lexlean::calculus::Outcome;
use lexlean::production::lower::{linked_modules, lower_root, roots};
use lexlean::production::preserve::{self, audit, audit_tokens, module_text};

use crate::preservation::{self, Mutation, Report};
use crate::support::{self, repo_root, P};

/// The projects whose every production root is certified: the example that
/// declares production roots and the coverage example, whose roots exercise
/// every runtime row of the production registry.
fn certified_projects() -> [(&'static str, P); 2] {
    [
        ("production", P::copy_example("production")),
        (
            "production-coverage",
            P::copy_example("production-coverage"),
        ),
    ]
}

/// Certifying both projects takes minutes; the cases share one run.
fn reports() -> &'static Vec<(&'static str, Report)> {
    static REPORTS: OnceLock<Vec<(&'static str, Report)>> = OnceLock::new();
    REPORTS.get_or_init(|| {
        certified_projects()
            .into_iter()
            .map(|(name, project)| (name, preservation::certify(&project, name)))
            .collect()
    })
}

/// Every language-1.2 example.
fn lowered_projects() -> Vec<(String, P)> {
    let mut out = Vec::new();
    let mut names: Vec<String> = std::fs::read_dir(repo_root().join("examples").as_std_path())
        .expect("examples")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    for name in names {
        let config = repo_root()
            .join("examples")
            .join(&name)
            .join("lexlean.toml");
        let text = std::fs::read_to_string(config.as_std_path()).unwrap_or_default();
        if text.contains("language = \"1.2\"") {
            out.push((name.clone(), P::copy_example(&name)));
        }
    }
    out
}

/// The runtime rows of the production registry that no root of `roots`
/// exercises: a row counts where a root's report realizes its construct,
/// and `type.parameter` where a root instantiates a generic definition.
fn unexercised(reports: &[&lexlean::production::RootReport]) -> Vec<String> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for report in reports {
        seen.extend(report.constructs.keys().cloned());
        if report
            .runtime
            .iter()
            .any(|member| !member.type_arguments.is_empty())
        {
            seen.insert("type.parameter".to_owned());
        }
    }
    let text = std::fs::read_to_string(
        repo_root()
            .join(lexlean::production::REGISTRY_PATH)
            .as_std_path(),
    )
    .expect("the production registry");
    let registry: toml::Value = toml::from_str(&text).expect("the registry parses");
    registry["construct"]
        .as_array()
        .expect("construct rows")
        .iter()
        .filter(|row| row["disposition"].as_str() == Some("runtime"))
        .filter_map(|row| row["key"].as_str())
        .filter(|key| !seen.contains(*key))
        .map(str::to_owned)
        .collect()
}

/// Run one preservation case.
///
/// # Panics
///
/// Panics when the case's claim does not hold.
#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    match id {
        // §17.17: the lowering.
        "SP-01" => {
            let mut lowered = 0usize;
            for (name, project) in lowered_projects() {
                let checked = support::checked_project(&project);
                let modules = linked_modules(&checked);
                for root in roots(&checked).expect("the eligibility reports") {
                    let first = lower_root(&modules, &root.module, &root.name, root.report)
                        .unwrap_or_else(|d| panic!("{name}: {}: {d:?}", root.report.root));
                    let second = lower_root(&modules, &root.module, &root.name, root.report)
                        .expect("lowers again");
                    assert_eq!(
                        first.program.to_file_bytes(),
                        second.program.to_file_bytes(),
                        "{}: two lowerings differ",
                        root.report.root
                    );
                    assert_eq!(
                        first.program.canonical().expect("a valid program"),
                        first.program,
                        "{}: not in first-binding order",
                        root.report.root
                    );
                    assert_eq!(
                        first.layout.functions.len(),
                        first.program.functions.len(),
                        "{}: a function without an origin",
                        root.report.root
                    );
                    assert_eq!(
                        first.layout.adts.len(),
                        first.program.adts.len(),
                        "{}: an ADT without an origin",
                        root.report.root
                    );
                    // A report whose closure omits a member disagrees with
                    // what the lowering reaches.
                    let mut planted = root.report.clone();
                    if planted.runtime.pop().is_some() {
                        let error = lower_root(&modules, &root.module, &root.name, &planted)
                            .expect_err("a closure disagreement is refused");
                        assert_eq!(error.code.as_str(), "LLI9001", "{error:?}");
                    }
                    lowered += 1;
                }
            }
            assert!(lowered > 0, "the examples declare production roots");
            let semantic = std::fs::read_to_string(
                repo_root()
                    .join(repo_model::exhaustive::SEMANTIC_SOURCE)
                    .as_std_path(),
            )
            .expect("the IR source");
            for path in repo_model::exhaustive::PRESERVATION_SOURCES {
                let text =
                    std::fs::read_to_string(repo_root().join(path).as_std_path()).expect(path);
                repo_model::exhaustive::audit_preservation(path, &text, &semantic)
                    .unwrap_or_else(|report| panic!("{report}"));
                for planted in [
                    "\nfn planted(t: Option<u8>) { if let Some(_) = t {} }\n",
                    "\nfn planted(t: &SemanticType) -> bool { matches!(t, SemanticType::Nat) }\n",
                    "\nfn planted(t: &SemanticType) { match t { SemanticType::Nat => {} _ => {} } }\n",
                ] {
                    let error = repo_model::exhaustive::audit_preservation(
                        path,
                        &format!("{text}{planted}"),
                        &semantic,
                    )
                    .expect_err("a planted default is refused");
                    assert!(error.contains(path), "{error}");
                }
            }
        }
        // §17.17: certificate A.
        "SP-02" => {
            for (name, project) in certified_projects() {
                let certified = preservation::certificates(&project);
                assert!(!certified.is_empty(), "{name} declares production roots");
                for entry in &certified {
                    assert!(
                        !entry.targets.is_empty(),
                        "{}: eligible somewhere",
                        entry.root
                    );
                    assert!(
                        entry.certificate.text.contains("theorem root"),
                        "{}: the certificate states the root theorem",
                        entry.root
                    );
                }
            }
            if !support::lean_backed("SP-02") {
                return;
            }
            for (name, report) in reports() {
                assert!(!report.certified.is_empty(), "{name}: certified roots");
            }
            let planted = preservation::plant(&P::copy_example("production-coverage"));
            let kinds: BTreeSet<Mutation> = planted.iter().map(|plant| plant.mutation).collect();
            assert_eq!(
                kinds,
                Mutation::ALL.into_iter().collect(),
                "every mutation is planted somewhere in the coverage example"
            );
            for plant in &planted {
                assert!(
                    !plant.rejection.is_empty(),
                    "{}: the certificate of a program with a {:?} mutation was accepted",
                    plant.root,
                    plant.mutation
                );
            }
        }
        // §17.17: the differential evaluator.
        "SP-03" => {
            for (name, project) in certified_projects() {
                let cases = crate::differential::cases(&project);
                assert!(
                    cases.values().map(Vec::len).sum::<usize>() > 0,
                    "{name}: seeded cases"
                );
                for case in cases.values().flatten() {
                    assert!(
                        !matches!(case.outcome, Outcome::Exhausted | Outcome::Stuck),
                        "{}: the interpreter stopped on {:?}",
                        case.root,
                        case.arguments
                    );
                }
            }
            if !support::lean_backed("SP-03") {
                return;
            }
            for (name, report) in reports() {
                assert!(report.differential > 0, "{name}: cases compared");
                // One altered outcome is a disagreement.
                let mut planted = report.denotes.clone();
                let root = {
                    let case = planted
                        .iter_mut()
                        .flat_map(|(_, _, cases)| cases.iter_mut())
                        .next()
                        .expect("a case");
                    case.outcome = match case.outcome {
                        Outcome::Overflow { steps } => Outcome::Value {
                            value: lexlean::calculus::Value::Unit,
                            steps,
                        },
                        _ => Outcome::Overflow { steps: 0 },
                    };
                    case.root.clone()
                };
                let failures = crate::differential::compare(&report.differential_output, &planted)
                    .expect_err("a planted disagreement is detected");
                assert!(failures.contains(&root), "{failures}");
            }
        }
        // §17.17: the library and the shipped calculus modules.
        "SP-04" => {
            let library = preserve::library().expect("the shipped registry");
            assert!(
                !library.declaration.is_empty(),
                "the registry lists declarations"
            );
            crate::calculus::shipped_modules(repo_root().as_std_path(), false)
                .unwrap_or_else(|reason| panic!("{reason}"));
            let mut environment: BTreeSet<String> = preserve::TARGET_MODULES
                .iter()
                .map(|module| (*module).to_owned())
                .collect();
            for module in &library.modules {
                let text = module_text(module).expect("a shipped module");
                audit_tokens(text, &environment)
                    .unwrap_or_else(|reason| panic!("{module}: {reason}"));
                for planted in [
                    "\ntheorem planted : False := sorry\n",
                    "\naxiom planted : False\n",
                    "\ntheorem planted : 1 = 1 := by native_decide\n",
                    "\nset_option debug.skipKernelTC true\n",
                    "\nimport Mathlib\n",
                    "\ntheorem planted : True := Lean.ofReduceBool _ _ rfl\n",
                ] {
                    audit_tokens(&format!("{text}{planted}"), &environment)
                        .expect_err("a planted escape is refused");
                }
                environment.insert(module.clone());
            }
            if !support::lean_backed("SP-04") {
                return;
            }
            for (name, report) in reports() {
                let certificates: Vec<_> = report
                    .certified
                    .iter()
                    .map(|entry| entry.certificate.clone())
                    .collect();
                audit(&report.audit_output, &certificates)
                    .unwrap_or_else(|reason| panic!("{name}: {reason}"));
                // A declaration whose axioms drift from the registry.
                let first = &library.declaration[0];
                let drifted = report
                    .audit_output
                    .replace(
                        &format!("'{}' does not depend on any axioms", first.name),
                        &format!("'{}' depends on axioms: [sorryAx]", first.name),
                    )
                    .replace(
                        &format!("'{}' depends on axioms: [", first.name),
                        &format!("'{}' depends on axioms: [sorryAx, ", first.name),
                    );
                audit(&drifted, &certificates).expect_err("a drifted declaration is refused");
            }
        }
        // §17.17, §22.8, §22.9: certificate A in verification.
        "SP-05" => {
            if !support::lean_backed("SP-05") {
                return;
            }
            let project = P::copy_example("production");
            let verified = support::verify_ok(&project);
            let root = verified.root.as_std_path();
            let record_bytes =
                std::fs::read(root.join("preserve/preservation.json")).expect("preservation.json");
            let record: serde_json::Value =
                serde_json::from_slice(&record_bytes).expect("preservation.json is JSON");
            support::assert_schema("preservation", "preservation.json", &record);
            let rows = record["roots"].as_array().expect("roots");
            let checked = support::checked_project(&project);
            assert_eq!(
                rows.len(),
                roots(&checked).expect("the eligibility reports").len(),
                "one certificate per production root"
            );
            for row in rows {
                let module = row["module"].as_str().expect("a module");
                let text = std::fs::read(
                    root.join("preserve")
                        .join(lexlean::production::preserve::module_path(module)),
                )
                .expect("the published certificate");
                assert_eq!(
                    row["sha256"].as_str(),
                    Some(
                        lexlean::artifact::content_id::Sha256Digest::of(&text)
                            .to_hex()
                            .as_str()
                    ),
                    "{module}: the record binds the published certificate"
                );
            }
            let attestation: serde_json::Value = serde_json::from_slice(
                &std::fs::read(root.join("attestation.json")).expect("attestation"),
            )
            .expect("attestation JSON");
            assert_eq!(
                attestation["preservation"]["sha256"].as_str(),
                Some(
                    lexlean::artifact::content_id::Sha256Digest::of(&record_bytes)
                        .to_hex()
                        .as_str()
                ),
                "the attestation binds preservation.json"
            );
            for (fixture, code) in [
                ("certificate-rejected", "LLV7013"),
                ("preservation-drift", "LLV7014"),
            ] {
                let case =
                    crate::fixtures::load_case(&repo_root().join("tests/negative").join(fixture))
                        .expect("the fixture loads");
                let observed = crate::fixtures::observe(&case).expect("the fixture runs");
                assert_eq!(observed.codes, [code], "{fixture}");
                assert!(
                    !observed.project.root.join(".lexlean/verified").exists()
                        || std::fs::read_dir(
                            observed
                                .project
                                .root
                                .join(".lexlean/verified")
                                .as_std_path()
                        )
                        .map(|mut entries| entries.next().is_none())
                        .unwrap_or(true),
                    "{fixture}: nothing is published"
                );
            }
        }
        // §17.13, §17.17: the certified examples exercise the registry.
        "SP-06" => {
            let mut held = Vec::new();
            for (_, project) in certified_projects() {
                held.push(support::checked_project(&project));
            }
            let all: Vec<_> = held
                .iter()
                .map(|checked| roots(checked).expect("the reports"))
                .collect();
            let reports: Vec<&lexlean::production::RootReport> =
                all.iter().flatten().map(|root| root.report).collect();
            let missing = unexercised(&reports);
            assert!(
                missing.is_empty(),
                "runtime constructs no certified root exercises: {missing:?}"
            );
            // Withholding the collection roots leaves their constructs
            // unexercised, and the check says so.
            let withheld: Vec<&lexlean::production::RootReport> = reports
                .iter()
                .copied()
                .filter(|report| !report.root.starts_with("Coverage.Colls."))
                .collect();
            let missing = unexercised(&withheld);
            assert!(
                missing.iter().any(|key| key == "primitive.map_insert"),
                "withheld collection roots are reported: {missing:?}"
            );
        }
        _ => panic!("no preservation case is wired for {id}"),
    }
}
