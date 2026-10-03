//! Language-1.2 models (SPEC.md §17.12, *Models, contracts, realizations,
//! and evidence*): linking, elaboration, and the generated obligations.
//!
//! A model declaration never carries meaning of its own. Linking checks it
//! and elaborates it into ordinary language-1.2 definitions and theorems,
//! which the ordinary rules then check again; every backend reads only those
//! elaborated declarations, so a model means exactly what its elaboration
//! means.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use super::{
    check_declaration_name, check_definition, check_member, check_name, check_theorem, check_type,
    check_type_argument, check_type_parameter_spelling, check_type_parameters,
    declaration_node_count, free_locals, function_info, legal_name, member_key, qualify_type,
    substitute_type, term_node_count, type_node_count, type_parameter_set, visit_terms,
    visit_terms_mut, AlphaRenamer, ArtifactRole, ArtifactSchema, CompositeForm, CompositeJunction,
    ContractPredicate, ContractState, ContractValidator, Environment, EvidenceClaim, MemberRef,
    ModelBinder, ModelCheck, ModelUse, NeuralDecoder, NeuralLayer, RealizationDescriptor,
    RealizationState, SemanticBranch, SemanticDeclaration, SemanticFailure, SemanticInteger,
    SemanticParameter, SemanticPrimitive, SemanticTerm, SemanticType, TensorElement,
    BUILTIN_CONSTRUCTOR_OWNERS,
};
use crate::code;
use crate::diagnostic::DiagnosticCode;

/// Every term a model declaration carries directly, in declaration order.
fn terms_of(declaration: &SemanticDeclaration) -> Vec<&SemanticTerm> {
    let mut out = Vec::new();
    match declaration {
        SemanticDeclaration::Realization {
            state, descriptor, ..
        } => {
            if let Some(state) = state {
                out.push(&state.initial);
            }
            match descriptor {
                RealizationDescriptor::Deterministic { body } => out.push(body),
                RealizationDescriptor::Rule { rules, default } => {
                    for rule in rules {
                        out.push(&rule.guard);
                        out.push(&rule.action);
                    }
                    out.push(default);
                }
                RealizationDescriptor::Statistical {
                    features, labels, ..
                } => {
                    out.push(features);
                    out.extend(labels.iter());
                }
                RealizationDescriptor::Neural {
                    encoder, decoder, ..
                } => {
                    out.push(encoder);
                    match decoder {
                        NeuralDecoder::Argmax { labels } => out.extend(labels.iter()),
                        NeuralDecoder::Function { body, .. } => out.push(body),
                    }
                }
                RealizationDescriptor::Composite { form } => {
                    if let CompositeForm::Branch { guard, .. } = form {
                        out.push(guard);
                    }
                }
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
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => {}
    }
    out
}

/// Visit every term of a model declaration and every subterm.
pub(super) fn declaration_terms(
    declaration: &SemanticDeclaration,
    visit: &mut impl FnMut(&SemanticTerm),
) {
    for term in terms_of(declaration) {
        visit_terms(term, visit);
    }
}

/// Visit every term of a model declaration mutably.
pub(super) fn declaration_terms_mut(
    declaration: &mut SemanticDeclaration,
    visit: &mut impl FnMut(&mut SemanticTerm),
) {
    if let SemanticDeclaration::Realization {
        state, descriptor, ..
    } = declaration
    {
        if let Some(state) = state {
            visit_terms_mut(&mut state.initial, visit);
        }
        match descriptor {
            RealizationDescriptor::Deterministic { body } => visit_terms_mut(body, visit),
            RealizationDescriptor::Rule { rules, default } => {
                for rule in rules {
                    visit_terms_mut(&mut rule.guard, visit);
                    visit_terms_mut(&mut rule.action, visit);
                }
                visit_terms_mut(default, visit);
            }
            RealizationDescriptor::Statistical {
                features, labels, ..
            } => {
                visit_terms_mut(features, visit);
                for label in labels {
                    visit_terms_mut(label, visit);
                }
            }
            RealizationDescriptor::Neural {
                encoder, decoder, ..
            } => {
                visit_terms_mut(encoder, visit);
                match decoder {
                    NeuralDecoder::Argmax { labels } => {
                        for label in labels {
                            visit_terms_mut(label, visit);
                        }
                    }
                    NeuralDecoder::Function { body, .. } => visit_terms_mut(body, visit),
                }
            }
            RealizationDescriptor::Composite { form } => {
                if let CompositeForm::Branch { guard, .. } = form {
                    visit_terms_mut(guard, visit);
                }
            }
        }
    }
}

fn use_types<'a>(model_use: &'a ModelUse, out: &mut Vec<&'a SemanticType>) {
    out.extend(model_use.type_arguments.iter());
}

/// Every type a model declaration states directly, in declaration order.
fn types_of(declaration: &SemanticDeclaration) -> Vec<&SemanticType> {
    let mut out = Vec::new();
    match declaration {
        SemanticDeclaration::Artifact { r#type, .. } => out.push(r#type),
        SemanticDeclaration::Contract {
            input,
            output,
            state,
            ..
        } => {
            out.push(&input.r#type);
            out.push(&output.r#type);
            if let Some(state) = state {
                out.push(&state.r#type);
            }
        }
        SemanticDeclaration::Realization {
            input,
            output,
            state,
            descriptor,
            ..
        } => {
            out.push(&input.r#type);
            out.push(output);
            if let Some(state) = state {
                out.push(&state.r#type);
            }
            if let RealizationDescriptor::Composite { form } = descriptor {
                match form {
                    CompositeForm::Sequence { stages, .. } => {
                        for stage in stages {
                            use_types(stage, &mut out);
                        }
                    }
                    CompositeForm::Fanout { left, right }
                    | CompositeForm::Product { left, right } => {
                        use_types(left, &mut out);
                        use_types(right, &mut out);
                    }
                    CompositeForm::Branch { then, r#else, .. } => {
                        use_types(then, &mut out);
                        use_types(r#else, &mut out);
                    }
                    CompositeForm::Scan { stage, .. } => use_types(stage, &mut out),
                }
            }
        }
        SemanticDeclaration::Evidence {
            contract,
            realization,
            claims,
            ..
        } => {
            use_types(contract, &mut out);
            use_types(realization, &mut out);
            for claim in claims {
                if let EvidenceClaim::EquivalentTo { reference, .. } = claim {
                    use_types(reference, &mut out);
                }
            }
        }
        SemanticDeclaration::Model {
            contract,
            realization,
            evidence,
            ..
        } => {
            use_types(contract, &mut out);
            use_types(realization, &mut out);
            for item in evidence {
                use_types(item, &mut out);
            }
        }
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => {}
    }
    out
}

/// Visit every type a model declaration states directly.
pub(super) fn declaration_types(
    declaration: &SemanticDeclaration,
    visit: &mut impl FnMut(&SemanticType),
) {
    for ty in types_of(declaration) {
        visit(ty);
    }
}

/// Visit every interface binder a model declaration names. Term binders are
/// visited through its terms.
pub(super) fn declaration_binders(declaration: &SemanticDeclaration, visit: &mut impl FnMut(&str)) {
    match declaration {
        SemanticDeclaration::Contract {
            input,
            output,
            state,
            ..
        } => {
            visit(&input.name);
            visit(&output.name);
            if let Some(state) = state {
                visit(&state.name);
                visit(&state.next);
            }
        }
        SemanticDeclaration::Realization {
            input,
            state,
            descriptor,
            ..
        } => {
            visit(&input.name);
            if let Some(state) = state {
                visit(&state.name);
            }
            if let RealizationDescriptor::Neural {
                decoder: NeuralDecoder::Function { binder, .. },
                ..
            } = descriptor
            {
                visit(binder);
            }
        }
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => {}
    }
}

fn count(values: usize) -> u64 {
    u64::try_from(values).unwrap_or(u64::MAX)
}

/// The nodes a model declaration's own source charges to `max_ir_nodes`:
/// every type, term, member reference, layer, rule, junction, claim, check,
/// shape dimension, and binder it states. Its elaboration is charged
/// separately (§17.12).
pub(super) fn source_node_count(declaration: &SemanticDeclaration) -> u64 {
    let types: u64 = types_of(declaration).into_iter().map(type_node_count).sum();
    let terms: u64 = terms_of(declaration).into_iter().map(term_node_count).sum();
    let shape: u64 = match declaration {
        SemanticDeclaration::Artifact { schema, .. } => match schema {
            super::ArtifactSchema::IntTensor { shape, .. } => 1 + count(shape.len()),
            super::ArtifactSchema::Bytes | super::ArtifactSchema::Utf8Lines => 1,
        },
        SemanticDeclaration::Contract {
            type_parameters,
            precondition,
            postcondition,
            invariant,
            validators,
            ..
        } => {
            count(type_parameters.len())
                + u64::from(precondition.is_some())
                + u64::from(postcondition.is_some())
                + u64::from(invariant.is_some())
                + validators
                    .iter()
                    .map(|validator| 3 + u64::from(validator.complete.is_some()))
                    .sum::<u64>()
        }
        SemanticDeclaration::Realization {
            type_parameters,
            descriptor,
            ..
        } => {
            count(type_parameters.len())
                + match descriptor {
                    RealizationDescriptor::Deterministic { .. } => 1,
                    RealizationDescriptor::Rule { rules, .. } => 1 + count(rules.len()),
                    RealizationDescriptor::Statistical { .. } => 5,
                    RealizationDescriptor::Neural { layers, .. } => {
                        3 + layers
                            .iter()
                            .map(|layer| match layer {
                                NeuralLayer::Dense { .. } => 3,
                                NeuralLayer::Relu => 1,
                                NeuralLayer::Requantize { .. } => 2,
                            })
                            .sum::<u64>()
                    }
                    RealizationDescriptor::Composite { form } => match form {
                        CompositeForm::Sequence { stages, junctions } => {
                            1 + count(stages.len()) + count(junctions.len())
                        }
                        CompositeForm::Fanout { .. } | CompositeForm::Product { .. } => 3,
                        CompositeForm::Branch { .. } => 3,
                        CompositeForm::Scan { junction, .. } => {
                            2 + u64::from(matches!(junction, CompositeJunction::Proved { .. }))
                        }
                    },
                }
        }
        SemanticDeclaration::Evidence {
            type_parameters,
            claims,
            ..
        } => count(type_parameters.len()) + 2 + count(claims.len()),
        SemanticDeclaration::Model {
            type_parameters,
            evidence,
            entry,
            ..
        } => count(type_parameters.len()) + 2 + count(evidence.len()) + count(entry.len()),
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => 0,
    };
    types.saturating_add(terms).saturating_add(shape)
}

/// The bytes of every artifact the project locks, by lowercase hexadecimal
/// SHA-256 (§10.1), and what elaborating artifact declarations may still
/// allocate (§17.12 rule 2). Linking reads an artifact only through its
/// digest.
///
/// One digest may be declared under any number of names, and each
/// declaration materializes the bytes again and decodes them again. So each
/// declaration is charged before it allocates: its length toward
/// `max_total_source_bytes`, and the exact node count of its decoded value
/// toward `max_ir_nodes`, both running across every module of the link.
#[derive(Debug)]
pub struct ArtifactStore {
    bytes: BTreeMap<String, Vec<u8>>,
    max_ir_nodes: u64,
    max_total_source_bytes: u64,
    /// The linked IR nodes of the modules before this one plus every value
    /// this module's artifact declarations have decoded so far.
    ir_nodes: std::cell::Cell<u64>,
    /// Every source byte counted so far, including each artifact
    /// declaration's materialized bytes.
    source_bytes: std::cell::Cell<u64>,
}

impl ArtifactStore {
    /// An empty store under the two limits that bound decoding.
    #[must_use]
    pub fn new(max_ir_nodes: u64, max_total_source_bytes: u64) -> Self {
        Self {
            bytes: BTreeMap::new(),
            max_ir_nodes,
            max_total_source_bytes,
            ir_nodes: std::cell::Cell::new(0),
            source_bytes: std::cell::Cell::new(0),
        }
    }

    /// Hold the bytes whose digest is `sha256`.
    pub fn insert(&mut self, sha256: String, bytes: Vec<u8>) {
        self.bytes.insert(sha256, bytes);
    }

    /// The bytes whose digest is `sha256`.
    #[must_use]
    pub fn get(&self, sha256: &str) -> Option<&Vec<u8>> {
        self.bytes.get(sha256)
    }

    /// Every held artifact's bytes.
    pub fn values(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.bytes.values()
    }

    /// Start charging a module: `ir_nodes` linked IR nodes precede it, and
    /// `source_bytes` source bytes are counted so far (the first call; later
    /// modules keep the running count).
    pub fn begin_module(&self, ir_nodes: u64, source_bytes: u64) {
        self.ir_nodes.set(ir_nodes);
        self.source_bytes
            .set(self.source_bytes.get().max(source_bytes));
    }

    /// Charge a declaration that elaborates to at most `nodes` IR nodes,
    /// before anything is elaborated (§17.12 reasoning rule 12): linking
    /// builds an elaboration proportional to the declaration it reads, so a
    /// declaration is refused for the size it would reach, not once it has
    /// reached it.
    pub(super) fn charge_nodes(&self, what: &str, nodes: u64) -> Result<(), SemanticFailure> {
        let total = self.ir_nodes.get().saturating_add(nodes);
        if total > self.max_ir_nodes {
            return Err(fail(
                code!("LLS8002"),
                format!(
                    "max_ir_nodes exceeded: configured {}, observed {total} IR nodes once {what} elaborates to at most {nodes}, before elaborating",
                    self.max_ir_nodes
                ),
            ));
        }
        self.ir_nodes.set(total);
        Ok(())
    }

    /// Charge artifact declaration `name`, of `length` bytes decoding to
    /// `nodes` IR nodes, before anything is materialized.
    fn charge(&self, name: &str, length: u64, nodes: u64) -> Result<(), SemanticFailure> {
        let bytes = self.source_bytes.get().saturating_add(length);
        if bytes > self.max_total_source_bytes {
            return Err(fail(
                code!("LLS8002"),
                format!(
                    "max_total_source_bytes exceeded: configured {}, observed {bytes} source bytes once artifact `{name}` materializes its {length} bytes",
                    self.max_total_source_bytes
                ),
            ));
        }
        let total = self.ir_nodes.get().saturating_add(nodes);
        if total > self.max_ir_nodes {
            return Err(fail(
                code!("LLS8002"),
                format!(
                    "max_ir_nodes exceeded: configured {}, observed {total} IR nodes once artifact `{name}` decodes to {nodes}, before decoding",
                    self.max_ir_nodes
                ),
            ));
        }
        self.source_bytes.set(bytes);
        self.ir_nodes.set(total);
        Ok(())
    }
}

/// The kind of a model or reasoning declaration: a declaration linking
/// elaborates into ordinary declarations (§17.12).
#[must_use]
pub fn elaborated_construct(declaration: &SemanticDeclaration) -> Option<&'static str> {
    declaration_construct(declaration)
        .or_else(|| super::reasoning::declaration_construct(declaration))
}

/// The kind of a model declaration, for diagnostics and the closed list of
/// language-1.2 constructs.
#[must_use]
pub fn declaration_construct(declaration: &SemanticDeclaration) -> Option<&'static str> {
    match declaration {
        SemanticDeclaration::Artifact { .. } => Some("artifact declaration"),
        SemanticDeclaration::Contract { .. } => Some("contract declaration"),
        SemanticDeclaration::Realization { .. } => Some("realization declaration"),
        SemanticDeclaration::Evidence { .. } => Some("evidence declaration"),
        SemanticDeclaration::Model { .. } => Some("model declaration"),
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => None,
    }
}

// ---------------------------------------------------------------------------
// Elaboration products.

/// A Lean-only restatement of a generated obligation against the fixed
/// `LexLeanModels` semantics, or the kernel check that an artifact's typed
/// value is exactly the decoding of its embedded bytes (§17.12).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CrossCheck {
    /// The generated Lean declaration name, below the module.
    pub name: String,
    /// The explicit type parameters it is stated under.
    pub type_parameters: Vec<String>,
    /// What it states.
    pub statement: CheckStatement,
    /// How it is proved.
    pub proof: CheckProof,
}

/// The statement of a cross-check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CheckStatement {
    /// `LexLeanModels.<helper>` applied to the arguments.
    Helper {
        helper: String,
        arguments: Vec<SemanticTerm>,
    },
    /// The artifact's value is the little-endian decoding of its bytes.
    Tensor {
        artifact: String,
        element: TensorElement,
        shape: Vec<u64>,
    },
    /// The artifact's value is the line decoding of its bytes.
    Lines { artifact: String },
    /// The artifact's value is its bytes.
    Bytes { artifact: String },
}

/// The proof of a cross-check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CheckProof {
    /// Exactly the named theorem at the explicit type arguments.
    Theorem {
        theorem: MemberRef,
        type_arguments: Vec<SemanticType>,
    },
    /// Kernel evaluation.
    Decide,
}

/// One generated obligation and the theorem that discharges it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Obligation {
    /// What the obligation is for, such as `claim satisfies_contract`.
    pub role: String,
    /// The discharging theorem.
    pub theorem: MemberRef,
    /// The obligation's type parameters.
    pub type_parameters: Vec<String>,
    /// Its parameters.
    pub parameters: Vec<SemanticParameter>,
    /// Its exact statement, up to the names of bound variables.
    pub statement: SemanticTerm,
}

/// What one declaration elaborates to.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Lowering {
    /// Ordinary declarations, in dependency order.
    pub(crate) declarations: Vec<SemanticDeclaration>,
    /// Lean-only cross-checks.
    pub(crate) checks: Vec<CrossCheck>,
    /// Generated obligations.
    pub(crate) obligations: Vec<Obligation>,
    /// For a model, the runtime checks every executable application needs.
    pub(crate) required: Vec<ModelCheck>,
    /// For a reasoning declaration, the theorems it generates (§17.12).
    pub(crate) theorems: Vec<super::reasoning::GeneratedTheorem>,
}

/// A linked module's elaboration (§17.12): per declaration, what it means,
/// and the model interfaces it exports to importers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Elaboration {
    pub(crate) lowered: Vec<Vec<SemanticDeclaration>>,
    pub(crate) checks: Vec<Vec<CrossCheck>>,
    pub(crate) obligations: Vec<Vec<Obligation>>,
    pub(crate) required: Vec<Vec<ModelCheck>>,
    pub(crate) theorems: Vec<Vec<super::reasoning::GeneratedTheorem>>,
    /// The model interfaces this module declares, keyed by local name.
    pub(crate) models: Models,
    /// The reasoning interfaces this module declares, keyed by local name.
    pub(crate) reasoning: super::reasoning::Reasoning,
}

/// The elaboration under construction, declaration by declaration.
pub(super) struct Builder {
    pending: Vec<Option<Lowering>>,
}

impl Builder {
    pub(super) fn new(declarations: usize) -> Self {
        Self {
            pending: vec![None; declarations],
        }
    }

    /// An ordinary declaration means `declarations`.
    pub(super) fn lower(&mut self, index: usize, declarations: Vec<SemanticDeclaration>) {
        if let Some(slot) = self.pending.get_mut(index) {
            *slot = Some(Lowering {
                declarations,
                ..Lowering::default()
            });
        }
    }

    /// A model declaration elaborated to `lowering`.
    pub(super) fn record(&mut self, index: usize, lowering: Lowering) {
        if let Some(slot) = self.pending.get_mut(index) {
            *slot = Some(lowering);
        }
    }

