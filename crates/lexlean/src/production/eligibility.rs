//! The production-eligibility analysis (SPEC.md §17.13).
//!
//! Every semantic construct is mapped to its registry key by an exhaustive
//! match that names every variant and destructures every field. This file
//! admits no wildcard or binding catch-all arm, no tuple default, no rest
//! pattern, no equality test on the IR, no `if let`, and no `matches!`:
//! a new construct or a new field cannot be admitted to production by a
//! default branch, and `cargo xtask validate-model` (audit-production)
//! rejects any such pattern planted here.

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

use super::{
    registry, BoundaryRow, ClosureMember, Disposition, EffectRow, ModuleReport, ReasoningRow,
    Registry, RootReport, TargetRow,
};
use crate::ir::semantic::{
    MemberRef, SemanticAssignment, SemanticBranch, SemanticConstructor, SemanticDeclaration,
    SemanticEdge, SemanticField, SemanticInteger, SemanticMapEntry, SemanticModule,
    SemanticParameter, SemanticPrimitive, SemanticProduction, SemanticTerm, SemanticTermination,
    SemanticType,
};

/// The owners of the built-in constructors linking admits without a module.
#[derive(Clone, Copy)]
enum BuiltinOwner {
    Bool,
    ContractViolation,
    Nat,
    List,
    Option,
    ReasoningFailure,
    Result,
}

impl BuiltinOwner {
    /// The number of type arguments a constructor of this type carries.
    const fn arity(self) -> usize {
        match self {
            Self::Bool | Self::ContractViolation | Self::Nat | Self::ReasoningFailure => 0,
            Self::List | Self::Option => 1,
            Self::Result => 2,
        }
    }
}

const BUILTIN_OWNERS: [(&str, BuiltinOwner); 7] = [
    ("Bool", BuiltinOwner::Bool),
    ("ContractViolation", BuiltinOwner::ContractViolation),
    ("Nat", BuiltinOwner::Nat),
    ("List", BuiltinOwner::List),
    ("Option", BuiltinOwner::Option),
    ("ReasoningFailure", BuiltinOwner::ReasoningFailure),
    ("Result", BuiltinOwner::Result),
];

/// One linked semantic module as the analysis sees it.
#[derive(Clone, Copy)]
pub struct LinkedModule<'a> {
    /// The generated Lean module name.
    pub lean_module: &'a str,
    /// The validated semantic module.
    pub semantic: &'a SemanticModule,
}

/// Why the analysis of a module stopped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalysisError {
    /// A root is not eligible for one of its targets (`LLT4005`).
    Ineligible(String),
    /// The embedded registry or a linking guarantee failed: a compiler
    /// defect, never a property of the source (`LLI9001`).
    Internal(String),
}

/// A reason a root is not eligible.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Violation {
    /// The call-path length, so the violation nearest the root is reported.
    depth: usize,
    instance: String,
    construct: String,
    reason: String,
}

/// The registry key of a primitive operation.
#[must_use]
pub const fn primitive_key(operation: SemanticPrimitive) -> &'static str {
    match operation {
        SemanticPrimitive::Subtract => "primitive.subtract",
        SemanticPrimitive::Multiply => "primitive.multiply",
        SemanticPrimitive::Quotient => "primitive.quotient",
        SemanticPrimitive::Remainder => "primitive.remainder",
        SemanticPrimitive::Negate => "primitive.negate",
        SemanticPrimitive::CheckedConvert => "primitive.checked_convert",
        SemanticPrimitive::CheckedAdd => "primitive.checked_add",
        SemanticPrimitive::CheckedSubtract => "primitive.checked_subtract",
        SemanticPrimitive::CheckedMultiply => "primitive.checked_multiply",
        SemanticPrimitive::CheckedNegate => "primitive.checked_negate",
        SemanticPrimitive::CheckedQuotient => "primitive.checked_quotient",
        SemanticPrimitive::BitAnd => "primitive.bit_and",
        SemanticPrimitive::BitOr => "primitive.bit_or",
        SemanticPrimitive::BitXor => "primitive.bit_xor",
        SemanticPrimitive::BitNot => "primitive.bit_not",
        SemanticPrimitive::ShiftLeft => "primitive.shift_left",
        SemanticPrimitive::ShiftRight => "primitive.shift_right",
        SemanticPrimitive::Append => "primitive.append",
        SemanticPrimitive::Length => "primitive.length",
        SemanticPrimitive::Index => "primitive.index",
        SemanticPrimitive::Slice => "primitive.slice",
        SemanticPrimitive::Utf8Encode => "primitive.utf8_encode",
        SemanticPrimitive::Utf8Decode => "primitive.utf8_decode",
        SemanticPrimitive::CompareBytes => "primitive.compare_bytes",
        SemanticPrimitive::Equal => "primitive.equal",
        SemanticPrimitive::SplitExact => "primitive.split_exact",
        SemanticPrimitive::Join => "primitive.join",
        SemanticPrimitive::ParseDecimal => "primitive.parse_decimal",
        SemanticPrimitive::FormatDecimal => "primitive.format_decimal",
        SemanticPrimitive::MapInsert => "primitive.map_insert",
        SemanticPrimitive::MapRemove => "primitive.map_remove",
        SemanticPrimitive::MapLookup => "primitive.map_lookup",
        SemanticPrimitive::MapContains => "primitive.map_contains",
        SemanticPrimitive::MapSize => "primitive.map_size",
        SemanticPrimitive::MapKeys => "primitive.map_keys",
        SemanticPrimitive::MapValues => "primitive.map_values",
        SemanticPrimitive::MapEntries => "primitive.map_entries",
        SemanticPrimitive::MapFold => "primitive.map_fold",
        SemanticPrimitive::SetInsert => "primitive.set_insert",
        SemanticPrimitive::SetRemove => "primitive.set_remove",
        SemanticPrimitive::SetContains => "primitive.set_contains",
        SemanticPrimitive::SetSize => "primitive.set_size",
        SemanticPrimitive::SetElements => "primitive.set_elements",
        SemanticPrimitive::SetUnion => "primitive.set_union",
        SemanticPrimitive::SetIntersection => "primitive.set_intersection",
        SemanticPrimitive::SetDifference => "primitive.set_difference",
        SemanticPrimitive::SetFold => "primitive.set_fold",
        SemanticPrimitive::ListFold => "primitive.list_fold",
        SemanticPrimitive::Iterate => "primitive.iterate",
        SemanticPrimitive::IterateUntil => "primitive.iterate_until",
        SemanticPrimitive::GraphSuccessors => "primitive.graph_successors",
        SemanticPrimitive::GraphReachable => "primitive.graph_reachable",
        SemanticPrimitive::GraphTopological => "primitive.graph_topological",
        SemanticPrimitive::LessThan => "primitive.less_than",
    }
}

/// Every primitive operation, in declaration order. [`primitive_index`] is
/// exhaustive, so a new operation cannot be added without extending it, and
/// the conformance suite checks the two agree.
pub const PRIMITIVES: [SemanticPrimitive; 54] = [
    SemanticPrimitive::Subtract,
    SemanticPrimitive::Multiply,
    SemanticPrimitive::Quotient,
    SemanticPrimitive::Remainder,
    SemanticPrimitive::Negate,
    SemanticPrimitive::CheckedConvert,
    SemanticPrimitive::CheckedAdd,
    SemanticPrimitive::CheckedSubtract,
    SemanticPrimitive::CheckedMultiply,
    SemanticPrimitive::CheckedNegate,
    SemanticPrimitive::CheckedQuotient,
    SemanticPrimitive::BitAnd,
    SemanticPrimitive::BitOr,
    SemanticPrimitive::BitXor,
    SemanticPrimitive::BitNot,
    SemanticPrimitive::ShiftLeft,
    SemanticPrimitive::ShiftRight,
    SemanticPrimitive::Append,
    SemanticPrimitive::Length,
    SemanticPrimitive::Index,
    SemanticPrimitive::Slice,
    SemanticPrimitive::Utf8Encode,
    SemanticPrimitive::Utf8Decode,
    SemanticPrimitive::CompareBytes,
    SemanticPrimitive::Equal,
    SemanticPrimitive::SplitExact,
    SemanticPrimitive::Join,
    SemanticPrimitive::ParseDecimal,
    SemanticPrimitive::FormatDecimal,
    SemanticPrimitive::MapInsert,
    SemanticPrimitive::MapRemove,
    SemanticPrimitive::MapLookup,
    SemanticPrimitive::MapContains,
    SemanticPrimitive::MapSize,
    SemanticPrimitive::MapKeys,
    SemanticPrimitive::MapValues,
    SemanticPrimitive::MapEntries,
    SemanticPrimitive::MapFold,
    SemanticPrimitive::SetInsert,
    SemanticPrimitive::SetRemove,
    SemanticPrimitive::SetContains,
    SemanticPrimitive::SetSize,
    SemanticPrimitive::SetElements,
    SemanticPrimitive::SetUnion,
    SemanticPrimitive::SetIntersection,
    SemanticPrimitive::SetDifference,
    SemanticPrimitive::SetFold,
    SemanticPrimitive::ListFold,
    SemanticPrimitive::Iterate,
    SemanticPrimitive::IterateUntil,
    SemanticPrimitive::GraphSuccessors,
    SemanticPrimitive::GraphReachable,
    SemanticPrimitive::GraphTopological,
    SemanticPrimitive::LessThan,
];

