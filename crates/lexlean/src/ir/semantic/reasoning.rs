//! Language-1.2 reasoning machines (SPEC.md §17.12, *Reasoning machines*):
//! logics, inference rules, verifiers, and reasoners; their linking,
//! elaboration, and generated theorems.
//!
//! Like a model declaration, a reasoning declaration carries no meaning of
//! its own. Linking checks it and elaborates it into ordinary language-1.2
//! inductives, structures, definitions, and theorems over the existing
//! primitives, which the ordinary rules then check again; every backend
//! reads only that elaboration, so a reasoner means exactly what its
//! elaboration means and there is no reasoner primitive.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::model::{
    self, anchor, anchor_use, boolean, call, check_axioms, constructor, eq, function_ref, if_then,
    implies, lambda, let_in, list_literal, list_type, local, matching, nat, option_type, pair,
    parameter, parameter_types, primitive, product, qualify_binder, substitution, var, Lowering,
    Obligation,
};
use super::{
    check_member, check_name, check_ordered_key, check_term, check_term_type_parameters,
    check_type, check_type_argument, check_type_parameter_spelling, check_type_parameters,
    function_info, infer_term, instantiate, member_key, qualify_type, substitute_type,
    type_parameter_set, ContractState, Environment, LogicInvariant, MemberRef, ModelBinder,
    ModelUse, ReasoningClaim, ReasoningStrategy, SearchOrder, SemanticConstructor,
    SemanticDeclaration, SemanticFailure, SemanticField, SemanticParameter, SemanticPrimitive,
    SemanticTerm, SemanticType,
};
use crate::code;
use crate::diagnostic::DiagnosticCode;

/// Every term a reasoning declaration carries directly, in declaration
/// order.
fn terms_of(declaration: &SemanticDeclaration) -> Vec<&SemanticTerm> {
    let mut out = Vec::new();
    match declaration {
        SemanticDeclaration::InferenceRule {
            binding,
            guard,
            conclusion,
            ..
        } => {
            if let Some(binding) = binding {
                out.push(&binding.candidates);
            }
            out.push(guard);
            out.push(conclusion);
        }
        SemanticDeclaration::Reasoner {
            observe,
            strategy,
            answer,
            ..
        } => {
            out.extend(observe.iter());
            match strategy {
                ReasoningStrategy::Forward { fuel } => out.extend(fuel.iter()),
                ReasoningStrategy::Search { fuel, frontier, .. } => {
                    out.extend(fuel.iter());
                    out.extend(frontier.iter());
                }
                ReasoningStrategy::GenerateAndVerify { generator, budget } => {
                    out.push(generator);
                    out.extend(budget.iter());
                }
            }
            out.extend(answer.iter().map(|answer| &answer.value));
        }
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::Verifier { .. } => {}
    }
    out
}

/// Visit every term of a reasoning declaration and every subterm.
pub(super) fn declaration_terms(
    declaration: &SemanticDeclaration,
    visit: &mut impl FnMut(&SemanticTerm),
) {
    for term in terms_of(declaration) {
        super::visit_terms(term, visit);
    }
}

/// Visit every term of a reasoning declaration mutably.
pub(super) fn declaration_terms_mut(
    declaration: &mut SemanticDeclaration,
    visit: &mut impl FnMut(&mut SemanticTerm),
) {
    match declaration {
        SemanticDeclaration::InferenceRule {
            binding,
            guard,
            conclusion,
            ..
        } => {
            if let Some(binding) = binding {
                super::visit_terms_mut(&mut binding.candidates, visit);
            }
            super::visit_terms_mut(guard, visit);
            super::visit_terms_mut(conclusion, visit);
        }
        SemanticDeclaration::Reasoner {
            observe,
            strategy,
            answer,
            ..
        } => {
            if let Some(observe) = observe {
                super::visit_terms_mut(observe, visit);
            }
            match strategy {
                ReasoningStrategy::Forward { fuel } => {
                    if let Some(fuel) = fuel {
                        super::visit_terms_mut(fuel, visit);
                    }
                }
                ReasoningStrategy::Search { fuel, frontier, .. } => {
                    if let Some(fuel) = fuel {
                        super::visit_terms_mut(fuel, visit);
                    }
                    if let Some(frontier) = frontier {
                        super::visit_terms_mut(frontier, visit);
                    }
                }
                ReasoningStrategy::GenerateAndVerify { generator, budget } => {
                    super::visit_terms_mut(generator, visit);
                    if let Some(budget) = budget {
                        super::visit_terms_mut(budget, visit);
                    }
                }
            }
            if let Some(answer) = answer {
                super::visit_terms_mut(&mut answer.value, visit);
            }
        }
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::Verifier { .. } => {}
    }
}

fn use_types<'a>(reference: &'a ModelUse, out: &mut Vec<&'a SemanticType>) {
    out.extend(reference.type_arguments.iter());
}

/// Every type a reasoning declaration states directly, in declaration
/// order.
fn types_of(declaration: &SemanticDeclaration) -> Vec<&SemanticType> {
    let mut out = Vec::new();
    match declaration {
        SemanticDeclaration::Logic { state, .. } => out.push(&state.r#type),
        SemanticDeclaration::InferenceRule { logic, binding, .. } => {
            use_types(logic, &mut out);
            if let Some(binding) = binding {
                out.push(&binding.r#type);
            }
        }
        SemanticDeclaration::Verifier {
            subject, candidate, ..
        } => {
            out.push(&subject.r#type);
            out.push(&candidate.r#type);
        }
        SemanticDeclaration::Reasoner {
            logic,
            observation,
            rules,
            answer,
            verifier,
            ..
        } => {
            if let Some(logic) = logic {
                use_types(logic, &mut out);
            }
            out.push(&observation.r#type);
            for rule in rules {
                use_types(rule, &mut out);
            }
            if let Some(answer) = answer {
                out.push(&answer.r#type);
            }
            use_types(verifier, &mut out);
        }
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. } => {}
    }
    out
}

/// Visit every type a reasoning declaration states directly.
pub(super) fn declaration_types(
    declaration: &SemanticDeclaration,
    visit: &mut impl FnMut(&SemanticType),
) {
    for ty in types_of(declaration) {
        visit(ty);
    }
}

/// Visit every interface binder a reasoning declaration names. Term binders
/// are visited through its terms.
pub(super) fn declaration_binders(declaration: &SemanticDeclaration, visit: &mut impl FnMut(&str)) {
    match declaration {
        SemanticDeclaration::Logic { state, .. } => {
            visit(&state.name);
            visit(&state.next);
        }
        SemanticDeclaration::InferenceRule { binding, .. } => {
            if let Some(binding) = binding {
                visit(&binding.name);
            }
        }
        SemanticDeclaration::Verifier {
            subject, candidate, ..
        } => {
            visit(&subject.name);
            visit(&candidate.name);
        }
        SemanticDeclaration::Reasoner {
            observation,
            answer,
            ..
        } => {
            visit(&observation.name);
            if let Some(answer) = answer {
                visit(&answer.name);
            }
        }
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. } => {}
    }
}

fn count(values: usize) -> u64 {
    u64::try_from(values).unwrap_or(u64::MAX)
}

/// The nodes a reasoning declaration's own source charges to
/// `max_ir_nodes`: every type, term, member reference, binder, rule, bound,
/// and claim it states. Its elaboration is charged separately (§17.12).
pub(super) fn source_node_count(declaration: &SemanticDeclaration) -> u64 {
    let types: u64 = types_of(declaration)
        .into_iter()
        .map(super::type_node_count)
        .sum();
    let terms: u64 = terms_of(declaration)
        .into_iter()
        .map(super::term_node_count)
        .sum();
    let shape: u64 = match declaration {
        SemanticDeclaration::Logic {
            type_parameters,
            invariant,
            ranking,
            ..
        } => {
            count(type_parameters.len())
                + 3
                + 2 * u64::from(invariant.is_some())
                + u64::from(ranking.is_some())
        }
        SemanticDeclaration::InferenceRule {
            type_parameters,
            binding,
            progress,
            ..
        } => {
            count(type_parameters.len())
                + 2
                + u64::from(binding.is_some())
                + u64::from(progress.is_some())
        }
        SemanticDeclaration::Verifier {
            type_parameters,
            complete,
            ..
        } => count(type_parameters.len()) + 5 + u64::from(complete.is_some()),
        SemanticDeclaration::Reasoner {
            type_parameters,
            logic,
            rules,
            strategy,
            claims,
            ..
        } => {
            3 + count(type_parameters.len())
                + 2 * u64::from(logic.is_some())
                + count(rules.len())
                + match strategy {
                    ReasoningStrategy::Forward { .. }
                    | ReasoningStrategy::GenerateAndVerify { .. } => 1,
                    ReasoningStrategy::Search { deduplicate, .. } => 2 + u64::from(*deduplicate),
                }
                + claims
                    .iter()
                    .map(|claim| match claim {
                        ReasoningClaim::InitialInvariant { .. }
                        | ReasoningClaim::Terminates { .. }
                        | ReasoningClaim::AnswerCorrect { .. } => 2,
                    })
                    .sum::<u64>()
        }
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. } => 0,
    };
    types.saturating_add(terms).saturating_add(shape)
}

/// The kind of a reasoning declaration, for diagnostics and the closed list
/// of language-1.2 constructs.
#[must_use]
pub fn declaration_construct(declaration: &SemanticDeclaration) -> Option<&'static str> {
    match declaration {
        SemanticDeclaration::Logic { .. } => Some("logic declaration"),
        SemanticDeclaration::InferenceRule { .. } => Some("inference_rule declaration"),
        SemanticDeclaration::Verifier { .. } => Some("verifier declaration"),
        SemanticDeclaration::Reasoner { .. } => Some("reasoner declaration"),
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. } => None,
    }
}

// ---------------------------------------------------------------------------
// Generated theorems.

/// A theorem a reasoning declaration generates (§17.12, *Reasoning machines*
/// rule 9): its statement and the fixed template that proves it. A theorem
/// stated as an ordinary proposition is registered like a prior theorem of
/// the module, so later proofs may `apply` it; one stated with the
/// `LexLeanReasoning` helpers exists only in Lean.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedTheorem {
    /// The generated Lean declaration name, below the module.
    pub name: String,
    /// Its explicit type parameters.
    pub type_parameters: Vec<String>,
    /// Its parameters.
    pub parameters: Vec<SemanticParameter>,
    /// What it states.
    pub statement: Formula,
    /// How it is proved.
    pub proof: Proof,
}

impl GeneratedTheorem {
    /// The ordinary proposition it states, when it states one.
    #[must_use]
    pub fn ordinary_statement(&self) -> Option<&SemanticTerm> {
        match &self.statement {
            Formula::Term { term } => Some(term),
            Formula::Helper { .. } | Formula::Implies { .. } | Formula::True => None,
        }
    }

    /// The name of the template that proves it.
    #[must_use]
    pub fn template(&self) -> &'static str {
        self.proof.template()
    }
}

/// A generated statement: an ordinary proposition, a fixed
/// `LexLeanReasoning` predicate, or an implication between them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Formula {
    /// An ordinary language-1.2 proposition.
    Term { term: SemanticTerm },
    /// `LexLeanReasoning.<helper>` applied to the arguments.
    Helper {
        helper: Helper,
        arguments: Vec<SemanticTerm>,
    },
    /// `premise -> conclusion`.
    Implies {
        premise: Box<Formula>,
        conclusion: Box<Formula>,
    },
    /// The trivially true proposition.
    True,
}

/// The fixed `LexLeanReasoning` propositions a statement may name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Helper {
    /// The reflexive-transitive closure of a relation.
    Star,
    /// Every element of a list satisfies a predicate.
    All,
    /// A relation preserves a predicate.
    Preserves,
    /// A Boolean check implies a specification.
    Sound,
    /// A specification implies a Boolean check.
    Complete,
    /// Every state a replay result holds is reachable.
    Reaches,
    /// Every frontier node replays, and a found answer is accepted.
    SearchOk,
}

impl Helper {
    /// Its Lean name below `LexLeanReasoning`.
    #[must_use]
    pub const fn lean(self) -> &'static str {
        match self {
            Self::Star => "Star",
            Self::All => "All",
            Self::Preserves => "Preserves",
            Self::Sound => "Sound",
            Self::Complete => "Complete",
            Self::Reaches => "Reaches",
            Self::SearchOk => "SearchOk",
        }
    }
}

/// The fixed `LexLeanReasoning` lemmas and constructors a proof term may
/// apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Lemma {
    Guarded,
    GuardedRank,
    StarPreserves,
    ReachesStart,
    FoldInvariant,
    AllAppend,
    AllSingle,
    CapAll,
    CapBound,
    FreshAll,
    SearchStart,
    IterateUntilInvariant,
    IterateUntilSimulate,
    IterateUntilCount,
    IterateUntilStops,
    IterateUntilBound,
    StarRefl,
    StarTail,
    AllNil,
    NoneSome,
    ZeroLe,
}

impl Lemma {
    /// Its Lean name below `LexLeanReasoning`.
    #[must_use]
    pub const fn lean(self) -> &'static str {
        match self {
            Self::Guarded => "guarded",
            Self::GuardedRank => "guardedRank",
            Self::StarPreserves => "starPreserves",
            Self::ReachesStart => "reachesStart",
            Self::FoldInvariant => "foldInvariant",
            Self::AllAppend => "allAppend",
            Self::AllSingle => "allSingle",
            Self::CapAll => "capAll",
            Self::CapBound => "capBound",
            Self::FreshAll => "freshAll",
            Self::SearchStart => "searchStart",
            Self::IterateUntilInvariant => "iterateUntilInvariant",
            Self::IterateUntilSimulate => "iterateUntilSimulate",
            Self::IterateUntilCount => "iterateUntilCount",
            Self::IterateUntilStops => "iterateUntilStops",
            Self::IterateUntilBound => "iterateUntilBound",
            Self::StarRefl => "Star.refl",
            Self::StarTail => "Star.tail",
            Self::AllNil => "All.nil",
            Self::NoneSome => "noneSome",
            Self::ZeroLe => "zeroLe",
        }
    }
}

/// A proof term: an application of fixed lemmas, prior theorems, and terms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ProofTerm {
    /// A fixed `LexLeanReasoning` lemma applied to arguments.
    Lemma {
        lemma: Lemma,
        arguments: Vec<ProofTerm>,
    },
    /// A prior or generated theorem at explicit type arguments, applied.
    Theorem {
        theorem: MemberRef,
        type_arguments: Vec<SemanticType>,
        arguments: Vec<ProofTerm>,
    },
    /// An ordinary term argument.
    Term { term: SemanticTerm },
    /// A predicate argument: `fun (binder : T) => body`.
    Predicate {
        binder: SemanticParameter,
        body: Formula,
    },
    /// A function of hypotheses: `fun h ... => body`.
    Assume {
        binders: Vec<String>,
        body: Box<ProofTerm>,
    },
    /// A hypothesis bound by an enclosing `Assume`.
    Hypothesis { name: String },
    /// A pair of proofs.
    Both {
        left: Box<ProofTerm>,
        right: Box<ProofTerm>,
    },
    /// An argument Lean infers.
    Infer,
    /// Reflexivity.
    Refl,
    /// The proof of `True`.
    Trivial,
}

/// How a generated theorem is proved: a proof term, or one fixed tactic
/// script with names substituted (§17.12, *Reasoning machines* rule 9).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Proof {
    /// A proof term.
    Term { term: ProofTerm },
    /// A proof term checked after unfolding exactly the named generated
    /// definitions, so that Lean compares the term's type with the goal
    /// syntactically instead of evaluating a bounded loop to compare them.
    Unfold {
        definitions: Vec<MemberRef>,
        term: ProofTerm,
    },
    /// `cases` over the step type, each arm one guarded-application
    /// theorem.
    FireCases { arms: Vec<FireArm> },
    /// Replaying a firing step gives its result.
    ReplayFire { replay: MemberRef },
    /// Replay keeps every held state reachable.
    ReplaySound {
        replay: MemberRef,
        fire_sound: MemberRef,
    },
    /// A forward step is sound for the relation.
    NextSound {
        next: MemberRef,
        fire_sound: MemberRef,
    },
    /// A forward step decreases the ranking.
    NextProgress {
        next: MemberRef,
        fire_progress: MemberRef,
        invariant: bool,
    },
    /// The traced loop stops exactly where the forward loop does.
    StepNone { step: MemberRef, next: MemberRef },
    /// The traced loop moves exactly where the forward loop does.
    StepSome { step: MemberRef, next: MemberRef },
    /// The traced loop keeps its trace replaying to its state.
    StepTrace {
        step: MemberRef,
        follow: MemberRef,
        replay_fire: MemberRef,
    },
    /// A loop step counts one iteration.
    StepCount { step: MemberRef },
    /// An accepted answer satisfies the specification.
    AcceptSound {
        accept: MemberRef,
        sound: MemberRef,
        type_arguments: Vec<SemanticType>,
    },
    /// A concluded answer satisfies the specification.
    ConcludeSound {
        conclude: MemberRef,
        accept_sound: MemberRef,
    },
    /// A concluded answer is the accepted answer.
    ConcludeAccept { conclude: MemberRef },
    /// A forward reasoner's explained answer replays and is sound.
    ExplainedForward {
        reasoner: MemberRef,
        run: MemberRef,
        answer: MemberRef,
        run_trace: MemberRef,
        conclude_accept: MemberRef,
        conclude_sound: MemberRef,
    },
    /// A node's trace extended by a firing step replays to its result.
    Extend {
        follow: MemberRef,
        replay_fire: MemberRef,
    },
    /// The successors of a binding-free rule replay.
    SuccessorsFree {
        successors: MemberRef,
        extend: MemberRef,
    },
    /// The successors of a binding rule replay.
    SuccessorsBound {
        successors: MemberRef,
        collect: MemberRef,
        extend: MemberRef,
        follow: MemberRef,
        node: SemanticType,
    },
    /// A search step keeps the search invariant.
    SearchStep {
        search_step: MemberRef,
        successors_ok: MemberRef,
        fresh_ok: Option<MemberRef>,
        order: SearchOrder,
    },
    /// A search step keeps the frontier peak within its bound.
    SearchPeak { search_step: MemberRef },
    /// A search step counts one iteration.
    SearchCount { search_step: MemberRef },
    /// A search reasoner's explained answer replays and is sound.
    ExplainedSearch {
        reasoner: MemberRef,
        run: MemberRef,
        answer: MemberRef,
        search_ok: MemberRef,
        accept_sound: MemberRef,
    },
    /// A forward reasoner's verdict is sound.
    VerdictForward {
        verdict: MemberRef,
        saturate: MemberRef,
        conclude_sound: MemberRef,
    },
    /// A search reasoner's verdict is sound.
    VerdictSearch {
        reasoner: MemberRef,
        verdict: MemberRef,
        explained: MemberRef,
    },
    /// A verified candidate satisfies the specification and verifies again.
    VerifySound {
        verify: MemberRef,
        sound: MemberRef,
        type_arguments: Vec<SemanticType>,
    },
    /// A generate-and-verify step keeps every found candidate verified.
    TrySound {
        attempt: MemberRef,
        verify_sound: MemberRef,
    },
    /// A generate-and-verify step checks within its budget.
    TryCount { attempt: MemberRef },
    /// A generate-and-verify reasoner's verdict is sound.
    VerdictGenerate {
        verdict: MemberRef,
        run: MemberRef,
        run_sound: MemberRef,
    },
    /// A generate-and-verify reasoner's explained answer replays and is
    /// sound.
    ExplainedGenerate {
        reasoner: MemberRef,
        run: MemberRef,
        run_sound: MemberRef,
        answer: MemberRef,
    },
}

