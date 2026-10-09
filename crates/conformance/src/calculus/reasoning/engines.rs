//! Calculus transcriptions of the other reasoning strategies the example
//! elaborates (SPEC.md §17.12, §17.14): the generic forward reasoner
//! `Spend` at a natural-number payload, the breadth-first search with
//! deduplication `Plan`, the depth-first search `Screen`, and the
//! generate-and-verify reasoner `Dose`.
//!
//! Each fixture transcribes the elaboration the compiler project's oracle
//! module computes, function for function: the rules' guards, conclusions,
//! and guarded applications; the binding candidates; the node, frontier,
//! and visited-set data of a search; the cut of the frontier to its bound
//! and the truncation it records; the budgeted left fold over a generator's
//! candidates; and the verifier-checked answer. Every collection step runs
//! through a realization-library instance (`list_fold`, `set_insert`,
//! `set_contains`, `iterate_until`), as production closures do. The kernel
//! decides that each transcription reduces to the value the oracle's own
//! reasoner computes on the same input, so a transcription that searches in
//! another order, forgets a visited state, or answers a verdict the reasoner
//! does not is refused by Lean.

use lexlean::calculus::library::Template;
use lexlean::calculus::{Expr, Function, Shape, Ty, Value};
use serde_json::Value as Json;

use super::{and, eq, failure, failure_t, le, lt, none, not, or, plus, some};
use crate::calculus::{
    arm, build, call, closure, cond, field, first, function, let_in, list_t, matching, nat, natv,
    opt_t, p, pair_t, res_t, second, v, Case, LibraryUse,
};

fn adt(index: u64) -> Ty {
    Ty::Adt { index }
}
fn pair_nat() -> Ty {
    pair_t(Ty::Nat, Ty::Nat)
}
fn construct(constructor: u64, ty: &Ty, fields: Vec<Expr>) -> Expr {
    build(Shape::Adt { constructor }, ty.clone(), fields)
}
fn pair_of(ty: &Ty, left: Expr, right: Expr) -> Expr {
    build(Shape::Pair, ty.clone(), vec![left, right])
}
fn minus(left: Expr, right: Expr) -> Expr {
    p(lexlean::calculus::Prim::NatSub, vec![left, right])
}
fn times(left: Expr, right: Expr) -> Expr {
    p(lexlean::calculus::Prim::NatMul, vec![left, right])
}
fn append(left: Expr, right: Expr) -> Expr {
    p(lexlean::calculus::Prim::Append, vec![left, right])
}
fn length(list: Expr) -> Expr {
    p(lexlean::calculus::Prim::Length, vec![list])
}
fn nil(element: &Ty) -> Expr {
    build(Shape::Nil, list_t(element.clone()), Vec::new())
}
fn singleton(element: &Ty, item: Expr) -> Expr {
    build(
        Shape::Cons,
        list_t(element.clone()),
        vec![item, nil(element)],
    )
}
fn list_of(element: &Ty, items: Vec<Expr>) -> Expr {
    items.into_iter().rev().fold(nil(element), |tail, head| {
        build(Shape::Cons, list_t(element.clone()), vec![head, tail])
    })
}
/// The six counters of a ledger as a chain of pairs.
fn ledger_chain_t() -> Ty {
    (1..6).fold(Ty::Nat, |tail, _| pair_t(Ty::Nat, tail))
}
fn ledger_chain(counters: Vec<Expr>) -> Expr {
    let mut counters = counters;
    let last = counters.pop().expect("six counters");
    counters
        .into_iter()
        .enumerate()
        .rev()
        .fold(last, |tail, (index, counter)| {
            pair_of(
                &(index + 1..6).fold(Ty::Nat, |tail, _| pair_t(Ty::Nat, tail)),
                counter,
                tail,
            )
        })
}
fn verdict_t(ok: Ty) -> Ty {
    res_t(ok, failure_t())
}
fn ok(ok_t: &Ty, value: Expr) -> Expr {
    build(Shape::Ok, verdict_t(ok_t.clone()), vec![value])
}
fn error(ok_t: &Ty, value: Expr) -> Expr {
    build(Shape::Error, verdict_t(ok_t.clone()), vec![value])
}

/// The index of the transcribed function `name` of engine `engine`.
///
/// # Panics
///
/// Panics on an engine or function the transcriptions do not name.
pub fn function_index(engine: &str, name: &str) -> usize {
    let names: &[&str] = match engine {
        "spend" => &SPEND,
        "plan" => &PLAN,
        "screen" => &SCREEN,
        "dose" => &DOSE,
        other => panic!("no engine {other}"),
    };
    Plan { names }.id(name) as usize
}

/// The functions of a transcription, by name, then the library instances.
/// A function's index is its position in `names`; instance `i` follows
/// them.
struct Plan<'a> {
    names: &'a [&'a str],
}

impl Plan<'_> {
    fn id(&self, name: &str) -> u64 {
        self.names
            .iter()
            .position(|candidate| *candidate == name)
            .unwrap_or_else(|| panic!("a transcribed function {name}")) as u64
    }
    fn lib(&self, index: u64) -> u64 {
        self.names.len() as u64 + index
    }
}

/// A fixture from a transcription: its functions in `plan.names` order,
/// its library instances after them, and the oracle's value on the same
/// input.
#[allow(clippy::too_many_arguments)]
fn engine_case(
    name: &str,
    adts: Vec<Vec<Vec<Ty>>>,
    plan: &Plan<'_>,
    functions: Vec<Function>,
    libraries: Vec<(Template, Vec<Ty>)>,
    arguments: Vec<Value>,
    fuel: u64,
    oracle: Json,
) -> Case {
    assert_eq!(
        functions.len(),
        plan.names.len(),
        "fixture {name}: every named function is defined"
    );
    let mut all = functions;
    let mut uses = Vec::new();
    for (template, types) in libraries {
        let at = all.len() as u64;
        let instance = template
            .instantiate(&types, at)
            .unwrap_or_else(|reason| panic!("fixture {name}: {reason}"));
        assert_eq!(
            instance.len(),
            1,
            "fixture {name}: one function an instance"
        );
        all.extend(instance);
        uses.push(LibraryUse {
            template,
            types,
            at,
        });
    }
    let mut out = crate::calculus::case(
        name,
        crate::calculus::program(adts, all),
        0,
        arguments,
        fuel,
    );
    out.libraries = uses;
    out.oracle = Some(oracle);
    out
}

