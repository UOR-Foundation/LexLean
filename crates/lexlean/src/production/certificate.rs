//! The source-to-calculus preservation certificate (SPEC.md §17.17.4).
//!
//! For one lowered root this generates a Lean module whose theorem `root`
//! states, for every argument, that the lowered program run from function 0
//! on the encoded arguments converges, and that it overflows exactly when
//! the generated width predicate is false and otherwise returns the
//! encoding of the root's own Lean denotation. Lean's kernel checks it.
//!
//! The proof is derived from the *source*: every definition instance gets a
//! width predicate (`__fits_<n>`) and a relation theorem (`__rel_<n>`) built
//! node by node from its semantic IR by one fixed rule per construct. The
//! lowered program enters only as the literal `__prog` the theorems are
//! about and through the [`Layout`] that names which function realizes which
//! instance. A lowering that realizes a construct other than as its rule
//! says --- a mutated operator, branch, constructor, index, or call ---
//! leaves a proof step whose statement does not reduce to the program's,
//! and the kernel refuses the module (§17.17.6).
//!
//! The certificate is translation validation: one kernel-checked theorem per
//! root, never a theorem about the lowering function itself.

// A `match` naming both `Some` and `None` is used where `if let` and
// `while let` would hide the second case; the audit forbids those forms here.
#![allow(
    clippy::single_match,
    clippy::single_match_else,
    clippy::while_let_loop
)]
// The compiler is the second line of defence behind the audit: a binding
// catch-all over an enum (`other =>`) is a default too.
#![deny(
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants
)]

use std::collections::{BTreeMap, BTreeSet};

mod data;
mod data_a;
pub mod data_b;

use super::eligibility::BUILTIN_OWNERS;
use super::lower::{fixed_kind, Layout, Lowered, Origin, Validation};
use super::source::{Local, Site, Source};
use super::RootReport;
use crate::backend::semantic::identifier as semantic_identifier;
use crate::calculus::{IntKind, Program, Ty};
use crate::code;
use crate::diagnostic::Diagnostic;
use crate::ir::semantic::{
    MemberRef, SemanticAssignment, SemanticBranch, SemanticDeclaration, SemanticEdge,
    SemanticInteger, SemanticMapEntry, SemanticParameter, SemanticPrimitive, SemanticTerm,
    SemanticTermination, SemanticType,
};

/// The preservation library namespace (§17.17.3).
pub const LIBRARY: &str = "LexLeanPreservation";

/// The library modules every certificate imports, in import order.
pub const LIBRARY_MODULES: [&str; 7] = [
    "LexLeanPreservation.Core",
    "LexLeanPreservation.Values",
    "LexLeanPreservation.Primitives",
    "LexLeanPreservation.Fixed",
    "LexLeanPreservation.Keys",
    "LexLeanPreservation.Templates",
    "LexLeanPreservation.Validate",
];

const SYNTAX: &str = "LexLeanTarget.TargetSyntax";

/// One generated certificate module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Certificate {
    /// The Lean module name.
    pub module: String,
    /// The module text.
    pub text: String,
    /// The fully qualified name of the root theorem.
    pub theorem: String,
    /// The fully qualified name of the theorem's observation, a function
    /// of the root's arguments.
    pub denote: String,
    /// The generated user modules the certificate imports.
    pub imports: Vec<String>,
    /// The certificate as kernel-term data, when it is held as data: a
    /// native core module the core backend rendered `text` from (SP-14).
    pub data: Option<crate::ir::core::CoreModule>,
    /// The record of a certificate held as data: its authoring notation.
    pub source: Option<String>,
}

/// One constructor of a closed document type, as tooling that writes
/// source values needs it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceConstructor {
    /// The fully qualified Lean constructor.
    pub lean: String,
    /// Its closed field types, in order.
    pub fields: Vec<SemanticType>,
}

/// The runtime type of a closed source type and, for a document type, its
/// constructors in declaration order (a structure has one). The
/// differential evaluator writes its sampled source values with these, so
/// Lean reads exactly the values the realization's interpreter receives.
///
/// # Errors
///
/// Returns `LLI9001` when the type is not closed or names no document.
pub fn source_type(
    modules: &BTreeMap<String, super::eligibility::LinkedModule<'_>>,
    ty: &SemanticType,
) -> Result<(RuntimeType, Vec<SourceConstructor>), Diagnostic> {
    let source = Source { modules };
    let lean = runtime_type(&source, ty).map_err(internal)?;
    let constructors = match ty {
        SemanticType::Named {
            member,
            arguments: _,
        } => {
            let module = member
                .module
                .clone()
                .ok_or_else(|| internal("an unanchored document type"))?;
            let owner = identifier(&format!(
                "{}.{}",
                source.lean_module(&module).map_err(internal)?,
                member.name
            ));
            let shape = source.document(ty).map_err(internal)?;
            shape
                .names
                .iter()
                .zip(&shape.fields)
                .map(|(name, fields)| SourceConstructor {
                    lean: format!("{owner}.{}", identifier(name)),
                    fields: fields.clone(),
                })
                .collect()
        }
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::Option { value: _ }
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::List { element: _ }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => Vec::new(),
    };
    Ok((lean, constructors))
}

/// The closed parameters and result of a root definition.
///
/// # Errors
///
/// Returns `LLI9001` when `module` declares no definition `name`.
pub fn root_signature(
    modules: &BTreeMap<String, super::eligibility::LinkedModule<'_>>,
    module: &str,
    name: &str,
) -> Result<(Vec<SemanticParameter>, SemanticType), Diagnostic> {
    Source { modules }
        .signature(module, name, &[])
        .map_err(internal)
}

/// A generation failure: the limit the certificate was generated under
/// (`LLS8002`), or an inconsistency inside LexLean (`LLI9001`).
fn failure(reason: String) -> Diagnostic {
    match reason.strip_prefix(super::lower::LIMIT) {
        Some(limit) => Diagnostic::new(
            code!("LLS8002"),
            format!("max_file_bytes exceeded in phase certificates: {limit}"),
        ),
        None => internal(reason),
    }
}

fn internal(reason: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(code!("LLI9001"), format!("phase preservation: {reason}"))
}

/// A local of a certificate scope: its source binding and, for a
/// function-typed local, the binders of its companion: the width predicate,
/// function index, captured values, and relation hypothesis of the closure
/// it is bound to.
#[derive(Clone)]
struct CLocal {
    local: Local,
    /// The local's type as the generic declaration writes it, before the
    /// instance's type arguments are substituted.
    generic: SemanticType,
    /// Whether the local is function-typed, so carries a companion.
    has_companion: bool,
}

/// What a well-founded body numbers: the hypothesis of each `if` and
/// `match` node, by address.
type Hypotheses = BTreeMap<usize, usize>;

#[derive(Clone)]
struct Ctx {
    site: Site,
    /// The same module with no instantiation: where generic types are read.
    generic: Site,
    /// The instance's type parameters, in declaration order, with their
    /// closed arguments.
    parameters: Vec<(String, SemanticType)>,
    locals: Vec<CLocal>,
    hypotheses: Hypotheses,
    function: u64,
}

impl Ctx {
    fn new(
        site: Site,
        parameters: Vec<(String, SemanticType)>,
        locals: Vec<CLocal>,
        hypotheses: Hypotheses,
        function: u64,
    ) -> Self {
        let generic = Site {
            module: site.module.clone(),
            substitution: BTreeMap::new(),
        };
        Self {
            site,
            generic,
            parameters,
            locals,
            hypotheses,
            function,
        }
    }

    fn lookup(&self, name: &str) -> Result<&CLocal, String> {
        self.locals
            .iter()
            .rev()
            .find(|local| local.local.name == name)
            .ok_or_else(|| format!("local `{name}` is unbound"))
    }

    fn sources(&self) -> Vec<Local> {
        self.locals
            .iter()
            .map(|local| local.local.clone())
            .collect()
    }

    fn generics(&self) -> Vec<Local> {
        self.locals
            .iter()
            .map(|local| Local {
                name: local.local.name.clone(),
                ty: local.generic.clone(),
            })
            .collect()
    }

    fn with(&self, extra: Vec<(Local, SemanticType)>) -> Self {
        let mut out = self.clone();
        out.locals
            .extend(extra.into_iter().map(|(local, generic)| CLocal {
                local,
                generic,
                has_companion: false,
            }));
        out
    }
}

/// How a function's relation theorem recurses.
#[derive(Clone)]
enum Recursion {
    None,
    /// Structural recursion on parameter `argument` by top-level equations.
    Structural {
        argument: String,
    },
    /// Well-founded recursion with the source's measure and evidence.
    WellFounded {
        termination: SemanticTermination,
        group: BTreeSet<String>,
    },
}

/// One function the certificate states a relation for.
// A certificate holds one subject per function, so boxing the larger
// variant would buy nothing but indirection.
#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
enum Subject<'a> {
    Definition {
        module: String,
        name: String,
        type_arguments: Vec<SemanticType>,
        /// Parameters with their closed and generic types.
        locals: Vec<(Local, SemanticType)>,
        result: SemanticType,
        body: &'a SemanticTerm,
        recursion: Recursion,
        site: Site,
        type_parameters: Vec<(String, SemanticType)>,
    },
    Instance {
        module: String,
        name: String,
        class: SemanticType,
        fields: &'a [SemanticAssignment],
    },
    Lambda {
        site: Site,
        captures: Vec<CLocal>,
        parameters: Vec<(Local, SemanticType)>,
        body: &'a SemanticTerm,
        hypotheses: Hypotheses,
        type_parameters: Vec<(String, SemanticType)>,
    },
}

