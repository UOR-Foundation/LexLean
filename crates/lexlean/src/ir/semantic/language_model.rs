//! Language-1.2 text components (SPEC.md §17.12, *Text components*):
//! tokenizers, context windows, text generators, and text classifiers.
//!
//! Like a model or a reasoning declaration, a text declaration carries no
//! meaning of its own: linking elaborates it into ordinary language-1.2
//! definitions and theorems, which the ordinary rules check again. This
//! module is both halves of the family: the source traversal every other
//! declaration kind gets from `semantic.rs`, the construct names the closed
//! list of language-1.2 constructs is built from, and the elaboration of a
//! tokenizer. A text declaration states no term and binds no interface name
//! of its own — its tokens are data and its members are references — so
//! only its types and its closed members contribute to the traversal.
//!
//! A tokenizer elaborates to definitions that apply the functions of the
//! embedded laws library ([`crate::text_laws`]) to its vocabulary, and to
//! generated theorems the library's lemmas prove after unfolding exactly
//! those definitions, in the manner of the reasoning family's generated
//! theorems. The laws themselves are the library's: nothing is spliced, and
//! the generated module imports the library's generated Lean.

use std::collections::BTreeSet;

use super::model::{self, Lowering};
use super::reasoning::{self, Formula, GeneratedTheorem, Proof, ProofTerm};
use super::{
    type_node_count, ArtifactRole, ArtifactSchema, Environment, FunctionInfo, GenerationPolicy,
    MemberRef, ModelUse, SemanticDeclaration, SemanticFailure, SemanticParameter, SemanticTerm,
    SemanticType, TokenizerScheme,
};
use crate::code;
use crate::text_laws::{self, names};

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

// ---------------------------------------------------------------------------
// The elaboration of a tokenizer.

/// The generated binder of a byte string.
const BYTES: &str = "__bytes";
/// The generated binder of a token list.
const TOKENS: &str = "__tokens";
/// The generated binder of a text.
const TEXT: &str = "__text";

fn interface(reason: String) -> SemanticFailure {
    model::fail(code!("LLT4006"), reason)
}

/// A member of the laws library, as an elaboration names it.
fn library_member(name: &str) -> MemberRef {
    MemberRef {
        module: Some(text_laws::PREFIX.to_owned()),
        name: name.to_owned(),
    }
}

fn list_of(element: SemanticType) -> SemanticType {
    SemanticType::List {
        element: Box::new(element),
    }
}

/// `list uint32`, the token type.
fn tokens() -> SemanticType {
    list_of(SemanticType::UInt32)
}

/// `list string`, a vocabulary's typed value.
fn strings() -> SemanticType {
    list_of(SemanticType::String)
}

fn parameter(name: &str, ty: SemanticType) -> SemanticParameter {
    SemanticParameter {
        name: name.to_owned(),
        r#type: ty,
    }
}

/// Register the library's functions a tokenizer's elaboration applies, with
/// their signatures, so that the derived definitions and generated
/// statements type-check like any call (§17.12, *Text components* rule 3).
/// The native data is what the compiler generates the Lean from; linking
/// relies on these signatures, and the loader refuses a native module that
/// does not declare every name.
fn register_library(env: &mut Environment<'_>) {
    let rows: [(&str, Vec<SemanticType>, SemanticType); 8] = [
        (names::VOCAB_SIZE, vec![strings()], SemanticType::Nat),
        (
            names::ENCODE_WITH,
            vec![strings(), SemanticType::Bytes],
            tokens(),
        ),
        (names::ENCODE_BYTES, vec![SemanticType::Bytes], tokens()),
        (
            names::DECODE_WITH,
            vec![strings(), tokens()],
            model::option_type(SemanticType::Bytes),
        ),
        (
            names::ENCODE_TEXT,
            vec![strings(), SemanticType::String],
            tokens(),
        ),
        (
            names::ENCODE_TEXT_BYTES,
            vec![SemanticType::String],
            tokens(),
        ),
        (
            names::DECODE_TEXT,
            vec![strings(), tokens()],
            model::option_type(SemanticType::String),
        ),
        (
            names::ALL_BELOW,
            vec![SemanticType::Nat, tokens()],
            SemanticType::Bool,
        ),
    ];
    for (name, parameters, result) in rows {
        env.functions
            .entry(super::member_key(&library_member(name)))
            .or_insert(FunctionInfo {
                type_parameters: Vec::new(),
                parameters,
                result,
                executable: false,
            });
    }
}

