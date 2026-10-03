//! Calculus transcriptions of the forward engine the reasoning example's
//! clinical module elaborates to (SPEC.md §17.12, §17.14).
//!
//! The `compiler` project's `ReasoningOracle` module states exactly the
//! declarations of `examples/reasoning/src/Clinic.lex.tex`, so its `Triage`
//! and `Review` reasoners elaborate, in Lean, to the same select, fire,
//! saturate, conclude, and verdict definitions the example verifies. Each
//! fixture here transcribes one of those elaborations by hand, function for
//! function: the rules' guards, conclusions, and guarded applications, the
//! first-applicable selection in declared priority order, the firing by
//! step, the fuel-bounded saturation through the `iterate_until` template,
//! and the verifier-checked conclusion. The fixture module states that the
//! kernel reduces each transcription to the value the oracle's own reasoner
//! computes on the same patient, so a transcription that searches
//! differently, skips a guard, or answers a verdict the reasoner does not is
//! refused by Lean.

use lexlean::calculus::library::Template;
use lexlean::calculus::{Expr, Function, Shape, Ty, Value};
use serde_json::{json, Value as Json};

use super::{
    arm, boolean, build, call, closure, cond, field, function, matching, nat, natv, opt_t, p,
    pair_t, res_t, v, Case, Prim,
};
use crate::lx;

/// The module stating the clinical declarations.
pub const ORACLE: &str = "ReasoningOracle";

/// The example source whose declarations the oracle states.
pub const SOURCE: &str = "examples/reasoning/src/Clinic.lex.tex";

/// The vital signs, in field order.
const VITALS: [&str; 6] = [
    "temperature",
    "heartRate",
    "respiratoryRate",
    "whiteCells",
    "systolic",
    "infection",
];

/// The chart's derived findings and level, in field order after `vitals`.
const FINDINGS: [&str; 8] = [
    "fever",
    "tachycardia",
    "tachypnea",
    "leukocytosis",
    "sirs",
    "sepsis",
    "shock",
    "level",
];

/// The rules in declared priority order: their constructor index in the
/// step type is their position.
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

/// Febrile, tachycardic, tachypneic, leukocytic, hypotensive, infected.
pub const SHOCKED: [u64; 6] = [392, 118, 26, 15, 82, 1];
/// Within every threshold.
pub const WELL: [u64; 6] = [368, 72, 14, 7, 120, 0];
/// Febrile and tachycardic without infection.
pub const FEBRILE: [u64; 6] = [390, 100, 14, 7, 120, 0];

// ADT indices of the transcriptions.
const VITALS_ADT: u64 = 0;
const CHART_ADT: u64 = 1;
const STEP_ADT: u64 = 2;
const LEDGER_ADT: u64 = 3;
const RUN_ADT: u64 = 4;

fn adt(index: u64) -> Ty {
    Ty::Adt { index }
}
fn chart_t() -> Ty {
    adt(CHART_ADT)
}
fn failure_t() -> Ty {
    pair_t(Ty::Bool, Ty::Bool)
}
fn failure(left: bool, right: bool) -> Expr {
    build(
        Shape::Pair,
        failure_t(),
        vec![boolean(left), boolean(right)],
    )
}
fn and(left: Expr, right: Expr) -> Expr {
    p(Prim::BoolAnd, vec![left, right])
}
fn or(left: Expr, right: Expr) -> Expr {
    p(Prim::BoolOr, vec![left, right])
}
fn not(value: Expr) -> Expr {
    p(Prim::BoolNot, vec![value])
}
fn eq(left: Expr, right: Expr) -> Expr {
    p(Prim::NatEq, vec![left, right])
}
fn lt(left: Expr, right: Expr) -> Expr {
    p(Prim::NatLt, vec![left, right])
}
fn le(left: Expr, right: Expr) -> Expr {
    p(Prim::NatLe, vec![left, right])
}
fn plus(left: Expr, right: Expr) -> Expr {
    p(Prim::NatAdd, vec![left, right])
}
fn some(ty: &Ty, value: Expr) -> Expr {
    build(Shape::Some, opt_t(ty.clone()), vec![value])
}
fn none(ty: &Ty) -> Expr {
    build(Shape::None, opt_t(ty.clone()), Vec::new())
}

