//! Lowering a production root to the realization calculus (SPEC.md §17.17.2).
//!
//! The runtime closure of an eligible root becomes one closed target
//! program whose function 0 is the root. Every construct of the closure is
//! realized by exactly the calculus elements its row of the realization
//! table names (§17.14), with one fixed rule per construct: no rule is a
//! default, so a construct this file does not name explicitly cannot be
//! lowered at all, and `cargo xtask validate-model` audits that the file
//! names every one (§17.17.7).
//!
//! The program is canonical: functions are numbered in discovery order from
//! the root, every local in first-binding order, every document type at one
//! ADT per closed instantiation in discovery order. Its [`Layout`] records
//! where each function and ADT came from, which the preservation
//! certificate reads to state its theorems; it never reads the expressions
//! this file produced.

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

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::eligibility::{integer_type, LinkedModule};
use super::source::{Constructor, Local, Site, Source};
use super::RootReport;
use crate::calculus::library::{validators, Template};
use crate::calculus::{Adt, Arm, Expr, Function, IntKind, Prim, Program, Shape, Ty, Value};
use crate::code;
use crate::config::Limits;
use crate::diagnostic::Diagnostic;
use crate::ir::semantic::{
    MemberRef, SemanticAssignment, SemanticBranch, SemanticDeclaration, SemanticEdge,
    SemanticInteger, SemanticMapEntry, SemanticParameter, SemanticPrimitive, SemanticTerm,
    SemanticType,
};

/// Where one target function came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Origin {
    /// A definition at a closed instantiation.
    Definition {
        module: String,
        name: String,
        type_arguments: Vec<SemanticType>,
        /// The eligibility report's instance key.
        instance: String,
    },
    /// A class instance value.
    Instance { module: String, name: String },
    /// The `ordinal`-th lambda, in evaluation order, of function `owner`.
    Lambda { owner: u64, ordinal: u64 },
    /// Member `member` of one instance of a library template, whose entry is
    /// function `entry`.
    Template {
        template: Template,
        types: Vec<Ty>,
        entry: u64,
        member: u64,
    },
    /// The boundary validator of the closed type `ty` (§17.17): it decides
    /// §17.12's invariants of a value of `ty` the root receives, with the
    /// key orders of `module`'s collection runtime.
    Validator {
        ty: SemanticType,
        module: String,
        kind: Validation,
    },
    /// The root's entry: it calls the validator of each parameter that
    /// carries an invariant, in parameter order, and the root on the
    /// arguments when every one holds; otherwise it returns `none`.
    Entry { validators: Vec<Option<u64>> },
}

/// What one boundary validator checks, by the validators it calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Validation {
    /// A type without an invariant, where a container's validator needs
    /// one for a component: always true.
    Trivial,
    /// A set's elements strictly ascend.
    Set,
    /// A map's keys strictly ascend and every value satisfies `value`.
    Map { value: u64 },
    /// Every element satisfies `element`.
    List { element: u64 },
    /// A present value satisfies `value`.
    Option { value: u64 },
    /// Both components satisfy theirs.
    Pair { left: u64, right: u64 },
    /// The value or the error satisfies its validator.
    Result { ok: u64, error: u64 },
    /// Each constructor's fields that carry an invariant satisfy theirs.
    Document { fields: Vec<Vec<Option<u64>>> },
}

/// Where one target ADT came from: the closed document type it realizes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdtOrigin {
    pub ty: SemanticType,
    /// The canonical spelling of the type.
    pub text: String,
}

/// Function and ADT provenance of a lowered program, index for index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    pub functions: Vec<Origin>,
    pub adts: Vec<AdtOrigin>,
}

/// A lowered root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lowered {
    pub program: Program,
    pub layout: Layout,
}

impl Lowered {
    /// The function a caller invokes: the entry when the root has one
    /// (§17.17), else the root itself, function 0.
    #[must_use]
    pub fn entry(&self) -> u64 {
        match self.layout.functions.last() {
            Some(Origin::Entry { validators: _ }) => (self.layout.functions.len() - 1) as u64,
            Some(
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
                },
            )
            | None => 0,
        }
    }
}

fn internal(reason: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(code!("LLI9001"), format!("phase lowering: {reason}"))
}

/// A body still to be lowered.
enum Pending {
    Definition {
        index: u64,
        module: String,
        name: String,
        type_arguments: Vec<SemanticType>,
    },
    Instance {
        index: u64,
        module: String,
        name: String,
    },
    Lambda {
        index: u64,
        site: Site,
        captures: Vec<Local>,
        parameters: Vec<Local>,
        body: SemanticTerm,
    },
}

/// The scope of one function body: source locals with their target names.
#[derive(Default, Clone)]
struct Scope {
    locals: Vec<(Local, u64)>,
    next: u64,
}

impl Scope {
    fn bind(&mut self, local: Local) -> u64 {
        let name = self.next;
        self.next += 1;
        self.locals.push((local, name));
        name
    }

    fn lookup(&self, name: &str) -> Result<&(Local, u64), String> {
        self.locals
            .iter()
            .rev()
            .find(|(local, _)| local.name == name)
            .ok_or_else(|| format!("local `{name}` is unbound"))
    }

    fn sources(&self) -> Vec<Local> {
        self.locals.iter().map(|(local, _)| local.clone()).collect()
    }
}

struct Lowerer<'a> {
    source: Source<'a>,
    functions: Vec<Option<Function>>,
    origins: Vec<Origin>,
    instances: BTreeMap<String, u64>,
    templates: BTreeMap<(String, Vec<Ty>), u64>,
    adts: Vec<Option<Adt>>,
    adt_origins: Vec<AdtOrigin>,
    adt_index: BTreeMap<String, u64>,
    queue: VecDeque<Pending>,
    /// Lambdas lowered so far per owner, for their ordinals.
    lambdas: BTreeMap<u64, u64>,
    /// Boundary validators by the type they check.
    validators: BTreeMap<String, u64>,
    /// The nodes of the lowered program charged so far, and the limit
    /// (`max_ir_nodes`) they are charged against.
    spent: u64,
    budget: u64,
    /// The functions whose bodies have been charged.
    charged: BTreeSet<usize>,
}

/// Marks a failure that is a resource limit, not a defect.
pub const LIMIT: &str = "limit: ";

/// Lower one eligible root. A failure is a compiler defect: eligibility
/// already admitted every construct the closure reaches.
///
/// # Errors
///
/// Returns `LLI9001` naming the construct that could not be lowered, or a
/// disagreement between the lowered closure and the eligibility report; and
/// `LLS8002` when the lowered program, which grows with the size of the types
/// a generic definition is instantiated at, exceeds `max_ir_nodes`, or when
/// the certificates it implies would exceed `max_file_bytes`.
pub fn lower_root(
    modules: &BTreeMap<String, LinkedModule<'_>>,
    module: &str,
    name: &str,
    report: &RootReport,
    limits: &Limits,
) -> Result<Lowered, Diagnostic> {
    let mut lowerer = Lowerer {
        source: Source { modules },
        functions: Vec::new(),
        origins: Vec::new(),
        instances: BTreeMap::new(),
        templates: BTreeMap::new(),
        adts: Vec::new(),
        adt_origins: Vec::new(),
        adt_index: BTreeMap::new(),
        queue: VecDeque::new(),
        lambdas: BTreeMap::new(),
        validators: BTreeMap::new(),
        spent: 0,
        budget: limits.max_ir_nodes,
        charged: BTreeSet::new(),
    };
    let describe = |reason: String| match reason.strip_prefix(LIMIT) {
        Some(limit) => {
            Diagnostic::new(code!("LLS8002"), format!("root `{}`: {limit}", report.root))
        }
        None => internal(reason),
    };
    let root = lowerer.definition(module, name, &[]).map_err(describe)?;
    if root != 0 {
        return Err(internal("the root is not function 0"));
    }
    loop {
        match lowerer.queue.pop_front() {
            Some(pending) => {
                lowerer.pending(pending).map_err(describe)?;
                lowerer.charge_functions().map_err(describe)?;
            }
            None => break,
        }
    }
    // The closure lowered here is the closure eligibility analysed, member
    // for member: a function realized without being admitted, or admitted
    // without being realized, is a defect in one of the two walks.
    let mut lowered: Vec<String> = lowerer
        .origins
        .iter()
        .filter_map(|origin| match origin {
            Origin::Definition {
                module: _,
                name: _,
                type_arguments: _,
                instance,
            } => Some(instance.clone()),
            Origin::Instance { module, name } => Some(lowerer.source.lean_name(
                module,
                &MemberRef {
                    module: Some(module.clone()),
                    name: name.clone(),
                },
            )),
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
            | Origin::Entry { validators: _ } => None,
        })
        .collect();
    lowered.sort();
    let mut admitted: Vec<String> = report
        .runtime
        .iter()
        .map(|member| member.instance.clone())
        .collect();
    admitted.sort();
    if lowered != admitted {
        return Err(internal(format!(
            "the lowered closure {lowered:?} is not the eligible closure {admitted:?}"
        )));
    }
    // The boundary follows the closure, so the root stays function 0 and
    // the closure's functions keep their indices.
    lowerer.entry(module, name).map_err(describe)?;
    lowerer.charge_functions().map_err(describe)?;
    let functions = lowerer
        .functions
        .into_iter()
        .enumerate()
        .map(|(index, function)| {
            function.ok_or_else(|| internal(format!("function {index} has no body")))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let adts = lowerer
        .adts
        .into_iter()
        .enumerate()
        .map(|(index, adt)| adt.ok_or_else(|| internal(format!("ADT {index} has no constructors"))))
        .collect::<Result<Vec<_>, _>>()?;
    let program = Program {
        spec: crate::calculus::PROGRAM_SPEC.to_owned(),
        adts,
        functions,
    };
    let canonical = program
        .canonical()
        .map_err(|reason| internal(format!("the lowered program is invalid: {reason}")))?;
    if canonical != program {
        return Err(internal(
            "the lowered program is not in first-binding order",
        ));
    }
    let lowered = Lowered {
        program,
        layout: Layout {
            functions: lowerer.origins,
            adts: lowerer.adt_origins,
        },
    };
    audit_boundary(&lowered).map_err(internal)?;
    // The certificates are generated as text from the program; refuse before
    // generating them when a lower bound on the largest already exceeds the
    // largest file the project allows. A program whose certificates fit is
    // never refused here: what the bound does not see, the generation under
    // the same limit stops.
    let (which, lower_bound) =
        certificate_lower_bounds(&measure_program(&lowered.program)).greatest();
    if lower_bound > limits.max_file_bytes {
        return Err(Diagnostic::new(
            code!("LLS8002"),
            format!(
                "root `{}`: max_file_bytes exceeded in phase lowering: configured {}, its certificate {which} is at least {lower_bound} bytes ({} program nodes)",
                report.root,
                limits.max_file_bytes,
                program_nodes(&lowered.program)
            ),
        ));
    }
    // The rendering of a field read prints the record's whole pattern, so
    // the Rust crate and certificate B grow with the square of a record's
    // arity, and the crate is built from the program after this. A program
    // whose reads alone would take more than the limit is refused here.
    let slots = record_slots(&lowered.program);
    let floor = slots.saturating_mul(CERTIFICATE_BYTES_PER_SLOT);
    if floor > limits.max_file_bytes {
        return Err(Diagnostic::new(
            code!("LLS8002"),
            format!(
                "root `{}`: max_file_bytes exceeded in phase lowering: configured {}, its field reads print {slots} record entries, which take its certificate B at least {floor} bytes",
                report.root, limits.max_file_bytes
            ),
        ));
    }
    Ok(lowered)
}

/// What the certificates repeat of a program, counted by kind: the nodes of
/// its types wherever a type is written, its expression nodes, and its shape
/// nodes (a field read, a constructor built, or an arm's binders), which
/// carry the record or constructor they range over.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Measure {
    /// Type nodes, at every position a type is written.
    pub types: u64,
    /// Expression nodes, an arm counting as one.
    pub exprs: u64,
    /// Field reads, constructors built, and the binders of arms, counted once
    /// each with the arm.
    pub shapes: u64,
    /// The bytes of the strings of the program's literals.
    pub string_bytes: u64,
    /// The hexadecimal digits of the program's bytes literals.
    pub hex_digits: u64,
    /// The decimal digits of the program's numeric literals.
    pub number_digits: u64,
    /// The sum over matches of the square of their arms: a match on `C`
    /// constructors states, for each arm, the arms it follows, so its proof
    /// grows with `C` squared.
    pub arm_pairs: u64,
    /// The arms of the widest match.
    pub widest_match: u64,
    /// The type nodes a certificate prints, at the least: those of every
    /// parameter, result, and constructor field, and, for the types written at
    /// the expressions of a function (a `Value`, `Let`, `Match`, or `Build`),
    /// the nodes of the largest one, once. A certificate prints the type of
    /// an expression where it needs it, not at every node, so a pair nested
    /// `d` deep is `d` type nodes and not the `d^2 / 2` of its `d`
    /// subterms; `types`, which charges `max_ir_nodes`, counts every one.
    pub printed_types: u64,
    /// The nodes of the largest type written at an expression of the function
    /// being measured, until [`measure_function`] folds it into
    /// `printed_types`.
    pub widest_type: u64,
}

impl Measure {
    /// Add the type written at an expression: every node counts toward
    /// `types`, and the largest toward `printed_types` once for the function.
    fn note_type(&mut self, ty: &Ty) {
        let mut measured = measure_type(ty);
        self.widest_type = self.widest_type.max(measured.printed_types);
        measured.printed_types = 0;
        self.add(measured);
    }

    fn add(&mut self, other: Measure) {
        self.widest_match = self.widest_match.max(other.widest_match);
        self.widest_type = self.widest_type.max(other.widest_type);
        self.printed_types = self.printed_types.saturating_add(other.printed_types);
        self.types = self.types.saturating_add(other.types);
        self.exprs = self.exprs.saturating_add(other.exprs);
        self.shapes = self.shapes.saturating_add(other.shapes);
        self.string_bytes = self.string_bytes.saturating_add(other.string_bytes);
        self.hex_digits = self.hex_digits.saturating_add(other.hex_digits);
        self.number_digits = self.number_digits.saturating_add(other.number_digits);
        self.arm_pairs = self.arm_pairs.saturating_add(other.arm_pairs);
    }
}

/// The sixteenths of a byte each unit of a [`Measure`] costs a certificate,
/// at the least. A certificate A, B, or E repeats different things of a
/// program, so each has its own costs: A repeats the types it prints, expressions,
/// and the pairs of arms of a match, B the expressions and shapes (it states each
/// with the Rust it relates and the derivation) and the literals, E the pairs
/// (its statements are about the entry, whatever else the program holds). Each cost is half of the greatest for which the
/// bound stays below that certificate for every root of the three example
/// corpora, for the families of programs that grow one dimension at a time
/// (let chains, call chains, many parameters, enumerations, nested matches,
/// wide structures, record copies, long literals, a generic chain, generic
/// instances, long names), and for programs that grow several at once
/// (`conformance_sp_02` asserts the bound is never above its certificate on
/// any of them, and that the largest scaling of each mixed shape whose
/// certificates fit under the default limit is not refused).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Costs {
    /// Bytes a certificate has whatever the program: imports and the
    /// statement of the root.
    pub base_bytes: u64,
    /// Sixteenths of a byte for each type node.
    pub type_node: u64,
    /// For each expression node.
    pub expr_node: u64,
    /// For each shape node.
    pub shape_node: u64,
    /// For each pair of arms of one match.
    pub arm_pair: u64,
    /// For each byte of a string literal.
    pub string_byte: u64,
    /// For each hexadecimal digit of a bytes literal.
    pub hex_digit: u64,
}

