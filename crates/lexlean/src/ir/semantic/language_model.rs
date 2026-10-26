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
//!
//! A context window and a text generator elaborate the same way: a window
//! to its capacity and the library's admission and push under its overflow
//! disposition, a generator to its scorer, its closed outcome types, and
//! the library's bounded generation loop under its policy, each with
//! generated theorems one library lemma proves.

use std::collections::{BTreeMap, BTreeSet};

use super::model::{self, Lowering};
use super::reasoning::{self, Formula, GeneratedTheorem, Proof, ProofTerm};
use super::{
    type_node_count, ArtifactRole, ArtifactSchema, Environment, FunctionInfo, GenerationPolicy,
    MemberRef, ModelUse, OverflowDisposition, SemanticConstructor, SemanticDeclaration,
    SemanticFailure, SemanticParameter, SemanticPrimitive, SemanticTerm, SemanticType,
    TokenizerScheme,
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
// The text interfaces a module declares and imports.

/// What linking knows about one context window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WindowInfo {
    capacity: u64,
    overflow: OverflowDisposition,
}

/// Every visible tokenizer and context window, by environment key (a local
/// name, or `module::name` for an import), so that a generator names them
/// across modules as a model names its contract.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Texts {
    tokenizers: BTreeMap<String, TokenizerScheme>,
    windows: BTreeMap<String, WindowInfo>,
}

impl Texts {
    /// The interfaces this module declares itself.
    pub(super) fn locals(&self) -> Self {
        Self {
            tokenizers: self
                .tokenizers
                .iter()
                .filter(|(key, _)| !key.contains("::"))
                .map(|(key, scheme)| (key.clone(), *scheme))
                .collect(),
            windows: self
                .windows
                .iter()
                .filter(|(key, _)| !key.contains("::"))
                .map(|(key, info)| (key.clone(), *info))
                .collect(),
        }
    }
}

