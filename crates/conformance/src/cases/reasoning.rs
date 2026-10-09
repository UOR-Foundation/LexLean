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

/// The guards a term evaluates, in the order it evaluates them: a `match`
/// its scrutinee then its branches, an `if` its condition and branches, a
/// `let` its value before its body. Document order of canonical JSON is
/// alphabetical by key, so it says nothing about evaluation.
fn guards_evaluated(value: &Json, out: &mut Vec<String>) {
    let each = |children: &[&str], out: &mut Vec<String>| {
        for child in children {
            guards_evaluated(&value[*child], out);
        }
    };
    match value["kind"].as_str() {
        Some("match") => {
            guards_evaluated(&value["scrutinee"], out);
            for branch in value["branches"].as_array().into_iter().flatten() {
                guards_evaluated(&branch["body"], out);
            }
        }
        Some("if") => each(&["condition", "then_value", "else_value"], out),
        Some("let") => each(&["value", "body"], out),
        Some("call") => {
            let name = value["function"]["name"].as_str().unwrap_or_default();
            if name.ends_with(".guard") {
                out.push(name.to_owned());
            }
            for argument in value["arguments"].as_array().into_iter().flatten() {
                guards_evaluated(argument, out);
            }
        }
        Some("primitive") => {
            for argument in value["arguments"].as_array().into_iter().flatten() {
                guards_evaluated(argument, out);
            }
        }
        _ => match value {
            Json::Object(map) => {
                for child in map.values() {
                    guards_evaluated(child, out);
                }
            }
            Json::Array(items) => {
                for item in items {
                    guards_evaluated(item, out);
                }
            }
            _ => {}
        },
    }
}

