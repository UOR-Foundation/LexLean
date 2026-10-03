//! The `Gnaf` module of the `compiler` project: the GNAF request model of
//! SPEC.md §17.15, written here as LexLean semantic terms.
//!
//! The committed `compiler/src/Gnaf.lex.tex` is exactly what [`module`]
//! renders, which `cargo xtask check-calculus` enforces, so the model a
//! reviewer reads here is the model Lean checks. The host transcription
//! `lexlean::gnaf` and the `GnafFixtures` theorems are both measured
//! against it.
//!
//! Binders a branch never reads are named `ignored<n>` (and the binders of
//! `first_rejection` `rejection<n>`) from one counter, in the order the
//! terms are built. The committed names depend on that order: declarations
//! are built in module order, and within one, Rust evaluates arguments left
//! to right, inner calls before the call that receives them. Reordering the
//! construction of two terms that both draw a name renames binders, which
//! the byte comparison reports.

use std::cell::Cell;

use lexlean::calculus::term::{self, SEMANTICS, SYNTAX};
use lexlean::gnaf::MODEL;
use serde_json::{json, Value as Json};

use crate::lx::{
    self, add, and, axioms, beq, ble, blt, boolean, call, call_at, cons, decide, definition, eq,
    first, forall, function_ref, function_t, generic, inductive, ite, let_in, list, list_t,
    local_t, mutual, nat, nat_t, nil, none, not, option_t, or, pair, parameter, parameter_t, prim,
    product_t, project, recursive, second, some, structure, theorem, var,
};

/// The path of the generated module, relative to the repository root.
pub const PATH: &str = "compiler/src/Gnaf.lex.tex";

/// The axioms Lean reports for a declaration that reaches
/// `TargetSemantics.run`: the evaluator's well-founded and quotient
/// machinery. Exactly the declarations that evaluate a realization carry
/// them.
pub const RUN_AXIOMS: [&str; 3] = ["Classical.choice", "Quot.sound", "propext"];

/// The axiom `simp` rewriting with an equation lemma introduces.
const SIMP_AXIOMS: [&str; 1] = ["propext"];

/// The identity type parameter of the generic order.
const ID: &str = "Id";

/// Names for binders the branch body never reads; see the module comment.
struct Unused {
    count: Cell<u32>,
}

impl Unused {
    fn new() -> Self {
        Self {
            count: Cell::new(0),
        }
    }
    fn named(&self, base: &str) -> String {
        self.count.set(self.count.get() + 1);
        format!("{base}{}", self.count.get())
    }
    fn next(&self) -> String {
        self.named("ignored")
    }
}

/// A binder list mixing fixed names with names drawn from [`Unused`],
/// evaluated left to right.
macro_rules! binders {
    ($($binder:expr),* $(,)?) => {
        vec![$(($binder).to_string()),*]
    };
}

// --- vocabulary ------------------------------------------------------------

/// §12.4 claim classes in the authority's order: each with the types of
/// the constants it carries.
fn claims() -> Vec<(&'static str, Vec<Json>)> {
    let string_t = lx::string_t;
    let named = |name: &'static str| (name, Vec::new());
    vec![
        named("exact"),
        named("normalForm"),
        named("canonical"),
        named("representationMinimal"),
        named("comparisonTheorem"),
        (
            "profileDefinedComparison",
            vec![string_t(), string_t(), string_t()],
        ),
        named("inputTotal"),
        named("globalOptimal"),
        named("argminComplete"),
        named("paretoOptimal"),
        named("frontierComplete"),
        named("pointwiseEnvelopeComplete"),
        named("queryFamilyAnswerComplete"),
        named("useCaseGlobalOptimal"),
        named("workloadArgminComplete"),
        named("workloadParetoOptimal"),
        named("workloadFrontierComplete"),
        named("familyOptimal"),
        named("competitiveBound"),
        named("competitiveOptimal"),
        named("asymptoticBound"),
        named("asymptoticOptimal"),
        named("useCaseClassComplete"),
        named("useCaseClassAnswerComplete"),
        named("maintainedUseCaseClass"),
        named("restrictedUniverseOptimal"),
        named("revisionPreserved"),
        named("bestKnown"),
        named("measuredBestAmongTested"),
        named("heuristicSelected"),
        ("instanceOptimal", vec![nat_t(), nat_t()]),
    ]
}
/// The claims the scalar (total) order answers.
const SCALAR_CLAIMS: [&str; 2] = ["globalOptimal", "argminComplete"];
/// The claims the componentwise (partial) order answers.
const VECTOR_CLAIMS: [&str; 2] = ["paretoOptimal", "frontierComplete"];
/// §12.4: the legacy alias a conforming answer never emits as a base class.
const ALIAS_CLAIM: &str = "restrictedUniverseOptimal";

/// §8.2, §9.1: the kinds of action a system may take inside the machine
/// boundary.
const ACTION_KINDS: [&str; 10] = [
    "observation",
    "preprocessing",
    "advice",
    "retainedState",
    "dispatch",
    "fallback",
    "communication",
    "randomness",
    "scheduling",
    "execution",
];
/// Actions the calculus machine itself performs.
const PERFORMED: [&str; 4] = ["observation", "dispatch", "fallback", "execution"];
/// Actions before the invocation.
const PREPARATION: [&str; 3] = ["preprocessing", "advice", "retainedState"];

/// The constructors of `TargetSyntax.Value`, in declaration order, with
/// their field counts.
const VALUE_CONSTRUCTORS: [(&str, usize); 23] = [
    ("unit", 0),
    ("bool", 1),
    ("nat", 1),
    ("int", 1),
    ("u8", 1),
    ("u16", 1),
    ("u32", 1),
    ("u64", 1),
    ("i8", 1),
    ("i16", 1),
    ("i32", 1),
    ("i64", 1),
    ("string", 1),
    ("bytes", 1),
    ("ordering", 1),
    ("none", 0),
    ("some", 1),
    ("ok", 1),
    ("error", 1),
    ("list", 1),
    ("pair", 2),
    ("adt", 2),
    ("closure", 2),
];

// --- terms of this model ---------------------------------------------------

fn syntax_t(name: &str) -> Json {
    term::named(SYNTAX, name)
}
fn value_t() -> Json {
    syntax_t("Value")
}
fn expr_t() -> Json {
    syntax_t("Expr")
}
fn rejection_t() -> Json {
    local_t("Rejection")
}
fn kind_t() -> Json {
    local_t("ActionKind")
}
fn selector_t() -> Json {
    local_t("Selector")
}
fn status_t() -> Json {
    local_t("Status")
}
fn nats_t() -> Json {
    list_t(nat_t())
}
fn id_t() -> Json {
    parameter_t(ID)
}
/// A row of the generic order: an identity and its cost.
fn row_t(cost: Json) -> Json {
    product_t(id_t(), cost)
}

/// A constructor of this module.
fn local(constructor: &str, arguments: Vec<Json>) -> Json {
    lx::construct(lx::member(constructor), arguments)
}
/// A constructor of `TargetSyntax`.
fn syntax(constructor: &str, arguments: Vec<Json>) -> Json {
    lx::construct(term::member(SYNTAX, constructor), arguments)
}
/// A call of a definition of this module.
fn local_call(function: &str, arguments: Vec<Json>) -> Json {
    call(lx::member(function), arguments)
}
/// A call of a generic definition of this module at the identity type
/// `id`.
fn generic_call(function: &str, id: Json, arguments: Vec<Json>) -> Json {
    call_at(lx::member(function), vec![id], arguments)
}
/// A call of a definition of `TargetSemantics`.
fn semantics_call(function: &str, arguments: Vec<Json>) -> Json {
    call(term::member(SEMANTICS, function), arguments)
}

/// A branch on a constructor of this module or a builtin.
fn arm(constructor: &str, binders: Vec<String>, body: Json) -> Json {
    lx::branch(lx::member(constructor), binders, body)
}
fn syntax_arm(constructor: &str, binders: Vec<String>, body: Json) -> Json {
    lx::branch(term::member(SYNTAX, constructor), binders, body)
}
fn semantics_arm(constructor: &str, binders: Vec<String>, body: Json) -> Json {
    lx::branch(term::member(SEMANTICS, constructor), binders, body)
}
fn matching(scrutinee: Json, branches: Vec<Json>) -> Json {
    lx::matching(scrutinee, branches)
}

fn on_list(
    scrutinee: Json,
    empty: Json,
    head: impl Into<String>,
    tail: impl Into<String>,
    nonempty: Json,
) -> Json {
    matching(
        scrutinee,
        vec![
            arm("List.nil", Vec::new(), empty),
            arm("List.cons", vec![head.into(), tail.into()], nonempty),
        ],
    )
}
fn on_option(scrutinee: Json, absent: Json, found: impl Into<String>, present: Json) -> Json {
    matching(
        scrutinee,
        vec![
            arm("Option.none", Vec::new(), absent),
            arm("Option.some", vec![found.into()], present),
        ],
    )
}
/// A match on `nat` by `zero` and `succ remaining`.
fn on_nat(scrutinee: Json, zero: Json, successor: Json) -> Json {
    matching(
        scrutinee,
        vec![
            arm("Nat.zero", Vec::new(), zero),
            arm("Nat.succ", binders!["remaining"], successor),
        ],
    )
}
/// A total match on a `TargetSyntax.Value`: the listed constructors take
/// their bodies, every other one `default`.
fn on_value(
    unused: &Unused,
    scrutinee: Json,
    cases: Vec<(&str, Vec<String>, Json)>,
    default: &Json,
) -> Json {
    let branches = VALUE_CONSTRUCTORS
        .iter()
        .map(|(constructor, fields)| {
            let name = format!("Value.{constructor}");
            match cases.iter().find(|(case, _, _)| case == constructor) {
                Some((_, binders, body)) => syntax_arm(&name, binders.clone(), body.clone()),
                None => syntax_arm(
                    &name,
                    (0..*fields).map(|_| unused.next()).collect(),
                    default.clone(),
                ),
            }
        })
        .collect();
    matching(scrutinee, branches)
}

fn yes() -> Json {
    boolean(true)
}
fn no() -> Json {
    boolean(false)
}

/// `none`: the request passes this check.
fn pass() -> Json {
    none(rejection_t())
}
/// `some` rejection `constructor arguments`.
fn reject(constructor: &str, arguments: Vec<Json>) -> Json {
    some(
        rejection_t(),
        local(&format!("Rejection.{constructor}"), arguments),
    )
}
/// The first rejection among `checks`, each an `Option Rejection`, or
/// `none`. Written as nested matches rather than a fold so every binder is
/// named in the generated Lean.
fn first_rejection(unused: &Unused, checks: Vec<Json>) -> Json {
    let mut checks = checks.into_iter();
    let Some(head) = checks.next() else {
        return pass();
    };
    let found = unused.named("rejection");
    let rest = first_rejection(unused, checks.collect());
    on_option(head, rest, found.clone(), some(rejection_t(), var(&found)))
}

fn status(constructor: &str, arguments: Vec<Json>) -> Json {
    local(&format!("Status.{constructor}"), arguments)
}

// --- proofs ----------------------------------------------------------------

/// `simp only` with these definitions and theorems, sorted as the proof
/// language requires.
fn simplify(names: &[&str]) -> Json {
    let mut names: Vec<&str> = names.to_vec();
    names.sort_unstable();
    names.dedup();
    json!({"kind": "simplify",
           "definitions": names.iter().map(|name| lx::member(name)).collect::<Vec<_>>()})
}
fn reflexivity() -> Json {
    json!({"kind": "reflexivity"})
}
fn proof_branch(constructor: &str, binders: &[&str], proof: Json) -> Json {
    json!({"constructor": constructor, "binders": binders, "proof": proof})
}
fn cases(scrutinee: &str, branches: Vec<Json>) -> Json {
    json!({"kind": "cases", "scrutinee": scrutinee, "branches": branches})
}
fn induction(scrutinee: &str, generalizing: &[&str], branches: Vec<Json>) -> Json {
    let mut out = json!({"kind": "induction", "scrutinee": scrutinee, "branches": branches});
    if !generalizing.is_empty() {
        out["generalizing"] = json!(generalizing);
    }
    out
}
/// A theorem over `parameters`.
fn lemma(name: &str, parameters: Vec<(&str, Json)>, statement: Json, proof: Json) -> Json {
    let mut out = theorem(name, statement, proof);
    out["parameters"] = parameters
        .into_iter()
        .map(|(name, ty)| parameter(name, ty))
        .collect();
    out
}
/// A theorem about every combination of Boolean values, decided.
fn boolean_law(name: &str, variables: &[&str], statement: Json) -> Json {
    let statement = variables.iter().rev().fold(statement, |body, variable| {
        forall(variable, lx::bool_t(), body)
    });
    theorem(name, statement, decide())
}

// --- the request -----------------------------------------------------------

