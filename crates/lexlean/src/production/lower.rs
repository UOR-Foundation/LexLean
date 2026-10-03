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

use std::collections::{BTreeMap, VecDeque};

use super::eligibility::{integer_type, LinkedModule};
use super::source::{Constructor, Local, Site, Source};
use super::RootReport;
use crate::calculus::library::Template;
use crate::calculus::{Adt, Arm, Expr, Function, IntKind, Prim, Program, Shape, Ty, Value};
use crate::code;
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
}

/// Lower one eligible root. A failure is a compiler defect: eligibility
/// already admitted every construct the closure reaches.
///
/// # Errors
///
/// Returns `LLI9001` naming the construct that could not be lowered, or a
/// disagreement between the lowered closure and the eligibility report.
pub fn lower_root(
    modules: &BTreeMap<String, LinkedModule<'_>>,
    module: &str,
    name: &str,
    report: &RootReport,
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
    };
    let root = lowerer.definition(module, name, &[]).map_err(internal)?;
    if root != 0 {
        return Err(internal("the root is not function 0"));
    }
    loop {
        match lowerer.queue.pop_front() {
            Some(pending) => lowerer.pending(pending).map_err(internal)?,
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
            } => None,
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
    Ok(Lowered {
        program,
        layout: Layout {
            functions: lowerer.origins,
            adts: lowerer.adt_origins,
        },
    })
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
        SemanticType::ContractViolation => true,
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
        | SemanticType::ContractViolation => None,
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
            | SemanticType::ContractViolation => Err(format!("{operation:?} of a non-map")),
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
            | SemanticType::ContractViolation => Err(format!("{operation:?} of a non-collection")),
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

    /// The target type of a closed source type.
    fn ty(&mut self, ty: &SemanticType) -> Result<Ty, String> {
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
            SemanticType::ContractViolation => Ty::Pair {
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
    fn violation_match(
        &mut self,
        lowered: Expr,
        branches: &[SemanticBranch],
        result: &Ty,
        scope: &mut Scope,
        site: &Site,
        owner: u64,
    ) -> Result<Expr, String> {
        let mut leaves: Vec<((bool, bool), &SemanticTerm)> = Vec::new();
        for branch in branches {
            let (constructor, _) =
                self.source
                    .branch(branch, &SemanticType::ContractViolation, site)?;
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
                    return self.violation_match(lowered, branches, &result, scope, site, owner);
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
                | SemanticType::ContractViolation => Err(format!("{operation:?} of a non-integer")),
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
                | SemanticType::ContractViolation => {
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
        | SemanticType::ContractViolation => Err("a projection of a non-product".to_owned()),
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
                | SemanticType::ContractViolation => {
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