/// The position of an operation in [`PRIMITIVES`].
#[must_use]
pub const fn primitive_index(operation: SemanticPrimitive) -> usize {
    match operation {
        SemanticPrimitive::Subtract => 0,
        SemanticPrimitive::Multiply => 1,
        SemanticPrimitive::Quotient => 2,
        SemanticPrimitive::Remainder => 3,
        SemanticPrimitive::Negate => 4,
        SemanticPrimitive::CheckedConvert => 5,
        SemanticPrimitive::CheckedAdd => 6,
        SemanticPrimitive::CheckedSubtract => 7,
        SemanticPrimitive::CheckedMultiply => 8,
        SemanticPrimitive::CheckedNegate => 9,
        SemanticPrimitive::CheckedQuotient => 10,
        SemanticPrimitive::BitAnd => 11,
        SemanticPrimitive::BitOr => 12,
        SemanticPrimitive::BitXor => 13,
        SemanticPrimitive::BitNot => 14,
        SemanticPrimitive::ShiftLeft => 15,
        SemanticPrimitive::ShiftRight => 16,
        SemanticPrimitive::Append => 17,
        SemanticPrimitive::Length => 18,
        SemanticPrimitive::Index => 19,
        SemanticPrimitive::Slice => 20,
        SemanticPrimitive::Utf8Encode => 21,
        SemanticPrimitive::Utf8Decode => 22,
        SemanticPrimitive::CompareBytes => 23,
        SemanticPrimitive::Equal => 24,
        SemanticPrimitive::SplitExact => 25,
        SemanticPrimitive::Join => 26,
        SemanticPrimitive::ParseDecimal => 27,
        SemanticPrimitive::FormatDecimal => 28,
        SemanticPrimitive::MapInsert => 29,
        SemanticPrimitive::MapRemove => 30,
        SemanticPrimitive::MapLookup => 31,
        SemanticPrimitive::MapContains => 32,
        SemanticPrimitive::MapSize => 33,
        SemanticPrimitive::MapKeys => 34,
        SemanticPrimitive::MapValues => 35,
        SemanticPrimitive::MapEntries => 36,
        SemanticPrimitive::MapFold => 37,
        SemanticPrimitive::SetInsert => 38,
        SemanticPrimitive::SetRemove => 39,
        SemanticPrimitive::SetContains => 40,
        SemanticPrimitive::SetSize => 41,
        SemanticPrimitive::SetElements => 42,
        SemanticPrimitive::SetUnion => 43,
        SemanticPrimitive::SetIntersection => 44,
        SemanticPrimitive::SetDifference => 45,
        SemanticPrimitive::SetFold => 46,
        SemanticPrimitive::ListFold => 47,
        SemanticPrimitive::Iterate => 48,
        SemanticPrimitive::IterateUntil => 49,
        SemanticPrimitive::GraphSuccessors => 50,
        SemanticPrimitive::GraphReachable => 51,
        SemanticPrimitive::GraphTopological => 52,
        SemanticPrimitive::LessThan => 53,
    }
}

/// Every type and term construct key the analysis can produce, other than
/// the primitive keys, which [`PRIMITIVES`] enumerates. The conformance suite
/// checks this list, the primitive keys, and the declaration keys against
/// the registry rows in both directions.
pub const STRUCTURAL_KEYS: [&str; 88] = [
    "type.type",
    "type.parameter",
    "type.nat",
    "type.bool",
    "type.prop",
    "type.unit",
    "type.int",
    "type.int8",
    "type.int16",
    "type.int32",
    "type.int64",
    "type.uint8",
    "type.uint16",
    "type.uint32",
    "type.uint64",
    "type.string",
    "type.bytes",
    "type.ordering",
    "type.option",
    "type.result",
    "type.list",
    "type.named",
    "type.product",
    "type.function",
    "type.map",
    "type.set",
    "type.contract_violation",
    "type.reasoning_failure",
    "term.var",
    "term.nat",
    "term.integer",
    "term.string",
    "term.bytes",
    "term.primitive",
    "term.bool",
    "term.unit",
    "term.nil",
    "term.cons",
    "term.record",
    "term.constructor",
    "term.instance_value",
    "term.project",
    "term.call",
    "term.if",
    "term.match",
    "term.eq",
    "term.le",
    "term.lt",
    "term.add",
    "term.beq",
    "term.ble",
    "term.blt",
    "term.and",
    "term.prop_and",
    "term.or",
    "term.not",
    "term.implies",
    "term.iff",
    "term.forall",
    "term.let",
    "term.pair",
    "term.first",
    "term.second",
    "term.lambda",
    "term.apply",
    "term.function_ref",
    "term.map_literal",
    "term.set_literal",
    "term.graph_literal",
    "term.checked_apply",
    "constructor.nat_succ",
    "declaration.structure",
    "declaration.class",
    "declaration.instance",
    "declaration.inductive",
    "declaration.inductive.recursive",
    "declaration.definition",
    "declaration.definition.recursive",
    "declaration.theorem",
    "declaration.artifact",
    "declaration.contract",
    "declaration.realization",
    "declaration.evidence",
    "declaration.model",
    "declaration.logic",
    "declaration.inference_rule",
    "declaration.verifier",
    "declaration.reasoner",
];

/// The registry key of a type, before its arguments are visited.
#[must_use]
pub const fn type_key(ty: &SemanticType) -> &'static str {
    match ty {
        SemanticType::Type => "type.type",
        SemanticType::Parameter { name: _ } => "type.parameter",
        SemanticType::Nat => "type.nat",
        SemanticType::Bool => "type.bool",
        SemanticType::Prop => "type.prop",
        SemanticType::Unit => "type.unit",
        SemanticType::Int => "type.int",
        SemanticType::Int8 => "type.int8",
        SemanticType::Int16 => "type.int16",
        SemanticType::Int32 => "type.int32",
        SemanticType::Int64 => "type.int64",
        SemanticType::UInt8 => "type.uint8",
        SemanticType::UInt16 => "type.uint16",
        SemanticType::UInt32 => "type.uint32",
        SemanticType::UInt64 => "type.uint64",
        SemanticType::String => "type.string",
        SemanticType::Bytes => "type.bytes",
        SemanticType::Ordering => "type.ordering",
        SemanticType::Option { value: _ } => "type.option",
        SemanticType::Result { ok: _, error: _ } => "type.result",
        SemanticType::List { element: _ } => "type.list",
        SemanticType::Named {
            member: _,
            arguments: _,
        } => "type.named",
        SemanticType::Product { left: _, right: _ } => "type.product",
        SemanticType::Function {
            parameters: _,
            result: _,
        } => "type.function",
        SemanticType::Map { key: _, value: _ } => "type.map",
        SemanticType::Set { element: _ } => "type.set",
        SemanticType::ContractViolation => "type.contract_violation",
        SemanticType::ReasoningFailure => "type.reasoning_failure",
    }
}

/// The registry key of a term, before its children are visited.
#[must_use]
pub const fn term_key(term: &SemanticTerm) -> &'static str {
    match term {
        SemanticTerm::Var { name: _ } => "term.var",
        SemanticTerm::Nat { value: _ } => "term.nat",
        SemanticTerm::Integer {
            representation: _,
            value: _,
        } => "term.integer",
        SemanticTerm::String { value: _ } => "term.string",
        SemanticTerm::Bytes { hex: _ } => "term.bytes",
        SemanticTerm::Primitive {
            operation: _,
            arguments: _,
            result: _,
        } => "term.primitive",
        SemanticTerm::Bool { value: _ } => "term.bool",
        SemanticTerm::Unit => "term.unit",
        SemanticTerm::Nil { element: _ } => "term.nil",
        SemanticTerm::Cons { head: _, tail: _ } => "term.cons",
        SemanticTerm::Record {
            r#type: _,
            type_arguments: _,
            fields: _,
        } => "term.record",
        SemanticTerm::Constructor {
            constructor: _,
            type_arguments: _,
            arguments: _,
        } => "term.constructor",
        SemanticTerm::InstanceValue {
            class: _,
            arguments: _,
            resolved: _,
        } => "term.instance_value",
        SemanticTerm::Project { value: _, field: _ } => "term.project",
        SemanticTerm::Call {
            function: _,
            type_arguments: _,
            arguments: _,
        } => "term.call",
        SemanticTerm::If {
            condition: _,
            then_value: _,
            else_value: _,
        } => "term.if",
        SemanticTerm::Match {
            scrutinee: _,
            branches: _,
        } => "term.match",
        SemanticTerm::Eq { left: _, right: _ } => "term.eq",
        SemanticTerm::Le { left: _, right: _ } => "term.le",
        SemanticTerm::Lt { left: _, right: _ } => "term.lt",
        SemanticTerm::Add { left: _, right: _ } => "term.add",
        SemanticTerm::Beq { left: _, right: _ } => "term.beq",
        SemanticTerm::Ble { left: _, right: _ } => "term.ble",
        SemanticTerm::Blt { left: _, right: _ } => "term.blt",
        SemanticTerm::And { left: _, right: _ } => "term.and",
        SemanticTerm::PropAnd { left: _, right: _ } => "term.prop_and",
        SemanticTerm::Or { left: _, right: _ } => "term.or",
        SemanticTerm::Not { value: _ } => "term.not",
        SemanticTerm::Implies {
            premise: _,
            conclusion: _,
        } => "term.implies",
        SemanticTerm::Iff { left: _, right: _ } => "term.iff",
        SemanticTerm::Forall { binder: _, body: _ } => "term.forall",
        SemanticTerm::Let {
            binder: _,
            value: _,
            body: _,
        } => "term.let",
        SemanticTerm::Pair { left: _, right: _ } => "term.pair",
        SemanticTerm::First { value: _ } => "term.first",
        SemanticTerm::Second { value: _ } => "term.second",
        SemanticTerm::Lambda {
            parameters: _,
            captures: _,
            body: _,
        } => "term.lambda",
        SemanticTerm::Apply {
            function: _,
            arguments: _,
        } => "term.apply",
        SemanticTerm::FunctionRef {
            function: _,
            type_arguments: _,
        } => "term.function_ref",
        SemanticTerm::MapLiteral {
            key: _,
            value: _,
            entries: _,
        } => "term.map_literal",
        SemanticTerm::SetLiteral {
            element: _,
            elements: _,
        } => "term.set_literal",
        SemanticTerm::GraphLiteral {
            node: _,
            nodes: _,
            edges: _,
        } => "term.graph_literal",
        SemanticTerm::CheckedApply {
            model: _,
            type_arguments: _,
            arguments: _,
            checks: _,
        } => "term.checked_apply",
    }
}