/// What certificate A costs at the least.
pub const COSTS_A: Costs = Costs {
    base_bytes: 1080,
    type_node: 284,
    expr_node: 176,
    shape_node: 0,
    arm_pair: 180,
    string_byte: 7,
    hex_digit: 19,
};

/// What certificate B costs at the least.
pub const COSTS_B: Costs = Costs {
    base_bytes: 800,
    type_node: 0,
    expr_node: 1072,
    shape_node: 89,
    arm_pair: 0,
    string_byte: 23,
    hex_digit: 59,
};

/// What certificate E costs at the least.
pub const COSTS_E: Costs = Costs {
    base_bytes: 690,
    type_node: 0,
    expr_node: 0,
    shape_node: 0,
    arm_pair: 2,
    string_byte: 0,
    hex_digit: 0,
};

impl Costs {
    /// The bytes a certificate with these costs takes at the least for a
    /// program of `measure`.
    #[must_use]
    pub fn bound(&self, measure: &Measure) -> u64 {
        let sixteenths = measure
            .printed_types
            .saturating_mul(self.type_node)
            .saturating_add(measure.exprs.saturating_mul(self.expr_node))
            .saturating_add(measure.shapes.saturating_mul(self.shape_node))
            .saturating_add(measure.arm_pairs.saturating_mul(self.arm_pair))
            .saturating_add(measure.string_bytes.saturating_mul(self.string_byte))
            .saturating_add(measure.hex_digits.saturating_mul(self.hex_digit));
        (sixteenths / 16).saturating_add(self.base_bytes)
    }
}

/// A lower bound on each of a program's certificates, in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds {
    /// Certificate A.
    pub a: u64,
    /// Certificate B, of the larger of the targets.
    pub b: u64,
    /// Certificate E.
    pub e: u64,
}

impl Bounds {
    /// The certificate whose bound is greatest, with the bound: the lower
    /// bound on the largest certificate. The certificates are large for
    /// different reasons, so the sum of what they have in common says
    /// nothing about the largest, and the greatest bound is the one that is
    /// a bound on it.
    #[must_use]
    pub fn greatest(&self) -> (&'static str, u64) {
        [("A", self.a), ("B", self.b), ("E", self.e)]
            .into_iter()
            .fold(
                ("A", 0),
                |best, next| if next.1 > best.1 { next } else { best },
            )
    }
}

/// A lower bound on each of a program's certificates, from what the program
/// states. It is used to refuse early and only to refuse early: a root is
/// refused when the greatest of the three exceeds `max_file_bytes`, and a
/// root whose certificates fit is never refused by it. The generation under
/// `max_file_bytes` is what bounds every program.
#[must_use]
pub fn certificate_lower_bounds(measure: &Measure) -> Bounds {
    Bounds {
        a: COSTS_A.bound(measure),
        b: COSTS_B.bound(measure),
        e: COSTS_E.bound(measure),
    }
}

fn measure_type(ty: &Ty) -> Measure {
    let mut out = Measure {
        types: 1,
        printed_types: 1,
        ..Measure::default()
    };
    match ty {
        Ty::Unit
        | Ty::Bool
        | Ty::Nat
        | Ty::Int
        | Ty::String
        | Ty::Bytes
        | Ty::Ordering
        | Ty::Fixed { width: _ }
        | Ty::Adt { index: _ } => {}
        Ty::Option { value: inner } | Ty::List { element: inner } => out.add(measure_type(inner)),
        Ty::Result {
            ok: left,
            error: right,
        }
        | Ty::Pair { left, right } => {
            out.add(measure_type(left));
            out.add(measure_type(right));
        }
        Ty::Fn { parameters, result } => {
            for parameter in parameters {
                out.add(measure_type(parameter));
            }
            out.add(measure_type(result));
        }
    }
    out
}

fn measure_expr(expr: &Expr) -> Measure {
    let mut out = Measure {
        exprs: 1,
        ..Measure::default()
    };
    let each = |out: &mut Measure, operands: &[Expr]| {
        for operand in operands {
            out.add(measure_expr(operand));
        }
    };
    match expr {
        Expr::Value { ty, value } => {
            out.note_type(ty);
            out.add(measure_literal(value));
        }
        Expr::Var { name: _ } => {}
        Expr::Let {
            name: _,
            ty,
            bound,
            body,
        } => {
            out.note_type(ty);
            out.add(measure_expr(bound));
            out.add(measure_expr(body));
        }
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => {
            out.add(measure_expr(condition));
            out.add(measure_expr(then_branch));
            out.add(measure_expr(else_branch));
        }
        Expr::Match {
            ty,
            scrutinee,
            arms,
        } => {
            out.note_type(ty);
            out.add(measure_expr(scrutinee));
            out.arm_pairs = out
                .arm_pairs
                .saturating_add((arms.len() as u64).saturating_mul(arms.len() as u64));
            out.widest_match = out.widest_match.max(arms.len() as u64);
            for arm in arms {
                out.exprs = out.exprs.saturating_add(1);
                out.shapes = out.shapes.saturating_add(1 + arm.binders.len() as u64);
                out.add(measure_expr(&arm.body));
            }
        }
        Expr::Build {
            shape: _,
            ty,
            operands,
        } => {
            out.shapes = out.shapes.saturating_add(1);
            out.note_type(ty);
            each(&mut out, operands);
        }
        Expr::Call {
            function: _,
            operands,
        }
        | Expr::Prim {
            operation: _,
            operands,
        } => each(&mut out, operands),
        Expr::Closure {
            function: _,
            captures,
        } => each(&mut out, captures),
        Expr::Apply { target, operands } => {
            out.add(measure_expr(target));
            each(&mut out, operands);
        }
        Expr::First { value } | Expr::Second { value } => out.add(measure_expr(value)),
        Expr::Field { value, index: _ } => {
            out.shapes = out.shapes.saturating_add(1);
            out.add(measure_expr(value));
        }
    }
    out
}

/// What the literal `value` is made of (see [`Measure`]).
fn measure_literal(value: &Value) -> Measure {
    let mut out = Measure::default();
    let each = |out: &mut Measure, items: &[Value]| {
        for item in items {
            out.add(measure_literal(item));
        }
    };
    match value {
        Value::String { value } => out.string_bytes = value.len() as u64,
        Value::Bytes { hex } => out.hex_digits = hex.len() as u64,
        Value::Nat { value }
        | Value::Int { value }
        | Value::U8 { value }
        | Value::U16 { value }
        | Value::U32 { value }
        | Value::U64 { value }
        | Value::I8 { value }
        | Value::I16 { value }
        | Value::I32 { value }
        | Value::I64 { value } => out.number_digits = value.len() as u64,
        Value::Some { value } | Value::Ok { value } | Value::Error { value } => {
            out.add(measure_literal(value));
        }
        Value::Pair { left, right } => {
            out.add(measure_literal(left));
            out.add(measure_literal(right));
        }
        Value::List { items } => each(&mut out, items),
        Value::Adt {
            constructor: _,
            fields,
        } => each(&mut out, fields),
        Value::Closure {
            function: _,
            captures,
        } => each(&mut out, captures),
        Value::Unit | Value::Bool { value: _ } | Value::Ordering { value: _ } | Value::None => {}
    }
    out
}