/// A generated theorem: an ordinary statement over one binder, proved by one
/// library lemma applied to the vocabulary and the binder after unfolding
/// exactly the named derived definitions.
fn law(
    name: String,
    binder: SemanticParameter,
    statement: SemanticTerm,
    unfold: &[&str],
    lemma: &str,
    pieces: SemanticTerm,
) -> GeneratedTheorem {
    let bound = model::var(&binder.name);
    GeneratedTheorem {
        name,
        type_parameters: Vec::new(),
        parameters: vec![binder],
        statement: Formula::Term { term: statement },
        proof: Proof::Unfold {
            definitions: unfold.iter().map(|name| model::local(name)).collect(),
            term: ProofTerm::Theorem {
                theorem: library_member(lemma),
                type_arguments: Vec::new(),
                arguments: vec![
                    ProofTerm::Term { term: pieces },
                    ProofTerm::Term { term: bound },
                ],
            },
        },
    }
}

/// Check a tokenizer and elaborate it (§17.12, *Text components*): its
/// vocabulary is a prior artifact of role `vocabulary`, and it elaborates
/// to `pieces`, `size`, `encode`, `decode`, `encodeText`, and `decodeText`
/// over the library's functions, and generates `roundtrip`, `in_range`, and
/// `text_roundtrip`, which the library's lemmas prove.
fn check_tokenizer(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::Tokenizer {
        name,
        type_parameters,
        scheme,
        vocabulary,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    if !type_parameters.is_empty() {
        return Err(interface(format!(
            "tokenizer `{name}` declares type parameters; a tokenizer is monomorphic over `bytes`, `string`, and `list uint32`"
        )));
    }
    if axioms.windows(2).any(|pair| pair[0] >= pair[1])
        || axioms.iter().any(|axiom| !super::legal_name(axiom))
    {
        return Err(interface(format!(
            "tokenizer `{name}` axiom policy is not sorted, unique, and qualified"
        )));
    }
    super::check_member(&vocabulary.member, env)?;
    let key = super::member_key(&vocabulary.member);
    if !vocabulary.type_arguments.is_empty() {
        return Err(interface(format!(
            "tokenizer `{name}` vocabulary `{key}` takes type arguments; an artifact has none"
        )));
    }
    let Some((role, schema)) = model::artifact_info(&vocabulary.member, env) else {
        return Err(interface(format!(
            "tokenizer `{name}` vocabulary `{key}` is not a prior artifact"
        )));
    };
    if role != ArtifactRole::Vocabulary {
        return Err(interface(format!(
            "tokenizer `{name}` vocabulary `{key}` has role {role:?}; a tokenizer needs role Vocabulary"
        )));
    }
    if !matches!(schema, ArtifactSchema::Utf8Lines) {
        return Err(interface(format!(
            "tokenizer `{name}` vocabulary `{key}` is a {} artifact; a tokenizer needs the utf8_lines schema",
            model::schema_kind(&schema)
        )));
    }
    register_library(env);

    let pieces = format!("{name}.pieces");
    let size = format!("{name}.size");
    let encode = format!("{name}.encode");
    let decode = format!("{name}.decode");
    let encode_text = format!("{name}.encodeText");
    let decode_text = format!("{name}.decodeText");
    let pieces_term = || model::call(&model::local(&pieces), &[], Vec::new());
    let library = |function: &str, arguments: Vec<SemanticTerm>| {
        model::call(&library_member(function), &[], arguments)
    };
    let (encode_body, encode_text_body, roundtrip, in_range, text_roundtrip) = match scheme {
        TokenizerScheme::LongestMatch => (
            library(names::ENCODE_WITH, vec![pieces_term(), model::var(BYTES)]),
            library(names::ENCODE_TEXT, vec![pieces_term(), model::var(TEXT)]),
            names::ROUNDTRIP,
            names::IN_RANGE,
            names::TEXT_ROUNDTRIP,
        ),
        TokenizerScheme::ByteLevel => (
            library(names::ENCODE_BYTES, vec![model::var(BYTES)]),
            library(names::ENCODE_TEXT_BYTES, vec![model::var(TEXT)]),
            names::ROUNDTRIP_BYTES,
            names::IN_RANGE_BYTES,
            names::TEXT_ROUNDTRIP_BYTES,
        ),
    };
    let definition = |name: &str, parameters: Vec<SemanticParameter>, result, body| {
        model::ordinary_definition(name.to_owned(), &[], parameters, result, body, false)
    };
    let declarations = vec![
        definition(
            &pieces,
            Vec::new(),
            strings(),
            model::call(&vocabulary.member, &[], Vec::new()),
        ),
        definition(
            &size,
            Vec::new(),
            SemanticType::Nat,
            library(names::VOCAB_SIZE, vec![pieces_term()]),
        ),
        definition(
            &encode,
            vec![parameter(BYTES, SemanticType::Bytes)],
            tokens(),
            encode_body,
        ),
        definition(
            &decode,
            vec![parameter(TOKENS, tokens())],
            model::option_type(SemanticType::Bytes),
            library(names::DECODE_WITH, vec![pieces_term(), model::var(TOKENS)]),
        ),
        definition(
            &encode_text,
            vec![parameter(TEXT, SemanticType::String)],
            tokens(),
            encode_text_body,
        ),
        definition(
            &decode_text,
            vec![parameter(TOKENS, tokens())],
            model::option_type(SemanticType::String),
            library(names::DECODE_TEXT, vec![pieces_term(), model::var(TOKENS)]),
        ),
    ];
    let apply = |function: &str, arguments: Vec<SemanticTerm>| {
        model::call(&model::local(function), &[], arguments)
    };
    let theorems = vec![
        // `decode (encode b) = some b`.
        law(
            format!("{name}.roundtrip"),
            parameter(BYTES, SemanticType::Bytes),
            model::eq(
                apply(&decode, vec![apply(&encode, vec![model::var(BYTES)])]),
                model::constructor(
                    "Option.some",
                    vec![SemanticType::Bytes],
                    vec![model::var(BYTES)],
                ),
            ),
            &[&encode, &decode],
            roundtrip,
            pieces_term(),
        ),
        // Every token `encode b` produces is below `size`.
        law(
            format!("{name}.in_range"),
            parameter(BYTES, SemanticType::Bytes),
            model::eq(
                library(
                    names::ALL_BELOW,
                    vec![
                        apply(&size, Vec::new()),
                        apply(&encode, vec![model::var(BYTES)]),
                    ],
                ),
                SemanticTerm::Bool { value: true },
            ),
            &[&size, &encode],
            in_range,
            pieces_term(),
        ),
        // `decodeText (encodeText s) = some s`.
        law(
            format!("{name}.text_roundtrip"),
            parameter(TEXT, SemanticType::String),
            model::eq(
                apply(
                    &decode_text,
                    vec![apply(&encode_text, vec![model::var(TEXT)])],
                ),
                model::constructor(
                    "Option.some",
                    vec![SemanticType::String],
                    vec![model::var(TEXT)],
                ),
            ),
            &[&encode_text, &decode_text],
            text_roundtrip,
            pieces_term(),
        ),
    ];
    Ok(Lowering {
        declarations,
        theorems,
        ..Lowering::default()
    })
}

/// Check one text declaration in its environment and elaborate it. Every
/// derived definition is checked by the ordinary rules as a generated
/// declaration, and every generated theorem's statement is checked and
/// registered as a prior theorem of the module (§17.12).
pub(super) fn check_declaration(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
    generated_names: &mut BTreeSet<String>,
) -> Result<Lowering, SemanticFailure> {
    let kind = declaration_construct(declaration).unwrap_or("text declaration");
    super::require_language_1_2(env, kind)?;
    let lowering = match declaration {
        SemanticDeclaration::Tokenizer { .. } => check_tokenizer(declaration, env)?,
        _ => Lowering::default(),
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
            _ => Err(format!("internal: `{derived_name}` is not a definition")),
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
        let checked = reasoning::check_generated(generated, env);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Every text kind is a classified construct, so a kind left out of the
    /// closed list fails here rather than linking as nothing.
    #[test]
    fn every_text_kind_is_classified() {
        let tokenizer = SemanticDeclaration::Tokenizer {
            name: "T".to_owned(),
            type_parameters: Vec::new(),
            scheme: TokenizerScheme::ByteLevel,
            vocabulary: ModelUse {
                member: model::local("V"),
                type_arguments: Vec::new(),
            },
            axioms: Vec::new(),
        };
        assert_eq!(
            elaborated_construct(&tokenizer),
            Some("tokenizer declaration")
        );
        let window = SemanticDeclaration::ContextWindow {
            name: "W".to_owned(),
            capacity: 1,
            overflow: super::super::OverflowDisposition::Refuse,
        };
        assert_eq!(
            elaborated_construct(&window),
            Some("context_window declaration")
        );
    }

    /// The library functions an elaboration applies are the ones the loader
    /// requires of the native module, so the two lists cannot drift apart.
    #[test]
    fn registered_functions_are_required_names() {
        for name in [
            names::VOCAB_SIZE,
            names::ENCODE_WITH,
            names::ENCODE_BYTES,
            names::DECODE_WITH,
            names::ENCODE_TEXT,
            names::ENCODE_TEXT_BYTES,
            names::DECODE_TEXT,
            names::ALL_BELOW,
        ] {
            assert!(
                names::ALL.contains(&name),
                "{name} is required of the library"
            );
        }
    }
}
