//! The hand-constructed GNAF requests of SPEC.md §17.15, the dependency
//! manifest UOR-GNAF §20 requires, and the generated `compiler` project
//! files that state their answers to Lean.
//!
//! Every request is written here, by hand; its expected answer is computed
//! by the host transcription `lexlean::gnaf::answer`. The committed
//! `compiler/gnaf/<name>.json` files, `compiler/gnaf.manifest.json`, the
//! `compiler/src/GnafFixtures` module, and the `Gnaf` model it imports
//! ([`crate::gnaf_model`]) are exactly what [`files`] renders, which
//! `cargo xtask check-calculus` enforces. In the module, each request is a
//! `Gnaf.Request` definition with theorems that the kernel reduces its
//! universe, its systems' statuses, and its answer to what the host
//! computes, so the model, not the transcription, is the oracle.

use std::collections::BTreeMap;

use lexlean::calculus::{term, Arm, Expr, Function, Prim, Program, Shape, Ty, Value, PROGRAM_SPEC};
use lexlean::gnaf::{
    self, Action, ActionKind, Answer, Boundary, Carrier, Charge, ClaimClass, Completeness, Fixture,
    Machine, Objective, OperandSize, Plan, Prepared, Request, Scope, FIXTURE_SPEC, HOST_CAPACITY,
    MANIFEST_SPEC, MODEL, REQUEST_SPEC,
};
use lexlean::Sha256Digest;
use serde_json::{json, Value as Json};

fn nat_t() -> Ty {
    Ty::Nat
}
fn list_t() -> Ty {
    Ty::List {
        element: Box::new(nat_t()),
    }
}
fn option_t() -> Ty {
    Ty::Option {
        value: Box::new(nat_t()),
    }
}
fn natv(number: u64) -> Value {
    Value::Nat {
        value: number.to_string(),
    }
}
fn listv(items: &[u64]) -> Value {
    Value::List {
        items: items.iter().copied().map(natv).collect(),
    }
}
fn var(name: u64) -> Expr {
    Expr::Var { name }
}
fn prim(operation: Prim, operands: Vec<Expr>) -> Expr {
    Expr::Prim {
        operation,
        operands,
    }
}
fn build(shape: Shape, operands: Vec<Expr>) -> Expr {
    Expr::Build {
        shape,
        ty: option_t(),
        operands,
    }
}
fn arm(shape: Shape, binders: Vec<u64>, body: Expr) -> Arm {
    Arm {
        shape,
        binders,
        body,
    }
}
fn on_list(scrutinee: Expr, empty: Expr, binders: Vec<u64>, nonempty: Expr) -> Expr {
    Expr::Match {
        ty: option_t(),
        scrutinee: Box::new(scrutinee),
        arms: vec![
            arm(Shape::Nil, Vec::new(), empty),
            arm(Shape::Cons, binders, nonempty),
        ],
    }
}
fn function(body: Expr) -> Function {
    Function {
        parameters: vec![0],
        types: vec![list_t()],
        result: option_t(),
        body,
    }
}
/// A plan that needs no preparation.
fn plan(function: Function) -> Plan {
    Plan {
        function,
        prepares: Vec::new(),
    }
}
/// A plan that needs `kind` prepared before every invocation.
fn preparing(function: Function, kind: ActionKind) -> Plan {
    Plan {
        function,
        prepares: vec![kind],
    }
}

/// The last element by structural recursion: cheap on short lists, one
/// round of matching per element on long ones.
fn last_by_recursion(at: u64) -> Function {
    function(on_list(
        var(0),
        build(Shape::None, Vec::new()),
        vec![1, 2],
        on_list(
            var(2),
            build(Shape::Some, vec![var(1)]),
            vec![3, 4],
            Expr::Call {
                function: at,
                operands: vec![var(2)],
            },
        ),
    ))
}

/// The last element by recursion two elements at a time: larger code, and
/// fewer calls than one element at a time.
fn last_by_pairs(at: u64) -> Function {
    function(on_list(
        var(0),
        build(Shape::None, Vec::new()),
        vec![1, 2],
        on_list(
            var(2),
            build(Shape::Some, vec![var(1)]),
            vec![3, 4],
            on_list(
                var(4),
                build(Shape::Some, vec![var(3)]),
                vec![5, 6],
                Expr::Call {
                    function: at,
                    operands: vec![var(4)],
                },
            ),
        ),
    ))
}

/// The last element by recursion that rebinds its argument every round:
/// larger and slower than plain recursion, so it is dominated.
fn last_by_rebinding(at: u64) -> Function {
    function(Expr::Let {
        name: 7,
        ty: list_t(),
        bound: Box::new(var(0)),
        body: Box::new(on_list(
            var(7),
            build(Shape::None, Vec::new()),
            vec![1, 2],
            on_list(
                var(2),
                build(Shape::Some, vec![var(1)]),
                vec![3, 4],
                Expr::Call {
                    function: at,
                    operands: vec![var(2)],
                },
            ),
        )),
    })
}

