//! `BS-16` (SPEC.md §17.15, §17.23): the validation of a GNAF request as
//! language-1.2 source, held to `lexlean::gnaf::validate`.
//!
//! The root takes the projection of a request that validation reads, as
//! module data: the constructors of the module's inductives in declaration
//! order. That order is read from the module's own text (`sorts`), so the
//! encoders below name a constructor and never number it, and a constructor
//! added to or moved in the module fails here instead of being encoded as
//! another. The program, the domain's values, the plans' functions, the
//! artifacts' digests, and the strings of a claim are not arguments: the
//! oracle runs on the full `Request`, the root on its projection.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::artifact::content_id::Sha256Digest;
use lexlean::calculus::Value;
use lexlean::gnaf::{
    self, Action, ActionKind, Boundary, Capacity, Carrier, Charge, ClaimClass, Completeness,
    Objective, OperandSize, Prepared, Rejection, Request, Scope,
};

use super::rust_validate::{eligibility_of, rustc_differential, value_nodes};
use super::{bytes, list, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.GnafValidate";
const SOURCE: &str = "bootstrap/src/GnafValidate.lex.tex";
const ROOTS: [(&str, &[&str]); 1] = [("validateRequest", &["allocation", "overflow", "recursion"])];

// ---------------------------------------------------------------- the sorts

/// The constructors of each inductive of the module, in declaration order.
fn sorts() -> &'static BTreeMap<String, Vec<String>> {
    static SORTS: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    SORTS.get_or_init(|| {
        let source =
            std::fs::read_to_string(repo_root().join(SOURCE).as_std_path()).expect("the module");
        let mut out = BTreeMap::new();
        for block in source.split("inductive{").skip(1) {
            let block = block.split("\n}\n").next().expect("a block");
            let name = block
                .split("name: ")
                .nth(1)
                .and_then(|rest| rest.split(',').next())
                .expect("a sort name")
                .trim()
                .to_owned();
            let constructors: Vec<String> = block
                .split("{name: ")
                .skip(1)
                .map(|rest| {
                    rest.split(',')
                        .next()
                        .expect("a constructor")
                        .trim()
                        .to_owned()
                })
                .collect();
            out.insert(name, constructors);
        }
        out
    })
}

fn index(sort: &str, constructor: &str) -> u64 {
    let found = sorts()
        .get(sort)
        .unwrap_or_else(|| panic!("the module has no sort `{sort}`"))
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`{sort}` has no constructor `{constructor}`"));
    u64::try_from(found).expect("an index")
}

fn adt(sort: &str, constructor: &str, fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor: index(sort, constructor),
        fields,
    }
}

/// A record: a structure is the one constructor of its own sort.
fn record(fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor: 0,
        fields,
    }
}

fn nat(number: u64) -> Value {
    Value::Nat {
        value: number.to_string(),
    }
}

fn count(items: usize) -> Value {
    nat(u64::try_from(items).expect("a count"))
}

// ----------------------------------------------------------------- encoders

fn enc_kind(kind: ActionKind) -> Value {
    adt("ActionKind", &pascal(kind.constructor()), Vec::new())
}

/// `retainedState` as `RetainedState`: the module names its constructors as
/// the model does, with the first letter raised.
fn pascal(name: &str) -> String {
    let mut characters = name.chars();
    characters
        .next()
        .map(|first| first.to_ascii_uppercase().to_string() + characters.as_str())
        .unwrap_or_default()
}

fn enc_charge(charge: Charge) -> Value {
    match charge {
        Charge::Steps => adt("Charge", "ChSteps", Vec::new()),
        Charge::Constant { cost } => adt("Charge", "ChConstant", vec![nat(cost)]),
        Charge::Free => adt("Charge", "ChFree", Vec::new()),
        Charge::Undeclared => adt("Charge", "ChUndeclared", Vec::new()),
    }
}

fn enc_action(action: &Action) -> Value {
    record(vec![enc_kind(action.kind), enc_charge(action.charge)])
}

fn enc_boundary(boundary: Boundary) -> Value {
    let name = match boundary {
        Boundary::Complete => "BdComplete",
        Boundary::PreparedState => "BdPreparedState",
        Boundary::PreparedPlan => "BdPreparedPlan",
    };
    adt("Boundary", name, Vec::new())
}

