//! GNAF requests over the production realization calculus (SPEC.md §17.15).
//!
//! The normative model is the LexLean module `compiler/src/Gnaf`, which Lean
//! elaborates and the kernel checks: it fixes the request components that
//! UOR-GNAF (`uor-gnaf/1-draft.2`) requires before any optimization (the
//! machine's accounting boundary and capacity, the complete-system universe
//! and its completeness theorem, the objective and its order, and the claim
//! with its scope), the fail-closed validation of a request, and the answer
//! the grammar universe has under the calculus denotation. This module is
//! the host side of that definition: the closed JSON form
//! `lexlean/gnaf-request/1`, the checks a request must pass before it is
//! posed (its universe identity among them), a step-for-step transcription
//! of validation and evaluation, and the emission of a request as a LexLean
//! term, so the kernel confirms every committed request's answer against the
//! model.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value as Json};

use crate::artifact::content_id::{FramedHasher, Sha256Digest};
use crate::calculus::{
    interp, term, Expr, Function, Outcome, Prim, Program, Ty, Value, PROGRAM_SPEC,
};
use crate::code;
use crate::diagnostic::Diagnostic;
use crate::error::LexLeanError;

/// The schema tag of a request.
pub const REQUEST_SPEC: &str = "lexlean/gnaf-request/1";

/// The schema tag of a fixture.
pub const FIXTURE_SPEC: &str = "lexlean/gnaf-fixture/1";

/// The schema tag of the dependency manifest (UOR-GNAF §20).
pub const MANIFEST_SPEC: &str = "lexlean/gnaf-manifest/1";

/// The capacity of the host evaluator: the largest machine it offers.
///
/// Fuel bounds evaluation depth, and the evaluator recurses once per level,
/// so the fuel capacity fixes the stack an evaluation can need. The domain
/// and charge capacities bound every declared charge a system can accrue,
/// at most `256 * 3 * 2^32` per invocation over at most 1024 invocations, far
/// below `2^63`, so the host's integers represent every declared cost
/// exactly; a step count beyond them is refused as beyond capacity, never
/// wrapped or turned into an unknown cost the model would not report.
pub const HOST_CAPACITY: Capacity = Capacity {
    fuel: 4096,
    domain: 1024,
    systems: 65_536,
    charge: 1 << 32,
};

/// The evaluation stack per level of fuel: about seven times what an
/// unoptimized build measures for the deepest per-level frame chain.
const STACK_PER_LEVEL: usize = 16 * 1024;

/// The module defining the model.
pub const MODEL: &str = "Gnaf";

/// A claim class of UOR-GNAF §12.4.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case", deny_unknown_fields)]
pub enum ClaimClass {
    Exact,
    NormalForm,
    Canonical,
    RepresentationMinimal,
    ComparisonTheorem,
    /// `profile-defined-comparison(ProfileId,ClassId,ComparisonResultShape)`.
    ProfileDefinedComparison {
        profile: String,
        class_id: String,
        shape: String,
    },
    InputTotal,
    GlobalOptimal,
    ArgminComplete,
    ParetoOptimal,
    FrontierComplete,
    PointwiseEnvelopeComplete,
    QueryFamilyAnswerComplete,
    UseCaseGlobalOptimal,
    WorkloadArgminComplete,
    WorkloadParetoOptimal,
    WorkloadFrontierComplete,
    FamilyOptimal,
    CompetitiveBound,
    CompetitiveOptimal,
    AsymptoticBound,
    AsymptoticOptimal,
    UseCaseClassComplete,
    UseCaseClassAnswerComplete,
    MaintainedUseCaseClass,
    RestrictedUniverseOptimal,
    RevisionPreserved,
    BestKnown,
    MeasuredBestAmongTested,
    HeuristicSelected,
    InstanceOptimal {
        alpha: u64,
        beta: u64,
    },
}

impl ClaimClass {
    /// The constructor name in the LexLean model.
    #[must_use]
    pub const fn constructor(&self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::NormalForm => "normalForm",
            Self::Canonical => "canonical",
            Self::RepresentationMinimal => "representationMinimal",
            Self::ComparisonTheorem => "comparisonTheorem",
            Self::ProfileDefinedComparison { .. } => "profileDefinedComparison",
            Self::InputTotal => "inputTotal",
            Self::GlobalOptimal => "globalOptimal",
            Self::ArgminComplete => "argminComplete",
            Self::ParetoOptimal => "paretoOptimal",
            Self::FrontierComplete => "frontierComplete",
            Self::PointwiseEnvelopeComplete => "pointwiseEnvelopeComplete",
            Self::QueryFamilyAnswerComplete => "queryFamilyAnswerComplete",
            Self::UseCaseGlobalOptimal => "useCaseGlobalOptimal",
            Self::WorkloadArgminComplete => "workloadArgminComplete",
            Self::WorkloadParetoOptimal => "workloadParetoOptimal",
            Self::WorkloadFrontierComplete => "workloadFrontierComplete",
            Self::FamilyOptimal => "familyOptimal",
            Self::CompetitiveBound => "competitiveBound",
            Self::CompetitiveOptimal => "competitiveOptimal",
            Self::AsymptoticBound => "asymptoticBound",
            Self::AsymptoticOptimal => "asymptoticOptimal",
            Self::UseCaseClassComplete => "useCaseClassComplete",
            Self::UseCaseClassAnswerComplete => "useCaseClassAnswerComplete",
            Self::MaintainedUseCaseClass => "maintainedUseCaseClass",
            Self::RestrictedUniverseOptimal => "restrictedUniverseOptimal",
            Self::RevisionPreserved => "revisionPreserved",
            Self::BestKnown => "bestKnown",
            Self::MeasuredBestAmongTested => "measuredBestAmongTested",
            Self::HeuristicSelected => "heuristicSelected",
            Self::InstanceOptimal { .. } => "instanceOptimal",
        }
    }
}

/// A kind of action a system may take inside the machine boundary
/// (UOR-GNAF §8.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    Observation,
    Preprocessing,
    Advice,
    RetainedState,
    Dispatch,
    Fallback,
    Communication,
    Randomness,
    Scheduling,
    Execution,
}

impl ActionKind {
    /// Every kind, in the model's order.
    pub const ALL: [Self; 10] = [
        Self::Observation,
        Self::Preprocessing,
        Self::Advice,
        Self::RetainedState,
        Self::Dispatch,
        Self::Fallback,
        Self::Communication,
        Self::Randomness,
        Self::Scheduling,
        Self::Execution,
    ];

    /// The constructor name in the LexLean model.
    #[must_use]
    pub const fn constructor(self) -> &'static str {
        match self {
            Self::Observation => "observation",
            Self::Preprocessing => "preprocessing",
            Self::Advice => "advice",
            Self::RetainedState => "retainedState",
            Self::Dispatch => "dispatch",
            Self::Fallback => "fallback",
            Self::Communication => "communication",
            Self::Randomness => "randomness",
            Self::Scheduling => "scheduling",
            Self::Execution => "execution",
        }
    }

    /// `Gnaf.performed`: the calculus machine itself does this work, so the
    /// step count is its only faithful charge.
    #[must_use]
    pub const fn performed(self) -> bool {
        matches!(
            self,
            Self::Observation | Self::Dispatch | Self::Fallback | Self::Execution
        )
    }

    /// `Gnaf.preparation`: work before the invocation, which the calculus
    /// has no phase for.
    #[must_use]
    pub const fn preparation(self) -> bool {
        matches!(
            self,
            Self::Preprocessing | Self::Advice | Self::RetainedState
        )
    }
}

