//! The `reasoning` suite: RS-01..RS-14, language-1.2 reasoning machines
//! (SPEC.md §17.12): logics, inference rules, verifiers, reasoners, their
//! elaboration, generated theorems, runtime boundary, and production.
//!
//! Every mutation starts from the committed `reasoning` example or from a
//! committed negative fixture, so a refusal is the mutation's own and not an
//! unrelated defect's.

use std::collections::BTreeSet;
use std::sync::OnceLock;

use lexlean::calculus::{interp, Outcome, Value};
use lexlean::{CheckRequest, Selection, Sha256Digest, VerifyRequest};
use serde_json::{json, Value as Json};

use crate::support::{self, VerifiedFixture, P};

const EXAMPLE: &str = "reasoning";

/// The modules of the committed example.
const MODULES: [&str; 5] = ["Clinic", "Budget", "Planner", "Screening", "Main"];

/// The clinical rules, in declared priority order.
const RULES: [&str; 8] = [
    "Fever",
    "Tachycardia",
    "Tachypnea",
    "Leukocytosis",
    "Sirs",
    "Sepsis",
    "Shock",
    "Escalate",
];

/// An edit of one declaration's semantic JSON.
type Edit = fn(&mut Json);

// ---------------------------------------------------------------------------
// Source access.

fn module_path(module: &str) -> String {
    format!("src/{module}.lex.tex")
}

fn data_bounds(text: &str) -> (usize, usize) {
    let start = text.find("\\semanticdata{").expect("semantic data") + "\\semanticdata{".len();
    let end = text
        .rfind("}\n\\end{semanticmodule}")
        .expect("end of semantic data");
    (start, end)
}

fn module_data(project: &P, module: &str) -> Json {
    let text = project.read(&module_path(module));
    let (start, end) = data_bounds(&text);
    serde_json::from_str(&text[start..end]).expect("module data")
}

fn write_module_data(project: &P, module: &str, data: &Json) {
    let path = module_path(module);
    let text = project.read(&path);
    let (start, end) = data_bounds(&text);
    project.write(
        &path,
        &format!(
            "{}{}{}",
            &text[..start],
            serde_json::to_string(data).expect("serializes"),
            &text[end..]
        ),
    );
}

fn declarations_mut(data: &mut Json) -> &mut Vec<Json> {
    data["declarations"].as_array_mut().expect("declarations")
}

fn declaration_mut<'a>(data: &'a mut Json, name: &str) -> &'a mut Json {
    declarations_mut(data)
        .iter_mut()
        .find(|declaration| declaration["name"] == name)
        .unwrap_or_else(|| panic!("`{name}` is declared"))
}

/// A copy of the committed example with one declaration edited.
fn mutated(module: &str, name: &str, edit: impl FnOnce(&mut Json)) -> P {
    let project = P::copy_example(EXAMPLE);
    let mut data = module_data(&project, module);
    edit(declaration_mut(&mut data, name));
    write_module_data(&project, module, &data);
    project
}

/// A copy of the committed example with declarations appended to one
/// module.
fn extended(module: &str, added: Vec<Json>) -> P {
    let project = P::copy_example(EXAMPLE);
    let mut data = module_data(&project, module);
    declarations_mut(&mut data).extend(added);
    write_module_data(&project, module, &data);
    project
}

/// The mutation is refused with `code`, naming `message`, before either
/// backend runs.
fn refused(project: &P, code: &str, message: &str) {
    let error = project.check_fails_with(code);
    assert!(
        error.to_string().contains(message),
        "expected {message:?} under {code}, got {error}"
    );
    project.assert_no_backend_output_with(code);
}

/// Each committed negative fixture fails with `code`, naming its message.
fn negatives(code: &str, cases: &[(&str, &str)]) {
    for (name, message) in cases {
        let project = P::negative(&format!("reasoning-{name}"));
        let error = project.check_fails_with(code);
        assert!(
            error.to_string().contains(message),
            "tests/negative/reasoning-{name}: expected {message:?}, got {error}"
        );
    }
}

/// Each committed negative fixture links and is refused by verification.
fn refused_by_lean(id: &str, names: &[&str]) {
    if !support::lean_backed(id) {
        return;
    }
    let _guard = support::env_lock();
    for name in names {
        let project = P::negative(&format!("reasoning-{name}"));
        project.check_ok();
        project.verify_fails_with("LLV7002");
    }
}

// ---------------------------------------------------------------------------
// Snapshots, renderings, and the shared verified run.

fn snapshot(project: &P) -> lexlean::SemanticSnapshot {
    project
        .engine()
        .snapshot(CheckRequest {
            selection: Selection::Entrypoints,
        })
        .expect("snapshot")
}

fn elaboration<'a>(
    snapshot: &'a lexlean::SemanticSnapshot,
    module: &str,
    name: &str,
) -> &'a lexlean::SnapshotElaboration {
    snapshot
        .modules()
        .iter()
        .find(|candidate| candidate.name() == module)
        .unwrap_or_else(|| panic!("module {module}"))
        .declarations()
        .iter()
        .find(|declaration| declaration.logical_id() == name)
        .unwrap_or_else(|| panic!("`{module}.{name}` in the snapshot"))
        .elaboration()
        .unwrap_or_else(|| panic!("`{module}.{name}` carries its elaboration"))
}

fn names(values: &[Json]) -> Vec<String> {
    values
        .iter()
        .map(|value| value["name"].as_str().expect("name").to_owned())
        .collect()
}

/// The elaborated declaration `name` of an elaboration.
fn derived<'a>(elaboration: &'a lexlean::SnapshotElaboration, name: &str) -> &'a Json {
    elaboration
        .declarations()
        .iter()
        .find(|declaration| declaration["name"] == name)
        .unwrap_or_else(|| panic!("`{name}` is elaborated"))
}

/// Every value of `key` in objects of kind `kind` beneath `value`, in
/// document order.
fn collect(value: &Json, kind: &str, key: &str, out: &mut Vec<Json>) {
    match value {
        Json::Object(object) => {
            if object.get("kind").and_then(Json::as_str) == Some(kind) {
                if let Some(found) = object.get(key) {
                    out.push(found.clone());
                }
            }
            for child in object.values() {
                collect(child, kind, key, out);
            }
        }
        Json::Array(items) => {
            for item in items {
                collect(item, kind, key, out);
            }
        }
        _ => {}
    }
}

/// The names of the functions called beneath `value`, in document order.
fn calls(value: &Json) -> Vec<String> {
    let mut found = Vec::new();
    collect(value, "call", "function", &mut found);
    found
        .iter()
        .map(|member| member["name"].as_str().expect("name").to_owned())
        .collect()
}

fn verified_reasoning() -> &'static VerifiedFixture {
    static FIXTURE: OnceLock<VerifiedFixture> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        let _guard = support::env_lock();
        let project = P::copy_example(EXAMPLE);
        let outcome = project
            .engine()
            .verify(VerifyRequest {
                selection: Selection::Entrypoints,
            })
            .expect("the reasoning example verifies under pinned Lean");
        let attestation: Json = serde_json::from_slice(
            &std::fs::read(outcome.root.join("attestation.json").as_std_path())
                .expect("attestation exists"),
        )
        .expect("attestation parses");
        VerifiedFixture {
            project,
            outcome,
            attestation,
        }
    })
}

fn reasoning_backed(id: &str) -> Option<&'static VerifiedFixture> {
    support::lean_backed(id).then(verified_reasoning)
}

fn attested<'a>(attestation: &'a Json, name: &str) -> &'a Json {
    attestation["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .find(|row| row["name"] == name)
        .unwrap_or_else(|| panic!("the attestation records `{name}`"))
}

fn assert_attested_ok(attestation: &Json, names: &[&str]) {
    for name in names {
        assert_eq!(attested(attestation, name)["result"], "ok", "{name}");
    }
}

/// The `module`'s eligibility report in the rendered build.
fn eligibility(project: &P, module: &str) -> Json {
    let build = support::rendered(project);
    let path = format!("production/Reasoning/{module}.eligibility.json");
    serde_json::from_slice(
        &build
            .files
            .iter()
            .find(|(candidate, _)| *candidate == path)
            .unwrap_or_else(|| panic!("{path} is published"))
            .1,
    )
    .expect("report JSON")
}

fn root<'a>(report: &'a Json, name: &str) -> &'a Json {
    report["roots"]
        .as_array()
        .expect("roots")
        .iter()
        .find(|root| root["root"] == name)
        .unwrap_or_else(|| panic!("root {name}"))
}