/// A chart's field: `vitals` is field 0, the findings follow.
fn finding(chart: Expr, name: &str) -> Expr {
    let index = FINDINGS
        .iter()
        .position(|finding| *finding == name)
        .expect("a finding");
    field(chart, index as u64 + 1)
}
fn vital(vitals: Expr, name: &str) -> Expr {
    let index = VITALS
        .iter()
        .position(|vital| *vital == name)
        .expect("a vital sign");
    field(vitals, index as u64)
}

/// The function indices every transcription shares, after its entry and its
/// own functions: the clinical tests, the rules, selection, firing, and
/// conclusion.
#[derive(Clone, Copy)]
struct Layout {
    /// The first shared function.
    base: u64,
}

impl Layout {
    const TESTS: [&'static str; 5] = [
        "feverish",
        "tachycardic",
        "tachypneic",
        "leukocytic",
        "hypotensive",
    ];
    fn test(self, name: &str) -> u64 {
        self.base
            + Self::TESTS
                .iter()
                .position(|test| *test == name)
                .expect("a clinical test") as u64
    }
    fn guard(self, rule: usize) -> u64 {
        self.base + 5 + rule as u64
    }
    fn conclusion(self, rule: usize) -> u64 {
        self.base + 13 + rule as u64
    }
    fn apply(self, rule: usize) -> u64 {
        self.base + 21 + rule as u64
    }
    fn observe(self) -> u64 {
        self.base + 29
    }
    fn select(self) -> u64 {
        self.base + 30
    }
    fn fire(self) -> u64 {
        self.base + 31
    }
    fn next(self) -> u64 {
        self.base + 32
    }
    fn extract(self) -> u64 {
        self.base + 33
    }
    fn accept(self) -> u64 {
        self.base + 34
    }
    fn conclude(self) -> u64 {
        self.base + 35
    }
    fn check(self) -> u64 {
        self.base + 36
    }
    /// The first function after the shared ones.
    fn end(self) -> u64 {
        self.base + 37
    }
}

/// The verifier a reasoner checks its answer with.
#[derive(Clone, Copy)]
enum Verifier {
    /// `recommendationCheck`: at most level 3, and level 3 for a febrile,
    /// tachycardic, hypotensive, infected patient.
    Recommendation,
    /// `dischargeCheck`: level 0.
    Outpatient,
}

/// The guard of rule `index` over the chart (variable 0).
fn guard(layout: Layout, index: usize) -> Expr {
    let chart = || v(0);
    let vitals = || field(v(0), 0);
    let fresh = |name: &str| eq(finding(chart(), name), nat(0));
    let test = |name: &str| call(layout.test(name), vec![vitals()]);
    let is = |name: &str, value: u64| eq(finding(chart(), name), nat(value));
    match RULES[index] {
        "Fever" => and(fresh("fever"), test("feverish")),
        "Tachycardia" => and(fresh("tachycardia"), test("tachycardic")),
        "Tachypnea" => and(fresh("tachypnea"), test("tachypneic")),
        "Leukocytosis" => and(fresh("leukocytosis"), test("leukocytic")),
        "Sirs" => and(
            fresh("sirs"),
            le(
                nat(2),
                plus(
                    plus(
                        plus(finding(chart(), "fever"), finding(chart(), "tachycardia")),
                        finding(chart(), "tachypnea"),
                    ),
                    finding(chart(), "leukocytosis"),
                ),
            ),
        ),
        "Sepsis" => and(
            fresh("sepsis"),
            and(is("sirs", 1), eq(vital(vitals(), "infection"), nat(1))),
        ),
        "Shock" => and(fresh("shock"), and(is("sepsis", 1), test("hypotensive"))),
        _ => and(
            lt(finding(chart(), "level"), nat(3)),
            or(
                and(is("level", 0), is("sirs", 1)),
                or(
                    and(is("level", 1), is("sepsis", 1)),
                    and(is("level", 2), is("shock", 1)),
                ),
            ),
        ),
    }
}

/// The conclusion of rule `index`: the chart with one finding raised.
fn conclusion(index: usize) -> Expr {
    let (raised, value) = match RULES[index] {
        "Escalate" => ("level", plus(finding(v(0), "level"), nat(1))),
        rule => (
            FINDINGS
                .iter()
                .copied()
                .find(|finding| finding.eq_ignore_ascii_case(rule))
                .expect("a rule raises its finding"),
            nat(1),
        ),
    };
    let mut fields = vec![field(v(0), 0)];
    let mut value = Some(value);
    for name in FINDINGS {
        fields.push(if name == raised {
            value.take().expect("raised once")
        } else {
            finding(v(0), name)
        });
    }
    build(Shape::Adt { constructor: 0 }, chart_t(), fields)
}