fn measure_function(function: &Function) -> Measure {
    let mut out = Measure::default();
    for ty in &function.types {
        out.add(measure_type(ty));
    }
    out.add(measure_type(&function.result));
    out.add(measure_expr(&function.body));
    out.printed_types = out.printed_types.saturating_add(out.widest_type);
    out.widest_type = 0;
    out
}

fn function_nodes(function: &Function) -> u64 {
    let measure = measure_function(function);
    measure.types.saturating_add(measure.exprs)
}

/// What a program has that the certificates repeat: the types and the
/// expressions of every function, and the types of every constructor.
#[must_use]
pub fn measure_program(program: &Program) -> Measure {
    let mut out = Measure::default();
    for function in &program.functions {
        out.add(measure_function(function));
    }
    for ty in program
        .adts
        .iter()
        .flat_map(|adt| adt.constructors.iter().flatten())
    {
        out.add(measure_type(ty));
    }
    out
}

/// The arms of a match above which the certificates of a program lift the
/// pinned Lean's heartbeat budgets: 100 constructors are checked within the
/// defaults (`maxHeartbeats` 200 000, `synthInstance.maxHeartbeats` 20 000),
/// 130 as well, 160 exhaust the second, 200 the first.
pub const UNBUDGETED_ARMS: u64 = 100;

/// The `set_option` lines a certificate about `program` starts with, besides
/// those every certificate has: none when its widest match has at most
/// [`UNBUDGETED_ARMS`] arms, and otherwise the removal of both heartbeat
/// budgets.
///
/// Certificate B states every arm of a match after the arms before it, and
/// the pinned Lean spends allocations on that faster than any bound this
/// project could state (a match on 200 constructors needs between 300 000 and
/// 400 000 heartbeats, one on 300 needs more than 3 800 000, where the
/// defaults are 200 000), and exhausting a budget in the middle of a proof
/// ends as a type mismatch that blames a valid certificate. The budget that
/// remains is the wall clock, `child_timeout_ms`, whose exhaustion is a
/// registered limit (`LLS8002`) and not a verdict on the certificate.
#[must_use]
pub fn budget_options(program: &Program) -> String {
    if measure_program(program).widest_match > UNBUDGETED_ARMS {
        "set_option maxHeartbeats 0\nset_option synthInstance.maxHeartbeats 0\n".to_owned()
    } else {
        String::new()
    }
}

/// The size of a program: the nodes of every type and expression in it.
#[must_use]
pub fn program_nodes(program: &Program) -> u64 {
    let measure = measure_program(program);
    measure.types.saturating_add(measure.exprs)
}

/// The bytes of certificate B that one slot of a field read costs, at the
/// least: a read of a record of `n` fields prints a pattern of `n` entries
/// into the crate's term, into the statement of the function, and into the
/// derivation, and the smallest of the three families measured (a record of
/// `n` fields copied field by field) has 22 bytes in each slot. The least
/// is used because a refusal on it is a refusal of a certificate that is
/// certainly too large.
pub const CERTIFICATE_BYTES_PER_SLOT: u64 = 16;

/// The entries of the patterns the Rust rendering of `program` prints for
/// its field reads: for each read of a field of a record, the arity of that
/// record, taken from the type of the expression read when the program
/// states it and from the widest record otherwise. The rendering and the
/// certificates grow with it, which no count of nodes shows: a record of
/// 800 fields copied field by field has 800 reads of 800 entries.
#[must_use]
pub fn record_slots(program: &Program) -> u64 {
    let widest = program
        .adts
        .iter()
        .filter(|adt| adt.constructors.len() == 1)
        .flat_map(|adt| adt.constructors.iter().map(Vec::len))
        .max()
        .unwrap_or(0) as u64;
    let mut total = 0_u64;
    for function in &program.functions {
        let mut scope: BTreeMap<u64, Ty> = BTreeMap::new();
        for (name, ty) in function.parameters.iter().zip(&function.types) {
            scope.insert(*name, ty.clone());
        }
        slots_in(program, &function.body, &mut scope, widest, &mut total);
    }
    total
}

/// The index of the document type `ty` is.
fn adt_index(ty: &Ty) -> Option<u64> {
    match ty {
        Ty::Adt { index } => Some(*index),
        Ty::Unit
        | Ty::Bool
        | Ty::Nat
        | Ty::Int
        | Ty::String
        | Ty::Bytes
        | Ty::Ordering
        | Ty::Fixed { width: _ }
        | Ty::Option { value: _ }
        | Ty::List { element: _ }
        | Ty::Result { ok: _, error: _ }
        | Ty::Pair { left: _, right: _ }
        | Ty::Fn {
            parameters: _,
            result: _,
        } => None,
    }
}

/// The components of the product `ty` is.
fn pair_parts(ty: Ty) -> Option<(Ty, Ty)> {
    match ty {
        Ty::Pair { left, right } => Some((*left, *right)),
        Ty::Unit
        | Ty::Bool
        | Ty::Nat
        | Ty::Int
        | Ty::String
        | Ty::Bytes
        | Ty::Ordering
        | Ty::Fixed { width: _ }
        | Ty::Adt { index: _ }
        | Ty::Option { value: _ }
        | Ty::List { element: _ }
        | Ty::Result { ok: _, error: _ }
        | Ty::Fn {
            parameters: _,
            result: _,
        } => None,
    }
}

/// The fields of the first constructor of document type `index`.
fn record_fields(program: &Program, index: u64) -> Option<&Vec<Ty>> {
    usize::try_from(index)
        .ok()
        .and_then(|at| program.adts.get(at))
        .and_then(|record| record.constructors.first())
}

/// The type of an expression where the program states it.
fn stated_type(program: &Program, scope: &BTreeMap<u64, Ty>, expr: &Expr) -> Option<Ty> {
    match expr {
        Expr::Var { name } => scope.get(name).cloned(),
        Expr::Value { ty, value: _ } => Some(ty.clone()),
        Expr::Build {
            shape: _,
            ty,
            operands: _,
        } => Some(ty.clone()),
        Expr::Call {
            function,
            operands: _,
        } => usize::try_from(*function)
            .ok()
            .and_then(|at| program.functions.get(at))
            .map(|called| called.result.clone()),
        Expr::Field { value, index } => stated_type(program, scope, value)
            .as_ref()
            .and_then(adt_index)
            .and_then(|adt| record_fields(program, adt))
            .and_then(|fields| usize::try_from(*index).ok().and_then(|at| fields.get(at)))
            .cloned(),
        Expr::First { value } => stated_type(program, scope, value)
            .and_then(pair_parts)
            .map(|(left, _)| left),
        Expr::Second { value } => stated_type(program, scope, value)
            .and_then(pair_parts)
            .map(|(_, right)| right),
        Expr::Let {
            name: _,
            ty: _,
            bound: _,
            body: _,
        }
        | Expr::Cond {
            condition: _,
            then_branch: _,
            else_branch: _,
        }
        | Expr::Match {
            ty: _,
            scrutinee: _,
            arms: _,
        }
        | Expr::Closure {
            function: _,
            captures: _,
        }
        | Expr::Apply {
            target: _,
            operands: _,
        }
        | Expr::Prim {
            operation: _,
            operands: _,
        } => None,
    }
}

fn slots_in(
    program: &Program,
    expr: &Expr,
    scope: &mut BTreeMap<u64, Ty>,
    widest: u64,
    total: &mut u64,
) {
    match expr {
        Expr::Value { ty: _, value: _ } | Expr::Var { name: _ } => {}
        Expr::Let {
            name,
            ty,
            bound,
            body,
        } => {
            slots_in(program, bound, scope, widest, total);
            let shadowed = scope.insert(*name, ty.clone());
            slots_in(program, body, scope, widest, total);
            match shadowed {
                Some(previous) => {
                    scope.insert(*name, previous);
                }
                None => {
                    scope.remove(name);
                }
            }
        }
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => {
            slots_in(program, condition, scope, widest, total);
            slots_in(program, then_branch, scope, widest, total);
            slots_in(program, else_branch, scope, widest, total);
        }
        Expr::Match {
            ty: _,
            scrutinee,
            arms,
        } => {
            slots_in(program, scrutinee, scope, widest, total);
            for arm in arms {
                slots_in(program, &arm.body, scope, widest, total);
            }
        }
        Expr::Build {
            shape: _,
            ty: _,
            operands,
        }
        | Expr::Call {
            function: _,
            operands,
        }
        | Expr::Prim {
            operation: _,
            operands,
        } => {
            for operand in operands {
                slots_in(program, operand, scope, widest, total);
            }
        }
        Expr::Closure {
            function: _,
            captures,
        } => {
            for capture in captures {
                slots_in(program, capture, scope, widest, total);
            }
        }
        Expr::Apply { target, operands } => {
            slots_in(program, target, scope, widest, total);
            for operand in operands {
                slots_in(program, operand, scope, widest, total);
            }
        }
        Expr::First { value } | Expr::Second { value } => {
            slots_in(program, value, scope, widest, total);
        }
        Expr::Field { value, index: _ } => {
            let arity = stated_type(program, scope, value)
                .as_ref()
                .and_then(adt_index)
                .and_then(|adt| record_fields(program, adt))
                .map(|fields| fields.len() as u64);
            *total = total.saturating_add(arity.unwrap_or(widest));
            slots_in(program, value, scope, widest, total);
        }
    }
}

/// The functions an expression calls or closes over, in evaluation order.
pub fn referenced(expr: &Expr, out: &mut Vec<u64>) {
    match expr {
        Expr::Value { ty: _, value: _ } | Expr::Var { name: _ } => {}
        Expr::Let {
            name: _,
            ty: _,
            bound,
            body,
        } => {
            referenced(bound, out);
            referenced(body, out);
        }
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => {
            referenced(condition, out);
            referenced(then_branch, out);
            referenced(else_branch, out);
        }
        Expr::Match {
            ty: _,
            scrutinee,
            arms,
        } => {
            referenced(scrutinee, out);
            for arm in arms {
                referenced(&arm.body, out);
            }
        }
        Expr::Build {
            shape: _,
            ty: _,
            operands,
        }
        | Expr::Prim {
            operation: _,
            operands,
        } => {
            for operand in operands {
                referenced(operand, out);
            }
        }
        Expr::Call { function, operands } => {
            out.push(*function);
            for operand in operands {
                referenced(operand, out);
            }
        }
        Expr::Closure { function, captures } => {
            out.push(*function);
            for capture in captures {
                referenced(capture, out);
            }
        }
        Expr::Apply { target, operands } => {
            referenced(target, out);
            for operand in operands {
                referenced(operand, out);
            }
        }
        Expr::First { value } | Expr::Second { value } | Expr::Field { value, index: _ } => {
            referenced(value, out);
        }
    }
}