/// The last element by slicing at the end: a fixed cost on every list,
/// above recursion's on short lists and far below it on long ones.
fn last_by_slice() -> Function {
    let one = || Expr::Value {
        ty: nat_t(),
        value: natv(1),
    };
    let length = || prim(Prim::Length, vec![var(0)]);
    function(Expr::Cond {
        condition: Box::new(prim(
            Prim::NatLt,
            vec![
                Expr::Value {
                    ty: nat_t(),
                    value: natv(0),
                },
                length(),
            ],
        )),
        then_branch: Box::new(Expr::Match {
            ty: option_t(),
            scrutinee: Box::new(prim(
                Prim::Slice,
                vec![var(0), prim(Prim::NatSub, vec![length(), one()]), one()],
            )),
            arms: vec![
                arm(Shape::None, Vec::new(), build(Shape::None, Vec::new())),
                arm(
                    Shape::Some,
                    vec![1],
                    on_list(
                        var(1),
                        build(Shape::None, Vec::new()),
                        vec![2, 3],
                        build(Shape::Some, vec![var(2)]),
                    ),
                ),
            ],
        }),
        else_branch: Box::new(build(Shape::None, Vec::new())),
    })
}

/// The first element: a wrong plan, inadmissible on every list of two or
/// more elements.
fn first_element() -> Function {
    function(on_list(
        var(0),
        build(Shape::None, Vec::new()),
        vec![1, 2],
        build(Shape::Some, vec![var(1)]),
    ))
}

/// A plan that never returns, so its cost is unknown at any fuel.
fn diverging(at: u64) -> Function {
    function(Expr::Call {
        function: at,
        operands: vec![var(0)],
    })
}

/// The problem: the last element of a list of naturals.
fn reference() -> Program {
    Program {
        spec: PROGRAM_SPEC.to_owned(),
        adts: Vec::new(),
        functions: vec![last_by_recursion(0)],
    }
}

fn short_lists() -> Vec<Value> {
    vec![listv(&[7]), listv(&[3]), listv(&[9])]
}

fn long_lists() -> Vec<Value> {
    vec![listv(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])]
}

/// Short and long lists, on which neither plan alone is best.
fn mixed_domain() -> Vec<Value> {
    let mut out = short_lists();
    out.extend(long_lists());
    out
}

fn accounted(kind: ActionKind, charge: Charge) -> Action {
    Action { kind, charge }
}

/// A prepared artifact's identity: the SHA-256 of the bytes it names.
fn artifact(bytes: &str) -> Sha256Digest {
    Sha256Digest::of(bytes.as_bytes())
}

/// The machine every valid request starts from: the host's capacity,
/// weighted operands, the four actions systems perform, each charged by
/// steps, compared on the complete boundary with nothing prepared.
fn machine() -> Machine {
    Machine {
        fuel: 256,
        capacity: HOST_CAPACITY,
        operand_size: OperandSize::Weighted,
        actions: [
            ActionKind::Observation,
            ActionKind::Dispatch,
            ActionKind::Fallback,
            ActionKind::Execution,
        ]
        .into_iter()
        .map(|kind| accounted(kind, Charge::Steps))
        .collect(),
        boundary: Boundary::Complete,
        prepared: Vec::new(),
    }
}

/// A grammar over the plans at indices 1.. of every realization.
fn grammar(plans: Vec<Plan>, thresholds: Vec<u64>) -> Carrier {
    Carrier::Grammar {
        argument: list_t(),
        result: option_t(),
        plans,
        thresholds,
    }
}

fn two_plans() -> Carrier {
    grammar(
        vec![plan(last_by_recursion(1)), plan(last_by_slice())],
        vec![3],
    )
}

/// The two plans with the slicing plan written twice: two distinct plans
/// of equal cost, so every optimum has a tie.
fn duplicated_plan() -> Carrier {
    grammar(
        vec![
            plan(last_by_recursion(1)),
            plan(last_by_slice()),
            plan(last_by_slice()),
        ],
        vec![3],
    )
}

/// GNAF-VEC-02 posed over the calculus: four fixed systems on long lists,
/// one fast and large, one between, one slow and small, and one slower and
/// larger than the small one.
fn pareto_envelope() -> Carrier {
    grammar(
        vec![
            plan(last_by_slice()),
            plan(last_by_pairs(2)),
            plan(last_by_recursion(3)),
            plan(last_by_rebinding(4)),
        ],
        Vec::new(),
    )
}

/// A request as written, its universe identity left to [`identified`].
fn request(
    carrier: Carrier,
    domain: Vec<Value>,
    objective: Objective,
    claim: ClaimClass,
) -> Request {
    Request {
        spec: REQUEST_SPEC.to_owned(),
        reference: reference(),
        domain,
        machine: machine(),
        carrier,
        completeness: Completeness::GrammarEquality,
        universe: Sha256Digest([0; 32]),
        objective,
        claim,
        scope: Scope::GrammarUniverse,
    }
}

/// The request with the universe identity its components determine, as
/// every committed request states it.
fn identified(mut request: Request) -> Request {
    request.universe = gnaf::universe_id(&request).expect("a canonical request");
    request
}

fn scalar() -> Request {
    request(
        two_plans(),
        mixed_domain(),
        Objective::Scalar,
        ClaimClass::GlobalOptimal,
    )
}

fn vector() -> Request {
    request(
        two_plans(),
        mixed_domain(),
        Objective::Vector,
        ClaimClass::FrontierComplete,
    )
}

fn with_machine(edit: impl FnOnce(&mut Machine)) -> Request {
    let mut out = scalar();
    edit(&mut out.machine);
    out
}

