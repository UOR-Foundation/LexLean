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

use std::collections::{BTreeMap, BTreeSet};

use super::eligibility::{integer_type, BUILTIN_OWNERS};
use super::lower::{fixed_kind, Layout, Lowered, Origin};
use super::source::{Constructor, Local, Site, Source};
use super::RootReport;
use crate::backend::semantic::{hypothesis, identifier, string_literal, term_uses};
use crate::calculus::library::Template;
use crate::calculus::{
    Arm, Expr, Function, IntKind, OrderingValue, Prim, Program, Shape, Ty, Value,
};
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
pub const LIBRARY_MODULES: [&str; 6] = [
    "LexLeanPreservation.Core",
    "LexLeanPreservation.Values",
    "LexLeanPreservation.Primitives",
    "LexLeanPreservation.Fixed",
    "LexLeanPreservation.Keys",
    "LexLeanPreservation.Templates",
];

const SYNTAX: &str = "LexLeanTarget.TargetSyntax";
const SEMANTICS: &str = "LexLeanTarget.TargetSemantics";
const NAT_BOUND: &str = "18446744073709551616";

/// One generated certificate module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Certificate {
    /// The Lean module name.
    pub module: String,
    /// The module text.
    pub text: String,
    /// The fully qualified name of the root theorem.
    pub theorem: String,
    /// The generated user modules the certificate imports.
    pub imports: Vec<String>,
}

fn internal(reason: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(code!("LLI9001"), format!("phase preservation: {reason}"))
}

fn lib(name: &str) -> String {
    format!("{LIBRARY}.{name}")
}

fn value_ctor(name: &str) -> String {
    format!("{SYNTAX}.Value.{name}")
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
    companion: Option<Companion>,
}

#[derive(Clone)]
struct Companion {
    fits: String,
    index: String,
    captures: String,
    relation: String,
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
                companion: None,
            }));
        out
    }
}

/// A proof that a term's lowering converges to the relation of its fits
/// predicate and denotation, and that predicate.
#[derive(Clone)]
struct Proof {
    proof: String,
    fits: String,
}

/// A function-typed argument with its companions.
struct FunctionArgument {
    /// The source value.
    value: String,
    /// Its fits function.
    fits: String,
    /// Its closure's function index.
    index: String,
    /// Its closure's encoded captures.
    captures: String,
    /// Its relation, a function of its parameters.
    relation: String,
    /// The proof of the closure expression.
    proof: Proof,
}

/// How a function's relation theorem recurses.
#[derive(Clone)]
enum Recursion {
    None,
    /// Structural recursion on parameter `argument` by top-level equations;
    /// a mutual group's label.
    Structural {
        argument: String,
        mutual: Option<String>,
    },
    /// Well-founded recursion with the source's measure and evidence.
    WellFounded {
        termination: SemanticTermination,
        group: BTreeSet<String>,
    },
}

/// One function the certificate states a relation for.
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
}

/// A container type that occurs nested in a recursive group of document
/// types.
struct Nested {
    index: u64,
    ty: SemanticType,
    /// The first ADT of the group, which names its mutual block.
    group: u64,
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
) -> Result<Certificate, Diagnostic> {
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
        }
    }
    let adts = lowered
        .layout
        .adts
        .iter()
        .enumerate()
        .map(|(index, adt)| (adt.text.clone(), index as u64))
        .collect();
    let mut generator = Gen {
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
    };
    generator.nest().map_err(internal)?;
    let text = generator
        .module(root_module, root_name, report, module)
        .map_err(internal)?;
    let mut imports: Vec<String> = modules
        .values()
        .map(|linked| linked.lean_module.to_owned())
        .collect();
    imports.sort();
    Ok(Certificate {
        module: module.to_owned(),
        theorem: format!("{module}.root"),
        text,
        imports,
    })
}

// --- Rendering the target program -------------------------------------------

fn kind_ctor(kind: IntKind) -> &'static str {
    kind.name()
}

fn render_ty(ty: &Ty) -> String {
    match ty {
        Ty::Unit => ".unit".to_owned(),
        Ty::Bool => ".bool".to_owned(),
        Ty::Nat => ".nat".to_owned(),
        Ty::Int => ".int".to_owned(),
        Ty::Fixed { width } => format!("(.fixed .{})", kind_ctor(*width)),
        Ty::String => ".string".to_owned(),
        Ty::Bytes => ".bytes".to_owned(),
        Ty::Ordering => ".ordering".to_owned(),
        Ty::Option { value } => format!("(.option {})", render_ty(value)),
        Ty::Result { ok, error } => format!("(.result {} {})", render_ty(ok), render_ty(error)),
        Ty::List { element } => format!("(.list {})", render_ty(element)),
        Ty::Pair { left, right } => format!("(.pair {} {})", render_ty(left), render_ty(right)),
        Ty::Adt { index } => format!("(.adt {index})"),
        Ty::Fn { parameters, result } => format!(
            "(.fn [{}] {})",
            parameters
                .iter()
                .map(render_ty)
                .collect::<Vec<_>>()
                .join(", "),
            render_ty(result)
        ),
    }
}

fn signed(text: &str) -> String {
    if text.starts_with('-') {
        format!("({text})")
    } else {
        text.to_owned()
    }
}

fn bytes_literal(hex: &str) -> Result<String, String> {
    let mut values = Vec::new();
    let digits = hex.as_bytes();
    if digits.len() % 2 != 0 {
        return Err(format!("byte literal `{hex}` has odd length"));
    }
    for pair in digits.chunks(2) {
        let text = std::str::from_utf8(pair).map_err(|_| "a non-ASCII byte literal")?;
        values.push(
            u8::from_str_radix(text, 16)
                .map_err(|_| format!("byte literal `{hex}` is not hexadecimal"))?
                .to_string(),
        );
    }
    Ok(format!("ByteArray.mk #[{}]", values.join(", ")))
}

fn render_value(value: &Value) -> Result<String, String> {
    Ok(match value {
        Value::Unit => ".unit".to_owned(),
        Value::Bool { value } => format!("(.bool {value})"),
        Value::Nat { value } => format!("(.nat {value})"),
        Value::Int { value } => format!("(.int {})", signed(value)),
        Value::U8 { value } => format!("(.u8 {value})"),
        Value::U16 { value } => format!("(.u16 {value})"),
        Value::U32 { value } => format!("(.u32 {value})"),
        Value::U64 { value } => format!("(.u64 {value})"),
        Value::I8 { value } => format!("(.i8 {})", signed(value)),
        Value::I16 { value } => format!("(.i16 {})", signed(value)),
        Value::I32 { value } => format!("(.i32 {})", signed(value)),
        Value::I64 { value } => format!("(.i64 {})", signed(value)),
        Value::String { value } => format!("(.string {})", string_literal(value)),
        Value::Bytes { hex } => format!("(.bytes ({}))", bytes_literal(hex)?),
        Value::Ordering { value } => format!(
            "(.ordering .{})",
            match value {
                OrderingValue::Lt => "less",
                OrderingValue::Eq => "same",
                OrderingValue::Gt => "more",
            }
        ),
        Value::None => ".none".to_owned(),
        Value::Some { value } => format!("(.some {})", render_value(value)?),
        Value::Ok { value } => format!("(.ok {})", render_value(value)?),
        Value::Error { value } => format!("(.error {})", render_value(value)?),
        Value::List { items } => format!(
            "(.list [{}])",
            items
                .iter()
                .map(render_value)
                .collect::<Result<Vec<_>, _>>()?
                .join(", ")
        ),
        Value::Pair { left, right } => {
            format!("(.pair {} {})", render_value(left)?, render_value(right)?)
        }
        Value::Adt {
            constructor,
            fields,
        } => format!(
            "(.adt {constructor} [{}])",
            fields
                .iter()
                .map(render_value)
                .collect::<Result<Vec<_>, _>>()?
                .join(", ")
        ),
        Value::Closure { function, captures } => format!(
            "(.closure {function} [{}])",
            captures
                .iter()
                .map(render_value)
                .collect::<Result<Vec<_>, _>>()?
                .join(", ")
        ),
    })
}

fn render_shape(shape: Shape) -> String {
    match shape {
        Shape::None => ".none".to_owned(),
        Shape::Some => ".some".to_owned(),
        Shape::Ok => ".ok".to_owned(),
        Shape::Error => ".error".to_owned(),
        Shape::Nil => ".nil".to_owned(),
        Shape::Cons => ".cons".to_owned(),
        Shape::Zero => ".zero".to_owned(),
        Shape::Succ => ".succ".to_owned(),
        Shape::Pair => ".pair".to_owned(),
        Shape::True => ".true".to_owned(),
        Shape::False => ".false".to_owned(),
        Shape::Unit => ".unit".to_owned(),
        Shape::Lt => ".lt".to_owned(),
        Shape::Eq => ".eq".to_owned(),
        Shape::Gt => ".gt".to_owned(),
        Shape::Adt { constructor } => format!("(.adt {constructor})"),
    }
}

fn render_prim(prim: &Prim) -> String {
    match prim {
        Prim::NatAdd => ".natAdd".to_owned(),
        Prim::NatSub => ".natSub".to_owned(),
        Prim::NatMul => ".natMul".to_owned(),
        Prim::NatQuot => ".natQuot".to_owned(),
        Prim::NatRem => ".natRem".to_owned(),
        Prim::NatEq => ".natEq".to_owned(),
        Prim::NatLe => ".natLe".to_owned(),
        Prim::NatLt => ".natLt".to_owned(),
        Prim::IntAdd => ".intAdd".to_owned(),
        Prim::IntSub => ".intSub".to_owned(),
        Prim::IntMul => ".intMul".to_owned(),
        Prim::IntNeg => ".intNeg".to_owned(),
        Prim::IntQuot => ".intQuot".to_owned(),
        Prim::IntRem => ".intRem".to_owned(),
        Prim::CheckedAdd => ".checkedAdd".to_owned(),
        Prim::CheckedSub => ".checkedSub".to_owned(),
        Prim::CheckedMul => ".checkedMul".to_owned(),
        Prim::CheckedNeg => ".checkedNeg".to_owned(),
        Prim::CheckedQuot => ".checkedQuot".to_owned(),
        Prim::BitAnd => ".bitAnd".to_owned(),
        Prim::BitOr => ".bitOr".to_owned(),
        Prim::BitXor => ".bitXor".to_owned(),
        Prim::BitNot => ".bitNot".to_owned(),
        Prim::ShiftLeft => ".shiftLeft".to_owned(),
        Prim::ShiftRight => ".shiftRight".to_owned(),
        Prim::Equal => ".equal".to_owned(),
        Prim::BoolNot => ".boolNot".to_owned(),
        Prim::BoolAnd => ".boolAnd".to_owned(),
        Prim::BoolOr => ".boolOr".to_owned(),
        Prim::Append => ".append".to_owned(),
        Prim::Length => ".length".to_owned(),
        Prim::Index => ".index".to_owned(),
        Prim::Slice => ".slice".to_owned(),
        Prim::Utf8Encode => ".utf8Encode".to_owned(),
        Prim::Utf8Decode => ".utf8Decode".to_owned(),
        Prim::CompareBytes => ".compareBytes".to_owned(),
        Prim::SplitExact => ".splitExact".to_owned(),
        Prim::Join => ".join".to_owned(),
        Prim::FormatDecimal => ".formatDecimal".to_owned(),
        Prim::Compare => ".compare".to_owned(),
        Prim::Convert { target } => format!("(.convert .{})", kind_ctor(*target)),
        Prim::ParseDecimal { target } => format!("(.parseDecimal {})", render_ty(target)),
    }
}

fn render_exprs(exprs: &[Expr]) -> Result<String, String> {
    Ok(format!(
        "[{}]",
        exprs
            .iter()
            .map(render_expr)
            .collect::<Result<Vec<_>, _>>()?
            .join(", ")
    ))
}

fn numbers(names: &[u64]) -> String {
    format!(
        "[{}]",
        names
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn render_expr(expr: &Expr) -> Result<String, String> {
    Ok(match expr {
        Expr::Value { ty, value } => format!("(.value {} {})", render_ty(ty), render_value(value)?),
        Expr::Var { name } => format!("(.var {name})"),
        Expr::Let {
            name,
            ty,
            bound,
            body,
        } => format!(
            "(.«let» {name} {} {} {})",
            render_ty(ty),
            render_expr(bound)?,
            render_expr(body)?
        ),
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => format!(
            "(.cond {} {} {})",
            render_expr(condition)?,
            render_expr(then_branch)?,
            render_expr(else_branch)?
        ),
        Expr::Match {
            ty,
            scrutinee,
            arms,
        } => format!(
            "(.«match» {} {} [{}])",
            render_ty(ty),
            render_expr(scrutinee)?,
            arms.iter()
                .map(
                    |Arm {
                         shape,
                         binders,
                         body,
                     }| {
                        Ok(format!(
                            "(.arm {} {} {})",
                            render_shape(*shape),
                            numbers(binders),
                            render_expr(body)?
                        ))
                    }
                )
                .collect::<Result<Vec<_>, String>>()?
                .join(", ")
        ),
        Expr::Build {
            shape,
            ty,
            operands,
        } => format!(
            "(.build {} {} {})",
            render_shape(*shape),
            render_ty(ty),
            render_exprs(operands)?
        ),
        Expr::Call { function, operands } => {
            format!("(.call {function} {})", render_exprs(operands)?)
        }
        Expr::Closure { function, captures } => {
            format!("(.closure {function} {})", render_exprs(captures)?)
        }
        Expr::Apply { target, operands } => format!(
            "(.apply {} {})",
            render_expr(target)?,
            render_exprs(operands)?
        ),
        Expr::Prim {
            operation,
            operands,
        } => format!(
            "(.prim {} {})",
            render_prim(operation),
            render_exprs(operands)?
        ),
        Expr::First { value } => format!("(.first {})", render_expr(value)?),
        Expr::Second { value } => format!("(.second {})", render_expr(value)?),
        Expr::Field { value, index } => format!("(.field {} {index})", render_expr(value)?),
    })
}

/// The program as a Lean term of type `Program`.
fn render_program(program: &Program) -> Result<String, String> {
    let mut out = String::from("{ adts := [");
    for (index, adt) in program.adts.iter().enumerate() {
        if index > 0 {
            out.push_str(",\n    ");
        }
        out.push_str("{ constructors := [");
        out.push_str(
            &adt.constructors
                .iter()
                .map(|fields| {
                    format!(
                        "[{}]",
                        fields.iter().map(render_ty).collect::<Vec<_>>().join(", ")
                    )
                })
                .collect::<Vec<_>>()
                .join(", "),
        );
        out.push_str("] }");
    }
    out.push_str("], functions := [");
    for (
        index,
        Function {
            parameters,
            types,
            result,
            body,
        },
    ) in program.functions.iter().enumerate()
    {
        if index > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "\n    {{ parameters := {}, types := [{}], result := {},\n      body := {} }}",
            numbers(parameters),
            types.iter().map(render_ty).collect::<Vec<_>>().join(", "),
            render_ty(result),
            render_expr(body)?
        ));
    }
    out.push_str("] }");
    Ok(out)
}

// --- Rendering the source -----------------------------------------------------

/// A Lean binder for a function-typed local's companion.
fn companion_names(stem: &str) -> Companion {
    Companion {
        fits: format!("__ff_{stem}"),
        index: format!("__fi_{stem}"),
        captures: format!("__fc_{stem}"),
        relation: format!("__fh_{stem}"),
    }
}

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
        | SemanticType::Set { element: _ } => None,
    }
}

/// A conjunction of fits predicates, in evaluation order.
fn and(left: &str, right: &str) -> String {
    format!("({left} && {right})")
}

/// The proof and fits of an operand list.
fn operands(proofs: Vec<Proof>) -> Proof {
    proofs.into_iter().rev().fold(
        Proof {
            proof: lib("convL_nil"),
            fits: "true".to_owned(),
        },
        |rest, head| Proof {
            proof: format!("({} {} {})", lib("convL_cons"), head.proof, rest.proof),
            fits: and(&head.fits, &rest.fits),
        },
    )
}

/// A built value: `conv_build` over the operands with `lemma` for the
/// constructor, whose fits is `fits`.
fn built(operands_proof: Proof, lemma: &str, fits: &str) -> Proof {
    Proof {
        proof: format!("({} {} {})", lib("conv_build"), operands_proof.proof, lemma),
        fits: and(&operands_proof.fits, fits),
    }
}

/// The width predicate a primitive result contributes.
enum Width {
    /// The result is always representable.
    Exact,
    /// A natural-number result realized in 64 bits.
    Nat,
    /// A mathematical-integer result realized in 64 bits.
    Int,
    /// A library predicate applied to the operands.
    Predicate(&'static str),
}

/// A primitive's relation lemma, the encoders it takes before its operands,
/// and the width predicate its result contributes.
struct PrimRule {
    lemma: String,
    prefix: Vec<String>,
    width: Width,
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

impl Gen<'_> {
    fn lean_module(&self, module: &str) -> Result<String, String> {
        self.source.lean_module(module).map(str::to_owned)
    }

    /// A member reference read at `site`, fully qualified.
    fn member(&self, member: &MemberRef, site: &Site) -> Result<String, String> {
        if member.module.is_none() {
            if member.name == "Result.error" {
                return Ok("Except.error".to_owned());
            }
            if member.name == "Result.ok" {
                return Ok("Except.ok".to_owned());
            }
            let owner = member.name.rsplit_once('.').map(|(owner, _)| owner);
            let builtin = BUILTIN_OWNERS.iter().any(|(name, _)| Some(*name) == owner);
            if builtin {
                return Ok(identifier(&member.name));
            }
        }
        let module = Source::module_of(member, site);
        Ok(identifier(&format!(
            "{}.{}",
            self.lean_module(&module)?,
            member.name
        )))
    }

    /// The Lean spelling of a closed type.
    fn ty(&self, ty: &SemanticType) -> Result<String, String> {
        self.render_type(ty, false)
    }