/// One arm of a `cases` over a reasoner's step type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FireArm {
    /// The step constructor's own name.
    pub constructor: String,
    /// Whether it carries the rule's binding.
    pub binding: bool,
    /// The rule's generated theorem.
    pub theorem: MemberRef,
    /// The rule's type arguments.
    pub type_arguments: Vec<SemanticType>,
}

impl Proof {
    /// The template's name.
    #[must_use]
    pub const fn template(&self) -> &'static str {
        match self {
            Self::Term { .. } => "term",
            Self::Unfold { .. } => "unfold",
            Self::FireCases { .. } => "fire_cases",
            Self::ReplayFire { .. } => "replay_fire",
            Self::ReplaySound { .. } => "replay_sound",
            Self::NextSound { .. } => "next_sound",
            Self::NextProgress { .. } => "next_progress",
            Self::StepNone { .. } => "step_none",
            Self::StepSome { .. } => "step_some",
            Self::StepTrace { .. } => "step_trace",
            Self::StepCount { .. } => "step_count",
            Self::AcceptSound { .. } => "accept_sound",
            Self::ConcludeSound { .. } => "conclude_sound",
            Self::ConcludeAccept { .. } => "conclude_accept",
            Self::ExplainedForward { .. } => "explained_forward",
            Self::Extend { .. } => "extend",
            Self::SuccessorsFree { .. } => "successors_free",
            Self::SuccessorsBound { .. } => "successors_bound",
            Self::SearchStep { .. } => "search_step",
            Self::SearchPeak { .. } => "search_peak",
            Self::SearchCount { .. } => "search_count",
            Self::ExplainedSearch { .. } => "explained_search",
            Self::VerdictForward { .. } => "verdict_forward",
            Self::VerdictSearch { .. } => "verdict_search",
            Self::VerifySound { .. } => "verify_sound",
            Self::TrySound { .. } => "try_sound",
            Self::TryCount { .. } => "try_count",
            Self::VerdictGenerate { .. } => "verdict_generate",
            Self::ExplainedGenerate { .. } => "explained_generate",
        }
    }
}

/// The closed reasoning-failure constructors, in declaration order. Each is
/// plain data, a pair of Booleans in Lean, so values cross modules
/// unchanged.
pub const FAILURES: [&str; 4] = [
    "ReasoningFailure.exhausted",
    "ReasoningFailure.unsolved",
    "ReasoningFailure.rejected",
    "ReasoningFailure.invalid_step",
];

// ---------------------------------------------------------------------------
// The reasoning environment.

/// What linking knows about one logic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LogicInfo {
    type_parameters: Vec<String>,
    state: ContractState,
    relation: MemberRef,
    invariant: Option<LogicInvariant>,
    ranking: Option<MemberRef>,
}

/// What linking knows about one inference rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuleInfo {
    type_parameters: Vec<String>,
    logic: ModelUse,
    binding: Option<(String, SemanticType)>,
    progress: bool,
    executable: bool,
}

/// What linking knows about one verifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifierInfo {
    type_parameters: Vec<String>,
    subject: ModelBinder,
    candidate: ModelBinder,
    specification: MemberRef,
    check: MemberRef,
    sound: MemberRef,
}

/// Every visible reasoning interface, by environment key (a local name, or
/// `module::name` for an import), and the generated functions executable
/// code may not reach directly.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Reasoning {
    logics: BTreeMap<String, LogicInfo>,
    rules: BTreeMap<String, RuleInfo>,
    verifiers: BTreeMap<String, VerifierInfo>,
    /// A rule's unguarded conclusion and a reasoner's unverified answer, by
    /// key, with what each is.
    guarded: BTreeMap<String, &'static str>,
}

impl Reasoning {
    /// The interfaces this module declares itself.
    pub(super) fn locals(&self) -> Self {
        fn local<T: Clone>(map: &BTreeMap<String, T>) -> BTreeMap<String, T> {
            map.iter()
                .filter(|(key, _)| !key.contains("::"))
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect()
        }
        Self {
            logics: local(&self.logics),
            rules: local(&self.rules),
            verifiers: local(&self.verifiers),
            guarded: local(&self.guarded),
        }
    }

    /// Why executable code may not reach the generated function `key`
    /// directly, if it may not.
    pub(super) fn guarded(&self, key: &str) -> Option<&'static str> {
        self.guarded.get(key).copied()
    }
}

