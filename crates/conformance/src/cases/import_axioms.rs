//! VR-25 and VR-26: the axiom audit and the status wording of imported
//! declarations (SPEC.md §4.2, §5.3, §5.4, §18.8, §22.6).
//!
//! The fixture is `tests/fixtures/import-axioms`: a host library `AxHost`
//! whose five theorems state the commutativity of `Nat` addition and carry,
//! respectively, no axiom, `sorryAx`, `Lean.ofReduceBool`,
//! `Lean.trustCompiler`, and the axiom `native_decide` generates; and a
//! project whose five entries denote them. The cases build the host with
//! `lake build`, write the document each case needs, and verify.

use std::process::Command;

use crate::support::{self, P};

const PROJECT: &str = "fixtures/import-axioms/project";

/// The status of an imported declaration (SPEC.md §5.3).
const IMPORTED_STATUS: &str = "replayed in the pinned environment";

/// One planted entry: its id, surface, the code that refuses it, the Lean
/// declaration it denotes, and a word its refusal must carry.
struct Planted {
    entry: &'static str,
    surface: &'static str,
    code: &'static str,
    lean_name: &'static str,
    axiom: &'static str,
}

const PLANTED: [Planted; 4] = [
    Planted {
        entry: "sorry-comm",
        surface: "sorrycomm",
        code: "LLV7023",
        lean_name: "AxHost.viaSorry",
        axiom: "sorryAx",
    },
    Planted {
        entry: "reduce-comm",
        surface: "reducecomm",
        code: "LLV7024",
        lean_name: "AxHost.viaReduce",
        axiom: "Lean.ofReduceBool",
    },
    Planted {
        entry: "trust-comm",
        surface: "trustcomm",
        code: "LLV7025",
        lean_name: "AxHost.viaTrust",
        axiom: "Lean.trustCompiler",
    },
    Planted {
        entry: "native-comm",
        surface: "nativecomm",
        code: "LLV7025",
        lean_name: "AxHost.viaNative",
        axiom: "native_decide",
    },
];

/// A document with one theorem per `(entry id, surface, policy)`, each closed
/// by its entry.
fn document(uses: &[(&str, &str, &str)]) -> String {
    let mut text = String::from(
        "\\begin{lexlean}{Main}\n\\useglossary{lexlean.std.int@1.0.0}\n\\useglossary{lexlean.std.nat@1.0.0}\n\\useglossary{test.ax@1.0.0}\n\\title{Natural number addition}\n",
    );
    for (index, (entry, surface, policy)) in uses.iter().enumerate() {
        text.push_str(&format!(
            "\n\\begin{{section}}{{s{index}}}\n\\heading{{Natural number}}\n\\parameters{{natural number \\(a\\); natural number \\(b\\)}}\n  \\begin{{theorem}}{{use-{entry}}}\n  {policy}\n  \\(a + b = b + a\\).\n  \\begin{{proof}}\n  Close the goal with \\({surface}(a, b)\\).\n  \\end{{proof}}\n  \\end{{theorem}}\n\\end{{section}}\n"
        ));
    }
    text.push_str("\\end{lexlean}\n");
    text
}