/// How one in-boundary action is accounted (UOR-GNAF §8.2, §9.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Charge {
    /// Charged by the calculus step count.
    Steps,
    /// A fixed charge per invocation for every plan that needs this
    /// preparation.
    Constant { cost: u64 },
    /// Explicitly declared free.
    Free,
    /// Present in the boundary with no declared accounting.
    Undeclared,
}

/// One in-boundary action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub kind: ActionKind,
    pub charge: Charge,
}

/// The comparison boundary (UOR-GNAF §10.8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Boundary {
    Complete,
    PreparedState,
    PreparedPlan,
}

/// The operand-size treatment of the cost model (UOR-GNAF §8.2, §9.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperandSize {
    /// Every primitive is charged its operands' and result's weight, as the
    /// calculus denotation charges it (§17.14).
    Weighted,
    /// Every primitive is charged one step whatever its operands' size.
    Unit,
}

/// The machine's hard capacity (UOR-GNAF §8.2): the largest fuel, domain,
/// universe, and declared charge it admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capacity {
    pub fuel: u64,
    pub domain: u64,
    pub systems: u64,
    pub charge: u64,
}

/// A prepared artifact bound in the common initial state of every
/// competitor (UOR-GNAF §10.8): the preparation action that produced it and
/// the SHA-256 of its bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Prepared {
    pub kind: ActionKind,
    pub artifact: Sha256Digest,
}

/// The machine contract: the calculus machine, its fuel and capacity, its
/// operand-size treatment, and its accounting boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Machine {
    pub fuel: u64,
    pub capacity: Capacity,
    pub operand_size: OperandSize,
    pub actions: Vec<Action>,
    pub boundary: Boundary,
    pub prepared: Vec<Prepared>,
}

/// One complete system of the grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Selector {
    /// Always run one plan.
    Fixed { plan: u64 },
    /// Run `small` when the argument's length is below `threshold`, else
    /// `large`.
    Dispatch {
        threshold: u64,
        small: u64,
        large: u64,
    },
}

/// A plan shared by every system of a grammar, with the preparation actions
/// it needs before an invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub function: Function,
    pub prepares: Vec<ActionKind>,
}

/// The universe carrier (UOR-GNAF §8.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Carrier {
    /// Every system the grammar generates over the shared plans.
    Grammar {
        argument: Ty,
        result: Ty,
        plans: Vec<Plan>,
        thresholds: Vec<u64>,
    },
    /// One system's internal plans taken as the universe of systems.
    InternalPlans,
    /// The candidates an optimizer returned.
    OptimizerOutput,
    /// The candidates one search encountered.
    Discovered { members: Vec<u64> },
    /// The candidates currently cached.
    Cached,
}

/// The universe's completeness evidence (UOR-GNAF §8.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Completeness {
    /// The kernel theorem `Gnaf.expandComplete`: the expansion's members are
    /// exactly the grammar's well-formed selectors.
    GrammarEquality,
    Missing,
    CitesUniverseId,
    CitesOptimizer,
}

/// The universe a claim ranges over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    GrammarUniverse,
    CalculusPrograms,
    RustPrograms,
}

/// The objective: total steps, or the vector (steps, size).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Objective {
    Scalar,
    Vector,
}

/// A request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub spec: String,
    pub reference: Program,
    pub domain: Vec<Value>,
    pub machine: Machine,
    pub carrier: Carrier,
    pub completeness: Completeness,
    /// The `SystemUniverseId` (UOR-GNAF §8.3): [`universe_id`] of the
    /// problem, machine, and carrier, fixed before evaluation.
    pub universe: Sha256Digest,
    pub objective: Objective,
    pub claim: ClaimClass,
    pub scope: Scope,
}

/// Why a request is refused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Rejection {
    EmptyDomain,
    InternalPlanUniverse,
    OptimizerDefinedUniverse,
    DiscoveredUniverse,
    CachedUniverse,
    MissingCompleteness,
    SelfReferentialCompleteness,
    OptimizerCompleteness,
    BeyondCapacity,
    UnitCostOperands,
    DuplicateAction { action: ActionKind },
    UnaccountedAction { action: ActionKind },
    HiddenCost { action: ActionKind },
    UnrealizableAction { action: ActionKind },
    UnboundPreparation { action: ActionKind },
    StrayPreparedArtifact { action: ActionKind },
    ClaimAlias,
    ScalarClaimOverPartialOrder,
    VectorClaimOverTotalOrder,
    UncoveredScope,
    UnsupportedClaim,
}

/// The answer of a request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Answer {
    Rejected {
        rejection: Rejection,
    },
    /// Every system attaining the minimum total steps, and that minimum.
    Argmin {
        members: Vec<u64>,
        steps: u64,
    },
    /// Every system no other admitted system strictly dominates.
    Frontier {
        members: Vec<u64>,
    },
    Infeasible,
    Incomplete,
}

/// A committed request with the answer the model gives it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fixture {
    pub spec: String,
    pub name: String,
    pub request: Request,
    pub expected: Answer,
}

/// A system's admission status (UOR-GNAF §8.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Admitted { steps: u64, size: u64 },
    Inadmissible,
    Unresolved,
}

fn malformed(reason: &str) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(
        code!("LLB6006"),
        format!("GNAF request: {reason}"),
    ))
}

fn beyond_capacity(reason: &str) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(
        code!("LLS8002"),
        format!("GNAF request: {reason}"),
    ))
}

/// The number of systems a carrier's grammar generates, `n + |T| * n^2`
/// for `n` plans and `|T|` thresholds, or `None` beyond `u64`. A carrier
/// that is not a grammar generates none.
#[must_use]
pub fn universe_size(carrier: &Carrier) -> Option<u64> {
    let Carrier::Grammar {
        plans, thresholds, ..
    } = carrier
    else {
        return Some(0);
    };
    let count = u64::try_from(plans.len()).ok()?;
    let pairs = count.checked_mul(count)?;
    u64::try_from(thresholds.len())
        .ok()?
        .checked_mul(pairs)?
        .checked_add(count)
}

/// The canonical JSON bytes of a request component, or `LLB6006` for a
/// number canonical JSON cannot represent.
fn canonical_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, LexLeanError> {
    let text = serde_json::to_string(value).map_err(|error| malformed(&error.to_string()))?;
    crate::artifact::canonical_json::Json::parse(text.as_bytes())
        .map(|json| json.to_file_bytes())
        .map_err(|error| malformed(&format!("not canonical JSON: {error}")))
}