    /// The Lean spelling of a type; a generic one may name the type
    /// parameters a wrapper binds.
    fn render_type(&self, ty: &SemanticType, generic: bool) -> Result<String, String> {
        Ok(match ty {
            SemanticType::Nat => "Nat".to_owned(),
            SemanticType::Bool => "Bool".to_owned(),
            SemanticType::Unit => "Unit".to_owned(),
            SemanticType::Int => "Int".to_owned(),
            SemanticType::Int8 => "Int8".to_owned(),
            SemanticType::Int16 => "Int16".to_owned(),
            SemanticType::Int32 => "Int32".to_owned(),
            SemanticType::Int64 => "Int64".to_owned(),
            SemanticType::UInt8 => "UInt8".to_owned(),
            SemanticType::UInt16 => "UInt16".to_owned(),
            SemanticType::UInt32 => "UInt32".to_owned(),
            SemanticType::UInt64 => "UInt64".to_owned(),
            SemanticType::String => "String".to_owned(),
            SemanticType::Bytes => "ByteArray".to_owned(),
            SemanticType::Ordering => "Ordering".to_owned(),
            SemanticType::Option { value } => {
                format!("(Option {})", self.render_type(value, generic)?)
            }
            SemanticType::Result { ok, error } => {
                format!(
                    "(Except {} {})",
                    self.render_type(error, generic)?,
                    self.render_type(ok, generic)?
                )
            }
            SemanticType::List { element } | SemanticType::Set { element } => {
                format!("(List {})", self.render_type(element, generic)?)
            }
            SemanticType::Product { left, right } => {
                format!(
                    "(Prod {} {})",
                    self.render_type(left, generic)?,
                    self.render_type(right, generic)?
                )
            }
            SemanticType::Map { key, value } => format!(
                "(List (Prod {} {}))",
                self.render_type(key, generic)?,
                self.render_type(value, generic)?
            ),
            SemanticType::Function { parameters, result } => {
                let mut out = String::from("(");
                for parameter in parameters {
                    out.push_str(&self.render_type(parameter, generic)?);
                    out.push_str(" -> ");
                }
                out.push_str(&self.render_type(result, generic)?);
                out.push(')');
                out
            }
            SemanticType::Named { member, arguments } => {
                let module = member.module.clone().ok_or("an unanchored document type")?;
                let mut out = format!(
                    "({}",
                    identifier(&format!("{}.{}", self.lean_module(&module)?, member.name))
                );
                for argument in arguments {
                    out.push(' ');
                    out.push_str(&self.render_type(argument, generic)?);
                }
                out.push(')');
                out
            }
            SemanticType::Parameter { name } if generic => identifier(name),
            SemanticType::Type | SemanticType::Prop | SemanticType::Parameter { name: _ } => {
                return Err(format!(
                    "`{}` has no runtime encoding",
                    self.source.type_text(ty)
                ));
            }
        })
    }

    /// The encoder of a closed first-order type into target values.
    fn enc(&self, ty: &SemanticType) -> Result<String, String> {
        match self.nested.get(&self.source.type_text(ty)) {
            Some(nested) => Ok(nested_enc(nested)),
            None => self.plain_enc(ty),
        }
    }

    /// The encoder of a type with no nested auxiliary encoder.
    fn plain_enc(&self, ty: &SemanticType) -> Result<String, String> {
        Ok(match ty {
            SemanticType::Nat => value_ctor("nat"),
            SemanticType::Bool => value_ctor("bool"),
            SemanticType::Int => value_ctor("int"),
            SemanticType::Int8 => value_ctor("i8"),
            SemanticType::Int16 => value_ctor("i16"),
            SemanticType::Int32 => value_ctor("i32"),
            SemanticType::Int64 => value_ctor("i64"),
            SemanticType::UInt8 => value_ctor("u8"),
            SemanticType::UInt16 => value_ctor("u16"),
            SemanticType::UInt32 => value_ctor("u32"),
            SemanticType::UInt64 => value_ctor("u64"),
            SemanticType::String => value_ctor("string"),
            SemanticType::Bytes => value_ctor("bytes"),
            SemanticType::Unit => lib("encUnit"),
            SemanticType::Ordering => lib("encOrdering"),
            SemanticType::Option { value } => {
                format!("({} {})", lib("encOption"), self.enc(value)?)
            }
            SemanticType::Result { ok, error } => format!(
                "({} {} {})",
                lib("encExcept"),
                self.enc(error)?,
                self.enc(ok)?
            ),
            SemanticType::List { element } | SemanticType::Set { element } => {
                format!("({} {})", lib("encList"), self.enc(element)?)
            }
            SemanticType::Product { left, right } => format!(
                "({} {} {})",
                lib("encPair"),
                self.enc(left)?,
                self.enc(right)?
            ),
            SemanticType::Map { key, value } => format!(
                "({} ({} {} {}))",
                lib("encList"),
                lib("encPair"),
                self.enc(key)?,
                self.enc(value)?
            ),
            SemanticType::Named {
                member: _,
                arguments: _,
            } => {
                let text = self.source.type_text(ty);
                let index = self
                    .adts
                    .get(&text)
                    .ok_or_else(|| format!("`{text}` has no ADT in the layout"))?;
                format!("__enc_{index}")
            }
            SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ } => {
                return Err(format!(
                    "`{}` has no value encoding",
                    self.source.type_text(ty)
                ));
            }
        })
    }

    /// The encoded value of a first-order local, or the closure value of a
    /// function-typed one.
    fn encoded_local(&self, local: &CLocal) -> Result<String, String> {
        match &local.companion {
            Some(companion) => Ok(format!(
                "({} {} {})",
                value_ctor("closure"),
                companion.index,
                companion.captures
            )),
            None => Ok(format!(
                "({} {})",
                self.enc(&local.local.ty)?,
                identifier(&local.local.name)
            )),
        }
    }

    fn runtime(&self, site: &Site, namespace: &str, operation: &str) -> Result<String, String> {
        Ok(format!(
            "{}.{namespace}.{operation}",
            self.lean_module(&site.module)?
        ))
    }

    /// A branch pattern at `site`.
    fn pattern(&self, branch: &SemanticBranch, site: &Site) -> Result<String, String> {
        let mut out = self.member(&branch.constructor, site)?;
        for binder in &branch.binders {
            out.push(' ');
            out.push_str(&identifier(binder));
        }
        Ok(out)
    }

    fn src_terms(&self, terms: &[SemanticTerm], ctx: &Ctx) -> Result<String, String> {
        let mut out = String::new();
        for term in terms {
            out.push_str(" (");
            out.push_str(&self.src(term, ctx)?);
            out.push(')');
        }
        Ok(out)
    }