fn set_charge(machine: &mut Machine, kind: ActionKind, charge: Charge) {
    machine.actions.retain(|action| action.kind != kind);
    machine.actions.push(accounted(kind, charge));
}

/// The two plans with the slicing plan needing `kind` prepared.
fn slice_prepares(kind: ActionKind) -> Carrier {
    grammar(
        vec![plan(last_by_recursion(1)), preparing(last_by_slice(), kind)],
        vec![3],
    )
}

/// The name of the request posing GNAF-VEC-02 over the calculus.
pub const PARETO_ENVELOPE: &str = "vec02-pareto-envelope";

/// The per-invocation preprocessing charge of the slicing plan in
/// `preparation-charged`.
pub const PREPROCESSING_CHARGE: u64 = 40;

/// Every request, by name, before its answer is computed.
#[allow(clippy::too_many_lines)]
fn requests() -> Vec<(&'static str, Request)> {
    let mut out = vec![
        ("argmin-over-systems", scalar()),
        (
            "argmin-complete",
            Request {
                claim: ClaimClass::ArgminComplete,
                ..scalar()
            },
        ),
        // GNAF-REJ-21: an equal-cost identity is never dropped.
        (
            "argmin-tie",
            Request {
                carrier: duplicated_plan(),
                claim: ClaimClass::ArgminComplete,
                ..scalar()
            },
        ),
        ("frontier-incomparable", vector()),
        (
            "frontier-pareto-optimal",
            Request {
                claim: ClaimClass::ParetoOptimal,
                ..vector()
            },
        ),
        (
            "frontier-tie",
            Request {
                carrier: duplicated_plan(),
                ..vector()
            },
        ),
        (
            PARETO_ENVELOPE,
            request(
                pareto_envelope(),
                long_lists(),
                Objective::Vector,
                ClaimClass::FrontierComplete,
            ),
        ),
        (
            "internal-best-on-short-lists",
            Request {
                domain: short_lists(),
                ..scalar()
            },
        ),
        (
            "internal-best-on-long-lists",
            Request {
                domain: long_lists(),
                ..scalar()
            },
        ),
        (
            "inadmissible-plan-excluded",
            Request {
                carrier: grammar(
                    vec![plan(last_by_recursion(1)), plan(first_element())],
                    vec![2],
                ),
                ..scalar()
            },
        ),
        (
            "infeasible-universe",
            Request {
                carrier: grammar(vec![plan(first_element())], Vec::new()),
                ..scalar()
            },
        ),
        (
            "unknown-cost-incomplete",
            Request {
                carrier: grammar(
                    vec![
                        plan(last_by_recursion(1)),
                        plan(last_by_slice()),
                        plan(diverging(3)),
                    ],
                    Vec::new(),
                ),
                ..scalar()
            },
        ),
        // The slicing plan's preprocessing is charged to exactly the
        // systems that can run it, which moves the argmin.
        (
            "preparation-charged",
            Request {
                carrier: slice_prepares(ActionKind::Preprocessing),
                machine: {
                    let mut machine = machine();
                    set_charge(
                        &mut machine,
                        ActionKind::Preprocessing,
                        Charge::Constant {
                            cost: PREPROCESSING_CHARGE,
                        },
                    );
                    machine
                },
                ..scalar()
            },
        ),
        (
            "preparation-common-state",
            Request {
                carrier: slice_prepares(ActionKind::RetainedState),
                machine: {
                    let mut machine = machine();
                    set_charge(&mut machine, ActionKind::RetainedState, Charge::Free);
                    machine.boundary = Boundary::PreparedState;
                    machine.prepared = vec![Prepared {
                        kind: ActionKind::RetainedState,
                        artifact: artifact("the slicing plan's retained state"),
                    }];
                    machine
                },
                ..scalar()
            },
        ),
        (
            "preparation-common-plan",
            Request {
                carrier: slice_prepares(ActionKind::Preprocessing),
                machine: {
                    let mut machine = machine();
                    set_charge(&mut machine, ActionKind::Preprocessing, Charge::Free);
                    machine.boundary = Boundary::PreparedPlan;
                    machine.prepared = vec![Prepared {
                        kind: ActionKind::Preprocessing,
                        artifact: artifact("the slicing plan's prepared plan"),
                    }];
                    machine
                },
                ..scalar()
            },
        ),
        // Fail-closed requests: each violates exactly one rule.
        (
            "reject-empty-domain",
            Request {
                domain: Vec::new(),
                ..scalar()
            },
        ),
        (
            "reject-internal-plan-universe",
            Request {
                carrier: Carrier::InternalPlans,
                ..scalar()
            },
        ),
        (
            "reject-optimizer-defined-universe",
            Request {
                carrier: Carrier::OptimizerOutput,
                ..scalar()
            },
        ),
        (
            "reject-discovered-universe",
            Request {
                carrier: Carrier::Discovered {
                    members: vec![0, 1],
                },
                ..scalar()
            },
        ),
        (
            "reject-cached-universe",
            Request {
                carrier: Carrier::Cached,
                ..scalar()
            },
        ),
        (
            "reject-missing-completeness",
            Request {
                completeness: Completeness::Missing,
                ..scalar()
            },
        ),
        (
            "reject-self-referential-completeness",
            Request {
                completeness: Completeness::CitesUniverseId,
                ..scalar()
            },
        ),
        (
            "reject-optimizer-completeness",
            Request {
                completeness: Completeness::CitesOptimizer,
                ..scalar()
            },
        ),
        (
            "reject-beyond-capacity",
            with_machine(|machine| machine.capacity.fuel = machine.fuel - 1),
        ),
        (
            "reject-unit-cost-operands",
            with_machine(|machine| machine.operand_size = OperandSize::Unit),
        ),
        (
            "reject-duplicate-action",
            with_machine(|machine| {
                machine
                    .actions
                    .push(accounted(ActionKind::Dispatch, Charge::Steps));
            }),
        ),
        (
            "reject-unaccounted-dispatch",
            with_machine(|machine| {
                machine
                    .actions
                    .retain(|action| action.kind != ActionKind::Dispatch);
            }),
        ),
        (
            "reject-zero-cost-dispatch",
            with_machine(|machine| {
                set_charge(machine, ActionKind::Dispatch, Charge::Constant { cost: 0 });
            }),
        ),
        (
            "reject-constant-cost-fallback",
            with_machine(|machine| {
                set_charge(machine, ActionKind::Fallback, Charge::Constant { cost: 1 });
            }),
        ),
        (
            "reject-free-observation",
            with_machine(|machine| set_charge(machine, ActionKind::Observation, Charge::Free)),
        ),
        (
            "reject-undeclared-execution",
            with_machine(|machine| {
                set_charge(machine, ActionKind::Execution, Charge::Undeclared);
            }),
        ),
        (
            "reject-zero-cost-preprocessing",
            with_machine(|machine| {
                set_charge(
                    machine,
                    ActionKind::Preprocessing,
                    Charge::Constant { cost: 0 },
                );
            }),
        ),
        (
            "reject-step-charged-advice",
            with_machine(|machine| set_charge(machine, ActionKind::Advice, Charge::Steps)),
        ),
        (
            "reject-undeclared-retained-state",
            with_machine(|machine| {
                set_charge(machine, ActionKind::RetainedState, Charge::Undeclared);
            }),
        ),
        (
            "reject-free-preprocessing-complete-boundary",
            with_machine(|machine| set_charge(machine, ActionKind::Preprocessing, Charge::Free)),
        ),
        // Free on a prepared boundary, but nothing it prepared is bound.
        (
            "reject-unbound-prepared-state",
            with_machine(|machine| {
                set_charge(machine, ActionKind::RetainedState, Charge::Free);
                machine.boundary = Boundary::PreparedState;
            }),
        ),
        // An artifact bound for an action that is charged, not free.
        (
            "reject-stray-prepared-artifact",
            with_machine(|machine| {
                set_charge(machine, ActionKind::Advice, Charge::Constant { cost: 3 });
                machine.boundary = Boundary::PreparedState;
                machine.prepared = vec![Prepared {
                    kind: ActionKind::Advice,
                    artifact: artifact("advice that is also charged"),
                }];
            }),
        ),
        // A plan needs preprocessing the machine does not account.
        (
            "reject-unaccounted-plan-preparation",
            Request {
                carrier: slice_prepares(ActionKind::Preprocessing),
                ..scalar()
            },
        ),
        (
            "reject-restricted-universe-alias",
            Request {
                claim: ClaimClass::RestrictedUniverseOptimal,
                ..scalar()
            },
        ),
        (
            "reject-scalar-claim-partial-order",
            Request {
                claim: ClaimClass::GlobalOptimal,
                ..vector()
            },
        ),
        (
            "reject-vector-claim-total-order",
            Request {
                claim: ClaimClass::FrontierComplete,
                ..scalar()
            },
        ),
        (
            "reject-unsupported-claim",
            Request {
                claim: ClaimClass::BestKnown,
                ..scalar()
            },
        ),
        (
            "reject-instance-optimal-claim",
            Request {
                claim: ClaimClass::InstanceOptimal { alpha: 2, beta: 1 },
                ..scalar()
            },
        ),
        (
            "reject-profile-defined-comparison-claim",
            Request {
                claim: ClaimClass::ProfileDefinedComparison {
                    profile: "lexlean-calculus".to_owned(),
                    class_id: "steps-per-domain".to_owned(),
                    shape: "member".to_owned(),
                },
                ..scalar()
            },
        ),
        (
            "reject-calculus-program-scope",
            Request {
                scope: Scope::CalculusPrograms,
                ..scalar()
            },
        ),
        (
            "reject-rust-program-scope",
            Request {
                scope: Scope::RustPrograms,
                ..scalar()
            },
        ),
    ];
    for kind in [
        ActionKind::Communication,
        ActionKind::Randomness,
        ActionKind::Scheduling,
    ] {
        let name = match kind {
            ActionKind::Communication => "reject-communication",
            ActionKind::Randomness => "reject-randomness",
            _ => "reject-scheduling",
        };
        out.push((
            name,
            with_machine(|machine| set_charge(machine, kind, Charge::Steps)),
        ));
    }
    out.into_iter()
        .map(|(name, request)| (name, identified(request)))
        .collect()
}