/// Make the text interfaces an imported module declares visible under
/// `module::name`.
pub(super) fn register_import(
    module: &str,
    elaboration: &model::Elaboration,
    env: &mut Environment<'_>,
) {
    for (name, scheme) in &elaboration.text.tokenizers {
        env.texts
            .tokenizers
            .insert(format!("{module}::{name}"), *scheme);
    }
    for (name, info) in &elaboration.text.windows {
        env.texts.windows.insert(format!("{module}::{name}"), *info);
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
/// The generated binder of one token.
const TOKEN: &str = "__token";
/// The generated binder of a seed.
const SEED: &str = "__seed";

fn interface(reason: String) -> SemanticFailure {
    model::fail(code!("LLT4006"), reason)
}

/// A bound a text declaration must state is absent or zero (§17.12, *Text
/// components* rule 5).
fn unbounded(reason: String) -> SemanticFailure {
    model::fail(code!("LLT4016"), reason)
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

/// `list uint32 -> list int`, a generator's scorer as the library loop takes it.
fn scorer_type() -> SemanticType {
    SemanticType::Function {
        parameters: vec![tokens()],
        result: Box::new(list_of(SemanticType::Int)),
    }
}

/// `product (list uint32) nat`: a greedy run's tokens and outcome code.
fn greedy_run() -> SemanticType {
    model::product(tokens(), SemanticType::Nat)
}

/// `product (product (list uint32) nat) uint32`: a sampled run and its next
/// seed.
fn sample_run() -> SemanticType {
    model::product(greedy_run(), SemanticType::UInt32)
}

/// Register the library's functions a text declaration's elaboration applies, with
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
    let context = || model::option_type(tokens());
    let loop_parameters = || {
        vec![
            scorer_type(),
            SemanticType::Nat,
            SemanticType::Nat,
            SemanticType::Bool,
            SemanticType::UInt32,
            SemanticType::Nat,
            tokens(),
        ]
    };
    let seeded = || {
        let mut parameters = loop_parameters();
        parameters.push(SemanticType::UInt32);
        parameters
    };
    let windows: [(&str, Vec<SemanticType>, SemanticType); 13] = [
        (
            names::FITS,
            vec![SemanticType::Nat, tokens()],
            SemanticType::Bool,
        ),
        (
            names::ADMIT_REFUSE,
            vec![SemanticType::Nat, tokens()],
            context(),
        ),
        (
            names::ADMIT_KEEP_LAST,
            vec![SemanticType::Nat, tokens()],
            context(),
        ),
        (
            names::PUSH_REFUSE,
            vec![SemanticType::Nat, tokens(), SemanticType::UInt32],
            context(),
        ),
        (
            names::PUSH_KEEP_LAST,
            vec![SemanticType::Nat, tokens(), SemanticType::UInt32],
            context(),
        ),
        (
            names::FITS_OPTION,
            vec![SemanticType::Nat, context()],
            SemanticType::Bool,
        ),
        (
            names::SUFFIX_OPTION,
            vec![tokens(), context()],
            SemanticType::Bool,
        ),
        (
            names::XORSHIFT32,
            vec![SemanticType::UInt32],
            SemanticType::UInt32,
        ),
        (
            names::AT_MOST,
            vec![SemanticType::Nat, tokens()],
            SemanticType::Bool,
        ),
        (names::GENERATE_GREEDY, loop_parameters(), greedy_run()),
        (
            names::GENERATE_GREEDY_GUARDED,
            loop_parameters(),
            greedy_run(),
        ),
        (names::GENERATE_SAMPLE, seeded(), sample_run()),
        (names::GENERATE_SAMPLE_GUARDED, seeded(), sample_run()),
    ];
    let classifiers: [(&str, Vec<SemanticType>, SemanticType); 4] = [
        (
            names::FEATURES,
            vec![SemanticType::Nat, tokens()],
            list_of(SemanticType::Int),
        ),
        (
            names::FIT_WIDTH,
            vec![SemanticType::Nat, list_of(SemanticType::Int)],
            list_of(SemanticType::Int),
        ),
        (
            names::TOTAL,
            vec![list_of(SemanticType::Int)],
            SemanticType::Int,
        ),
        (names::TOKEN_COUNT, vec![tokens()], SemanticType::Int),
    ];
    for (name, parameters, result) in rows.into_iter().chain(windows).chain(classifiers) {
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
    law_over(
        name,
        vec![binder],
        statement,
        unfold,
        lemma,
        vec![pieces, bound],
    )
}

/// A generated theorem over several binders, proved by one library lemma
/// applied to `arguments` after unfolding exactly the named derived
/// definitions.
fn law_over(
    name: String,
    binders: Vec<SemanticParameter>,
    statement: SemanticTerm,
    unfold: &[&str],
    lemma: &str,
    arguments: Vec<SemanticTerm>,
) -> GeneratedTheorem {
    GeneratedTheorem {
        name,
        type_parameters: Vec::new(),
        parameters: binders,
        statement: Formula::Term { term: statement },
        proof: Proof::Unfold {
            definitions: unfold.iter().map(|name| model::local(name)).collect(),
            term: ProofTerm::Theorem {
                theorem: library_member(lemma),
                type_arguments: Vec::new(),
                arguments: arguments
                    .into_iter()
                    .map(|term| ProofTerm::Term { term })
                    .collect(),
            },
        },
    }
}

/// A text declaration's axiom policy is sorted, unique, and qualified: it is
/// the exact policy of every Lean declaration it elaborates to.
fn check_policy(kind: &str, name: &str, axioms: &[String]) -> Result<(), SemanticFailure> {
    if axioms.windows(2).any(|pair| pair[0] >= pair[1])
        || axioms.iter().any(|axiom| !super::legal_name(axiom))
    {
        return Err(interface(format!(
            "{kind} `{name}` axiom policy is not sorted, unique, and qualified"
        )));
    }
    Ok(())
}

fn library_call(function: &str, arguments: Vec<SemanticTerm>) -> SemanticTerm {
    model::call(&library_member(function), &[], arguments)
}

fn local_call(function: &str, arguments: Vec<SemanticTerm>) -> SemanticTerm {
    model::call(&model::local(function), &[], arguments)
}

fn is_true(term: SemanticTerm) -> SemanticTerm {
    model::eq(term, SemanticTerm::Bool { value: true })
}

fn plain_definition(
    name: &str,
    parameters: Vec<SemanticParameter>,
    result: SemanticType,
    body: SemanticTerm,
) -> SemanticDeclaration {
    model::ordinary_definition(name.to_owned(), &[], parameters, result, body, false)
}

/// A member of the module that declares `owner`: the derived definition
/// `owner.suffix`, in `owner`'s module when it is imported.
fn sibling(owner: &MemberRef, suffix: &str) -> MemberRef {
    MemberRef {
        module: owner.module.clone(),
        name: format!("{}.{suffix}", owner.name),
    }
}

// ---------------------------------------------------------------------------
// The elaboration of a context window.

/// Check a context window and elaborate it (§17.12, *Text components*): a
/// capacity of at least one token (`LLT4016`), elaborated to `capacity`,
/// `fit`, and `push` over the library's functions under the overflow
/// disposition, with `bounded`, `push_bounded`, and `exact` generated, and
/// `suffix` under `keep_last` or `refuses` under `refuse`.
fn check_context_window(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::ContextWindow {
        name,
        capacity,
        overflow,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_policy("context window", name, axioms)?;
    if *capacity == 0 {
        return Err(unbounded(format!(
            "context window `{name}` has capacity 0; a window holds at least one token"
        )));
    }
    register_library(env);
    let capacity_name = format!("{name}.capacity");
    let admit = format!("{name}.fit");
    let push = format!("{name}.push");
    let capacity_term = || local_call(&capacity_name, Vec::new());
    let (admit_with, push_with, bounded, push_bounded, exact) = match overflow {
        OverflowDisposition::Refuse => (
            names::ADMIT_REFUSE,
            names::PUSH_REFUSE,
            names::ADMIT_REFUSE_BOUNDED,
            names::PUSH_REFUSE_BOUNDED,
            names::ADMIT_REFUSE_EXACT,
        ),
        OverflowDisposition::KeepLast => (
            names::ADMIT_KEEP_LAST,
            names::PUSH_KEEP_LAST,
            names::ADMIT_KEEP_LAST_BOUNDED,
            names::PUSH_KEEP_LAST_BOUNDED,
            names::ADMIT_KEEP_LAST_EXACT,
        ),
    };
    let declarations = vec![
        plain_definition(
            &capacity_name,
            Vec::new(),
            SemanticType::Nat,
            model::nat(*capacity),
        ),
        plain_definition(
            &admit,
            vec![parameter(TOKENS, tokens())],
            model::option_type(tokens()),
            library_call(admit_with, vec![capacity_term(), model::var(TOKENS)]),
        ),
        plain_definition(
            &push,
            vec![
                parameter(TOKENS, tokens()),
                parameter(TOKEN, SemanticType::UInt32),
            ],
            model::option_type(tokens()),
            library_call(
                push_with,
                vec![capacity_term(), model::var(TOKENS), model::var(TOKEN)],
            ),
        ),
    ];
    let admitted = || local_call(&admit, vec![model::var(TOKENS)]);
    let fits = |value: bool| {
        model::eq(
            library_call(names::FITS, vec![capacity_term(), model::var(TOKENS)]),
            SemanticTerm::Bool { value },
        )
    };
    let over_tokens = || vec![parameter(TOKENS, tokens())];
    let mut theorems = vec![
        // Every admitted context fits the window.
        law_over(
            format!("{name}.bounded"),
            over_tokens(),
            is_true(library_call(
                names::FITS_OPTION,
                vec![capacity_term(), admitted()],
            )),
            &[&admit],
            bounded,
            vec![capacity_term(), model::var(TOKENS)],
        ),
        // Every pushed context fits the window.
        law_over(
            format!("{name}.push_bounded"),
            vec![
                parameter(TOKENS, tokens()),
                parameter(TOKEN, SemanticType::UInt32),
            ],
            is_true(library_call(
                names::FITS_OPTION,
                vec![
                    capacity_term(),
                    local_call(&push, vec![model::var(TOKENS), model::var(TOKEN)]),
                ],
            )),
            &[&push],
            push_bounded,
            vec![capacity_term(), model::var(TOKENS), model::var(TOKEN)],
        ),
        // A context that fits is admitted unchanged.
        law_over(
            format!("{name}.exact"),
            over_tokens(),
            model::implies(
                fits(true),
                model::eq(
                    admitted(),
                    model::constructor("Option.some", vec![tokens()], vec![model::var(TOKENS)]),
                ),
            ),
            &[&admit],
            exact,
            vec![capacity_term(), model::var(TOKENS)],
        ),
    ];
    theorems.push(match overflow {
        // What `keep_last` keeps is the end of the context.
        OverflowDisposition::KeepLast => law_over(
            format!("{name}.suffix"),
            over_tokens(),
            is_true(library_call(
                names::SUFFIX_OPTION,
                vec![model::var(TOKENS), admitted()],
            )),
            &[&admit],
            names::ADMIT_KEEP_LAST_SUFFIX,
            vec![capacity_term(), model::var(TOKENS)],
        ),
        // `refuse` admits nothing that does not fit.
        OverflowDisposition::Refuse => law_over(
            format!("{name}.refuses"),
            over_tokens(),
            model::implies(
                fits(false),
                model::eq(
                    admitted(),
                    model::constructor("Option.none", vec![tokens()], Vec::new()),
                ),
            ),
            &[&admit],
            names::ADMIT_REFUSE_REFUSES,
            vec![capacity_term(), model::var(TOKENS)],
        ),
    });
    env.texts.windows.insert(
        name.clone(),
        WindowInfo {
            capacity: *capacity,
            overflow: *overflow,
        },
    );
    Ok(Lowering {
        declarations,
        theorems,
        ..Lowering::default()
    })
}

// ---------------------------------------------------------------------------
// The elaboration of a text generator.

/// The closed outcome codes of the library's generation loop, in the order
/// `LexLeanText.genGo` writes them.
const STOPPED: u64 = 0;
const EXHAUSTED: u64 = 1;
const UNKNOWN_TOKEN: u64 = 2;

/// The constructors of a generator's `End` type, in declaration order.
const ENDS: [&str; 2] = ["stop", "exhausted"];
/// The constructors of a generator's `Failure` type, in declaration order:
/// the closed `text_failure` members.
const FAILURES: [&str; 3] = ["unknown_token", "context_overflow", "invalid_utf8"];

fn named(name: &str) -> SemanticType {
    SemanticType::Named {
        member: model::local(name),
        arguments: Vec::new(),
    }
}

fn enumeration(name: &str, constructors: &[&str]) -> SemanticDeclaration {
    SemanticDeclaration::Inductive {
        name: name.to_owned(),
        type_parameters: Vec::new(),
        parameters: Vec::new(),
        constructors: constructors
            .iter()
            .map(|constructor| SemanticConstructor {
                name: (*constructor).to_owned(),
                fields: Vec::new(),
            })
            .collect(),
        mutual: None,
    }
}

fn result_of(ok: SemanticType, error: SemanticType) -> SemanticType {
    SemanticType::Result {
        ok: Box::new(ok),
        error: Box::new(error),
    }
}

fn nat_equals(left: SemanticTerm, right: u64) -> SemanticTerm {
    model::primitive(
        SemanticPrimitive::Equal,
        vec![left, model::nat(right)],
        SemanticType::Bool,
    )
}

/// Check a text generator and elaborate it (§17.12, *Text components*).
///
/// Its tokenizer and window are prior text declarations; its scorer a prior
/// stateless model from `list uint32` to `list int` whose every runtime
/// check its evidence discharges; its stop a prior definition of one
/// `uint32`; `max_tokens` at least one (`LLT4016`). It elaborates to `score`,
/// the closed types `End` and `Failure`, `run` (the library's loop under the
/// policy), `generateTokens`, and `generate`, and generates `bounded` and
/// `context_bounded`. A `greedy` generator takes no seed; a `sample`
/// generator takes the seed and returns the next one.
#[allow(clippy::too_many_lines)]
fn check_text_generator(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::TextGenerator {
        name,
        tokenizer,
        window,
        scorer,
        max_tokens,
        stop,
        policy,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_policy("text generator", name, axioms)?;
    if *max_tokens == 0 {
        return Err(unbounded(format!(
            "text generator `{name}` has max_tokens 0; a generator states a bound of at least one token"
        )));
    }
    super::check_member(tokenizer, env)?;
    let tokenizer_key = super::member_key(tokenizer);
    if !env.texts.tokenizers.contains_key(&tokenizer_key) {
        return Err(interface(format!(
            "text generator `{name}` tokenizer `{tokenizer_key}` is not a prior tokenizer"
        )));
    }
    super::check_member(window, env)?;
    let window_key = super::member_key(window);
    let Some(window_info) = env.texts.windows.get(&window_key).copied() else {
        return Err(interface(format!(
            "text generator `{name}` window `{window_key}` is not a prior context window"
        )));
    };
    super::check_member(&scorer.member, env)?;
    let scorer_key = super::member_key(&scorer.member);
    let scoring = model::scorer_interface(scorer, env, interface).map_err(|failure| {
        interface(format!(
            "text generator `{name}` scorer `{scorer_key}`: {}",
            failure.reason
        ))
    })?;
    if scoring.stateful {
        return Err(interface(format!(
            "text generator `{name}` scorer `{scorer_key}` is stateful; a scorer maps one context to its scores"
        )));
    }
    if scoring.input != tokens() || scoring.output != list_of(SemanticType::Int) {
        return Err(interface(format!(
            "text generator `{name}` scorer `{scorer_key}` does not map `list uint32` to `list int`"
        )));
    }
    if !scoring.required.is_empty() {
        return Err(interface(format!(
            "text generator `{name}` scorer `{scorer_key}` needs runtime checks its evidence does not discharge; a scorer is applied on every step"
        )));
    }
    super::check_member(stop, env)?;
    let stop_key = super::member_key(stop);
    match env.functions.get(&stop_key) {
        Some(info)
            if info.parameters.is_empty()
                && info.type_parameters.is_empty()
                && info.result == SemanticType::UInt32 => {}
        _ => {
            return Err(interface(format!(
                "text generator `{name}` stop `{stop_key}` is not a prior definition of one `uint32` token"
            )));
        }
    }
    register_library(env);

    let sampled = matches!(policy, GenerationPolicy::Sample { .. });
    let score = format!("{name}.score");
    let end_type = format!("{name}.End");
    let failure_type = format!("{name}.Failure");
    let run = format!("{name}.run");
    let generate_tokens = format!("{name}.generateTokens");
    let generate = format!("{name}.generate");
    let end = || named(&end_type);
    let failure = || named(&failure_type);
    let end_value = |constructor: &str| {
        model::constructor(&format!("{end_type}.{constructor}"), Vec::new(), Vec::new())
    };
    let failure_value = |constructor: &str| {
        model::constructor(
            &format!("{failure_type}.{constructor}"),
            Vec::new(),
            Vec::new(),
        )
    };

    // The loop's arguments, in the library's order: the scorer, the
    // vocabulary size, the capacity, the disposition, the stop token, the
    // fuel, the prompt, and under `sample` the seed.
    let loop_arguments = |prompt: SemanticTerm| {
        let mut arguments = vec![
            model::function_ref(&model::local(&score), &[]),
            model::call(&sibling(tokenizer, "size"), &[], Vec::new()),
            model::call(&sibling(window, "capacity"), &[], Vec::new()),
            SemanticTerm::Bool {
                value: window_info.overflow == OverflowDisposition::KeepLast,
            },
            model::call(stop, &[], Vec::new()),
            model::nat(*max_tokens),
            prompt,
        ];
        if sampled {
            arguments.push(model::var(SEED));
        }
        arguments
    };
    let (loop_function, guarded_function, bounded_lemma, guarded_lemma, run_type) = if sampled {
        (
            names::GENERATE_SAMPLE,
            names::GENERATE_SAMPLE_GUARDED,
            names::GENERATE_SAMPLE_BOUNDED,
            names::GENERATE_SAMPLE_GUARDED_LAW,
            sample_run(),
        )
    } else {
        (
            names::GENERATE_GREEDY,
            names::GENERATE_GREEDY_GUARDED,
            names::GENERATE_GREEDY_BOUNDED,
            names::GENERATE_GREEDY_GUARDED_LAW,
            greedy_run(),
        )
    };
    let seeded = |mut parameters: Vec<SemanticParameter>| {
        if sampled {
            parameters.push(parameter(SEED, SemanticType::UInt32));
        }
        parameters
    };
    let seeded_arguments = |mut arguments: Vec<SemanticTerm>| {
        if sampled {
            arguments.push(model::var(SEED));
        }
        arguments
    };

    // `generateTokens`: the run's outcome code read into the closed types.
    let outcome = "__outcome";
    let (tokens_and_code, next_seed) = if sampled {
        (
            model::first(model::var(outcome)),
            Some(model::second(model::var(outcome))),
        )
    } else {
        (model::var(outcome), None)
    };
    let tokens_ok_type = |ok: SemanticType| match &next_seed {
        Some(_) => model::product(ok, SemanticType::UInt32),
        None => ok,
    };
    let token_result = result_of(tokens_ok_type(model::product(tokens(), end())), failure());
    let ok = |value: SemanticTerm, ty: &SemanticType| {
        model::constructor("Result.ok", vec![ty.clone(), failure()], vec![value])
    };
    let error = |constructor: &str, ty: &SemanticType| {
        model::constructor(
            "Result.error",
            vec![ty.clone(), failure()],
            vec![failure_value(constructor)],
        )
    };
    let tokens_ok = tokens_ok_type(model::product(tokens(), end()));
    let ended = |constructor: &str| {
        let value = model::pair(
            model::first(tokens_and_code.clone()),
            end_value(constructor),
        );
        let value = match &next_seed {
            Some(seed) => model::pair(value, seed.clone()),
            None => value,
        };
        ok(value, &tokens_ok)
    };
    let code = model::second(tokens_and_code.clone());
    let tokens_body = model::let_in(
        outcome,
        run_type.clone(),
        local_call(&run, seeded_arguments(vec![model::var(TOKENS)])),
        model::if_then(
            nat_equals(code.clone(), STOPPED),
            ended("stop"),
            model::if_then(
                nat_equals(code.clone(), EXHAUSTED),
                ended("exhausted"),
                model::if_then(
                    nat_equals(code, UNKNOWN_TOKEN),
                    error("unknown_token", &tokens_ok),
                    error("context_overflow", &tokens_ok),
                ),
            ),
        ),
    );

    // `generate`: the prompt encoded, the tokens generated, the output decoded.
    let text_ok = tokens_ok_type(model::product(SemanticType::String, end()));
    let generated = "__generated";
    let decoded = "__decoded";
    let failed = "__failed";
    let (generated_pair, generated_seed) = if sampled {
        (
            model::first(model::var(generated)),
            Some(model::second(model::var(generated))),
        )
    } else {
        (model::var(generated), None)
    };
    let text_value = {
        let value = model::pair(model::var(decoded), model::second(generated_pair.clone()));
        match generated_seed {
            Some(seed) => model::pair(value, seed),
            None => value,
        }
    };
    let text_body = model::matching(
        local_call(
            &generate_tokens,
            seeded_arguments(vec![model::call(
                &sibling(tokenizer, "encodeText"),
                &[],
                vec![model::var(TEXT)],
            )]),
        ),
        vec![
            (
                "Result.ok",
                vec![generated],
                model::matching(
                    model::call(
                        &sibling(tokenizer, "decodeText"),
                        &[],
                        vec![model::first(generated_pair)],
                    ),
                    vec![
                        ("Option.none", Vec::new(), error("invalid_utf8", &text_ok)),
                        ("Option.some", vec![decoded], ok(text_value, &text_ok)),
                    ],
                ),
            ),
            (
                "Result.error",
                vec![failed],
                model::constructor(
                    "Result.error",
                    vec![text_ok.clone(), failure()],
                    vec![model::var(failed)],
                ),
            ),
        ],
    );

    let declarations = vec![
        plain_definition(
            &score,
            vec![parameter(TOKENS, tokens())],
            list_of(SemanticType::Int),
            model::call(
                &scorer.member,
                &scorer.type_arguments,
                vec![model::var(TOKENS)],
            ),
        ),
        enumeration(&end_type, &ENDS),
        enumeration(&failure_type, &FAILURES),
        plain_definition(
            &run,
            seeded(vec![parameter(TOKENS, tokens())]),
            run_type.clone(),
            library_call(loop_function, loop_arguments(model::var(TOKENS))),
        ),
        plain_definition(
            &generate_tokens,
            seeded(vec![parameter(TOKENS, tokens())]),
            token_result,
            tokens_body,
        ),
        plain_definition(
            &generate,
            seeded(vec![parameter(TEXT, SemanticType::String)]),
            result_of(text_ok.clone(), failure()),
            text_body,
        ),
    ];

    let ran = local_call(&run, seeded_arguments(vec![model::var(TOKENS)]));
    let generated_tokens = if sampled {
        model::first(model::first(ran.clone()))
    } else {
        model::first(ran.clone())
    };
    let theorems = vec![
        // At most `max_tokens` tokens are generated.
        law_over(
            format!("{name}.bounded"),
            seeded(vec![parameter(TOKENS, tokens())]),
            is_true(library_call(
                names::AT_MOST,
                vec![model::nat(*max_tokens), generated_tokens],
            )),
            &[&run],
            bounded_lemma,
            loop_arguments(model::var(TOKENS)),
        ),
        // The scorer is consulted only on contexts that fit the window: the
        // run equals the run whose scorer sees nothing that does not fit.
        law_over(
            format!("{name}.context_bounded"),
            seeded(vec![parameter(TOKENS, tokens())]),
            model::eq(
                ran,
                library_call(guarded_function, loop_arguments(model::var(TOKENS))),
            ),
            &[&run],
            guarded_lemma,
            loop_arguments(model::var(TOKENS)),
        ),
    ];
    Ok(Lowering {
        declarations,
        theorems,
        ..Lowering::default()
    })
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
    env.texts.tokenizers.insert(name.clone(), *scheme);
    Ok(Lowering {
        declarations,
        theorems,
        ..Lowering::default()
    })
}

// ---------------------------------------------------------------------------
// The elaboration of a text classifier.

/// Check a text classifier and elaborate it (§17.12, *Text components*
/// rule 8): its tokenizer is a prior tokenizer and its model a prior
/// stateless model from `list int` whose every runtime check its evidence
/// discharges (it is applied on every text). It elaborates to `features`,
/// `classifyTokens`, and `classify`, and generates `features_width`,
/// `features_total`, and `features_fit`.
#[allow(clippy::too_many_lines)]
fn check_text_classifier(
    declaration: &SemanticDeclaration,
    env: &mut Environment<'_>,
) -> Result<Lowering, SemanticFailure> {
    let SemanticDeclaration::TextClassifier {
        name,
        tokenizer,
        model: classifier_model,
        axioms,
    } = declaration
    else {
        return Ok(Lowering::default());
    };
    check_policy("text classifier", name, axioms)?;
    super::check_member(tokenizer, env)?;
    let tokenizer_key = super::member_key(tokenizer);
    let Some(scheme) = env.texts.tokenizers.get(&tokenizer_key).copied() else {
        return Err(interface(format!(
            "text classifier `{name}` tokenizer `{tokenizer_key}` is not a prior tokenizer"
        )));
    };
    super::check_member(&classifier_model.member, env)?;
    let model_key = super::member_key(&classifier_model.member);
    let scoring = model::scorer_interface(classifier_model, env, interface).map_err(|failure| {
        interface(format!(
            "text classifier `{name}` model `{model_key}`: {}",
            failure.reason
        ))
    })?;
    if scoring.stateful {
        return Err(interface(format!(
            "text classifier `{name}` model `{model_key}` is stateful; a classifier maps one feature vector to its label"
        )));
    }
    if !scoring.required.is_empty() {
        return Err(interface(format!(
            "text classifier `{name}` model `{model_key}` needs runtime checks its evidence does not discharge; a classifier applies its model to every text"
        )));
    }
    if scoring.input != list_of(SemanticType::Int) {
        return Err(interface(format!(
            "text classifier `{name}` model `{model_key}` does not take `list int`"
        )));
    }
    let label = scoring.output;
    register_library(env);

    let features = format!("{name}.features");
    let classify_tokens = format!("{name}.classifyTokens");
    let classify = format!("{name}.classify");
    let size = || model::call(&sibling(tokenizer, "size"), &[], Vec::new());
    let pieces = || model::call(&sibling(tokenizer, "pieces"), &[], Vec::new());
    let encoded = || {
        model::call(
            &sibling(tokenizer, "encodeText"),
            &[],
            vec![model::var(TEXT)],
        )
    };
    let featured = |tokens: SemanticTerm| local_call(&features, vec![tokens]);
    let declarations = vec![
        plain_definition(
            &features,
            vec![parameter(TOKENS, tokens())],
            list_of(SemanticType::Int),
            library_call(names::FEATURES, vec![size(), model::var(TOKENS)]),
        ),
        plain_definition(
            &classify_tokens,
            vec![parameter(TOKENS, tokens())],
            label.clone(),
            model::call(
                &classifier_model.member,
                &classifier_model.type_arguments,
                vec![featured(model::var(TOKENS))],
            ),
        ),
        plain_definition(
            &classify,
            vec![parameter(TEXT, SemanticType::String)],
            label,
            local_call(&classify_tokens, vec![encoded()]),
        ),
    ];
    let total_lemma = match scheme {
        TokenizerScheme::LongestMatch => names::FEATURES_TOTAL_TEXT,
        TokenizerScheme::ByteLevel => names::FEATURES_TOTAL_TEXT_BYTES,
    };
    let over_tokens = || vec![parameter(TOKENS, tokens())];
    let theorems = vec![
        // The features have exactly `size` entries.
        law_over(
            format!("{name}.features_width"),
            over_tokens(),
            model::eq(
                model::primitive(
                    SemanticPrimitive::Length,
                    vec![featured(model::var(TOKENS))],
                    SemanticType::Nat,
                ),
                size(),
            ),
            &[&features],
            names::FEATURES_WIDTH,
            vec![size(), model::var(TOKENS)],
        ),
        // The features of an encoded text sum to its token count: no token
        // is dropped, because every token indexes the vocabulary.
        law_over(
            format!("{name}.features_total"),
            vec![parameter(TEXT, SemanticType::String)],
            model::eq(
                library_call(names::TOTAL, vec![featured(encoded())]),
                library_call(names::TOKEN_COUNT, vec![encoded()]),
            ),
            &[&features],
            total_lemma,
            vec![pieces(), model::var(TEXT)],
        ),
        // The fixed-width map leaves the features unchanged.
        law_over(
            format!("{name}.features_fit"),
            over_tokens(),
            model::eq(
                library_call(names::FIT_WIDTH, vec![size(), featured(model::var(TOKENS))]),
                featured(model::var(TOKENS)),
            ),
            &[&features],
            names::FEATURES_FIT,
            vec![size(), model::var(TOKENS)],
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
        SemanticDeclaration::TextClassifier { .. } => check_text_classifier(declaration, env)?,
        SemanticDeclaration::ContextWindow { .. } => check_context_window(declaration, env)?,
        SemanticDeclaration::TextGenerator { .. } => check_text_generator(declaration, env)?,
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
            // A generator's closed outcome types, registered as a reasoner's
            // generated inductives are.
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
            _ => Err(format!(
                "internal: `{derived_name}` is not a definition or an inductive"
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
            axioms: Vec::new(),
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