/// The functions every transcription shares, in [`Layout`] order.
fn shared(layout: Layout, verifier: Verifier) -> Vec<Function> {
    let vitals_t = adt(VITALS_ADT);
    let step_t = adt(STEP_ADT);
    let mut out = Vec::new();
    // The clinical tests.
    for (test, body) in [
        ("feverish", lt(nat(380), vital(v(0), "temperature"))),
        ("tachycardic", lt(nat(90), vital(v(0), "heartRate"))),
        ("tachypneic", lt(nat(20), vital(v(0), "respiratoryRate"))),
        ("leukocytic", lt(nat(12), vital(v(0), "whiteCells"))),
        ("hypotensive", lt(vital(v(0), "systolic"), nat(90))),
    ] {
        debug_assert_eq!(layout.test(test), layout.base + out.len() as u64);
        out.push(function(vec![vitals_t.clone()], Ty::Bool, body));
    }
    for index in 0..RULES.len() {
        out.push(function(vec![chart_t()], Ty::Bool, guard(layout, index)));
    }
    for index in 0..RULES.len() {
        out.push(function(vec![chart_t()], chart_t(), conclusion(index)));
    }
    // The guarded application: the only way a step applies a rule.
    for index in 0..RULES.len() {
        out.push(function(
            vec![chart_t()],
            opt_t(chart_t()),
            cond(
                call(layout.guard(index), vec![v(0)]),
                some(&chart_t(), call(layout.conclusion(index), vec![v(0)])),
                none(&chart_t()),
            ),
        ));
    }
    // observe: the admitted chart.
    let mut admitted = vec![v(0)];
    admitted.extend(FINDINGS.iter().map(|_| nat(0)));
    out.push(function(
        vec![vitals_t.clone()],
        chart_t(),
        build(Shape::Adt { constructor: 0 }, chart_t(), admitted),
    ));
    // select: the first rule whose guard holds, in priority order.
    let mut selected = none(&step_t);
    for index in (0..RULES.len()).rev() {
        selected = cond(
            call(layout.guard(index), vec![v(0)]),
            some(
                &step_t,
                build(
                    Shape::Adt {
                        constructor: index as u64,
                    },
                    step_t.clone(),
                    Vec::new(),
                ),
            ),
            selected,
        );
    }
    out.push(function(vec![chart_t()], opt_t(step_t.clone()), selected));
    // fire: the step's guarded application.
    out.push(function(
        vec![chart_t(), step_t.clone()],
        opt_t(chart_t()),
        matching(
            opt_t(chart_t()),
            v(1),
            (0..RULES.len())
                .map(|index| {
                    arm(
                        Shape::Adt {
                            constructor: index as u64,
                        },
                        Vec::new(),
                        call(layout.apply(index), vec![v(0)]),
                    )
                })
                .collect(),
        ),
    ));
    // next: fire the selected rule.
    out.push(function(
        vec![chart_t()],
        opt_t(chart_t()),
        matching(
            opt_t(chart_t()),
            call(layout.select(), vec![v(0)]),
            vec![
                arm(Shape::None, Vec::new(), none(&chart_t())),
                arm(Shape::Some, vec![1], call(layout.fire(), vec![v(0), v(1)])),
            ],
        ),
    ));
    // extract: the chart's level.
    out.push(function(
        vec![chart_t()],
        opt_t(Ty::Nat),
        some(&Ty::Nat, finding(v(0), "level")),
    ));
    // accept: the extracted answer, if the verifier's check accepts it.
    out.push(function(
        vec![vitals_t.clone(), chart_t()],
        opt_t(Ty::Nat),
        matching(
            opt_t(Ty::Nat),
            call(layout.extract(), vec![v(1)]),
            vec![
                arm(Shape::None, Vec::new(), none(&Ty::Nat)),
                arm(
                    Shape::Some,
                    vec![2],
                    cond(
                        call(layout.check(), vec![v(0), v(2)]),
                        some(&Ty::Nat, v(2)),
                        none(&Ty::Nat),
                    ),
                ),
            ],
        ),
    ));
    // conclude: the accepted answer, or rejected, or unsolved.
    let verdict_t = res_t(Ty::Nat, failure_t());
    out.push(function(
        vec![vitals_t.clone(), chart_t()],
        verdict_t.clone(),
        matching(
            verdict_t.clone(),
            call(layout.accept(), vec![v(0), v(1)]),
            vec![
                arm(
                    Shape::Some,
                    vec![2],
                    build(Shape::Ok, verdict_t.clone(), vec![v(2)]),
                ),
                arm(
                    Shape::None,
                    Vec::new(),
                    matching(
                        verdict_t.clone(),
                        call(layout.extract(), vec![v(1)]),
                        vec![
                            arm(
                                Shape::Some,
                                vec![3],
                                build(Shape::Error, verdict_t.clone(), vec![failure(true, false)]),
                            ),
                            arm(
                                Shape::None,
                                Vec::new(),
                                build(Shape::Error, verdict_t.clone(), vec![failure(false, true)]),
                            ),
                        ],
                    ),
                ),
            ],
        ),
    ));
    // The verifier's executable check.
    let check = match verifier {
        Verifier::Recommendation => {
            let test = |name: &str| call(layout.test(name), vec![v(0)]);
            and(
                le(v(1), nat(3)),
                or(
                    eq(v(1), nat(3)),
                    not(and(
                        test("hypotensive"),
                        and(
                            eq(vital(v(0), "infection"), nat(1)),
                            and(test("feverish"), test("tachycardic")),
                        ),
                    )),
                ),
            )
        }
        Verifier::Outpatient => eq(v(1), nat(0)),
    };
    out.push(function(vec![vitals_t, Ty::Nat], Ty::Bool, check));
    debug_assert_eq!(layout.end(), layout.base + out.len() as u64);
    out
}