    /// The finished elaboration. A data declaration admitted with its group
    /// means itself; any other unelaborated declaration is a compiler defect.
    pub(super) fn finish(
        self,
        declarations: &[SemanticDeclaration],
        models: Models,
        reasoning: super::reasoning::Reasoning,
    ) -> Result<Elaboration, SemanticFailure> {
        let mut elaboration = Elaboration {
            models,
            reasoning,
            ..Elaboration::default()
        };
        for (declaration, lowering) in declarations.iter().zip(self.pending) {
            let lowering = match lowering {
                Some(lowering) => lowering,
                None => match declaration {
                    SemanticDeclaration::Structure { .. }
                    | SemanticDeclaration::Class { .. }
                    | SemanticDeclaration::Inductive { .. } => Lowering {
                        declarations: vec![declaration.clone()],
                        ..Lowering::default()
                    },
                    _ => {
                        return Err(format!(
                            "internal: declaration `{}` was not elaborated",
                            declaration.name()
                        )
                        .into());
                    }
                },
            };
            elaboration.lowered.push(lowering.declarations);
            elaboration.checks.push(lowering.checks);
            elaboration.obligations.push(lowering.obligations);
            elaboration.required.push(lowering.required);
            elaboration.theorems.push(lowering.theorems);
        }
        Ok(elaboration)
    }
}

impl Elaboration {
    /// Canonicalize every collection literal of the elaborated declarations,
    /// as linking does for the source ones.
    pub(super) fn normalize(&mut self, normalize: &mut impl FnMut(&mut SemanticTerm)) {
        for declaration in self.lowered.iter_mut().flatten() {
            super::declaration_terms_mut(declaration, normalize);
        }
    }

    /// The nodes the elaboration charges to `max_ir_nodes` beyond the
    /// source: every declaration a model elaborates to, every ordinary
    /// declaration whose checked applications were elaborated, and every
    /// generated obligation statement.
    /// An ordinary declaration that elaborates to itself is already charged
    /// as source and is not charged again.
    pub(super) fn node_count(&self, source: &[SemanticDeclaration]) -> u64 {
        let declarations: u64 = self
            .lowered
            .iter()
            .enumerate()
            .filter(|(index, lowered)| {
                source.get(*index).is_none_or(|declaration| {
                    lowered.as_slice() != std::slice::from_ref(declaration)
                })
            })
            .flat_map(|(_, lowered)| lowered)
            .map(declaration_node_count)
            .sum();
        let obligations: u64 = self
            .obligations
            .iter()
            .flatten()
            .map(|obligation| {
                term_node_count(&obligation.statement)
                    + obligation
                        .parameters
                        .iter()
                        .map(|parameter| type_node_count(&parameter.r#type))
                        .sum::<u64>()
            })
            .sum();
        let checks = self
            .checks
            .iter()
            .map(|checks| checks.len() as u64)
            .sum::<u64>();
        let theorems: u64 = self
            .theorems
            .iter()
            .flatten()
            .map(super::reasoning::theorem_node_count)
            .sum();
        declarations
            .saturating_add(obligations)
            .saturating_add(checks)
            .saturating_add(theorems)
    }

    /// Every cross-check, in declaration order.
    #[must_use]
    pub fn all_checks(&self) -> Vec<&CrossCheck> {
        self.checks.iter().flatten().collect()
    }

    /// The elaborated declarations of source declaration `index`.
    #[must_use]
    pub fn lowered(&self, index: usize) -> &[SemanticDeclaration] {
        self.lowered.get(index).map_or(&[], Vec::as_slice)
    }

    /// The Lean-only cross-checks of source declaration `index`.
    #[must_use]
    pub fn checks(&self, index: usize) -> &[CrossCheck] {
        self.checks.get(index).map_or(&[], Vec::as_slice)
    }

    /// The generated obligations of source declaration `index`.
    #[must_use]
    pub fn obligations(&self, index: usize) -> &[Obligation] {
        self.obligations.get(index).map_or(&[], Vec::as_slice)
    }

    /// The theorems source declaration `index` generates, in order.
    #[must_use]
    pub fn theorems(&self, index: usize) -> &[super::reasoning::GeneratedTheorem] {
        self.theorems.get(index).map_or(&[], Vec::as_slice)
    }

    /// Every generated theorem, in declaration order.
    #[must_use]
    pub fn all_theorems(&self) -> Vec<&super::reasoning::GeneratedTheorem> {
        self.theorems.iter().flatten().collect()
    }

    /// The runtime checks executable code must apply to the model declared
    /// at `index` (empty for any other declaration).
    #[must_use]
    pub fn required(&self, index: usize) -> &[ModelCheck] {
        self.required.get(index).map_or(&[], Vec::as_slice)
    }
}

// ---------------------------------------------------------------------------
// The model environment.

/// What linking knows about one artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArtifactInfo {
    role: ArtifactRole,
    schema: ArtifactSchema,
}

/// What linking knows about one contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ContractInfo {
    type_parameters: Vec<String>,
    input: ModelBinder,
    output: ModelBinder,
    state: Option<ContractState>,
    precondition: Option<MemberRef>,
    postcondition: Option<MemberRef>,
    invariant: Option<MemberRef>,
    validators: Vec<ContractValidator>,
}

/// What linking knows about one realization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RealizationInfo {
    type_parameters: Vec<String>,
    input: ModelBinder,
    output: SemanticType,
    state: Option<SemanticType>,
    /// The state binder the effective preconditions mention, when stateful.
    state_binder: Option<String>,
    /// Propositions over the input (and state) binder that a model binding
    /// must establish from its contract's invariant and precondition.
    effective: Vec<SemanticTerm>,
}

/// The closed claim kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ClaimKind {
    DatasetAgreement,
    EquivalentTo,
    InitialInvariant,
    PreservesInvariant,
    SatisfiesContract,
}

impl ClaimKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::DatasetAgreement => "dataset_agreement",
            Self::EquivalentTo => "equivalent_to",
            Self::InitialInvariant => "initial_invariant",
            Self::PreservesInvariant => "preserves_invariant",
            Self::SatisfiesContract => "satisfies_contract",
        }
    }

    fn of(claim: &EvidenceClaim) -> (Self, &MemberRef) {
        match claim {
            EvidenceClaim::DatasetAgreement { theorem, .. } => (Self::DatasetAgreement, theorem),
            EvidenceClaim::EquivalentTo { theorem, .. } => (Self::EquivalentTo, theorem),
            EvidenceClaim::InitialInvariant { theorem } => (Self::InitialInvariant, theorem),
            EvidenceClaim::PreservesInvariant { theorem } => (Self::PreservesInvariant, theorem),
            EvidenceClaim::SatisfiesContract { theorem } => (Self::SatisfiesContract, theorem),
        }
    }
}

/// What linking knows about one evidence declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EvidenceInfo {
    type_parameters: Vec<String>,
    contract: ModelUse,
    realization: ModelUse,
    claims: BTreeSet<ClaimKind>,
}

/// What linking knows about one model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModelInfo {
    type_parameters: Vec<String>,
    contract: ModelUse,
    realization: ModelUse,
    /// The claim kinds its evidence discharges.
    discharged: BTreeSet<ClaimKind>,
}

/// Every visible model interface, by environment key (a local name, or
/// `module::name` for an import).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Models {
    artifacts: BTreeMap<String, ArtifactInfo>,
    contracts: BTreeMap<String, ContractInfo>,
    realizations: BTreeMap<String, RealizationInfo>,
    evidence: BTreeMap<String, EvidenceInfo>,
    models: BTreeMap<String, ModelInfo>,
    /// Realizations and their companion definitions: executable source
    /// applies them only through a model.
    realization_functions: BTreeSet<String>,
}

impl Models {
    /// The interfaces this module declares itself.
    pub(super) fn locals(&self) -> Self {
        fn local<T: Clone>(map: &BTreeMap<String, T>) -> BTreeMap<String, T> {
            map.iter()
                .filter(|(key, _)| !key.contains("::"))
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect()
        }
        Self {
            artifacts: local(&self.artifacts),
            contracts: local(&self.contracts),
            realizations: local(&self.realizations),
            evidence: local(&self.evidence),
            models: local(&self.models),
            realization_functions: self
                .realization_functions
                .iter()
                .filter(|key| !key.contains("::"))
                .cloned()
                .collect(),
        }
    }
}

// ---------------------------------------------------------------------------
// Qualification of an imported module's interfaces.

fn is_builtin_constructor(member: &MemberRef) -> bool {
    member.module.is_none()
        && member
            .name
            .split_once('.')
            .is_some_and(|(owner, _)| BUILTIN_CONSTRUCTOR_OWNERS.contains(&owner))
}

pub(super) fn anchor(member: &MemberRef, module: &str) -> MemberRef {
    MemberRef {
        module: member.module.clone().or_else(|| Some(module.to_owned())),
        name: member.name.clone(),
    }
}

pub(super) fn anchor_use(model_use: &ModelUse, module: &str) -> ModelUse {
    ModelUse {
        member: anchor(&model_use.member, module),
        type_arguments: model_use
            .type_arguments
            .iter()
            .map(|argument| qualify_type(argument, module))
            .collect(),
    }
}

/// A term of `module` as an importer names it: every document reference
/// gains its module, built-in constructors stay as they are.
pub(super) fn qualify_term(term: &SemanticTerm, module: &str) -> SemanticTerm {
    fn types(types: &mut [SemanticType], module: &str) {
        for ty in types {
            *ty = qualify_type(ty, module);
        }
    }
    let mut out = term.clone();
    visit_terms_mut(&mut out, &mut |node| match node {
        SemanticTerm::Call {
            function,
            type_arguments,
            ..
        }
        | SemanticTerm::FunctionRef {
            function,
            type_arguments,
        }
        | SemanticTerm::CheckedApply {
            model: function,
            type_arguments,
            ..
        } => {
            *function = anchor(function, module);
            types(type_arguments, module);
        }
        SemanticTerm::Constructor {
            constructor,
            type_arguments,
            ..
        } => {
            if !is_builtin_constructor(constructor) {
                *constructor = anchor(constructor, module);
            }
            types(type_arguments, module);
        }
        SemanticTerm::Record {
            r#type,
            type_arguments,
            ..
        } => {
            *r#type = anchor(r#type, module);
            types(type_arguments, module);
        }
        SemanticTerm::InstanceValue {
            class,
            arguments,
            resolved,
        } => {
            *class = anchor(class, module);
            *resolved = anchor(resolved, module);
            types(arguments, module);
        }
        SemanticTerm::Match { branches, .. } => {
            for branch in branches {
                if !is_builtin_constructor(&branch.constructor) {
                    branch.constructor = anchor(&branch.constructor, module);
                }
            }
        }
        SemanticTerm::Nil { element } => *element = qualify_type(element, module),
        SemanticTerm::Primitive { result, .. } => *result = qualify_type(result, module),
        SemanticTerm::Forall { binder, .. } | SemanticTerm::Let { binder, .. } => {
            binder.r#type = qualify_type(&binder.r#type, module);
        }
        SemanticTerm::Lambda { parameters, .. } => {
            for parameter in parameters {
                parameter.r#type = qualify_type(&parameter.r#type, module);
            }
        }
        SemanticTerm::MapLiteral { key, value, .. } => {
            *key = qualify_type(key, module);
            *value = qualify_type(value, module);
        }
        SemanticTerm::SetLiteral { element, .. } => *element = qualify_type(element, module),
        SemanticTerm::GraphLiteral { node, .. } => *node = qualify_type(node, module),
        SemanticTerm::Var { .. }
        | SemanticTerm::Nat { .. }
        | SemanticTerm::Integer { .. }
        | SemanticTerm::String { .. }
        | SemanticTerm::Bytes { .. }
        | SemanticTerm::Bool { .. }
        | SemanticTerm::Unit
        | SemanticTerm::Cons { .. }
        | SemanticTerm::Project { .. }
        | SemanticTerm::If { .. }
        | SemanticTerm::Eq { .. }
        | SemanticTerm::Le { .. }
        | SemanticTerm::Lt { .. }
        | SemanticTerm::Add { .. }
        | SemanticTerm::Beq { .. }
        | SemanticTerm::Ble { .. }
        | SemanticTerm::Blt { .. }
        | SemanticTerm::And { .. }
        | SemanticTerm::PropAnd { .. }
        | SemanticTerm::Or { .. }
        | SemanticTerm::Not { .. }
        | SemanticTerm::Implies { .. }
        | SemanticTerm::Iff { .. }
        | SemanticTerm::Pair { .. }
        | SemanticTerm::First { .. }
        | SemanticTerm::Second { .. }
        | SemanticTerm::Apply { .. } => {}
    });
    out
}

pub(super) fn qualify_binder(binder: &ModelBinder, module: &str) -> ModelBinder {
    ModelBinder {
        name: binder.name.clone(),
        r#type: qualify_type(&binder.r#type, module),
    }
}

/// Register an imported module's model interfaces under `module::name`.
pub(super) fn register_import(module: &str, elaboration: &Elaboration, models: &mut Models) {
    let key = |name: &str| format!("{module}::{name}");
    let exported = &elaboration.models;
    for (name, info) in &exported.artifacts {
        models.artifacts.insert(key(name), info.clone());
    }
    for (name, info) in &exported.contracts {
        models.contracts.insert(
            key(name),
            ContractInfo {
                type_parameters: info.type_parameters.clone(),
                input: qualify_binder(&info.input, module),
                output: qualify_binder(&info.output, module),
                state: info.state.as_ref().map(|state| ContractState {
                    name: state.name.clone(),
                    next: state.next.clone(),
                    r#type: qualify_type(&state.r#type, module),
                }),
                precondition: info
                    .precondition
                    .as_ref()
                    .map(|member| anchor(member, module)),
                postcondition: info
                    .postcondition
                    .as_ref()
                    .map(|member| anchor(member, module)),
                invariant: info.invariant.as_ref().map(|member| anchor(member, module)),
                validators: info
                    .validators
                    .iter()
                    .map(|validator| ContractValidator {
                        predicate: validator.predicate,
                        validator: anchor(&validator.validator, module),
                        sound: anchor(&validator.sound, module),
                        complete: validator
                            .complete
                            .as_ref()
                            .map(|member| anchor(member, module)),
                    })
                    .collect(),
            },
        );
    }
    for (name, info) in &exported.realizations {
        models.realizations.insert(
            key(name),
            RealizationInfo {
                type_parameters: info.type_parameters.clone(),
                input: qualify_binder(&info.input, module),
                output: qualify_type(&info.output, module),
                state: info.state.as_ref().map(|state| qualify_type(state, module)),
                state_binder: info.state_binder.clone(),
                effective: info
                    .effective
                    .iter()
                    .map(|term| qualify_term(term, module))
                    .collect(),
            },
        );
    }
    for (name, info) in &exported.evidence {
        models.evidence.insert(
            key(name),
            EvidenceInfo {
                type_parameters: info.type_parameters.clone(),
                contract: anchor_use(&info.contract, module),
                realization: anchor_use(&info.realization, module),
                claims: info.claims.clone(),
            },
        );
    }
    for (name, info) in &exported.models {
        models.models.insert(
            key(name),
            ModelInfo {
                type_parameters: info.type_parameters.clone(),
                contract: anchor_use(&info.contract, module),
                realization: anchor_use(&info.realization, module),
                discharged: info.discharged.clone(),
            },
        );
    }
    for name in &exported.realization_functions {
        models.realization_functions.insert(key(name));
    }
}

// ---------------------------------------------------------------------------
// Failures.

pub(super) fn fail(code: DiagnosticCode, reason: String) -> SemanticFailure {
    SemanticFailure { code, reason }
}

fn interface(reason: String) -> SemanticFailure {
    fail(code!("LLT4006"), reason)
}

fn composition(reason: String) -> SemanticFailure {
    fail(code!("LLT4007"), reason)
}

fn boundary(reason: String) -> SemanticFailure {
    fail(code!("LLT4008"), reason)
}

fn unestablished(reason: String) -> SemanticFailure {
    fail(code!("LLT4009"), reason)
}

// ---------------------------------------------------------------------------
// Term and type builders. Every generated binder begins with two
// underscores, which no source name can, so none captures a source local.

pub(super) fn local(name: &str) -> MemberRef {
    MemberRef {
        module: None,
        name: name.to_owned(),
    }
}

pub(super) fn var(name: &str) -> SemanticTerm {
    SemanticTerm::Var {
        name: name.to_owned(),
    }
}

pub(super) fn call(
    function: &MemberRef,
    type_arguments: &[SemanticType],
    arguments: Vec<SemanticTerm>,
) -> SemanticTerm {
    SemanticTerm::Call {
        function: function.clone(),
        type_arguments: type_arguments.to_vec(),
        arguments,
    }
}

pub(super) fn function_ref(function: &MemberRef, type_arguments: &[SemanticType]) -> SemanticTerm {
    SemanticTerm::FunctionRef {
        function: function.clone(),
        type_arguments: type_arguments.to_vec(),
    }
}

