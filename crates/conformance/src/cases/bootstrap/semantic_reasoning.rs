//! `BS-72` (SPEC.md §17.23, §17.12): the declarations of logics, inference
//! rules, verifiers, reasoners, tokenizers, context windows, text generators,
//! and text classifiers of the semantic module language, and the semantic
//! module of any declarations, as language-1.2 source,
//! `LexLeanBoot.SemanticReasoning`, held to the byte to the serde form of those
//! variants of `lexlean::ir::semantic::SemanticDeclaration` and of
//! `SemanticModule` in canonical JSON, as the linked IR embeds them.
//!
//! `reasoningDeclarationText` writes one such declaration with every datatype
//! it holds; `semanticModuleText` writes a module of declarations of every
//! kind, the ordinary ones through `SemanticDecls`, the model ones through
//! `SemanticModels`, and these. The module, its encoders here, and the example
//! builders here are generated from one description of the oracle's types.
//! Every such declaration and every semantic module of every committed project
//! that links is a run, and so are examples that reach every variant.
//!
//! Each planted defect renames one serialized kind or name and is detected by
//! the first example that holds it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::ir::semantic::{
    GenerationPolicy, LogicInvariant, MemberRef, OverflowDisposition, ReasoningAnswer,
    ReasoningClaim, ReasoningStrategy, RuleBinding, SamplingDistribution, SamplingGenerator,
    SearchOrder, SemanticDeclaration, SemanticModule, SemanticTerm, SemanticType, TokenizerScheme,
};
use serde_json::Value as Json2;

use super::ir_document::linked_projects;
use super::rust_validate::{eligibility_of, rustc_differential};
use super::semantic_decls::{enc_declaration, is_ordinary};
use super::semantic_models::{
    enc_contract_state, enc_model_binder, enc_model_declaration, enc_model_use,
    example_contract_state, example_model_binder, example_model_use, is_model,
};
use super::semantic_terms::enc_term;
use super::semantic_types::{canonical, enc_type};
use super::{list, option, pair, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SemanticReasoning";
const SOURCE: &str = "bootstrap/src/SemanticReasoning.lex.tex";
const ROOTS: [(&str, &[&str]); 2] = [
    (
        "reasoningDeclarationText",
        &["allocation", "overflow", "recursion"],
    ),
    (
        "wholeSemanticModuleText",
        &["allocation", "overflow", "recursion"],
    ),
];
/// A committed module whose JSON is longer runs under rustc only.
const EVALUATOR_BYTES: usize = 262_144;

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The examples: one per step, the step's digits choosing every variant.
const EXAMPLES: usize = 150;
/// Seeded examples under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 100;
const RUSTC_SEEDED: usize = 500;

// ---------------------------------------------------------------- the sorts

fn constructors(sort: &str) -> &'static Vec<String> {
    static TABLES: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    let tables = TABLES.get_or_init(|| {
        let source = std::fs::read_to_string(repo_root().join(SOURCE).as_std_path())
            .expect("the module's source");
        let mut out = BTreeMap::new();
        for block in source.split("inductive{").skip(1) {
            let block = block.split("\n}\n").next().expect("a block");
            let name = block
                .split("name: ")
                .nth(1)
                .and_then(|rest| rest.split(',').next())
                .expect("the inductive's name")
                .trim()
                .to_owned();
            let names: Vec<String> = block
                .split("constructors:")
                .nth(1)
                .expect("the constructors")
                .split("name: ")
                .skip(1)
                .map(|rest| {
                    rest.split([',', '\n'])
                        .next()
                        .expect("a constructor")
                        .trim()
                        .to_owned()
                })
                .collect();
            out.insert(name, names);
        }
        out
    });
    tables
        .get(sort)
        .unwrap_or_else(|| panic!("the module has no inductive `{sort}`"))
}

fn adt(sort: &str, constructor: &str, fields: Vec<Value>) -> Value {
    let found = constructors(sort)
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`{sort}` has no constructor `{constructor}`"));
    Value::Adt {
        constructor: u64::try_from(found).expect("an index"),
        fields,
    }
}