/// Every fixture, sorted by name, its answer computed by the host.
///
/// # Panics
///
/// Panics on a request the host does not load, which no generated request
/// is.
#[must_use]
pub fn fixtures() -> Vec<Fixture> {
    let mut out: Vec<Fixture> = requests()
        .into_iter()
        .map(|(name, request)| {
            let bytes = gnaf::to_file_bytes(&request);
            let request =
                gnaf::load(&bytes).unwrap_or_else(|error| panic!("request {name}: {error:?}"));
            Fixture {
                spec: FIXTURE_SPEC.to_owned(),
                name: name.to_owned(),
                expected: gnaf::answer(&request)
                    .unwrap_or_else(|error| panic!("request {name}: {error:?}")),
                request,
            }
        })
        .collect();
    out.sort_by(|left, right| left.name.cmp(&right.name));
    out
}

/// The module stating every fixture's answer.
pub const FIXTURES_MODULE: &str = "GnafFixtures";

/// The path of the dependency manifest, relative to the repository root.
pub const MANIFEST_PATH: &str = "compiler/gnaf.manifest.json";

/// A theorem that the kernel reduces `function request` to `expected`.
fn stated(name: &str, function: &str, request: &str, axioms: &[&str], expected: Json) -> Json {
    let mut out = json!({"kind": "theorem", "name": name, "parameters": [],
           "statement": {"kind": "eq",
               "left": {"kind": "call", "function": term::member(MODEL, function),
                        "arguments": [{"kind": "call", "function": {"name": request}, "arguments": []}]},
               "right": expected},
           "proof": {"kind": "reflexivity"}});
    if !axioms.is_empty() {
        out["axioms"] = json!(axioms);
    }
    out
}