/// The `SystemUniverseId` of a request: the framed SHA-256 of its problem
/// (reference and domain), machine, and carrier, each in canonical JSON.
/// It is computed from the request alone, before any evaluation, so a
/// stated identity is only an equality check against it (UOR-GNAF §8.3).
///
/// # Errors
///
/// `LLB6006` for a component holding a number canonical JSON cannot
/// represent.
pub fn universe_id(request: &Request) -> Result<Sha256Digest, LexLeanError> {
    let problem = json!({"reference": request.reference, "domain": request.domain});
    let mut hasher = FramedHasher::new("lexlean-gnaf-universe-v1");
    hasher.frame("problem", &canonical_bytes(&problem)?);
    hasher.frame("machine", &canonical_bytes(&request.machine)?);
    hasher.frame("carrier", &canonical_bytes(&request.carrier)?);
    Ok(hasher.finish())
}

/// The host's capacities: the machine a request asks for is one the host
/// evaluator offers, and the request's use of it fits.
fn check_capacity(request: &Request) -> Result<(), LexLeanError> {
    let asked = request.machine.capacity;
    for (name, asked, offered) in [
        ("fuel", asked.fuel, HOST_CAPACITY.fuel),
        ("domain", asked.domain, HOST_CAPACITY.domain),
        ("systems", asked.systems, HOST_CAPACITY.systems),
        ("charge", asked.charge, HOST_CAPACITY.charge),
    ] {
        if asked > offered {
            return Err(beyond_capacity(&format!(
                "a {name} capacity of {asked} exceeds the evaluator's {offered}"
            )));
        }
    }
    if request.machine.fuel > HOST_CAPACITY.fuel {
        return Err(beyond_capacity(&format!(
            "fuel {} exceeds the evaluator's capacity {}",
            request.machine.fuel, HOST_CAPACITY.fuel
        )));
    }
    if request.domain.len() as u64 > HOST_CAPACITY.domain {
        return Err(beyond_capacity(&format!(
            "a domain of {} arguments exceeds the evaluator's capacity {}",
            request.domain.len(),
            HOST_CAPACITY.domain
        )));
    }
    match universe_size(&request.carrier) {
        Some(size) if size <= HOST_CAPACITY.systems => {}
        _ => {
            return Err(beyond_capacity(&format!(
                "the universe exceeds the evaluator's capacity of {} systems",
                HOST_CAPACITY.systems
            )))
        }
    }
    if let Some(cost) = request
        .machine
        .actions
        .iter()
        .find_map(|action| match action.charge {
            Charge::Constant { cost } if cost > HOST_CAPACITY.charge => Some(cost),
            _ => None,
        })
    {
        return Err(beyond_capacity(&format!(
            "a charge of {cost} exceeds the evaluator's capacity {}",
            HOST_CAPACITY.charge
        )));
    }
    Ok(())
}

/// Read a request and check what the model presupposes of it: its programs
/// and plans are valid calculus code, the reference defines the problem on
/// every domain argument, its data are well formed, its universe identity
/// is the one its components determine, and it fits the host's capacity.
/// Validation of the GNAF components is not an error: a refused request has
/// the answer `rejected`.
///
/// # Errors
///
/// `LLB6006` for a malformed request, an invalid reference program, a
/// domain argument on which the reference returns no value, a grammar whose
/// realizations are not valid programs or whose thresholds are not strictly
/// increasing, a plan preparing by an action that is not preparation or
/// naming one twice, a number canonical JSON cannot represent, or a stated
/// universe identity that differs from [`universe_id`]; `LLS8002` for a
/// request beyond [`HOST_CAPACITY`].
pub fn load(bytes: &[u8]) -> Result<Request, LexLeanError> {
    let request: Request =
        serde_json::from_slice(bytes).map_err(|error| malformed(&format!("malformed: {error}")))?;
    if request.spec != REQUEST_SPEC {
        return Err(malformed(&format!(
            "spec `{}`, expected `{REQUEST_SPEC}`",
            request.spec
        )));
    }
    check_capacity(&request)?;
    crate::calculus::check::check(&request.reference)
        .map_err(|reason| malformed(&format!("reference: {reason}")))?;
    // The problem is a unary function: every domain argument is an argument
    // of the reference's entry, and the grammar's systems have exactly the
    // entry's signature, so no argument can make a system stuck by type.
    for (position, argument) in request.domain.iter().enumerate() {
        crate::calculus::check::check_arguments(
            &request.reference,
            0,
            std::slice::from_ref(argument),
        )
        .map_err(|reason| malformed(&format!("domain argument {position}: {reason}")))?;
    }
    if let Carrier::Grammar {
        argument,
        result,
        plans,
        thresholds,
    } = &request.carrier
    {
        let entry = request
            .reference
            .functions
            .first()
            .ok_or_else(|| malformed("the reference has no entry function"))?;
        if entry.types != std::slice::from_ref(argument) || &entry.result != result {
            return Err(malformed(
                "the grammar's argument and result types differ from the reference entry's",
            ));
        }
        // A threshold named twice would make one system two members.
        if thresholds.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(malformed("the thresholds are not strictly increasing"));
        }
        for (index, plan) in plans.iter().enumerate() {
            for (position, kind) in plan.prepares.iter().enumerate() {
                if !kind.preparation() {
                    return Err(malformed(&format!(
                        "plan {index} prepares by `{}`, which is not a preparation action",
                        kind.constructor()
                    )));
                }
                if plan.prepares[..position].contains(kind) {
                    return Err(malformed(&format!(
                        "plan {index} names its preparation `{}` twice",
                        kind.constructor()
                    )));
                }
            }
        }
        for (index, selector) in expand(&request.carrier).into_iter().enumerate() {
            let system = realize(&request.carrier, selector).expect("a grammar carrier");
            crate::calculus::check::check(&system)
                .map_err(|reason| malformed(&format!("system {index}: {reason}")))?;
        }
    }
    // The reference defines the problem: an argument on which it returns no
    // value has no correct answer, so it is no argument of the problem.
    on_evaluation_stack(&request, |request| {
        request
            .domain
            .iter()
            .enumerate()
            .find_map(|(position, argument)| {
                match interp::run(
                    &request.reference,
                    request.machine.fuel,
                    0,
                    std::slice::from_ref(argument),
                ) {
                    Outcome::Value { .. } => None,
                    other => Some(malformed(&format!(
                        "domain argument {position}: the reference returns no value ({})",
                        match other {
                            Outcome::Overflow { .. } => "overflow",
                            Outcome::Stuck => "stuck",
                            Outcome::Exhausted | Outcome::Value { .. } => "exhausted",
                        }
                    ))),
                }
            })
    })?
    .map_or(Ok(()), Err)?;
    let identity = universe_id(&request)?;
    if request.universe != identity {
        return Err(malformed(&format!(
            "the stated universe identity {} is not {identity}, the identity of the request's problem, machine, and carrier",
            request.universe
        )));
    }
    Ok(request)
}

fn find_action(actions: &[Action], wanted: ActionKind) -> Option<Charge> {
    actions
        .iter()
        .find(|action| action.kind == wanted)
        .map(|action| action.charge)
}

/// `Gnaf.checkDistinct`.
fn check_distinct(actions: &[Action]) -> Option<Rejection> {
    actions
        .iter()
        .find(|action| {
            actions
                .iter()
                .filter(|other| other.kind == action.kind)
                .count()
                > 1
        })
        .map(|action| Rejection::DuplicateAction {
            action: action.kind,
        })
}

