//! GNAF requests over forward-chaining plans (SPEC.md §17.12, §17.15).
//!
//! The problem is the escalation level a clinical rule base derives from a
//! patient's observed findings, the same three derived findings and levels
//! as the reasoning example's `Triage` (SIRS from two of fever,
//! tachycardia, tachypnea, and leukocytosis; sepsis from SIRS and an
//! infection; shock from sepsis and hypotension), over findings observed
//! as the bits of a natural number. Each plan is a fixed calculus function,
//! so its rule search is execution the machine charges in steps:
//!
//! - forward chaining in a fixed priority order (shock, sepsis, SIRS),
//!   restarting the scan after every firing, as a forward reasoner selects
//!   the first applicable rule (the reference);
//! - one sweep firing each rule in dependency order;
//! - a goal-directed decision that derives nothing and tests each level's
//!   premises directly.
//!
//! The universe is the grammar's three systems, fixed before any search
//! runs; a request that charges the search nothing, or takes as its
//! universe the candidates a search encountered, is refused.

use lexlean::calculus::{Expr, Function, Prim, Program, Ty, Value, PROGRAM_SPEC};
use lexlean::gnaf::{
    ActionKind, Carrier, Charge, ClaimClass, Completeness, Objective, Request, Scope, REQUEST_SPEC,
};
use lexlean::Sha256Digest;

use super::{machine, plan, prim, set_charge, var};

/// The observed findings' bits, then the derived findings'.
pub(crate) const FEVER: u64 = 0;
pub(crate) const TACHYCARDIA: u64 = 1;
pub(crate) const TACHYPNEA: u64 = 2;
pub(crate) const LEUKOCYTOSIS: u64 = 3;
pub(crate) const INFECTION: u64 = 4;
pub(crate) const HYPOTENSION: u64 = 5;
const SIRS: u64 = 6;
const SEPSIS: u64 = 7;
const SHOCK: u64 = 8;

/// The patients, as observed findings: every finding; none; fever and
/// tachycardia with an infection; fever and tachycardia alone; and
/// tachypnea with hypotension.
pub const DOMAIN: [u64; 5] = [
    0b11_1111,
    0,
    (1 << FEVER) | (1 << TACHYCARDIA) | (1 << INFECTION),
    (1 << FEVER) | (1 << TACHYCARDIA),
    (1 << TACHYPNEA) | (1 << HYPOTENSION),
];

fn nat(number: u64) -> Expr {
    Expr::Value {
        ty: Ty::Nat,
        value: Value::Nat {
            value: number.to_string(),
        },
    }
}
fn cond(condition: Expr, then_branch: Expr, else_branch: Expr) -> Expr {
    Expr::Cond {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
    }
}
fn and(left: Expr, right: Expr) -> Expr {
    prim(Prim::BoolAnd, vec![left, right])
}
fn not(value: Expr) -> Expr {
    prim(Prim::BoolNot, vec![value])
}
fn add(left: Expr, right: Expr) -> Expr {
    prim(Prim::NatAdd, vec![left, right])
}
fn let_in(name: u64, bound: Expr, body: Expr) -> Expr {
    Expr::Let {
        name,
        ty: Ty::Nat,
        bound: Box::new(bound),
        body: Box::new(body),
    }
}

/// The value of finding `bit` in `findings`: 0 or 1.
fn bit_value(findings: Expr, bit: u64) -> Expr {
    // A quotient or remainder by zero is the third operand; neither
    // divisor here is zero.
    prim(
        Prim::NatRem,
        vec![
            prim(Prim::NatQuot, vec![findings, nat(1 << bit), nat(0)]),
            nat(2),
            nat(0),
        ],
    )
}
/// Whether `findings` holds finding `bit`.
fn holds(findings: Expr, bit: u64) -> Expr {
    prim(Prim::NatEq, vec![bit_value(findings, bit), nat(1)])
}
/// Two of fever, tachycardia, tachypnea, and leukocytosis.
fn sirs_criteria(findings: &Expr) -> Expr {
    prim(
        Prim::NatLe,
        vec![
            nat(2),
            add(
                add(
                    bit_value(findings.clone(), FEVER),
                    bit_value(findings.clone(), TACHYCARDIA),
                ),
                add(
                    bit_value(findings.clone(), TACHYPNEA),
                    bit_value(findings.clone(), LEUKOCYTOSIS),
                ),
            ),
        ],
    )
}

