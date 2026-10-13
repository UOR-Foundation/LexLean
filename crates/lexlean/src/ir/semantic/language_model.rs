//! Language-1.2 text components (SPEC.md §17.12, *Text components*):
//! tokenizers, context windows, text generators, and text classifiers.
//!
//! Like a model or a reasoning declaration, a text declaration carries no
//! meaning of its own: linking elaborates it into ordinary language-1.2
//! definitions and theorems, which the ordinary rules check again. This
//! module is the structural half of the family: the source traversal every
//! other declaration kind gets from `semantic.rs`, and the construct names
//! the closed list of language-1.2 constructs is built from. A text
//! declaration states no term and binds no interface name of its own — its
//! tokens are data and its members are references — so only its types and
//! its closed members contribute to the traversal.

use super::{
    type_node_count, GenerationPolicy, ModelUse, SemanticDeclaration, SemanticTerm, SemanticType,
};

/// Every type a text declaration states directly, in declaration order: the
/// explicit type arguments of each model use it names.
fn use_types<'a>(model_use: &'a ModelUse, out: &mut Vec<&'a SemanticType>) {
    out.extend(model_use.type_arguments.iter());
}

fn types_of(declaration: &SemanticDeclaration) -> Vec<&SemanticType> {
    let mut out = Vec::new();
    match declaration {
        SemanticDeclaration::Tokenizer { vocabulary, .. } => use_types(vocabulary, &mut out),
        SemanticDeclaration::TextGenerator { scorer, .. } => use_types(scorer, &mut out),
        SemanticDeclaration::TextClassifier { model, .. } => use_types(model, &mut out),
        SemanticDeclaration::ContextWindow { .. }
        | SemanticDeclaration::Structure { .. }
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
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => {}
    }
    out
}

/// Visit every type a text declaration states directly.
pub(super) fn declaration_types(
    declaration: &SemanticDeclaration,
    visit: &mut impl FnMut(&SemanticType),
) {
    for ty in types_of(declaration) {
        visit(ty);
    }
}

/// Visit every interface binder a text declaration names. A text declaration
/// binds no interface name: its member references name declarations and its
/// tokens are values, so there is nothing to visit.
pub(super) fn declaration_binders(
    _declaration: &SemanticDeclaration,
    _visit: &mut impl FnMut(&str),
) {
}

/// Visit every term a text declaration carries. It carries none: a tokenizer
/// is a scheme and a vocabulary, a window a capacity and a disposition, a
/// generator member references and bounds, and a classifier member
/// references.
pub(super) fn declaration_terms(
    _declaration: &SemanticDeclaration,
    _visit: &mut impl FnMut(&SemanticTerm),
) {
}

/// Visit every term a text declaration carries mutably. It carries none, as
/// `declaration_terms`.
pub(super) fn declaration_terms_mut(
    _declaration: &mut SemanticDeclaration,
    _visit: &mut impl FnMut(&mut SemanticTerm),
) {
}

fn count(values: usize) -> u64 {
    u64::try_from(values).unwrap_or(u64::MAX)
}

/// The nodes a text declaration's own source charges to `max_ir_nodes`:
/// every type it states, its type parameters, and every member reference and
/// closed member it names. Its elaboration is charged separately (§17.12).
pub(super) fn source_node_count(declaration: &SemanticDeclaration) -> u64 {
    let types: u64 = types_of(declaration).into_iter().map(type_node_count).sum();
    let shape: u64 = match declaration {
        // The scheme, the vocabulary reference, and the type parameters.
        SemanticDeclaration::Tokenizer {
            type_parameters, ..
        } => count(type_parameters.len()) + 2,
        // The capacity and the overflow disposition.
        SemanticDeclaration::ContextWindow { .. } => 2,
        // The tokenizer, window, scorer, and stop references, `max_tokens`,
        // and the policy: `sample` names a generator and a distribution.
        SemanticDeclaration::TextGenerator { policy, .. } => {
            5 + match policy {
                GenerationPolicy::Greedy => 1,
                GenerationPolicy::Sample { .. } => 3,
            }
        }
        // The tokenizer and model references.
        SemanticDeclaration::TextClassifier { .. } => 2,
        _ => 0,
    };
    types.saturating_add(shape)
}

/// The kind of a text declaration or of any other family declaration: a
/// declaration linking elaborates into ordinary declarations (§17.12).
#[must_use]
pub fn elaborated_construct(declaration: &SemanticDeclaration) -> Option<&'static str> {
    declaration_construct(declaration)
        .or_else(|| super::model::declaration_construct(declaration))
        .or_else(|| super::reasoning::declaration_construct(declaration))
}

/// The kind of a text declaration, for diagnostics and the closed list of
/// language-1.2 constructs.
#[must_use]
pub fn declaration_construct(declaration: &SemanticDeclaration) -> Option<&'static str> {
    match declaration {
        SemanticDeclaration::Tokenizer { .. } => Some("tokenizer declaration"),
        SemanticDeclaration::ContextWindow { .. } => Some("context_window declaration"),
        SemanticDeclaration::TextGenerator { .. } => Some("text_generator declaration"),
        SemanticDeclaration::TextClassifier { .. } => Some("text_classifier declaration"),
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
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. } => None,
    }
}
