//! The `gnaf` suite: GN-01..GN-08, GNAF requests over the production
//! realization calculus (SPEC.md §17.15).

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::OnceLock;

use lexlean::calculus::{interp, Outcome, Program, Value};
use lexlean::gnaf::{
    self, Action, ActionKind, Answer, Boundary, Carrier, Charge, ClaimClass, Completeness, Fixture,
    Objective, OperandSize, Prepared, Rejection, Request, Scope, Selector, Status, HOST_CAPACITY,
};
use lexlean::Sha256Digest;
use serde::Deserialize;
use serde_json::{json, Value as Json};

use crate::gnaf as fixtures;
use crate::support::{self, repo_root, P};

/// The committed fixtures, read from disk exactly as published.
fn committed() -> Vec<(PathBuf, Vec<u8>, Fixture)> {
    let dir = repo_root().join("compiler/gnaf");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir.as_std_path()).expect("compiler/gnaf exists") {
        let path = entry.expect("entry").path();
        let bytes = std::fs::read(&path).expect("fixture bytes");
        let fixture: Fixture = serde_json::from_slice(&bytes)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        out.push((path, bytes, fixture));
    }
    out.sort_by(|left, right| left.0.cmp(&right.0));
    assert!(!out.is_empty(), "GNAF has hand-constructed requests");
    out
}

fn read_json(relative: &str) -> Json {
    serde_json::from_slice(
        &std::fs::read(repo_root().join(relative).as_std_path())
            .unwrap_or_else(|error| panic!("{relative}: {error}")),
    )
    .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn schema(name: &str) -> Json {
    read_json(&format!("schemas/{name}"))
}

fn fixture(name: &str) -> Fixture {
    fixtures::fixtures()
        .into_iter()
        .find(|fixture| fixture.name == name)
        .unwrap_or_else(|| panic!("no GNAF fixture {name}"))
}

fn request(name: &str) -> Request {
    fixture(name).request
}

fn answer(request: &Request) -> Answer {
    gnaf::answer(request).expect("within capacity")
}

fn statuses(request: &Request) -> Vec<(u64, Selector, Status)> {
    gnaf::statuses(request).expect("within capacity")
}

fn rejected(rejection: Rejection) -> Answer {
    Answer::Rejected { rejection }
}

fn admitted(status: Status) -> (u64, u64) {
    match status {
        Status::Admitted { steps, size } => (steps, size),
        other => panic!("not admitted: {other:?}"),
    }
}

fn admitted_steps(status: Status) -> u64 {
    admitted(status).0
}

fn index(value: u64) -> usize {
    usize::try_from(value).expect("an index fits")
}

/// A load expected to fail closed with `code` naming `message`.
fn refuses(bytes: &[u8], code: &str, message: &str) {
    let error = gnaf::load(bytes)
        .err()
        .unwrap_or_else(|| panic!("an invalid request loads: expected `{message}`"));
    support::expect_code(&error, code);
    assert!(
        error.to_string().contains(message),
        "expected `{message}` in: {error}"
    );
}

/// The universe identity of `request`, which every committed request
/// states.
fn universe_id(request: &Request) -> Sha256Digest {
    gnaf::universe_id(request).expect("a canonical request")
}

fn identified(mut request: Request) -> Request {
    request.universe = universe_id(&request);
    request
}

/// A request's JSON with its universe identity restated for its edited
/// components, so that a refusal is the edit's and not the identity's.
fn reidentified(json: &Json) -> Vec<u8> {
    let request: Request = serde_json::from_value(json.clone()).expect("a request");
    let mut json = json.clone();
    json["universe"] = json!(universe_id(&request).to_hex());
    bytes(&json)
}

fn request_json(name: &str) -> Json {
    serde_json::to_value(request(name)).expect("request serializes")
}

fn bytes(json: &Json) -> Vec<u8> {
    serde_json::to_vec(json).expect("serializes")
}

/// The semantic data of a committed `compiler/src` module.
fn module_data(text: &str) -> Json {
    let start = text.find("\\semanticdata{").expect("semantic data") + "\\semanticdata{".len();
    let end = text
        .rfind("}\n\\end{semanticmodule}")
        .expect("end of semantic data");
    serde_json::from_str(&text[start..end]).expect("module data")
}

fn with_module_data(text: &str, data: &Json) -> String {
    let start = text.find("\\semanticdata{").expect("semantic data") + "\\semanticdata{".len();
    let end = text
        .rfind("}\n\\end{semanticmodule}")
        .expect("end of semantic data");
    format!(
        "{}{}{}",
        &text[..start],
        serde_json::to_string(data).expect("serializes"),
        &text[end..]
    )
}

fn model_text() -> String {
    std::fs::read_to_string(repo_root().join("compiler/src/Gnaf.lex.tex").as_std_path())
        .expect("Gnaf source")
}

/// The declaration `name` of the committed `Gnaf` model.
fn declaration(name: &str) -> Json {
    module_data(&model_text())["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .find(|declaration| declaration["name"] == name)
        .unwrap_or_else(|| panic!("`Gnaf.{name}` is declared"))
        .clone()
}

/// Every name the declaration `name` calls or references, transitively
/// through the model's own declarations; another module's member is
/// `Module.name`.
fn dependencies(name: &str) -> BTreeSet<String> {
    let data = module_data(&model_text());
    let declarations = data["declarations"].as_array().expect("declarations");
    let mut out = BTreeSet::new();
    let mut pending = vec![name.to_owned()];
    while let Some(next) = pending.pop() {
        let Some(found) = declarations
            .iter()
            .find(|declaration| declaration["name"] == next.as_str())
        else {
            continue;
        };
        let mut referenced = Vec::new();
        collect_references(&found["body"], &mut referenced);
        for reference in referenced {
            if out.insert(reference.clone()) {
                pending.push(reference);
            }
        }
    }
    out
}

fn collect_references(term: &Json, out: &mut Vec<String>) {
    match term {
        Json::Object(object) => {
            if matches!(
                object.get("kind").and_then(Json::as_str),
                Some("call" | "function_ref")
            ) {
                if let Some(function) = object.get("function") {
                    let module = function.get("module").and_then(Json::as_str);
                    if let Some(name) = function.get("name").and_then(Json::as_str) {
                        out.push(
                            module.map_or_else(
                                || name.to_owned(),
                                |module| format!("{module}.{name}"),
                            ),
                        );
                    }
                }
            }
            for value in object.values() {
                collect_references(value, out);
            }
        }
        Json::Array(items) => {
            for item in items {
                collect_references(item, out);
            }
        }
        _ => {}
    }
}

/// A list literal's elements, decoded by `item`.
fn decode_list<T>(term: &Json, item: impl Fn(&Json) -> T) -> Vec<T> {
    let mut out = Vec::new();
    let mut cursor = term;
    while cursor["kind"] == "cons" {
        out.push(item(&cursor["head"]));
        cursor = &cursor["tail"];
    }
    assert_eq!(cursor["kind"], "nil", "a list literal: {term}");
    out
}

fn decode_nat(term: &Json) -> u64 {
    assert_eq!(term["kind"], "nat", "a natural literal: {term}");
    term["value"]
        .as_str()
        .expect("digits")
        .parse()
        .expect("a natural")
}

fn decode_nats(term: &Json) -> Vec<u64> {
    decode_list(term, decode_nat)
}

fn decode_pair<A, B>(term: &Json, left: impl Fn(&Json) -> A, right: impl Fn(&Json) -> B) -> (A, B) {
    assert_eq!(term["kind"], "pair", "a pair: {term}");
    (left(&term["left"]), right(&term["right"]))
}

fn decode_bool(term: &Json) -> bool {
    assert_eq!(term["kind"], "bool", "a Boolean literal: {term}");
    term["value"].as_bool().expect("a Boolean")
}

/// `some (members, cost)` of the order's argmin.
fn decode_found<T>(term: &Json, member: impl Fn(&Json) -> T) -> (Vec<T>, u64) {
    assert_eq!(
        term["constructor"]["name"], "Option.some",
        "an argmin: {term}"
    );
    decode_pair(
        &term["arguments"][0],
        |members| decode_list(members, &member),
        decode_nat,
    )
}

/// A theorem `function arguments = right` of the committed model, as the
/// function's name, its arguments, and its right-hand side.
fn statement(name: &str) -> (String, Vec<Json>, Json) {
    let theorem = declaration(name);
    assert_eq!(theorem["kind"], "theorem", "{name} is a theorem");
    let statement = &theorem["statement"];
    assert_eq!(statement["kind"], "eq", "{name} states an equality");
    let left = &statement["left"];
    assert_eq!(left["kind"], "call", "{name} states a call");
    (
        left["function"]["name"]
            .as_str()
            .expect("a name")
            .to_owned(),
        left["arguments"].as_array().expect("arguments").clone(),
        statement["right"].clone(),
    )
}

/// The arguments of a call of `function`.
fn called<'a>(term: &'a Json, function: &str) -> &'a [Json] {
    assert_eq!(term["kind"], "call", "a call of {function}: {term}");
    assert_eq!(term["function"]["name"], function, "a call of {function}");
    term["arguments"].as_array().expect("arguments")
}