/// `Gnaf.checkPerformed`.
fn check_performed(machine: &Machine, wanted: ActionKind) -> Option<Rejection> {
    match find_action(&machine.actions, wanted) {
        None => Some(Rejection::UnaccountedAction { action: wanted }),
        Some(Charge::Steps) => None,
        Some(Charge::Constant { .. } | Charge::Free | Charge::Undeclared) => {
            Some(Rejection::HiddenCost { action: wanted })
        }
    }
}

/// `Gnaf.checkDeclared`.
fn check_declared(machine: &Machine, action: &Action) -> Option<Rejection> {
    let kind = action.kind;
    if kind.performed() {
        return None;
    }
    if !kind.preparation() {
        return Some(Rejection::UnrealizableAction { action: kind });
    }
    match action.charge {
        Charge::Constant { cost } if cost > 0 => None,
        Charge::Free => {
            let bound = machine
                .prepared
                .iter()
                .any(|prepared| prepared.kind == kind);
            match machine.boundary {
                Boundary::Complete => Some(Rejection::UnboundPreparation { action: kind }),
                Boundary::PreparedState | Boundary::PreparedPlan => {
                    (!bound).then_some(Rejection::UnboundPreparation { action: kind })
                }
            }
        }
        Charge::Steps | Charge::Constant { .. } | Charge::Undeclared => {
            Some(Rejection::HiddenCost { action: kind })
        }
    }
}

/// `Gnaf.checkPrepared`: a prepared artifact is excluded from every
/// system's cost only as the product of an action admitted free.
fn check_prepared(machine: &Machine) -> Option<Rejection> {
    machine.prepared.iter().find_map(|prepared| {
        (find_action(&machine.actions, prepared.kind) != Some(Charge::Free)).then_some(
            Rejection::StrayPreparedArtifact {
                action: prepared.kind,
            },
        )
    })
}

/// `Gnaf.checkPlanPreparation`: every preparation a plan needs is
/// accounted in the machine.
fn check_plan_preparation(request: &Request) -> Option<Rejection> {
    let Carrier::Grammar { plans, .. } = &request.carrier else {
        return None;
    };
    plans
        .iter()
        .flat_map(|plan| plan.prepares.iter())
        .find(|kind| find_action(&request.machine.actions, **kind).is_none())
        .map(|kind| Rejection::UnaccountedAction { action: *kind })
}

/// `Gnaf.checkCapacity`: the request's use of the machine fits the
/// capacity the machine binds.
fn check_request_capacity(request: &Request) -> Option<Rejection> {
    let capacity = request.machine.capacity;
    let charges = request
        .machine
        .actions
        .iter()
        .any(|action| match action.charge {
            Charge::Constant { cost } => cost > capacity.charge,
            Charge::Steps | Charge::Free | Charge::Undeclared => false,
        });
    let systems = universe_size(&request.carrier).is_none_or(|size| size > capacity.systems);
    (request.machine.fuel > capacity.fuel
        || request.domain.len() as u64 > capacity.domain
        || systems
        || charges)
        .then_some(Rejection::BeyondCapacity)
}

fn check_claim(request: &Request) -> Option<Rejection> {
    match request.claim {
        // §12.4: a legacy alias that a conforming answer never emits as a
        // base class; the base class with its scope is the request.
        ClaimClass::RestrictedUniverseOptimal => Some(Rejection::ClaimAlias),
        ClaimClass::GlobalOptimal | ClaimClass::ArgminComplete => (request.objective
            == Objective::Vector)
            .then_some(Rejection::ScalarClaimOverPartialOrder),
        ClaimClass::ParetoOptimal | ClaimClass::FrontierComplete => {
            (request.objective == Objective::Scalar).then_some(Rejection::VectorClaimOverTotalOrder)
        }
        ClaimClass::Exact
        | ClaimClass::NormalForm
        | ClaimClass::Canonical
        | ClaimClass::RepresentationMinimal
        | ClaimClass::ComparisonTheorem
        | ClaimClass::ProfileDefinedComparison { .. }
        | ClaimClass::InputTotal
        | ClaimClass::PointwiseEnvelopeComplete
        | ClaimClass::QueryFamilyAnswerComplete
        | ClaimClass::UseCaseGlobalOptimal
        | ClaimClass::WorkloadArgminComplete
        | ClaimClass::WorkloadParetoOptimal
        | ClaimClass::WorkloadFrontierComplete
        | ClaimClass::FamilyOptimal
        | ClaimClass::CompetitiveBound
        | ClaimClass::CompetitiveOptimal
        | ClaimClass::AsymptoticBound
        | ClaimClass::AsymptoticOptimal
        | ClaimClass::UseCaseClassComplete
        | ClaimClass::UseCaseClassAnswerComplete
        | ClaimClass::MaintainedUseCaseClass
        | ClaimClass::RevisionPreserved
        | ClaimClass::BestKnown
        | ClaimClass::MeasuredBestAmongTested
        | ClaimClass::HeuristicSelected
        | ClaimClass::InstanceOptimal { .. } => Some(Rejection::UnsupportedClaim),
    }
}

/// `Gnaf.validate`: the first violated rule, in the model's order.
#[must_use]
pub fn validate(request: &Request) -> Option<Rejection> {
    if request.domain.is_empty() {
        return Some(Rejection::EmptyDomain);
    }
    match &request.carrier {
        Carrier::Grammar { .. } => {}
        Carrier::InternalPlans => return Some(Rejection::InternalPlanUniverse),
        Carrier::OptimizerOutput => return Some(Rejection::OptimizerDefinedUniverse),
        Carrier::Discovered { .. } => return Some(Rejection::DiscoveredUniverse),
        Carrier::Cached => return Some(Rejection::CachedUniverse),
    }
    match request.completeness {
        Completeness::GrammarEquality => {}
        Completeness::Missing => return Some(Rejection::MissingCompleteness),
        Completeness::CitesUniverseId => return Some(Rejection::SelfReferentialCompleteness),
        Completeness::CitesOptimizer => return Some(Rejection::OptimizerCompleteness),
    }
    if let Some(rejection) = check_request_capacity(request) {
        return Some(rejection);
    }
    if request.machine.operand_size == OperandSize::Unit {
        return Some(Rejection::UnitCostOperands);
    }
    if let Some(rejection) = check_distinct(&request.machine.actions) {
        return Some(rejection);
    }
    let performed = ActionKind::ALL.into_iter().filter(|kind| kind.performed());
    if let Some(rejection) = performed
        .filter_map(|kind| check_performed(&request.machine, kind))
        .next()
    {
        return Some(rejection);
    }
    if let Some(rejection) = request
        .machine
        .actions
        .iter()
        .find_map(|action| check_declared(&request.machine, action))
    {
        return Some(rejection);
    }
    if let Some(rejection) = check_prepared(&request.machine) {
        return Some(rejection);
    }
    if let Some(rejection) = check_plan_preparation(request) {
        return Some(rejection);
    }
    if let Some(rejection) = check_claim(request) {
        return Some(rejection);
    }
    match request.scope {
        Scope::GrammarUniverse => None,
        Scope::CalculusPrograms | Scope::RustPrograms => Some(Rejection::UncoveredScope),
    }
}