/// Run the pinned Lean on `text` as a standalone module in `project`'s
/// workspace: whether it is accepted, and what it printed.
fn lean_run(project: &P, name: &str, text: &str) -> (bool, String) {
    let _guard = support::env_lock();
    let inner = lexlean::project::Project::load(&project.root.join("lexlean.toml")).expect("load");
    let toolchain =
        lexlean::verify::toolchain::preflight(&inner.config.limits).expect("the pinned toolchain");
    let scratch = project.root.join(".lexlean/runtime-scratch");
    std::fs::create_dir_all(scratch.as_std_path()).expect("scratch");
    let source = scratch.join(format!("{name}.lean"));
    std::fs::write(source.as_std_path(), text).expect("write module");
    let normalizer = lexlean::verify::child::Normalizer::new(
        &scratch,
        &project.root,
        &project.root,
        &toolchain.root,
    );
    let bin = toolchain.root.join("bin");
    let record = lexlean::verify::child::run(
        &lexlean::verify::child::ChildSpec {
            tool: "lean",
            module: Some(name.to_owned()),
            program: &toolchain.lake.path,
            executable_sha256: toolchain.lean.sha256,
            argv: vec!["env".to_owned(), "lean".to_owned(), source.to_string()],
            cwd: &project.root,
            extra_env: vec![("LEAN_PATH".to_owned(), scratch.to_string())],
            home: lexlean::verify::child::ChildHome::Toolchain {
                toolchain_bin: &bin,
            },
        },
        &inner.config.limits,
        &normalizer,
    )
    .expect("lean runs");
    (
        record.exit_code == 0 && !record.stdout.contains("error"),
        record.stdout,
    )
}

/// The `kind` constants of the alternatives beneath schema `$defs` entry
/// `definition`.
fn schema_kinds(schema: &Json, definition: &str) -> BTreeSet<String> {
    fn walk(value: &Json, out: &mut BTreeSet<String>) {
        match value {
            Json::Object(map) => {
                if let Some(constant) = map
                    .get("properties")
                    .and_then(|properties| properties.get("kind"))
                    .and_then(|kind| kind.get("const"))
                    .and_then(Json::as_str)
                {
                    out.insert(constant.to_owned());
                }
                for child in map.values() {
                    walk(child, out);
                }
            }
            Json::Array(items) => {
                for item in items {
                    walk(item, out);
                }
            }
            _ => {}
        }
    }
    let mut found = BTreeSet::new();
    walk(&schema["$defs"][definition], &mut found);
    found
}