/// The boundary discipline (§17.17): §17.12's invariants are checked at run
/// time only where a value enters the program. A boundary validator is
/// referenced by the entry and by validators only, a validator references
/// validators only, the entry references validators and the root only and
/// is referenced by nothing, and the entry exists exactly when some
/// validator does. Inside the program the invariants hold by construction,
/// as the operations' proofs establish, so a validator call there would turn
/// a proof-only invariant into a runtime check.
///
/// # Errors
///
/// Returns the first reference that breaks the discipline.
pub fn audit_boundary(lowered: &Lowered) -> Result<(), String> {
    let origins = &lowered.layout.functions;
    let role = |index: u64| match origins.get(index as usize) {
        Some(Origin::Validator {
            ty: _,
            module: _,
            kind: _,
        }) => Role::Validator,
        Some(Origin::Entry { validators: _ }) => Role::Entry,
        Some(
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
            },
        )
        | None => Role::Program,
    };
    let mut entries = 0;
    let mut validators = 0;
    for (index, function) in lowered.program.functions.iter().enumerate() {
        let caller = role(index as u64);
        match caller {
            Role::Validator => validators += 1,
            Role::Entry => {
                entries += 1;
                if index + 1 != lowered.program.functions.len() {
                    return Err(format!("the entry is function {index}, not the last"));
                }
            }
            Role::Program => {}
        }
        let mut callees = Vec::new();
        referenced(&function.body, &mut callees);
        for callee in callees {
            let admitted = match (caller, role(callee)) {
                (Role::Validator | Role::Entry, Role::Validator)
                | (Role::Program, Role::Program) => true,
                (Role::Entry, Role::Program) => callee == 0,
                (Role::Program, Role::Validator)
                | (Role::Validator, Role::Program)
                | (Role::Program | Role::Validator | Role::Entry, Role::Entry) => false,
            };
            if !admitted {
                return Err(format!(
                    "{} {index} references function {callee}: a boundary validator runs only at the entry, because inside the program §17.12's invariants hold by construction and are proof-only",
                    caller.name()
                ));
            }
        }
    }
    if entries > 1 || (entries == 1) != (validators > 0) {
        return Err(format!(
            "{entries} entries for {validators} boundary validators"
        ));
    }
    Ok(())
}

/// A function's part in the boundary discipline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    Program,
    Validator,
    Entry,
}

impl Role {
    fn name(self) -> &'static str {
        match self {
            Self::Program => "function",
            Self::Validator => "the boundary validator",
            Self::Entry => "the entry",
        }
    }
}

/// The linked modules of a checked project, as the analysis reads them.
#[must_use]
pub fn linked_modules(checked: &crate::link::CheckedProject) -> BTreeMap<String, LinkedModule<'_>> {
    checked
        .modules
        .iter()
        .filter_map(|(name, module)| {
            module.document.semantic.as_ref().map(|semantic| {
                (
                    name.clone(),
                    LinkedModule {
                        lean_module: &module.document.lean_module,
                        semantic,
                    },
                )
            })
        })
        .collect()
}

/// One production root of a checked project: its module, declaration, and
/// eligibility record.
pub struct Root<'a> {
    pub module: String,
    pub name: String,
    pub report: &'a RootReport,
}

/// Every production root of a checked project, in module then report
/// order.
///
/// # Errors
///
/// Returns `LLI9001` when a report names a root its module does not hold.
pub fn roots(checked: &crate::link::CheckedProject) -> Result<Vec<Root<'_>>, Diagnostic> {
    let mut out = Vec::new();
    for (module, checked_module) in &checked.modules {
        let reports = match &checked_module.production {
            Some(report) => &report.roots,
            None => continue,
        };
        let prefix = format!("{}.", checked_module.document.lean_module);
        for report in reports {
            let name = report
                .root
                .strip_prefix(&prefix)
                .ok_or_else(|| internal(format!("root `{}` is outside its module", report.root)))?;
            out.push(Root {
                module: module.clone(),
                name: name.to_owned(),
                report,
            });
        }
    }
    Ok(out)
}

fn nat_value(value: &str) -> Expr {
    Expr::Value {
        ty: Ty::Nat,
        value: Value::Nat {
            value: value.to_owned(),
        },
    }
}

fn build(shape: Shape, ty: Ty, operands: Vec<Expr>) -> Expr {
    Expr::Build {
        shape,
        ty,
        operands,
    }
}

fn prim(operation: Prim, operands: Vec<Expr>) -> Expr {
    Expr::Prim {
        operation,
        operands,
    }
}

fn boolean(value: bool) -> Expr {
    build(
        if value { Shape::True } else { Shape::False },
        Ty::Bool,
        Vec::new(),
    )
}

fn cond(condition: Expr, then_branch: Expr, else_branch: Expr) -> Expr {
    Expr::Cond {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
    }
}

/// Whether a closed type is `ContractViolation`.
const fn is_violation(ty: &SemanticType) -> bool {
    match ty {
        SemanticType::ContractViolation | SemanticType::ReasoningFailure => true,
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
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Set { element: _ } => false,
    }
}

/// The fixed width a source integer type realizes, if it is one.
pub(crate) const fn fixed_kind(ty: &SemanticType) -> Option<IntKind> {
    match ty {
        SemanticType::Int8 => Some(IntKind::I8),
        SemanticType::Int16 => Some(IntKind::I16),
        SemanticType::Int32 => Some(IntKind::I32),
        SemanticType::Int64 => Some(IntKind::I64),
        SemanticType::UInt8 => Some(IntKind::U8),
        SemanticType::UInt16 => Some(IntKind::U16),
        SemanticType::UInt32 => Some(IntKind::U32),
        SemanticType::UInt64 => Some(IntKind::U64),
        SemanticType::Type
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
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
        | SemanticType::ReasoningFailure => None,
    }
}

/// The literal value of an integer of representation `representation`.
fn integer_value(representation: SemanticInteger, value: &str) -> (Ty, Value) {
    let value = value.to_owned();
    match representation {
        SemanticInteger::Int => (Ty::Int, Value::Int { value }),
        SemanticInteger::Int8 => (Ty::Fixed { width: IntKind::I8 }, Value::I8 { value }),
        SemanticInteger::Int16 => (
            Ty::Fixed {
                width: IntKind::I16,
            },
            Value::I16 { value },
        ),
        SemanticInteger::Int32 => (
            Ty::Fixed {
                width: IntKind::I32,
            },
            Value::I32 { value },
        ),
        SemanticInteger::Int64 => (
            Ty::Fixed {
                width: IntKind::I64,
            },
            Value::I64 { value },
        ),
        SemanticInteger::UInt8 => (Ty::Fixed { width: IntKind::U8 }, Value::U8 { value }),
        SemanticInteger::UInt16 => (
            Ty::Fixed {
                width: IntKind::U16,
            },
            Value::U16 { value },
        ),
        SemanticInteger::UInt32 => (
            Ty::Fixed {
                width: IntKind::U32,
            },
            Value::U32 { value },
        ),
        SemanticInteger::UInt64 => (
            Ty::Fixed {
                width: IntKind::U64,
            },
            Value::U64 { value },
        ),
    }
}