/// `Gnaf.expand`: every fixed plan, then for every threshold every dispatch
/// from a small plan to a large plan, both ranging over every plan, in that
/// order. Empty for a carrier that is not a grammar.
#[must_use]
pub fn expand(carrier: &Carrier) -> Vec<Selector> {
    let Carrier::Grammar {
        plans, thresholds, ..
    } = carrier
    else {
        return Vec::new();
    };
    let count = plans.len() as u64;
    let mut out: Vec<Selector> = (0..count).map(|plan| Selector::Fixed { plan }).collect();
    for threshold in thresholds {
        for small in 0..count {
            for large in 0..count {
                out.push(Selector::Dispatch {
                    threshold: *threshold,
                    small,
                    large,
                });
            }
        }
    }
    out
}

/// `Gnaf.wellFormed`: the grammar's membership semantics, stated without
/// the expansion; `Gnaf.expandComplete` proves the two agree.
#[must_use]
pub fn well_formed(carrier: &Carrier, selector: Selector) -> bool {
    let Carrier::Grammar {
        plans, thresholds, ..
    } = carrier
    else {
        return false;
    };
    let count = plans.len() as u64;
    match selector {
        Selector::Fixed { plan } => plan < count,
        Selector::Dispatch {
            threshold,
            small,
            large,
        } => thresholds.contains(&threshold) && small < count && large < count,
    }
}

/// `Gnaf.entryBody`.
#[must_use]
pub fn entry_body(selector: Selector) -> Expr {
    let argument = || Expr::Var { name: 0 };
    let run = |plan: u64| Expr::Call {
        function: plan + 1,
        operands: vec![argument()],
    };
    match selector {
        Selector::Fixed { plan } => run(plan),
        Selector::Dispatch {
            threshold,
            small,
            large,
        } => Expr::Cond {
            condition: Box::new(Expr::Prim {
                operation: Prim::NatLt,
                operands: vec![
                    Expr::Prim {
                        operation: Prim::Length,
                        operands: vec![argument()],
                    },
                    Expr::Value {
                        ty: Ty::Nat,
                        value: Value::Nat {
                            value: threshold.to_string(),
                        },
                    },
                ],
            }),
            then_branch: Box::new(run(small)),
            else_branch: Box::new(run(large)),
        },
    }
}

/// `Gnaf.realize`: the selector over the shared plans at indices 1...
#[must_use]
pub fn realize(carrier: &Carrier, selector: Selector) -> Option<Program> {
    let Carrier::Grammar {
        argument,
        result,
        plans,
        ..
    } = carrier
    else {
        return None;
    };
    let mut functions = vec![Function {
        parameters: vec![0],
        types: vec![argument.clone()],
        result: result.clone(),
        body: entry_body(selector),
    }];
    functions.extend(plans.iter().map(|plan| plan.function.clone()));
    Some(Program {
        spec: PROGRAM_SPEC.to_owned(),
        adts: Vec::new(),
        functions,
    })
}

/// `Gnaf.exprCallees`: the functions an expression calls or closes over,
/// in evaluation order.
fn callees(expr: &Expr, out: &mut Vec<u64>) {
    let all = |exprs: &[Expr], out: &mut Vec<u64>| {
        for expr in exprs {
            callees(expr, out);
        }
    };
    match expr {
        Expr::Value { .. } | Expr::Var { .. } => {}
        Expr::Let { bound, body, .. } => {
            callees(bound, out);
            callees(body, out);
        }
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => {
            callees(condition, out);
            callees(then_branch, out);
            callees(else_branch, out);
        }
        Expr::Match {
            scrutinee, arms, ..
        } => {
            callees(scrutinee, out);
            for arm in arms {
                callees(&arm.body, out);
            }
        }
        Expr::Build { operands, .. } | Expr::Prim { operands, .. } => all(operands, out),
        Expr::Call { function, operands }
        | Expr::Closure {
            function,
            captures: operands,
        } => {
            out.push(*function);
            all(operands, out);
        }
        Expr::Apply { target, operands } => {
            callees(target, out);
            all(operands, out);
        }
        Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
            callees(value, out);
        }
    }
}

/// `Gnaf.reachable`: the functions a program can run from its entry, in
/// discovery order. Each of `|functions|` rounds adds every function a known
/// one calls or closes over, which reaches the fixed point.
#[must_use]
pub fn reachable(program: &Program) -> Vec<u64> {
    let mut known = vec![0u64];
    for _ in 0..program.functions.len() {
        let mut found = Vec::new();
        for index in &known {
            if let Some(function) = usize::try_from(*index)
                .ok()
                .and_then(|index| program.functions.get(index))
            {
                callees(&function.body, &mut found);
            }
        }
        for index in found {
            if !known.contains(&index) {
                known.push(index);
            }
        }
    }
    known
}

fn expr_size(expr: &Expr) -> u64 {
    let all = |exprs: &[Expr]| exprs.iter().map(expr_size).sum::<u64>();
    1 + match expr {
        Expr::Value { .. } | Expr::Var { .. } => 0,
        Expr::Let { bound, body, .. } => expr_size(bound) + expr_size(body),
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => expr_size(condition) + expr_size(then_branch) + expr_size(else_branch),
        Expr::Match {
            scrutinee, arms, ..
        } => expr_size(scrutinee) + arms.iter().map(|arm| 1 + expr_size(&arm.body)).sum::<u64>(),
        Expr::Build { operands, .. }
        | Expr::Call { operands, .. }
        | Expr::Closure {
            captures: operands, ..
        }
        | Expr::Prim { operands, .. } => all(operands),
        Expr::Apply { target, operands } => expr_size(target) + all(operands),
        Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
            expr_size(value)
        }
    }
}

/// `Gnaf.reachableSize`: one per expression node and per arm of every
/// function the program can run, so code a system never reaches is not its
/// size.
#[must_use]
pub fn program_size(program: &Program) -> u64 {
    reachable(program)
        .into_iter()
        .filter_map(|index| {
            usize::try_from(index)
                .ok()
                .and_then(|index| program.functions.get(index))
        })
        .map(|function| expr_size(&function.body))
        .sum()
}

/// `Gnaf.unitCharge`: what one plan's need of `kind` costs per invocation.
fn unit_charge(actions: &[Action], kind: ActionKind) -> u64 {
    match find_action(actions, kind) {
        Some(Charge::Constant { cost }) => cost,
        Some(Charge::Steps | Charge::Free | Charge::Undeclared) | None => 0,
    }
}

/// `Gnaf.systemCharge`: the per-invocation preparation of every plan a
/// system can run, so a plan's preparation is charged exactly to the
/// systems that use it; `None` beyond `u64`.
#[must_use]
pub fn system_charge(request: &Request, program: &Program) -> Option<u64> {
    let Carrier::Grammar { plans, .. } = &request.carrier else {
        return Some(0);
    };
    reachable(program)
        .into_iter()
        .filter_map(|index| {
            index
                .checked_sub(1)
                .and_then(|plan| usize::try_from(plan).ok())
                .and_then(|plan| plans.get(plan))
        })
        .flat_map(|plan| plan.prepares.iter())
        .try_fold(0u64, |total, kind| {
            total.checked_add(unit_charge(&request.machine.actions, *kind))
        })
}