/// Register an imported module's reasoning interfaces under `module::name`,
/// and every ordinary theorem its reasoning declarations generate, so proofs
/// may apply them.
pub(super) fn register_import(
    module: &str,
    elaboration: &model::Elaboration,
    env: &mut Environment<'_>,
) {
    let key = |name: &str| format!("{module}::{name}");
    let exported = &elaboration.reasoning;
    for (name, info) in &exported.logics {
        env.reasoning.logics.insert(
            key(name),
            LogicInfo {
                type_parameters: info.type_parameters.clone(),
                state: ContractState {
                    name: info.state.name.clone(),
                    next: info.state.next.clone(),
                    r#type: qualify_type(&info.state.r#type, module),
                },
                relation: anchor(&info.relation, module),
                invariant: info.invariant.as_ref().map(|invariant| LogicInvariant {
                    predicate: anchor(&invariant.predicate, module),
                    preserves: anchor(&invariant.preserves, module),
                }),
                ranking: info.ranking.as_ref().map(|member| anchor(member, module)),
            },
        );
    }
    for (name, info) in &exported.rules {
        env.reasoning.rules.insert(
            key(name),
            RuleInfo {
                type_parameters: info.type_parameters.clone(),
                logic: anchor_use(&info.logic, module),
                binding: info
                    .binding
                    .as_ref()
                    .map(|(binder, ty)| (binder.clone(), qualify_type(ty, module))),
                progress: info.progress,
                executable: info.executable,
            },
        );
    }
    for (name, info) in &exported.verifiers {
        env.reasoning.verifiers.insert(
            key(name),
            VerifierInfo {
                type_parameters: info.type_parameters.clone(),
                subject: qualify_binder(&info.subject, module),
                candidate: qualify_binder(&info.candidate, module),
                specification: anchor(&info.specification, module),
                check: anchor(&info.check, module),
                sound: anchor(&info.sound, module),
            },
        );
    }
    for (name, what) in &exported.guarded {
        env.reasoning.guarded.insert(key(name), what);
    }
    for theorem in elaboration.theorems.iter().flatten() {
        if theorem.ordinary_statement().is_some() {
            let name = key(&theorem.name);
            env.proof_type_parameters
                .insert(name.clone(), theorem.type_parameters.clone());
            env.proof_rules.insert(
                name,
                theorem
                    .parameters
                    .iter()
                    .map(|parameter| qualify_type(&parameter.r#type, module))
                    .collect(),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Diagnostics and small builders.

fn fail(code: DiagnosticCode, reason: String) -> SemanticFailure {
    model::fail(code, reason)
}

/// A reasoning interface or obligation mismatch (`LLT4010`).
fn mismatch(reason: String) -> SemanticFailure {
    fail(code!("LLT4010"), reason)
}

/// An unbounded strategy or missing termination evidence (`LLT4011`).
fn unbounded(reason: String) -> SemanticFailure {
    fail(code!("LLT4011"), reason)
}

fn project(value: SemanticTerm, field: &str) -> SemanticTerm {
    SemanticTerm::Project {
        value: Box::new(value),
        field: field.to_owned(),
    }
}

fn record(name: &str, fields: Vec<(&str, SemanticTerm)>) -> SemanticTerm {
    SemanticTerm::Record {
        r#type: local(name),
        type_arguments: Vec::new(),
        fields: fields
            .into_iter()
            .map(|(field, value)| super::SemanticAssignment {
                field: field.to_owned(),
                value,
            })
            .collect(),
    }
}

fn named(name: &str) -> SemanticType {
    SemanticType::Named {
        member: local(name),
        arguments: Vec::new(),
    }
}

fn result_of(ok: SemanticType) -> SemanticType {
    SemanticType::Result {
        ok: Box::new(ok),
        error: Box::new(SemanticType::ReasoningFailure),
    }
}

fn ok_of(ok: &SemanticType, value: SemanticTerm) -> SemanticTerm {
    constructor(
        "Result.ok",
        vec![ok.clone(), SemanticType::ReasoningFailure],
        vec![value],
    )
}

fn error_of(ok: &SemanticType, value: SemanticTerm) -> SemanticTerm {
    constructor(
        "Result.error",
        vec![ok.clone(), SemanticType::ReasoningFailure],
        vec![value],
    )
}

fn failure_value(name: &str) -> SemanticTerm {
    constructor(&format!("ReasoningFailure.{name}"), Vec::new(), Vec::new())
}

fn some_of(ty: &SemanticType, value: SemanticTerm) -> SemanticTerm {
    constructor("Option.some", vec![ty.clone()], vec![value])
}

fn none_of(ty: &SemanticType) -> SemanticTerm {
    constructor("Option.none", vec![ty.clone()], Vec::new())
}

fn add(left: SemanticTerm, right: SemanticTerm) -> SemanticTerm {
    SemanticTerm::Add {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn blt(left: SemanticTerm, right: SemanticTerm) -> SemanticTerm {
    SemanticTerm::Blt {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn le(left: SemanticTerm, right: SemanticTerm) -> SemanticTerm {
    SemanticTerm::Le {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn lt(left: SemanticTerm, right: SemanticTerm) -> SemanticTerm {
    SemanticTerm::Lt {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn both(left: SemanticTerm, right: SemanticTerm) -> SemanticTerm {
    SemanticTerm::PropAnd {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn and_bool(left: SemanticTerm, right: SemanticTerm) -> SemanticTerm {
    SemanticTerm::And {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn not_bool(value: SemanticTerm) -> SemanticTerm {
    SemanticTerm::Not {
        value: Box::new(value),
    }
}

fn length(list: SemanticTerm) -> SemanticTerm {
    primitive(SemanticPrimitive::Length, vec![list], SemanticType::Nat)
}

fn append(left: SemanticTerm, right: SemanticTerm, element: &SemanticType) -> SemanticTerm {
    primitive(
        SemanticPrimitive::Append,
        vec![left, right],
        list_type(element.clone()),
    )
}

fn single(element: &SemanticType, value: SemanticTerm) -> SemanticTerm {
    list_literal(element, vec![value])
}

fn nil(element: &SemanticType) -> SemanticTerm {
    SemanticTerm::Nil {
        element: element.clone(),
    }
}

fn list_fold(
    step: SemanticTerm,
    initial: SemanticTerm,
    values: SemanticTerm,
    state: SemanticType,
) -> SemanticTerm {
    primitive(
        SemanticPrimitive::ListFold,
        vec![step, initial, values],
        state,
    )
}

fn iterate_until(
    step: SemanticTerm,
    fuel: SemanticTerm,
    initial: SemanticTerm,
    state: &SemanticType,
) -> SemanticTerm {
    primitive(
        SemanticPrimitive::IterateUntil,
        vec![step, fuel, initial],
        product(state.clone(), SemanticType::Bool),
    )
}

fn definition(
    name: &str,
    parameters: Vec<SemanticParameter>,
    result: SemanticType,
    body: SemanticTerm,
    executable: bool,
    axioms: &[String],
) -> SemanticDeclaration {
    definition_at(name, &[], parameters, result, body, executable, axioms)
}

fn definition_at(
    name: &str,
    type_parameters: &[String],
    parameters: Vec<SemanticParameter>,
    result: SemanticType,
    body: SemanticTerm,
    executable: bool,
    axioms: &[String],
) -> SemanticDeclaration {
    SemanticDeclaration::Definition {
        name: name.to_owned(),
        type_parameters: type_parameters.to_vec(),
        parameters,
        result,
        recursive_argument: None,
        body,
        axioms: axioms.to_vec(),
        executable,
        mutual: None,
        termination: None,
        production: None,
    }
}

fn theorem(
    name: &str,
    type_parameters: &[String],
    parameters: Vec<SemanticParameter>,
    statement: Formula,
    proof: Proof,
) -> GeneratedTheorem {
    GeneratedTheorem {
        name: name.to_owned(),
        type_parameters: type_parameters.to_vec(),
        parameters,
        statement,
        proof,
    }
}

fn forall(name: &str, ty: &SemanticType, body: SemanticTerm) -> SemanticTerm {
    SemanticTerm::Forall {
        binder: parameter(name, ty),
        body: Box::new(body),
    }
}

fn term(statement: SemanticTerm) -> Formula {
    Formula::Term { term: statement }
}

fn helper(helper: Helper, arguments: Vec<SemanticTerm>) -> Formula {
    Formula::Helper { helper, arguments }
}

fn entails(premise: Formula, conclusion: Formula) -> Formula {
    Formula::Implies {
        premise: Box::new(premise),
        conclusion: Box::new(conclusion),
    }
}

fn by_term(proof: ProofTerm) -> Proof {
    Proof::Term { term: proof }
}

/// A proof term checked after unfolding the given generated definitions.
fn unfolding(definitions: Vec<MemberRef>, proof: ProofTerm) -> Proof {
    Proof::Unfold {
        definitions,
        term: proof,
    }
}

fn lemma(lemma: Lemma, arguments: Vec<ProofTerm>) -> ProofTerm {
    ProofTerm::Lemma { lemma, arguments }
}

fn cite(
    theorem: &MemberRef,
    type_arguments: &[SemanticType],
    arguments: Vec<ProofTerm>,
) -> ProofTerm {
    ProofTerm::Theorem {
        theorem: theorem.clone(),
        type_arguments: type_arguments.to_vec(),
        arguments,
    }
}

fn given(value: SemanticTerm) -> ProofTerm {
    ProofTerm::Term { term: value }
}

fn hypothesis(name: &str) -> ProofTerm {
    ProofTerm::Hypothesis {
        name: name.to_owned(),
    }
}

fn assume(binders: &[&str], body: ProofTerm) -> ProofTerm {
    ProofTerm::Assume {
        binders: binders.iter().map(|binder| (*binder).to_owned()).collect(),
        body: Box::new(body),
    }
}

fn predicate(name: &str, ty: &SemanticType, body: Formula) -> ProofTerm {
    ProofTerm::Predicate {
        binder: parameter(name, ty),
        body,
    }
}

// ---------------------------------------------------------------------------
// Shared checks.

/// A generated member of the declaration `owner` names: `owner.suffix` in
/// `owner`'s module.
fn generated(owner: &MemberRef, suffix: &str) -> MemberRef {
    MemberRef {
        module: owner.module.clone(),
        name: format!("{}.{suffix}", owner.name),
    }
}

/// The signature of prior definition `member` instantiated at
/// `type_arguments`, which must be exactly as many as its type parameters.
fn signature(
    member: &MemberRef,
    type_arguments: &[SemanticType],
    env: &Environment<'_>,
    what: &str,
) -> Result<(Vec<SemanticType>, SemanticType, bool), SemanticFailure> {
    check_member(member, env)?;
    let Some(info) = function_info(member, env) else {
        return Err(mismatch(format!(
            "{what} `{}` is not a prior definition",
            member_key(member)
        )));
    };
    if info.type_parameters.len() != type_arguments.len() {
        return Err(mismatch(format!(
            "{what} `{}` has {} type parameter(s); it must have exactly {}",
            member_key(member),
            info.type_parameters.len(),
            type_arguments.len()
        )));
    }
    let (parameters, result) = instantiate(info, type_arguments);
    Ok((parameters, result, info.executable))
}

/// `member` is a prior definition taking exactly `parameters` to `result`.
fn require_signature(
    member: &MemberRef,
    type_arguments: &[SemanticType],
    parameters: &[SemanticType],
    result: &SemanticType,
    env: &Environment<'_>,
    what: &str,
) -> Result<bool, SemanticFailure> {
    let (observed, observed_result, executable) = signature(member, type_arguments, env, what)?;
    if observed != parameters || &observed_result != result {
        return Err(mismatch(format!(
            "{what} `{}` must take ({}) to {result}, but takes ({}) to {observed_result}",
            member_key(member),
            parameters
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", "),
            observed
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )));
    }
    Ok(executable)
}

/// `term`, over the typed `locals`, is well formed and has type `expected`.
fn require_term(
    term: &SemanticTerm,
    locals: &[(&str, &SemanticType)],
    scope: &BTreeSet<String>,
    expected: &SemanticType,
    env: &Environment<'_>,
    what: &str,
    failure: fn(String) -> SemanticFailure,
) -> Result<(), SemanticFailure> {
    check_term_type_parameters(term, scope)?;
    let names: BTreeSet<String> = locals.iter().map(|(name, _)| (*name).to_owned()).collect();
    check_term(term, &names, env, None, &BTreeSet::new())?;
    let typed: BTreeMap<String, SemanticType> = locals
        .iter()
        .map(|(name, ty)| ((*name).to_owned(), (*ty).clone()))
        .collect();
    match infer_term(term, &typed, env)? {
        Some(observed) if &observed != expected => Err(failure(format!(
            "{what} has type {observed}, expected {expected}"
        ))),
        _ => Ok(()),
    }
}

/// Check the type parameters, interface binders, and written types of a
/// reasoning declaration.
fn check_signature(
    name: &str,
    type_parameters: &[String],
    binders: &[&str],
    types: &[&SemanticType],
    env: &Environment<'_>,
) -> Result<BTreeSet<String>, SemanticFailure> {
    let scope = type_parameter_set(type_parameters)?;
    let mut seen = BTreeSet::new();
    for binder in binders {
        check_name(binder, "interface binder")?;
        if !seen.insert((*binder).to_owned()) {
            return Err(mismatch(format!(
                "`{name}` binds the interface name `{binder}` twice"
            )));
        }
    }
    check_type_parameter_spelling(type_parameters, &seen, env)?;
    for ty in types {
        check_type(ty, env)?;
        check_type_parameters(ty, &scope)?;
    }
    Ok(scope)
}

/// Check a use's type arguments against the used declaration's type
/// parameters, returning the substitution.
fn use_at(
    reference: &ModelUse,
    type_parameters: &[String],
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
    what: &str,
) -> Result<BTreeMap<String, SemanticType>, SemanticFailure> {
    check_member(&reference.member, env)?;
    if reference.type_arguments.len() != type_parameters.len() {
        return Err(mismatch(format!(
            "{what} `{}` expects {} type argument(s), received {}",
            member_key(&reference.member),
            type_parameters.len(),
            reference.type_arguments.len()
        )));
    }
    for argument in &reference.type_arguments {
        check_type_argument(argument, env)?;
        check_type_parameters(argument, scope)?;
    }
    Ok(substitution(type_parameters, &reference.type_arguments))
}

/// A logic at one instantiation.
#[derive(Debug, Clone)]
struct LogicAt {
    member: MemberRef,
    type_arguments: Vec<SemanticType>,
    state: SemanticType,
    state_binder: String,
    relation: MemberRef,
    invariant: Option<LogicInvariant>,
    ranking: Option<MemberRef>,
}

impl LogicAt {
    fn relation(&self, from: SemanticTerm, to: SemanticTerm) -> SemanticTerm {
        call(&self.relation, &self.type_arguments, vec![from, to])
    }

    fn relation_ref(&self) -> SemanticTerm {
        function_ref(&self.relation, &self.type_arguments)
    }

    fn invariant(&self, state: SemanticTerm) -> Option<SemanticTerm> {
        self.invariant
            .as_ref()
            .map(|invariant| call(&invariant.predicate, &self.type_arguments, vec![state]))
    }

    fn invariant_ref(&self) -> Option<SemanticTerm> {
        self.invariant
            .as_ref()
            .map(|invariant| function_ref(&invariant.predicate, &self.type_arguments))
    }

    fn rank(&self, state: SemanticTerm) -> Option<SemanticTerm> {
        self.ranking
            .as_ref()
            .map(|ranking| call(ranking, &self.type_arguments, vec![state]))
    }

    fn rank_ref(&self) -> Option<SemanticTerm> {
        self.ranking
            .as_ref()
            .map(|ranking| function_ref(ranking, &self.type_arguments))
    }

    /// The logic's generated `L.preserves` theorem.
    fn preserves(&self) -> MemberRef {
        generated(&self.member, "preserves")
    }
}

fn resolve_logic(
    reference: &ModelUse,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
    what: &str,
) -> Result<LogicAt, SemanticFailure> {
    let key = member_key(&reference.member);
    let Some(info) = env.reasoning.logics.get(&key) else {
        return Err(mismatch(format!("{what} `{key}` is not a prior logic")));
    };
    let map = use_at(reference, &info.type_parameters, scope, env, what)?;
    Ok(LogicAt {
        member: reference.member.clone(),
        type_arguments: reference.type_arguments.clone(),
        state: substitute_type(&info.state.r#type, &map),
        state_binder: info.state.name.clone(),
        relation: info.relation.clone(),
        invariant: info.invariant.clone(),
        ranking: info.ranking.clone(),
    })
}

/// An inference rule at one instantiation.
#[derive(Debug, Clone)]
struct RuleAt {
    member: MemberRef,
    type_arguments: Vec<SemanticType>,
    logic: ModelUse,
    binding: Option<(String, SemanticType)>,
    progress: bool,
    executable: bool,
}

impl RuleAt {
    fn companion(&self, suffix: &str) -> MemberRef {
        generated(&self.member, suffix)
    }
}

fn resolve_rule(
    reference: &ModelUse,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
) -> Result<RuleAt, SemanticFailure> {
    let key = member_key(&reference.member);
    let Some(info) = env.reasoning.rules.get(&key) else {
        return Err(mismatch(format!("`{key}` is not a prior inference rule")));
    };
    let map = use_at(
        reference,
        &info.type_parameters,
        scope,
        env,
        "inference rule",
    )?;
    Ok(RuleAt {
        member: reference.member.clone(),
        type_arguments: reference.type_arguments.clone(),
        logic: ModelUse {
            member: info.logic.member.clone(),
            type_arguments: info
                .logic
                .type_arguments
                .iter()
                .map(|argument| substitute_type(argument, &map))
                .collect(),
        },
        binding: info
            .binding
            .as_ref()
            .map(|(name, ty)| (name.clone(), substitute_type(ty, &map))),
        progress: info.progress,
        executable: info.executable,
    })
}

/// A verifier at one instantiation.
#[derive(Debug, Clone)]
struct VerifierAt {
    type_arguments: Vec<SemanticType>,
    subject: SemanticType,
    candidate: SemanticType,
    specification: MemberRef,
    check: MemberRef,
    sound: MemberRef,
}

fn resolve_verifier(
    reference: &ModelUse,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
) -> Result<VerifierAt, SemanticFailure> {
    let key = member_key(&reference.member);
    let Some(info) = env.reasoning.verifiers.get(&key) else {
        return Err(mismatch(format!("`{key}` is not a prior verifier")));
    };
    let map = use_at(reference, &info.type_parameters, scope, env, "verifier")?;
    Ok(VerifierAt {
        type_arguments: reference.type_arguments.clone(),
        subject: substitute_type(&info.subject.r#type, &map),
        candidate: substitute_type(&info.candidate.r#type, &map),
        specification: info.specification.clone(),
        check: info.check.clone(),
        sound: info.sound.clone(),
    })
}

/// The obligation `theorem` must state, compared up to the names of bound
/// variables (`LLT4010` otherwise).
fn require_obligation(
    env: &Environment<'_>,
    obligation: &Obligation,
    code: DiagnosticCode,
) -> Result<(), SemanticFailure> {
    model::require_statement(env, &obligation.theorem, obligation, code)
}

/// Every binder a reasoning declaration's source writes is a source name:
/// its elaborated copy is checked admitting generated binders, so the
/// source is checked first, as for an ordinary declaration (§17.12).
fn check_source_binders(declaration: &SemanticDeclaration) -> Result<(), SemanticFailure> {
    let mut failure = None;
    super::declaration_binders(declaration, &mut |binder| {
        if failure.is_none() {
            if let Err(reason) = check_name(binder, "binder") {
                failure = Some(reason);
            }
        }
    });
    failure.map_or(Ok(()), |reason| Err(reason.into()))
}

/// Check every source term of a reasoning declaration at the runtime
/// boundary and elaborate its checked model applications.
fn lower_source(
    owner: &str,
    terms: Vec<&mut SemanticTerm>,
    executable: bool,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
) -> Result<(), SemanticFailure> {
    let mut counter = 0;
    for term in terms {
        model::check_boundary(owner, term, executable, scope, env)?;
        let mut failure = None;
        super::visit_terms_mut(term, &mut |node| {
            if failure.is_none() && matches!(node, SemanticTerm::CheckedApply { .. }) {
                match model::expand(node, &mut counter, scope, env) {
                    Ok(expanded) => *node = expanded,
                    Err(reason) => failure = Some(reason),
                }
            }
        });
        if let Some(reason) = failure {
            return Err(reason);
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Logics.

fn check_logic(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::Logic {
        name,
        type_parameters,
        state,
        relation,
        invariant,
        ranking,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_axioms(name, axioms)?;
    check_signature(
        name,
        type_parameters,
        &[&state.name, &state.next],
        &[&state.r#type],
        env,
    )?;
    let arguments = parameter_types(type_parameters);
    let s = &state.r#type;
    require_signature(
        relation,
        &arguments,
        &[s.clone(), s.clone()],
        &SemanticType::Prop,
        env,
        &format!("logic `{name}` relation"),
    )?;
    if let Some(ranking) = ranking {
        require_signature(
            ranking,
            &arguments,
            std::slice::from_ref(s),
            &SemanticType::Nat,
            env,
            &format!("logic `{name}` ranking"),
        )?;
    }
    let mut lowering = Lowering::default();
    if let Some(invariant) = invariant {
        require_signature(
            &invariant.predicate,
            &arguments,
            std::slice::from_ref(s),
            &SemanticType::Prop,
            env,
            &format!("logic `{name}` invariant"),
        )?;
        let before = var(&state.name);
        let after = var(&state.next);
        let holds = |value: SemanticTerm| call(&invariant.predicate, &arguments, vec![value]);
        let obligation = Obligation {
            role: format!("logic `{name}` preserves its invariant"),
            theorem: invariant.preserves.clone(),
            type_parameters: type_parameters.clone(),
            parameters: vec![parameter(&state.name, s), parameter(&state.next, s)],
            statement: implies(
                holds(before.clone()),
                implies(
                    call(relation, &arguments, vec![before, after.clone()]),
                    holds(after),
                ),
            ),
        };
        require_obligation(env, &obligation, code!("LLT4010"))?;
        lowering.theorems.push(theorem(
            &format!("{name}.preserves"),
            type_parameters,
            Vec::new(),
            helper(
                Helper::Preserves,
                vec![
                    function_ref(relation, &arguments),
                    function_ref(&invariant.predicate, &arguments),
                ],
            ),
            by_term(cite(&invariant.preserves, &arguments, Vec::new())),
        ));
        lowering.obligations.push(obligation);
    }
    env.reasoning.logics.insert(
        name.clone(),
        LogicInfo {
            type_parameters: type_parameters.clone(),
            state: state.clone(),
            relation: relation.clone(),
            invariant: invariant.clone(),
            ranking: ranking.clone(),
        },
    );
    Ok(lowering)
}

// ---------------------------------------------------------------------------
// Inference rules.

#[allow(clippy::too_many_lines)]
fn check_rule(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::InferenceRule {
        name,
        type_parameters,
        logic,
        binding,
        guard,
        conclusion,
        soundness,
        progress,
        executable,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_axioms(name, axioms)?;
    let scope = type_parameter_set(type_parameters)?;
    let at = resolve_logic(
        logic,
        &scope,
        env,
        &format!("inference rule `{name}` logic"),
    )?;
    let s = at.state.clone();
    let mut binders = vec![at.state_binder.as_str()];
    let mut types = vec![&s];
    if let Some(binding) = binding {
        binders.push(&binding.name);
        types.push(&binding.r#type);
    }
    check_signature(name, type_parameters, &binders, &types, env)?;
    let state = var(&at.state_binder);
    check_source_binders(declaration)?;
    // The rule's source terms are executable exactly when the rule is, and
    // never reach another rule's conclusion or a reasoner's raw answer;
    // they are typed as linking elaborated them.
    let mut sources: Vec<SemanticTerm> = binding
        .iter()
        .map(|binding| binding.candidates.clone())
        .chain([guard.clone(), conclusion.clone()])
        .collect();
    lower_source(name, sources.iter_mut().collect(), *executable, &scope, env)?;
    let (candidates_term, guard_term, conclusion_term) = match binding {
        Some(_) => (
            Some(sources[0].clone()),
            sources[1].clone(),
            sources[2].clone(),
        ),
        None => (None, sources[0].clone(), sources[1].clone()),
    };
    let mut locals: Vec<(&str, &SemanticType)> = vec![(at.state_binder.as_str(), &s)];
    env.derived = true;
    let typed = (|| {
        if let (Some(binding), Some(candidates)) = (binding, &candidates_term) {
            require_term(
                candidates,
                &locals,
                &scope,
                &list_type(binding.r#type.clone()),
                env,
                &format!("inference rule `{name}` candidates"),
                mismatch,
            )?;
            locals.push((binding.name.as_str(), &binding.r#type));
        }
        require_term(
            &guard_term,
            &locals,
            &scope,
            &SemanticType::Bool,
            env,
            &format!("inference rule `{name}` guard"),
            mismatch,
        )?;
        require_term(
            &conclusion_term,
            &locals,
            &scope,
            &s,
            env,
            &format!("inference rule `{name}` conclusion"),
            mismatch,
        )
    })();
    env.derived = false;
    typed?;
    let own = parameter_types(type_parameters);
    let local_member = |suffix: &str| local(&format!("{name}.{suffix}"));
    let mut parameters = vec![parameter(&at.state_binder, &s)];
    let mut arguments = vec![state.clone()];
    if let Some(binding) = binding {
        parameters.push(parameter(&binding.name, &binding.r#type));
        arguments.push(var(&binding.name));
    }
    let mut lowering = Lowering::default();
    // Soundness: the guard implies the logic's relation to the conclusion,
    // both as linking elaborated them (a theorem's statement is compared
    // after its own checked applications are elaborated).
    let accepted = eq(guard_term.clone(), boolean(true));
    let soundness_obligation = Obligation {
        role: format!("inference rule `{name}` soundness"),
        theorem: soundness.clone(),
        type_parameters: type_parameters.clone(),
        parameters: parameters.clone(),
        statement: implies(
            accepted.clone(),
            at.relation(state.clone(), conclusion_term.clone()),
        ),
    };
    require_obligation(env, &soundness_obligation, code!("LLT4010"))?;
    lowering.obligations.push(soundness_obligation);
    if let Some(progress) = progress {
        let (Some(before), Some(after)) =
            (at.rank(state.clone()), at.rank(conclusion_term.clone()))
        else {
            return Err(mismatch(format!(
                "inference rule `{name}` states progress, but logic `{}` has no ranking",
                member_key(&at.member)
            )));
        };
        let obligation = Obligation {
            role: format!("inference rule `{name}` progress"),
            theorem: progress.clone(),
            type_parameters: type_parameters.clone(),
            parameters: parameters.clone(),
            statement: model::premised(
                at.invariant(state.clone()),
                implies(accepted, lt(after, before)),
            ),
        };
        require_obligation(env, &obligation, code!("LLT4010"))?;
        lowering.obligations.push(obligation);
    }
    // Companions: the guard, the conclusion, the candidates, and the
    // guarded application, which is the only way a step applies the rule.
    lowering.declarations.push(definition_at(
        &format!("{name}.guard"),
        type_parameters,
        parameters.clone(),
        SemanticType::Bool,
        guard_term,
        *executable,
        axioms,
    ));
    lowering.declarations.push(definition_at(
        &format!("{name}.conclusion"),
        type_parameters,
        parameters.clone(),
        s.clone(),
        conclusion_term,
        *executable,
        axioms,
    ));
    if let (Some(binding), Some(candidates)) = (binding, candidates_term) {
        lowering.declarations.push(definition_at(
            &format!("{name}.candidates"),
            type_parameters,
            vec![parameter(&at.state_binder, &s)],
            list_type(binding.r#type.clone()),
            candidates,
            *executable,
            axioms,
        ));
    }
    let guard_call = call(&local_member("guard"), &own, arguments.clone());
    let conclusion_call = call(&local_member("conclusion"), &own, arguments.clone());
    lowering.declarations.push(definition_at(
        &format!("{name}.apply"),
        type_parameters,
        parameters.clone(),
        option_type(s.clone()),
        if_then(
            guard_call.clone(),
            some_of(&s, conclusion_call.clone()),
            none_of(&s),
        ),
        *executable,
        axioms,
    ));
    let target = "__t";
    let applied = eq(
        call(&local_member("apply"), &own, arguments.clone()),
        some_of(&s, var(target)),
    );
    let mut theorem_parameters = parameters.clone();
    theorem_parameters.push(parameter(target, &s));
    let user_arguments: Vec<ProofTerm> = arguments.iter().cloned().map(given).collect();
    lowering.theorems.push(theorem(
        &format!("{name}.apply_sound"),
        type_parameters,
        theorem_parameters.clone(),
        term(implies(
            applied.clone(),
            at.relation(state.clone(), var(target)),
        )),
        by_term(lemma(
            Lemma::Guarded,
            vec![
                given(at.relation_ref()),
                given(state.clone()),
                given(guard_call.clone()),
                given(conclusion_call.clone()),
                cite(soundness, &own, user_arguments.clone()),
                given(var(target)),
            ],
        )),
    ));
    if let Some(progress) = progress {
        let (Some(rank_ref), Some(before), Some(after)) =
            (at.rank_ref(), at.rank(state.clone()), at.rank(var(target)))
        else {
            return Err(format!("internal: rule `{name}` progress lost its ranking").into());
        };
        let decreases = implies(applied, lt(after, before));
        let ranked = |evidence: ProofTerm| {
            lemma(
                Lemma::GuardedRank,
                vec![
                    given(rank_ref.clone()),
                    given(state.clone()),
                    given(guard_call.clone()),
                    given(conclusion_call.clone()),
                    evidence,
                    given(var(target)),
                ],
            )
        };
        let (statement, proof) = match at.invariant(state.clone()) {
            Some(holds) => {
                let mut evidence = user_arguments.clone();
                evidence.push(hypothesis("llJ"));
                (
                    implies(holds, decreases),
                    assume(&["llJ"], ranked(cite(progress, &own, evidence))),
                )
            }
            None => (decreases, ranked(cite(progress, &own, user_arguments))),
        };
        lowering.theorems.push(theorem(
            &format!("{name}.apply_progress"),
            type_parameters,
            theorem_parameters,
            term(statement),
            by_term(proof),
        ));
    }
    env.reasoning.rules.insert(
        name.clone(),
        RuleInfo {
            type_parameters: type_parameters.clone(),
            logic: logic.clone(),
            binding: binding
                .as_ref()
                .map(|binding| (binding.name.clone(), binding.r#type.clone())),
            progress: progress.is_some(),
            executable: *executable,
        },
    );
    env.reasoning.guarded.insert(
        format!("{name}.conclusion"),
        "the unguarded conclusion of a rule",
    );
    Ok(lowering)
}

// ---------------------------------------------------------------------------
// Verifiers.

fn check_verifier(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::Verifier {
        name,
        type_parameters,
        subject,
        candidate,
        specification,
        check,
        sound,
        complete,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_axioms(name, axioms)?;
    check_signature(
        name,
        type_parameters,
        &[&subject.name, &candidate.name],
        &[&subject.r#type, &candidate.r#type],
        env,
    )?;
    let arguments = parameter_types(type_parameters);
    let interface = [subject.r#type.clone(), candidate.r#type.clone()];
    require_signature(
        specification,
        &arguments,
        &interface,
        &SemanticType::Prop,
        env,
        &format!("verifier `{name}` specification"),
    )?;
    let executable = require_signature(
        check,
        &arguments,
        &interface,
        &SemanticType::Bool,
        env,
        &format!("verifier `{name}` check"),
    )?;
    if !executable {
        return Err(mismatch(format!(
            "verifier `{name}` check `{}` is not executable, so no reasoner can run it",
            member_key(check)
        )));
    }
    let parameters = vec![
        parameter(&subject.name, &subject.r#type),
        parameter(&candidate.name, &candidate.r#type),
    ];
    let values = vec![var(&subject.name), var(&candidate.name)];
    let accepted = eq(call(check, &arguments, values.clone()), boolean(true));
    let holds = call(specification, &arguments, values);
    let mut lowering = Lowering::default();
    for (role, theorem_member, statement, helper_kind) in [
        (
            "sound",
            Some(sound),
            implies(accepted.clone(), holds.clone()),
            Helper::Sound,
        ),
        (
            "complete",
            complete.as_ref(),
            implies(holds, accepted),
            Helper::Complete,
        ),
    ] {
        let Some(theorem_member) = theorem_member else {
            continue;
        };
        let obligation = Obligation {
            role: format!("verifier `{name}` {role}"),
            theorem: theorem_member.clone(),
            type_parameters: type_parameters.clone(),
            parameters: parameters.clone(),
            statement,
        };
        require_obligation(env, &obligation, code!("LLT4010"))?;
        lowering.obligations.push(obligation);
        lowering.theorems.push(theorem(
            &format!("{name}.{role}"),
            type_parameters,
            Vec::new(),
            helper(
                helper_kind,
                vec![
                    function_ref(check, &arguments),
                    function_ref(specification, &arguments),
                ],
            ),
            by_term(cite(theorem_member, &arguments, Vec::new())),
        ));
    }
    env.reasoning.verifiers.insert(
        name.clone(),
        VerifierInfo {
            type_parameters: type_parameters.clone(),
            subject: subject.clone(),
            candidate: candidate.clone(),
            specification: specification.clone(),
            check: check.clone(),
            sound: sound.clone(),
        },
    );
    Ok(lowering)
}

// ---------------------------------------------------------------------------
// Reasoners.

/// The checked interface of one rule-based reasoner, from which its
/// elaboration is generated.
struct Engine<'a> {
    name: &'a str,
    logic: LogicAt,
    rules: Vec<RuleAt>,
    verifier: VerifierAt,
    /// The observation binder and type.
    x: &'a str,
    input: SemanticType,
    /// The state, answer, and failure types.
    state: SemanticType,
    answer: SemanticType,
    /// The elaborated observation, answer, and bound terms.
    observe: SemanticTerm,
    answer_binder: &'a str,
    extract: SemanticTerm,
    fuel: SemanticTerm,
    frontier: Option<SemanticTerm>,
    order: SearchOrder,
    deduplicate: bool,
    initial: Option<MemberRef>,
    terminates: Option<MemberRef>,
    /// The theorem that the answer term is correct on every state, which
    /// erases the verifier's check.
    answer_correct: Option<MemberRef>,
    /// The reasoner's type parameters: every elaborated declaration and
    /// generated theorem takes them (see [`generalize`]).
    type_parameters: &'a [String],
    executable: bool,
    axioms: &'a [String],
}

/// The six counters of a reasoner's ledger, in field order.
pub const LEDGER: [&str; 6] = [
    "iterations",
    "attempts",
    "firings",
    "expansions",
    "verifications",
    "frontier",
];

/// One value per ledger counter.
struct Counts {
    iterations: SemanticTerm,
    attempts: SemanticTerm,
    firings: SemanticTerm,
    expansions: SemanticTerm,
    verifications: SemanticTerm,
    frontier: SemanticTerm,
}

impl Counts {
    /// Every counter zero but the frontier.
    fn zero(frontier: SemanticTerm) -> Self {
        Self {
            iterations: nat(0),
            attempts: nat(0),
            firings: nat(0),
            expansions: nat(0),
            verifications: nat(0),
            frontier,
        }
    }

    /// The counters of the ledger `ledger` holds, unchanged.
    fn of(ledger: &SemanticTerm) -> Self {
        let field = |name: &str| project(ledger.clone(), name);
        Self {
            iterations: field("iterations"),
            attempts: field("attempts"),
            firings: field("firings"),
            expansions: field("expansions"),
            verifications: field("verifications"),
            frontier: field("frontier"),
        }
    }

    fn fields(self) -> [SemanticTerm; 6] {
        [
            self.iterations,
            self.attempts,
            self.firings,
            self.expansions,
            self.verifications,
            self.frontier,
        ]
    }
}

/// `value + 1`.
fn succ(value: SemanticTerm) -> SemanticTerm {
    add(value, nat(1))
}

/// The ledger structure every reasoner elaborates.
fn ledger_structure(name: &str) -> SemanticDeclaration {
    SemanticDeclaration::Structure {
        name: format!("{name}.Ledger"),
        type_parameters: Vec::new(),
        parameters: Vec::new(),
        fields: LEDGER
            .into_iter()
            .map(|field| SemanticField {
                name: field.to_owned(),
                r#type: SemanticType::Nat,
            })
            .collect(),
    }
}

/// A ledger record of reasoner `name`.
fn ledger_record(name: &str, counts: Counts) -> SemanticTerm {
    record(
        &format!("{name}.Ledger"),
        LEDGER.into_iter().zip(counts.fields()).collect(),
    )
}

impl Engine<'_> {
    fn own(&self, suffix: &str) -> MemberRef {
        local(&format!("{}.{suffix}", self.name))
    }

    fn own_name(&self, suffix: &str) -> String {
        format!("{}.{suffix}", self.name)
    }

    fn me(&self) -> MemberRef {
        local(self.name)
    }

    fn call_own(&self, suffix: &str, arguments: Vec<SemanticTerm>) -> SemanticTerm {
        call(&self.own(suffix), &[], arguments)
    }

    fn step_type(&self) -> SemanticType {
        named(&self.own_name("Step"))
    }

    fn ledger_type(&self) -> SemanticType {
        named(&self.own_name("Ledger"))
    }

    fn node_type(&self) -> SemanticType {
        named(&self.own_name("Node"))
    }

    fn run_type(&self) -> SemanticType {
        named(&self.own_name("Run"))
    }

    fn search_type(&self) -> SemanticType {
        named(&self.own_name("Search"))
    }

    fn replay_type(&self) -> SemanticType {
        result_of(self.state.clone())
    }

    fn explained_type(&self) -> SemanticType {
        product(self.answer.clone(), list_type(self.step_type()))
    }

    fn step_value(&self, rule: &RuleAt, binding: Option<SemanticTerm>) -> SemanticTerm {
        constructor(
            &format!("{}.Step.{}", self.name, rule.member.name),
            Vec::new(),
            binding.into_iter().collect(),
        )
    }

    fn define(
        &self,
        suffix: &str,
        parameters: Vec<SemanticParameter>,
        result: SemanticType,
        body: SemanticTerm,
    ) -> SemanticDeclaration {
        definition(
            &self.own_name(suffix),
            parameters,
            result,
            body,
            self.executable,
            self.axioms,
        )
    }

    fn ledger(&self, counts: Counts) -> SemanticTerm {
        ledger_record(self.name, counts)
    }

    /// `follow x n.trace = ok n.state` for a node or loop record `n`.
    fn replays(&self, value: &SemanticTerm) -> SemanticTerm {
        eq(
            self.call_own("follow", vec![var(self.x), project(value.clone(), "trace")]),
            ok_of(&self.state, project(value.clone(), "state")),
        )
    }

    fn spec(&self, candidate: SemanticTerm) -> SemanticTerm {
        call(
            &self.verifier.specification,
            &self.verifier.type_arguments,
            vec![var(self.x), candidate],
        )
    }

    fn cap(&self, list: SemanticTerm) -> SemanticTerm {
        let node = self.node_type();
        let bound = self.frontier.clone().unwrap_or_else(|| nat(0));
        list_fold(
            lambda(
                vec![("__acc", list_type(node.clone())), ("__n", node.clone())],
                if_then(
                    blt(length(var("__acc")), bound),
                    append(var("__acc"), single(&node, var("__n")), &node),
                    var("__acc"),
                ),
            ),
            nil(&node),
            list,
            list_type(node),
        )
    }

    /// The guard evaluations of a rule over the state `state`: one for a
    /// binding-free rule, one per candidate for a binding rule.
    fn guard_count(rule: &RuleAt, state: SemanticTerm) -> SemanticTerm {
        match rule.binding {
            None => nat(1),
            Some(_) => length(call(
                &rule.companion("candidates"),
                &rule.type_arguments,
                vec![state],
            )),
        }
    }
}

/// The claims of a reasoner, strictly sorted by kind, each at most once.
struct Claims {
    initial: Option<MemberRef>,
    terminates: Option<MemberRef>,
    answer_correct: Option<MemberRef>,
}

fn sorted_claims(name: &str, claims: &[ReasoningClaim]) -> Result<Claims, SemanticFailure> {
    let mut out = Claims {
        initial: None,
        terminates: None,
        answer_correct: None,
    };
    let mut previous: Option<u8> = None;
    for claim in claims {
        let rank = match claim {
            ReasoningClaim::InitialInvariant { .. } => 0,
            ReasoningClaim::Terminates { .. } => 1,
            ReasoningClaim::AnswerCorrect { .. } => 2,
        };
        if previous.is_some_and(|prior| prior >= rank) {
            return Err(mismatch(format!(
                "reasoner `{name}` claims are not strictly sorted by kind"
            )));
        }
        previous = Some(rank);
        match claim {
            ReasoningClaim::InitialInvariant { theorem } => out.initial = Some(theorem.clone()),
            ReasoningClaim::Terminates { theorem } => out.terminates = Some(theorem.clone()),
            ReasoningClaim::AnswerCorrect { theorem } => {
                out.answer_correct = Some(theorem.clone());
            }
        }
    }
    Ok(out)
}

#[allow(clippy::too_many_lines)]
fn check_reasoner(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::Reasoner {
        name,
        type_parameters,
        logic,
        observation,
        observe,
        rules,
        strategy,
        answer,
        verifier,
        claims,
        executable,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_axioms(name, axioms)?;
    if let ReasoningStrategy::GenerateAndVerify { generator, budget } = strategy {
        return check_generator(declaration, generator, budget.as_ref(), env);
    }
    let (Some(logic), Some(observe), Some(answer)) = (logic, observe, answer) else {
        return Err(mismatch(format!(
            "reasoner `{name}` applies inference rules, so it names its logic, the state it observes, and its answer"
        )));
    };
    let scope = check_signature(
        name,
        type_parameters,
        &[&observation.name, &answer.name],
        &[&observation.r#type, &answer.r#type],
        env,
    )?;
    let at = resolve_logic(logic, &scope, env, &format!("reasoner `{name}` logic"))?;
    let state = at.state.clone();
    let verifier_at = resolve_verifier(verifier, &scope, env)?;
    if verifier_at.subject != observation.r#type || verifier_at.candidate != answer.r#type {
        return Err(mismatch(format!(
            "reasoner `{name}` observes {} and answers {}, but verifier `{}` checks a {} subject and a {} candidate",
            observation.r#type,
            answer.r#type,
            member_key(&verifier.member),
            verifier_at.subject,
            verifier_at.candidate
        )));
    }
    if rules.is_empty() {
        return Err(mismatch(format!(
            "reasoner `{name}` names no inference rule"
        )));
    }
    let mut resolved = Vec::new();
    let mut constructors = BTreeSet::new();
    for rule in rules {
        let rule_at = resolve_rule(rule, &scope, env)?;
        if rule_at.logic.member != logic.member
            || rule_at.logic.type_arguments != logic.type_arguments
        {
            return Err(mismatch(format!(
                "reasoner `{name}` reasons in logic `{}`, but rule `{}` is over logic `{}`",
                member_key(&logic.member),
                member_key(&rule.member),
                member_key(&rule_at.logic.member)
            )));
        }
        if !constructors.insert(rule.member.name.clone()) {
            return Err(mismatch(format!(
                "reasoner `{name}` names two rules called `{}`; rules are distinct and name the steps of its trace",
                rule.member.name
            )));
        }
        if *executable && !rule_at.executable {
            return Err(mismatch(format!(
                "executable reasoner `{name}` applies rule `{}`, which is not executable",
                member_key(&rule.member)
            )));
        }
        resolved.push(rule_at);
    }
    let x = observation.name.as_str();
    let input_locals = [(x, &observation.r#type)];
    check_source_binders(declaration)?;
    let missing = |what: &str| {
        unbounded(format!(
            "reasoner `{name}` declares no {what}; every reasoning strategy is bounded by an explicit natural-number {what}"
        ))
    };
    let (fuel, frontier, order, deduplicate) = match strategy {
        ReasoningStrategy::Forward { fuel } => (
            fuel.clone().ok_or_else(|| missing("fuel"))?,
            None,
            SearchOrder::BreadthFirst,
            false,
        ),
        ReasoningStrategy::Search {
            order,
            fuel,
            frontier,
            deduplicate,
        } => {
            let fuel = fuel.clone().ok_or_else(|| missing("fuel"))?;
            let frontier = frontier.clone().ok_or_else(|| missing("frontier"))?;
            if matches!(&frontier, SemanticTerm::Nat { value } if value == "0") {
                return Err(unbounded(format!(
                    "reasoner `{name}` bounds its frontier by zero, so it can explore nothing"
                )));
            }
            if *deduplicate {
                check_ordered_key(&state).map_err(|reason| {
                    unbounded(format!(
                        "reasoner `{name}` deduplicates states of type {state}, which is not an ordered key type: {reason}"
                    ))
                })?;
            }
            (fuel, Some(frontier), *order, *deduplicate)
        }
        ReasoningStrategy::GenerateAndVerify { .. } => {
            return Err(format!("internal: reasoner `{name}` generates and verifies").into());
        }
    };
    // The source terms run at the boundary exactly when the reasoner does;
    // they are typed, and obligations stated over them, as linking
    // elaborated them.
    let mut sources = vec![observe.clone(), answer.value.clone(), fuel];
    if let Some(frontier) = &frontier {
        sources.push(frontier.clone());
    }
    lower_source(name, sources.iter_mut().collect(), *executable, &scope, env)?;
    env.derived = true;
    let typed = (|| {
        require_term(
            &sources[0],
            &input_locals,
            &scope,
            &state,
            env,
            &format!("reasoner `{name}` observation"),
            mismatch,
        )?;
        require_term(
            &sources[1],
            &[(answer.name.as_str(), &state)],
            &scope,
            &option_type(answer.r#type.clone()),
            env,
            &format!("reasoner `{name}` answer"),
            mismatch,
        )?;
        for (term, what) in sources[2..].iter().zip(["fuel", "frontier"]) {
            require_term(
                term,
                &input_locals,
                &scope,
                &SemanticType::Nat,
                env,
                &format!("reasoner `{name}` {what}"),
                unbounded,
            )?;
        }
        Ok::<(), SemanticFailure>(())
    })();
    env.derived = false;
    typed?;
    let observed = sources[0].clone();
    let fuel = sources[2].clone();
    let Claims {
        initial,
        terminates,
        answer_correct,
    } = sorted_claims(name, claims)?;
    let x_parameter = vec![parameter(x, &observation.r#type)];
    let mut lowering = Lowering::default();
    if let Some(theorem_member) = &initial {
        let Some(holds) = at.invariant(observed.clone()) else {
            return Err(mismatch(format!(
                "reasoner `{name}` claims an initial invariant, but logic `{}` has none",
                member_key(&logic.member)
            )));
        };
        let obligation = Obligation {
            role: format!("reasoner `{name}` initial invariant"),
            theorem: theorem_member.clone(),
            type_parameters: type_parameters.clone(),
            parameters: x_parameter.clone(),
            statement: holds,
        };
        require_obligation(env, &obligation, code!("LLT4010"))?;
        lowering.obligations.push(obligation);
    }
    if let Some(theorem_member) = &terminates {
        if !matches!(strategy, ReasoningStrategy::Forward { .. }) {
            return Err(unbounded(format!(
                "reasoner `{name}` claims termination, which only a forward reasoner can claim; a search is bounded by its fuel and frontier"
            )));
        }
        let Some(ranked) = at.rank(observed.clone()) else {
            return Err(unbounded(format!(
                "reasoner `{name}` claims termination, but logic `{}` has no ranking",
                member_key(&logic.member)
            )));
        };
        if let Some(rule) = resolved.iter().find(|rule| !rule.progress) {
            return Err(unbounded(format!(
                "reasoner `{name}` claims termination, but rule `{}` states no progress",
                member_key(&rule.member)
            )));
        }
        if at.invariant.is_some() && initial.is_none() {
            return Err(unbounded(format!(
                "reasoner `{name}` claims termination, but its rules' progress assumes the invariant of logic `{}` and it claims no initial invariant",
                member_key(&logic.member)
            )));
        }
        let obligation = Obligation {
            role: format!("reasoner `{name}` fuel bound"),
            theorem: theorem_member.clone(),
            type_parameters: type_parameters.clone(),
            parameters: x_parameter.clone(),
            statement: lt(ranked, fuel.clone()),
        };
        require_obligation(env, &obligation, code!("LLT4010"))?;
        lowering.obligations.push(obligation);
    }
    if let Some(theorem_member) = &answer_correct {
        // Correct on every state, the answer needs no check on the states
        // the reasoner reaches; the obligation names only prior source, so
        // it is stated, and proved, before the reasoner.
        let answered = sources[1].clone();
        let mut parameters = x_parameter.clone();
        parameters.push(parameter(&answer.name, &state));
        parameters.push(parameter("__v", &answer.r#type));
        let obligation = Obligation {
            role: format!("reasoner `{name}` answer correctness"),
            theorem: theorem_member.clone(),
            type_parameters: type_parameters.clone(),
            parameters,
            statement: implies(
                eq(answered, some_of(&answer.r#type, var("__v"))),
                call(
                    &verifier_at.specification,
                    &verifier_at.type_arguments,
                    vec![var(x), var("__v")],
                ),
            ),
        };
        require_obligation(env, &obligation, code!("LLT4010"))?;
        lowering.obligations.push(obligation);
    }
    let engine = Engine {
        name,
        logic: at,
        rules: resolved,
        verifier: verifier_at,
        x,
        input: observation.r#type.clone(),
        state,
        answer: answer.r#type.clone(),
        observe: observed,
        answer_binder: &answer.name,
        extract: sources[1].clone(),
        fuel,
        frontier: sources.get(3).cloned(),
        order,
        deduplicate,
        initial,
        terminates,
        answer_correct,
        type_parameters,
        executable: *executable,
        axioms,
    };
    elaborate_common(&engine, &mut lowering);
    if engine.frontier.is_some() {
        elaborate_search(&engine, &mut lowering);
    } else {
        elaborate_forward(&engine, &mut lowering);
    }
    if engine.answer_correct.is_none() {
        env.reasoning.guarded.insert(
            engine.own_name("extract"),
            "the unverified answer of a reasoner",
        );
    }
    generalize(name, type_parameters, &mut lowering);
    Ok(lowering)
}

/// The checked interface of one generate-and-verify reasoner.
struct Generate<'a> {
    name: &'a str,
    verifier: VerifierAt,
    x: &'a str,
    input: SemanticType,
    answer: SemanticType,
    generator: SemanticTerm,
    budget: SemanticTerm,
    executable: bool,
    axioms: &'a [String],
}

impl Generate<'_> {
    fn own(&self, suffix: &str) -> MemberRef {
        local(&format!("{}.{suffix}", self.name))
    }

    fn own_name(&self, suffix: &str) -> String {
        format!("{}.{suffix}", self.name)
    }

    fn call_own(&self, suffix: &str, arguments: Vec<SemanticTerm>) -> SemanticTerm {
        call(&self.own(suffix), &[], arguments)
    }

    fn define(
        &self,
        suffix: &str,
        parameters: Vec<SemanticParameter>,
        result: SemanticType,
        body: SemanticTerm,
    ) -> SemanticDeclaration {
        definition(
            &self.own_name(suffix),
            parameters,
            result,
            body,
            self.executable,
            self.axioms,
        )
    }

    fn spec(&self, candidate: SemanticTerm) -> SemanticTerm {
        call(
            &self.verifier.specification,
            &self.verifier.type_arguments,
            vec![var(self.x), candidate],
        )
    }

    /// `verify x v = some v /\ Spec x v`: what every accepted candidate
    /// satisfies.
    fn accepted(&self, candidate: SemanticTerm) -> SemanticTerm {
        both(
            eq(
                self.call_own("verify", vec![var(self.x), candidate.clone()]),
                some_of(&self.answer, candidate.clone()),
            ),
            self.spec(candidate),
        )
    }

    /// `forall v, t.found = some v -> accepted v` for a trial `t`.
    fn found_accepted(&self, trial: SemanticTerm) -> SemanticTerm {
        forall(
            "__v",
            &self.answer,
            implies(
                eq(project(trial, "found"), some_of(&self.answer, var("__v"))),
                self.accepted(var("__v")),
            ),
        )
    }
}

/// Check a generate-and-verify reasoner: no logic, observed state, answer,
/// rule, or claim; a generator of candidate answers over the observation;
/// and a mandatory budget of checks.
#[allow(clippy::too_many_lines)]
fn check_generator(
    declaration: &SemanticDeclaration,
    generator: &SemanticTerm,
    budget: Option<&SemanticTerm>,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::Reasoner {
        name,
        type_parameters,
        logic,
        observation,
        observe,
        rules,
        answer,
        verifier,
        claims,
        executable,
        axioms,
        ..
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    if logic.is_some() || observe.is_some() || answer.is_some() {
        return Err(mismatch(format!(
            "generate-and-verify reasoner `{name}` draws its answers from its generator, so it names no logic, observed state, or answer"
        )));
    }
    if !rules.is_empty() {
        return Err(mismatch(format!(
            "generate-and-verify reasoner `{name}` applies no inference rule; its verifier checks every candidate its generator proposes"
        )));
    }
    if !claims.is_empty() {
        return Err(mismatch(format!(
            "generate-and-verify reasoner `{name}` makes no claim: an answer is used only once its verifier accepts it"
        )));
    }
    let scope = check_signature(
        name,
        type_parameters,
        &[&observation.name],
        &[&observation.r#type],
        env,
    )?;
    let verifier_at = resolve_verifier(verifier, &scope, env)?;
    if verifier_at.subject != observation.r#type {
        return Err(mismatch(format!(
            "reasoner `{name}` observes {}, but verifier `{}` checks a {} subject",
            observation.r#type,
            member_key(&verifier.member),
            verifier_at.subject
        )));
    }
    let Some(budget) = budget else {
        return Err(unbounded(format!(
            "reasoner `{name}` declares no budget; a generate-and-verify reasoner checks at most an explicit natural-number budget of candidates"
        )));
    };
    check_source_binders(declaration)?;
    let x = observation.name.as_str();
    let input_locals = [(x, &observation.r#type)];
    let answer_type = verifier_at.candidate.clone();
    let mut sources = [generator.clone(), budget.clone()];
    lower_source(name, sources.iter_mut().collect(), *executable, &scope, env)?;
    env.derived = true;
    let typed = (|| {
        require_term(
            &sources[0],
            &input_locals,
            &scope,
            &list_type(answer_type.clone()),
            env,
            &format!("reasoner `{name}` generator"),
            mismatch,
        )?;
        require_term(
            &sources[1],
            &input_locals,
            &scope,
            &SemanticType::Nat,
            env,
            &format!("reasoner `{name}` budget"),
            unbounded,
        )
    })();
    env.derived = false;
    typed?;
    let engine = Generate {
        name,
        verifier: verifier_at,
        x,
        input: observation.r#type.clone(),
        answer: answer_type,
        generator: sources[0].clone(),
        budget: sources[1].clone(),
        executable: *executable,
        axioms,
    };
    let mut lowering = Lowering::default();
    elaborate_generate(&engine, &mut lowering);
    generalize(name, type_parameters, &mut lowering);
    Ok(lowering)
}

/// A generate-and-verify reasoner: check the generator's candidates in
/// order, at most the budget of them, and answer the first the verifier
/// accepts. Its trace names the accepted candidate, and replaying it checks
/// that candidate again.
#[allow(clippy::too_many_lines)]
fn elaborate_generate(engine: &Generate<'_>, lowering: &mut Lowering) {
    let x = engine.x;
    let r = engine.answer.clone();
    let step = named(&engine.own_name("Step"));
    let trial = named(&engine.own_name("Trial"));
    let ledger = named(&engine.own_name("Ledger"));
    lowering.declarations.push(SemanticDeclaration::Inductive {
        name: engine.own_name("Step"),
        type_parameters: Vec::new(),
        parameters: Vec::new(),
        constructors: vec![SemanticConstructor {
            name: "candidate".to_owned(),
            fields: vec![r.clone()],
        }],
        mutual: None,
    });
    lowering.declarations.push(ledger_structure(engine.name));
    lowering.declarations.push(engine.define(
        "candidates",
        vec![parameter(x, &engine.input)],
        list_type(r.clone()),
        engine.generator.clone(),
    ));
    lowering.declarations.push(engine.define(
        "verify",
        vec![parameter(x, &engine.input), parameter("__c", &r)],
        option_type(r.clone()),
        if_then(
            call(
                &engine.verifier.check,
                &engine.verifier.type_arguments,
                vec![var(x), var("__c")],
            ),
            some_of(&r, var("__c")),
            none_of(&r),
        ),
    ));
    lowering.declarations.push(SemanticDeclaration::Structure {
        name: engine.own_name("Trial"),
        type_parameters: Vec::new(),
        parameters: Vec::new(),
        fields: vec![
            SemanticField {
                name: "found".to_owned(),
                r#type: option_type(r.clone()),
            },
            SemanticField {
                name: "truncated".to_owned(),
                r#type: SemanticType::Bool,
            },
            SemanticField {
                name: "ledger".to_owned(),
                r#type: ledger.clone(),
            },
        ],
    });
    let trial_record = |found: SemanticTerm, truncated: SemanticTerm, counts: SemanticTerm| {
        record(
            &engine.own_name("Trial"),
            vec![
                ("found", found),
                ("truncated", truncated),
                ("ledger", counts),
            ],
        )
    };
    let current = var("__t");
    let current_ledger = project(current.clone(), "ledger");
    let unchanged = Counts::of(&current_ledger);
    let checked = Counts {
        iterations: succ(unchanged.iterations.clone()),
        attempts: succ(unchanged.attempts.clone()),
        verifications: succ(unchanged.verifications.clone()),
        ..Counts::of(&current_ledger)
    };
    lowering.declarations.push(engine.define(
        "try",
        vec![
            parameter(x, &engine.input),
            parameter("__t", &trial),
            parameter("__c", &r),
        ],
        trial.clone(),
        matching(
            project(current.clone(), "found"),
            vec![
                ("Option.some", vec!["__hit"], current.clone()),
                (
                    "Option.none",
                    vec![],
                    matching(
                        blt(
                            project(current_ledger.clone(), "verifications"),
                            engine.budget.clone(),
                        ),
                        vec![
                            (
                                "Bool.true",
                                vec![],
                                trial_record(
                                    engine.call_own("verify", vec![var(x), var("__c")]),
                                    project(current.clone(), "truncated"),
                                    ledger_record(engine.name, checked),
                                ),
                            ),
                            (
                                "Bool.false",
                                vec![],
                                trial_record(none_of(&r), boolean(true), current_ledger),
                            ),
                        ],
                    ),
                ),
            ],
        ),
    ));
    let start = trial_record(
        none_of(&r),
        boolean(false),
        ledger_record(engine.name, Counts::zero(nat(0))),
    );
    let tried = lambda(
        vec![("__t", trial.clone()), ("__c", r.clone())],
        engine.call_own("try", vec![var(x), var("__t"), var("__c")]),
    );
    let candidates = engine.call_own("candidates", vec![var(x)]);
    lowering.declarations.push(engine.define(
        "run",
        vec![parameter(x, &engine.input)],
        trial.clone(),
        list_fold(
            tried.clone(),
            start.clone(),
            candidates.clone(),
            trial.clone(),
        ),
    ));
    // No candidate was accepted: the budget cut the candidates short
    // (`exhausted`), the generator proposed none (`unsolved`), or the
    // verifier refused every one (`rejected`).
    lowering.declarations.push(engine.define(
        "failure",
        vec![parameter("__t", &trial)],
        SemanticType::ReasoningFailure,
        matching(
            project(current.clone(), "truncated"),
            vec![
                ("Bool.true", vec![], failure_value("exhausted")),
                (
                    "Bool.false",
                    vec![],
                    matching(
                        SemanticTerm::Beq {
                            left: Box::new(project(
                                project(current.clone(), "ledger"),
                                "verifications",
                            )),
                            right: Box::new(nat(0)),
                        },
                        vec![
                            ("Bool.true", vec![], failure_value("unsolved")),
                            ("Bool.false", vec![], failure_value("rejected")),
                        ],
                    ),
                ),
            ],
        ),
    ));
    let ran = engine.call_own("run", vec![var(x)]);
    let final_trial = var("__final");
    let failed =
        |ok: &SemanticType| error_of(ok, engine.call_own("failure", vec![final_trial.clone()]));
    lowering.declarations.push(engine.define(
        "verdict",
        vec![parameter(x, &engine.input)],
        result_of(r.clone()),
        let_in(
            "__final",
            trial.clone(),
            ran.clone(),
            matching(
                project(final_trial.clone(), "found"),
                vec![
                    ("Option.some", vec!["__v"], ok_of(&r, var("__v"))),
                    ("Option.none", vec![], failed(&r)),
                ],
            ),
        ),
    ));
    let explained = product(r.clone(), list_type(step.clone()));
    let candidate_step = |value: SemanticTerm| {
        constructor(
            &format!("{}.Step.candidate", engine.name),
            Vec::new(),
            vec![value],
        )
    };
    lowering.declarations.push(definition(
        engine.name,
        vec![parameter(x, &engine.input)],
        result_of(explained.clone()),
        let_in(
            "__final",
            trial.clone(),
            ran.clone(),
            matching(
                project(final_trial.clone(), "found"),
                vec![
                    (
                        "Option.some",
                        vec!["__v"],
                        ok_of(
                            &explained,
                            pair(var("__v"), single(&step, candidate_step(var("__v")))),
                        ),
                    ),
                    ("Option.none", vec![], failed(&explained)),
                ],
            ),
        ),
        engine.executable,
        engine.axioms,
    ));
    lowering.declarations.push(engine.define(
        "answer",
        vec![
            parameter(x, &engine.input),
            parameter("__trace", &list_type(step.clone())),
        ],
        option_type(r.clone()),
        matching(
            var("__trace"),
            vec![
                ("List.nil", vec![], none_of(&r)),
                (
                    "List.cons",
                    vec!["__step", "__rest"],
                    matching(
                        var("__rest"),
                        vec![
                            (
                                "List.nil",
                                vec![],
                                matching(
                                    var("__step"),
                                    vec![(
                                        &*format!("{}.Step.candidate", engine.name),
                                        vec!["__c"],
                                        engine.call_own("verify", vec![var(x), var("__c")]),
                                    )],
                                ),
                            ),
                            ("List.cons", vec!["__next", "__more"], none_of(&r)),
                        ],
                    ),
                ),
            ],
        ),
    ));
    // Generated theorems.
    let x_only = vec![parameter(x, &engine.input)];
    lowering.theorems.push(theorem(
        &engine.own_name("verify_sound"),
        &[],
        vec![
            parameter(x, &engine.input),
            parameter("__c", &r),
            parameter("__v", &r),
        ],
        term(implies(
            eq(
                engine.call_own("verify", vec![var(x), var("__c")]),
                some_of(&r, var("__v")),
            ),
            engine.accepted(var("__v")),
        )),
        Proof::VerifySound {
            verify: engine.own("verify"),
            sound: engine.verifier.sound.clone(),
            type_arguments: engine.verifier.type_arguments.clone(),
        },
    ));
    let tried_trial = engine.call_own("try", vec![var(x), var("__t"), var("__c")]);
    let trial_parameters = vec![
        parameter(x, &engine.input),
        parameter("__t", &trial),
        parameter("__c", &r),
    ];
    lowering.theorems.push(theorem(
        &engine.own_name("try_sound"),
        &[],
        trial_parameters.clone(),
        term(implies(
            engine.found_accepted(var("__t")),
            engine.found_accepted(tried_trial.clone()),
        )),
        Proof::TrySound {
            attempt: engine.own("try"),
            verify_sound: engine.own("verify_sound"),
        },
    ));
    let checks = |value: SemanticTerm| project(project(value, "ledger"), "verifications");
    lowering.theorems.push(theorem(
        &engine.own_name("try_count"),
        &[],
        trial_parameters,
        term(implies(
            le(checks(var("__t")), engine.budget.clone()),
            le(checks(tried_trial), engine.budget.clone()),
        )),
        Proof::TryCount {
            attempt: engine.own("try"),
        },
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("run_sound"),
        &[],
        x_only.clone(),
        term(engine.found_accepted(ran.clone())),
        unfolding(
            vec![engine.own("run")],
            lemma(
                Lemma::FoldInvariant,
                vec![
                    given(tried.clone()),
                    predicate("__t", &trial, term(engine.found_accepted(var("__t")))),
                    cite(&engine.own("try_sound"), &[], vec![given(var(x))]),
                    given(candidates.clone()),
                    given(start.clone()),
                    assume(
                        &["llV", "llE"],
                        lemma(Lemma::NoneSome, vec![ProofTerm::Infer, hypothesis("llE")]),
                    ),
                ],
            ),
        ),
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("verifications_bounded"),
        &[],
        x_only.clone(),
        term(le(checks(ran.clone()), engine.budget.clone())),
        unfolding(
            vec![engine.own("run")],
            lemma(
                Lemma::FoldInvariant,
                vec![
                    given(tried),
                    predicate(
                        "__t",
                        &trial,
                        term(le(checks(var("__t")), engine.budget.clone())),
                    ),
                    cite(&engine.own("try_count"), &[], vec![given(var(x))]),
                    given(candidates),
                    given(start),
                    lemma(Lemma::ZeroLe, vec![ProofTerm::Infer]),
                ],
            ),
        ),
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("verdict_sound"),
        &[],
        x_only.clone(),
        term(forall(
            "__v",
            &r,
            implies(
                eq(
                    engine.call_own("verdict", vec![var(x)]),
                    ok_of(&r, var("__v")),
                ),
                engine.spec(var("__v")),
            ),
        )),
        Proof::VerdictGenerate {
            verdict: engine.own("verdict"),
            run: engine.own("run"),
            run_sound: engine.own("run_sound"),
        },
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("explained"),
        &[],
        x_only,
        term(forall(
            "__v",
            &r,
            forall(
                "__trace",
                &list_type(step),
                implies(
                    eq(
                        call(&local(engine.name), &[], vec![var(x)]),
                        ok_of(&explained, pair(var("__v"), var("__trace"))),
                    ),
                    both(
                        eq(
                            engine.call_own("answer", vec![var(x), var("__trace")]),
                            some_of(&r, var("__v")),
                        ),
                        engine.spec(var("__v")),
                    ),
                ),
            ),
        )),
        Proof::ExplainedGenerate {
            reasoner: local(engine.name),
            run: engine.own("run"),
            run_sound: engine.own("run_sound"),
            answer: engine.own("answer"),
        },
    ));
}

/// Make the elaboration of a generic reasoner generic: every elaborated
/// declaration and generated theorem takes the reasoner's type parameters,
/// and every reference to one of them, as a call, a function value, a
/// constructor, a record, a named type, or a cited theorem, passes them on.
/// The elaboration is built at no type arguments and rewritten here, once,
/// so no template can forget one.
fn generalize(name: &str, type_parameters: &[String], lowering: &mut Lowering) {
    fn rewrite(
        value: &mut serde_json::Value,
        own: &BTreeSet<String>,
        arguments: &serde_json::Value,
    ) {
        match value {
            serde_json::Value::Array(items) => {
                items
                    .iter_mut()
                    .for_each(|item| rewrite(item, own, arguments));
            }
            serde_json::Value::Object(object) => {
                let is_own = |key: &str, object: &serde_json::Map<String, serde_json::Value>| {
                    object.get(key).is_some_and(|member| {
                        member.get("module").is_none()
                            && member
                                .get("name")
                                .and_then(serde_json::Value::as_str)
                                .is_some_and(|name| own.contains(name))
                    })
                };
                let target = match object.get("kind").and_then(serde_json::Value::as_str) {
                    Some("call" | "function_ref") if is_own("function", object) => {
                        Some("type_arguments")
                    }
                    Some("constructor") if is_own("constructor", object) => Some("type_arguments"),
                    Some("record") if is_own("type", object) => Some("type_arguments"),
                    Some("theorem") if is_own("theorem", object) => Some("type_arguments"),
                    Some("named") if is_own("member", object) => Some("arguments"),
                    _ => None,
                };
                object
                    .values_mut()
                    .for_each(|child| rewrite(child, own, arguments));
                if let Some(target) = target {
                    object.insert(target.to_owned(), arguments.clone());
                }
            }
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }
    if type_parameters.is_empty() {
        return;
    }
    let mut own = BTreeSet::from([name.to_owned()]);
    for declaration in &lowering.declarations {
        own.insert(declaration.name().to_owned());
        if let SemanticDeclaration::Inductive {
            name: inductive,
            constructors,
            ..
        } = declaration
        {
            for constructor in constructors {
                own.insert(format!("{inductive}.{}", constructor.name));
            }
        }
    }
    own.extend(
        lowering
            .theorems
            .iter()
            .map(|generated| generated.name.clone()),
    );
    let arguments =
        serde_json::to_value(parameter_types(type_parameters)).expect("types serialize");
    let parameters = serde_json::to_value(type_parameters).expect("names serialize");
    for declaration in &mut lowering.declarations {
        let mut value = serde_json::to_value(&*declaration).expect("a declaration serializes");
        rewrite(&mut value, &own, &arguments);
        value["type_parameters"] = parameters.clone();
        *declaration =
            serde_json::from_value(value).expect("a generalized declaration deserializes");
    }
    for generated in &mut lowering.theorems {
        let mut value = serde_json::to_value(&*generated).expect("a theorem serializes");
        rewrite(&mut value, &own, &arguments);
        value["type_parameters"] = parameters.clone();
        *generated = serde_json::from_value(value).expect("a generalized theorem deserializes");
    }
}

/// The declarations and theorems every reasoner elaborates to: its step
/// type and ledger, observation, guarded firing, replay of a trace, and
/// verified answer.
#[allow(clippy::too_many_lines)]
fn elaborate_common(engine: &Engine<'_>, lowering: &mut Lowering) {
    let s = engine.state.clone();
    let step = engine.step_type();
    let x = engine.x;
    lowering.declarations.push(SemanticDeclaration::Inductive {
        name: engine.own_name("Step"),
        type_parameters: Vec::new(),
        parameters: Vec::new(),
        constructors: engine
            .rules
            .iter()
            .map(|rule| SemanticConstructor {
                name: rule.member.name.clone(),
                fields: rule.binding.iter().map(|(_, ty)| ty.clone()).collect(),
            })
            .collect(),
        mutual: None,
    });
    lowering.declarations.push(ledger_structure(engine.name));
    lowering.declarations.push(engine.define(
        "observe",
        vec![parameter(x, &engine.input)],
        s.clone(),
        engine.observe.clone(),
    ));
    // E.fire: one arm per rule, each exactly the rule's guarded
    // application, so no step applies a rule whose guard does not hold.
    let arms: Vec<(String, Vec<&str>, SemanticTerm)> = engine
        .rules
        .iter()
        .map(|rule| {
            let constructor_name = format!("{}.Step.{}", engine.name, rule.member.name);
            let mut arguments = vec![var("__s")];
            let mut binders = Vec::new();
            if rule.binding.is_some() {
                arguments.push(var("__b"));
                binders.push("__b");
            }
            (
                constructor_name,
                binders,
                call(&rule.companion("apply"), &rule.type_arguments, arguments),
            )
        })
        .collect();
    lowering.declarations.push(
        engine.define(
            "fire",
            vec![parameter("__s", &s), parameter("__step", &step)],
            option_type(s.clone()),
            matching(
                var("__step"),
                arms.iter()
                    .map(|(name, binders, body)| (name.as_str(), binders.clone(), body.clone()))
                    .collect(),
            ),
        ),
    );
    let replay_type = engine.replay_type();
    lowering.declarations.push(engine.define(
        "replay",
        vec![parameter("__acc", &replay_type), parameter("__step", &step)],
        replay_type.clone(),
        matching(
            var("__acc"),
            vec![
                ("Result.error", vec!["__e"], error_of(&s, var("__e"))),
                (
                    "Result.ok",
                    vec!["__s"],
                    matching(
                        engine.call_own("fire", vec![var("__s"), var("__step")]),
                        vec![
                            (
                                "Option.none",
                                vec![],
                                error_of(&s, failure_value("invalid_step")),
                            ),
                            ("Option.some", vec!["__t"], ok_of(&s, var("__t"))),
                        ],
                    ),
                ),
            ],
        ),
    ));
    lowering.declarations.push(engine.define(
        "follow",
        vec![
            parameter(x, &engine.input),
            parameter("__trace", &list_type(step.clone())),
        ],
        replay_type.clone(),
        list_fold(
            function_ref(&engine.own("replay"), &[]),
            ok_of(&s, engine.call_own("observe", vec![var(x)])),
            var("__trace"),
            replay_type.clone(),
        ),
    ));
    let r = engine.answer.clone();
    lowering.declarations.push(engine.define(
        "extract",
        vec![parameter(engine.answer_binder, &s)],
        option_type(r.clone()),
        engine.extract.clone(),
    ));
    // The accepted answer: the extracted answer the verifier's check
    // accepts, or, when the answer is proved correct on every state, the
    // extracted answer itself, with the check erased.
    let checked = match &engine.answer_correct {
        Some(_) => engine.call_own("extract", vec![var("__s")]),
        None => matching(
            engine.call_own("extract", vec![var("__s")]),
            vec![
                ("Option.none", vec![], none_of(&r)),
                (
                    "Option.some",
                    vec!["__v"],
                    if_then(
                        call(
                            &engine.verifier.check,
                            &engine.verifier.type_arguments,
                            vec![var(x), var("__v")],
                        ),
                        some_of(&r, var("__v")),
                        none_of(&r),
                    ),
                ),
            ],
        ),
    };
    lowering.declarations.push(engine.define(
        "accept",
        vec![parameter(x, &engine.input), parameter("__s", &s)],
        option_type(r.clone()),
        checked,
    ));
    lowering.declarations.push(engine.define(
        "answer",
        vec![
            parameter(x, &engine.input),
            parameter("__trace", &list_type(step.clone())),
        ],
        option_type(r.clone()),
        matching(
            engine.call_own("follow", vec![var(x), var("__trace")]),
            vec![
                (
                    "Result.ok",
                    vec!["__s"],
                    engine.call_own("accept", vec![var(x), var("__s")]),
                ),
                ("Result.error", vec!["__e"], none_of(&r)),
            ],
        ),
    ));
    // Generated theorems.
    let at = &engine.logic;
    let fired = eq(
        engine.call_own("fire", vec![var("__s"), var("__step")]),
        some_of(&s, var("__t")),
    );
    let fire_parameters = vec![
        parameter("__s", &s),
        parameter("__step", &step),
        parameter("__t", &s),
    ];
    let fire_arms = |suffix: &str| -> Vec<FireArm> {
        engine
            .rules
            .iter()
            .map(|rule| FireArm {
                constructor: rule.member.name.clone(),
                binding: rule.binding.is_some(),
                theorem: rule.companion(suffix),
                type_arguments: rule.type_arguments.clone(),
            })
            .collect()
    };
    lowering.theorems.push(theorem(
        &engine.own_name("fire_sound"),
        &[],
        fire_parameters.clone(),
        term(implies(fired.clone(), at.relation(var("__s"), var("__t")))),
        Proof::FireCases {
            arms: fire_arms("apply_sound"),
        },
    ));
    if engine.terminates.is_some() {
        if let (Some(before), Some(after)) = (at.rank(var("__s")), at.rank(var("__t"))) {
            lowering.theorems.push(theorem(
                &engine.own_name("fire_progress"),
                &[],
                fire_parameters.clone(),
                term(model::premised(
                    at.invariant(var("__s")),
                    implies(fired.clone(), lt(after, before)),
                )),
                Proof::FireCases {
                    arms: fire_arms("apply_progress"),
                },
            ));
        }
    }
    lowering.theorems.push(theorem(
        &engine.own_name("replay_fire"),
        &[],
        fire_parameters,
        term(implies(
            fired,
            eq(
                engine.call_own("replay", vec![ok_of(&s, var("__s")), var("__step")]),
                ok_of(&s, var("__t")),
            ),
        )),
        Proof::ReplayFire {
            replay: engine.own("replay"),
        },
    ));
    let start = engine.call_own("observe", vec![var(x)]);
    let reaches = |held: SemanticTerm| {
        helper(
            Helper::Reaches,
            vec![at.relation_ref(), start.clone(), held],
        )
    };
    lowering.theorems.push(theorem(
        &engine.own_name("replay_sound"),
        &[],
        vec![
            parameter(x, &engine.input),
            parameter("__acc", &replay_type),
            parameter("__step", &step),
        ],
        entails(
            reaches(var("__acc")),
            reaches(engine.call_own("replay", vec![var("__acc"), var("__step")])),
        ),
        Proof::ReplaySound {
            replay: engine.own("replay"),
            fire_sound: engine.own("fire_sound"),
        },
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("derivation"),
        &[],
        vec![
            parameter(x, &engine.input),
            parameter("__trace", &list_type(step.clone())),
        ],
        reaches(engine.call_own("follow", vec![var(x), var("__trace")])),
        by_term(lemma(
            Lemma::FoldInvariant,
            vec![
                given(function_ref(&engine.own("replay"), &[])),
                predicate("__acc", &replay_type, reaches(var("__acc"))),
                cite(&engine.own("replay_sound"), &[], vec![given(var(x))]),
                given(var("__trace")),
                given(ok_of(&s, start.clone())),
                lemma(
                    Lemma::ReachesStart,
                    vec![given(at.relation_ref()), given(start.clone())],
                ),
            ],
        )),
    ));
    if let (Some(invariant_ref), Some(holds_start)) =
        (at.invariant_ref(), at.invariant(start.clone()))
    {
        let followed = eq(
            engine.call_own("follow", vec![var(x), var("__trace")]),
            ok_of(&s, var("__s")),
        );
        let conclusion = at.invariant(var("__s")).unwrap_or_else(|| boolean(true));
        let preserved = |initial: ProofTerm| {
            lemma(
                Lemma::StarPreserves,
                vec![
                    given(at.relation_ref()),
                    given(invariant_ref.clone()),
                    cite(&at.preserves(), &at.type_arguments, Vec::new()),
                    given(start.clone()),
                    given(var("__s")),
                    cite(
                        &engine.own("derivation"),
                        &[],
                        vec![
                            given(var(x)),
                            given(var("__trace")),
                            given(var("__s")),
                            hypothesis("llE"),
                        ],
                    ),
                    initial,
                ],
            )
        };
        let (statement, proof) = match &engine.initial {
            Some(initial) => (
                implies(followed, conclusion),
                assume(
                    &["llE"],
                    preserved(cite(
                        initial,
                        &parameter_types(engine.type_parameters),
                        vec![given(var(x))],
                    )),
                ),
            ),
            None => (
                implies(holds_start, implies(followed, conclusion)),
                assume(&["llI", "llE"], preserved(hypothesis("llI"))),
            ),
        };
        lowering.theorems.push(theorem(
            &engine.own_name("follow_invariant"),
            &[],
            vec![
                parameter(x, &engine.input),
                parameter("__trace", &list_type(step.clone())),
                parameter("__s", &s),
            ],
            term(statement),
            by_term(proof),
        ));
    }
    let r_type = engine.answer.clone();
    lowering.theorems.push(theorem(
        &engine.own_name("accept_sound"),
        &[],
        vec![
            parameter(x, &engine.input),
            parameter("__s", &s),
            parameter("__v", &r_type),
        ],
        term(implies(
            eq(
                engine.call_own("accept", vec![var(x), var("__s")]),
                some_of(&r_type, var("__v")),
            ),
            engine.spec(var("__v")),
        )),
        match &engine.answer_correct {
            Some(correct) => by_term(assume(
                &["llE"],
                cite(
                    correct,
                    &parameter_types(engine.type_parameters),
                    vec![
                        given(var(x)),
                        given(var("__s")),
                        given(var("__v")),
                        hypothesis("llE"),
                    ],
                ),
            )),
            None => Proof::AcceptSound {
                accept: engine.own("accept"),
                sound: engine.verifier.sound.clone(),
                type_arguments: engine.verifier.type_arguments.clone(),
            },
        },
    ));
}

/// A forward reasoner: fire the first applicable rule, in priority and
/// candidate order, until none applies or the fuel is spent.
#[allow(clippy::too_many_lines)]
fn elaborate_forward(engine: &Engine<'_>, lowering: &mut Lowering) {
    let s = engine.state.clone();
    let step = engine.step_type();
    let x = engine.x;
    let at = &engine.logic;
    let option_step = option_type(step.clone());
    // E.select: the first applicable step. Each rule is tried in declared
    // order and each binding in candidate order; nothing is allocated for a
    // binding-free rule.
    let mut select = none_of(&step);
    for rule in engine.rules.iter().rev() {
        select = match &rule.binding {
            None => if_then(
                call(
                    &rule.companion("guard"),
                    &rule.type_arguments,
                    vec![var("__s")],
                ),
                some_of(&step, engine.step_value(rule, None)),
                select,
            ),
            Some((_, binding)) => matching(
                list_fold(
                    lambda(
                        vec![("__acc", option_step.clone()), ("__b", binding.clone())],
                        matching(
                            var("__acc"),
                            vec![
                                (
                                    "Option.some",
                                    vec!["__found"],
                                    some_of(&step, var("__found")),
                                ),
                                (
                                    "Option.none",
                                    vec![],
                                    if_then(
                                        call(
                                            &rule.companion("guard"),
                                            &rule.type_arguments,
                                            vec![var("__s"), var("__b")],
                                        ),
                                        some_of(&step, engine.step_value(rule, Some(var("__b")))),
                                        none_of(&step),
                                    ),
                                ),
                            ],
                        ),
                    ),
                    none_of(&step),
                    call(
                        &rule.companion("candidates"),
                        &rule.type_arguments,
                        vec![var("__s")],
                    ),
                    option_step.clone(),
                ),
                vec![
                    (
                        "Option.some",
                        vec!["__found"],
                        some_of(&step, var("__found")),
                    ),
                    ("Option.none", vec![], select),
                ],
            ),
        };
    }
    lowering.declarations.push(engine.define(
        "select",
        vec![parameter("__s", &s)],
        option_step.clone(),
        select,
    ));
    // E.attempts: the guard evaluations `select` makes on a state, in the
    // same order, so the ledger accounts every rule tried, not only every
    // rule fired.
    let mut attempts = nat(0);
    for rule in engine.rules.iter().rev() {
        attempts = match &rule.binding {
            None => if_then(
                call(
                    &rule.companion("guard"),
                    &rule.type_arguments,
                    vec![var("__s")],
                ),
                nat(1),
                succ(attempts),
            ),
            Some((_, binding)) => {
                let tried = product(SemanticType::Bool, SemanticType::Nat);
                let acc = var("__acc");
                let_in(
                    "__tried",
                    tried.clone(),
                    list_fold(
                        lambda(
                            vec![("__acc", tried.clone()), ("__b", binding.clone())],
                            if_then(
                                model::first(acc.clone()),
                                acc.clone(),
                                pair(
                                    call(
                                        &rule.companion("guard"),
                                        &rule.type_arguments,
                                        vec![var("__s"), var("__b")],
                                    ),
                                    succ(model::second(acc)),
                                ),
                            ),
                        ),
                        pair(boolean(false), nat(0)),
                        call(
                            &rule.companion("candidates"),
                            &rule.type_arguments,
                            vec![var("__s")],
                        ),
                        tried,
                    ),
                    if_then(
                        model::first(var("__tried")),
                        model::second(var("__tried")),
                        add(model::second(var("__tried")), attempts),
                    ),
                )
            }
        };
    }
    lowering.declarations.push(engine.define(
        "attempts",
        vec![parameter("__s", &s)],
        SemanticType::Nat,
        attempts,
    ));
    lowering.declarations.push(engine.define(
        "next",
        vec![parameter("__s", &s)],
        option_type(s.clone()),
        matching(
            engine.call_own("select", vec![var("__s")]),
            vec![
                ("Option.none", vec![], none_of(&s)),
                (
                    "Option.some",
                    vec!["__step"],
                    engine.call_own("fire", vec![var("__s"), var("__step")]),
                ),
            ],
        ),
    ));
    let observed = engine.call_own("observe", vec![var(x)]);
    lowering.declarations.push(engine.define(
        "saturate",
        vec![parameter(x, &engine.input)],
        product(s.clone(), SemanticType::Bool),
        iterate_until(
            function_ref(&engine.own("next"), &[]),
            engine.fuel.clone(),
            observed.clone(),
            &s,
        ),
    ));
    let run = engine.run_type();
    let ledger = engine.ledger_type();
    lowering.declarations.push(SemanticDeclaration::Structure {
        name: engine.own_name("Run"),
        type_parameters: Vec::new(),
        parameters: Vec::new(),
        fields: vec![
            SemanticField {
                name: "state".to_owned(),
                r#type: s.clone(),
            },
            SemanticField {
                name: "trace".to_owned(),
                r#type: list_type(step.clone()),
            },
            SemanticField {
                name: "ledger".to_owned(),
                r#type: ledger.clone(),
            },
        ],
    });
    lowering.declarations.push(engine.define(
        "start",
        vec![parameter(x, &engine.input)],
        run.clone(),
        record(
            &engine.own_name("Run"),
            vec![
                ("state", observed.clone()),
                ("trace", nil(&step)),
                ("ledger", engine.ledger(Counts::zero(nat(0)))),
            ],
        ),
    ));
    let current = var("__r");
    let counter = |field: &str| project(project(current.clone(), "ledger"), field);
    lowering.declarations.push(engine.define(
        "step",
        vec![parameter("__r", &run)],
        option_type(run.clone()),
        matching(
            engine.call_own("select", vec![project(current.clone(), "state")]),
            vec![
                ("Option.none", vec![], none_of(&run)),
                (
                    "Option.some",
                    vec!["__step"],
                    matching(
                        engine.call_own(
                            "fire",
                            vec![project(current.clone(), "state"), var("__step")],
                        ),
                        vec![
                            ("Option.none", vec![], none_of(&run)),
                            (
                                "Option.some",
                                vec!["__t"],
                                some_of(
                                    &run,
                                    record(
                                        &engine.own_name("Run"),
                                        vec![
                                            ("state", var("__t")),
                                            (
                                                "trace",
                                                append(
                                                    project(current.clone(), "trace"),
                                                    single(&step, var("__step")),
                                                    &step,
                                                ),
                                            ),
                                            (
                                                "ledger",
                                                engine.ledger(Counts {
                                                    iterations: succ(counter("iterations")),
                                                    attempts: add(
                                                        counter("attempts"),
                                                        engine.call_own(
                                                            "attempts",
                                                            vec![project(current.clone(), "state")],
                                                        ),
                                                    ),
                                                    firings: succ(counter("firings")),
                                                    ..Counts::of(&project(
                                                        current.clone(),
                                                        "ledger",
                                                    ))
                                                }),
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
    ));
    lowering.declarations.push(engine.define(
        "run",
        vec![parameter(x, &engine.input)],
        product(run.clone(), SemanticType::Bool),
        iterate_until(
            function_ref(&engine.own("step"), &[]),
            engine.fuel.clone(),
            engine.call_own("start", vec![var(x)]),
            &run,
        ),
    ));
    // E.account: the run's ledger with the actions after its last step:
    // the scan that found no rule applicable, and the one check of the
    // saturated state's answer.
    let final_ledger = project(model::first(var("__final")), "ledger");
    lowering.declarations.push(engine.define(
        "account",
        vec![parameter(x, &engine.input)],
        ledger.clone(),
        let_in(
            "__final",
            product(run.clone(), SemanticType::Bool),
            engine.call_own("run", vec![var(x)]),
            matching(
                model::second(var("__final")),
                vec![
                    (
                        "Bool.true",
                        vec![],
                        engine.ledger(Counts {
                            attempts: add(
                                project(final_ledger.clone(), "attempts"),
                                engine.call_own(
                                    "attempts",
                                    vec![project(model::first(var("__final")), "state")],
                                ),
                            ),
                            verifications: if engine.answer_correct.is_some() {
                                project(final_ledger.clone(), "verifications")
                            } else {
                                succ(project(final_ledger.clone(), "verifications"))
                            },
                            ..Counts::of(&final_ledger)
                        }),
                    ),
                    ("Bool.false", vec![], final_ledger.clone()),
                ],
            ),
        ),
    ));
    // A forward reasoner answers only from a saturated state, so its answer
    // never depends on how much fuel was left: a run cut short by its fuel
    // is `exhausted` whatever its partial state holds.
    let r = engine.answer.clone();
    lowering.declarations.push(engine.define(
        "conclude",
        vec![parameter(x, &engine.input), parameter("__s", &s)],
        result_of(r.clone()),
        matching(
            engine.call_own("accept", vec![var(x), var("__s")]),
            vec![
                ("Option.some", vec!["__v"], ok_of(&r, var("__v"))),
                (
                    "Option.none",
                    vec![],
                    matching(
                        engine.call_own("extract", vec![var("__s")]),
                        vec![
                            (
                                "Option.some",
                                vec!["__v"],
                                error_of(&r, failure_value("rejected")),
                            ),
                            (
                                "Option.none",
                                vec![],
                                error_of(&r, failure_value("unsolved")),
                            ),
                        ],
                    ),
                ),
            ],
        ),
    ));
    let saturated = engine.call_own("saturate", vec![var(x)]);
    lowering.declarations.push(engine.define(
        "verdict",
        vec![parameter(x, &engine.input)],
        result_of(r.clone()),
        let_in(
            "__final",
            product(s.clone(), SemanticType::Bool),
            saturated.clone(),
            matching(
                model::second(var("__final")),
                vec![
                    (
                        "Bool.true",
                        vec![],
                        engine.call_own("conclude", vec![var(x), model::first(var("__final"))]),
                    ),
                    (
                        "Bool.false",
                        vec![],
                        error_of(&r, failure_value("exhausted")),
                    ),
                ],
            ),
        ),
    ));
    let explained = engine.explained_type();
    let final_run = model::first(var("__final"));
    lowering.declarations.push(definition(
        engine.name,
        vec![parameter(x, &engine.input)],
        result_of(explained.clone()),
        let_in(
            "__final",
            product(run.clone(), SemanticType::Bool),
            engine.call_own("run", vec![var(x)]),
            matching(
                model::second(var("__final")),
                vec![
                    (
                        "Bool.true",
                        vec![],
                        matching(
                            engine.call_own(
                                "conclude",
                                vec![var(x), project(final_run.clone(), "state")],
                            ),
                            vec![
                                (
                                    "Result.ok",
                                    vec!["__v"],
                                    ok_of(
                                        &explained,
                                        pair(var("__v"), project(final_run, "trace")),
                                    ),
                                ),
                                (
                                    "Result.error",
                                    vec!["__e"],
                                    error_of(&explained, var("__e")),
                                ),
                            ],
                        ),
                    ),
                    (
                        "Bool.false",
                        vec![],
                        error_of(&explained, failure_value("exhausted")),
                    ),
                ],
            ),
        ),
        engine.executable,
        engine.axioms,
    ));
    // Generated theorems.
    let pair_types = |first: &str, second: &str, ty: &SemanticType| {
        vec![parameter(first, ty), parameter(second, ty)]
    };
    let next_some = eq(
        engine.call_own("next", vec![var("__s")]),
        some_of(&s, var("__t")),
    );
    lowering.theorems.push(theorem(
        &engine.own_name("next_sound"),
        &[],
        pair_types("__s", "__t", &s),
        term(implies(
            next_some.clone(),
            at.relation(var("__s"), var("__t")),
        )),
        Proof::NextSound {
            next: engine.own("next"),
            fire_sound: engine.own("fire_sound"),
        },
    ));
    if let (Some(invariant), Some(holds_s), Some(holds_t)) = (
        at.invariant.as_ref(),
        at.invariant(var("__s")),
        at.invariant(var("__t")),
    ) {
        lowering.theorems.push(theorem(
            &engine.own_name("next_preserves"),
            &[],
            pair_types("__s", "__t", &s),
            term(implies(next_some.clone(), implies(holds_s, holds_t))),
            by_term(assume(
                &["llE", "llH"],
                cite(
                    &invariant.preserves,
                    &at.type_arguments,
                    vec![
                        given(var("__s")),
                        given(var("__t")),
                        hypothesis("llH"),
                        cite(
                            &engine.own("next_sound"),
                            &[],
                            vec![given(var("__s")), given(var("__t")), hypothesis("llE")],
                        ),
                    ],
                ),
            )),
        ));
    }
    if engine.terminates.is_some() {
        if let (Some(before), Some(after)) = (at.rank(var("__s")), at.rank(var("__t"))) {
            lowering.theorems.push(theorem(
                &engine.own_name("next_progress"),
                &[],
                pair_types("__s", "__t", &s),
                term(implies(
                    next_some.clone(),
                    model::premised(at.invariant(var("__s")), lt(after, before)),
                )),
                Proof::NextProgress {
                    next: engine.own("next"),
                    fire_progress: engine.own("fire_progress"),
                    invariant: at.invariant.is_some(),
                },
            ));
        }
    }
    let step_some =
        |q: SemanticTerm| eq(engine.call_own("step", vec![var("__r")]), some_of(&run, q));
    lowering.theorems.push(theorem(
        &engine.own_name("step_none"),
        &[],
        vec![parameter("__r", &run)],
        term(implies(
            eq(engine.call_own("step", vec![var("__r")]), none_of(&run)),
            eq(
                engine.call_own("next", vec![project(var("__r"), "state")]),
                none_of(&s),
            ),
        )),
        Proof::StepNone {
            step: engine.own("step"),
            next: engine.own("next"),
        },
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("step_some"),
        &[],
        pair_types("__r", "__q", &run),
        term(implies(
            step_some(var("__q")),
            eq(
                engine.call_own("next", vec![project(var("__r"), "state")]),
                some_of(&s, project(var("__q"), "state")),
            ),
        )),
        Proof::StepSome {
            step: engine.own("step"),
            next: engine.own("next"),
        },
    ));
    let mut traced = vec![parameter(x, &engine.input)];
    traced.extend(pair_types("__r", "__q", &run));
    lowering.theorems.push(theorem(
        &engine.own_name("step_trace"),
        &[],
        traced,
        term(implies(
            step_some(var("__q")),
            implies(engine.replays(&var("__r")), engine.replays(&var("__q"))),
        )),
        Proof::StepTrace {
            step: engine.own("step"),
            follow: engine.own("follow"),
            replay_fire: engine.own("replay_fire"),
        },
    ));
    let iterations = |value: SemanticTerm| project(project(value, "ledger"), "iterations");
    lowering.theorems.push(theorem(
        &engine.own_name("step_count"),
        &[],
        pair_types("__r", "__q", &run),
        term(implies(
            step_some(var("__q")),
            eq(iterations(var("__q")), add(iterations(var("__r")), nat(1))),
        )),
        Proof::StepCount {
            step: engine.own("step"),
        },
    ));
    let x_only = vec![parameter(x, &engine.input)];
    let saturated_state = model::first(saturated.clone());
    let ran = engine.call_own("run", vec![var(x)]);
    let ran_record = model::first(ran.clone());
    let star = |target: SemanticTerm| {
        helper(
            Helper::Star,
            vec![at.relation_ref(), observed.clone(), target],
        )
    };
    lowering.theorems.push(theorem(
        &engine.own_name("saturate_derivation"),
        &[],
        x_only.clone(),
        star(saturated_state.clone()),
        unfolding(
            vec![engine.own("saturate")],
            lemma(
                Lemma::IterateUntilInvariant,
                vec![
                    given(function_ref(&engine.own("next"), &[])),
                    predicate("__s", &s, star(var("__s"))),
                    assume(
                        &["llA", "llB", "llE", "llH"],
                        lemma(
                            Lemma::StarTail,
                            vec![
                                ProofTerm::Infer,
                                hypothesis("llA"),
                                hypothesis("llB"),
                                hypothesis("llH"),
                                cite(
                                    &engine.own("next_sound"),
                                    &[],
                                    vec![hypothesis("llA"), hypothesis("llB"), hypothesis("llE")],
                                ),
                            ],
                        ),
                    ),
                    given(engine.fuel.clone()),
                    given(observed.clone()),
                    lemma(Lemma::StarRefl, vec![ProofTerm::Infer]),
                ],
            ),
        ),
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("run_state"),
        &[],
        x_only.clone(),
        term(both(
            eq(
                project(ran_record.clone(), "state"),
                saturated_state.clone(),
            ),
            eq(model::second(ran.clone()), model::second(saturated.clone())),
        )),
        unfolding(
            vec![engine.own("run"), engine.own("saturate")],
            lemma(
                Lemma::IterateUntilSimulate,
                vec![
                    given(function_ref(&engine.own("step"), &[])),
                    given(function_ref(&engine.own("next"), &[])),
                    given(lambda(
                        vec![("__r", run.clone())],
                        project(var("__r"), "state"),
                    )),
                    cite(&engine.own("step_none"), &[], Vec::new()),
                    cite(&engine.own("step_some"), &[], Vec::new()),
                    given(engine.fuel.clone()),
                    given(engine.call_own("start", vec![var(x)])),
                ],
            ),
        ),
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("run_trace"),
        &[],
        x_only.clone(),
        term(engine.replays(&ran_record)),
        unfolding(
            vec![engine.own("run")],
            lemma(
                Lemma::IterateUntilInvariant,
                vec![
                    given(function_ref(&engine.own("step"), &[])),
                    predicate("__r", &run, term(engine.replays(&var("__r")))),
                    cite(&engine.own("step_trace"), &[], vec![given(var(x))]),
                    given(engine.fuel.clone()),
                    given(engine.call_own("start", vec![var(x)])),
                    ProofTerm::Refl,
                ],
            ),
        ),
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("iterations_bounded"),
        &[],
        x_only.clone(),
        term(le(iterations(ran_record.clone()), engine.fuel.clone())),
        unfolding(
            vec![engine.own("run")],
            lemma(
                Lemma::IterateUntilCount,
                vec![
                    given(function_ref(&engine.own("step"), &[])),
                    given(lambda(vec![("__r", run.clone())], iterations(var("__r")))),
                    cite(&engine.own("step_count"), &[], Vec::new()),
                    given(engine.fuel.clone()),
                    given(engine.call_own("start", vec![var(x)])),
                    ProofTerm::Refl,
                ],
            ),
        ),
    ));
    if let (Some(invariant_ref), Some(holds_start), Some(holds_final)) = (
        at.invariant_ref(),
        at.invariant(observed.clone()),
        at.invariant(saturated_state.clone()),
    ) {
        let preserved = |initial: ProofTerm| {
            lemma(
                Lemma::IterateUntilInvariant,
                vec![
                    given(function_ref(&engine.own("next"), &[])),
                    given(invariant_ref.clone()),
                    cite(&engine.own("next_preserves"), &[], Vec::new()),
                    given(engine.fuel.clone()),
                    given(observed.clone()),
                    initial,
                ],
            )
        };
        let (statement, proof) = match &engine.initial {
            Some(initial) => (
                holds_final,
                preserved(cite(
                    initial,
                    &parameter_types(engine.type_parameters),
                    vec![given(var(x))],
                )),
            ),
            None => (
                implies(holds_start, holds_final),
                assume(&["llI"], preserved(hypothesis("llI"))),
            ),
        };
        lowering.theorems.push(theorem(
            &engine.own_name("saturate_invariant"),
            &[],
            x_only.clone(),
            term(statement),
            unfolding(vec![engine.own("saturate")], proof),
        ));
    }
    if let (Some(terminates), Some(rank_ref)) = (&engine.terminates, at.rank_ref()) {
        let (invariant, keep, initial) = match (at.invariant_ref(), &engine.initial) {
            (Some(invariant_ref), Some(initial)) => (
                given(invariant_ref),
                cite(
                    &engine.own("next_preserves"),
                    &[],
                    vec![
                        hypothesis("llA"),
                        hypothesis("llB"),
                        hypothesis("llE"),
                        hypothesis("llH"),
                    ],
                ),
                cite(
                    initial,
                    &parameter_types(engine.type_parameters),
                    vec![given(var(x))],
                ),
            ),
            _ => (
                predicate("__s", &s, Formula::True),
                ProofTerm::Trivial,
                ProofTerm::Trivial,
            ),
        };
        let mut progress_arguments = vec![hypothesis("llA"), hypothesis("llB"), hypothesis("llE")];
        if at.invariant.is_some() {
            progress_arguments.push(hypothesis("llH"));
        }
        lowering.theorems.push(theorem(
            &engine.own_name("saturates"),
            &[],
            x_only.clone(),
            term(eq(model::second(saturated.clone()), boolean(true))),
            unfolding(
                vec![engine.own("saturate")],
                lemma(
                    Lemma::IterateUntilStops,
                    vec![
                        given(function_ref(&engine.own("next"), &[])),
                        invariant,
                        given(rank_ref),
                        assume(
                            &[
                                "llA",
                                "llB",
                                "llE",
                                if at.invariant.is_some() {
                                    "llH"
                                } else {
                                    "_llH"
                                },
                            ],
                            ProofTerm::Both {
                                left: Box::new(keep),
                                right: Box::new(cite(
                                    &engine.own("next_progress"),
                                    &[],
                                    progress_arguments,
                                )),
                            },
                        ),
                        given(engine.fuel.clone()),
                        given(observed.clone()),
                        initial,
                        cite(
                            terminates,
                            &parameter_types(engine.type_parameters),
                            vec![given(var(x))],
                        ),
                    ],
                ),
            ),
        ));
    }
    let concluded = eq(
        engine.call_own("conclude", vec![var(x), var("__s")]),
        ok_of(&r, var("__v")),
    );
    let conclude_parameters = vec![
        parameter(x, &engine.input),
        parameter("__s", &s),
        parameter("__v", &r),
    ];
    lowering.theorems.push(theorem(
        &engine.own_name("conclude_accept"),
        &[],
        conclude_parameters.clone(),
        term(implies(
            concluded.clone(),
            eq(
                engine.call_own("accept", vec![var(x), var("__s")]),
                some_of(&r, var("__v")),
            ),
        )),
        Proof::ConcludeAccept {
            conclude: engine.own("conclude"),
        },
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("conclude_sound"),
        &[],
        conclude_parameters,
        term(implies(concluded, engine.spec(var("__v")))),
        Proof::ConcludeSound {
            conclude: engine.own("conclude"),
            accept_sound: engine.own("accept_sound"),
        },
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("verdict_sound"),
        &[],
        vec![parameter(x, &engine.input)],
        term(forall(
            "__v",
            &r,
            implies(
                eq(
                    engine.call_own("verdict", vec![var(x)]),
                    ok_of(&r, var("__v")),
                ),
                engine.spec(var("__v")),
            ),
        )),
        Proof::VerdictForward {
            verdict: engine.own("verdict"),
            saturate: engine.own("saturate"),
            conclude_sound: engine.own("conclude_sound"),
        },
    ));
    explained_theorem(
        engine,
        lowering,
        Proof::ExplainedForward {
            reasoner: engine.me(),
            run: engine.own("run"),
            answer: engine.own("answer"),
            run_trace: engine.own("run_trace"),
            conclude_accept: engine.own("conclude_accept"),
            conclude_sound: engine.own("conclude_sound"),
        },
    );
}

/// `E x = ok (v, trace) -> E.answer x trace = some v /\ Spec x v`: the
/// explained answer replays from its trace and meets the specification.
fn explained_theorem(engine: &Engine<'_>, lowering: &mut Lowering, proof: Proof) {
    let x = engine.x;
    let r = engine.answer.clone();
    let explained = engine.explained_type();
    lowering.theorems.push(theorem(
        &engine.own_name("explained"),
        &[],
        vec![parameter(x, &engine.input)],
        term(forall(
            "__v",
            &r,
            forall(
                "__trace",
                &list_type(engine.step_type()),
                implies(
                    eq(
                        call(&engine.me(), &[], vec![var(x)]),
                        ok_of(&explained, pair(var("__v"), var("__trace"))),
                    ),
                    both(
                        eq(
                            engine.call_own("answer", vec![var(x), var("__trace")]),
                            some_of(&r, var("__v")),
                        ),
                        engine.spec(var("__v")),
                    ),
                ),
            ),
        )),
        proof,
    ));
}

/// A search reasoner: expand a bounded frontier of nodes, breadth- or
/// depth-first, until a verified answer is found, the frontier empties, or
/// the fuel is spent.
#[allow(clippy::too_many_lines)]
fn elaborate_search(engine: &Engine<'_>, lowering: &mut Lowering) {
    let s = engine.state.clone();
    let step = engine.step_type();
    let x = engine.x;
    let node = engine.node_type();
    let nodes = list_type(node.clone());
    let visited_type = SemanticType::Set {
        element: Box::new(s.clone()),
    };
    let frontier_bound = engine.frontier.clone().unwrap_or_else(|| nat(1));
    let r = engine.answer.clone();
    let hit = product(r.clone(), node.clone());
    let ledger = engine.ledger_type();
    let search = engine.search_type();
    lowering.declarations.push(SemanticDeclaration::Structure {
        name: engine.own_name("Node"),
        type_parameters: Vec::new(),
        parameters: Vec::new(),
        fields: vec![
            SemanticField {
                name: "state".to_owned(),
                r#type: s.clone(),
            },
            SemanticField {
                name: "trace".to_owned(),
                r#type: list_type(step.clone()),
            },
        ],
    });
    let mut fields = vec![SemanticField {
        name: "frontier".to_owned(),
        r#type: nodes.clone(),
    }];
    if engine.deduplicate {
        fields.push(SemanticField {
            name: "visited".to_owned(),
            r#type: visited_type.clone(),
        });
    }
    fields.extend([
        SemanticField {
            name: "found".to_owned(),
            r#type: option_type(hit.clone()),
        },
        SemanticField {
            name: "truncated".to_owned(),
            r#type: SemanticType::Bool,
        },
        SemanticField {
            name: "ledger".to_owned(),
            r#type: ledger.clone(),
        },
    ]);
    lowering.declarations.push(SemanticDeclaration::Structure {
        name: engine.own_name("Search"),
        type_parameters: Vec::new(),
        parameters: Vec::new(),
        fields,
    });
    // One successor list per rule, in declared and candidate order, each
    // node extending its parent's trace by the step that produced it.
    let child = |state: SemanticTerm, value: SemanticTerm| {
        record(
            &engine.own_name("Node"),
            vec![
                ("state", state),
                (
                    "trace",
                    append(project(var("__node"), "trace"), single(&step, value), &step),
                ),
            ],
        )
    };
    let parent_state = project(var("__node"), "state");
    let mut successor_lists = Vec::new();
    for rule in &engine.rules {
        let prefix = format!("{}.{}", engine.name, rule.member.name);
        match &rule.binding {
            None => {
                let value = engine.step_value(rule, None);
                lowering.declarations.push(definition(
                    &format!("{prefix}.successors"),
                    vec![parameter("__node", &node)],
                    nodes.clone(),
                    matching(
                        engine.call_own("fire", vec![parent_state.clone(), value.clone()]),
                        vec![
                            ("Option.none", vec![], nil(&node)),
                            (
                                "Option.some",
                                vec!["__t"],
                                single(&node, child(var("__t"), value)),
                            ),
                        ],
                    ),
                    engine.executable,
                    engine.axioms,
                ));
            }
            Some((_, binding)) => {
                let value = engine.step_value(rule, Some(var("__b")));
                lowering.declarations.push(definition(
                    &format!("{prefix}.collect"),
                    vec![
                        parameter("__node", &node),
                        parameter("__acc", &nodes),
                        parameter("__b", binding),
                    ],
                    nodes.clone(),
                    matching(
                        engine.call_own("fire", vec![parent_state.clone(), value.clone()]),
                        vec![
                            ("Option.none", vec![], var("__acc")),
                            (
                                "Option.some",
                                vec!["__t"],
                                append(
                                    var("__acc"),
                                    single(&node, child(var("__t"), value)),
                                    &node,
                                ),
                            ),
                        ],
                    ),
                    engine.executable,
                    engine.axioms,
                ));
                lowering.declarations.push(definition(
                    &format!("{prefix}.successors"),
                    vec![parameter("__node", &node)],
                    nodes.clone(),
                    list_fold(
                        lambda(
                            vec![("__acc", nodes.clone()), ("__b", binding.clone())],
                            call(
                                &local(&format!("{prefix}.collect")),
                                &[],
                                vec![var("__node"), var("__acc"), var("__b")],
                            ),
                        ),
                        nil(&node),
                        call(
                            &rule.companion("candidates"),
                            &rule.type_arguments,
                            vec![parent_state.clone()],
                        ),
                        nodes.clone(),
                    ),
                    engine.executable,
                    engine.axioms,
                ));
            }
        }
        successor_lists.push(call(
            &local(&format!("{prefix}.successors")),
            &[],
            vec![var("__node")],
        ));
    }
    let mut all = nil(&node);
    for list in successor_lists.into_iter().rev() {
        all = append(list, all, &node);
    }
    lowering.declarations.push(engine.define(
        "successors",
        vec![parameter("__node", &node)],
        nodes.clone(),
        all,
    ));
    let fresh_type = product(nodes.clone(), visited_type.clone());
    if engine.deduplicate {
        let acc = var("__acc");
        let key = project(var("__n"), "state");
        lowering.declarations.push(engine.define(
            "fresh",
            vec![
                parameter("__visited", &visited_type),
                parameter("__nodes", &nodes),
            ],
            fresh_type.clone(),
            list_fold(
                lambda(
                    vec![("__acc", fresh_type.clone()), ("__n", node.clone())],
                    if_then(
                        primitive(
                            SemanticPrimitive::SetContains,
                            vec![model::second(acc.clone()), key.clone()],
                            SemanticType::Bool,
                        ),
                        acc.clone(),
                        pair(
                            append(model::first(acc.clone()), single(&node, var("__n")), &node),
                            primitive(
                                SemanticPrimitive::SetInsert,
                                vec![model::second(acc), key],
                                visited_type.clone(),
                            ),
                        ),
                    ),
                ),
                pair(nil(&node), var("__visited")),
                var("__nodes"),
                fresh_type.clone(),
            ),
        ));
    }
    let observed = engine.call_own("observe", vec![var(x)]);
    let root = record(
        &engine.own_name("Node"),
        vec![("state", observed.clone()), ("trace", nil(&step))],
    );
    let search_record = |frontier: SemanticTerm,
                         visited: Option<SemanticTerm>,
                         found: SemanticTerm,
                         truncated: SemanticTerm,
                         ledger_value: SemanticTerm| {
        let mut fields = vec![("frontier", frontier)];
        if let Some(visited) = visited {
            fields.push(("visited", visited));
        }
        fields.extend([
            ("found", found),
            ("truncated", truncated),
            ("ledger", ledger_value),
        ]);
        record(&engine.own_name("Search"), fields)
    };
    lowering.declarations.push(engine.define(
        "start",
        vec![parameter(x, &engine.input)],
        search.clone(),
        let_in(
            "__first",
            nodes.clone(),
            engine.cap(single(&node, root)),
            search_record(
                var("__first"),
                engine.deduplicate.then(|| {
                    primitive(
                        SemanticPrimitive::SetInsert,
                        vec![
                            SemanticTerm::SetLiteral {
                                element: s.clone(),
                                elements: Vec::new(),
                            },
                            observed.clone(),
                        ],
                        visited_type.clone(),
                    )
                }),
                none_of(&hit),
                blt(frontier_bound.clone(), nat(1)),
                engine.ledger(Counts::zero(length(var("__first")))),
            ),
        ),
    ));
    // E.attempts: every guard evaluation expanding a node makes, one per
    // binding-free rule and one per candidate of a binding rule.
    let mut attempts = nat(0);
    for rule in engine.rules.iter().rev() {
        attempts = add(Engine::guard_count(rule, var("__s")), attempts);
    }
    lowering.declarations.push(engine.define(
        "attempts",
        vec![parameter("__s", &s)],
        SemanticType::Nat,
        attempts,
    ));
    // Each popped node's answer is checked once, unless the answer is
    // proved correct and the check erased.
    let checked_once = |count: SemanticTerm| {
        if engine.answer_correct.is_some() {
            count
        } else {
            succ(count)
        }
    };
    let current = var("__r");
    let counter = |field: &str| project(project(current.clone(), "ledger"), field);
    let fresh_nodes = if engine.deduplicate {
        model::first(var("__fresh"))
    } else {
        var("__fresh")
    };
    let ordered = match engine.order {
        SearchOrder::BreadthFirst => append(var("__rest"), fresh_nodes.clone(), &node),
        SearchOrder::DepthFirst => append(fresh_nodes.clone(), var("__rest"), &node),
    };
    let fresh_value = if engine.deduplicate {
        engine.call_own(
            "fresh",
            vec![project(current.clone(), "visited"), var("__successors")],
        )
    } else {
        var("__successors")
    };
    let expanded = let_in(
        "__successors",
        nodes.clone(),
        engine.call_own("successors", vec![var("__node")]),
        let_in(
            "__fresh",
            if engine.deduplicate {
                fresh_type.clone()
            } else {
                nodes.clone()
            },
            fresh_value,
            let_in(
                "__ordered",
                nodes.clone(),
                ordered,
                let_in(
                    "__next",
                    nodes.clone(),
                    engine.cap(var("__ordered")),
                    some_of(
                        &search,
                        search_record(
                            var("__next"),
                            engine.deduplicate.then(|| model::second(var("__fresh"))),
                            none_of(&hit),
                            SemanticTerm::Or {
                                left: Box::new(project(current.clone(), "truncated")),
                                right: Box::new(blt(
                                    frontier_bound.clone(),
                                    length(var("__ordered")),
                                )),
                            },
                            engine.ledger(Counts {
                                iterations: succ(counter("iterations")),
                                attempts: add(
                                    counter("attempts"),
                                    engine.call_own(
                                        "attempts",
                                        vec![project(var("__node"), "state")],
                                    ),
                                ),
                                firings: add(counter("firings"), length(var("__successors"))),
                                expansions: succ(counter("expansions")),
                                verifications: checked_once(counter("verifications")),
                                frontier: if_then(
                                    blt(counter("frontier"), length(var("__next"))),
                                    length(var("__next")),
                                    counter("frontier"),
                                ),
                            }),
                        ),
                    ),
                ),
            ),
        ),
    );
    let accepted = search_record(
        var("__rest"),
        engine
            .deduplicate
            .then(|| project(current.clone(), "visited")),
        some_of(&hit, pair(var("__v"), var("__node"))),
        project(current.clone(), "truncated"),
        engine.ledger(Counts {
            iterations: succ(counter("iterations")),
            verifications: checked_once(counter("verifications")),
            ..Counts::of(&project(current.clone(), "ledger"))
        }),
    );
    lowering.declarations.push(engine.define(
        "searchStep",
        vec![parameter(x, &engine.input), parameter("__r", &search)],
        option_type(search.clone()),
        matching(
            project(current.clone(), "found"),
            vec![
                ("Option.some", vec!["__hit"], none_of(&search)),
                (
                    "Option.none",
                    vec![],
                    matching(
                        project(current.clone(), "frontier"),
                        vec![
                            ("List.nil", vec![], none_of(&search)),
                            (
                                "List.cons",
                                vec!["__node", "__rest"],
                                matching(
                                    engine.call_own(
                                        "accept",
                                        vec![var(x), project(var("__node"), "state")],
                                    ),
                                    vec![
                                        ("Option.some", vec!["__v"], some_of(&search, accepted)),
                                        ("Option.none", vec![], expanded),
                                    ],
                                ),
                            ),
                        ],
                    ),
                ),
            ],
        ),
    ));
    let stepper = lambda(
        vec![("__r", search.clone())],
        engine.call_own("searchStep", vec![var(x), var("__r")]),
    );
    lowering.declarations.push(engine.define(
        "run",
        vec![parameter(x, &engine.input)],
        product(search.clone(), SemanticType::Bool),
        iterate_until(
            stepper.clone(),
            engine.fuel.clone(),
            engine.call_own("start", vec![var(x)]),
            &search,
        ),
    ));
    lowering.declarations.push(engine.define(
        "failure",
        vec![
            parameter("__saturated", &SemanticType::Bool),
            parameter("__truncated", &SemanticType::Bool),
        ],
        SemanticType::ReasoningFailure,
        if_then(
            and_bool(var("__saturated"), not_bool(var("__truncated"))),
            failure_value("unsolved"),
            failure_value("exhausted"),
        ),
    ));
    let explained = engine.explained_type();
    let final_search = model::first(var("__final"));
    lowering.declarations.push(definition(
        engine.name,
        vec![parameter(x, &engine.input)],
        result_of(explained.clone()),
        let_in(
            "__final",
            product(search.clone(), SemanticType::Bool),
            engine.call_own("run", vec![var(x)]),
            matching(
                project(final_search.clone(), "found"),
                vec![
                    (
                        "Option.some",
                        vec!["__hit"],
                        ok_of(
                            &explained,
                            pair(
                                model::first(var("__hit")),
                                project(model::second(var("__hit")), "trace"),
                            ),
                        ),
                    ),
                    (
                        "Option.none",
                        vec![],
                        error_of(
                            &explained,
                            engine.call_own(
                                "failure",
                                vec![
                                    model::second(var("__final")),
                                    project(final_search, "truncated"),
                                ],
                            ),
                        ),
                    ),
                ],
            ),
        ),
        engine.executable,
        engine.axioms,
    ));
    lowering.declarations.push(engine.define(
        "verdict",
        vec![parameter(x, &engine.input)],
        result_of(r.clone()),
        matching(
            call(&engine.me(), &[], vec![var(x)]),
            vec![
                (
                    "Result.ok",
                    vec!["__p"],
                    ok_of(&r, model::first(var("__p"))),
                ),
                ("Result.error", vec!["__e"], error_of(&r, var("__e"))),
            ],
        ),
    ));
    // Generated theorems.
    let replays_node = |value: SemanticTerm| engine.replays(&value);
    let node_ok = predicate("__n", &node, term(replays_node(var("__n"))));
    let node_ok_term = lambda(vec![("__n", node.clone())], replays_node(var("__n")));
    let all_ok = |list: SemanticTerm| helper(Helper::All, vec![node_ok_term.clone(), list]);
    lowering.theorems.push(theorem(
        &engine.own_name("extend"),
        &[],
        vec![
            parameter(x, &engine.input),
            parameter("__node", &node),
            parameter("__step", &step),
            parameter("__t", &s),
        ],
        term(implies(
            replays_node(var("__node")),
            implies(
                eq(
                    engine.call_own("fire", vec![parent_state.clone(), var("__step")]),
                    some_of(&s, var("__t")),
                ),
                eq(
                    engine.call_own(
                        "follow",
                        vec![
                            var(x),
                            append(
                                project(var("__node"), "trace"),
                                single(&step, var("__step")),
                                &step,
                            ),
                        ],
                    ),
                    ok_of(&s, var("__t")),
                ),
            ),
        )),
        Proof::Extend {
            follow: engine.own("follow"),
            replay_fire: engine.own("replay_fire"),
        },
    ));
    let node_parameters = vec![parameter(x, &engine.input), parameter("__node", &node)];
    let mut successor_oks = Vec::new();
    for rule in &engine.rules {
        let prefix = format!("{}.{}", engine.name, rule.member.name);
        let successors = local(&format!("{prefix}.successors"));
        let proof = match rule.binding {
            None => Proof::SuccessorsFree {
                successors: successors.clone(),
                extend: engine.own("extend"),
            },
            Some(_) => Proof::SuccessorsBound {
                successors: successors.clone(),
                collect: local(&format!("{prefix}.collect")),
                extend: engine.own("extend"),
                follow: engine.own("follow"),
                node: node.clone(),
            },
        };
        lowering.theorems.push(theorem(
            &format!("{prefix}.successors_ok"),
            &[],
            node_parameters.clone(),
            entails(
                term(replays_node(var("__node"))),
                all_ok(call(&successors, &[], vec![var("__node")])),
            ),
            proof,
        ));
        successor_oks.push(local(&format!("{prefix}.successors_ok")));
    }
    let mut chain = lemma(Lemma::AllNil, Vec::new());
    for ok in successor_oks.iter().rev() {
        chain = lemma(
            Lemma::AllAppend,
            vec![
                ProofTerm::Infer,
                ProofTerm::Infer,
                ProofTerm::Infer,
                cite(
                    ok,
                    &[],
                    vec![given(var(x)), given(var("__node")), hypothesis("llH")],
                ),
                chain,
            ],
        );
    }
    lowering.theorems.push(theorem(
        &engine.own_name("successors_ok"),
        &[],
        node_parameters,
        entails(
            term(replays_node(var("__node"))),
            all_ok(engine.call_own("successors", vec![var("__node")])),
        ),
        by_term(assume(&["llH"], chain)),
    ));
    if engine.deduplicate {
        lowering.theorems.push(theorem(
            &engine.own_name("fresh_ok"),
            &[],
            vec![
                parameter(x, &engine.input),
                parameter("__visited", &visited_type),
                parameter("__nodes", &nodes),
            ],
            entails(
                all_ok(var("__nodes")),
                all_ok(model::first(
                    engine.call_own("fresh", vec![var("__visited"), var("__nodes")]),
                )),
            ),
            by_term(lemma(
                Lemma::FreshAll,
                vec![
                    given(lambda(
                        vec![("__n", node.clone())],
                        project(var("__n"), "state"),
                    )),
                    node_ok.clone(),
                    given(var("__visited")),
                    given(var("__nodes")),
                ],
            )),
        ));
    }
    let accept_term = lambda(
        vec![("__n", node.clone())],
        engine.call_own("accept", vec![var(x), project(var("__n"), "state")]),
    );
    let search_ok = |value: SemanticTerm| {
        helper(
            Helper::SearchOk,
            vec![
                node_ok_term.clone(),
                accept_term.clone(),
                project(value.clone(), "frontier"),
                project(value, "found"),
            ],
        )
    };
    let stepped = eq(
        engine.call_own("searchStep", vec![var(x), var("__r")]),
        some_of(&search, var("__q")),
    );
    let step_parameters = vec![
        parameter(x, &engine.input),
        parameter("__r", &search),
        parameter("__q", &search),
    ];
    lowering.theorems.push(theorem(
        &engine.own_name("search_step"),
        &[],
        step_parameters.clone(),
        entails(
            term(stepped.clone()),
            entails(search_ok(var("__r")), search_ok(var("__q"))),
        ),
        Proof::SearchStep {
            search_step: engine.own("searchStep"),
            successors_ok: engine.own("successors_ok"),
            fresh_ok: engine.deduplicate.then(|| engine.own("fresh_ok")),
            order: engine.order,
        },
    ));
    let peak = |value: SemanticTerm| project(project(value, "ledger"), "frontier");
    lowering.theorems.push(theorem(
        &engine.own_name("search_peak"),
        &[],
        step_parameters.clone(),
        term(implies(
            stepped.clone(),
            implies(
                le(peak(var("__r")), frontier_bound.clone()),
                le(peak(var("__q")), frontier_bound.clone()),
            ),
        )),
        Proof::SearchPeak {
            search_step: engine.own("searchStep"),
        },
    ));
    let iterations = |value: SemanticTerm| project(project(value, "ledger"), "iterations");
    lowering.theorems.push(theorem(
        &engine.own_name("search_count"),
        &[],
        step_parameters,
        term(implies(
            stepped,
            eq(iterations(var("__q")), add(iterations(var("__r")), nat(1))),
        )),
        Proof::SearchCount {
            search_step: engine.own("searchStep"),
        },
    ));
    let x_only = vec![parameter(x, &engine.input)];
    let ran = model::first(engine.call_own("run", vec![var(x)]));
    let start = engine.call_own("start", vec![var(x)]);
    let root_value = record(
        &engine.own_name("Node"),
        vec![("state", observed), ("trace", nil(&step))],
    );
    lowering.theorems.push(theorem(
        &engine.own_name("search_ok"),
        &[],
        x_only.clone(),
        search_ok(ran.clone()),
        unfolding(
            vec![engine.own("run")],
            lemma(
                Lemma::IterateUntilInvariant,
                vec![
                    given(stepper.clone()),
                    predicate("__r", &search, search_ok(var("__r"))),
                    cite(&engine.own("search_step"), &[], vec![given(var(x))]),
                    given(engine.fuel.clone()),
                    given(start.clone()),
                    lemma(
                        Lemma::SearchStart,
                        vec![
                            given(node_ok_term.clone()),
                            given(accept_term.clone()),
                            ProofTerm::Infer,
                            lemma(
                                Lemma::CapAll,
                                vec![
                                    given(node_ok_term.clone()),
                                    given(frontier_bound.clone()),
                                    ProofTerm::Infer,
                                    lemma(
                                        Lemma::AllSingle,
                                        vec![
                                            given(node_ok_term.clone()),
                                            given(root_value),
                                            ProofTerm::Refl,
                                        ],
                                    ),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ),
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("frontier_bounded"),
        &[],
        x_only.clone(),
        term(le(peak(ran.clone()), frontier_bound.clone())),
        unfolding(
            vec![engine.own("run")],
            lemma(
                Lemma::IterateUntilBound,
                vec![
                    given(stepper.clone()),
                    given(lambda(vec![("__r", search.clone())], peak(var("__r")))),
                    given(frontier_bound.clone()),
                    cite(&engine.own("search_peak"), &[], vec![given(var(x))]),
                    given(engine.fuel.clone()),
                    given(start.clone()),
                    lemma(
                        Lemma::CapBound,
                        vec![given(frontier_bound.clone()), ProofTerm::Infer],
                    ),
                ],
            ),
        ),
    ));
    lowering.theorems.push(theorem(
        &engine.own_name("iterations_bounded"),
        &[],
        x_only,
        term(le(iterations(ran), engine.fuel.clone())),
        unfolding(
            vec![engine.own("run")],
            lemma(
                Lemma::IterateUntilCount,
                vec![
                    given(stepper),
                    given(lambda(
                        vec![("__r", search.clone())],
                        iterations(var("__r")),
                    )),
                    cite(&engine.own("search_count"), &[], vec![given(var(x))]),
                    given(engine.fuel.clone()),
                    given(start),
                    ProofTerm::Refl,
                ],
            ),
        ),
    ));
    explained_theorem(
        engine,
        lowering,
        Proof::ExplainedSearch {
            reasoner: engine.me(),
            run: engine.own("run"),
            answer: engine.own("answer"),
            search_ok: engine.own("search_ok"),
            accept_sound: engine.own("accept_sound"),
        },
    );
    lowering.theorems.push(theorem(
        &engine.own_name("verdict_sound"),
        &[],
        vec![parameter(x, &engine.input)],
        term(forall(
            "__v",
            &r,
            implies(
                eq(
                    engine.call_own("verdict", vec![var(x)]),
                    ok_of(&r, var("__v")),
                ),
                engine.spec(var("__v")),
            ),
        )),
        Proof::VerdictSearch {
            reasoner: engine.me(),
            verdict: engine.own("verdict"),
            explained: engine.own("explained"),
        },
    ));
}

// ---------------------------------------------------------------------------
// One reasoning declaration.

/// Check one reasoning declaration, elaborate it, check each elaborated
/// declaration by the ordinary rules, and register each generated theorem.
pub(super) fn check_declaration(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
    generated_names: &mut BTreeSet<String>,
) -> Result<Lowering, SemanticFailure> {
    let kind = declaration_construct(declaration).unwrap_or("declaration");
    super::require_language_1_2(env, kind)?;
    let lowering = match declaration {
        SemanticDeclaration::Logic { .. } => check_logic(declaration, env)?,
        SemanticDeclaration::InferenceRule { .. } => check_rule(declaration, env)?,
        SemanticDeclaration::Verifier { .. } => check_verifier(declaration, env)?,
        SemanticDeclaration::Reasoner { .. } => check_reasoner(declaration, env)?,
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. } => Lowering::default(),
    };
    let name = declaration.name();
    for derived in &lowering.declarations {
        let derived_name = derived.name();
        if derived_name != name {
            super::check_declaration_name(derived_name, env)?;
            if !generated_names.insert(derived_name.to_owned()) {
                return Err(format!("duplicate generated name `{derived_name}`").into());
            }
        }
        env.derived = true;
        let checked = match derived {
            SemanticDeclaration::Definition { .. } => super::check_definition(derived, env),
            SemanticDeclaration::Inductive {
                name: inductive,
                type_parameters,
                parameters,
                constructors,
                ..
            } => super::register_inductive_group(
                &[super::InductiveRow {
                    name: inductive,
                    type_parameters,
                    parameters,
                    constructors,
                }],
                None,
                env,
                generated_names,
            ),
            SemanticDeclaration::Structure { .. } => {
                super::register_structure(derived, env, generated_names)
            }
            _ => Err(format!(
                "internal: `{derived_name}` is not a definition, inductive, or structure"
            )),
        };
        env.derived = false;
        env.current_type_parameters.clear();
        checked.map_err(|reason| {
            format!("{kind} `{name}` elaborates to `{derived_name}`, which is ill-formed: {reason}")
        })?;
    }
    for generated in &lowering.theorems {
        if !generated_names.insert(generated.name.clone()) {
            return Err(format!("duplicate generated name `{}`", generated.name).into());
        }
        env.derived = true;
        let checked = check_generated(generated, env);
        env.derived = false;
        checked.map_err(|reason| {
            format!(
                "{kind} `{name}` generates `{}`, which is ill-formed: {reason}",
                generated.name
            )
        })?;
    }
    Ok(lowering)
}

/// Check a generated theorem's signature and every term of its statement,
/// then register one that states an ordinary proposition as a prior
/// theorem. Its proof is one fixed template, checked by Lean.
fn check_generated(generated: &GeneratedTheorem, env: &mut Environment<'_>) -> Result<(), String> {
    let scope = type_parameter_set(&generated.type_parameters)?;
    let locals = super::check_parameters(&generated.parameters, env, &scope)?;
    let typed = super::typed_locals(&generated.parameters);
    let mut terms = Vec::new();
    formula_terms(&generated.statement, &mut terms);
    for value in &terms {
        check_term_type_parameters(value, &scope)?;
    }
    if let Some(statement) = generated.ordinary_statement() {
        check_term(statement, &locals, env, None, &BTreeSet::new())?;
        super::require_type(
            infer_term(statement, &typed, env)?,
            &SemanticType::Prop,
            &format!("generated theorem `{}` statement", generated.name),
        )?;
        env.theorems.insert(
            generated.name.clone(),
            (
                generated.type_parameters.clone(),
                generated.parameters.clone(),
                statement.clone(),
            ),
        );
        env.proof_type_parameters
            .insert(generated.name.clone(), generated.type_parameters.clone());
        env.proof_rules.insert(
            generated.name.clone(),
            generated
                .parameters
                .iter()
                .map(|parameter| parameter.r#type.clone())
                .collect(),
        );
    } else {
        for value in &terms {
            check_term(value, &locals, env, None, &BTreeSet::new())?;
        }
    }
    Ok(())
}

/// Every term a generated statement carries, outermost first.
pub(crate) fn formula_terms<'a>(formula: &'a Formula, out: &mut Vec<&'a SemanticTerm>) {
    match formula {
        Formula::Term { term } => out.push(term),
        Formula::Helper { arguments, .. } => out.extend(arguments.iter()),
        Formula::Implies {
            premise,
            conclusion,
        } => {
            formula_terms(premise, out);
            formula_terms(conclusion, out);
        }
        Formula::True => {}
    }
}

/// The nodes a generated theorem charges to `max_ir_nodes`: its parameter
/// types, every statement term, and one per formula connective.
pub(crate) fn theorem_node_count(generated: &GeneratedTheorem) -> u64 {
    fn formula(value: &Formula) -> u64 {
        match value {
            Formula::Term { term } => super::term_node_count(term),
            Formula::Helper { arguments, .. } => {
                1 + arguments.iter().map(super::term_node_count).sum::<u64>()
            }
            Formula::Implies {
                premise,
                conclusion,
            } => 1 + formula(premise) + formula(conclusion),
            Formula::True => 1,
        }
    }
    1 + generated
        .parameters
        .iter()
        .map(|parameter| super::type_node_count(&parameter.r#type))
        .sum::<u64>()
        + formula(&generated.statement)
}