/// The declarations stating one fixture: its request; that the model's
/// universe of it is the host's expansion; for an answered request, that
/// every system's status is the host's; and that the model answers it with
/// the expected answer.
///
/// # Panics
///
/// Panics on a request the host cannot evaluate, which no committed
/// request is.
#[must_use]
pub fn fixture_declarations(fixture: &Fixture) -> Vec<Json> {
    let run_axioms = crate::gnaf_model::RUN_AXIOMS;
    let id = crate::calculus::identifier(&fixture.name);
    let request_name = format!("{id}Request");
    let selector_t = term::named(MODEL, "Selector");
    let universe = term::list(
        &selector_t,
        gnaf::expand(&fixture.request.carrier)
            .into_iter()
            .map(gnaf::selector_term)
            .collect(),
    );
    let mut out = vec![
        json!({"kind": "definition", "name": request_name, "parameters": [],
               "result": term::named(MODEL, "Request"), "body": gnaf::request_term(&fixture.request)}),
        stated(
            &format!("{id}Universe"),
            "universeOf",
            &request_name,
            &[],
            universe,
        ),
    ];
    if !matches!(fixture.expected, Answer::Rejected { .. }) {
        let entry_t = json!({"kind": "product", "left": {"kind": "nat"},
                             "right": term::named(MODEL, "Status")});
        let statuses = gnaf::statuses(&fixture.request)
            .unwrap_or_else(|error| panic!("{}: {error:?}", fixture.name));
        out.push(stated(
            &format!("{id}Statuses"),
            "statusesOf",
            &request_name,
            &run_axioms,
            term::list(
                &entry_t,
                statuses
                    .into_iter()
                    .map(|(index, _, status)| {
                        json!({"kind": "pair", "left": term::nat(index),
                               "right": gnaf::status_term(status)})
                    })
                    .collect(),
            ),
        ));
    }
    out.push(stated(
        &format!("{id}Answer"),
        "answer",
        &request_name,
        &run_axioms,
        gnaf::answer_term(&fixture.expected),
    ));
    if fixture.name == PARETO_ENVELOPE {
        out.extend(pareto_envelope_rejections(fixture, &request_name));
    }
    out
}

/// GNAF-REJ-29 and GNAF-REJ-14 over the posed GNAF-VEC-02: the frontier
/// with any one member omitted is not certified, the complete frontier in
/// another order is, and the componentwise minima are attained by no
/// system.
fn pareto_envelope_rejections(fixture: &Fixture, request_name: &str) -> Vec<Json> {
    let run_axioms = crate::gnaf_model::RUN_AXIOMS;
    let Answer::Frontier { members } = &fixture.expected else {
        panic!("{PARETO_ENVELOPE} has a frontier")
    };
    let certifies = |name: String, claimed: &[u64], expected: bool| {
        json!({"kind": "theorem", "name": name, "parameters": [], "axioms": run_axioms,
               "statement": {"kind": "eq",
                   "left": {"kind": "call", "function": term::member(MODEL, "certifies"),
                            "arguments": [{"kind": "call", "function": {"name": request_name}, "arguments": []},
                                          gnaf::answer_term(&Answer::Frontier { members: claimed.to_vec() })]},
                   "right": {"kind": "bool", "value": expected}},
               "proof": {"kind": "reflexivity"}})
    };
    let mut out: Vec<Json> = (0..members.len())
        .map(|omitted| {
            let mut claimed = members.clone();
            let dropped = claimed.remove(omitted);
            certifies(
                format!("vec02ParetoEnvelopeOmits{dropped}"),
                &claimed,
                false,
            )
        })
        .collect();
    let mut reordered = members.clone();
    reordered.reverse();
    out.push(certifies(
        "vec02ParetoEnvelopeCertified".to_owned(),
        &reordered,
        true,
    ));
    out.push(stated(
        "vec02ParetoEnvelopeMinimaUnattained",
        "minimaAttained",
        request_name,
        &run_axioms,
        json!({"kind": "bool", "value": false}),
    ));
    out
}