    /// The Lean denotation of a source term, as the generated module states
    /// it but fully qualified.
    #[allow(clippy::too_many_lines)]
    fn src(&self, term: &SemanticTerm, ctx: &Ctx) -> Result<String, String> {
        let site = &ctx.site;
        let binary = |operator: &str, left: &SemanticTerm, right: &SemanticTerm| {
            Ok::<String, String>(format!(
                "({} {operator} {})",
                self.src(left, ctx)?,
                self.src(right, ctx)?
            ))
        };
        Ok(match term {
            SemanticTerm::Var { name } => identifier(name),
            SemanticTerm::Nat { value } => format!("({value} : Nat)"),
            SemanticTerm::Integer {
                representation,
                value,
            } => format!("({value} : {representation:?})"),
            SemanticTerm::String { value } => string_literal(value),
            SemanticTerm::Bytes { hex } => format!("({})", bytes_literal(hex)?),
            SemanticTerm::Primitive {
                operation,
                arguments,
                result,
            } => {
                let result = self.source.close(result, site);
                let specialization = match operation {
                    SemanticPrimitive::CheckedAdd => Some("checkedAddInt64"),
                    SemanticPrimitive::CheckedSubtract => Some("checkedSubtractInt64"),
                    SemanticPrimitive::CheckedMultiply => Some("checkedMultiplyInt64"),
                    SemanticPrimitive::CheckedNegate => Some("checkedNegateInt64"),
                    SemanticPrimitive::CheckedQuotient => Some("checkedQuotientInt64"),
                    SemanticPrimitive::Subtract
                    | SemanticPrimitive::Multiply
                    | SemanticPrimitive::Quotient
                    | SemanticPrimitive::Remainder
                    | SemanticPrimitive::Negate
                    | SemanticPrimitive::CheckedConvert
                    | SemanticPrimitive::BitAnd
                    | SemanticPrimitive::BitOr
                    | SemanticPrimitive::BitXor
                    | SemanticPrimitive::BitNot
                    | SemanticPrimitive::ShiftLeft
                    | SemanticPrimitive::ShiftRight
                    | SemanticPrimitive::Append
                    | SemanticPrimitive::Length
                    | SemanticPrimitive::Index
                    | SemanticPrimitive::Slice
                    | SemanticPrimitive::Utf8Encode
                    | SemanticPrimitive::Utf8Decode
                    | SemanticPrimitive::CompareBytes
                    | SemanticPrimitive::Equal
                    | SemanticPrimitive::SplitExact
                    | SemanticPrimitive::Join
                    | SemanticPrimitive::ParseDecimal
                    | SemanticPrimitive::FormatDecimal
                    | SemanticPrimitive::MapInsert
                    | SemanticPrimitive::MapRemove
                    | SemanticPrimitive::MapLookup
                    | SemanticPrimitive::MapContains
                    | SemanticPrimitive::MapSize
                    | SemanticPrimitive::MapKeys
                    | SemanticPrimitive::MapValues
                    | SemanticPrimitive::MapEntries
                    | SemanticPrimitive::MapFold
                    | SemanticPrimitive::SetInsert
                    | SemanticPrimitive::SetRemove
                    | SemanticPrimitive::SetContains
                    | SemanticPrimitive::SetSize
                    | SemanticPrimitive::SetElements
                    | SemanticPrimitive::SetUnion
                    | SemanticPrimitive::SetIntersection
                    | SemanticPrimitive::SetDifference
                    | SemanticPrimitive::SetFold
                    | SemanticPrimitive::ListFold
                    | SemanticPrimitive::Iterate
                    | SemanticPrimitive::IterateUntil
                    | SemanticPrimitive::GraphSuccessors
                    | SemanticPrimitive::GraphReachable
                    | SemanticPrimitive::GraphTopological => None,
                };
                let int64 = SemanticType::Option {
                    value: Box::new(SemanticType::Int64),
                };
                let (namespace, name) = match specialization {
                    Some(name) if result == int64 => ("LexLeanRuntime", name),
                    Some(_) | None => primitive_name(*operation),
                };
                format!(
                    "({}{} : {})",
                    self.runtime(site, namespace, name)?,
                    self.src_terms(arguments, ctx)?,
                    self.ty(&result)?
                )
            }
            SemanticTerm::Bool { value } => value.to_string(),
            SemanticTerm::Unit => "()".to_owned(),
            SemanticTerm::Nil { element } => format!(
                "([] : List {})",
                self.ty(&self.source.close(element, site))?
            ),
            SemanticTerm::Cons { head, tail } => {
                format!("({} :: {})", self.src(head, ctx)?, self.src(tail, ctx)?)
            }
            SemanticTerm::Record {
                r#type: _,
                type_arguments: _,
                fields,
            } => {
                let ty = self.source.infer(term, &ctx.sources(), site)?;
                format!(
                    "({{ {} }} : {})",
                    fields
                        .iter()
                        .map(|SemanticAssignment { field, value }| {
                            Ok::<String, String>(format!(
                                "{} := {}",
                                identifier(field),
                                self.src(value, ctx)?
                            ))
                        })
                        .collect::<Result<Vec<_>, _>>()?
                        .join(", "),
                    self.ty(&ty)?
                )
            }
            SemanticTerm::Constructor {
                constructor,
                type_arguments: _,
                arguments,
            } => {
                let ty = self.source.infer(term, &ctx.sources(), site)?;
                format!(
                    "({}{} : {})",
                    self.member(constructor, site)?,
                    self.src_terms(arguments, ctx)?,
                    self.ty(&ty)?
                )
            }
            SemanticTerm::InstanceValue {
                class: _,
                arguments: _,
                resolved,
            } => self.member(resolved, site)?,
            SemanticTerm::Project { value, field } => {
                format!("({}).{}", self.src(value, ctx)?, identifier(field))
            }
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } => {
                let mut out = format!("({}", self.member(function, site)?);
                for argument in type_arguments {
                    out.push(' ');
                    out.push_str(&self.ty(&self.source.close(argument, site))?);
                }
                out.push_str(&self.src_terms(arguments, ctx)?);
                out.push(')');
                out
            }
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => match ctx.hypotheses.get(&(std::ptr::from_ref(term) as usize)) {
                Some(index) => format!(
                    "(match (generalizing := false) {} : {} with | true => {} | false => {})",
                    hypothesis(*index),
                    self.src(condition, ctx)?,
                    self.src(then_value, ctx)?,
                    self.src(else_value, ctx)?
                ),
                None => format!(
                    "(if {} then {} else {})",
                    self.src(condition, ctx)?,
                    self.src(then_value, ctx)?,
                    self.src(else_value, ctx)?
                ),
            },
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => {
                let mut alternatives = Vec::new();
                for branch in branches {
                    let inner = ctx.with(self.branch_locals(branch, scrutinee, ctx)?);
                    alternatives.push(self.src(&branch.body, &inner)?);
                }
                let result = self.ty(&self.source.infer(term, &ctx.sources(), site)?)?;
                let hypothesis = ctx
                    .hypotheses
                    .get(&(std::ptr::from_ref(term) as usize))
                    .map(|index| hypothesis(*index));
                self.select(
                    ctx,
                    scrutinee,
                    &self.src(scrutinee, ctx)?,
                    branches,
                    &alternatives,
                    &result,
                    hypothesis.as_deref(),
                )?
            }
            SemanticTerm::Add { left, right } => binary("+", left, right)?,
            SemanticTerm::Beq { left, right } => format!(
                "(Nat.beq {} {})",
                self.src(left, ctx)?,
                self.src(right, ctx)?
            ),
            SemanticTerm::Ble { left, right } => format!(
                "(Nat.ble {} {})",
                self.src(left, ctx)?,
                self.src(right, ctx)?
            ),
            SemanticTerm::Blt { left, right } => format!(
                "(Nat.blt {} {})",
                self.src(left, ctx)?,
                self.src(right, ctx)?
            ),
            SemanticTerm::And { left, right } => binary("&&", left, right)?,
            SemanticTerm::Or { left, right } => binary("||", left, right)?,
            SemanticTerm::Not { value } => format!("(!{})", self.src(value, ctx)?),
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => {
                let (local, generic) = self.let_local(binder, ctx);
                let ty = local.ty.clone();
                let inner = ctx.with(vec![(local, generic)]);
                format!(
                    "(let {} : {} := {}; {})",
                    identifier(&binder.name),
                    self.ty(&ty)?,
                    self.src(value, ctx)?,
                    self.src(body, &inner)?
                )
            }
            SemanticTerm::Pair { left, right } => {
                format!("({}, {})", self.src(left, ctx)?, self.src(right, ctx)?)
            }
            SemanticTerm::First { value } => format!("({}).1", self.src(value, ctx)?),
            SemanticTerm::Second { value } => format!("({}).2", self.src(value, ctx)?),
            SemanticTerm::Lambda {
                parameters,
                captures: _,
                body,
            } => {
                let bound: Vec<(Local, SemanticType)> = parameters
                    .iter()
                    .map(|binder| self.let_local(binder, ctx))
                    .collect();
                let mut out = String::from("(fun");
                for (local, _) in &bound {
                    out.push_str(&format!(
                        " ({} : {})",
                        identifier(&local.name),
                        self.ty(&local.ty)?
                    ));
                }
                out.push_str(&format!(" => {})", self.src(body, &ctx.with(bound))?));
                out
            }
            SemanticTerm::Apply {
                function,
                arguments,
            } => format!(
                "({}{})",
                self.src(function, ctx)?,
                self.src_terms(arguments, ctx)?
            ),
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                let mut out = format!("({}", self.member(function, site)?);
                for argument in type_arguments {
                    out.push(' ');
                    out.push_str(&self.ty(&self.source.close(argument, site))?);
                }
                out.push(')');
                out
            }
            SemanticTerm::MapLiteral {
                key,
                value,
                entries,
            } => format!(
                "([{}] : List (Prod {} {}))",
                entries
                    .iter()
                    .map(|SemanticMapEntry { key, value }| {
                        Ok::<String, String>(format!(
                            "({}, {})",
                            self.src(key, ctx)?,
                            self.src(value, ctx)?
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", "),
                self.ty(&self.source.close(key, site))?,
                self.ty(&self.source.close(value, site))?
            ),
            SemanticTerm::SetLiteral { element, elements } => format!(
                "([{}] : List {})",
                elements
                    .iter()
                    .map(|element| self.src(element, ctx))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", "),
                self.ty(&self.source.close(element, site))?
            ),
            SemanticTerm::GraphLiteral { node, nodes, edges } => {
                let node = self.ty(&self.source.close(node, site))?;
                let mut items = Vec::new();
                for source in nodes {
                    let mut targets = Vec::new();
                    for SemanticEdge {
                        source: from,
                        target,
                    } in edges
                    {
                        if from == source {
                            targets.push(self.src(target, ctx)?);
                        }
                    }
                    items.push(format!(
                        "({}, [{}])",
                        self.src(source, ctx)?,
                        targets.join(", ")
                    ));
                }
                format!(
                    "([{}] : List (Prod {node} (List {node})))",
                    items.join(", ")
                )
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
            | SemanticTerm::Forall { binder: _, body: _ } => {
                return Err(format!(
                    "the formal-only construct `{}` has no runtime denotation",
                    super::eligibility::term_key(term)
                ));
            }
        })
    }
}

/// The runtime namespace and name a primitive is rendered with.
const fn primitive_name(operation: SemanticPrimitive) -> (&'static str, &'static str) {
    match operation {
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
    }
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
        | SemanticType::Set { element: _ } => width_suffix(ty),
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
        } => Err("an element of a non-collection".to_owned()),
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
        | SemanticType::Set { element: _ } => Err("a non-option primitive result".to_owned()),
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
        | SemanticType::Set { element: _ } => {
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
        | SemanticType::Set { element: _ } => {
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
        | SemanticType::UInt64 => width_suffix(ty),
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

impl Gen<'_> {
    /// The relation lemma of a directly realized primitive at its operand
    /// types.
    #[allow(clippy::too_many_lines)]
    fn prim_rule(
        &self,
        operation: SemanticPrimitive,
        types: &[SemanticType],
        result: &SemanticType,
        site: &Site,
    ) -> Result<PrimRule, String> {
        let first = types
            .first()
            .ok_or_else(|| format!("{operation:?} has no operand"))?;
        let rule = |lemma: String, width: Width| PrimRule {
            lemma,
            prefix: Vec::new(),
            width,
        };
        let numeric_rule = |nat: &str, nat_width: Width, int: &str| -> Result<PrimRule, String> {
            Ok(match numeric(first)? {
                Numeric::Nat => rule(lib(nat), nat_width),
                Numeric::Int => rule(lib(int), Width::Int),
            })
        };
        Ok(match operation {
            SemanticPrimitive::Subtract => {
                numeric_rule("prim_natSub", Width::Exact, "prim_intSub")?
            }
            SemanticPrimitive::Multiply => numeric_rule("prim_natMul", Width::Nat, "prim_intMul")?,
            SemanticPrimitive::Quotient => {
                numeric_rule("prim_natQuot", Width::Exact, "prim_intQuot")?
            }
            SemanticPrimitive::Remainder => {
                numeric_rule("prim_natRem", Width::Exact, "prim_intRem")?
            }
            SemanticPrimitive::Negate => rule(lib("prim_intNeg"), Width::Int),
            SemanticPrimitive::CheckedConvert => rule(
                lib(&format!(
                    "prim_convert_{}_{}",
                    integer_suffix(first)?,
                    width_suffix(&option_value(result)?)?
                )),
                Width::Exact,
            ),
            SemanticPrimitive::CheckedAdd => rule(
                lib(&format!("prim_checkedAdd_{}", width_suffix(first)?)),
                Width::Exact,
            ),
            SemanticPrimitive::CheckedSubtract => rule(
                lib(&format!("prim_checkedSub_{}", width_suffix(first)?)),
                Width::Exact,
            ),
            // The 64-bit signed loops are taken by their clauses (§17.17.3).
            SemanticPrimitive::CheckedMultiply => {
                let suffix = width_suffix(first)?;
                PrimRule {
                    lemma: lib(&format!("prim_checkedMul_{suffix}")),
                    prefix: if suffix == "i64" {
                        vec![
                            self.runtime(site, "LexLeanRuntime", "multiplyMagnitudeInt64")?,
                            "(fun _ _ _ _ => rfl)".to_owned(),
                            "(fun _ _ _ _ _ => rfl)".to_owned(),
                        ]
                    } else {
                        Vec::new()
                    },
                    width: Width::Exact,
                }
            }
            SemanticPrimitive::CheckedNegate => rule(
                lib(&format!("prim_checkedNeg_{}", width_suffix(first)?)),
                Width::Exact,
            ),
            SemanticPrimitive::CheckedQuotient => {
                let suffix = width_suffix(first)?;
                PrimRule {
                    lemma: lib(&format!("prim_checkedQuot_{suffix}")),
                    prefix: if suffix == "i64" {
                        vec![
                            self.runtime(site, "LexLeanRuntime", "divideMagnitudeInt64")?,
                            "(fun _ _ _ _ => rfl)".to_owned(),
                            "(fun _ _ _ _ _ => rfl)".to_owned(),
                        ]
                    } else {
                        Vec::new()
                    },
                    width: Width::Exact,
                }
            }
            SemanticPrimitive::BitAnd => rule(
                lib(&format!("prim_bitAnd_{}", width_suffix(first)?)),
                Width::Exact,
            ),
            SemanticPrimitive::BitOr => rule(
                lib(&format!("prim_bitOr_{}", width_suffix(first)?)),
                Width::Exact,
            ),
            SemanticPrimitive::BitXor => rule(
                lib(&format!("prim_bitXor_{}", width_suffix(first)?)),
                Width::Exact,
            ),
            SemanticPrimitive::BitNot => rule(
                lib(&format!("prim_bitNot_{}", width_suffix(first)?)),
                Width::Exact,
            ),
            SemanticPrimitive::ShiftLeft => rule(
                lib(&format!("prim_shiftLeft_{}", width_suffix(first)?)),
                Width::Exact,
            ),
            SemanticPrimitive::ShiftRight => rule(
                lib(&format!("prim_shiftRight_{}", width_suffix(first)?)),
                Width::Exact,
            ),
            SemanticPrimitive::Append => match sequence(first)? {
                Sequence::List(_) => PrimRule {
                    lemma: lib("prim_append_list"),
                    prefix: vec![self.list_bundle(first)?],
                    width: Width::Exact,
                },
                Sequence::Bytes => rule(lib("prim_append_bytes"), Width::Exact),
                Sequence::String => return Err("Append of strings".to_owned()),
            },
            SemanticPrimitive::Length => match sequence(first)? {
                Sequence::List(_) => PrimRule {
                    lemma: lib("prim_length_list"),
                    prefix: vec![self.list_bundle(first)?],
                    width: Width::Nat,
                },
                Sequence::Bytes => rule(lib("prim_length_bytes"), Width::Nat),
                Sequence::String => rule(lib("prim_length_string"), Width::Nat),
            },
            SemanticPrimitive::MapSize | SemanticPrimitive::SetSize => PrimRule {
                lemma: lib("prim_length_list"),
                prefix: vec![self.list_bundle(first)?],
                width: Width::Nat,
            },
            SemanticPrimitive::Index => match sequence(first)? {
                Sequence::List(element) => PrimRule {
                    lemma: lib("prim_index_list"),
                    prefix: vec![
                        self.list_bundle(first)?,
                        self.option_bundle(&SemanticType::Option {
                            value: Box::new(element),
                        })?,
                        "(fun _ => rfl)".to_owned(),
                        self.runtime(site, "LexLeanRuntime", "listIndex")?,
                        "(fun _ => rfl)".to_owned(),
                        "(fun _ _ => rfl)".to_owned(),
                        "(fun _ _ _ => rfl)".to_owned(),
                    ],
                    width: Width::Exact,
                },
                Sequence::Bytes => rule(lib("prim_index_bytes"), Width::Exact),
                Sequence::String => return Err("Index of a string".to_owned()),
            },
            SemanticPrimitive::Slice => match sequence(first)? {
                Sequence::List(_) => PrimRule {
                    lemma: lib("prim_slice_list"),
                    prefix: vec![
                        self.list_bundle(first)?,
                        self.option_bundle(&SemanticType::Option {
                            value: Box::new(first.clone()),
                        })?,
                        "(fun _ => rfl)".to_owned(),
                    ],
                    width: Width::Exact,
                },
                Sequence::Bytes => rule(lib("prim_slice_bytes"), Width::Exact),
                Sequence::String => return Err("Slice of a string".to_owned()),
            },
            SemanticPrimitive::Utf8Encode => rule(lib("prim_utf8Encode"), Width::Exact),
            SemanticPrimitive::Utf8Decode => rule(lib("prim_utf8Decode"), Width::Exact),
            SemanticPrimitive::CompareBytes => rule(lib("prim_compareBytes"), Width::Exact),
            SemanticPrimitive::Equal => rule(
                lib(&format!("prim_equal_{}", equal_suffix(first)?)),
                Width::Exact,
            ),
            SemanticPrimitive::SplitExact => rule(lib("prim_splitExact"), Width::Exact),
            SemanticPrimitive::Join => rule(lib("prim_join"), Width::Exact),
            SemanticPrimitive::ParseDecimal => {
                let target = option_value(result)?;
                match numeric(&target) {
                    Ok(Numeric::Int) => rule(
                        lib("prim_parseDecimal_int"),
                        Width::Predicate("decimalFits"),
                    ),
                    Ok(Numeric::Nat) | Err(_) => rule(
                        lib(&format!("prim_parseDecimal_{}", width_suffix(&target)?)),
                        Width::Exact,
                    ),
                }
            }
            SemanticPrimitive::FormatDecimal => rule(
                lib(&format!("prim_formatDecimal_{}", integer_suffix(first)?)),
                Width::Exact,
            ),
            SemanticPrimitive::MapEntries
            | SemanticPrimitive::SetElements
            | SemanticPrimitive::MapInsert
            | SemanticPrimitive::MapRemove
            | SemanticPrimitive::MapLookup
            | SemanticPrimitive::MapContains
            | SemanticPrimitive::MapKeys
            | SemanticPrimitive::MapValues
            | SemanticPrimitive::MapFold
            | SemanticPrimitive::SetInsert
            | SemanticPrimitive::SetRemove
            | SemanticPrimitive::SetContains
            | SemanticPrimitive::SetUnion
            | SemanticPrimitive::SetIntersection
            | SemanticPrimitive::SetDifference
            | SemanticPrimitive::SetFold
            | SemanticPrimitive::ListFold
            | SemanticPrimitive::Iterate
            | SemanticPrimitive::IterateUntil
            | SemanticPrimitive::GraphSuccessors
            | SemanticPrimitive::GraphReachable
            | SemanticPrimitive::GraphTopological => {
                return Err(format!("{operation:?} is not a direct primitive"));
            }
        })
    }
}

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

    /// The arguments a callee's relation and fits take for one source
    /// argument: the value, and for a function value its companion.
    /// Returns `(fits arguments, relation arguments, operand proof)`.
    fn argument(
        &mut self,
        argument: &SemanticTerm,
        ctx: &Ctx,
    ) -> Result<(String, String, Proof), String> {
        match self.function_argument(argument, ctx)? {
            Some(companion) => Ok((
                format!("({}) ({})", companion.value, companion.fits),
                format!(
                    "({}) ({}) ({}) ({}) ({})",
                    companion.value,
                    companion.fits,
                    companion.index,
                    companion.captures,
                    companion.relation
                ),
                companion.proof,
            )),
            None => {
                let proof = self.prove(argument, ctx)?;
                let value = format!("({})", self.src(argument, ctx)?);
                Ok((value.clone(), value, proof))
            }
        }
    }

    /// A function-typed argument's value and companions: its fits function,
    /// closure index, encoded captures, and relation; `None` for a
    /// first-order argument.
    fn function_argument(
        &mut self,
        argument: &SemanticTerm,
        ctx: &Ctx,
    ) -> Result<Option<FunctionArgument>, String> {
        let ty = self.source.infer(argument, &ctx.sources(), &ctx.site)?;
        let parameters = match function_type(&ty) {
            Some((parameters, _)) => parameters.to_vec(),
            None => return Ok(None),
        };
        let binders: Vec<String> = (0..parameters.len())
            .map(|index| format!("__p{index}"))
            .collect();
        let typed: String = binders
            .iter()
            .zip(&parameters)
            .map(|(binder, ty)| Ok::<String, String>(format!(" ({binder} : {})", self.ty(ty)?)))
            .collect::<Result<Vec<_>, _>>()?
            .concat();
        let applied = binders.join(" ");
        match argument {
            SemanticTerm::Var { name } => {
                let local = ctx.lookup(name)?;
                let companion = local
                    .companion
                    .as_ref()
                    .ok_or_else(|| format!("function-typed `{name}` has no companion"))?;
                Ok(Some(FunctionArgument {
                    value: identifier(name),
                    fits: companion.fits.clone(),
                    index: companion.index.clone(),
                    captures: companion.captures.clone(),
                    relation: companion.relation.clone(),
                    proof: Proof {
                        proof: format!("({} rfl)", lib("conv_var")),
                        fits: "true".to_owned(),
                    },
                }))
            }
            SemanticTerm::Lambda {
                parameters: _,
                captures,
                body: _,
            } => {
                let index = self.next_lambda(ctx.function)?;
                let mut fits_captures = String::new();
                let mut relation_captures = String::new();
                let mut encoded = Vec::new();
                let mut proofs = Vec::new();
                for capture in captures {
                    let local = ctx.lookup(capture)?.clone();
                    let name = identifier(capture);
                    match &local.companion {
                        Some(companion) => {
                            fits_captures.push_str(&format!(" ({name}) ({})", companion.fits));
                            relation_captures.push_str(&format!(
                                " ({name}) ({}) ({}) ({}) ({})",
                                companion.fits,
                                companion.index,
                                companion.captures,
                                companion.relation
                            ));
                        }
                        None => {
                            fits_captures.push_str(&format!(" ({name})"));
                            relation_captures.push_str(&format!(" ({name})"));
                        }
                    }
                    encoded.push(self.encoded_local(&local)?);
                    proofs.push(Proof {
                        proof: format!("({} rfl)", lib("conv_var")),
                        fits: "true".to_owned(),
                    });
                }
                let captured = operands(proofs);
                Ok(Some(FunctionArgument {
                    value: self.src(argument, ctx)?,
                    fits: format!("fun{typed} => __fits_{index}{fits_captures} {applied}"),
                    index: index.to_string(),
                    captures: format!("[{}]", encoded.join(", ")),
                    relation: format!("fun{typed} => __rel_{index}{relation_captures} {applied}"),
                    proof: Proof {
                        proof: format!("({} {})", lib("conv_closure"), captured.proof),
                        fits: captured.fits,
                    },
                }))
            }
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                let module = Source::module_of(function, &ctx.site);
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.source.close(argument, &ctx.site))
                    .collect();
                let index = self.instance_index(&module, &function.name, &closed)?;
                Ok(Some(FunctionArgument {
                    value: self.src(argument, ctx)?,
                    fits: format!("fun{typed} => __fits_{index} {applied}"),
                    index: index.to_string(),
                    captures: "[]".to_owned(),
                    relation: format!("fun{typed} => __rel_{index} {applied}"),
                    proof: Proof {
                        proof: format!("({} {})", lib("conv_closure"), lib("convL_nil")),
                        fits: "true".to_owned(),
                    },
                }))
            }
            SemanticTerm::Nat { value: _ }
            | SemanticTerm::Integer {
                representation: _,
                value: _,
            }
            | SemanticTerm::String { value: _ }
            | SemanticTerm::Bytes { hex: _ }
            | SemanticTerm::Primitive {
                operation: _,
                arguments: _,
                result: _,
            }
            | SemanticTerm::Bool { value: _ }
            | SemanticTerm::Unit
            | SemanticTerm::Nil { element: _ }
            | SemanticTerm::Cons { head: _, tail: _ }
            | SemanticTerm::Record {
                r#type: _,
                type_arguments: _,
                fields: _,
            }
            | SemanticTerm::Constructor {
                constructor: _,
                type_arguments: _,
                arguments: _,
            }
            | SemanticTerm::InstanceValue {
                class: _,
                arguments: _,
                resolved: _,
            }
            | SemanticTerm::Project { value: _, field: _ }
            | SemanticTerm::Call {
                function: _,
                type_arguments: _,
                arguments: _,
            }
            | SemanticTerm::If {
                condition: _,
                then_value: _,
                else_value: _,
            }
            | SemanticTerm::Match {
                scrutinee: _,
                branches: _,
            }
            | SemanticTerm::Eq { left: _, right: _ }
            | SemanticTerm::Le { left: _, right: _ }
            | SemanticTerm::Lt { left: _, right: _ }
            | SemanticTerm::Add { left: _, right: _ }
            | SemanticTerm::Beq { left: _, right: _ }
            | SemanticTerm::Ble { left: _, right: _ }
            | SemanticTerm::Blt { left: _, right: _ }
            | SemanticTerm::And { left: _, right: _ }
            | SemanticTerm::PropAnd { left: _, right: _ }
            | SemanticTerm::Or { left: _, right: _ }
            | SemanticTerm::Not { value: _ }
            | SemanticTerm::Implies {
                premise: _,
                conclusion: _,
            }
            | SemanticTerm::Iff { left: _, right: _ }
            | SemanticTerm::Forall { binder: _, body: _ }
            | SemanticTerm::Let {
                binder: _,
                value: _,
                body: _,
            }
            | SemanticTerm::Pair { left: _, right: _ }
            | SemanticTerm::First { value: _ }
            | SemanticTerm::Second { value: _ }
            | SemanticTerm::Apply {
                function: _,
                arguments: _,
            }
            | SemanticTerm::MapLiteral {
                key: _,
                value: _,
                entries: _,
            }
            | SemanticTerm::SetLiteral {
                element: _,
                elements: _,
            }
            | SemanticTerm::GraphLiteral {
                node: _,
                nodes: _,
                edges: _,
            } => Err(format!(
                "a function value `{}` is neither a parameter, a lambda, nor a function reference",
                super::eligibility::term_key(argument)
            )),
        }
    }
}