fn strings(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

// ---------------------------------------------------------------------------
// The cases.

/// Run the case for one RS conformance ID.
///
/// # Panics
///
/// Panics when the case's assertion fails, and for an unwired ID.
pub fn run(id: &str) {
    match id {
        "RS-01" => rs_01(),
        "RS-02" => rs_02(),
        "RS-03" => rs_03(),
        "RS-04" => rs_04(),
        "RS-05" => rs_05(),
        "RS-06" => rs_06(),
        "RS-07" => rs_07(),
        "RS-08" => rs_08(),
        "RS-09" => rs_09(),
        "RS-10" => rs_10(),
        "RS-11" => rs_11(),
        "RS-12" => rs_12(),
        "RS-13" => rs_13(),
        "RS-14" => rs_14(),
        _ => panic!("no reasoning case {id}"),
    }
}

/// §17.12: the reasoning constructs belong to the closed schemas, are routed
/// out of language 1.1, and admit no member, strategy, or claim outside them.
#[allow(clippy::too_many_lines)]
fn rs_01() {
    let module = support::schema("semantic-module-v2");
    let declarations = schema_kinds(&module, "declaration");
    for kind in ["logic", "inference_rule", "verifier", "reasoner"] {
        assert!(declarations.contains(kind), "module schema lacks `{kind}`");
    }
    let strategies = strings(&["forward", "generate_and_verify", "search"]);
    let claims = strings(&["answer_correct", "initial_invariant", "terminates"]);
    assert_eq!(schema_kinds(&module, "reasoningStrategy"), strategies);
    assert_eq!(schema_kinds(&module, "reasoningClaim"), claims);
    assert!(schema_kinds(&module, "type").contains("reasoning_failure"));
    let snapshot_schema = support::schema("semantic-snapshot-v2");
    for prefix in ["elaboration_", "semantic_"] {
        assert_eq!(
            schema_kinds(&snapshot_schema, &format!("{prefix}reasoningStrategy")),
            strategies,
            "the snapshot schema closes over the same strategies"
        );
        assert_eq!(
            schema_kinds(&snapshot_schema, &format!("{prefix}reasoningClaim")),
            claims
        );
    }
    // Every committed module is an instance of the closed schema, and its
    // snapshot an instance of the snapshot schema.
    let project = P::copy_example(EXAMPLE);
    for name in MODULES {
        support::assert_schema(
            "semantic-module-v2",
            &format!("examples/reasoning/src/{name}.lex.tex"),
            &module_data(&project, name),
        );
    }
    let value: Json = serde_json::from_slice(&snapshot(&project).canonical_bytes()).expect("JSON");
    support::assert_schema("semantic-snapshot-v2", "the reasoning snapshot", &value);
    // Language 1.1 routes every construct out before either backend runs.
    let nat = json!({"kind": "nat"});
    let member = |name: &str| json!({"member": {"name": name}});
    let under_1_1 = [
        (
            json!({"kind": "inference_rule", "name": "N", "logic": member("L"),
                   "guard": {"kind": "bool", "value": true}, "conclusion": {"kind": "var", "name": "s"},
                   "soundness": {"name": "T"}}),
            "`inference_rule declaration` is a language-1.2 construct",
        ),
        (
            json!({"kind": "verifier", "name": "V", "subject": {"name": "x", "type": nat},
                   "candidate": {"name": "r", "type": nat}, "specification": {"name": "S"},
                   "check": {"name": "c"}, "sound": {"name": "T"}}),
            "`verifier declaration` is a language-1.2 construct",
        ),
        (
            json!({"kind": "reasoner", "name": "E", "observation": {"name": "x", "type": nat},
                   "rules": [], "strategy": {"kind": "generate_and_verify",
                   "generator": {"kind": "nil", "element": nat}, "budget": {"kind": "nat", "value": "1"}},
                   "verifier": member("V")}),
            "`reasoner declaration` is a language-1.2 construct",
        ),
        (
            json!({"kind": "definition", "name": "f",
                   "parameters": [{"name": "x", "type": {"kind": "reasoning_failure"}}],
                   "result": nat, "body": {"kind": "nat", "value": "0"}}),
            "`reasoning_failure type` is a language-1.2 construct",
        ),
    ];
    negatives(
        "LLT4001",
        &[(
            "under-1.1",
            "`logic declaration` is a language-1.2 construct",
        )],
    );
    for (declaration, message) in under_1_1 {
        let project = P::negative("reasoning-under-1.1");
        let mut data = module_data(&project, "Main");
        data["declarations"] = json!([declaration]);
        write_module_data(&project, "Main", &data);
        refused(&project, "LLT4001", message);
    }
    // No member, strategy, or claim outside the closed schema.
    let edits: [(&str, &str, Edit, &str); 8] = [
        (
            "Clinic",
            "Triage",
            |declaration| declaration["oracle"] = json!("ask the model which rule applies"),
            "unknown field `oracle`",
        ),
        (
            "Clinic",
            "Fever",
            |declaration| declaration["prompt"] = json!("decide whether the patient is febrile"),
            "unknown field `prompt`",
        ),
        (
            "Clinic",
            "Recommendation",
            |declaration| declaration["model"] = json!("a judge"),
            "unknown field `model`",
        ),
        (
            "Clinic",
            "Findings",
            |declaration| declaration["explanation"] = json!("free text"),
            "unknown field `explanation`",
        ),
        (
            "Clinic",
            "Triage",
            |declaration| declaration["strategy"]["kind"] = json!("neural"),
            "unknown variant `neural`",
        ),
        (
            "Planner",
            "Plan",
            |declaration| declaration["strategy"]["order"] = json!("random"),
            "unknown variant `random`",
        ),
        (
            "Screening",
            "Dose",
            |declaration| declaration["strategy"]["temperature"] = json!(1),
            "unknown field `temperature`",
        ),
        (
            "Clinic",
            "Triage",
            |declaration| {
                declaration["claims"]
                    .as_array_mut()
                    .expect("claims")
                    .push(json!({"kind": "calibrated", "theorem": {"name": "admitted_pending"}}));
            },
            "unknown variant `calibrated`",
        ),
    ];
    for (module, name, edit, message) in edits {
        refused(&mutated(module, name, edit), "LLT4001", message);
    }
    negatives(
        "LLT4001",
        &[
            ("binder-reserved", "invalid interface binder name `__s`"),
            ("opaque-member", "unknown field `oracle`"),
            ("unregistered-strategy", "unknown variant `neural`"),
            (
                "unregistered-claim",
                "unknown variant `calibrated_confidence`",
            ),
        ],
    );
}

/// §17.12: logics and rules, their exact obligations and elaborations.
fn rs_02() {
    let project = P::copy_example(EXAMPLE);
    let snapshot = snapshot(&project);
    let logic = elaboration(&snapshot, "Clinic", "Findings");
    assert_eq!(names(logic.theorems()), ["Findings.preserves"]);
    assert_eq!(logic.obligations().len(), 1);
    let fever = elaboration(&snapshot, "Clinic", "Fever");
    assert_eq!(
        names(fever.declarations()),
        ["Fever.guard", "Fever.conclusion", "Fever.apply"],
        "a binding-free rule is its guard, conclusion, and guarded application"
    );
    assert_eq!(
        names(fever.theorems()),
        ["Fever.apply_sound", "Fever.apply_progress"]
    );
    assert_eq!(fever.obligations().len(), 2, "soundness and progress");
    let pour = elaboration(&snapshot, "Planner", "Pour");
    assert_eq!(
        names(pour.declarations()),
        [
            "Pour.guard",
            "Pour.conclusion",
            "Pour.candidates",
            "Pour.apply"
        ],
        "a binding rule has its candidates"
    );
    // The guarded application is the only application: it tests the guard
    // before it yields the conclusion.
    let apply = derived(fever, "Fever.apply");
    assert_eq!(apply["body"]["kind"], "if");
    assert_eq!(calls(&apply["body"]), ["Fever.guard", "Fever.conclusion"]);
    // Generic rules keep their type parameters.
    for declaration in elaboration(&snapshot, "Budget", "Tick").declarations() {
        assert_eq!(
            declaration["type_parameters"],
            json!(["T"]),
            "{declaration}"
        );
    }
    negatives(
        "LLT4010",
        &[
            ("interface-rebound", "binds the interface name `s` twice"),
            ("relation-signature", "must take (Memo, Memo) to Prop"),
            (
                "invariant-unpreserved",
                "`sound_preserved` does not state exactly the generated obligation",
            ),
            ("rule-unknown-logic", "is not a prior logic"),
            (
                "rule-type-arguments",
                "expects 0 type argument(s), received 1",
            ),
            ("rule-guard-type", "guard has type Nat, expected Bool"),
            (
                "rule-candidates-type",
                "candidates has type Nat, expected List (Nat)",
            ),
            (
                "rule-soundness-inexact",
                "`fever_sound` does not state exactly the generated obligation",
            ),
            (
                "rule-progress-inexact",
                "`fever_progress` does not state exactly the generated obligation",
            ),
            (
                "progress-without-ranking",
                "states progress, but logic `Triage` has no ranking",
            ),
            (
                "rule-foreign-logic",
                "but rule `Echo` is over logic `Other`",
            ),
            (
                "relation-not-definition",
                "relation `sound_preserved` is not a prior definition",
            ),
            (
                "logic-type-parameters",
                "relation `Justified` has 0 type parameter(s); it must have exactly 1",
            ),
            (
                "invariant-signature",
                "invariant `Justified` must take (Memo) to Prop",
            ),
            (
                "ranking-signature",
                "ranking `Sound` must take (Memo) to Nat",
            ),
            (
                "rule-conclusion-type",
                "conclusion has type Nat, expected Memo",
            ),
        ],
    );
    // In the example: a weakened soundness theorem, and a generic reasoner
    // using its logic without the type argument.
    let weakened = mutated("Clinic", "fever_sound", |declaration| {
        declaration["statement"]["conclusion"] = json!({"kind": "le",
            "left": {"kind": "nat", "value": "0"}, "right": {"kind": "nat", "value": "1"}});
    });
    refused(
        &weakened,
        "LLT4010",
        "`fever_sound` does not state exactly the generated obligation",
    );
    let untyped = mutated("Budget", "Spend", |declaration| {
        declaration["logic"]["type_arguments"] = json!([]);
    });
    refused(
        &untyped,
        "LLT4010",
        "expects 1 type argument(s), received 0",
    );
}

/// §17.12, §22.6: verifiers and their exact, Lean-restated theorems.
fn rs_03() {
    let project = P::copy_example(EXAMPLE);
    let snapshot = snapshot(&project);
    assert_eq!(
        names(elaboration(&snapshot, "Clinic", "Recommendation").theorems()),
        ["Recommendation.sound", "Recommendation.complete"]
    );
    assert_eq!(
        names(elaboration(&snapshot, "Clinic", "Outpatient").theorems()),
        ["Outpatient.sound"],
        "completeness is restated only when it is claimed"
    );
    negatives(
        "LLT4010",
        &[
            ("verifier-signature", "must take (Nat, Nat) to Bool"),
            ("verifier-formal-check", "is not executable"),
            (
                "verifier-inexact",
                "`bounded_complete` does not state exactly the generated obligation",
            ),
            ("unknown-verifier", "is not a prior verifier"),
            (
                "specification-signature",
                "specification `Sound` must take (Nat, Nat) to Prop",
            ),
            ("unknown-rule", "`Bound` is not a prior inference rule"),
            (
                "generate-subject-mismatch",
                "observes Memo, but verifier `Bound` checks a Nat subject",
            ),
            (
                "verifier-type-mismatch",
                "but verifier `Bound` checks a Nat subject and a Nat candidate",
            ),
            (
                "generator-type",
                "generator has type Nat, expected List (Nat)",
            ),
        ],
    );
    if let Some(fixture) = reasoning_backed("RS-03") {
        assert_attested_ok(
            &fixture.attestation,
            &[
                "Reasoning.Clinic.Recommendation.sound",
                "Reasoning.Clinic.Recommendation.complete",
                "Reasoning.Clinic.Outpatient.sound",
                "Reasoning.Screening.SafeDose.sound",
            ],
        );
    }
    refused_by_lean("RS-03", &["inconsistent-verifier"]);
}

/// §17.12, §17.13: every reasoner is ordinary declarations over existing
/// primitives, and the formal runtime reaches no production closure.
fn rs_04() {
    let project = P::copy_example(EXAMPLE);
    let snapshot = snapshot(&project);
    let mut operations = BTreeSet::new();
    let mut reasoners = 0;
    for module in snapshot.modules() {
        for declaration in module.declarations() {
            if declaration.kind() != "reasoner" {
                continue;
            }
            reasoners += 1;
            let elaboration = declaration.elaboration().expect("an elaboration");
            for derived in elaboration.declarations() {
                assert!(
                    ["definition", "inductive", "structure"]
                        .contains(&derived["kind"].as_str().expect("kind")),
                    "{} elaborates to ordinary declarations: {derived}",
                    declaration.logical_id()
                );
                let mut found = Vec::new();
                collect(derived, "primitive", "operation", &mut found);
                operations.extend(
                    found
                        .iter()
                        .map(|operation| operation.as_str().expect("operation").to_owned()),
                );
            }
        }
    }
    assert_eq!(
        reasoners, 8,
        "Triage, Review, Spend, Refund, Plan, Dose, Screen, Grade"
    );
    // Only the existing primitives: every operation the language-1.2 term
    // schema admits, none of them a reasoning primitive.
    let admitted: BTreeSet<String> = {
        let mut found = BTreeSet::new();
        let text = support::schema("semantic-module-v2")["$defs"]["term"].to_string();
        for operation in &operations {
            if text.contains(&format!("\"{operation}\"")) {
                found.insert(operation.clone());
            }
        }
        found
    };
    assert_eq!(
        admitted, operations,
        "elaborations use only admitted primitives"
    );
    for needed in ["list_fold", "iterate_until", "set_insert", "set_contains"] {
        assert!(operations.contains(needed), "an elaboration uses {needed}");
    }
    // A generic reasoner's whole elaboration is generic.
    let spend = elaboration(&snapshot, "Budget", "Spend");
    for derived in spend.declarations() {
        assert_eq!(derived["type_parameters"], json!(["T"]), "{derived}");
    }
    for theorem in spend.theorems() {
        assert_eq!(theorem["type_parameters"], json!(["T"]), "{theorem}");
    }
    // The formal runtime is emitted, and no closure reaches it.
    let build = support::rendered(&project);
    assert!(support::lean_text(&build, "Clinic").contains("namespace LexLeanReasoning"));
    let report = eligibility(&project, "Main");
    for root in report["roots"].as_array().expect("roots") {
        for member in root["runtime_closure"].as_array().expect("closure") {
            let name = member["declaration"].as_str().expect("declaration");
            assert!(!name.contains("LexLeanReasoning"), "{name} is formal-only");
        }
    }
    if let Some(fixture) = reasoning_backed("RS-04") {
        let input = std::fs::read_to_string(
            fixture
                .outcome
                .root
                .join("production/compiler-input.json")
                .as_std_path(),
        )
        .expect("compiler input");
        assert!(
            !input.contains("LexLeanReasoning"),
            "Lean extracts no runtime lemma"
        );
    }
}

/// §17.12: forward chaining in priority order through guarded applications,
/// accounted in the ledger, answering only from a saturated state.
fn rs_05() {
    let project = P::copy_example(EXAMPLE);
    let snapshot = snapshot(&project);
    let triage = elaboration(&snapshot, "Clinic", "Triage");
    let guards: Vec<String> = RULES.iter().map(|rule| format!("{rule}.guard")).collect();
    assert_eq!(
        calls(&derived(triage, "Triage.select")["body"]),
        guards,
        "select tries the rules in declared priority order"
    );
    assert_eq!(
        calls(&derived(triage, "Triage.attempts")["body"]),
        guards,
        "attempts counts exactly the guards select evaluates"
    );
    let applications: Vec<String> = RULES.iter().map(|rule| format!("{rule}.apply")).collect();
    assert_eq!(
        calls(&derived(triage, "Triage.fire")["body"]),
        applications,
        "every step fires through its rule's guarded application"
    );
    assert!(calls(&derived(triage, "Triage.step")["body"]).contains(&"Triage.attempts".to_owned()));
    // The answer is concluded only from a saturated state, and the account
    // adds the final scan.
    let called = |name: &str| -> BTreeSet<String> {
        calls(&derived(triage, name)["body"]).into_iter().collect()
    };
    assert_eq!(
        called("Triage.verdict"),
        strings(&["Triage.conclude", "Triage.saturate"])
    );
    assert_eq!(
        called("Triage.account"),
        strings(&["Triage.attempts", "Triage.run"])
    );
    // A binding rule is tried in candidate order.
    let spend = elaboration(&snapshot, "Budget", "Spend");
    assert!(calls(&derived(spend, "Spend.select")["body"]).contains(&"Burn.candidates".to_owned()));
    // Reversing the priority order links, and the kernel refuses the
    // concrete derivations stated in the declared order.
    let reversed = mutated("Clinic", "Triage", |declaration| {
        declaration["rules"]
            .as_array_mut()
            .expect("rules")
            .reverse();
    });
    reversed.check_ok();
    let reversed_snapshot = snapshot_of(&reversed);
    let mut backwards = guards.clone();
    backwards.reverse();
    assert_eq!(
        calls(
            &derived(
                elaboration(&reversed_snapshot, "Clinic", "Triage"),
                "Triage.select"
            )["body"]
        ),
        backwards
    );
    if let Some(fixture) = reasoning_backed("RS-05") {
        assert_attested_ok(
            &fixture.attestation,
            &[
                "Reasoning.Main.triage_shock",
                "Reasoning.Main.triage_iterations",
                "Reasoning.Main.triage_well",
                "Reasoning.Main.review_exhausted",
                "Reasoning.Main.review_rejected",
                "Reasoning.Main.refund_drained",
                "Reasoning.Main.refund_unsolved",
                "Reasoning.Main.spend_drained",
                "Reasoning.Clinic.Triage.verdict_sound",
                "Reasoning.Clinic.Triage.explained",
            ],
        );
        let _guard = support::env_lock();
        reversed.verify_fails_with("LLV7002");
    }
}

fn snapshot_of(project: &P) -> lexlean::SemanticSnapshot {
    snapshot(project)
}

/// §17.12: bounded search and generate-and-verify.
fn rs_06() {
    let project = P::copy_example(EXAMPLE);
    let snapshot = snapshot(&project);
    let plan = elaboration(&snapshot, "Planner", "Plan");
    assert!(names(plan.declarations()).contains(&"Plan.fresh".to_owned()));
    let screen = elaboration(&snapshot, "Screening", "Screen");
    assert!(!names(screen.declarations()).contains(&"Screen.fresh".to_owned()));
    for (elaboration, name) in [(plan, "Plan"), (screen, "Screen")] {
        let theorems = names(elaboration.theorems());
        for bound in [
            "iterations_bounded",
            "frontier_bounded",
            "verdict_sound",
            "explained",
        ] {
            assert!(
                theorems.contains(&format!("{name}.{bound}")),
                "{name} generates {bound}"
            );
        }
    }
    let dose = elaboration(&snapshot, "Screening", "Dose");
    assert_eq!(
        names(dose.declarations()),
        [
            "Dose.Step",
            "Dose.Ledger",
            "Dose.candidates",
            "Dose.verify",
            "Dose.Trial",
            "Dose.try",
            "Dose.run",
            "Dose.failure",
            "Dose.verdict",
            "Dose",
            "Dose.answer",
        ]
    );
    assert!(names(dose.theorems()).contains(&"Dose.verifications_bounded".to_owned()));
    // The verifier checks every candidate before it is answered.
    assert_eq!(calls(&derived(dose, "Dose.verify")["body"]), ["safeCheck"]);
    negatives(
        "LLT4011",
        &[
            ("forward-without-fuel", "declares no fuel"),
            ("search-without-frontier", "declares no frontier"),
            ("zero-frontier", "bounds its frontier by zero"),
            (
                "deduplicate-unordered-state",
                "which is not an ordered key type",
            ),
            ("fuel-not-nat", "fuel has type Bool, expected Nat"),
            ("frontier-not-nat", "frontier has type Bool, expected Nat"),
            ("generate-without-budget", "declares no budget"),
            ("budget-not-nat", "budget has type Bool, expected Nat"),
        ],
    );
    negatives(
        "LLT4010",
        &[
            ("generate-with-rules", "applies no inference rule"),
            (
                "generate-with-logic",
                "names no logic, observed state, or answer",
            ),
            ("generate-claims", "makes no claim"),
            ("rules-without-logic", "so it names its logic"),
            ("no-rules", "names no inference rule"),
            ("duplicate-rule", "names two rules called `Fever`"),
            ("formal-rule", "which is not executable"),
            ("answer-type", "answer has type Nat, expected Option (Nat)"),
            (
                "observation-type",
                "observation has type Nat, expected Memo",
            ),
        ],
    );
    if let Some(fixture) = reasoning_backed("RS-06") {
        assert_attested_ok(
            &fixture.attestation,
            &[
                "Reasoning.Main.plan_found",
                "Reasoning.Main.plan_unsolved",
                "Reasoning.Main.plan_frontier",
                "Reasoning.Main.screen_found",
                "Reasoning.Main.screen_truncated",
                "Reasoning.Main.dose_found",
                "Reasoning.Main.dose_exhausted",
                "Reasoning.Screening.Dose.verdict_sound",
                "Reasoning.Screening.Dose.verifications_bounded",
                "Reasoning.Planner.Plan.frontier_bounded",
            ],
        );
    }
}

/// §17.12, §22.6: termination evidence.
fn rs_07() {
    let project = P::copy_example(EXAMPLE);
    let snapshot = snapshot(&project);
    let theorems =
        |module: &str, name: &str| names(elaboration(&snapshot, module, name).theorems());
    assert!(theorems("Clinic", "Triage").contains(&"Triage.saturates".to_owned()));
    assert!(theorems("Budget", "Spend").contains(&"Spend.saturates".to_owned()));
    assert!(
        !theorems("Clinic", "Review").contains(&"Review.saturates".to_owned()),
        "a reasoner that claims no termination is not proved to saturate"
    );
    negatives(
        "LLT4011",
        &[
            (
                "terminating-search",
                "which only a forward reasoner can claim",
            ),
            (
                "terminates-without-progress",
                "rule `Pick` states no progress",
            ),
            ("terminates-without-initial", "claims no initial invariant"),
            (
                "terminates-without-ranking",
                "logic `Triage` has no ranking",
            ),
        ],
    );
    negatives(
        "LLT4010",
        &[
            ("fuel-bound-inexact", "`clinic_fuel` does not state exactly"),
            ("initial-inexact", "`clinic_initial` does not state exactly"),
            (
                "initial-without-invariant",
                "claims an initial invariant, but logic `Triage` has none",
            ),
            ("claims-unsorted", "claims are not strictly sorted by kind"),
        ],
    );
    if let Some(fixture) = reasoning_backed("RS-07") {
        assert_attested_ok(
            &fixture.attestation,
            &[
                "Reasoning.Clinic.Triage.saturates",
                "Reasoning.Budget.Spend.saturates",
                "Reasoning.Main.Grade.saturates",
            ],
        );
    }
    refused_by_lean("RS-07", &["nonterminating-rule", "insufficient-fuel"]);
}

/// The emitted `LexLeanReasoning` runtime of a rendered module, and the
/// text after it.
fn runtime_of(lean: &str) -> (&str, &str) {
    let start = lean.find("namespace LexLeanReasoning\n").expect("runtime");
    let end = lean[start..]
        .find("end LexLeanReasoning\n")
        .expect("runtime end")
        + start;
    (&lean[start..end], &lean[end..])
}

/// The names a runtime block declares.
fn runtime_names(runtime: &str) -> Vec<String> {
    runtime
        .lines()
        .filter_map(|line| {
            [
                "public theorem ",
                "@[expose] public def ",
                "public inductive ",
            ]
            .iter()
            .find_map(|prefix| line.strip_prefix(prefix))
        })
        .filter_map(|rest| rest.split_whitespace().next())
        .map(str::to_owned)
        .collect()
}

/// Whether `text` mentions the runtime name `name` as a whole word.
fn mentions(text: &str, name: &str) -> bool {
    text.match_indices(name).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + name.len()..].chars().next();
        let boundary = |c: Option<char>| c.is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
        boundary(after) && (boundary(before) || before == Some('.'))
    })
}