/// The template that realizes a collection primitive, with the closed
/// source types its instance is fixed by, or `None` for the collection
/// primitives realized directly.
pub(crate) fn template_of(
    operation: SemanticPrimitive,
    types: &[SemanticType],
) -> Result<Option<(Template, Vec<SemanticType>)>, String> {
    let map = |ty: &SemanticType| -> Result<(SemanticType, SemanticType), String> {
        match ty {
            SemanticType::Map { key, value } => Ok((key.as_ref().clone(), value.as_ref().clone())),
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
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Set { element: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => Err(format!("{operation:?} of a non-map")),
        }
    };
    let element = |ty: &SemanticType| -> Result<SemanticType, String> {
        match ty {
            SemanticType::Set { element } | SemanticType::List { element } => {
                Ok(element.as_ref().clone())
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
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => Err(format!("{operation:?} of a non-collection")),
        }
    };
    let argument = |index: usize| -> Result<&SemanticType, String> {
        types
            .get(index)
            .ok_or_else(|| format!("{operation:?} lacks argument {index}"))
    };
    Ok(Some(match operation {
        SemanticPrimitive::MapInsert => {
            let (key, value) = map(argument(0)?)?;
            (Template::MapInsert, vec![key, value])
        }
        SemanticPrimitive::MapRemove => {
            let (key, value) = map(argument(0)?)?;
            (Template::MapRemove, vec![key, value])
        }
        SemanticPrimitive::MapLookup => {
            let (key, value) = map(argument(0)?)?;
            (Template::MapLookup, vec![key, value])
        }
        SemanticPrimitive::MapContains => {
            let (key, value) = map(argument(0)?)?;
            (Template::MapContains, vec![key, value])
        }
        SemanticPrimitive::MapKeys => {
            let (key, value) = map(argument(0)?)?;
            (Template::MapKeys, vec![key, value])
        }
        SemanticPrimitive::MapValues => {
            let (key, value) = map(argument(0)?)?;
            (Template::MapValues, vec![key, value])
        }
        SemanticPrimitive::MapFold => {
            let (key, value) = map(argument(2)?)?;
            (Template::MapFold, vec![key, value, argument(1)?.clone()])
        }
        SemanticPrimitive::SetInsert => (Template::SetInsert, vec![element(argument(0)?)?]),
        SemanticPrimitive::SetRemove => (Template::SetRemove, vec![element(argument(0)?)?]),
        SemanticPrimitive::SetContains => (Template::SetContains, vec![element(argument(0)?)?]),
        SemanticPrimitive::SetUnion => (Template::SetUnion, vec![element(argument(0)?)?]),
        SemanticPrimitive::SetIntersection => {
            (Template::SetIntersection, vec![element(argument(0)?)?])
        }
        SemanticPrimitive::SetDifference => (Template::SetDifference, vec![element(argument(0)?)?]),
        SemanticPrimitive::SetFold => (
            Template::SetFold,
            vec![element(argument(2)?)?, argument(1)?.clone()],
        ),
        SemanticPrimitive::ListFold => (
            Template::ListFold,
            vec![element(argument(2)?)?, argument(1)?.clone()],
        ),
        SemanticPrimitive::Iterate => (Template::Iterate, vec![argument(2)?.clone()]),
        SemanticPrimitive::IterateUntil => (Template::IterateUntil, vec![argument(2)?.clone()]),
        SemanticPrimitive::GraphSuccessors => {
            let (key, _) = map(argument(0)?)?;
            (Template::GraphSuccessors, vec![key])
        }
        SemanticPrimitive::GraphReachable => {
            let (key, _) = map(argument(0)?)?;
            (Template::GraphReachable, vec![key])
        }
        SemanticPrimitive::GraphTopological => {
            let (key, _) = map(argument(0)?)?;
            (Template::GraphTopological, vec![key])
        }
        SemanticPrimitive::MapSize
        | SemanticPrimitive::MapEntries
        | SemanticPrimitive::SetSize
        | SemanticPrimitive::SetElements
        | SemanticPrimitive::Subtract
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
        | SemanticPrimitive::LessThan => return Ok(None),
    }))
}

impl Lowerer<'_> {
    /// Charge `nodes` to `max_ir_nodes`.
    fn charge(&mut self, nodes: u64) -> Result<(), String> {
        self.spent = self.spent.saturating_add(nodes);
        if self.spent > self.budget {
            return Err(format!(
                "{LIMIT}max_ir_nodes exceeded in phase lowering: configured {}, observed at least {} nodes of the lowered program",
                self.budget, self.spent
            ));
        }
        Ok(())
    }

    /// Charge the body of every function lowered since the last charge.
    fn charge_functions(&mut self) -> Result<(), String> {
        for index in 0..self.functions.len() {
            if self.charged.contains(&index) {
                continue;
            }
            let nodes = match &self.functions[index] {
                Some(function) => function_nodes(function),
                None => continue,
            };
            self.charged.insert(index);
            self.charge(nodes)?;
        }
        Ok(())
    }

    /// Reserve the next function index.
    fn reserve(&mut self, origin: Origin) -> u64 {
        self.functions.push(None);
        self.origins.push(origin);
        (self.functions.len() - 1) as u64
    }

    fn instance_key(&self, module: &str, name: &str, arguments: &[SemanticType]) -> String {
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
        key
    }

    /// The function of a definition instance, reserved on first sight.
    fn definition(
        &mut self,
        module: &str,
        name: &str,
        type_arguments: &[SemanticType],
    ) -> Result<u64, String> {
        let instance = self.instance_key(module, name, type_arguments);
        match self.instances.get(&instance) {
            Some(index) => return Ok(*index),
            None => {}
        }
        let index = self.reserve(Origin::Definition {
            module: module.to_owned(),
            name: name.to_owned(),
            type_arguments: type_arguments.to_vec(),
            instance: instance.clone(),
        });
        self.instances.insert(instance, index);
        self.queue.push_back(Pending::Definition {
            index,
            module: module.to_owned(),
            name: name.to_owned(),
            type_arguments: type_arguments.to_vec(),
        });
        Ok(index)
    }

    /// The function of a class instance value, reserved on first sight.
    fn class_instance(&mut self, module: &str, name: &str) -> u64 {
        let instance = self.instance_key(module, name, &[]);
        match self.instances.get(&instance) {
            Some(index) => return *index,
            None => {}
        }
        let index = self.reserve(Origin::Instance {
            module: module.to_owned(),
            name: name.to_owned(),
        });
        self.instances.insert(instance, index);
        self.queue.push_back(Pending::Instance {
            index,
            module: module.to_owned(),
            name: name.to_owned(),
        });
        index
    }

    /// The entry function of a template instance, instantiated on first
    /// sight with every helper it carries.
    fn template(&mut self, template: Template, types: &[SemanticType]) -> Result<u64, String> {
        let types = types
            .iter()
            .map(|ty| self.ty(ty))
            .collect::<Result<Vec<_>, _>>()?;
        let key = (template.name().to_owned(), types.clone());
        match self.templates.get(&key) {
            Some(index) => return Ok(*index),
            None => {}
        }
        let at = self.functions.len() as u64;
        let functions = template.instantiate(&types, at)?;
        for (member, function) in functions.into_iter().enumerate() {
            self.functions.push(Some(function));
            self.origins.push(Origin::Template {
                template,
                types: types.clone(),
                entry: at,
                member: member as u64,
            });
        }
        self.templates.insert(key, at);
        Ok(at)
    }

    /// The root's entry (§17.17), when a parameter carries an invariant of
    /// §17.12: it validates every such parameter, in order, and calls the
    /// root on the arguments when all hold, returning `some` of its result,
    /// and `none` otherwise.
    fn entry(&mut self, module: &str, name: &str) -> Result<(), String> {
        let (parameters, result) = self.source.signature(module, name, &[])?;
        let mut validators = Vec::new();
        let mut types = Vec::new();
        for SemanticParameter { name: _, r#type } in &parameters {
            validators.push(self.validator(r#type, module)?);
            types.push(self.ty(r#type)?);
        }
        if validators.iter().all(Option::is_none) {
            return Ok(());
        }
        let outcome = Ty::Option {
            value: Box::new(self.ty(&result)?),
        };
        let arguments = (0..types.len() as u64)
            .map(|name| Expr::Var { name })
            .collect();
        let refused = || build(Shape::None, outcome.clone(), Vec::new());
        let mut body = build(
            Shape::Some,
            outcome.clone(),
            vec![Expr::Call {
                function: 0,
                operands: arguments,
            }],
        );
        for (position, validator) in validators.iter().enumerate().rev() {
            match validator {
                Some(validator) => {
                    body = cond(
                        Expr::Call {
                            function: *validator,
                            operands: vec![Expr::Var {
                                name: position as u64,
                            }],
                        },
                        body,
                        refused(),
                    );
                }
                None => {}
            }
        }
        let index = self.reserve(Origin::Entry { validators });
        self.functions[index as usize] = Some(Function {
            parameters: (0..types.len() as u64).collect(),
            types,
            result: outcome,
            body,
        });
        Ok(())
    }

    /// Whether a value of a closed type can hold a map or a set, whose
    /// ascending order §17.12 states as an invariant.
    fn carries(&self, ty: &SemanticType, seen: &mut BTreeSet<String>) -> Result<bool, String> {
        Ok(match ty {
            SemanticType::Map { key: _, value: _ } | SemanticType::Set { element: _ } => true,
            SemanticType::Option { value: inner } | SemanticType::List { element: inner } => {
                self.carries(inner, seen)?
            }
            SemanticType::Product { left, right }
            | SemanticType::Result {
                ok: left,
                error: right,
            } => self.carries(left, seen)? || self.carries(right, seen)?,
            SemanticType::Named {
                member: _,
                arguments: _,
            } => {
                if !seen.insert(self.source.type_text(ty)) {
                    return Ok(false);
                }
                let shape = self.source.document(ty)?;
                let mut any = false;
                for fields in &shape.fields {
                    for field in fields {
                        any = self.carries(field, seen)? || any;
                    }
                }
                any
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
            | SemanticType::ReasoningFailure => false,
        })
    }

    /// The boundary validator of a closed type written in `module`, or
    /// `None` when its values carry no invariant. One validator per type;
    /// the index is reserved before the components are validated, so a
    /// recursive type's validator calls itself.
    fn validator(&mut self, ty: &SemanticType, module: &str) -> Result<Option<u64>, String> {
        if !self.carries(ty, &mut BTreeSet::new())? {
            return Ok(None);
        }
        let text = self.source.type_text(ty);
        match self.validators.get(&text) {
            Some(index) => return Ok(Some(*index)),
            None => {}
        }
        let lowered = self.ty(ty)?;
        let index = self.reserve(Origin::Validator {
            ty: ty.clone(),
            module: module.to_owned(),
            kind: Validation::Trivial,
        });
        self.validators.insert(text, index);
        let (kind, function) = match ty {
            SemanticType::Set { element } => {
                let key = self.ty(element)?;
                (Validation::Set, validators::set_of(index, &key))
            }
            SemanticType::Map { key, value } => {
                let checked = self.component(value, module)?;
                let key = self.ty(key)?;
                let stored = self.ty(value)?;
                (
                    Validation::Map { value: checked },
                    validators::map_of(index, checked, &key, &stored),
                )
            }
            SemanticType::List { element } => {
                let checked = self.component(element, module)?;
                let element = self.ty(element)?;
                (
                    Validation::List { element: checked },
                    validators::list_of(index, checked, &element),
                )
            }
            SemanticType::Option { value } => {
                let checked = self.component(value, module)?;
                let value = self.ty(value)?;
                (
                    Validation::Option { value: checked },
                    validators::option_of(checked, &value),
                )
            }
            SemanticType::Product { left, right } => {
                let first = self.component(left, module)?;
                let second = self.component(right, module)?;
                let (left, right) = (self.ty(left)?, self.ty(right)?);
                (
                    Validation::Pair {
                        left: first,
                        right: second,
                    },
                    validators::pair_of(first, second, &left, &right),
                )
            }
            SemanticType::Result { ok, error } => {
                let value = self.component(ok, module)?;
                let failure = self.component(error, module)?;
                let (ok, error) = (self.ty(ok)?, self.ty(error)?);
                (
                    Validation::Result {
                        ok: value,
                        error: failure,
                    },
                    validators::result_of(value, failure, &ok, &error),
                )
            }
            SemanticType::Named {
                member,
                arguments: _,
            } => {
                let adt = match lowered {
                    Ty::Adt { index } => index,
                    Ty::Nat
                    | Ty::Int
                    | Ty::Bool
                    | Ty::Unit
                    | Ty::String
                    | Ty::Bytes
                    | Ty::Ordering
                    | Ty::Fixed { width: _ }
                    | Ty::Option { value: _ }
                    | Ty::Result { ok: _, error: _ }
                    | Ty::List { element: _ }
                    | Ty::Pair { left: _, right: _ }
                    | Ty::Fn {
                        parameters: _,
                        result: _,
                    } => return Err(format!("`{}` is not an ADT", self.source.type_text(ty))),
                };
                // A document's fields are written in its declaring module.
                let declaring = member.module.clone().unwrap_or_else(|| module.to_owned());
                let shape = self.source.document(ty)?;
                let mut fields = Vec::new();
                for constructor in &shape.fields {
                    let mut checked = Vec::new();
                    for field in constructor {
                        checked.push(self.validator(field, &declaring)?);
                    }
                    fields.push(checked);
                }
                let function = validators::document(adt, &fields);
                (Validation::Document { fields }, function)
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
            | SemanticType::ReasoningFailure => {
                return Err(format!(
                    "`{}` carries no invariant",
                    self.source.type_text(ty)
                ));
            }
        };
        self.origins[index as usize] = Origin::Validator {
            ty: ty.clone(),
            module: module.to_owned(),
            kind,
        };
        self.functions[index as usize] = Some(function);
        Ok(Some(index))
    }

    /// The validator a container needs for a component: the component's
    /// own, or the trivial one of its type when it carries no invariant.
    fn component(&mut self, ty: &SemanticType, module: &str) -> Result<u64, String> {
        match self.validator(ty, module)? {
            Some(index) => Ok(index),
            None => {
                let text = format!("trivial {}", self.source.type_text(ty));
                match self.validators.get(&text) {
                    Some(index) => return Ok(*index),
                    None => {}
                }
                let lowered = self.ty(ty)?;
                let index = self.reserve(Origin::Validator {
                    ty: ty.clone(),
                    module: module.to_owned(),
                    kind: Validation::Trivial,
                });
                self.validators.insert(text, index);
                self.functions[index as usize] = Some(validators::trivial(&lowered));
                Ok(index)
            }
        }
    }

    /// The target type of a closed source type.
    fn ty(&mut self, ty: &SemanticType) -> Result<Ty, String> {
        self.charge(1)?;
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
                value: Box::new(self.ty(value)?),
            },
            SemanticType::Result { ok, error } => Ty::Result {
                ok: Box::new(self.ty(ok)?),
                error: Box::new(self.ty(error)?),
            },
            SemanticType::List { element } | SemanticType::Set { element } => Ty::List {
                element: Box::new(self.ty(element)?),
            },
            SemanticType::Product { left, right } => Ty::Pair {
                left: Box::new(self.ty(left)?),
                right: Box::new(self.ty(right)?),
            },
            SemanticType::Map { key, value } => Ty::List {
                element: Box::new(Ty::Pair {
                    left: Box::new(self.ty(key)?),
                    right: Box::new(self.ty(value)?),
                }),
            },
            SemanticType::Function { parameters, result } => Ty::Fn {
                parameters: parameters
                    .iter()
                    .map(|parameter| self.ty(parameter))
                    .collect::<Result<Vec<_>, _>>()?,
                result: Box::new(self.ty(result)?),
            },
            SemanticType::Named {
                member: _,
                arguments: _,
            } => Ty::Adt {
                index: self.adt(ty)?,
            },
            // A violation is the pair of Booleans it lowers to (§17.12
            // rule 9).
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => Ty::Pair {
                left: Box::new(Ty::Bool),
                right: Box::new(Ty::Bool),
            },
            SemanticType::Type | SemanticType::Prop | SemanticType::Parameter { name: _ } => {
                return Err(format!(
                    "`{}` has no runtime realization",
                    self.source.type_text(ty)
                ));
            }
        })
    }

    /// The ADT of a closed document type, declared on first sight. The
    /// index is reserved before the fields are lowered, so a recursive type
    /// refers to itself.
    fn adt(&mut self, ty: &SemanticType) -> Result<u64, String> {
        let text = self.source.type_text(ty);
        match self.adt_index.get(&text) {
            Some(index) => return Ok(*index),
            None => {}
        }
        let index = self.adts.len() as u64;
        self.adts.push(None);
        self.adt_origins.push(AdtOrigin {
            ty: ty.clone(),
            text: text.clone(),
        });
        self.adt_index.insert(text, index);
        let shape = self.source.document(ty)?;
        let constructors = shape
            .fields
            .iter()
            .map(|fields| {
                fields
                    .iter()
                    .map(|field| self.ty(field))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.adts[index as usize] = Some(Adt { constructors });
        Ok(index)
    }

    fn pending(&mut self, pending: Pending) -> Result<(), String> {
        match pending {
            Pending::Definition {
                index,
                module,
                name,
                type_arguments,
            } => {
                let declaration = self.source.declaration(&module, &name);
                let (type_parameters, parameters, result, body) = match declaration {
                    Some(SemanticDeclaration::Definition {
                        name: _,
                        type_parameters,
                        parameters,
                        result,
                        recursive_argument: _,
                        body,
                        axioms: _,
                        executable: _,
                        mutual: _,
                        termination: _,
                        production: _,
                    }) => (type_parameters, parameters, result, body),
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
                        },
                    )
                    | None => return Err(format!("`{module}.{name}` is not a definition")),
                };
                let site = Site {
                    module: module.clone(),
                    substitution: Source::instantiation(type_parameters, &type_arguments)?,
                };
                let mut scope = Scope::default();
                let mut types = Vec::new();
                let mut names = Vec::new();
                for SemanticParameter { name, r#type } in parameters {
                    let ty = self.source.close(r#type, &site);
                    types.push(self.ty(&ty)?);
                    names.push(scope.bind(Local {
                        name: name.clone(),
                        ty,
                    }));
                }
                let result = self.ty(&self.source.close(result, &site))?;
                let body = self.term(body, &mut scope, &site, index)?;
                self.functions[index as usize] = Some(Function {
                    parameters: names,
                    types,
                    result,
                    body,
                });
                Ok(())
            }
            Pending::Instance {
                index,
                module,
                name,
            } => {
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
                        },
                    )
                    | None => return Err(format!("`{module}.{name}` is not an instance")),
                };
                let site = Site {
                    module: module.clone(),
                    substitution: BTreeMap::new(),
                };
                let class_ty = self.source.close(
                    &SemanticType::Named {
                        member: class.clone(),
                        arguments: arguments.clone(),
                    },
                    &site,
                );
                let mut scope = Scope::default();
                let body = self.record(&class_ty, fields, &mut scope, &site, index)?;
                let result = self.ty(&class_ty)?;
                self.functions[index as usize] = Some(Function {
                    parameters: Vec::new(),
                    types: Vec::new(),
                    result,
                    body,
                });
                Ok(())
            }
            Pending::Lambda {
                index,
                site,
                captures,
                parameters,
                body,
            } => {
                let mut scope = Scope::default();
                let mut types = Vec::new();
                let mut names = Vec::new();
                for local in captures.into_iter().chain(parameters) {
                    types.push(self.ty(&local.ty)?);
                    names.push(scope.bind(local));
                }
                let result_ty = self.source.infer(&body, &scope.sources(), &site)?;
                let result = self.ty(&result_ty)?;
                let body = self.term(&body, &mut scope, &site, index)?;
                self.functions[index as usize] = Some(Function {
                    parameters: names,
                    types,
                    result,
                    body,
                });
                Ok(())
            }
        }
    }

    fn terms(
        &mut self,
        terms: &[SemanticTerm],
        scope: &mut Scope,
        site: &Site,
        owner: u64,
    ) -> Result<Vec<Expr>, String> {
        terms
            .iter()
            .map(|term| self.term(term, scope, site, owner))
            .collect()
    }

    /// A record of closed type `ty`: its fields in declaration order.
    fn record(
        &mut self,
        ty: &SemanticType,
        fields: &[SemanticAssignment],
        scope: &mut Scope,
        site: &Site,
        owner: u64,
    ) -> Result<Expr, String> {
        let shape = self.source.document(ty)?;
        let mut operands = Vec::new();
        for field in &shape.field_names {
            let SemanticAssignment { field: _, value } = fields
                .iter()
                .find(|assignment| &assignment.field == field)
                .ok_or_else(|| format!("field `{field}` is not assigned"))?;
            operands.push(self.term(value, scope, site, owner)?);
        }
        if operands.len() != fields.len() {
            return Err("a record assigns a field its type does not declare".to_owned());
        }
        let target = self.ty(ty)?;
        Ok(build(Shape::Adt { constructor: 0 }, target, operands))
    }

    /// A constructor applied to its operands at its target type: a
    /// violation is the pair of Booleans it lowers to.
    fn constructed(constructor: &Constructor, target: Ty, operands: Vec<Expr>) -> Expr {
        match constructor {
            Constructor::Violation(first, second) => {
                build(Shape::Pair, target, vec![boolean(*first), boolean(*second)])
            }
            Constructor::Bool(_)
            | Constructor::Zero
            | Constructor::Succ
            | Constructor::Nil
            | Constructor::Cons
            | Constructor::OptionNone
            | Constructor::OptionSome
            | Constructor::Ok
            | Constructor::Error
            | Constructor::Document { ty: _, index: _ } => {
                build(Self::shape(constructor), target, operands)
            }
        }
    }

    /// A match over a violation: its pair, then each Boolean in turn, each
    /// leaf the branch of the constructor that pair of Booleans is. The
    /// branches are lowered in leaf order, so locals stay in first-binding
    /// order whatever order the source lists them in.
    #[allow(clippy::too_many_arguments)]
    fn violation_match(
        &mut self,
        lowered: Expr,
        scrutinee_ty: &SemanticType,
        branches: &[SemanticBranch],
        result: &Ty,
        scope: &mut Scope,
        site: &Site,
        owner: u64,
    ) -> Result<Expr, String> {
        let mut leaves: Vec<((bool, bool), &SemanticTerm)> = Vec::new();
        for branch in branches {
            let (constructor, _) = self.source.branch(branch, scrutinee_ty, site)?;
            let key = match constructor {
                Constructor::Violation(first, second) => (first, second),
                Constructor::Bool(_)
                | Constructor::Zero
                | Constructor::Succ
                | Constructor::Nil
                | Constructor::Cons
                | Constructor::OptionNone
                | Constructor::OptionSome
                | Constructor::Ok
                | Constructor::Error
                | Constructor::Document { ty: _, index: _ } => {
                    return Err("a violation match over another constructor".to_owned());
                }
            };
            if !branch.binders.is_empty() {
                return Err("a violation constructor binds nothing".to_owned());
            }
            leaves.push((key, &branch.body));
        }
        let depth = scope.locals.len();
        let first = scope.bind(Local {
            name: "__violation_first".to_owned(),
            ty: SemanticType::Bool,
        });
        let second = scope.bind(Local {
            name: "__violation_second".to_owned(),
            ty: SemanticType::Bool,
        });
        let mut halves = Vec::new();
        for first_value in [false, true] {
            let mut arms = Vec::new();
            for second_value in [false, true] {
                let body = leaves
                    .iter()
                    .find(|(key, _)| *key == (first_value, second_value))
                    .map(|(_, body)| *body)
                    .ok_or("a violation match misses a constructor")?;
                arms.push(Arm {
                    shape: if second_value {
                        Shape::True
                    } else {
                        Shape::False
                    },
                    binders: Vec::new(),
                    body: self.term(body, scope, site, owner)?,
                });
            }
            halves.push(Arm {
                shape: if first_value {
                    Shape::True
                } else {
                    Shape::False
                },
                binders: Vec::new(),
                body: Expr::Match {
                    ty: result.clone(),
                    scrutinee: Box::new(Expr::Var { name: second }),
                    arms,
                },
            });
        }
        scope.locals.truncate(depth);
        Ok(Expr::Match {
            ty: result.clone(),
            scrutinee: Box::new(lowered),
            arms: vec![Arm {
                shape: Shape::Pair,
                binders: vec![first, second],
                body: Expr::Match {
                    ty: result.clone(),
                    scrutinee: Box::new(Expr::Var { name: first }),
                    arms: halves,
                },
            }],
        })
    }

    /// The shape of a constructor at its closed type.
    fn shape(constructor: &Constructor) -> Shape {
        match constructor {
            Constructor::Bool(true) => Shape::True,
            Constructor::Bool(false) => Shape::False,
            Constructor::Zero => Shape::Zero,
            Constructor::Succ => Shape::Succ,
            Constructor::Nil => Shape::Nil,
            Constructor::Cons => Shape::Cons,
            Constructor::OptionNone => Shape::None,
            Constructor::OptionSome => Shape::Some,
            Constructor::Ok => Shape::Ok,
            Constructor::Error => Shape::Error,
            Constructor::Violation(_, _) => Shape::Pair,
            Constructor::Document { ty: _, index } => Shape::Adt {
                constructor: *index as u64,
            },
        }
    }

    /// A cons chain of `items` onto `nil` at list type `list`.
    fn list(list: &Ty, items: Vec<Expr>) -> Expr {
        items
            .into_iter()
            .rev()
            .fold(build(Shape::Nil, list.clone(), Vec::new()), |tail, head| {
                build(Shape::Cons, list.clone(), vec![head, tail])
            })
    }

    #[allow(clippy::too_many_lines)]
    fn term(
        &mut self,
        term: &SemanticTerm,
        scope: &mut Scope,
        site: &Site,
        owner: u64,
    ) -> Result<Expr, String> {
        Ok(match term {
            SemanticTerm::Var { name } => Expr::Var {
                name: scope.lookup(name)?.1,
            },
            SemanticTerm::Nat { value } => nat_value(value),
            SemanticTerm::Integer {
                representation,
                value,
            } => {
                let (ty, value) = integer_value(*representation, value);
                Expr::Value { ty, value }
            }
            SemanticTerm::String { value } => Expr::Value {
                ty: Ty::String,
                value: Value::String {
                    value: value.clone(),
                },
            },
            SemanticTerm::Bytes { hex } => Expr::Value {
                ty: Ty::Bytes,
                value: Value::Bytes { hex: hex.clone() },
            },
            SemanticTerm::Primitive {
                operation,
                arguments,
                result,
            } => self.primitive(*operation, arguments, result, scope, site, owner)?,
            SemanticTerm::Bool { value } => boolean(*value),
            SemanticTerm::Unit => build(Shape::Unit, Ty::Unit, Vec::new()),
            SemanticTerm::Nil { element } => {
                let ty = self.ty(&SemanticType::List {
                    element: Box::new(self.source.close(element, site)),
                })?;
                build(Shape::Nil, ty, Vec::new())
            }
            SemanticTerm::Cons { head, tail } => {
                let ty = self.source.infer(term, &scope.sources(), site)?;
                let ty = self.ty(&ty)?;
                let head = self.term(head, scope, site, owner)?;
                let tail = self.term(tail, scope, site, owner)?;
                build(Shape::Cons, ty, vec![head, tail])
            }
            SemanticTerm::Record {
                r#type: _,
                type_arguments: _,
                fields,
            } => {
                let ty = self.source.infer(term, &scope.sources(), site)?;
                self.record(&ty, fields, scope, site, owner)?
            }
            SemanticTerm::Constructor {
                constructor,
                type_arguments,
                arguments,
            } => {
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.source.close(argument, site))
                    .collect();
                let (constructor, ty) = self.source.constructor(constructor, &closed, site)?;
                let target = self.ty(&ty)?;
                let operands = self.terms(arguments, scope, site, owner)?;
                Self::constructed(&constructor, target, operands)
            }
            SemanticTerm::InstanceValue {
                class: _,
                arguments: _,
                resolved,
            } => {
                let module = Source::module_of(resolved, site);
                let function = self.class_instance(&module, &resolved.name);
                Expr::Call {
                    function,
                    operands: Vec::new(),
                }
            }
            SemanticTerm::Project { value, field } => {
                let ty = self.source.infer(value, &scope.sources(), site)?;
                let shape = self.source.document(&ty)?;
                let index = shape
                    .field_names
                    .iter()
                    .position(|name| name == field)
                    .ok_or_else(|| {
                        format!("`{}` has no field `{field}`", self.source.type_text(&ty))
                    })?;
                Expr::Field {
                    value: Box::new(self.term(value, scope, site, owner)?),
                    index: index as u64,
                }
            }
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } => {
                let module = Source::module_of(function, site);
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.source.close(argument, site))
                    .collect();
                let function = self.definition(&module, &function.name, &closed)?;
                Expr::Call {
                    function,
                    operands: self.terms(arguments, scope, site, owner)?,
                }
            }
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => cond(
                self.term(condition, scope, site, owner)?,
                self.term(then_value, scope, site, owner)?,
                self.term(else_value, scope, site, owner)?,
            ),
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => {
                let scrutinee_ty = self.source.infer(scrutinee, &scope.sources(), site)?;
                let result = self.source.infer(term, &scope.sources(), site)?;
                let result = self.ty(&result)?;
                let lowered = self.term(scrutinee, scope, site, owner)?;
                if is_violation(&scrutinee_ty) {
                    return self.violation_match(
                        lowered,
                        &scrutinee_ty,
                        branches,
                        &result,
                        scope,
                        site,
                        owner,
                    );
                }
                let mut arms = Vec::new();
                for branch in branches {
                    let SemanticBranch {
                        constructor: _,
                        binders,
                        body,
                    } = branch;
                    let (constructor, fields) = self.source.branch(branch, &scrutinee_ty, site)?;
                    if fields.len() != binders.len() {
                        return Err(format!(
                            "branch `{}` binds {} of {} field(s)",
                            branch.constructor.name,
                            binders.len(),
                            fields.len()
                        ));
                    }
                    let depth = scope.locals.len();
                    let names = binders
                        .iter()
                        .zip(fields)
                        .map(|(name, ty)| {
                            scope.bind(Local {
                                name: name.clone(),
                                ty,
                            })
                        })
                        .collect();
                    let body = self.term(body, scope, site, owner)?;
                    scope.locals.truncate(depth);
                    arms.push(Arm {
                        shape: Self::shape(&constructor),
                        binders: names,
                        body,
                    });
                }
                Expr::Match {
                    ty: result,
                    scrutinee: Box::new(lowered),
                    arms,
                }
            }
            SemanticTerm::Add { left, right } => prim(
                Prim::NatAdd,
                vec![
                    self.term(left, scope, site, owner)?,
                    self.term(right, scope, site, owner)?,
                ],
            ),
            SemanticTerm::Beq { left, right } => prim(
                Prim::NatEq,
                vec![
                    self.term(left, scope, site, owner)?,
                    self.term(right, scope, site, owner)?,
                ],
            ),
            SemanticTerm::Ble { left, right } => prim(
                Prim::NatLe,
                vec![
                    self.term(left, scope, site, owner)?,
                    self.term(right, scope, site, owner)?,
                ],
            ),
            SemanticTerm::Blt { left, right } => prim(
                Prim::NatLt,
                vec![
                    self.term(left, scope, site, owner)?,
                    self.term(right, scope, site, owner)?,
                ],
            ),
            // `&&` and `||` evaluate their right operand only when the left
            // does not decide the result (§17.14).
            SemanticTerm::And { left, right } => cond(
                self.term(left, scope, site, owner)?,
                self.term(right, scope, site, owner)?,
                boolean(false),
            ),
            SemanticTerm::Or { left, right } => cond(
                self.term(left, scope, site, owner)?,
                boolean(true),
                self.term(right, scope, site, owner)?,
            ),
            SemanticTerm::Not { value } => {
                prim(Prim::BoolNot, vec![self.term(value, scope, site, owner)?])
            }
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => {
                let ty = self.source.close(&binder.r#type, site);
                let target = self.ty(&ty)?;
                let bound = self.term(value, scope, site, owner)?;
                let depth = scope.locals.len();
                let name = scope.bind(Local {
                    name: binder.name.clone(),
                    ty,
                });
                let body = self.term(body, scope, site, owner)?;
                scope.locals.truncate(depth);
                Expr::Let {
                    name,
                    ty: target,
                    bound: Box::new(bound),
                    body: Box::new(body),
                }
            }
            SemanticTerm::Pair { left, right } => {
                let ty = self.source.infer(term, &scope.sources(), site)?;
                let ty = self.ty(&ty)?;
                build(
                    Shape::Pair,
                    ty,
                    vec![
                        self.term(left, scope, site, owner)?,
                        self.term(right, scope, site, owner)?,
                    ],
                )
            }
            SemanticTerm::First { value } => Expr::First {
                value: Box::new(self.term(value, scope, site, owner)?),
            },
            SemanticTerm::Second { value } => Expr::Second {
                value: Box::new(self.term(value, scope, site, owner)?),
            },
            SemanticTerm::Lambda {
                parameters,
                captures,
                body,
            } => {
                let ordinal = self.lambdas.entry(owner).or_insert(0);
                let this_ordinal = *ordinal;
                *ordinal += 1;
                let index = self.reserve(Origin::Lambda {
                    owner,
                    ordinal: this_ordinal,
                });
                let mut captured = Vec::new();
                let mut operands = Vec::new();
                for capture in captures {
                    let (local, name) = scope.lookup(capture)?;
                    captured.push(local.clone());
                    operands.push(Expr::Var { name: *name });
                }
                let parameters = parameters
                    .iter()
                    .map(|SemanticParameter { name, r#type }| Local {
                        name: name.clone(),
                        ty: self.source.close(r#type, site),
                    })
                    .collect();
                self.queue.push_back(Pending::Lambda {
                    index,
                    site: site.clone(),
                    captures: captured,
                    parameters,
                    body: body.as_ref().clone(),
                });
                Expr::Closure {
                    function: index,
                    captures: operands,
                }
            }
            SemanticTerm::Apply {
                function,
                arguments,
            } => Expr::Apply {
                target: Box::new(self.term(function, scope, site, owner)?),
                operands: self.terms(arguments, scope, site, owner)?,
            },
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                let module = Source::module_of(function, site);
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.source.close(argument, site))
                    .collect();
                Expr::Closure {
                    function: self.definition(&module, &function.name, &closed)?,
                    captures: Vec::new(),
                }
            }
            SemanticTerm::MapLiteral {
                key,
                value,
                entries,
            } => {
                let entry = SemanticType::Product {
                    left: Box::new(self.source.close(key, site)),
                    right: Box::new(self.source.close(value, site)),
                };
                let pair = self.ty(&entry)?;
                let list = self.ty(&SemanticType::List {
                    element: Box::new(entry),
                })?;
                let mut items = Vec::new();
                for SemanticMapEntry { key, value } in entries {
                    items.push(build(
                        Shape::Pair,
                        pair.clone(),
                        vec![
                            self.term(key, scope, site, owner)?,
                            self.term(value, scope, site, owner)?,
                        ],
                    ));
                }
                Self::list(&list, items)
            }
            SemanticTerm::SetLiteral { element, elements } => {
                let list = self.ty(&SemanticType::List {
                    element: Box::new(self.source.close(element, site)),
                })?;
                let items = self.terms(elements, scope, site, owner)?;
                Self::list(&list, items)
            }
            // Every backend reads elaborated declarations, in which a checked
            // application is the ordinary term it means (§17.12).
            SemanticTerm::CheckedApply {
                model: _,
                type_arguments: _,
                arguments: _,
                checks: _,
            } => {
                return Err(
                    "a checked model application is lowered only through its elaboration"
                        .to_owned(),
                );
            }
            // The Lean rendering lists each node with its targets in edge
            // order; the realization builds exactly that list.
            SemanticTerm::GraphLiteral { node, nodes, edges } => {
                let node = self.source.close(node, site);
                let targets = self.ty(&SemanticType::List {
                    element: Box::new(node.clone()),
                })?;
                let entry = SemanticType::Product {
                    left: Box::new(node.clone()),
                    right: Box::new(SemanticType::List {
                        element: Box::new(node),
                    }),
                };
                let pair = self.ty(&entry)?;
                let list = self.ty(&SemanticType::List {
                    element: Box::new(entry),
                })?;
                let mut items = Vec::new();
                for source in nodes {
                    let mut successors = Vec::new();
                    for SemanticEdge {
                        source: from,
                        target,
                    } in edges
                    {
                        if from == source {
                            successors.push(self.term(target, scope, site, owner)?);
                        }
                    }
                    items.push(build(
                        Shape::Pair,
                        pair.clone(),
                        vec![
                            self.term(source, scope, site, owner)?,
                            Self::list(&targets, successors),
                        ],
                    ));
                }
                Self::list(&list, items)
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
                    "the formal-only construct `{}` reached lowering",
                    super::eligibility::term_key(term)
                ));
            }
        })
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn primitive(
        &mut self,
        operation: SemanticPrimitive,
        arguments: &[SemanticTerm],
        result: &SemanticType,
        scope: &mut Scope,
        site: &Site,
        owner: u64,
    ) -> Result<Expr, String> {
        let locals = scope.sources();
        let types = arguments
            .iter()
            .map(|argument| self.source.infer(argument, &locals, site))
            .collect::<Result<Vec<_>, _>>()?;
        let result = self.source.close(result, site);
        match template_of(operation, &types)? {
            Some((template, instance)) => {
                let function = self.template(template, &instance)?;
                return Ok(Expr::Call {
                    function,
                    operands: self.terms(arguments, scope, site, owner)?,
                });
            }
            None => {}
        }
        let first = types
            .first()
            .ok_or_else(|| format!("{operation:?} has no operand"))?;
        let integer = |nat: Prim, int: Prim| -> Result<Prim, String> {
            match first {
                SemanticType::Nat => Ok(nat),
                SemanticType::Int => Ok(int),
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
                | SemanticType::ReasoningFailure => Err(format!("{operation:?} of a non-integer")),
            }
        };
        let option_value = || -> Result<SemanticType, String> {
            match &result {
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
                | SemanticType::ReasoningFailure => {
                    Err(format!("{operation:?} does not return an option"))
                }
            }
        };
        let operation = match operation {
            SemanticPrimitive::Subtract => integer(Prim::NatSub, Prim::IntSub)?,
            SemanticPrimitive::Multiply => integer(Prim::NatMul, Prim::IntMul)?,
            SemanticPrimitive::Quotient => integer(Prim::NatQuot, Prim::IntQuot)?,
            SemanticPrimitive::Remainder => integer(Prim::NatRem, Prim::IntRem)?,
            SemanticPrimitive::Negate => Prim::IntNeg,
            SemanticPrimitive::CheckedConvert => Prim::Convert {
                target: fixed_kind(&option_value()?).ok_or("a conversion to a non-fixed width")?,
            },
            SemanticPrimitive::CheckedAdd => Prim::CheckedAdd,
            SemanticPrimitive::CheckedSubtract => Prim::CheckedSub,
            SemanticPrimitive::CheckedMultiply => Prim::CheckedMul,
            SemanticPrimitive::CheckedNegate => Prim::CheckedNeg,
            SemanticPrimitive::CheckedQuotient => Prim::CheckedQuot,
            SemanticPrimitive::BitAnd => Prim::BitAnd,
            SemanticPrimitive::BitOr => Prim::BitOr,
            SemanticPrimitive::BitXor => Prim::BitXor,
            SemanticPrimitive::BitNot => Prim::BitNot,
            SemanticPrimitive::ShiftLeft => Prim::ShiftLeft,
            SemanticPrimitive::ShiftRight => Prim::ShiftRight,
            SemanticPrimitive::Append => Prim::Append,
            SemanticPrimitive::Length | SemanticPrimitive::MapSize | SemanticPrimitive::SetSize => {
                Prim::Length
            }
            SemanticPrimitive::Index => Prim::Index,
            SemanticPrimitive::Slice => Prim::Slice,
            SemanticPrimitive::Utf8Encode => Prim::Utf8Encode,
            SemanticPrimitive::Utf8Decode => Prim::Utf8Decode,
            SemanticPrimitive::CompareBytes => Prim::CompareBytes,
            SemanticPrimitive::Equal => Prim::Equal,
            SemanticPrimitive::SplitExact => Prim::SplitExact,
            SemanticPrimitive::Join => Prim::Join,
            SemanticPrimitive::ParseDecimal => Prim::ParseDecimal {
                target: self.ty(&option_value()?)?,
            },
            SemanticPrimitive::FormatDecimal => Prim::FormatDecimal,
            // A key comparison is the realization's `compare` and a match on
            // the order it returns (§17.14 `primitive.less_than`).
            SemanticPrimitive::LessThan => {
                let order = prim(Prim::Compare, self.terms(arguments, scope, site, owner)?);
                let arm = |shape: Shape, value: bool| Arm {
                    shape,
                    binders: Vec::new(),
                    body: boolean(value),
                };
                return Ok(Expr::Match {
                    ty: Ty::Bool,
                    scrutinee: Box::new(order),
                    arms: vec![
                        arm(Shape::Lt, true),
                        arm(Shape::Eq, false),
                        arm(Shape::Gt, false),
                    ],
                });
            }
            // The entry and element lists are the realized representation
            // itself (§17.14 `representation`).
            SemanticPrimitive::MapEntries | SemanticPrimitive::SetElements => {
                if arguments.len() != 1 {
                    return Err(format!("{operation:?} takes one operand"));
                }
                return self.term(&arguments[0], scope, site, owner);
            }
            SemanticPrimitive::MapInsert
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
                return Err(format!("{operation:?} has no template"));
            }
        };
        Ok(prim(operation, self.terms(arguments, scope, site, owner)?))
    }
}