impl Gen<'_> {
    fn conv_var() -> Proof {
        Proof {
            proof: format!("({} rfl)", lib("conv_var")),
            fits: "true".to_owned(),
        }
    }

    fn proofs(&mut self, terms: &[SemanticTerm], ctx: &Ctx) -> Result<Vec<Proof>, String> {
        terms.iter().map(|term| self.prove(term, ctx)).collect()
    }

    /// The constructor lemma and fits of building `constructor`.
    fn construct_lemma(&self, constructor: &Constructor, whole: &str) -> (String, String) {
        match constructor {
            Constructor::Bool(true) => (lib("construct_true"), "true".to_owned()),
            Constructor::Bool(false) => (lib("construct_false"), "true".to_owned()),
            Constructor::Zero => (lib("construct_zero"), "true".to_owned()),
            Constructor::Succ => (
                lib("construct_succ"),
                format!("(Nat.blt {whole} {NAT_BOUND})"),
            ),
            Constructor::Nil => (lib("construct_nil"), "true".to_owned()),
            Constructor::Cons => (lib("construct_cons"), "true".to_owned()),
            Constructor::OptionNone => (lib("construct_none"), "true".to_owned()),
            Constructor::OptionSome => (lib("construct_some"), "true".to_owned()),
            Constructor::Ok => (lib("construct_ok"), "true".to_owned()),
            Constructor::Error => (lib("construct_error"), "true".to_owned()),
            Constructor::Document { ty: _, index: _ } => (lib("construct_adt"), "true".to_owned()),
        }
    }

    /// A literal list built as a cons chain.
    fn cons_chain(items: Vec<Proof>) -> Proof {
        items.into_iter().rev().fold(
            built(operands(Vec::new()), &lib("construct_nil"), "true"),
            |tail, head| built(operands(vec![head, tail]), &lib("construct_cons"), "true"),
        )
    }

    /// The fields of a record of closed type `ty`, in declaration order.
    fn record(
        &mut self,
        ty: &SemanticType,
        fields: &[SemanticAssignment],
        ctx: &Ctx,
    ) -> Result<Proof, String> {
        let shape = self.source.document(ty)?;
        let mut proofs = Vec::new();
        for field in &shape.field_names {
            let SemanticAssignment { field: _, value } = fields
                .iter()
                .find(|assignment| &assignment.field == field)
                .ok_or_else(|| format!("field `{field}` is not assigned"))?;
            proofs.push(self.prove(value, ctx)?);
        }
        Ok(built(operands(proofs), &lib("construct_adt"), "true"))
    }

    #[allow(clippy::too_many_lines)]
    fn prove(&mut self, term: &SemanticTerm, ctx: &Ctx) -> Result<Proof, String> {
        let site = ctx.site.clone();
        let value = || Proof {
            proof: lib("conv_value"),
            fits: "true".to_owned(),
        };
        Ok(match term {
            SemanticTerm::Var { name: _ } => Self::conv_var(),
            SemanticTerm::Nat { value: _ }
            | SemanticTerm::Integer {
                representation: _,
                value: _,
            }
            | SemanticTerm::String { value: _ }
            | SemanticTerm::Bytes { hex: _ } => value(),
            SemanticTerm::Primitive {
                operation,
                arguments,
                result,
            } => self.primitive(term, *operation, arguments, result, ctx)?,
            SemanticTerm::Bool { value } => built(
                operands(Vec::new()),
                &lib(if *value {
                    "construct_true"
                } else {
                    "construct_false"
                }),
                "true",
            ),
            SemanticTerm::Unit => built(operands(Vec::new()), &lib("construct_unit"), "true"),
            SemanticTerm::Nil { element: _ } => {
                built(operands(Vec::new()), &lib("construct_nil"), "true")
            }
            SemanticTerm::Cons { head, tail } => {
                let head = self.prove(head, ctx)?;
                let tail = self.prove(tail, ctx)?;
                built(operands(vec![head, tail]), &lib("construct_cons"), "true")
            }
            SemanticTerm::Record {
                r#type: _,
                type_arguments: _,
                fields,
            } => {
                let ty = self.source.infer(term, &ctx.sources(), &site)?;
                self.record(&ty, fields, ctx)?
            }
            SemanticTerm::Constructor {
                constructor,
                type_arguments,
                arguments,
            } => {
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.source.close(argument, &site))
                    .collect();
                let (constructor, _) = self.source.constructor(constructor, &closed, &site)?;
                let proofs = self.proofs(arguments, ctx)?;
                let whole = self.src(term, ctx)?;
                let (lemma, fits) = self.construct_lemma(&constructor, &whole);
                built(operands(proofs), &lemma, &fits)
            }
            SemanticTerm::InstanceValue {
                class: _,
                arguments: _,
                resolved,
            } => {
                let module = Source::module_of(resolved, &site);
                let index = self.instance_index(&module, &resolved.name, &[])?;
                Proof {
                    proof: format!("({} {} __rel_{index})", lib("conv_call"), lib("convL_nil")),
                    fits: and("true", &format!("__fits_{index}")),
                }
            }
            SemanticTerm::Project { value, field: _ } => {
                let inner = self.prove(value, ctx)?;
                Proof {
                    proof: format!("({} {} rfl)", lib("conv_field"), inner.proof),
                    fits: inner.fits,
                }
            }
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
                let index = self.instance_index(&module, &function.name, &closed)?;
                let mut fits_arguments = String::new();
                let mut relation_arguments = String::new();
                let mut proofs = Vec::new();
                for argument in arguments {
                    let (fits, relation, proof) = self.argument(argument, ctx)?;
                    fits_arguments.push(' ');
                    fits_arguments.push_str(&fits);
                    relation_arguments.push(' ');
                    relation_arguments.push_str(&relation);
                    proofs.push(proof);
                }
                let list = operands(proofs);
                Proof {
                    proof: format!(
                        "({} {} (__rel_{index}{relation_arguments}))",
                        lib("conv_call"),
                        list.proof
                    ),
                    fits: and(&list.fits, &format!("(__fits_{index}{fits_arguments})")),
                }
            }
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => self.conditional(term, condition, then_value, else_value, ctx)?,
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => self.matching(term, scrutinee, branches, ctx)?,
            SemanticTerm::Add { left, right } => {
                let list = operands(vec![self.prove(left, ctx)?, self.prove(right, ctx)?]);
                let (left, right) = (self.src(left, ctx)?, self.src(right, ctx)?);
                Proof {
                    proof: format!(
                        "({} {} ({} {left} {right}))",
                        lib("conv_prim"),
                        list.proof,
                        lib("prim_natAdd")
                    ),
                    fits: and(
                        &list.fits,
                        &format!("(Nat.blt ({left} + {right}) {NAT_BOUND})"),
                    ),
                }
            }
            SemanticTerm::Beq { left, right } => self.comparison("prim_natEq", left, right, ctx)?,
            SemanticTerm::Ble { left, right } => self.comparison("prim_natLe", left, right, ctx)?,
            SemanticTerm::Blt { left, right } => self.comparison("prim_natLt", left, right, ctx)?,
            // `a && b` is `if a then b else false` and `a || b` is
            // `if a then true else b` (§17.14).
            SemanticTerm::And { left, right } => {
                let condition = self.prove(left, ctx)?;
                let then_branch = self.prove(right, ctx)?;
                let else_branch = built(operands(Vec::new()), &lib("construct_false"), "true");
                let (left, right) = (self.src(left, ctx)?, self.src(right, ctx)?);
                Proof {
                    proof: format!(
                        "({} (fun (__b : Bool) => if __b then {} else {}) (fun (__b : Bool) => {} (__b && {right})) {left} {} (fun _ => {}) (fun _ => {}))",
                        lib("conv_cond"),
                        then_branch.fits,
                        else_branch.fits,
                        value_ctor("bool"),
                        condition.proof,
                        then_branch.proof,
                        else_branch.proof
                    ),
                    fits: and(
                        &condition.fits,
                        &format!(
                            "(if {left} then {} else {})",
                            then_branch.fits, else_branch.fits
                        ),
                    ),
                }
            }
            SemanticTerm::Or { left, right } => {
                let condition = self.prove(left, ctx)?;
                let then_branch = built(operands(Vec::new()), &lib("construct_true"), "true");
                let else_branch = self.prove(right, ctx)?;
                let (left, right) = (self.src(left, ctx)?, self.src(right, ctx)?);
                Proof {
                    proof: format!(
                        "({} (fun (__b : Bool) => if __b then {} else {}) (fun (__b : Bool) => {} (__b || {right})) {left} {} (fun _ => {}) (fun _ => {}))",
                        lib("conv_cond"),
                        then_branch.fits,
                        else_branch.fits,
                        value_ctor("bool"),
                        condition.proof,
                        then_branch.proof,
                        else_branch.proof
                    ),
                    fits: and(
                        &condition.fits,
                        &format!(
                            "(if {left} then {} else {})",
                            then_branch.fits, else_branch.fits
                        ),
                    ),
                }
            }
            SemanticTerm::Not { value } => {
                let list = operands(vec![self.prove(value, ctx)?]);
                Proof {
                    proof: format!(
                        "({} {} ({} {}))",
                        lib("conv_prim"),
                        list.proof,
                        lib("prim_boolNot"),
                        self.src(value, ctx)?
                    ),
                    fits: and(&list.fits, "true"),
                }
            }
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => {
                let (local, generic) = self.let_local(binder, ctx);
                let ty = local.ty.clone();
                let bound = self.prove(value, ctx)?;
                let inner = ctx.with(vec![(local, generic)]);
                let rest = self.prove(body, &inner)?;
                let binding = format!(
                    "let {} : {} := {};",
                    identifier(&binder.name),
                    self.ty(&ty)?,
                    self.src(value, ctx)?
                );
                Proof {
                    proof: format!(
                        "({binding} {} {} {})",
                        lib("conv_let"),
                        bound.proof,
                        rest.proof
                    ),
                    fits: and(&bound.fits, &format!("({binding} {})", rest.fits)),
                }
            }
            SemanticTerm::Pair { left, right } => {
                let list = operands(vec![self.prove(left, ctx)?, self.prove(right, ctx)?]);
                built(list, &lib("construct_pair"), "true")
            }
            SemanticTerm::First { value } => {
                let inner = self.prove(value, ctx)?;
                Proof {
                    proof: format!("({} {})", lib("conv_first"), inner.proof),
                    fits: inner.fits,
                }
            }
            SemanticTerm::Second { value } => {
                let inner = self.prove(value, ctx)?;
                Proof {
                    proof: format!("({} {})", lib("conv_second"), inner.proof),
                    fits: inner.fits,
                }
            }
            SemanticTerm::Apply {
                function,
                arguments,
            } => self.application(function, arguments, ctx)?,
            SemanticTerm::MapLiteral {
                key: _,
                value: _,
                entries,
            } => {
                let mut items = Vec::new();
                for SemanticMapEntry { key, value } in entries {
                    let list = operands(vec![self.prove(key, ctx)?, self.prove(value, ctx)?]);
                    items.push(built(list, &lib("construct_pair"), "true"));
                }
                Self::cons_chain(items)
            }
            SemanticTerm::SetLiteral {
                element: _,
                elements,
            } => {
                let items = self.proofs(elements, ctx)?;
                Self::cons_chain(items)
            }
            SemanticTerm::GraphLiteral {
                node: _,
                nodes,
                edges,
            } => {
                let mut items = Vec::new();
                for source in nodes {
                    let mut targets = Vec::new();
                    for SemanticEdge {
                        source: from,
                        target,
                    } in edges
                    {
                        if from == source {
                            targets.push(self.prove(target, ctx)?);
                        }
                    }
                    let source = self.prove(source, ctx)?;
                    let list = operands(vec![source, Self::cons_chain(targets)]);
                    items.push(built(list, &lib("construct_pair"), "true"));
                }
                Self::cons_chain(items)
            }
            SemanticTerm::Lambda {
                parameters: _,
                captures: _,
                body: _,
            }
            | SemanticTerm::FunctionRef {
                function: _,
                type_arguments: _,
            } => {
                return Err(
                    "a function value outside an argument or application position".to_owned(),
                );
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
            | SemanticTerm::Forall { binder: _, body: _ } => {
                return Err(format!(
                    "the formal-only construct `{}` has no realization",
                    super::eligibility::term_key(term)
                ));
            }
        })
    }

    /// A natural-number comparison, realized by its primitive.
    fn comparison(
        &mut self,
        lemma: &str,
        left: &SemanticTerm,
        right: &SemanticTerm,
        ctx: &Ctx,
    ) -> Result<Proof, String> {
        let list = operands(vec![self.prove(left, ctx)?, self.prove(right, ctx)?]);
        Ok(Proof {
            proof: format!(
                "({} {} ({} {} {}))",
                lib("conv_prim"),
                list.proof,
                lib(lemma),
                self.src(left, ctx)?,
                self.src(right, ctx)?
            ),
            fits: and(&list.fits, "true"),
        })
    }

    fn primitive(
        &mut self,
        whole: &SemanticTerm,
        operation: SemanticPrimitive,
        arguments: &[SemanticTerm],
        result: &SemanticType,
        ctx: &Ctx,
    ) -> Result<Proof, String> {
        let site = ctx.site.clone();
        let locals = ctx.sources();
        let types = arguments
            .iter()
            .map(|argument| self.source.infer(argument, &locals, &site))
            .collect::<Result<Vec<_>, _>>()?;
        let result = self.source.close(result, &site);
        match super::lower::template_of(operation, &types)? {
            Some((template, instance)) => {
                return self.template_call(template, &instance, arguments, &types, ctx);
            }
            None => {}
        }
        match operation {
            SemanticPrimitive::MapEntries | SemanticPrimitive::SetElements => {
                if arguments.len() != 1 {
                    return Err(format!("{operation:?} takes one operand"));
                }
                return self.prove(&arguments[0], ctx);
            }
            SemanticPrimitive::Subtract
            | SemanticPrimitive::Multiply
            | SemanticPrimitive::Quotient
            | SemanticPrimitive::Remainder
            | SemanticPrimitive::Negate
            | SemanticPrimitive::CheckedConvert
            | SemanticPrimitive::CheckedAdd
            | SemanticPrimitive::CheckedSubtract
            | SemanticPrimitive::CheckedMultiply
            | SemanticPrimitive::CheckedNegate
            | SemanticPrimitive::CheckedQuotient
            | SemanticPrimitive::BitAnd
            | SemanticPrimitive::BitOr
            | SemanticPrimitive::BitXor
            | SemanticPrimitive::BitNot
            | SemanticPrimitive::ShiftLeft
            | SemanticPrimitive::ShiftRight
            | SemanticPrimitive::Append
            | SemanticPrimitive::Length
            | SemanticPrimitive::Index
            | SemanticPrimitive::Slice
            | SemanticPrimitive::Utf8Encode
            | SemanticPrimitive::Utf8Decode
            | SemanticPrimitive::CompareBytes
            | SemanticPrimitive::Equal
            | SemanticPrimitive::SplitExact
            | SemanticPrimitive::Join
            | SemanticPrimitive::ParseDecimal
            | SemanticPrimitive::FormatDecimal
            | SemanticPrimitive::MapSize
            | SemanticPrimitive::SetSize
            | SemanticPrimitive::MapInsert
            | SemanticPrimitive::MapRemove
            | SemanticPrimitive::MapLookup
            | SemanticPrimitive::MapContains
            | SemanticPrimitive::MapKeys
            | SemanticPrimitive::MapValues
            | SemanticPrimitive::MapFold
            | SemanticPrimitive::SetInsert
            | SemanticPrimitive::SetRemove
            | SemanticPrimitive::SetContains
            | SemanticPrimitive::SetUnion
            | SemanticPrimitive::SetIntersection
            | SemanticPrimitive::SetDifference
            | SemanticPrimitive::SetFold
            | SemanticPrimitive::ListFold
            | SemanticPrimitive::Iterate
            | SemanticPrimitive::IterateUntil
            | SemanticPrimitive::GraphSuccessors
            | SemanticPrimitive::GraphReachable
            | SemanticPrimitive::GraphTopological => {}
        }
        let rule = self.prim_rule(operation, &types, &result, &site)?;
        let list = operands(self.proofs(arguments, ctx)?);
        let sources = arguments
            .iter()
            .map(|argument| self.src(argument, ctx))
            .collect::<Result<Vec<_>, _>>()?;
        let whole = self.src(whole, ctx)?;
        let fits = match rule.width {
            Width::Exact => "true".to_owned(),
            Width::Nat => format!("(Nat.blt {whole} {NAT_BOUND})"),
            Width::Int => format!("({} {whole})", lib("intFits")),
            Width::Predicate(name) => format!("({} {})", lib(name), sources.join(" ")),
        };
        let mut lemma = rule.lemma;
        for argument in rule.prefix.iter().chain(&sources) {
            lemma.push(' ');
            lemma.push_str(argument);
        }
        Ok(Proof {
            proof: format!("({} {} ({lemma}))", lib("conv_prim"), list.proof),
            fits: and(&list.fits, &fits),
        })
    }

    fn conditional(
        &mut self,
        whole: &SemanticTerm,
        condition: &SemanticTerm,
        then_value: &SemanticTerm,
        else_value: &SemanticTerm,
        ctx: &Ctx,
    ) -> Result<Proof, String> {
        let result = self.source.infer(whole, &ctx.sources(), &ctx.site)?;
        let encoder = self.enc(&result)?;
        let tested = self.prove(condition, ctx)?;
        let then_branch = self.prove(then_value, ctx)?;
        let else_branch = self.prove(else_value, ctx)?;
        let condition = self.src(condition, ctx)?;
        let (then_value, else_value) = (self.src(then_value, ctx)?, self.src(else_value, ctx)?);
        let hypothesis = ctx
            .hypotheses
            .get(&(std::ptr::from_ref(whole) as usize))
            .map(|index| hypothesis(*index));
        let select = |scrutinee: &str, on_true: &str, on_false: &str| {
            match &hypothesis {
            Some(name) => format!(
                "(match (generalizing := false) {name} : {scrutinee} with | true => {on_true} | false => {on_false})"
            ),
            None => format!("(if {scrutinee} then {on_true} else {on_false})"),
        }
        };
        let binder = hypothesis.clone().unwrap_or_else(|| "_".to_owned());
        Ok(Proof {
            proof: format!(
                "({} (fun (__b : Bool) => {}) (fun (__b : Bool) => {encoder} {}) {condition} {} (fun {binder} => {}) (fun {binder} => {}))",
                lib("conv_cond"),
                select("__b", &then_branch.fits, &else_branch.fits),
                select("__b", &then_value, &else_value),
                tested.proof,
                then_branch.proof,
                else_branch.proof
            ),
            fits: and(
                &tested.fits,
                &select(&condition, &then_branch.fits, &else_branch.fits),
            ),
        })
    }

    fn matching(
        &mut self,
        whole: &SemanticTerm,
        scrutinee: &SemanticTerm,
        branches: &[SemanticBranch],
        ctx: &Ctx,
    ) -> Result<Proof, String> {
        let site = ctx.site.clone();
        let scrutinee_ty = self.source.infer(scrutinee, &ctx.sources(), &site)?;
        let result = self.source.infer(whole, &ctx.sources(), &site)?;
        let (scrutinee_encoder, encoder, lean_ty, result_ty) = (
            self.enc(&scrutinee_ty)?,
            self.enc(&result)?,
            self.ty(&scrutinee_ty)?,
            self.ty(&result)?,
        );
        let tested = self.prove(scrutinee, ctx)?;
        let hypothesis = ctx
            .hypotheses
            .get(&(std::ptr::from_ref(whole) as usize))
            .map(|index| hypothesis(*index));
        let mut fits_alternatives = Vec::new();
        let mut value_alternatives = Vec::new();
        let mut proof_arms = String::new();
        for (position, branch) in branches.iter().enumerate() {
            let inner = ctx.with(self.branch_locals(branch, scrutinee, ctx)?);
            let body = self.prove(&branch.body, &inner)?;
            let pattern = self.pattern(branch, &site)?;
            fits_alternatives.push(body.fits);
            value_alternatives.push(self.src(&branch.body, &inner)?);
            let mut arm = format!("({} rfl rfl {})", lib("convA_hit"), body.proof);
            for _ in 0..position {
                arm = format!("({} rfl {arm})", lib("convA_miss"));
            }
            match &hypothesis {
                Some(name) => proof_arms.push_str(&format!(" | {pattern}, {name} => {arm}")),
                None => proof_arms.push_str(&format!(" | {pattern} => {arm}")),
            }
        }
        let scrutinee_src = self.src(scrutinee, ctx)?;
        let fits_at = |gen: &Self, discriminant: &str| {
            gen.select(
                ctx,
                scrutinee,
                discriminant,
                branches,
                &fits_alternatives,
                "Bool",
                hypothesis.as_deref(),
            )
        };
        let value_at = |gen: &Self, discriminant: &str| {
            gen.select(
                ctx,
                scrutinee,
                discriminant,
                branches,
                &value_alternatives,
                &result_ty,
                hypothesis.as_deref(),
            )
        };
        let cases = match &hypothesis {
            Some(name) => {
                format!("(fun (__a : {lean_ty}) {name} => match __a, {name} with{proof_arms})")
            }
            None => format!("(fun (__a : {lean_ty}) _ => match __a with{proof_arms})"),
        };
        Ok(Proof {
            proof: format!(
                "({} {scrutinee_encoder} (fun (__a : {lean_ty}) => {}) (fun (__a : {lean_ty}) => {encoder} {}) {scrutinee_src} {} {cases})",
                lib("conv_matchV"),
                fits_at(self, "__a")?,
                value_at(self, "__a")?,
                tested.proof
            ),
            fits: and(&tested.fits, &fits_at(self, &scrutinee_src)?),
        })
    }

    fn application(
        &mut self,
        function: &SemanticTerm,
        arguments: &[SemanticTerm],
        ctx: &Ctx,
    ) -> Result<Proof, String> {
        // The function is evaluated before its arguments (§17.14).
        let (target, relation, fits) = match function {
            SemanticTerm::Var { name } => {
                let companion = ctx
                    .lookup(name)?
                    .companion
                    .clone()
                    .ok_or_else(|| format!("applied `{name}` has no companion"))?;
                (Self::conv_var(), companion.relation, companion.fits)
            }
            SemanticTerm::Lambda {
                parameters: _,
                captures,
                body: _,
            } => {
                let index = self.next_lambda(ctx.function)?;
                let mut fits = format!("__fits_{index}");
                let mut relation = format!("__rel_{index}");
                let mut proofs = Vec::new();
                for capture in captures {
                    let local = ctx.lookup(capture)?;
                    let name = identifier(capture);
                    match &local.companion {
                        Some(companion) => {
                            fits.push_str(&format!(" ({name}) ({})", companion.fits));
                            relation.push_str(&format!(
                                " ({name}) ({}) ({}) ({}) ({})",
                                companion.fits,
                                companion.index,
                                companion.captures,
                                companion.relation
                            ));
                        }
                        None => {
                            fits.push_str(&format!(" ({name})"));
                            relation.push_str(&format!(" ({name})"));
                        }
                    }
                    proofs.push(Self::conv_var());
                }
                let captured = operands(proofs);
                (
                    Proof {
                        proof: format!("({} {})", lib("conv_closure"), captured.proof),
                        fits: captured.fits,
                    },
                    format!("({relation})"),
                    format!("({fits})"),
                )
            }
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                let module = Source::module_of(function, &ctx.site);
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.source.close(argument, &ctx.site))
                    .collect();
                let index = self.instance_index(&module, &function.name, &closed)?;
                (
                    Proof {
                        proof: format!("({} {})", lib("conv_closure"), lib("convL_nil")),
                        fits: "true".to_owned(),
                    },
                    format!("__rel_{index}"),
                    format!("__fits_{index}"),
                )
            }
            SemanticTerm::Nat { value: _ }
            | SemanticTerm::Integer {
                representation: _,
                value: _,
            }
            | SemanticTerm::String { value: _ }
            | SemanticTerm::Bytes { hex: _ }
            | SemanticTerm::Primitive {
                operation: _,
                arguments: _,
                result: _,
            }
            | SemanticTerm::Bool { value: _ }
            | SemanticTerm::Unit
            | SemanticTerm::Nil { element: _ }
            | SemanticTerm::Cons { head: _, tail: _ }
            | SemanticTerm::Record {
                r#type: _,
                type_arguments: _,
                fields: _,
            }
            | SemanticTerm::Constructor {
                constructor: _,
                type_arguments: _,
                arguments: _,
            }
            | SemanticTerm::InstanceValue {
                class: _,
                arguments: _,
                resolved: _,
            }
            | SemanticTerm::Project { value: _, field: _ }
            | SemanticTerm::Call {
                function: _,
                type_arguments: _,
                arguments: _,
            }
            | SemanticTerm::If {
                condition: _,
                then_value: _,
                else_value: _,
            }
            | SemanticTerm::Match {
                scrutinee: _,
                branches: _,
            }
            | SemanticTerm::Eq { left: _, right: _ }
            | SemanticTerm::Le { left: _, right: _ }
            | SemanticTerm::Lt { left: _, right: _ }
            | SemanticTerm::Add { left: _, right: _ }
            | SemanticTerm::Beq { left: _, right: _ }
            | SemanticTerm::Ble { left: _, right: _ }
            | SemanticTerm::Blt { left: _, right: _ }
            | SemanticTerm::And { left: _, right: _ }
            | SemanticTerm::PropAnd { left: _, right: _ }
            | SemanticTerm::Or { left: _, right: _ }
            | SemanticTerm::Not { value: _ }
            | SemanticTerm::Implies {
                premise: _,
                conclusion: _,
            }
            | SemanticTerm::Iff { left: _, right: _ }
            | SemanticTerm::Forall { binder: _, body: _ }
            | SemanticTerm::Let {
                binder: _,
                value: _,
                body: _,
            }
            | SemanticTerm::Pair { left: _, right: _ }
            | SemanticTerm::First { value: _ }
            | SemanticTerm::Second { value: _ }
            | SemanticTerm::Apply {
                function: _,
                arguments: _,
            }
            | SemanticTerm::MapLiteral {
                key: _,
                value: _,
                entries: _,
            }
            | SemanticTerm::SetLiteral {
                element: _,
                elements: _,
            }
            | SemanticTerm::GraphLiteral {
                node: _,
                nodes: _,
                edges: _,
            } => {
                return Err(format!(
                    "the applied `{}` is neither a parameter, a lambda, nor a function reference",
                    super::eligibility::term_key(function)
                ));
            }
        };
        let list = operands(self.proofs(arguments, ctx)?);
        let sources = self.src_terms(arguments, ctx)?;
        Ok(Proof {
            proof: format!(
                "({} {} {} ({relation}{sources}))",
                lib("conv_apply"),
                target.proof,
                list.proof
            ),
            fits: and(
                &target.fits,
                &and(&list.fits, &format!("({fits}{sources})")),
            ),
        })
    }
}