fn enc_machine(machine: &gnaf::Machine) -> Value {
    let capacity = &machine.capacity;
    record(vec![
        nat(machine.fuel),
        record(vec![
            nat(capacity.fuel),
            nat(capacity.domain),
            nat(capacity.systems),
            nat(capacity.charge),
        ]),
        adt(
            "OperandSize",
            match machine.operand_size {
                OperandSize::Weighted => "OsWeighted",
                OperandSize::Unit => "OsUnit",
            },
            Vec::new(),
        ),
        list(machine.actions.iter().map(enc_action).collect()),
        enc_boundary(machine.boundary),
        list(
            machine
                .prepared
                .iter()
                .map(|prepared| enc_kind(prepared.kind))
                .collect(),
        ),
    ])
}

fn enc_carrier(carrier: &Carrier) -> Value {
    match carrier {
        Carrier::Grammar {
            plans, thresholds, ..
        } => adt(
            "Carrier",
            "CaGrammar",
            vec![
                count(plans.len()),
                list(
                    plans
                        .iter()
                        .flat_map(|plan| plan.prepares.iter())
                        .map(|kind| enc_kind(*kind))
                        .collect(),
                ),
                count(thresholds.len()),
            ],
        ),
        Carrier::InternalPlans => adt("Carrier", "CaInternalPlans", Vec::new()),
        Carrier::OptimizerOutput => adt("Carrier", "CaOptimizerOutput", Vec::new()),
        Carrier::Discovered { .. } => adt("Carrier", "CaDiscovered", Vec::new()),
        Carrier::Cached => adt("Carrier", "CaCached", Vec::new()),
    }
}

fn enc_request(request: &Request) -> Value {
    record(vec![
        count(request.domain.len()),
        enc_machine(&request.machine),
        enc_carrier(&request.carrier),
        adt(
            "Completeness",
            match request.completeness {
                Completeness::GrammarEquality => "CoGrammarEquality",
                Completeness::Missing => "CoMissing",
                Completeness::CitesUniverseId => "CoCitesUniverseId",
                Completeness::CitesOptimizer => "CoCitesOptimizer",
            },
            Vec::new(),
        ),
        adt(
            "Objective",
            match request.objective {
                Objective::Scalar => "ObScalar",
                Objective::Vector => "ObVector",
            },
            Vec::new(),
        ),
        adt(
            "ClaimClass",
            &format!("Cl{}", pascal(request.claim.constructor())),
            Vec::new(),
        ),
        adt(
            "Scope",
            match request.scope {
                Scope::GrammarUniverse => "ScGrammarUniverse",
                Scope::CalculusPrograms => "ScCalculusPrograms",
                Scope::RustPrograms => "ScRustPrograms",
            },
            Vec::new(),
        ),
    ])
}