/// The `GnafFixtures` module.
///
/// # Panics
///
/// Panics only if `serde_json` cannot serialize a JSON value.
#[must_use]
pub fn fixtures_module(fixtures: &[Fixture]) -> String {
    let declarations: Vec<Json> = fixtures.iter().flat_map(fixture_declarations).collect();
    crate::lx::module_tex(
        FIXTURES_MODULE,
        &[term::SYNTAX, term::SEMANTICS, MODEL],
        declarations,
    )
}

/// The authority's provisional identifier, revision, and SHA-256, as
/// `model/authorities.toml` records them; `conformance_gn_07` holds the two
/// equal.
pub const AUTHORITY: (&str, &str, &str, &str) = (
    "UOR-GNAF-1-DRAFT-2",
    "uor-gnaf/1-draft.2",
    "917306fd2b5a397ab02c5d38918fb8620fcc5ae0",
    "5c342373b2ff809bfd607c413cafd0582d32bb097544c6597ff7d674fe99200a",
);

/// The vendored copy of the authority, relative to the repository root.
pub const AUTHORITY_PATH: &str = "model/authorities/UOR-GNAF-v1-draft.2.md";

/// UOR-GNAF §20's dependency manifest: the draft's exact revision, every
/// profile this instantiation depends on with its role, every restriction
/// of the admitted universe, and the strongest honest claim classes.
#[must_use]
pub fn manifest() -> Json {
    let (authority, identifier, revision, sha256) = AUTHORITY;
    json!({
        "spec": MANIFEST_SPEC,
        "authority": {
            "id": authority,
            "identifier": identifier,
            "revision": revision,
            "sha256": sha256,
            "vendored": AUTHORITY_PATH,
        },
        "profiles": [
            {"name": "kind", "role": "semantic",
             "source": "compiler/src/TargetSyntax.lex.tex",
             "statement": "The calculus types and values of SPEC.md §17.14 are the kinds of every problem, argument, and result."},
            {"name": "operation", "role": "semantic",
             "source": "compiler/src/TargetSyntax.lex.tex",
             "statement": "The calculus expressions and primitives of SPEC.md §17.14 are the operations every plan is written in."},
            {"name": "machine", "role": "semantic",
             "source": "compiler/src/TargetSemantics.lex.tex",
             "statement": "TargetSemantics.run with the request's fuel is the sequential deterministic machine every system runs on, within the capacity the request binds."},
            {"name": "cost", "role": "semantic",
             "source": "compiler/src/Gnaf.lex.tex",
             "statement": "Steps as TargetSemantics charges them, operands weighted, summed over the domain, plus each reachable plan's declared preparation per invocation; size is the reachable code's node and arm count."},
            {"name": "proof", "role": "verification-only",
             "source": "LEAN-REL-4-32-1",
             "statement": "Lean 4.32.1's kernel decides every committed request's universe, statuses, and answer, the completeness theorem Gnaf.expandComplete, and the authority vectors."},
            {"name": "transcription", "role": "realization-only",
             "source": "crates/lexlean/src/gnaf.rs",
             "statement": "lexlean::gnaf loads a request, checks it, and computes the answer the model gives it; its answers are evidence through the kernel's agreement on the committed requests."},
            {"name": "interchange", "role": "realization-only",
             "source": "schemas/gnaf-request.schema.json",
             "statement": "lexlean/gnaf-request/1 is LexLean's own request form; no UOR-GNAF interchange profile is claimed."},
            {"name": "address", "role": "realization-only",
             "source": "crates/lexlean/src/gnaf.rs",
             "statement": "The SystemUniverseId is the framed SHA-256 lexlean-gnaf-universe-v1 of the problem, machine, and carrier; no UOR-GNAF address profile is claimed."},
        ],
        "measured": [],
        "restrictions": [
            "The universe is a grammar of fixed systems, one per plan, and of dispatching systems that observe the argument's length against a declared threshold and run one of two plans.",
            "The plans are the request's own calculus functions; no other calculus program and no Rust program is in the universe.",
            "The machine is sequential and deterministic: communication, randomness, and scheduling are not admitted.",
            "The domain is a finite nonempty list of arguments on which the reference returns a value.",
            "Preparation is a declared constant per invocation of each plan needing it, or a prepared artifact bound in the common initial state.",
            "The capacity is at most the host evaluator's: fuel 4096, 1024 arguments, 65536 systems, and charges of 2^32.",
        ],
        "strongest_claims": [
            {"class": "global_optimal", "scope": "grammar_universe", "objective": "scalar"},
            {"class": "argmin_complete", "scope": "grammar_universe", "objective": "scalar"},
            {"class": "pareto_optimal", "scope": "grammar_universe", "objective": "vector"},
            {"class": "frontier_complete", "scope": "grammar_universe", "objective": "vector"},
        ],
        "non_claims": [
            "No claim is made over calculus programs outside the grammar or over Rust programs.",
            "No claim is made about machine time; a cost is the calculus's step accounting.",
            "UOR-NAF is informative to UOR-GNAF and is neither cited nor relied upon.",
        ],
    })
}

/// The calculus definitions both GNAF schemas share with
/// `schemas/target-program.schema.json`, taken from it so the copies cannot
/// drift.
fn calculus_definitions() -> serde_json::Map<String, Json> {
    let program: Json =
        serde_json::from_str(include_str!("../../../schemas/target-program.schema.json"))
            .expect("the target program schema is JSON");
    program["$defs"]
        .as_object()
        .expect("the target program schema has definitions")
        .clone()
}