// ---------------------------------------------------------------------------
// Spend: the generic countdown reasoner, at a natural-number payload.

const SPEND: [&str; 17] = [
    "verdict",
    "observe",
    "tick_guard",
    "tick_concl",
    "tick_apply",
    "burn_guard",
    "burn_concl",
    "burn_candidates",
    "burn_apply",
    "select_fold",
    "select",
    "fire",
    "next",
    "extract",
    "empty_check",
    "accept",
    "conclude",
];

/// `Spend.verdict` at `T = Nat`: the forward engine over a rule without a
/// binding (`Tick`) and a rule over the candidates 2 and 1 (`Burn`),
/// answering what remains of the budget when the verifier `Drained` accepts
/// it.
pub fn spend_case(x: u64, oracle: Json) -> Case {
    let plan = Plan { names: &SPEND };
    let id = |name: &str| plan.id(name);
    let state = pair_nat();
    let step = adt(0);
    let steps_option = opt_t(step.clone());
    let verdict = verdict_t(Ty::Nat);
    let fold = plan.lib(0);
    let iterate = plan.lib(1);
    let step_of = |constructor: u64, fields: Vec<Expr>| construct(constructor, &step, fields);
    let functions = vec![
        // verdict: saturate under the fuel, then conclude, or exhausted.
        function(
            vec![state.clone()],
            verdict.clone(),
            let_in(
                1,
                pair_t(state.clone(), Ty::Bool),
                call(
                    iterate,
                    vec![
                        closure(id("next"), Vec::new()),
                        plus(second(v(0)), nat(4)),
                        call(id("observe"), vec![v(0)]),
                    ],
                ),
                cond(
                    second(v(1)),
                    call(id("conclude"), vec![v(0), first(v(1))]),
                    error(&Ty::Nat, failure(false, false)),
                ),
            ),
        ),
        // observe: the budget opened with its allowance.
        function(
            vec![state.clone()],
            state.clone(),
            pair_of(&state, first(v(0)), plus(second(v(0)), nat(3))),
        ),
        function(vec![state.clone()], Ty::Bool, lt(nat(2), second(v(0)))),
        function(
            vec![state.clone()],
            state.clone(),
            pair_of(&state, first(v(0)), minus(second(v(0)), nat(1))),
        ),
        function(
            vec![state.clone()],
            opt_t(state.clone()),
            cond(
                call(id("tick_guard"), vec![v(0)]),
                some(&state, call(id("tick_concl"), vec![v(0)])),
                none(&state),
            ),
        ),
        function(
            vec![state.clone(), Ty::Nat],
            Ty::Bool,
            and(le(v(1), second(v(0))), lt(nat(0), v(1))),
        ),
        function(
            vec![state.clone(), Ty::Nat],
            state.clone(),
            pair_of(&state, first(v(0)), minus(second(v(0)), v(1))),
        ),
        function(
            vec![state.clone()],
            list_t(Ty::Nat),
            list_of(&Ty::Nat, vec![nat(2), nat(1)]),
        ),
        function(
            vec![state.clone(), Ty::Nat],
            opt_t(state.clone()),
            cond(
                call(id("burn_guard"), vec![v(0), v(1)]),
                some(&state, call(id("burn_concl"), vec![v(0), v(1)])),
                none(&state),
            ),
        ),
        // select_fold: the first candidate whose guard holds.
        function(
            vec![state.clone(), steps_option.clone(), Ty::Nat],
            steps_option.clone(),
            matching(
                steps_option.clone(),
                v(1),
                vec![
                    arm(Shape::Some, vec![3], some(&step, v(3))),
                    arm(
                        Shape::None,
                        Vec::new(),
                        cond(
                            call(id("burn_guard"), vec![v(0), v(2)]),
                            some(&step, step_of(1, vec![v(2)])),
                            none(&step),
                        ),
                    ),
                ],
            ),
        ),
        // select: the first applicable rule in declared order.
        function(
            vec![state.clone()],
            steps_option.clone(),
            matching(
                steps_option.clone(),
                cond(
                    call(id("tick_guard"), vec![v(0)]),
                    some(&step, step_of(0, Vec::new())),
                    none(&step),
                ),
                vec![
                    arm(Shape::Some, vec![1], some(&step, v(1))),
                    arm(
                        Shape::None,
                        Vec::new(),
                        call(
                            fold,
                            vec![
                                closure(id("select_fold"), vec![v(0)]),
                                none(&step),
                                call(id("burn_candidates"), vec![v(0)]),
                            ],
                        ),
                    ),
                ],
            ),
        ),
        // fire: the step's guarded application.
        function(
            vec![state.clone(), step.clone()],
            opt_t(state.clone()),
            matching(
                opt_t(state.clone()),
                v(1),
                vec![
                    arm(
                        Shape::Adt { constructor: 0 },
                        Vec::new(),
                        call(id("tick_apply"), vec![v(0)]),
                    ),
                    arm(
                        Shape::Adt { constructor: 1 },
                        vec![2],
                        call(id("burn_apply"), vec![v(0), v(2)]),
                    ),
                ],
            ),
        ),
        // next: fire the selected rule.
        function(
            vec![state.clone()],
            opt_t(state.clone()),
            matching(
                opt_t(state.clone()),
                call(id("select"), vec![v(0)]),
                vec![
                    arm(Shape::None, Vec::new(), none(&state)),
                    arm(Shape::Some, vec![1], call(id("fire"), vec![v(0), v(1)])),
                ],
            ),
        ),
        // extract: what remains of the budget.
        function(
            vec![state.clone()],
            opt_t(Ty::Nat),
            some(&Ty::Nat, second(v(0))),
        ),
        // The verifier `Drained`: nothing remains.
        function(vec![state.clone(), Ty::Nat], Ty::Bool, eq(v(1), nat(0))),
        // accept: the extracted answer if the verifier's check accepts it.
        function(
            vec![state.clone(), state.clone()],
            opt_t(Ty::Nat),
            matching(
                opt_t(Ty::Nat),
                call(id("extract"), vec![v(1)]),
                vec![
                    arm(Shape::None, Vec::new(), none(&Ty::Nat)),
                    arm(
                        Shape::Some,
                        vec![2],
                        let_in(
                            3,
                            Ty::Bool,
                            call(id("empty_check"), vec![v(0), v(2)]),
                            cond(v(3), some(&Ty::Nat, v(2)), none(&Ty::Nat)),
                        ),
                    ),
                ],
            ),
        ),
        // conclude: the accepted answer, or rejected, or unsolved.
        function(
            vec![state.clone(), state.clone()],
            verdict.clone(),
            matching(
                verdict.clone(),
                call(id("accept"), vec![v(0), v(1)]),
                vec![
                    arm(Shape::Some, vec![2], ok(&Ty::Nat, v(2))),
                    arm(
                        Shape::None,
                        Vec::new(),
                        matching(
                            verdict.clone(),
                            call(id("extract"), vec![v(1)]),
                            vec![
                                arm(Shape::Some, vec![3], error(&Ty::Nat, failure(true, false))),
                                arm(
                                    Shape::None,
                                    Vec::new(),
                                    error(&Ty::Nat, failure(false, true)),
                                ),
                            ],
                        ),
                    ),
                ],
            ),
        ),
    ];
    engine_case(
        &format!("reasoning-spend-{x}"),
        vec![vec![Vec::new(), vec![Ty::Nat]]],
        &plan,
        functions,
        vec![
            (Template::ListFold, vec![Ty::Nat, steps_option]),
            (Template::IterateUntil, vec![state.clone()]),
        ],
        vec![Value::Pair {
            left: Box::new(natv(x)),
            right: Box::new(natv(x)),
        }],
        4000,
        oracle,
    )
}