/// The ADTs: the vital signs, the chart, the steps, and, for a traced run,
/// the ledger and the run.
fn adts(traced: bool) -> Vec<Vec<Vec<Ty>>> {
    let mut chart = vec![adt(VITALS_ADT)];
    chart.extend(FINDINGS.iter().map(|_| Ty::Nat));
    let mut out = vec![
        vec![VITALS.iter().map(|_| Ty::Nat).collect()],
        vec![chart],
        RULES.iter().map(|_| Vec::new()).collect(),
    ];
    if traced {
        out.push(vec![vec![Ty::Nat; 6]]);
        out.push(vec![vec![
            chart_t(),
            super::list_t(adt(STEP_ADT)),
            adt(LEDGER_ADT),
        ]]);
    }
    out
}

/// A patient as a calculus value.
fn patient(values: [u64; 6]) -> Value {
    Value::Adt {
        constructor: 0,
        fields: values.into_iter().map(natv).collect(),
    }
}

/// The same patient as an oracle term.
fn patient_term(values: [u64; 6]) -> Json {
    lx::record(
        lexlean::calculus::term::member(ORACLE, "Vitals"),
        VITALS
            .iter()
            .zip(values)
            .map(|(name, value)| (*name, lx::nat(value)))
            .collect(),
    )
}

fn oracle_call(name: &str, arguments: Vec<Json>) -> Json {
    lx::call(lexlean::calculus::term::member(ORACLE, name), arguments)
}

/// `match result with ok v => Value.ok (ok v) | error e => Value.error
/// (encodeFailure e)`.
fn encode_result(result: Json, ok: impl FnOnce(Json) -> Json) -> Json {
    lx::matching(
        result,
        vec![
            lx::branch(
                lx::member("Result.ok"),
                vec!["found".to_owned()],
                lx::value("ok", vec![ok(lx::var("found"))]),
            ),
            lx::branch(
                lx::member("Result.error"),
                vec!["failure".to_owned()],
                lx::value(
                    "error",
                    vec![lx::call(
                        lx::member("encodeFailure"),
                        vec![lx::var("failure")],
                    )],
                ),
            ),
        ],
    )
}