fn schema_id(name: &str) -> String {
    format!("https://github.com/afflom/lexlean/schemas/{name}")
}

fn closed(properties: Json) -> Json {
    let required: Vec<&String> = properties.as_object().expect("properties").keys().collect();
    json!({"additionalProperties": false, "properties": properties, "required": required,
           "type": "object"})
}

fn natural() -> Json {
    json!({"minimum": 0, "type": "integer"})
}

fn named_enum(names: &[&str]) -> Json {
    json!({"enum": names, "type": "string"})
}

fn snake(name: &str) -> String {
    let mut out = String::new();
    for character in name.chars() {
        if character.is_ascii_uppercase() {
            out.push('_');
            out.push(character.to_ascii_lowercase());
        } else {
            out.push(character);
        }
    }
    out
}

/// The definitions of a request, an answer, and their parts.
#[allow(clippy::too_many_lines)]
fn gnaf_definitions() -> serde_json::Map<String, Json> {
    let mut out = calculus_definitions();
    let reference = |name: &str| json!({"$ref": format!("#/$defs/{name}")});
    let kinds: Vec<String> = ActionKind::ALL
        .iter()
        .map(|kind| snake(kind.constructor()))
        .collect();
    let kind_names: Vec<&str> = kinds.iter().map(String::as_str).collect();
    let nullary_claims: Vec<&str> = CLAIM_CLASSES
        .iter()
        .copied()
        .filter(|class| !matches!(*class, "profile_defined_comparison" | "instance_optimal"))
        .collect();
    let definitions = [
        ("action_kind", named_enum(&kind_names)),
        (
            "charge",
            json!({"oneOf": [
                closed(json!({"kind": named_enum(&["steps", "free", "undeclared"])})),
                closed(json!({"cost": natural(), "kind": {"const": "constant"}})),
            ]}),
        ),
        (
            "action",
            closed(json!({"charge": reference("charge"), "kind": reference("action_kind")})),
        ),
        (
            "digest",
            json!({"pattern": "^[0-9a-f]{64}$", "type": "string"}),
        ),
        (
            "capacity",
            closed(
                json!({"charge": natural(), "domain": natural(), "fuel": natural(),
                          "systems": natural()}),
            ),
        ),
        (
            "prepared",
            closed(json!({"artifact": reference("digest"), "kind": reference("action_kind")})),
        ),
        (
            "machine",
            closed(json!({
                "actions": {"items": reference("action"), "type": "array"},
                "boundary": named_enum(&["complete", "prepared_state", "prepared_plan"]),
                "capacity": reference("capacity"),
                "fuel": natural(),
                "operand_size": named_enum(&["weighted", "unit"]),
                "prepared": {"items": reference("prepared"), "type": "array"},
            })),
        ),
        (
            "plan",
            closed(json!({"function": reference("function"),
                          "prepares": {"items": reference("action_kind"), "type": "array"}})),
        ),
        (
            "carrier",
            json!({"oneOf": [
                closed(json!({"argument": reference("ty"), "kind": {"const": "grammar"},
                              "plans": {"items": reference("plan"), "type": "array"},
                              "result": reference("ty"),
                              "thresholds": {"items": natural(), "type": "array"}})),
                closed(json!({"kind": named_enum(&["internal_plans", "optimizer_output", "cached"])})),
                closed(json!({"kind": {"const": "discovered"},
                              "members": {"items": natural(), "type": "array"}})),
            ]}),
        ),
        (
            "claim",
            json!({"oneOf": [
                closed(json!({"class": named_enum(&nullary_claims)})),
                closed(json!({"class": {"const": "profile_defined_comparison"},
                              "class_id": {"type": "string"}, "profile": {"type": "string"},
                              "shape": {"type": "string"}})),
                closed(json!({"alpha": natural(), "beta": natural(),
                              "class": {"const": "instance_optimal"}})),
            ]}),
        ),
        (
            "rejection",
            json!({"oneOf": [
                closed(json!({"kind": named_enum(&REJECTIONS)})),
                closed(json!({"action": reference("action_kind"),
                              "kind": named_enum(&ACTION_REJECTIONS)})),
            ]}),
        ),
        (
            "answer",
            json!({"oneOf": [
                closed(json!({"kind": {"const": "rejected"}, "rejection": reference("rejection")})),
                closed(json!({"kind": {"const": "argmin"},
                              "members": {"items": natural(), "type": "array"},
                              "steps": natural()})),
                closed(json!({"kind": {"const": "frontier"},
                              "members": {"items": natural(), "type": "array"}})),
                closed(json!({"kind": named_enum(&["infeasible", "incomplete"])})),
            ]}),
        ),
        ("request", request_object()),
    ];
    for (name, definition) in definitions {
        out.insert(name.to_owned(), definition);
    }
    out
}