/// The type of an integer literal representation.
const fn integer_type(representation: SemanticInteger) -> SemanticType {
    match representation {
        SemanticInteger::Int => SemanticType::Int,
        SemanticInteger::Int8 => SemanticType::Int8,
        SemanticInteger::Int16 => SemanticType::Int16,
        SemanticInteger::Int32 => SemanticType::Int32,
        SemanticInteger::Int64 => SemanticType::Int64,
        SemanticInteger::UInt8 => SemanticType::UInt8,
        SemanticInteger::UInt16 => SemanticType::UInt16,
        SemanticInteger::UInt32 => SemanticType::UInt32,
        SemanticInteger::UInt64 => SemanticType::UInt64,
    }
}

/// An unbounded value representation realized in a fixed target width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Representation {
    Nat,
    Int,
}

impl Representation {
    /// The registry spelling of the `overflow` column.
    const fn as_str(self) -> &'static str {
        match self {
            Self::Nat => "nat",
            Self::Int => "int",
        }
    }

    /// The literal construct whose value is checked against the width.
    const fn literal_key(self) -> &'static str {
        match self {
            Self::Nat => "term.nat",
            Self::Int => "term.integer",
        }
    }

    /// The width of this representation on a target.
    const fn bits(self, target: &super::Target) -> u32 {
        match self {
            Self::Nat => target.natural_bits,
            Self::Int => target.integer_bits,
        }
    }

    /// The inclusive range of this representation at `bits`.
    const fn range(self, bits: u32) -> (i128, i128) {
        match self {
            Self::Nat => (0, (1_i128 << bits) - 1),
            Self::Int => (-(1_i128 << (bits - 1)), (1_i128 << (bits - 1)) - 1),
        }
    }
}

/// The unbounded representation a result type realizes in a target width,
/// for the registry's `overflow` column: `Nat` and `Int`, directly or as
/// the value of an `Option`.
fn representation(ty: &SemanticType) -> Option<Representation> {
    match ty {
        SemanticType::Nat => Some(Representation::Nat),
        SemanticType::Int => Some(Representation::Int),
        SemanticType::Option { value } => representation(value),
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
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure
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
        | SemanticType::Set { element: _ } => None,
    }
}

/// The canonical spelling of a type in a report.
fn type_text(ty: &SemanticType, owner: &Owner<'_>) -> String {
    let wrap = |inner: &SemanticType| format!("({})", type_text(inner, owner));
    match ty {
        SemanticType::Type => "Type".to_owned(),
        SemanticType::Parameter { name } => name.clone(),
        SemanticType::Nat => "Nat".to_owned(),
        SemanticType::Bool => "Bool".to_owned(),
        SemanticType::Prop => "Prop".to_owned(),
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
        SemanticType::Option { value } => format!("Option {}", wrap(value)),
        SemanticType::Result { ok, error } => format!("Except {} {}", wrap(error), wrap(ok)),
        SemanticType::List { element } => format!("List {}", wrap(element)),
        SemanticType::Named { member, arguments } => {
            let mut text = owner.lean_name(member);
            for argument in arguments {
                text.push(' ');
                text.push_str(&wrap(argument));
            }
            text
        }
        SemanticType::Product { left, right } => format!("Prod {} {}", wrap(left), wrap(right)),
        SemanticType::Function { parameters, result } => {
            let mut text = String::new();
            for parameter in parameters {
                text.push_str(&wrap(parameter));
                text.push_str(" -> ");
            }
            text.push_str(&wrap(result));
            text
        }
        SemanticType::Map { key, value } => format!("Map {} {}", wrap(key), wrap(value)),
        SemanticType::Set { element } => format!("Set {}", wrap(element)),
        SemanticType::ContractViolation => "ContractViolation".to_owned(),
        SemanticType::ReasoningFailure => "ReasoningFailure".to_owned(),
    }
}

/// Substitute declaration type parameters.
fn substitute(ty: &SemanticType, map: &BTreeMap<String, SemanticType>) -> SemanticType {
    let boxed = |inner: &SemanticType| Box::new(substitute(inner, map));
    match ty {
        SemanticType::Parameter { name } => match map.get(name) {
            Some(bound) => bound.clone(),
            None => ty.clone(),
        },
        SemanticType::Option { value } => SemanticType::Option {
            value: boxed(value),
        },
        SemanticType::Result { ok, error } => SemanticType::Result {
            ok: boxed(ok),
            error: boxed(error),
        },
        SemanticType::List { element } => SemanticType::List {
            element: boxed(element),
        },
        SemanticType::Named { member, arguments } => SemanticType::Named {
            member: member.clone(),
            arguments: arguments
                .iter()
                .map(|argument| substitute(argument, map))
                .collect(),
        },
        SemanticType::Product { left, right } => SemanticType::Product {
            left: boxed(left),
            right: boxed(right),
        },
        SemanticType::Function { parameters, result } => SemanticType::Function {
            parameters: parameters
                .iter()
                .map(|parameter| substitute(parameter, map))
                .collect(),
            result: boxed(result),
        },
        SemanticType::Map { key, value } => SemanticType::Map {
            key: boxed(key),
            value: boxed(value),
        },
        SemanticType::Set { element } => SemanticType::Set {
            element: boxed(element),
        },
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
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => ty.clone(),
    }
}

/// Re-anchor a type written in `from` so that it reads the same in another
/// module: every local reference gains its module.
fn anchor(ty: &SemanticType, from: &str) -> SemanticType {
    let boxed = |inner: &SemanticType| Box::new(anchor(inner, from));
    match ty {
        SemanticType::Named { member, arguments } => SemanticType::Named {
            member: anchor_member(member, from),
            arguments: arguments
                .iter()
                .map(|argument| anchor(argument, from))
                .collect(),
        },
        SemanticType::Option { value } => SemanticType::Option {
            value: boxed(value),
        },
        SemanticType::Result { ok, error } => SemanticType::Result {
            ok: boxed(ok),
            error: boxed(error),
        },
        SemanticType::List { element } => SemanticType::List {
            element: boxed(element),
        },
        SemanticType::Product { left, right } => SemanticType::Product {
            left: boxed(left),
            right: boxed(right),
        },
        SemanticType::Function { parameters, result } => SemanticType::Function {
            parameters: parameters
                .iter()
                .map(|parameter| anchor(parameter, from))
                .collect(),
            result: boxed(result),
        },
        SemanticType::Map { key, value } => SemanticType::Map {
            key: boxed(key),
            value: boxed(value),
        },
        SemanticType::Set { element } => SemanticType::Set {
            element: boxed(element),
        },
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
        | SemanticType::ReasoningFailure => ty.clone(),
    }
}

fn anchor_member(member: &MemberRef, from: &str) -> MemberRef {
    MemberRef {
        module: Some(member.module.clone().unwrap_or_else(|| from.to_owned())),
        name: member.name.clone(),
    }
}

/// The module a reference is resolved in.
struct Owner<'a> {
    module: &'a str,
    modules: &'a BTreeMap<String, LinkedModule<'a>>,
}

impl Owner<'_> {
    fn module_of<'m>(&'m self, member: &'m MemberRef) -> &'m str {
        member.module.as_deref().unwrap_or(self.module)
    }

    fn lean_name(&self, member: &MemberRef) -> String {
        let module = self.module_of(member);
        let prefix = self
            .modules
            .get(module)
            .map_or(module, |linked| linked.lean_module);
        format!("{prefix}.{}", member.name)
    }
}

/// What the closure walk is looking at.
#[derive(Clone)]
enum Item {
    Definition {
        module: String,
        name: String,
        type_arguments: Vec<SemanticType>,
    },
    Instance {
        module: String,
        name: String,
    },
}