fn nat(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

fn enc_member(member: &MemberRef) -> Value {
    pair(
        option(member.module.as_deref().map(text)),
        text(&member.name),
    )
}

// ------------------------------------------- generated from the type schema

pub(super) fn enc_logic_invariant(value: &LogicInvariant) -> Value {
    adt(
        "LogicInvariant",
        "LogicInvariantOf",
        vec![enc_member(&value.predicate), enc_member(&value.preserves)],
    )
}

pub(super) fn example_logic_invariant(step: usize) -> LogicInvariant {
    LogicInvariant {
        predicate: MemberRef {
            module: step.is_multiple_of(2).then(|| "M".to_owned()),
            name: format!("m{step}"),
        },
        preserves: MemberRef {
            module: step.is_multiple_of(2).then(|| "M".to_owned()),
            name: format!("m{step}"),
        },
    }
}

pub(super) fn enc_rule_binding(value: &RuleBinding) -> Value {
    adt(
        "RuleBinding",
        "RuleBindingOf",
        vec![
            text(&value.name),
            enc_type(&value.r#type),
            enc_term(&value.candidates),
        ],
    )
}

pub(super) fn example_rule_binding(step: usize) -> RuleBinding {
    RuleBinding {
        name: format!("s{step}"),
        r#type: if step.is_multiple_of(2) {
            SemanticType::Nat
        } else {
            SemanticType::Bytes
        },
        candidates: SemanticTerm::Nat {
            value: step.to_string(),
        },
    }
}

pub(super) fn enc_reasoning_answer(value: &ReasoningAnswer) -> Value {
    adt(
        "ReasoningAnswer",
        "ReasoningAnswerOf",
        vec![
            text(&value.name),
            enc_type(&value.r#type),
            enc_term(&value.value),
        ],
    )
}

pub(super) fn example_reasoning_answer(step: usize) -> ReasoningAnswer {
    ReasoningAnswer {
        name: format!("s{step}"),
        r#type: if step.is_multiple_of(2) {
            SemanticType::Nat
        } else {
            SemanticType::Bytes
        },
        value: SemanticTerm::Nat {
            value: step.to_string(),
        },
    }
}

pub(super) fn enc_search_order(value: &SearchOrder) -> Value {
    adt("SearchOrder", &format!("SearchOrder{value:?}"), Vec::new())
}

pub(super) const ALL_SEARCH_ORDER: &[SearchOrder] =
    &[SearchOrder::BreadthFirst, SearchOrder::DepthFirst];

pub(super) fn example_search_order(step: usize) -> SearchOrder {
    ALL_SEARCH_ORDER[step % ALL_SEARCH_ORDER.len()]
}

pub(super) fn enc_reasoning_strategy(value: &ReasoningStrategy) -> Value {
    match value {
        ReasoningStrategy::Forward { fuel } => adt(
            "ReasoningStrategy",
            "ReasoningStrategyForward",
            vec![option(fuel.as_ref().map(enc_term))],
        ),
        ReasoningStrategy::Search {
            order,
            fuel,
            frontier,
            deduplicate,
        } => adt(
            "ReasoningStrategy",
            "ReasoningStrategySearch",
            vec![
                enc_search_order(order),
                option(fuel.as_ref().map(enc_term)),
                option(frontier.as_ref().map(enc_term)),
                Value::Bool {
                    value: *deduplicate,
                },
            ],
        ),
        ReasoningStrategy::GenerateAndVerify { generator, budget } => adt(
            "ReasoningStrategy",
            "ReasoningStrategyGenerateAndVerify",
            vec![enc_term(generator), option(budget.as_ref().map(enc_term))],
        ),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_reasoning_strategy(step: usize) -> ReasoningStrategy {
    let rest = step / 3;
    match step % 3 {
        0 => ReasoningStrategy::Forward {
            fuel: (rest % 3 != 2).then(|| SemanticTerm::Nat {
                value: rest.to_string(),
            }),
        },
        1 => ReasoningStrategy::Search {
            order: example_search_order(rest),
            fuel: (rest % 3 != 2).then(|| SemanticTerm::Nat {
                value: rest.to_string(),
            }),
            frontier: (rest % 3 != 2).then(|| SemanticTerm::Nat {
                value: rest.to_string(),
            }),
            deduplicate: rest.is_multiple_of(2),
        },
        _ => ReasoningStrategy::GenerateAndVerify {
            generator: SemanticTerm::Nat {
                value: rest.to_string(),
            },
            budget: (rest % 3 != 2).then(|| SemanticTerm::Nat {
                value: rest.to_string(),
            }),
        },
    }
}

pub(super) fn enc_reasoning_claim(value: &ReasoningClaim) -> Value {
    match value {
        ReasoningClaim::InitialInvariant { theorem } => adt(
            "ReasoningClaim",
            "ReasoningClaimInitialInvariant",
            vec![enc_member(theorem)],
        ),
        ReasoningClaim::Terminates { theorem } => adt(
            "ReasoningClaim",
            "ReasoningClaimTerminates",
            vec![enc_member(theorem)],
        ),
        ReasoningClaim::ObservationInvariant {
            predicate,
            initial,
            preserved,
        } => adt(
            "ReasoningClaim",
            "ReasoningClaimObservationInvariant",
            vec![
                enc_member(predicate),
                enc_member(initial),
                enc_member(preserved),
            ],
        ),
        ReasoningClaim::AnswerCorrect { theorem } => adt(
            "ReasoningClaim",
            "ReasoningClaimAnswerCorrect",
            vec![enc_member(theorem)],
        ),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_reasoning_claim(step: usize) -> ReasoningClaim {
    let rest = step / 4;
    match step % 4 {
        0 => ReasoningClaim::InitialInvariant {
            theorem: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
        },
        1 => ReasoningClaim::Terminates {
            theorem: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
        },
        2 => ReasoningClaim::ObservationInvariant {
            predicate: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            initial: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            preserved: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
        },
        _ => ReasoningClaim::AnswerCorrect {
            theorem: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
        },
    }
}

pub(super) fn enc_tokenizer_scheme(value: &TokenizerScheme) -> Value {
    adt(
        "TokenizerScheme",
        &format!("TokenizerScheme{value:?}"),
        Vec::new(),
    )
}

pub(super) const ALL_TOKENIZER_SCHEME: &[TokenizerScheme] =
    &[TokenizerScheme::ByteLevel, TokenizerScheme::LongestMatch];

pub(super) fn example_tokenizer_scheme(step: usize) -> TokenizerScheme {
    ALL_TOKENIZER_SCHEME[step % ALL_TOKENIZER_SCHEME.len()]
}

pub(super) fn enc_overflow_disposition(value: &OverflowDisposition) -> Value {
    adt(
        "OverflowDisposition",
        &format!("OverflowDisposition{value:?}"),
        Vec::new(),
    )
}

pub(super) const ALL_OVERFLOW_DISPOSITION: &[OverflowDisposition] =
    &[OverflowDisposition::Refuse, OverflowDisposition::KeepLast];

pub(super) fn example_overflow_disposition(step: usize) -> OverflowDisposition {
    ALL_OVERFLOW_DISPOSITION[step % ALL_OVERFLOW_DISPOSITION.len()]
}

pub(super) fn enc_sampling_generator(value: &SamplingGenerator) -> Value {
    adt(
        "SamplingGenerator",
        &format!("SamplingGenerator{value:?}"),
        Vec::new(),
    )
}

pub(super) const ALL_SAMPLING_GENERATOR: &[SamplingGenerator] = &[SamplingGenerator::Xorshift32];

pub(super) fn example_sampling_generator(step: usize) -> SamplingGenerator {
    ALL_SAMPLING_GENERATOR[step % ALL_SAMPLING_GENERATOR.len()]
}

pub(super) fn enc_sampling_distribution(value: &SamplingDistribution) -> Value {
    adt(
        "SamplingDistribution",
        &format!("SamplingDistribution{value:?}"),
        Vec::new(),
    )
}

pub(super) const ALL_SAMPLING_DISTRIBUTION: &[SamplingDistribution] =
    &[SamplingDistribution::Proportional];

pub(super) fn example_sampling_distribution(step: usize) -> SamplingDistribution {
    ALL_SAMPLING_DISTRIBUTION[step % ALL_SAMPLING_DISTRIBUTION.len()]
}

pub(super) fn enc_generation_policy(value: &GenerationPolicy) -> Value {
    match value {
        GenerationPolicy::Greedy => adt("GenerationPolicy", "GenerationPolicyGreedy", vec![]),
        GenerationPolicy::Sample {
            generator,
            distribution,
        } => adt(
            "GenerationPolicy",
            "GenerationPolicySample",
            vec![
                enc_sampling_generator(generator),
                enc_sampling_distribution(distribution),
            ],
        ),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_generation_policy(step: usize) -> GenerationPolicy {
    let rest = step / 2;
    match step % 2 {
        0 => GenerationPolicy::Greedy,
        _ => GenerationPolicy::Sample {
            generator: example_sampling_generator(rest),
            distribution: example_sampling_distribution(rest),
        },
    }
}

pub(super) fn enc_reasoning_declaration(value: &SemanticDeclaration) -> Value {
    match value {
        SemanticDeclaration::Logic {
            name,
            type_parameters,
            state,
            relation,
            invariant,
            ranking,
            axioms,
        } => adt(
            "ReasoningDeclaration",
            "ReasoningDeclarationLogic",
            vec![
                text(name),
                list(type_parameters.iter().map(|item| text(item)).collect()),
                enc_contract_state(state),
                enc_member(relation),
                option(invariant.as_ref().map(enc_logic_invariant)),
                option(ranking.as_ref().map(enc_member)),
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        SemanticDeclaration::InferenceRule {
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
        } => adt(
            "ReasoningDeclaration",
            "ReasoningDeclarationInferenceRule",
            vec![
                text(name),
                list(type_parameters.iter().map(|item| text(item)).collect()),
                enc_model_use(logic),
                option(binding.as_ref().map(enc_rule_binding)),
                enc_term(guard),
                enc_term(conclusion),
                enc_member(soundness),
                option(progress.as_ref().map(enc_member)),
                Value::Bool { value: *executable },
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        SemanticDeclaration::Verifier {
            name,
            type_parameters,
            subject,
            candidate,
            specification,
            check,
            sound,
            complete,
            axioms,
        } => adt(
            "ReasoningDeclaration",
            "ReasoningDeclarationVerifier",
            vec![
                text(name),
                list(type_parameters.iter().map(|item| text(item)).collect()),
                enc_model_binder(subject),
                enc_model_binder(candidate),
                enc_member(specification),
                enc_member(check),
                enc_member(sound),
                option(complete.as_ref().map(enc_member)),
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        SemanticDeclaration::Reasoner {
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
        } => adt(
            "ReasoningDeclaration",
            "ReasoningDeclarationReasoner",
            vec![
                text(name),
                list(type_parameters.iter().map(|item| text(item)).collect()),
                option(logic.as_ref().map(enc_model_use)),
                enc_model_binder(observation),
                option(observe.as_ref().map(enc_term)),
                list(rules.iter().map(enc_model_use).collect()),
                enc_reasoning_strategy(strategy),
                option(answer.as_ref().map(enc_reasoning_answer)),
                enc_model_use(verifier),
                list(claims.iter().map(enc_reasoning_claim).collect()),
                Value::Bool { value: *executable },
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        SemanticDeclaration::Tokenizer {
            name,
            type_parameters,
            scheme,
            vocabulary,
            axioms,
        } => adt(
            "ReasoningDeclaration",
            "ReasoningDeclarationTokenizer",
            vec![
                text(name),
                list(type_parameters.iter().map(|item| text(item)).collect()),
                enc_tokenizer_scheme(scheme),
                enc_model_use(vocabulary),
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        SemanticDeclaration::ContextWindow {
            name,
            capacity,
            overflow,
            axioms,
        } => adt(
            "ReasoningDeclaration",
            "ReasoningDeclarationContextWindow",
            vec![
                text(name),
                nat(*capacity),
                enc_overflow_disposition(overflow),
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        SemanticDeclaration::TextGenerator {
            name,
            tokenizer,
            window,
            scorer,
            max_tokens,
            stop,
            policy,
            axioms,
        } => adt(
            "ReasoningDeclaration",
            "ReasoningDeclarationTextGenerator",
            vec![
                text(name),
                enc_member(tokenizer),
                enc_member(window),
                enc_model_use(scorer),
                nat(*max_tokens),
                enc_member(stop),
                enc_generation_policy(policy),
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        SemanticDeclaration::TextClassifier {
            name,
            tokenizer,
            model,
            axioms,
        } => adt(
            "ReasoningDeclaration",
            "ReasoningDeclarationTextClassifier",
            vec![
                text(name),
                enc_member(tokenizer),
                enc_model_use(model),
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        other => panic!("not a declaration of this module: {other:?}"),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_reasoning_declaration(step: usize) -> SemanticDeclaration {
    let rest = step / 8;
    match step % 8 {
        0 => SemanticDeclaration::Logic {
            name: format!("s{rest}"),
            type_parameters: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
            state: example_contract_state(rest),
            relation: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            invariant: (rest % 3 != 2).then(|| example_logic_invariant(rest)),
            ranking: (rest % 3 != 2).then(|| MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            }),
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
        1 => SemanticDeclaration::InferenceRule {
            name: format!("s{rest}"),
            type_parameters: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
            logic: example_model_use(rest),
            binding: (rest % 3 != 2).then(|| example_rule_binding(rest)),
            guard: SemanticTerm::Nat {
                value: rest.to_string(),
            },
            conclusion: SemanticTerm::Nat {
                value: rest.to_string(),
            },
            soundness: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            progress: (rest % 3 != 2).then(|| MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            }),
            executable: rest.is_multiple_of(2),
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
        2 => SemanticDeclaration::Verifier {
            name: format!("s{rest}"),
            type_parameters: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
            subject: example_model_binder(rest),
            candidate: example_model_binder(rest),
            specification: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            check: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            sound: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            complete: (rest % 3 != 2).then(|| MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            }),
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
        3 => SemanticDeclaration::Reasoner {
            name: format!("s{rest}"),
            type_parameters: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
            logic: (rest % 3 != 2).then(|| example_model_use(rest)),
            observation: example_model_binder(rest),
            observe: (rest % 3 != 2).then(|| SemanticTerm::Nat {
                value: rest.to_string(),
            }),
            rules: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(example_model_use).collect()
            },
            strategy: example_reasoning_strategy(rest),
            answer: (rest % 3 != 2).then(|| example_reasoning_answer(rest)),
            verifier: example_model_use(rest),
            claims: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(example_reasoning_claim).collect()
            },
            executable: rest.is_multiple_of(2),
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
        4 => SemanticDeclaration::Tokenizer {
            name: format!("s{rest}"),
            type_parameters: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
            scheme: example_tokenizer_scheme(rest),
            vocabulary: example_model_use(rest),
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
        5 => SemanticDeclaration::ContextWindow {
            name: format!("s{rest}"),
            capacity: u64::try_from(rest).expect("a step") + 1,
            overflow: example_overflow_disposition(rest),
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
        6 => SemanticDeclaration::TextGenerator {
            name: format!("s{rest}"),
            tokenizer: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            window: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            scorer: example_model_use(rest),
            max_tokens: u64::try_from(rest).expect("a step") + 1,
            stop: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            policy: example_generation_policy(rest),
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
        _ => SemanticDeclaration::TextClassifier {
            name: format!("s{rest}"),
            tokenizer: MemberRef {
                module: rest.is_multiple_of(2).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            model: example_model_use(rest),
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
    }
}

/// Every serialized kind of a tagged variant.
const KINDS: &[&str] = &[
    "forward",
    "search",
    "generate_and_verify",
    "initial_invariant",
    "terminates",
    "observation_invariant",
    "answer_correct",
    "greedy",
    "sample",
    "logic",
    "inference_rule",
    "verifier",
    "reasoner",
    "tokenizer",
    "context_window",
    "text_generator",
    "text_classifier",
];

// ------------------------------------------------------------- the inputs

fn is_reasoning(declaration: &SemanticDeclaration) -> bool {
    !is_ordinary(declaration) && !is_model(declaration)
}

pub(super) fn enc_any(declaration: &SemanticDeclaration) -> Value {
    if is_ordinary(declaration) {
        adt(
            "AnyDeclaration",
            "AnyOrdinary",
            vec![enc_declaration(declaration)],
        )
    } else if is_model(declaration) {
        adt(
            "AnyDeclaration",
            "AnyModel",
            vec![enc_model_declaration(declaration)],
        )
    } else {
        adt(
            "AnyDeclaration",
            "AnyReasoning",
            vec![enc_reasoning_declaration(declaration)],
        )
    }
}

fn module_run(module: &SemanticModule) -> Run {
    (
        vec![
            text(&module.spec),
            list(module.declarations.iter().map(enc_any).collect()),
        ],
        serde_json::to_value(text(&canonical(module))).expect("a value is JSON"),
    )
}

/// Every distinct semantic module of every committed project that links.
fn committed_modules() -> Vec<SemanticModule> {
    let mut out = BTreeMap::new();
    for project in linked_projects() {
        for module in project.modules.values() {
            if let Some(semantic) = &module.semantic {
                out.insert(canonical(semantic), semantic.clone());
            }
        }
    }
    out.into_values().collect()
}

fn run(declaration: &SemanticDeclaration) -> Run {
    (
        vec![enc_reasoning_declaration(declaration)],
        serde_json::to_value(text(&canonical(declaration))).expect("a value is JSON"),
    )
}

/// Every distinct reasoning or text declaration of every committed semantic
/// module that links.
fn committed() -> Vec<SemanticDeclaration> {
    let mut out = BTreeMap::new();
    for project in linked_projects() {
        for module in project.modules.values() {
            if let Some(semantic) = &module.semantic {
                for declaration in semantic
                    .declarations
                    .iter()
                    .filter(|item| is_reasoning(item))
                {
                    out.insert(canonical(declaration), declaration.clone());
                }
            }
        }
    }
    out.into_values().collect()
}

fn seeded_examples(count: usize, seed: u64) -> Vec<SemanticDeclaration> {
    let mut seeded = Seeded(seed);
    (0..count)
        .map(|_| example_reasoning_declaration(seeded.below(1_000_000)))
        .collect()
}

// ------------------------------------------------------------------- plants

/// The serialized names a plant renames besides the kinds, each with the
/// member that holds it.
const NAMES: &[(&str, &str)] = &[
    ("breadth_first", "order"),
    ("longest_match", "scheme"),
    ("keep_last", "overflow"),
    ("xorshift32", "generator"),
];

struct Plant {
    label: String,
    site: String,
    replacement: String,
    needle: String,
}

/// One plant per serialized kind and per name of `NAMES`, each with the
/// JSON fragment its vector must hold.
fn plant_list() -> Vec<Plant> {
    let mut out: Vec<Plant> = KINDS
        .iter()
        .map(|kind| Plant {
            label: format!("the kind `{kind}` is misspelled"),
            site: format!("(lit \"kind\"), (ctor LexLeanStd.Json::Json.JStr (lit \"{kind}\"))"),
            replacement: format!(
                "(lit \"kind\"), (ctor LexLeanStd.Json::Json.JStr (lit \"{kind}x\"))"
            ),
            needle: format!("\"kind\":\"{kind}\""),
        })
        .collect();
    out.extend(NAMES.iter().map(|(name, holder)| Plant {
        label: format!("the name `{name}` is misspelled"),
        site: format!("(ctor LexLeanStd.Json::Json.JStr (lit \"{name}\"))"),
        replacement: format!("(ctor LexLeanStd.Json::Json.JStr (lit \"{name}x\"))"),
        needle: format!("\"{holder}\":\"{name}\""),
    }));
    out
}

fn plants(examples: &[SemanticDeclaration]) -> usize {
    let mut missed = Vec::new();
    let list = plant_list();
    for plant in &list {
        let vector = examples
            .iter()
            .find(|example| canonical(*example).contains(&plant.needle))
            .unwrap_or_else(|| panic!("{}: no example holds {}", plant.label, plant.needle));
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/SemanticReasoning.lex.tex");
        assert_eq!(
            source.matches(&plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/SemanticReasoning.lex.tex",
            &source.replacen(&plant.site, &plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, &[run(vector)], 1).first() {
            Some(first) => eprintln!(
                "BS-72: plant detected: {}: {}",
                plant.label,
                first.chars().take(120).collect::<String>()
            ),
            None => missed.push(plant.label.clone()),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects their own vector does not detect: {missed:?}"
    );
    list.len()
}

fn export(lowered: &BTreeMap<&str, super::Lowered>) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some((member, low)) = exported_root
            .root
            .strip_prefix(&format!("{MODULE}."))
            .and_then(|member| lowered.get(member).map(|low| (member, low)))
        else {
            continue;
        };
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-72: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
            let package: Json2 = serde_json::from_slice(
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
                serde_json::to_value(&low.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
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
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("SemanticReasoning"),
        "SemanticReasoning is a linked module of the project"
    );
    eligibility_of(&project, "SemanticReasoning", MODULE, &ROOTS);

    let examples: Vec<SemanticDeclaration> =
        (0..EXAMPLES).map(example_reasoning_declaration).collect();
    let distinct: BTreeSet<String> = examples.iter().map(canonical).collect();
    let corpus = committed();
    assert!(
        corpus.len() >= 10,
        "the committed semantic modules carry {} distinct reasoning and text declarations",
        corpus.len()
    );
    let modules = committed_modules();
    assert!(
        modules.len() >= 100
            && modules
                .iter()
                .any(|module| module.declarations.iter().any(is_model)),
        "{} committed semantic modules, among them one with a model declaration",
        modules.len()
    );
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_examples(EVALUATOR_SEEDED, 0x0B55_0071),
            seeded_examples(RUSTC_SEEDED, 0x0B55_1071),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-72: {} examples ({} distinct), {} committed declarations, {} committed modules, {} and {} seeded examples ({:?})",
        examples.len(),
        distinct.len(),
        corpus.len(),
        modules.len(),
        EVALUATOR_SEEDED,
        RUSTC_SEEDED,
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    let root = ROOTS[0].0;
    if normative {
        let mut runs: Vec<Run> = examples.iter().map(run).collect();
        runs.extend(corpus.iter().map(run));
        runs.extend(evaluator_seeded.iter().map(run));
        let disagreements = super::evaluate(&lowered[root], &runs, 5);
        assert!(
            disagreements.is_empty(),
            "{root} and the Rust oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-72: {root}: the evaluator agrees on {} runs ({:?})",
            runs.len(),
            started.elapsed()
        );
        let module_root = ROOTS[1].0;
        let module_runs: Vec<Run> = modules
            .iter()
            .filter(|module| canonical(*module).len() <= EVALUATOR_BYTES)
            .map(module_run)
            .collect();
        let disagreements = super::evaluate(&lowered[module_root], &module_runs, 5);
        assert!(
            disagreements.is_empty(),
            "{module_root} and the Rust oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-72: {module_root}: the evaluator agrees on {} modules ({:?})",
            module_runs.len(),
            started.elapsed()
        );
        let planted = plants(&examples);
        eprintln!(
            "BS-72: {planted} planted defects detected ({:?})",
            started.elapsed()
        );
    } else {
        eprintln!("BS-72: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: Vec<Run> = examples.iter().map(run).collect();
    compiled.extend(corpus.iter().map(run));
    compiled.extend(rustc_seeded.iter().map(run));
    let module_runs: Vec<Run> = modules.iter().map(module_run).collect();
    let total = rustc_differential(
        "BS-72",
        &[
            (root, &lowered[root], compiled.as_slice()),
            (ROOTS[1].0, &lowered[ROOTS[1].0], module_runs.as_slice()),
        ],
        "oracle",
    );
    eprintln!("BS-72: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-72") {
        return;
    }
    export(&lowered);
    eprintln!("BS-72: passed in {:?}", started.elapsed());
}
