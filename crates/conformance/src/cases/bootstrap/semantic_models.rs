//! `BS-71` (SPEC.md §17.23, §17.12): the declarations of models, contracts,
//! realizations, evidence, and artifacts of the semantic module language as
//! language-1.2 source, `LexLeanBoot.SemanticModels`, held to the byte to the
//! serde form of those variants of `lexlean::ir::semantic::SemanticDeclaration`
//! in canonical JSON, as the linked IR embeds them.
//!
//! `modelDeclarationText` writes an artifact, contract, realization, evidence,
//! or model declaration with every datatype it holds: artifact roles and
//! schemas, binders, states, validators, uses, rules, the statistical, neural,
//! and composite realization descriptors with their layers, decoders, forms,
//! and junctions, and evidence claims, each optional member left out exactly
//! where the oracle's serde attributes skip it. The module, its encoders here,
//! and the example builders here are generated from one description of the
//! oracle's types. Every such declaration of every committed semantic module
//! that links is a run, and so are examples that reach every variant, every
//! optional member present and absent, and every list empty and not.
//!
//! Each planted defect renames one serialized kind or name and is detected by
//! the first example that holds it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::ir::semantic::{
    ArtifactRole, ArtifactSchema, CompositeForm, CompositeJunction, ContractPredicate,
    ContractState, ContractValidator, EvidenceClaim, MemberRef, ModelBinder, ModelUse,
    NeuralArchitecture, NeuralDecoder, NeuralLayer, RealizationDescriptor, RealizationRule,
    RealizationState, SemanticDeclaration, SemanticTerm, SemanticType, StatisticalScheme,
    TensorElement,
};
use serde_json::Value as Json2;