struct Walk<'a> {
    registry: &'static Registry,
    modules: &'a BTreeMap<String, LinkedModule<'a>>,
    queue: VecDeque<(Item, Vec<String>)>,
    members: BTreeMap<String, ClosureMember>,
    member_order: Vec<String>,
    types: BTreeMap<String, String>,
    visited_types: BTreeSet<(String, String)>,
    erased: BTreeSet<String>,
    constructs: BTreeMap<String, BTreeSet<String>>,
    /// Effect name to `(construct, instance)` sources.
    effects: BTreeMap<String, BTreeSet<(String, String)>>,
    /// Allocation sources, kept apart: they are an effect on a target with
    /// allocation and a violation on one without.
    allocation: BTreeSet<(String, String)>,
    /// `(literal, representation, instance)` for width checks per target.
    literals: BTreeSet<(String, Representation, String)>,
    violations: Vec<Violation>,
}

/// The context of one closure member being walked.
struct Site<'s> {
    module: &'s str,
    instance: &'s str,
    depth: usize,
    substitution: &'s BTreeMap<String, SemanticType>,
}

impl<'a> Walk<'a> {
    fn owner<'o>(&'o self, module: &'o str) -> Owner<'o> {
        Owner {
            module,
            modules: self.modules,
        }
    }

    fn violation(&mut self, site: &Site<'_>, construct: &str, reason: String) {
        self.violations.push(Violation {
            depth: site.depth,
            instance: site.instance.to_owned(),
            construct: construct.to_owned(),
            reason,
        });
    }

    /// Record one construct occurrence and apply its registry row.
    fn construct(&mut self, site: &Site<'_>, key: &str, result: Option<&SemanticType>) {
        self.constructs
            .entry(key.to_owned())
            .or_default()
            .insert(site.instance.to_owned());
        let row = match self.registry.constructs.get(key) {
            Some(row) => row,
            None => {
                self.violation(
                    site,
                    key,
                    format!("construct `{key}` has no production disposition"),
                );
                return;
            }
        };
        match row.disposition {
            Disposition::Runtime => {}
            Disposition::FormalOnly => {
                self.violation(
                    site,
                    key,
                    format!("construct `{key}` is formal-only and has no runtime realization"),
                );
            }
            Disposition::Erased => {
                self.violation(
                    site,
                    key,
                    format!("construct `{key}` is proof-only and cannot be realized"),
                );
            }
        }
        if row.allocation {
            self.allocation
                .insert((key.to_owned(), site.instance.to_owned()));
        }
        if row.recursion {
            self.effects
                .entry("recursion".to_owned())
                .or_default()
                .insert((key.to_owned(), site.instance.to_owned()));
        }
        let realized = result.and_then(representation);
        let overflows = match realized {
            Some(representation) => row
                .overflow
                .iter()
                .any(|name| name == representation.as_str()),
            None => false,
        };
        if overflows {
            self.effects
                .entry("overflow".to_owned())
                .or_default()
                .insert((key.to_owned(), site.instance.to_owned()));
        }
    }