/// §17.12, §22.6: fixed templates over the axiom-free runtime, every lemma
/// and template used, a mutated lemma refused.
#[allow(clippy::too_many_lines)]
fn rs_08() {
    let project = P::copy_example(EXAMPLE);
    let build = support::rendered(&project);
    let clinic = support::lean_text(&build, "Clinic");
    let (runtime, _) = runtime_of(&clinic);
    let declared = runtime_names(runtime);
    assert!(declared.len() >= 30, "{declared:?}");
    // Every runtime name is used by a generated statement or proof of the
    // example, directly or through another runtime name so used.
    let generated: String = MODULES
        .iter()
        .map(|module| runtime_of(&support::lean_text(&build, module)).1.to_owned())
        .collect();
    // Each runtime name's block runs from its declaration to the next.
    let mut blocks: Vec<(&String, String)> = Vec::new();
    for line in runtime.lines() {
        match runtime_names(line).first() {
            Some(name) => {
                let name = declared
                    .iter()
                    .find(|candidate| *candidate == name)
                    .expect("declared");
                blocks.push((name, format!("{line}\n")));
            }
            None => {
                if let Some((_, block)) = blocks.last_mut() {
                    block.push_str(line);
                    block.push('\n');
                }
            }
        }
    }
    let mut used: BTreeSet<&String> = declared
        .iter()
        .filter(|name| mentions(&generated, &format!("LexLeanReasoning.{name}")))
        .collect();
    loop {
        let before = used.len();
        for (name, _) in &blocks {
            if used.contains(name) {
                continue;
            }
            if blocks
                .iter()
                .any(|(user, block)| used.contains(user) && user != name && mentions(block, name))
            {
                used.insert(name);
            }
        }
        if used.len() == before {
            break;
        }
    }
    let unused: Vec<&String> = declared
        .iter()
        .filter(|name| !used.contains(name))
        .collect();
    assert!(
        unused.is_empty(),
        "runtime names no example uses: {unused:?}"
    );
    // Every template is produced by the example.
    let snapshot = snapshot(&project);
    let mut templates = BTreeSet::new();
    let mut generated_names = Vec::new();
    for module in snapshot.modules() {
        for declaration in module.declarations() {
            if let Some(elaboration) = declaration.elaboration() {
                for theorem in elaboration.theorems() {
                    templates.insert(theorem["template"].as_str().expect("template").to_owned());
                    generated_names.push(format!(
                        "Reasoning.{}.{}",
                        module.name(),
                        theorem["name"].as_str().expect("name")
                    ));
                }
            }
        }
    }
    let closed: BTreeSet<String> = support::schema("semantic-snapshot-v2")["$defs"]
        ["generatedTheorem"]["properties"]["template"]["enum"]
        .as_array()
        .expect("templates")
        .iter()
        .map(|template| template.as_str().expect("template").to_owned())
        .collect();
    assert_eq!(
        templates, closed,
        "every template is produced by the example"
    );
    let Some(fixture) = reasoning_backed("RS-08") else {
        return;
    };
    for name in &generated_names {
        assert_eq!(
            attested(&fixture.attestation, name)["result"],
            "ok",
            "{name}"
        );
    }
    // The runtime alone is accepted by pinned Lean, and a mutated lemma
    // statement is refused.
    let start = clinic
        .find("namespace LexLeanReasoning\n")
        .expect("runtime");
    let end = clinic.find("end LexLeanReasoning\n").expect("runtime end")
        + "end LexLeanReasoning\n".len();
    let namespace = clinic
        .lines()
        .find_map(|line| line.strip_prefix("namespace "))
        .expect("the module namespace");
    let runtime_module = format!("{}end {namespace}\n", &clinic[..end]);
    assert!(start < end);
    // Every runtime lemma is accepted, and depends on no axiom.
    let audited = format!(
        "{}{}end {namespace}\n",
        &clinic[..end],
        declared
            .iter()
            .map(|name| format!("#print axioms LexLeanReasoning.{name}\n"))
            .collect::<String>()
    );
    let workspace = P::copy_example(EXAMPLE);
    let (accepted, printed) = lean_run(&workspace, "RuntimeAccepted", &audited);
    assert!(
        accepted,
        "the emitted runtime is accepted by pinned Lean: {printed}"
    );
    assert_eq!(
        printed.matches("does not depend on any axioms").count(),
        declared.len(),
        "every runtime name is axiom-free: {printed}"
    );
    let sound = "= some t -> r s t := by";
    assert!(
        runtime_module.contains(sound),
        "the guarded lemma states soundness"
    );
    let planted = runtime_module.replacen(sound, "= some t -> r t s := by", 1);
    assert!(
        !lean_run(&workspace, "RuntimeMutated", &planted).0,
        "a mutated runtime lemma statement is refused"
    );
}