/// The request object, without the schema's own identification.
fn request_object() -> Json {
    let reference = |name: &str| json!({"$ref": format!("#/$defs/{name}")});
    closed(json!({
        "carrier": reference("carrier"),
        "claim": reference("claim"),
        "completeness": named_enum(&["grammar_equality", "missing", "cites_universe_id",
                                     "cites_optimizer"]),
        "domain": {"items": reference("value"), "type": "array"},
        "machine": reference("machine"),
        "objective": named_enum(&["scalar", "vector"]),
        "reference": reference("program"),
        "scope": named_enum(&["grammar_universe", "calculus_programs", "rust_programs"]),
        "spec": {"const": REQUEST_SPEC},
        "universe": reference("digest"),
    }))
}

/// The §12.4 claim classes as the request form spells them.
pub const CLAIM_CLASSES: [&str; 31] = [
    "exact",
    "normal_form",
    "canonical",
    "representation_minimal",
    "comparison_theorem",
    "profile_defined_comparison",
    "input_total",
    "global_optimal",
    "argmin_complete",
    "pareto_optimal",
    "frontier_complete",
    "pointwise_envelope_complete",
    "query_family_answer_complete",
    "use_case_global_optimal",
    "workload_argmin_complete",
    "workload_pareto_optimal",
    "workload_frontier_complete",
    "family_optimal",
    "competitive_bound",
    "competitive_optimal",
    "asymptotic_bound",
    "asymptotic_optimal",
    "use_case_class_complete",
    "use_case_class_answer_complete",
    "maintained_use_case_class",
    "restricted_universe_optimal",
    "revision_preserved",
    "best_known",
    "measured_best_among_tested",
    "heuristic_selected",
    "instance_optimal",
];

/// The rejections that carry nothing.
const REJECTIONS: [&str; 15] = [
    "empty_domain",
    "internal_plan_universe",
    "optimizer_defined_universe",
    "discovered_universe",
    "cached_universe",
    "missing_completeness",
    "self_referential_completeness",
    "optimizer_completeness",
    "beyond_capacity",
    "unit_cost_operands",
    "claim_alias",
    "scalar_claim_over_partial_order",
    "vector_claim_over_total_order",
    "uncovered_scope",
    "unsupported_claim",
];

/// The rejections that name an action.
const ACTION_REJECTIONS: [&str; 6] = [
    "duplicate_action",
    "unaccounted_action",
    "hidden_cost",
    "unrealizable_action",
    "unbound_preparation",
    "stray_prepared_artifact",
];

/// `schemas/gnaf-request.schema.json` and `schemas/gnaf-fixture.schema.json`.
#[must_use]
pub fn schemas() -> [(&'static str, Json); 2] {
    let definitions = gnaf_definitions();
    let mut request = request_object();
    let object = request.as_object_mut().expect("an object");
    object.insert(
        "$id".to_owned(),
        json!(schema_id("gnaf-request.schema.json")),
    );
    object.insert(
        "$schema".to_owned(),
        json!("https://json-schema.org/draft/2020-12/schema"),
    );
    object.insert(
        "title".to_owned(),
        json!("LexLean GNAF request over the production realization calculus (SPEC.md §17.15)"),
    );
    let mut request_definitions = definitions.clone();
    request_definitions.remove("request");
    request_definitions.remove("answer");
    request_definitions.remove("rejection");
    object.insert("$defs".to_owned(), Json::Object(request_definitions));
    let mut fixture = closed(json!({
        "expected": {"$ref": "#/$defs/answer"},
        "name": {"pattern": "^[a-z0-9]+(-[a-z0-9]+)*$", "type": "string"},
        "request": {"$ref": "#/$defs/request"},
        "spec": {"const": FIXTURE_SPEC},
    }));
    let object = fixture.as_object_mut().expect("an object");
    object.insert(
        "$id".to_owned(),
        json!(schema_id("gnaf-fixture.schema.json")),
    );
    object.insert(
        "$schema".to_owned(),
        json!("https://json-schema.org/draft/2020-12/schema"),
    );
    object.insert(
        "title".to_owned(),
        json!("LexLean GNAF fixture (SPEC.md §17.15)"),
    );
    object.insert("$defs".to_owned(), Json::Object(definitions));
    [
        ("schemas/gnaf-request.schema.json", request),
        ("schemas/gnaf-fixture.schema.json", fixture),
    ]
}

/// Every generated file, by path relative to the repository root.
#[must_use]
pub fn files() -> BTreeMap<String, Vec<u8>> {
    let fixtures = fixtures();
    let mut out = BTreeMap::new();
    for fixture in &fixtures {
        out.insert(
            format!("compiler/gnaf/{}.json", fixture.name),
            gnaf::to_file_bytes(fixture),
        );
    }
    out.insert(
        format!("compiler/src/{FIXTURES_MODULE}.lex.tex"),
        fixtures_module(&fixtures).into_bytes(),
    );
    out.insert(
        crate::gnaf_model::PATH.to_owned(),
        crate::gnaf_model::module().into_bytes(),
    );
    out.insert(MANIFEST_PATH.to_owned(), gnaf::to_file_bytes(&manifest()));
    for (path, schema) in schemas() {
        out.insert(path.to_owned(), gnaf::to_file_bytes(&schema));
    }
    out
}

/// The answer a fixture is expected to have, for the conformance cases that
/// read one by name.
///
/// # Panics
///
/// Panics on a name no fixture has.
#[must_use]
pub fn expected(name: &str) -> Answer {
    fixtures()
        .into_iter()
        .find(|fixture| fixture.name == name)
        .unwrap_or_else(|| panic!("no GNAF fixture {name}"))
        .expected
}