/// The guard of the rule deriving `derived`: it is not yet derived and its
/// premises hold.
fn guard(findings: &Expr, derived: u64) -> Expr {
    let premises = match derived {
        SIRS => sirs_criteria(findings),
        SEPSIS => and(
            holds(findings.clone(), SIRS),
            holds(findings.clone(), INFECTION),
        ),
        _ => and(
            holds(findings.clone(), SEPSIS),
            holds(findings.clone(), HYPOTENSION),
        ),
    };
    and(not(holds(findings.clone(), derived)), premises)
}
/// The conclusion of the rule deriving `derived`: the finding added.
fn conclude(findings: Expr, derived: u64) -> Expr {
    add(findings, nat(1 << derived))
}
/// The level the saturated `findings` warrant.
fn level(findings: &Expr) -> Expr {
    cond(
        holds(findings.clone(), SHOCK),
        nat(3),
        cond(
            holds(findings.clone(), SEPSIS),
            nat(2),
            cond(holds(findings.clone(), SIRS), nat(1), nat(0)),
        ),
    )
}

fn function(body: Expr) -> Function {
    Function {
        parameters: vec![0],
        types: vec![Ty::Nat],
        result: Ty::Nat,
        body,
    }
}

/// Forward chaining in priority order shock, sepsis, SIRS: fire the first
/// applicable rule and search again from the top, until no rule applies.
pub(crate) fn forward_priority(at: u64) -> Function {
    let findings = var(0);
    let mut body = level(&findings);
    for derived in [SIRS, SEPSIS, SHOCK] {
        body = cond(
            guard(&findings, derived),
            Expr::Call {
                function: at,
                operands: vec![conclude(findings.clone(), derived)],
            },
            body,
        );
    }
    function(body)
}

/// One sweep in dependency order: each rule fires at most once, on the
/// findings the previous rules left.
pub(crate) fn forward_sweep() -> Function {
    let mut body = level(&var(3));
    for (derived, from, to) in [(SHOCK, 2, 3), (SEPSIS, 1, 2), (SIRS, 0, 1)] {
        body = let_in(
            to,
            cond(
                guard(&var(from), derived),
                conclude(var(from), derived),
                var(from),
            ),
            body,
        );
    }
    function(body)
}

/// The level decided from the observed findings alone, most severe first.
pub(crate) fn goal_directed() -> Function {
    let findings = var(0);
    let sirs = sirs_criteria(&findings);
    let infected = and(sirs.clone(), holds(findings.clone(), INFECTION));
    function(cond(
        and(infected.clone(), holds(findings.clone(), HYPOTENSION)),
        nat(3),
        cond(infected, nat(2), cond(sirs, nat(1), nat(0))),
    ))
}

/// The problem the plans answer: the reference's forward chaining.
fn reference() -> Program {
    Program {
        spec: PROGRAM_SPEC.to_owned(),
        adts: Vec::new(),
        functions: vec![forward_priority(0)],
    }
}

/// The grammar over the three plans; the findings are no list, so no
/// threshold dispatches between them.
fn plans() -> Carrier {
    Carrier::Grammar {
        argument: Ty::Nat,
        result: Ty::Nat,
        plans: vec![
            plan(forward_priority(1)),
            plan(forward_sweep()),
            plan(goal_directed()),
        ],
        thresholds: Vec::new(),
    }
}

fn request(carrier: Carrier, objective: Objective, claim: ClaimClass) -> Request {
    Request {
        spec: REQUEST_SPEC.to_owned(),
        reference: reference(),
        domain: DOMAIN
            .iter()
            .map(|findings| Value::Nat {
                value: findings.to_string(),
            })
            .collect(),
        machine: machine(),
        carrier,
        completeness: Completeness::GrammarEquality,
        universe: Sha256Digest([0; 32]),
        objective,
        claim,
        scope: Scope::GrammarUniverse,
    }
}

/// The reasoning requests, by name, before their universe identity.
pub fn requests() -> Vec<(&'static str, Request)> {
    let argmin = || request(plans(), Objective::Scalar, ClaimClass::ArgminComplete);
    vec![
        ("reasoning-argmin", argmin()),
        (
            "reasoning-frontier",
            request(plans(), Objective::Vector, ClaimClass::FrontierComplete),
        ),
        // The search is execution: charging it nothing hides its cost.
        ("reject-reasoning-free-search", {
            let mut out = argmin();
            set_charge(&mut out.machine, ActionKind::Execution, Charge::Free);
            out
        }),
        // The candidates one search encountered are no universe fixed
        // before the search.
        (
            "reject-reasoning-discovered-universe",
            Request {
                carrier: Carrier::Discovered {
                    members: vec![0, 2],
                },
                ..argmin()
            },
        ),
    ]
}