/// A copy of the compiler whose `GnafFixtures` states `planted`, its
/// declarations then changed by `edit`: the messages Lean reports when it
/// refuses it.
fn planted_compiler(planted: &[Fixture], edit: impl FnOnce(&mut Vec<Json>)) -> Vec<String> {
    let text = fixtures::fixtures_module(planted);
    let mut data = module_data(&text);
    edit(data["declarations"].as_array_mut().expect("declarations"));
    let project = P::compiler();
    project.write("src/GnafFixtures.lex.tex", &with_module_data(&text, &data));
    let _guard = support::env_lock();
    let error = project.verify_fails_with("LLV7002");
    error
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.clone())
        .collect()
}

/// The planted fixture verification: a copy of the compiler whose
/// `GnafFixtures` states a wrong answer, answers computed from universes
/// with systems omitted, ties with a member dropped, and a universe with a
/// system omitted. Pinned Lean must reject every one.
fn planted_fixtures() -> &'static Vec<String> {
    static ERRORS: OnceLock<Vec<String>> = OnceLock::new();
    ERRORS.get_or_init(|| {
        let mut wrong = fixture("argmin-complete");
        let Answer::Argmin { members, steps } = wrong.expected.clone() else {
            panic!("argmin-complete has an argmin")
        };
        wrong.expected = Answer::Argmin {
            members,
            steps: steps + 1,
        };
        let omitted = |name: &str| {
            let mut planted = fixture(name);
            let mut narrowed = planted.request.clone();
            let Carrier::Grammar { thresholds, .. } = &mut narrowed.carrier else {
                panic!("{name} has a grammar")
            };
            thresholds.clear();
            planted.expected = answer(&narrowed);
            assert_ne!(
                planted.expected,
                fixture(name).expected,
                "{name}: the omission changes the answer"
            );
            planted
        };
        // GNAF-REJ-21: an equal-cost identity dropped from the answer.
        let mut argmin_tie = fixture("argmin-tie");
        let Answer::Argmin { members, steps } = argmin_tie.expected.clone() else {
            panic!("argmin-tie has an argmin")
        };
        assert!(members.len() > 1, "argmin-tie ties");
        argmin_tie.expected = Answer::Argmin {
            members: members[..1].to_vec(),
            steps,
        };
        let mut frontier_tie = fixture("frontier-tie");
        let Answer::Frontier { members } = frontier_tie.expected.clone() else {
            panic!("frontier-tie has a frontier")
        };
        frontier_tie.expected = Answer::Frontier {
            members: members[..members.len() - 1].to_vec(),
        };
        let planted = vec![
            wrong,
            omitted("argmin-over-systems"),
            omitted("frontier-incomparable"),
            argmin_tie,
            frontier_tie,
            fixture("inadmissible-plan-excluded"),
        ];
        planted_compiler(&planted, |declarations| {
            // The universe of `inadmissible-plan-excluded` stated without
            // its last system.
            let universe = declarations
                .iter_mut()
                .find(|declaration| declaration["name"] == "inadmissiblePlanExcludedUniverse")
                .expect("the universe theorem");
            let mut cursor = &mut universe["statement"]["right"];
            while cursor["tail"]["kind"] == "cons" {
                cursor = &mut cursor["tail"];
            }
            *cursor = cursor["tail"].clone();
        })
    })
}

/// The planted authority vector: a copy of the compiler whose `Gnaf`
/// states that the GNAF-VEC-02 frontier omits `rC`.
fn planted_vector() -> Vec<String> {
    let project = P::compiler();
    let text = model_text();
    let mut data = module_data(&text);
    let theorem = data["declarations"]
        .as_array_mut()
        .expect("declarations")
        .iter_mut()
        .find(|declaration| declaration["name"] == "vec02Frontier")
        .expect("vec02Frontier");
    // [0, 1, 2] becomes [0, 1]: drop the last cons cell.
    let tail = &mut theorem["statement"]["right"]["tail"]["tail"];
    assert_eq!(
        tail["kind"], "cons",
        "the stated frontier has three members"
    );
    *tail = tail["tail"].clone();
    project.write("src/Gnaf.lex.tex", &with_module_data(&text, &data));
    let _guard = support::env_lock();
    let error = project.verify_fails_with("LLV7002");
    error
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.clone())
        .collect()
}

/// The machine of a request with one action's charge replaced.
fn charged(name: &str, kind: ActionKind, charge: Charge) -> Request {
    let mut out = request(name);
    out.machine.actions.retain(|action| action.kind != kind);
    out.machine.actions.push(Action { kind, charge });
    out
}

/// The claim classes of UOR-GNAF §12.4.
fn every_claim() -> Vec<ClaimClass> {
    use ClaimClass as C;
    vec![
        C::Exact,
        C::NormalForm,
        C::Canonical,
        C::RepresentationMinimal,
        C::ComparisonTheorem,
        C::ProfileDefinedComparison {
            profile: "p".to_owned(),
            class_id: "c".to_owned(),
            shape: "s".to_owned(),
        },
        C::InputTotal,
        C::GlobalOptimal,
        C::ArgminComplete,
        C::ParetoOptimal,
        C::FrontierComplete,
        C::PointwiseEnvelopeComplete,
        C::QueryFamilyAnswerComplete,
        C::UseCaseGlobalOptimal,
        C::WorkloadArgminComplete,
        C::WorkloadParetoOptimal,
        C::WorkloadFrontierComplete,
        C::FamilyOptimal,
        C::CompetitiveBound,
        C::CompetitiveOptimal,
        C::AsymptoticBound,
        C::AsymptoticOptimal,
        C::UseCaseClassComplete,
        C::UseCaseClassAnswerComplete,
        C::MaintainedUseCaseClass,
        C::RestrictedUniverseOptimal,
        C::RevisionPreserved,
        C::BestKnown,
        C::MeasuredBestAmongTested,
        C::HeuristicSelected,
        C::InstanceOptimal { alpha: 2, beta: 1 },
    ]
}

/// The steps of `program` on `argument` under the request's fuel.
fn run_steps(request: &Request, program: &Program, argument: &Value) -> u64 {
    match interp::run(
        program,
        request.machine.fuel,
        0,
        std::slice::from_ref(argument),
    ) {
        Outcome::Value { steps, .. } => steps,
        other => panic!("{other:?}"),
    }
}