/// Constructors without fields.
fn none_of(names: &[&'static str]) -> Vec<(&'static str, Vec<Json>)> {
    names.iter().map(|name| (*name, Vec::new())).collect()
}

fn request_types() -> Vec<Json> {
    vec![
        inductive("ClaimClass", claims()),
        // §8.2, §9.1: how each admitted action is accounted.
        inductive("ActionKind", none_of(&ACTION_KINDS)),
        inductive(
            "Charge",
            vec![
                ("steps", Vec::new()),
                ("constant", vec![nat_t()]),
                ("free", Vec::new()),
                ("undeclared", Vec::new()),
            ],
        ),
        structure(
            "Action",
            vec![("kind", kind_t()), ("charge", local_t("Charge"))],
        ),
        // §10.8: the comparison boundary.
        inductive(
            "Boundary",
            none_of(&["complete", "preparedState", "preparedPlan"]),
        ),
        // §8.2, §9.1: the cost model's operand-size treatment.
        inductive("OperandSize", none_of(&["weighted", "unit"])),
        // §8.2: the machine's hard capacity.
        structure(
            "Capacity",
            vec![
                ("fuel", nat_t()),
                ("domain", nat_t()),
                ("systems", nat_t()),
                ("charge", nat_t()),
            ],
        ),
        // §10.8: a prepared artifact bound in the common initial state, by
        // the action that prepared it and the SHA-256 of its bytes.
        structure(
            "Prepared",
            vec![("kind", kind_t()), ("artifact", lx::string_t())],
        ),
        structure(
            "Machine",
            vec![
                ("fuel", nat_t()),
                ("capacity", local_t("Capacity")),
                ("operandSize", local_t("OperandSize")),
                ("actions", list_t(local_t("Action"))),
                ("boundary", local_t("Boundary")),
                ("prepared", list_t(local_t("Prepared"))),
            ],
        ),
        // The complete-system grammar: every system is a selector over the
        // shared plan functions, which sit at indices 1.. of every
        // realization.
        inductive(
            "Selector",
            vec![
                ("fixed", vec![nat_t()]),
                ("dispatch", vec![nat_t(), nat_t(), nat_t()]),
            ],
        ),
        // A shared plan and the preparation actions it needs.
        structure(
            "Plan",
            vec![
                ("function", syntax_t("Function")),
                ("prepares", list_t(kind_t())),
            ],
        ),
        structure(
            "Grammar",
            vec![
                ("argument", syntax_t("Ty")),
                ("result", syntax_t("Ty")),
                ("plans", list_t(local_t("Plan"))),
                ("thresholds", nats_t()),
            ],
        ),
        // §8.3: the universe carrier and its completeness evidence.
        inductive(
            "Carrier",
            vec![
                ("grammar", vec![local_t("Grammar")]),
                ("internalPlans", Vec::new()),
                ("optimizerOutput", Vec::new()),
                ("discovered", vec![nats_t()]),
                ("cached", Vec::new()),
            ],
        ),
        inductive(
            "Completeness",
            none_of(&[
                "grammarEquality",
                "missing",
                "citesUniverseId",
                "citesOptimizer",
            ]),
        ),
        inductive(
            "Scope",
            none_of(&["grammarUniverse", "calculusPrograms", "rustPrograms"]),
        ),
        // §9.2, §9.3: a scalar objective is total steps; a vector objective
        // is (steps, size), ordered componentwise.
        inductive("Objective", none_of(&["scalar", "vector"])),
        // `universe` is the SystemUniverseId the host computes from the
        // problem, machine, and carrier before evaluation; the model
        // carries it as bound data.
        structure(
            "Request",
            vec![
                ("reference", syntax_t("Program")),
                ("domain", list_t(value_t())),
                ("machine", local_t("Machine")),
                ("carrier", local_t("Carrier")),
                ("completeness", local_t("Completeness")),
                ("universe", lx::string_t()),
                ("objective", local_t("Objective")),
                ("claim", local_t("ClaimClass")),
                ("scope", local_t("Scope")),
            ],
        ),
        inductive(
            "Rejection",
            vec![
                ("emptyDomain", Vec::new()),
                ("internalPlanUniverse", Vec::new()),
                ("optimizerDefinedUniverse", Vec::new()),
                ("discoveredUniverse", Vec::new()),
                ("cachedUniverse", Vec::new()),
                ("missingCompleteness", Vec::new()),
                ("selfReferentialCompleteness", Vec::new()),
                ("optimizerCompleteness", Vec::new()),
                ("beyondCapacity", Vec::new()),
                ("unitCostOperands", Vec::new()),
                ("duplicateAction", vec![kind_t()]),
                ("unaccountedAction", vec![kind_t()]),
                ("hiddenCost", vec![kind_t()]),
                ("unrealizableAction", vec![kind_t()]),
                ("unboundPreparation", vec![kind_t()]),
                ("strayPreparedArtifact", vec![kind_t()]),
                ("claimAlias", Vec::new()),
                ("scalarClaimOverPartialOrder", Vec::new()),
                ("vectorClaimOverTotalOrder", Vec::new()),
                ("uncoveredScope", Vec::new()),
                ("unsupportedClaim", Vec::new()),
            ],
        ),
        inductive(
            "Status",
            vec![
                ("admitted", vec![nat_t(), nat_t()]),
                ("inadmissible", Vec::new()),
                ("unresolved", Vec::new()),
            ],
        ),
        inductive(
            "Answer",
            vec![
                ("rejected", vec![rejection_t()]),
                ("argmin", vec![nats_t(), nat_t()]),
                ("frontier", vec![nats_t()]),
                ("infeasible", Vec::new()),
                ("incomplete", Vec::new()),
            ],
        ),
    ]
}

// --- the machine contract --------------------------------------------------

/// A match on `kind` with one branch per action kind.
fn on_kind(body: impl Fn(usize, &str) -> Json) -> Json {
    matching(
        var("kind"),
        ACTION_KINDS
            .iter()
            .enumerate()
            .map(|(index, kind)| arm(&format!("ActionKind.{kind}"), Vec::new(), body(index, kind)))
            .collect(),
    )
}
/// Whether `kind` is one of `kinds`.
fn kind_in(kinds: &[&str]) -> Json {
    on_kind(|_, kind| boolean(kinds.contains(&kind)))
}
fn hidden_cost() -> Json {
    reject("hiddenCost", vec![var("kind")])
}
/// A free preparation action is excluded from every system's cost only on
/// a prepared boundary whose common initial state binds an artifact it
/// prepared (§10.8).
fn prepared_bound() -> Json {
    let unbound = || reject("unboundPreparation", vec![var("kind")]);
    let bound = || {
        ite(
            local_call(
                "preparedFor",
                vec![project(var("machine"), "prepared"), var("kind")],
            ),
            pass(),
            unbound(),
        )
    };
    matching(
        project(var("machine"), "boundary"),
        vec![
            arm("Boundary.complete", Vec::new(), unbound()),
            arm("Boundary.preparedState", Vec::new(), bound()),
            arm("Boundary.preparedPlan", Vec::new(), bound()),
        ],
    )
}

#[allow(clippy::too_many_lines)]
fn machine_contract(unused: &Unused) -> Vec<Json> {
    let actions_t = || list_t(local_t("Action"));
    let action_kind = || project(var("action"), "kind");
    let same_kind_as_wanted = || local_call("sameKind", vec![action_kind(), var("wanted")]);
    vec![
        definition(
            "kindIndex",
            vec![parameter("kind", kind_t())],
            nat_t(),
            on_kind(|index, _| nat(index as u64)),
        ),
        definition(
            "sameKind",
            vec![parameter("left", kind_t()), parameter("right", kind_t())],
            lx::bool_t(),
            beq(
                local_call("kindIndex", vec![var("left")]),
                local_call("kindIndex", vec![var("right")]),
            ),
        ),
        // Actions the calculus machine itself performs: their work is
        // calculus steps, so the step count is their only faithful charge.
        definition(
            "performed",
            vec![parameter("kind", kind_t())],
            lx::bool_t(),
            kind_in(&PERFORMED),
        ),
        // Actions before the invocation; the calculus has no such phase, so
        // their cost is a declared constant per plan that needs them, or a
        // bound prepared artifact.
        definition(
            "preparation",
            vec![parameter("kind", kind_t())],
            lx::bool_t(),
            kind_in(&PREPARATION),
        ),
        recursive(
            "actions",
            definition(
                "findAction",
                vec![
                    parameter("actions", actions_t()),
                    parameter("wanted", kind_t()),
                ],
                option_t(local_t("Charge")),
                on_list(
                    var("actions"),
                    none(local_t("Charge")),
                    "action",
                    "rest",
                    ite(
                        same_kind_as_wanted(),
                        some(local_t("Charge"), project(var("action"), "charge")),
                        local_call("findAction", vec![var("rest"), var("wanted")]),
                    ),
                ),
            ),
        ),
        recursive(
            "actions",
            definition(
                "countKind",
                vec![
                    parameter("actions", actions_t()),
                    parameter("wanted", kind_t()),
                ],
                nat_t(),
                on_list(
                    var("actions"),
                    nat(0),
                    "action",
                    "rest",
                    add(
                        ite(same_kind_as_wanted(), nat(1), nat(0)),
                        local_call("countKind", vec![var("rest"), var("wanted")]),
                    ),
                ),
            ),
        ),
        // Each kind is accounted at most once: two charges for one kind
        // leave its cost ambiguous.
        recursive(
            "actions",
            definition(
                "checkDistinct",
                vec![
                    parameter("actions", actions_t()),
                    parameter("all", actions_t()),
                ],
                option_t(rejection_t()),
                on_list(
                    var("actions"),
                    pass(),
                    "action",
                    "rest",
                    ite(
                        blt(
                            nat(1),
                            local_call("countKind", vec![var("all"), action_kind()]),
                        ),
                        reject("duplicateAction", vec![action_kind()]),
                        local_call("checkDistinct", vec![var("rest"), var("all")]),
                    ),
                ),
            ),
        ),
        // Every action some system of the universe performs is declared and
        // charged by steps; a constant, free, or undeclared charge hides or
        // replaces the work the calculus actually counts.
        definition(
            "checkPerformed",
            vec![
                parameter("machine", local_t("Machine")),
                parameter("wanted", kind_t()),
            ],
            option_t(rejection_t()),
            on_option(
                local_call(
                    "findAction",
                    vec![project(var("machine"), "actions"), var("wanted")],
                ),
                reject("unaccountedAction", vec![var("wanted")]),
                "charge",
                matching(
                    var("charge"),
                    vec![
                        arm("Charge.steps", Vec::new(), pass()),
                        arm(
                            "Charge.constant",
                            binders![unused.next()],
                            reject("hiddenCost", vec![var("wanted")]),
                        ),
                        arm(
                            "Charge.free",
                            Vec::new(),
                            reject("hiddenCost", vec![var("wanted")]),
                        ),
                        arm(
                            "Charge.undeclared",
                            Vec::new(),
                            reject("hiddenCost", vec![var("wanted")]),
                        ),
                    ],
                ),
            ),
        ),
        // Whether the common initial state binds an artifact `kind`
        // prepared.
        recursive(
            "prepared",
            definition(
                "preparedFor",
                vec![
                    parameter("prepared", list_t(local_t("Prepared"))),
                    parameter("kind", kind_t()),
                ],
                lx::bool_t(),
                on_list(
                    var("prepared"),
                    no(),
                    "artifact",
                    "rest",
                    or(
                        local_call(
                            "sameKind",
                            vec![project(var("artifact"), "kind"), var("kind")],
                        ),
                        local_call("preparedFor", vec![var("rest"), var("kind")]),
                    ),
                ),
            ),
        ),
        // An admitted preparation action costs a positive constant per
        // invocation of every plan that needs it, or is free only through a
        // prepared artifact bound in the common initial state (§10.8).
        // Communication, randomness and scheduling are not actions of the
        // sequential deterministic calculus machine, so a universe
        // admitting them is not the one the grammar generates.
        definition(
            "checkDeclared",
            vec![
                parameter("machine", local_t("Machine")),
                parameter("action", local_t("Action")),
            ],
            option_t(rejection_t()),
            let_in(
                "kind",
                kind_t(),
                action_kind(),
                ite(
                    local_call("performed", vec![var("kind")]),
                    pass(),
                    ite(
                        local_call("preparation", vec![var("kind")]),
                        matching(
                            project(var("action"), "charge"),
                            vec![
                                arm("Charge.steps", Vec::new(), hidden_cost()),
                                arm(
                                    "Charge.constant",
                                    binders!["amount"],
                                    ite(beq(var("amount"), nat(0)), hidden_cost(), pass()),
                                ),
                                arm("Charge.free", Vec::new(), prepared_bound()),
                                arm("Charge.undeclared", Vec::new(), hidden_cost()),
                            ],
                        ),
                        reject("unrealizableAction", vec![var("kind")]),
                    ),
                ),
            ),
        ),
        recursive(
            "actions",
            definition(
                "checkAllDeclared",
                vec![
                    parameter("machine", local_t("Machine")),
                    parameter("actions", actions_t()),
                ],
                option_t(rejection_t()),
                on_list(
                    var("actions"),
                    pass(),
                    "action",
                    "rest",
                    on_option(
                        local_call("checkDeclared", vec![var("machine"), var("action")]),
                        local_call("checkAllDeclared", vec![var("machine"), var("rest")]),
                        "rejection",
                        some(rejection_t(), var("rejection")),
                    ),
                ),
            ),
        ),
        // A prepared artifact leaves every system's cost only as the product
        // of an action admitted free; bound for any other action it would
        // exclude work that is charged.
        recursive(
            "prepared",
            definition(
                "checkPrepared",
                vec![
                    parameter("machine", local_t("Machine")),
                    parameter("prepared", list_t(local_t("Prepared"))),
                ],
                option_t(rejection_t()),
                on_list(var("prepared"), pass(), "artifact", "rest", {
                    let stray = || {
                        reject(
                            "strayPreparedArtifact",
                            vec![project(var("artifact"), "kind")],
                        )
                    };
                    on_option(
                        local_call(
                            "findAction",
                            vec![
                                project(var("machine"), "actions"),
                                project(var("artifact"), "kind"),
                            ],
                        ),
                        stray(),
                        "charge",
                        matching(
                            var("charge"),
                            vec![
                                arm("Charge.steps", Vec::new(), stray()),
                                arm("Charge.constant", binders![unused.next()], stray()),
                                arm(
                                    "Charge.free",
                                    Vec::new(),
                                    local_call("checkPrepared", vec![var("machine"), var("rest")]),
                                ),
                                arm("Charge.undeclared", Vec::new(), stray()),
                            ],
                        ),
                    )
                }),
            ),
        ),
        // What one plan's need of `kind` costs per invocation: the declared
        // constant, or nothing when the action is free.
        definition(
            "unitCharge",
            vec![
                parameter("actions", actions_t()),
                parameter("kind", kind_t()),
            ],
            nat_t(),
            on_option(
                local_call("findAction", vec![var("actions"), var("kind")]),
                nat(0),
                "charge",
                matching(
                    var("charge"),
                    vec![
                        arm("Charge.steps", Vec::new(), nat(0)),
                        arm("Charge.constant", binders!["amount"], var("amount")),
                        arm("Charge.free", Vec::new(), nat(0)),
                        arm("Charge.undeclared", Vec::new(), nat(0)),
                    ],
                ),
            ),
        ),
        recursive(
            "kinds",
            definition(
                "planCharge",
                vec![
                    parameter("actions", actions_t()),
                    parameter("kinds", list_t(kind_t())),
                ],
                nat_t(),
                on_list(
                    var("kinds"),
                    nat(0),
                    "kind",
                    "rest",
                    add(
                        local_call("unitCharge", vec![var("actions"), var("kind")]),
                        local_call("planCharge", vec![var("actions"), var("rest")]),
                    ),
                ),
            ),
        ),
        // Every preparation a plan needs is an action the machine accounts.
        recursive(
            "kinds",
            definition(
                "checkKindsAccounted",
                vec![
                    parameter("actions", actions_t()),
                    parameter("kinds", list_t(kind_t())),
                ],
                option_t(rejection_t()),
                on_list(
                    var("kinds"),
                    pass(),
                    "kind",
                    "rest",
                    on_option(
                        local_call("findAction", vec![var("actions"), var("kind")]),
                        reject("unaccountedAction", vec![var("kind")]),
                        unused.next(),
                        local_call("checkKindsAccounted", vec![var("actions"), var("rest")]),
                    ),
                ),
            ),
        ),
        recursive(
            "plans",
            definition(
                "checkPlanPreparation",
                vec![
                    parameter("actions", actions_t()),
                    parameter("plans", list_t(local_t("Plan"))),
                ],
                option_t(rejection_t()),
                on_list(
                    var("plans"),
                    pass(),
                    "plan",
                    "rest",
                    on_option(
                        local_call(
                            "checkKindsAccounted",
                            vec![var("actions"), project(var("plan"), "prepares")],
                        ),
                        local_call("checkPlanPreparation", vec![var("actions"), var("rest")]),
                        "rejection",
                        some(rejection_t(), var("rejection")),
                    ),
                ),
            ),
        ),
        // Whether some declared constant exceeds the capacity's charge.
        recursive(
            "actions",
            definition(
                "chargeBeyond",
                vec![
                    parameter("actions", actions_t()),
                    parameter("bound", nat_t()),
                ],
                lx::bool_t(),
                on_list(
                    var("actions"),
                    no(),
                    "action",
                    "rest",
                    or(
                        matching(
                            project(var("action"), "charge"),
                            vec![
                                arm("Charge.steps", Vec::new(), no()),
                                arm(
                                    "Charge.constant",
                                    binders!["amount"],
                                    blt(var("bound"), var("amount")),
                                ),
                                arm("Charge.free", Vec::new(), no()),
                                arm("Charge.undeclared", Vec::new(), no()),
                            ],
                        ),
                        local_call("chargeBeyond", vec![var("rest"), var("bound")]),
                    ),
                ),
            ),
        ),
    ]
}

// --- the universe ----------------------------------------------------------

/// `front ++ back`, written out so the membership proofs can unfold it.
fn append_all(item: Json, front: Json, back: Json) -> Json {
    call_at(lx::member("appendAll"), vec![item], vec![front, back])
}

/// The grammar's systems, independently of any evaluation: every fixed
/// plan, then for every threshold every dispatch from a small plan to a
/// large plan, both ranging over every plan.
#[allow(clippy::too_many_lines)]
fn universe() -> Vec<Json> {
    let item_t = || parameter_t("Item");
    let selector = |name: &str, arguments: Vec<Json>| local(&format!("Selector.{name}"), arguments);
    let selectors_in =
        |selector: Json, selectors: Json| local_call("selectorIn", vec![selector, selectors]);
    let nat_in = |item: Json, items: Json| local_call("natIn", vec![item, items]);
    let plan_count = || prim("length", vec![project(var("grammar"), "plans")], nat_t());
    vec![
        generic(
            &["Item"],
            recursive(
                "front",
                definition(
                    "appendAll",
                    vec![
                        parameter("front", list_t(item_t())),
                        parameter("back", list_t(item_t())),
                    ],
                    list_t(item_t()),
                    on_list(
                        var("front"),
                        var("back"),
                        "head",
                        "rest",
                        cons(var("head"), append_all(item_t(), var("rest"), var("back"))),
                    ),
                ),
            ),
        ),
        // The plan indices `0, 1, .., count - 1`.
        recursive(
            "count",
            definition(
                "range",
                vec![parameter("count", nat_t())],
                nats_t(),
                on_nat(
                    var("count"),
                    nil(nat_t()),
                    append_all(
                        nat_t(),
                        local_call("range", vec![var("remaining")]),
                        list(nat_t(), vec![var("remaining")]),
                    ),
                ),
            ),
        ),
        recursive(
            "items",
            definition(
                "natIn",
                vec![parameter("item", nat_t()), parameter("items", nats_t())],
                lx::bool_t(),
                on_list(
                    var("items"),
                    no(),
                    "head",
                    "rest",
                    or(
                        beq(var("item"), var("head")),
                        nat_in(var("item"), var("rest")),
                    ),
                ),
            ),
        ),
        // Selector identity: the same constructor with the same fields.
        definition(
            "selectorEq",
            vec![
                parameter("left", selector_t()),
                parameter("right", selector_t()),
            ],
            lx::bool_t(),
            matching(
                var("left"),
                vec![
                    arm(
                        "Selector.fixed",
                        binders!["plan"],
                        matching(
                            var("right"),
                            vec![
                                arm(
                                    "Selector.fixed",
                                    binders!["other"],
                                    beq(var("plan"), var("other")),
                                ),
                                arm(
                                    "Selector.dispatch",
                                    binders!["ignoredThreshold", "ignoredSmall", "ignoredLarge"],
                                    no(),
                                ),
                            ],
                        ),
                    ),
                    arm(
                        "Selector.dispatch",
                        binders!["threshold", "small", "large"],
                        matching(
                            var("right"),
                            vec![
                                arm("Selector.fixed", binders!["ignoredPlan"], no()),
                                arm(
                                    "Selector.dispatch",
                                    binders!["otherThreshold", "otherSmall", "otherLarge"],
                                    and(
                                        beq(var("threshold"), var("otherThreshold")),
                                        and(
                                            beq(var("small"), var("otherSmall")),
                                            beq(var("large"), var("otherLarge")),
                                        ),
                                    ),
                                ),
                            ],
                        ),
                    ),
                ],
            ),
        ),
        recursive(
            "selectors",
            definition(
                "selectorIn",
                vec![
                    parameter("selector", selector_t()),
                    parameter("selectors", list_t(selector_t())),
                ],
                lx::bool_t(),
                on_list(
                    var("selectors"),
                    no(),
                    "head",
                    "rest",
                    or(
                        local_call("selectorEq", vec![var("selector"), var("head")]),
                        selectors_in(var("selector"), var("rest")),
                    ),
                ),
            ),
        ),
        recursive(
            "plans",
            definition(
                "fixedOver",
                vec![parameter("plans", nats_t())],
                list_t(selector_t()),
                on_list(
                    var("plans"),
                    nil(selector_t()),
                    "plan",
                    "rest",
                    cons(
                        selector("fixed", vec![var("plan")]),
                        local_call("fixedOver", vec![var("rest")]),
                    ),
                ),
            ),
        ),
        recursive(
            "larges",
            definition(
                "dispatchOver",
                vec![
                    parameter("threshold", nat_t()),
                    parameter("small", nat_t()),
                    parameter("larges", nats_t()),
                ],
                list_t(selector_t()),
                on_list(
                    var("larges"),
                    nil(selector_t()),
                    "large",
                    "rest",
                    cons(
                        selector(
                            "dispatch",
                            vec![var("threshold"), var("small"), var("large")],
                        ),
                        local_call(
                            "dispatchOver",
                            vec![var("threshold"), var("small"), var("rest")],
                        ),
                    ),
                ),
            ),
        ),
        recursive(
            "smalls",
            definition(
                "dispatchSmalls",
                vec![
                    parameter("threshold", nat_t()),
                    parameter("smalls", nats_t()),
                    parameter("larges", nats_t()),
                ],
                list_t(selector_t()),
                on_list(
                    var("smalls"),
                    nil(selector_t()),
                    "small",
                    "rest",
                    append_all(
                        selector_t(),
                        local_call(
                            "dispatchOver",
                            vec![var("threshold"), var("small"), var("larges")],
                        ),
                        local_call(
                            "dispatchSmalls",
                            vec![var("threshold"), var("rest"), var("larges")],
                        ),
                    ),
                ),
            ),
        ),
        recursive(
            "thresholds",
            definition(
                "dispatchThresholds",
                vec![
                    parameter("thresholds", nats_t()),
                    parameter("plans", nats_t()),
                ],
                list_t(selector_t()),
                on_list(
                    var("thresholds"),
                    nil(selector_t()),
                    "threshold",
                    "rest",
                    append_all(
                        selector_t(),
                        local_call(
                            "dispatchSmalls",
                            vec![var("threshold"), var("plans"), var("plans")],
                        ),
                        local_call("dispatchThresholds", vec![var("rest"), var("plans")]),
                    ),
                ),
            ),
        ),
        // §8.3 GenerationGrammar: the universe in expansion order.
        definition(
            "expand",
            vec![parameter("grammar", local_t("Grammar"))],
            list_t(selector_t()),
            let_in(
                "all",
                nats_t(),
                local_call("range", vec![plan_count()]),
                append_all(
                    selector_t(),
                    local_call("fixedOver", vec![var("all")]),
                    local_call(
                        "dispatchThresholds",
                        vec![project(var("grammar"), "thresholds"), var("all")],
                    ),
                ),
            ),
        ),
        // §8.3 MembershipSemantics, stated without the expansion: a fixed
        // system runs an existing plan; a dispatch runs existing plans at a
        // declared threshold.
        definition(
            "wellFormed",
            vec![
                parameter("grammar", local_t("Grammar")),
                parameter("selector", selector_t()),
            ],
            lx::bool_t(),
            let_in(
                "count",
                nat_t(),
                plan_count(),
                matching(
                    var("selector"),
                    vec![
                        arm(
                            "Selector.fixed",
                            binders!["plan"],
                            blt(var("plan"), var("count")),
                        ),
                        arm(
                            "Selector.dispatch",
                            binders!["threshold", "small", "large"],
                            and(
                                nat_in(var("threshold"), project(var("grammar"), "thresholds")),
                                and(
                                    blt(var("small"), var("count")),
                                    blt(var("large"), var("count")),
                                ),
                            ),
                        ),
                    ],
                ),
            ),
        ),
    ]
}

/// §8.3 CompletenessProposition: `Gnaf.expandComplete` proves, for every
/// grammar and every selector, that the selector is a member of the
/// expansion exactly when it is well formed, so the universe is the
/// grammar's by theorem rather than by the expansion's say-so. The lemmas
/// before it are the Boolean and arithmetic facts it rewrites with.
#[allow(clippy::too_many_lines)]
fn completeness() -> Vec<Json> {
    let a = || var("a");
    let b = || var("b");
    let c = || var("c");
    let selector = |name: &str, arguments: Vec<Json>| local(&format!("Selector.{name}"), arguments);
    let selector_in =
        |selector: Json, selectors: Json| local_call("selectorIn", vec![selector, selectors]);
    let nat_in = |item: Json, items: Json| local_call("natIn", vec![item, items]);
    let fixed = || selector("fixed", vec![var("plan")]);
    let dispatch = || {
        selector(
            "dispatch",
            vec![var("threshold"), var("small"), var("large")],
        )
    };
    let next = |value: &str| add(var(value), nat(1));
    let list_induction =
        |scrutinee: &str, generalizing: &[&str], nil_proof: Json, cons_proof: Json| {
            induction(
                scrutinee,
                generalizing,
                vec![
                    proof_branch("nil", &[], nil_proof),
                    proof_branch("cons", &["head", "rest", "hypothesis"], cons_proof),
                ],
            )
        };
    let simp_lemma = |name: &str, parameters: Vec<(&str, Json)>, statement: Json, proof: Json| {
        axioms(&SIMP_AXIOMS, lemma(name, parameters, statement, proof))
    };
    let nats = || nats_t();
    vec![
        boolean_law(
            "orAssociative",
            &["a", "b", "c"],
            eq(or(or(a(), b()), c()), or(a(), or(b(), c()))),
        ),
        boolean_law("orFalse", &["a"], eq(or(a(), no()), a())),
        boolean_law("falseOr", &["a"], eq(or(no(), a()), a())),
        boolean_law("andFalse", &["a"], eq(and(a(), no()), no())),
        boolean_law("falseAnd", &["a"], eq(and(no(), a()), no())),
        boolean_law(
            "andOverOr",
            &["a", "b", "c"],
            eq(or(and(a(), b()), and(a(), c())), and(a(), or(b(), c()))),
        ),
        boolean_law(
            "orUnderAnd",
            &["a", "b", "c"],
            eq(or(and(a(), c()), and(b(), c())), and(or(a(), b()), c())),
        ),
        lemma(
            "bltNext",
            vec![("left", nat_t()), ("right", nat_t())],
            eq(
                blt(next("left"), next("right")),
                blt(var("left"), var("right")),
            ),
            reflexivity(),
        ),
        lemma(
            "beqNext",
            vec![("left", nat_t()), ("right", nat_t())],
            eq(
                beq(next("left"), next("right")),
                beq(var("left"), var("right")),
            ),
            reflexivity(),
        ),
        // Below `bound + 1` is below `bound` or `bound` itself.
        simp_lemma(
            "bltSucc",
            vec![("item", nat_t()), ("bound", nat_t())],
            eq(
                blt(var("item"), next("bound")),
                or(
                    blt(var("item"), var("bound")),
                    beq(var("item"), var("bound")),
                ),
            ),
            induction(
                "item",
                &["bound"],
                vec![
                    proof_branch(
                        "zero",
                        &[],
                        cases(
                            "bound",
                            vec![
                                proof_branch("zero", &[], reflexivity()),
                                proof_branch("succ", &["smaller"], reflexivity()),
                            ],
                        ),
                    ),
                    proof_branch(
                        "succ",
                        &["previous", "hypothesis"],
                        cases(
                            "bound",
                            vec![
                                proof_branch("zero", &[], reflexivity()),
                                proof_branch(
                                    "succ",
                                    &["smaller"],
                                    simplify(&["beqNext", "bltNext", "hypothesis"]),
                                ),
                            ],
                        ),
                    ),
                ],
            ),
        ),
        simp_lemma(
            "natInAppend",
            vec![("item", nat_t()), ("front", nats()), ("back", nats())],
            eq(
                nat_in(var("item"), append_all(nat_t(), var("front"), var("back"))),
                or(
                    nat_in(var("item"), var("front")),
                    nat_in(var("item"), var("back")),
                ),
            ),
            list_induction(
                "front",
                &[],
                reflexivity(),
                simplify(&["appendAll", "hypothesis", "natIn", "orAssociative"]),
            ),
        ),
        simp_lemma(
            "selectorInAppend",
            vec![
                ("selector", selector_t()),
                ("front", list_t(selector_t())),
                ("back", list_t(selector_t())),
            ],
            eq(
                selector_in(
                    var("selector"),
                    append_all(selector_t(), var("front"), var("back")),
                ),
                or(
                    selector_in(var("selector"), var("front")),
                    selector_in(var("selector"), var("back")),
                ),
            ),
            list_induction(
                "front",
                &[],
                reflexivity(),
                simplify(&["appendAll", "hypothesis", "orAssociative", "selectorIn"]),
            ),
        ),
        simp_lemma(
            "natInRange",
            vec![("item", nat_t()), ("count", nat_t())],
            eq(
                nat_in(var("item"), local_call("range", vec![var("count")])),
                blt(var("item"), var("count")),
            ),
            induction(
                "count",
                &[],
                vec![
                    proof_branch("zero", &[], reflexivity()),
                    proof_branch(
                        "succ",
                        &["smaller", "hypothesis"],
                        simplify(&[
                            "bltSucc",
                            "hypothesis",
                            "natIn",
                            "natInAppend",
                            "orFalse",
                            "range",
                        ]),
                    ),
                ],
            ),
        ),
        simp_lemma(
            "fixedInFixed",
            vec![("plan", nat_t()), ("plans", nats())],
            eq(
                selector_in(fixed(), local_call("fixedOver", vec![var("plans")])),
                nat_in(var("plan"), var("plans")),
            ),
            list_induction(
                "plans",
                &[],
                reflexivity(),
                simplify(&[
                    "fixedOver",
                    "hypothesis",
                    "natIn",
                    "selectorEq",
                    "selectorIn",
                ]),
            ),
        ),
        simp_lemma(
            "dispatchInFixed",
            vec![
                ("threshold", nat_t()),
                ("small", nat_t()),
                ("large", nat_t()),
                ("plans", nats()),
            ],
            eq(
                selector_in(dispatch(), local_call("fixedOver", vec![var("plans")])),
                no(),
            ),
            list_induction(
                "plans",
                &[],
                reflexivity(),
                simplify(&[
                    "falseOr",
                    "fixedOver",
                    "hypothesis",
                    "selectorEq",
                    "selectorIn",
                ]),
            ),
        ),
        simp_lemma(
            "fixedInOver",
            vec![
                ("plan", nat_t()),
                ("level", nat_t()),
                ("origin", nat_t()),
                ("larges", nats()),
            ],
            eq(
                selector_in(
                    fixed(),
                    local_call(
                        "dispatchOver",
                        vec![var("level"), var("origin"), var("larges")],
                    ),
                ),
                no(),
            ),
            list_induction(
                "larges",
                &[],
                reflexivity(),
                simplify(&[
                    "dispatchOver",
                    "falseOr",
                    "hypothesis",
                    "selectorEq",
                    "selectorIn",
                ]),
            ),
        ),
        simp_lemma(
            "fixedInSmalls",
            vec![
                ("plan", nat_t()),
                ("level", nat_t()),
                ("smalls", nats()),
                ("larges", nats()),
            ],
            eq(
                selector_in(
                    fixed(),
                    local_call(
                        "dispatchSmalls",
                        vec![var("level"), var("smalls"), var("larges")],
                    ),
                ),
                no(),
            ),
            list_induction(
                "smalls",
                &[],
                reflexivity(),
                simplify(&[
                    "dispatchSmalls",
                    "falseOr",
                    "fixedInOver",
                    "hypothesis",
                    "selectorInAppend",
                ]),
            ),
        ),
        simp_lemma(
            "fixedInDispatch",
            vec![("plan", nat_t()), ("thresholds", nats()), ("plans", nats())],
            eq(
                selector_in(
                    fixed(),
                    local_call("dispatchThresholds", vec![var("thresholds"), var("plans")]),
                ),
                no(),
            ),
            list_induction(
                "thresholds",
                &[],
                reflexivity(),
                simplify(&[
                    "dispatchThresholds",
                    "falseOr",
                    "fixedInSmalls",
                    "hypothesis",
                    "selectorInAppend",
                ]),
            ),
        ),
        simp_lemma(
            "dispatchInOver",
            vec![
                ("threshold", nat_t()),
                ("small", nat_t()),
                ("large", nat_t()),
                ("level", nat_t()),
                ("origin", nat_t()),
                ("larges", nats()),
            ],
            eq(
                selector_in(
                    dispatch(),
                    local_call(
                        "dispatchOver",
                        vec![var("level"), var("origin"), var("larges")],
                    ),
                ),
                and(
                    beq(var("threshold"), var("level")),
                    and(
                        beq(var("small"), var("origin")),
                        nat_in(var("large"), var("larges")),
                    ),
                ),
            ),
            list_induction(
                "larges",
                &[],
                simplify(&["andFalse", "dispatchOver", "natIn", "selectorIn"]),
                simplify(&[
                    "andOverOr",
                    "dispatchOver",
                    "hypothesis",
                    "natIn",
                    "selectorEq",
                    "selectorIn",
                ]),
            ),
        ),
        simp_lemma(
            "dispatchInSmalls",
            vec![
                ("threshold", nat_t()),
                ("small", nat_t()),
                ("large", nat_t()),
                ("level", nat_t()),
                ("smalls", nats()),
                ("larges", nats()),
            ],
            eq(
                selector_in(
                    dispatch(),
                    local_call(
                        "dispatchSmalls",
                        vec![var("level"), var("smalls"), var("larges")],
                    ),
                ),
                and(
                    beq(var("threshold"), var("level")),
                    and(
                        nat_in(var("small"), var("smalls")),
                        nat_in(var("large"), var("larges")),
                    ),
                ),
            ),
            list_induction(
                "smalls",
                &[],
                simplify(&[
                    "andFalse",
                    "dispatchSmalls",
                    "falseAnd",
                    "natIn",
                    "selectorIn",
                ]),
                simplify(&[
                    "andOverOr",
                    "dispatchInOver",
                    "dispatchSmalls",
                    "hypothesis",
                    "natIn",
                    "orUnderAnd",
                    "selectorInAppend",
                ]),
            ),
        ),
        simp_lemma(
            "dispatchInThresholds",
            vec![
                ("threshold", nat_t()),
                ("small", nat_t()),
                ("large", nat_t()),
                ("thresholds", nats()),
                ("plans", nats()),
            ],
            eq(
                selector_in(
                    dispatch(),
                    local_call("dispatchThresholds", vec![var("thresholds"), var("plans")]),
                ),
                and(
                    nat_in(var("threshold"), var("thresholds")),
                    and(
                        nat_in(var("small"), var("plans")),
                        nat_in(var("large"), var("plans")),
                    ),
                ),
            ),
            list_induction(
                "thresholds",
                &[],
                reflexivity(),
                simplify(&[
                    "dispatchInSmalls",
                    "dispatchThresholds",
                    "hypothesis",
                    "natIn",
                    "orUnderAnd",
                    "selectorInAppend",
                ]),
            ),
        ),
        simp_lemma(
            "expandComplete",
            vec![("grammar", local_t("Grammar")), ("selector", selector_t())],
            eq(
                selector_in(var("selector"), local_call("expand", vec![var("grammar")])),
                local_call("wellFormed", vec![var("grammar"), var("selector")]),
            ),
            cases(
                "selector",
                vec![
                    proof_branch(
                        "fixed",
                        &["plan"],
                        simplify(&[
                            "expand",
                            "fixedInDispatch",
                            "fixedInFixed",
                            "natInRange",
                            "orFalse",
                            "selectorInAppend",
                            "wellFormed",
                        ]),
                    ),
                    proof_branch(
                        "dispatch",
                        &["threshold", "small", "large"],
                        simplify(&[
                            "dispatchInFixed",
                            "dispatchInThresholds",
                            "expand",
                            "falseOr",
                            "natInRange",
                            "selectorInAppend",
                            "wellFormed",
                        ]),
                    ),
                ],
            ),
        ),
    ]
}

// --- validation ------------------------------------------------------------

/// The claim must be answerable by the request's order: a scalar claim
/// over the total order, a vector claim over the componentwise one; the
/// §12.4 alias is refused for its base class with a displayed scope, and no
/// other claim is decided.
fn check_claim(unused: &Unused) -> Json {
    let objective = |scalar: Json, vector: Json| {
        matching(
            project(var("request"), "objective"),
            vec![
                arm("Objective.scalar", Vec::new(), scalar),
                arm("Objective.vector", Vec::new(), vector),
            ],
        )
    };
    let branches: Vec<Json> = claims()
        .into_iter()
        .map(|(claim, constants)| {
            let body = if SCALAR_CLAIMS.contains(&claim) {
                objective(pass(), reject("scalarClaimOverPartialOrder", Vec::new()))
            } else if VECTOR_CLAIMS.contains(&claim) {
                objective(reject("vectorClaimOverTotalOrder", Vec::new()), pass())
            } else if claim == ALIAS_CLAIM {
                reject("claimAlias", Vec::new())
            } else {
                reject("unsupportedClaim", Vec::new())
            };
            arm(
                &format!("ClaimClass.{claim}"),
                constants.iter().map(|_| unused.next()).collect(),
                body,
            )
        })
        .collect();
    matching(project(var("request"), "claim"), branches)
}

/// The request's use of the machine fits the capacity the machine binds:
/// its fuel, its domain, its universe, and every declared charge.
fn check_capacity() -> Json {
    let machine = || project(var("request"), "machine");
    let capacity = |field: &str| project(project(machine(), "capacity"), field);
    ite(
        or(
            or(
                blt(capacity("fuel"), project(machine(), "fuel")),
                blt(
                    capacity("domain"),
                    prim("length", vec![project(var("request"), "domain")], nat_t()),
                ),
            ),
            or(
                blt(
                    capacity("systems"),
                    prim(
                        "length",
                        vec![local_call("expand", vec![var("grammar")])],
                        nat_t(),
                    ),
                ),
                local_call(
                    "chargeBeyond",
                    vec![project(machine(), "actions"), capacity("charge")],
                ),
            ),
        ),
        reject("beyondCapacity", Vec::new()),
        pass(),
    )
}

fn validation(unused: &Unused) -> Vec<Json> {
    let request_t = || local_t("Request");
    let field = |name: &str| project(var("request"), name);
    let actions = || project(field("machine"), "actions");
    // `checkClaim` precedes `validate` in the module, so it draws its
    // unused names first.
    let claim = check_claim(unused);
    let on_grammar = |unused: &Unused, check: Json| {
        matching(
            field("carrier"),
            vec![
                arm("Carrier.grammar", binders!["grammar"], check),
                arm("Carrier.internalPlans", Vec::new(), pass()),
                arm("Carrier.optimizerOutput", Vec::new(), pass()),
                arm("Carrier.discovered", binders![unused.next()], pass()),
                arm("Carrier.cached", Vec::new(), pass()),
            ],
        )
    };
    let mut checks = vec![
        // §8.1: vacuous truth never establishes exactness or optimality.
        on_list(
            field("domain"),
            reject("emptyDomain", Vec::new()),
            unused.next(),
            unused.next(),
            pass(),
        ),
        matching(
            field("carrier"),
            vec![
                arm("Carrier.grammar", binders![unused.next()], pass()),
                arm(
                    "Carrier.internalPlans",
                    Vec::new(),
                    reject("internalPlanUniverse", Vec::new()),
                ),
                arm(
                    "Carrier.optimizerOutput",
                    Vec::new(),
                    reject("optimizerDefinedUniverse", Vec::new()),
                ),
                arm(
                    "Carrier.discovered",
                    binders![unused.next()],
                    reject("discoveredUniverse", Vec::new()),
                ),
                arm(
                    "Carrier.cached",
                    Vec::new(),
                    reject("cachedUniverse", Vec::new()),
                ),
            ],
        ),
        // `grammarEquality` cites `expandComplete`, which the kernel proves
        // for every grammar; any other evidence is refused.
        matching(
            field("completeness"),
            vec![
                arm("Completeness.grammarEquality", Vec::new(), pass()),
                arm(
                    "Completeness.missing",
                    Vec::new(),
                    reject("missingCompleteness", Vec::new()),
                ),
                arm(
                    "Completeness.citesUniverseId",
                    Vec::new(),
                    reject("selfReferentialCompleteness", Vec::new()),
                ),
                arm(
                    "Completeness.citesOptimizer",
                    Vec::new(),
                    reject("optimizerCompleteness", Vec::new()),
                ),
            ],
        ),
        on_grammar(unused, check_capacity()),
        // §8.2: a unit charge for a primitive whose work grows with its
        // operands is an unbounded action at unit cost.
        matching(
            project(field("machine"), "operandSize"),
            vec![
                arm("OperandSize.weighted", Vec::new(), pass()),
                arm(
                    "OperandSize.unit",
                    Vec::new(),
                    reject("unitCostOperands", Vec::new()),
                ),
            ],
        ),
        local_call("checkDistinct", vec![actions(), actions()]),
    ];
    checks.extend(PERFORMED.iter().map(|kind| {
        local_call(
            "checkPerformed",
            vec![
                field("machine"),
                local(&format!("ActionKind.{kind}"), Vec::new()),
            ],
        )
    }));
    checks.extend([
        local_call("checkAllDeclared", vec![field("machine"), actions()]),
        local_call(
            "checkPrepared",
            vec![field("machine"), project(field("machine"), "prepared")],
        ),
        on_grammar(
            unused,
            local_call(
                "checkPlanPreparation",
                vec![actions(), project(var("grammar"), "plans")],
            ),
        ),
        local_call("checkClaim", vec![var("request")]),
        matching(
            field("scope"),
            vec![
                arm("Scope.grammarUniverse", Vec::new(), pass()),
                arm(
                    "Scope.calculusPrograms",
                    Vec::new(),
                    reject("uncoveredScope", Vec::new()),
                ),
                arm(
                    "Scope.rustPrograms",
                    Vec::new(),
                    reject("uncoveredScope", Vec::new()),
                ),
            ],
        ),
    ]);
    vec![
        definition(
            "checkClaim",
            vec![parameter("request", request_t())],
            option_t(rejection_t()),
            claim,
        ),
        // §17.15: the first violated rule, in this order.
        definition(
            "validate",
            vec![parameter("request", request_t())],
            option_t(rejection_t()),
            first_rejection(unused, checks),
        ),
    ]
}

// --- realization -----------------------------------------------------------

/// `Expr.var 0`, the realization entry's one argument.
fn argument() -> Json {
    syntax("Expr.var", vec![nat(0)])
}
/// A call of the function at `index` on the entry's argument.
fn call_on_argument(index: Json) -> Json {
    syntax("Expr.call", vec![index, list(expr_t(), vec![argument()])])
}

fn realization() -> Vec<Json> {
    let length_below_threshold = syntax(
        "Expr.prim",
        vec![
            syntax("Prim.natLt", Vec::new()),
            list(
                expr_t(),
                vec![
                    syntax(
                        "Expr.prim",
                        vec![
                            syntax("Prim.length", Vec::new()),
                            list(expr_t(), vec![argument()]),
                        ],
                    ),
                    syntax(
                        "Expr.value",
                        vec![
                            syntax("Ty.nat", Vec::new()),
                            syntax("Value.nat", vec![var("threshold")]),
                        ],
                    ),
                ],
            ),
        ],
    );
    let entry = lx::record(
        term::member(SYNTAX, "Function"),
        vec![
            ("parameters", list(nat_t(), vec![nat(0)])),
            (
                "types",
                list(syntax_t("Ty"), vec![project(var("grammar"), "argument")]),
            ),
            ("result", project(var("grammar"), "result")),
            ("body", local_call("entryBody", vec![var("selector")])),
        ],
    );
    vec![
        // A system is the complete selector-plus-executor (§8.5): the entry
        // selector over the shared plans at indices 1.., so every system of
        // one grammar shares their code and differs only in its entry.
        definition(
            "entryBody",
            vec![parameter("selector", selector_t())],
            expr_t(),
            matching(
                var("selector"),
                vec![
                    arm(
                        "Selector.fixed",
                        binders!["plan"],
                        call_on_argument(add(var("plan"), nat(1))),
                    ),
                    arm(
                        "Selector.dispatch",
                        binders!["threshold", "small", "large"],
                        syntax(
                            "Expr.cond",
                            vec![
                                length_below_threshold,
                                call_on_argument(add(var("small"), nat(1))),
                                call_on_argument(add(var("large"), nat(1))),
                            ],
                        ),
                    ),
                ],
            ),
        ),
        recursive(
            "plans",
            definition(
                "planFunctions",
                vec![parameter("plans", list_t(local_t("Plan")))],
                list_t(syntax_t("Function")),
                on_list(
                    var("plans"),
                    nil(syntax_t("Function")),
                    "plan",
                    "rest",
                    cons(
                        project(var("plan"), "function"),
                        local_call("planFunctions", vec![var("rest")]),
                    ),
                ),
            ),
        ),
        definition(
            "realize",
            vec![
                parameter("grammar", local_t("Grammar")),
                parameter("selector", selector_t()),
            ],
            syntax_t("Program"),
            lx::record(
                term::member(SYNTAX, "Program"),
                vec![
                    ("adts", nil(syntax_t("Adt"))),
                    (
                        "functions",
                        cons(
                            entry,
                            local_call("planFunctions", vec![project(var("grammar"), "plans")]),
                        ),
                    ),
                ],
            ),
        ),
    ]
}

// --- value equality --------------------------------------------------------

/// Value equality, structurally over the nested `Value` family: equal
/// constructors with equal fields.
fn value_equality(unused: &Unused) -> Vec<Json> {
    let both_have = |constructor: &'static str,
                     left: &[&str],
                     right: &[&str],
                     equal: Json|
     -> (&'static str, Vec<String>, Json) {
        let right_side = on_value(
            unused,
            var("right"),
            vec![(constructor, binders_of(right), equal)],
            &no(),
        );
        (constructor, binders_of(left), right_side)
    };
    let same_item = |constructor: &'static str, equal: fn(Json, Json) -> Json| {
        both_have(
            constructor,
            &["leftItem"],
            &["rightItem"],
            equal(var("leftItem"), var("rightItem")),
        )
    };
    let decidable = |left: Json, right: Json| prim("equal", vec![left, right], lx::bool_t());
    let value_eq = |left: Json, right: Json| local_call("valueEq", vec![left, right]);
    let values_eq = |left: Json, right: Json| local_call("valuesEq", vec![left, right]);
    let same_order = |left: Json, right: Json| semantics_call("sameOrder", vec![left, right]);
    let same_int = |left: Json, right: Json| {
        semantics_call(
            "sameOrder",
            vec![
                semantics_call("orderInt", vec![left, right]),
                syntax("Order.same", Vec::new()),
            ],
        )
    };
    // Built in this order, which fixes the unused binder names; the match
    // itself lists constructors in declaration order.
    let mut cases = vec![
        both_have("unit", &[], &[], yes()),
        both_have("none", &[], &[], yes()),
        same_item("bool", decidable),
        same_item("nat", beq),
        same_item("string", decidable),
        same_item("bytes", decidable),
        same_item("int", same_int),
        same_item("ordering", same_order),
        same_item("some", value_eq),
        same_item("ok", value_eq),
        same_item("error", value_eq),
        same_item("list", values_eq),
        both_have(
            "pair",
            &["leftFirst", "leftSecond"],
            &["rightFirst", "rightSecond"],
            and(
                value_eq(var("leftFirst"), var("rightFirst")),
                value_eq(var("leftSecond"), var("rightSecond")),
            ),
        ),
        both_have(
            "adt",
            &["leftTag", "leftFields"],
            &["rightTag", "rightFields"],
            and(
                beq(var("leftTag"), var("rightTag")),
                values_eq(var("leftFields"), var("rightFields")),
            ),
        ),
        both_have(
            "closure",
            &["leftFunction", "leftCaptures"],
            &["rightFunction", "rightCaptures"],
            and(
                beq(var("leftFunction"), var("rightFunction")),
                values_eq(var("leftCaptures"), var("rightCaptures")),
            ),
        ),
    ];
    for fixed in ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"] {
        cases.push(same_item(fixed, decidable));
    }
    let value_eq_body = on_value(unused, var("left"), cases, &no());
    vec![
        mutual(
            "ValueEquality",
            recursive(
                "left",
                definition(
                    "valueEq",
                    vec![parameter("left", value_t()), parameter("right", value_t())],
                    lx::bool_t(),
                    value_eq_body,
                ),
            ),
        ),
        mutual(
            "ValueEquality",
            recursive(
                "left",
                definition(
                    "valuesEq",
                    vec![
                        parameter("left", list_t(value_t())),
                        parameter("right", list_t(value_t())),
                    ],
                    lx::bool_t(),
                    on_list(
                        var("left"),
                        on_list(var("right"), yes(), unused.next(), unused.next(), no()),
                        "leftHead",
                        "leftTail",
                        on_list(
                            var("right"),
                            no(),
                            "rightHead",
                            "rightTail",
                            and(
                                value_eq(var("leftHead"), var("rightHead")),
                                values_eq(var("leftTail"), var("rightTail")),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ]
}

fn binders_of(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_owned()).collect()
}

// --- the code a system can run ---------------------------------------------

/// A structural fold over `TargetSyntax.Expr` in the mutual group `group`:
/// `leaf` for a node without subexpressions, `node` for one with, and
/// `reference` for the function a call or closure names.
struct ExprFold<'a> {
    group: &'a str,
    expr: &'a str,
    exprs: &'a str,
    arms: &'a str,
    arm: &'a str,
    result: Json,
    empty: Json,
    /// Combine a node's own contribution with its children's.
    combine: fn(Json, Json) -> Json,
    /// A node's own contribution.
    own: Json,
    /// The contribution of a call or closure of `function`.
    reference: fn(Json) -> Json,
}

impl ExprFold<'_> {
    fn of(&self, name: &str) -> Json {
        local_call(self.expr, vec![var(name)])
    }
    #[allow(clippy::too_many_lines)]
    fn declarations(&self, unused: &Unused) -> Vec<Json> {
        let combine = self.combine;
        let own = || self.own.clone();
        let operands = |head: Json| combine(head, local_call(self.exprs, vec![var("operands")]));
        let expression = matching(
            var("expression"),
            vec![
                syntax_arm("Expr.value", binders![unused.next(), unused.next()], own()),
                syntax_arm("Expr.var", binders![unused.next()], own()),
                syntax_arm(
                    "Expr.let",
                    binders![unused.next(), unused.next(), "bound", "body"],
                    combine(own(), combine(self.of("bound"), self.of("body"))),
                ),
                syntax_arm(
                    "Expr.cond",
                    binders!["condition", "thenBranch", "elseBranch"],
                    combine(
                        own(),
                        combine(
                            self.of("condition"),
                            combine(self.of("thenBranch"), self.of("elseBranch")),
                        ),
                    ),
                ),
                syntax_arm(
                    "Expr.match",
                    binders![unused.next(), "scrutinee", "arms"],
                    combine(
                        own(),
                        combine(
                            self.of("scrutinee"),
                            local_call(self.arms, vec![var("arms")]),
                        ),
                    ),
                ),
                syntax_arm(
                    "Expr.build",
                    binders![unused.next(), unused.next(), "operands"],
                    operands(own()),
                ),
                syntax_arm(
                    "Expr.call",
                    binders!["function", "operands"],
                    operands((self.reference)(var("function"))),
                ),
                syntax_arm(
                    "Expr.closure",
                    binders!["function", "operands"],
                    operands((self.reference)(var("function"))),
                ),
                syntax_arm(
                    "Expr.apply",
                    binders!["target", "operands"],
                    combine(
                        own(),
                        combine(
                            self.of("target"),
                            local_call(self.exprs, vec![var("operands")]),
                        ),
                    ),
                ),
                syntax_arm(
                    "Expr.prim",
                    binders![unused.next(), "operands"],
                    operands(own()),
                ),
                syntax_arm(
                    "Expr.first",
                    binders!["inner"],
                    combine(own(), self.of("inner")),
                ),
                syntax_arm(
                    "Expr.second",
                    binders!["inner"],
                    combine(own(), self.of("inner")),
                ),
                syntax_arm(
                    "Expr.field",
                    binders!["inner", unused.next()],
                    combine(own(), self.of("inner")),
                ),
            ],
        );
        let arm_body = matching(
            var("arm"),
            vec![syntax_arm(
                "Arm.arm",
                binders![unused.next(), unused.next(), "body"],
                combine(own(), self.of("body")),
            )],
        );
        let over_list = |function: &str, element: &str, list_name: &str, item: Json| {
            recursive(
                list_name,
                mutual(
                    self.group,
                    definition(
                        function,
                        vec![parameter(list_name, list_t(syntax_t(element)))],
                        self.result.clone(),
                        on_list(
                            var(list_name),
                            self.empty.clone(),
                            "head",
                            "rest",
                            combine(item, local_call(function, vec![var("rest")])),
                        ),
                    ),
                ),
            )
        };
        vec![
            recursive(
                "expression",
                mutual(
                    self.group,
                    definition(
                        self.expr,
                        vec![parameter("expression", expr_t())],
                        self.result.clone(),
                        expression,
                    ),
                ),
            ),
            over_list(
                self.exprs,
                "Expr",
                "expressions",
                local_call(self.expr, vec![var("head")]),
            ),
            over_list(
                self.arms,
                "Arm",
                "arms",
                local_call(self.arm, vec![var("head")]),
            ),
            recursive(
                "arm",
                mutual(
                    self.group,
                    definition(
                        self.arm,
                        vec![parameter("arm", syntax_t("Arm"))],
                        self.result.clone(),
                        arm_body,
                    ),
                ),
            ),
        ]
    }
}

fn plus(left: Json, right: Json) -> Json {
    add(left, right)
}
fn concatenated(left: Json, right: Json) -> Json {
    append_all(nat_t(), left, right)
}
fn no_reference(_function: Json) -> Json {
    nat(1)
}
fn names_function(function: Json) -> Json {
    list(nat_t(), vec![function])
}

/// The size of the code a system can run, and the preparation of the plans
/// it can run: both range over the functions reachable from the entry, so
/// a plan no system of the selector can call is neither its size nor its
/// charge.
#[allow(clippy::too_many_lines)]
fn reachable_code(unused: &Unused) -> Vec<Json> {
    let functions_t = || list_t(syntax_t("Function"));
    let mut out = ExprFold {
        group: "ExpressionSize",
        expr: "exprSize",
        exprs: "exprsSize",
        arms: "armsSize",
        arm: "armSize",
        result: nat_t(),
        empty: nat(0),
        combine: plus,
        own: nat(1),
        reference: no_reference,
    }
    .declarations(unused);
    out.extend(
        ExprFold {
            group: "ExpressionCallees",
            expr: "exprCallees",
            exprs: "exprsCallees",
            arms: "armsCallees",
            arm: "armCallees",
            result: nats_t(),
            empty: nil(nat_t()),
            combine: concatenated,
            own: nil(nat_t()),
            reference: names_function,
        }
        .declarations(unused),
    );
    let at = |name: &str, element: Json| {
        recursive(
            "items",
            definition(
                name,
                vec![
                    parameter("items", list_t(element.clone())),
                    parameter("index", nat_t()),
                ],
                option_t(element.clone()),
                on_list(
                    var("items"),
                    none(element.clone()),
                    "head",
                    "rest",
                    on_nat(
                        var("index"),
                        some(element, var("head")),
                        local_call(name, vec![var("rest"), var("remaining")]),
                    ),
                ),
            ),
        )
    };
    out.extend([
        at("functionAt", syntax_t("Function")),
        at("planAt", local_t("Plan")),
        // The functions the known ones call or close over.
        recursive(
            "known",
            definition(
                "calleesOf",
                vec![
                    parameter("functions", functions_t()),
                    parameter("known", nats_t()),
                ],
                nats_t(),
                on_list(
                    var("known"),
                    nil(nat_t()),
                    "index",
                    "rest",
                    append_all(
                        nat_t(),
                        on_option(
                            local_call("functionAt", vec![var("functions"), var("index")]),
                            nil(nat_t()),
                            "function",
                            local_call("exprCallees", vec![project(var("function"), "body")]),
                        ),
                        local_call("calleesOf", vec![var("functions"), var("rest")]),
                    ),
                ),
            ),
        ),
        // `known` followed by every member of `found` it lacks, in order.
        recursive(
            "found",
            definition(
                "addNew",
                vec![parameter("known", nats_t()), parameter("found", nats_t())],
                nats_t(),
                on_list(
                    var("found"),
                    var("known"),
                    "index",
                    "rest",
                    ite(
                        local_call("natIn", vec![var("index"), var("known")]),
                        local_call("addNew", vec![var("known"), var("rest")]),
                        local_call(
                            "addNew",
                            vec![
                                append_all(
                                    nat_t(),
                                    var("known"),
                                    list(nat_t(), vec![var("index")]),
                                ),
                                var("rest"),
                            ],
                        ),
                    ),
                ),
            ),
        ),
        recursive(
            "rounds",
            definition(
                "closeOver",
                vec![
                    parameter("rounds", nat_t()),
                    parameter("functions", functions_t()),
                    parameter("known", nats_t()),
                ],
                nats_t(),
                on_nat(
                    var("rounds"),
                    var("known"),
                    local_call(
                        "closeOver",
                        vec![
                            var("remaining"),
                            var("functions"),
                            local_call(
                                "addNew",
                                vec![
                                    var("known"),
                                    local_call("calleesOf", vec![var("functions"), var("known")]),
                                ],
                            ),
                        ],
                    ),
                ),
            ),
        ),
        // Every function the entry can reach, in discovery order: each
        // round adds at least one function or changes nothing, so as many
        // rounds as functions reach the fixed point.
        definition(
            "reachable",
            vec![parameter("functions", functions_t())],
            nats_t(),
            local_call(
                "closeOver",
                vec![
                    prim("length", vec![var("functions")], nat_t()),
                    var("functions"),
                    list(nat_t(), vec![nat(0)]),
                ],
            ),
        ),
        // One per expression node and per arm of every reachable function.
        recursive(
            "indices",
            definition(
                "reachableSize",
                vec![
                    parameter("functions", functions_t()),
                    parameter("indices", nats_t()),
                ],
                nat_t(),
                on_list(
                    var("indices"),
                    nat(0),
                    "index",
                    "rest",
                    add(
                        on_option(
                            local_call("functionAt", vec![var("functions"), var("index")]),
                            nat(0),
                            "function",
                            local_call("exprSize", vec![project(var("function"), "body")]),
                        ),
                        local_call("reachableSize", vec![var("functions"), var("rest")]),
                    ),
                ),
            ),
        ),
        // The per-invocation preparation of every reachable plan; index 0
        // is the entry, plan `k` is function `k + 1`.
        recursive(
            "indices",
            definition(
                "reachableCharge",
                vec![
                    parameter("actions", list_t(local_t("Action"))),
                    parameter("plans", list_t(local_t("Plan"))),
                    parameter("indices", nats_t()),
                ],
                nat_t(),
                on_list(
                    var("indices"),
                    nat(0),
                    "index",
                    "rest",
                    add(
                        on_nat(
                            var("index"),
                            nat(0),
                            on_option(
                                local_call("planAt", vec![var("plans"), var("remaining")]),
                                nat(0),
                                "plan",
                                local_call(
                                    "planCharge",
                                    vec![var("actions"), project(var("plan"), "prepares")],
                                ),
                            ),
                        ),
                        local_call(
                            "reachableCharge",
                            vec![var("actions"), var("plans"), var("rest")],
                        ),
                    ),
                ),
            ),
        ),
    ]);
    out
}

// --- the order -------------------------------------------------------------

/// The generic order every answer and every authority vector is computed
/// by: rows pair an identity with its cost, a scalar cost is one natural,
/// a vector cost a list of them.
#[allow(clippy::too_many_lines)]
fn order() -> Vec<Json> {
    let scalar_rows_t = || list_t(row_t(nat_t()));
    let vector_rows_t = || list_t(row_t(nats_t()));
    let ids_t = || list_t(id_t());
    let same_t = || function_t(vec![id_t(), id_t()], lx::bool_t());
    let at_id = |function: &str, arguments: Vec<Json>| generic_call(function, id_t(), arguments);
    let cost_of = |row: &str| second(var(row));
    let id_of = |row: &str| first(var(row));
    let found_t = || product_t(ids_t(), nat_t());
    vec![
        // Componentwise at most, over vectors of one length; vectors of
        // different lengths are incomparable.
        recursive(
            "left",
            definition(
                "weaklyBelow",
                vec![parameter("left", nats_t()), parameter("right", nats_t())],
                lx::bool_t(),
                on_list(
                    var("left"),
                    on_list(var("right"), yes(), "ignoredHead", "ignoredRest", no()),
                    "leftHead",
                    "leftRest",
                    on_list(
                        var("right"),
                        no(),
                        "rightHead",
                        "rightRest",
                        and(
                            ble(var("leftHead"), var("rightHead")),
                            local_call("weaklyBelow", vec![var("leftRest"), var("rightRest")]),
                        ),
                    ),
                ),
            ),
        ),
        // §9.3: strict dominance, at most in every component and below in
        // one. Equal costs dominate neither way.
        definition(
            "dominates",
            vec![parameter("left", nats_t()), parameter("right", nats_t())],
            lx::bool_t(),
            and(
                local_call("weaklyBelow", vec![var("left"), var("right")]),
                not(local_call("weaklyBelow", vec![var("right"), var("left")])),
            ),
        ),
        generic(
            &[ID],
            recursive(
                "rows",
                definition(
                    "dominatedIn",
                    vec![
                        parameter("cost", nats_t()),
                        parameter("rows", vector_rows_t()),
                    ],
                    lx::bool_t(),
                    on_list(
                        var("rows"),
                        no(),
                        "row",
                        "rest",
                        or(
                            local_call("dominates", vec![cost_of("row"), var("cost")]),
                            at_id("dominatedIn", vec![var("cost"), var("rest")]),
                        ),
                    ),
                ),
            ),
        ),
        generic(
            &[ID],
            recursive(
                "candidates",
                definition(
                    "frontierFrom",
                    vec![
                        parameter("candidates", vector_rows_t()),
                        parameter("rows", vector_rows_t()),
                    ],
                    ids_t(),
                    on_list(
                        var("candidates"),
                        nil(id_t()),
                        "row",
                        "rest",
                        ite(
                            at_id("dominatedIn", vec![cost_of("row"), var("rows")]),
                            at_id("frontierFrom", vec![var("rest"), var("rows")]),
                            cons(
                                id_of("row"),
                                at_id("frontierFrom", vec![var("rest"), var("rows")]),
                            ),
                        ),
                    ),
                ),
            ),
        ),
        // §10.2: every identity no row strictly dominates, in row order.
        generic(
            &[ID],
            definition(
                "frontier",
                vec![parameter("rows", vector_rows_t())],
                ids_t(),
                at_id("frontierFrom", vec![var("rows"), var("rows")]),
            ),
        ),
        generic(
            &[ID],
            recursive(
                "rows",
                definition(
                    "scalarMinimum",
                    vec![
                        parameter("rows", scalar_rows_t()),
                        parameter("best", nat_t()),
                    ],
                    nat_t(),
                    on_list(
                        var("rows"),
                        var("best"),
                        "row",
                        "rest",
                        at_id(
                            "scalarMinimum",
                            vec![
                                var("rest"),
                                ite(
                                    blt(cost_of("row"), var("best")),
                                    cost_of("row"),
                                    var("best"),
                                ),
                            ],
                        ),
                    ),
                ),
            ),
        ),
        generic(
            &[ID],
            recursive(
                "rows",
                definition(
                    "attaining",
                    vec![
                        parameter("rows", scalar_rows_t()),
                        parameter("value", nat_t()),
                    ],
                    ids_t(),
                    on_list(
                        var("rows"),
                        nil(id_t()),
                        "row",
                        "rest",
                        ite(
                            beq(cost_of("row"), var("value")),
                            cons(
                                id_of("row"),
                                at_id("attaining", vec![var("rest"), var("value")]),
                            ),
                            at_id("attaining", vec![var("rest"), var("value")]),
                        ),
                    ),
                ),
            ),
        ),
        // §10.1: every identity attaining the least cost, in row order,
        // and that cost; nothing for no rows.
        generic(
            &[ID],
            definition(
                "argmin",
                vec![parameter("rows", scalar_rows_t())],
                option_t(found_t()),
                on_list(
                    var("rows"),
                    none(found_t()),
                    "head",
                    "ignoredRest",
                    let_in(
                        "best",
                        nat_t(),
                        at_id("scalarMinimum", vec![var("rows"), cost_of("head")]),
                        some(
                            found_t(),
                            pair(
                                at_id("attaining", vec![var("rows"), var("best")]),
                                var("best"),
                            ),
                        ),
                    ),
                ),
            ),
        ),
        generic(
            &[ID],
            recursive(
                "items",
                definition(
                    "anyBy",
                    vec![
                        parameter("same", same_t()),
                        parameter("item", id_t()),
                        parameter("items", ids_t()),
                    ],
                    lx::bool_t(),
                    on_list(
                        var("items"),
                        no(),
                        "head",
                        "rest",
                        or(
                            lx::apply(var("same"), vec![var("item"), var("head")]),
                            at_id("anyBy", vec![var("same"), var("item"), var("rest")]),
                        ),
                    ),
                ),
            ),
        ),
        generic(
            &[ID],
            recursive(
                "items",
                definition(
                    "allIn",
                    vec![
                        parameter("same", same_t()),
                        parameter("items", ids_t()),
                        parameter("others", ids_t()),
                    ],
                    lx::bool_t(),
                    on_list(
                        var("items"),
                        yes(),
                        "head",
                        "rest",
                        and(
                            at_id("anyBy", vec![var("same"), var("head"), var("others")]),
                            at_id("allIn", vec![var("same"), var("rest"), var("others")]),
                        ),
                    ),
                ),
            ),
        ),
        // The same identity set: an answer is identity-complete only with
        // every member and no other (GNAF-REJ-21, GNAF-REJ-29).
        generic(
            &[ID],
            definition(
                "sameIdsBy",
                vec![
                    parameter("same", same_t()),
                    parameter("left", ids_t()),
                    parameter("right", ids_t()),
                ],
                lx::bool_t(),
                and(
                    at_id("allIn", vec![var("same"), var("left"), var("right")]),
                    at_id("allIn", vec![var("same"), var("right"), var("left")]),
                ),
            ),
        ),
        // §12.7: a claimed argmin is certified exactly when it is the
        // complete argmin at its cost.
        generic(
            &[ID],
            definition(
                "certifiesArgmin",
                vec![
                    parameter("same", same_t()),
                    parameter("rows", scalar_rows_t()),
                    parameter("claimed", ids_t()),
                    parameter("cost", nat_t()),
                ],
                lx::bool_t(),
                on_option(
                    at_id("argmin", vec![var("rows")]),
                    no(),
                    "found",
                    and(
                        beq(second(var("found")), var("cost")),
                        at_id(
                            "sameIdsBy",
                            vec![var("same"), first(var("found")), var("claimed")],
                        ),
                    ),
                ),
            ),
        ),
        generic(
            &[ID],
            definition(
                "certifiesFrontier",
                vec![
                    parameter("same", same_t()),
                    parameter("rows", vector_rows_t()),
                    parameter("claimed", ids_t()),
                ],
                lx::bool_t(),
                at_id(
                    "sameIdsBy",
                    vec![
                        var("same"),
                        at_id("frontier", vec![var("rows")]),
                        var("claimed"),
                    ],
                ),
            ),
        ),
        definition(
            "vectorEq",
            vec![parameter("left", nats_t()), parameter("right", nats_t())],
            lx::bool_t(),
            and(
                local_call("weaklyBelow", vec![var("left"), var("right")]),
                local_call("weaklyBelow", vec![var("right"), var("left")]),
            ),
        ),
        // Some row costs exactly `cost`.
        generic(
            &[ID],
            recursive(
                "rows",
                definition(
                    "attains",
                    vec![
                        parameter("rows", vector_rows_t()),
                        parameter("cost", nats_t()),
                    ],
                    lx::bool_t(),
                    on_list(
                        var("rows"),
                        no(),
                        "row",
                        "rest",
                        or(
                            local_call("vectorEq", vec![cost_of("row"), var("cost")]),
                            at_id("attains", vec![var("rest"), var("cost")]),
                        ),
                    ),
                ),
            ),
        ),
        recursive(
            "left",
            definition(
                "pointwiseMinimum",
                vec![parameter("left", nats_t()), parameter("right", nats_t())],
                nats_t(),
                on_list(
                    var("left"),
                    nil(nat_t()),
                    "leftHead",
                    "leftRest",
                    on_list(
                        var("right"),
                        nil(nat_t()),
                        "rightHead",
                        "rightRest",
                        cons(
                            ite(
                                blt(var("rightHead"), var("leftHead")),
                                var("rightHead"),
                                var("leftHead"),
                            ),
                            local_call("pointwiseMinimum", vec![var("leftRest"), var("rightRest")]),
                        ),
                    ),
                ),
            ),
        ),
        generic(
            &[ID],
            recursive(
                "rows",
                definition(
                    "minimumFrom",
                    vec![
                        parameter("rows", vector_rows_t()),
                        parameter("minimum", nats_t()),
                    ],
                    nats_t(),
                    on_list(
                        var("rows"),
                        var("minimum"),
                        "row",
                        "rest",
                        at_id(
                            "minimumFrom",
                            vec![
                                var("rest"),
                                local_call(
                                    "pointwiseMinimum",
                                    vec![var("minimum"), cost_of("row")],
                                ),
                            ],
                        ),
                    ),
                ),
            ),
        ),
        // §9.3: the least value of every component, each possibly from a
        // different row; a vector optimum must be attained, so these minima
        // are one only if some row attains them.
        generic(
            &[ID],
            definition(
                "componentwiseMinimum",
                vec![parameter("rows", vector_rows_t())],
                nats_t(),
                on_list(
                    var("rows"),
                    nil(nat_t()),
                    "head",
                    "ignoredRest",
                    at_id("minimumFrom", vec![var("rows"), cost_of("head")]),
                ),
            ),
        ),
        definition(
            "natSame",
            vec![parameter("left", nat_t()), parameter("right", nat_t())],
            lx::bool_t(),
            beq(var("left"), var("right")),
        ),
    ]
}

// --- admission and cost ----------------------------------------------------

/// `TargetSemantics.run fuel program 0 [argument]`: the program's entry on
/// one domain argument.
fn run(program: Json, argument: Json, fuel: Json) -> Json {
    semantics_call(
        "run",
        vec![fuel, program, nat(0), list(value_t(), vec![argument])],
    )
}
/// A match on an outcome: a value with its steps, or a failure whose kind
/// (`overflow`, `stuck`, `exhausted`) selects the body.
fn on_outcome(
    unused: &Unused,
    outcome: Json,
    value: &str,
    steps: impl Into<String>,
    produced: Json,
    failed: impl Fn(&str) -> Json,
) -> Json {
    matching(
        outcome,
        vec![
            semantics_arm(
                "Outcome.value",
                vec![value.to_owned(), steps.into()],
                produced,
            ),
            semantics_arm(
                "Outcome.overflow",
                binders![unused.next()],
                failed("overflow"),
            ),
            semantics_arm("Outcome.stuck", Vec::new(), failed("stuck")),
            semantics_arm("Outcome.exhausted", Vec::new(), failed("exhausted")),
        ],
    )
}
/// Carry a status through unchanged except for `admitted`.
fn on_status(scrutinee: Json, admitted_binders: Vec<String>, admitted: Json) -> Json {
    matching(
        scrutinee,
        vec![
            arm("Status.admitted", admitted_binders, admitted),
            arm(
                "Status.inadmissible",
                Vec::new(),
                status("inadmissible", Vec::new()),
            ),
            arm(
                "Status.unresolved",
                Vec::new(),
                status("unresolved", Vec::new()),
            ),
        ],
    )
}

#[allow(clippy::too_many_lines)]
fn admission(unused: &Unused) -> Vec<Json> {
    let field = |name: &str| project(var("request"), name);
    let machine = |name: &str| project(field("machine"), name);
    let unresolved = || status("unresolved", Vec::new());
    let checked = on_outcome(
        unused,
        run(var("system"), var("argument"), var("fuel")),
        "produced",
        "steps",
        on_outcome(
            unused,
            run(var("reference"), var("argument"), var("fuel")),
            "expected",
            unused.next(),
            ite(
                local_call("valueEq", vec![var("produced"), var("expected")]),
                on_status(
                    local_call(
                        "statusOn",
                        vec![var("system"), var("reference"), var("fuel"), var("rest")],
                    ),
                    binders!["restSteps", "size"],
                    status(
                        "admitted",
                        vec![add(var("steps"), var("restSteps")), var("size")],
                    ),
                ),
                status("inadmissible", Vec::new()),
            ),
            |_| unresolved(),
        ),
        // Only fuel running out leaves the system's correctness unknown;
        // overflowing or getting stuck where the reference does not is a
        // wrong answer.
        |failure| {
            if failure == "exhausted" {
                unresolved()
            } else {
                status("inadmissible", Vec::new())
            }
        },
    );
    let functions = || project(var("system"), "functions");
    let per_invocation = prim(
        "multiply",
        vec![
            prim("length", vec![field("domain")], nat_t()),
            local_call(
                "reachableCharge",
                vec![
                    machine("actions"),
                    project(var("grammar"), "plans"),
                    var("indices"),
                ],
            ),
        ],
        nat_t(),
    );
    let costed = on_status(
        local_call(
            "statusOn",
            vec![
                var("system"),
                field("reference"),
                machine("fuel"),
                field("domain"),
            ],
        ),
        binders!["steps", unused.next()],
        status(
            "admitted",
            vec![
                add(var("steps"), per_invocation),
                local_call("reachableSize", vec![functions(), var("indices")]),
            ],
        ),
    );
    vec![
        // §8.4: admission is correctness on every domain invocation; an
        // exhausted evaluation leaves eligibility unknown, never false.
        axioms(
            &RUN_AXIOMS,
            recursive(
                "domain",
                definition(
                    "statusOn",
                    vec![
                        parameter("system", syntax_t("Program")),
                        parameter("reference", syntax_t("Program")),
                        parameter("fuel", nat_t()),
                        parameter("domain", list_t(value_t())),
                    ],
                    status_t(),
                    on_list(
                        var("domain"),
                        status("admitted", vec![nat(0), nat(0)]),
                        "argument",
                        "rest",
                        checked,
                    ),
                ),
            ),
        ),
        // The cost of an admitted system: its steps over the domain plus
        // the preparation of every plan it can run on every invocation,
        // and the size of the code it can run.
        axioms(
            &RUN_AXIOMS,
            definition(
                "status",
                vec![
                    parameter("request", local_t("Request")),
                    parameter("grammar", local_t("Grammar")),
                    parameter("selector", selector_t()),
                ],
                status_t(),
                let_in(
                    "system",
                    syntax_t("Program"),
                    local_call("realize", vec![var("grammar"), var("selector")]),
                    let_in(
                        "indices",
                        nats_t(),
                        local_call("reachable", vec![functions()]),
                        costed,
                    ),
                ),
            ),
        ),
        axioms(
            &RUN_AXIOMS,
            recursive(
                "selectors",
                definition(
                    "statuses",
                    vec![
                        parameter("request", local_t("Request")),
                        parameter("grammar", local_t("Grammar")),
                        parameter("selectors", list_t(selector_t())),
                        parameter("next", nat_t()),
                    ],
                    list_t(product_t(nat_t(), status_t())),
                    on_list(
                        var("selectors"),
                        nil(product_t(nat_t(), status_t())),
                        "selector",
                        "rest",
                        cons(
                            pair(
                                var("next"),
                                local_call(
                                    "status",
                                    vec![var("request"), var("grammar"), var("selector")],
                                ),
                            ),
                            local_call(
                                "statuses",
                                vec![
                                    var("request"),
                                    var("grammar"),
                                    var("rest"),
                                    add(var("next"), nat(1)),
                                ],
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ]
}

// --- the answer ------------------------------------------------------------

/// A system's index paired with its status.
fn entry_t() -> Json {
    product_t(nat_t(), status_t())
}
/// An admitted system's index paired with its (steps, size).
fn costed_t() -> Json {
    product_t(nat_t(), nats_t())
}

/// The statuses of every system of `grammar`, in expansion order.
fn evaluated(request: Json, grammar: Json) -> Json {
    local_call(
        "statuses",
        vec![
            request,
            grammar.clone(),
            local_call("expand", vec![grammar]),
            nat(0),
        ],
    )
}

/// A match on the request's carrier: `grammar` binds the grammar, every
/// other carrier is `otherwise`.
fn on_carrier(unused: &Unused, grammar: Json, otherwise: &Json) -> Json {
    matching(
        project(var("request"), "carrier"),
        vec![
            arm("Carrier.grammar", binders!["grammar"], grammar),
            arm("Carrier.internalPlans", Vec::new(), otherwise.clone()),
            arm("Carrier.optimizerOutput", Vec::new(), otherwise.clone()),
            arm(
                "Carrier.discovered",
                binders![unused.next()],
                otherwise.clone(),
            ),
            arm("Carrier.cached", Vec::new(), otherwise.clone()),
        ],
    )
}

#[allow(clippy::too_many_lines)]
fn answer(unused: &Unused) -> Vec<Json> {
    let nat_rows =
        |function: &str, arguments: Vec<Json>| generic_call(function, nat_t(), arguments);
    let answers_t = || local_t("Answer");
    let same_ids = |left: Json, right: Json| {
        nat_rows(
            "sameIdsBy",
            vec![function_ref(lx::member("natSame")), left, right],
        )
    };
    // Only an argmin certifies an argmin and only a frontier a frontier.
    let claimed_as = |unused: &Unused, kind: &str, certified: Json| {
        let other = |name: &str, fields: usize, unused: &Unused| {
            arm(
                &format!("Answer.{name}"),
                (0..fields).map(|_| unused.next()).collect(),
                no(),
            )
        };
        let branches: Vec<Json> = [
            ("rejected", 1),
            ("argmin", 2),
            ("frontier", 1),
            ("infeasible", 0),
            ("incomplete", 0),
        ]
        .into_iter()
        .map(|(name, fields)| {
            if name == kind {
                let binders = if fields == 2 {
                    binders!["claimedMembers", "claimedSteps"]
                } else {
                    binders!["claimedMembers"]
                };
                arm(&format!("Answer.{name}"), binders, certified.clone())
            } else {
                other(name, fields, unused)
            }
        })
        .collect();
        matching(var("claimed"), branches)
    };
    let certified_argmin = claimed_as(
        unused,
        "argmin",
        and(
            beq(var("steps"), var("claimedSteps")),
            same_ids(var("members"), var("claimedMembers")),
        ),
    );
    let certified_frontier = claimed_as(
        unused,
        "frontier",
        same_ids(var("members"), var("claimedMembers")),
    );
    let certified = matching(
        local_call("answer", vec![var("request")]),
        vec![
            arm("Answer.rejected", binders![unused.next()], no()),
            arm(
                "Answer.argmin",
                binders!["members", "steps"],
                certified_argmin,
            ),
            arm("Answer.frontier", binders!["members"], certified_frontier),
            arm("Answer.infeasible", Vec::new(), no()),
            arm("Answer.incomplete", Vec::new(), no()),
        ],
    );
    let evaluate_body = let_in(
        "entries",
        list_t(entry_t()),
        evaluated(var("request"), var("grammar")),
        ite(
            local_call("anyUnresolved", vec![var("entries")]),
            local("Answer.incomplete", Vec::new()),
            let_in(
                "rows",
                list_t(costed_t()),
                local_call("admittedRows", vec![var("entries")]),
                matching(
                    project(var("request"), "objective"),
                    vec![
                        arm(
                            "Objective.scalar",
                            Vec::new(),
                            on_option(
                                nat_rows(
                                    "argmin",
                                    vec![local_call("scalarRows", vec![var("rows")])],
                                ),
                                local("Answer.infeasible", Vec::new()),
                                "found",
                                local(
                                    "Answer.argmin",
                                    vec![first(var("found")), second(var("found"))],
                                ),
                            ),
                        ),
                        arm(
                            "Objective.vector",
                            Vec::new(),
                            on_list(
                                var("rows"),
                                local("Answer.infeasible", Vec::new()),
                                unused.next(),
                                unused.next(),
                                local(
                                    "Answer.frontier",
                                    vec![nat_rows("frontier", vec![var("rows")])],
                                ),
                            ),
                        ),
                    ],
                ),
            ),
        ),
    );
    let answer_body = on_option(
        local_call("validate", vec![var("request")]),
        on_carrier(
            unused,
            local_call("evaluate", vec![var("request"), var("grammar")]),
            &local("Answer.incomplete", Vec::new()),
        ),
        "rejection",
        local("Answer.rejected", vec![var("rejection")]),
    );
    let minima_body = on_carrier(
        unused,
        let_in(
            "rows",
            list_t(costed_t()),
            local_call(
                "admittedRows",
                vec![evaluated(var("request"), var("grammar"))],
            ),
            nat_rows(
                "attains",
                vec![
                    var("rows"),
                    nat_rows("componentwiseMinimum", vec![var("rows")]),
                ],
            ),
        ),
        &no(),
    );
    let universe_body = on_carrier(
        unused,
        local_call("expand", vec![var("grammar")]),
        &nil(selector_t()),
    );
    let statuses_body = on_carrier(
        unused,
        evaluated(var("request"), var("grammar")),
        &nil(entry_t()),
    );
    vec![
        recursive(
            "entries",
            definition(
                "anyUnresolved",
                vec![parameter("entries", list_t(entry_t()))],
                lx::bool_t(),
                on_list(
                    var("entries"),
                    no(),
                    "entry",
                    "rest",
                    matching(
                        second(var("entry")),
                        vec![
                            arm("Status.unresolved", Vec::new(), yes()),
                            arm(
                                "Status.admitted",
                                binders![unused.next(), unused.next()],
                                local_call("anyUnresolved", vec![var("rest")]),
                            ),
                            arm(
                                "Status.inadmissible",
                                Vec::new(),
                                local_call("anyUnresolved", vec![var("rest")]),
                            ),
                        ],
                    ),
                ),
            ),
        ),
        // The admitted systems as rows of the order, costed (steps, size).
        recursive(
            "entries",
            definition(
                "admittedRows",
                vec![parameter("entries", list_t(entry_t()))],
                list_t(costed_t()),
                on_list(
                    var("entries"),
                    nil(costed_t()),
                    "entry",
                    "rest",
                    matching(
                        second(var("entry")),
                        vec![
                            arm(
                                "Status.admitted",
                                binders!["steps", "size"],
                                cons(
                                    pair(
                                        first(var("entry")),
                                        list(nat_t(), vec![var("steps"), var("size")]),
                                    ),
                                    local_call("admittedRows", vec![var("rest")]),
                                ),
                            ),
                            arm(
                                "Status.inadmissible",
                                Vec::new(),
                                local_call("admittedRows", vec![var("rest")]),
                            ),
                            arm(
                                "Status.unresolved",
                                Vec::new(),
                                local_call("admittedRows", vec![var("rest")]),
                            ),
                        ],
                    ),
                ),
            ),
        ),
        // The scalar objective of each row: its steps.
        recursive(
            "rows",
            definition(
                "scalarRows",
                vec![parameter("rows", list_t(costed_t()))],
                list_t(product_t(nat_t(), nat_t())),
                on_list(
                    var("rows"),
                    nil(product_t(nat_t(), nat_t())),
                    "row",
                    "rest",
                    cons(
                        pair(
                            first(var("row")),
                            on_list(
                                second(var("row")),
                                nat(0),
                                "steps",
                                unused.next(),
                                var("steps"),
                            ),
                        ),
                        local_call("scalarRows", vec![var("rest")]),
                    ),
                ),
            ),
        ),
        // An unknown cost anywhere leaves the answer incomplete: the
        // unresolved system might be the optimum. Otherwise the generic
        // order answers.
        axioms(
            &RUN_AXIOMS,
            definition(
                "evaluate",
                vec![
                    parameter("request", local_t("Request")),
                    parameter("grammar", local_t("Grammar")),
                ],
                answers_t(),
                evaluate_body,
            ),
        ),
        // The answer: a rejected request, or the request's grammar universe
        // evaluated by the calculus denotation.
        axioms(
            &RUN_AXIOMS,
            definition(
                "answer",
                vec![parameter("request", local_t("Request"))],
                answers_t(),
                answer_body,
            ),
        ),
        // §12.7: whether a claimed answer is exactly the request's optimum,
        // identity-complete; only optimality answers are certified.
        axioms(
            &RUN_AXIOMS,
            definition(
                "certifies",
                vec![
                    parameter("request", local_t("Request")),
                    parameter("claimed", answers_t()),
                ],
                lx::bool_t(),
                certified,
            ),
        ),
        // §9.3, GNAF-REJ-14: whether some admitted system attains the
        // componentwise minima of the admitted costs.
        axioms(
            &RUN_AXIOMS,
            definition(
                "minimaAttained",
                vec![parameter("request", local_t("Request"))],
                lx::bool_t(),
                minima_body,
            ),
        ),
        // The request's universe, before any evaluation.
        definition(
            "universeOf",
            vec![parameter("request", local_t("Request"))],
            list_t(selector_t()),
            universe_body,
        ),
        axioms(
            &RUN_AXIOMS,
            definition(
                "statusesOf",
                vec![parameter("request", local_t("Request"))],
                list_t(entry_t()),
                statuses_body,
            ),
        ),
    ]
}

// --- the authority's normative fixtures -------------------------------------

/// An operation of GNAF-VEC-01: `(identity, (from, (to, cost)))`.
fn operation_t() -> Json {
    product_t(nat_t(), product_t(nat_t(), product_t(nat_t(), nat_t())))
}
/// A partial composition: `(path, (end, cost))`.
fn walk_t() -> Json {
    product_t(nats_t(), product_t(nat_t(), nat_t()))
}

/// GNAF-VEC-01's grammar, the well-typed finite compositions of the
/// operations, by layers of one more operation each; and GNAF-VEC-04's
/// local rewriting and GNAF-VEC-17's hidden environment.
#[allow(clippy::too_many_lines)]
fn vector_definitions() -> Vec<Json> {
    let op = |part: fn(Json) -> Json| part(var("operation"));
    let from = |operation: Json| first(second(operation));
    let to = |operation: Json| first(second(second(operation)));
    let cost = |operation: Json| second(second(second(operation)));
    vec![
        // Every operation leaving `end` extends the walk.
        recursive(
            "operations",
            definition(
                "extendWalk",
                vec![
                    parameter("operations", list_t(operation_t())),
                    parameter("walk", walk_t()),
                ],
                list_t(walk_t()),
                on_list(
                    var("operations"),
                    nil(walk_t()),
                    "operation",
                    "rest",
                    ite(
                        beq(op(from), first(second(var("walk")))),
                        cons(
                            pair(
                                append_all(
                                    nat_t(),
                                    first(var("walk")),
                                    list(nat_t(), vec![first(var("operation"))]),
                                ),
                                pair(op(to), add(second(second(var("walk"))), op(cost))),
                            ),
                            local_call("extendWalk", vec![var("rest"), var("walk")]),
                        ),
                        local_call("extendWalk", vec![var("rest"), var("walk")]),
                    ),
                ),
            ),
        ),
        recursive(
            "walks",
            definition(
                "extendAll",
                vec![
                    parameter("operations", list_t(operation_t())),
                    parameter("walks", list_t(walk_t())),
                ],
                list_t(walk_t()),
                on_list(
                    var("walks"),
                    nil(walk_t()),
                    "walk",
                    "rest",
                    append_all(
                        walk_t(),
                        local_call("extendWalk", vec![var("operations"), var("walk")]),
                        local_call("extendAll", vec![var("operations"), var("rest")]),
                    ),
                ),
            ),
        ),
        // The walks ending at `target`, as rows (path, cost).
        recursive(
            "walks",
            definition(
                "arrivals",
                vec![
                    parameter("target", nat_t()),
                    parameter("walks", list_t(walk_t())),
                ],
                list_t(product_t(nats_t(), nat_t())),
                on_list(
                    var("walks"),
                    nil(product_t(nats_t(), nat_t())),
                    "walk",
                    "rest",
                    ite(
                        beq(first(second(var("walk"))), var("target")),
                        cons(
                            pair(first(var("walk")), second(second(var("walk")))),
                            local_call("arrivals", vec![var("target"), var("rest")]),
                        ),
                        local_call("arrivals", vec![var("target"), var("rest")]),
                    ),
                ),
            ),
        ),
        // The compositions reaching `target` with at most `depth - 1`
        // operations, from the walks `layer` has.
        recursive(
            "depth",
            definition(
                "compositions",
                vec![
                    parameter("depth", nat_t()),
                    parameter("operations", list_t(operation_t())),
                    parameter("target", nat_t()),
                    parameter("layer", list_t(walk_t())),
                ],
                list_t(product_t(nats_t(), nat_t())),
                on_nat(
                    var("depth"),
                    nil(product_t(nats_t(), nat_t())),
                    append_all(
                        product_t(nats_t(), nat_t()),
                        local_call("arrivals", vec![var("target"), var("layer")]),
                        local_call(
                            "compositions",
                            vec![
                                var("remaining"),
                                var("operations"),
                                var("target"),
                                local_call("extendAll", vec![var("operations"), var("layer")]),
                            ],
                        ),
                    ),
                ),
            ),
        ),
        // A one-step local rewrite system: `normalize` follows rewrites
        // until no rule applies, within `fuel` steps.
        recursive(
            "rules",
            definition(
                "rewriteOnce",
                vec![
                    parameter("rules", list_t(product_t(nat_t(), nat_t()))),
                    parameter("item", nat_t()),
                ],
                option_t(nat_t()),
                on_list(
                    var("rules"),
                    none(nat_t()),
                    "rule",
                    "rest",
                    ite(
                        beq(first(var("rule")), var("item")),
                        some(nat_t(), second(var("rule"))),
                        local_call("rewriteOnce", vec![var("rest"), var("item")]),
                    ),
                ),
            ),
        ),
        recursive(
            "fuel",
            definition(
                "normalize",
                vec![
                    parameter("fuel", nat_t()),
                    parameter("rules", list_t(product_t(nat_t(), nat_t()))),
                    parameter("item", nat_t()),
                ],
                nat_t(),
                on_nat(
                    var("fuel"),
                    var("item"),
                    on_option(
                        local_call("rewriteOnce", vec![var("rules"), var("item")]),
                        var("item"),
                        "next",
                        local_call(
                            "normalize",
                            vec![var("remaining"), var("rules"), var("next")],
                        ),
                    ),
                ),
            ),
        ),
        // GNAF-VEC-17: system `system` costs 0 when it equals the hidden bit
        // and 10 otherwise.
        definition(
            "hiddenBitCost",
            vec![parameter("system", nat_t()), parameter("hidden", nat_t())],
            nat_t(),
            ite(beq(var("system"), var("hidden")), nat(0), nat(10)),
        ),
        // The two systems' costs once the environment chose `hidden`.
        definition(
            "environmentRows",
            vec![parameter("hidden", nat_t())],
            list_t(product_t(nat_t(), nat_t())),
            list(
                product_t(nat_t(), nat_t()),
                (0..2)
                    .map(|system| {
                        pair(
                            nat(system),
                            local_call("hiddenBitCost", vec![nat(system), var("hidden")]),
                        )
                    })
                    .collect(),
            ),
        ),
        // A uniform system acts before the bit is known, so its guarantee is
        // its worst case over both environments.
        definition(
            "worstCase",
            vec![parameter("system", nat_t())],
            nat_t(),
            let_in(
                "atZero",
                nat_t(),
                local_call("hiddenBitCost", vec![var("system"), nat(0)]),
                let_in(
                    "atOne",
                    nat_t(),
                    local_call("hiddenBitCost", vec![var("system"), nat(1)]),
                    ite(
                        blt(var("atZero"), var("atOne")),
                        var("atOne"),
                        var("atZero"),
                    ),
                ),
            ),
        ),
        definition(
            "uniformRows",
            Vec::new(),
            list_t(product_t(nat_t(), nat_t())),
            list(
                product_t(nat_t(), nat_t()),
                (0..2)
                    .map(|system| pair(nat(system), local_call("worstCase", vec![nat(system)])))
                    .collect(),
            ),
        ),
        // The fair randomized system's expected cost once the environment
        // chose `hidden`: the mean over both systems.
        definition(
            "fairExpectation",
            vec![parameter("hidden", nat_t())],
            nat_t(),
            prim(
                "quotient",
                vec![
                    add(
                        local_call("hiddenBitCost", vec![nat(0), var("hidden")]),
                        local_call("hiddenBitCost", vec![nat(1), var("hidden")]),
                    ),
                    nat(2),
                    // The divisor is the two systems, never zero.
                    nat(0),
                ],
                nat_t(),
            ),
        ),
    ]
}

fn nats(numbers: &[u64]) -> Json {
    list(nat_t(), numbers.iter().map(|number| nat(*number)).collect())
}
/// A scalar cost table: `(identity, cost)` rows.
pub fn scalar_table(rows: &[(u64, u64)]) -> Json {
    list(
        product_t(nat_t(), nat_t()),
        rows.iter()
            .map(|(identity, cost)| pair(nat(*identity), nat(*cost)))
            .collect(),
    )
}
/// A vector cost table: `(identity, costs)` rows.
pub fn vector_table(rows: &[(u64, &[u64])]) -> Json {
    list(
        product_t(nat_t(), nats_t()),
        rows.iter()
            .map(|(identity, costs)| pair(nat(*identity), nats(costs)))
            .collect(),
    )
}
/// GNAF-VEC-01's operations `(identity, from, to, cost)`.
pub fn operations(rows: &[(u64, u64, u64, u64)]) -> Json {
    list(
        operation_t(),
        rows.iter()
            .map(|(identity, from, to, cost)| {
                pair(nat(*identity), pair(nat(*from), pair(nat(*to), nat(*cost))))
            })
            .collect(),
    )
}
/// `some (members, cost)` of the order's argmin over identities `id`.
fn found(id: Json, members: Vec<Json>, cost: u64) -> Json {
    some(
        product_t(list_t(id.clone()), nat_t()),
        pair(list(id, members), nat(cost)),
    )
}
/// `call arguments = expected`, by reduction in the kernel.
fn reduced(name: &str, left: Json, expected: Json) -> Json {
    theorem(name, eq(left, expected), reflexivity())
}

/// GNAF-VEC-01's operations of the snapshot `S0`, and of `S1`, which adds
/// `rAC4`: `rAB = 0`, `rBC = 1`, `rAC6 = 2`, `rAC4 = 3` over the values
/// `a0 = 0`, `b0 = 1`, `c0 = 2`.
pub const VEC01_S0: [(u64, u64, u64, u64); 3] = [(0, 0, 1, 2), (1, 1, 2, 3), (2, 0, 2, 6)];
/// See [`VEC01_S0`].
pub const VEC01_S1: [(u64, u64, u64, u64); 4] =
    [(0, 0, 1, 2), (1, 1, 2, 3), (2, 0, 2, 6), (3, 0, 2, 4)];
/// GNAF-VEC-02's admitted costs: `rA = (1,3)`, `rB = (2,2)`, `rC = (3,1)`,
/// `rD = (3,3)`.
pub const VEC02: [(u64, [u64; 2]); 4] = [(0, [1, 3]), (1, [2, 2]), (2, [3, 1]), (3, [3, 3])];
/// GNAF-VEC-04's representations `a`, `b`, `c` at costs 2, 1, 0, and its
/// only local rewrite `a -> b`.
pub const VEC04: [(u64, u64); 3] = [(0, 2), (1, 1), (2, 0)];

/// The compositions of `operations` from `a0` to `c0`: walks of up to as
/// many operations as there are, from the empty walk at `a0`.
pub fn vec01_compositions(operations_table: &[(u64, u64, u64, u64)]) -> Json {
    local_call(
        "compositions",
        vec![
            nat(operations_table.len() as u64 + 1),
            operations(operations_table),
            nat(2),
            list(walk_t(), vec![pair(nil(nat_t()), pair(nat(0), nat(0)))]),
        ],
    )
}

/// The authority's vectors, each a theorem over the generic order the
/// answers are computed by.
#[allow(clippy::too_many_lines)]
fn authority_vectors() -> Vec<Json> {
    let paths =
        |members: &[&[u64]]| -> Vec<Json> { members.iter().map(|path| nats(path)).collect() };
    let path_order =
        |function: &str, arguments: Vec<Json>| generic_call(function, nats_t(), arguments);
    let nat_order =
        |function: &str, arguments: Vec<Json>| generic_call(function, nat_t(), arguments);
    let natsame = || function_ref(lx::member("natSame"));
    let vec02 = || {
        vector_table(
            &VEC02
                .iter()
                .map(|(identity, cost)| (*identity, cost.as_slice()))
                .collect::<Vec<_>>(),
        )
    };
    let vec04 = || scalar_table(&VEC04);
    let rules = || list(product_t(nat_t(), nat_t()), vec![pair(nat(0), nat(1))]);
    vec![
        // GNAF-VEC-01 (§16.1): the S0 compositions are [rAB, rBC] at 5 and
        // [rAC6] at 6, optimum [rAB, rBC] at 5.
        reduced(
            "vec01Optimum",
            path_order("argmin", vec![vec01_compositions(&VEC01_S0)]),
            found(nats_t(), paths(&[&[0, 1]]), 5),
        ),
        // Admitting rAC4 at 4 in S1 makes [rAC4] the optimum at 4.
        reduced(
            "vec01Extension",
            path_order("argmin", vec![vec01_compositions(&VEC01_S1)]),
            found(nats_t(), paths(&[&[3]]), 4),
        ),
        // The S0 certificate holds for S0 and is invalid for S1.
        reduced(
            "vec01CertificateHolds",
            path_order(
                "certifiesArgmin",
                vec![
                    function_ref(lx::member("vectorEq")),
                    vec01_compositions(&VEC01_S0),
                    list(nats_t(), paths(&[&[0, 1]])),
                    nat(5),
                ],
            ),
            yes(),
        ),
        reduced(
            "vec01CertificateInvalidated",
            path_order(
                "certifiesArgmin",
                vec![
                    function_ref(lx::member("vectorEq")),
                    vec01_compositions(&VEC01_S1),
                    list(nats_t(), paths(&[&[0, 1]])),
                    nat(5),
                ],
            ),
            no(),
        ),
        // GNAF-VEC-02 (§16.2): the complete frontier is exactly
        // {rA, rB, rC}; rD is dominated.
        reduced(
            "vec02Frontier",
            nat_order("frontier", vec![vec02()]),
            nats(&[0, 1, 2]),
        ),
        // GNAF-REJ-29 (§16.24): a frontier response omitting rC is not
        // frontier-complete.
        reduced(
            "rej29FrontierOmission",
            nat_order("certifiesFrontier", vec![natsame(), vec02(), nats(&[0, 1])]),
            no(),
        ),
        // GNAF-REJ-14 (§16.24): the componentwise minima (1,1) come from rA
        // and rC; no candidate attains them, so they are no vector optimum.
        reduced(
            "rej14ComponentwiseMinima",
            nat_order("componentwiseMinimum", vec![vec02()]),
            nats(&[1, 1]),
        ),
        reduced(
            "rej14MinimaUnattained",
            nat_order(
                "attains",
                vec![vec02(), nat_order("componentwiseMinimum", vec![vec02()])],
            ),
            no(),
        ),
        // GNAF-VEC-04 (§16.4): with only the local rewrite a -> b, a
        // normalizes to b, the global minimum is c at 0, and a claim that b
        // is optimal because it is normal is refused.
        reduced(
            "vec04NormalFormIsB",
            local_call("normalize", vec![nat(10), rules(), nat(0)]),
            nat(1),
        ),
        reduced(
            "vec04GlobalMinimumIsC",
            nat_order("argmin", vec![vec04()]),
            found(nat_t(), vec![nat(2)], 0),
        ),
        reduced(
            "vec04NormalFormRefused",
            nat_order(
                "certifiesArgmin",
                vec![
                    natsame(),
                    vec04(),
                    list(
                        nat_t(),
                        vec![local_call("normalize", vec![nat(10), rules(), nat(0)])],
                    ),
                    nat(1),
                ],
            ),
            no(),
        ),
        // GNAF-VEC-17 (§16.17): R0 and R1 cost 0 when their index equals
        // the hidden bit h and 10 otherwise. The pointwise envelope is 0 for
        // each h, every uniform system's worst case is 10, the fair
        // randomized system expects 5, and no uniform system is certified
        // at the pointwise value.
        reduced(
            "vec17EnvelopeAtZero",
            nat_order("argmin", vec![local_call("environmentRows", vec![nat(0)])]),
            found(nat_t(), vec![nat(0)], 0),
        ),
        reduced(
            "vec17EnvelopeAtOne",
            nat_order("argmin", vec![local_call("environmentRows", vec![nat(1)])]),
            found(nat_t(), vec![nat(1)], 0),
        ),
        reduced(
            "vec17UniformWorstCase",
            nat_order("argmin", vec![local_call("uniformRows", Vec::new())]),
            found(nat_t(), vec![nat(0), nat(1)], 10),
        ),
        reduced(
            "vec17FairExpectationAtZero",
            local_call("fairExpectation", vec![nat(0)]),
            nat(5),
        ),
        reduced(
            "vec17FairExpectationAtOne",
            local_call("fairExpectation", vec![nat(1)]),
            nat(5),
        ),
        reduced(
            "vec17PointwiseValueRefused",
            nat_order(
                "certifiesArgmin",
                vec![
                    natsame(),
                    local_call("uniformRows", Vec::new()),
                    nats(&[0]),
                    nat(0),
                ],
            ),
            no(),
        ),
    ]
}

// --- the module ------------------------------------------------------------

/// Every declaration of the model, in module order: each group may only
/// refer to the groups before it.
fn declarations() -> Vec<Json> {
    let unused = Unused::new();
    let mut out = request_types();
    out.extend(machine_contract(&unused));
    out.extend(universe());
    out.extend(completeness());
    out.extend(validation(&unused));
    out.extend(realization());
    out.extend(value_equality(&unused));
    out.extend(reachable_code(&unused));
    out.extend(order());
    out.extend(admission(&unused));
    out.extend(answer(&unused));
    out.extend(vector_definitions());
    out.extend(authority_vectors());
    out
}

/// The `Gnaf` module text.
#[must_use]
pub fn module() -> String {
    lx::module_tex(MODEL, &[SYNTAX, SEMANTICS], declarations())
}
