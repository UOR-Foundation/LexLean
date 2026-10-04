//! Calculus transcriptions of the forward engine the reasoning example's
//! clinical module elaborates to (SPEC.md §17.12, §17.14).
//!
//! The `compiler` project's oracle modules state exactly the declarations
//! of the reasoning example's reasoners: `ReasoningOracle` those of
//! `examples/reasoning/src/Clinic.lex.tex`, so its `Triage` and `Review`
//! reasoners elaborate, in Lean, to the same select, fire, saturate,
//! conclude, and verdict definitions the example verifies; `GradeOracle`
//! the `Grade` reasoner of `Main` over the clinical rules; and
//! `BudgetOracle`, `PlannerOracle`, and `ScreeningOracle` the other three
//! modules, with the generic forward reasoner `Spend`, the breadth-first
//! search `Plan`, the depth-first search `Screen`, and the
//! generate-and-verify reasoner `Dose`. Each fixture here transcribes one of
//! those elaborations by hand, function for function: the rules' guards,
//! conclusions, and guarded applications, the first-applicable selection in
//! declared priority order, the firing by step, the fuel-bounded saturation
//! through the `iterate_until` template, the nodes, frontier, and visited
//! set of a search, the budgeted fold of a generator's candidates, and the
//! verifier-checked conclusion. The fixture module states that the kernel
//! reduces each transcription to the value the oracle's own reasoner
//! computes on the same input, so a transcription that searches
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

mod engines;

/// The module stating that each reasoning fixture computes what its
/// oracle's reasoner does.
pub const FIXTURES: &str = "ReasoningFixtures";

/// The module stating the clinical declarations.
pub const ORACLE: &str = "ReasoningOracle";
/// The module stating the `Grade` reasoner over the clinical rules.
pub const GRADE: &str = "GradeOracle";
/// The module stating the budget declarations.
pub const BUDGET: &str = "BudgetOracle";
/// The module stating the jug-planning declarations.
pub const PLANNER: &str = "PlannerOracle";
/// The module stating the dose-screening declarations.
pub const SCREENING: &str = "ScreeningOracle";

/// The oracle modules, in the order each may import the ones before it.
pub const ORACLE_MODULES: [&str; 5] = [ORACLE, GRADE, BUDGET, PLANNER, SCREENING];

/// The example source whose declarations the oracle states.
pub const SOURCE: &str = "examples/reasoning/src/Clinic.lex.tex";

/// An oracle module: the example module it states, the declarations of it
/// it states (all of them when none are named), and the module it imports.
pub struct OracleSpec {
    /// The oracle's module name.
    pub module: &'static str,
    /// The example source, relative to the repository root.
    pub source: &'static str,
    /// The declarations, by name, or every declaration.
    pub select: Option<&'static [&'static str]>,
    /// The example module's name for the module the oracle imports, and
    /// the oracle's name for it.
    pub import: Option<(&'static str, &'static str)>,
}

/// The oracles, in [`ORACLE_MODULES`] order.
pub const ORACLES: [OracleSpec; 5] = [
    OracleSpec {
        module: ORACLE,
        source: SOURCE,
        select: None,
        import: None,
    },
    OracleSpec {
        module: GRADE,
        source: "examples/reasoning/src/Main.lex.tex",
        select: Some(&[
            "Graded",
            "gradedCheck",
            "graded_sound",
            "Scale",
            "grade_admitted",
            "grade_pending",
            "grade_correct",
            "Grade",
        ]),
        import: Some(("Clinic", ORACLE)),
    },
    OracleSpec {
        module: BUDGET,
        source: "examples/reasoning/src/Budget.lex.tex",
        select: None,
        import: None,
    },
    OracleSpec {
        module: PLANNER,
        source: "examples/reasoning/src/Planner.lex.tex",
        select: None,
        import: None,
    },
    OracleSpec {
        module: SCREENING,
        source: "examples/reasoning/src/Screening.lex.tex",
        select: None,
        import: None,
    },
];

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
    /// Whether the reasoner runs its verifier's check: a reasoner whose
    /// answer is proved correct (`answer_correct`) does not, so it has no
    /// check function.
    checked: bool,
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
        assert!(
            self.checked,
            "a reasoner without a check has no check function"
        );
        self.base + 36
    }
    /// The first function after the shared ones.
    fn end(self) -> u64 {
        self.base + 36 + u64::from(self.checked)
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
    /// No check: the answer is proved correct on every reached state.
    Erased,
}