struct Gen<'a> {
    source: Source<'a>,
    layout: &'a Layout,
    program: &'a Program,
    instances: BTreeMap<String, u64>,
    lambdas: BTreeMap<(u64, u64), u64>,
    templates: BTreeMap<(String, Vec<Ty>), u64>,
    adts: BTreeMap<String, u64>,
    subjects: BTreeMap<u64, Subject<'a>>,
    /// Lambdas found while walking each function, by ordinal.
    found: BTreeMap<u64, u64>,
    /// Container types nested inside a recursive group of document types, by
    /// type text: their encoders must join the group's mutual block, because
    /// Lean accepts structural recursion through a nested occurrence only
    /// when the container gets its own auxiliary function.
    nested: BTreeMap<String, Nested>,
    /// The bytes a certificate may take, `max_file_bytes`: generation stops
    /// with a limit as soon as a part it has built is beyond it.
    limit: u64,
    /// The module being generated, which its declarations are written under
    /// by their full names wherever a parameter's name could shadow them.
    module: String,
}

/// A container type that occurs nested in a recursive group of document
/// types.
struct Nested {
    index: u64,
    ty: SemanticType,
    /// The first ADT of the group, which names its mutual block.
    group: u64,
}

/// Certificate E of one lowered root in one target (SPEC.md §17.17):
/// certificate A's observation of the root on its encoded arguments,
/// realized by the root's rendering, composed from certificate A (module
/// `module_a`) and certificate B (module `module_b`), as module `module`.
/// `fallible` is whether the rendering of the root returns `R<T>`.
///
/// # Errors
///
/// Returns `LLI9001` when the layout and the source disagree, which a
/// lowering produced by [`super::lower::lower_root`] rules out.
#[allow(clippy::too_many_arguments)]
pub fn certificate_e(
    modules: &BTreeMap<String, super::eligibility::LinkedModule<'_>>,
    root_module: &str,
    root_name: &str,
    lowered: &Lowered,
    module_a: &str,
    module_b: &str,
    module: &str,
    fallible: bool,
    max_bytes: u64,
) -> Result<Certificate, Diagnostic> {
    let mut generator = generator(modules, lowered, max_bytes);
    generator.nest().map_err(failure)?;
    let data = generator
        .compose_data(root_module, root_name, module_a, module_b, module, fallible)
        .map_err(failure)?;
    let text = crate::backend::core::lean_text_legacy(&data, module)
        .map_err(|diagnostic| internal(diagnostic.message))?;
    let source = record_source(module, &data, "E").map_err(internal)?;
    generator
        .within(source.len(), "certificate E")
        .map_err(failure)?;
    Ok(Certificate {
        module: module.to_owned(),
        theorem: format!("{module}.root"),
        denote: format!("{module_a}.denote"),
        text,
        imports: Vec::new(),
        data: Some(data),
        source: Some(source),
    })
}

/// The record of a certificate held as data: the whole source of the module
/// in the authoring notation (SPEC.md §17.21.8), which reads back to the data.
fn record_source(
    module: &str,
    data: &crate::ir::core::CoreModule,
    kind: &str,
) -> Result<String, String> {
    let name = module.rsplit('.').next().unwrap_or(module);
    crate::sdk::assemble_core(
        &crate::sdk::SourceHeader {
            name,
            glossaries: &[],
            imports: &[],
            title: &format!("Certificate {kind} {module}"),
        },
        data,
    )
    .map_err(|error| error.message)
}

/// The generator of a lowered root's certificates, before nesting.
fn generator<'a>(
    modules: &'a BTreeMap<String, super::eligibility::LinkedModule<'a>>,
    lowered: &'a Lowered,
    limit: u64,
) -> Gen<'a> {
    let source = Source { modules };
    let mut instances = BTreeMap::new();
    let mut lambdas = BTreeMap::new();
    let mut templates = BTreeMap::new();
    for (index, origin) in lowered.layout.functions.iter().enumerate() {
        let index = index as u64;
        match origin {
            Origin::Definition {
                module: _,
                name: _,
                type_arguments: _,
                instance,
            } => {
                instances.insert(instance.clone(), index);
            }
            Origin::Instance { module, name } => {
                instances.insert(
                    source.lean_name(
                        module,
                        &MemberRef {
                            module: Some(module.clone()),
                            name: name.clone(),
                        },
                    ),
                    index,
                );
            }
            Origin::Lambda { owner, ordinal } => {
                lambdas.insert((*owner, *ordinal), index);
            }
            Origin::Template {
                template,
                types,
                entry,
                member,
            } => {
                if *member == 0 {
                    templates.insert((template.name().to_owned(), types.clone()), *entry);
                }
            }
            // The boundary's functions are related by their own theorems,
            // which no source construct calls.
            Origin::Validator {
                ty: _,
                module: _,
                kind: _,
            }
            | Origin::Entry { validators: _ } => {}
        }
    }
    let adts = lowered
        .layout
        .adts
        .iter()
        .enumerate()
        .map(|(index, adt)| (adt.text.clone(), index as u64))
        .collect();
    Gen {
        source,
        layout: &lowered.layout,
        program: &lowered.program,
        instances,
        lambdas,
        templates,
        adts,
        subjects: BTreeMap::new(),
        found: BTreeMap::new(),
        nested: BTreeMap::new(),
        limit,
        module: String::new(),
    }
}

/// Generate the certificate of one lowered root as module `module`.
///
/// # Errors
///
/// Returns `LLI9001` when the layout and the source disagree, which a
/// lowering produced by [`super::lower::lower_root`] rules out.
pub fn certificate(
    modules: &BTreeMap<String, super::eligibility::LinkedModule<'_>>,
    root_module: &str,
    root_name: &str,
    report: &RootReport,
    lowered: &Lowered,
    module: &str,
    max_bytes: u64,
) -> Result<Certificate, Diagnostic> {
    let mut generator = generator(modules, lowered, max_bytes);
    generator.nest().map_err(failure)?;
    let data = generator
        .module_data(root_module, root_name, report, module)
        .map_err(failure)?;
    let text = crate::backend::core::lean_text_legacy(&data, module)
        .map_err(|diagnostic| internal(diagnostic.message))?;
    // The certificate's weight is the module as recorded, in the authoring
    // notation: the rendering to Lean text is the core backend's, and its size
    // is the backend's, not a property of the certificate.
    let source = record_source(module, &data, "A").map_err(internal)?;
    generator
        .within(source.len(), "certificate A")
        .map_err(failure)?;
    let mut imports: Vec<String> = modules
        .values()
        .map(|linked| linked.lean_module.to_owned())
        .collect();
    imports.sort();
    Ok(Certificate {
        module: module.to_owned(),
        theorem: format!("{module}.root"),
        denote: format!("{module}.denote"),
        text,
        imports,
        data: Some(data),
        source: Some(source),
    })
}