use super::ir_document::linked_projects;
use super::rust_validate::{eligibility_of, rustc_differential};
use super::semantic_terms::enc_term;
use super::semantic_types::{canonical, enc_type};
use super::{list, option, pair, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SemanticModels";
const SOURCE: &str = "bootstrap/src/SemanticModels.lex.tex";
const ROOTS: [(&str, &[&str]); 1] = [(
    "modelDeclarationText",
    &["allocation", "overflow", "recursion"],
)];

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

pub(super) fn enc_artifact_role(value: &ArtifactRole) -> Value {
    adt(
        "ArtifactRole",
        &format!("ArtifactRole{value:?}"),
        Vec::new(),
    )
}

pub(super) const ALL_ARTIFACT_ROLE: &[ArtifactRole] = &[
    ArtifactRole::Parameters,
    ArtifactRole::Vocabulary,
    ArtifactRole::Dataset,
    ArtifactRole::Table,
    ArtifactRole::Binary,
];

pub(super) fn example_artifact_role(step: usize) -> ArtifactRole {
    ALL_ARTIFACT_ROLE[step % ALL_ARTIFACT_ROLE.len()]
}

pub(super) fn enc_tensor_element(value: &TensorElement) -> Value {
    adt(
        "TensorElement",
        &format!("TensorElement{value:?}"),
        Vec::new(),
    )
}

pub(super) const ALL_TENSOR_ELEMENT: &[TensorElement] = &[
    TensorElement::Int8,
    TensorElement::Int16,
    TensorElement::Int32,
    TensorElement::Int64,
    TensorElement::UInt8,
    TensorElement::UInt16,
    TensorElement::UInt32,
    TensorElement::UInt64,
];

pub(super) fn example_tensor_element(step: usize) -> TensorElement {
    ALL_TENSOR_ELEMENT[step % ALL_TENSOR_ELEMENT.len()]
}

pub(super) fn enc_artifact_schema(value: &ArtifactSchema) -> Value {
    match value {
        ArtifactSchema::Bytes => adt("ArtifactSchema", "ArtifactSchemaBytes", vec![]),
        ArtifactSchema::IntTensor { element, shape } => adt(
            "ArtifactSchema",
            "ArtifactSchemaIntTensor",
            vec![
                enc_tensor_element(element),
                list(shape.iter().map(|item| nat(*item)).collect()),
            ],
        ),
        ArtifactSchema::Utf8Lines => adt("ArtifactSchema", "ArtifactSchemaUtf8Lines", vec![]),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_artifact_schema(step: usize) -> ArtifactSchema {
    let rest = step / 3;
    match step % 3 {
        0 => ArtifactSchema::Bytes,
        1 => ArtifactSchema::IntTensor {
            element: example_tensor_element(rest),
            shape: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2)
                    .map(|next| u64::try_from(next).expect("a step") + 1)
                    .collect()
            },
        },
        _ => ArtifactSchema::Utf8Lines,
    }
}

pub(super) fn enc_model_binder(value: &ModelBinder) -> Value {
    adt(
        "ModelBinder",
        "ModelBinderOf",
        vec![text((&value.name)), enc_type((&value.r#type))],
    )
}

pub(super) fn example_model_binder(step: usize) -> ModelBinder {
    ModelBinder {
        name: format!("s{step}"),
        r#type: if step % 2 == 0 {
            SemanticType::Nat
        } else {
            SemanticType::Bytes
        },
    }
}

pub(super) fn enc_contract_state(value: &ContractState) -> Value {
    adt(
        "ContractState",
        "ContractStateOf",
        vec![
            text((&value.name)),
            text((&value.next)),
            enc_type((&value.r#type)),
        ],
    )
}

pub(super) fn example_contract_state(step: usize) -> ContractState {
    ContractState {
        name: format!("s{step}"),
        next: format!("s{step}"),
        r#type: if step % 2 == 0 {
            SemanticType::Nat
        } else {
            SemanticType::Bytes
        },
    }
}

pub(super) fn enc_contract_predicate(value: &ContractPredicate) -> Value {
    adt(
        "ContractPredicate",
        &format!("ContractPredicate{value:?}"),
        Vec::new(),
    )
}

pub(super) const ALL_CONTRACT_PREDICATE: &[ContractPredicate] = &[
    ContractPredicate::Invariant,
    ContractPredicate::Postcondition,
    ContractPredicate::Precondition,
];

pub(super) fn example_contract_predicate(step: usize) -> ContractPredicate {
    ALL_CONTRACT_PREDICATE[step % ALL_CONTRACT_PREDICATE.len()]
}

pub(super) fn enc_contract_validator(value: &ContractValidator) -> Value {
    adt(
        "ContractValidator",
        "ContractValidatorOf",
        vec![
            enc_contract_predicate((&value.predicate)),
            enc_member((&value.validator)),
            enc_member((&value.sound)),
            option((&value.complete).as_ref().map(|item| enc_member(item))),
        ],
    )
}

pub(super) fn example_contract_validator(step: usize) -> ContractValidator {
    ContractValidator {
        predicate: example_contract_predicate(step),
        validator: MemberRef {
            module: (step % 2 == 0).then(|| "M".to_owned()),
            name: format!("m{step}"),
        },
        sound: MemberRef {
            module: (step % 2 == 0).then(|| "M".to_owned()),
            name: format!("m{step}"),
        },
        complete: (step % 3 != 2).then(|| MemberRef {
            module: (step % 2 == 0).then(|| "M".to_owned()),
            name: format!("m{step}"),
        }),
    }
}

pub(super) fn enc_model_use(value: &ModelUse) -> Value {
    adt(
        "ModelUse",
        "ModelUseOf",
        vec![
            enc_member((&value.member)),
            list(
                (&value.type_arguments)
                    .iter()
                    .map(|item| enc_type(item))
                    .collect(),
            ),
        ],
    )
}

pub(super) fn example_model_use(step: usize) -> ModelUse {
    ModelUse {
        member: MemberRef {
            module: (step % 2 == 0).then(|| "M".to_owned()),
            name: format!("m{step}"),
        },
        type_arguments: if step % 3 == 2 {
            Vec::new()
        } else {
            (step..step + 2)
                .map(|next| {
                    if next % 2 == 0 {
                        SemanticType::Nat
                    } else {
                        SemanticType::Bytes
                    }
                })
                .collect()
        },
    }
}

pub(super) fn enc_realization_state(value: &RealizationState) -> Value {
    adt(
        "RealizationState",
        "RealizationStateOf",
        vec![
            text((&value.name)),
            enc_type((&value.r#type)),
            enc_term((&value.initial)),
        ],
    )
}

pub(super) fn example_realization_state(step: usize) -> RealizationState {
    RealizationState {
        name: format!("s{step}"),
        r#type: if step % 2 == 0 {
            SemanticType::Nat
        } else {
            SemanticType::Bytes
        },
        initial: SemanticTerm::Nat {
            value: step.to_string(),
        },
    }
}

pub(super) fn enc_realization_rule(value: &RealizationRule) -> Value {
    adt(
        "RealizationRule",
        "RealizationRuleOf",
        vec![
            text((&value.name)),
            enc_term((&value.guard)),
            enc_term((&value.action)),
        ],
    )
}

pub(super) fn example_realization_rule(step: usize) -> RealizationRule {
    RealizationRule {
        name: format!("s{step}"),
        guard: SemanticTerm::Nat {
            value: step.to_string(),
        },
        action: SemanticTerm::Nat {
            value: step.to_string(),
        },
    }
}

pub(super) fn enc_statistical_scheme(value: &StatisticalScheme) -> Value {
    adt(
        "StatisticalScheme",
        &format!("StatisticalScheme{value:?}"),
        Vec::new(),
    )
}

pub(super) const ALL_STATISTICAL_SCHEME: &[StatisticalScheme] = &[StatisticalScheme::LinearScoring];

pub(super) fn example_statistical_scheme(step: usize) -> StatisticalScheme {
    ALL_STATISTICAL_SCHEME[step % ALL_STATISTICAL_SCHEME.len()]
}

pub(super) fn enc_neural_architecture(value: &NeuralArchitecture) -> Value {
    adt(
        "NeuralArchitecture",
        &format!("NeuralArchitecture{value:?}"),
        Vec::new(),
    )
}

pub(super) const ALL_NEURAL_ARCHITECTURE: &[NeuralArchitecture] =
    &[NeuralArchitecture::IntegerFeedforward];

pub(super) fn example_neural_architecture(step: usize) -> NeuralArchitecture {
    ALL_NEURAL_ARCHITECTURE[step % ALL_NEURAL_ARCHITECTURE.len()]
}

pub(super) fn enc_neural_layer(value: &NeuralLayer) -> Value {
    match value {
        NeuralLayer::Dense {
            inputs,
            outputs,
            weights,
            bias,
        } => adt(
            "NeuralLayer",
            "NeuralLayerDense",
            vec![
                nat(*inputs),
                nat(*outputs),
                enc_member(weights),
                enc_member(bias),
            ],
        ),
        NeuralLayer::Relu => adt("NeuralLayer", "NeuralLayerRelu", vec![]),
        NeuralLayer::Requantize {
            shift,
            minimum,
            maximum,
        } => adt(
            "NeuralLayer",
            "NeuralLayerRequantize",
            vec![nat(*shift), text(minimum), text(maximum)],
        ),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_neural_layer(step: usize) -> NeuralLayer {
    let rest = step / 3;
    match step % 3 {
        0 => NeuralLayer::Dense {
            inputs: u64::try_from(rest).expect("a step") + 1,
            outputs: u64::try_from(rest).expect("a step") + 1,
            weights: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            bias: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
        },
        1 => NeuralLayer::Relu,
        _ => NeuralLayer::Requantize {
            shift: u64::try_from(rest).expect("a step") + 1,
            minimum: format!("s{rest}"),
            maximum: format!("s{rest}"),
        },
    }
}

pub(super) fn enc_neural_decoder(value: &NeuralDecoder) -> Value {
    match value {
        NeuralDecoder::Argmax { labels } => adt(
            "NeuralDecoder",
            "NeuralDecoderArgmax",
            vec![list(labels.iter().map(|item| enc_term(item)).collect())],
        ),
        NeuralDecoder::Function { binder, body } => adt(
            "NeuralDecoder",
            "NeuralDecoderFunction",
            vec![text(binder), enc_term(body)],
        ),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_neural_decoder(step: usize) -> NeuralDecoder {
    let rest = step / 2;
    match step % 2 {
        0 => NeuralDecoder::Argmax {
            labels: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2)
                    .map(|next| SemanticTerm::Nat {
                        value: next.to_string(),
                    })
                    .collect()
            },
        },
        _ => NeuralDecoder::Function {
            binder: format!("s{rest}"),
            body: SemanticTerm::Nat {
                value: rest.to_string(),
            },
        },
    }
}

pub(super) fn enc_composite_junction(value: &CompositeJunction) -> Value {
    match value {
        CompositeJunction::Unconditional => adt(
            "CompositeJunction",
            "CompositeJunctionUnconditional",
            vec![],
        ),
        CompositeJunction::Proved { evidence } => adt(
            "CompositeJunction",
            "CompositeJunctionProved",
            vec![enc_member(evidence)],
        ),
        CompositeJunction::Checked => adt("CompositeJunction", "CompositeJunctionChecked", vec![]),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_composite_junction(step: usize) -> CompositeJunction {
    let rest = step / 3;
    match step % 3 {
        0 => CompositeJunction::Unconditional,
        1 => CompositeJunction::Proved {
            evidence: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
        },
        _ => CompositeJunction::Checked,
    }
}

pub(super) fn enc_composite_form(value: &CompositeForm) -> Value {
    match value {
        CompositeForm::Sequence { stages, junctions } => adt(
            "CompositeForm",
            "CompositeFormSequence",
            vec![
                list(stages.iter().map(|item| enc_model_use(item)).collect()),
                list(
                    junctions
                        .iter()
                        .map(|item| enc_composite_junction(item))
                        .collect(),
                ),
            ],
        ),
        CompositeForm::Fanout { left, right } => adt(
            "CompositeForm",
            "CompositeFormFanout",
            vec![enc_model_use(left), enc_model_use(right)],
        ),
        CompositeForm::Product { left, right } => adt(
            "CompositeForm",
            "CompositeFormProduct",
            vec![enc_model_use(left), enc_model_use(right)],
        ),
        CompositeForm::Branch {
            guard,
            then,
            r#else,
        } => adt(
            "CompositeForm",
            "CompositeFormBranch",
            vec![enc_term(guard), enc_model_use(then), enc_model_use(r#else)],
        ),
        CompositeForm::Scan { stage, junction } => adt(
            "CompositeForm",
            "CompositeFormScan",
            vec![enc_model_use(stage), enc_composite_junction(junction)],
        ),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_composite_form(step: usize) -> CompositeForm {
    let rest = step / 5;
    match step % 5 {
        0 => CompositeForm::Sequence {
            stages: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2)
                    .map(|next| example_model_use(next))
                    .collect()
            },
            junctions: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2)
                    .map(|next| example_composite_junction(next))
                    .collect()
            },
        },
        1 => CompositeForm::Fanout {
            left: example_model_use(rest),
            right: example_model_use(rest),
        },
        2 => CompositeForm::Product {
            left: example_model_use(rest),
            right: example_model_use(rest),
        },
        3 => CompositeForm::Branch {
            guard: SemanticTerm::Nat {
                value: rest.to_string(),
            },
            then: example_model_use(rest),
            r#else: example_model_use(rest),
        },
        _ => CompositeForm::Scan {
            stage: example_model_use(rest),
            junction: example_composite_junction(rest),
        },
    }
}

pub(super) fn enc_realization_descriptor(value: &RealizationDescriptor) -> Value {
    match value {
        RealizationDescriptor::Deterministic { body } => adt(
            "RealizationDescriptor",
            "RealizationDescriptorDeterministic",
            vec![enc_term(body)],
        ),
        RealizationDescriptor::Rule { rules, default } => adt(
            "RealizationDescriptor",
            "RealizationDescriptorRule",
            vec![
                list(
                    rules
                        .iter()
                        .map(|item| enc_realization_rule(item))
                        .collect(),
                ),
                enc_term(default),
            ],
        ),
        RealizationDescriptor::Statistical {
            scheme,
            features,
            width,
            width_evidence,
            weights,
            bias,
            labels,
        } => adt(
            "RealizationDescriptor",
            "RealizationDescriptorStatistical",
            vec![
                enc_statistical_scheme(scheme),
                enc_term(features),
                nat(*width),
                enc_member(width_evidence),
                enc_member(weights),
                enc_member(bias),
                list(labels.iter().map(|item| enc_term(item)).collect()),
            ],
        ),
        RealizationDescriptor::Neural {
            architecture,
            encoder,
            width,
            width_evidence,
            layers,
            decoder,
        } => adt(
            "RealizationDescriptor",
            "RealizationDescriptorNeural",
            vec![
                enc_neural_architecture(architecture),
                enc_term(encoder),
                nat(*width),
                enc_member(width_evidence),
                list(layers.iter().map(|item| enc_neural_layer(item)).collect()),
                enc_neural_decoder(decoder),
            ],
        ),
        RealizationDescriptor::Composite { form } => adt(
            "RealizationDescriptor",
            "RealizationDescriptorComposite",
            vec![enc_composite_form(form)],
        ),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_realization_descriptor(step: usize) -> RealizationDescriptor {
    let rest = step / 5;
    match step % 5 {
        0 => RealizationDescriptor::Deterministic {
            body: SemanticTerm::Nat {
                value: rest.to_string(),
            },
        },
        1 => RealizationDescriptor::Rule {
            rules: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2)
                    .map(|next| example_realization_rule(next))
                    .collect()
            },
            default: SemanticTerm::Nat {
                value: rest.to_string(),
            },
        },
        2 => RealizationDescriptor::Statistical {
            scheme: example_statistical_scheme(rest),
            features: SemanticTerm::Nat {
                value: rest.to_string(),
            },
            width: u64::try_from(rest).expect("a step") + 1,
            width_evidence: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            weights: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            bias: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            labels: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2)
                    .map(|next| SemanticTerm::Nat {
                        value: next.to_string(),
                    })
                    .collect()
            },
        },
        3 => RealizationDescriptor::Neural {
            architecture: example_neural_architecture(rest),
            encoder: SemanticTerm::Nat {
                value: rest.to_string(),
            },
            width: u64::try_from(rest).expect("a step") + 1,
            width_evidence: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            layers: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2)
                    .map(|next| example_neural_layer(next))
                    .collect()
            },
            decoder: example_neural_decoder(rest),
        },
        _ => RealizationDescriptor::Composite {
            form: example_composite_form(rest),
        },
    }
}

pub(super) fn enc_evidence_claim(value: &EvidenceClaim) -> Value {
    match value {
        EvidenceClaim::DatasetAgreement {
            dataset,
            comparison,
            comparison_sound,
            examples,
            agreements,
            theorem,
        } => adt(
            "EvidenceClaim",
            "EvidenceClaimDatasetAgreement",
            vec![
                enc_member(dataset),
                enc_member(comparison),
                enc_member(comparison_sound),
                nat(*examples),
                nat(*agreements),
                enc_member(theorem),
            ],
        ),
        EvidenceClaim::EquivalentTo { reference, theorem } => adt(
            "EvidenceClaim",
            "EvidenceClaimEquivalentTo",
            vec![enc_model_use(reference), enc_member(theorem)],
        ),
        EvidenceClaim::InitialInvariant { theorem } => adt(
            "EvidenceClaim",
            "EvidenceClaimInitialInvariant",
            vec![enc_member(theorem)],
        ),
        EvidenceClaim::PreservesInvariant { theorem } => adt(
            "EvidenceClaim",
            "EvidenceClaimPreservesInvariant",
            vec![enc_member(theorem)],
        ),
        EvidenceClaim::SatisfiesContract { theorem } => adt(
            "EvidenceClaim",
            "EvidenceClaimSatisfiesContract",
            vec![enc_member(theorem)],
        ),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_evidence_claim(step: usize) -> EvidenceClaim {
    let rest = step / 5;
    match step % 5 {
        0 => EvidenceClaim::DatasetAgreement {
            dataset: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            comparison: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            comparison_sound: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
            examples: u64::try_from(rest).expect("a step") + 1,
            agreements: u64::try_from(rest).expect("a step") + 1,
            theorem: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
        },
        1 => EvidenceClaim::EquivalentTo {
            reference: example_model_use(rest),
            theorem: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
        },
        2 => EvidenceClaim::InitialInvariant {
            theorem: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
        },
        3 => EvidenceClaim::PreservesInvariant {
            theorem: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
        },
        _ => EvidenceClaim::SatisfiesContract {
            theorem: MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            },
        },
    }
}

pub(super) fn enc_model_declaration(value: &SemanticDeclaration) -> Value {
    match value {
        SemanticDeclaration::Artifact {
            name,
            role,
            sha256,
            length,
            schema,
            r#type,
            axioms,
        } => adt(
            "ModelDeclaration",
            "ModelDeclarationArtifact",
            vec![
                text(name),
                enc_artifact_role(role),
                text(sha256),
                nat(*length),
                enc_artifact_schema(schema),
                enc_type(r#type),
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        SemanticDeclaration::Contract {
            name,
            type_parameters,
            input,
            output,
            state,
            precondition,
            postcondition,
            invariant,
            validators,
            axioms,
        } => adt(
            "ModelDeclaration",
            "ModelDeclarationContract",
            vec![
                text(name),
                list(type_parameters.iter().map(|item| text(item)).collect()),
                enc_model_binder(input),
                enc_model_binder(output),
                option(state.as_ref().map(|item| enc_contract_state(item))),
                option(precondition.as_ref().map(|item| enc_member(item))),
                option(postcondition.as_ref().map(|item| enc_member(item))),
                option(invariant.as_ref().map(|item| enc_member(item))),
                list(
                    validators
                        .iter()
                        .map(|item| enc_contract_validator(item))
                        .collect(),
                ),
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        SemanticDeclaration::Realization {
            name,
            type_parameters,
            input,
            output,
            state,
            descriptor,
            executable,
            axioms,
        } => adt(
            "ModelDeclaration",
            "ModelDeclarationRealization",
            vec![
                text(name),
                list(type_parameters.iter().map(|item| text(item)).collect()),
                enc_model_binder(input),
                enc_type(output),
                option(state.as_ref().map(|item| enc_realization_state(item))),
                enc_realization_descriptor(descriptor),
                Value::Bool { value: *executable },
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        SemanticDeclaration::Evidence {
            name,
            type_parameters,
            contract,
            realization,
            claims,
            axioms,
        } => adt(
            "ModelDeclaration",
            "ModelDeclarationEvidence",
            vec![
                text(name),
                list(type_parameters.iter().map(|item| text(item)).collect()),
                enc_model_use(contract),
                enc_model_use(realization),
                list(claims.iter().map(|item| enc_evidence_claim(item)).collect()),
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        SemanticDeclaration::Model {
            name,
            type_parameters,
            contract,
            realization,
            evidence,
            entry,
            axioms,
        } => adt(
            "ModelDeclaration",
            "ModelDeclarationModel",
            vec![
                text(name),
                list(type_parameters.iter().map(|item| text(item)).collect()),
                enc_model_use(contract),
                enc_model_use(realization),
                list(evidence.iter().map(|item| enc_model_use(item)).collect()),
                list(entry.iter().map(|item| enc_member(item)).collect()),
                list(axioms.iter().map(|item| text(item)).collect()),
            ],
        ),
        other => panic!("not a declaration of this module: {other:?}"),
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn example_model_declaration(step: usize) -> SemanticDeclaration {
    let rest = step / 5;
    match step % 5 {
        0 => SemanticDeclaration::Artifact {
            name: format!("s{rest}"),
            role: example_artifact_role(rest),
            sha256: format!("s{rest}"),
            length: u64::try_from(rest).expect("a step") + 1,
            schema: example_artifact_schema(rest),
            r#type: if rest % 2 == 0 {
                SemanticType::Nat
            } else {
                SemanticType::Bytes
            },
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
        1 => SemanticDeclaration::Contract {
            name: format!("s{rest}"),
            type_parameters: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
            input: example_model_binder(rest),
            output: example_model_binder(rest),
            state: (rest % 3 != 2).then(|| example_contract_state(rest)),
            precondition: (rest % 3 != 2).then(|| MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            }),
            postcondition: (rest % 3 != 2).then(|| MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            }),
            invariant: (rest % 3 != 2).then(|| MemberRef {
                module: (rest % 2 == 0).then(|| "M".to_owned()),
                name: format!("m{rest}"),
            }),
            validators: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2)
                    .map(|next| example_contract_validator(next))
                    .collect()
            },
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
        2 => SemanticDeclaration::Realization {
            name: format!("s{rest}"),
            type_parameters: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
            input: example_model_binder(rest),
            output: if rest % 2 == 0 {
                SemanticType::Nat
            } else {
                SemanticType::Bytes
            },
            state: (rest % 3 != 2).then(|| example_realization_state(rest)),
            descriptor: example_realization_descriptor(rest),
            executable: rest % 2 == 0,
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
        3 => SemanticDeclaration::Evidence {
            name: format!("s{rest}"),
            type_parameters: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
            contract: example_model_use(rest),
            realization: example_model_use(rest),
            claims: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2)
                    .map(|next| example_evidence_claim(next))
                    .collect()
            },
            axioms: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
        },
        _ => SemanticDeclaration::Model {
            name: format!("s{rest}"),
            type_parameters: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2).map(|next| format!("s{next}")).collect()
            },
            contract: example_model_use(rest),
            realization: example_model_use(rest),
            evidence: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2)
                    .map(|next| example_model_use(next))
                    .collect()
            },
            entry: if rest % 3 == 2 {
                Vec::new()
            } else {
                (rest..rest + 2)
                    .map(|next| MemberRef {
                        module: (next % 2 == 0).then(|| "M".to_owned()),
                        name: format!("m{next}"),
                    })
                    .collect()
            },
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
    "bytes",
    "int_tensor",
    "utf8_lines",
    "dense",
    "relu",
    "requantize",
    "argmax",
    "function",
    "unconditional",
    "proved",
    "checked",
    "sequence",
    "fanout",
    "product",
    "branch",
    "scan",
    "deterministic",
    "rule",
    "statistical",
    "neural",
    "composite",
    "dataset_agreement",
    "equivalent_to",
    "initial_invariant",
    "preserves_invariant",
    "satisfies_contract",
    "artifact",
    "contract",
    "realization",
    "evidence",
    "model",
];

// ------------------------------------------------------------- the inputs

pub(super) fn is_model(declaration: &SemanticDeclaration) -> bool {
    matches!(
        declaration,
        SemanticDeclaration::Artifact { .. }
            | SemanticDeclaration::Contract { .. }
            | SemanticDeclaration::Realization { .. }
            | SemanticDeclaration::Evidence { .. }
            | SemanticDeclaration::Model { .. }
    )
}

fn run(declaration: &SemanticDeclaration) -> Run {
    (
        vec![enc_model_declaration(declaration)],
        serde_json::to_value(text(&canonical(declaration))).expect("a value is JSON"),
    )
}

/// Every distinct model-family declaration of every committed semantic
/// module that links.
fn committed() -> Vec<SemanticDeclaration> {
    let mut out = BTreeMap::new();
    for project in linked_projects() {
        for module in project.modules.values() {
            if let Some(semantic) = &module.semantic {
                for declaration in semantic.declarations.iter().filter(|item| is_model(item)) {
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
        .map(|_| example_model_declaration(seeded.below(1_000_000)))
        .collect()
}

// ------------------------------------------------------------------- plants

/// The serialized names a plant renames besides the kinds, each with the
/// member that holds it.
const NAMES: &[(&str, &str)] = &[
    ("uint8", "element"),
    ("linear_scoring", "scheme"),
    ("precondition", "predicate"),
    ("integer_feedforward", "architecture"),
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
        let source = planted.read_raw("src/SemanticModels.lex.tex");
        assert_eq!(
            source.matches(&plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/SemanticModels.lex.tex",
            &source.replacen(&plant.site, &plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, &[run(vector)], 1).first() {
            Some(first) => eprintln!(
                "BS-71: plant detected: {}: {}",
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
            "BS-71: {}: largest certificate part {largest} bytes",
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
        checked_units.units.contains_key("SemanticModels"),
        "SemanticModels is a linked module of the project"
    );
    eligibility_of(&project, "SemanticModels", MODULE, &ROOTS);

    let examples: Vec<SemanticDeclaration> = (0..EXAMPLES).map(example_model_declaration).collect();
    let distinct: BTreeSet<String> = examples.iter().map(canonical).collect();
    let corpus = committed();
    assert!(
        corpus.len() >= 20,
        "the committed semantic modules carry {} distinct model-family declarations",
        corpus.len()
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
        "BS-71: {} examples ({} distinct), {} committed declarations, {} and {} seeded examples ({:?})",
        examples.len(),
        distinct.len(),
        corpus.len(),
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
            "BS-71: {root}: the evaluator agrees on {} runs ({:?})",
            runs.len(),
            started.elapsed()
        );
        let planted = plants(&examples);
        eprintln!(
            "BS-71: {planted} planted defects detected ({:?})",
            started.elapsed()
        );
    } else {
        eprintln!("BS-71: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: Vec<Run> = examples.iter().map(run).collect();
    compiled.extend(corpus.iter().map(run));
    compiled.extend(rustc_seeded.iter().map(run));
    let total = rustc_differential(
        "BS-71",
        &[(root, &lowered[root], compiled.as_slice())],
        "oracle",
    );
    eprintln!("BS-71: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-71") {
        return;
    }
    export(&lowered);
    eprintln!("BS-71: passed in {:?}", started.elapsed());
}