// ---------------------------------------------------------------------------
// The searches: nodes, a frontier cut to its bound, and the verdict of the
// found node, or why none was found.

/// The data shared by the two searches: a node is a state and the steps
/// that led to it; a search is its frontier, its found answer and node,
/// and whether the frontier was ever cut. A search with deduplication
/// carries its visited states too.
struct Searching {
    step: Ty,
    node: Ty,
    search: Ty,
    nodes: Ty,
    state: Ty,
    answer: Ty,
    deduplicate: bool,
}

impl Searching {
    fn new(deduplicate: bool, answer: Ty) -> Self {
        let node = adt(1);
        Self {
            step: adt(0),
            nodes: list_t(node.clone()),
            node,
            search: adt(2),
            state: pair_nat(),
            answer,
            deduplicate,
        }
    }
    fn hit(&self) -> Ty {
        pair_t(self.answer.clone(), self.node.clone())
    }
    /// The ledger of a run: its six counters, as the elaboration states them.
    fn ledger(&self) -> Ty {
        adt(3)
    }
    fn ledger_at(&self) -> u64 {
        self.found_at() + 2
    }
    /// The counter `index` of the ledger of search `search`.
    fn counter(&self, search: Expr, index: u64) -> Expr {
        field(field(search, self.ledger_at()), index)
    }
    /// A ledger from its counters.
    fn ledger_of(&self, counters: [Expr; 6]) -> Expr {
        construct(0, &self.ledger(), counters.into())
    }
    /// The ledger of `search` with the counters `changed` replaced.
    fn changed(&self, search: &Expr, changed: [Option<Expr>; 6]) -> Expr {
        let counters: Vec<Expr> = changed
            .into_iter()
            .enumerate()
            .map(|(index, new)| new.unwrap_or_else(|| self.counter(search.clone(), index as u64)))
            .collect();
        self.ledger_of(
            counters
                .try_into()
                .unwrap_or_else(|_| panic!("six counters")),
        )
    }
    /// What the transcription answers: the explained verdict, and the ledger
    /// of the run as a chain of six pairs.
    fn entry_type(&self) -> Ty {
        pair_t(
            res_t(
                pair_t(self.answer.clone(), list_t(self.step.clone())),
                failure_t(),
            ),
            ledger_chain_t(),
        )
    }
    /// The entry's body, over variable `fin`: the finished search and
    /// whether it saturated.
    fn explained(&self, fin: u64) -> Expr {
        let explained_t = res_t(
            pair_t(self.answer.clone(), list_t(self.step.clone())),
            failure_t(),
        );
        let finished = || first(v(fin));
        let counters = (0..6)
            .map(|index| self.counter(finished(), index))
            .collect();
        pair_of(
            &self.entry_type(),
            matching(
                explained_t.clone(),
                field(finished(), self.found_at()),
                vec![
                    arm(
                        Shape::Some,
                        vec![fin + 1],
                        build(
                            Shape::Ok,
                            explained_t.clone(),
                            vec![pair_of(
                                &pair_t(self.answer.clone(), list_t(self.step.clone())),
                                first(v(fin + 1)),
                                field(second(v(fin + 1)), 1),
                            )],
                        ),
                    ),
                    arm(
                        Shape::None,
                        Vec::new(),
                        build(
                            Shape::Error,
                            explained_t,
                            vec![self
                                .failure(second(v(fin)), field(finished(), self.truncated_at()))],
                        ),
                    ),
                ],
            ),
            ledger_chain(counters),
        )
    }
    /// The ADTs: the steps, the nodes, and the search.
    fn adts(&self, steps: Vec<Vec<Ty>>) -> Vec<Vec<Vec<Ty>>> {
        let mut search = vec![self.nodes.clone()];
        if self.deduplicate {
            search.push(list_t(self.state.clone()));
        }
        search.push(opt_t(self.hit()));
        search.push(Ty::Bool);
        search.push(self.ledger());
        vec![
            steps,
            vec![vec![self.state.clone(), list_t(self.step.clone())]],
            vec![search],
            vec![vec![Ty::Nat; 6]],
        ]
    }
    fn node_of(&self, state: Expr, trace: Expr) -> Expr {
        construct(0, &self.node, vec![state, trace])
    }
    /// The trace of `node` with `step` appended.
    fn extended(&self, node: Expr, step: Expr) -> Expr {
        append(field(node, 1), singleton(&self.step, step))
    }
    fn search_of(
        &self,
        frontier: Expr,
        visited: Option<Expr>,
        found: Expr,
        truncated: Expr,
        ledger: Expr,
    ) -> Expr {
        let mut fields = vec![frontier];
        fields.extend(visited);
        fields.push(found);
        fields.push(truncated);
        fields.push(ledger);
        construct(0, &self.search, fields)
    }
    /// The index of the `found` field.
    fn found_at(&self) -> u64 {
        if self.deduplicate {
            2
        } else {
            1
        }
    }
    fn truncated_at(&self) -> u64 {
        self.found_at() + 1
    }
    /// The frontier cut to its bound: a fold that keeps the first `cap`
    /// nodes.
    fn cap_step(&self) -> Function {
        function(
            vec![Ty::Nat, self.nodes.clone(), self.node.clone()],
            self.nodes.clone(),
            cond(
                lt(length(v(1)), v(0)),
                append(v(1), singleton(&self.node, v(2))),
                v(1),
            ),
        )
    }
    /// `failure saturated truncated`: a saturated search that never cut its
    /// frontier is unsolved, any other without an answer is exhausted.
    fn failure(&self, saturated: Expr, truncated: Expr) -> Expr {
        cond(
            and(saturated, not(truncated)),
            failure(false, true),
            failure(false, false),
        )
    }
}