/// §17.12: the runtime boundary, answer correctness, and traces as evidence.
#[allow(clippy::too_many_lines)]
fn rs_09() {
    negatives(
        "LLT4012",
        &[
            (
                "rule-bypass",
                "reaches the unguarded conclusion of a rule `Fever.conclusion`",
            ),
            (
                "unverified-answer",
                "reaches the unverified answer of a reasoner `Clinic.extract`",
            ),
        ],
    );
    negatives(
        "LLT4010",
        &[
            (
                "answer-correct-inexact",
                "`clinic_correct` does not state exactly the generated obligation",
            ),
            (
                "answer-correct-unsorted",
                "claims are not strictly sorted by kind",
            ),
        ],
    );
    let nat = json!({"kind": "nat"});
    let chart =
        json!({"kind": "named", "member": {"module": "Clinic", "name": "Chart"}, "arguments": []});
    let option_nat = json!({"kind": "option", "value": nat});
    let direct = |name: &str, function: Json, result: &Json| {
        json!({"kind": "definition", "name": name, "executable": true,
               "parameters": [{"name": "c", "type": chart}], "result": result,
               "body": {"kind": "call", "function": function,
                        "arguments": [{"kind": "var", "name": "c"}]}})
    };
    // Across modules, a conclusion or an unverified answer reached directly
    // is refused; an answer proved correct is not unverified.
    refused(
        &extended(
            "Main",
            vec![direct(
                "shortcut",
                json!({"module": "Clinic", "name": "Fever.conclusion"}),
                &chart,
            )],
        ),
        "LLT4012",
        "reaches the unguarded conclusion of a rule",
    );
    refused(
        &extended(
            "Main",
            vec![direct(
                "unchecked",
                json!({"module": "Clinic", "name": "Triage.extract"}),
                &option_nat,
            )],
        ),
        "LLT4012",
        "reaches the unverified answer of a reasoner",
    );
    extended(
        "Main",
        vec![direct(
            "capped",
            json!({"name": "Grade.extract"}),
            &option_nat,
        )],
    )
    .check_ok();
    // The proved answer's check is erased; every other reasoner's runs.
    let project = P::copy_example(EXAMPLE);
    let snapshot = snapshot(&project);
    let grade = elaboration(&snapshot, "Main", "Grade");
    assert_eq!(
        calls(&derived(grade, "Grade.accept")["body"]),
        ["Grade.extract"]
    );
    assert_eq!(
        grade.obligations().len(),
        3,
        "invariant, fuel bound, correctness"
    );
    let triage = elaboration(&snapshot, "Clinic", "Triage");
    assert_eq!(
        calls(&derived(triage, "Triage.accept")["body"])
            .into_iter()
            .collect::<BTreeSet<_>>(),
        strings(&["Triage.extract", "recommendationCheck"])
    );
    // A trace is evidence by replay: a step that does not fire is
    // invalid_step.
    let mut constructors = Vec::new();
    collect(
        &derived(triage, "Triage.replay")["body"],
        "constructor",
        "constructor",
        &mut constructors,
    );
    assert!(constructors
        .iter()
        .any(|constructor| constructor["name"] == "ReasoningFailure.invalid_step"));
    if let Some(fixture) = reasoning_backed("RS-09") {
        assert_attested_ok(
            &fixture.attestation,
            &[
                "Reasoning.Main.triage_replays",
                "Reasoning.Main.triage_forged",
                "Reasoning.Main.triage_inapplicable",
                "Reasoning.Main.grade_shock",
                "Reasoning.Main.grade_correct",
                "Reasoning.Main.Grade.accept_sound",
                "Reasoning.Clinic.Triage.derivation",
            ],
        );
    }
    refused_by_lean("RS-09", &["forged-trace", "false-answer-correct"]);
}