/// UOR-GNAF §20's dependency manifest, read with unknown fields refused.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    spec: String,
    authority: ManifestAuthority,
    profiles: Vec<ManifestProfile>,
    measured: Vec<String>,
    restrictions: Vec<String>,
    strongest_claims: Vec<ManifestClaim>,
    non_claims: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestAuthority {
    id: String,
    identifier: String,
    revision: String,
    sha256: String,
    vendored: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestProfile {
    name: String,
    role: String,
    source: String,
    statement: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestClaim {
    class: String,
    scope: String,
    objective: String,
}

/// Run the case for one GN conformance ID.
///
/// # Panics
///
/// Panics when the capability does not hold.
pub fn run(id: &str) {
    match id {
        "GN-01" => gn_01(),
        "GN-02" => gn_02(),
        "GN-03" => gn_03(),
        "GN-04" => gn_04(),
        "GN-05" => gn_05(),
        "GN-06" => gn_06(),
        "GN-07" => gn_07(),
        "GN-08" => gn_08(),
        _ => panic!("no GNAF case {id}"),
    }
}

/// §17.15: closed request form, canonical fixtures, fail-closed load, the
/// universe identity, and the host's capacities.
#[allow(clippy::too_many_lines)]
fn gn_01() {
    let request_schema = schema("gnaf-request.schema.json");
    let fixture_schema = schema("gnaf-fixture.schema.json");
    for (path, bytes, fixture) in committed() {
        let name = path
            .file_stem()
            .expect("stem")
            .to_string_lossy()
            .into_owned();
        assert_eq!(fixture.name, name, "a fixture is named by its file");
        assert_eq!(fixture.spec, gnaf::FIXTURE_SPEC);
        assert_eq!(
            bytes,
            gnaf::to_file_bytes(&fixture),
            "{name}: committed bytes are canonical"
        );
        let json: Json = serde_json::from_slice(&bytes).expect("json");
        let violations = crate::schema::validate(&fixture_schema, &json);
        assert!(violations.is_empty(), "{name}: {violations:?}");
        let violations = crate::schema::validate(&request_schema, &json["request"]);
        assert!(violations.is_empty(), "{name}: {violations:?}");
        assert_eq!(
            fixture.request.universe,
            universe_id(&fixture.request),
            "{name}: the stated universe identity is its components'"
        );
        let loaded = gnaf::load(&gnaf::to_file_bytes(&fixture.request))
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(answer(&loaded), fixture.expected, "{name}");
    }
    crate::calculus::check(repo_root().as_std_path(), false)
        .expect("the committed requests and schemas equal their generator");
    // The schemas' calculus definitions are the target program schema's own.
    let program = schema("target-program.schema.json");
    for (name, definition) in program["$defs"].as_object().expect("definitions") {
        for gnaf_schema in [&request_schema, &fixture_schema] {
            assert_eq!(
                &gnaf_schema["$defs"][name], definition,
                "{name} is the target program's definition"
            );
        }
    }

    let base = request_json("argmin-over-systems");
    refuses(b"{", "LLB6006", "malformed");
    let mut unknown = base.clone();
    unknown["optimizer"] = json!("fastest");
    refuses(&bytes(&unknown), "LLB6006", "unknown field");
    let mut spec = base.clone();
    spec["spec"] = json!("lexlean/gnaf-request/0");
    refuses(
        &bytes(&spec),
        "LLB6006",
        "expected `lexlean/gnaf-request/1`",
    );
    let mut reference = base.clone();
    reference["reference"]["functions"][0]["result"] = json!({"kind": "bool"});
    refuses(&reidentified(&reference), "LLB6006", "reference:");
    let mut plan = base.clone();
    plan["carrier"]["plans"][1]["function"]["result"] = json!({"kind": "bool"});
    refuses(&reidentified(&plan), "LLB6006", "system");
    let mut domain = base.clone();
    domain["domain"][0] = json!({"kind": "bool", "value": true});
    refuses(&reidentified(&domain), "LLB6006", "domain argument 0:");
    let mut signature = base.clone();
    signature["carrier"]["result"] = json!({"kind": "nat"});
    refuses(
        &reidentified(&signature),
        "LLB6006",
        "differ from the reference entry's",
    );
    let mut repeated = base.clone();
    repeated["carrier"]["thresholds"] = json!([3, 3]);
    refuses(&reidentified(&repeated), "LLB6006", "strictly increasing");
    let mut not_preparation = base.clone();
    not_preparation["carrier"]["plans"][1]["prepares"] = json!(["dispatch"]);
    refuses(
        &reidentified(&not_preparation),
        "LLB6006",
        "is not a preparation action",
    );
    let mut twice = base.clone();
    twice["carrier"]["plans"][1]["prepares"] = json!(["advice", "advice"]);
    refuses(&reidentified(&twice), "LLB6006", "twice");
    // The reference defines the problem: an argument on which it returns no
    // value is refused, never left to make systems unresolved.
    let mut starved = base.clone();
    starved["machine"]["fuel"] = json!(1);
    refuses(
        &reidentified(&starved),
        "LLB6006",
        "the reference returns no value (exhausted)",
    );
    // The universe identity is only an equality check: a request stating
    // any other identity is refused.
    let mut identity = base.clone();
    identity["universe"] = json!(Sha256Digest::of(b"another universe").to_hex());
    refuses(&bytes(&identity), "LLB6006", "universe identity");
    let mut edited = base.clone();
    edited["carrier"]["thresholds"] = json!([4]);
    refuses(&bytes(&edited), "LLB6006", "universe identity");
    let mut not_hex = base.clone();
    not_hex["universe"] = json!("UNIVERSE");
    refuses(&bytes(&not_hex), "LLB6006", "malformed");
    for violations in [&unknown, &spec, &not_hex] {
        assert!(
            !crate::schema::validate(&request_schema, violations).is_empty(),
            "the schema refuses what the loader refuses"
        );
    }

    // The capacities: a machine asking for more than the evaluator offers,
    // and a request using more, each before any evaluation.
    for (field, offered) in [
        ("fuel", HOST_CAPACITY.fuel),
        ("domain", HOST_CAPACITY.domain),
        ("systems", HOST_CAPACITY.systems),
        ("charge", HOST_CAPACITY.charge),
    ] {
        let mut asked = base.clone();
        asked["machine"]["capacity"][field] = json!(offered + 1);
        refuses(
            &reidentified(&asked),
            "LLS8002",
            &format!("a {field} capacity"),
        );
    }
    let mut fuel = base.clone();
    fuel["machine"]["fuel"] = json!(HOST_CAPACITY.fuel + 1);
    refuses(&reidentified(&fuel), "LLS8002", "fuel");
    let mut universe = base.clone();
    let one_plan = base["carrier"]["plans"][1].clone();
    universe["carrier"]["plans"] = json!(vec![one_plan; 257]);
    refuses(&reidentified(&universe), "LLS8002", "universe");
    let mut arguments = base.clone();
    arguments["domain"] = json!(vec![
        base["domain"][0].clone();
        index(HOST_CAPACITY.domain) + 1
    ]);
    refuses(&reidentified(&arguments), "LLS8002", "domain");
    // A charge beyond the capacity is refused before it can reach the
    // host's integers, whether canonical JSON can state it or not.
    for cost in [1 << 40, u64::MAX] {
        let huge = charged(
            "argmin-over-systems",
            ActionKind::Preprocessing,
            Charge::Constant { cost },
        );
        let json = serde_json::to_value(&huge).expect("json");
        // Canonical JSON cannot state `u64::MAX`, so no identity can be
        // restated for it; the capacity check precedes the identity's.
        let posed = if cost == u64::MAX {
            bytes(&json)
        } else {
            reidentified(&json)
        };
        refuses(&posed, "LLS8002", "charge");
        support::expect_code(&gnaf::answer(&huge).expect_err("over capacity"), "LLS8002");
    }
    // A threshold canonical JSON cannot represent fails closed, never
    // panicking where the universe identity is computed.
    let mut enormous = base.clone();
    enormous["carrier"]["thresholds"] = json!([u64::MAX]);
    refuses(&bytes(&enormous), "LLB6006", "not canonical JSON");
    let mut beyond = request("argmin-over-systems");
    beyond.machine.fuel = HOST_CAPACITY.fuel + 1;
    support::expect_code(
        &gnaf::answer(&beyond).expect_err("over capacity"),
        "LLS8002",
    );

    // At full capacity a diverging plan recurses as deep as the fuel lets
    // it and its unknown cost leaves the answer incomplete.
    let mut deepest = request("unknown-cost-incomplete");
    deepest.machine.fuel = HOST_CAPACITY.fuel;
    let loaded = gnaf::load(&gnaf::to_file_bytes(&identified(deepest))).expect("within capacity");
    assert_eq!(answer(&loaded), Answer::Incomplete);
}

/// §17.15: the kernel is the oracle for every committed universe, status,
/// and answer.
fn gn_02() {
    let fixtures = fixtures::fixtures();
    let module = fixtures::fixtures_module(&fixtures);
    for fixture in &fixtures {
        let id = crate::calculus::identifier(&fixture.name);
        for theorem in ["Request", "Universe", "Answer"] {
            assert!(
                module.contains(&format!("\"name\":\"{id}{theorem}\"")),
                "{id}{theorem}"
            );
        }
        assert_eq!(
            module.contains(&format!("\"name\":\"{id}Statuses\"")),
            !matches!(fixture.expected, Answer::Rejected { .. }),
            "{id}: every answered request states its systems' statuses"
        );
    }
    assert_eq!(
        std::fs::read_to_string(
            repo_root()
                .join("compiler/src/GnafFixtures.lex.tex")
                .as_std_path()
        )
        .expect("module"),
        module,
        "the committed module equals its generator"
    );
    let main = std::fs::read_to_string(repo_root().join("compiler/src/Main.lex.tex").as_std_path())
        .expect("Main");
    for import in ["Gnaf", "GnafFixtures"] {
        assert!(
            main.contains(&format!("\\importmodule{{{import}}}")),
            "the entrypoint reaches {import}, so verify checks it"
        );
    }
    if support::lean_backed("GN-02") {
        let verified = support::verified_compiler();
        assert!(
            verified.outcome.units.contains_key("GnafFixtures"),
            "the fixture module is verified"
        );
        // Lean names the request whose stated answer or universe the
        // kernel refuses to reduce to.
        let planted = planted_fixtures();
        for (function, request) in [
            ("answer", "argminCompleteRequest"),
            ("answer", "argminOverSystemsRequest"),
            ("answer", "frontierIncomparableRequest"),
            ("answer", "argminTieRequest"),
            ("answer", "frontierTieRequest"),
            ("universeOf", "inadmissiblePlanExcludedRequest"),
        ] {
            assert!(
                planted.iter().any(|message| message.contains(&format!(
                    "Gnaf.{function} {request}\nis not definitionally equal"
                ))),
                "{request}: Lean rejects the planted {function}: {planted:#?}"
            );
        }
    }
}

/// §17.15: the universe is the grammar's expansion, fixed by its identity
/// before evaluation and complete by theorem, and no other carrier or
/// evidence is admitted.
#[allow(clippy::too_many_lines)]
fn gn_03() {
    let scalar = request("argmin-over-systems");
    let dispatch = |small, large| Selector::Dispatch {
        threshold: 3,
        small,
        large,
    };
    assert_eq!(
        gnaf::expand(&scalar.carrier),
        vec![
            Selector::Fixed { plan: 0 },
            Selector::Fixed { plan: 1 },
            dispatch(0, 0),
            dispatch(0, 1),
            dispatch(1, 0),
            dispatch(1, 1),
        ]
    );
    // The membership semantics the completeness theorem relates to the
    // expansion: every selector in and around the grammar is a member
    // exactly when it is well formed.
    let members = gnaf::expand(&scalar.carrier);
    for plan in 0..4 {
        let selector = Selector::Fixed { plan };
        assert_eq!(
            members.contains(&selector),
            gnaf::well_formed(&scalar.carrier, selector),
            "{selector:?}"
        );
        for threshold in [2, 3, 4] {
            for large in 0..4 {
                let selector = Selector::Dispatch {
                    threshold,
                    small: plan,
                    large,
                };
                assert_eq!(
                    members.contains(&selector),
                    gnaf::well_formed(&scalar.carrier, selector),
                    "{selector:?}"
                );
            }
        }
    }
    // §8.3 CompletenessProposition: the model proves membership equals
    // well-formedness for every grammar and selector, and neither depends
    // on evaluation or on anything outside the model.
    let complete = declaration("expandComplete");
    assert_eq!(complete["kind"], "theorem");
    let parameters: Vec<&str> = complete["parameters"]
        .as_array()
        .expect("parameters")
        .iter()
        .map(|parameter| parameter["name"].as_str().expect("a name"))
        .collect();
    assert_eq!(
        parameters,
        ["grammar", "selector"],
        "every grammar and selector"
    );
    let (function, arguments, right) = statement("expandComplete");
    assert_eq!(function, "selectorIn");
    assert_eq!(arguments[0], json!({"kind": "var", "name": "selector"}));
    assert_eq!(
        called(&arguments[1], "expand"),
        [json!({"kind": "var", "name": "grammar"})]
    );
    assert_eq!(
        called(&right, "wellFormed"),
        [
            json!({"kind": "var", "name": "grammar"}),
            json!({"kind": "var", "name": "selector"})
        ]
    );
    let evaluation = ["statusOn", "status", "statuses", "evaluate", "answer"];
    for definition in ["expand", "wellFormed"] {
        for name in dependencies(definition) {
            assert!(
                !name.contains('.') && !evaluation.contains(&name.as_str()),
                "`{definition}` depends only on the model, not on `{name}`"
            );
        }
    }
    let imports: Vec<String> = model_text()
        .lines()
        .filter_map(|line| line.strip_prefix("\\importmodule{"))
        .map(|line| line.trim_end_matches('}').to_owned())
        .collect();
    assert_eq!(
        imports,
        ["TargetSyntax", "TargetSemantics"],
        "the model imports the calculus and nothing that could optimize"
    );
    // Membership does not depend on any evaluation: a fuel at which
    // nothing returns leaves every member in the universe.
    let mut starved = scalar.clone();
    starved.machine.fuel = 1;
    assert_eq!(statuses(&starved).len(), 6);
    assert!(statuses(&starved)
        .iter()
        .all(|(_, _, status)| *status == Status::Unresolved));
    assert_eq!(answer(&starved), Answer::Incomplete);
    // The universe identity is fixed by the problem, machine, and carrier
    // alone: another fuel or another grammar is another universe, another
    // claim is not.
    let identity = universe_id(&scalar);
    assert_ne!(universe_id(&starved), identity);
    let mut regrammared = scalar.clone();
    let Carrier::Grammar { thresholds, .. } = &mut regrammared.carrier else {
        panic!("a grammar")
    };
    thresholds.push(5);
    assert_ne!(universe_id(&regrammared), identity);
    assert_eq!(
        universe_id(&Request {
            claim: ClaimClass::ArgminComplete,
            ..scalar.clone()
        }),
        identity
    );
    for (name, rejection) in [
        (
            "reject-internal-plan-universe",
            Rejection::InternalPlanUniverse,
        ),
        (
            "reject-optimizer-defined-universe",
            Rejection::OptimizerDefinedUniverse,
        ),
        ("reject-discovered-universe", Rejection::DiscoveredUniverse),
        ("reject-cached-universe", Rejection::CachedUniverse),
        (
            "reject-missing-completeness",
            Rejection::MissingCompleteness,
        ),
        (
            "reject-self-referential-completeness",
            Rejection::SelfReferentialCompleteness,
        ),
        (
            "reject-optimizer-completeness",
            Rejection::OptimizerCompleteness,
        ),
        ("reject-empty-domain", Rejection::EmptyDomain),
    ] {
        assert_eq!(fixtures::expected(name), rejected(rejection), "{name}");
    }
    // A discovered carrier that happens to list every member is still not a
    // universe fixed before discovery.
    let everything = Request {
        carrier: Carrier::Discovered {
            members: (0..6).collect(),
        },
        ..scalar.clone()
    };
    assert_eq!(answer(&everything), rejected(Rejection::DiscoveredUniverse));
    assert_eq!(
        answer(&Request {
            completeness: Completeness::CitesOptimizer,
            ..scalar
        }),
        rejected(Rejection::OptimizerCompleteness)
    );
    if support::lean_backed("GN-03") {
        let verified = support::verified_compiler();
        assert!(
            verified.outcome.units.contains_key("Gnaf"),
            "the completeness theorem is verified"
        );
    }
}

/// §17.15: the machine contract accounts every action, binds prepared
/// artifacts, declares its operand-size treatment, and bounds the request
/// by its capacity.
#[allow(clippy::too_many_lines)]
fn gn_04() {
    use ActionKind as K;
    for (name, rejection) in [
        (
            "reject-duplicate-action",
            Rejection::DuplicateAction {
                action: K::Dispatch,
            },
        ),
        (
            "reject-unaccounted-dispatch",
            Rejection::UnaccountedAction {
                action: K::Dispatch,
            },
        ),
        (
            "reject-zero-cost-dispatch",
            Rejection::HiddenCost {
                action: K::Dispatch,
            },
        ),
        (
            "reject-constant-cost-fallback",
            Rejection::HiddenCost {
                action: K::Fallback,
            },
        ),
        (
            "reject-free-observation",
            Rejection::HiddenCost {
                action: K::Observation,
            },
        ),
        (
            "reject-undeclared-execution",
            Rejection::HiddenCost {
                action: K::Execution,
            },
        ),
        (
            "reject-zero-cost-preprocessing",
            Rejection::HiddenCost {
                action: K::Preprocessing,
            },
        ),
        (
            "reject-step-charged-advice",
            Rejection::HiddenCost { action: K::Advice },
        ),
        (
            "reject-undeclared-retained-state",
            Rejection::HiddenCost {
                action: K::RetainedState,
            },
        ),
        (
            "reject-free-preprocessing-complete-boundary",
            Rejection::UnboundPreparation {
                action: K::Preprocessing,
            },
        ),
        (
            "reject-unbound-prepared-state",
            Rejection::UnboundPreparation {
                action: K::RetainedState,
            },
        ),
        (
            "reject-stray-prepared-artifact",
            Rejection::StrayPreparedArtifact { action: K::Advice },
        ),
        (
            "reject-unaccounted-plan-preparation",
            Rejection::UnaccountedAction {
                action: K::Preprocessing,
            },
        ),
        ("reject-unit-cost-operands", Rejection::UnitCostOperands),
        ("reject-beyond-capacity", Rejection::BeyondCapacity),
        (
            "reject-communication",
            Rejection::UnrealizableAction {
                action: K::Communication,
            },
        ),
        (
            "reject-randomness",
            Rejection::UnrealizableAction {
                action: K::Randomness,
            },
        ),
        (
            "reject-scheduling",
            Rejection::UnrealizableAction {
                action: K::Scheduling,
            },
        ),
    ] {
        assert_eq!(fixtures::expected(name), rejected(rejection), "{name}");
    }
    // Every performed kind, under every charge but steps, is hidden.
    for kind in ActionKind::ALL.into_iter().filter(|kind| kind.performed()) {
        for charge in [
            Charge::Constant { cost: 0 },
            Charge::Constant { cost: 5 },
            Charge::Free,
            Charge::Undeclared,
        ] {
            assert_eq!(
                gnaf::validate(&charged("argmin-over-systems", kind, charge)),
                Some(Rejection::HiddenCost { action: kind }),
                "{kind:?} charged {charge:?}"
            );
        }
    }
    // The capacity bounds the request: fuel, domain, universe, and every
    // declared charge.
    let base = request("argmin-over-systems");
    let mut capped = base.clone();
    capped.machine.capacity.domain = base.domain.len() as u64 - 1;
    assert_eq!(gnaf::validate(&capped), Some(Rejection::BeyondCapacity));
    let mut capped = base.clone();
    capped.machine.capacity.systems = 5;
    assert_eq!(gnaf::validate(&capped), Some(Rejection::BeyondCapacity));
    let mut capped = charged(
        "argmin-over-systems",
        K::Preprocessing,
        Charge::Constant { cost: 9 },
    );
    capped.machine.capacity.charge = 8;
    assert_eq!(gnaf::validate(&capped), Some(Rejection::BeyondCapacity));

    // A plan's declared preparation is charged on every invocation to
    // exactly the systems that can run it: the slicing plan's
    // preprocessing reaches the fixed slicing system and every dispatch
    // through it, and moves the argmin.
    let plain = statuses(&request("argmin-over-systems"));
    let prepared = statuses(&request("preparation-charged"));
    let domain = base.domain.len() as u64;
    for ((_, selector, before), (_, _, after)) in plain.iter().zip(&prepared) {
        let uses_slice = match selector {
            Selector::Fixed { plan } => *plan == 1,
            Selector::Dispatch { small, large, .. } => *small == 1 || *large == 1,
        };
        let charge = if uses_slice {
            domain * fixtures::PREPROCESSING_CHARGE
        } else {
            0
        };
        assert_eq!(
            admitted(*after),
            (admitted_steps(*before) + charge, admitted(*before).1),
            "{selector:?}"
        );
    }
    assert_ne!(
        fixtures::expected("preparation-charged"),
        fixtures::expected("argmin-over-systems"),
        "a plan's preparation changes which system is optimal"
    );
    // A prepared artifact bound in the common initial state is excluded
    // for every system; without the artifact the exclusion is refused.
    for name in ["preparation-common-state", "preparation-common-plan"] {
        assert_eq!(
            fixtures::expected(name),
            fixtures::expected("argmin-over-systems"),
            "{name}"
        );
        let mut unbound = request(name);
        let kind = unbound.machine.prepared[0].kind;
        unbound.machine.prepared.clear();
        assert_eq!(
            gnaf::validate(&unbound),
            Some(Rejection::UnboundPreparation { action: kind }),
            "{name}"
        );
        let mut elsewhere = request(name);
        elsewhere.machine.prepared[0].kind = K::Advice;
        assert_eq!(
            gnaf::validate(&elsewhere),
            Some(Rejection::UnboundPreparation { action: kind }),
            "{name}: an artifact of another action binds nothing"
        );
        let mut complete = request(name);
        complete.machine.boundary = Boundary::Complete;
        assert_eq!(
            gnaf::validate(&complete),
            Some(Rejection::UnboundPreparation { action: kind }),
            "{name}: nothing is excluded at the complete boundary"
        );
    }
    let mut stray = request("preparation-common-state");
    stray.machine.prepared.push(Prepared {
        kind: K::Preprocessing,
        artifact: Sha256Digest::of(b"an unaccounted artifact"),
    });
    assert_eq!(
        gnaf::validate(&stray),
        Some(Rejection::StrayPreparedArtifact {
            action: K::Preprocessing
        })
    );
    let unit = Request {
        machine: gnaf::Machine {
            operand_size: OperandSize::Unit,
            ..base.machine.clone()
        },
        ..base
    };
    assert_eq!(gnaf::validate(&unit), Some(Rejection::UnitCostOperands));
}

/// §17.15: claims and orders, ties, a frontier of incomparable systems,
/// and GNAF-VEC-02 posed as a request.
#[allow(clippy::too_many_lines)]
fn gn_05() {
    for (name, rejection) in [
        (
            "reject-scalar-claim-partial-order",
            Rejection::ScalarClaimOverPartialOrder,
        ),
        (
            "reject-vector-claim-total-order",
            Rejection::VectorClaimOverTotalOrder,
        ),
        ("reject-unsupported-claim", Rejection::UnsupportedClaim),
        ("reject-instance-optimal-claim", Rejection::UnsupportedClaim),
        (
            "reject-profile-defined-comparison-claim",
            Rejection::UnsupportedClaim,
        ),
        ("reject-restricted-universe-alias", Rejection::ClaimAlias),
        ("reject-calculus-program-scope", Rejection::UncoveredScope),
        ("reject-rust-program-scope", Rejection::UncoveredScope),
    ] {
        assert_eq!(fixtures::expected(name), rejected(rejection), "{name}");
    }
    let scalar_claims = [ClaimClass::GlobalOptimal, ClaimClass::ArgminComplete];
    let vector_claims = [ClaimClass::ParetoOptimal, ClaimClass::FrontierComplete];
    let claims = every_claim();
    assert_eq!(claims.len(), 31, "every §12.4 class");
    for claim in claims {
        for objective in [Objective::Scalar, Objective::Vector] {
            let posed = Request {
                claim: claim.clone(),
                objective,
                ..request("argmin-over-systems")
            };
            let expected = if scalar_claims.contains(&claim) {
                (objective == Objective::Vector).then_some(Rejection::ScalarClaimOverPartialOrder)
            } else if vector_claims.contains(&claim) {
                (objective == Objective::Scalar).then_some(Rejection::VectorClaimOverTotalOrder)
            } else if claim == ClaimClass::RestrictedUniverseOptimal {
                Some(Rejection::ClaimAlias)
            } else {
                Some(Rejection::UnsupportedClaim)
            };
            assert_eq!(gnaf::validate(&posed), expected, "{claim:?} {objective:?}");
            if expected.is_none() {
                for scope in [Scope::CalculusPrograms, Scope::RustPrograms] {
                    assert_eq!(
                        gnaf::validate(&Request {
                            scope,
                            ..posed.clone()
                        }),
                        Some(Rejection::UncoveredScope)
                    );
                }
            }
        }
    }
    // The frontier holds systems neither of which dominates the other, and
    // no weighting turns them into a total order: the scalar answer keeps
    // only the fastest.
    assert_eq!(
        fixtures::expected("frontier-pareto-optimal"),
        fixtures::expected("frontier-incomparable")
    );
    let entries = statuses(&request("frontier-incomparable"));
    let Answer::Frontier { members } = fixtures::expected("frontier-incomparable") else {
        panic!("a frontier")
    };
    assert!(members.len() >= 2, "{members:?}");
    let cost = |member: u64| {
        let (steps, size) = admitted(entries[index(member)].2);
        vec![steps, size]
    };
    for left in &members {
        for right in &members {
            assert!(
                !gnaf::dominates(&cost(*left), &cost(*right)),
                "{left} and {right} are incomparable"
            );
        }
    }
    let fastest = members
        .iter()
        .map(|member| cost(*member)[0])
        .min()
        .expect("members");
    let Answer::Argmin { steps, .. } = fixtures::expected("argmin-over-systems") else {
        panic!("an argmin")
    };
    assert_eq!(steps, fastest);
    // GNAF-REJ-21: equal-cost identities are all kept. The duplicated plan
    // makes distinct systems of equal cost, and both answers list each.
    let tie = statuses(&request("argmin-tie"));
    let Answer::Argmin { members, steps } = fixtures::expected("argmin-tie") else {
        panic!("an argmin")
    };
    assert!(members.len() >= 2, "a tie: {members:?}");
    let attaining: Vec<u64> = tie
        .iter()
        .filter(|(_, _, status)| {
            matches!(status, Status::Admitted { steps: cost, .. } if *cost == steps)
        })
        .map(|(index, _, _)| *index)
        .collect();
    assert_eq!(members, attaining);
    let Answer::Frontier { members } = fixtures::expected("frontier-tie") else {
        panic!("a frontier")
    };
    let costs: Vec<(u64, u64)> = members
        .iter()
        .map(|member| admitted(tie[index(*member)].2))
        .collect();
    assert!(
        costs
            .iter()
            .enumerate()
            .any(|(position, cost)| costs[position + 1..].contains(cost)),
        "the frontier keeps two members of equal cost: {members:?} {costs:?}"
    );
    // GNAF-VEC-02 posed over the calculus: four systems, a frontier of
    // exactly three, the fourth dominated; an omitted member is not
    // certified, the frontier in another order is, and the componentwise
    // minima are attained by no system (GNAF-REJ-29, GNAF-REJ-14).
    let envelope = request(fixtures::PARETO_ENVELOPE);
    let entries = statuses(&envelope);
    assert_eq!(entries.len(), 4);
    let Answer::Frontier { members } = fixtures::expected(fixtures::PARETO_ENVELOPE) else {
        panic!("a frontier")
    };
    assert_eq!(members.len(), 3, "{members:?}");
    let rows: Vec<(u64, Vec<u64>)> = entries
        .iter()
        .map(|(position, _, status)| {
            let (steps, size) = admitted(*status);
            (*position, vec![steps, size])
        })
        .collect();
    let dominated: Vec<u64> = rows
        .iter()
        .filter(|(position, _)| !members.contains(position))
        .map(|(position, _)| *position)
        .collect();
    assert_eq!(dominated.len(), 1);
    assert!(rows
        .iter()
        .any(|(_, cost)| gnaf::dominates(cost, &rows[index(dominated[0])].1)));
    for omitted in 0..members.len() {
        let mut claimed = members.clone();
        claimed.remove(omitted);
        assert!(
            !gnaf::certifies(&envelope, &Answer::Frontier { members: claimed })
                .expect("within capacity")
        );
    }
    let mut reordered = members.clone();
    reordered.reverse();
    assert!(
        gnaf::certifies(&envelope, &Answer::Frontier { members: reordered })
            .expect("within capacity")
    );
    assert!(!gnaf::minima_attained(&envelope).expect("within capacity"));
    assert!(!gnaf::attains(&rows, &gnaf::componentwise_minimum(&rows)));
}

/// §17.15: complete systems, not internal plans or envelopes, costed by
/// the code they can run.
#[allow(clippy::too_many_lines)]
fn gn_06() {
    let whole = request("argmin-over-systems");
    let entries = statuses(&whole);
    let Answer::Argmin { members, steps } = fixtures::expected("argmin-over-systems") else {
        panic!("an argmin")
    };
    assert_eq!(members.len(), 1);
    let optimum = entries[index(members[0])].1;
    assert!(
        matches!(optimum, Selector::Dispatch { .. }),
        "the optimum is a dispatching system: {optimum:?}"
    );
    let best_plan = entries
        .iter()
        .filter(|(_, selector, _)| matches!(selector, Selector::Fixed { .. }))
        .map(|(_, _, status)| admitted_steps(*status))
        .min()
        .expect("fixed systems");
    assert!(
        best_plan > steps,
        "the best single plan ({best_plan}) is not the optimum ({steps})"
    );
    // Each restricted domain has its own best plan; together they form
    // the per-input envelope, which no system attains.
    let short = fixtures::expected("internal-best-on-short-lists");
    let long = fixtures::expected("internal-best-on-long-lists");
    let (
        Answer::Argmin {
            members: short_best,
            steps: short_steps,
        },
        Answer::Argmin {
            members: long_best,
            steps: long_steps,
        },
    ) = (short, long)
    else {
        panic!("both restricted requests have an argmin")
    };
    assert_eq!((short_best, long_best), (vec![0], vec![1]));
    let envelope = short_steps + long_steps;
    assert!(envelope < steps, "{envelope} < {steps}");
    assert!(entries
        .iter()
        .all(|(_, _, status)| admitted_steps(*status) > envelope));
    // Selection is charged: on every argument the dispatching system costs
    // strictly more than the plan it selects, by the work of observing the
    // argument and choosing.
    let Selector::Dispatch {
        threshold,
        small,
        large,
    } = optimum
    else {
        panic!("a dispatching system")
    };
    let realized = |selector| gnaf::realize(&whole.carrier, selector).expect("a grammar");
    for argument in &whole.domain {
        let Value::List { items } = argument else {
            panic!("a list argument")
        };
        let chosen = if (items.len() as u64) < threshold {
            small
        } else {
            large
        };
        assert!(
            run_steps(&whole, &realized(optimum), argument)
                > run_steps(
                    &whole,
                    &realized(Selector::Fixed { plan: chosen }),
                    argument
                ),
            "selection is charged on {argument:?}"
        );
    }
    // A system's size is the code it can run: its entry and the plans it
    // reaches, never a plan it cannot call.
    for (_, selector, status) in &entries {
        let program = realized(*selector);
        let reached: Vec<u64> = match selector {
            Selector::Fixed { plan } => vec![0, plan + 1],
            Selector::Dispatch { small, large, .. } if small == large => vec![0, small + 1],
            Selector::Dispatch { small, large, .. } => {
                let mut out = vec![0, small + 1, large + 1];
                out.sort_unstable();
                out
            }
        };
        let mut found = gnaf::reachable(&program);
        found.sort_unstable();
        assert_eq!(found, reached, "{selector:?}");
        let size: u64 = reached
            .iter()
            .map(|reached| {
                let single = Program {
                    functions: vec![program.functions[index(*reached)].clone()],
                    ..program.clone()
                };
                gnaf::program_size(&single)
            })
            .sum();
        assert_eq!(admitted(*status).1, size, "{selector:?}");
    }
    let sizes: BTreeSet<u64> = entries
        .iter()
        .filter(|(_, selector, _)| matches!(selector, Selector::Fixed { .. }))
        .map(|(_, _, status)| admitted(*status).1)
        .collect();
    assert_eq!(
        sizes.len(),
        2,
        "fixed systems of different plans differ in size"
    );
    // An inadmissible plan is excluded, an unresolved one is never
    // removed, and nothing admitted is infeasible.
    let excluded = statuses(&request("inadmissible-plan-excluded"));
    assert_eq!(excluded[1].2, Status::Inadmissible);
    assert_eq!(
        fixtures::expected("inadmissible-plan-excluded"),
        Answer::Argmin {
            members: vec![0],
            steps: admitted_steps(excluded[0].2)
        }
    );
    let unknown = statuses(&request("unknown-cost-incomplete"));
    assert_eq!(unknown[2].2, Status::Unresolved);
    assert!(matches!(unknown[0].2, Status::Admitted { .. }));
    assert_eq!(
        fixtures::expected("unknown-cost-incomplete"),
        Answer::Incomplete
    );
    assert_eq!(
        fixtures::expected("infeasible-universe"),
        Answer::Infeasible
    );
}

/// §17.15, §27.4: the authority's vectors are theorems over the order the
/// answers are computed by, stated with the authority's own numbers; the
/// authority is vendored, pinned, and some-true.
#[allow(clippy::too_many_lines)]
fn gn_07() {
    let nat_rows = |term: &Json| decode_list(term, |row| decode_pair(row, decode_nat, decode_nat));
    let vector_rows =
        |term: &Json| decode_list(term, |row| decode_pair(row, decode_nat, decode_nats));
    let operations = |term: &Json| {
        decode_list(term, |row| {
            let (identity, rest) = decode_pair(row, decode_nat, Clone::clone);
            let (from, rest) = decode_pair(&rest, decode_nat, Clone::clone);
            let (to, cost) = decode_pair(&rest, decode_nat, decode_nat);
            (identity, from, to, cost)
        })
    };
    // GNAF-VEC-01 (§16.1): rAB a0 -> b0 at 2, rBC b0 -> c0 at 3, rAC6
    // a0 -> c0 at 6, and in S1 rAC4 a0 -> c0 at 4.
    let s0: Vec<(u64, u64, u64, u64)> = vec![(0, 0, 1, 2), (1, 1, 2, 3), (2, 0, 2, 6)];
    let mut s1 = s0.clone();
    s1.push((3, 0, 2, 4));
    let compositions = |term: &Json, expected: &[(u64, u64, u64, u64)]| {
        let arguments = called(term, "compositions");
        assert_eq!(
            operations(&arguments[1]),
            expected,
            "the vector's operations"
        );
        assert_eq!(decode_nat(&arguments[2]), 2, "to c0");
        assert_eq!(decode_nat(&arguments[0]), expected.len() as u64 + 1);
    };
    let (function, arguments, right) = statement("vec01Optimum");
    assert_eq!(function, "argmin");
    compositions(&arguments[0], &s0);
    assert_eq!(decode_found(&right, decode_nats), (vec![vec![0, 1]], 5));
    let (function, arguments, right) = statement("vec01Extension");
    assert_eq!(function, "argmin");
    compositions(&arguments[0], &s1);
    assert_eq!(decode_found(&right, decode_nats), (vec![vec![3]], 4));
    for (name, operations, holds) in [
        ("vec01CertificateHolds", &s0, true),
        ("vec01CertificateInvalidated", &s1, false),
    ] {
        let (function, arguments, right) = statement(name);
        assert_eq!(function, "certifiesArgmin");
        compositions(&arguments[1], operations);
        assert_eq!(decode_list(&arguments[2], decode_nats), vec![vec![0, 1]]);
        assert_eq!(decode_nat(&arguments[3]), 5);
        assert_eq!(decode_bool(&right), holds, "{name}");
    }
    // GNAF-VEC-02 (§16.2) and its rejections GNAF-REJ-29 and GNAF-REJ-14.
    let vec02 = vec![
        (0, vec![1, 3]),
        (1, vec![2, 2]),
        (2, vec![3, 1]),
        (3, vec![3, 3]),
    ];
    let (function, arguments, right) = statement("vec02Frontier");
    assert_eq!(function, "frontier");
    assert_eq!(vector_rows(&arguments[0]), vec02);
    assert_eq!(decode_nats(&right), vec![0, 1, 2]);
    let (function, arguments, right) = statement("rej29FrontierOmission");
    assert_eq!(function, "certifiesFrontier");
    assert_eq!(vector_rows(&arguments[1]), vec02);
    assert_eq!(decode_nats(&arguments[2]), vec![0, 1]);
    assert!(!decode_bool(&right));
    let (function, arguments, right) = statement("rej14ComponentwiseMinima");
    assert_eq!(function, "componentwiseMinimum");
    assert_eq!(vector_rows(&arguments[0]), vec02);
    assert_eq!(decode_nats(&right), vec![1, 1]);
    let (function, arguments, right) = statement("rej14MinimaUnattained");
    assert_eq!(function, "attains");
    assert_eq!(vector_rows(&arguments[0]), vec02);
    assert_eq!(
        vector_rows(&called(&arguments[1], "componentwiseMinimum")[0]),
        vec02
    );
    assert!(!decode_bool(&right));
    // GNAF-VEC-04 (§16.4): a normalizes to b under a -> b alone, c is the
    // global minimum at 0, and certifying the normal form b is refused.
    let vec04 = vec![(0, 2), (1, 1), (2, 0)];
    let normalizes_a = |arguments: &[Json]| {
        assert_eq!(
            decode_list(&arguments[1], |rule| decode_pair(
                rule, decode_nat, decode_nat
            )),
            vec![(0, 1)],
            "the only rule is a -> b"
        );
        assert_eq!(decode_nat(&arguments[2]), 0, "from a");
    };
    let (function, arguments, right) = statement("vec04NormalFormIsB");
    assert_eq!(function, "normalize");
    normalizes_a(&arguments);
    assert_eq!(decode_nat(&right), 1);
    let (function, arguments, right) = statement("vec04GlobalMinimumIsC");
    assert_eq!(function, "argmin");
    assert_eq!(nat_rows(&arguments[0]), vec04);
    assert_eq!(decode_found(&right, decode_nat), (vec![2], 0));
    let (function, arguments, right) = statement("vec04NormalFormRefused");
    assert_eq!(function, "certifiesArgmin");
    assert_eq!(nat_rows(&arguments[1]), vec04);
    let claimed = decode_list(&arguments[2], Clone::clone);
    assert_eq!(claimed.len(), 1);
    normalizes_a(called(&claimed[0], "normalize"));
    assert_eq!(decode_nat(&arguments[3]), 1);
    assert!(!decode_bool(&right));
    // GNAF-VEC-17 (§16.17): the pointwise envelope is 0 for each hidden
    // bit, every uniform system's worst case is 10, the fair randomized
    // system expects 5, and the pointwise value certifies no system.
    let hidden = declaration("hiddenBitCost");
    assert_eq!(
        hidden["body"]["then_value"],
        json!({"kind": "nat", "value": "0"})
    );
    assert_eq!(
        hidden["body"]["else_value"],
        json!({"kind": "nat", "value": "10"})
    );
    for (name, bit) in [("vec17EnvelopeAtZero", 0), ("vec17EnvelopeAtOne", 1)] {
        let (function, arguments, right) = statement(name);
        assert_eq!(function, "argmin");
        assert_eq!(
            decode_nat(&called(&arguments[0], "environmentRows")[0]),
            bit
        );
        assert_eq!(decode_found(&right, decode_nat), (vec![bit], 0));
    }
    let (function, arguments, right) = statement("vec17UniformWorstCase");
    assert_eq!(function, "argmin");
    assert!(called(&arguments[0], "uniformRows").is_empty());
    assert_eq!(decode_found(&right, decode_nat), (vec![0, 1], 10));
    for (name, bit) in [
        ("vec17FairExpectationAtZero", 0),
        ("vec17FairExpectationAtOne", 1),
    ] {
        let (function, arguments, right) = statement(name);
        assert_eq!(function, "fairExpectation");
        assert_eq!(decode_nat(&arguments[0]), bit);
        assert_eq!(decode_nat(&right), 5);
    }
    let (function, arguments, right) = statement("vec17PointwiseValueRefused");
    assert_eq!(function, "certifiesArgmin");
    assert!(called(&arguments[1], "uniformRows").is_empty());
    assert_eq!(decode_nats(&arguments[2]), vec![0]);
    assert_eq!(decode_nat(&arguments[3]), 0);
    assert!(!decode_bool(&right));
    // The vectors and the answers share one order: `evaluate` computes its
    // argmin and frontier by the very definitions the vectors state.
    let evaluate = dependencies("evaluate");
    for order in ["argmin", "frontier"] {
        assert!(evaluate.contains(order), "evaluate answers by `{order}`");
    }

    // The authority: vendored, its bytes' SHA-256 recomputed, pinned by
    // revision, and some-true.
    let model = repo_model::Model::load_from_repo_root().expect("model");
    let (identity, identifier, revision, sha256) = fixtures::AUTHORITY;
    let authority = model
        .authorities
        .authority
        .iter()
        .find(|authority| authority.id == identity)
        .expect("the GNAF authority row");
    assert_eq!(authority.revision.as_deref(), Some(revision));
    assert!(authority
        .immutable_url
        .as_deref()
        .expect("immutable URL")
        .contains(revision));
    assert!(authority.citation.contains(revision));
    assert_eq!(authority.acquired_sha256.as_deref(), Some(sha256));
    assert_eq!(authority.checksum, format!("sha256:{sha256}"));
    assert_eq!(
        authority.vendored.as_deref(),
        Some(fixtures::AUTHORITY_PATH)
    );
    let vendored = std::fs::read(repo_root().join(fixtures::AUTHORITY_PATH).as_std_path())
        .expect("the vendored authority");
    assert_eq!(Sha256Digest::of(&vendored).to_hex(), sha256);
    assert_eq!(authority.canonical_identifier.as_deref(), Some(identifier));
    assert_eq!(authority.source_role.as_deref(), Some("normative"));
    assert_eq!(authority.license.as_deref(), Some("Apache-2.0 OR MIT"));
    assert_eq!(authority.redistribution.as_deref(), Some("redistributable"));
    let claim = model
        .ledger
        .claim
        .iter()
        .find(|claim| claim.authority.as_deref() == Some(identity))
        .expect("a ledger claim");
    assert_eq!(claim.level, repo_model::Level::SomeTrue);
    assert!(
        !model
            .authorities
            .authority
            .iter()
            .any(|authority| authority.statement.contains("UOR-NAF")),
        "UOR-NAF is informative and cited as no authority"
    );
    if support::lean_backed("GN-07") {
        let verified = support::verified_compiler();
        assert!(
            verified.outcome.units.contains_key("Gnaf"),
            "the model is verified"
        );
        let planted = planted_vector();
        assert!(
            planted
                .iter()
                .any(|message| message.contains(
                    "frontier Nat [(0, [1, 3]), (1, [2, 2]), (2, [3, 1]), (3, [3, 3])]\nis not definitionally equal to the right-hand side\n  [0, 1]"
                )),
            "Lean rejects a frontier omitting rC: {planted:#?}"
        );
    }
}

/// UOR-GNAF §20: the dependency manifest names the draft's revision, every
/// profile with its role, every restriction of the universe, and the claim
/// classes the model actually supports.
fn gn_08() {
    let bytes = std::fs::read(repo_root().join(fixtures::MANIFEST_PATH).as_std_path())
        .expect("the manifest");
    assert_eq!(
        bytes,
        gnaf::to_file_bytes(&fixtures::manifest()),
        "the committed manifest equals its generator"
    );
    let manifest: Manifest = serde_json::from_slice(&bytes).expect("a closed manifest");
    assert_eq!(manifest.spec, gnaf::MANIFEST_SPEC);
    let (identity, identifier, revision, sha256) = fixtures::AUTHORITY;
    let model = repo_model::Model::load_from_repo_root().expect("model");
    let row = model
        .authorities
        .authority
        .iter()
        .find(|authority| authority.id == identity)
        .expect("the GNAF authority row");
    assert_eq!(manifest.authority.id, identity);
    assert_eq!(manifest.authority.identifier, identifier);
    assert_eq!(
        Some(manifest.authority.identifier.as_str()),
        row.canonical_identifier.as_deref()
    );
    assert_eq!(manifest.authority.revision, revision);
    assert_eq!(
        Some(manifest.authority.revision.as_str()),
        row.revision.as_deref()
    );
    assert_eq!(manifest.authority.sha256, sha256);
    assert_eq!(
        Some(manifest.authority.sha256.as_str()),
        row.acquired_sha256.as_deref()
    );
    assert_eq!(manifest.authority.vendored, fixtures::AUTHORITY_PATH);
    // §20: every kind, operation, machine, cost, proof, interchange, and
    // address profile, each with its role.
    let names: Vec<&str> = manifest
        .profiles
        .iter()
        .map(|profile| profile.name.as_str())
        .collect();
    for required in [
        "kind",
        "operation",
        "machine",
        "cost",
        "proof",
        "interchange",
        "address",
    ] {
        assert!(names.contains(&required), "the {required} profile is named");
    }
    for profile in &manifest.profiles {
        assert!(
            [
                "semantic",
                "verification-only",
                "realization-only",
                "measured"
            ]
            .contains(&profile.role.as_str()),
            "{}: role {}",
            profile.name,
            profile.role
        );
        assert!(!profile.statement.is_empty(), "{}", profile.name);
        let exists = repo_root().join(&profile.source).exists()
            || model
                .authorities
                .authority
                .iter()
                .any(|authority| authority.id == profile.source);
        assert!(
            exists,
            "{}: the source {} exists",
            profile.name, profile.source
        );
    }
    assert!(
        manifest.measured.is_empty(),
        "nothing measured is a dependency of any answer"
    );
    assert!(manifest.restrictions.len() >= 4);
    let capacity = format!(
        "fuel {}, {} arguments, {} systems",
        HOST_CAPACITY.fuel, HOST_CAPACITY.domain, HOST_CAPACITY.systems
    );
    assert!(
        manifest
            .restrictions
            .iter()
            .any(|line| line.contains(&capacity)),
        "the capacity restriction states the host's: {capacity}"
    );
    assert!(!manifest.non_claims.is_empty());
    // The strongest claims are exactly the classes the model answers, each
    // over the grammar universe and under its own order.
    let mut accepted: Vec<(String, String)> = Vec::new();
    for claim in every_claim() {
        for objective in [Objective::Scalar, Objective::Vector] {
            let posed = Request {
                claim: claim.clone(),
                objective,
                ..request("argmin-over-systems")
            };
            if gnaf::validate(&posed).is_none() {
                let class = serde_json::to_value(&posed.claim).expect("json")["class"]
                    .as_str()
                    .expect("a class")
                    .to_owned();
                let objective = serde_json::to_value(objective)
                    .expect("json")
                    .as_str()
                    .expect("an objective")
                    .to_owned();
                accepted.push((class, objective));
            }
        }
    }
    let stated: Vec<(String, String)> = manifest
        .strongest_claims
        .iter()
        .map(|claim| {
            assert_eq!(claim.scope, "grammar_universe", "{}", claim.class);
            (claim.class.clone(), claim.objective.clone())
        })
        .collect();
    assert_eq!(stated, accepted);
}