/// `Gnaf.statusOn`: the first invocation, in domain order, that does not
/// produce the reference value decides; an exhausted evaluation leaves
/// admission unresolved. `Err` when the step total leaves the host's
/// integers.
fn status_on(
    system: &Program,
    reference: &Program,
    fuel: u64,
    domain: &[Value],
) -> Result<Status, LexLeanError> {
    let mut steps = 0u64;
    for argument in domain {
        let arguments = std::slice::from_ref(argument);
        let produced = match interp::run(system, fuel, 0, arguments) {
            Outcome::Value { value, steps } => (value, steps),
            Outcome::Exhausted => return Ok(Status::Unresolved),
            Outcome::Overflow { .. } | Outcome::Stuck => return Ok(Status::Inadmissible),
        };
        let Outcome::Value {
            value: expected, ..
        } = interp::run(reference, fuel, 0, arguments)
        else {
            return Ok(Status::Unresolved);
        };
        if produced.0 != expected {
            return Ok(Status::Inadmissible);
        }
        steps = steps.checked_add(produced.1).ok_or_else(|| {
            beyond_capacity("a system's step total exceeds the evaluator's integers")
        })?;
    }
    Ok(Status::Admitted { steps, size: 0 })
}

/// The `LLI9001` an evaluation reports when its thread panics: a panic is a
/// defect of this host, never a property of the request or the platform.
fn evaluation_panicked(payload: &(dyn std::any::Any + Send)) -> LexLeanError {
    let reason = payload
        .downcast_ref::<&str>()
        .map(|text| (*text).to_owned())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "a panic without a message".to_owned());
    LexLeanError::from_diagnostic(Diagnostic::new(
        code!("LLI9001"),
        format!("GNAF evaluation: {reason}"),
    ))
}

/// Run an evaluation of `request` on a thread whose stack its fuel cannot
/// exhaust.
fn on_evaluation_stack<T: Send>(
    request: &Request,
    evaluation: impl FnOnce(&Request) -> T + Send,
) -> Result<T, LexLeanError> {
    check_capacity(request)?;
    let levels = usize::try_from(request.machine.fuel).unwrap_or(usize::MAX);
    let stack = levels.saturating_add(64).saturating_mul(STACK_PER_LEVEL);
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .name("gnaf-evaluation".to_owned())
            .stack_size(stack)
            .spawn_scoped(scope, || evaluation(request))
            .map_err(|error| {
                LexLeanError::from_diagnostic(Diagnostic::new(
                    code!("LLV7010"),
                    format!("GNAF evaluation thread: {error}"),
                ))
            })?
            .join()
            .map_err(|payload| evaluation_panicked(payload.as_ref()))
    })
}

/// `Gnaf.statuses`: every system of a grammar request with its status,
/// numbered in expansion order, each admitted system charged the
/// preparation of the plans it can run on every invocation.
///
/// # Errors
///
/// `LLS8002` for a request beyond the host's capacities, `LLV7010` when the
/// platform cannot provide the evaluation thread, `LLI9001` when the
/// evaluation fails internally.
pub fn statuses(request: &Request) -> Result<Vec<(u64, Selector, Status)>, LexLeanError> {
    on_evaluation_stack(request, evaluate_statuses)?
}

fn evaluate_statuses(request: &Request) -> Result<Vec<(u64, Selector, Status)>, LexLeanError> {
    expand(&request.carrier)
        .into_iter()
        .enumerate()
        .map(|(index, selector)| {
            let system = realize(&request.carrier, selector).expect("a grammar carrier");
            let status = match status_on(
                &system,
                &request.reference,
                request.machine.fuel,
                &request.domain,
            )? {
                Status::Admitted { steps, .. } => {
                    let charged = system_charge(request, &system)
                        .and_then(|charge| charge.checked_mul(request.domain.len() as u64))
                        .and_then(|charged| charged.checked_add(steps))
                        .ok_or_else(|| {
                            beyond_capacity("a system's cost exceeds the evaluator's integers")
                        })?;
                    Status::Admitted {
                        steps: charged,
                        size: program_size(&system),
                    }
                }
                other => other,
            };
            Ok((index as u64, selector, status))
        })
        .collect()
}

// --- the order -------------------------------------------------------------

/// `Gnaf.argmin`: every identity attaining the least cost, in row order,
/// and that cost; `None` for no rows.
#[must_use]
pub fn argmin<Id: Clone>(rows: &[(Id, u64)]) -> Option<(Vec<Id>, u64)> {
    let best = rows.iter().map(|(_, cost)| *cost).min()?;
    Some((
        rows.iter()
            .filter(|(_, cost)| *cost == best)
            .map(|(id, _)| id.clone())
            .collect(),
        best,
    ))
}

/// `Gnaf.weaklyBelow`: componentwise at most, over vectors of one length.
#[must_use]
pub fn weakly_below(left: &[u64], right: &[u64]) -> bool {
    left.len() == right.len() && left.iter().zip(right).all(|(left, right)| left <= right)
}

/// `Gnaf.dominates` (UOR-GNAF §9.3): at most in every component and below
/// in one.
#[must_use]
pub fn dominates(left: &[u64], right: &[u64]) -> bool {
    weakly_below(left, right) && !weakly_below(right, left)
}

/// `Gnaf.frontier`: every identity no row strictly dominates, in row order;
/// rows of equal cost dominate neither each other nor themselves.
#[must_use]
pub fn frontier<Id: Clone>(rows: &[(Id, Vec<u64>)]) -> Vec<Id> {
    rows.iter()
        .filter(|(_, cost)| !rows.iter().any(|(_, other)| dominates(other, cost)))
        .map(|(id, _)| id.clone())
        .collect()
}

/// `Gnaf.sameIdsBy`: the same identity set.
fn same_members(left: &[u64], right: &[u64]) -> bool {
    left.iter().all(|id| right.contains(id)) && right.iter().all(|id| left.contains(id))
}

/// `Gnaf.componentwiseMinimum`: the least value of every component, each
/// possibly from a different row.
#[must_use]
pub fn componentwise_minimum<Id>(rows: &[(Id, Vec<u64>)]) -> Vec<u64> {
    let mut rows = rows.iter();
    let Some((_, first)) = rows.next() else {
        return Vec::new();
    };
    rows.fold(first.clone(), |minimum, (_, cost)| {
        minimum
            .iter()
            .zip(cost)
            .map(|(left, right)| *left.min(right))
            .collect()
    })
}

/// `Gnaf.attains`: some row costs exactly `cost`.
#[must_use]
pub fn attains<Id>(rows: &[(Id, Vec<u64>)], cost: &[u64]) -> bool {
    rows.iter()
        .any(|(_, row)| weakly_below(row, cost) && weakly_below(cost, row))
}

/// `Gnaf.answer`.
///
/// # Errors
///
/// `LLS8002` for a request beyond the host's capacities, `LLV7010` when the
/// platform cannot provide the evaluation thread, `LLI9001` when the
/// evaluation fails internally.
pub fn answer(request: &Request) -> Result<Answer, LexLeanError> {
    on_evaluation_stack(request, evaluate_answer)?
}