/// The smallest `max_ir_nodes` under which `project` links: what linking
/// charges it, elaboration included.
fn charged_nodes(project: &P) -> u64 {
    let config = project.read("lexlean.toml");
    let links = |limit: u64| {
        project.write(
            "lexlean.toml",
            &config.replace("max_ir_nodes = 2000000", &format!("max_ir_nodes = {limit}")),
        );
        project.relock();
        project.check_err_or_ok()
    };
    let (mut low, mut high) = (1_u64, 2_000_000_u64);
    assert!(links(high), "the project links under the default limit");
    while low < high {
        let middle = low + (high - low) / 2;
        if links(middle) {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    project.write("lexlean.toml", &config);
    project.relock();
    low
}

/// The number of JSON values in `value`: an upper bound on the nodes its
/// source states.
fn json_size(value: &Json) -> u64 {
    match value {
        Json::Object(map) => 1 + map.values().map(json_size).sum::<u64>(),
        Json::Array(items) => 1 + items.iter().map(json_size).sum::<u64>(),
        _ => 1,
    }
}

/// The nesting depth of a JSON value.
fn json_depth(value: &Json) -> usize {
    match value {
        Json::Object(map) => 1 + map.values().map(json_depth).max().unwrap_or(0),
        Json::Array(items) => 1 + items.iter().map(json_depth).max().unwrap_or(0),
        _ => 0,
    }
}

/// The clinic of the forged-trace fixture with `copies` of each of its two
/// rules (a binding-free one and a binding one), all named by its reasoner,
/// which is a search when `search` is set.
fn many_rules(copies: usize, search: bool) -> P {
    let project = P::negative("reasoning-forged-trace");
    let mut data = module_data(&project, "Main");
    let declarations = declarations_mut(&mut data);
    let at = declarations
        .iter()
        .position(|declaration| declaration["name"] == "Clinic")
        .expect("the reasoner");
    let mut added = Vec::new();
    let mut uses = Vec::new();
    for (rule, theorems) in [
        ("Fever", ["fever_sound", "fever_progress"]),
        ("Pick", ["pick_sound", "pick_progress"]),
    ] {
        let original = |name: &str| {
            declarations
                .iter()
                .find(|declaration| declaration["name"] == name)
                .unwrap_or_else(|| panic!("`{name}`"))
                .clone()
        };
        for copy in 0..copies {
            let renamed = |name: &str| format!("{name}{copy}");
            let mut sound = original(theorems[0]);
            sound["name"] = json!(renamed(theorems[0]));
            let mut progress = original(theorems[1]);
            progress["name"] = json!(renamed(theorems[1]));
            let mut copied = original(rule);
            copied["name"] = json!(renamed(rule));
            copied["soundness"] = json!({"name": renamed(theorems[0])});
            copied["progress"] = json!({"name": renamed(theorems[1])});
            added.extend([sound, progress, copied]);
            uses.push(json!({"member": {"name": renamed(rule)}}));
        }
    }
    for (offset, declaration) in added.into_iter().enumerate() {
        declarations.insert(at + offset, declaration);
    }
    let clinic = declaration_mut(&mut data, "Clinic");
    clinic["rules"].as_array_mut().expect("rules").extend(uses);
    if search {
        clinic["strategy"] = json!({"kind": "search", "order": "depth_first",
            "fuel": {"kind": "nat", "value": "40"}, "frontier": {"kind": "nat", "value": "5"}});
        clinic["claims"] =
            json!([{"kind": "initial_invariant", "theorem": {"name": "clinic_initial"}}]);
    }
    write_module_data(&project, "Main", &data);
    project
}

/// §17.12, §21.4: identity, node charging, snapshots, and the document.
#[allow(clippy::too_many_lines)]
fn rs_10() {
    let id = |project: &P| support::checked_project(project).semantic_id;
    let base = P::copy_example(EXAMPLE);
    let mut ids = vec![id(&base)];
    let edits: [(&str, &str, Edit); 5] = [
        ("Clinic", "Review", |declaration| {
            declaration["strategy"]["fuel"] = json!({"kind": "nat", "value": "7"});
        }),
        ("Clinic", "Review", |declaration| {
            declaration["rules"]
                .as_array_mut()
                .expect("rules")
                .swap(0, 1);
        }),
        ("Planner", "Plan", |declaration| {
            declaration["strategy"]["order"] = json!("depth_first");
        }),
        ("Screening", "Dose", |declaration| {
            declaration["strategy"]["budget"] = json!({"kind": "nat", "value": "4"});
        }),
        ("Main", "Grade", |declaration| {
            declaration["claims"].as_array_mut().expect("claims").pop();
        }),
    ];
    for (module, name, edit) in edits {
        let project = mutated(module, name, edit);
        project.check_ok();
        ids.push(id(&project));
    }
    let distinct: BTreeSet<String> = ids.iter().map(Sha256Digest::to_hex).collect();
    assert_eq!(
        distinct.len(),
        ids.len(),
        "every reasoning change changes the semantic ID"
    );
    // A reasoner whose elaboration would exceed the limit is refused before
    // it is elaborated, by a charge its elaboration never exceeds.
    negatives(
        "LLS8002",
        &[(
            "elaboration-over-limit",
            "once reasoner `Clinic` elaborates to at most",
        )],
    );
    let over = P::negative("reasoning-elaboration-over-limit");
    assert!(
        over.check_fails_with("LLS8002")
            .to_string()
            .contains("before elaborating"),
        "the refusal is charged before elaboration"
    );
    // Many rules: linking succeeds under the default limits, and the
    // elaboration nests with the logarithm of the rules (every pass over a
    // term recurses on its depth, and a Windows main thread has one MiB).
    for search in [false, true] {
        let project = many_rules(300, search);
        project.check_ok();
        let snapshot = snapshot(&project);
        let clinic = elaboration(&snapshot, "Main", "Clinic");
        let bodies: &[&str] = if search {
            &["Clinic.successors", "Clinic.attempts"]
        } else {
            &["Clinic.select", "Clinic.attempts"]
        };
        for name in bodies {
            let depth = json_depth(&derived(clinic, name)["body"]);
            assert!(
                depth <= 80,
                "{name} of 602 rules nests {depth} deep, not logarithmically"
            );
        }
        assert_eq!(
            derived(clinic, "Clinic.Step")["constructors"]
                .as_array()
                .expect("constructors")
                .len(),
            602,
            "the elaboration holds every rule"
        );
    }
    // An elaboration is charged, not only its source.
    let with = P::negative("reasoning-forged-trace");
    let mut data = module_data(&with, "Main");
    let source = data["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .find(|declaration| declaration["name"] == "Clinic")
        .expect("the reasoner")
        .clone();
    let charged_with = charged_nodes(&with);
    let without = P::negative("reasoning-forged-trace");
    declarations_mut(&mut data).retain(|declaration| {
        declaration["name"] != "Clinic" && declaration["name"] != "forged_answer"
    });
    write_module_data(&without, "Main", &data);
    let charged_without = charged_nodes(&without);
    let reasoner_nodes = charged_with - charged_without;
    assert!(
        reasoner_nodes > 4 * json_size(&source),
        "the reasoner charges its elaboration: {reasoner_nodes} nodes for a source of {} values",
        json_size(&source)
    );
    // Snapshots carry every elaboration with its generated theorems.
    let snapshot = snapshot(&base);
    let value: Json = serde_json::from_slice(&snapshot.canonical_bytes()).expect("JSON");
    support::assert_schema("semantic-snapshot-v2", "the reasoning snapshot", &value);
    for module in snapshot.modules() {
        for declaration in module.declarations() {
            // A logic generates its preservation theorem only when it has an
            // invariant; every rule, verifier, and reasoner generates some.
            if ["inference_rule", "verifier", "reasoner"].contains(&declaration.kind()) {
                let elaboration = declaration.elaboration().expect("an elaboration");
                assert!(
                    !elaboration.theorems().is_empty(),
                    "{} generates theorems",
                    declaration.logical_id()
                );
            }
        }
    }
    // The document: a closed catalog, no trace value, no verification claim.
    let build = support::rendered(&base);
    for module in MODULES {
        let tex = support::tex_text(&build, module);
        assert!(
            !tex.to_lowercase().contains("verified"),
            "{module}: the document never says verified"
        );
    }
    let clinic = support::tex_text(&build, "Clinic");
    for line in [
        "Rules in priority order: \\texttt{Fever, Tachycardia, Tachypnea, Leukocytosis, Sirs, Sepsis, Shock, Escalate}",
        "Strategy: \\texttt{forward, at most 11 iterations}",
        "Generates theorem (",
        "Obligation (reasoner Triage fuel bound)",
    ] {
        assert!(clinic.contains(line), "the catalog states {line:?}");
    }
    let catalog = clinic
        .split("\\subsection*{\\texttt{Triage}}")
        .nth(1)
        .expect("the Triage catalog");
    for rule in RULES {
        assert!(
            !catalog.contains(&format!("Triage.Step.{rule} ::")),
            "the catalog shows no trace value"
        );
    }
    assert!(support::tex_text(&build, "Screening").contains("generate and verify the candidates"));
}

/// §17.13, §17.14, §22.10: production dispositions, realization, the
/// eligibility report's reasoning rows, and extraction.
#[allow(clippy::too_many_lines)]
fn rs_11() {
    let registry = lexlean::production::registry().expect("registry");
    for (key, disposition) in [
        ("declaration.logic", "erased"),
        ("declaration.inference_rule", "runtime"),
        ("declaration.verifier", "erased"),
        ("declaration.reasoner", "runtime"),
        ("type.reasoning_failure", "runtime"),
    ] {
        assert_eq!(
            registry.constructs[key].disposition.as_str(),
            disposition,
            "{key}"
        );
    }
    let table: BTreeSet<&str> = lexlean::calculus::realization::TABLE
        .iter()
        .map(|(key, _)| *key)
        .collect();
    for key in [
        "declaration.inference_rule",
        "declaration.reasoner",
        "type.reasoning_failure",
    ] {
        assert!(table.contains(key), "the realization table covers {key}");
    }
    let project = P::copy_example(EXAMPLE);
    let report = eligibility(&project, "Main");
    support::assert_schema("production-eligibility", "the Main report", &report);
    let rows = |name: &str| -> Vec<Json> {
        root(&report, name)["reasoning"]
            .as_array()
            .unwrap_or_else(|| panic!("{name} records its reasoning"))
            .clone()
    };
    let triage = rows("Reasoning.Main.triageLevel");
    assert_eq!(triage.len(), 1);
    assert_eq!(triage[0]["reasoner"], "Reasoning.Clinic.Triage");
    assert_eq!(triage[0]["strategy"], "forward");
    assert_eq!(
        triage[0]["ledger"],
        json!(lexlean::ir::semantic::reasoning::LEDGER)
    );
    assert_eq!(
        triage[0]["rules"],
        json!(RULES
            .iter()
            .map(|rule| format!("Reasoning.Clinic.{rule}"))
            .collect::<Vec<_>>())
    );
    let bounds: Vec<String> = serde_json::from_value(triage[0]["bounds"].clone()).expect("bounds");
    assert!(bounds.contains(&"Reasoning.Clinic.Triage.iterations_bounded".to_owned()));
    assert!(bounds.contains(&"Reasoning.Clinic.Triage.saturates".to_owned()));
    let dose = rows("Reasoning.Main.doseLevel");
    assert_eq!(dose[0]["strategy"], "generate_and_verify");
    assert!(dose[0].get("budget").is_some() && dose[0].get("fuel").is_none());
    assert_eq!(
        dose[0]["bounds"],
        json!(["Reasoning.Screening.Dose.verifications_bounded"])
    );
    let plan = rows("Reasoning.Main.planLeft");
    assert_eq!(plan[0]["strategy"], "breadth_first");
    assert_eq!(plan[0]["deduplicate"], true);
    assert!(plan[0].get("frontier").is_some());
    assert_eq!(
        rows("Reasoning.Main.spendLeft")[0]["reasoner"],
        "Reasoning.Budget.Spend"
    );
    let closure = |name: &str| -> BTreeSet<String> {
        root(&report, name)["runtime_closure"]
            .as_array()
            .expect("closure")
            .iter()
            .map(|member| {
                member["declaration"]
                    .as_str()
                    .expect("declaration")
                    .to_owned()
            })
            .collect()
    };
    let triage_closure = closure("Reasoning.Main.triageLevel");
    for member in [
        "Reasoning.Clinic.Triage.verdict",
        "Reasoning.Clinic.Triage.select",
        "Reasoning.Clinic.Fever.apply",
        "Reasoning.Clinic.recommendationCheck",
    ] {
        assert!(
            triage_closure.contains(member),
            "triageLevel reaches {member}"
        );
    }
    for erased in [
        "Reasoning.Clinic.Findings",
        "Reasoning.Clinic.Recommendation",
    ] {
        assert!(!triage_closure.contains(erased), "{erased} is erased");
    }
    for name in ["triageLevel", "gradeLevel"] {
        let targets = &root(&report, &format!("Reasoning.Main.{name}"))["targets"];
        assert!(
            targets
                .as_array()
                .expect("targets")
                .iter()
                .all(|target| target["status"] == "eligible"),
            "{name}: {targets}"
        );
    }
    if let Some(fixture) = reasoning_backed("RS-11") {
        let input: Json = serde_json::from_slice(
            &std::fs::read(
                fixture
                    .outcome
                    .root
                    .join("production/compiler-input.json")
                    .as_std_path(),
            )
            .expect("compiler input"),
        )
        .expect("compiler input JSON");
        for name in [
            "triageLevel",
            "gradeLevel",
            "doseLevel",
            "planLeft",
            "spendLeft",
        ] {
            let root_name = format!("Reasoning.Main.{name}");
            let extracted: BTreeSet<String> = input["closures"]
                .as_array()
                .expect("closures")
                .iter()
                .find(|closure| closure["root"] == root_name.as_str())
                .unwrap_or_else(|| panic!("Lean extracted {root_name}"))["declarations"]
                .as_array()
                .expect("declarations")
                .iter()
                .map(|name| name.as_str().expect("name").to_owned())
                .collect();
            assert_eq!(extracted, closure(&root_name), "{root_name}");
        }
    }
}

fn count(value: &Value) -> u64 {
    match value {
        Value::Nat { value } => value.parse().expect("nat"),
        other => panic!("a count, not {other:?}"),
    }
}

/// §17.14, §17.16: the clinical oracle, its transcriptions, their packages,
/// and their accounting.
#[allow(clippy::too_many_lines)]
fn rs_12() {
    use crate::calculus::reasoning;
    let repository = support::repo_root();
    let read = |path: &str| {
        std::fs::read_to_string(repository.join(path).as_std_path())
            .unwrap_or_else(|error| panic!("{path}: {error}"))
    };
    let declarations = |text: &str| -> Json {
        let (start, end) = data_bounds(text);
        serde_json::from_str::<Json>(&text[start..end]).expect("module data")["declarations"]
            .clone()
    };
    assert_eq!(
        declarations(&read("compiler/src/ReasoningOracle.lex.tex")),
        declarations(&read(reasoning::SOURCE)),
        "the oracle states exactly the clinical module"
    );
    let cases: Vec<crate::calculus::Case> = crate::calculus::cases()
        .into_iter()
        .filter(|case| case.fixture.name.starts_with("reasoning-"))
        .collect();
    let named: Vec<&str> = cases
        .iter()
        .map(|case| case.fixture.name.as_str())
        .collect();
    assert_eq!(
        named,
        [
            "reasoning-review-exhausted",
            "reasoning-review-rejected",
            "reasoning-triage-shock",
            "reasoning-triage-traced",
            "reasoning-triage-well",
        ]
    );
    let failure = |left: bool, right: bool| Value::Error {
        value: Box::new(Value::Pair {
            left: Box::new(Value::Bool { value: left }),
            right: Box::new(Value::Bool { value: right }),
        }),
    };
    let level = |n: u64| Value::Ok {
        value: Box::new(Value::Nat {
            value: n.to_string(),
        }),
    };
    let fixtures_text = read("compiler/src/TargetFixtures.lex.tex");
    for case in &cases {
        assert!(
            crate::calculus::kernel_reducible(&case.fixture),
            "{}",
            case.fixture.name
        );
        assert!(
            case.oracle.is_some(),
            "{} states the oracle",
            case.fixture.name
        );
        let id = crate::calculus::identifier(&case.fixture.name);
        assert!(
            fixtures_text.contains(&format!("\"name\":\"{id}Agrees\"")),
            "{id}Agrees is stated"
        );
        let Outcome::Value { value, .. } = &case.fixture.expected else {
            panic!("{} returns a value", case.fixture.name)
        };
        let expected = match case.fixture.name.as_str() {
            "reasoning-triage-shock" => Some(level(3)),
            "reasoning-triage-well" => Some(level(0)),
            "reasoning-review-exhausted" => Some(failure(false, false)),
            "reasoning-review-rejected" => Some(failure(true, false)),
            _ => None,
        };
        if let Some(expected) = expected {
            assert_eq!(value, &expected, "{}", case.fixture.name);
        }
    }
    // The traced transcription accounts its search; every transcription on
    // the same patient charges at least that account in steps.
    let find = |name: &str| {
        cases
            .iter()
            .find(|case| case.fixture.name == name)
            .unwrap_or_else(|| panic!("{name}"))
    };
    let traced = find("reasoning-triage-traced");
    let Outcome::Value {
        value: Value::Pair { left, right },
        ..
    } = &traced.fixture.expected
    else {
        panic!("the traced transcription returns its answer and account")
    };
    let Value::Ok { value: explained } = left.as_ref() else {
        panic!("the shocked patient is answered")
    };
    let Value::Pair {
        left: answered,
        right: trace,
    } = explained.as_ref()
    else {
        panic!("an answer with its trace")
    };
    assert_eq!(count(answered), 3);
    let Value::List { items: trace } = trace.as_ref() else {
        panic!("a trace")
    };
    let Value::Pair {
        left: attempts,
        right: firings,
    } = right.as_ref()
    else {
        panic!("an account")
    };
    let (attempts, firings) = (count(attempts), count(firings));
    assert_eq!(
        firings,
        trace.len() as u64,
        "every firing is a step of the trace"
    );
    assert!(
        attempts > firings,
        "every firing follows a guard evaluation"
    );
    let shock = find("reasoning-triage-shock");
    for case in [traced, shock] {
        let Outcome::Value { steps, .. } = &case.fixture.expected else {
            panic!("a value")
        };
        assert!(
            *steps >= attempts + firings,
            "{} charges its search: {steps} steps for {attempts} attempts and {firings} firings",
            case.fixture.name
        );
    }
    // The check is falsifiable: a transcription answering the shocked
    // patient without searching computes the same value and is charged
    // fewer steps than the account.
    let mut constant = shock.fixture.program.clone();
    constant.functions[0].body = lexlean::calculus::Expr::Value {
        ty: constant.functions[0].result.clone(),
        value: level(3),
    };
    let Outcome::Value {
        value,
        steps: constant_steps,
    } = interp::run(&constant, 4000, 0, &shock.fixture.arguments)
    else {
        panic!("the constant transcription returns")
    };
    assert_eq!(value, level(3));
    assert!(
        constant_steps < attempts + firings,
        "an uncharged search is detected"
    );
    // The packages: every verdict transcription in both profiles, the
    // traced one, which allocates its trace, in rust-std only.
    let packaged: BTreeSet<(String, String)> = crate::rust_packages::packages()
        .into_iter()
        .filter(|committed| committed.fixture.starts_with("reasoning-"))
        .map(|committed| (committed.fixture, committed.profile.target().to_owned()))
        .collect();
    for name in [
        "reasoning-review-exhausted",
        "reasoning-review-rejected",
        "reasoning-triage-shock",
        "reasoning-triage-well",
    ] {
        for target in ["rust-core", "rust-std"] {
            assert!(
                packaged.contains(&(name.to_owned(), target.to_owned())),
                "{name} in {target}"
            );
        }
    }
    let traced_in = |target: &str| {
        packaged.contains(&("reasoning-triage-traced".to_owned(), target.to_owned()))
    };
    assert!(traced_in("rust-std") && !traced_in("rust-core"));
    assert!(
        crate::calculus::check(repository.as_std_path(), false).is_ok(),
        "the committed transcriptions, oracle, and packages equal their generator"
    );
    if support::lean_backed("RS-12") {
        let verified = support::verified_compiler();
        for unit in ["ReasoningOracle", "TargetFixtures"] {
            assert!(
                verified.outcome.units.contains_key(unit),
                "{unit} is verified"
            );
        }
        for case in &cases {
            let id = crate::calculus::identifier(&case.fixture.name);
            assert_attested_ok(
                &verified.attestation,
                &[&format!("Compiler.TargetFixtures.{id}Agrees")],
            );
        }
    }
}

/// §17.15: GNAF over forward-chaining plans.
fn rs_13() {
    use crate::gnaf::reasoning::DOMAIN;
    use lexlean::gnaf::{ActionKind, Answer, Carrier, Rejection, Selector};
    let fixtures = crate::gnaf::fixtures();
    let find = |name: &str| {
        fixtures
            .iter()
            .find(|fixture| fixture.name == name)
            .unwrap_or_else(|| panic!("request {name}"))
    };
    let argmin = find("reasoning-argmin");
    let Carrier::Grammar {
        plans, thresholds, ..
    } = &argmin.request.carrier
    else {
        panic!("a grammar of plans")
    };
    assert_eq!(plans.len(), 3);
    assert!(thresholds.is_empty());
    // Each plan's rule search is charged in steps: the forward plans pay for
    // the rules they try; the goal-directed plan searches nothing.
    let charged = |plan: u64| -> u64 {
        let system = lexlean::gnaf::realize(&argmin.request.carrier, Selector::Fixed { plan })
            .expect("a system");
        DOMAIN
            .iter()
            .map(|findings| {
                match interp::run(
                    &system,
                    argmin.request.machine.fuel,
                    0,
                    &[Value::Nat {
                        value: findings.to_string(),
                    }],
                ) {
                    Outcome::Value { steps, .. } => steps,
                    other => panic!("plan {plan} returns: {other:?}"),
                }
            })
            .sum()
    };
    let (forward, sweep, goal) = (charged(0), charged(1), charged(2));
    assert!(forward > sweep && sweep > goal, "{forward} {sweep} {goal}");
    match &argmin.expected {
        Answer::Argmin { members, steps } => {
            assert_eq!(members, &[2]);
            assert_eq!(*steps, goal);
        }
        other => panic!("an argmin, not {other:?}"),
    }
    assert!(matches!(
        find("reasoning-frontier").expected,
        Answer::Frontier { .. }
    ));
    assert_eq!(
        find("reject-reasoning-free-search").expected,
        Answer::Rejected {
            rejection: Rejection::HiddenCost {
                action: ActionKind::Execution
            }
        }
    );
    assert_eq!(
        find("reject-reasoning-discovered-universe").expected,
        Answer::Rejected {
            rejection: Rejection::DiscoveredUniverse
        }
    );
    if support::lean_backed("RS-13") {
        let verified = support::verified_compiler();
        assert_attested_ok(
            &verified.attestation,
            &[
                "Compiler.GnafFixtures.reasoningArgminAnswer",
                "Compiler.GnafFixtures.reasoningArgminStatuses",
                "Compiler.GnafFixtures.reasoningFrontierAnswer",
            ],
        );
    }
}

/// §17.12, §28.6: the committed example verifies, and a planted rule
/// threshold mutation is refused by verification.
fn rs_14() {
    let project = P::copy_example(EXAMPLE);
    project.check_ok();
    project.fmt_check_ok();
    let Some(fixture) = reasoning_backed("RS-14") else {
        return;
    };
    assert_eq!(fixture.attestation["status"], "verified");
    let rows = fixture.attestation["declarations"]
        .as_array()
        .expect("declarations");
    assert!(rows.iter().all(|row| row["result"] == "ok"));
    assert_attested_ok(
        &fixture.attestation,
        &[
            "Reasoning.Main.triage_shock",
            "Reasoning.Main.plan_found",
            "Reasoning.Main.dose_found",
            "Reasoning.Main.grade_shock",
            "Reasoning.Main.gateway_level",
        ],
    );
    // Plant: tachycardia from 80 beats instead of 90. Linking cannot see
    // that the rule now concludes a finding its relation does not justify.
    let planted = mutated("Clinic", "tachycardic", |declaration| {
        assert_eq!(
            declaration["body"]["left"],
            json!({"kind": "nat", "value": "90"})
        );
        declaration["body"]["left"] = json!({"kind": "nat", "value": "80"});
    });
    planted.check_ok();
    let _guard = support::env_lock();
    planted.verify_fails_with("LLV7002");
}