pub(super) fn eq(left: SemanticTerm, right: SemanticTerm) -> SemanticTerm {
    SemanticTerm::Eq {
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub(super) fn implies(premise: SemanticTerm, conclusion: SemanticTerm) -> SemanticTerm {
    SemanticTerm::Implies {
        premise: Box::new(premise),
        conclusion: Box::new(conclusion),
    }
}

/// `premise -> conclusion`, or the conclusion alone without a premise.
pub(super) fn premised(premise: Option<SemanticTerm>, conclusion: SemanticTerm) -> SemanticTerm {
    match premise {
        Some(premise) => implies(premise, conclusion),
        None => conclusion,
    }
}

pub(super) fn first(value: SemanticTerm) -> SemanticTerm {
    SemanticTerm::First {
        value: Box::new(value),
    }
}

pub(super) fn second(value: SemanticTerm) -> SemanticTerm {
    SemanticTerm::Second {
        value: Box::new(value),
    }
}

pub(super) fn pair(left: SemanticTerm, right: SemanticTerm) -> SemanticTerm {
    SemanticTerm::Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub(super) fn boolean(value: bool) -> SemanticTerm {
    SemanticTerm::Bool { value }
}

pub(super) fn nat(value: u64) -> SemanticTerm {
    SemanticTerm::Nat {
        value: value.to_string(),
    }
}

fn int(value: &str) -> SemanticTerm {
    SemanticTerm::Integer {
        representation: SemanticInteger::Int,
        value: value.to_owned(),
    }
}

pub(super) fn constructor(
    name: &str,
    type_arguments: Vec<SemanticType>,
    arguments: Vec<SemanticTerm>,
) -> SemanticTerm {
    SemanticTerm::Constructor {
        constructor: local(name),
        type_arguments,
        arguments,
    }
}

pub(super) fn let_in(
    name: &str,
    ty: SemanticType,
    value: SemanticTerm,
    body: SemanticTerm,
) -> SemanticTerm {
    SemanticTerm::Let {
        binder: SemanticParameter {
            name: name.to_owned(),
            r#type: ty,
        },
        value: Box::new(value),
        body: Box::new(body),
    }
}

pub(super) fn if_then(
    condition: SemanticTerm,
    then_value: SemanticTerm,
    else_value: SemanticTerm,
) -> SemanticTerm {
    SemanticTerm::If {
        condition: Box::new(condition),
        then_value: Box::new(then_value),
        else_value: Box::new(else_value),
    }
}

pub(super) fn primitive(
    operation: SemanticPrimitive,
    arguments: Vec<SemanticTerm>,
    result: SemanticType,
) -> SemanticTerm {
    SemanticTerm::Primitive {
        operation,
        arguments,
        result,
    }
}

/// A lambda capturing exactly the locals its body uses beyond its own
/// parameters, sorted (§17.12).
pub(super) fn lambda(parameters: Vec<(&str, SemanticType)>, body: SemanticTerm) -> SemanticTerm {
    let mut bound: BTreeSet<String> = parameters
        .iter()
        .map(|(name, _)| (*name).to_owned())
        .collect();
    let mut captures = BTreeSet::new();
    free_locals(&body, &mut bound, &mut captures);
    SemanticTerm::Lambda {
        parameters: parameters
            .into_iter()
            .map(|(name, ty)| SemanticParameter {
                name: name.to_owned(),
                r#type: ty,
            })
            .collect(),
        captures: captures.into_iter().collect(),
        body: Box::new(body),
    }
}

pub(super) fn matching(
    scrutinee: SemanticTerm,
    branches: Vec<(&str, Vec<&str>, SemanticTerm)>,
) -> SemanticTerm {
    SemanticTerm::Match {
        scrutinee: Box::new(scrutinee),
        branches: branches
            .into_iter()
            .map(|(name, binders, body)| SemanticBranch {
                constructor: local(name),
                binders: binders.into_iter().map(str::to_owned).collect(),
                body,
            })
            .collect(),
    }
}

pub(super) fn list_type(element: SemanticType) -> SemanticType {
    SemanticType::List {
        element: Box::new(element),
    }
}

fn int_list() -> SemanticType {
    list_type(SemanticType::Int)
}

pub(super) fn product(left: SemanticType, right: SemanticType) -> SemanticType {
    SemanticType::Product {
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub(super) fn option_type(value: SemanticType) -> SemanticType {
    SemanticType::Option {
        value: Box::new(value),
    }
}

fn result_type(ok: SemanticType) -> SemanticType {
    SemanticType::Result {
        ok: Box::new(ok),
        error: Box::new(SemanticType::ContractViolation),
    }
}

fn ok_value(ok: &SemanticType, value: SemanticTerm) -> SemanticTerm {
    constructor(
        "Result.ok",
        vec![ok.clone(), SemanticType::ContractViolation],
        vec![value],
    )
}

fn refusal(ok: &SemanticType, check: ModelCheck) -> SemanticTerm {
    constructor(
        "Result.error",
        vec![ok.clone(), SemanticType::ContractViolation],
        vec![constructor(
            violation_constructor(check),
            Vec::new(),
            Vec::new(),
        )],
    )
}

/// The constructors of `ContractViolation`, in declaration order.
pub const VIOLATIONS: [&str; 4] = [
    "ContractViolation.precondition",
    "ContractViolation.input_invariant",
    "ContractViolation.postcondition",
    "ContractViolation.output_invariant",
];

/// The `ContractViolation` constructor naming a failed check.
#[must_use]
pub fn violation_constructor(check: ModelCheck) -> &'static str {
    match check {
        ModelCheck::Precondition => "ContractViolation.precondition",
        ModelCheck::InputInvariant => "ContractViolation.input_invariant",
        ModelCheck::Postcondition => "ContractViolation.postcondition",
        ModelCheck::OutputInvariant => "ContractViolation.output_invariant",
    }
}

/// The serialized name of a check.
#[must_use]
pub const fn check_name_of(check: ModelCheck) -> &'static str {
    match check {
        ModelCheck::InputInvariant => "input_invariant",
        ModelCheck::OutputInvariant => "output_invariant",
        ModelCheck::Postcondition => "postcondition",
        ModelCheck::Precondition => "precondition",
    }
}

/// A list literal. A long list is a balanced tree of `append`s of short
/// literals, so no generated term nests deeper than a fixed chunk plus the
/// logarithm of its length.
pub(super) fn list_literal(element: &SemanticType, items: Vec<SemanticTerm>) -> SemanticTerm {
    const CHUNK: usize = 32;
    if items.len() <= CHUNK {
        let mut out = SemanticTerm::Nil {
            element: element.clone(),
        };
        for item in items.into_iter().rev() {
            out = SemanticTerm::Cons {
                head: Box::new(item),
                tail: Box::new(out),
            };
        }
        return out;
    }
    let mut items = items;
    let half = items.len().div_ceil(CHUNK).div_ceil(2) * CHUNK;
    let right = items.split_off(half);
    primitive(
        SemanticPrimitive::Append,
        vec![list_literal(element, items), list_literal(element, right)],
        list_type(element.clone()),
    )
}

/// Substitute type parameters throughout a term. Only types have the tag
/// `parameter`, so the rewrite cannot touch a term node.
pub(super) fn substitute_term_types(
    term: &SemanticTerm,
    map: &BTreeMap<String, SemanticType>,
) -> SemanticTerm {
    fn rewrite(value: &mut serde_json::Value, map: &BTreeMap<String, serde_json::Value>) {
        match value {
            serde_json::Value::Array(items) => items.iter_mut().for_each(|item| rewrite(item, map)),
            serde_json::Value::Object(object) => {
                if object.get("kind").and_then(serde_json::Value::as_str) == Some("parameter") {
                    if let Some(serde_json::Value::String(name)) = object.get("name") {
                        if let Some(replacement) = map.get(name) {
                            *value = replacement.clone();
                            return;
                        }
                    }
                }
                object.values_mut().for_each(|child| rewrite(child, map));
            }
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }
    if map.is_empty() {
        return term.clone();
    }
    let map: BTreeMap<String, serde_json::Value> = map
        .iter()
        .map(|(name, ty)| {
            (
                name.clone(),
                serde_json::to_value(ty).expect("type serializes"),
            )
        })
        .collect();
    let mut value = serde_json::to_value(term).expect("semantic term serializes");
    rewrite(&mut value, &map);
    serde_json::from_value(value).expect("a substituted term deserializes")
}

pub(super) fn substitution(
    parameters: &[String],
    arguments: &[SemanticType],
) -> BTreeMap<String, SemanticType> {
    parameters
        .iter()
        .cloned()
        .zip(arguments.iter().cloned())
        .collect()
}

pub(super) fn parameter_types(parameters: &[String]) -> Vec<SemanticType> {
    parameters
        .iter()
        .map(|name| SemanticType::Parameter { name: name.clone() })
        .collect()
}

pub(super) fn parameter(name: &str, ty: &SemanticType) -> SemanticParameter {
    SemanticParameter {
        name: name.to_owned(),
        r#type: ty.clone(),
    }
}

// ---------------------------------------------------------------------------
// Statement-exact obligations.

/// The canonical form of a statement over its parameters: every type
/// parameter, parameter, and bound local renamed positionally.
pub(super) fn canonical(
    type_parameters: &[String],
    parameters: &[SemanticParameter],
    statement: &SemanticTerm,
) -> (Vec<SemanticType>, SemanticTerm) {
    let mut renamer = AlphaRenamer {
        next: 0,
        scopes: Vec::new(),
        types: type_parameters
            .iter()
            .enumerate()
            .map(|(index, name)| (name.clone(), format!("T{index}")))
            .collect(),
    };
    let types = parameters
        .iter()
        .map(|parameter| {
            let ty = renamer.ty(&parameter.r#type);
            renamer.bind(&parameter.name);
            ty
        })
        .collect();
    (types, renamer.term(statement))
}

/// `theorem` is a prior theorem of this module whose type parameters,
/// parameters, and statement are exactly the obligation's, up to the names
/// of bound variables.
pub(super) fn require_statement(
    env: &Environment<'_>,
    theorem: &MemberRef,
    obligation: &Obligation,
    code: DiagnosticCode,
) -> Result<(), SemanticFailure> {
    let role = &obligation.role;
    if theorem.module.is_some() {
        return Err(fail(
            code,
            format!(
                "{role}: `{}` must be a prior theorem of this module",
                member_key(theorem)
            ),
        ));
    }
    let Some((type_parameters, parameters, statement)) = env.theorems.get(&theorem.name) else {
        return Err(fail(
            code,
            format!(
                "{role}: `{}` is not a prior theorem of this module",
                theorem.name
            ),
        ));
    };
    let expected = canonical(
        &obligation.type_parameters,
        &obligation.parameters,
        &obligation.statement,
    );
    if type_parameters.len() != obligation.type_parameters.len()
        || canonical(type_parameters, parameters, statement) != expected
    {
        return Err(fail(
            code,
            format!(
                "{role}: `{}` does not state exactly the generated obligation over ({}): {}",
                theorem.name,
                obligation
                    .parameters
                    .iter()
                    .map(|parameter| format!("{} : {}", parameter.name, parameter.r#type))
                    .collect::<Vec<_>>()
                    .join(", "),
                serde_json::to_string(&obligation.statement).expect("statement serializes")
            ),
        ));
    }
    Ok(())
}

pub(super) fn check_axioms(name: &str, axioms: &[String]) -> Result<(), SemanticFailure> {
    if axioms.windows(2).any(|pair| pair[0] >= pair[1])
        || axioms.iter().any(|axiom| !legal_name(axiom))
    {
        return Err(format!("`{name}` axiom policy is not sorted, unique, and qualified").into());
    }
    Ok(())
}

/// Check the type parameters of a model declaration and the types it
/// writes over them.
pub(super) fn check_signature(
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
            return Err(interface(format!(
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

/// Check a use's member kind and type arguments, returning its
/// substitution over the used declaration's type parameters.
pub(super) fn use_substitution(
    model_use: &ModelUse,
    type_parameters: &[String],
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
    what: &str,
) -> Result<BTreeMap<String, SemanticType>, SemanticFailure> {
    check_member(&model_use.member, env)?;
    if model_use.type_arguments.len() != type_parameters.len() {
        return Err(format!(
            "{what} `{}` expects {} type argument(s), received {}",
            member_key(&model_use.member),
            type_parameters.len(),
            model_use.type_arguments.len()
        )
        .into());
    }
    for argument in &model_use.type_arguments {
        check_type_argument(argument, env)?;
        check_type_parameters(argument, scope)?;
    }
    Ok(substitution(type_parameters, &model_use.type_arguments))
}

// ---------------------------------------------------------------------------
// Contracts and models at one instantiation.

/// One predicate applied at an instantiation.
#[derive(Debug, Clone)]
struct Predicate {
    member: MemberRef,
    type_arguments: Vec<SemanticType>,
}

impl Predicate {
    fn apply(&self, arguments: Vec<SemanticTerm>) -> SemanticTerm {
        call(&self.member, &self.type_arguments, arguments)
    }

    fn reference(&self) -> SemanticTerm {
        function_ref(&self.member, &self.type_arguments)
    }
}

/// A contract at one instantiation.
#[derive(Debug, Clone)]
struct ContractAt {
    input: ModelBinder,
    output: ModelBinder,
    state: Option<ContractState>,
    precondition: Option<Predicate>,
    postcondition: Option<Predicate>,
    invariant: Option<Predicate>,
    validators: BTreeMap<ContractPredicate, Predicate>,
}

impl ContractAt {
    fn new(info: &ContractInfo, type_arguments: &[SemanticType]) -> Self {
        let map = substitution(&info.type_parameters, type_arguments);
        let predicate = |member: &Option<MemberRef>| {
            member.as_ref().map(|member| Predicate {
                member: member.clone(),
                type_arguments: type_arguments.to_vec(),
            })
        };
        Self {
            input: ModelBinder {
                name: info.input.name.clone(),
                r#type: substitute_type(&info.input.r#type, &map),
            },
            output: ModelBinder {
                name: info.output.name.clone(),
                r#type: substitute_type(&info.output.r#type, &map),
            },
            state: info.state.as_ref().map(|state| ContractState {
                name: state.name.clone(),
                next: state.next.clone(),
                r#type: substitute_type(&state.r#type, &map),
            }),
            precondition: predicate(&info.precondition),
            postcondition: predicate(&info.postcondition),
            invariant: predicate(&info.invariant),
            validators: info
                .validators
                .iter()
                .map(|validator| {
                    (
                        validator.predicate,
                        Predicate {
                            member: validator.validator.clone(),
                            type_arguments: type_arguments.to_vec(),
                        },
                    )
                })
                .collect(),
        }
    }

    fn state_type(&self) -> Option<&SemanticType> {
        self.state.as_ref().map(|state| &state.r#type)
    }

    /// The parameters a predicate of this contract takes, named by the
    /// contract's binders.
    fn predicate_parameters(&self, predicate: ContractPredicate) -> Vec<SemanticParameter> {
        let input = parameter(&self.input.name, &self.input.r#type);
        let output = parameter(&self.output.name, &self.output.r#type);
        match (&self.state, predicate) {
            (None, ContractPredicate::Precondition) => vec![input],
            (None, ContractPredicate::Postcondition) => vec![input, output],
            (None, ContractPredicate::Invariant) => Vec::new(),
            (Some(state), ContractPredicate::Precondition) => {
                vec![parameter(&state.name, &state.r#type), input]
            }
            (Some(state), ContractPredicate::Postcondition) => vec![
                parameter(&state.name, &state.r#type),
                input,
                parameter(&state.next, &state.r#type),
                output,
            ],
            (Some(state), ContractPredicate::Invariant) => {
                vec![parameter(&state.name, &state.r#type)]
            }
        }
    }

    fn predicate(&self, predicate: ContractPredicate) -> Option<&Predicate> {
        match predicate {
            ContractPredicate::Precondition => self.precondition.as_ref(),
            ContractPredicate::Postcondition => self.postcondition.as_ref(),
            ContractPredicate::Invariant => self.invariant.as_ref(),
        }
    }
}

/// The runtime checks an executable application of a model needs: each
/// predicate of its contract that its evidence does not discharge.
fn required_checks(contract: &ContractAt, discharged: &BTreeSet<ClaimKind>) -> Vec<ModelCheck> {
    let mut out = Vec::new();
    if contract.invariant.is_some() {
        out.push(ModelCheck::InputInvariant);
        if !discharged.contains(&ClaimKind::PreservesInvariant) {
            out.push(ModelCheck::OutputInvariant);
        }
    }
    if contract.postcondition.is_some() && !discharged.contains(&ClaimKind::SatisfiesContract) {
        out.push(ModelCheck::Postcondition);
    }
    if contract.precondition.is_some() {
        out.push(ModelCheck::Precondition);
    }
    out
}

/// A model use resolved at its instantiation.
#[derive(Debug, Clone)]
struct ModelAt {
    member: MemberRef,
    type_arguments: Vec<SemanticType>,
    contract: ContractAt,
    discharged: BTreeSet<ClaimKind>,
}

impl ModelAt {
    fn required(&self) -> Vec<ModelCheck> {
        required_checks(&self.contract, &self.discharged)
    }

    fn apply(&self, arguments: Vec<SemanticTerm>) -> SemanticTerm {
        call(&self.member, &self.type_arguments, arguments)
    }
}

fn resolve_model(
    model_use: &ModelUse,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
    failure: fn(String) -> SemanticFailure,
) -> Result<ModelAt, SemanticFailure> {
    let key = member_key(&model_use.member);
    let Some(info) = env.models.models.get(&key) else {
        return Err(failure(format!("`{key}` is not a prior model")));
    };
    let map = use_substitution(model_use, &info.type_parameters, scope, env, "model")?;
    let contract_use = ModelUse {
        member: info.contract.member.clone(),
        type_arguments: info
            .contract
            .type_arguments
            .iter()
            .map(|argument| substitute_type(argument, &map))
            .collect(),
    };
    let contract = env
        .models
        .contracts
        .get(&member_key(&contract_use.member))
        .ok_or_else(|| failure(format!("model `{key}` names a missing contract")))?;
    Ok(ModelAt {
        member: model_use.member.clone(),
        type_arguments: model_use.type_arguments.clone(),
        contract: ContractAt::new(contract, &contract_use.type_arguments),
        discharged: info.discharged.clone(),
    })
}

fn resolve_contract(
    contract_use: &ModelUse,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
    failure: fn(String) -> SemanticFailure,
) -> Result<ContractAt, SemanticFailure> {
    let key = member_key(&contract_use.member);
    let Some(info) = env.models.contracts.get(&key) else {
        return Err(failure(format!("`{key}` is not a prior contract")));
    };
    let _ = use_substitution(contract_use, &info.type_parameters, scope, env, "contract")?;
    Ok(ContractAt::new(info, &contract_use.type_arguments))
}

/// A realization at one instantiation.
#[derive(Debug, Clone)]
struct RealizationAt {
    member: MemberRef,
    type_arguments: Vec<SemanticType>,
    input: ModelBinder,
    output: SemanticType,
    state: Option<SemanticType>,
    state_binder: Option<String>,
    effective: Vec<SemanticTerm>,
}

impl RealizationAt {
    fn apply(&self, arguments: Vec<SemanticTerm>) -> SemanticTerm {
        call(&self.member, &self.type_arguments, arguments)
    }

    fn initial(&self) -> SemanticTerm {
        call(
            &MemberRef {
                module: self.member.module.clone(),
                name: format!("{}.initial", self.member.name),
            },
            &self.type_arguments,
            Vec::new(),
        )
    }

    /// The function type an equivalent reference must have.
    fn signature(&self) -> (Vec<SemanticType>, SemanticType) {
        match &self.state {
            Some(state) => (
                vec![state.clone(), self.input.r#type.clone()],
                product(state.clone(), self.output.clone()),
            ),
            None => (vec![self.input.r#type.clone()], self.output.clone()),
        }
    }
}

fn resolve_realization(
    realization_use: &ModelUse,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
    failure: fn(String) -> SemanticFailure,
) -> Result<RealizationAt, SemanticFailure> {
    let key = member_key(&realization_use.member);
    let Some(info) = env.models.realizations.get(&key) else {
        return Err(failure(format!("`{key}` is not a prior realization")));
    };
    let map = use_substitution(
        realization_use,
        &info.type_parameters,
        scope,
        env,
        "realization",
    )?;
    Ok(RealizationAt {
        member: realization_use.member.clone(),
        type_arguments: realization_use.type_arguments.clone(),
        input: ModelBinder {
            name: info.input.name.clone(),
            r#type: substitute_type(&info.input.r#type, &map),
        },
        output: substitute_type(&info.output, &map),
        state: info
            .state
            .as_ref()
            .map(|state| substitute_type(state, &map)),
        state_binder: info.state_binder.clone(),
        effective: info
            .effective
            .iter()
            .map(|term| substitute_term_types(term, &map))
            .collect(),
    })
}

/// The reason a contract and a realization do not share one interface.
fn interface_mismatch(contract: &ContractAt, realization: &RealizationAt) -> Option<String> {
    if contract.input.r#type != realization.input.r#type {
        return Some(format!(
            "input {} differs from the contract's {}",
            realization.input.r#type, contract.input.r#type
        ));
    }
    if contract.output.r#type != realization.output {
        return Some(format!(
            "output {} differs from the contract's {}",
            realization.output, contract.output.r#type
        ));
    }
    match (contract.state_type(), &realization.state) {
        (None, None) => None,
        (Some(expected), Some(observed)) if expected == observed => None,
        (expected, observed) => Some(format!(
            "state {} differs from the contract's {}",
            observed
                .as_ref()
                .map_or_else(|| "none".to_owned(), ToString::to_string),
            expected.map_or_else(|| "none".to_owned(), ToString::to_string)
        )),
    }
}

// ---------------------------------------------------------------------------
// Contracts.

const fn predicate_name(predicate: ContractPredicate) -> &'static str {
    match predicate {
        ContractPredicate::Invariant => "invariant",
        ContractPredicate::Postcondition => "postcondition",
        ContractPredicate::Precondition => "precondition",
    }
}

/// The function at `member` with `type_parameters` instantiated at the
/// contract's own parameters, as `(parameters, result, executable)`.
pub(super) fn signature_at(
    member: &MemberRef,
    type_parameters: &[String],
    env: &Environment<'_>,
    what: &str,
) -> Result<(Vec<SemanticType>, SemanticType, bool), SemanticFailure> {
    check_member(member, env)?;
    let Some(info) = function_info(member, env) else {
        return Err(interface(format!(
            "{what} `{}` is not a prior definition",
            member_key(member)
        )));
    };
    if info.type_parameters.len() != type_parameters.len() {
        return Err(interface(format!(
            "{what} `{}` has {} type parameter(s); it must have exactly the {} of its contract",
            member_key(member),
            info.type_parameters.len(),
            type_parameters.len()
        )));
    }
    let (parameters, result) = super::instantiate(info, &parameter_types(type_parameters));
    Ok((parameters, result, info.executable))
}

fn check_contract(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::Contract {
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
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_axioms(name, axioms)?;
    let mut binders = vec![input.name.as_str(), output.name.as_str()];
    let mut types = vec![&input.r#type, &output.r#type];
    if let Some(state) = state {
        binders.push(&state.name);
        binders.push(&state.next);
        types.push(&state.r#type);
    }
    check_signature(name, type_parameters, &binders, &types, env)?;
    if invariant.is_some() && state.is_none() {
        return Err(interface(format!(
            "contract `{name}` states an invariant but no state"
        )));
    }
    if precondition.is_none() && postcondition.is_none() && invariant.is_none() {
        return Err(interface(format!(
            "contract `{name}` names no precondition, postcondition, or invariant"
        )));
    }
    let info = ContractInfo {
        type_parameters: type_parameters.clone(),
        input: input.clone(),
        output: output.clone(),
        state: state.clone(),
        precondition: precondition.clone(),
        postcondition: postcondition.clone(),
        invariant: invariant.clone(),
        validators: validators.clone(),
    };
    let at = ContractAt::new(&info, &parameter_types(type_parameters));
    for predicate in [
        ContractPredicate::Precondition,
        ContractPredicate::Postcondition,
        ContractPredicate::Invariant,
    ] {
        let Some(member) = at.predicate(predicate) else {
            continue;
        };
        let what = format!("contract `{name}` {}", predicate_name(predicate));
        let (parameters, result, _) = signature_at(&member.member, type_parameters, env, &what)?;
        let expected: Vec<SemanticType> = at
            .predicate_parameters(predicate)
            .into_iter()
            .map(|parameter| parameter.r#type)
            .collect();
        if parameters != expected || result != SemanticType::Prop {
            return Err(interface(format!(
                "{what} `{}` must take ({}) to Prop, but takes ({}) to {result}",
                member_key(&member.member),
                expected
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", "),
                parameters
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )));
        }
    }
    let mut lowering = Lowering::default();
    let mut previous = None;
    for validator in validators {
        if previous.is_some_and(|prior| prior >= validator.predicate) {
            return Err(interface(format!(
                "contract `{name}` validators are not strictly sorted by predicate"
            )));
        }
        previous = Some(validator.predicate);
        let predicate_label = predicate_name(validator.predicate);
        let Some(predicate) = at.predicate(validator.predicate) else {
            return Err(interface(format!(
                "contract `{name}` validates the absent {predicate_label}"
            )));
        };
        let what = format!("contract `{name}` {predicate_label} validator");
        let (parameters, result, executable) =
            signature_at(&validator.validator, type_parameters, env, &what)?;
        let expected = at.predicate_parameters(validator.predicate);
        let expected_types: Vec<SemanticType> = expected
            .iter()
            .map(|parameter| parameter.r#type.clone())
            .collect();
        if parameters != expected_types || result != SemanticType::Bool {
            return Err(interface(format!(
                "{what} `{}` must take the predicate's parameters to Bool",
                member_key(&validator.validator)
            )));
        }
        if !executable {
            return Err(interface(format!(
                "{what} `{}` is not executable, so it cannot run at a runtime boundary",
                member_key(&validator.validator)
            )));
        }
        let arguments: Vec<SemanticTerm> = expected
            .iter()
            .map(|parameter| var(&parameter.name))
            .collect();
        let check = Predicate {
            member: validator.validator.clone(),
            type_arguments: parameter_types(type_parameters),
        };
        let accepted = eq(check.apply(arguments.clone()), boolean(true));
        let holds = predicate.apply(arguments);
        let arity = expected.len();
        for (role, theorem, statement, helper) in [
            (
                "sound",
                Some(&validator.sound),
                implies(accepted.clone(), holds.clone()),
                format!("Sound{arity}"),
            ),
            (
                "complete",
                validator.complete.as_ref(),
                implies(holds, accepted),
                format!("Complete{arity}"),
            ),
        ] {
            let Some(theorem) = theorem else {
                continue;
            };
            let obligation = Obligation {
                role: format!("validator {predicate_label} {role}"),
                theorem: theorem.clone(),
                type_parameters: type_parameters.clone(),
                parameters: expected.clone(),
                statement,
            };
            require_statement(env, theorem, &obligation, code!("LLT4006"))?;
            lowering.checks.push(CrossCheck {
                name: format!("{name}.{predicate_label}_{role}"),
                type_parameters: type_parameters.clone(),
                statement: CheckStatement::Helper {
                    helper,
                    arguments: vec![check.reference(), predicate.reference()],
                },
                proof: CheckProof::Theorem {
                    theorem: theorem.clone(),
                    type_arguments: parameter_types(type_parameters),
                },
            });
            lowering.obligations.push(obligation);
        }
    }
    env.models.contracts.insert(name.clone(), info);
    Ok(lowering)
}

// ---------------------------------------------------------------------------
// Artifacts.

/// The byte width and signedness of a tensor element.
#[must_use]
pub const fn element_encoding(element: TensorElement) -> (usize, bool) {
    match element {
        TensorElement::Int8 => (1, true),
        TensorElement::Int16 => (2, true),
        TensorElement::Int32 => (4, true),
        TensorElement::Int64 => (8, true),
        TensorElement::UInt8 => (1, false),
        TensorElement::UInt16 => (2, false),
        TensorElement::UInt32 => (4, false),
        TensorElement::UInt64 => (8, false),
    }
}

/// The value type a schema decodes to.
fn schema_type(schema: &ArtifactSchema) -> SemanticType {
    match schema {
        ArtifactSchema::Bytes => SemanticType::Bytes,
        ArtifactSchema::IntTensor { shape, .. } => {
            let mut ty = SemanticType::Int;
            for _ in shape {
                ty = list_type(ty);
            }
            ty
        }
        ArtifactSchema::Utf8Lines => list_type(SemanticType::String),
    }
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(out, "{byte:02x}").expect("writing to a string");
    }
    out
}

/// Nest `values` row-major under `shape`.
fn nest(shape: &[u64], values: &[i128]) -> SemanticTerm {
    // The element type of each level, innermost first, built once.
    let mut types = vec![SemanticType::Int];
    for _ in 1..shape.len() {
        let inner = types[types.len() - 1].clone();
        types.push(list_type(inner));
    }
    nest_level(shape, values, &types)
}

fn nest_level(shape: &[u64], values: &[i128], types: &[SemanticType]) -> SemanticTerm {
    match shape {
        [] | [_] => list_literal(
            &SemanticType::Int,
            values.iter().map(|value| int(&value.to_string())).collect(),
        ),
        [rows, rest @ ..] => {
            let stride = values.len() / usize::try_from(*rows).unwrap_or(1).max(1);
            list_literal(
                &types[rest.len()],
                values
                    .chunks(stride.max(1))
                    .map(|chunk| nest_level(rest, chunk, types))
                    .collect(),
            )
        }
    }
}

/// The exact node count of the list literal [`list_literal`] builds from
/// `items` elements of `item` nodes each, whose element type has `element`
/// nodes.
fn list_literal_nodes(items: u64, item: u64, element: u64) -> u64 {
    const CHUNK: u64 = 32;
    if items <= CHUNK {
        // A cons per element, then the typed nil.
        return items
            .saturating_mul(item.saturating_add(1))
            .saturating_add(1 + element);
    }
    let half = items.div_ceil(CHUNK).div_ceil(2) * CHUNK;
    // The append primitive and its list result type, over the two halves.
    (2 + element)
        .saturating_add(list_literal_nodes(half, item, element))
        .saturating_add(list_literal_nodes(items - half, item, element))
}

/// The exact node count of the value [`nest`] builds under `shape`.
fn nest_nodes(shape: &[u64]) -> u64 {
    match shape.split_first() {
        Some((rows, rest)) if !rest.is_empty() => {
            list_literal_nodes(*rows, nest_nodes(rest), rest.len() as u64 + 1)
        }
        // As in `nest_level`, the innermost level lists its values.
        _ => list_literal_nodes(
            shape
                .iter()
                .fold(1u64, |total, extent| total.saturating_mul(*extent)),
            1,
            1,
        ),
    }
}

/// Check an artifact's bytes against its schema without decoding them, and
/// return the exact node count of the value [`decode`] would build: the
/// count is charged before anything is allocated (§17.12 rule 2).
fn decoded_nodes(schema: &ArtifactSchema, bytes: &[u8]) -> Result<u64, String> {
    match schema {
        ArtifactSchema::Bytes => Ok(1),
        ArtifactSchema::IntTensor { element, shape } => {
            if shape.is_empty() || shape.contains(&0) {
                return Err(
                    "an integer tensor has at least one dimension and no empty one".to_owned(),
                );
            }
            let (width, _) = element_encoding(*element);
            let count = shape
                .iter()
                .try_fold(1usize, |total, dimension| {
                    usize::try_from(*dimension)
                        .ok()
                        .and_then(|dimension| total.checked_mul(dimension))
                })
                .ok_or("the tensor shape's element count overflows")?;
            let expected = count
                .checked_mul(width)
                .ok_or("the tensor shape's byte length overflows")?;
            if bytes.len() != expected {
                return Err(format!(
                    "{} bytes cannot hold {count} elements of {width} byte(s) each ({expected} bytes)",
                    bytes.len()
                ));
            }
            Ok(nest_nodes(shape))
        }
        ArtifactSchema::Utf8Lines => {
            let text = std::str::from_utf8(bytes)
                .map_err(|error| format!("the bytes are not UTF-8: {error}"))?;
            if text.contains('\r') {
                return Err("UTF-8 lines contain a carriage return".to_owned());
            }
            if !text.is_empty() && !text.ends_with('\n') {
                return Err("the last UTF-8 line is not LF-terminated".to_owned());
            }
            let lines = bytes.iter().filter(|byte| **byte == b'\n').count() as u64;
            Ok(list_literal_nodes(lines, 1, 1))
        }
    }
}

/// Decode an artifact's bytes under its schema, or say why they violate it.
fn decode(schema: &ArtifactSchema, bytes: &[u8]) -> Result<SemanticTerm, String> {
    match schema {
        ArtifactSchema::Bytes => Ok(SemanticTerm::Bytes { hex: hex(bytes) }),
        ArtifactSchema::IntTensor { element, shape } => {
            if shape.is_empty() || shape.contains(&0) {
                return Err(
                    "an integer tensor has at least one dimension and no empty one".to_owned(),
                );
            }
            let (width, signed) = element_encoding(*element);
            let count = shape
                .iter()
                .try_fold(1usize, |total, dimension| {
                    usize::try_from(*dimension)
                        .ok()
                        .and_then(|dimension| total.checked_mul(dimension))
                })
                .ok_or("the tensor shape's element count overflows")?;
            let expected = count
                .checked_mul(width)
                .ok_or("the tensor shape's byte length overflows")?;
            if bytes.len() != expected {
                return Err(format!(
                    "{} bytes cannot hold {count} elements of {width} byte(s) each ({expected} bytes)",
                    bytes.len()
                ));
            }
            let values: Vec<i128> = bytes
                .chunks_exact(width)
                .map(|chunk| {
                    let mut magnitude: u128 = 0;
                    for (index, byte) in chunk.iter().enumerate() {
                        magnitude |= u128::from(*byte) << (8 * index);
                    }
                    let bits = 8 * width;
                    let value = i128::try_from(magnitude).expect("at most 64 bits");
                    if signed && magnitude >> (bits - 1) == 1 {
                        value - (1i128 << bits)
                    } else {
                        value
                    }
                })
                .collect();
            Ok(nest(shape, &values))
        }
        ArtifactSchema::Utf8Lines => {
            let text = std::str::from_utf8(bytes)
                .map_err(|error| format!("the bytes are not UTF-8: {error}"))?;
            if text.contains('\r') {
                return Err("UTF-8 lines contain a carriage return".to_owned());
            }
            if !text.is_empty() && !text.ends_with('\n') {
                return Err("the last UTF-8 line is not LF-terminated".to_owned());
            }
            let lines: Vec<SemanticTerm> = if text.is_empty() {
                Vec::new()
            } else {
                text[..text.len() - 1]
                    .split('\n')
                    .map(|line| SemanticTerm::String {
                        value: line.to_owned(),
                    })
                    .collect()
            };
            Ok(list_literal(&SemanticType::String, lines))
        }
    }
}

fn role_admits(role: ArtifactRole, schema: &ArtifactSchema) -> bool {
    match role {
        ArtifactRole::Parameters | ArtifactRole::Table => {
            matches!(schema, ArtifactSchema::IntTensor { .. })
        }
        ArtifactRole::Vocabulary => matches!(schema, ArtifactSchema::Utf8Lines),
        ArtifactRole::Binary => matches!(schema, ArtifactSchema::Bytes),
        ArtifactRole::Dataset => true,
    }
}

fn ordinary_definition(
    name: String,
    type_parameters: &[String],
    parameters: Vec<SemanticParameter>,
    result: SemanticType,
    body: SemanticTerm,
    executable: bool,
) -> SemanticDeclaration {
    SemanticDeclaration::Definition {
        name,
        type_parameters: type_parameters.to_vec(),
        parameters,
        result,
        recursive_argument: None,
        body,
        axioms: Vec::new(),
        executable,
        mutual: None,
        termination: None,
        production: None,
    }
}

/// The most dimensions an integer tensor artifact may declare: a closed
/// language bound, so a shape is never an unbounded input (§17.12 rule 2).
pub const MAX_TENSOR_RANK: usize = 16;

fn check_artifact(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
    artifacts: &ArtifactStore,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::Artifact {
        name,
        role,
        sha256,
        length,
        schema,
        r#type,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_axioms(name, axioms)?;
    if sha256.len() != 64
        || !sha256
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(format!(
            "artifact `{name}` SHA-256 `{sha256}` is not 64 lowercase hexadecimal digits"
        )
        .into());
    }
    let Some(bytes) = artifacts.get(sha256) else {
        return Err(fail(
            code!("LLR3007"),
            format!("artifact `{name}` has no project artifact source with SHA-256 {sha256}"),
        ));
    };
    let observed = crate::artifact::content_id::Sha256Digest::of(bytes).to_hex();
    if &observed != sha256 {
        return Err(fail(
            code!("LLR3007"),
            format!("artifact `{name}` bytes have SHA-256 {observed}, not the declared {sha256}"),
        ));
    }
    if u64::try_from(bytes.len()).ok() != Some(*length) {
        return Err(fail(
            code!("LLR3007"),
            format!(
                "artifact `{name}` has {} bytes, not the declared length {length}",
                bytes.len()
            ),
        ));
    }
    let schema_violation =
        |reason: String| fail(code!("LLR3008"), format!("artifact `{name}`: {reason}"));
    if !role_admits(*role, schema) {
        return Err(schema_violation(format!(
            "the role {role:?} does not admit the {} schema",
            schema_kind(schema)
        )));
    }
    // The shape is bounded and the declared type checked before any byte is
    // decoded (§10.2, R5).
    if let ArtifactSchema::IntTensor { shape, .. } = schema {
        if shape.len() > MAX_TENSOR_RANK {
            return Err(schema_violation(format!(
                "an integer tensor has at most {MAX_TENSOR_RANK} dimensions, not {}",
                shape.len()
            )));
        }
    }
    let expected = schema_type(schema);
    if r#type != &expected {
        return Err(schema_violation(format!(
            "the declared type {} is not the schema's value type {expected}",
            r#type
        )));
    }
    // The bytes are checked against the schema and the decoded value's
    // exact size charged, with the bytes it materializes again, before
    // anything is allocated: the charge runs across every declaration, so
    // naming one digest many times cannot amplify (§10.2, R5).
    let nodes = decoded_nodes(schema, bytes).map_err(schema_violation)?;
    artifacts.charge(name, *length, nodes)?;
    let decoded = decode(schema, bytes).map_err(schema_violation)?;
    let lowering = Lowering {
        declarations: vec![
            ordinary_definition(
                format!("{name}.bytes"),
                &[],
                Vec::new(),
                SemanticType::Bytes,
                SemanticTerm::Bytes { hex: hex(bytes) },
                false,
            ),
            ordinary_definition(name.clone(), &[], Vec::new(), expected, decoded, true),
        ],
        checks: vec![CrossCheck {
            name: format!("{name}.decoded"),
            type_parameters: Vec::new(),
            statement: match schema {
                ArtifactSchema::Bytes => CheckStatement::Bytes {
                    artifact: name.clone(),
                },
                ArtifactSchema::IntTensor { element, shape } => CheckStatement::Tensor {
                    artifact: name.clone(),
                    element: *element,
                    shape: shape.clone(),
                },
                ArtifactSchema::Utf8Lines => CheckStatement::Lines {
                    artifact: name.clone(),
                },
            },
            proof: CheckProof::Decide,
        }],
        ..Lowering::default()
    };
    env.models.artifacts.insert(
        name.clone(),
        ArtifactInfo {
            role: *role,
            schema: schema.clone(),
        },
    );
    Ok(lowering)
}

/// The serialized kind of a schema.
#[must_use]
pub const fn schema_kind(schema: &ArtifactSchema) -> &'static str {
    match schema {
        ArtifactSchema::Bytes => "bytes",
        ArtifactSchema::IntTensor { .. } => "int_tensor",
        ArtifactSchema::Utf8Lines => "utf8_lines",
    }
}

/// An artifact used as a parameter tensor of exactly `shape`.
fn parameter_slot(
    member: &MemberRef,
    shape: &[u64],
    env: &Environment<'_>,
    what: &str,
) -> Result<SemanticTerm, SemanticFailure> {
    check_member(member, env)?;
    let key = member_key(member);
    let Some(info) = env.models.artifacts.get(&key) else {
        return Err(interface(format!("{what} `{key}` is not a prior artifact")));
    };
    if info.role != ArtifactRole::Parameters {
        return Err(interface(format!(
            "{what} `{key}` has role {:?}; a parameter slot needs role Parameters",
            info.role
        )));
    }
    match &info.schema {
        ArtifactSchema::IntTensor {
            shape: observed, ..
        } if observed == shape => Ok(call(member, &[], Vec::new())),
        other => Err(interface(format!(
            "{what} `{key}` is a {} {}; the slot needs an integer tensor of shape {shape:?}",
            schema_kind(other),
            match other {
                ArtifactSchema::IntTensor { shape, .. } => format!("of shape {shape:?}"),
                ArtifactSchema::Bytes | ArtifactSchema::Utf8Lines => String::new(),
            }
        ))),
    }
}

// ---------------------------------------------------------------------------
// The exact integer kernels, as ordinary language-1.2 terms.

/// `left + right` over mathematical integers.
fn int_add(left: SemanticTerm, right: SemanticTerm) -> SemanticTerm {
    primitive(
        SemanticPrimitive::Subtract,
        vec![
            left,
            primitive(SemanticPrimitive::Negate, vec![right], SemanticType::Int),
        ],
        SemanticType::Int,
    )
}

fn less_than(left: SemanticTerm, right: SemanticTerm) -> SemanticTerm {
    primitive(
        SemanticPrimitive::LessThan,
        vec![left, right],
        SemanticType::Bool,
    )
}

pub(super) fn append_one(
    list: SemanticTerm,
    element: &SemanticType,
    value: SemanticTerm,
) -> SemanticTerm {
    primitive(
        SemanticPrimitive::Append,
        vec![list, list_literal(element, vec![value])],
        list_type(element.clone()),
    )
}

pub(super) fn fold(
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

/// The dot product of `row` and `values`, pairing positions (zip).
fn dot(row: SemanticTerm, values: SemanticTerm) -> SemanticTerm {
    let state = product(SemanticType::Int, int_list());
    first(fold(
        lambda(
            vec![("__sum", state.clone()), ("__weight", SemanticType::Int)],
            matching(
                second(var("__sum")),
                vec![
                    ("List.nil", Vec::new(), var("__sum")),
                    (
                        "List.cons",
                        vec!["__value", "__rest"],
                        pair(
                            int_add(
                                first(var("__sum")),
                                primitive(
                                    SemanticPrimitive::Multiply,
                                    vec![var("__weight"), var("__value")],
                                    SemanticType::Int,
                                ),
                            ),
                            var("__rest"),
                        ),
                    ),
                ],
            ),
        ),
        pair(int("0"), values),
        row,
        state,
    ))
}

/// `W values + b`: one output per row of `W` paired with `b`.
fn dense(weights: SemanticTerm, bias: SemanticTerm, values: &str) -> SemanticTerm {
    let state = product(int_list(), int_list());
    first(fold(
        lambda(
            vec![("__acc", state.clone()), ("__row", int_list())],
            matching(
                second(var("__acc")),
                vec![
                    ("List.nil", Vec::new(), var("__acc")),
                    (
                        "List.cons",
                        vec!["__bias", "__biases"],
                        pair(
                            append_one(
                                first(var("__acc")),
                                &SemanticType::Int,
                                int_add(var("__bias"), dot(var("__row"), var(values))),
                            ),
                            var("__biases"),
                        ),
                    ),
                ],
            ),
        ),
        pair(
            SemanticTerm::Nil {
                element: SemanticType::Int,
            },
            bias,
        ),
        weights,
        state,
    ))
}

/// Apply `map` to every value, in order.
fn each(values: &str, map: impl FnOnce(SemanticTerm) -> SemanticTerm) -> SemanticTerm {
    fold(
        lambda(
            vec![("__out", int_list()), ("__value", SemanticType::Int)],
            append_one(var("__out"), &SemanticType::Int, map(var("__value"))),
        ),
        SemanticTerm::Nil {
            element: SemanticType::Int,
        },
        var(values),
        int_list(),
    )
}

fn relu(values: &str) -> SemanticTerm {
    each(values, |value| {
        if_then(less_than(value.clone(), int("0")), int("0"), value)
    })
}

/// `2^shift` in canonical decimal.
fn power_of_two(shift: u64) -> String {
    let mut digits: Vec<u8> = vec![1];
    for _ in 0..shift {
        let mut carry = 0;
        for digit in &mut digits {
            let doubled = *digit * 2 + carry;
            *digit = doubled % 10;
            carry = doubled / 10;
        }
        if carry > 0 {
            digits.push(carry);
        }
    }
    digits
        .iter()
        .rev()
        .map(|digit| char::from(b'0' + digit))
        .collect()
}

/// Truncating division by `2^shift`, clamped to `[minimum, maximum]`.
fn requantize(values: &str, shift: u64, minimum: &str, maximum: &str) -> SemanticTerm {
    let divisor = power_of_two(shift);
    each(values, |value| {
        let_in(
            "__quotient",
            SemanticType::Int,
            primitive(
                SemanticPrimitive::Quotient,
                vec![value, int(&divisor), int("0")],
                SemanticType::Int,
            ),
            if_then(
                less_than(var("__quotient"), int(minimum)),
                int(minimum),
                if_then(
                    less_than(int(maximum), var("__quotient")),
                    int(maximum),
                    var("__quotient"),
                ),
            ),
        )
    })
}

/// The label paired with the first maximal score, the first label if the
/// scores are empty.
fn argmax(labels: &[SemanticTerm], label: &SemanticType, scores: SemanticTerm) -> SemanticTerm {
    let leader = product(label.clone(), SemanticType::Int);
    let state = product(option_type(leader.clone()), list_type(label.clone()));
    let some = |value: SemanticTerm| constructor("Option.some", vec![leader.clone()], vec![value]);
    matching(
        first(fold(
            lambda(
                vec![("__best", state.clone()), ("__score", SemanticType::Int)],
                matching(
                    second(var("__best")),
                    vec![
                        ("List.nil", Vec::new(), var("__best")),
                        (
                            "List.cons",
                            vec!["__label", "__labels"],
                            pair(
                                matching(
                                    first(var("__best")),
                                    vec![
                                        (
                                            "Option.none",
                                            Vec::new(),
                                            some(pair(var("__label"), var("__score"))),
                                        ),
                                        (
                                            "Option.some",
                                            vec!["__leader"],
                                            if_then(
                                                less_than(second(var("__leader")), var("__score")),
                                                some(pair(var("__label"), var("__score"))),
                                                some(var("__leader")),
                                            ),
                                        ),
                                    ],
                                ),
                                var("__labels"),
                            ),
                        ),
                    ],
                ),
            ),
            pair(
                constructor("Option.none", vec![leader.clone()], Vec::new()),
                list_literal(label, labels.to_vec()),
            ),
            scores,
            state,
        )),
        vec![
            ("Option.none", Vec::new(), labels[0].clone()),
            ("Option.some", vec!["__winner"], first(var("__winner"))),
        ],
    )
}

/// Compare two canonical decimal integers.
fn compare_decimal(left: &str, right: &str) -> std::cmp::Ordering {
    let sign = |value: &str| value.starts_with('-');
    let magnitude = |value: &str| value.trim_start_matches('-').to_owned();
    match (sign(left), sign(right)) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        (negative, _) => {
            let (a, b) = (magnitude(left), magnitude(right));
            let order = a.len().cmp(&b.len()).then_with(|| a.cmp(&b));
            if negative {
                order.reverse()
            } else {
                order
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Realizations.

/// Labels are closed values of the output type: literals or constructors
/// over literals, never a term over the input.
fn check_labels(name: &str, labels: &[SemanticTerm]) -> Result<(), SemanticFailure> {
    if labels.is_empty() {
        return Err(interface(format!("realization `{name}` has no labels")));
    }
    for label in labels {
        let mut free = BTreeSet::new();
        free_locals(label, &mut BTreeSet::new(), &mut free);
        if let Some(local) = free.first() {
            return Err(interface(format!(
                "realization `{name}` label mentions the local `{local}`; a label is a closed value"
            )));
        }
    }
    Ok(())
}

/// The width obligation of an encoder or feature term over the input.
fn width_obligation(
    name: &str,
    type_parameters: &[String],
    input: &ModelBinder,
    term: &SemanticTerm,
    width: u64,
    evidence: &MemberRef,
    env: &Environment<'_>,
) -> Result<Obligation, SemanticFailure> {
    if width == 0 {
        return Err(interface(format!("realization `{name}` declares width 0")));
    }
    let obligation = Obligation {
        role: format!("realization `{name}` width"),
        theorem: evidence.clone(),
        type_parameters: type_parameters.to_vec(),
        parameters: vec![parameter(&input.name, &input.r#type)],
        statement: eq(
            primitive(
                SemanticPrimitive::Length,
                vec![term.clone()],
                SemanticType::Nat,
            ),
            nat(width),
        ),
    };
    require_statement(env, evidence, &obligation, code!("LLT4006"))?;
    Ok(obligation)
}

/// One stage of a composite at its instantiation. A stage's checks after
/// it runs (its output invariant and postcondition, unless its evidence
/// discharges them) run right after it, so each needs a sound validator.
/// Every form takes stateless and stateful stages alike, except a scan,
/// which threads exactly one stateful stage (`scan`).
fn composite_stage(
    name: &str,
    stage: &ModelUse,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
    scan: bool,
) -> Result<ModelAt, SemanticFailure> {
    let at = resolve_model(stage, scope, env, composition)?;
    let key = member_key(&stage.member);
    if scan && at.contract.state.is_none() {
        return Err(composition(format!(
            "composite `{name}` stage `{key}` is stateless; a scan threads one stateful stage"
        )));
    }
    for check in post_run(&at) {
        let predicate = match check {
            ModelCheck::OutputInvariant => ContractPredicate::Invariant,
            _ => ContractPredicate::Postcondition,
        };
        if !at.contract.validators.contains_key(&predicate) {
            return Err(composition(format!(
                "composite `{name}` stage `{key}` has no evidence discharging its {} and no sound validator to check it at run time",
                check_name_of(check)
            )));
        }
    }
    Ok(at)
}

/// The checks a stage needs after it runs, in the fixed order.
fn post_run(stage: &ModelAt) -> Vec<ModelCheck> {
    let required = stage.required();
    [ModelCheck::OutputInvariant, ModelCheck::Postcondition]
        .into_iter()
        .filter(|check| required.contains(check))
        .collect()
}

/// `rest`, guarded by a stage's checks after it runs: the output invariant
/// of its new state, then its postcondition; a refusal is `refuse(check)`.
fn guard_after(
    stage: &ModelAt,
    state: Option<(SemanticTerm, SemanticTerm)>,
    input: &SemanticTerm,
    output: &SemanticTerm,
    rest: SemanticTerm,
    refuse: &dyn Fn(ModelCheck) -> SemanticTerm,
) -> SemanticTerm {
    let checks = post_run(stage);
    let mut body = rest;
    if checks.contains(&ModelCheck::Postcondition) {
        let validator = &stage.contract.validators[&ContractPredicate::Postcondition];
        let arguments = match &state {
            Some((before, after)) => {
                vec![before.clone(), input.clone(), after.clone(), output.clone()]
            }
            None => vec![input.clone(), output.clone()],
        };
        body = if_then(
            validator.apply(arguments),
            body,
            refuse(ModelCheck::Postcondition),
        );
    }
    if let (true, Some((_, after))) = (checks.contains(&ModelCheck::OutputInvariant), &state) {
        let validator = &stage.contract.validators[&ContractPredicate::Invariant];
        body = if_then(
            validator.apply(vec![after.clone()]),
            body,
            refuse(ModelCheck::OutputInvariant),
        );
    }
    body
}

/// `rest`, guarded by a stage's validators before it runs: the input
/// invariant of its state, then its precondition.
fn guard_before(
    stage: &ModelAt,
    state: Option<&SemanticTerm>,
    input: &SemanticTerm,
    rest: SemanticTerm,
    refuse: &dyn Fn(ModelCheck) -> SemanticTerm,
) -> SemanticTerm {
    let mut body = rest;
    if let Some(validator) = stage
        .contract
        .validators
        .get(&ContractPredicate::Precondition)
    {
        let arguments = match state {
            Some(state) => vec![state.clone(), input.clone()],
            None => vec![input.clone()],
        };
        body = if_then(
            validator.apply(arguments),
            body,
            refuse(ModelCheck::Precondition),
        );
    }
    if let (Some(validator), Some(state)) = (
        stage.contract.validators.get(&ContractPredicate::Invariant),
        state,
    ) {
        body = if_then(
            validator.apply(vec![state.clone()]),
            body,
            refuse(ModelCheck::InputInvariant),
        );
    }
    body
}

/// The right-nested product of `items` (one item is itself).
fn nested<T: Clone>(items: &[T], pair_of: &dyn Fn(T, T) -> T) -> Option<T> {
    let (last, init) = items.split_last()?;
    Some(
        init.iter()
            .rev()
            .fold(last.clone(), |rest, item| pair_of(item.clone(), rest)),
    )
}

/// Component `index` of a right-nested product of `count` components.
fn component(value: &SemanticTerm, index: usize, count: usize) -> SemanticTerm {
    let mut out = value.clone();
    for _ in 0..index {
        out = second(out);
    }
    if index + 1 < count {
        first(out)
    } else {
        out
    }
}

struct CompositeLowering {
    body: SemanticTerm,
    effective: Vec<SemanticTerm>,
    obligations: Vec<Obligation>,
}

/// The state a sequence, fan-out, product, or branch threads: the
/// right-nested product of its stateful stages' states, in stage order.
/// Each stateful stage reads, and may replace, only its own component.
struct Threaded {
    /// The stage indices that are stateful, in stage order.
    stateful: Vec<usize>,
    /// The composite's state binder, when any stage is stateful.
    state: Option<SemanticTerm>,
}

impl Threaded {
    /// The component of the composite state that stage `index` reads.
    fn component(&self, index: usize) -> Option<SemanticTerm> {
        let position = self.stateful.iter().position(|stage| *stage == index)?;
        Some(component(
            self.state.as_ref()?,
            position,
            self.stateful.len(),
        ))
    }

    /// The composite's next state: each stage in `updated` replaces its
    /// component, every other component is carried unchanged.
    fn next(&self, updated: &[(usize, SemanticTerm)]) -> Option<SemanticTerm> {
        let components: Vec<SemanticTerm> = self
            .stateful
            .iter()
            .map(|index| {
                updated
                    .iter()
                    .find(|(stage, _)| stage == index)
                    .map(|(_, value)| value.clone())
                    .or_else(|| self.component(*index))
                    .expect("a stateful stage has a component")
            })
            .collect();
        nested(&components, &|left, right| pair(left, right))
    }

    /// The composite's result: its next state paired with `value`, or
    /// `value` alone when nothing is stateful.
    fn result(&self, updated: &[(usize, SemanticTerm)], value: SemanticTerm) -> SemanticTerm {
        match self.next(updated) {
            Some(state) => pair(state, value),
            None => value,
        }
    }

    /// A refusal: the state the composite was given, with the violation.
    fn refusal(&self, ok: &SemanticType, check: ModelCheck) -> SemanticTerm {
        let refused = refusal(ok, check);
        match &self.state {
            Some(state) => pair(state.clone(), refused),
            None => refused,
        }
    }
}

/// Check the state a composite declares against its stateful stages.
fn thread_state(
    name: &str,
    stages: &[&ModelAt],
    state: Option<&RealizationState>,
) -> Result<Threaded, SemanticFailure> {
    let mismatch = |what: String| composition(format!("composite `{name}`: {what}"));
    let stateful: Vec<usize> = (0..stages.len())
        .filter(|index| stages[*index].contract.state.is_some())
        .collect();
    let types: Vec<SemanticType> = stateful
        .iter()
        .filter_map(|index| stages[*index].contract.state_type().cloned())
        .collect();
    match (nested(&types, &|left, right| product(left, right)), state) {
        (None, None) => {}
        (None, Some(_)) => {
            return Err(mismatch(
                "no stage is stateful, so the composite declares no state".to_owned(),
            ));
        }
        (Some(expected), None) => {
            return Err(mismatch(format!(
                "its stateful stages thread a state of type {expected}, which it must declare"
            )));
        }
        (Some(expected), Some(declared)) => {
            if declared.r#type != expected {
                return Err(mismatch(format!(
                    "the state is {}, expected {expected}",
                    declared.r#type
                )));
            }
        }
    }
    Ok(Threaded {
        stateful,
        state: state.map(|state| var(&state.name)),
    })
}

/// The arguments of a stage's precondition: its state component, when
/// stateful, and its input.
fn stage_arguments(current: Option<&SemanticTerm>, input: &SemanticTerm) -> Vec<SemanticTerm> {
    match current {
        Some(current) => vec![current.clone(), input.clone()],
        None => vec![input.clone()],
    }
}

/// A stage's invariant (of its state component) and precondition, as
/// propositions the composite's entry establishes.
fn stage_requirements(
    stage: &ModelAt,
    current: Option<&SemanticTerm>,
    input: &SemanticTerm,
    invariant: bool,
) -> Vec<SemanticTerm> {
    let mut out = Vec::new();
    if let (true, Some(predicate), Some(current)) = (invariant, &stage.contract.invariant, current)
    {
        out.push(predicate.apply(vec![current.clone()]));
    }
    if let Some(predicate) = &stage.contract.precondition {
        out.push(predicate.apply(stage_arguments(current, input)));
    }
    out
}

/// The stage's application inside `body`, built inside-out: a stateful
/// stage binds its step, its output, and its new state; the stage's checks
/// after it runs guard `body`.
#[allow(clippy::too_many_arguments)]
fn bind_stage(
    stage: &ModelAt,
    current: Option<&SemanticTerm>,
    argument: &SemanticTerm,
    output: &str,
    step: &str,
    after: &str,
    body: SemanticTerm,
    refuse: &dyn Fn(ModelCheck) -> SemanticTerm,
) -> SemanticTerm {
    let out = var(output);
    let out_type = stage.contract.output.r#type.clone();
    match (current, stage.contract.state_type()) {
        (Some(current), Some(stage_state)) => {
            let body = guard_after(
                stage,
                Some((current.clone(), var(after))),
                argument,
                &out,
                body,
                refuse,
            );
            let_in(
                step,
                product(stage_state.clone(), out_type.clone()),
                stage.apply(vec![current.clone(), argument.clone()]),
                let_in(
                    output,
                    out_type,
                    second(var(step)),
                    let_in(after, stage_state.clone(), first(var(step)), body),
                ),
            )
        }
        _ => {
            let body = guard_after(stage, None, argument, &out, body, refuse);
            let_in(output, out_type, stage.apply(vec![argument.clone()]), body)
        }
    }
}

// The composite's interface (input, output, state) and its scope are
// separate inputs of one check; bundling them would only rename them.
#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
fn check_composite(
    name: &str,
    type_parameters: &[String],
    scope: &BTreeSet<String>,
    input: &ModelBinder,
    output: &SemanticType,
    state: Option<&RealizationState>,
    form: &CompositeForm,
    env: &Environment<'_>,
) -> Result<CompositeLowering, SemanticFailure> {
    let x = var(&input.name);
    let mismatch = |what: String| composition(format!("composite `{name}`: {what}"));
    let expect = |label: &str, observed: &SemanticType, expected: &SemanticType| {
        if observed == expected {
            Ok(())
        } else {
            Err(mismatch(format!(
                "{label} is {observed}, expected {expected}"
            )))
        }
    };
    let stateless_form = || {
        state.map_or(Ok(()), |_| {
            Err(mismatch(
                "a scan threads its stage's state itself and declares none".to_owned(),
            ))
        })
    };
    match form {
        CompositeForm::Sequence { stages, junctions } => {
            if stages.len() < 2 || junctions.len() + 1 != stages.len() {
                return Err(mismatch(format!(
                    "a sequence has at least two stages and one junction per stage after the first; found {} stage(s) and {} junction(s)",
                    stages.len(),
                    junctions.len()
                )));
            }
            let stages = stages
                .iter()
                .map(|stage| composite_stage(name, stage, scope, env, false))
                .collect::<Result<Vec<_>, _>>()?;
            expect(
                "the first stage input",
                &stages[0].contract.input.r#type,
                &input.r#type,
            )?;
            for (index, pair) in stages.windows(2).enumerate() {
                expect(
                    &format!("stage {} input", index + 2),
                    &pair[1].contract.input.r#type,
                    &pair[0].contract.output.r#type,
                )?;
            }
            let threaded = thread_state(name, &stages.iter().collect::<Vec<_>>(), state)?;
            let last = stages[stages.len() - 1].contract.output.r#type.clone();
            let checked = junctions
                .iter()
                .any(|junction| matches!(junction, CompositeJunction::Checked))
                || stages.iter().any(|stage| !post_run(stage).is_empty());
            let produced = if checked {
                result_type(last.clone())
            } else {
                last.clone()
            };
            expect("the output", output, &produced)?;
            // The first stage's invariant and precondition, and the invariant
            // of every later stage's state component that no junction checks,
            // hold at the composite's entry: a stage's component is the one the
            // composite was given.
            let mut effective =
                stage_requirements(&stages[0], threaded.component(0).as_ref(), &x, true);
            let mut obligations = Vec::new();
            for (index, junction) in junctions.iter().enumerate() {
                let (before, stage) = (&stages[index], &stages[index + 1]);
                let label = index + 1;
                let key = member_key(&stage.member);
                let current = threaded.component(index + 1);
                let invariant = current.is_some() && stage.contract.invariant.is_some();
                let precondition = stage.contract.precondition.is_some();
                if invariant && !matches!(junction, CompositeJunction::Checked) {
                    if let (Some(predicate), Some(current)) = (&stage.contract.invariant, &current)
                    {
                        effective.push(predicate.apply(vec![current.clone()]));
                    }
                }
                match junction {
                    CompositeJunction::Unconditional => {
                        if precondition {
                            return Err(composition(format!(
                                "composite `{name}` junction {label}: stage `{key}` has a precondition, so the junction must be proved or checked"
                            )));
                        }
                    }
                    CompositeJunction::Proved { evidence } => {
                        let Some(next) = &stage.contract.precondition else {
                            return Err(composition(format!(
                                "composite `{name}` junction {label}: stage `{key}` has no precondition, so the junction is unconditional"
                            )));
                        };
                        // Over the composite's state (each stage reads its own
                        // component) and the earlier stage's input: what holds
                        // when the earlier stage runs implies the later stage's
                        // precondition of its output.
                        let value = var(&input.name);
                        let before_current = threaded.component(index);
                        let produced = match &before_current {
                            Some(current) => {
                                second(before.apply(vec![current.clone(), value.clone()]))
                            }
                            None => before.apply(vec![value.clone()]),
                        };
                        let premises =
                            stage_requirements(before, before_current.as_ref(), &value, true);
                        let statement = premises.into_iter().rev().fold(
                            next.apply(stage_arguments(current.as_ref(), &produced)),
                            |conclusion, premise| implies(premise, conclusion),
                        );
                        let mut parameters = Vec::new();
                        if let (Some(declared), Some(_)) = (state, &threaded.state) {
                            parameters.push(parameter(&declared.name, &declared.r#type));
                        }
                        parameters.push(parameter(&input.name, &before.contract.input.r#type));
                        let obligation = Obligation {
                            role: format!("composite `{name}` junction {label}"),
                            theorem: evidence.clone(),
                            type_parameters: type_parameters.to_vec(),
                            parameters,
                            statement,
                        };
                        require_statement(env, evidence, &obligation, code!("LLT4007"))?;
                        obligations.push(obligation);
                    }
                    CompositeJunction::Checked => {
                        if !precondition && !invariant {
                            return Err(composition(format!(
                                "composite `{name}` junction {label}: stage `{key}` has no precondition, so the junction is unconditional"
                            )));
                        }
                        for (present, predicate, what) in [
                            (
                                precondition,
                                ContractPredicate::Precondition,
                                "precondition",
                            ),
                            (invariant, ContractPredicate::Invariant, "invariant"),
                        ] {
                            if present && !stage.contract.validators.contains_key(&predicate) {
                                return Err(composition(format!(
                                    "composite `{name}` junction {label}: stage `{key}` has no {what} validator to check"
                                )));
                            }
                        }
                    }
                }
            }
            // let __stage1 := M1 x; ... each stage's checks before it at a
            // checked junction and its checks after it right after it; a
            // refusal returns the violation (and, when stateful, the state
            // the composite was given).
            let refuse = |check: ModelCheck| threaded.refusal(&last, check);
            let stage_name = |index: usize| format!("__stage{}", index + 1);
            let state_name = |index: usize| format!("__state{}", index + 1);
            let step_name = |index: usize| format!("__step{}", index + 1);
            let updated: Vec<(usize, SemanticTerm)> = threaded
                .stateful
                .iter()
                .map(|index| (*index, var(&state_name(*index))))
                .collect();
            let mut final_value = var(&stage_name(stages.len() - 1));
            if checked {
                final_value = ok_value(&last, final_value);
            }
            let mut body = threaded.result(&updated, final_value);
            for index in (0..stages.len()).rev() {
                let stage = &stages[index];
                let argument = if index == 0 {
                    x.clone()
                } else {
                    var(&stage_name(index - 1))
                };
                let current = threaded.component(index);
                body = bind_stage(
                    stage,
                    current.as_ref(),
                    &argument,
                    &stage_name(index),
                    &step_name(index),
                    &state_name(index),
                    body,
                    &refuse,
                );
                if index > 0 && matches!(junctions[index - 1], CompositeJunction::Checked) {
                    body = guard_before(stage, current.as_ref(), &argument, body, &refuse);
                }
            }
            Ok(CompositeLowering {
                body,
                effective,
                obligations,
            })
        }
        CompositeForm::Fanout { left, right } | CompositeForm::Product { left, right } => {
            let left = composite_stage(name, left, scope, env, false)?;
            let right = composite_stage(name, right, scope, env, false)?;
            let fanout = matches!(form, CompositeForm::Fanout { .. });
            let (left_input, right_input) = if fanout {
                for (label, stage) in [("left", &left), ("right", &right)] {
                    expect(
                        &format!("the {label} stage input"),
                        &stage.contract.input.r#type,
                        &input.r#type,
                    )?;
                }
                (x.clone(), x.clone())
            } else {
                expect(
                    "the input",
                    &input.r#type,
                    &product(
                        left.contract.input.r#type.clone(),
                        right.contract.input.r#type.clone(),
                    ),
                )?;
                (first(x.clone()), second(x.clone()))
            };
            let threaded = thread_state(name, &[&left, &right], state)?;
            let both = product(
                left.contract.output.r#type.clone(),
                right.contract.output.r#type.clone(),
            );
            let checked = !post_run(&left).is_empty() || !post_run(&right).is_empty();
            expect(
                "the output",
                output,
                &if checked {
                    result_type(both.clone())
                } else {
                    both.clone()
                },
            )?;
            let mut effective = Vec::new();
            for (index, stage, argument) in [(0, &left, &left_input), (1, &right, &right_input)] {
                effective.extend(stage_requirements(
                    stage,
                    threaded.component(index).as_ref(),
                    argument,
                    true,
                ));
            }
            let body = if checked || threaded.state.is_some() {
                let refuse = |check: ModelCheck| threaded.refusal(&both, check);
                let value = pair(var("__left"), var("__right"));
                let updated: Vec<(usize, SemanticTerm)> = threaded
                    .stateful
                    .iter()
                    .map(|index| {
                        (
                            *index,
                            var(if *index == 0 {
                                "__left_state"
                            } else {
                                "__right_state"
                            }),
                        )
                    })
                    .collect();
                let inner = threaded.result(
                    &updated,
                    if checked {
                        ok_value(&both, value)
                    } else {
                        value
                    },
                );
                let inner = bind_stage(
                    &right,
                    threaded.component(1).as_ref(),
                    &right_input,
                    "__right",
                    "__right_step",
                    "__right_state",
                    inner,
                    &refuse,
                );
                bind_stage(
                    &left,
                    threaded.component(0).as_ref(),
                    &left_input,
                    "__left",
                    "__left_step",
                    "__left_state",
                    inner,
                    &refuse,
                )
            } else {
                pair(left.apply(vec![left_input]), right.apply(vec![right_input]))
            };
            Ok(CompositeLowering {
                body,
                effective,
                obligations: Vec::new(),
            })
        }
        CompositeForm::Branch {
            guard,
            then,
            r#else,
        } => {
            let then = composite_stage(name, then, scope, env, false)?;
            let otherwise = composite_stage(name, r#else, scope, env, false)?;
            let threaded = thread_state(name, &[&then, &otherwise], state)?;
            let raw = then.contract.output.r#type.clone();
            let checked = !post_run(&then).is_empty() || !post_run(&otherwise).is_empty();
            for (label, stage) in [("then", &then), ("else", &otherwise)] {
                expect(
                    &format!("the {label} stage input"),
                    &stage.contract.input.r#type,
                    &input.r#type,
                )?;
                expect(
                    &format!("the {label} stage output"),
                    &stage.contract.output.r#type,
                    &if checked { raw.clone() } else { output.clone() },
                )?;
            }
            if checked {
                expect("the output", output, &result_type(raw.clone()))?;
            }
            // Each arm's requirements hold at entry when the guard selects
            // it; the arm not taken leaves its state component unchanged.
            let mut effective = Vec::new();
            for (index, value, stage) in [(0, true, &then), (1, false, &otherwise)] {
                for requirement in
                    stage_requirements(stage, threaded.component(index).as_ref(), &x, true)
                {
                    effective.push(implies(eq(guard.clone(), boolean(value)), requirement));
                }
            }
            let arm = |index: usize, stage: &ModelAt, binder: &str| {
                if checked || threaded.state.is_some() {
                    let refuse = |check: ModelCheck| threaded.refusal(&raw, check);
                    let after = format!("{binder}_state");
                    let updated: Vec<(usize, SemanticTerm)> = threaded
                        .stateful
                        .iter()
                        .filter(|stateful| **stateful == index)
                        .map(|stateful| (*stateful, var(&after)))
                        .collect();
                    let value = if checked {
                        ok_value(&raw, var(binder))
                    } else {
                        var(binder)
                    };
                    bind_stage(
                        stage,
                        threaded.component(index).as_ref(),
                        &x,
                        binder,
                        &format!("{binder}_step"),
                        &after,
                        threaded.result(&updated, value),
                        &refuse,
                    )
                } else {
                    stage.apply(vec![x.clone()])
                }
            };
            Ok(CompositeLowering {
                body: if_then(
                    guard.clone(),
                    arm(0, &then, "__then"),
                    arm(1, &otherwise, "__else"),
                ),
                effective,
                obligations: Vec::new(),
            })
        }
        CompositeForm::Scan { stage, junction } => {
            stateless_form()?;
            let stage = composite_stage(name, stage, scope, env, true)?;
            let key = member_key(&stage.member);
            let state = stage
                .contract
                .state_type()
                .cloned()
                .ok_or_else(|| mismatch("the scanned stage has no state".to_owned()))?;
            let item = stage.contract.input.r#type.clone();
            let out = stage.contract.output.r#type.clone();
            expect("the input", &input.r#type, &list_type(item.clone()))?;
            let checked = matches!(junction, CompositeJunction::Checked);
            let after = post_run(&stage);
            if !after.is_empty() && !checked {
                return Err(mismatch(format!(
                    "the scanned stage `{key}` needs its {} checked at run time, so the junction must be checked",
                    after.iter().map(|check| check_name_of(*check)).collect::<Vec<_>>().join(", ")
                )));
            }
            let produced = if checked {
                result_type(list_type(out.clone()))
            } else {
                list_type(out.clone())
            };
            expect("the output", output, &produced)?;
            let invariant_held = stage.contract.invariant.is_none()
                || (stage.discharged.contains(&ClaimKind::InitialInvariant)
                    && stage.discharged.contains(&ClaimKind::PreservesInvariant));
            let mut obligations = Vec::new();
            match junction {
                CompositeJunction::Unconditional => {
                    if stage.contract.precondition.is_some() || !invariant_held {
                        return Err(mismatch(format!(
                            "the scanned stage `{key}` has a precondition or an invariant its evidence does not carry from the initial state, so the junction must be proved or checked"
                        )));
                    }
                }
                CompositeJunction::Proved { evidence } => {
                    let Some(precondition) = &stage.contract.precondition else {
                        return Err(mismatch(format!(
                            "the scanned stage `{key}` has no precondition, so the junction is unconditional"
                        )));
                    };
                    if !invariant_held {
                        return Err(mismatch(format!(
                            "the scanned stage `{key}` needs initial_invariant and preserves_invariant evidence for a proved junction"
                        )));
                    }
                    let contract_state = stage
                        .contract
                        .state
                        .clone()
                        .ok_or_else(|| mismatch("the scanned stage has no state".to_owned()))?;
                    let s = var(&contract_state.name);
                    let item_value = var(&stage.contract.input.name);
                    let obligation = Obligation {
                        role: format!("composite `{name}` scan junction"),
                        theorem: evidence.clone(),
                        type_parameters: type_parameters.to_vec(),
                        parameters: vec![
                            parameter(&contract_state.name, &state),
                            parameter(&stage.contract.input.name, &item),
                        ],
                        statement: premised(
                            stage
                                .contract
                                .invariant
                                .as_ref()
                                .map(|predicate| predicate.apply(vec![s.clone()])),
                            precondition.apply(vec![s, item_value]),
                        ),
                    };
                    require_statement(env, evidence, &obligation, code!("LLT4007"))?;
                    obligations.push(obligation);
                }
                CompositeJunction::Checked => {
                    for (predicate, label) in [
                        (ContractPredicate::Invariant, "invariant"),
                        (ContractPredicate::Precondition, "precondition"),
                    ] {
                        if stage.contract.predicate(predicate).is_some()
                            && !stage.contract.validators.contains_key(&predicate)
                        {
                            return Err(mismatch(format!(
                                "the scanned stage `{key}` has no {label} validator to check"
                            )));
                        }
                    }
                    if stage.contract.precondition.is_none()
                        && stage.contract.invariant.is_none()
                        && after.is_empty()
                    {
                        return Err(mismatch(format!(
                            "the scanned stage `{key}` has nothing to check, so the junction is unconditional"
                        )));
                    }
                }
            }
            let initial = call(
                &MemberRef {
                    module: stage.member.module.clone(),
                    name: format!("{}.initial", stage.member.name),
                },
                &stage.type_arguments,
                Vec::new(),
            );
            let accumulated = product(state.clone(), list_type(out.clone()));
            let step_type = product(state.clone(), out.clone());
            let step = |current: SemanticTerm, history: SemanticTerm| {
                let_in(
                    "__step",
                    step_type.clone(),
                    stage.apply(vec![current, var("__item")]),
                    pair(
                        first(var("__step")),
                        append_one(history, &out, second(var("__step"))),
                    ),
                )
            };
            let empty = SemanticTerm::Nil {
                element: out.clone(),
            };
            let body = if checked {
                let folded = result_type(accumulated.clone());
                let refuse = |check: ModelCheck| refusal(&accumulated, check);
                let current = first(var("__state"));
                let ran = guard_after(
                    &stage,
                    Some((current.clone(), first(var("__step")))),
                    &var("__item"),
                    &second(var("__step")),
                    ok_value(
                        &accumulated,
                        pair(
                            first(var("__step")),
                            append_one(second(var("__state")), &out, second(var("__step"))),
                        ),
                    ),
                    &refuse,
                );
                let guarded = guard_before(
                    &stage,
                    Some(&current),
                    &var("__item"),
                    let_in(
                        "__step",
                        step_type.clone(),
                        stage.apply(vec![current.clone(), var("__item")]),
                        ran,
                    ),
                    &refuse,
                );
                let_in(
                    "__scanned",
                    folded.clone(),
                    fold(
                        lambda(
                            vec![("__acc", folded.clone()), ("__item", item)],
                            matching(
                                var("__acc"),
                                vec![
                                    ("Result.error", vec!["__violation"], var("__acc")),
                                    ("Result.ok", vec!["__state"], guarded),
                                ],
                            ),
                        ),
                        ok_value(&accumulated, pair(initial, empty)),
                        x.clone(),
                        folded,
                    ),
                    matching(
                        var("__scanned"),
                        vec![
                            (
                                "Result.error",
                                vec!["__violation"],
                                constructor(
                                    "Result.error",
                                    vec![list_type(out.clone()), SemanticType::ContractViolation],
                                    vec![var("__violation")],
                                ),
                            ),
                            (
                                "Result.ok",
                                vec!["__state"],
                                ok_value(&list_type(out.clone()), second(var("__state"))),
                            ),
                        ],
                    ),
                )
            } else {
                second(fold(
                    lambda(
                        vec![("__acc", accumulated.clone()), ("__item", item)],
                        step(first(var("__acc")), second(var("__acc"))),
                    ),
                    pair(initial, empty),
                    x.clone(),
                    accumulated,
                ))
            };
            Ok(CompositeLowering {
                body,
                effective: Vec::new(),
                obligations,
            })
        }
    }
}

/// The type of a descriptor term is fixed by its slot: a rule action or a
/// label is an output, a guard is a Boolean, an encoder or a feature map is an
/// integer vector. A term of another type is therefore an interface mismatch
/// (LLT4006) of the realization, or, for a branch guard, of the composition
/// (LLT4007), rather than an incidental ill-typed body. A term that has no
/// type at all is left to the ordinary check of the elaborated declaration,
/// which reports it as any ill-formed term (LLT4001).
fn check_slots(
    name: &str,
    input: &ModelBinder,
    output: &SemanticType,
    state: Option<&RealizationState>,
    descriptor: &RealizationDescriptor,
    env: &Environment<'_>,
) -> Result<(), SemanticFailure> {
    let mut locals = BTreeMap::from([(input.name.clone(), input.r#type.clone())]);
    let result = match state {
        Some(state) => {
            locals.insert(state.name.clone(), state.r#type.clone());
            product(state.r#type.clone(), output.clone())
        }
        None => output.clone(),
    };
    let slot = |term: &SemanticTerm,
                locals: &BTreeMap<String, SemanticType>,
                expected: &SemanticType,
                what: &str,
                failure: fn(String) -> SemanticFailure| {
        match super::infer_term(term, locals, env) {
            Ok(Some(observed)) if &observed != expected => Err(failure(format!(
                "realization `{name}` {what} has type {observed}, expected {expected}"
            ))),
            _ => Ok(()),
        }
    };
    match descriptor {
        RealizationDescriptor::Deterministic { body } => {
            slot(body, &locals, &result, "denotation", interface)
        }
        RealizationDescriptor::Rule { rules, default } => {
            for rule in rules {
                let what = format!("rule `{}`", rule.name);
                slot(
                    &rule.guard,
                    &locals,
                    &SemanticType::Bool,
                    &format!("{what} guard"),
                    interface,
                )?;
                slot(
                    &rule.action,
                    &locals,
                    &result,
                    &format!("{what} action"),
                    interface,
                )?;
            }
            slot(default, &locals, &result, "default", interface)
        }
        RealizationDescriptor::Statistical {
            features, labels, ..
        } => {
            slot(features, &locals, &int_list(), "feature map", interface)?;
            for (index, label) in labels.iter().enumerate() {
                slot(
                    label,
                    &BTreeMap::new(),
                    output,
                    &format!("label {}", index + 1),
                    interface,
                )?;
            }
            Ok(())
        }
        RealizationDescriptor::Neural {
            encoder, decoder, ..
        } => {
            slot(encoder, &locals, &int_list(), "encoder", interface)?;
            match decoder {
                NeuralDecoder::Argmax { labels } => {
                    for (index, label) in labels.iter().enumerate() {
                        slot(
                            label,
                            &BTreeMap::new(),
                            output,
                            &format!("label {}", index + 1),
                            interface,
                        )?;
                    }
                    Ok(())
                }
                NeuralDecoder::Function { binder, body } => {
                    let mut decoded = locals.clone();
                    decoded.insert(binder.clone(), int_list());
                    slot(body, &decoded, &result, "decoder", interface)
                }
            }
        }
        RealizationDescriptor::Composite { form } => match form {
            CompositeForm::Branch { guard, .. } => slot(
                guard,
                &locals,
                &SemanticType::Bool,
                "branch guard",
                composition,
            ),
            CompositeForm::Sequence { .. }
            | CompositeForm::Fanout { .. }
            | CompositeForm::Product { .. }
            | CompositeForm::Scan { .. } => Ok(()),
        },
    }
}

fn check_realization(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::Realization {
        name,
        type_parameters,
        input,
        output,
        state,
        descriptor,
        executable,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_axioms(name, axioms)?;
    let mut binders = vec![input.name.as_str()];
    let mut types = vec![&input.r#type, output];
    if let Some(state) = state {
        binders.push(&state.name);
        types.push(&state.r#type);
    }
    if let RealizationDescriptor::Neural {
        decoder: NeuralDecoder::Function { binder, .. },
        ..
    } = descriptor
    {
        binders.push(binder);
    }
    let scope = check_signature(name, type_parameters, &binders, &types, env)?;
    // Every source term of the descriptor is ordinary code: its checked
    // applications are elaborated, and in an executable realization every
    // model application in it must be validated (§17.12).
    let mut source = declaration.clone();
    lower_terms(&mut source, *executable, env)?;
    let SemanticDeclaration::Realization {
        state, descriptor, ..
    } = &source
    else {
        return Ok(Lowering::default());
    };
    check_slots(name, input, output, state.as_ref(), descriptor, env)?;
    let companion = |suffix: &str| format!("{name}.{suffix}");
    let x = var(&input.name);
    let input_parameter = parameter(&input.name, &input.r#type);
    let (parameters, result) = match state {
        Some(state) => (
            vec![
                parameter(&state.name, &state.r#type),
                input_parameter.clone(),
            ],
            product(state.r#type.clone(), output.clone()),
        ),
        None => (vec![input_parameter.clone()], output.clone()),
    };
    let definition = |name: String,
                      parameters: Vec<SemanticParameter>,
                      result: SemanticType,
                      body: SemanticTerm| {
        ordinary_definition(name, type_parameters, parameters, result, body, *executable)
    };
    let stateless = |kind: &str| {
        if state.is_some() {
            Err(interface(format!(
                "{kind} realization `{name}` is stateless; thread state through a deterministic stage or a scan"
            )))
        } else {
            Ok(())
        }
    };
    let own = |suffix: &str| {
        call(
            &local(&companion(suffix)),
            &parameter_types(type_parameters),
            vec![x.clone()],
        )
    };
    let mut lowering = Lowering::default();
    let mut effective = Vec::new();
    if let Some(state) = state {
        lowering.declarations.push(definition(
            companion("initial"),
            Vec::new(),
            state.r#type.clone(),
            state.initial.clone(),
        ));
    }
    match descriptor {
        RealizationDescriptor::Deterministic { body } => {
            lowering
                .declarations
                .push(definition(name.clone(), parameters, result, body.clone()));
        }
        RealizationDescriptor::Rule { rules, default } => {
            if rules.is_empty() {
                return Err(interface(format!(
                    "rule realization `{name}` has no rules; a fixed result is deterministic"
                )));
            }
            let mut names = BTreeSet::new();
            for rule in rules {
                check_name(&rule.name, "rule")?;
                if !names.insert(rule.name.as_str()) {
                    return Err(interface(format!(
                        "rule realization `{name}` repeats the rule `{}`",
                        rule.name
                    )));
                }
            }
            let mut denotation = default.clone();
            let mut selected = nat(rules.len() as u64);
            for (index, rule) in rules.iter().enumerate().rev() {
                denotation = if_then(rule.guard.clone(), rule.action.clone(), denotation);
                selected = if_then(rule.guard.clone(), nat(index as u64), selected);
            }
            lowering.declarations.push(definition(
                companion("selected"),
                parameters.clone(),
                SemanticType::Nat,
                selected,
            ));
            lowering
                .declarations
                .push(definition(name.clone(), parameters, result, denotation));
        }
        RealizationDescriptor::Statistical {
            scheme: _,
            features,
            width,
            width_evidence,
            weights,
            bias,
            labels,
        } => {
            stateless("statistical")?;
            check_labels(name, labels)?;
            let classes = labels.len() as u64;
            let weights = parameter_slot(weights, &[classes, *width], env, "statistical weights")?;
            let bias = parameter_slot(bias, &[classes], env, "statistical bias")?;
            lowering.obligations.push(width_obligation(
                name,
                type_parameters,
                input,
                features,
                *width,
                width_evidence,
                env,
            )?);
            lowering.declarations.push(definition(
                companion("features"),
                parameters.clone(),
                int_list(),
                features.clone(),
            ));
            lowering.declarations.push(definition(
                companion("scores"),
                parameters.clone(),
                int_list(),
                let_in(
                    "__values",
                    int_list(),
                    own("features"),
                    dense(weights, bias, "__values"),
                ),
            ));
            lowering.declarations.push(definition(
                name.clone(),
                parameters,
                result,
                argmax(labels, output, own("scores")),
            ));
        }
        RealizationDescriptor::Neural {
            architecture: _,
            encoder,
            width,
            width_evidence,
            layers,
            decoder,
        } => {
            stateless("neural")?;
            if layers.is_empty() {
                return Err(interface(format!(
                    "neural realization `{name}` has no layers"
                )));
            }
            lowering.obligations.push(width_obligation(
                name,
                type_parameters,
                input,
                encoder,
                *width,
                width_evidence,
                env,
            )?);
            lowering.declarations.push(definition(
                companion("encoded"),
                parameters.clone(),
                int_list(),
                encoder.clone(),
            ));
            let mut current = *width;
            let mut logits = own("encoded");
            for (index, layer) in layers.iter().enumerate() {
                let what = format!("neural realization `{name}` layer {}", index + 1);
                let body = match layer {
                    NeuralLayer::Dense {
                        inputs,
                        outputs,
                        weights,
                        bias,
                    } => {
                        if *inputs != current || *outputs == 0 {
                            return Err(interface(format!(
                                "{what} takes {inputs} input(s) to {outputs} output(s), but receives {current}"
                            )));
                        }
                        current = *outputs;
                        dense(
                            parameter_slot(
                                weights,
                                &[*outputs, *inputs],
                                env,
                                &format!("{what} weights"),
                            )?,
                            parameter_slot(bias, &[*outputs], env, &format!("{what} bias"))?,
                            "__values",
                        )
                    }
                    NeuralLayer::Relu => relu("__values"),
                    NeuralLayer::Requantize {
                        shift,
                        minimum,
                        maximum,
                    } => {
                        for bound in [minimum, maximum] {
                            super::check_integer_literal(SemanticInteger::Int, bound)
                                .map_err(|reason| interface(format!("{what}: {reason}")))?;
                        }
                        if compare_decimal(minimum, maximum) == std::cmp::Ordering::Greater {
                            return Err(interface(format!(
                                "{what} clamps to the empty range [{minimum}, {maximum}]"
                            )));
                        }
                        if *shift > 4096 {
                            return Err(interface(format!(
                                "{what} shifts by {shift}; a requantization shift is at most 4096"
                            )));
                        }
                        requantize("__values", *shift, minimum, maximum)
                    }
                };
                let layer_name = companion(&format!("layer{}", index + 1));
                lowering.declarations.push(definition(
                    layer_name.clone(),
                    vec![parameter("__values", &int_list())],
                    int_list(),
                    body,
                ));
                logits = call(
                    &local(&layer_name),
                    &parameter_types(type_parameters),
                    vec![logits],
                );
            }
            lowering.declarations.push(definition(
                companion("logits"),
                parameters.clone(),
                int_list(),
                logits,
            ));
            let denotation = match decoder {
                NeuralDecoder::Argmax { labels } => {
                    check_labels(name, labels)?;
                    if labels.len() as u64 != current {
                        return Err(interface(format!(
                            "neural realization `{name}` decodes {current} value(s) with {} label(s)",
                            labels.len()
                        )));
                    }
                    argmax(labels, output, own("logits"))
                }
                NeuralDecoder::Function { binder, body } => {
                    let_in(binder, int_list(), own("logits"), body.clone())
                }
            };
            lowering
                .declarations
                .push(definition(name.clone(), parameters, result, denotation));
        }
        RealizationDescriptor::Composite { form } => {
            let composite = check_composite(
                name,
                type_parameters,
                &scope,
                input,
                output,
                state.as_ref(),
                form,
                env,
            )?;
            effective = composite.effective;
            lowering.obligations.extend(composite.obligations);
            lowering.declarations.push(definition(
                name.clone(),
                parameters,
                result,
                composite.body,
            ));
        }
    }
    for derived in &lowering.declarations {
        env.models
            .realization_functions
            .insert(derived.name().to_owned());
    }
    env.models.realizations.insert(
        name.clone(),
        RealizationInfo {
            type_parameters: type_parameters.clone(),
            input: input.clone(),
            output: output.clone(),
            state: state.as_ref().map(|state| state.r#type.clone()),
            state_binder: state.as_ref().map(|state| state.name.clone()),
            effective,
        },
    );
    Ok(lowering)
}

// ---------------------------------------------------------------------------
// Evidence.

/// The statement of one claim about `realization` against `contract`.
#[allow(clippy::too_many_lines)]
fn claim_obligation(
    evidence: &str,
    type_parameters: &[String],
    contract: &ContractAt,
    realization: &RealizationAt,
    claim: &EvidenceClaim,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
) -> Result<(Vec<Obligation>, CheckStatement), SemanticFailure> {
    let (kind, theorem) = ClaimKind::of(claim);
    let mut obligations = Vec::new();
    let role = format!("evidence `{evidence}` claim {}", kind.as_str());
    let vacuous = |reason: &str| unestablished(format!("{role}: {reason}"));
    let x = var(&contract.input.name);
    let input = parameter(&contract.input.name, &contract.input.r#type);
    let helper = |helper: &str, arguments: Vec<SemanticTerm>| CheckStatement::Helper {
        helper: helper.to_owned(),
        arguments,
    };
    let r = function_ref(&realization.member, &realization.type_arguments);
    let reference = |predicate: &Option<Predicate>| predicate.as_ref().map(Predicate::reference);
    let (parameters, statement, check) = match (claim, &contract.state) {
        (EvidenceClaim::SatisfiesContract { .. }, state) => {
            let Some(post) = &contract.postcondition else {
                return Err(vacuous("the contract has no postcondition to satisfy"));
            };
            match state {
                None => {
                    let pre = contract
                        .precondition
                        .as_ref()
                        .map(|p| p.apply(vec![x.clone()]));
                    let statement = premised(
                        pre,
                        post.apply(vec![x.clone(), realization.apply(vec![x.clone()])]),
                    );
                    let check = match &contract.precondition {
                        Some(pre) => {
                            helper("Satisfies", vec![pre.reference(), post.reference(), r])
                        }
                        None => helper("SatisfiesTotal", vec![post.reference(), r]),
                    };
                    (vec![input], statement, check)
                }
                Some(state) => {
                    let s = var(&state.name);
                    let step = realization.apply(vec![s.clone(), x.clone()]);
                    let conclusion = post.apply(vec![
                        s.clone(),
                        x.clone(),
                        first(step.clone()),
                        second(step),
                    ]);
                    let statement = premised(
                        contract
                            .invariant
                            .as_ref()
                            .map(|j| j.apply(vec![s.clone()])),
                        premised(
                            contract
                                .precondition
                                .as_ref()
                                .map(|p| p.apply(vec![s.clone(), x.clone()])),
                            conclusion,
                        ),
                    );
                    let check = match (
                        reference(&contract.invariant),
                        reference(&contract.precondition),
                    ) {
                        (Some(j), Some(p)) => {
                            helper("SatisfiesStep", vec![j, p, post.reference(), r])
                        }
                        (None, Some(p)) => {
                            helper("SatisfiesStepNoInvariant", vec![p, post.reference(), r])
                        }
                        (Some(j), None) => {
                            helper("SatisfiesStepNoPrecondition", vec![j, post.reference(), r])
                        }
                        (None, None) => helper("SatisfiesStepTotal", vec![post.reference(), r]),
                    };
                    (
                        vec![parameter(&state.name, &state.r#type), input],
                        statement,
                        check,
                    )
                }
            }
        }
        (EvidenceClaim::PreservesInvariant { .. }, Some(state)) => {
            let Some(invariant) = &contract.invariant else {
                return Err(vacuous("the contract has no invariant to preserve"));
            };
            let s = var(&state.name);
            let statement = implies(
                invariant.apply(vec![s.clone()]),
                premised(
                    contract
                        .precondition
                        .as_ref()
                        .map(|p| p.apply(vec![s.clone(), x.clone()])),
                    invariant.apply(vec![first(realization.apply(vec![s.clone(), x.clone()]))]),
                ),
            );
            let check = match reference(&contract.precondition) {
                Some(p) => helper("Preserves", vec![invariant.reference(), p, r]),
                None => helper("PreservesTotal", vec![invariant.reference(), r]),
            };
            (
                vec![parameter(&state.name, &state.r#type), input],
                statement,
                check,
            )
        }
        (EvidenceClaim::InitialInvariant { .. }, Some(_)) => {
            let Some(invariant) = &contract.invariant else {
                return Err(vacuous("the contract has no invariant to establish"));
            };
            (
                Vec::new(),
                invariant.apply(vec![realization.initial()]),
                helper(
                    "Initial",
                    vec![invariant.reference(), realization.initial()],
                ),
            )
        }
        (
            EvidenceClaim::PreservesInvariant { .. } | EvidenceClaim::InitialInvariant { .. },
            None,
        ) => {
            return Err(vacuous("a stateless contract has no invariant"));
        }
        (
            EvidenceClaim::EquivalentTo {
                reference: target, ..
            },
            state,
        ) => {
            check_member(&target.member, env)?;
            let key = member_key(&target.member);
            let Some(info) = function_info(&target.member, env) else {
                return Err(vacuous(&format!(
                    "the reference `{key}` is not a prior definition or model"
                )));
            };
            if info.type_parameters.len() != target.type_arguments.len() {
                return Err(vacuous(&format!(
                    "the reference `{key}` expects {} type argument(s)",
                    info.type_parameters.len()
                )));
            }
            for argument in &target.type_arguments {
                check_type_argument(argument, env)?;
                check_type_parameters(argument, scope)?;
            }
            if env.models.realization_functions.contains(&key) {
                return Err(vacuous(&format!(
                    "the reference `{key}` is a realization; refer to a model or an ordinary definition"
                )));
            }
            let (expected_parameters, expected_result) = realization.signature();
            let (parameters, result) = super::instantiate(info, &target.type_arguments);
            if parameters != expected_parameters || result != expected_result {
                return Err(vacuous(&format!(
                    "the reference `{key}` does not have the realization's signature"
                )));
            }
            let f = function_ref(&target.member, &target.type_arguments);
            match state {
                None => {
                    let statement = premised(
                        contract
                            .precondition
                            .as_ref()
                            .map(|p| p.apply(vec![x.clone()])),
                        eq(
                            realization.apply(vec![x.clone()]),
                            call(&target.member, &target.type_arguments, vec![x.clone()]),
                        ),
                    );
                    let check = match reference(&contract.precondition) {
                        Some(p) => helper("Equivalent", vec![p, r, f]),
                        None => helper("EquivalentTotal", vec![r, f]),
                    };
                    (vec![input], statement, check)
                }
                Some(state) => {
                    let s = var(&state.name);
                    let statement = premised(
                        contract
                            .invariant
                            .as_ref()
                            .map(|j| j.apply(vec![s.clone()])),
                        premised(
                            contract
                                .precondition
                                .as_ref()
                                .map(|p| p.apply(vec![s.clone(), x.clone()])),
                            eq(
                                realization.apply(vec![s.clone(), x.clone()]),
                                call(
                                    &target.member,
                                    &target.type_arguments,
                                    vec![s.clone(), x.clone()],
                                ),
                            ),
                        ),
                    );
                    let check = match (
                        reference(&contract.invariant),
                        reference(&contract.precondition),
                    ) {
                        (Some(j), Some(p)) => helper("EquivalentStep", vec![j, p, r, f]),
                        (None, Some(p)) => helper("EquivalentStepNoInvariant", vec![p, r, f]),
                        (Some(j), None) => helper("EquivalentStepNoPrecondition", vec![j, r, f]),
                        (None, None) => helper("EquivalentStepTotal", vec![r, f]),
                    };
                    (
                        vec![parameter(&state.name, &state.r#type), input],
                        statement,
                        check,
                    )
                }
            }
        }
        (
            EvidenceClaim::DatasetAgreement {
                dataset,
                comparison,
                comparison_sound,
                examples,
                agreements,
                ..
            },
            None,
        ) => {
            if agreements > examples {
                return Err(vacuous(&format!(
                    "{agreements} agreements cannot be counted over {examples} examples"
                )));
            }
            let example = product(
                contract.input.r#type.clone(),
                contract.output.r#type.clone(),
            );
            check_member(dataset, env)?;
            let dataset_key = member_key(dataset);
            match function_info(dataset, env) {
                Some(info)
                    if info.type_parameters.is_empty()
                        && info.parameters.is_empty()
                        && info.result == list_type(example.clone()) => {}
                _ => {
                    return Err(vacuous(&format!(
                        "the dataset `{dataset_key}` is not a prior definition of type {}",
                        list_type(example.clone())
                    )));
                }
            }
            check_member(comparison, env)?;
            let comparison_key = member_key(comparison);
            let Some(info) = function_info(comparison, env) else {
                return Err(vacuous(&format!(
                    "the comparison `{comparison_key}` is not a prior definition"
                )));
            };
            let output = &contract.output.r#type;
            if !info.type_parameters.is_empty()
                || info.parameters != [output.clone(), output.clone()]
                || info.result != SemanticType::Bool
            {
                return Err(vacuous(&format!(
                    "the comparison `{comparison_key}` must take ({output}, {output}) to Bool"
                )));
            }
            // The comparison is sound for equality, so an agreement it
            // counts is an equal output.
            let (left, right) = ("__expected", "__observed");
            let sound = Obligation {
                role: format!("{role} comparison"),
                theorem: comparison_sound.clone(),
                type_parameters: type_parameters.to_vec(),
                parameters: vec![parameter(left, output), parameter(right, output)],
                statement: implies(
                    eq(
                        call(comparison, &[], vec![var(left), var(right)]),
                        boolean(true),
                    ),
                    eq(var(left), var(right)),
                ),
            };
            require_statement(env, comparison_sound, &sound, code!("LLT4009"))?;
            obligations.push(sound);
            let data = call(dataset, &[], Vec::new());
            let counted = fold(
                lambda(
                    vec![
                        ("__count", SemanticType::Nat),
                        ("__example", example.clone()),
                    ],
                    if_then(
                        call(
                            comparison,
                            &[],
                            vec![
                                realization.apply(vec![first(var("__example"))]),
                                second(var("__example")),
                            ],
                        ),
                        SemanticTerm::Add {
                            left: Box::new(var("__count")),
                            right: Box::new(nat(1)),
                        },
                        var("__count"),
                    ),
                ),
                nat(0),
                data.clone(),
                SemanticType::Nat,
            );
            let statement = SemanticTerm::PropAnd {
                left: Box::new(eq(
                    primitive(
                        SemanticPrimitive::Length,
                        vec![data.clone()],
                        SemanticType::Nat,
                    ),
                    nat(*examples),
                )),
                right: Box::new(eq(counted, nat(*agreements))),
            };
            let check = helper(
                "Agreement",
                vec![
                    r,
                    function_ref(comparison, &[]),
                    data,
                    nat(*examples),
                    nat(*agreements),
                ],
            );
            (Vec::new(), statement, check)
        }
        (EvidenceClaim::DatasetAgreement { .. }, Some(_)) => {
            return Err(vacuous(
                "dataset agreement is stated for stateless realizations",
            ));
        }
    };
    let obligation = Obligation {
        role,
        theorem: theorem.clone(),
        type_parameters: type_parameters.to_vec(),
        parameters,
        statement,
    };
    require_statement(env, theorem, &obligation, code!("LLT4009"))?;
    obligations.insert(0, obligation);
    Ok((obligations, check))
}

fn check_evidence(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::Evidence {
        name,
        type_parameters,
        contract,
        realization,
        claims,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_axioms(name, axioms)?;
    let scope = check_signature(name, type_parameters, &[], &[], env)?;
    let contract_at = resolve_contract(contract, &scope, env, unestablished)?;
    let realization_at = resolve_realization(realization, &scope, env, unestablished)?;
    if let Some(reason) = interface_mismatch(&contract_at, &realization_at) {
        return Err(interface(format!(
            "evidence `{name}`: realization `{}` does not have the interface of contract `{}`: {reason}",
            member_key(&realization.member),
            member_key(&contract.member)
        )));
    }
    if claims.is_empty() {
        return Err(unestablished(format!("evidence `{name}` makes no claim")));
    }
    let mut lowering = Lowering::default();
    let mut previous: Option<(ClaimKind, String)> = None;
    let mut kinds = BTreeSet::new();
    for claim in claims {
        let (kind, theorem) = ClaimKind::of(claim);
        let key = (kind, member_key(theorem));
        if previous.as_ref().is_some_and(|prior| prior >= &key) {
            return Err(unestablished(format!(
                "evidence `{name}` claims are not strictly sorted by kind and theorem"
            )));
        }
        previous = Some(key);
        let (obligations, statement) = claim_obligation(
            name,
            type_parameters,
            &contract_at,
            &realization_at,
            claim,
            &scope,
            env,
        )?;
        lowering.checks.push(CrossCheck {
            name: format!("{name}.{}", theorem.name),
            type_parameters: type_parameters.clone(),
            statement,
            proof: match kind {
                // A dataset count is closed: the kernel recounts it against
                // the fixed definition rather than trusting the theorem's
                // unfolding.
                ClaimKind::DatasetAgreement => CheckProof::Decide,
                ClaimKind::EquivalentTo
                | ClaimKind::InitialInvariant
                | ClaimKind::PreservesInvariant
                | ClaimKind::SatisfiesContract => CheckProof::Theorem {
                    theorem: theorem.clone(),
                    type_arguments: parameter_types(type_parameters),
                },
            },
        });
        lowering.obligations.extend(obligations);
        kinds.insert(kind);
    }
    env.models.evidence.insert(
        name.clone(),
        EvidenceInfo {
            type_parameters: type_parameters.clone(),
            contract: contract.clone(),
            realization: realization.clone(),
            claims: kinds,
        },
    );
    Ok(lowering)
}

// ---------------------------------------------------------------------------
// Models.

fn check_model(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::Model {
        name,
        type_parameters,
        contract,
        realization,
        evidence,
        entry,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_axioms(name, axioms)?;
    let scope = check_signature(name, type_parameters, &[], &[], env)?;
    let contract_at = resolve_contract(contract, &scope, env, interface)?;
    let realization_at = resolve_realization(realization, &scope, env, interface)?;
    if let Some(reason) = interface_mismatch(&contract_at, &realization_at) {
        return Err(interface(format!(
            "model `{name}`: realization `{}` does not have the interface of contract `{}`: {reason}",
            member_key(&realization.member),
            member_key(&contract.member)
        )));
    }
    let mut discharged = BTreeSet::new();
    let mut seen = BTreeSet::new();
    for item in evidence {
        let key = member_key(&item.member);
        if !seen.insert(key.clone()) {
            return Err(unestablished(format!(
                "model `{name}` lists evidence `{key}` twice"
            )));
        }
        let Some(info) = env.models.evidence.get(&key).cloned() else {
            return Err(unestablished(format!(
                "model `{name}`: `{key}` is not prior evidence"
            )));
        };
        let map = use_substitution(item, &info.type_parameters, &scope, env, "evidence")?;
        let at = |model_use: &ModelUse| ModelUse {
            member: model_use.member.clone(),
            type_arguments: model_use
                .type_arguments
                .iter()
                .map(|argument| substitute_type(argument, &map))
                .collect(),
        };
        if at(&info.contract) != *contract || at(&info.realization) != *realization {
            return Err(unestablished(format!(
                "model `{name}`: evidence `{key}` is about another contract or realization"
            )));
        }
        discharged.extend(info.claims.iter().copied());
    }
    // Every effective precondition of the realization follows from the
    // contract's precondition at the model's entry.
    if entry.len() != realization_at.effective.len() {
        return Err(composition(format!(
            "model `{name}` lists {} entry theorem(s); its realization has {} effective precondition(s)",
            entry.len(),
            realization_at.effective.len()
        )));
    }
    let mut lowering = Lowering::default();
    let x = &realization_at.input.name;
    // A stateful realization's effective preconditions mention its state:
    // the entry theorem may assume the contract's invariant of it too.
    let stateful = realization_at
        .state
        .as_ref()
        .zip(realization_at.state_binder.as_ref());
    for (index, (theorem, effective)) in entry.iter().zip(&realization_at.effective).enumerate() {
        let (parameters, premise_arguments, invariant) = match stateful {
            Some((state_type, binder)) => (
                vec![
                    parameter(binder, state_type),
                    parameter(x, &realization_at.input.r#type),
                ],
                vec![var(binder), var(x)],
                contract_at
                    .invariant
                    .as_ref()
                    .map(|predicate| predicate.apply(vec![var(binder)])),
            ),
            None => (
                vec![parameter(x, &realization_at.input.r#type)],
                vec![var(x)],
                None,
            ),
        };
        let obligation = Obligation {
            role: format!("model `{name}` entry {}", index + 1),
            theorem: theorem.clone(),
            type_parameters: type_parameters.clone(),
            parameters,
            statement: premised(
                invariant,
                premised(
                    contract_at
                        .precondition
                        .as_ref()
                        .map(|predicate| predicate.apply(premise_arguments)),
                    effective.clone(),
                ),
            ),
        };
        require_statement(env, theorem, &obligation, code!("LLT4007"))?;
        lowering.obligations.push(obligation);
    }
    let executable = env
        .functions
        .get(&member_key(&realization.member))
        .is_some_and(|info| info.executable);
    let parameters: Vec<SemanticParameter> = match &realization_at.state {
        Some(state) => vec![
            parameter("__state", state),
            parameter("__input", &realization_at.input.r#type),
        ],
        None => vec![parameter("__input", &realization_at.input.r#type)],
    };
    let arguments = parameters
        .iter()
        .map(|parameter| var(&parameter.name))
        .collect();
    let (_, result) = realization_at.signature();
    if let Some(state) = &realization_at.state {
        lowering.declarations.push(ordinary_definition(
            format!("{name}.initial"),
            type_parameters,
            Vec::new(),
            state.clone(),
            realization_at.initial(),
            executable,
        ));
    }
    lowering.declarations.push(ordinary_definition(
        name.clone(),
        type_parameters,
        parameters,
        result,
        realization_at.apply(arguments),
        executable,
    ));
    lowering.required = required_checks(&contract_at, &discharged);
    env.models.models.insert(
        name.clone(),
        ModelInfo {
            type_parameters: type_parameters.clone(),
            contract: contract.clone(),
            realization: realization.clone(),
            discharged,
        },
    );
    Ok(lowering)
}

// ---------------------------------------------------------------------------
// The runtime boundary and checked applications.

/// Check every model application in `term`: each checked application names
/// a model and only checks its contract can run, and in executable code
/// every application validates each predicate its evidence does not
/// discharge.
pub(super) fn check_boundary(
    owner: &str,
    term: &SemanticTerm,
    executable: bool,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
) -> Result<(), SemanticFailure> {
    let mut failure = None;
    visit_terms(term, &mut |node| {
        if failure.is_some() {
            return;
        }
        let outcome = match node {
            SemanticTerm::CheckedApply {
                model,
                type_arguments,
                checks,
                ..
            } => check_application(owner, model, type_arguments, checks, executable, scope, env),
            SemanticTerm::Call { function, .. } | SemanticTerm::FunctionRef { function, .. }
                if executable =>
            {
                let key = member_key(function);
                if let Some(what) = env.reasoning.guarded(&key) {
                    Err(fail(
                        code!("LLT4012"),
                        format!(
                            "executable `{owner}` reaches {what} `{key}` directly; it runs only through its reasoner's guarded application and verifier"
                        ),
                    ))
                } else if env.models.realization_functions.contains(&key) {
                    Err(boundary(format!(
                        "executable `{owner}` applies the realization function `{key}` directly; a realization runs only through its model"
                    )))
                } else if env.models.models.contains_key(&key) {
                    let model_use = ModelUse {
                        member: function.clone(),
                        type_arguments: match node {
                            SemanticTerm::Call { type_arguments, .. }
                            | SemanticTerm::FunctionRef { type_arguments, .. } => {
                                type_arguments.clone()
                            }
                            _ => Vec::new(),
                        },
                    };
                    resolve_model(&model_use, scope, env, boundary).and_then(|at| {
                        let required = at.required();
                        if required.is_empty() {
                            Ok(())
                        } else {
                            Err(boundary(format!(
                                "executable `{owner}` applies model `{key}` without its runtime checks [{}]; apply it with checked_apply",
                                required.iter().map(|check| check_name_of(*check)).collect::<Vec<_>>().join(", ")
                            )))
                        }
                    })
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        };
        if let Err(reason) = outcome {
            failure = Some(reason);
        }
    });
    failure.map_or(Ok(()), Err)
}

fn check_application(
    owner: &str,
    model: &MemberRef,
    type_arguments: &[SemanticType],
    checks: &[ModelCheck],
    executable: bool,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
) -> Result<(), SemanticFailure> {
    let key = member_key(model);
    let at = resolve_model(
        &ModelUse {
            member: model.clone(),
            type_arguments: type_arguments.to_vec(),
        },
        scope,
        env,
        boundary,
    )?;
    if checks.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(boundary(format!(
            "`{owner}` checks model `{key}` with checks that are not strictly sorted"
        )));
    }
    for check in checks {
        let predicate = match check {
            ModelCheck::InputInvariant | ModelCheck::OutputInvariant => {
                ContractPredicate::Invariant
            }
            ModelCheck::Precondition => ContractPredicate::Precondition,
            ModelCheck::Postcondition => ContractPredicate::Postcondition,
        };
        if at.contract.predicate(predicate).is_none()
            || !at.contract.validators.contains_key(&predicate)
        {
            return Err(boundary(format!(
                "`{owner}` checks the {} of model `{key}`, which its contract does not validate with a sound validator",
                check_name_of(*check)
            )));
        }
    }
    if executable {
        let missing: Vec<&str> = at
            .required()
            .into_iter()
            .filter(|check| !checks.contains(check))
            .map(check_name_of)
            .collect();
        if !missing.is_empty() {
            return Err(boundary(format!(
                "executable `{owner}` applies model `{key}` without the runtime checks [{}] its evidence does not discharge",
                missing.join(", ")
            )));
        }
    }
    Ok(())
}

/// The ordinary term a checked application means: the fixed check order
/// input invariant, precondition, run, output invariant, postcondition,
/// returning the first refusal.
pub(super) fn expand(
    term: &SemanticTerm,
    counter: &mut usize,
    scope: &BTreeSet<String>,
    env: &Environment<'_>,
) -> Result<SemanticTerm, SemanticFailure> {
    let SemanticTerm::CheckedApply {
        model,
        type_arguments,
        arguments,
        checks,
    } = term
    else {
        return Ok(term.clone());
    };
    let at = resolve_model(
        &ModelUse {
            member: model.clone(),
            type_arguments: type_arguments.clone(),
        },
        scope,
        env,
        boundary,
    )?;
    *counter += 1;
    let n = *counter;
    let has = |check: ModelCheck| checks.contains(&check);
    let validator = |predicate: ContractPredicate| at.contract.validators[&predicate].clone();
    let input = format!("__checked{n}_input");
    let input_type = at.contract.input.r#type.clone();
    let output_type = at.contract.output.r#type.clone();
    match (&at.contract.state, arguments.as_slice()) {
        (None, [argument]) => {
            let output = format!("__checked{n}_output");
            let mut body = ok_value(&output_type, var(&output));
            if has(ModelCheck::Postcondition) {
                body = if_then(
                    validator(ContractPredicate::Postcondition)
                        .apply(vec![var(&input), var(&output)]),
                    body,
                    refusal(&output_type, ModelCheck::Postcondition),
                );
            }
            body = let_in(
                &output,
                output_type.clone(),
                at.apply(vec![var(&input)]),
                body,
            );
            if has(ModelCheck::Precondition) {
                body = if_then(
                    validator(ContractPredicate::Precondition).apply(vec![var(&input)]),
                    body,
                    refusal(&output_type, ModelCheck::Precondition),
                );
            }
            Ok(let_in(&input, input_type, argument.clone(), body))
        }
        (Some(state), [state_argument, argument]) => {
            let current = format!("__checked{n}_state");
            let step = format!("__checked{n}_step");
            let step_type = product(state.r#type.clone(), output_type);
            let mut body = ok_value(&step_type, var(&step));
            if has(ModelCheck::Postcondition) {
                body = if_then(
                    validator(ContractPredicate::Postcondition).apply(vec![
                        var(&current),
                        var(&input),
                        first(var(&step)),
                        second(var(&step)),
                    ]),
                    body,
                    refusal(&step_type, ModelCheck::Postcondition),
                );
            }
            if has(ModelCheck::OutputInvariant) {
                body = if_then(
                    validator(ContractPredicate::Invariant).apply(vec![first(var(&step))]),
                    body,
                    refusal(&step_type, ModelCheck::OutputInvariant),
                );
            }
            body = let_in(
                &step,
                step_type.clone(),
                at.apply(vec![var(&current), var(&input)]),
                body,
            );
            if has(ModelCheck::Precondition) {
                body = if_then(
                    validator(ContractPredicate::Precondition)
                        .apply(vec![var(&current), var(&input)]),
                    body,
                    refusal(&step_type, ModelCheck::Precondition),
                );
            }
            if has(ModelCheck::InputInvariant) {
                body = if_then(
                    validator(ContractPredicate::Invariant).apply(vec![var(&current)]),
                    body,
                    refusal(&step_type, ModelCheck::InputInvariant),
                );
            }
            Ok(let_in(
                &current,
                state.r#type.clone(),
                state_argument.clone(),
                let_in(&input, input_type, argument.clone(), body),
            ))
        }
        (state, _) => Err(format!(
            "checked application of model `{}` expects {} argument(s), received {}",
            member_key(model),
            if state.is_some() { 2 } else { 1 },
            arguments.len()
        )
        .into()),
    }
}

pub(super) fn type_scope(declaration: &SemanticDeclaration) -> BTreeSet<String> {
    let parameters: &[String] = match declaration {
        SemanticDeclaration::Definition {
            type_parameters, ..
        }
        | SemanticDeclaration::Theorem {
            type_parameters, ..
        }
        | SemanticDeclaration::Realization {
            type_parameters, ..
        } => type_parameters,
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => &[],
    };
    parameters.iter().cloned().collect()
}

/// Check the runtime boundary of every term of `declaration`, then replace
/// each checked application by the ordinary term it means. A definition's
/// termination measure is formal whatever the definition is.
fn lower_terms(
    declaration: &mut SemanticDeclaration,
    executable: bool,
    env: &Environment<'_>,
) -> Result<(), SemanticFailure> {
    let owner = declaration.name().to_owned();
    let scope = type_scope(declaration);
    if let SemanticDeclaration::Definition {
        body, termination, ..
    } = declaration
    {
        check_boundary(&owner, body, executable, &scope, env)?;
        if let Some(termination) = termination {
            check_boundary(&owner, &termination.measure, false, &scope, env)?;
        }
    } else {
        let mut failure = None;
        let mut terms = Vec::new();
        super::declaration_terms(declaration, &mut |term| terms.push(term.clone()));
        // Visiting yields every subterm; checking the outermost terms is
        // enough because the check itself descends.
        let mut outer = Vec::new();
        match declaration {
            SemanticDeclaration::Realization { .. } => {
                outer.extend(terms_of(declaration).into_iter().cloned());
            }
            SemanticDeclaration::Instance { fields, .. } => {
                outer.extend(fields.iter().map(|field| field.value.clone()));
            }
            SemanticDeclaration::Theorem {
                statement, proof, ..
            } => {
                outer.push(statement.clone());
                super::proof_terms(proof, &mut |term| outer.push(term.clone()));
            }
            SemanticDeclaration::Structure { .. }
            | SemanticDeclaration::Class { .. }
            | SemanticDeclaration::Inductive { .. }
            | SemanticDeclaration::Definition { .. }
            | SemanticDeclaration::Artifact { .. }
            | SemanticDeclaration::Contract { .. }
            | SemanticDeclaration::Evidence { .. }
            | SemanticDeclaration::Model { .. }
            | SemanticDeclaration::Logic { .. }
            | SemanticDeclaration::InferenceRule { .. }
            | SemanticDeclaration::Verifier { .. }
            | SemanticDeclaration::Reasoner { .. } => {}
        }
        drop(terms);
        for term in &outer {
            if let Err(reason) = check_boundary(&owner, term, executable, &scope, env) {
                failure = Some(reason);
                break;
            }
        }
        if let Some(failure) = failure {
            return Err(failure);
        }
    }
    let mut counter = 0;
    let mut failure = None;
    super::declaration_terms_mut(declaration, &mut |term| {
        if failure.is_none() && matches!(term, SemanticTerm::CheckedApply { .. }) {
            match expand(term, &mut counter, &scope, env) {
                Ok(expanded) => *term = expanded,
                Err(reason) => failure = Some(reason),
            }
        }
    });
    failure.map_or(Ok(()), Err)
}

/// One ordinary declaration as linking checks it: its runtime boundary
/// checked and its checked model applications elaborated (§17.12). An
/// executable definition's body and every instance field are executable
/// code; theorems and other definitions are formal.
pub(super) fn lower_ordinary(
    declaration: &SemanticDeclaration,
    env: &Environment<'_>,
) -> Result<SemanticDeclaration, SemanticFailure> {
    let executable = match declaration {
        SemanticDeclaration::Definition { executable, .. } => *executable,
        SemanticDeclaration::Instance { .. } => true,
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => false,
    };
    // The elaborated copy is checked admitting generated binders, so every
    // source binder is checked as source first: no source name begins with
    // an underscore.
    let mut failure = None;
    super::declaration_binders(declaration, &mut |binder| {
        if failure.is_none() {
            if let Err(reason) = check_name(binder, "binder") {
                failure = Some(reason);
            }
        }
    });
    if let Some(reason) = failure {
        return Err(reason.into());
    }
    let mut out = declaration.clone();
    lower_terms(&mut out, executable, env)?;
    Ok(out)
}

// ---------------------------------------------------------------------------
// One model declaration.

/// Check one model declaration, elaborate it, and check each elaborated
/// declaration by the ordinary rules.
pub(super) fn check_declaration(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
    artifacts: &ArtifactStore,
    generated_names: &mut BTreeSet<String>,
) -> Result<Lowering, SemanticFailure> {
    let kind = declaration_construct(declaration).unwrap_or("declaration");
    super::require_language_1_2(env, kind)?;
    let lowering = match declaration {
        SemanticDeclaration::Artifact { .. } => check_artifact(declaration, env, artifacts)?,
        SemanticDeclaration::Contract { .. } => check_contract(declaration, env)?,
        SemanticDeclaration::Realization { .. } => check_realization(declaration, env)?,
        SemanticDeclaration::Evidence { .. } => check_evidence(declaration, env)?,
        SemanticDeclaration::Model { .. } => check_model(declaration, env)?,
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => Lowering::default(),
    };
    let name = declaration.name();
    for derived in &lowering.declarations {
        let derived_name = derived.name();
        if derived_name != name {
            check_declaration_name(derived_name, env)?;
            if !generated_names.insert(derived_name.to_owned()) {
                return Err(format!("duplicate generated name `{derived_name}`").into());
            }
        }
        env.derived = true;
        let checked = match derived {
            SemanticDeclaration::Definition { .. } => check_definition(derived, env),
            SemanticDeclaration::Theorem { .. } => check_theorem(derived, env),
            _ => Err(format!(
                "internal: `{derived_name}` is not a definition or theorem"
            )),
        };
        env.derived = false;
        env.current_type_parameters.clear();
        checked.map_err(|reason| {
            format!("{kind} `{name}` elaborates to `{derived_name}`, which is ill-formed: {reason}")
        })?;
    }
    for check in &lowering.checks {
        if !generated_names.insert(check.name.clone()) {
            return Err(format!("duplicate generated name `{}`", check.name).into());
        }
    }
    Ok(lowering)
}

#[cfg(test)]
mod tests {
    use super::{decode, decoded_nodes, term_node_count, ArtifactSchema, TensorElement};

    /// The charge made before decoding is the exact size of what decoding
    /// builds, at and around every chunk boundary of the list literal.
    #[test]
    fn decoded_nodes_is_exact() {
        let mut cases: Vec<(ArtifactSchema, Vec<u8>)> = vec![(ArtifactSchema::Bytes, vec![7; 5])];
        for lines in [0usize, 1, 31, 32, 33, 64, 65, 100, 1000, 4097] {
            cases.push((ArtifactSchema::Utf8Lines, b"a\n".repeat(lines)));
        }
        for shape in [
            vec![1u64],
            vec![32],
            vec![33],
            vec![97],
            vec![3, 50],
            vec![40, 3],
            vec![2, 3, 70],
            vec![65, 2, 1, 3],
        ] {
            let count = usize::try_from(shape.iter().product::<u64>()).expect("small");
            cases.push((
                ArtifactSchema::IntTensor {
                    element: TensorElement::Int16,
                    shape,
                },
                vec![0x81; count * 2],
            ));
        }
        for (schema, bytes) in cases {
            let decoded = decode(&schema, &bytes).expect("decodes");
            assert_eq!(
                decoded_nodes(&schema, &bytes),
                Ok(term_node_count(&decoded)),
                "{schema:?} over {} bytes",
                bytes.len()
            );
        }
    }
}