fn admitted_rows(entries: &[(u64, Selector, Status)]) -> Vec<(u64, Vec<u64>)> {
    entries
        .iter()
        .filter_map(|(index, _, status)| match status {
            Status::Admitted { steps, size } => Some((*index, vec![*steps, *size])),
            Status::Inadmissible | Status::Unresolved => None,
        })
        .collect()
}

fn evaluate_answer(request: &Request) -> Result<Answer, LexLeanError> {
    if let Some(rejection) = validate(request) {
        return Ok(Answer::Rejected { rejection });
    }
    let entries = evaluate_statuses(request)?;
    if entries
        .iter()
        .any(|(_, _, status)| *status == Status::Unresolved)
    {
        return Ok(Answer::Incomplete);
    }
    let rows = admitted_rows(&entries);
    Ok(match request.objective {
        Objective::Scalar => {
            let scalar: Vec<(u64, u64)> = rows.iter().map(|(id, cost)| (*id, cost[0])).collect();
            match argmin(&scalar) {
                None => Answer::Infeasible,
                Some((members, steps)) => Answer::Argmin { members, steps },
            }
        }
        Objective::Vector if rows.is_empty() => Answer::Infeasible,
        Objective::Vector => Answer::Frontier {
            members: frontier(&rows),
        },
    })
}

/// `Gnaf.certifies`: whether `claimed` is exactly the request's optimum,
/// identity-complete (UOR-GNAF §16.24, GNAF-REJ-21 and GNAF-REJ-29): an
/// argmin with the same members and steps, or a frontier with the same
/// members. No other answer is a certifiable optimality claim.
///
/// # Errors
///
/// As [`answer`].
pub fn certifies(request: &Request, claimed: &Answer) -> Result<bool, LexLeanError> {
    Ok(match (answer(request)?, claimed) {
        (
            Answer::Argmin { members, steps },
            Answer::Argmin {
                members: claimed,
                steps: claimed_steps,
            },
        ) => same_members(&members, claimed) && steps == *claimed_steps,
        (Answer::Frontier { members }, Answer::Frontier { members: claimed }) => {
            same_members(&members, claimed)
        }
        _ => false,
    })
}

/// `Gnaf.minimaAttained`: whether some admitted system attains the
/// componentwise minimum of the admitted (steps, size) costs (UOR-GNAF
/// §9.3, GNAF-REJ-14).
///
/// # Errors
///
/// As [`answer`].
pub fn minima_attained(request: &Request) -> Result<bool, LexLeanError> {
    let rows = admitted_rows(&statuses(request)?);
    Ok(attains(&rows, &componentwise_minimum(&rows)))
}

// --- LexLean terms -------------------------------------------------------------

fn model(name: &str, arguments: Vec<Json>) -> Json {
    json!({"kind": "constructor", "constructor": term::member(MODEL, name), "arguments": arguments})
}

fn string(text: &str) -> Json {
    json!({"kind": "string", "value": text})
}

fn record(name: &str, fields: Vec<(&str, Json)>) -> Json {
    let fields: Vec<Json> = fields
        .into_iter()
        .map(|(field, value)| json!({"field": field, "value": value}))
        .collect();
    json!({"kind": "record", "type": term::member(MODEL, name), "fields": fields})
}

fn action_kind(kind: ActionKind) -> Json {
    model(&format!("ActionKind.{}", kind.constructor()), Vec::new())
}

fn charge(charge: &Charge) -> Json {
    match charge {
        Charge::Steps => model("Charge.steps", Vec::new()),
        Charge::Constant { cost } => model("Charge.constant", vec![term::nat(*cost)]),
        Charge::Free => model("Charge.free", Vec::new()),
        Charge::Undeclared => model("Charge.undeclared", Vec::new()),
    }
}

fn kinds(kinds: &[ActionKind]) -> Json {
    term::list(
        &term::named(MODEL, "ActionKind"),
        kinds.iter().map(|kind| action_kind(*kind)).collect(),
    )
}

fn machine(machine: &Machine) -> Json {
    let boundary = match machine.boundary {
        Boundary::Complete => "Boundary.complete",
        Boundary::PreparedState => "Boundary.preparedState",
        Boundary::PreparedPlan => "Boundary.preparedPlan",
    };
    let operand_size = match machine.operand_size {
        OperandSize::Weighted => "OperandSize.weighted",
        OperandSize::Unit => "OperandSize.unit",
    };
    let capacity = machine.capacity;
    let actions = machine
        .actions
        .iter()
        .map(|action| {
            record(
                "Action",
                vec![
                    ("kind", action_kind(action.kind)),
                    ("charge", charge(&action.charge)),
                ],
            )
        })
        .collect();
    let prepared = machine
        .prepared
        .iter()
        .map(|prepared| {
            record(
                "Prepared",
                vec![
                    ("kind", action_kind(prepared.kind)),
                    ("artifact", string(&prepared.artifact.to_hex())),
                ],
            )
        })
        .collect();
    record(
        "Machine",
        vec![
            ("fuel", term::nat(machine.fuel)),
            (
                "capacity",
                record(
                    "Capacity",
                    vec![
                        ("fuel", term::nat(capacity.fuel)),
                        ("domain", term::nat(capacity.domain)),
                        ("systems", term::nat(capacity.systems)),
                        ("charge", term::nat(capacity.charge)),
                    ],
                ),
            ),
            ("operandSize", model(operand_size, Vec::new())),
            (
                "actions",
                term::list(&term::named(MODEL, "Action"), actions),
            ),
            ("boundary", model(boundary, Vec::new())),
            (
                "prepared",
                term::list(&term::named(MODEL, "Prepared"), prepared),
            ),
        ],
    )
}

fn carrier(carrier: &Carrier) -> Json {
    let nat_type = json!({"kind": "nat"});
    match carrier {
        Carrier::Grammar {
            argument,
            result,
            plans,
            thresholds,
        } => model(
            "Carrier.grammar",
            vec![record(
                "Grammar",
                vec![
                    ("argument", term::ty(argument)),
                    ("result", term::ty(result)),
                    (
                        "plans",
                        term::list(
                            &term::named(MODEL, "Plan"),
                            plans
                                .iter()
                                .map(|plan| {
                                    record(
                                        "Plan",
                                        vec![
                                            ("function", term::function(&plan.function)),
                                            ("prepares", kinds(&plan.prepares)),
                                        ],
                                    )
                                })
                                .collect(),
                        ),
                    ),
                    (
                        "thresholds",
                        term::list(
                            &nat_type,
                            thresholds
                                .iter()
                                .map(|threshold| term::nat(*threshold))
                                .collect(),
                        ),
                    ),
                ],
            )],
        ),
        Carrier::InternalPlans => model("Carrier.internalPlans", Vec::new()),
        Carrier::OptimizerOutput => model("Carrier.optimizerOutput", Vec::new()),
        Carrier::Discovered { members } => model(
            "Carrier.discovered",
            vec![term::list(
                &nat_type,
                members.iter().map(|member| term::nat(*member)).collect(),
            )],
        ),
        Carrier::Cached => model("Carrier.cached", Vec::new()),
    }
}