/// The Lean spelling of a semantic name in a certificate: the backend's, and
/// in addition quoted when a segment is spelled like a token the token audit
/// forbids (`kernel`, `extern`), so that a user's name is an identifier to the
/// audit as it is to Lean, whatever it is.
///
/// A local that linking generated (`__acc`, `__b`, `__s`, `__step`: the
/// elaboration of a model or a reasoner names its own binders so) is spelled
/// `_y` and the rest. The certificates bind names of their own that begin with
/// two underscores (`__b` for the condition of a conditional, `__a`, `__x0`),
/// and a source local of the same spelling under one of those binders is
/// captured: `Step.Burn __b` was a Boolean. No source name begins with an
/// underscore, so the other spelling is no one's.
fn identifier(name: &str) -> String {
    if name.starts_with("__") && !name.contains('.') {
        return format!("_y{}", name.chars().skip(2).collect::<String>());
    }
    semantic_identifier(name)
        .split('.')
        .map(|segment| {
            if super::preserve::FORBIDDEN_TOKENS.contains(&segment) {
                format!("«{segment}»")
            } else {
                segment.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join(".")
}

// --- Rendering the target program -------------------------------------------

/// A closed source type as the runtime spells it, by the structure of its
/// Lean type and not by its text: the differential evaluator writes its
/// sampled values and their types from this.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeType {
    /// A type of Lean's prelude with no arguments, by name.
    Atom(&'static str),
    /// `Option` of a type.
    Option(Box<RuntimeType>),
    /// `Except` of the error and the value.
    Except(Box<RuntimeType>, Box<RuntimeType>),
    /// `List` of a type.
    List(Box<RuntimeType>),
    /// `Prod` of two types.
    Prod(Box<RuntimeType>, Box<RuntimeType>),
    /// A function type: its parameters, then its result.
    Function(Vec<RuntimeType>, Box<RuntimeType>),
    /// A document type by its qualified Lean name, applied to its arguments.
    Named(String, Vec<RuntimeType>),
}

fn runtime_type(source: &Source<'_>, ty: &SemanticType) -> Result<RuntimeType, String> {
    let again = |ty: &SemanticType| runtime_type(source, ty).map(Box::new);
    Ok(match ty {
        SemanticType::Nat => RuntimeType::Atom("Nat"),
        SemanticType::Bool => RuntimeType::Atom("Bool"),
        SemanticType::Unit => RuntimeType::Atom("Unit"),
        SemanticType::Int => RuntimeType::Atom("Int"),
        SemanticType::Int8 => RuntimeType::Atom("Int8"),
        SemanticType::Int16 => RuntimeType::Atom("Int16"),
        SemanticType::Int32 => RuntimeType::Atom("Int32"),
        SemanticType::Int64 => RuntimeType::Atom("Int64"),
        SemanticType::UInt8 => RuntimeType::Atom("UInt8"),
        SemanticType::UInt16 => RuntimeType::Atom("UInt16"),
        SemanticType::UInt32 => RuntimeType::Atom("UInt32"),
        SemanticType::UInt64 => RuntimeType::Atom("UInt64"),
        SemanticType::String => RuntimeType::Atom("String"),
        SemanticType::Bytes => RuntimeType::Atom("ByteArray"),
        SemanticType::Ordering => RuntimeType::Atom("Ordering"),
        SemanticType::ContractViolation | SemanticType::ReasoningFailure => RuntimeType::Prod(
            Box::new(RuntimeType::Atom("Bool")),
            Box::new(RuntimeType::Atom("Bool")),
        ),
        SemanticType::Option { value } => RuntimeType::Option(again(value)?),
        SemanticType::Result { ok, error } => RuntimeType::Except(again(error)?, again(ok)?),
        SemanticType::List { element } | SemanticType::Set { element } => {
            RuntimeType::List(again(element)?)
        }
        SemanticType::Product { left, right } => RuntimeType::Prod(again(left)?, again(right)?),
        SemanticType::Map { key, value } => {
            RuntimeType::List(Box::new(RuntimeType::Prod(again(key)?, again(value)?)))
        }
        SemanticType::Function { parameters, result } => RuntimeType::Function(
            parameters
                .iter()
                .map(|parameter| runtime_type(source, parameter))
                .collect::<Result<_, _>>()?,
            again(result)?,
        ),
        SemanticType::Named { member, arguments } => {
            let module = member.module.clone().ok_or("an unanchored document type")?;
            RuntimeType::Named(
                identifier(&format!("{}.{}", source.lean_module(&module)?, member.name)),
                arguments
                    .iter()
                    .map(|argument| runtime_type(source, argument))
                    .collect::<Result<_, _>>()?,
            )
        }
        SemanticType::Type | SemanticType::Prop | SemanticType::Parameter { name: _ } => {
            return Err(format!(
                "`{}` has no runtime encoding",
                source.type_text(ty)
            ));
        }
    })
}

// --- Rendering the source -----------------------------------------------------

/// The parameter and result types of a closed function type.
fn function_type(ty: &SemanticType) -> Option<(&[SemanticType], &SemanticType)> {
    match ty {
        SemanticType::Function { parameters, result } => Some((parameters.as_slice(), result)),
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::Option { value: _ }
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::List { element: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => None,
    }
}

impl Gen<'_> {
    /// The locals a branch binds, with their closed and generic types.
    fn branch_locals(
        &self,
        branch: &SemanticBranch,
        scrutinee: &SemanticTerm,
        ctx: &Ctx,
    ) -> Result<Vec<(Local, SemanticType)>, String> {
        let closed = self.source.infer(scrutinee, &ctx.sources(), &ctx.site)?;
        let generic = self
            .source
            .infer(scrutinee, &ctx.generics(), &ctx.generic)?;
        let (_, closed_fields) = self.source.branch(branch, &closed, &ctx.site)?;
        let (_, generic_fields) = self.source.branch(branch, &generic, &ctx.generic)?;
        Ok(branch
            .binders
            .iter()
            .zip(closed_fields.into_iter().zip(generic_fields))
            .map(|(name, (ty, generic))| {
                (
                    Local {
                        name: name.clone(),
                        ty,
                    },
                    generic,
                )
            })
            .collect())
    }

    /// A binder's local with its closed and generic types.
    fn let_local(&self, binder: &SemanticParameter, ctx: &Ctx) -> (Local, SemanticType) {
        (
            Local {
                name: binder.name.clone(),
                ty: self.source.close(&binder.r#type, &ctx.site),
            },
            self.source.close(&binder.r#type, &ctx.generic),
        )
    }
}

impl Gen<'_> {}

/// The runtime namespace and name a primitive is rendered with.
fn primitive_name(operation: SemanticPrimitive) -> Result<(&'static str, &'static str), String> {
    Ok(match operation {
        SemanticPrimitive::Subtract => ("LexLeanRuntime", "subtract"),
        SemanticPrimitive::Multiply => ("LexLeanRuntime", "multiply"),
        SemanticPrimitive::Quotient => ("LexLeanRuntime", "quotient"),
        SemanticPrimitive::Remainder => ("LexLeanRuntime", "remainder"),
        SemanticPrimitive::Negate => ("LexLeanRuntime", "negate"),
        SemanticPrimitive::CheckedConvert => ("LexLeanRuntime", "checkedConvert"),
        SemanticPrimitive::CheckedAdd => ("LexLeanRuntime", "checkedAdd"),
        SemanticPrimitive::CheckedSubtract => ("LexLeanRuntime", "checkedSubtract"),
        SemanticPrimitive::CheckedMultiply => ("LexLeanRuntime", "checkedMultiply"),
        SemanticPrimitive::CheckedNegate => ("LexLeanRuntime", "checkedNegate"),
        SemanticPrimitive::CheckedQuotient => ("LexLeanRuntime", "checkedQuotient"),
        SemanticPrimitive::BitAnd => ("LexLeanRuntime", "bitAnd"),
        SemanticPrimitive::BitOr => ("LexLeanRuntime", "bitOr"),
        SemanticPrimitive::BitXor => ("LexLeanRuntime", "bitXor"),
        SemanticPrimitive::BitNot => ("LexLeanRuntime", "bitNot"),
        SemanticPrimitive::ShiftLeft => ("LexLeanRuntime", "shiftLeft"),
        SemanticPrimitive::ShiftRight => ("LexLeanRuntime", "shiftRight"),
        SemanticPrimitive::Append => ("LexLeanRuntime", "append"),
        SemanticPrimitive::Length => ("LexLeanRuntime", "length"),
        SemanticPrimitive::Index => ("LexLeanRuntime", "index"),
        SemanticPrimitive::Slice => ("LexLeanRuntime", "slice"),
        SemanticPrimitive::Utf8Encode => ("LexLeanRuntime", "utf8Encode"),
        SemanticPrimitive::Utf8Decode => ("LexLeanRuntime", "utf8Decode"),
        SemanticPrimitive::CompareBytes => ("LexLeanRuntime", "compareBytes"),
        SemanticPrimitive::Equal => ("LexLeanRuntime", "equal"),
        SemanticPrimitive::SplitExact => ("LexLeanRuntime", "splitExact"),
        SemanticPrimitive::Join => ("LexLeanRuntime", "join"),
        SemanticPrimitive::ParseDecimal => ("LexLeanRuntime", "parseDecimal"),
        SemanticPrimitive::FormatDecimal => ("LexLeanRuntime", "formatDecimal"),
        SemanticPrimitive::MapInsert => ("LexLeanCollections", "mapInsert"),
        SemanticPrimitive::MapRemove => ("LexLeanCollections", "mapRemove"),
        SemanticPrimitive::MapLookup => ("LexLeanCollections", "mapLookup"),
        SemanticPrimitive::MapContains => ("LexLeanCollections", "mapContains"),
        SemanticPrimitive::MapSize => ("LexLeanCollections", "mapSize"),
        SemanticPrimitive::MapKeys => ("LexLeanCollections", "mapKeys"),
        SemanticPrimitive::MapValues => ("LexLeanCollections", "mapValues"),
        SemanticPrimitive::MapEntries => ("LexLeanCollections", "mapEntries"),
        SemanticPrimitive::MapFold => ("LexLeanCollections", "mapFold"),
        SemanticPrimitive::SetInsert => ("LexLeanCollections", "setInsert"),
        SemanticPrimitive::SetRemove => ("LexLeanCollections", "setRemove"),
        SemanticPrimitive::SetContains => ("LexLeanCollections", "setContains"),
        SemanticPrimitive::SetSize => ("LexLeanCollections", "setSize"),
        SemanticPrimitive::SetElements => ("LexLeanCollections", "setElements"),
        SemanticPrimitive::SetUnion => ("LexLeanCollections", "setUnion"),
        SemanticPrimitive::SetIntersection => ("LexLeanCollections", "setIntersection"),
        SemanticPrimitive::SetDifference => ("LexLeanCollections", "setDifference"),
        SemanticPrimitive::SetFold => ("LexLeanCollections", "setFold"),
        SemanticPrimitive::ListFold => ("LexLeanCollections", "listFold"),
        SemanticPrimitive::Iterate => ("LexLeanCollections", "iterate"),
        SemanticPrimitive::IterateUntil => ("LexLeanCollections", "iterateUntil"),
        SemanticPrimitive::GraphSuccessors => ("LexLeanCollections", "graphSuccessors"),
        SemanticPrimitive::GraphReachable => ("LexLeanCollections", "graphReachable"),
        SemanticPrimitive::GraphTopological => ("LexLeanCollections", "graphTopological"),
        SemanticPrimitive::LessThan => ("LexLeanCollections", "lessThan"),
        // Certificates read elaborated declarations, in which a widening
        // conversion is the ordinary term it means (§17.12).
        SemanticPrimitive::ToInt => {
            return Err(format!(
                "{operation:?} is certified only through its elaboration"
            ));
        }
        SemanticPrimitive::ToNat => {
            return Err(format!(
                "{operation:?} is certified only through its elaboration"
            ));
        }
    })
}

/// The fixed-width suffix of a library lemma.
fn width_suffix(ty: &SemanticType) -> Result<&'static str, String> {
    fixed_kind(ty)
        .map(IntKind::name)
        .ok_or_else(|| "a fixed-width lemma at a non-fixed type".to_owned())
}

/// The integer suffix of a library lemma: `int` or a fixed width.
fn integer_suffix(ty: &SemanticType) -> Result<&'static str, String> {
    match ty {
        SemanticType::Int => Ok("int"),
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::Option { value: _ }
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::List { element: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => width_suffix(ty),
    }
}

/// The element type of a closed list, set, or map's entry list.
fn element_of(ty: &SemanticType) -> Result<SemanticType, String> {
    match ty {
        SemanticType::List { element } | SemanticType::Set { element } => {
            Ok(element.as_ref().clone())
        }
        SemanticType::Map { key, value } => Ok(SemanticType::Product {
            left: key.clone(),
            right: value.clone(),
        }),
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::Option { value: _ }
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => Err("an element of a non-collection".to_owned()),
    }
}

/// The value type of a closed option.
fn option_value(ty: &SemanticType) -> Result<SemanticType, String> {
    match ty {
        SemanticType::Option { value } => Ok(value.as_ref().clone()),
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::List { element: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => Err("a non-option primitive result".to_owned()),
    }
}

/// Whether a closed type is `Nat`, `Int`, or another type, for the
/// primitives defined at both.
enum Numeric {
    Nat,
    Int,
}

fn numeric(ty: &SemanticType) -> Result<Numeric, String> {
    match ty {
        SemanticType::Nat => Ok(Numeric::Nat),
        SemanticType::Int => Ok(Numeric::Int),
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::Option { value: _ }
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::List { element: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => {
            Err("an arithmetic primitive at a non-integer".to_owned())
        }
    }
}

/// Lists, byte arrays, and strings, for the sequence primitives.
enum Sequence {
    List(SemanticType),
    Bytes,
    String,
}

fn sequence(ty: &SemanticType) -> Result<Sequence, String> {
    match ty {
        SemanticType::List { element } => Ok(Sequence::List(element.as_ref().clone())),
        SemanticType::Bytes => Ok(Sequence::Bytes),
        SemanticType::String => Ok(Sequence::String),
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::Ordering
        | SemanticType::Option { value: _ }
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => {
            Err("a sequence primitive at a non-sequence".to_owned())
        }
    }
}

/// The suffix of an `equal` lemma.
fn equal_suffix(ty: &SemanticType) -> Result<&'static str, String> {
    match ty {
        SemanticType::Nat => Ok("nat"),
        SemanticType::Bool => Ok("bool"),
        SemanticType::String => Ok("string"),
        SemanticType::Bytes => Ok("bytes"),
        SemanticType::Ordering => Ok("ordering"),
        SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => width_suffix(ty),
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Option { value: _ }
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::List { element: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Set { element: _ } => Err("an equality at a non-scalar type".to_owned()),
    }
}

impl Gen<'_> {}

impl Gen<'_> {
    /// The function index of the next lambda of `function`, in the order
    /// lowering met it.
    fn next_lambda(&mut self, function: u64) -> Result<u64, String> {
        let ordinal = self.found.entry(function).or_insert(0);
        let current = *ordinal;
        *ordinal += 1;
        self.lambdas
            .get(&(function, current))
            .copied()
            .ok_or_else(|| format!("lambda {current} of function {function} is not in the layout"))
    }

    fn instance_index(
        &self,
        module: &str,
        name: &str,
        arguments: &[SemanticType],
    ) -> Result<u64, String> {
        let mut key = self.source.lean_name(
            module,
            &MemberRef {
                module: Some(module.to_owned()),
                name: name.to_owned(),
            },
        );
        for argument in arguments {
            key.push_str(" (");
            key.push_str(&self.source.type_text(argument));
            key.push(')');
        }
        self.instances
            .get(&key)
            .copied()
            .ok_or_else(|| format!("`{key}` is not in the layout"))
    }
}

impl Gen<'_> {
    /// Refuse a part of a certificate of `len` bytes when it is larger than
    /// the certificate may be.
    fn within(&self, len: usize, what: &str) -> Result<(), String> {
        if len as u64 > self.limit {
            return Err(format!(
                "{}a part of {what} is {len} bytes, beyond the {} bytes of max_file_bytes",
                super::lower::LIMIT,
                self.limit
            ));
        }
        Ok(())
    }
}

impl Gen<'_> {
    /// A scope binding `locals` in order, function-typed ones with companions.
    fn scope(locals: &[(Local, SemanticType)]) -> Vec<CLocal> {
        locals
            .iter()
            .map(|(local, generic)| CLocal {
                local: local.clone(),
                generic: generic.clone(),
                has_companion: function_type(&local.ty).is_some(),
            })
            .collect()
    }
}

/// Strongly connected components of `graph` in dependency-first order.
fn components(graph: &BTreeMap<u64, BTreeSet<u64>>) -> Vec<Vec<u64>> {
    struct State<'g> {
        graph: &'g BTreeMap<u64, BTreeSet<u64>>,
        index: BTreeMap<u64, usize>,
        low: BTreeMap<u64, usize>,
        stack: Vec<u64>,
        on_stack: BTreeSet<u64>,
        next: usize,
        out: Vec<Vec<u64>>,
    }
    fn visit(state: &mut State<'_>, node: u64) {
        state.index.insert(node, state.next);
        state.low.insert(node, state.next);
        state.next += 1;
        state.stack.push(node);
        state.on_stack.insert(node);
        let successors: Vec<u64> = state
            .graph
            .get(&node)
            .map(|edges| edges.iter().copied().collect())
            .unwrap_or_default();
        for successor in successors {
            if !state.graph.contains_key(&successor) {
                continue;
            }
            match state.index.get(&successor).copied() {
                None => {
                    visit(state, successor);
                    let low = state.low[&node].min(state.low[&successor]);
                    state.low.insert(node, low);
                }
                Some(index) => {
                    if state.on_stack.contains(&successor) {
                        let low = state.low[&node].min(index);
                        state.low.insert(node, low);
                    }
                }
            }
        }
        if state.low[&node] == state.index[&node] {
            let mut component = Vec::new();
            loop {
                let member = state.stack.pop().expect("the node is on the stack");
                state.on_stack.remove(&member);
                component.push(member);
                if member == node {
                    break;
                }
            }
            component.sort_unstable();
            state.out.push(component);
        }
    }
    let mut state = State {
        graph,
        index: BTreeMap::new(),
        low: BTreeMap::new(),
        stack: Vec::new(),
        on_stack: BTreeSet::new(),
        next: 0,
        out: Vec::new(),
    };
    for node in graph.keys() {
        if !state.index.contains_key(node) {
            visit(&mut state, *node);
        }
    }
    state.out
}

impl<'a> Gen<'a> {
    /// The hypothesis numbering of a well-founded definition body.
    fn hypotheses(
        name: &str,
        group: &BTreeSet<String>,
        type_parameters: &[String],
        parameters: &[SemanticParameter],
        body: &SemanticTerm,
    ) -> Result<Hypotheses, String> {
        let plan =
            crate::ir::semantic::well_founded_plan(name, group, type_parameters, parameters, body)?;
        Ok(plan
            .addresses
            .iter()
            .enumerate()
            .map(|(index, address)| (*address, index))
            .collect())
    }

    /// The well-founded group of a definition: itself, or every member of
    /// its mutual label.
    fn group(&self, module: &str, name: &str, mutual: Option<&String>) -> BTreeSet<String> {
        let mut group = BTreeSet::from([name.to_owned()]);
        let label = match mutual {
            Some(label) => label,
            None => return group,
        };
        for declaration in self
            .source
            .modules
            .get(module)
            .map(|linked| linked.semantic.lowered_declarations())
            .unwrap_or_default()
        {
            match declaration {
                SemanticDeclaration::Definition {
                    name,
                    type_parameters: _,
                    parameters: _,
                    result: _,
                    recursive_argument: _,
                    body: _,
                    axioms: _,
                    executable: _,
                    mutual: Some(other),
                    termination: _,
                    production: _,
                } => {
                    if other == label {
                        group.insert(name.clone());
                    }
                }
                SemanticDeclaration::Definition {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    result: _,
                    recursive_argument: _,
                    body: _,
                    axioms: _,
                    executable: _,
                    mutual: None,
                    termination: _,
                    production: _,
                }
                | SemanticDeclaration::Structure {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    fields: _,
                }
                | SemanticDeclaration::Class {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    fields: _,
                }
                | SemanticDeclaration::Instance {
                    name: _,
                    class: _,
                    arguments: _,
                    priority: _,
                    fields: _,
                }
                | SemanticDeclaration::Inductive {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    constructors: _,
                    mutual: _,
                }
                | SemanticDeclaration::Theorem {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    statement: _,
                    proof: _,
                    axioms: _,
                }
                | SemanticDeclaration::Artifact {
                    name: _,
                    role: _,
                    sha256: _,
                    length: _,
                    schema: _,
                    r#type: _,
                    axioms: _,
                }
                | SemanticDeclaration::Contract {
                    name: _,
                    type_parameters: _,
                    input: _,
                    output: _,
                    state: _,
                    precondition: _,
                    postcondition: _,
                    invariant: _,
                    validators: _,
                    axioms: _,
                }
                | SemanticDeclaration::Realization {
                    name: _,
                    type_parameters: _,
                    input: _,
                    output: _,
                    state: _,
                    descriptor: _,
                    executable: _,
                    axioms: _,
                }
                | SemanticDeclaration::Evidence {
                    name: _,
                    type_parameters: _,
                    contract: _,
                    realization: _,
                    claims: _,
                    axioms: _,
                }
                | SemanticDeclaration::Model {
                    name: _,
                    type_parameters: _,
                    contract: _,
                    realization: _,
                    evidence: _,
                    entry: _,
                    axioms: _,
                }
                | SemanticDeclaration::Logic {
                    name: _,
                    type_parameters: _,
                    state: _,
                    relation: _,
                    invariant: _,
                    ranking: _,
                    axioms: _,
                }
                | SemanticDeclaration::InferenceRule {
                    name: _,
                    type_parameters: _,
                    logic: _,
                    binding: _,
                    guard: _,
                    conclusion: _,
                    soundness: _,
                    progress: _,
                    executable: _,
                    axioms: _,
                }
                | SemanticDeclaration::Verifier {
                    name: _,
                    type_parameters: _,
                    subject: _,
                    candidate: _,
                    specification: _,
                    check: _,
                    sound: _,
                    complete: _,
                    axioms: _,
                }
                | SemanticDeclaration::Reasoner {
                    name: _,
                    type_parameters: _,
                    logic: _,
                    observation: _,
                    observe: _,
                    rules: _,
                    strategy: _,
                    answer: _,
                    verifier: _,
                    claims: _,
                    executable: _,
                    axioms: _,
                }
                | SemanticDeclaration::Tokenizer {
                    name: _,
                    type_parameters: _,
                    scheme: _,
                    vocabulary: _,
                    axioms: _,
                }
                | SemanticDeclaration::ContextWindow {
                    name: _,
                    capacity: _,
                    overflow: _,
                }
                | SemanticDeclaration::TextGenerator {
                    name: _,
                    tokenizer: _,
                    window: _,
                    scorer: _,
                    max_tokens: _,
                    stop: _,
                    policy: _,
                }
                | SemanticDeclaration::TextClassifier {
                    name: _,
                    tokenizer: _,
                    model: _,
                } => {}
            }
        }
        group
    }

    /// Record every function the certificate relates and what each one's
    /// relation depends on.
    #[allow(clippy::too_many_lines)]
    fn collect(&mut self) -> Result<BTreeMap<u64, BTreeSet<u64>>, String> {
        let mut graph: BTreeMap<u64, BTreeSet<u64>> = BTreeMap::new();
        let origins: Vec<(u64, Origin)> = self
            .layout
            .functions
            .iter()
            .enumerate()
            .map(|(index, origin)| (index as u64, origin.clone()))
            .collect();
        let mut counters: BTreeMap<u64, u64> = BTreeMap::new();
        for (index, origin) in origins {
            match origin {
                Origin::Definition {
                    module,
                    name,
                    type_arguments,
                    instance: _,
                } => {
                    let declaration = self.source.declaration(&module, &name);
                    let (
                        type_parameters,
                        parameters,
                        body,
                        recursive_argument,
                        mutual,
                        termination,
                    ) = match declaration {
                        Some(SemanticDeclaration::Definition {
                            name: _,
                            type_parameters,
                            parameters,
                            result: _,
                            recursive_argument,
                            body,
                            axioms: _,
                            executable: _,
                            mutual,
                            termination,
                            production: _,
                        }) => (
                            type_parameters,
                            parameters,
                            body,
                            recursive_argument,
                            mutual,
                            termination,
                        ),
                        Some(
                            SemanticDeclaration::Structure {
                                name: _,
                                type_parameters: _,
                                parameters: _,
                                fields: _,
                            }
                            | SemanticDeclaration::Class {
                                name: _,
                                type_parameters: _,
                                parameters: _,
                                fields: _,
                            }
                            | SemanticDeclaration::Instance {
                                name: _,
                                class: _,
                                arguments: _,
                                priority: _,
                                fields: _,
                            }
                            | SemanticDeclaration::Inductive {
                                name: _,
                                type_parameters: _,
                                parameters: _,
                                constructors: _,
                                mutual: _,
                            }
                            | SemanticDeclaration::Theorem {
                                name: _,
                                type_parameters: _,
                                parameters: _,
                                statement: _,
                                proof: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Artifact {
                                name: _,
                                role: _,
                                sha256: _,
                                length: _,
                                schema: _,
                                r#type: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Contract {
                                name: _,
                                type_parameters: _,
                                input: _,
                                output: _,
                                state: _,
                                precondition: _,
                                postcondition: _,
                                invariant: _,
                                validators: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Realization {
                                name: _,
                                type_parameters: _,
                                input: _,
                                output: _,
                                state: _,
                                descriptor: _,
                                executable: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Evidence {
                                name: _,
                                type_parameters: _,
                                contract: _,
                                realization: _,
                                claims: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Model {
                                name: _,
                                type_parameters: _,
                                contract: _,
                                realization: _,
                                evidence: _,
                                entry: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Logic {
                                name: _,
                                type_parameters: _,
                                state: _,
                                relation: _,
                                invariant: _,
                                ranking: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::InferenceRule {
                                name: _,
                                type_parameters: _,
                                logic: _,
                                binding: _,
                                guard: _,
                                conclusion: _,
                                soundness: _,
                                progress: _,
                                executable: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Verifier {
                                name: _,
                                type_parameters: _,
                                subject: _,
                                candidate: _,
                                specification: _,
                                check: _,
                                sound: _,
                                complete: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Reasoner {
                                name: _,
                                type_parameters: _,
                                logic: _,
                                observation: _,
                                observe: _,
                                rules: _,
                                strategy: _,
                                answer: _,
                                verifier: _,
                                claims: _,
                                executable: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Tokenizer {
                                name: _,
                                type_parameters: _,
                                scheme: _,
                                vocabulary: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::ContextWindow {
                                name: _,
                                capacity: _,
                                overflow: _,
                            }
                            | SemanticDeclaration::TextGenerator {
                                name: _,
                                tokenizer: _,
                                window: _,
                                scorer: _,
                                max_tokens: _,
                                stop: _,
                                policy: _,
                            }
                            | SemanticDeclaration::TextClassifier {
                                name: _,
                                tokenizer: _,
                                model: _,
                            },
                        )
                        | None => return Err(format!("`{module}.{name}` is not a definition")),
                    };
                    let site = Site {
                        module: module.clone(),
                        substitution: Source::instantiation(type_parameters, &type_arguments)?,
                    };
                    let (closed, result) =
                        self.source.signature(&module, &name, &type_arguments)?;
                    let identity: Vec<SemanticType> = type_parameters
                        .iter()
                        .map(|parameter| SemanticType::Parameter {
                            name: parameter.clone(),
                        })
                        .collect();
                    let (generic, _) = self.source.signature(&module, &name, &identity)?;
                    let locals: Vec<(Local, SemanticType)> = closed
                        .into_iter()
                        .zip(generic)
                        .map(|(closed, generic)| {
                            (
                                Local {
                                    name: closed.name,
                                    ty: closed.r#type,
                                },
                                generic.r#type,
                            )
                        })
                        .collect();
                    let instance_parameters: Vec<(String, SemanticType)> = type_parameters
                        .iter()
                        .cloned()
                        .zip(type_arguments.iter().cloned())
                        .collect();
                    let (recursion, hypotheses) = match termination {
                        Some(termination) => {
                            let group = self.group(&module, &name, mutual.as_ref());
                            let hypotheses =
                                Self::hypotheses(&name, &group, type_parameters, parameters, body)?;
                            (
                                Recursion::WellFounded {
                                    termination: termination.clone(),
                                    group,
                                },
                                hypotheses,
                            )
                        }
                        None => match recursive_argument {
                            Some(argument) => (
                                Recursion::Structural {
                                    argument: argument.clone(),
                                },
                                Hypotheses::new(),
                            ),
                            None => (Recursion::None, Hypotheses::new()),
                        },
                    };
                    let ctx = Ctx::new(
                        site.clone(),
                        instance_parameters.clone(),
                        Self::scope(&locals),
                        hypotheses,
                        index,
                    );
                    let mut dependencies = BTreeSet::new();
                    self.discover(body, &ctx, &mut dependencies, &mut counters)?;
                    graph.insert(index, dependencies);
                    self.subjects.insert(
                        index,
                        Subject::Definition {
                            module,
                            name,
                            type_arguments,
                            locals,
                            result,
                            body,
                            recursion,
                            site,
                            type_parameters: instance_parameters,
                        },
                    );
                }
                Origin::Instance { module, name } => {
                    let (class, arguments, fields) = match self.source.declaration(&module, &name) {
                        Some(SemanticDeclaration::Instance {
                            name: _,
                            class,
                            arguments,
                            priority: _,
                            fields,
                        }) => (class, arguments, fields),
                        Some(
                            SemanticDeclaration::Structure {
                                name: _,
                                type_parameters: _,
                                parameters: _,
                                fields: _,
                            }
                            | SemanticDeclaration::Class {
                                name: _,
                                type_parameters: _,
                                parameters: _,
                                fields: _,
                            }
                            | SemanticDeclaration::Definition {
                                name: _,
                                type_parameters: _,
                                parameters: _,
                                result: _,
                                recursive_argument: _,
                                body: _,
                                axioms: _,
                                executable: _,
                                mutual: _,
                                termination: _,
                                production: _,
                            }
                            | SemanticDeclaration::Inductive {
                                name: _,
                                type_parameters: _,
                                parameters: _,
                                constructors: _,
                                mutual: _,
                            }
                            | SemanticDeclaration::Theorem {
                                name: _,
                                type_parameters: _,
                                parameters: _,
                                statement: _,
                                proof: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Artifact {
                                name: _,
                                role: _,
                                sha256: _,
                                length: _,
                                schema: _,
                                r#type: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Contract {
                                name: _,
                                type_parameters: _,
                                input: _,
                                output: _,
                                state: _,
                                precondition: _,
                                postcondition: _,
                                invariant: _,
                                validators: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Realization {
                                name: _,
                                type_parameters: _,
                                input: _,
                                output: _,
                                state: _,
                                descriptor: _,
                                executable: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Evidence {
                                name: _,
                                type_parameters: _,
                                contract: _,
                                realization: _,
                                claims: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Model {
                                name: _,
                                type_parameters: _,
                                contract: _,
                                realization: _,
                                evidence: _,
                                entry: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Logic {
                                name: _,
                                type_parameters: _,
                                state: _,
                                relation: _,
                                invariant: _,
                                ranking: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::InferenceRule {
                                name: _,
                                type_parameters: _,
                                logic: _,
                                binding: _,
                                guard: _,
                                conclusion: _,
                                soundness: _,
                                progress: _,
                                executable: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Verifier {
                                name: _,
                                type_parameters: _,
                                subject: _,
                                candidate: _,
                                specification: _,
                                check: _,
                                sound: _,
                                complete: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Reasoner {
                                name: _,
                                type_parameters: _,
                                logic: _,
                                observation: _,
                                observe: _,
                                rules: _,
                                strategy: _,
                                answer: _,
                                verifier: _,
                                claims: _,
                                executable: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::Tokenizer {
                                name: _,
                                type_parameters: _,
                                scheme: _,
                                vocabulary: _,
                                axioms: _,
                            }
                            | SemanticDeclaration::ContextWindow {
                                name: _,
                                capacity: _,
                                overflow: _,
                            }
                            | SemanticDeclaration::TextGenerator {
                                name: _,
                                tokenizer: _,
                                window: _,
                                scorer: _,
                                max_tokens: _,
                                stop: _,
                                policy: _,
                            }
                            | SemanticDeclaration::TextClassifier {
                                name: _,
                                tokenizer: _,
                                model: _,
                            },
                        )
                        | None => return Err(format!("`{module}.{name}` is not an instance")),
                    };
                    let site = Site {
                        module: module.clone(),
                        substitution: BTreeMap::new(),
                    };
                    let class = self.source.close(
                        &SemanticType::Named {
                            member: class.clone(),
                            arguments: arguments.clone(),
                        },
                        &site,
                    );
                    let ctx = Ctx::new(site, Vec::new(), Vec::new(), Hypotheses::new(), index);
                    let mut dependencies = BTreeSet::new();
                    for SemanticAssignment { field: _, value } in fields {
                        self.discover(value, &ctx, &mut dependencies, &mut counters)?;
                    }
                    graph.insert(index, dependencies);
                    self.subjects.insert(
                        index,
                        Subject::Instance {
                            module,
                            name,
                            class,
                            fields,
                        },
                    );
                }
                Origin::Lambda {
                    owner: _,
                    ordinal: _,
                }
                | Origin::Template {
                    template: _,
                    types: _,
                    entry: _,
                    member: _,
                }
                | Origin::Validator {
                    ty: _,
                    module: _,
                    kind: _,
                }
                | Origin::Entry { validators: _ } => {}
            }
        }
        // Lambdas were registered as their owners were walked; walk each
        // lambda body for its own dependencies and nested lambdas.
        let mut pending: Vec<u64> = self
            .subjects
            .iter()
            .filter_map(|(index, subject)| match subject {
                Subject::Lambda {
                    site: _,
                    captures: _,
                    parameters: _,
                    body: _,
                    hypotheses: _,
                    type_parameters: _,
                } => Some(*index),
                Subject::Definition {
                    module: _,
                    name: _,
                    type_arguments: _,
                    locals: _,
                    result: _,
                    body: _,
                    recursion: _,
                    site: _,
                    type_parameters: _,
                }
                | Subject::Instance {
                    module: _,
                    name: _,
                    class: _,
                    fields: _,
                } => None,
            })
            .collect();
        loop {
            let index = match pending.pop() {
                Some(index) => index,
                None => break,
            };
            if graph.contains_key(&index) {
                continue;
            }
            let ctx = self.lambda_ctx(index)?;
            let body = match self.subjects.get(&index) {
                Some(Subject::Lambda {
                    site: _,
                    captures: _,
                    parameters: _,
                    body,
                    hypotheses: _,
                    type_parameters: _,
                }) => *body,
                Some(
                    Subject::Definition {
                        module: _,
                        name: _,
                        type_arguments: _,
                        locals: _,
                        result: _,
                        body: _,
                        recursion: _,
                        site: _,
                        type_parameters: _,
                    }
                    | Subject::Instance {
                        module: _,
                        name: _,
                        class: _,
                        fields: _,
                    },
                )
                | None => return Err(format!("function {index} is not a lambda")),
            };
            let before: BTreeSet<u64> = self.subjects.keys().copied().collect();
            let mut dependencies = BTreeSet::new();
            self.discover(body, &ctx, &mut dependencies, &mut counters)?;
            graph.insert(index, dependencies);
            pending.extend(
                self.subjects
                    .keys()
                    .copied()
                    .filter(|key| !before.contains(key)),
            );
        }
        Ok(graph)
    }

    /// The scope of a lambda's own function: its captures, then its
    /// parameters.
    fn lambda_ctx(&self, index: u64) -> Result<Ctx, String> {
        match self.subjects.get(&index) {
            Some(Subject::Lambda {
                site,
                captures,
                parameters,
                body: _,
                hypotheses,
                type_parameters,
            }) => {
                let mut locals = captures.clone();
                locals.extend(Self::scope(parameters));
                Ok(Ctx::new(
                    site.clone(),
                    type_parameters.clone(),
                    locals,
                    hypotheses.clone(),
                    index,
                ))
            }
            Some(
                Subject::Definition {
                    module: _,
                    name: _,
                    type_arguments: _,
                    locals: _,
                    result: _,
                    body: _,
                    recursion: _,
                    site: _,
                    type_parameters: _,
                }
                | Subject::Instance {
                    module: _,
                    name: _,
                    class: _,
                    fields: _,
                },
            )
            | None => Err(format!("function {index} is not a lambda")),
        }
    }

    fn discover_all(
        &mut self,
        terms: &'a [SemanticTerm],
        ctx: &Ctx,
        dependencies: &mut BTreeSet<u64>,
        counters: &mut BTreeMap<u64, u64>,
    ) -> Result<(), String> {
        for term in terms {
            self.discover(term, ctx, dependencies, counters)?;
        }
        Ok(())
    }

    /// Walk a body in lowering order, recording the functions its relation
    /// depends on and registering each lambda it forms.
    #[allow(clippy::too_many_lines)]
    fn discover(
        &mut self,
        term: &'a SemanticTerm,
        ctx: &Ctx,
        dependencies: &mut BTreeSet<u64>,
        counters: &mut BTreeMap<u64, u64>,
    ) -> Result<(), String> {
        let site = ctx.site.clone();
        match term {
            SemanticTerm::Var { name: _ }
            | SemanticTerm::Nat { value: _ }
            | SemanticTerm::Integer {
                representation: _,
                value: _,
            }
            | SemanticTerm::String { value: _ }
            | SemanticTerm::Bytes { hex: _ }
            | SemanticTerm::Bool { value: _ }
            | SemanticTerm::Unit
            | SemanticTerm::Nil { element: _ } => Ok(()),
            SemanticTerm::Primitive {
                operation: _,
                arguments,
                result: _,
            }
            | SemanticTerm::Constructor {
                constructor: _,
                type_arguments: _,
                arguments,
            } => self.discover_all(arguments, ctx, dependencies, counters),
            SemanticTerm::Cons {
                head: left,
                tail: right,
            }
            | SemanticTerm::Add { left, right }
            | SemanticTerm::Beq { left, right }
            | SemanticTerm::Ble { left, right }
            | SemanticTerm::Blt { left, right }
            | SemanticTerm::And { left, right }
            | SemanticTerm::Or { left, right }
            | SemanticTerm::Pair { left, right } => {
                self.discover(left, ctx, dependencies, counters)?;
                self.discover(right, ctx, dependencies, counters)
            }
            SemanticTerm::Record {
                r#type: _,
                type_arguments: _,
                fields,
            } => {
                let ty = self.source.infer(term, &ctx.sources(), &site)?;
                let shape = self.source.document(&ty)?;
                for field in &shape.field_names {
                    let SemanticAssignment { field: _, value } = fields
                        .iter()
                        .find(|assignment| &assignment.field == field)
                        .ok_or_else(|| format!("field `{field}` is not assigned"))?;
                    self.discover(value, ctx, dependencies, counters)?;
                }
                Ok(())
            }
            SemanticTerm::InstanceValue {
                class: _,
                arguments: _,
                resolved,
            } => {
                let module = Source::module_of(resolved, &site);
                dependencies.insert(self.instance_index(&module, &resolved.name, &[])?);
                Ok(())
            }
            SemanticTerm::Project { value, field: _ }
            | SemanticTerm::Not { value }
            | SemanticTerm::First { value }
            | SemanticTerm::Second { value } => self.discover(value, ctx, dependencies, counters),
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } => {
                let module = Source::module_of(function, &site);
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.source.close(argument, &site))
                    .collect();
                dependencies.insert(self.instance_index(&module, &function.name, &closed)?);
                self.discover_all(arguments, ctx, dependencies, counters)
            }
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                let module = Source::module_of(function, &site);
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.source.close(argument, &site))
                    .collect();
                dependencies.insert(self.instance_index(&module, &function.name, &closed)?);
                Ok(())
            }
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => {
                self.discover(condition, ctx, dependencies, counters)?;
                self.discover(then_value, ctx, dependencies, counters)?;
                self.discover(else_value, ctx, dependencies, counters)
            }
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => {
                self.discover(scrutinee, ctx, dependencies, counters)?;
                for branch in branches {
                    let inner = ctx.with(self.branch_locals(branch, scrutinee, ctx)?);
                    self.discover(&branch.body, &inner, dependencies, counters)?;
                }
                Ok(())
            }
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => {
                self.discover(value, ctx, dependencies, counters)?;
                let inner = ctx.with(vec![self.let_local(binder, ctx)]);
                self.discover(body, &inner, dependencies, counters)
            }
            SemanticTerm::Lambda {
                parameters,
                captures,
                body,
            } => {
                let ordinal = counters.entry(ctx.function).or_insert(0);
                let current = *ordinal;
                *ordinal += 1;
                let index = self
                    .lambdas
                    .get(&(ctx.function, current))
                    .copied()
                    .ok_or_else(|| {
                        format!(
                            "lambda {current} of function {} is not in the layout",
                            ctx.function
                        )
                    })?;
                dependencies.insert(index);
                let mut captured = Vec::new();
                for capture in captures {
                    let local = ctx.lookup(capture)?;
                    captured.push((local.local.clone(), local.generic.clone()));
                }
                let parameters = parameters
                    .iter()
                    .map(|binder| self.let_local(binder, ctx))
                    .collect();
                self.subjects.insert(
                    index,
                    Subject::Lambda {
                        site: site.clone(),
                        captures: Self::scope(&captured),
                        parameters,
                        body,
                        hypotheses: ctx.hypotheses.clone(),
                        type_parameters: ctx.parameters.clone(),
                    },
                );
                Ok(())
            }
            SemanticTerm::Apply {
                function,
                arguments,
            } => {
                self.discover(function, ctx, dependencies, counters)?;
                self.discover_all(arguments, ctx, dependencies, counters)
            }
            SemanticTerm::MapLiteral {
                key: _,
                value: _,
                entries,
            } => {
                for SemanticMapEntry { key, value } in entries {
                    self.discover(key, ctx, dependencies, counters)?;
                    self.discover(value, ctx, dependencies, counters)?;
                }
                Ok(())
            }
            SemanticTerm::SetLiteral {
                element: _,
                elements,
            } => self.discover_all(elements, ctx, dependencies, counters),
            // Certificates read elaborated declarations, in which a checked
            // application is the ordinary term it means (§17.12).
            SemanticTerm::CheckedApply {
                model: _,
                type_arguments: _,
                arguments: _,
                checks: _,
            } => Err(
                "a checked model application is certified only through its elaboration".to_owned(),
            ),
            SemanticTerm::GraphLiteral {
                node: _,
                nodes,
                edges,
            } => {
                for source in nodes {
                    for SemanticEdge {
                        source: from,
                        target,
                    } in edges
                    {
                        if from == source {
                            self.discover(target, ctx, dependencies, counters)?;
                        }
                    }
                    self.discover(source, ctx, dependencies, counters)?;
                }
                Ok(())
            }
            SemanticTerm::Eq { left: _, right: _ }
            | SemanticTerm::Le { left: _, right: _ }
            | SemanticTerm::Lt { left: _, right: _ }
            | SemanticTerm::PropAnd { left: _, right: _ }
            | SemanticTerm::Implies {
                premise: _,
                conclusion: _,
            }
            | SemanticTerm::Iff { left: _, right: _ }
            | SemanticTerm::Forall { binder: _, body: _ } => Err(format!(
                "the formal-only construct `{}` reached the certificate",
                super::eligibility::term_key(term)
            )),
        }
    }
}

/// The closed document types a type mentions.
fn named_types(ty: &SemanticType, out: &mut Vec<SemanticType>) {
    match ty {
        SemanticType::Named {
            member: _,
            arguments,
        } => {
            out.push(ty.clone());
            for argument in arguments {
                named_types(argument, out);
            }
        }
        SemanticType::Option { value: inner }
        | SemanticType::List { element: inner }
        | SemanticType::Set { element: inner } => named_types(inner, out),
        SemanticType::Result {
            ok: left,
            error: right,
        }
        | SemanticType::Product { left, right }
        | SemanticType::Map {
            key: left,
            value: right,
        } => {
            named_types(left, out);
            named_types(right, out);
        }
        SemanticType::Function { parameters, result } => {
            for parameter in parameters {
                named_types(parameter, out);
            }
            named_types(result, out);
        }
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => {}
    }
}

impl<'a> Gen<'a> {
    /// The dependency graph of the program's document types: each ADT points
    /// at the ADTs its fields mention.
    fn adt_graph(&self) -> Result<BTreeMap<u64, BTreeSet<u64>>, String> {
        let mut graph: BTreeMap<u64, BTreeSet<u64>> = BTreeMap::new();
        for (index, adt) in self.layout.adts.iter().enumerate() {
            let shape = self.source.document(&adt.ty)?;
            let mut dependencies = BTreeSet::new();
            for fields in &shape.fields {
                for field in fields {
                    let mut named = Vec::new();
                    named_types(field, &mut named);
                    for ty in named {
                        let text = self.source.type_text(&ty);
                        dependencies.insert(
                            *self
                                .adts
                                .get(&text)
                                .ok_or_else(|| format!("`{text}` has no ADT in the layout"))?,
                        );
                    }
                }
            }
            graph.insert(index as u64, dependencies);
        }
        Ok(graph)
    }

    /// Register every container type nested inside a recursive group of
    /// document types.
    fn nest(&mut self) -> Result<(), String> {
        let graph = self.adt_graph()?;
        for component in components(&graph) {
            let members: BTreeSet<u64> = component.iter().copied().collect();
            let recursive = component.len() > 1
                || component
                    .iter()
                    .any(|index| graph.get(index).is_some_and(|edges| edges.contains(index)));
            if !recursive {
                continue;
            }
            for index in &component {
                let ty = self.layout.adts[*index as usize].ty.clone();
                let shape = self.source.document(&ty)?;
                for fields in &shape.fields {
                    for field in fields {
                        self.register(field, &members, component[0])?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Register `ty` and its nested containers when it mentions a member of
    /// the group; inner containers first, so indices follow dependencies.
    fn register(
        &mut self,
        ty: &SemanticType,
        members: &BTreeSet<u64>,
        group: u64,
    ) -> Result<(), String> {
        let mut named = Vec::new();
        named_types(ty, &mut named);
        let touches = named.iter().any(|named| {
            self.adts
                .get(&self.source.type_text(named))
                .is_some_and(|index| members.contains(index))
        });
        if !touches {
            return Ok(());
        }
        match ty {
            SemanticType::Named {
                member: _,
                arguments: _,
            } => return Ok(()),
            SemanticType::Option { value: inner }
            | SemanticType::List { element: inner }
            | SemanticType::Set { element: inner } => self.register(inner, members, group)?,
            SemanticType::Map { key, value } => self.register(
                &SemanticType::Product {
                    left: key.clone(),
                    right: value.clone(),
                },
                members,
                group,
            )?,
            SemanticType::Result {
                ok: left,
                error: right,
            }
            | SemanticType::Product { left, right } => {
                self.register(left, members, group)?;
                self.register(right, members, group)?;
            }
            SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64
            | SemanticType::String
            | SemanticType::Bytes
            | SemanticType::Ordering
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {
                return Err(format!(
                    "`{}` cannot hold a document type in a field",
                    self.source.type_text(ty)
                ));
            }
        }
        let text = self.source.type_text(ty);
        let index = self.nested.len() as u64;
        self.nested.entry(text).or_insert(Nested {
            index,
            ty: ty.clone(),
            group,
        });
        Ok(())
    }
}

impl<'a> Gen<'a> {}

/// The Lean type of an integer literal's representation.
const fn integer_lean(representation: SemanticInteger) -> &'static str {
    match representation {
        SemanticInteger::Int => "Int",
        SemanticInteger::Int8 => "Int8",
        SemanticInteger::Int16 => "Int16",
        SemanticInteger::Int32 => "Int32",
        SemanticInteger::Int64 => "Int64",
        SemanticInteger::UInt8 => "UInt8",
        SemanticInteger::UInt16 => "UInt16",
        SemanticInteger::UInt32 => "UInt32",
        SemanticInteger::UInt64 => "UInt64",
    }
}

/// The key type, value type, and node type of a template instance.
fn instance_type(instance: &[SemanticType], position: usize) -> Result<SemanticType, String> {
    instance
        .get(position)
        .cloned()
        .ok_or_else(|| format!("a template instance lacks type argument {position}"))
}

impl Gen<'_> {
    /// The target type of a closed source type, with document types read
    /// from the layout.
    fn target_ty(&self, ty: &SemanticType) -> Result<Ty, String> {
        Ok(match ty {
            SemanticType::Nat => Ty::Nat,
            SemanticType::Bool => Ty::Bool,
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => Ty::Pair {
                left: Box::new(Ty::Bool),
                right: Box::new(Ty::Bool),
            },
            SemanticType::Unit => Ty::Unit,
            SemanticType::Int => Ty::Int,
            SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64 => Ty::Fixed {
                width: fixed_kind(ty).ok_or("a fixed width without a kind")?,
            },
            SemanticType::String => Ty::String,
            SemanticType::Bytes => Ty::Bytes,
            SemanticType::Ordering => Ty::Ordering,
            SemanticType::Option { value } => Ty::Option {
                value: Box::new(self.target_ty(value)?),
            },
            SemanticType::Result { ok, error } => Ty::Result {
                ok: Box::new(self.target_ty(ok)?),
                error: Box::new(self.target_ty(error)?),
            },
            SemanticType::List { element } | SemanticType::Set { element } => Ty::List {
                element: Box::new(self.target_ty(element)?),
            },
            SemanticType::Product { left, right } => Ty::Pair {
                left: Box::new(self.target_ty(left)?),
                right: Box::new(self.target_ty(right)?),
            },
            SemanticType::Map { key, value } => Ty::List {
                element: Box::new(Ty::Pair {
                    left: Box::new(self.target_ty(key)?),
                    right: Box::new(self.target_ty(value)?),
                }),
            },
            SemanticType::Function { parameters, result } => Ty::Fn {
                parameters: parameters
                    .iter()
                    .map(|parameter| self.target_ty(parameter))
                    .collect::<Result<Vec<_>, _>>()?,
                result: Box::new(self.target_ty(result)?),
            },
            SemanticType::Named {
                member: _,
                arguments: _,
            } => {
                let text = self.source.type_text(ty);
                Ty::Adt {
                    index: *self
                        .adts
                        .get(&text)
                        .ok_or_else(|| format!("`{text}` has no ADT in the layout"))?,
                }
            }
            SemanticType::Type | SemanticType::Prop | SemanticType::Parameter { name: _ } => {
                return Err(format!(
                    "`{}` has no runtime realization",
                    self.source.type_text(ty)
                ));
            }
        })
    }
}

impl Gen<'_> {}

// --- The boundary (SPEC.md §17.17) ------------------------------------------

/// The components of a map, product, or result type.
fn parts(ty: &SemanticType) -> Result<(SemanticType, SemanticType), String> {
    match ty {
        SemanticType::Map {
            key: left,
            value: right,
        }
        | SemanticType::Product { left, right }
        | SemanticType::Result {
            ok: left,
            error: right,
        } => Ok((left.as_ref().clone(), right.as_ref().clone())),
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::Option { value: _ }
        | SemanticType::List { element: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => Err(format!("{ty:?} has no two components")),
    }
}

/// The kind of validator a type calls for, by name: a set's, a map's, and so
/// on, and `scalar` for a type with no component that could carry the
/// invariant.
fn shape_tag(ty: &SemanticType) -> &'static str {
    match ty {
        SemanticType::Set { element: _ } => "set",
        SemanticType::Map { key: _, value: _ } => "map",
        SemanticType::List { element: _ } => "list",
        SemanticType::Option { value: _ } => "option",
        SemanticType::Product { left: _, right: _ } => "pair",
        SemanticType::Result { ok: _, error: _ } => "result",
        SemanticType::Named {
            member: _,
            arguments: _,
        } => "document",
        SemanticType::Type
        | SemanticType::Prop
        | SemanticType::Parameter { name: _ }
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => "scalar",
    }
}

/// Whether a validator checks nothing.
fn is_trivial(kind: &Validation) -> bool {
    match kind {
        Validation::Trivial => true,
        Validation::Set
        | Validation::Map { value: _ }
        | Validation::List { element: _ }
        | Validation::Option { value: _ }
        | Validation::Pair { left: _, right: _ }
        | Validation::Result { ok: _, error: _ }
        | Validation::Document { fields: _ } => false,
    }
}

/// The name of what a validator checks, as [`shape_tag`] names a type.
fn kind_tag(kind: &Validation) -> &'static str {
    match kind {
        Validation::Trivial => "scalar",
        Validation::Set => "set",
        Validation::Map { value: _ } => "map",
        Validation::List { element: _ } => "list",
        Validation::Option { value: _ } => "option",
        Validation::Pair { left: _, right: _ } => "pair",
        Validation::Result { ok: _, error: _ } => "result",
        Validation::Document { fields: _ } => "document",
    }
}

/// The components of a container type that a validator checks, in the order
/// [`checked`] lists the validators.
fn component_types(ty: &SemanticType) -> Vec<SemanticType> {
    match ty {
        SemanticType::Map { key: _, value } => vec![value.as_ref().clone()],
        SemanticType::List { element: inner } | SemanticType::Option { value: inner } => {
            vec![inner.as_ref().clone()]
        }
        SemanticType::Product { left, right }
        | SemanticType::Result {
            ok: left,
            error: right,
        } => vec![left.as_ref().clone(), right.as_ref().clone()],
        SemanticType::Set { element: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Type
        | SemanticType::Prop
        | SemanticType::Parameter { name: _ }
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => Vec::new(),
    }
}

/// The validators a validator calls.
fn checked(kind: &Validation) -> Vec<u64> {
    match kind {
        Validation::Trivial | Validation::Set => Vec::new(),
        Validation::Map { value }
        | Validation::List { element: value }
        | Validation::Option { value } => vec![*value],
        Validation::Pair { left, right } => vec![*left, *right],
        Validation::Result { ok, error } => vec![*ok, *error],
        Validation::Document { fields } => fields.iter().flatten().flatten().copied().collect(),
    }
}

impl Gen<'_> {
    /// The root's boundary validators by function index.
    fn checks(&self) -> BTreeMap<u64, (&SemanticType, &str, &Validation)> {
        let mut out = BTreeMap::new();
        for (index, origin) in self.layout.functions.iter().enumerate() {
            match origin {
                Origin::Validator { ty, module, kind } => {
                    out.insert(index as u64, (ty, module.as_str(), kind));
                }
                Origin::Definition {
                    module: _,
                    name: _,
                    type_arguments: _,
                    instance: _,
                }
                | Origin::Instance { module: _, name: _ }
                | Origin::Lambda {
                    owner: _,
                    ordinal: _,
                }
                | Origin::Template {
                    template: _,
                    types: _,
                    entry: _,
                    member: _,
                }
                | Origin::Entry { validators: _ } => {}
            }
        }
        out
    }

    /// The root's entry, if it has one: its index and the validator of
    /// each parameter.
    fn entry_of(&self) -> Option<(u64, &[Option<u64>])> {
        let last = self.layout.functions.len().checked_sub(1)?;
        match &self.layout.functions[last] {
            Origin::Entry { validators } => Some((last as u64, validators.as_slice())),
            Origin::Definition {
                module: _,
                name: _,
                type_arguments: _,
                instance: _,
            }
            | Origin::Instance { module: _, name: _ }
            | Origin::Lambda {
                owner: _,
                ordinal: _,
            }
            | Origin::Template {
                template: _,
                types: _,
                entry: _,
                member: _,
            }
            | Origin::Validator {
                ty: _,
                module: _,
                kind: _,
            } => None,
        }
    }

    /// Whether an ADT belongs to a recursive group, as its encoder does.
    fn recursive_adt(&self, index: u64, graph: &BTreeMap<u64, BTreeSet<u64>>) -> bool {
        self.nested.values().any(|nested| nested.group == index)
            || components(graph)
                .iter()
                .any(|component| component.len() > 1 && component.contains(&index))
            || graph
                .get(&index)
                .is_some_and(|edges| edges.contains(&index))
    }

    /// Whether a validator is stated by the recursion of a recursive group
    /// of document types: a document of the group, or a container nested in
    /// it, whose encoder is an auxiliary of the group's mutual block.
    fn grouped(
        &self,
        ty: &SemanticType,
        kind: &Validation,
        graph: &BTreeMap<u64, BTreeSet<u64>>,
    ) -> Result<bool, String> {
        let text = self.source.type_text(ty);
        Ok(match kind {
            Validation::Trivial => false,
            Validation::Document { fields: _ } => {
                let index = *self
                    .adts
                    .get(&text)
                    .ok_or_else(|| format!("`{text}` has no ADT in the layout"))?;
                self.recursive_adt(index, graph)
            }
            Validation::Set
            | Validation::Map { value: _ }
            | Validation::List { element: _ }
            | Validation::Option { value: _ }
            | Validation::Pair { left: _, right: _ }
            | Validation::Result { ok: _, error: _ } => self.nested.contains_key(&text),
        })
    }

    /// Whether a closed type can hold a map, set, or graph, whose strictly
    /// ascending order §17.12 states as an invariant: decided here from the
    /// source type alone, by this generator's own recursion, so that what
    /// the lowering decided to validate is checked against what the source
    /// says needs validating, not against itself.
    fn invariant_carrier(
        &self,
        ty: &SemanticType,
        seen: &mut BTreeSet<String>,
    ) -> Result<bool, String> {
        match ty {
            SemanticType::Map { key: _, value: _ } | SemanticType::Set { element: _ } => Ok(true),
            SemanticType::Option { value: inner } | SemanticType::List { element: inner } => {
                self.invariant_carrier(inner, seen)
            }
            SemanticType::Product { left, right }
            | SemanticType::Result {
                ok: left,
                error: right,
            } => {
                let first = self.invariant_carrier(left, seen)?;
                let second = self.invariant_carrier(right, seen)?;
                Ok(first || second)
            }
            SemanticType::Named {
                member: _,
                arguments: _,
            } => {
                if !seen.insert(self.source.type_text(ty)) {
                    return Ok(false);
                }
                let shape = self.source.document(ty)?;
                let mut found = false;
                for field in shape.fields.iter().flatten() {
                    found = self.invariant_carrier(field, seen)? || found;
                }
                Ok(found)
            }
            SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64
            | SemanticType::String
            | SemanticType::Bytes
            | SemanticType::Ordering
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => Ok(false),
        }
    }

    /// The boundary the lowering built is the boundary the source types
    /// call for: the entry validates exactly the parameters whose types carry
    /// the invariant, each by the validator of its own type, and every
    /// validator checks exactly the components of its type that carry it.
    /// A lowering that skips one is refused here, before any proof is
    /// written, so no theorem is stated about a weaker boundary.
    fn verify_boundary(&self, parameters: &[SemanticType]) -> Result<(), String> {
        let checks = self.checks();
        let carries = |ty: &SemanticType| self.invariant_carrier(ty, &mut BTreeSet::new());
        // `index` is the validator of a component of type `expected`.
        let component =
            |index: u64, expected: &SemanticType, context: &str| -> Result<(), String> {
                let (ty, _, kind) = checks
                    .get(&index)
                    .ok_or_else(|| format!("{context}: function {index} is not a validator"))?;
                if self.source.type_text(ty) != self.source.type_text(expected) {
                    return Err(format!(
                        "{context}: the validator {index} checks `{}`, not `{}`",
                        self.source.type_text(ty),
                        self.source.type_text(expected)
                    ));
                }
                let needed = carries(expected)?;
                if needed == is_trivial(kind) {
                    return Err(format!(
                        "{context}: `{}` {} the invariant but its validator {index} {}",
                        self.source.type_text(expected),
                        if needed { "carries" } else { "does not carry" },
                        if needed {
                            "checks nothing"
                        } else {
                            "checks something"
                        }
                    ));
                }
                Ok(())
            };
        let mut any = false;
        for parameter in parameters {
            any = carries(parameter)? || any;
        }
        match self.entry_of() {
            None => {
                if any {
                    return Err(
                        "a parameter carries §17.12's invariant but the root has no entry"
                            .to_owned(),
                    );
                }
            }
            Some((_, validators)) => {
                if validators.len() != parameters.len() {
                    return Err(format!(
                        "the entry validates {} parameters of {}",
                        validators.len(),
                        parameters.len()
                    ));
                }
                for (position, (parameter, validator)) in
                    parameters.iter().zip(validators).enumerate()
                {
                    let context = format!("parameter {position}");
                    match validator {
                        Some(index) => component(*index, parameter, &context)?,
                        None => {
                            if carries(parameter)? {
                                return Err(format!(
                                    "{context}: `{}` carries §17.12's invariant but the entry does not validate it",
                                    self.source.type_text(parameter)
                                ));
                            }
                        }
                    }
                }
            }
        }
        for (index, (ty, _, kind)) in &checks {
            let context = format!("the validator {index} of `{}`", self.source.type_text(ty));
            // A validator of a container checks that kind of container.
            let (shape, tag) = (shape_tag(ty), kind_tag(kind));
            match kind {
                Validation::Trivial => {
                    if carries(ty)? {
                        return Err(format!(
                            "{context} is trivial but its type carries the invariant"
                        ));
                    }
                    continue;
                }
                Validation::Document { fields: _ }
                | Validation::Set
                | Validation::Map { value: _ }
                | Validation::List { element: _ }
                | Validation::Option { value: _ }
                | Validation::Pair { left: _, right: _ }
                | Validation::Result { ok: _, error: _ } => {
                    if shape != tag {
                        return Err(format!("{context} is a {tag} validator of a {shape}"));
                    }
                }
            }
            match kind {
                Validation::Trivial => {}
                Validation::Document { fields } => {
                    let shape = self.source.document(ty)?;
                    if shape.fields.len() != fields.len() {
                        return Err(format!("{context} checks another number of constructors"));
                    }
                    for (declared, checked) in shape.fields.iter().zip(fields) {
                        if declared.len() != checked.len() {
                            return Err(format!("{context} checks another number of fields"));
                        }
                        for (field, validator) in declared.iter().zip(checked) {
                            match validator {
                                Some(index) => component(*index, field, &context)?,
                                None => {
                                    if carries(field)? {
                                        return Err(format!(
                                            "{context} does not validate a field `{}` that carries the invariant",
                                            self.source.type_text(field)
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
                Validation::Set
                | Validation::Map { value: _ }
                | Validation::List { element: _ }
                | Validation::Option { value: _ }
                | Validation::Pair { left: _, right: _ }
                | Validation::Result { ok: _, error: _ } => {
                    let expected = component_types(ty);
                    let checked = checked(kind);
                    if expected.len() != checked.len() {
                        return Err(format!("{context} checks another number of components"));
                    }
                    for (index, expected) in checked.iter().zip(&expected) {
                        component(*index, expected, &context)?;
                    }
                }
            }
        }
        Ok(())
    }
}