fn evaluated_guards(value: &Json) -> Vec<String> {
    let mut out = Vec::new();
    guards_evaluated(value, &mut out);
    out
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
    let claims = strings(&[
        "answer_correct",
        "initial_invariant",
        "observation_invariant",
        "terminates",
    ]);
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
            (
                "reserved-declaration-name",
                "declaration name `LexLeanReasoning.Star` is reserved",
            ),
            (
                "reserved-declaration-namespace",
                "declaration name `LexLeanReasoning` is reserved",
            ),
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
            (
                "reasoner-rule-type-arguments",
                "reasoner `Spend` reasons in logic `Countdown<T>`, but rule `Tick<Bool>` is over logic `Countdown<Bool>`",
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
                "verifier-subject-mismatch",
                "but verifier `Probe` checks a Bool subject and a Nat candidate",
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
        reasoners, 12,
        "Triage, Review, Spend, Refund, Plan, PlanFit, PlanFitR, PlanFitU, Dose, Screen, Grade, Cap"
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
        evaluated_guards(&derived(triage, "Triage.select")["body"]),
        guards,
        "select tries the rules in declared priority order"
    );
    assert_eq!(
        evaluated_guards(&derived(triage, "Triage.attempts")["body"]),
        guards,
        "attempts counts exactly the guards select evaluates, in its order"
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
        evaluated_guards(
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
            (
                "claims-duplicate",
                "claims are not strictly sorted by kind, each at most once",
            ),
            (
                "observation-unsorted",
                "claims are not strictly sorted by kind, each at most once",
            ),
            (
                "observation-invariant-signature",
                "observation invariant `Sound` must take (Nat, Memo) to Prop, but takes (Memo) to Prop",
            ),
            (
                "observation-initial-inexact",
                "observation invariant initial: `related_initial` does not state exactly the generated obligation",
            ),
            (
                "observation-preserved-inexact",
                "observation invariant preservation: `related_preserved` does not state exactly the generated obligation",
            ),
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
            // The same boundary by every other way a name is mentioned: a
            // function value folded, bound, applied, iterated, or called
            // from a lambda or a branch; a rule's own guard; a reasoner's
            // observation or generator; and another reasoner's answer.
            (
                "rule-bypass-ref",
                "executable `shortcut` reaches the unguarded conclusion of a rule `Pick.conclusion`",
            ),
            (
                "rule-bypass-alias",
                "executable `shortcut` reaches the unguarded conclusion of a rule `Fever.conclusion`",
            ),
            (
                "rule-bypass-lambda",
                "executable `shortcut` reaches the unguarded conclusion of a rule `Fever.conclusion`",
            ),
            (
                "rule-bypass-iterate",
                "executable `shortcut` reaches the unguarded conclusion of a rule `Fever.conclusion`",
            ),
            (
                "rule-bypass-branch",
                "executable `shortcut` reaches the unguarded conclusion of a rule `Fever.conclusion`",
            ),
            (
                "rule-bypass-guard",
                "executable `Pick` reaches the unguarded conclusion of a rule `Fever.conclusion`",
            ),
            (
                "rule-bypass-observe",
                "executable `Clinic` reaches the unguarded conclusion of a rule `Fever.conclusion`",
            ),
            (
                "rule-bypass-generator",
                "executable `Guess` reaches the unguarded conclusion of a rule `Fever.conclusion`",
            ),
            (
                "extract-bypass-ref",
                "executable `unchecked` reaches the unverified answer of a reasoner `Clinic.extract`",
            ),
            (
                "extract-bypass-alias",
                "executable `unchecked` reaches the unverified answer of a reasoner `Clinic.extract`",
            ),
            (
                "extract-bypass-branch",
                "executable `unchecked` reaches the unverified answer of a reasoner `Clinic.extract`",
            ),
            (
                "extract-bypass-reasoner",
                "executable `Echo` reaches the unverified answer of a reasoner `Clinic.extract`",
            ),
            // An answer proved correct only under an invariant is
            // unconstrained outside it, so its `extract` stays guarded.
            (
                "extract-bypass-invariant",
                "executable `forged` reaches the unverified answer of a reasoner `Clinic.extract`",
            ),
            // So do the members that read an answer from a state or a
            // search the caller supplies once the check is erased: `accept`
            // and `conclude` of a forward reasoner, `accept` and
            // `searchStep` of a search, by a call, a function value, or an
            // alias, under either conditional form.
            (
                "accept-bypass-invariant",
                "executable `forged` reaches the unverified answer of a reasoner `Clinic.accept`",
            ),
            (
                "accept-bypass-ref",
                "executable `forged` reaches the unverified answer of a reasoner `Clinic.accept`",
            ),
            (
                "accept-bypass-alias",
                "executable `forged` reaches the unverified answer of a reasoner `Clinic.accept`",
            ),
            (
                "accept-bypass-observation",
                "executable `forged` reaches the unverified answer of a reasoner `Clinic.accept`",
            ),
            (
                "conclude-bypass-invariant",
                "executable `forged` reaches the unverified answer of a reasoner `Clinic.conclude`",
            ),
            (
                "conclude-bypass-observation",
                "executable `forged` reaches the unverified answer of a reasoner `Clinic.conclude`",
            ),
            (
                "search-accept-bypass-invariant",
                "executable `forged` reaches the unverified answer of a reasoner `Clinic.accept`",
            ),
            (
                "search-step-bypass-invariant",
                "executable `forged` reaches the unverified answer of a reasoner `Clinic.searchStep`",
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
            (
                "answer-invariant-without-initial",
                "claims an answer correct under the invariant of logic `Triage`, which holds of every state its run reaches only given the initial invariant, and it claims none",
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
    // Only an answer correct on every state is read without its verifier:
    // `Cap`'s is, and `Grade`'s, correct only for a chart that relates to its
    // patient, is not, so a chart that does not (a level of 99) would be
    // answered unchecked.
    extended(
        "Main",
        vec![direct(
            "capped",
            json!({"name": "Cap.extract"}),
            &option_nat,
        )],
    )
    .check_ok();
    refused(
        &extended(
            "Main",
            vec![direct(
                "forgedGrade",
                json!({"name": "Grade.extract"}),
                &option_nat,
            )],
        ),
        "LLT4012",
        "executable `forgedGrade` reaches the unverified answer of a reasoner `Grade.extract`",
    );
    // Every member that reads an answer from a state or a search its caller
    // supplies is behind the same boundary where the check was erased by a
    // conditional proof (`Grade`, `PlanFit`, `PlanFitR`), and in front of it
    // where the answer is correct on every state (`Cap`, `PlanFitU`) or the
    // check runs (`Plan`).
    let named = |module: &str, name: &str| json!({"kind": "named", "member": {"module": module, "name": name}, "arguments": []});
    let vitals = named("Clinic", "Vitals");
    let jugs = json!({"kind": "product", "left": nat, "right": nat});
    let failing = json!({"kind": "result", "ok": nat, "error": {"kind": "reasoning_failure"}});
    let reading = |reasoner: &str, member: &str, module: Option<&str>| {
        let (parameters, result) = match (reasoner, member) {
            ("Grade" | "Cap", "accept") => {
                (vec![("v", &vitals), ("c", &chart)], option_nat.clone())
            }
            ("Grade" | "Cap", _) => (vec![("v", &vitals), ("c", &chart)], failing.clone()),
            ("Plan", "accept") => (
                vec![("v", &nat), ("c", &jugs)],
                json!({"kind": "option", "value": jugs}),
            ),
            (_, "accept") => (vec![("v", &nat), ("c", &jugs)], option_nat.clone()),
            _ => {
                let search = named("Planner", &format!("{reasoner}.Search"));
                let result = json!({"kind": "option", "value": search});
                return json!({"kind": "definition", "name": "forgedRead", "executable": true,
                           "parameters": [{"name": "v", "type": nat}, {"name": "c", "type": search}],
                           "result": result,
                           "body": {"kind": "call",
                                    "function": {"module": module, "name": format!("{reasoner}.{member}")},
                                    "arguments": [{"kind": "var", "name": "v"}, {"kind": "var", "name": "c"}]}});
            }
        };
        let function = match module {
            Some(module) => json!({"module": module, "name": format!("{reasoner}.{member}")}),
            None => json!({"name": format!("{reasoner}.{member}")}),
        };
        json!({"kind": "definition", "name": "forgedRead", "executable": true,
                   "parameters": parameters.iter().map(|(n, t)| json!({"name": n, "type": t})).collect::<Vec<_>>(),
                   "result": result,
                   "body": {"kind": "call", "function": function,
                            "arguments": parameters.iter().map(|(n, _)| json!({"kind": "var", "name": n})).collect::<Vec<_>>()}})
    };
    for (reasoner, member, module) in [
        ("Grade", "accept", None),
        ("Grade", "conclude", None),
        ("PlanFit", "accept", Some("Planner")),
        ("PlanFit", "searchStep", Some("Planner")),
        ("PlanFitR", "accept", Some("Planner")),
        ("PlanFitR", "searchStep", Some("Planner")),
    ] {
        refused(
            &extended("Main", vec![reading(reasoner, member, module)]),
            "LLT4012",
            &format!(
                "executable `forgedRead` reaches the unverified answer of a reasoner `{}{reasoner}.{member}`",
                module.map(|module| format!("{module}::")).unwrap_or_default()
            ),
        );
    }
    for (reasoner, member, module) in [
        ("Cap", "accept", None),
        ("Cap", "conclude", None),
        ("PlanFitU", "accept", Some("Planner")),
        ("PlanFitU", "searchStep", Some("Planner")),
        ("Plan", "accept", Some("Planner")),
        ("Plan", "searchStep", Some("Planner")),
    ] {
        extended("Main", vec![reading(reasoner, member, module)]).check_ok();
    }
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
        5,
        "initial invariant, fuel bound, observation invariant (initial and preserved), correctness"
    );
    // Grade's answer reads the chart's level, which is on the scale, and
    // at 3 only for a hypotensive patient, because every chart a run reaches
    // relates to the patient: its correctness is stated under that
    // observation invariant, discharged by the run's relation theorem.
    let correctness = grade
        .obligations()
        .iter()
        .find(|obligation| obligation["role"] == "reasoner `Grade` answer correctness")
        .expect("Grade's answer correctness");
    assert_eq!(correctness["statement"]["kind"], "implies");
    assert_eq!(
        correctness["statement"]["premise"]["function"],
        json!({"name": "Related"})
    );
    assert_eq!(
        correctness["statement"]["premise"]["arguments"][0],
        json!({"kind": "var", "name": "v"}),
        "the premise relates the observation, not the state alone"
    );
    let templates = |declaration: &lexlean::SnapshotElaboration| -> BTreeSet<String> {
        declaration
            .theorems()
            .iter()
            .filter_map(|theorem| theorem["template"].as_str().map(str::to_owned))
            .collect()
    };
    assert!(templates(grade).contains("run_invariant"));
    // The stronger form, correct on every state whatever the rules derived,
    // states no premise, needs no run invariant, and erases the check too.
    let cap = elaboration(&snapshot, "Main", "Cap");
    let unconditional = cap
        .obligations()
        .iter()
        .find(|obligation| obligation["role"] == "reasoner `Cap` answer correctness")
        .expect("Cap's answer correctness");
    assert_eq!(unconditional["statement"]["kind"], "implies");
    assert_eq!(unconditional["statement"]["premise"]["kind"], "eq");
    assert!(!templates(cap).contains("run_invariant"));
    assert_eq!(calls(&derived(cap, "Cap.accept")["body"]), ["Cap.extract"]);
    assert!(!templates(elaboration(&snapshot, "Clinic", "Triage")).contains("run_invariant"));
    // A search claims its answer correct in all three forms, and its check
    // is erased too: under the logic's invariant, under an observation
    // invariant, and on every state.
    let planner = |name: &str| elaboration(&snapshot, "Planner", name);
    for (name, premise) in [
        ("PlanFit", Some("Fits")),
        ("PlanFitR", Some("Held")),
        ("PlanFitU", None),
    ] {
        let fit = planner(name);
        let correctness = fit
            .obligations()
            .iter()
            .find(|obligation| {
                obligation["role"] == format!("reasoner `{name}` answer correctness")
            })
            .unwrap_or_else(|| panic!("{name}'s answer correctness"));
        match premise {
            Some(predicate) => assert_eq!(
                correctness["statement"]["premise"]["function"],
                json!({"name": predicate}),
                "{name}"
            ),
            None => assert_eq!(correctness["statement"]["premise"]["kind"], "eq", "{name}"),
        }
        assert_eq!(
            calls(&derived(fit, &format!("{name}.accept"))["body"]),
            [format!("{name}.extract")],
            "{name}'s check is erased"
        );
        assert!(
            templates(fit).contains("search_growth"),
            "{name} bounds its verifications whether or not the check runs"
        );
    }
    // Formal code, a statement and not a run, may name what executable code
    // may not.
    extended(
        "Main",
        vec![json!({"kind": "definition", "name": "formal",
                    "parameters": [{"name": "c", "type": chart}], "result": chart,
                    "body": {"kind": "call",
                             "function": {"module": "Clinic", "name": "Fever.conclusion"},
                             "arguments": [{"kind": "var", "name": "c"}]}})],
    )
    .check_ok();
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
    refused_by_lean(
        "RS-09",
        &[
            "forged-trace",
            "false-answer-correct",
            "false-invariant-answer-correct",
            "false-observation-answer-correct",
        ],
    );
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

/// A right-nested pair of `fields` terms.
fn nested(fields: usize, leaf: &dyn Fn(usize) -> Json) -> Json {
    (0..fields - 1).rev().fold(
        leaf(fields - 1),
        |tail, index| json!({"kind": "pair", "left": leaf(index), "right": tail}),
    )
}

/// A right-nested product of `fields` copies of `base`.
fn nested_type(fields: usize, base: &Json) -> Json {
    (0..fields - 1).fold(
        base.clone(),
        |tail, _| json!({"kind": "product", "left": base, "right": tail}),
    )
}

/// A type of `leaves` leaves, balanced so that a type of any size stays
/// within the JSON nesting limit the compiler reads under.
fn balanced(leaves: usize, leaf: &Json) -> Json {
    if leaves <= 1 {
        return leaf.clone();
    }
    json!({"kind": "product", "left": balanced(leaves / 2, leaf),
        "right": balanced(leaves - leaves / 2, leaf)})
}

/// Every reference to a declaration of `ours` (but not of `rules`) in `value`
/// gains `extra` among its type arguments, and every name of `ours` is
/// renamed with a trailing `Q`, so a copy of a module stands beside it.
fn widen(value: &mut Json, ours: &BTreeSet<String>, rules: &BTreeSet<String>, extra: &Json) {
    match value {
        Json::Object(map) => {
            let widened = |name: Option<String>| {
                name.is_some_and(|name| ours.contains(&name) && !rules.contains(&name))
            };
            let named = |map: &serde_json::Map<String, Json>, key: &str| {
                map.get(key)
                    .and_then(|reference| reference.get("name"))
                    .and_then(Json::as_str)
                    .map(str::to_owned)
            };
            let gain = |map: &mut serde_json::Map<String, Json>| {
                let arguments = map
                    .entry("type_arguments")
                    .or_insert_with(|| json!([]))
                    .as_array_mut()
                    .expect("type arguments");
                arguments.push(extra.clone());
            };
            if map.get("kind") == Some(&json!("call")) && widened(named(map, "function")) {
                gain(map);
            }
            if map.contains_key("type_arguments") && widened(named(map, "member")) {
                gain(map);
            }
            map.values_mut()
                .for_each(|value| widen(value, ours, rules, extra));
            let renamed = map
                .get("name")
                .and_then(Json::as_str)
                .filter(|name| {
                    ours.contains(*name)
                        && map.get("kind") != Some(&json!("var"))
                        && !map.contains_key("type")
                })
                .map(|name| format!("{name}Q"));
            if let Some(name) = renamed {
                map.insert("name".to_owned(), json!(name));
            }
        }
        Json::Array(items) => items
            .iter_mut()
            .for_each(|value| widen(value, ours, rules, extra)),
        _ => {}
    }
}

/// A copy of the example's `Budget` module in which every declaration but the
/// reasoners gains a type parameter nothing mentions, and its rules, their
/// theorems, and the reasoner use everything else at a type of `size` leaves:
/// each rule's use of its logic copies that type into the rule's soundness
/// and progress statements, so the rule's charge has to cover it.
fn wide_rule_logic(size: usize) -> P {
    let project = P::copy_example(EXAMPLE);
    let mut data = module_data(&project, "Budget");
    let nat = json!({"kind": "nat"});
    let big = balanced(size, &nat);
    let phantom = json!({"kind": "parameter", "name": "Q0"});
    let rules: BTreeSet<String> = [
        "Tick",
        "Burn",
        "tick_sound",
        "tick_progress",
        "burn_sound",
        "burn_progress",
    ]
    .map(str::to_owned)
    .into();
    let originals: Vec<Json> = declarations_mut(&mut data)
        .iter()
        .filter(|declaration| declaration["name"] != "Refund")
        .cloned()
        .collect();
    let ours: BTreeSet<String> = originals
        .iter()
        .filter(|declaration| declaration["name"] != "Spend")
        .map(|declaration| declaration["name"].as_str().expect("a name").to_owned())
        .collect();
    let mut copies = Vec::new();
    for mut declaration in originals {
        let name = declaration["name"].as_str().expect("a name").to_owned();
        let concrete = declaration["kind"] == "reasoner" || rules.contains(&name);
        widen(
            &mut declaration,
            &ours,
            &rules,
            if concrete { &big } else { &phantom },
        );
        if declaration["kind"] == "reasoner" {
            declaration["name"] = json!(format!("{name}Q"));
            declaration["claims"] = json!([]);
        } else if !concrete {
            let mut parameters = declaration["type_parameters"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            parameters.push(json!("Q0"));
            declaration["type_parameters"] = Json::Array(parameters);
        }
        copies.push(declaration);
    }
    declarations_mut(&mut data).extend(copies);
    write_module_data(&project, "Budget", &data);
    project
}

/// A copy of the example's `Screening` module whose generate-and-verify
/// reasoner `Dose` has a verifier with a type parameter nothing mentions,
/// used at a type of `size` leaves: the reasoner's own use of its verifier
/// copies that type into its declarations, so its charge has to cover it.
fn wide_generator(size: usize) -> P {
    let project = P::copy_example(EXAMPLE);
    let mut data = module_data(&project, "Screening");
    let nat = json!({"kind": "nat"});
    let big = balanced(size, &nat);
    let names = ["Safe", "safeCheck", "safe_sound"];
    let original = |data: &mut Json, name: &str| declaration_mut(data, name).clone();
    fn requalify(value: &mut Json, names: &[&str]) {
        match value {
            Json::Object(map) => {
                if let Some(name) = map
                    .get("function")
                    .and_then(|function| function["name"].as_str())
                    .filter(|name| names.contains(name))
                    .map(str::to_owned)
                {
                    map["function"]["name"] = json!(format!("{name}Q"));
                    map.insert(
                        "type_arguments".to_owned(),
                        json!([{"kind": "parameter", "name": "Q0"}]),
                    );
                }
                if let Some(Json::Array(definitions)) = map.get_mut("definitions") {
                    for definition in definitions {
                        let name = definition["name"].as_str().expect("a name").to_owned();
                        definition["name"] = json!(format!("{name}Q"));
                    }
                }
                map.values_mut().for_each(|value| requalify(value, names));
            }
            Json::Array(items) => items.iter_mut().for_each(|value| requalify(value, names)),
            _ => {}
        }
    }
    let mut added = Vec::new();
    for name in ["Safe", "safeCheck", "safe_sound", "SafeDose"] {
        let mut copy = original(&mut data, name);
        copy["name"] = json!(format!("{name}Q"));
        copy["type_parameters"] = json!(["Q0"]);
        requalify(&mut copy, &names);
        if name == "SafeDose" {
            copy["specification"] = json!({"name": "SafeQ"});
            copy["check"] = json!({"name": "safeCheckQ"});
            copy["sound"] = json!({"name": "safe_soundQ"});
        }
        added.push(copy);
    }
    let mut dose = original(&mut data, "Dose");
    dose["name"] = json!("DoseQ");
    dose["verifier"] = json!({"member": {"name": "SafeDoseQ"}, "type_arguments": [big]});
    added.push(dose);
    declarations_mut(&mut data).extend(added);
    write_module_data(&project, "Screening", &data);
    project
}

/// A copy of the example whose `Budget` module states a reasoner over
/// `copies` rules, each used at a type of `size` nested products: every use
/// copies its type arguments into each declaration that mentions the rule,
/// and the charge covers that. With `phantom`, its verifier is a copy of the
/// example's with a type parameter that nothing mentions, used at a large
/// type while every rule is used at a small one: its type arguments are
/// copied too, and so are charged.
fn wide_type_arguments(copies: usize, size: usize, search: bool, phantom: bool) -> P {
    let project = P::copy_example(EXAMPLE);
    let mut data = module_data(&project, "Budget");
    let declarations = declarations_mut(&mut data);
    let original = |name: &str| {
        declarations
            .iter()
            .find(|declaration| declaration["name"] == name)
            .unwrap_or_else(|| panic!("`{name}`"))
            .clone()
    };
    let nat = json!({"kind": "nat"});
    // With `phantom` the rules, the logic, and the state stay small and only
    // the phantom argument is large, so nothing but the verifier's own type
    // arguments carries the size.
    let wide = balanced(if phantom { 2 } else { size }, &nat);
    let big = balanced(size, &nat);
    let state = json!({"kind": "product", "left": wide, "right": nat});
    let at = |ty: &Json| vec![ty.clone()];
    let mut added = Vec::new();
    let mut uses = Vec::new();
    for copy in 0..copies {
        let renamed = |name: &str| format!("{name}{copy}");
        let mut sound = original("tick_sound");
        sound["name"] = json!(renamed("tick_sound"));
        let mut progress = original("tick_progress");
        progress["name"] = json!(renamed("tick_progress"));
        let mut rule = original("Tick");
        rule["name"] = json!(renamed("Tick"));
        rule["soundness"] = json!({"name": renamed("tick_sound")});
        rule["progress"] = json!({"name": renamed("tick_progress")});
        added.extend([sound, progress, rule]);
        uses.push(json!({"member": {"name": renamed("Tick")}, "type_arguments": at(&wide)}));
    }
    if phantom {
        // `Empty`, `emptyCheck`, `empty_sound`, and `Drained` again, over a
        // parameter `Q` that none of them mentions.
        fn qualify(value: &mut Json) {
            match value {
                Json::Object(object) => {
                    let named = |object: &serde_json::Map<String, Json>| {
                        matches!(
                            object
                                .get("function")
                                .and_then(|member| member["name"].as_str()),
                            Some("Empty" | "emptyCheck")
                        )
                    };
                    if object.get("kind") == Some(&json!("call")) && named(object) {
                        let name = object["function"]["name"]
                            .as_str()
                            .expect("a name")
                            .to_owned();
                        object["function"]["name"] = json!(format!("{name}Q"));
                        object.insert(
                            "type_arguments".to_owned(),
                            json!([{"kind": "parameter", "name": "T"}, {"kind": "parameter", "name": "Q"}]),
                        );
                    }
                    object.values_mut().for_each(qualify);
                }
                Json::Array(items) => items.iter_mut().for_each(qualify),
                _ => {}
            }
        }
        for name in ["Empty", "emptyCheck", "empty_sound", "Drained"] {
            let mut copy = original(name);
            copy["name"] = json!(format!("{name}Q"));
            copy["type_parameters"] = json!(["T", "Q"]);
            qualify(&mut copy);
            if name == "Drained" {
                copy["specification"] = json!({"name": "EmptyQ"});
                copy["check"] = json!({"name": "emptyCheckQ"});
                copy["sound"] = json!({"name": "empty_soundQ"});
            }
            if name == "empty_sound" {
                copy["proof"]["definitions"] = json!([{"name": "EmptyQ"}, {"name": "emptyCheckQ"}]);
            }
            added.push(copy);
        }
    }
    let mut reasoner = original("Spend");
    reasoner["name"] = json!("Wide");
    reasoner
        .as_object_mut()
        .expect("a reasoner")
        .remove("type_parameters");
    reasoner["observation"]["type"] = state.clone();
    reasoner["logic"]["type_arguments"] = json!(at(&wide));
    reasoner["verifier"]["type_arguments"] = json!([state]);
    if phantom {
        reasoner["verifier"] =
            json!({"member": {"name": "DrainedQ"}, "type_arguments": [state, big]});
    }
    reasoner["rules"] = Json::Array(uses);
    reasoner["claims"] = json!([]);
    if search {
        reasoner["strategy"] = json!({"kind": "search", "order": "breadth_first",
            "deduplicate": true,
            "fuel": reasoner["strategy"]["fuel"], "frontier": {"kind": "nat", "value": "20"}});
    }
    added.push(reasoner);
    declarations.extend(added);
    write_module_data(&project, "Budget", &data);
    project
}

/// A reasoner over a state of `fields` natural numbers with `rules` rules,
/// each concluding a whole new state, half of them binding a tuple drawn from
/// a list of candidates: the shapes that stress what linking charges for an
/// elaboration, which is larger than its source by the weight of the types
/// and terms it copies.
#[allow(clippy::too_many_lines)]
fn wide_reasoner(strategy: &str, rules: usize, fields: usize, width: usize) -> P {
    let project = P::negative("reasoning-forged-trace");
    let nat = json!({"kind": "nat"});
    let number = |value: usize| json!({"kind": "nat", "value": value.to_string()});
    let var = |name: &str| json!({"kind": "var", "name": name});
    let state_type = nested_type(fields, &nat);
    let tuple_type = nested_type(width, &nat);
    let call = |name: &str, arguments: Vec<Json>| json!({"kind": "call", "function": {"name": name}, "arguments": arguments});
    let first = |value: Json| json!({"kind": "first", "value": value});
    let truth = json!({"kind": "bool", "value": true});
    let trivial = json!({"kind": "le", "left": number(0), "right": number(0)});
    let mut declarations = vec![
        json!({"kind": "definition", "name": "Rel", "parameters": [
            {"name": "s", "type": state_type}, {"name": "t", "type": state_type}],
            "result": {"kind": "prop"}, "body": trivial}),
        json!({"kind": "logic", "name": "L",
            "state": {"name": "s", "next": "t", "type": state_type}, "relation": {"name": "Rel"}}),
        json!({"kind": "definition", "name": "Spec", "parameters": [
            {"name": "x", "type": nat}, {"name": "r", "type": nat}],
            "result": {"kind": "prop"}, "body": trivial}),
        json!({"kind": "definition", "name": "chk", "executable": true, "parameters": [
            {"name": "x", "type": nat}, {"name": "r", "type": nat}],
            "result": {"kind": "bool"}, "body": truth}),
        json!({"kind": "theorem", "name": "chk_sound", "parameters": [
            {"name": "x", "type": nat}, {"name": "r", "type": nat}],
            "statement": {"kind": "implies",
                "premise": {"kind": "eq", "left": call("chk", vec![var("x"), var("r")]), "right": truth},
                "conclusion": call("Spec", vec![var("x"), var("r")])},
            "proof": {"kind": "linear_arithmetic", "definitions": [{"name": "Spec"}, {"name": "chk"}]}}),
        json!({"kind": "verifier", "name": "V", "subject": {"name": "x", "type": nat},
            "candidate": {"name": "r", "type": nat}, "specification": {"name": "Spec"},
            "check": {"name": "chk"}, "sound": {"name": "chk_sound"}}),
    ];
    let mut uses = Vec::new();
    for index in 0..rules {
        let name = format!("R{index}");
        let sound = format!("r{index}_sound");
        let state = var("s");
        let conclusion = nested(
            fields,
            &|field| json!({"kind": "add", "left": first(state.clone()), "right": number(field + index)}),
        );
        let mut parameters = vec![json!({"name": "s", "type": state_type})];
        let mut guard =
            json!({"kind": "blt", "left": number(index), "right": first(state.clone())});
        let mut binding = None;
        if index % 2 == 1 {
            let candidates: Vec<Json> = (0..4)
                .map(|candidate| nested(width, &|field| number(field + candidate)))
                .collect();
            parameters.push(json!({"name": "b", "type": tuple_type}));
            guard = json!({"kind": "and",
                "left": {"kind": "blt", "left": first(var("b")), "right": first(state.clone())},
                "right": guard});
            binding = Some(json!({"name": "b", "type": tuple_type,
                "candidates": candidates.iter().rev().fold(
                    json!({"kind": "nil", "element": tuple_type}),
                    |tail, head| json!({"kind": "cons", "head": head, "tail": tail}))}));
        }
        declarations.push(
            json!({"kind": "theorem", "name": sound, "parameters": parameters,
            "statement": {"kind": "implies",
                "premise": {"kind": "eq", "left": guard, "right": truth},
                "conclusion": call("Rel", vec![state.clone(), conclusion.clone()])},
            "proof": {"kind": "linear_arithmetic", "definitions": [{"name": "Rel"}]}}),
        );
        let mut rule = json!({"kind": "inference_rule", "name": name, "logic": {"member": {"name": "L"}},
            "guard": guard, "conclusion": conclusion, "soundness": {"name": sound}, "executable": true});
        if let Some(binding) = binding {
            rule["binding"] = binding;
        }
        declarations.push(rule);
        uses.push(json!({"member": {"name": name}}));
    }
    let strategy = match strategy {
        "forward" => json!({"kind": "forward", "fuel": number(50)}),
        "breadth_first" => json!({"kind": "search", "order": "breadth_first", "fuel": number(50),
            "frontier": number(20), "deduplicate": true}),
        _ => {
            json!({"kind": "search", "order": "depth_first", "fuel": number(50), "frontier": number(20)})
        }
    };
    declarations.push(json!({"kind": "reasoner", "name": "E", "logic": {"member": {"name": "L"}},
        "observation": {"name": "x", "type": nat},
        "observe": nested(fields, &|field| json!({"kind": "add", "left": var("x"), "right": number(field)})),
        "rules": uses, "strategy": strategy,
        "answer": {"name": "s", "type": nat, "value": {"kind": "constructor",
            "constructor": {"name": "Option.some"}, "type_arguments": [nat],
            "arguments": [first(var("s"))]}},
        "verifier": {"member": {"name": "V"}}, "executable": true}));
    let mut data = module_data(&project, "Main");
    data["declarations"] = Json::Array(declarations);
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
    // Large states, bindings, and terms: the charge covers every shape's
    // elaboration (an elaboration beyond its charge is an internal error, so
    // each of these linking is the check).
    for (strategy, rules, fields, width) in [
        ("forward", 20, 40, 4),
        ("breadth_first", 20, 40, 4),
        ("depth_first", 40, 12, 4),
        ("forward", 3, 100, 4),
        ("breadth_first", 12, 100, 4),
        // A wide binding over a small state.
        ("forward", 12, 2, 100),
        ("depth_first", 12, 2, 100),
    ] {
        wide_reasoner(strategy, rules, fields, width).check_ok();
    }
    // Large type arguments at every rule use are charged too.
    for (search, phantom) in [(false, false), (true, false), (false, true), (true, true)] {
        wide_type_arguments(if phantom { 2 } else { 40 }, 100, search, phantom).check_ok();
    }
    // A verifier's own type arguments, which no rule mentions, are charged.
    wide_type_arguments(2, 1200, false, true).check_ok();
    // A rule over a logic used at a large type, and a generate-and-verify
    // reasoner whose verifier has a parameter nothing mentions at a large
    // type, are charged for the copies of those type arguments too.
    wide_rule_logic(1200).check_ok();
    wide_generator(1200).check_ok();
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
    assert!(bounds.contains(&"Reasoning.Clinic.Triage.firings_bounded".to_owned()));
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
    let plan_bounds: Vec<String> =
        serde_json::from_value(plan[0]["bounds"].clone()).expect("bounds");
    for bound in [
        "iterations_bounded",
        "expansions_bounded",
        "verifications_bounded",
        "frontier_bounded",
    ] {
        assert!(
            plan_bounds.contains(&format!("Reasoning.Planner.Plan.{bound}")),
            "Plan states {bound}"
        );
    }
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

/// The numbers of the ledger a committed example theorem states, in counter
/// order: its right-hand side is a right-nested pair of literals.
fn stated_ledger(theorems: &[Json], name: &str) -> [u64; 6] {
    let theorem = theorems
        .iter()
        .find(|declaration| declaration["name"] == name)
        .unwrap_or_else(|| panic!("the example states the ledger theorem {name}"));
    let mut node = &theorem["statement"]["right"];
    let mut out = Vec::new();
    while node["kind"] == "pair" {
        out.push(node["left"]["value"].as_str().expect("a literal"));
        node = &node["right"];
    }
    out.push(node["value"].as_str().expect("a literal"));
    out.iter()
        .map(|number| number.parse().expect("a count"))
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap_or_else(|_| panic!("{name} states six counters"))
}

/// A module's declarations, with every reference to `from` made to `to` and
/// every definitions list in key order, so two statements of one reasoner
/// compare equal.
fn normalized(mut declarations: Json, from: &str, to: &str) -> Json {
    fn walk(value: &mut Json, from: &str, to: &str) {
        match value {
            Json::Object(object) => {
                if object.get("module").and_then(Json::as_str) == Some(from) {
                    object.insert("module".to_owned(), json!(to));
                }
                for child in object.values_mut() {
                    walk(child, from, to);
                }
                if let Some(Json::Array(definitions)) = object.get_mut("definitions") {
                    definitions.sort_by_key(|member| {
                        format!(
                            "{}::{}",
                            member["module"].as_str().unwrap_or(""),
                            member["name"].as_str().unwrap_or("")
                        )
                    });
                }
            }
            Json::Array(items) => items.iter_mut().for_each(|item| walk(item, from, to)),
            _ => {}
        }
    }
    walk(&mut declarations, from, to);
    declarations
}

/// Transcriptions that disagree with their oracles: `Plan` for target 2
/// trying the pour candidates in the other order, `Plan` for target 5
/// with its visited-state check removed (so it never saturates and answers
/// exhausted where the oracle's search is unsolved), and `Dose` for weight
/// 120 with a budget of four candidates (so it rejects where the oracle
/// exhausts its budget). The interpreter follows each mutation, so only
/// the statement against the oracle fails.
fn planted_transcriptions() -> Vec<crate::calculus::Case> {
    let named = |name: &str| {
        crate::calculus::cases()
            .into_iter()
            .find(|case| case.fixture.name == name)
            .unwrap_or_else(|| panic!("fixture {name}"))
    };
    let rerun = |mut case: crate::calculus::Case| {
        lexlean::calculus::check::check(&case.fixture.program).expect("still well typed");
        case.fixture.expected = interp::run(
            &case.fixture.program,
            case.fixture.fuel,
            case.fixture.entry,
            &case.fixture.arguments,
        );
        case
    };
    // `fresh_step` keeps its second branch: every node is fresh.
    let mut plan = named("reasoning-plan-5");
    let at = crate::calculus::reasoning::function_index("plan", "fresh_step");
    let lexlean::calculus::Expr::Cond { else_branch, .. } =
        plan.fixture.program.functions[at].body.clone()
    else {
        panic!("fresh_step tests whether a state was visited")
    };
    plan.fixture.program.functions[at].body = *else_branch;
    // Without the check the search runs longer than the fixture's fuel.
    plan.fixture.fuel = 400_000;
    // `attempt` compares the verifications with 4, not 3.
    let mut dose = named("reasoning-dose-120");
    let at = crate::calculus::reasoning::function_index("dose", "attempt");
    fn widen(expression: &mut lexlean::calculus::Expr) {
        use lexlean::calculus::Expr;
        match expression {
            Expr::Value {
                value: Value::Nat { value },
                ..
            } if value == "3" => *value = "4".to_owned(),
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => {
                widen(condition);
                widen(then_branch);
                widen(else_branch);
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                widen(scrutinee);
                arms.iter_mut().for_each(|arm| widen(&mut arm.body));
            }
            Expr::Prim { operands, .. }
            | Expr::Build { operands, .. }
            | Expr::Call { operands, .. } => operands.iter_mut().for_each(widen),
            Expr::Field { value, .. } => widen(value),
            _ => {}
        }
    }
    widen(&mut dose.fixture.program.functions[at].body);
    // A search that tries the jugs' candidates in the other order reaches
    // the same answer in another number of iterations.
    let mut reversed = named("reasoning-plan-2");
    let at = crate::calculus::reasoning::function_index("plan", "pour_candidates");
    fn swapped(expression: &mut lexlean::calculus::Expr) {
        use lexlean::calculus::Expr;
        match expression {
            Expr::Value {
                value: Value::Nat { value },
                ..
            } if value == "1" || value == "3" => {
                *value = if value == "1" { "3" } else { "1" }.to_owned();
            }
            Expr::Build { operands, .. } => operands.iter_mut().for_each(swapped),
            _ => {}
        }
    }
    swapped(&mut reversed.fixture.program.functions[at].body);
    vec![rerun(plan), rerun(dose), rerun(reversed)]
}

/// The messages with which pinned Lean refuses a copy of the compiler
/// project whose reasoning fixtures state [`planted_transcriptions`].
fn planted_verification() -> &'static Vec<String> {
    static ERROR: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    ERROR.get_or_init(|| {
        let project = P::compiler();
        project.write(
            "src/ReasoningFixtures.lex.tex",
            &crate::calculus::reasoning_fixtures_module(&planted_transcriptions()),
        );
        let _guard = support::env_lock();
        project
            .verify_fails_with("LLV7002")
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.clone())
            .collect()
    })
}

/// §17.14, §17.16: the oracles, the transcriptions of every reasoner the
/// production roots run, their packages, and their accounting.
#[allow(clippy::too_many_lines)]
fn rs_12() {
    use crate::calculus::reasoning;
    let repository = support::repo_root();
    let read = |path: &str| {
        std::fs::read_to_string(repository.join(path).as_std_path())
            .unwrap_or_else(|error| panic!("{path}: {error}"))
    };
    let declarations = |text: &str| -> Vec<Json> {
        let (start, end) = data_bounds(text);
        serde_json::from_str::<Json>(&text[start..end]).expect("module data")["declarations"]
            .as_array()
            .expect("declarations")
            .clone()
    };
    // Each oracle states exactly the declarations of the reasoners it
    // names, as structured data after `normalized` (key-ordered definition
    // lists, the oracle's module name for the example's), not as bytes: the whole of an example module, or the reasoner and what it
    // is declared with, over the clinical module's oracle.
    for spec in reasoning::ORACLES {
        let committed = declarations(&read(&format!("compiler/src/{}.lex.tex", spec.module)));
        let mut source = declarations(&read(spec.source));
        if let Some(names) = spec.select {
            source.retain(|declaration| {
                declaration["name"]
                    .as_str()
                    .is_some_and(|name| names.contains(&name))
            });
        }
        let (from, to) = spec.import.unwrap_or(("", ""));
        assert_eq!(
            normalized(json!(committed), "", ""),
            normalized(json!(source), from, to),
            "{} states exactly the declarations of {}",
            spec.module,
            spec.source
        );
    }
    // Every reasoner a production root runs is stated by an oracle.
    let project = P::copy_example(EXAMPLE);
    let report = eligibility(&project, "Main");
    let production: BTreeSet<String> = report["roots"]
        .as_array()
        .expect("roots")
        .iter()
        .flat_map(|root| root["reasoning"].as_array().expect("reasoning").clone())
        .map(|row| row["reasoner"].as_str().expect("reasoner").to_owned())
        .collect();
    let stated: BTreeSet<String> = reasoning::ORACLES
        .iter()
        .flat_map(|spec| {
            let module = spec.source.rsplit('/').next().expect("a file name");
            let module = module.trim_end_matches(".lex.tex").to_owned();
            declarations(&read(spec.source))
                .into_iter()
                .filter(|declaration| declaration["kind"] == "reasoner")
                .filter(|declaration| {
                    spec.select.is_none_or(|names| {
                        names.contains(&declaration["name"].as_str().unwrap_or(""))
                    })
                })
                .map(move |declaration| {
                    format!(
                        "Reasoning.{module}.{}",
                        declaration["name"].as_str().expect("name")
                    )
                })
        })
        .collect();
    assert_eq!(
        production,
        BTreeSet::from(
            [
                "Reasoning.Clinic.Triage",
                "Reasoning.Main.Grade",
                "Reasoning.Planner.Plan",
                "Reasoning.Screening.Dose",
                "Reasoning.Screening.Screen",
                "Reasoning.Budget.Spend",
            ]
            .map(str::to_owned)
        ),
        "the production roots run these reasoners"
    );
    assert!(
        production.is_subset(&stated),
        "every production reasoner is stated by an oracle: {production:?} / {stated:?}"
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
            "reasoning-dose-120",
            "reasoning-dose-60",
            "reasoning-gnaf-goal",
            "reasoning-gnaf-priority",
            "reasoning-gnaf-sweep",
            "reasoning-grade-shock",
            "reasoning-grade-well",
            "reasoning-plan-2",
            "reasoning-plan-5",
            "reasoning-review-exhausted",
            "reasoning-review-rejected",
            "reasoning-screen-0",
            "reasoning-screen-100",
            "reasoning-screen-60",
            "reasoning-spend-2",
            "reasoning-triage-shock",
            "reasoning-triage-traced",
            "reasoning-triage-well",
        ]
    );
    // Each production reasoner has a transcription whose oracle is that
    // reasoner's own verdict, or its explained answer.
    for reasoner in ["Triage", "Grade", "Plan", "Dose", "Screen", "Spend"] {
        let verdict = format!("\"name\":\"{reasoner}.verdict\"");
        let explained = format!("\"name\":\"{reasoner}\"");
        assert!(
            cases
                .iter()
                .any(|case| case.oracle.as_ref().is_some_and(|oracle| {
                    let text = oracle.to_string();
                    text.contains(&verdict) || text.contains(&explained)
                })),
            "a transcription of {reasoner}"
        );
    }
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
    let jugs = |left: u64, right: u64| Value::Ok {
        value: Box::new(Value::Pair {
            left: Box::new(Value::Nat {
                value: left.to_string(),
            }),
            right: Box::new(Value::Nat {
                value: right.to_string(),
            }),
        }),
    };
    let fixtures_text = read("compiler/src/ReasoningFixtures.lex.tex");
    let ledgers = declarations(&read("examples/reasoning/src/Main.lex.tex"));
    // The example's kernel-decided ledger of the reasoner's run on each
    // fixture's argument.
    let ledger_of = |fixture: &str| -> Option<[u64; 6]> {
        let theorem = match fixture {
            "reasoning-triage-shock" | "reasoning-triage-traced" => "triage_ledger",
            "reasoning-triage-well" => "triage_well_ledger",
            "reasoning-review-exhausted" => "review_ledger",
            "reasoning-review-rejected" => "review_rejected_ledger",
            "reasoning-grade-shock" => "grade_ledger",
            "reasoning-grade-well" => "grade_well_ledger",
            "reasoning-spend-2" => "spend_ledger",
            "reasoning-plan-2" => "plan_ledger",
            "reasoning-plan-5" => "plan_unsolved_ledger",
            "reasoning-screen-60" => "screen_ledger",
            "reasoning-screen-100" => "screen_truncated_ledger",
            "reasoning-screen-0" => "screen_unsolved_ledger",
            "reasoning-dose-60" => "dose_ledger",
            "reasoning-dose-120" => "dose_exhausted_ledger",
            _ => return None,
        };
        Some(stated_ledger(&ledgers, theorem))
    };
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
        let Outcome::Value {
            value: answered,
            steps,
        } = &case.fixture.expected
        else {
            panic!("{} returns a value", case.fixture.name)
        };
        // A search and a generation answer the explained verdict with the
        // ledger of the run: the verdict is compared here without its steps,
        // and the ledger with the one the example states for the same run.
        let explained = ["reasoning-plan-", "reasoning-screen-", "reasoning-dose-"]
            .iter()
            .any(|prefix| case.fixture.name.starts_with(prefix));
        let value = if explained {
            let Value::Pair { left, right } = answered else {
                panic!("{} answers a verdict and a ledger", case.fixture.name)
            };
            let mut chain = Vec::new();
            let mut node = right.as_ref();
            while let Value::Pair { left, right } = node {
                chain.push(count(left));
                node = right;
            }
            chain.push(count(node));
            assert_eq!(
                Some(chain),
                ledger_of(&case.fixture.name).map(Vec::from),
                "{} accounts the ledger the example states",
                case.fixture.name
            );
            match left.as_ref() {
                Value::Ok { value } => {
                    let Value::Pair { left, right } = value.as_ref() else {
                        panic!("an answer with its steps")
                    };
                    let Value::List { items } = right.as_ref() else {
                        panic!("steps")
                    };
                    assert!(!items.is_empty(), "an answer names the steps that reach it");
                    Value::Ok {
                        value: left.clone(),
                    }
                }
                other => other.clone(),
            }
        } else {
            answered.clone()
        };
        let value = &value;
        let expected = match case.fixture.name.as_str() {
            "reasoning-triage-shock" | "reasoning-grade-shock" => Some(level(3)),
            "reasoning-triage-well" | "reasoning-grade-well" => Some(level(0)),
            "reasoning-review-exhausted" | "reasoning-screen-100" | "reasoning-dose-120" => {
                Some(failure(false, false))
            }
            "reasoning-review-rejected" => Some(failure(true, false)),
            "reasoning-plan-5" | "reasoning-screen-0" => Some(failure(false, true)),
            "reasoning-plan-2" => Some(jugs(2, 2)),
            "reasoning-screen-60" | "reasoning-dose-60" => Some(level(30)),
            "reasoning-spend-2" => Some(level(0)),
            _ => None,
        };
        if let Some(expected) = expected {
            assert_eq!(value, &expected, "{}", case.fixture.name);
        }
        // A transcription charges at least the guard evaluations and
        // firings the reasoner's ledger accounts on the same argument.
        if let Some(ledger) = ledger_of(&case.fixture.name) {
            let floor = ledger[1] + ledger[2];
            assert!(
                *steps >= floor,
                "{} charges its search: {steps} steps for {floor} guard evaluations and firings",
                case.fixture.name
            );
            // The floor is falsifiable: the same answer from a transcription
            // that searches nothing is charged fewer steps than the ledger.
            if floor > 0 {
                let mut constant = case.fixture.program.clone();
                constant.functions[0].body = lexlean::calculus::Expr::Value {
                    ty: constant.functions[0].result.clone(),
                    value: answered.clone(),
                };
                let Outcome::Value {
                    value: reached,
                    steps: constant_steps,
                } = interp::run(
                    &constant,
                    case.fixture.fuel,
                    case.fixture.entry,
                    &case.fixture.arguments,
                )
                else {
                    panic!("the constant transcription returns")
                };
                assert_eq!(&reached, answered);
                assert!(
                    constant_steps < floor,
                    "{}: an uncharged search is detected ({constant_steps} < {floor})",
                    case.fixture.name
                );
            }
        }
    }
    // The traced transcription accounts its search, and the kernel decides
    // that the account is the oracle's; the example states the same ledger.
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
    let stated = ledger_of("reasoning-triage-traced").expect("a ledger");
    assert_eq!(
        (attempts, firings),
        (stated[1], stated[2]),
        "the account the kernel decides is the ledger the example states"
    );
    // The packages: every transcription in rust-std, and in rust-core when
    // it allocates nothing: the traced one allocates its trace, the others'
    // searches and candidate lists are lists.
    let packaged: BTreeSet<(String, String)> = crate::rust_packages::packages()
        .into_iter()
        .filter(|committed| committed.fixture.starts_with("reasoning-"))
        .map(|committed| (committed.fixture, committed.profile.target().to_owned()))
        .collect();
    let allocating = [
        "reasoning-triage-traced",
        "reasoning-spend-2",
        "reasoning-plan-2",
        "reasoning-plan-5",
        "reasoning-screen-0",
        "reasoning-screen-60",
        "reasoning-screen-100",
        "reasoning-dose-60",
        "reasoning-dose-120",
    ];
    for name in &named {
        assert!(
            packaged.contains(&((*name).to_owned(), "rust-std".to_owned())),
            "{name} in rust-std"
        );
        assert_eq!(
            packaged.contains(&((*name).to_owned(), "rust-core".to_owned())),
            !allocating.contains(name),
            "{name} in rust-core exactly when it allocates nothing"
        );
    }
    assert!(
        crate::calculus::check(repository.as_std_path(), false).is_ok(),
        "the committed transcriptions, oracles, and packages equal their generator"
    );
    if support::lean_backed("RS-12") {
        let verified = support::verified_compiler();
        for unit in [
            "ReasoningOracle",
            "GradeOracle",
            "BudgetOracle",
            "PlannerOracle",
            "ScreeningOracle",
            "ReasoningFixtures",
        ] {
            assert!(
                verified.outcome.units.contains_key(unit),
                "{unit} is verified"
            );
        }
        for case in &cases {
            let id = crate::calculus::identifier(&case.fixture.name);
            assert_attested_ok(
                &verified.attestation,
                &[&format!("Compiler.ReasoningFixtures.{id}Agrees")],
            );
        }
        // The statements are falsifiable: a transcription that forgets the
        // states it visited, and one that checks a candidate too many,
        // compute other verdicts than their oracles, and Lean refuses them.
        let planted = planted_verification();
        for (run, oracle) in [
            ("reasoningPlan5Run", "PlannerOracle.Plan 5"),
            ("reasoningDose120Run", "ScreeningOracle.Dose 120"),
            ("reasoningPlan2Run", "PlannerOracle.Plan 2"),
        ] {
            assert!(
                planted
                    .iter()
                    .any(|message| message.contains(run) && message.contains(oracle)),
                "a transcription that disagrees with its oracle is refused ({run}): {planted:#?}"
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
    // Each plan is the problem the oracle's Triage solves: on every patient
    // of the domain it computes the level Triage derives from the same
    // findings, observed as vital signs, as the kernel decides, and the
    // function the fixture states is the plan the request declares.
    let cases = crate::calculus::cases();
    let fixtures_text = std::fs::read_to_string(
        support::repo_root()
            .join("compiler/src/ReasoningFixtures.lex.tex")
            .as_std_path(),
    )
    .expect("the reasoning fixtures");
    for (plan, name) in [
        (0, "reasoning-gnaf-priority"),
        (1, "reasoning-gnaf-sweep"),
        (2, "reasoning-gnaf-goal"),
    ] {
        let case = cases
            .iter()
            .find(|case| case.fixture.name == name)
            .unwrap_or_else(|| panic!("fixture {name}"));
        assert_eq!(
            case.fixture.arguments,
            DOMAIN
                .iter()
                .map(|findings| Value::Nat {
                    value: findings.to_string()
                })
                .collect::<Vec<_>>(),
            "{name} runs the request's domain"
        );
        let system = lexlean::gnaf::realize(&argmin.request.carrier, Selector::Fixed { plan })
            .expect("a system")
            .canonical()
            .expect("a valid system");
        assert_eq!(
            case.fixture.program.functions[1],
            system.functions[1 + plan as usize],
            "{name} states the plan the request declares"
        );
        let id = crate::calculus::identifier(name);
        assert!(
            fixtures_text.contains(&format!("\"name\":\"{id}Agrees\"")),
            "{id}Agrees is stated"
        );
        let Outcome::Value { value, .. } = &case.fixture.expected else {
            panic!("{name} returns a value")
        };
        // Every finding, none, an infection after two findings, two findings
        // alone, and tachypnea with hypotension.
        let levels = [3u64, 0, 2, 1, 0];
        let mut tuple = Value::Nat {
            value: levels[levels.len() - 1].to_string(),
        };
        for level in levels[..levels.len() - 1].iter().rev() {
            tuple = Value::Pair {
                left: Box::new(Value::Nat {
                    value: level.to_string(),
                }),
                right: Box::new(tuple),
            };
        }
        assert_eq!(value, &tuple, "{name} computes Triage's levels");
    }
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
                "Compiler.ReasoningFixtures.reasoningGnafPriorityAgrees",
                "Compiler.ReasoningFixtures.reasoningGnafSweepAgrees",
                "Compiler.ReasoningFixtures.reasoningGnafGoalAgrees",
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