fn claim(claim: &ClaimClass) -> Json {
    let name = format!("ClaimClass.{}", claim.constructor());
    match claim {
        ClaimClass::InstanceOptimal { alpha, beta } => {
            model(&name, vec![term::nat(*alpha), term::nat(*beta)])
        }
        ClaimClass::ProfileDefinedComparison {
            profile,
            class_id,
            shape,
        } => model(
            &name,
            vec![string(profile), string(class_id), string(shape)],
        ),
        _ => model(&name, Vec::new()),
    }
}

/// A selector as a `Gnaf.Selector` term.
#[must_use]
pub fn selector_term(selector: Selector) -> Json {
    match selector {
        Selector::Fixed { plan } => model("Selector.fixed", vec![term::nat(plan)]),
        Selector::Dispatch {
            threshold,
            small,
            large,
        } => model(
            "Selector.dispatch",
            vec![term::nat(threshold), term::nat(small), term::nat(large)],
        ),
    }
}

/// A status as a `Gnaf.Status` term.
#[must_use]
pub fn status_term(status: Status) -> Json {
    match status {
        Status::Admitted { steps, size } => {
            model("Status.admitted", vec![term::nat(steps), term::nat(size)])
        }
        Status::Inadmissible => model("Status.inadmissible", Vec::new()),
        Status::Unresolved => model("Status.unresolved", Vec::new()),
    }
}

/// A request as a `Gnaf.Request` term.
#[must_use]
pub fn request_term(request: &Request) -> Json {
    let completeness = match request.completeness {
        Completeness::GrammarEquality => "Completeness.grammarEquality",
        Completeness::Missing => "Completeness.missing",
        Completeness::CitesUniverseId => "Completeness.citesUniverseId",
        Completeness::CitesOptimizer => "Completeness.citesOptimizer",
    };
    let objective = match request.objective {
        Objective::Scalar => "Objective.scalar",
        Objective::Vector => "Objective.vector",
    };
    let scope = match request.scope {
        Scope::GrammarUniverse => "Scope.grammarUniverse",
        Scope::CalculusPrograms => "Scope.calculusPrograms",
        Scope::RustPrograms => "Scope.rustPrograms",
    };
    record(
        "Request",
        vec![
            ("reference", term::program(&request.reference)),
            (
                "domain",
                term::list(
                    &term::named(term::SYNTAX, "Value"),
                    request.domain.iter().map(term::value).collect(),
                ),
            ),
            ("machine", machine(&request.machine)),
            ("carrier", carrier(&request.carrier)),
            ("completeness", model(completeness, Vec::new())),
            ("universe", string(&request.universe.to_hex())),
            ("objective", model(objective, Vec::new())),
            ("claim", claim(&request.claim)),
            ("scope", model(scope, Vec::new())),
        ],
    )
}

fn rejection_term(rejection: &Rejection) -> Json {
    let simple = |name: &str| model(&format!("Rejection.{name}"), Vec::new());
    let naming = |name: &str, action: ActionKind| {
        model(&format!("Rejection.{name}"), vec![action_kind(action)])
    };
    match rejection {
        Rejection::EmptyDomain => simple("emptyDomain"),
        Rejection::InternalPlanUniverse => simple("internalPlanUniverse"),
        Rejection::OptimizerDefinedUniverse => simple("optimizerDefinedUniverse"),
        Rejection::DiscoveredUniverse => simple("discoveredUniverse"),
        Rejection::CachedUniverse => simple("cachedUniverse"),
        Rejection::MissingCompleteness => simple("missingCompleteness"),
        Rejection::SelfReferentialCompleteness => simple("selfReferentialCompleteness"),
        Rejection::OptimizerCompleteness => simple("optimizerCompleteness"),
        Rejection::BeyondCapacity => simple("beyondCapacity"),
        Rejection::UnitCostOperands => simple("unitCostOperands"),
        Rejection::DuplicateAction { action } => naming("duplicateAction", *action),
        Rejection::UnaccountedAction { action } => naming("unaccountedAction", *action),
        Rejection::HiddenCost { action } => naming("hiddenCost", *action),
        Rejection::UnrealizableAction { action } => naming("unrealizableAction", *action),
        Rejection::UnboundPreparation { action } => naming("unboundPreparation", *action),
        Rejection::StrayPreparedArtifact { action } => naming("strayPreparedArtifact", *action),
        Rejection::ClaimAlias => simple("claimAlias"),
        Rejection::ScalarClaimOverPartialOrder => simple("scalarClaimOverPartialOrder"),
        Rejection::VectorClaimOverTotalOrder => simple("vectorClaimOverTotalOrder"),
        Rejection::UncoveredScope => simple("uncoveredScope"),
        Rejection::UnsupportedClaim => simple("unsupportedClaim"),
    }
}

/// An answer as a `Gnaf.Answer` term.
#[must_use]
pub fn answer_term(answer: &Answer) -> Json {
    let nat_type = json!({"kind": "nat"});
    let ids = |members: &[u64]| {
        term::list(
            &nat_type,
            members.iter().map(|member| term::nat(*member)).collect(),
        )
    };
    match answer {
        Answer::Rejected { rejection } => model("Answer.rejected", vec![rejection_term(rejection)]),
        Answer::Argmin { members, steps } => {
            model("Answer.argmin", vec![ids(members), term::nat(*steps)])
        }
        Answer::Frontier { members } => model("Answer.frontier", vec![ids(members)]),
        Answer::Infeasible => model("Answer.infeasible", Vec::new()),
        Answer::Incomplete => model("Answer.incomplete", Vec::new()),
    }
}

/// The canonical file bytes of a request.
///
/// # Panics
///
/// Panics only if `serde_json` produces text the canonical JSON parser
/// rejects, which would be an internal invariant failure.
#[must_use]
pub fn to_file_bytes<T: Serialize>(value: &T) -> Vec<u8> {
    let text = serde_json::to_string(value).expect("serializes");
    crate::artifact::canonical_json::Json::parse(text.as_bytes())
        .expect("is JSON")
        .to_file_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// S6 of the #45 review: a panic on the evaluation thread is this
    /// host's defect, so it is reported as `LLI9001`, never as the
    /// platform's `LLV7010`.
    #[test]
    fn a_panicking_evaluation_is_an_internal_failure() {
        let request = Request {
            spec: REQUEST_SPEC.to_owned(),
            reference: Program {
                spec: PROGRAM_SPEC.to_owned(),
                adts: Vec::new(),
                functions: Vec::new(),
            },
            domain: Vec::new(),
            machine: Machine {
                fuel: 1,
                capacity: HOST_CAPACITY,
                operand_size: OperandSize::Weighted,
                actions: Vec::new(),
                boundary: Boundary::Complete,
                prepared: Vec::new(),
            },
            carrier: Carrier::Cached,
            completeness: Completeness::Missing,
            universe: Sha256Digest([0; 32]),
            objective: Objective::Scalar,
            claim: ClaimClass::BestKnown,
            scope: Scope::GrammarUniverse,
        };
        let error = on_evaluation_stack(&request, |_| -> u64 { panic!("planted") })
            .expect_err("the evaluation panicked");
        let codes: Vec<&str> = error
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();
        assert_eq!(codes, ["LLI9001"]);
        assert!(error.to_string().contains("planted"), "{error}");
    }
}