/// The binders, arguments, and statement pieces of one function's relation.
struct Signature {
    /// Binders of the fits predicate.
    fits_binders: Vec<String>,
    /// Binders of the relation theorem.
    rel_binders: Vec<String>,
    /// The fits predicate applied to the binders.
    fits_applied: String,
    /// The encoded argument list the function is called with.
    arguments: String,
    /// The encoded denotation of the call.
    value: String,
    /// The pattern binder names, one per binder, for equations.
    names: Vec<String>,
}

impl Gen<'_> {
    /// A scope binding `locals` in order, function-typed ones with companions
    /// named by `stem` and position.
    fn scope(locals: &[(Local, SemanticType)], stem: &str) -> Vec<CLocal> {
        locals
            .iter()
            .enumerate()
            .map(|(position, (local, generic))| CLocal {
                local: local.clone(),
                generic: generic.clone(),
                companion: function_type(&local.ty)
                    .map(|_| companion_names(&format!("{stem}{position}"))),
            })
            .collect()
    }

    /// The binders and arguments of a function over `locals`, whose
    /// denotation applied to them is `denotation`.
    fn signature(
        &self,
        index: u64,
        locals: &[CLocal],
        result: &SemanticType,
        denotation: &str,
    ) -> Result<Signature, String> {
        let mut fits_binders = Vec::new();
        let mut rel_binders = Vec::new();
        let mut fits_applied = format!("__fits_{index}");
        let mut arguments = Vec::new();
        let mut names = Vec::new();
        for local in locals {
            let name = identifier(&local.local.name);
            let ty = self.ty(&local.local.ty)?;
            fits_binders.push(format!("({name} : {ty})"));
            rel_binders.push(format!("({name} : {ty})"));
            fits_applied.push_str(&format!(" {name}"));
            names.push(name.clone());
            match &local.companion {
                Some(companion) => {
                    let (parameters, codomain) =
                        function_type(&local.local.ty).ok_or("a companion of a non-function")?;
                    let mut domain = String::new();
                    let mut binders = String::new();
                    let mut applied = String::new();
                    let mut encoded = Vec::new();
                    for (position, parameter) in parameters.iter().enumerate() {
                        let lean = self.ty(parameter)?;
                        domain.push_str(&format!("{lean} -> "));
                        binders.push_str(&format!(" (__p{position} : {lean})"));
                        applied.push_str(&format!(" __p{position}"));
                        encoded.push(format!("({} __p{position})", self.enc(parameter)?));
                    }
                    fits_binders.push(format!("({} : {domain}Bool)", companion.fits));
                    rel_binders.push(format!("({} : {domain}Bool)", companion.fits));
                    rel_binders.push(format!("({} : Nat)", companion.index));
                    rel_binders.push(format!("({} : List {SYNTAX}.Value)", companion.captures));
                    rel_binders.push(format!(
                        "({} : ∀{binders}, {} __prog {} ({SEMANTICS}.LexLeanRuntime.append {} [{}]) ({} ({}{applied}) ({} ({name}{applied}))))",
                        companion.relation,
                        lib("FunRel"),
                        companion.index,
                        companion.captures,
                        encoded.join(", "),
                        lib("Rel"),
                        companion.fits,
                        self.enc(codomain)?
                    ));
                    fits_applied.push_str(&format!(" {}", companion.fits));
                    names.push(companion.fits.clone());
                    names.push(companion.index.clone());
                    names.push(companion.captures.clone());
                    names.push(companion.relation.clone());
                }
                None => {}
            }
            arguments.push(self.encoded_local(local)?);
        }
        Ok(Signature {
            fits_binders,
            rel_binders,
            fits_applied,
            arguments: format!("[{}]", arguments.join(", ")),
            value: format!("({} ({denotation}))", self.enc(result)?),
            names,
        })
    }

    fn statement(&self, index: u64, signature: &Signature) -> String {
        format!(
            "{} __prog {index} {} ({} ({}) {})",
            lib("FunRel"),
            signature.arguments,
            lib("Rel"),
            signature.fits_applied,
            signature.value
        )
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
            .map(|linked| linked.semantic.declarations.as_slice())
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
                                    mutual: mutual.clone(),
                                },
                                Hypotheses::new(),
                            ),
                            None => (Recursion::None, Hypotheses::new()),
                        },
                    };
                    let ctx = Ctx::new(
                        site.clone(),
                        instance_parameters.clone(),
                        Self::scope(&locals, ""),
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
                } => {}
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
                locals.extend(Self::scope(parameters, "p"));
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
                let scrutinee_ty = self.source.infer(scrutinee, &ctx.sources(), &site)?;
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
                        captures: Self::scope(&captured, "c"),
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
        | SemanticType::Ordering => {}
    }
}