/// A copy of the fixture with its host library built (offline: the library
/// has no dependency).
fn prepared() -> P {
    let project = P::copy_tests_project(PROJECT);
    let output = Command::new("lake")
        .arg("build")
        .arg("AxHost")
        .current_dir(project.root.as_std_path())
        .output()
        .expect("lake runs; is leanprover/lean4:v4.32.1 installed?");
    assert!(
        output.status.success(),
        "the host library builds: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    project
}

fn use_documents(project: &P, uses: &[(&str, &str, &str)]) {
    project.write_raw("src/Main.lex.tex", &document(uses));
    project.relock();
}

fn verify(project: &P) -> Result<lexlean::VerifiedProject, lexlean::LexLeanError> {
    project.engine().verify(lexlean::VerifyRequest {
        selection: lexlean::Selection::Entrypoints,
    })
}

fn attestation(outcome: &lexlean::VerifiedProject) -> serde_json::Value {
    serde_json::from_slice(
        &std::fs::read(outcome.root.join("attestation.json").as_std_path())
            .expect("attestation exists"),
    )
    .expect("attestation parses")
}

/// The refusal of `planted` under `policy`: the code, the entry, the Lean
/// name, and the axiom the message carries, and no verified directory.
fn refused(project: &P, planted: &Planted, policy: &str) {
    refused_with(project, planted, policy, planted.code, planted.axiom);
}

fn refused_with(project: &P, planted: &Planted, policy: &str, code: &str, axiom: &str) {
    use_documents(project, &[(planted.entry, planted.surface, policy)]);
    let error = project.verify_fails_with(code);
    let text = error.to_string();
    for needle in [
        format!("test.ax::{}", planted.entry),
        planted.lean_name.to_owned(),
        axiom.to_owned(),
    ] {
        assert!(
            text.contains(needle.as_str()),
            "{} under `{policy}` names `{needle}`: {text}",
            planted.entry
        );
    }
    let published = project.root.join(".lexlean/verified");
    assert!(
        !published.as_std_path().exists()
            || support::file_set(&published)
                .iter()
                .all(|name| !name.ends_with("attestation.json")),
        "a refused import publishes no attestation"
    );
}

fn observed_of(rows: &serde_json::Value, key: &str, value: &str) -> Vec<String> {
    let row = rows
        .as_array()
        .expect("rows are an array")
        .iter()
        .find(|row| row[key] == value)
        .unwrap_or_else(|| panic!("no row with {key} = {value}: {rows}"));
    row["observed"]
        .as_array()
        .expect("observed is an array")
        .iter()
        .map(|axiom| axiom.as_str().expect("axiom is a string").to_owned())
        .collect()
}

/// The accepted project: three imports, each under a policy that names its
/// axioms. The native axiom cannot be named, so it has no accepted form.
fn accepted(project: &P) -> serde_json::Value {
    use_documents(
        project,
        &[
            ("clean-comm", "cleancomm", "\\noaxioms"),
            ("sorry-comm", "sorrycomm", "\\allowaxioms{sorryAx}"),
            (
                "reduce-comm",
                "reducecomm",
                "\\exactaxioms{Lean.ofReduceBool;Lean.trustCompiler}",
            ),
            (
                "trust-comm",
                "trustcomm",
                "\\allowaxioms{Lean.trustCompiler}",
            ),
        ],
    );
    let outcome = verify(project).expect("imports whose axioms a policy names verify");
    attestation(&outcome)
}

/// VR-25: record, refuse, and accept under an explicit policy.
pub(super) fn audit() {
    if !support::lean_backed("VR-25") {
        return;
    }
    let project = prepared();

    // Every planted import is refused with its own code when the document's
    // policy does not name its axiom: under `\noaxioms`, and under an allow
    // list that names something else.
    for planted in &PLANTED {
        refused(&project, planted, "\\noaxioms");
        refused(&project, planted, "\\allowaxioms{propext}");
    }

    // `Lean.ofReduceBool` rests on `Lean.trustCompiler`, so a policy that
    // names only the first leaves the second unnamed.
    let half = &PLANTED[1];
    refused_with(
        &project,
        half,
        "\\allowaxioms{Lean.ofReduceBool}",
        "LLV7025",
        "Lean.trustCompiler",
    );

    // The axiom `native_decide` generates carries a hygiene mark that no
    // policy can spell, so it stays refused whatever a policy lists.
    refused(
        &project,
        &PLANTED[3],
        "\\allowaxioms{AxHost.viaNative._native.native_decide.ax_1_1}",
    );

    // A policy that names an axiom accepts the import that carries it, and
    // the attestation records the observed set of every import.
    let attestation = accepted(&project);
    let imports = &attestation["imports"];
    assert_eq!(imports.as_array().expect("imports are recorded").len(), 4);
    assert!(observed_of(imports, "name", "AxHost.clean").is_empty());
    assert_eq!(observed_of(imports, "name", "AxHost.viaSorry"), ["sorryAx"]);
    assert_eq!(
        observed_of(imports, "name", "AxHost.viaReduce"),
        ["Lean.ofReduceBool", "Lean.trustCompiler"]
    );
    assert_eq!(
        observed_of(imports, "name", "AxHost.viaTrust"),
        ["Lean.trustCompiler"]
    );
    // Only the used imports outside the toolchain's own libraries are
    // recorded: `Nat` and `Nat.add` of the signatures are not.
    let names: Vec<&str> = imports
        .as_array()
        .expect("array")
        .iter()
        .map(|row| row["name"].as_str().expect("name"))
        .collect();
    assert!(
        names.iter().all(|name| name.starts_with("AxHost.")),
        "{names:?}"
    );

    // The generated declarations are still held to their own policies: the
    // theorem over `AxHost.viaSorry` records the observed `sorryAx`.
    let sorry_row = attestation["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .find(|row| {
            row["name"]
                .as_str()
                .is_some_and(|name| name.contains("sorry"))
        })
        .expect("the theorem over the sorry import has a policy row");
    assert_eq!(sorry_row["observed"], serde_json::json!(["sorryAx"]));
    assert_eq!(sorry_row["policy"]["kind"], "allow");
}

/// VR-26: the status words of imported declarations.
pub(super) fn status() {
    // The specification states the wording where it states the evidence
    // states.
    let spec = support::spec_text();
    let section = spec
        .split("### 5.3 Evidence states")
        .nth(1)
        .and_then(|rest| rest.split("### 5.4").next())
        .expect("§5.3 exists");
    assert!(section.contains(IMPORTED_STATUS), "§5.3 states the wording");
    assert!(
        section.contains("never") && section.contains("proved by LexLean"),
        "§5.3 forbids the claim"
    );
    assert_eq!(lexlean::verify::imports::IMPORTED_STATUS, IMPORTED_STATUS);

    if !support::lean_backed("VR-26") {
        return;
    }
    let project = prepared();
    let attestation = accepted(&project);
    // The project is verified; its imports are replayed in the pinned
    // environment.
    assert_eq!(attestation["status"], "verified");
    let imports = attestation["imports"].as_array().expect("imports");
    assert!(!imports.is_empty());
    for row in imports {
        assert_eq!(row["status"], IMPORTED_STATUS, "{row}");
        let text = row.to_string().to_lowercase();
        for word in ["proved", "verified", "kernel-checked"] {
            assert!(
                !text.contains(word),
                "`{word}` in the row of an import: {text}"
            );
        }
    }
}