    /// The reason an anchored type may not cross a production-root
    /// boundary, if any: a root takes and returns first-order data only, so
    /// no universe, proposition, type parameter, or function may occur in
    /// it, including inside the fields of a document type it names.
    fn boundary(
        &self,
        ty: &SemanticType,
        visiting: &mut BTreeSet<String>,
        crossing: &mut BTreeSet<Representation>,
    ) -> Option<String> {
        match ty {
            SemanticType::Type => Some("a type universe".to_owned()),
            SemanticType::Prop => Some("a proposition".to_owned()),
            SemanticType::Parameter { name } => {
                Some(format!("the uninstantiated type parameter `{name}`"))
            }
            SemanticType::Function {
                parameters: _,
                result: _,
            } => Some("a function, so a closure would escape the root".to_owned()),
            SemanticType::Option { value: inner }
            | SemanticType::List { element: inner }
            | SemanticType::Set { element: inner } => self.boundary(inner, visiting, crossing),
            SemanticType::Result {
                ok: left,
                error: right,
            }
            | SemanticType::Product { left, right }
            | SemanticType::Map {
                key: left,
                value: right,
            } => self
                .boundary(left, visiting, crossing)
                .or_else(|| self.boundary(right, visiting, crossing)),
            SemanticType::Named { member, arguments } => {
                let module = member.module.clone().unwrap_or_default();
                let key = format!("{module}::{}", member.name);
                if !visiting.insert(key) {
                    return arguments
                        .iter()
                        .find_map(|argument| self.boundary(argument, visiting, crossing));
                }
                let (type_parameters, fields) = match self.declaration(&module, &member.name) {
                    Some(
                        SemanticDeclaration::Structure {
                            name: _,
                            type_parameters,
                            parameters: _,
                            fields,
                        }
                        | SemanticDeclaration::Class {
                            name: _,
                            type_parameters,
                            parameters: _,
                            fields,
                        },
                    ) => (
                        type_parameters.clone(),
                        fields
                            .iter()
                            .map(|SemanticField { name: _, r#type }| r#type.clone())
                            .collect::<Vec<_>>(),
                    ),
                    Some(SemanticDeclaration::Inductive {
                        name: _,
                        type_parameters,
                        parameters: _,
                        constructors,
                        mutual: _,
                    }) => (
                        type_parameters.clone(),
                        constructors
                            .iter()
                            .flat_map(|SemanticConstructor { name: _, fields }| {
                                fields.iter().cloned()
                            })
                            .collect(),
                    ),
                    Some(
                        SemanticDeclaration::Instance {
                            name: _,
                            class: _,
                            arguments: _,
                            priority: _,
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
                    | None => {
                        return Some(format!(
                            "`{}`, which is not a linked data type",
                            member.name
                        ));
                    }
                };
                let substitution: BTreeMap<String, SemanticType> = type_parameters
                    .into_iter()
                    .zip(arguments.iter().cloned())
                    .collect();
                fields.iter().find_map(|field| {
                    let field = substitute(&anchor(field, &module), &substitution);
                    self.boundary(&field, visiting, crossing)
                })
            }
            // A natural number or integer crosses the boundary in the
            // target's width; the report records each one (§17.13).
            SemanticType::Nat => {
                crossing.insert(Representation::Nat);
                None
            }
            SemanticType::Int => {
                crossing.insert(Representation::Int);
                None
            }
            SemanticType::Bool
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
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => None,
        }
    }

    fn enqueue(&mut self, item: Item, site: &Site<'_>) {
        let mut path = self
            .members
            .get(site.instance)
            .map(|member| member.path.clone())
            .unwrap_or_default();
        path.push(String::new());
        self.queue.push_back((item, path));
    }

    /// The declaration `name` of `module` as linking elaborated it: a
    /// model declaration is the ordinary declarations it means, and a
    /// checked application the ordinary term it means (§17.12).
    fn declaration(&self, module: &str, name: &str) -> Option<&'a SemanticDeclaration> {
        self.modules.get(module).and_then(|linked| {
            linked
                .semantic
                .lowered_declarations()
                .into_iter()
                .find(|declaration| declaration.name() == name)
        })
    }

    /// The model construct whose elaboration declares `name`, if any.
    fn model_owner(&self, module: &str, name: &str) -> Option<&'static str> {
        let linked = self.modules.get(module)?;
        for (index, source) in linked.semantic.declarations.iter().enumerate() {
            let key = match source {
                SemanticDeclaration::Artifact {
                    name: _,
                    role: _,
                    sha256: _,
                    length: _,
                    schema: _,
                    r#type: _,
                    axioms: _,
                } => "declaration.artifact",
                SemanticDeclaration::Contract {
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
                } => "declaration.contract",
                SemanticDeclaration::Realization {
                    name: _,
                    type_parameters: _,
                    input: _,
                    output: _,
                    state: _,
                    descriptor: _,
                    executable: _,
                    axioms: _,
                } => "declaration.realization",
                SemanticDeclaration::Evidence {
                    name: _,
                    type_parameters: _,
                    contract: _,
                    realization: _,
                    claims: _,
                    axioms: _,
                } => "declaration.evidence",
                SemanticDeclaration::Model {
                    name: _,
                    type_parameters: _,
                    contract: _,
                    realization: _,
                    evidence: _,
                    entry: _,
                    axioms: _,
                } => "declaration.model",
                SemanticDeclaration::Logic {
                    name: _,
                    type_parameters: _,
                    state: _,
                    relation: _,
                    invariant: _,
                    ranking: _,
                    axioms: _,
                } => "declaration.logic",
                SemanticDeclaration::InferenceRule {
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
                } => "declaration.inference_rule",
                SemanticDeclaration::Verifier {
                    name: _,
                    type_parameters: _,
                    subject: _,
                    candidate: _,
                    specification: _,
                    check: _,
                    sound: _,
                    complete: _,
                    axioms: _,
                } => "declaration.verifier",
                SemanticDeclaration::Reasoner {
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
                } => "declaration.reasoner",
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
                | SemanticDeclaration::Theorem {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    statement: _,
                    proof: _,
                    axioms: _,
                } => continue,
            };
            for derived in linked.semantic.elaboration.lowered(index) {
                if derived.name() == name {
                    return Some(key);
                }
            }
        }
        None
    }

    fn ty(&mut self, ty: &SemanticType, site: &Site<'_>) {
        // Substituted arguments are already anchored where they were
        // written; anchoring afterwards resolves only this module's text.
        let ty = anchor(&substitute(ty, site.substitution), site.module);
        let owner = self.owner(site.module);
        let text = type_text(&ty, &owner);
        if !self
            .visited_types
            .insert((site.instance.to_owned(), text.clone()))
        {
            return;
        }
        let key = type_key(&ty);
        self.construct(site, key, None);
        match &ty {
            SemanticType::Named { member, arguments } => {
                self.named(member, arguments, &text, site);
            }
            // The walk instantiates every type parameter from the
            // monomorphic root down, so one that survives substitution has
            // no type to be realized at.
            SemanticType::Parameter { name } => self.violation(
                site,
                key,
                format!("the type parameter `{name}` is never instantiated in the closure"),
            ),
            SemanticType::Option { value: inner }
            | SemanticType::List { element: inner }
            | SemanticType::Set { element: inner } => self.ty(inner, site),
            SemanticType::Result {
                ok: left,
                error: right,
            }
            | SemanticType::Product { left, right }
            | SemanticType::Map {
                key: left,
                value: right,
            } => {
                self.ty(left, site);
                self.ty(right, site);
            }
            SemanticType::Function { parameters, result } => {
                for parameter in parameters {
                    self.ty(parameter, site);
                }
                self.ty(result, site);
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
            | SemanticType::Ordering
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {}
        }
        self.types.entry(text).or_insert_with(|| key.to_owned());
    }

    /// Visit a document type: its declaration row and every field type
    /// under the instantiation.
    fn named(
        &mut self,
        member: &MemberRef,
        arguments: &[SemanticType],
        text: &str,
        site: &Site<'_>,
    ) {
        let module = member
            .module
            .clone()
            .unwrap_or_else(|| site.module.to_owned());
        let declaration = match self.declaration(&module, &member.name) {
            Some(declaration) => declaration,
            None => {
                self.violation(
                    site,
                    "type.named",
                    format!("type `{text}` has no linked declaration"),
                );
                return;
            }
        };
        let (key, type_parameters, fields) = match declaration {
            SemanticDeclaration::Structure {
                name: _,
                type_parameters,
                parameters: _,
                fields,
            } => (
                "declaration.structure",
                type_parameters,
                fields
                    .iter()
                    .map(|SemanticField { name: _, r#type }| r#type.clone())
                    .collect::<Vec<_>>(),
            ),
            SemanticDeclaration::Class {
                name: _,
                type_parameters,
                parameters: _,
                fields,
            } => (
                "declaration.class",
                type_parameters,
                fields
                    .iter()
                    .map(|SemanticField { name: _, r#type }| r#type.clone())
                    .collect::<Vec<_>>(),
            ),
            SemanticDeclaration::Inductive {
                name,
                type_parameters,
                parameters: _,
                constructors,
                mutual,
            } => {
                let group = inductive_group(self.modules, &module, name, mutual.as_ref());
                let fields: Vec<SemanticType> = constructors
                    .iter()
                    .flat_map(|SemanticConstructor { name: _, fields }| fields.iter().cloned())
                    .collect();
                let recursive = fields.iter().any(|field| mentions(field, &module, &group));
                (
                    if recursive {
                        "declaration.inductive.recursive"
                    } else {
                        "declaration.inductive"
                    },
                    type_parameters,
                    fields,
                )
            }
            SemanticDeclaration::Instance {
                name: _,
                class: _,
                arguments: _,
                priority: _,
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
            } => {
                self.violation(
                    site,
                    "type.named",
                    format!("type `{text}` names a declaration that is not a type"),
                );
                return;
            }
        };
        self.construct(site, key, None);
        let substitution: BTreeMap<String, SemanticType> = type_parameters
            .iter()
            .cloned()
            .zip(arguments.iter().cloned())
            .collect();
        let field_site = Site {
            module: &module,
            instance: site.instance,
            depth: site.depth,
            substitution: &substitution,
        };
        for field in fields {
            self.ty(&field, &field_site);
        }
    }

    fn terms(&mut self, terms: &[SemanticTerm], site: &Site<'_>) {
        for term in terms {
            self.term(term, site);
        }
    }

    fn parameters(&mut self, parameters: &[SemanticParameter], site: &Site<'_>) {
        for SemanticParameter { name: _, r#type } in parameters {
            self.ty(r#type, site);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn term(&mut self, term: &SemanticTerm, site: &Site<'_>) {
        let key = term_key(term);
        match term {
            SemanticTerm::Var { name: _ }
            | SemanticTerm::Bool { value: _ }
            | SemanticTerm::Unit
            | SemanticTerm::String { value: _ }
            | SemanticTerm::Bytes { hex: _ } => self.construct(site, key, None),
            SemanticTerm::Nat { value } => {
                self.construct(site, key, None);
                self.literals.insert((
                    value.clone(),
                    Representation::Nat,
                    site.instance.to_owned(),
                ));
            }
            SemanticTerm::Integer {
                representation,
                value,
            } => {
                self.construct(site, key, None);
                let ty = integer_type(*representation);
                match self::representation(&ty) {
                    Some(realized) => {
                        self.literals
                            .insert((value.clone(), realized, site.instance.to_owned()));
                    }
                    None => {}
                }
                self.ty(&ty, site);
            }
            SemanticTerm::Primitive {
                operation,
                arguments,
                result,
            } => {
                self.construct(site, key, None);
                let substituted = substitute(result, site.substitution);
                self.construct(site, primitive_key(*operation), Some(&substituted));
                self.ty(result, site);
                self.terms(arguments, site);
            }
            SemanticTerm::Nil { element } => {
                self.construct(site, key, None);
                self.ty(element, site);
            }
            SemanticTerm::Cons { head, tail } => {
                self.construct(site, key, None);
                self.term(head, site);
                self.term(tail, site);
            }
            SemanticTerm::Record {
                r#type,
                type_arguments,
                fields,
            } => {
                self.construct(site, key, None);
                let named = SemanticType::Named {
                    member: r#type.clone(),
                    arguments: type_arguments.clone(),
                };
                self.ty(&named, site);
                for SemanticAssignment { field: _, value } in fields {
                    self.term(value, site);
                }
            }
            SemanticTerm::Constructor {
                constructor,
                type_arguments,
                arguments,
            } => {
                self.construct(site, key, None);
                self.constructor_type(constructor, type_arguments, site);
                self.terms(arguments, site);
            }
            SemanticTerm::InstanceValue {
                class,
                arguments,
                resolved,
            } => {
                self.construct(site, key, None);
                let named = SemanticType::Named {
                    member: class.clone(),
                    arguments: arguments.clone(),
                };
                self.ty(&named, site);
                let module = resolved
                    .module
                    .clone()
                    .unwrap_or_else(|| site.module.to_owned());
                self.enqueue(
                    Item::Instance {
                        module,
                        name: resolved.name.clone(),
                    },
                    site,
                );
            }
            SemanticTerm::Project { value, field: _ } => {
                self.construct(site, key, None);
                self.term(value, site);
            }
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } => {
                self.construct(site, key, None);
                self.call(function, type_arguments, site);
                self.terms(arguments, site);
            }
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                self.construct(site, key, None);
                self.call(function, type_arguments, site);
            }
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => {
                self.construct(site, key, None);
                self.term(condition, site);
                self.term(then_value, site);
                self.term(else_value, site);
            }
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => {
                self.construct(site, key, None);
                self.term(scrutinee, site);
                for SemanticBranch {
                    constructor: _,
                    binders: _,
                    body,
                } in branches
                {
                    self.term(body, site);
                }
            }
            SemanticTerm::Add { left, right } => {
                self.construct(site, key, Some(&SemanticType::Nat));
                self.term(left, site);
                self.term(right, site);
            }
            SemanticTerm::Eq { left, right }
            | SemanticTerm::Le { left, right }
            | SemanticTerm::Lt { left, right }
            | SemanticTerm::Beq { left, right }
            | SemanticTerm::Ble { left, right }
            | SemanticTerm::Blt { left, right }
            | SemanticTerm::And { left, right }
            | SemanticTerm::PropAnd { left, right }
            | SemanticTerm::Or { left, right }
            | SemanticTerm::Iff { left, right }
            | SemanticTerm::Pair { left, right }
            | SemanticTerm::Implies {
                premise: left,
                conclusion: right,
            } => {
                self.construct(site, key, None);
                self.term(left, site);
                self.term(right, site);
            }
            SemanticTerm::Not { value }
            | SemanticTerm::First { value }
            | SemanticTerm::Second { value } => {
                self.construct(site, key, None);
                self.term(value, site);
            }
            SemanticTerm::Forall { binder, body } => {
                self.construct(site, key, None);
                self.parameters(core::slice::from_ref(binder), site);
                self.term(body, site);
            }
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => {
                self.construct(site, key, None);
                self.parameters(core::slice::from_ref(binder), site);
                self.term(value, site);
                self.term(body, site);
            }
            SemanticTerm::Lambda {
                parameters,
                captures: _,
                body,
            } => {
                self.construct(site, key, None);
                self.parameters(parameters, site);
                self.term(body, site);
            }
            SemanticTerm::Apply {
                function,
                arguments,
            } => {
                self.construct(site, key, None);
                self.term(function, site);
                self.terms(arguments, site);
            }
            SemanticTerm::MapLiteral {
                key: key_type,
                value: value_type,
                entries,
            } => {
                self.construct(site, key, None);
                let map = SemanticType::Map {
                    key: Box::new(key_type.clone()),
                    value: Box::new(value_type.clone()),
                };
                self.ty(&map, site);
                for SemanticMapEntry { key, value } in entries {
                    self.term(key, site);
                    self.term(value, site);
                }
            }
            SemanticTerm::SetLiteral { element, elements } => {
                self.construct(site, key, None);
                let set = SemanticType::Set {
                    element: Box::new(element.clone()),
                };
                self.ty(&set, site);
                self.terms(elements, site);
            }
            SemanticTerm::GraphLiteral { node, nodes, edges } => {
                self.construct(site, key, None);
                let graph = SemanticType::Map {
                    key: Box::new(node.clone()),
                    value: Box::new(SemanticType::Set {
                        element: Box::new(node.clone()),
                    }),
                };
                self.ty(&graph, site);
                self.terms(nodes, site);
                for SemanticEdge { source, target } in edges {
                    self.term(source, site);
                    self.term(target, site);
                }
            }
            // The analysis reads elaborated declarations, in which every
            // checked application is the ordinary term it means (§17.12);
            // one reaching here was not elaborated and is never realized.
            SemanticTerm::CheckedApply {
                model: _,
                type_arguments: _,
                arguments: _,
                checks: _,
            } => {
                self.construct(site, key, None);
                self.violation(
                    site,
                    key,
                    "a checked model application is realized only through its elaboration"
                        .to_owned(),
                );
            }
        }
    }

    fn constructor_type(
        &mut self,
        constructor: &MemberRef,
        type_arguments: &[SemanticType],
        site: &Site<'_>,
    ) {
        let owner = match constructor.name.rsplit_once('.') {
            Some((owner, _)) => owner,
            None => {
                self.violation(
                    site,
                    "term.constructor",
                    format!("constructor `{}` names no type", constructor.name),
                );
                return;
            }
        };
        // A module-less constructor names this module's type or, for the
        // owners below, a built-in type; linking admits no other.
        let builtin = match &constructor.module {
            Some(_) => None,
            None => BUILTIN_OWNERS
                .iter()
                .find(|(name, _)| *name == owner)
                .map(|(_, builtin)| *builtin),
        };
        let ty = match builtin {
            None => SemanticType::Named {
                member: MemberRef {
                    module: constructor.module.clone(),
                    name: owner.to_owned(),
                },
                arguments: type_arguments.to_vec(),
            },
            Some(builtin) => {
                if type_arguments.len() != builtin.arity() {
                    self.violation(
                        site,
                        "term.constructor",
                        format!(
                            "built-in constructor `{}` has {} type arguments",
                            constructor.name,
                            type_arguments.len()
                        ),
                    );
                    return;
                }
                if constructor.name == "Nat.succ" {
                    // The successor is an addition and overflows like one.
                    self.construct(site, "constructor.nat_succ", Some(&SemanticType::Nat));
                }
                let argument = |index: usize| Box::new(type_arguments[index].clone());
                match builtin {
                    BuiltinOwner::Bool => SemanticType::Bool,
                    BuiltinOwner::ContractViolation => SemanticType::ContractViolation,
                    BuiltinOwner::ReasoningFailure => SemanticType::ReasoningFailure,
                    BuiltinOwner::Nat => SemanticType::Nat,
                    BuiltinOwner::List => SemanticType::List {
                        element: argument(0),
                    },
                    BuiltinOwner::Option => SemanticType::Option { value: argument(0) },
                    BuiltinOwner::Result => SemanticType::Result {
                        ok: argument(0),
                        error: argument(1),
                    },
                }
            }
        };
        self.ty(&ty, site);
    }

    fn call(&mut self, function: &MemberRef, type_arguments: &[SemanticType], site: &Site<'_>) {
        let module = function
            .module
            .clone()
            .unwrap_or_else(|| site.module.to_owned());
        let arguments = type_arguments
            .iter()
            .map(|argument| anchor(&substitute(argument, site.substitution), site.module))
            .collect();
        self.enqueue(
            Item::Definition {
                module,
                name: function.name.clone(),
                type_arguments: arguments,
            },
            site,
        );
    }

    /// Visit one queued closure member.
    fn visit(&mut self, item: &Item, mut path: Vec<String>) {
        let (module, name, type_arguments) = match item {
            Item::Definition {
                module,
                name,
                type_arguments,
            } => (module.clone(), name.clone(), type_arguments.clone()),
            Item::Instance { module, name } => (module.clone(), name.clone(), Vec::new()),
        };
        let owner = self.owner(&module);
        let declaration_name = owner.lean_name(&MemberRef {
            module: Some(module.clone()),
            name: name.clone(),
        });
        let mut instance = declaration_name.clone();
        let argument_texts: Vec<String> = type_arguments
            .iter()
            .map(|argument| type_text(argument, &owner))
            .collect();
        for text in &argument_texts {
            instance.push_str(" (");
            instance.push_str(text);
            instance.push(')');
        }
        if self.members.contains_key(&instance) {
            return;
        }
        match path.last_mut() {
            Some(last) => last.clone_from(&instance),
            None => path.push(instance.clone()),
        }
        let depth = path.len();
        let declaration = self.declaration(&module, &name);
        let construct = match declaration {
            Some(SemanticDeclaration::Definition {
                name: _,
                type_parameters: _,
                parameters: _,
                result: _,
                recursive_argument,
                body: _,
                axioms: _,
                executable: _,
                mutual,
                termination,
                production: _,
            }) => {
                if recursive_argument.is_some() || mutual.is_some() || termination.is_some() {
                    "declaration.definition.recursive"
                } else {
                    "declaration.definition"
                }
            }
            Some(SemanticDeclaration::Instance {
                name: _,
                class: _,
                arguments: _,
                priority: _,
                fields: _,
            }) => "declaration.instance",
            Some(SemanticDeclaration::Theorem {
                name: _,
                type_parameters: _,
                parameters: _,
                statement: _,
                proof: _,
                axioms: _,
            }) => "declaration.theorem",
            Some(SemanticDeclaration::Artifact {
                name: _,
                role: _,
                sha256: _,
                length: _,
                schema: _,
                r#type: _,
                axioms: _,
            }) => "declaration.artifact",
            Some(SemanticDeclaration::Contract {
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
            }) => "declaration.contract",
            Some(SemanticDeclaration::Realization {
                name: _,
                type_parameters: _,
                input: _,
                output: _,
                state: _,
                descriptor: _,
                executable: _,
                axioms: _,
            }) => "declaration.realization",
            Some(SemanticDeclaration::Evidence {
                name: _,
                type_parameters: _,
                contract: _,
                realization: _,
                claims: _,
                axioms: _,
            }) => "declaration.evidence",
            Some(SemanticDeclaration::Model {
                name: _,
                type_parameters: _,
                contract: _,
                realization: _,
                evidence: _,
                entry: _,
                axioms: _,
            }) => "declaration.model",
            Some(SemanticDeclaration::Logic {
                name: _,
                type_parameters: _,
                state: _,
                relation: _,
                invariant: _,
                ranking: _,
                axioms: _,
            }) => "declaration.logic",
            Some(SemanticDeclaration::InferenceRule {
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
            }) => "declaration.inference_rule",
            Some(SemanticDeclaration::Verifier {
                name: _,
                type_parameters: _,
                subject: _,
                candidate: _,
                specification: _,
                check: _,
                sound: _,
                complete: _,
                axioms: _,
            }) => "declaration.verifier",
            Some(SemanticDeclaration::Reasoner {
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
            }) => "declaration.reasoner",
            Some(SemanticDeclaration::Structure {
                name: _,
                type_parameters: _,
                parameters: _,
                fields: _,
            }) => "declaration.structure",
            Some(SemanticDeclaration::Class {
                name: _,
                type_parameters: _,
                parameters: _,
                fields: _,
            }) => "declaration.class",
            Some(SemanticDeclaration::Inductive {
                name: _,
                type_parameters: _,
                parameters: _,
                constructors: _,
                mutual: _,
            }) => "declaration.inductive",
            None => "declaration.definition",
        };
        // A declaration elaborated from a model declaration is that
        // construct, realized by the ordinary definition it elaborates to
        // (§17.12, §17.13).
        let construct = match self.model_owner(&module, &name) {
            Some(owner) => owner,
            None => construct,
        };
        self.members.insert(
            instance.clone(),
            ClosureMember {
                instance: instance.clone(),
                declaration: declaration_name.clone(),
                construct: construct.to_owned(),
                type_arguments: argument_texts,
                path,
            },
        );
        self.member_order.push(instance.clone());
        let empty = BTreeMap::new();
        let site = Site {
            module: &module,
            instance: &instance,
            depth,
            substitution: &empty,
        };
        match declaration {
            Some(SemanticDeclaration::Definition {
                name: _,
                type_parameters,
                parameters,
                result,
                recursive_argument: _,
                body,
                axioms: _,
                executable,
                mutual: _,
                termination,
                production: _,
            }) => {
                self.construct(&site, construct, None);
                // A checked application is realized through its
                // elaboration; the root still reports that it uses one.
                if self
                    .modules
                    .get(module.as_str())
                    .is_some_and(|linked| linked.semantic.applies_checked(&name))
                {
                    self.construct(&site, "term.checked_apply", None);
                }
                if !*executable {
                    self.violation(
                        &site,
                        construct,
                        format!("`{declaration_name}` is not declared executable"),
                    );
                }
                let substitution: BTreeMap<String, SemanticType> = type_parameters
                    .iter()
                    .cloned()
                    .zip(type_arguments.iter().cloned())
                    .collect();
                let site = Site {
                    module: &module,
                    instance: &instance,
                    depth,
                    substitution: &substitution,
                };
                self.parameters(parameters, &site);
                self.ty(result, &site);
                self.term(body, &site);
                match termination {
                    Some(SemanticTermination {
                        measure: _,
                        evidence,
                    }) => {
                        for theorem in evidence {
                            let lean = self.owner(&module).lean_name(theorem);
                            self.erased.insert(lean);
                        }
                    }
                    None => {}
                }
            }
            Some(SemanticDeclaration::Instance {
                name: _,
                class,
                arguments,
                priority: _,
                fields,
            }) => {
                self.construct(&site, construct, None);
                let named = SemanticType::Named {
                    member: class.clone(),
                    arguments: arguments.clone(),
                };
                self.ty(&named, &site);
                for SemanticAssignment { field: _, value } in fields {
                    self.term(value, &site);
                }
            }
            Some(
                SemanticDeclaration::Theorem {
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
                | SemanticDeclaration::Inductive {
                    name: _,
                    type_parameters: _,
                    parameters: _,
                    constructors: _,
                    mutual: _,
                },
            ) => {
                self.construct(&site, construct, None);
                self.violation(
                    &site,
                    construct,
                    format!("`{declaration_name}` is not a computational definition"),
                );
            }
            None => {
                self.violation(
                    &site,
                    construct,
                    format!("`{declaration_name}` is an unresolved dependency"),
                );
            }
        }
    }
}

/// The member names of an inductive's group: itself, or every inductive of
/// its module with the same mutual label.
fn inductive_group(
    modules: &BTreeMap<String, LinkedModule<'_>>,
    module: &str,
    name: &str,
    mutual: Option<&String>,
) -> BTreeSet<String> {
    let mut group = BTreeSet::from([name.to_owned()]);
    let label = match mutual {
        Some(label) => label,
        None => return group,
    };
    let declarations = modules
        .get(module)
        .map(|linked| linked.semantic.declarations.as_slice())
        .unwrap_or_default();
    for declaration in declarations {
        match declaration {
            SemanticDeclaration::Inductive {
                name,
                type_parameters: _,
                parameters: _,
                constructors: _,
                mutual: Some(other),
            } => {
                if other == label {
                    group.insert(name.clone());
                }
            }
            SemanticDeclaration::Inductive {
                name: _,
                type_parameters: _,
                parameters: _,
                constructors: _,
                mutual: None,
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
            } => {}
        }
    }
    group
}

/// Whether a field type of an inductive in `module` mentions its group.
fn mentions(ty: &SemanticType, module: &str, group: &BTreeSet<String>) -> bool {
    match ty {
        SemanticType::Named { member, arguments } => {
            let same_module = member.module.as_deref().unwrap_or(module) == module;
            (same_module && group.contains(&member.name))
                || arguments
                    .iter()
                    .any(|argument| mentions(argument, module, group))
        }
        SemanticType::Option { value: inner }
        | SemanticType::List { element: inner }
        | SemanticType::Set { element: inner } => mentions(inner, module, group),
        SemanticType::Result {
            ok: left,
            error: right,
        }
        | SemanticType::Product { left, right }
        | SemanticType::Map {
            key: left,
            value: right,
        } => mentions(left, module, group) || mentions(right, module, group),
        SemanticType::Function { parameters, result } => {
            parameters
                .iter()
                .any(|parameter| mentions(parameter, module, group))
                || mentions(result, module, group)
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
        | SemanticType::ReasoningFailure => false,
    }
}

/// Whether a canonical decimal literal exceeds a representation width. A
/// literal too long for `i128` exceeds every registered width.
fn exceeds(value: &str, representation: Representation, bits: u32) -> bool {
    let (low, high) = representation.range(bits);
    match value.parse::<i128>() {
        Ok(parsed) => parsed < low || parsed > high,
        Err(_) => true,
    }
}

/// Analyse one root against every target it names.
fn analyse_root(
    module: &str,
    name: &str,
    type_parameters: &[String],
    parameters: &[SemanticParameter],
    result: &SemanticType,
    production: &SemanticProduction,
    modules: &BTreeMap<String, LinkedModule<'_>>,
) -> Result<RootReport, AnalysisError> {
    let registry = registry().map_err(AnalysisError::Internal)?;
    let mut walk = Walk {
        registry,
        modules,
        queue: VecDeque::new(),
        members: BTreeMap::new(),
        member_order: Vec::new(),
        types: BTreeMap::new(),
        visited_types: BTreeSet::new(),
        erased: BTreeSet::new(),
        constructs: BTreeMap::new(),
        effects: BTreeMap::new(),
        allocation: BTreeSet::new(),
        literals: BTreeSet::new(),
        violations: Vec::new(),
    };
    let root_ref = MemberRef {
        module: Some(module.to_owned()),
        name: name.to_owned(),
    };
    let root = walk.owner(module).lean_name(&root_ref);
    // The production boundary: first-order data in, first-order data out.
    let empty = BTreeMap::new();
    // Boundary violations sit at depth zero: they are reported before
    // anything the root's body reaches.
    let root_site = Site {
        module,
        instance: &root,
        depth: 0,
        substitution: &empty,
    };
    // A root is realized once, so it is monomorphic: a type parameter it
    // declares has no instantiation, whether or not its signature mentions
    // the parameter.
    if !type_parameters.is_empty() {
        walk.violation(
            &root_site,
            "type.parameter",
            format!(
                "production root declares the type parameters ({}); a root is realized at one type",
                type_parameters.join(", ")
            ),
        );
    }
    // Every natural-number or integer representation crossing the
    // boundary, by position, for the report's per-target widths.
    let mut crossings: Vec<(String, BTreeSet<Representation>)> = Vec::new();
    for SemanticParameter {
        name: parameter,
        r#type,
    } in parameters
    {
        let anchored = anchor(r#type, module);
        let mut crossing = BTreeSet::new();
        let reason = walk.boundary(&anchored, &mut BTreeSet::new(), &mut crossing);
        crossings.push((format!("parameter {parameter}"), crossing));
        match reason {
            Some(reason) => walk.violation(
                &root_site,
                type_key(r#type),
                format!("root parameter `{parameter}` holds {reason}"),
            ),
            None => {}
        }
    }
    let mut crossing = BTreeSet::new();
    let reason = walk.boundary(&anchor(result, module), &mut BTreeSet::new(), &mut crossing);
    crossings.push(("result".to_owned(), crossing));
    match reason {
        Some(reason) => walk.violation(
            &root_site,
            type_key(result),
            format!("root result holds {reason}"),
        ),
        None => {}
    }
    walk.queue.push_back((
        Item::Definition {
            module: module.to_owned(),
            name: name.to_owned(),
            type_arguments: Vec::new(),
        },
        Vec::new(),
    ));
    loop {
        match walk.queue.pop_front() {
            Some((item, path)) => walk.visit(&item, path),
            None => break,
        }
    }
    let mut targets = Vec::new();
    for target_id in &production.targets {
        // `check_declaration` admitted only registered targets in linking.
        let target = registry.targets.get(target_id).ok_or_else(|| {
            AnalysisError::Internal(format!(
                "production root `{root}` names unregistered target `{target_id}`"
            ))
        })?;
        let mut violations = walk.violations.clone();
        let depth_of = |instance: &str| {
            walk.members
                .get(instance)
                .map_or(1, |member| member.path.len())
        };
        for (literal, representation, instance) in &walk.literals {
            let bits = representation.bits(target);
            if exceeds(literal, *representation, bits) {
                violations.push(Violation {
                    depth: depth_of(instance),
                    instance: instance.clone(),
                    construct: representation.literal_key().to_owned(),
                    reason: format!(
                        "literal {literal} does not fit the {bits}-bit {} representation of target `{target_id}`",
                        representation.as_str()
                    ),
                });
            }
        }
        let mut effects = walk.effects.clone();
        if target.allocation {
            if !walk.allocation.is_empty() {
                effects.insert("allocation".to_owned(), walk.allocation.clone());
            }
        } else {
            for (construct, instance) in &walk.allocation {
                violations.push(Violation {
                    depth: depth_of(instance),
                    instance: instance.clone(),
                    construct: construct.clone(),
                    reason: format!(
                        "construct `{construct}` requires heap allocation, which target `{target_id}` does not provide"
                    ),
                });
            }
        }
        for (effect, sources) in &effects {
            if !production.effects.contains(effect) {
                let mut ordered: Vec<&(String, String)> = sources.iter().collect();
                ordered.sort_by_key(|(construct, instance)| {
                    (depth_of(instance), instance.clone(), construct.clone())
                });
                let (construct, instance) = ordered[0];
                violations.push(Violation {
                    depth: depth_of(instance),
                    instance: instance.clone(),
                    construct: construct.clone(),
                    reason: format!(
                        "effect mismatch: construct `{construct}` realizes effect `{effect}`, which the root does not admit"
                    ),
                });
            }
        }
        violations.sort();
        match violations.first() {
            Some(first) => {
                let path = walk
                    .members
                    .get(&first.instance)
                    .map_or_else(|| first.instance.clone(), |member| member.path.join(" -> "));
                return Err(AnalysisError::Ineligible(format!(
                    "production root `{root}` is not eligible for target `{target_id}`: {} (in `{}`, reached by {path}; {} violation(s) in total)",
                    first.reason,
                    first.instance,
                    violations.len()
                )));
            }
            None => {}
        }
        targets.push(TargetRow {
            target: target_id.clone(),
            effects: effects
                .into_iter()
                .map(|(effect, sources)| EffectRow { effect, sources })
                .collect(),
            boundary: crossings
                .iter()
                .flat_map(|(position, crossing)| {
                    crossing.iter().map(|representation| BoundaryRow {
                        position: position.clone(),
                        representation: representation.as_str().to_owned(),
                        bits: representation.bits(target),
                    })
                })
                .collect(),
        });
    }
    let runtime: Vec<ClosureMember> = walk
        .member_order
        .iter()
        .filter_map(|instance| walk.members.get(instance).cloned())
        .collect();
    let reasoning = reasoning_rows(&runtime, modules);
    Ok(RootReport {
        root,
        declared_effects: production.effects.clone(),
        runtime,
        types: walk.types,
        erased: walk.erased,
        constructs: walk.constructs,
        targets,
        reasoning,
    })
}

/// The canonical semantic JSON of a bound term, as the report states it.
fn canonical_term(term: &SemanticTerm) -> String {
    let text = serde_json::to_string(term).expect("semantic terms serialize");
    crate::artifact::canonical_json::Json::parse(text.as_bytes())
        .expect("a serialized term is JSON")
        .to_canonical_string()
}

/// §17.12, §17.13: the explicit resource account of every reasoner whose
/// elaboration the runtime closure reaches. A reasoner's cost is its loop,
/// bounded by its fuel (and, for a search, its frontier), and the
/// generated theorems that bound its ledger are named, never asserted.
fn reasoning_rows(
    runtime: &[ClosureMember],
    modules: &BTreeMap<String, LinkedModule<'_>>,
) -> Vec<ReasoningRow> {
    use crate::ir::semantic::{ReasoningStrategy, SearchOrder};
    let reached: BTreeSet<&str> = runtime
        .iter()
        .map(|member| member.declaration.as_str())
        .collect();
    let mut rows = Vec::new();
    for linked in modules.values() {
        let qualify = |name: &str| format!("{}.{name}", linked.lean_module);
        for (index, declaration) in linked.semantic.declarations.iter().enumerate() {
            let SemanticDeclaration::Reasoner {
                name,
                type_parameters: _,
                logic: _,
                observation: _,
                observe: _,
                rules,
                strategy,
                answer: _,
                verifier: _,
                claims: _,
                executable: _,
                axioms: _,
            } = declaration
            else {
                continue;
            };
            let runs = linked
                .semantic
                .elaboration
                .lowered(index)
                .iter()
                .any(|derived| reached.contains(qualify(derived.name()).as_str()));
            if !runs {
                continue;
            }
            let (kind, deduplicate, fuel, frontier, budget) = match strategy {
                ReasoningStrategy::Forward { fuel } => {
                    ("forward", false, fuel.as_ref(), None, None)
                }
                ReasoningStrategy::GenerateAndVerify {
                    budget,
                    generator: _,
                } => ("generate_and_verify", false, None, None, budget.as_ref()),
                ReasoningStrategy::Search {
                    order,
                    fuel,
                    frontier,
                    deduplicate,
                } => (
                    match order {
                        SearchOrder::BreadthFirst => "breadth_first",
                        SearchOrder::DepthFirst => "depth_first",
                    },
                    *deduplicate,
                    fuel.as_ref(),
                    frontier.as_ref(),
                    None,
                ),
            };
            let rule_names = rules
                .iter()
                .map(|rule| match &rule.member.module {
                    Some(module) => modules.get(module).map_or_else(
                        || format!("{module}.{}", rule.member.name),
                        |other| format!("{}.{}", other.lean_module, rule.member.name),
                    ),
                    None => qualify(&rule.member.name),
                })
                .collect();
            let bounds = linked
                .semantic
                .elaboration
                .theorems(index)
                .iter()
                .filter(|theorem| {
                    [
                        "iterations_bounded",
                        "firings_bounded",
                        "expansions_bounded",
                        "frontier_bounded",
                        "verifications_bounded",
                        "saturates",
                    ]
                    .iter()
                    .any(|suffix| theorem.name == format!("{name}.{suffix}"))
                })
                .map(|theorem| qualify(&theorem.name))
                .collect();
            rows.push(ReasoningRow {
                reasoner: qualify(name),
                strategy: kind.to_owned(),
                deduplicate,
                fuel: fuel.map(canonical_term),
                budget: budget.map(canonical_term),
                frontier: frontier.map(canonical_term),
                rules: rule_names,
                ledger: crate::ir::semantic::reasoning::LEDGER
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                bounds,
            });
        }
    }
    rows.sort_by(|left, right| left.reasoner.cmp(&right.reasoner));
    rows
}

/// The eligibility report of one module, or `None` when it declares no
/// production root.
///
/// # Errors
///
/// Returns the first reason, in a deterministic order, that a root is not
/// eligible for one of its targets.
pub fn analyse_module(
    module: &str,
    modules: &BTreeMap<String, LinkedModule<'_>>,
) -> Result<Option<ModuleReport>, AnalysisError> {
    let linked = match modules.get(module) {
        Some(linked) => linked,
        None => return Ok(None),
    };
    let mut roots = Vec::new();
    for declaration in &linked.semantic.declarations {
        match declaration {
            SemanticDeclaration::Definition {
                name,
                type_parameters,
                parameters,
                result,
                recursive_argument: _,
                body: _,
                axioms: _,
                executable: _,
                mutual: _,
                termination: _,
                production: Some(production),
            } => roots.push(analyse_root(
                module,
                name,
                type_parameters,
                parameters,
                result,
                production,
                modules,
            )?),
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
                termination: _,
                production: None,
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
            } => {}
        }
    }
    if roots.is_empty() {
        return Ok(None);
    }
    Ok(Some(ModuleReport {
        module: module.to_owned(),
        roots,
    }))
}

#[cfg(test)]
mod tests {
    use super::{analyse_module, AnalysisError, LinkedModule};
    use crate::ir::semantic::SemanticModule;
    use std::collections::BTreeMap;

    /// Analyse one module of unlinked IR. Linking refuses these modules, so
    /// only the analysis can show that it refuses them too.
    fn ineligibility(declarations: &str) -> String {
        let text =
            format!(r#"{{"declarations":{declarations},"spec":"lexlean/semantic-module/2"}}"#);
        let semantic: SemanticModule = serde_json::from_str(&text).expect("module JSON");
        let modules = BTreeMap::from([(
            "Main".to_owned(),
            LinkedModule {
                lean_module: "Probe.Main",
                semantic: &semantic,
            },
        )]);
        match analyse_module("Main", &modules) {
            Ok(report) => panic!("the root is reported eligible: {report:?}"),
            Err(AnalysisError::Ineligible(reason)) => reason,
            Err(AnalysisError::Internal(reason)) => panic!("internal failure: {reason}"),
        }
    }

    const ROOT: &str = r#""executable":true,"kind":"definition","name":"entry","parameters":[],"production":{"effects":[],"targets":["rust-std"]},"result":{"kind":"nat"}"#;

    #[test]
    fn a_reached_theorem_is_proof_only() {
        let reason = ineligibility(&format!(
            r#"[{{"kind":"theorem","name":"fact","parameters":[],"proof":{{"kind":"decide"}},"statement":{{"kind":"le","left":{{"kind":"nat","value":"0"}},"right":{{"kind":"nat","value":"1"}}}}}},{{"body":{{"arguments":[],"function":{{"name":"fact"}},"kind":"call"}},{ROOT}}}]"#
        ));
        assert!(
            reason.contains("`Probe.Main.fact` is not a computational definition")
                || reason.contains("construct `declaration.theorem` is proof-only"),
            "{reason}"
        );
    }

    #[test]
    fn a_missing_dependency_is_unresolved() {
        let reason = ineligibility(&format!(
            r#"[{{"body":{{"arguments":[],"function":{{"name":"absent"}},"kind":"call"}},{ROOT}}}]"#
        ));
        assert!(reason.contains("is an unresolved dependency"), "{reason}");
    }

    #[test]
    fn a_non_executable_callee_is_refused() {
        let reason = ineligibility(&format!(
            r#"[{{"body":{{"kind":"nat","value":"1"}},"kind":"definition","name":"helper","parameters":[],"result":{{"kind":"nat"}}}},{{"body":{{"arguments":[],"function":{{"name":"helper"}},"kind":"call"}},{ROOT}}}]"#
        ));
        assert!(reason.contains("is not declared executable"), "{reason}");
    }
}