impl Verifier {
    const fn checked(self) -> bool {
        !matches!(self, Self::Erased)
    }
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
                // The check's result is bound before it is tested, so the
                // rendered arm is not a bare `if` the Rust lints would
                // rewrite into a filter. A proved answer is used as
                // extracted.
                arm(
                    Shape::Some,
                    vec![2],
                    if layout.checked {
                        super::let_in(
                            3,
                            Ty::Bool,
                            call(layout.check(), vec![v(0), v(2)]),
                            cond(v(3), some(&Ty::Nat, v(2)), none(&Ty::Nat)),
                        )
                    } else {
                        some(&Ty::Nat, v(2))
                    },
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
        Verifier::Erased => {
            debug_assert_eq!(layout.end(), layout.base + out.len() as u64);
            return out;
        }
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
    oracle_in(ORACLE, name, Vec::new(), arguments)
}

/// A call of a declaration of oracle `module` at `type_arguments`.
fn oracle_in(module: &str, name: &str, type_arguments: Vec<Json>, arguments: Vec<Json>) -> Json {
    let function = lexlean::calculus::term::member(module, name);
    if type_arguments.is_empty() {
        lx::call(function, arguments)
    } else {
        lx::call_at(function, type_arguments, arguments)
    }
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
    let layout = Layout {
        base: 1,
        checked: verifier.checked(),
    };
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
    let layout = Layout {
        base: 4,
        checked: true,
    };
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
    out.libraries = vec![super::LibraryUse {
        template: Template::IterateUntil,
        types: vec![state],
        at,
    }];
    out.oracle = Some(oracle);
    out
}

/// The verdict of reasoner `reasoner` on `values`, encoded as a value.
fn verdict_oracle(module: &str, reasoner: &str, values: [u64; 6]) -> Json {
    encode_result(
        oracle_in(
            module,
            &format!("{reasoner}.verdict"),
            Vec::new(),
            vec![patient_term(values)],
        ),
        |level| lx::value("nat", vec![level]),
    )
}

/// The verdict of the reasoner `reasoner` of oracle `module` on `arguments`
/// at `type_arguments`, as the value of a `result nat reasoning_failure`.
fn nat_verdict(
    module: &str,
    reasoner: &str,
    type_arguments: Vec<Json>,
    arguments: Vec<Json>,
) -> Json {
    encode_result(
        oracle_in(
            module,
            &format!("{reasoner}.verdict"),
            type_arguments,
            arguments,
        ),
        |found| lx::value("nat", vec![found]),
    )
}

/// The verdict of `Plan` for `target`: the jugs, or why none.
fn plan_verdict(target: u64) -> Json {
    encode_result(
        oracle_in(PLANNER, "Plan.verdict", Vec::new(), vec![lx::nat(target)]),
        |jugs| {
            lx::value(
                "pair",
                vec![
                    lx::value("nat", vec![lx::first(jugs.clone())]),
                    lx::value("nat", vec![lx::second(jugs)]),
                ],
            )
        },
    )
}

/// The vital signs of the patient whose observed findings are `findings`,
/// one bit each (fever, tachycardia, tachypnea, leukocytosis, infection,
/// hypotension): a finding is the clinical test the rule base states.
fn vitals_of_findings(findings: u64) -> [u64; 6] {
    use crate::gnaf::reasoning as bits;
    let has = |bit: u64| findings >> bit & 1 == 1;
    [
        if has(bits::FEVER) { 392 } else { 368 },
        if has(bits::TACHYCARDIA) { 118 } else { 72 },
        if has(bits::TACHYPNEA) { 26 } else { 14 },
        if has(bits::LEUKOCYTOSIS) { 15 } else { 7 },
        if has(bits::HYPOTENSION) { 82 } else { 120 },
        u64::from(has(bits::INFECTION)),
    ]
}

/// The GNAF plans (SPEC.md §17.15) as fixtures: each plan, on every patient
/// of the request's domain, as the level `Triage` derives from the same
/// findings, observed as the vital signs that state them. The kernel decides
/// each agreement, so the plans pose the problem `Triage` solves.
fn gnaf_cases() -> Vec<Case> {
    use crate::gnaf::reasoning as plans;
    let verdicts = nat_tuple_oracle(
        plans::DOMAIN
            .iter()
            .map(|findings| {
                lx::value(
                    "nat",
                    vec![lx::matching(
                        oracle_call(
                            "Triage.verdict",
                            vec![patient_term(vitals_of_findings(*findings))],
                        ),
                        vec![
                            lx::branch(
                                lx::member("Result.ok"),
                                vec!["level".to_owned()],
                                lx::var("level"),
                            ),
                            lx::branch(
                                lx::member("Result.error"),
                                vec!["failure".to_owned()],
                                lx::nat(9),
                            ),
                        ],
                    )],
                )
            })
            .collect(),
    );
    let arguments: Vec<Value> = plans::DOMAIN
        .iter()
        .map(|findings| natv(*findings))
        .collect();
    // The entry runs the plan, function 1, on each patient in turn.
    let tuple_t = plans::DOMAIN[1..]
        .iter()
        .fold(Ty::Nat, |tail, _| pair_t(Ty::Nat, tail));
    let entry = || {
        let last = plans::DOMAIN.len() - 1;
        let body = (0..last)
            .rev()
            .fold(call(1, vec![v(last as u64)]), |tail, index| {
                build(
                    Shape::Pair,
                    (index..last).fold(Ty::Nat, |tail, _| pair_t(Ty::Nat, tail)),
                    vec![call(1, vec![v(index as u64)]), tail],
                )
            });
        function(vec![Ty::Nat; plans::DOMAIN.len()], tuple_t.clone(), body)
    };
    [
        ("reasoning-gnaf-priority", plans::forward_priority(1)),
        ("reasoning-gnaf-sweep", plans::forward_sweep()),
        ("reasoning-gnaf-goal", plans::goal_directed()),
    ]
    .into_iter()
    .map(|(name, plan)| {
        let mut out = super::case(
            name,
            super::program(Vec::new(), vec![entry(), plan]),
            0,
            arguments.clone(),
            10_000,
        );
        out.oracle = Some(verdicts.clone());
        out
    })
    .collect()
}

/// A right-nested tuple of values, as a pair chain.
fn nat_tuple_oracle(mut items: Vec<Json>) -> Json {
    let last = items.pop().expect("a nonempty tuple");
    items
        .into_iter()
        .rev()
        .fold(last, |tail, item| lx::value("pair", vec![item, tail]))
}

/// Every reasoning fixture.
pub fn cases() -> Vec<Case> {
    let verdict = |name: &str,
                   module: &str,
                   reasoner: &str,
                   fuel: u64,
                   verifier: Verifier,
                   values: [u64; 6]| {
        reasoning_case(
            name,
            false,
            verdict_functions(fuel, verifier),
            chart_t(),
            vec![patient(values)],
            verdict_oracle(module, reasoner, values),
        )
    };
    let account = oracle_call("Triage.account", vec![patient_term(SHOCKED)]);
    let ledger = |counter: &str| lx::value("nat", vec![lx::project(account.clone(), counter)]);
    let mut out = vec![
        verdict(
            "reasoning-triage-shock",
            ORACLE,
            "Triage",
            11,
            Verifier::Recommendation,
            SHOCKED,
        ),
        verdict(
            "reasoning-triage-well",
            ORACLE,
            "Triage",
            11,
            Verifier::Recommendation,
            WELL,
        ),
        verdict(
            "reasoning-review-exhausted",
            ORACLE,
            "Review",
            6,
            Verifier::Outpatient,
            SHOCKED,
        ),
        verdict(
            "reasoning-review-rejected",
            ORACLE,
            "Review",
            6,
            Verifier::Outpatient,
            FEBRILE,
        ),
        // Grade: the clinical rules again, answering the level as
        // extracted, since its correctness is proved.
        verdict(
            "reasoning-grade-shock",
            GRADE,
            "Grade",
            11,
            Verifier::Erased,
            SHOCKED,
        ),
        verdict(
            "reasoning-grade-well",
            GRADE,
            "Grade",
            11,
            Verifier::Erased,
            WELL,
        ),
        engines::spend_case(
            2,
            nat_verdict(
                BUDGET,
                "Spend",
                vec![lx::nat_t()],
                vec![lx::pair(lx::nat(2), lx::nat(2))],
            ),
        ),
        engines::plan_case(2, plan_verdict(2)),
        engines::plan_case(5, plan_verdict(5)),
        engines::screen_case(
            60,
            nat_verdict(SCREENING, "Screen", Vec::new(), vec![lx::nat(60)]),
        ),
        engines::screen_case(
            100,
            nat_verdict(SCREENING, "Screen", Vec::new(), vec![lx::nat(100)]),
        ),
        engines::screen_case(
            0,
            nat_verdict(SCREENING, "Screen", Vec::new(), vec![lx::nat(0)]),
        ),
        engines::dose_case(
            60,
            nat_verdict(SCREENING, "Dose", Vec::new(), vec![lx::nat(60)]),
        ),
        engines::dose_case(
            120,
            nat_verdict(SCREENING, "Dose", Vec::new(), vec![lx::nat(120)]),
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
    ];
    // The GNAF plans, tied to the oracle's level on the same findings.
    out.extend(gnaf_cases());
    out
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

/// `declarations` with every reference to module `from` made to module `to`,
/// and the definitions a proof unfolds in their key order again, which the
/// module's name is part of.
fn renamed_module(declarations: &mut Json, from: &str, to: &str) {
    match declarations {
        Json::Object(object) => {
            if object.get("module").is_some_and(|module| module == from) {
                object.insert("module".to_owned(), Json::String(to.to_owned()));
            }
            for value in object.values_mut() {
                renamed_module(value, from, to);
            }
            if let Some(Json::Array(definitions)) = object.get_mut("definitions") {
                let key = |member: &Json| match member["module"].as_str() {
                    Some(module) => format!("{module}::{}", member["name"].as_str().unwrap_or("")),
                    None => member["name"].as_str().unwrap_or("").to_owned(),
                };
                definitions.sort_by_key(key);
            }
        }
        Json::Array(items) => {
            for item in items {
                renamed_module(item, from, to);
            }
        }
        _ => {}
    }
}

/// The index of the transcribed function `name` of engine `engine`
/// (`spend`, `plan`, `screen`, or `dose`).
///
/// # Panics
///
/// Panics on an engine or function the transcriptions do not name.
#[must_use]
pub fn function_index(engine: &str, name: &str) -> usize {
    engines::function_index(engine, name)
}

/// The oracle modules, by name: each states exactly the declarations of the
/// example module it names (those it selects, when it selects), with the
/// module it imports under the oracle's name for it.
///
/// # Panics
///
/// Panics if an example source is missing, states no semantic data, or
/// lacks a declaration an oracle selects.
#[must_use]
pub fn oracle_modules() -> Vec<(&'static str, String)> {
    ORACLES
        .iter()
        .map(|spec| {
            let path = crate::support::repo_root().join(spec.source);
            let source =
                std::fs::read_to_string(path.as_std_path()).expect("the reasoning example");
            let mut declarations = crate::rust_packages::declarations(&source)
                .unwrap_or_else(|reason| panic!("{}: {reason}", spec.source));
            if let Some(names) = spec.select {
                declarations.retain(|declaration| {
                    declaration["name"]
                        .as_str()
                        .is_some_and(|name| names.contains(&name))
                });
                assert_eq!(
                    declarations.len(),
                    names.len(),
                    "{} states every declaration {} selects",
                    spec.source,
                    spec.module
                );
            }
            let mut declarations = Json::Array(declarations);
            let imports: Vec<&str> = spec.import.iter().map(|(_, oracle)| *oracle).collect();
            if let Some((from, to)) = spec.import {
                renamed_module(&mut declarations, from, to);
            }
            let Json::Array(declarations) = declarations else {
                unreachable!("an array")
            };
            (
                spec.module,
                lx::module_tex(spec.module, &imports, declarations),
            )
        })
        .collect()
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