/// The components of a closed product type.
fn product(ty: SemanticType) -> Result<(SemanticType, SemanticType), String> {
    match ty {
        SemanticType::Product { left, right } => Ok((*left, *right)),
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
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => Err("a projection of a non-product".to_owned()),
    }
}

impl Source<'_> {
    /// The closed type of a term read at `site` with `locals` in scope.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn infer(
        &self,
        term: &SemanticTerm,
        locals: &[Local],
        site: &Site,
    ) -> Result<SemanticType, String> {
        let local = |name: &str| -> Result<SemanticType, String> {
            locals
                .iter()
                .rev()
                .find(|local| local.name == name)
                .map(|local| local.ty.clone())
                .ok_or_else(|| format!("local `{name}` is unbound"))
        };
        let extended = |extra: Vec<Local>| -> Vec<Local> {
            let mut out = locals.to_vec();
            out.extend(extra);
            out
        };
        Ok(match term {
            SemanticTerm::Var { name } => local(name)?,
            SemanticTerm::Nat { value: _ } | SemanticTerm::Add { left: _, right: _ } => {
                SemanticType::Nat
            }
            SemanticTerm::Integer {
                representation,
                value: _,
            } => integer_type(*representation),
            SemanticTerm::String { value: _ } => SemanticType::String,
            SemanticTerm::Bytes { hex: _ } => SemanticType::Bytes,
            SemanticTerm::Primitive {
                operation: _,
                arguments: _,
                result,
            } => self.close(result, site),
            SemanticTerm::Bool { value: _ }
            | SemanticTerm::Beq { left: _, right: _ }
            | SemanticTerm::Ble { left: _, right: _ }
            | SemanticTerm::Blt { left: _, right: _ }
            | SemanticTerm::And { left: _, right: _ }
            | SemanticTerm::Or { left: _, right: _ }
            | SemanticTerm::Not { value: _ } => SemanticType::Bool,
            SemanticTerm::Unit => SemanticType::Unit,
            SemanticTerm::Nil { element } => SemanticType::List {
                element: Box::new(self.close(element, site)),
            },
            SemanticTerm::Cons { head: _, tail } => self.infer(tail, locals, site)?,
            SemanticTerm::Record {
                r#type,
                type_arguments,
                fields: _,
            } => self.close(
                &SemanticType::Named {
                    member: r#type.clone(),
                    arguments: type_arguments.clone(),
                },
                site,
            ),
            SemanticTerm::Constructor {
                constructor,
                type_arguments,
                arguments: _,
            } => {
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.close(argument, site))
                    .collect();
                self.constructor(constructor, &closed, site)?.1
            }
            SemanticTerm::InstanceValue {
                class,
                arguments,
                resolved: _,
            } => self.close(
                &SemanticType::Named {
                    member: class.clone(),
                    arguments: arguments.clone(),
                },
                site,
            ),
            SemanticTerm::Project { value, field } => {
                let ty = self.infer(value, locals, site)?;
                let shape = self.document(&ty)?;
                let index = shape
                    .field_names
                    .iter()
                    .position(|name| name == field)
                    .ok_or_else(|| format!("no field `{field}`"))?;
                shape.fields[0][index].clone()
            }
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments: _,
            } => {
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.close(argument, site))
                    .collect();
                self.signature(&Self::module_of(function, site), &function.name, &closed)?
                    .1
            }
            SemanticTerm::If {
                condition: _,
                then_value,
                else_value: _,
            } => self.infer(then_value, locals, site)?,
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => {
                let scrutinee = self.infer(scrutinee, locals, site)?;
                let branch = branches.first().ok_or("an empty match")?;
                let (_, fields) = self.branch(branch, &scrutinee, site)?;
                let bound = branch
                    .binders
                    .iter()
                    .zip(fields)
                    .map(|(name, ty)| Local {
                        name: name.clone(),
                        ty,
                    })
                    .collect();
                self.infer(&branch.body, &extended(bound), site)?
            }
            SemanticTerm::Let {
                binder,
                value: _,
                body,
            } => self.infer(
                body,
                &extended(vec![Local {
                    name: binder.name.clone(),
                    ty: self.close(&binder.r#type, site),
                }]),
                site,
            )?,
            SemanticTerm::Pair { left, right } => SemanticType::Product {
                left: Box::new(self.infer(left, locals, site)?),
                right: Box::new(self.infer(right, locals, site)?),
            },
            SemanticTerm::First { value } => product(self.infer(value, locals, site)?)?.0,
            SemanticTerm::Second { value } => product(self.infer(value, locals, site)?)?.1,
            SemanticTerm::Lambda {
                parameters,
                captures: _,
                body,
            } => {
                let bound: Vec<Local> = parameters
                    .iter()
                    .map(|SemanticParameter { name, r#type }| Local {
                        name: name.clone(),
                        ty: self.close(r#type, site),
                    })
                    .collect();
                SemanticType::Function {
                    parameters: bound.iter().map(|local| local.ty.clone()).collect(),
                    result: Box::new(self.infer(body, &extended(bound), site)?),
                }
            }
            SemanticTerm::Apply {
                function,
                arguments: _,
            } => match self.infer(function, locals, site)? {
                SemanticType::Function {
                    parameters: _,
                    result,
                } => *result,
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
                | SemanticType::ReasoningFailure => {
                    return Err("an application of a non-function".to_owned());
                }
            },
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.close(argument, site))
                    .collect();
                let (parameters, result) =
                    self.signature(&Self::module_of(function, site), &function.name, &closed)?;
                SemanticType::Function {
                    parameters: parameters
                        .into_iter()
                        .map(|SemanticParameter { name: _, r#type }| r#type)
                        .collect(),
                    result: Box::new(result),
                }
            }
            SemanticTerm::MapLiteral {
                key,
                value,
                entries: _,
            } => SemanticType::Map {
                key: Box::new(self.close(key, site)),
                value: Box::new(self.close(value, site)),
            },
            SemanticTerm::SetLiteral {
                element,
                elements: _,
            } => SemanticType::Set {
                element: Box::new(self.close(element, site)),
            },
            // Every backend reads elaborated declarations, in which a checked
            // application is the ordinary term it means (§17.12).
            SemanticTerm::CheckedApply {
                model: _,
                type_arguments: _,
                arguments: _,
                checks: _,
            } => {
                return Err(
                    "a checked model application is lowered only through its elaboration"
                        .to_owned(),
                );
            }
            SemanticTerm::GraphLiteral {
                node,
                nodes: _,
                edges: _,
            } => {
                let node = self.close(node, site);
                SemanticType::Map {
                    key: Box::new(node.clone()),
                    value: Box::new(SemanticType::Set {
                        element: Box::new(node),
                    }),
                }
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
            | SemanticTerm::Forall { binder: _, body: _ } => SemanticType::Prop,
        })
    }
}