/// A run of `validateRequest`: the projection and what the Rust validation
/// makes of the request: nothing, or the `serde_json` bytes of the rejection.
fn run_of(request: &Request) -> Run {
    let outcome = match gnaf::validate(request) {
        None => Value::None,
        Some(rejection) => Value::Some {
            value: Box::new(bytes(&serde_json::to_vec(&rejection).expect("a rejection"))),
        },
    };
    (
        vec![enc_request(request)],
        serde_json::to_value(Value::Ok {
            value: Box::new(outcome),
        })
        .expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------- corpus

/// The requests the committed fixtures state: the GNAF requests and the ones
/// written for certification, distinct as requests.
fn committed() -> Vec<(String, Request)> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for (name, request) in crate::certification::requests() {
        let key = serde_json::to_string(&request).expect("a request");
        if seen.insert(key) {
            out.push((name, request));
        }
    }
    for fixture in crate::gnaf::fixtures() {
        let key = serde_json::to_string(&fixture.request).expect("a request");
        if seen.insert(key) {
            out.push((fixture.name, fixture.request));
        }
    }
    out
}

/// A request that validation accepts, to build the others from.
fn accepted(committed: &[(String, Request)]) -> Request {
    committed
        .iter()
        .map(|(_, request)| request)
        .find(|request| gnaf::validate(request).is_none())
        .expect("a committed request that validation accepts")
        .clone()
}

fn charges(capacity: Capacity) -> Vec<Charge> {
    vec![
        Charge::Steps,
        Charge::Constant { cost: 0 },
        Charge::Constant { cost: 1 },
        Charge::Constant {
            cost: capacity.charge,
        },
        Charge::Constant {
            cost: capacity.charge + 1,
        },
        Charge::Free,
        Charge::Undeclared,
    ]
}

fn claims() -> Vec<ClaimClass> {
    vec![
        ClaimClass::Exact,
        ClaimClass::NormalForm,
        ClaimClass::Canonical,
        ClaimClass::RepresentationMinimal,
        ClaimClass::ComparisonTheorem,
        ClaimClass::ProfileDefinedComparison {
            profile: "p".to_owned(),
            class_id: "c".to_owned(),
            shape: "s".to_owned(),
        },
        ClaimClass::InputTotal,
        ClaimClass::GlobalOptimal,
        ClaimClass::ArgminComplete,
        ClaimClass::ParetoOptimal,
        ClaimClass::FrontierComplete,
        ClaimClass::PointwiseEnvelopeComplete,
        ClaimClass::QueryFamilyAnswerComplete,
        ClaimClass::UseCaseGlobalOptimal,
        ClaimClass::WorkloadArgminComplete,
        ClaimClass::WorkloadParetoOptimal,
        ClaimClass::WorkloadFrontierComplete,
        ClaimClass::FamilyOptimal,
        ClaimClass::CompetitiveBound,
        ClaimClass::CompetitiveOptimal,
        ClaimClass::AsymptoticBound,
        ClaimClass::AsymptoticOptimal,
        ClaimClass::UseCaseClassComplete,
        ClaimClass::UseCaseClassAnswerComplete,
        ClaimClass::MaintainedUseCaseClass,
        ClaimClass::RestrictedUniverseOptimal,
        ClaimClass::RevisionPreserved,
        ClaimClass::BestKnown,
        ClaimClass::MeasuredBestAmongTested,
        ClaimClass::HeuristicSelected,
        ClaimClass::InstanceOptimal { alpha: 1, beta: 0 },
    ]
}

/// A prepared artifact of a kind: validation reads its kind only.
fn prepared(kind: ActionKind) -> Prepared {
    Prepared {
        kind,
        artifact: Sha256Digest::of(kind.constructor().as_bytes()),
    }
}

fn action(kind: ActionKind, charge: Charge) -> Action {
    Action { kind, charge }
}

/// The actions of a machine that accounts for every performed kind by steps.
fn performing() -> Vec<Action> {
    ActionKind::ALL
        .into_iter()
        .filter(|kind| kind.performed())
        .map(|kind| action(kind, Charge::Steps))
        .collect()
}

/// The machines the rules read: each of the 21 rejections, and each of the
/// ten action kinds for the six that carry one, three times each, built by
/// hand from an accepted request.
fn hand_built(base: &Request) -> Vec<Request> {
    let mut out = Vec::new();
    let with = |change: &dyn Fn(&mut Request)| {
        let mut request = base.clone();
        change(&mut request);
        request
    };
    out.push(base.clone());
    out.push(with(&|r| r.domain.clear()));
    for carrier in [
        Carrier::InternalPlans,
        Carrier::OptimizerOutput,
        Carrier::Discovered { members: vec![0, 1] },
        Carrier::Cached,
    ] {
        out.push(with(&|r| r.carrier = carrier.clone()));
    }
    for completeness in [
        Completeness::Missing,
        Completeness::CitesUniverseId,
        Completeness::CitesOptimizer,
    ] {
        out.push(with(&|r| r.completeness = completeness));
    }
    // The capacity: each of the four counts one past, and the universe.
    out.push(with(&|r| r.machine.fuel = r.machine.capacity.fuel + 1));
    out.push(with(&|r| r.machine.capacity.domain = 0));
    out.push(with(&|r| r.machine.capacity.systems = 0));
    out.push(with(&|r| r.machine.capacity.systems = 1));
    out.push(with(&|r| {
        r.machine.actions.push(action(
            ActionKind::Preprocessing,
            Charge::Constant {
                cost: r.machine.capacity.charge + 1,
            },
        ));
    }));
    out.push(with(&|r| r.machine.capacity.domain = r.domain.len() as u64));
    out.push(with(&|r| r.machine.operand_size = OperandSize::Unit));
    for kind in ActionKind::ALL {
        for _ in 0..3 {
            // Duplicate: the kind accounted twice.
            out.push(with(&|r| {
                r.machine.actions.push(action(kind, Charge::Steps));
                r.machine.actions.push(action(kind, Charge::Free));
            }));
            // Unaccounted: the kind removed from a machine that performs it.
            out.push(with(&|r| r.machine.actions.retain(|a| a.kind != kind)));
            // Hidden cost: the kind accounted with each charge but the right one.
            for charge in [
                Charge::Constant { cost: 2 },
                Charge::Free,
                Charge::Undeclared,
                Charge::Constant { cost: 0 },
            ] {
                out.push(with(&|r| {
                    r.machine.actions.retain(|a| a.kind != kind);
                    r.machine.actions.push(action(kind, charge));
                }));
            }
            // Free under each boundary, bound and unbound.
            for boundary in [
                Boundary::Complete,
                Boundary::PreparedState,
                Boundary::PreparedPlan,
            ] {
                for bound in [false, true] {
                    out.push(with(&|r| {
                        r.machine.actions.retain(|a| a.kind != kind);
                        r.machine.actions.push(action(kind, Charge::Free));
                        r.machine.boundary = boundary;
                        r.machine.prepared = if bound { vec![prepared(kind)] } else { vec![] };
                    }));
                }
            }
            // A prepared artifact of the kind, whatever the kind is accounted as.
            for charge in charges(base.machine.capacity) {
                out.push(with(&|r| {
                    r.machine.actions.retain(|a| a.kind != kind);
                    r.machine.actions.push(action(kind, charge));
                    r.machine.prepared = vec![prepared(kind)];
                }));
            }
            // A plan that prepares by the kind.
            out.push(with(&|r| {
                if let Carrier::Grammar { plans, .. } = &mut r.carrier {
                    if let Some(plan) = plans.first_mut() {
                        plan.prepares = vec![kind];
                    }
                }
            }));
        }
    }
    // The claim over each objective.
    for claim in claims() {
        for objective in [Objective::Scalar, Objective::Vector] {
            for scope in [
                Scope::GrammarUniverse,
                Scope::CalculusPrograms,
                Scope::RustPrograms,
            ] {
                out.push(with(&|r| {
                    r.claim = claim.clone();
                    r.objective = objective;
                    r.scope = scope;
                }));
            }
        }
    }
    out
}

/// One field-level mutant of a committed request, drawn from the seed.
fn mutant(base: &Request, seeded: &mut Seeded) -> Request {
    let mut request = base.clone();
    for _ in 0..=seeded.below(3) {
        match seeded.below(14) {
            0 => {
                if seeded.chance(50) {
                    request.domain.clear();
                } else if let Some(first) = request.domain.first().cloned() {
                    for _ in 0..seeded.below(3) {
                        request.domain.push(first.clone());
                    }
                }
            }
            1 => {
                request.carrier = match seeded.below(4) {
                    0 => Carrier::InternalPlans,
                    1 => Carrier::OptimizerOutput,
                    2 => Carrier::Discovered {
                        members: vec![seeded.next() % 8],
                    },
                    _ => Carrier::Cached,
                };
            }
            2 => {
                request.completeness = *seeded.pick(&[
                    Completeness::GrammarEquality,
                    Completeness::Missing,
                    Completeness::CitesUniverseId,
                    Completeness::CitesOptimizer,
                ]);
            }
            3 => {
                let capacity = &mut request.machine.capacity;
                let value = seeded.next() % 6;
                match seeded.below(4) {
                    0 => capacity.fuel = value,
                    1 => capacity.domain = value,
                    2 => capacity.systems = value * value,
                    _ => capacity.charge = value,
                }
            }
            4 => request.machine.fuel = seeded.next() % 6000,
            5 => {
                request.machine.operand_size = match request.machine.operand_size {
                    OperandSize::Weighted => OperandSize::Unit,
                    OperandSize::Unit => OperandSize::Weighted,
                };
            }
            6 => {
                let actions = &mut request.machine.actions;
                match seeded.below(5) {
                    0 if !actions.is_empty() => {
                        let at = seeded.below(actions.len());
                        actions.remove(at);
                    }
                    1 if !actions.is_empty() => {
                        let at = seeded.below(actions.len());
                        let copy = actions[at].clone();
                        actions.insert(seeded.below(actions.len() + 1), copy);
                    }
                    2 if !actions.is_empty() => {
                        let at = seeded.below(actions.len());
                        actions[at].kind = *seeded.pick(&ActionKind::ALL);
                    }
                    3 if !actions.is_empty() => {
                        let at = seeded.below(actions.len());
                        let capacity = request.machine.capacity;
                        actions[at].charge = *seeded.pick(&charges(capacity));
                    }
                    _ => {
                        let capacity = request.machine.capacity;
                        let kind = *seeded.pick(&ActionKind::ALL);
                        let charge = *seeded.pick(&charges(capacity));
                        actions.push(action(kind, charge));
                    }
                }
            }
            7 => {
                request.machine.boundary = *seeded.pick(&[
                    Boundary::Complete,
                    Boundary::PreparedState,
                    Boundary::PreparedPlan,
                ]);
            }
            8 => {
                if seeded.chance(60) {
                    let kind = *seeded.pick(&ActionKind::ALL);
                    request.machine.prepared.push(prepared(kind));
                } else if !request.machine.prepared.is_empty() {
                    let at = seeded.below(request.machine.prepared.len());
                    request.machine.prepared.remove(at);
                }
            }
            9 => {
                if let Carrier::Grammar { plans, .. } = &mut request.carrier {
                    if !plans.is_empty() {
                        let at = seeded.below(plans.len());
                        if seeded.chance(70) {
                            plans[at].prepares.push(*seeded.pick(&ActionKind::ALL));
                        } else {
                            plans[at].prepares.clear();
                        }
                    }
                }
            }
            10 => {
                if let Carrier::Grammar {
                    plans, thresholds, ..
                } = &mut request.carrier
                {
                    if seeded.chance(50) {
                        let next = thresholds.last().map_or(1, |last| last + 1 + seeded.next() % 4);
                        thresholds.push(next);
                    } else if !plans.is_empty() {
                        let copy = plans[seeded.below(plans.len())].clone();
                        plans.push(copy);
                    }
                }
            }
            11 => {
                request.objective = match request.objective {
                    Objective::Scalar => Objective::Vector,
                    Objective::Vector => Objective::Scalar,
                };
            }
            12 => request.claim = seeded.pick(&claims()).clone(),
            _ => {
                request.scope = *seeded.pick(&[
                    Scope::GrammarUniverse,
                    Scope::CalculusPrograms,
                    Scope::RustPrograms,
                ]);
            }
        }
    }
    request
}

/// The serde name of a rejection, to count which of them the corpus reaches.
fn rule_of(rejection: &Rejection) -> String {
    serde_json::to_value(rejection).expect("a rejection")["kind"]
        .as_str()
        .expect("a kind")
        .to_owned()
}

// ------------------------------------------------------------------- plants

/// A defect planted in the source: the check it removes, the text it is
/// made at, and the corpus the differential must see it on.
const PLANTS: [super::Plant; 12] = [
    // The empty domain is accepted.
    super::Plant {
        label: "empty domain accepted",
        site: "((project request domainLength) =? 0)",
        replacement: "false",
        root: "validateRequest",
    },
    // A constant charge of 0 is accepted for a preparation.
    super::Plant {
        label: "constant charge of zero accepted",
        site: "(if (0 <? cost) then",
        replacement: "(if (0 <=? cost) then",
        root: "validateRequest",
    },
    // A free preparation is accepted under the complete boundary.
    super::Plant {
        label: "free preparation accepted under the complete boundary",
        site: "| Boundary.BdComplete => (ctor Option.some [Rejection] (Rejection.RjUnboundPreparation kind))",
        replacement: "| Boundary.BdComplete => (ctor Option.none [Rejection])",
        root: "validateRequest",
    },
    // A prepared artifact is matched against steps instead of free.
    super::Plant {
        label: "prepared artifact matched against steps",
        site: "| Charge.ChFree => (ctor Option.none [Rejection])\n        | Charge.ChSteps => (ctor Option.some [Rejection] (Rejection.RjStrayPreparedArtifact kind))",
        replacement: "| Charge.ChFree => (ctor Option.some [Rejection] (Rejection.RjStrayPreparedArtifact kind))\n        | Charge.ChSteps => (ctor Option.none [Rejection])",
        root: "validateRequest",
    },
    // A duplicate is counted at one occurrence.
    super::Plant {
        label: "duplicate counted at one occurrence",
        site: "(if (1 <? (countKind actions (project action kind)))",
        replacement: "(if (0 <? (countKind actions (project action kind)))",
        root: "validateRequest",
    },
    // The fallback kind is missing from the performed kinds.
    super::Plant {
        label: "fallback not performed",
        site: "| ActionKind.Fallback => true",
        replacement: "| ActionKind.Fallback => false",
        root: "validateRequest",
    },
    // The domain capacity is compared with `>=`.
    super::Plant {
        label: "domain capacity compared with at-least",
        site: "((project capacity domain) <? (project request domainLength))",
        replacement: "((project capacity domain) <=? (project request domainLength))",
        root: "validateRequest",
    },
    // The universe size is n + |T| * n for n + |T| * n * n.
    super::Plant {
        label: "universe size without the square",
        site: "(plans + (thresholds * (plans * plans)))",
        replacement: "(plans + (thresholds * plans))",
        root: "validateRequest",
    },
    // The legacy alias is refused as unsupported.
    super::Plant {
        label: "alias refused as unsupported",
        site: "| ClaimClass.ClRestrictedUniverseOptimal => (ctor Option.some [Rejection] Rejection.RjClaimAlias)",
        replacement: "| ClaimClass.ClRestrictedUniverseOptimal => (ctor Option.some [Rejection] Rejection.RjUnsupportedClaim)",
        root: "validateRequest",
    },
    // The scalar and vector claim checks are swapped.
    super::Plant {
        label: "scalar and vector claim checks swapped",
        site: "| ClaimClass.ClGlobalOptimal => (match (project request objective)\n      | Objective.ObVector =>",
        replacement: "| ClaimClass.ClGlobalOptimal => (match (project request objective)\n      | Objective.ObScalar =>",
        root: "validateRequest",
    },
    // A scope over Rust programs is admitted.
    super::Plant {
        label: "rust programs scope admitted",
        site: "| Scope.ScRustPrograms => (ctor Option.some [Rejection] Rejection.RjUncoveredScope)",
        replacement: "| Scope.ScRustPrograms => (ctor Option.none [Rejection])",
        root: "validateRequest",
    },
    // The unit operand-size check comes before the capacity check.
    super::Plant {
        label: "operand size checked before capacity",
        site: "(match (checkCapacity request)\n    | Option.some held => (ctor Option.some [Rejection] held)\n    | Option.none => (afterOperands request))",
        replacement: "(match (afterOperands request)\n    | Option.some held => (ctor Option.some [Rejection] held)\n    | Option.none => (checkCapacity request))",
        root: "validateRequest",
    },
];

const MUTANTS: usize = 3_000;
const EVALUATOR_NODES: usize = 4_000;

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("GnafValidate"),
        "GnafValidate is a linked module of the project"
    );
    eligibility_of(&project, "GnafValidate", MODULE, &ROOTS);
    // The constructors the encoders name are the module's own, in its order.
    assert_eq!(
        sorts().keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "ActionKind",
            "Boundary",
            "Carrier",
            "Charge",
            "ClaimClass",
            "Completeness",
            "Objective",
            "OperandSize",
            "Rejection",
            "Scope"
        ],
        "the sorts of the module"
    );

    let committed = committed();
    let base = accepted(&committed);
    let hand = hand_built(&base);
    let mut seeded = Seeded(0x0B55_0016);
    let drawn: Vec<Request> = (0..MUTANTS)
        .map(|at| mutant(&committed[at % committed.len()].1, &mut seeded))
        .collect();
    let requests: Vec<&Request> = committed
        .iter()
        .map(|(_, request)| request)
        .chain(hand.iter())
        .chain(drawn.iter())
        .collect();
    let vectors: Vec<Run> = requests.iter().map(|request| run_of(request)).collect();
    let refusals: Vec<Rejection> = requests
        .iter()
        .filter_map(|request| gnaf::validate(request))
        .collect();
    eprintln!(
        "BS-16: {} committed requests, {} hand-built, {} mutants, {} refused ({:?})",
        committed.len(),
        hand.len(),
        drawn.len(),
        refusals.len(),
        started.elapsed()
    );
    assert_eq!(
        (committed.len(), hand.len(), drawn.len(), refusals.len()),
        (COMMITTED, HAND_BUILT, MUTANTS, REFUSED),
        "the committed requests, the hand-built, the seeded mutants, and the refused among them"
    );
    let rules: BTreeSet<String> = refusals.iter().map(rule_of).collect();
    assert_eq!(
        rules.len(),
        21,
        "every one of the 21 rejections is reached by a vector: {rules:?}"
    );
    let kinds: BTreeSet<(String, String)> = refusals
        .iter()
        .filter_map(|rejection| {
            let value = serde_json::to_value(rejection).expect("a rejection");
            Some((
                value["kind"].as_str()?.to_owned(),
                value["action"].as_str()?.to_owned(),
            ))
        })
        .collect();
    eprintln!("BS-16: {} (rule, action kind) pairs reached", kinds.len());

    let checked = support::checked_project(&project);
    let root = super::lower(&checked, &project, MODULE, "validateRequest");
    let normative = support::normative_host();
    if !normative {
        eprintln!(
            "BS-16: platform-bound host: the evaluator and the planted defects run on the normative host ({:?})",
            started.elapsed()
        );
    }
    if normative {
        let within: Vec<Run> = vectors
            .iter()
            .filter(|run| run.0.iter().map(value_nodes).sum::<usize>() <= EVALUATOR_NODES)
            .cloned()
            .collect();
        assert_eq!(within.len(), WITHIN, "the vectors the evaluator is given");
        let disagreements = super::evaluate(&root, &within, 5);
        assert!(
            disagreements.is_empty(),
            "the evaluator and the Rust validation disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-16: the evaluator agrees on {} vectors ({:?})",
            within.len(),
            started.elapsed()
        );
        // The gate is armed: each defect planted in a copy of the source is a
        // disagreement with the oracle.
        for plant in &PLANTS {
            let planted = support::P::bootstrap();
            let source = planted.read_raw("src/GnafValidate.lex.tex");
            assert_eq!(
                source.matches(plant.site).count(),
                1,
                "{}: the plant site occurs once",
                plant.label
            );
            planted.write(
                "src/GnafValidate.lex.tex",
                &source.replacen(plant.site, plant.replacement, 1),
            );
            let checked = support::checked_project(&planted);
            let low = super::lower(&checked, &planted, MODULE, plant.root);
            let disagreements = super::evaluate(&low, &within, 1);
            assert!(
                !disagreements.is_empty(),
                "{}: the planted defect is detected by the differential",
                plant.label
            );
            eprintln!(
                "BS-16: plant detected: {}: {}",
                plant.label,
                &disagreements[0][..disagreements[0].len().min(120)]
            );
        }
    }
    // On every host: the rendered rust-std package of the root, compiled by
    // that host's pinned rustc, agrees with the oracle on the vectors.
    rustc_differential("BS-16", &[("validateRequest", &root, &vectors)], "vectors");
    if !normative {
        return;
    }
    // Under the pinned Lean, the production export certifies the root; its
    // packages are the committed ones and realize the program the
    // differential ran.
    if !support::lean_backed("BS-16") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let packages = harness::tree(&bundle.join("packages"));
    let committed_packages = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed_packages.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed_packages.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some(member) = exported_root.root.strip_prefix(&format!("{MODULE}.")) else {
            continue;
        };
        assert_eq!(member, "validateRequest");
        for output in &exported_root.outputs {
            let package: serde_json::Value = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&output.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("package JSON");
            assert_eq!(
                package["program"],
                serde_json::to_value(&root.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    eprintln!("BS-16: passed in {:?}", started.elapsed());
}

// The counts the specification states, over the committed tree: SPEC.md
// §17.23, the register statement of BS-16, its scenario, and VERIFICATION.md
// restate them, and the case fails when the tree no longer gives them. They
// are written when the case first measures them.
const COMMITTED: usize = 0;
const HAND_BUILT: usize = 0;
const REFUSED: usize = 0;
const WITHIN: usize = 0;