const PLAN: [&str; 27] = [
    "verdict",
    "fill_guard",
    "fill_concl",
    "fill_apply",
    "empty_guard",
    "empty_concl",
    "empty_apply",
    "pour_guard",
    "pour_concl",
    "pour_candidates",
    "pour_apply",
    "fire",
    "observe",
    "extract",
    "measures_check",
    "accept",
    "fill_succ",
    "empty_succ",
    "pour_collect",
    "pour_succ",
    "successors",
    "fresh_step",
    "fresh",
    "cap_step",
    "start",
    "search_step",
    "attempts",
];

/// `Plan.verdict`: breadth-first search with deduplication over two jugs,
/// pouring by the candidates 1, 2, and 3, for a target level.
#[allow(clippy::too_many_lines)]
pub fn plan_case(target: u64, oracle: Json) -> Case {
    let plan = Plan { names: &PLAN };
    let id = |name: &str| plan.id(name);
    let s = Searching::new(true, pair_nat());
    let jugs = s.state.clone();
    let step = s.step.clone();
    let node = s.node.clone();
    let nodes = s.nodes.clone();
    let search = s.search.clone();
    let visited = list_t(jugs.clone());
    let fresh_t = pair_t(nodes.clone(), visited.clone());
    let fold_cap = plan.lib(0);
    let fold_collect = plan.lib(1);
    let fold_fresh = plan.lib(2);
    let contains = plan.lib(3);
    let insert = plan.lib(4);
    let iterate = plan.lib(5);
    let step_of = |constructor: u64, fields: Vec<Expr>| construct(constructor, &step, fields);
    let apply_of = |guard: &str, conclusion: &str, binding: bool| {
        let mut parameters = vec![jugs.clone()];
        let arguments: Vec<Expr> = if binding {
            parameters.push(Ty::Nat);
            vec![v(0), v(1)]
        } else {
            vec![v(0)]
        };
        function(
            parameters,
            opt_t(jugs.clone()),
            cond(
                call(id(guard), arguments.clone()),
                some(&jugs, call(id(conclusion), arguments)),
                none(&jugs),
            ),
        )
    };
    let functions = vec![
        // Plan: the search under its fuel, answering the found state and the
        // steps that reach it, or why none, with the run's ledger.
        function(
            vec![Ty::Nat],
            s.entry_type(),
            let_in(
                1,
                pair_t(search.clone(), Ty::Bool),
                call(
                    iterate,
                    vec![
                        closure(id("search_step"), vec![v(0)]),
                        times(v(0), nat(8)),
                        call(id("start"), vec![v(0)]),
                    ],
                ),
                s.explained(1),
            ),
        ),
        // FillA: fill the first jug.
        function(
            vec![jugs.clone()],
            Ty::Bool,
            and(lt(first(v(0)), nat(4)), le(second(v(0)), nat(3))),
        ),
        function(
            vec![jugs.clone()],
            jugs.clone(),
            pair_of(&jugs, nat(4), second(v(0))),
        ),
        apply_of("fill_guard", "fill_concl", false),
        // EmptyB: empty the second jug.
        function(
            vec![jugs.clone()],
            Ty::Bool,
            and(lt(nat(0), second(v(0))), le(first(v(0)), nat(4))),
        ),
        function(
            vec![jugs.clone()],
            jugs.clone(),
            pair_of(&jugs, first(v(0)), nat(0)),
        ),
        apply_of("empty_guard", "empty_concl", false),
        // Pour k: pour k from the first jug into the second.
        function(
            vec![jugs.clone(), Ty::Nat],
            Ty::Bool,
            and(
                le(v(1), first(v(0))),
                and(
                    le(plus(second(v(0)), v(1)), nat(3)),
                    le(first(v(0)), nat(4)),
                ),
            ),
        ),
        function(
            vec![jugs.clone(), Ty::Nat],
            jugs.clone(),
            pair_of(&jugs, minus(first(v(0)), v(1)), plus(second(v(0)), v(1))),
        ),
        function(
            vec![jugs.clone()],
            list_t(Ty::Nat),
            list_of(&Ty::Nat, vec![nat(1), nat(2), nat(3)]),
        ),
        apply_of("pour_guard", "pour_concl", true),
        // fire: the step's guarded application.
        function(
            vec![jugs.clone(), step.clone()],
            opt_t(jugs.clone()),
            matching(
                opt_t(jugs.clone()),
                v(1),
                vec![
                    arm(
                        Shape::Adt { constructor: 0 },
                        Vec::new(),
                        call(id("fill_apply"), vec![v(0)]),
                    ),
                    arm(
                        Shape::Adt { constructor: 1 },
                        Vec::new(),
                        call(id("empty_apply"), vec![v(0)]),
                    ),
                    arm(
                        Shape::Adt { constructor: 2 },
                        vec![2],
                        call(id("pour_apply"), vec![v(0), v(2)]),
                    ),
                ],
            ),
        ),
        // observe: both jugs empty.
        function(vec![Ty::Nat], jugs.clone(), pair_of(&jugs, nat(0), nat(0))),
        // extract: the jugs themselves.
        function(vec![jugs.clone()], opt_t(jugs.clone()), some(&jugs, v(0))),
        // The verifier `Measured`: the second jug holds the target.
        function(
            vec![Ty::Nat, jugs.clone()],
            Ty::Bool,
            and(eq(second(v(1)), v(0)), le(first(v(1)), nat(4))),
        ),
        function(
            vec![Ty::Nat, jugs.clone()],
            opt_t(jugs.clone()),
            matching(
                opt_t(jugs.clone()),
                call(id("extract"), vec![v(1)]),
                vec![
                    arm(Shape::None, Vec::new(), none(&jugs)),
                    arm(
                        Shape::Some,
                        vec![2],
                        let_in(
                            3,
                            Ty::Bool,
                            call(id("measures_check"), vec![v(0), v(2)]),
                            cond(v(3), some(&jugs, v(2)), none(&jugs)),
                        ),
                    ),
                ],
            ),
        ),
        // fill_succ, empty_succ: the successor of a node by a rule without
        // a binding, if it fires.
        function(
            vec![node.clone()],
            nodes.clone(),
            matching(
                nodes.clone(),
                call(id("fire"), vec![field(v(0), 0), step_of(0, Vec::new())]),
                vec![
                    arm(Shape::None, Vec::new(), nil(&node)),
                    arm(
                        Shape::Some,
                        vec![1],
                        singleton(
                            &node,
                            s.node_of(v(1), s.extended(v(0), step_of(0, Vec::new()))),
                        ),
                    ),
                ],
            ),
        ),
        function(
            vec![node.clone()],
            nodes.clone(),
            matching(
                nodes.clone(),
                call(id("fire"), vec![field(v(0), 0), step_of(1, Vec::new())]),
                vec![
                    arm(Shape::None, Vec::new(), nil(&node)),
                    arm(
                        Shape::Some,
                        vec![1],
                        singleton(
                            &node,
                            s.node_of(v(1), s.extended(v(0), step_of(1, Vec::new()))),
                        ),
                    ),
                ],
            ),
        ),
        // pour_collect: the fold step over the candidates of the binding.
        function(
            vec![node.clone(), nodes.clone(), Ty::Nat],
            nodes.clone(),
            matching(
                nodes.clone(),
                call(id("fire"), vec![field(v(0), 0), step_of(2, vec![v(2)])]),
                vec![
                    arm(Shape::None, Vec::new(), v(1)),
                    arm(
                        Shape::Some,
                        vec![3],
                        append(
                            v(1),
                            singleton(
                                &node,
                                s.node_of(v(3), s.extended(v(0), step_of(2, vec![v(2)]))),
                            ),
                        ),
                    ),
                ],
            ),
        ),
        function(
            vec![node.clone()],
            nodes.clone(),
            call(
                fold_collect,
                vec![
                    closure(id("pour_collect"), vec![v(0)]),
                    nil(&node),
                    call(id("pour_candidates"), vec![field(v(0), 0)]),
                ],
            ),
        ),
        // successors: every rule's, in declared order.
        function(
            vec![node.clone()],
            nodes.clone(),
            append(
                call(id("fill_succ"), vec![v(0)]),
                append(
                    call(id("empty_succ"), vec![v(0)]),
                    call(id("pour_succ"), vec![v(0)]),
                ),
            ),
        ),
        // fresh: the nodes whose state was not visited, and the visited
        // states with theirs.
        function(
            vec![fresh_t.clone(), node.clone()],
            fresh_t.clone(),
            cond(
                call(contains, vec![second(v(0)), field(v(1), 0)]),
                v(0),
                pair_of(
                    &fresh_t,
                    append(first(v(0)), singleton(&node, v(1))),
                    call(insert, vec![second(v(0)), field(v(1), 0)]),
                ),
            ),
        ),
        function(
            vec![visited.clone(), nodes.clone()],
            fresh_t.clone(),
            call(
                fold_fresh,
                vec![
                    closure(id("fresh_step"), Vec::new()),
                    pair_of(&fresh_t, nil(&node), v(0)),
                    v(1),
                ],
            ),
        ),
        s.cap_step(),
        // start: the observed root, within the frontier bound.
        function(
            vec![Ty::Nat],
            search.clone(),
            let_in(
                1,
                Ty::Nat,
                plus(v(0), v(0)),
                let_in(
                    2,
                    nodes.clone(),
                    call(
                        fold_cap,
                        vec![
                            closure(id("cap_step"), vec![v(1)]),
                            nil(&node),
                            singleton(
                                &node,
                                s.node_of(call(id("observe"), vec![v(0)]), nil(&step)),
                            ),
                        ],
                    ),
                    s.search_of(
                        v(2),
                        Some(call(
                            insert,
                            vec![nil(&jugs), call(id("observe"), vec![v(0)])],
                        )),
                        none(&s.hit()),
                        lt(v(1), nat(1)),
                    s.ledger_of([nat(0), nat(0), nat(0), nat(0), nat(0), length(v(2))]),
),
                ),
            ),
        ),
        // search_step: pop the head node; answer it if accepted; else
        // expand it, append its fresh successors, and cut the frontier.
        function(
            vec![Ty::Nat, search.clone()],
            opt_t(search.clone()),
            matching(
                opt_t(search.clone()),
                field(v(1), s.found_at()),
                vec![
                    arm(Shape::Some, vec![2], none(&search)),
                    arm(
                        Shape::None,
                        Vec::new(),
                        matching(
                            opt_t(search.clone()),
                            field(v(1), 0),
                            vec![
                                arm(Shape::Nil, Vec::new(), none(&search)),
                                arm(
                                    Shape::Cons,
                                    vec![3, 4],
                                    matching(
                                        opt_t(search.clone()),
                                        call(id("accept"), vec![v(0), field(v(3), 0)]),
                                        vec![
                                            arm(
                                                Shape::Some,
                                                vec![5],
                                                some(
                                                    &search,
                                                    s.search_of(
                                                        v(4),
                                                        Some(field(v(1), 1)),
                                                        some(
                                                            &s.hit(),
                                                            pair_of(&s.hit(), v(5), v(3)),
                                                        ),
                                                        field(v(1), s.truncated_at()),
                                                    s.changed(&v(1), [Some(plus(s.counter(v(1), 0), nat(1))), None, None, None, Some(plus(s.counter(v(1), 4), nat(1))), None]),
),
                                                ),
                                            ),
                                            arm(
                                                Shape::None,
                                                Vec::new(),
                                                let_in(
                                                    6,
                                                    nodes.clone(),
                                                    call(id("successors"), vec![v(3)]),
                                                    let_in(
                                                        7,
                                                        fresh_t.clone(),
                                                        call(
                                                            id("fresh"),
                                                            vec![field(v(1), 1), v(6)],
                                                        ),
                                                        let_in(
                                                            8,
                                                            nodes.clone(),
                                                            append(v(4), first(v(7))),
                                                            let_in(
                                                                9,
                                                                nodes.clone(),
                                                                call(
                                                                    fold_cap,
                                                                    vec![
                                                                        closure(
                                                                            id("cap_step"),
                                                                            vec![plus(v(0), v(0))],
                                                                        ),
                                                                        nil(&node),
                                                                        v(8),
                                                                    ],
                                                                ),
                                                                some(
                                                                    &search,
                                                                    s.search_of(
                                                                        v(9),
                                                                        Some(second(v(7))),
                                                                        none(&s.hit()),
                                                                        or(
                                                                            field(
                                                                                v(1),
                                                                                s.truncated_at(),
                                                                            ),
                                                                            lt(
                                                                                plus(v(0), v(0)),
                                                                                length(v(8)),
                                                                            ),
                                                                        ),
                                                                    s.changed(&v(1), [Some(plus(s.counter(v(1), 0), nat(1))), Some(plus(s.counter(v(1), 1), call(id("attempts"), vec![field(v(3), 0)]))), Some(plus(s.counter(v(1), 2), length(v(6)))), Some(plus(s.counter(v(1), 3), nat(1))), Some(plus(s.counter(v(1), 4), nat(1))), Some(cond(lt(s.counter(v(1), 5), length(v(9))), length(v(9)), s.counter(v(1), 5)))]),
),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                ),
                            ],
                        ),
                    ),
                ],
            ),
        ),
        // attempts: the guard evaluations an expansion of a state makes, one
        // for each rule without a binding and one for each candidate of a
        // rule with one.
        function(
            vec![jugs.clone()],
            Ty::Nat,
            plus(
                nat(1),
                plus(nat(1), length(call(id("pour_candidates"), vec![v(0)]))),
            ),
        ),
    ];
    engine_case(
        &format!("reasoning-plan-{target}"),
        s.adts(vec![Vec::new(), Vec::new(), vec![Ty::Nat]]),
        &plan,
        functions,
        vec![
            (Template::ListFold, vec![node.clone(), nodes.clone()]),
            (Template::ListFold, vec![Ty::Nat, nodes.clone()]),
            (Template::ListFold, vec![node.clone(), fresh_t]),
            (Template::SetContains, vec![jugs.clone()]),
            (Template::SetInsert, vec![jugs.clone()]),
            (Template::IterateUntil, vec![search]),
        ],
        vec![natv(target)],
        100_000,
        oracle,
    )
}

const SCREEN: [&str; 17] = [
    "verdict",
    "propose_guard",
    "propose_concl",
    "propose_apply",
    "dose_candidates",
    "propose_candidates",
    "fire",
    "observe",
    "extract",
    "safe_check",
    "accept",
    "propose_collect",
    "successors",
    "cap_step",
    "start",
    "search_step",
    "attempts",
];

/// `Dose.candidates`: the dose model's proposals for a weight, through its
/// contract's postcondition check: a list of at most four doses, or none.
fn dose_candidates() -> Function {
    let doses = || {
        list_of(
            &Ty::Nat,
            vec![
                minus(v(0), nat(10)),
                minus(v(0), nat(20)),
                minus(v(0), nat(30)),
                minus(v(0), nat(50)),
            ],
        )
    };
    function(
        vec![Ty::Nat],
        list_t(Ty::Nat),
        let_in(
            1,
            list_t(Ty::Nat),
            doses(),
            cond(le(length(v(1)), nat(4)), v(1), nil(&Ty::Nat)),
        ),
    )
}

/// The verifier `SafeDose`: twice the dose is within the weight.
fn safe_check() -> Function {
    function(vec![Ty::Nat, Ty::Nat], Ty::Bool, le(plus(v(1), v(1)), v(0)))
}

/// `Screen.verdict`: depth-first search over the doses the model proposes,
/// one rule with a binding, a frontier of three, and eight iterations.
#[allow(clippy::too_many_lines)]
pub fn screen_case(weight: u64, oracle: Json) -> Case {
    let plan = Plan { names: &SCREEN };
    let id = |name: &str| plan.id(name);
    let s = Searching::new(false, Ty::Nat);
    let pick = s.state.clone();
    let step = s.step.clone();
    let node = s.node.clone();
    let nodes = s.nodes.clone();
    let search = s.search.clone();
    let fold_cap = plan.lib(0);
    let fold_collect = plan.lib(1);
    let iterate = plan.lib(2);
    let step_of = |dose: Expr| construct(0, &step, vec![dose]);
    let functions = vec![
        function(
            vec![Ty::Nat],
            s.entry_type(),
            let_in(
                1,
                pair_t(search.clone(), Ty::Bool),
                call(
                    iterate,
                    vec![
                        closure(id("search_step"), vec![v(0)]),
                        nat(8),
                        call(id("start"), vec![v(0)]),
                    ],
                ),
                s.explained(1),
            ),
        ),
        // Propose d: nothing proposed yet, and a positive dose.
        function(
            vec![pick.clone(), Ty::Nat],
            Ty::Bool,
            and(eq(second(v(0)), nat(0)), lt(nat(0), v(1))),
        ),
        function(
            vec![pick.clone(), Ty::Nat],
            pick.clone(),
            pair_of(&pick, first(v(0)), v(1)),
        ),
        function(
            vec![pick.clone(), Ty::Nat],
            opt_t(pick.clone()),
            cond(
                call(id("propose_guard"), vec![v(0), v(1)]),
                some(&pick, call(id("propose_concl"), vec![v(0), v(1)])),
                none(&pick),
            ),
        ),
        dose_candidates(),
        // The binding's candidates: the model's proposals for the weight.
        function(
            vec![pick.clone()],
            list_t(Ty::Nat),
            call(id("dose_candidates"), vec![first(v(0))]),
        ),
        function(
            vec![pick.clone(), step.clone()],
            opt_t(pick.clone()),
            matching(
                opt_t(pick.clone()),
                v(1),
                vec![arm(
                    Shape::Adt { constructor: 0 },
                    vec![2],
                    call(id("propose_apply"), vec![v(0), v(2)]),
                )],
            ),
        ),
        function(vec![Ty::Nat], pick.clone(), pair_of(&pick, v(0), nat(0))),
        // extract: the proposed dose, if there is one.
        function(
            vec![pick.clone()],
            opt_t(Ty::Nat),
            cond(
                lt(nat(0), second(v(0))),
                some(&Ty::Nat, second(v(0))),
                none(&Ty::Nat),
            ),
        ),
        safe_check(),
        function(
            vec![Ty::Nat, pick.clone()],
            opt_t(Ty::Nat),
            matching(
                opt_t(Ty::Nat),
                call(id("extract"), vec![v(1)]),
                vec![
                    arm(Shape::None, Vec::new(), none(&Ty::Nat)),
                    arm(
                        Shape::Some,
                        vec![2],
                        let_in(
                            3,
                            Ty::Bool,
                            call(id("safe_check"), vec![v(0), v(2)]),
                            cond(v(3), some(&Ty::Nat, v(2)), none(&Ty::Nat)),
                        ),
                    ),
                ],
            ),
        ),
        function(
            vec![node.clone(), nodes.clone(), Ty::Nat],
            nodes.clone(),
            matching(
                nodes.clone(),
                call(id("fire"), vec![field(v(0), 0), step_of(v(2))]),
                vec![
                    arm(Shape::None, Vec::new(), v(1)),
                    arm(
                        Shape::Some,
                        vec![3],
                        append(
                            v(1),
                            singleton(&node, s.node_of(v(3), s.extended(v(0), step_of(v(2))))),
                        ),
                    ),
                ],
            ),
        ),
        function(
            vec![node.clone()],
            nodes.clone(),
            call(
                fold_collect,
                vec![
                    closure(id("propose_collect"), vec![v(0)]),
                    nil(&node),
                    call(id("propose_candidates"), vec![field(v(0), 0)]),
                ],
            ),
        ),
        s.cap_step(),
        function(
            vec![Ty::Nat],
            search.clone(),
            let_in(
                1,
                nodes.clone(),
                call(
                    fold_cap,
                    vec![
                        closure(id("cap_step"), vec![nat(3)]),
                        nil(&node),
                        singleton(
                            &node,
                            s.node_of(call(id("observe"), vec![v(0)]), nil(&step)),
                        ),
                    ],
                ),
                s.search_of(
                    v(1),
                    None,
                    none(&s.hit()),
                    lt(nat(3), nat(1)),
                    s.ledger_of([nat(0), nat(0), nat(0), nat(0), nat(0), length(v(1))]),
                ),
            ),
        ),
        // search_step: depth first, so the successors precede the rest of
        // the frontier, and nothing is deduplicated.
        function(
            vec![Ty::Nat, search.clone()],
            opt_t(search.clone()),
            matching(
                opt_t(search.clone()),
                field(v(1), s.found_at()),
                vec![
                    arm(Shape::Some, vec![2], none(&search)),
                    arm(
                        Shape::None,
                        Vec::new(),
                        matching(
                            opt_t(search.clone()),
                            field(v(1), 0),
                            vec![
                                arm(Shape::Nil, Vec::new(), none(&search)),
                                arm(
                                    Shape::Cons,
                                    vec![3, 4],
                                    matching(
                                        opt_t(search.clone()),
                                        call(id("accept"), vec![v(0), field(v(3), 0)]),
                                        vec![
                                            arm(
                                                Shape::Some,
                                                vec![5],
                                                some(
                                                    &search,
                                                    s.search_of(
                                                        v(4),
                                                        None,
                                                        some(
                                                            &s.hit(),
                                                            pair_of(&s.hit(), v(5), v(3)),
                                                        ),
                                                        field(v(1), s.truncated_at()),
                                                        s.changed(
                                                            &v(1),
                                                            [
                                                                Some(plus(
                                                                    s.counter(v(1), 0),
                                                                    nat(1),
                                                                )),
                                                                None,
                                                                None,
                                                                None,
                                                                Some(plus(
                                                                    s.counter(v(1), 4),
                                                                    nat(1),
                                                                )),
                                                                None,
                                                            ],
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            arm(
                                                Shape::None,
                                                Vec::new(),
                                                let_in(
                                                    6,
                                                    nodes.clone(),
                                                    call(id("successors"), vec![v(3)]),
                                                    let_in(
                                                        7,
                                                        nodes.clone(),
                                                        append(v(6), v(4)),
                                                        let_in(
                                                            8,
                                                            nodes.clone(),
                                                            call(
                                                                fold_cap,
                                                                vec![
                                                                    closure(
                                                                        id("cap_step"),
                                                                        vec![nat(3)],
                                                                    ),
                                                                    nil(&node),
                                                                    v(7),
                                                                ],
                                                            ),
                                                            some(
                                                                &search,
                                                                s.search_of(
                                                                    v(8),
                                                                    None,
                                                                    none(&s.hit()),
                                                                    or(
                                                                        field(
                                                                            v(1),
                                                                            s.truncated_at(),
                                                                        ),
                                                                        lt(nat(3), length(v(7))),
                                                                    ),
                                                                    s.changed(
                                                                        &v(1),
                                                                        [
                                                                            Some(plus(
                                                                                s.counter(v(1), 0),
                                                                                nat(1),
                                                                            )),
                                                                            Some(plus(
                                                                                s.counter(v(1), 1),
                                                                                call(
                                                                                    id("attempts"),
                                                                                    vec![field(
                                                                                        v(3),
                                                                                        0,
                                                                                    )],
                                                                                ),
                                                                            )),
                                                                            Some(plus(
                                                                                s.counter(v(1), 2),
                                                                                length(v(6)),
                                                                            )),
                                                                            Some(plus(
                                                                                s.counter(v(1), 3),
                                                                                nat(1),
                                                                            )),
                                                                            Some(plus(
                                                                                s.counter(v(1), 4),
                                                                                nat(1),
                                                                            )),
                                                                            Some(cond(
                                                                                lt(
                                                                                    s.counter(
                                                                                        v(1),
                                                                                        5,
                                                                                    ),
                                                                                    length(v(8)),
                                                                                ),
                                                                                length(v(8)),
                                                                                s.counter(v(1), 5),
                                                                            )),
                                                                        ],
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                ),
                            ],
                        ),
                    ),
                ],
            ),
        ),
        function(
            vec![pick.clone()],
            Ty::Nat,
            length(call(id("propose_candidates"), vec![v(0)])),
        ),
    ];
    engine_case(
        &format!("reasoning-screen-{weight}"),
        s.adts(vec![vec![Ty::Nat]]),
        &plan,
        functions,
        vec![
            (Template::ListFold, vec![node.clone(), nodes.clone()]),
            (Template::ListFold, vec![Ty::Nat, nodes]),
            (Template::IterateUntil, vec![search]),
        ],
        vec![natv(weight)],
        10_000,
        oracle,
    )
}

const DOSE: [&str; 6] = [
    "verdict",
    "dose_candidates",
    "safe_check",
    "verify",
    "attempt",
    "failure",
];

/// `Dose`: generate and verify the dose model's proposals, checking at most
/// three, left to right; it answers the accepted dose and the one step that
/// names it, or why none, with the run's ledger.
pub fn dose_case(weight: u64, oracle: Json) -> Case {
    let plan = Plan { names: &DOSE };
    let id = |name: &str| plan.id(name);
    // found, truncated, and the ledger of the run.
    let trial = adt(0);
    let ledger = adt(1);
    let step = adt(2);
    let fold = plan.lib(0);
    let trial_of = |found: Expr, truncated: Expr, counters: Expr| {
        construct(0, &trial, vec![found, truncated, counters])
    };
    let counters = |values: [Expr; 6]| construct(0, &ledger, values.into());
    let counter = |trial: Expr, index: u64| field(field(trial, 2), index);
    let explained_t = res_t(pair_t(Ty::Nat, list_t(step.clone())), failure_t());
    let entry_t = pair_t(explained_t.clone(), ledger_chain_t());
    let functions = vec![
        function(
            vec![Ty::Nat],
            entry_t.clone(),
            let_in(
                1,
                trial.clone(),
                call(
                    fold,
                    vec![
                        closure(id("attempt"), vec![v(0)]),
                        trial_of(
                            none(&Ty::Nat),
                            super::boolean(false),
                            counters([nat(0), nat(0), nat(0), nat(0), nat(0), nat(0)]),
                        ),
                        call(id("dose_candidates"), vec![v(0)]),
                    ],
                ),
                pair_of(
                    &entry_t,
                    matching(
                        explained_t.clone(),
                        field(v(1), 0),
                        vec![
                            arm(
                                Shape::Some,
                                vec![2],
                                build(
                                    Shape::Ok,
                                    explained_t.clone(),
                                    vec![pair_of(
                                        &pair_t(Ty::Nat, list_t(step.clone())),
                                        v(2),
                                        singleton(&step, construct(0, &step, vec![v(2)])),
                                    )],
                                ),
                            ),
                            arm(
                                Shape::None,
                                Vec::new(),
                                build(
                                    Shape::Error,
                                    explained_t.clone(),
                                    vec![call(id("failure"), vec![v(1)])],
                                ),
                            ),
                        ],
                    ),
                    ledger_chain((0..6).map(|index| counter(v(1), index)).collect()),
                ),
            ),
        ),
        dose_candidates(),
        safe_check(),
        // verify: the candidate if the verifier's check accepts it.
        function(
            vec![Ty::Nat, Ty::Nat],
            opt_t(Ty::Nat),
            cond(
                call(id("safe_check"), vec![v(0), v(1)]),
                some(&Ty::Nat, v(1)),
                none(&Ty::Nat),
            ),
        ),
        // attempt: check the next candidate while fewer than the budget
        // have been checked, counting the iteration, the guard evaluation,
        // and the verification; else record the cut.
        function(
            vec![Ty::Nat, trial.clone(), Ty::Nat],
            trial.clone(),
            matching(
                trial.clone(),
                field(v(1), 0),
                vec![
                    arm(Shape::Some, vec![3], v(1)),
                    arm(
                        Shape::None,
                        Vec::new(),
                        cond(
                            lt(counter(v(1), 4), nat(3)),
                            trial_of(
                                call(id("verify"), vec![v(0), v(2)]),
                                field(v(1), 1),
                                counters([
                                    plus(counter(v(1), 0), nat(1)),
                                    plus(counter(v(1), 1), nat(1)),
                                    counter(v(1), 2),
                                    counter(v(1), 3),
                                    plus(counter(v(1), 4), nat(1)),
                                    counter(v(1), 5),
                                ]),
                            ),
                            trial_of(none(&Ty::Nat), super::boolean(true), field(v(1), 2)),
                        ),
                    ),
                ],
            ),
        ),
        // failure: a cut generator is exhausted, one that proposed nothing
        // is unsolved, and one whose every candidate was refused is
        // rejected.
        function(
            vec![trial.clone()],
            failure_t(),
            cond(
                field(v(0), 1),
                failure(false, false),
                cond(
                    eq(counter(v(0), 4), nat(0)),
                    failure(false, true),
                    failure(true, false),
                ),
            ),
        ),
    ];
    engine_case(
        &format!("reasoning-dose-{weight}"),
        vec![
            vec![vec![opt_t(Ty::Nat), Ty::Bool, ledger.clone()]],
            vec![vec![Ty::Nat; 6]],
            vec![vec![Ty::Nat]],
        ],
        &plan,
        functions,
        vec![(Template::ListFold, vec![Ty::Nat, trial])],
        vec![natv(weight)],
        4000,
        oracle,
    )
}