/// The verdict transcription of a forward reasoner with `fuel` and
/// `verifier`: entry 0 is `verdict`, the `iterate_until` instance follows
/// the shared functions.
fn verdict_functions(fuel: u64, verifier: Verifier) -> (Vec<Function>, u64) {
    let layout = Layout { base: 1 };
    let iterate = layout.end();
    let saturated = pair_t(chart_t(), Ty::Bool);
    let verdict_t = res_t(Ty::Nat, failure_t());
    let entry = function(
        vec![adt(VITALS_ADT)],
        verdict_t.clone(),
        super::let_in(
            1,
            saturated,
            call(
                iterate,
                vec![
                    closure(layout.next(), Vec::new()),
                    nat(fuel),
                    call(layout.observe(), vec![v(0)]),
                ],
            ),
            cond(
                super::second(v(1)),
                call(layout.conclude(), vec![v(0), super::first(v(1))]),
                build(Shape::Error, verdict_t, vec![failure(false, false)]),
            ),
        ),
    );
    let mut functions = vec![entry];
    functions.extend(shared(layout, verifier));
    (functions, iterate)
}

/// The traced transcription of `Triage`: the explained verdict with the
/// run's guard evaluations and firings, as `Triage.account` counts them.
fn traced_functions() -> (Vec<Function>, u64) {
    let layout = Layout { base: 4 };
    let iterate = layout.end();
    let step_t = adt(STEP_ADT);
    let steps_t = super::list_t(step_t.clone());
    let run_t = adt(RUN_ADT);
    let ledger_t = adt(LEDGER_ADT);
    let explained_t = res_t(pair_t(Ty::Nat, steps_t.clone()), failure_t());
    let result_t = pair_t(explained_t.clone(), pair_t(Ty::Nat, Ty::Nat));
    let ledger = |run: Expr, index: u64| field(field(run, 2), index);
    // 0 patient; 1 the final run and whether it saturated; 2 the run.
    let entry = function(
        vec![adt(VITALS_ADT)],
        result_t.clone(),
        super::let_in(
            1,
            pair_t(run_t.clone(), Ty::Bool),
            call(
                iterate,
                vec![closure(2, Vec::new()), nat(11), call(1, vec![v(0)])],
            ),
            super::let_in(
                2,
                run_t.clone(),
                super::first(v(1)),
                build(
                    Shape::Pair,
                    result_t,
                    vec![
                        cond(
                            super::second(v(1)),
                            matching(
                                explained_t.clone(),
                                call(layout.conclude(), vec![v(0), field(v(2), 0)]),
                                vec![
                                    arm(
                                        Shape::Ok,
                                        vec![3],
                                        build(
                                            Shape::Ok,
                                            explained_t.clone(),
                                            vec![build(
                                                Shape::Pair,
                                                pair_t(Ty::Nat, steps_t.clone()),
                                                vec![v(3), field(v(2), 1)],
                                            )],
                                        ),
                                    ),
                                    arm(
                                        Shape::Error,
                                        vec![4],
                                        build(Shape::Error, explained_t.clone(), vec![v(4)]),
                                    ),
                                ],
                            ),
                            build(Shape::Error, explained_t, vec![failure(false, false)]),
                        ),
                        // `Triage.account`: the final scan's guard
                        // evaluations count when the run saturated.
                        build(
                            Shape::Pair,
                            pair_t(Ty::Nat, Ty::Nat),
                            vec![
                                cond(
                                    super::second(v(1)),
                                    plus(ledger(v(2), 1), call(3, vec![field(v(2), 0)])),
                                    ledger(v(2), 1),
                                ),
                                ledger(v(2), 2),
                            ],
                        ),
                    ],
                ),
            ),
        ),
    );
    // start: the observed chart, no steps, a zero ledger.
    let start = function(
        vec![adt(VITALS_ADT)],
        run_t.clone(),
        build(
            Shape::Adt { constructor: 0 },
            run_t.clone(),
            vec![
                call(layout.observe(), vec![v(0)]),
                build(Shape::Nil, steps_t.clone(), Vec::new()),
                build(
                    Shape::Adt { constructor: 0 },
                    ledger_t.clone(),
                    vec![nat(0), nat(0), nat(0), nat(0), nat(0), nat(0)],
                ),
            ],
        ),
    );
    // step: fire the selected rule, appending it to the trace and counting
    // the iteration and the firing. 0 run; 1 step; 2 the fired chart.
    let step = function(
        vec![run_t.clone()],
        opt_t(run_t.clone()),
        matching(
            opt_t(run_t.clone()),
            call(layout.select(), vec![field(v(0), 0)]),
            vec![
                arm(Shape::None, Vec::new(), none(&run_t)),
                arm(
                    Shape::Some,
                    vec![1],
                    matching(
                        opt_t(run_t.clone()),
                        call(layout.fire(), vec![field(v(0), 0), v(1)]),
                        vec![
                            arm(Shape::None, Vec::new(), none(&run_t)),
                            arm(
                                Shape::Some,
                                vec![2],
                                some(
                                    &run_t,
                                    build(
                                        Shape::Adt { constructor: 0 },
                                        run_t.clone(),
                                        vec![
                                            v(2),
                                            p(
                                                Prim::Append,
                                                vec![
                                                    field(v(0), 1),
                                                    build(
                                                        Shape::Cons,
                                                        steps_t.clone(),
                                                        vec![
                                                            v(1),
                                                            build(Shape::Nil, steps_t, Vec::new()),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                            build(
                                                Shape::Adt { constructor: 0 },
                                                ledger_t,
                                                vec![
                                                    plus(ledger(v(0), 0), nat(1)),
                                                    plus(
                                                        ledger(v(0), 1),
                                                        call(3, vec![field(v(0), 0)]),
                                                    ),
                                                    plus(ledger(v(0), 2), nat(1)),
                                                    ledger(v(0), 3),
                                                    ledger(v(0), 4),
                                                    ledger(v(0), 5),
                                                ],
                                            ),
                                        ],
                                    ),
                                ),
                            ),
                        ],
                    ),
                ),
            ],
        ),
    );
    // attempts: the guard evaluations `select` makes on a chart.
    let mut attempts = nat(0);
    for index in (0..RULES.len()).rev() {
        attempts = cond(
            call(layout.guard(index), vec![v(0)]),
            nat(1),
            plus(attempts, nat(1)),
        );
    }
    let attempts = function(vec![chart_t()], Ty::Nat, attempts);
    let mut functions = vec![entry, start, step, attempts];
    functions.extend(shared(layout, Verifier::Recommendation));
    (functions, iterate)
}

/// A reasoning fixture: the transcription, its `iterate_until` instance,
/// and the oracle's value on the same patient.
fn reasoning_case(
    name: &str,
    traced: bool,
    (functions, at): (Vec<Function>, u64),
    state: Ty,
    arguments: Vec<Value>,
    oracle: Json,
) -> Case {
    debug_assert_eq!(at, functions.len() as u64);
    let mut all = functions;
    all.extend(
        Template::IterateUntil
            .instantiate(std::slice::from_ref(&state), at)
            .unwrap_or_else(|reason| panic!("fixture {name}: {reason}")),
    );
    let mut out = super::case(name, super::program(adts(traced), all), 0, arguments, 4000);
    out.library = Some(super::LibraryUse {
        template: Template::IterateUntil,
        types: vec![state],
        at,
    });
    out.oracle = Some(oracle);
    out
}

/// The verdict of reasoner `reasoner` on `values`, encoded as a value.
fn verdict_oracle(reasoner: &str, values: [u64; 6]) -> Json {
    encode_result(
        oracle_call(&format!("{reasoner}.verdict"), vec![patient_term(values)]),
        |level| lx::value("nat", vec![level]),
    )
}

/// Every reasoning fixture.
pub fn cases() -> Vec<Case> {
    let verdict = |name: &str, reasoner: &str, fuel: u64, verifier: Verifier, values: [u64; 6]| {
        reasoning_case(
            name,
            false,
            verdict_functions(fuel, verifier),
            chart_t(),
            vec![patient(values)],
            verdict_oracle(reasoner, values),
        )
    };
    let account = oracle_call("Triage.account", vec![patient_term(SHOCKED)]);
    let ledger = |counter: &str| lx::value("nat", vec![lx::project(account.clone(), counter)]);
    vec![
        verdict(
            "reasoning-triage-shock",
            "Triage",
            11,
            Verifier::Recommendation,
            SHOCKED,
        ),
        verdict(
            "reasoning-triage-well",
            "Triage",
            11,
            Verifier::Recommendation,
            WELL,
        ),
        verdict(
            "reasoning-review-exhausted",
            "Review",
            6,
            Verifier::Outpatient,
            SHOCKED,
        ),
        verdict(
            "reasoning-review-rejected",
            "Review",
            6,
            Verifier::Outpatient,
            FEBRILE,
        ),
        reasoning_case(
            "reasoning-triage-traced",
            true,
            traced_functions(),
            adt(RUN_ADT),
            vec![patient(SHOCKED)],
            lx::value(
                "pair",
                vec![
                    encode_result(
                        oracle_call("Triage", vec![patient_term(SHOCKED)]),
                        |found| {
                            lx::value(
                                "pair",
                                vec![
                                    lx::value("nat", vec![lx::first(found.clone())]),
                                    lx::value(
                                        "list",
                                        vec![lx::call(
                                            lx::member("encodeTriageSteps"),
                                            vec![lx::second(found)],
                                        )],
                                    ),
                                ],
                            )
                        },
                    ),
                    lx::value("pair", vec![ledger("attempts"), ledger("firings")]),
                ],
            ),
        ),
    ]
}

/// The encoders the fixture module states the oracle's values with: a
/// reasoning failure as its pair of Booleans, and a `Triage` step as the
/// constructor of the transcription's step type.
pub fn encoders() -> Vec<Json> {
    let value_t = lexlean::calculus::term::named(lexlean::calculus::term::SYNTAX, "Value");
    let failure_branch = |name: &str, left: bool, right: bool| {
        lx::branch(
            lx::member(&format!("ReasoningFailure.{name}")),
            Vec::new(),
            lx::value(
                "pair",
                vec![
                    lx::value("bool", vec![lx::boolean(left)]),
                    lx::value("bool", vec![lx::boolean(right)]),
                ],
            ),
        )
    };
    let encode_failure = lx::definition(
        "encodeFailure",
        vec![lx::parameter(
            "failure",
            json!({"kind": "reasoning_failure"}),
        )],
        value_t.clone(),
        lx::matching(
            lx::var("failure"),
            vec![
                failure_branch("exhausted", false, false),
                failure_branch("unsolved", false, true),
                failure_branch("rejected", true, false),
                failure_branch("invalid_step", true, true),
            ],
        ),
    );
    let step_t = lexlean::calculus::term::named(ORACLE, "Triage.Step");
    let encode_step = lx::definition(
        "encodeTriageStep",
        vec![lx::parameter("step", step_t.clone())],
        value_t.clone(),
        lx::matching(
            lx::var("step"),
            RULES
                .iter()
                .enumerate()
                .map(|(index, rule)| {
                    lx::branch(
                        lexlean::calculus::term::member(ORACLE, &format!("Triage.Step.{rule}")),
                        Vec::new(),
                        lx::value("adt", vec![lx::nat(index as u64), lx::nil(value_t.clone())]),
                    )
                })
                .collect(),
        ),
    );
    let encode_steps = lx::recursive(
        "steps",
        lx::definition(
            "encodeTriageSteps",
            vec![lx::parameter("steps", lx::list_t(step_t))],
            lx::list_t(value_t.clone()),
            lx::matching(
                lx::var("steps"),
                vec![
                    lx::branch(lx::member("List.nil"), Vec::new(), lx::nil(value_t)),
                    lx::branch(
                        lx::member("List.cons"),
                        vec!["step".to_owned(), "rest".to_owned()],
                        lx::cons(
                            lx::call(lx::member("encodeTriageStep"), vec![lx::var("step")]),
                            lx::call(lx::member("encodeTriageSteps"), vec![lx::var("rest")]),
                        ),
                    ),
                ],
            ),
        ),
    );
    vec![encode_failure, encode_step, encode_steps]
}

/// The oracle module: exactly the declarations of [`SOURCE`].
///
/// # Panics
///
/// Panics if the example source is missing or states no semantic data.
#[must_use]
pub fn oracle_module() -> String {
    let path = crate::support::repo_root().join(SOURCE);
    let source = std::fs::read_to_string(path.as_std_path()).expect("the reasoning example");
    let declarations = crate::rust_packages::declarations(&source)
        .unwrap_or_else(|reason| panic!("{SOURCE}: {reason}"));
    lx::module_tex(ORACLE, &[], declarations)
}

/// The patient values of a reasoning fixture's argument, for a test.
#[must_use]
pub fn fixture_vitals(name: &str) -> Option<[u64; 6]> {
    match name {
        "reasoning-triage-shock" | "reasoning-review-exhausted" | "reasoning-triage-traced" => {
            Some(SHOCKED)
        }
        "reasoning-triage-well" => Some(WELL),
        "reasoning-review-rejected" => Some(FEBRILE),
        _ => None,
    }
}