/// The encoder of a nested container outside its group's mutual block.
fn nested_enc(nested: &Nested) -> String {
    match &nested.ty {
        SemanticType::List { element: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::Map { key: _, value: _ } => {
            format!("({} __L_{})", lib("ListEnc.enc"), nested.index)
        }
        SemanticType::Option { value: _ } => {
            format!("({} __O_{})", lib("OptEnc.enc"), nested.index)
        }
        SemanticType::Result { ok: _, error: _ }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Function {
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
        | SemanticType::Ordering => format!("__aux_{}", nested.index),
    }
}

fn mutual_block(items: &[String]) -> String {
    if items.len() == 1 {
        return items[0].clone();
    }
    format!("mutual\n{}end\n\n", items.concat())
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
            | SemanticType::Ordering => {
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

    /// `arg` encoded inside a recursive group's mutual block, where a nested
    /// container's bundle is not yet defined and its auxiliary function is
    /// applied directly, as structural recursion requires.
    fn raw_apply(&self, ty: &SemanticType, arg: &str) -> Result<String, String> {
        match self.nested.get(&self.source.type_text(ty)) {
            Some(nested) => Ok(match &nested.ty {
                SemanticType::List { element: _ }
                | SemanticType::Set { element: _ }
                | SemanticType::Map { key: _, value: _ } => {
                    format!("({} (__items_{} {arg}))", value_ctor("list"), nested.index)
                }
                SemanticType::Option { value: _ }
                | SemanticType::Result { ok: _, error: _ }
                | SemanticType::Product { left: _, right: _ }
                | SemanticType::Named {
                    member: _,
                    arguments: _,
                }
                | SemanticType::Function {
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
                | SemanticType::Ordering => format!("(__aux_{} {arg})", nested.index),
            }),
            None => Ok(format!("({} {arg})", self.enc(ty)?)),
        }
    }

    /// The list encoder bundle of a list, set or map type.
    fn list_bundle(&self, ty: &SemanticType) -> Result<String, String> {
        match self.nested.get(&self.source.type_text(ty)) {
            Some(nested) => Ok(format!("__L_{}", nested.index)),
            None => Ok(format!(
                "({} {})",
                lib("listEnc"),
                self.enc(&element_of(ty)?)?
            )),
        }
    }

    /// The option encoder bundle of an option type.
    fn option_bundle(&self, ty: &SemanticType) -> Result<String, String> {
        match self.nested.get(&self.source.type_text(ty)) {
            Some(nested) => Ok(format!("__O_{}", nested.index)),
            None => Ok(format!(
                "({} {})",
                lib("optEnc"),
                self.enc(&option_value(ty)?)?
            )),
        }
    }

    /// The auxiliary function of a nested container, and its bundle when the
    /// container is a list or an option.
    fn auxiliary(&self, nested: &Nested) -> Result<(String, String), String> {
        let lean = self.ty(&nested.ty)?;
        let n = nested.index;
        let value = format!("{SYNTAX}.Value");
        Ok(match &nested.ty {
            SemanticType::List { element: _ }
            | SemanticType::Set { element: _ }
            | SemanticType::Map { key: _, value: _ } => {
                let element = element_of(&nested.ty)?;
                (
                    format!(
                        "def __items_{n} : {lean} -> List {value}\n  | [] => []\n  | __x0 :: __x1 => {} :: __items_{n} __x1\n\n",
                        self.raw_apply(&element, "__x0")?
                    ),
                    format!(
                        "def __L_{n} : {} {} :=\n  ⟨{}, __items_{n}, __items_{n}.eq_1, __items_{n}.eq_2⟩\n\n",
                        lib("ListEnc"),
                        self.ty(&element)?,
                        self.enc(&element)?
                    ),
                )
            }
            SemanticType::Option { value: inner } => (
                format!(
                    "def __aux_{n} : {lean} -> {value}\n  | none => {}\n  | some __x0 => {} {}\n\n",
                    value_ctor("none"),
                    value_ctor("some"),
                    self.raw_apply(inner, "__x0")?
                ),
                format!(
                    "def __O_{n} : {} {} :=\n  ⟨{}, __aux_{n}, __aux_{n}.eq_1, __aux_{n}.eq_2⟩\n\n",
                    lib("OptEnc"),
                    self.ty(inner)?,
                    self.enc(inner)?
                ),
            ),
            SemanticType::Product { left, right } => (
                format!(
                    "def __aux_{n} : {lean} -> {value}\n  | (__x0, __x1) => {} {} {}\n\n",
                    value_ctor("pair"),
                    self.raw_apply(left, "__x0")?,
                    self.raw_apply(right, "__x1")?
                ),
                String::new(),
            ),
            SemanticType::Result { ok, error } => (
                format!(
                    "def __aux_{n} : {lean} -> {value}\n  | Except.error __x0 => {} {}\n  | Except.ok __x0 => {} {}\n\n",
                    value_ctor("error"),
                    self.raw_apply(error, "__x0")?,
                    value_ctor("ok"),
                    self.raw_apply(ok, "__x0")?
                ),
                String::new(),
            ),
            SemanticType::Named {
                member: _,
                arguments: _,
            }
            | SemanticType::Function {
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
            | SemanticType::Ordering => {
                return Err(format!(
                    "`{}` is not a nested container",
                    self.source.type_text(&nested.ty)
                ));
            }
        })
    }

    /// The encoder definitions of every document type the program realizes,
    /// dependencies first; a mutual group of types gets one mutual block,
    /// joined by the auxiliary encoders of the containers nested in it.
    fn encoders(&self) -> Result<String, String> {
        let graph = self.adt_graph()?;
        let mut texts: BTreeMap<u64, String> = BTreeMap::new();
        for (index, adt) in self.layout.adts.iter().enumerate() {
            let index = index as u64;
            let shape = self.source.document(&adt.ty)?;
            let lean = self.ty(&adt.ty)?;
            let recursive = self.nested.values().any(|nested| nested.group == index)
                || components(&graph)
                    .iter()
                    .any(|component| component.len() > 1 && component.contains(&index))
                || graph
                    .get(&index)
                    .is_some_and(|edges| edges.contains(&index));
            let text = if shape.kind == "inductive" || recursive {
                let owner = self.ty(&adt.ty)?;
                let owner = owner
                    .trim_start_matches('(')
                    .split(' ')
                    .next()
                    .unwrap_or_default()
                    .trim_end_matches(')')
                    .to_owned();
                let mut text = format!("def __enc_{index} : {lean} -> {SYNTAX}.Value\n");
                for (position, (constructor, fields)) in
                    shape.names.iter().zip(&shape.fields).enumerate()
                {
                    let binders: Vec<String> = (0..fields.len())
                        .map(|field| format!("__x{field}"))
                        .collect();
                    let encoded = fields
                        .iter()
                        .zip(&binders)
                        .map(|(field, binder)| self.raw_apply(field, binder))
                        .collect::<Result<Vec<_>, _>>()?;
                    let pattern = if shape.kind == "inductive" {
                        format!(
                            "{owner}.{}{}",
                            identifier(constructor),
                            binders
                                .iter()
                                .map(|binder| format!(" {binder}"))
                                .collect::<String>()
                        )
                    } else {
                        format!("⟨{}⟩", binders.join(", "))
                    };
                    text.push_str(&format!(
                        "  | {pattern} => {} {position} [{}]\n",
                        value_ctor("adt"),
                        encoded.join(", ")
                    ));
                }
                text.push('\n');
                text
            } else {
                let fields = shape
                    .field_names
                    .iter()
                    .zip(&shape.fields[0])
                    .map(|(name, ty)| {
                        Ok::<String, String>(format!(
                            "({} (__s).{})",
                            self.enc(ty)?,
                            identifier(name)
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                format!(
                    "def __enc_{index} (__s : {lean}) : {SYNTAX}.Value :=\n  {} 0 [{}]\n\n",
                    value_ctor("adt"),
                    fields.join(", ")
                )
            };
            texts.insert(index, text);
        }
        let mut out = String::new();
        for component in components(&graph) {
            let mut items: Vec<String> =
                component.iter().map(|index| texts[index].clone()).collect();
            let mut bundles = String::new();
            let mut nested: Vec<&Nested> = self
                .nested
                .values()
                .filter(|nested| nested.group == component[0])
                .collect();
            nested.sort_by_key(|nested| nested.index);
            for nested in nested {
                let (auxiliary, bundle) = self.auxiliary(nested)?;
                items.push(auxiliary);
                bundles.push_str(&bundle);
            }
            out.push_str(&mutual_block(&items));
            out.push_str(&bundles);
        }
        Ok(out)
    }

    /// The decreasing proof of a well-founded instance: exactly the source's
    /// evidence applications at the instance's types.
    fn decreasing(
        &self,
        module: &str,
        name: &str,
        type_arguments: &[SemanticType],
        group: &BTreeSet<String>,
    ) -> Result<String, String> {
        let (type_parameters, parameters, body, termination) =
            match self.source.declaration(module, name) {
                Some(SemanticDeclaration::Definition {
                    name: _,
                    type_parameters,
                    parameters,
                    result: _,
                    recursive_argument: _,
                    body,
                    axioms: _,
                    executable: _,
                    mutual: _,
                    termination: Some(termination),
                    production: _,
                }) => (type_parameters, parameters, body, termination),
                Some(
                    SemanticDeclaration::Definition {
                        name: _,
                        type_parameters: _,
                        parameters: _,
                        result: _,
                        recursive_argument: _,
                        body: _,
                        axioms: _,
                        executable: _,
                        mutual: _,
                        termination: None,
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
                    },
                )
                | None => return Err(format!("`{module}.{name}` has no termination evidence")),
            };
        let plan =
            crate::ir::semantic::well_founded_plan(name, group, type_parameters, parameters, body)?;
        let site = Site {
            module: module.to_owned(),
            substitution: BTreeMap::new(),
        };
        let mut text = "decreasing_by all_goals first".to_owned();
        for (call, evidence) in plan.sites.iter().zip(&termination.evidence) {
            let mut application =
                format!(" | (have __evidence := {}", self.member(evidence, &site)?);
            for argument in type_arguments {
                application.push_str(&format!(" {}", self.ty(argument)?));
            }
            for parameter in parameters {
                application.push_str(&format!(" ({})", identifier(&parameter.name)));
            }
            for _ in plan.binders(call) {
                application.push_str(" _");
            }
            for step in &call.path {
                application.push_str(&format!(" ({})", hypothesis(step.node())));
            }
            application.push_str("; subst_vars; exact __evidence)");
            text.push_str(&application);
        }
        text.push('\n');
        Ok(text)
    }
}

impl<'a> Gen<'a> {
    fn module(
        &mut self,
        root_module: &str,
        root_name: &str,
        report: &RootReport,
        name: &str,
    ) -> Result<String, String> {
        let graph = self.collect()?;
        let mut out = String::new();
        for module in LIBRARY_MODULES {
            out.push_str(&format!("import {module}\n"));
        }
        let mut imports: Vec<&str> = self
            .source
            .modules
            .values()
            .map(|linked| linked.lean_module)
            .collect();
        imports.sort_unstable();
        for module in imports {
            out.push_str(&format!("import {module}\n"));
        }
        out.push_str("set_option autoImplicit false\n");
        out.push_str("set_option maxRecDepth 100000\n");
        out.push_str("set_option linter.unusedVariables false\n");
        out.push_str(&format!("namespace {name}\n\n"));
        out.push_str(&format!(
            "def __prog : {SYNTAX}.Program :=\n  {}\n\n",
            render_program(self.program)?
        ));
        out.push_str(&self.encoders()?);
        self.found.clear();
        for component in components(&graph) {
            out.push_str(&self.component(&component, &graph)?);
        }
        // The root: function 0, run on its encoded arguments.
        let root_lean = self.source.lean_name(
            root_module,
            &MemberRef {
                module: Some(root_module.to_owned()),
                name: root_name.to_owned(),
            },
        );
        if root_lean != report.root {
            return Err(format!(
                "the certificate root `{root_lean}` is not the report's `{}`",
                report.root
            ));
        }
        let (parameters, result) = self.source.signature(root_module, root_name, &[])?;
        let locals: Vec<(Local, SemanticType)> = parameters
            .into_iter()
            .map(|SemanticParameter { name, r#type }| {
                (
                    Local {
                        name,
                        ty: r#type.clone(),
                    },
                    r#type,
                )
            })
            .collect();
        let scope = Self::scope(&locals, "");
        let mut denotation = identifier(&root_lean);
        for (local, _) in &locals {
            denotation.push_str(&format!(" {}", identifier(&local.name)));
        }
        let signature = self.signature(0, &scope, &result, &denotation)?;
        out.push_str(&format!(
            "theorem root {} : {} __prog 0 {} ({} ({}) {}) :=\n  {} (__rel_0 {})\n\n",
            signature.rel_binders.join(" "),
            lib("RunConv"),
            signature.arguments,
            lib("Rel"),
            signature.fits_applied,
            signature.value,
            lib("run_of_funRel"),
            signature.names.join(" ")
        ));
        out.push_str(&format!("end {name}\n"));
        Ok(out)
    }

    /// The denotation of a definition instance applied to `locals`.
    fn denotation(
        &self,
        module: &str,
        name: &str,
        type_arguments: &[SemanticType],
        locals: &[CLocal],
    ) -> Result<String, String> {
        let mut out = identifier(&self.source.lean_name(
            module,
            &MemberRef {
                module: Some(module.to_owned()),
                name: name.to_owned(),
            },
        ));
        for argument in type_arguments {
            out.push_str(&format!(" {}", self.ty(argument)?));
        }
        for local in locals {
            out.push_str(&format!(" {}", identifier(&local.local.name)));
        }
        Ok(out)
    }

    /// One strongly connected component of the relation graph: a single
    /// function, or a recursive group emitted as mutual blocks.
    #[allow(clippy::too_many_lines)]
    fn component(
        &mut self,
        component: &[u64],
        graph: &BTreeMap<u64, BTreeSet<u64>>,
    ) -> Result<String, String> {
        let recursive = component.len() > 1
            || graph
                .get(&component[0])
                .is_some_and(|edges| edges.contains(&component[0]));
        let mut fits_items = Vec::new();
        let mut rel_items = Vec::new();
        for index in component {
            let subject = self
                .subjects
                .get(index)
                .cloned()
                .ok_or_else(|| format!("function {index} has no subject"))?;
            let (fits, rel) = match subject {
                Subject::Definition {
                    module,
                    name,
                    type_arguments,
                    locals,
                    result,
                    body,
                    recursion,
                    site,
                    type_parameters,
                } => {
                    let scope = Self::scope(&locals, "");
                    let denotation = self.denotation(&module, &name, &type_arguments, &scope)?;
                    let signature = self.signature(*index, &scope, &result, &denotation)?;
                    let statement = self.statement(*index, &signature);
                    match recursion {
                        Recursion::None => {
                            if recursive {
                                return Err(format!(
                                    "`{module}.{name}` recurses without recursion evidence"
                                ));
                            }
                            let ctx =
                                Ctx::new(site, type_parameters, scope, Hypotheses::new(), *index);
                            let body = self.prove(body, &ctx)?;
                            (
                                format!(
                                    "def __fits_{index} {} : Bool :=\n  {}\n\n",
                                    signature.fits_binders.join(" "),
                                    body.fits
                                ),
                                format!(
                                    "theorem __rel_{index} {} : {statement} :=\n  {} rfl rfl {}\n\n",
                                    signature.rel_binders.join(" "),
                                    lib("funRel_intro"),
                                    body.proof
                                ),
                            )
                        }
                        Recursion::Structural { argument, mutual } => {
                            let (scrutinee, branches) = match body {
                                SemanticTerm::Match {
                                    scrutinee,
                                    branches,
                                } => (scrutinee, branches),
                                SemanticTerm::Var { name: _ }
                                | SemanticTerm::Nat { value: _ }
                                | SemanticTerm::Integer {
                                    representation: _,
                                    value: _,
                                }
                                | SemanticTerm::String { value: _ }
                                | SemanticTerm::Bytes { hex: _ }
                                | SemanticTerm::Primitive {
                                    operation: _,
                                    arguments: _,
                                    result: _,
                                }
                                | SemanticTerm::Bool { value: _ }
                                | SemanticTerm::Unit
                                | SemanticTerm::Nil { element: _ }
                                | SemanticTerm::Cons { head: _, tail: _ }
                                | SemanticTerm::Record {
                                    r#type: _,
                                    type_arguments: _,
                                    fields: _,
                                }
                                | SemanticTerm::Constructor {
                                    constructor: _,
                                    type_arguments: _,
                                    arguments: _,
                                }
                                | SemanticTerm::InstanceValue {
                                    class: _,
                                    arguments: _,
                                    resolved: _,
                                }
                                | SemanticTerm::Project { value: _, field: _ }
                                | SemanticTerm::Call {
                                    function: _,
                                    type_arguments: _,
                                    arguments: _,
                                }
                                | SemanticTerm::If {
                                    condition: _,
                                    then_value: _,
                                    else_value: _,
                                }
                                | SemanticTerm::Eq { left: _, right: _ }
                                | SemanticTerm::Le { left: _, right: _ }
                                | SemanticTerm::Lt { left: _, right: _ }
                                | SemanticTerm::Add { left: _, right: _ }
                                | SemanticTerm::Beq { left: _, right: _ }
                                | SemanticTerm::Ble { left: _, right: _ }
                                | SemanticTerm::Blt { left: _, right: _ }
                                | SemanticTerm::And { left: _, right: _ }
                                | SemanticTerm::PropAnd { left: _, right: _ }
                                | SemanticTerm::Or { left: _, right: _ }
                                | SemanticTerm::Not { value: _ }
                                | SemanticTerm::Implies {
                                    premise: _,
                                    conclusion: _,
                                }
                                | SemanticTerm::Iff { left: _, right: _ }
                                | SemanticTerm::Forall { binder: _, body: _ }
                                | SemanticTerm::Let {
                                    binder: _,
                                    value: _,
                                    body: _,
                                }
                                | SemanticTerm::Pair { left: _, right: _ }
                                | SemanticTerm::First { value: _ }
                                | SemanticTerm::Second { value: _ }
                                | SemanticTerm::Lambda {
                                    parameters: _,
                                    captures: _,
                                    body: _,
                                }
                                | SemanticTerm::Apply {
                                    function: _,
                                    arguments: _,
                                }
                                | SemanticTerm::FunctionRef {
                                    function: _,
                                    type_arguments: _,
                                }
                                | SemanticTerm::MapLiteral {
                                    key: _,
                                    value: _,
                                    entries: _,
                                }
                                | SemanticTerm::SetLiteral {
                                    element: _,
                                    elements: _,
                                }
                                | SemanticTerm::GraphLiteral {
                                    node: _,
                                    nodes: _,
                                    edges: _,
                                } => {
                                    return Err(format!(
                                        "structural `{module}.{name}` is not a top-level match"
                                    ));
                                }
                            };
                            let scrutinee_is_argument = match scrutinee.as_ref() {
                                SemanticTerm::Var { name } => name == &argument,
                                SemanticTerm::Nat { value: _ }
                                | SemanticTerm::Integer {
                                    representation: _,
                                    value: _,
                                }
                                | SemanticTerm::String { value: _ }
                                | SemanticTerm::Bytes { hex: _ }
                                | SemanticTerm::Primitive {
                                    operation: _,
                                    arguments: _,
                                    result: _,
                                }
                                | SemanticTerm::Bool { value: _ }
                                | SemanticTerm::Unit
                                | SemanticTerm::Nil { element: _ }
                                | SemanticTerm::Cons { head: _, tail: _ }
                                | SemanticTerm::Record {
                                    r#type: _,
                                    type_arguments: _,
                                    fields: _,
                                }
                                | SemanticTerm::Constructor {
                                    constructor: _,
                                    type_arguments: _,
                                    arguments: _,
                                }
                                | SemanticTerm::InstanceValue {
                                    class: _,
                                    arguments: _,
                                    resolved: _,
                                }
                                | SemanticTerm::Project { value: _, field: _ }
                                | SemanticTerm::Call {
                                    function: _,
                                    type_arguments: _,
                                    arguments: _,
                                }
                                | SemanticTerm::If {
                                    condition: _,
                                    then_value: _,
                                    else_value: _,
                                }
                                | SemanticTerm::Match {
                                    scrutinee: _,
                                    branches: _,
                                }
                                | SemanticTerm::Eq { left: _, right: _ }
                                | SemanticTerm::Le { left: _, right: _ }
                                | SemanticTerm::Lt { left: _, right: _ }
                                | SemanticTerm::Add { left: _, right: _ }
                                | SemanticTerm::Beq { left: _, right: _ }
                                | SemanticTerm::Ble { left: _, right: _ }
                                | SemanticTerm::Blt { left: _, right: _ }
                                | SemanticTerm::And { left: _, right: _ }
                                | SemanticTerm::PropAnd { left: _, right: _ }
                                | SemanticTerm::Or { left: _, right: _ }
                                | SemanticTerm::Not { value: _ }
                                | SemanticTerm::Implies {
                                    premise: _,
                                    conclusion: _,
                                }
                                | SemanticTerm::Iff { left: _, right: _ }
                                | SemanticTerm::Forall { binder: _, body: _ }
                                | SemanticTerm::Let {
                                    binder: _,
                                    value: _,
                                    body: _,
                                }
                                | SemanticTerm::Pair { left: _, right: _ }
                                | SemanticTerm::First { value: _ }
                                | SemanticTerm::Second { value: _ }
                                | SemanticTerm::Lambda {
                                    parameters: _,
                                    captures: _,
                                    body: _,
                                }
                                | SemanticTerm::Apply {
                                    function: _,
                                    arguments: _,
                                }
                                | SemanticTerm::FunctionRef {
                                    function: _,
                                    type_arguments: _,
                                }
                                | SemanticTerm::MapLiteral {
                                    key: _,
                                    value: _,
                                    entries: _,
                                }
                                | SemanticTerm::SetLiteral {
                                    element: _,
                                    elements: _,
                                }
                                | SemanticTerm::GraphLiteral {
                                    node: _,
                                    nodes: _,
                                    edges: _,
                                } => false,
                            };
                            if !scrutinee_is_argument {
                                return Err(format!(
                                    "structural `{module}.{name}` does not match on `{argument}`"
                                ));
                            }
                            let position = locals
                                .iter()
                                .position(|(local, _)| local.name == argument)
                                .ok_or_else(|| format!("`{argument}` is not a parameter"))?;
                            let argument_ty = locals[position].0.ty.clone();
                            let argument_generic = locals[position].1.clone();
                            let generic_site = Site {
                                module: site.module.clone(),
                                substitution: BTreeMap::new(),
                            };
                            let mut fits_equations = String::new();
                            let mut rel_equations = String::new();
                            for (branch_index, branch) in branches.iter().enumerate() {
                                let (_, fields) =
                                    self.source.branch(branch, &argument_ty, &site)?;
                                let (_, generic_fields) =
                                    self.source
                                        .branch(branch, &argument_generic, &generic_site)?;
                                let mut ctx_locals: Vec<CLocal> = scope
                                    .iter()
                                    .filter(|local| local.local.name != argument)
                                    .cloned()
                                    .collect();
                                ctx_locals.extend(
                                    branch
                                        .binders
                                        .iter()
                                        .zip(fields.into_iter().zip(generic_fields))
                                        .map(|(name, (ty, generic))| CLocal {
                                            local: Local {
                                                name: name.clone(),
                                                ty,
                                            },
                                            generic,
                                            companion: None,
                                        }),
                                );
                                let ctx = Ctx::new(
                                    site.clone(),
                                    type_parameters.clone(),
                                    ctx_locals,
                                    Hypotheses::new(),
                                    *index,
                                );
                                let proof = self.prove(&branch.body, &ctx)?;
                                let pattern = format!("({})", self.pattern(branch, &site)?);
                                let patterns: Vec<String> = signature
                                    .names
                                    .iter()
                                    .map(|binder| {
                                        if binder == &identifier(&argument) {
                                            pattern.clone()
                                        } else {
                                            binder.clone()
                                        }
                                    })
                                    .collect();
                                let fits_patterns: Vec<String> = patterns
                                    .iter()
                                    .zip(&signature.names)
                                    .filter(|(_, binder)| {
                                        !binder.starts_with("__fi_")
                                            && !binder.starts_with("__fc_")
                                            && !binder.starts_with("__fh_")
                                    })
                                    .map(|(pattern, _)| pattern.clone())
                                    .collect();
                                fits_equations.push_str(&format!(
                                    "  | {} => (true && {})\n",
                                    fits_patterns.join(", "),
                                    proof.fits
                                ));
                                let mut arm =
                                    format!("({} rfl rfl {})", lib("convA_hit"), proof.proof);
                                for _ in 0..branch_index {
                                    arm = format!("({} rfl {arm})", lib("convA_miss"));
                                }
                                // The arm rewrites by the fits equation
                                // rather than leaving the unifier to unfold
                                // `__fits`: through a nested inductive Lean
                                // has no smart unfolding, and full unfolding
                                // also reduces the leading `true &&`, which
                                // misaligns the conjunction.
                                rel_equations.push_str(&format!(
                                    "  | {} => by rw [__fits_{index}.eq_def]; exact {} rfl rfl ({} ({} rfl) {arm})\n",
                                    patterns.join(", "),
                                    lib("funRel_intro"),
                                    lib("conv_match"),
                                    lib("conv_var")
                                ));
                            }
                            let structural = |names: &[String]| match &mutual {
                                Some(_) => format!(
                                    "termination_by structural {} => {}\n",
                                    names
                                        .iter()
                                        .map(|binder| {
                                            if binder == &identifier(&argument) {
                                                binder.clone()
                                            } else {
                                                "_".to_owned()
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                        .join(" "),
                                    identifier(&argument)
                                ),
                                None => String::new(),
                            };
                            let fits_names: Vec<String> = signature
                                .names
                                .iter()
                                .filter(|binder| {
                                    !binder.starts_with("__fi_")
                                        && !binder.starts_with("__fc_")
                                        && !binder.starts_with("__fh_")
                                })
                                .cloned()
                                .collect();
                            (
                                format!(
                                    "def __fits_{index} : ∀ {}, Bool\n{fits_equations}{}\n",
                                    signature.fits_binders.join(" "),
                                    structural(&fits_names)
                                ),
                                format!(
                                    "theorem __rel_{index} : ∀ {}, {statement}\n{rel_equations}{}\n",
                                    signature.rel_binders.join(" "),
                                    structural(&signature.names)
                                ),
                            )
                        }
                        Recursion::WellFounded { termination, group } => {
                            let declaration = self.source.declaration(&module, &name);
                            let (declared_type_parameters, source_parameters) = match declaration {
                                Some(SemanticDeclaration::Definition {
                                    name: _,
                                    type_parameters,
                                    parameters,
                                    result: _,
                                    recursive_argument: _,
                                    body: _,
                                    axioms: _,
                                    executable: _,
                                    mutual: _,
                                    termination: _,
                                    production: _,
                                }) => (type_parameters, parameters),
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
                                    },
                                )
                                | None => {
                                    return Err(format!("`{module}.{name}` is not a definition"))
                                }
                            };
                            let hypotheses = Self::hypotheses(
                                &name,
                                &group,
                                declared_type_parameters,
                                source_parameters,
                                body,
                            )?;
                            let ctx = Ctx::new(
                                site.clone(),
                                type_parameters.clone(),
                                scope.clone(),
                                hypotheses,
                                *index,
                            );
                            let proof = self.prove(body, &ctx)?;
                            let measure_ctx = Ctx::new(
                                site.clone(),
                                type_parameters.clone(),
                                scope.clone(),
                                Hypotheses::new(),
                                *index,
                            );
                            let measure = self.src(&termination.measure, &measure_ctx)?;
                            let decreasing =
                                self.decreasing(&module, &name, &type_arguments, &group)?;
                            (
                                format!(
                                    "def __fits_{index} {} : Bool :=\n  {}\ntermination_by {measure}\n{decreasing}\n",
                                    signature.fits_binders.join(" "),
                                    proof.fits
                                ),
                                format!(
                                    "theorem __rel_{index} {} : {statement} := by\n  rw [{}.eq_def, __fits_{index}.eq_def]\n  exact {} rfl rfl {}\ntermination_by {measure}\n{decreasing}\n",
                                    signature.rel_binders.join(" "),
                                    identifier(&self.source.lean_name(&module, &MemberRef {
                                        module: Some(module.clone()),
                                        name: name.clone(),
                                    })),
                                    lib("funRel_intro"),
                                    proof.proof
                                ),
                            )
                        }
                    }
                }
                Subject::Instance {
                    module,
                    name,
                    class,
                    fields,
                } => {
                    if recursive {
                        return Err(format!("instance `{module}.{name}` is recursive"));
                    }
                    let ctx = Ctx::new(
                        Site {
                            module: module.clone(),
                            substitution: BTreeMap::new(),
                        },
                        Vec::new(),
                        Vec::new(),
                        Hypotheses::new(),
                        *index,
                    );
                    let record = self.record(&class, fields, &ctx)?;
                    let instance = identifier(&self.source.lean_name(
                        &module,
                        &MemberRef {
                            module: Some(module.clone()),
                            name: name.clone(),
                        },
                    ));
                    (
                        format!("def __fits_{index} : Bool :=\n  {}\n\n", record.fits),
                        format!(
                            "theorem __rel_{index} : {} __prog {index} [] ({} __fits_{index} ({} {instance})) :=\n  {} rfl rfl {}\n\n",
                            lib("FunRel"),
                            lib("Rel"),
                            self.enc(&class)?,
                            lib("funRel_intro"),
                            record.proof
                        ),
                    )
                }
                Subject::Lambda {
                    site: _,
                    captures: _,
                    parameters: _,
                    body,
                    hypotheses: _,
                    type_parameters: _,
                } => {
                    if recursive {
                        return Err(format!("lambda function {index} is recursive"));
                    }
                    let ctx = self.lambda_ctx(*index)?;
                    let result = self.source.infer(body, &ctx.sources(), &ctx.site)?;
                    let denotation = self.src(body, &ctx)?;
                    let signature = self.signature(*index, &ctx.locals, &result, &denotation)?;
                    let statement = self.statement(*index, &signature);
                    let proof = self.prove(body, &ctx)?;
                    (
                        format!(
                            "def __fits_{index} {} : Bool :=\n  {}\n\n",
                            signature.fits_binders.join(" "),
                            proof.fits
                        ),
                        format!(
                            "theorem __rel_{index} {} : {statement} :=\n  {} rfl rfl {}\n\n",
                            signature.rel_binders.join(" "),
                            lib("funRel_intro"),
                            proof.proof
                        ),
                    )
                }
            };
            fits_items.push(fits);
            rel_items.push(rel);
        }
        Ok(format!(
            "{}{}",
            mutual_block(&fits_items),
            mutual_block(&rel_items)
        ))
    }
}

/// A clause proof that unfolds a collection-runtime function once and
/// rewrites by the comparison hypothesis `h`, with `binders` wildcards
/// before it.
fn clause(binders: usize, function: &str) -> String {
    format!(
        "(fun{} h => by simp only [{function}, h])",
        " _".repeat(binders)
    )
}

/// A clause of the pair key order: the order's instances unfold in the
/// hypothesis and the goal alike before the hypothesis rewrites.
fn pair_clause(compare: &str) -> String {
    format!("(fun _ _ h => by simp only [{compare}] at h ⊢; simp only [h])")
}

/// A clause that holds by computation, over `binders` arguments.
fn computed(binders: usize) -> String {
    match binders {
        0 => "rfl".to_owned(),
        _ => format!("(fun{} => rfl)", " _".repeat(binders)),
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

    /// The source key order of `ty` in the collection runtime `coll`.
    fn key_order(&self, ty: &SemanticType, coll: &str) -> Result<String, String> {
        let lean = self.ty(ty)?;
        Ok(format!(
            "({coll}.Key.compare : {lean} -> {lean} -> Ordering)"
        ))
    }

    /// The proof that the realization's comparison agrees with the key
    /// order of `ty` in the collection runtime `coll`.
    fn key_spec(&self, ty: &SemanticType, coll: &str) -> Result<String, String> {
        let order = self.key_order(ty, coll)?;
        let scalar = |lemma: &str| format!("({} {order} (fun _ _ => rfl))", lib(lemma));
        Ok(match ty {
            SemanticType::Nat => scalar("keySpec_nat"),
            SemanticType::Int => scalar("keySpec_int"),
            SemanticType::Bool => scalar("keySpec_bool"),
            SemanticType::Int8 => scalar("keySpec_i8"),
            SemanticType::Int16 => scalar("keySpec_i16"),
            SemanticType::Int32 => scalar("keySpec_i32"),
            SemanticType::Int64 => scalar("keySpec_i64"),
            SemanticType::UInt8 => scalar("keySpec_u8"),
            SemanticType::UInt16 => scalar("keySpec_u16"),
            SemanticType::UInt32 => scalar("keySpec_u32"),
            SemanticType::UInt64 => scalar("keySpec_u64"),
            SemanticType::String => {
                let codes = format!("{coll}.compareCodes");
                format!(
                    "({} {codes} rfl (fun _ _ => rfl) (fun _ _ => rfl) {} {} {} {order} (fun _ _ => rfl))",
                    lib("keySpec_string"),
                    clause(4, &codes),
                    clause(4, &codes),
                    clause(4, &codes)
                )
            }
            SemanticType::Product { left, right } => {
                let compare = format!("{coll}.Key.compare");
                format!(
                    "({} {} {} {order} {} {} {})",
                    lib("keySpec_pair"),
                    self.key_spec(left, coll)?,
                    self.key_spec(right, coll)?,
                    pair_clause(&compare),
                    pair_clause(&compare),
                    pair_clause(&compare)
                )
            }
            SemanticType::Unit
            | SemanticType::Bytes
            | SemanticType::Ordering
            | SemanticType::Option { value: _ }
            | SemanticType::Result { ok: _, error: _ }
            | SemanticType::List { element: _ }
            | SemanticType::Set { element: _ }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::Named {
                member: _,
                arguments: _,
            }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ } => {
                return Err(format!("`{}` has no key order", self.source.type_text(ty)));
            }
        })
    }

    /// The set-runtime clauses of key type `ty`.
    fn set_ops(&self, ty: &SemanticType, coll: &str) -> Result<String, String> {
        let insert = format!("{coll}.insertElement");
        let remove = format!("{coll}.removeElement");
        let contains = format!("{coll}.containsElement");
        Ok(format!(
            "(⟨{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}⟩ : {} {} {insert} {remove} {contains})",
            computed(1),
            clause(3, &insert),
            clause(3, &insert),
            clause(3, &insert),
            computed(1),
            clause(3, &remove),
            clause(3, &remove),
            clause(3, &remove),
            computed(1),
            clause(3, &contains),
            clause(3, &contains),
            clause(3, &contains),
            lib("SetOps"),
            self.key_order(ty, coll)?
        ))
    }

    /// The lookup-runtime clauses of key type `key` and value type `value`.
    fn look_ops(
        &self,
        key: &SemanticType,
        value: &SemanticType,
        coll: &str,
    ) -> Result<String, String> {
        let lookup = format!("{coll}.lookupEntry");
        Ok(format!(
            "(⟨{}, {}, {}, {}⟩ : {} {} ({lookup} : {} -> List (Prod {} {}) -> Option {}))",
            computed(1),
            clause(4, &lookup),
            clause(4, &lookup),
            clause(4, &lookup),
            lib("LookOps"),
            self.key_order(key, coll)?,
            self.ty(key)?,
            self.ty(key)?,
            self.ty(value)?,
            self.ty(value)?
        ))
    }

    /// `n` function-placement hypotheses, each discharged by computation.
    fn placed(n: usize) -> String {
        vec!["rfl"; n].join(" ")
    }

    /// A call of a collection template: the call's operands, then the
    /// template's lemma instantiated at the call's runtime functions, whose
    /// defining clauses the certificate discharges by unfolding them.
    #[allow(clippy::too_many_lines)]
    fn template_call(
        &mut self,
        template: Template,
        instance: &[SemanticType],
        arguments: &[SemanticTerm],
        types: &[SemanticType],
        ctx: &Ctx,
    ) -> Result<Proof, String> {
        let tys = instance
            .iter()
            .map(|ty| self.target_ty(ty))
            .collect::<Result<Vec<_>, _>>()?;
        let entry = *self
            .templates
            .get(&(template.name().to_owned(), tys.clone()))
            .ok_or_else(|| format!("template {} has no instance in the layout", template.name()))?;
        let rendered: Vec<String> = tys.iter().map(render_ty).collect();
        let coll = format!("{}.LexLeanCollections", self.lean_module(&ctx.site.module)?);
        let mut proofs = Vec::new();
        let mut values = Vec::new();
        let mut functions = Vec::new();
        for argument in arguments {
            match self.function_argument(argument, ctx)? {
                Some(companion) => {
                    values.push(format!("({})", companion.value));
                    proofs.push(companion.proof.clone());
                    functions.push(companion);
                }
                None => {
                    values.push(format!("({})", self.src(argument, ctx)?));
                    proofs.push(self.prove(argument, ctx)?);
                }
            }
        }
        let value = |index: usize| -> Result<String, String> {
            values
                .get(index)
                .cloned()
                .ok_or_else(|| format!("template {} lacks operand {index}", template.name()))
        };
        let argument_type = |index: usize| -> Result<SemanticType, String> {
            types
                .get(index)
                .cloned()
                .ok_or_else(|| format!("template {} lacks operand {index}", template.name()))
        };
        let step = || -> Result<&FunctionArgument, String> {
            functions
                .first()
                .ok_or_else(|| format!("template {} lacks its step", template.name()))
        };
        let at = |names: &[&str]| -> String {
            names
                .iter()
                .zip(&rendered)
                .map(|(name, ty)| format!(" ({name} := {ty})"))
                .collect::<String>()
        };
        let head = format!("(p := __prog) (fi := {entry})");
        let insert_entry = format!("{coll}.insertEntry");
        let remove_entry = format!("{coll}.removeEntry");
        let lookup_entry = format!("{coll}.lookupEntry");
        let (lemma, fits) = match template {
            Template::MapInsert => {
                let key = instance_type(instance, 0)?;
                let stored = instance_type(instance, 1)?;
                (
                    format!(
                        "({} {} {} {} (fun _ => rfl) {insert_entry} {} {} {} {} {head}{} rfl {} {} {})",
                        lib("tpl_mapInsert"),
                        self.key_spec(&key, &coll)?,
                        self.enc(&stored)?,
                        self.list_bundle(&argument_type(0)?)?,
                        computed(2),
                        clause(5, &insert_entry),
                        clause(5, &insert_entry),
                        clause(5, &insert_entry),
                        at(&["tk", "tw"]),
                        value(0)?,
                        value(1)?,
                        value(2)?
                    ),
                    "true".to_owned(),
                )
            }
            Template::MapRemove => {
                let key = instance_type(instance, 0)?;
                let stored = instance_type(instance, 1)?;
                (
                    format!(
                        "({} {} {} {} (fun _ => rfl) {remove_entry} {} {} {} {} {head}{} rfl {} {})",
                        lib("tpl_mapRemove"),
                        self.key_spec(&key, &coll)?,
                        self.enc(&stored)?,
                        self.list_bundle(&argument_type(0)?)?,
                        computed(1),
                        clause(4, &remove_entry),
                        clause(4, &remove_entry),
                        clause(4, &remove_entry),
                        at(&["tk", "tw"]),
                        value(0)?,
                        value(1)?
                    ),
                    "true".to_owned(),
                )
            }
            Template::MapLookup => {
                let key = instance_type(instance, 0)?;
                let stored = instance_type(instance, 1)?;
                (
                    format!(
                        "({} {} {} {} (fun _ => rfl) {} (fun _ => rfl) {lookup_entry} {} {} {} {} {head}{} rfl {} {})",
                        lib("tpl_mapLookup"),
                        self.key_spec(&key, &coll)?,
                        self.enc(&stored)?,
                        self.list_bundle(&argument_type(0)?)?,
                        self.option_bundle(&SemanticType::Option {
                            value: Box::new(stored.clone()),
                        })?,
                        computed(1),
                        clause(4, &lookup_entry),
                        clause(4, &lookup_entry),
                        clause(4, &lookup_entry),
                        at(&["tk", "tw"]),
                        value(0)?,
                        value(1)?
                    ),
                    "true".to_owned(),
                )
            }
            Template::MapContains => {
                let key = instance_type(instance, 0)?;
                let stored = instance_type(instance, 1)?;
                (
                    format!(
                        "({} {} {} {} (fun _ => rfl) {lookup_entry} {} {} {} {} {head}{} {} {} {})",
                        lib("tpl_mapContains"),
                        self.key_spec(&key, &coll)?,
                        self.enc(&stored)?,
                        self.list_bundle(&argument_type(0)?)?,
                        computed(1),
                        clause(4, &lookup_entry),
                        clause(4, &lookup_entry),
                        clause(4, &lookup_entry),
                        at(&["tk", "tw"]),
                        Self::placed(2),
                        value(0)?,
                        value(1)?
                    ),
                    "true".to_owned(),
                )
            }
            Template::MapKeys | Template::MapValues => {
                let key = instance_type(instance, 0)?;
                let stored = instance_type(instance, 1)?;
                let (lemma, projected) = match template {
                    Template::MapKeys => ("tpl_mapKeys", key.clone()),
                    Template::MapValues => ("tpl_mapValues", stored.clone()),
                    Template::MapInsert
                    | Template::MapRemove
                    | Template::MapLookup
                    | Template::MapContains
                    | Template::MapFold
                    | Template::SetInsert
                    | Template::SetRemove
                    | Template::SetContains
                    | Template::SetUnion
                    | Template::SetIntersection
                    | Template::SetDifference
                    | Template::SetFold
                    | Template::ListFold
                    | Template::Iterate
                    | Template::IterateUntil
                    | Template::GraphSuccessors
                    | Template::GraphReachable
                    | Template::GraphTopological => {
                        return Err("a projection of a non-projection template".to_owned());
                    }
                };
                (
                    format!(
                        "({} (ek := {}) (ev := {}) {} (fun _ => rfl) {} (fun _ => rfl) {head}{} rfl {})",
                        lib(lemma),
                        self.enc(&key)?,
                        self.enc(&stored)?,
                        self.list_bundle(&argument_type(0)?)?,
                        self.list_bundle(&SemanticType::List {
                            element: Box::new(projected),
                        })?,
                        at(&["tk", "tw"]),
                        value(0)?
                    ),
                    "true".to_owned(),
                )
            }
            Template::MapFold => {
                let key = instance_type(instance, 0)?;
                let stored = instance_type(instance, 1)?;
                let state = instance_type(instance, 2)?;
                let step = step()?;
                (
                    format!(
                        "({} (ek := {}) (ev := {}) (es := {}) {} (fun _ => rfl) {} ({}) {} {} ({}) {head}{} rfl {} {})",
                        lib("tpl_mapFold"),
                        self.enc(&key)?,
                        self.enc(&stored)?,
                        self.enc(&state)?,
                        self.list_bundle(&argument_type(2)?)?,
                        value(0)?,
                        step.fits,
                        step.index,
                        step.captures,
                        step.relation,
                        at(&["tk", "tw", "ts"]),
                        value(1)?,
                        value(2)?
                    ),
                    format!(
                        "({} (fun __st __e => ({}) __st __e.1 __e.2) (fun __st __e => {} __st __e.1 __e.2) {} {})",
                        lib("foldFits"),
                        step.fits,
                        value(0)?,
                        value(1)?,
                        value(2)?
                    ),
                )
            }
            Template::SetInsert | Template::SetRemove | Template::SetContains => {
                let key = instance_type(instance, 0)?;
                let (lemma, function) = match template {
                    Template::SetInsert => ("tpl_setInsert", "insertElement"),
                    Template::SetRemove => ("tpl_setRemove", "removeElement"),
                    Template::SetContains => ("tpl_setContains", "containsElement"),
                    Template::MapInsert
                    | Template::MapRemove
                    | Template::MapLookup
                    | Template::MapContains
                    | Template::MapKeys
                    | Template::MapValues
                    | Template::MapFold
                    | Template::SetUnion
                    | Template::SetIntersection
                    | Template::SetDifference
                    | Template::SetFold
                    | Template::ListFold
                    | Template::Iterate
                    | Template::IterateUntil
                    | Template::GraphSuccessors
                    | Template::GraphReachable
                    | Template::GraphTopological => {
                        return Err("an element operation of a non-element template".to_owned());
                    }
                };
                let function = format!("{coll}.{function}");
                (
                    format!(
                        "({} {} {} (fun _ => rfl) {function} {} {} {} {} {head}{} rfl {} {})",
                        lib(lemma),
                        self.key_spec(&key, &coll)?,
                        self.list_bundle(&argument_type(0)?)?,
                        computed(1),
                        clause(3, &function),
                        clause(3, &function),
                        clause(3, &function),
                        at(&["tk"]),
                        value(0)?,
                        value(1)?
                    ),
                    "true".to_owned(),
                )
            }
            Template::SetUnion => {
                let key = instance_type(instance, 0)?;
                let insert = format!("{coll}.insertElement");
                (
                    format!(
                        "({} {} {} (fun _ => rfl) {insert} {} {} {} {} {head}{} {} {} {})",
                        lib("tpl_setUnion"),
                        self.key_spec(&key, &coll)?,
                        self.list_bundle(&argument_type(0)?)?,
                        computed(1),
                        clause(3, &insert),
                        clause(3, &insert),
                        clause(3, &insert),
                        at(&["tk"]),
                        Self::placed(2),
                        value(1)?,
                        value(0)?
                    ),
                    "true".to_owned(),
                )
            }
            Template::SetIntersection | Template::SetDifference => {
                let key = instance_type(instance, 0)?;
                let contains = format!("{coll}.containsElement");
                let lemma = match template {
                    Template::SetIntersection => "tpl_setIntersection",
                    Template::SetDifference => "tpl_setDifference",
                    Template::MapInsert
                    | Template::MapRemove
                    | Template::MapLookup
                    | Template::MapContains
                    | Template::MapKeys
                    | Template::MapValues
                    | Template::MapFold
                    | Template::SetInsert
                    | Template::SetRemove
                    | Template::SetContains
                    | Template::SetUnion
                    | Template::SetFold
                    | Template::ListFold
                    | Template::Iterate
                    | Template::IterateUntil
                    | Template::GraphSuccessors
                    | Template::GraphReachable
                    | Template::GraphTopological => {
                        return Err("a filter of a non-filter template".to_owned());
                    }
                };
                (
                    format!(
                        "({} {} {} (fun _ => rfl) {contains} {} {} {} {} {head}{} {} {} {})",
                        lib(lemma),
                        self.key_spec(&key, &coll)?,
                        self.list_bundle(&argument_type(0)?)?,
                        computed(1),
                        clause(3, &contains),
                        clause(3, &contains),
                        clause(3, &contains),
                        at(&["tk"]),
                        Self::placed(2),
                        value(0)?,
                        value(1)?
                    ),
                    "true".to_owned(),
                )
            }
            Template::SetFold | Template::ListFold => {
                let element = instance_type(instance, 0)?;
                let state = instance_type(instance, 1)?;
                let step = step()?;
                (
                    format!(
                        "({} (ea := {}) (es := {}) {} (fun _ => rfl) {} ({}) {} {} ({}) {head}{} rfl {} {})",
                        lib("tpl_listFold"),
                        self.enc(&element)?,
                        self.enc(&state)?,
                        self.list_bundle(&argument_type(2)?)?,
                        value(0)?,
                        step.fits,
                        step.index,
                        step.captures,
                        step.relation,
                        at(&["ta", "ts"]),
                        value(1)?,
                        value(2)?
                    ),
                    format!(
                        "({} ({}) {} {} {})",
                        lib("foldFits"),
                        step.fits,
                        value(0)?,
                        value(1)?,
                        value(2)?
                    ),
                )
            }
            Template::Iterate => {
                let state = instance_type(instance, 0)?;
                let step = step()?;
                (
                    format!(
                        "({} (es := {}) {} ({}) {} {} ({}) ({coll}.iterate {}) {} {} {head}{} rfl {} {})",
                        lib("tpl_iterate"),
                        self.enc(&state)?,
                        value(0)?,
                        step.fits,
                        step.index,
                        step.captures,
                        step.relation,
                        value(0)?,
                        computed(1),
                        computed(2),
                        at(&["ts"]),
                        value(1)?,
                        value(2)?
                    ),
                    format!(
                        "({} ({}) {} {} {})",
                        lib("iterateFits"),
                        step.fits,
                        value(0)?,
                        value(1)?,
                        value(2)?
                    ),
                )
            }
            Template::IterateUntil => {
                let state = instance_type(instance, 0)?;
                let step = step()?;
                let until = format!("{coll}.iterateUntil");
                (
                    format!(
                        "({} (es := {}) {} (fun _ => rfl) {} (fun _ => rfl) {} ({}) {} {} ({}) ({until} {}) {} {} {} {head}{} rfl {} {})",
                        lib("tpl_iterateUntil"),
                        self.enc(&state)?,
                        self.option_bundle(&SemanticType::Option {
                            value: Box::new(state.clone()),
                        })?,
                        self.enc(&SemanticType::Product {
                            left: Box::new(state.clone()),
                            right: Box::new(SemanticType::Bool),
                        })?,
                        value(0)?,
                        step.fits,
                        step.index,
                        step.captures,
                        step.relation,
                        value(0)?,
                        computed(1),
                        clause(2, &until),
                        clause(3, &until),
                        at(&["ts"]),
                        value(1)?,
                        value(2)?
                    ),
                    format!(
                        "({} ({}) {} {} {})",
                        lib("untilFits"),
                        step.fits,
                        value(0)?,
                        value(1)?,
                        value(2)?
                    ),
                )
            }
            Template::GraphSuccessors => {
                let node = instance_type(instance, 0)?;
                let nodes = SemanticType::List {
                    element: Box::new(node.clone()),
                };
                (
                    format!(
                        "({} {} {} {} (fun _ => rfl) {lookup_entry} {} {} {} {} {head}{} {} {} {})",
                        lib("tpl_graphSuccessors"),
                        self.key_spec(&node, &coll)?,
                        self.list_bundle(&nodes)?,
                        self.list_bundle(&argument_type(0)?)?,
                        computed(1),
                        clause(4, &lookup_entry),
                        clause(4, &lookup_entry),
                        clause(4, &lookup_entry),
                        at(&["tk"]),
                        Self::placed(2),
                        value(0)?,
                        value(1)?
                    ),
                    "true".to_owned(),
                )
            }
            Template::GraphReachable | Template::GraphTopological => {
                let node = instance_type(instance, 0)?;
                let nodes = SemanticType::List {
                    element: Box::new(node.clone()),
                };
                let graph = value(0)?;
                let successors = format!("({coll}.graphSuccessors {graph})");
                let common = format!(
                    "{} {} (fun _ => rfl) {} (fun _ => rfl)",
                    self.key_spec(&node, &coll)?,
                    self.list_bundle(&nodes)?,
                    self.list_bundle(&argument_type(0)?)?,
                );
                let fits = format!("({} {coll}.insertElement {graph})", lib("graphFits"));
                match template {
                    Template::GraphReachable => {
                        let reach = format!("{coll}.reachableFrom");
                        (
                            format!(
                                "({} {common} {} {} {graph} {successors} (fun _ => rfl) ({reach} {graph}) {} {} {} {head}{} {} {})",
                                lib("tpl_graphReachable"),
                                self.set_ops(&node, &coll)?,
                                self.look_ops(&node, &nodes, &coll)?,
                                computed(2),
                                clause(3, &reach),
                                clause(5, &reach),
                                at(&["tk"]),
                                Self::placed(13),
                                value(1)?
                            ),
                            fits,
                        )
                    }
                    Template::GraphTopological => {
                        let topological = format!("{coll}.topological");
                        let remove = format!("{coll}.removeElement");
                        let contains = format!("{coll}.containsElement");
                        (
                            format!(
                                "({} {common} {} (fun _ => rfl) {} {} {graph} {successors} (fun _ => rfl) (⟨{}, {}, {}, (fun _ _ _ _ h => by simp only [{topological}, h] <;> rfl), {}⟩ : {} {successors} {contains} {remove} ({topological} {graph})) {head}{} {})",
                                lib("tpl_graphTopological"),
                                self.option_bundle(&SemanticType::Option {
                                    value: Box::new(nodes.clone()),
                                })?,
                                self.set_ops(&node, &coll)?,
                                self.look_ops(&node, &nodes, &coll)?,
                                computed(1),
                                computed(3),
                                computed(2),
                                clause(5, &topological),
                                lib("TopoOps"),
                                at(&["tk"]),
                                Self::placed(12)
                            ),
                            fits,
                        )
                    }
                    Template::MapInsert
                    | Template::MapRemove
                    | Template::MapLookup
                    | Template::MapContains
                    | Template::MapKeys
                    | Template::MapValues
                    | Template::MapFold
                    | Template::SetInsert
                    | Template::SetRemove
                    | Template::SetContains
                    | Template::SetUnion
                    | Template::SetIntersection
                    | Template::SetDifference
                    | Template::SetFold
                    | Template::ListFold
                    | Template::Iterate
                    | Template::IterateUntil
                    | Template::GraphSuccessors => {
                        return Err("a traversal of a non-traversal template".to_owned());
                    }
                }
            }
        };
        let list = operands(proofs);
        Ok(Proof {
            proof: format!("({} {} {lemma})", lib("conv_call"), list.proof),
            fits: and(&list.fits, &fits),
        })
    }
}

/// Whether a type mentions the type parameter `name`.
fn mentions_parameter(ty: &SemanticType, name: &str) -> bool {
    let mut found = false;
    let mut stack = vec![ty];
    loop {
        let current = match stack.pop() {
            Some(current) => current,
            None => break,
        };
        match current {
            SemanticType::Parameter { name: other } => found |= other == name,
            SemanticType::Option { value: inner }
            | SemanticType::List { element: inner }
            | SemanticType::Set { element: inner } => stack.push(inner),
            SemanticType::Result {
                ok: left,
                error: right,
            }
            | SemanticType::Product { left, right }
            | SemanticType::Map {
                key: left,
                value: right,
            } => {
                stack.push(left);
                stack.push(right);
            }
            SemanticType::Named {
                member: _,
                arguments,
            } => stack.extend(arguments.iter()),
            SemanticType::Function { parameters, result } => {
                stack.extend(parameters.iter());
                stack.push(result);
            }
            SemanticType::Type
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
            | SemanticType::Ordering => {}
        }
    }
    found
}

impl Gen<'_> {
    /// A source `match` with rendered alternatives, elaborated so that the
    /// pinned Lean reuses the source's own matcher.
    ///
    /// Lean shares one auxiliary matcher among matches with the same
    /// discriminant types and patterns, and the kernel compares two matches
    /// on an unknown discriminant only when they share it. A match in a
    /// generic definition has a matcher abstracted over the type parameters
    /// its discriminant type mentions; the instance's match is therefore
    /// written inside the same abstraction and applied to the type
    /// arguments, its alternatives passed in as functions.
    #[allow(clippy::too_many_arguments)]
    fn select(
        &self,
        ctx: &Ctx,
        scrutinee: &SemanticTerm,
        scrutinee_src: &str,
        branches: &[SemanticBranch],
        alternatives: &[String],
        result: &str,
        hypothesis: Option<&str>,
    ) -> Result<String, String> {
        let generic = self
            .source
            .infer(scrutinee, &ctx.generics(), &ctx.generic)?;
        let abstracted: Vec<&(String, SemanticType)> = ctx
            .parameters
            .iter()
            .filter(|(name, _)| mentions_parameter(&generic, name))
            .collect();
        let head = |discriminant: &str| match hypothesis {
            Some(name) => format!("match (generalizing := false) {name} : {discriminant} with"),
            None => format!("match {discriminant} with"),
        };
        if abstracted.is_empty() {
            let mut out = format!("({}", head(scrutinee_src));
            for (branch, alternative) in branches.iter().zip(alternatives) {
                out.push_str(&format!(
                    " | {} => {alternative}",
                    self.pattern(branch, &ctx.site)?
                ));
            }
            out.push(')');
            return Ok(out);
        }
        let closed = self.source.infer(scrutinee, &ctx.sources(), &ctx.site)?;
        let mut binders = String::new();
        for (name, _) in &abstracted {
            binders.push_str(&format!(" ({} : Type)", identifier(name)));
        }
        binders.push_str(&format!(" (__a : {})", self.render_type(&generic, true)?));
        let mut arms = String::new();
        let mut arguments = String::new();
        for (position, (branch, alternative)) in branches.iter().zip(alternatives).enumerate() {
            let (_, generic_fields) = self.source.branch(branch, &generic, &ctx.generic)?;
            let (_, closed_fields) = self.source.branch(branch, &closed, &ctx.site)?;
            let pattern = self.pattern(branch, &ctx.site)?;
            let mut domain = String::new();
            let mut parameters = String::new();
            let mut applied = String::new();
            for (binder, (generic_ty, closed_ty)) in branch
                .binders
                .iter()
                .zip(generic_fields.iter().zip(&closed_fields))
            {
                let name = identifier(binder);
                domain.push_str(&format!(
                    " ({name} : {})",
                    self.render_type(generic_ty, true)?
                ));
                parameters.push_str(&format!(" ({name} : {})", self.ty(closed_ty)?));
                applied.push_str(&format!(" {name}"));
            }
            let continuation = match hypothesis {
                Some(name) => {
                    parameters.push_str(&format!(" {name}"));
                    applied.push_str(&format!(" {name}"));
                    format!("∀{domain}, __a = {pattern} -> {result}")
                }
                None if domain.is_empty() => result.to_owned(),
                None => format!("∀{domain}, {result}"),
            };
            binders.push_str(&format!(" (__k{position} : {continuation})"));
            arms.push_str(&format!(" | {pattern} => __k{position}{applied}"));
            if parameters.is_empty() {
                arguments.push_str(&format!(" ({alternative})"));
            } else {
                arguments.push_str(&format!(" (fun{parameters} => {alternative})"));
            }
        }
        let mut out = format!("((fun{binders} => {}{arms})", head("__a"));
        for (_, argument) in &abstracted {
            out.push_str(&format!(" {}", self.ty(argument)?));
        }
        out.push_str(&format!(" {scrutinee_src}{arguments})"));
        Ok(out)
    }
}
