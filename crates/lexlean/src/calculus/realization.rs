//! The realization table (SPEC.md §17.14): for every construct whose
//! production disposition is `runtime` (§17.13), the calculus elements that
//! realize it.
//!
//! A reference names a type (`type:<kind>`), a literal (`value:<kind>`), an
//! expression (`expr:<kind>`), a constructor shape (`shape:<kind>`), a
//! primitive (`prim:<kind>`), a library template (`template:<name>`), or one
//! of four structural realizations: a program `function`,
//! `monomorphization` (a type parameter is substituted by its closed type
//! argument before realization, §17.13), `representation` (the operation is
//! the identity on the realized representation), and `indirection` (a type
//! that contains itself holds that occurrence behind a shared heap handle).
//! The conformance suite checks that the table and the runtime rows of the
//! production registry are in bijection, that every reference names an
//! element that exists, that fixtures exercise every element, and that a
//! construct requires heap allocation exactly when its realization does
//! (`allocates`).

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use super::check::Checker;
use super::library::Template;
use super::{Adt, Arm, Expr, Function, IntKind, OrderingValue, Prim, Program, Shape, Ty, Value};

/// The structural realizations that are not calculus syntax.
pub const STRUCTURAL: [&str; 4] = [
    "function",
    "monomorphization",
    "representation",
    "indirection",
];

/// Every runtime construct and its realization.
pub const TABLE: &[(&str, &[&str])] = &[
    ("type.parameter", &["monomorphization"]),
    ("type.nat", &["type:nat"]),
    ("type.bool", &["type:bool"]),
    ("type.unit", &["type:unit"]),
    ("type.int", &["type:int"]),
    ("type.int8", &["type:fixed"]),
    ("type.int16", &["type:fixed"]),
    ("type.int32", &["type:fixed"]),
    ("type.int64", &["type:fixed"]),
    ("type.uint8", &["type:fixed"]),
    ("type.uint16", &["type:fixed"]),
    ("type.uint32", &["type:fixed"]),
    ("type.uint64", &["type:fixed"]),
    ("type.string", &["type:string"]),
    ("type.bytes", &["type:bytes"]),
    ("type.ordering", &["type:ordering"]),
    ("type.option", &["type:option"]),
    ("type.result", &["type:result"]),
    ("type.list", &["type:list"]),
    ("type.named", &["type:adt"]),
    ("type.product", &["type:pair"]),
    ("type.function", &["type:fn"]),
    ("type.map", &["type:list", "type:pair"]),
    ("type.set", &["type:list"]),
    // A refusal is the pair of Booleans (after run, invariant) (§17.12).
    ("type.contract_violation", &["type:pair", "type:bool"]),
    // A reasoning failure is the pair of Booleans (answered, replay).
    ("type.reasoning_failure", &["type:pair", "type:bool"]),
    ("term.var", &["expr:var"]),
    ("term.nat", &["expr:value", "value:nat"]),
    (
        "term.integer",
        &[
            "expr:value",
            "value:int",
            "value:i8",
            "value:i16",
            "value:i32",
            "value:i64",
            "value:u8",
            "value:u16",
            "value:u32",
            "value:u64",
        ],
    ),
    ("term.string", &["expr:value", "value:string"]),
    ("term.bytes", &["expr:value", "value:bytes"]),
    ("term.primitive", &["expr:prim"]),
    ("term.bool", &["expr:build", "shape:true", "shape:false"]),
    ("term.unit", &["expr:build", "shape:unit"]),
    ("term.nil", &["expr:build", "shape:nil"]),
    ("term.cons", &["expr:build", "shape:cons"]),
    ("term.record", &["expr:build", "shape:adt"]),
    (
        "term.constructor",
        &[
            "expr:build",
            "shape:adt",
            "shape:none",
            "shape:some",
            "shape:ok",
            "shape:error",
            "shape:zero",
            "shape:pair",
            "shape:true",
            "shape:false",
        ],
    ),
    ("term.instance_value", &["expr:call", "function"]),
    ("term.project", &["expr:field"]),
    ("term.call", &["expr:call"]),
    ("term.if", &["expr:cond"]),
    ("term.match", &["expr:match"]),
    ("term.add", &["prim:nat_add"]),
    ("term.beq", &["prim:nat_eq"]),
    ("term.ble", &["prim:nat_le"]),
    ("term.blt", &["prim:nat_lt"]),
    // Lean's `&&` and `||` do not evaluate their right operand when the left
    // decides the result, so the realization branches rather than applying
    // a strict primitive: an overflow on the right is never reached.
    ("term.and", &["expr:cond"]),
    ("term.or", &["expr:cond"]),
    ("term.not", &["prim:bool_not"]),
    ("term.let", &["expr:let"]),
    ("term.pair", &["expr:build", "shape:pair"]),
    ("term.first", &["expr:first"]),
    ("term.second", &["expr:second"]),
    ("term.lambda", &["expr:closure", "function"]),
    ("term.apply", &["expr:apply"]),
    ("term.function_ref", &["expr:closure"]),
    (
        "term.map_literal",
        &["expr:build", "shape:cons", "shape:nil", "shape:pair"],
    ),
    (
        "term.set_literal",
        &["expr:build", "shape:cons", "shape:nil"],
    ),
    (
        "term.graph_literal",
        &["expr:build", "shape:cons", "shape:nil", "shape:pair"],
    ),
    // A checked application is realized as what it elaborates to: lets,
    // the model and validator calls, conditionals, the projections of a
    // stateful step, the result constructors, and the refusal value, a
    // `contract_violation` pair of Booleans (§17.12 rules 8 and 9).
    (
        "term.checked_apply",
        &[
            "expr:let",
            "expr:call",
            "expr:cond",
            "expr:first",
            "expr:second",
            "expr:build",
            "shape:ok",
            "shape:error",
            "shape:pair",
            "shape:true",
            "shape:false",
        ],
    ),
    ("constructor.nat_succ", &["expr:build", "shape:succ"]),
    ("primitive.subtract", &["prim:nat_sub", "prim:int_sub"]),
    ("primitive.multiply", &["prim:nat_mul", "prim:int_mul"]),
    ("primitive.quotient", &["prim:nat_quot", "prim:int_quot"]),
    ("primitive.remainder", &["prim:nat_rem", "prim:int_rem"]),
    ("primitive.negate", &["prim:int_neg"]),
    ("primitive.checked_convert", &["prim:convert"]),
    ("primitive.checked_add", &["prim:checked_add"]),
    ("primitive.checked_subtract", &["prim:checked_sub"]),
    ("primitive.checked_multiply", &["prim:checked_mul"]),
    ("primitive.checked_negate", &["prim:checked_neg"]),
    ("primitive.checked_quotient", &["prim:checked_quot"]),
    ("primitive.bit_and", &["prim:bit_and"]),
    ("primitive.bit_or", &["prim:bit_or"]),
    ("primitive.bit_xor", &["prim:bit_xor"]),
    ("primitive.bit_not", &["prim:bit_not"]),
    ("primitive.shift_left", &["prim:shift_left"]),
    ("primitive.shift_right", &["prim:shift_right"]),
    ("primitive.append", &["prim:append"]),
    ("primitive.length", &["prim:length"]),
    ("primitive.index", &["prim:index"]),
    ("primitive.slice", &["prim:slice"]),
    ("primitive.utf8_encode", &["prim:utf8_encode"]),
    ("primitive.utf8_decode", &["prim:utf8_decode"]),
    ("primitive.compare_bytes", &["prim:compare_bytes"]),
    ("primitive.equal", &["prim:equal"]),
    ("primitive.split_exact", &["prim:split_exact"]),
    ("primitive.join", &["prim:join"]),
    ("primitive.parse_decimal", &["prim:parse_decimal"]),
    ("primitive.format_decimal", &["prim:format_decimal"]),
    ("primitive.map_insert", &["template:map_insert"]),
    ("primitive.map_remove", &["template:map_remove"]),
    ("primitive.map_lookup", &["template:map_lookup"]),
    ("primitive.map_contains", &["template:map_contains"]),
    ("primitive.map_size", &["prim:length"]),
    ("primitive.map_keys", &["template:map_keys"]),
    ("primitive.map_values", &["template:map_values"]),
    ("primitive.map_entries", &["representation"]),
    ("primitive.map_fold", &["template:map_fold"]),
    ("primitive.set_insert", &["template:set_insert"]),
    ("primitive.set_remove", &["template:set_remove"]),
    ("primitive.set_contains", &["template:set_contains"]),
    ("primitive.set_size", &["prim:length"]),
    ("primitive.set_elements", &["representation"]),
    ("primitive.set_union", &["template:set_union"]),
    ("primitive.set_intersection", &["template:set_intersection"]),
    ("primitive.set_difference", &["template:set_difference"]),
    ("primitive.set_fold", &["template:set_fold"]),
    ("primitive.list_fold", &["template:list_fold"]),
    (
        "primitive.less_than",
        &[
            "prim:compare",
            "expr:match",
            "shape:lt",
            "shape:eq",
            "shape:gt",
            "expr:build",
            "shape:true",
            "shape:false",
        ],
    ),
    ("primitive.iterate", &["template:iterate"]),
    ("primitive.iterate_until", &["template:iterate_until"]),
    ("primitive.graph_successors", &["template:graph_successors"]),
    ("primitive.graph_reachable", &["template:graph_reachable"]),
    (
        "primitive.graph_topological",
        &["template:graph_topological"],
    ),
    ("declaration.structure", &["type:adt", "expr:field"]),
    ("declaration.class", &["type:adt"]),
    ("declaration.instance", &["function", "shape:adt"]),
    ("declaration.inductive", &["type:adt", "shape:adt"]),
    (
        "declaration.inductive.recursive",
        &["type:adt", "shape:adt", "indirection"],
    ),
    ("declaration.definition", &["function"]),
    (
        "declaration.definition.recursive",
        &["function", "expr:call"],
    ),
    // A model construct is realized through the ordinary definitions it
    // elaborates to: an artifact as a constant function, a realization as
    // its denotation and companions, a model as a call of its realization.
    ("declaration.artifact", &["function"]),
    ("declaration.realization", &["function"]),
    ("declaration.model", &["function", "expr:call"]),
    // A rule is realized as its guard, conclusion, candidates, and guarded
    // application; a reasoner as the definitions of its bounded loops, whose
    // step and record types are ordinary data declarations (§17.12).
    ("declaration.inference_rule", &["function"]),
    ("declaration.reasoner", &["function"]),
];

/// The serialized `kind` tag of a closed calculus value.
pub(crate) fn kind<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|json| {
            json.get("kind")
                .and_then(|kind| kind.as_str())
                .map(str::to_owned)
        })
        .unwrap_or_default()
}

/// One sample of every type kind; [`ty_index`] proves the list complete.
fn ty_samples() -> Vec<Ty> {
    let nat = || Box::new(Ty::Nat);
    vec![
        Ty::Unit,
        Ty::Bool,
        Ty::Nat,
        Ty::Int,
        Ty::Fixed { width: IntKind::U8 },
        Ty::String,
        Ty::Bytes,
        Ty::Ordering,
        Ty::Option { value: nat() },
        Ty::Result {
            ok: nat(),
            error: nat(),
        },
        Ty::List { element: nat() },
        Ty::Pair {
            left: nat(),
            right: nat(),
        },
        Ty::Adt { index: 0 },
        Ty::Fn {
            parameters: vec![Ty::Nat],
            result: nat(),
        },
    ]
}

const fn ty_index(ty: &Ty) -> usize {
    match ty {
        Ty::Unit => 0,
        Ty::Bool => 1,
        Ty::Nat => 2,
        Ty::Int => 3,
        Ty::Fixed { .. } => 4,
        Ty::String => 5,
        Ty::Bytes => 6,
        Ty::Ordering => 7,
        Ty::Option { .. } => 8,
        Ty::Result { .. } => 9,
        Ty::List { .. } => 10,
        Ty::Pair { .. } => 11,
        Ty::Adt { .. } => 12,
        Ty::Fn { .. } => 13,
    }
}

fn value_samples() -> Vec<Value> {
    let text = || "0".to_owned();
    let unit = || Box::new(Value::Unit);
    vec![
        Value::Unit,
        Value::Bool { value: false },
        Value::Nat { value: text() },
        Value::Int { value: text() },
        Value::U8 { value: text() },
        Value::U16 { value: text() },
        Value::U32 { value: text() },
        Value::U64 { value: text() },
        Value::I8 { value: text() },
        Value::I16 { value: text() },
        Value::I32 { value: text() },
        Value::I64 { value: text() },
        Value::String {
            value: String::new(),
        },
        Value::Bytes { hex: String::new() },
        Value::Ordering {
            value: OrderingValue::Eq,
        },
        Value::None,
        Value::Some { value: unit() },
        Value::Ok { value: unit() },
        Value::Error { value: unit() },
        Value::List { items: Vec::new() },
        Value::Pair {
            left: unit(),
            right: unit(),
        },
        Value::Adt {
            constructor: 0,
            fields: Vec::new(),
        },
        Value::Closure {
            function: 0,
            captures: Vec::new(),
        },
    ]
}

const fn value_index(value: &Value) -> usize {
    match value {
        Value::Unit => 0,
        Value::Bool { .. } => 1,
        Value::Nat { .. } => 2,
        Value::Int { .. } => 3,
        Value::U8 { .. } => 4,
        Value::U16 { .. } => 5,
        Value::U32 { .. } => 6,
        Value::U64 { .. } => 7,
        Value::I8 { .. } => 8,
        Value::I16 { .. } => 9,
        Value::I32 { .. } => 10,
        Value::I64 { .. } => 11,
        Value::String { .. } => 12,
        Value::Bytes { .. } => 13,
        Value::Ordering { .. } => 14,
        Value::None => 15,
        Value::Some { .. } => 16,
        Value::Ok { .. } => 17,
        Value::Error { .. } => 18,
        Value::List { .. } => 19,
        Value::Pair { .. } => 20,
        Value::Adt { .. } => 21,
        Value::Closure { .. } => 22,
    }
}

fn expr_samples() -> Vec<Expr> {
    let leaf = || Box::new(Expr::Var { name: 0 });
    vec![
        Expr::Value {
            ty: Ty::Unit,
            value: Value::Unit,
        },
        Expr::Var { name: 0 },
        Expr::Let {
            name: 0,
            ty: Ty::Unit,
            bound: leaf(),
            body: leaf(),
        },
        Expr::Cond {
            condition: leaf(),
            then_branch: leaf(),
            else_branch: leaf(),
        },
        Expr::Match {
            ty: Ty::Unit,
            scrutinee: leaf(),
            arms: Vec::new(),
        },
        Expr::Build {
            shape: Shape::Unit,
            ty: Ty::Unit,
            operands: Vec::new(),
        },
        Expr::Call {
            function: 0,
            operands: Vec::new(),
        },
        Expr::Closure {
            function: 0,
            captures: Vec::new(),
        },
        Expr::Apply {
            target: leaf(),
            operands: Vec::new(),
        },
        Expr::Prim {
            operation: Prim::BoolNot,
            operands: Vec::new(),
        },
        Expr::First { value: leaf() },
        Expr::Second { value: leaf() },
        Expr::Field {
            value: leaf(),
            index: 0,
        },
    ]
}

const fn expr_index(expr: &Expr) -> usize {
    match expr {
        Expr::Value { .. } => 0,
        Expr::Var { .. } => 1,
        Expr::Let { .. } => 2,
        Expr::Cond { .. } => 3,
        Expr::Match { .. } => 4,
        Expr::Build { .. } => 5,
        Expr::Call { .. } => 6,
        Expr::Closure { .. } => 7,
        Expr::Apply { .. } => 8,
        Expr::Prim { .. } => 9,
        Expr::First { .. } => 10,
        Expr::Second { .. } => 11,
        Expr::Field { .. } => 12,
    }
}

fn shape_samples() -> Vec<Shape> {
    vec![
        Shape::None,
        Shape::Some,
        Shape::Ok,
        Shape::Error,
        Shape::Nil,
        Shape::Cons,
        Shape::Zero,
        Shape::Succ,
        Shape::Pair,
        Shape::True,
        Shape::False,
        Shape::Unit,
        Shape::Lt,
        Shape::Eq,
        Shape::Gt,
        Shape::Adt { constructor: 0 },
    ]
}

const fn shape_index(shape: Shape) -> usize {
    match shape {
        Shape::None => 0,
        Shape::Some => 1,
        Shape::Ok => 2,
        Shape::Error => 3,
        Shape::Nil => 4,
        Shape::Cons => 5,
        Shape::Zero => 6,
        Shape::Succ => 7,
        Shape::Pair => 8,
        Shape::True => 9,
        Shape::False => 10,
        Shape::Unit => 11,
        Shape::Lt => 12,
        Shape::Eq => 13,
        Shape::Gt => 14,
        Shape::Adt { .. } => 15,
    }
}

fn prim_samples() -> Vec<Prim> {
    vec![
        Prim::NatAdd,
        Prim::NatSub,
        Prim::NatMul,
        Prim::NatQuot,
        Prim::NatRem,
        Prim::NatEq,
        Prim::NatLe,
        Prim::NatLt,
        Prim::IntAdd,
        Prim::IntSub,
        Prim::IntMul,
        Prim::IntNeg,
        Prim::IntQuot,
        Prim::IntRem,
        Prim::CheckedAdd,
        Prim::CheckedSub,
        Prim::CheckedMul,
        Prim::CheckedNeg,
        Prim::CheckedQuot,
        Prim::BitAnd,
        Prim::BitOr,
        Prim::BitXor,
        Prim::BitNot,
        Prim::ShiftLeft,
        Prim::ShiftRight,
        Prim::Equal,
        Prim::BoolNot,
        Prim::BoolAnd,
        Prim::BoolOr,
        Prim::Append,
        Prim::Length,
        Prim::Index,
        Prim::Slice,
        Prim::Utf8Encode,
        Prim::Utf8Decode,
        Prim::CompareBytes,
        Prim::SplitExact,
        Prim::Join,
        Prim::FormatDecimal,
        Prim::Compare,
        Prim::Convert {
            target: IntKind::U8,
        },
        Prim::ParseDecimal { target: Ty::Int },
    ]
}

const fn prim_index(prim: &Prim) -> usize {
    match prim {
        Prim::NatAdd => 0,
        Prim::NatSub => 1,
        Prim::NatMul => 2,
        Prim::NatQuot => 3,
        Prim::NatRem => 4,
        Prim::NatEq => 5,
        Prim::NatLe => 6,
        Prim::NatLt => 7,
        Prim::IntAdd => 8,
        Prim::IntSub => 9,
        Prim::IntMul => 10,
        Prim::IntNeg => 11,
        Prim::IntQuot => 12,
        Prim::IntRem => 13,
        Prim::CheckedAdd => 14,
        Prim::CheckedSub => 15,
        Prim::CheckedMul => 16,
        Prim::CheckedNeg => 17,
        Prim::CheckedQuot => 18,
        Prim::BitAnd => 19,
        Prim::BitOr => 20,
        Prim::BitXor => 21,
        Prim::BitNot => 22,
        Prim::ShiftLeft => 23,
        Prim::ShiftRight => 24,
        Prim::Equal => 25,
        Prim::BoolNot => 26,
        Prim::BoolAnd => 27,
        Prim::BoolOr => 28,
        Prim::Append => 29,
        Prim::Length => 30,
        Prim::Index => 31,
        Prim::Slice => 32,
        Prim::Utf8Encode => 33,
        Prim::Utf8Decode => 34,
        Prim::CompareBytes => 35,
        Prim::SplitExact => 36,
        Prim::Join => 37,
        Prim::FormatDecimal => 38,
        Prim::Compare => 39,
        Prim::Convert { .. } => 40,
        Prim::ParseDecimal { .. } => 41,
    }
}

/// Every element of the calculus, as `<class>:<kind>`.
///
/// # Errors
///
/// Returns the class whose sample list does not cover every variant once,
/// which would make coverage claims about that class unfounded.
pub fn all_elements() -> Result<BTreeSet<String>, String> {
    fn complete<T>(class: &str, samples: &[T], index: impl Fn(&T) -> usize) -> Result<(), String> {
        let indices: BTreeSet<usize> = samples.iter().map(index).collect();
        if indices.len() == samples.len() && indices.iter().copied().eq(0..samples.len()) {
            Ok(())
        } else {
            Err(format!(
                "the {class} samples do not name every variant exactly once"
            ))
        }
    }
    let (types, values, exprs, shapes, prims) = (
        ty_samples(),
        value_samples(),
        expr_samples(),
        shape_samples(),
        prim_samples(),
    );
    complete("type", &types, ty_index)?;
    complete("value", &values, value_index)?;
    complete("expr", &exprs, expr_index)?;
    complete("shape", &shapes, |shape| shape_index(*shape))?;
    complete("prim", &prims, prim_index)?;
    let mut out = BTreeSet::new();
    out.extend(types.iter().map(|ty| format!("type:{}", kind(ty))));
    out.extend(values.iter().map(|value| format!("value:{}", kind(value))));
    out.extend(exprs.iter().map(|expr| format!("expr:{}", kind(expr))));
    out.extend(shapes.iter().map(|shape| format!("shape:{}", kind(shape))));
    out.extend(prims.iter().map(|prim| format!("prim:{}", kind(prim))));
    Ok(out)
}

/// Every element a program uses.
#[must_use]
pub fn program_elements(program: &Program) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut visit = Visit { out: &mut out };
    for adt in &program.adts {
        visit.adt(adt);
    }
    for function in &program.functions {
        visit.function(function);
    }
    out
}

/// Every element a program builds: the shapes it constructs, its literals,
/// and its primitives, but not the shapes it matches or the types it names.
#[must_use]
pub fn built_elements(program: &Program) -> BTreeSet<String> {
    fn walk(expr: &Expr, out: &mut BTreeSet<String>) {
        let mut children: Vec<&Expr> = Vec::new();
        match expr {
            Expr::Value { value, .. } => {
                for element in value_elements(value) {
                    out.insert(element);
                }
            }
            Expr::Var { .. } => {}
            Expr::Let { bound, body, .. } => children.extend([bound.as_ref(), body.as_ref()]),
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => children.extend([
                condition.as_ref(),
                then_branch.as_ref(),
                else_branch.as_ref(),
            ]),
            Expr::Match {
                scrutinee, arms, ..
            } => {
                children.push(scrutinee);
                children.extend(arms.iter().map(|arm| &arm.body));
            }
            Expr::Build {
                shape, operands, ..
            } => {
                out.insert(format!("shape:{}", kind(shape)));
                children.extend(operands);
            }
            Expr::Prim {
                operation,
                operands,
            } => {
                out.insert(format!("prim:{}", kind(operation)));
                children.extend(operands);
            }
            Expr::Call { operands, .. }
            | Expr::Closure {
                captures: operands, ..
            } => children.extend(operands),
            Expr::Apply { target, operands } => {
                children.push(target);
                children.extend(operands);
            }
            Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
                children.push(value);
            }
        }
        for child in children {
            walk(child, out);
        }
    }
    let mut out = BTreeSet::new();
    for function in &program.functions {
        walk(&function.body, &mut out);
    }
    out
}

/// Every (primitive, width) pair a program applies: the width of a
/// fixed-width first operand, or the target of a conversion or a parse, and
/// `int` for decimal formatting and parsing of `int`.
///
/// # Errors
///
/// Returns the reason the program is ill-typed.
pub fn fixed_uses(program: &Program) -> Result<BTreeSet<(String, String)>, String> {
    fn width(ty: &Ty) -> Option<&'static str> {
        match ty {
            Ty::Fixed { width } => Some(width.name()),
            Ty::Int => Some("int"),
            _ => None,
        }
    }
    fn walk(
        checker: &Checker<'_>,
        expr: &Expr,
        scope: &mut Vec<(u64, Ty)>,
        out: &mut BTreeSet<(String, String)>,
    ) -> Result<(), String> {
        let mut children: Vec<&Expr> = Vec::new();
        match expr {
            Expr::Value { .. } | Expr::Var { .. } => {}
            Expr::Let {
                name,
                ty,
                bound,
                body,
            } => {
                walk(checker, bound, scope, out)?;
                scope.push((*name, ty.clone()));
                let walked = walk(checker, body, scope, out);
                scope.pop();
                return walked;
            }
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => children.extend([
                condition.as_ref(),
                then_branch.as_ref(),
                else_branch.as_ref(),
            ]),
            Expr::Match {
                scrutinee, arms, ..
            } => {
                walk(checker, scrutinee, scope, out)?;
                let scrutinee_ty = checker.expr(scrutinee, scope)?;
                for arm in arms {
                    let fields = checker.shape_fields(arm.shape, &scrutinee_ty)?;
                    let depth = scope.len();
                    scope.extend(arm.binders.iter().copied().zip(fields));
                    let walked = walk(checker, &arm.body, scope, out);
                    scope.truncate(depth);
                    walked?;
                }
                return Ok(());
            }
            Expr::Prim {
                operation,
                operands,
            } => {
                let used = match operation {
                    Prim::Convert { target } => Some(target.name()),
                    Prim::ParseDecimal { target } => width(target),
                    _ => match operands.first() {
                        Some(operand) => width(&checker.expr(operand, scope)?),
                        None => None,
                    },
                };
                // `int` is a width only of decimal formatting and parsing;
                // its arithmetic and order are their own primitives.
                let decimal = matches!(operation, Prim::FormatDecimal | Prim::ParseDecimal { .. });
                if let Some(used) = used.filter(|used| *used != "int" || decimal) {
                    out.insert((kind(operation), used.to_owned()));
                }
                children.extend(operands);
            }
            Expr::Build { operands, .. }
            | Expr::Call { operands, .. }
            | Expr::Closure {
                captures: operands, ..
            } => children.extend(operands),
            Expr::Apply { target, operands } => {
                children.push(target);
                children.extend(operands);
            }
            Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
                children.push(value);
            }
        }
        for child in children {
            walk(checker, child, scope, out)?;
        }
        Ok(())
    }
    let checker = Checker { program };
    let mut out = BTreeSet::new();
    for function in &program.functions {
        let mut scope: Vec<(u64, Ty)> = function
            .parameters
            .iter()
            .copied()
            .zip(function.types.iter().cloned())
            .collect();
        walk(&checker, &function.body, &mut scope, &mut out)?;
    }
    Ok(out)
}

/// The (primitive, width) pairs the calculus admits: every fixed-width
/// primitive at every width, negation at the signed widths, and decimal
/// formatting and parsing at `int` too.
#[must_use]
pub fn fixed_pairs() -> BTreeSet<(String, String)> {
    let mut out = BTreeSet::new();
    for width in IntKind::ALL {
        let mut operations = vec![
            Prim::CheckedAdd,
            Prim::CheckedSub,
            Prim::CheckedMul,
            Prim::CheckedQuot,
            Prim::BitAnd,
            Prim::BitOr,
            Prim::BitXor,
            Prim::BitNot,
            Prim::ShiftLeft,
            Prim::ShiftRight,
            Prim::Equal,
            Prim::Compare,
            Prim::FormatDecimal,
            Prim::Convert { target: width },
            Prim::ParseDecimal {
                target: Ty::Fixed { width },
            },
        ];
        if width.signed() {
            operations.push(Prim::CheckedNeg);
        }
        for operation in operations {
            out.insert((kind(&operation), width.name().to_owned()));
        }
    }
    out.insert((kind(&Prim::FormatDecimal), "int".to_owned()));
    out.insert((
        kind(&Prim::ParseDecimal { target: Ty::Int }),
        "int".to_owned(),
    ));
    out
}

/// Every element a value uses.
#[must_use]
pub fn value_elements(value: &Value) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    Visit { out: &mut out }.value(value);
    out
}

struct Visit<'a> {
    out: &'a mut BTreeSet<String>,
}

impl Visit<'_> {
    fn adt(&mut self, adt: &Adt) {
        for ty in adt.constructors.iter().flatten() {
            self.ty(ty);
        }
    }

    fn function(&mut self, function: &Function) {
        for ty in function.types.iter().chain([&function.result]) {
            self.ty(ty);
        }
        self.expr(&function.body);
    }

    fn ty(&mut self, ty: &Ty) {
        self.out.insert(format!("type:{}", kind(ty)));
        match ty {
            Ty::Option { value } | Ty::List { element: value } => self.ty(value),
            Ty::Result { ok, error } => {
                self.ty(ok);
                self.ty(error);
            }
            Ty::Pair { left, right } => {
                self.ty(left);
                self.ty(right);
            }
            Ty::Fn { parameters, result } => {
                for parameter in parameters {
                    self.ty(parameter);
                }
                self.ty(result);
            }
            Ty::Unit
            | Ty::Bool
            | Ty::Nat
            | Ty::Int
            | Ty::Fixed { .. }
            | Ty::String
            | Ty::Bytes
            | Ty::Ordering
            | Ty::Adt { .. } => {}
        }
    }

    fn value(&mut self, value: &Value) {
        self.out.insert(format!("value:{}", kind(value)));
        match value {
            Value::Some { value } | Value::Ok { value } | Value::Error { value } => {
                self.value(value)
            }
            Value::List { items: values }
            | Value::Adt { fields: values, .. }
            | Value::Closure {
                captures: values, ..
            } => {
                for value in values {
                    self.value(value);
                }
            }
            Value::Pair { left, right } => {
                self.value(left);
                self.value(right);
            }
            Value::Unit
            | Value::Bool { .. }
            | Value::Nat { .. }
            | Value::Int { .. }
            | Value::U8 { .. }
            | Value::U16 { .. }
            | Value::U32 { .. }
            | Value::U64 { .. }
            | Value::I8 { .. }
            | Value::I16 { .. }
            | Value::I32 { .. }
            | Value::I64 { .. }
            | Value::String { .. }
            | Value::Bytes { .. }
            | Value::Ordering { .. }
            | Value::None => {}
        }
    }

    fn arm(&mut self, arm: &Arm) {
        self.out.insert(format!("shape:{}", kind(&arm.shape)));
        self.expr(&arm.body);
    }

    fn expr(&mut self, expr: &Expr) {
        self.out.insert(format!("expr:{}", kind(expr)));
        match expr {
            Expr::Value { ty, value } => {
                self.ty(ty);
                self.value(value);
            }
            Expr::Var { .. } => {}
            Expr::Let {
                ty, bound, body, ..
            } => {
                self.ty(ty);
                self.expr(bound);
                self.expr(body);
            }
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => {
                self.expr(condition);
                self.expr(then_branch);
                self.expr(else_branch);
            }
            Expr::Match {
                ty,
                scrutinee,
                arms,
            } => {
                self.ty(ty);
                self.expr(scrutinee);
                for arm in arms {
                    self.arm(arm);
                }
            }
            Expr::Build {
                shape,
                ty,
                operands,
            } => {
                self.out.insert(format!("shape:{}", kind(shape)));
                self.ty(ty);
                for operand in operands {
                    self.expr(operand);
                }
            }
            Expr::Call { operands, .. }
            | Expr::Closure {
                captures: operands, ..
            } => {
                for operand in operands {
                    self.expr(operand);
                }
            }
            Expr::Apply { target, operands } => {
                self.expr(target);
                for operand in operands {
                    self.expr(operand);
                }
            }
            Expr::Prim {
                operation,
                operands,
            } => {
                self.out.insert(format!("prim:{}", kind(operation)));
                if let Prim::ParseDecimal { target } = operation {
                    self.ty(target);
                }
                for operand in operands {
                    self.expr(operand);
                }
            }
            Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
                self.expr(value)
            }
        }
    }
}

/// Whether a realization element requires heap allocation (§17.13): a
/// type whose values live on the heap, a literal or constructor that builds
/// heap storage, a primitive that produces new heap storage, a template
/// whose instance builds any, or `indirection`. Reading, comparing, or
/// measuring heap storage that already exists allocates nothing.
///
/// # Errors
///
/// Returns the reference that names no element, or a template that does
/// not instantiate.
pub fn allocates(reference: &str) -> Result<bool, String> {
    const HEAP: [&str; 15] = [
        "type:string",
        "type:bytes",
        "type:list",
        "value:string",
        "value:bytes",
        "value:list",
        "shape:cons",
        "prim:append",
        "prim:slice",
        "prim:utf8_encode",
        "prim:utf8_decode",
        "prim:split_exact",
        "prim:join",
        "prim:format_decimal",
        "indirection",
    ];
    if HEAP.contains(&reference) {
        return Ok(true);
    }
    if let Some(name) = reference.strip_prefix("template:") {
        let template =
            Template::named(name).ok_or_else(|| format!("`{reference}` names no template"))?;
        let instance = Program {
            spec: super::PROGRAM_SPEC.to_owned(),
            adts: Vec::new(),
            functions: template.instantiate(&vec![Ty::Nat; template.arity()], 0)?,
        };
        // Only what the instance builds counts: its types are its operands'
        // and results', which exist before it runs, and taking a cell apart
        // allocates nothing.
        return Ok(built_elements(&instance)
            .iter()
            .any(|element| HEAP.contains(&element.as_str())));
    }
    if all_elements()?.contains(reference) || STRUCTURAL.contains(&reference) {
        return Ok(false);
    }
    Err(format!("`{reference}` names no calculus element"))
}

/// Whether the realization of `construct` requires heap allocation.
///
/// # Errors
///
/// Returns the construct without a row or a reference to no element.
pub fn construct_allocates(construct: &str) -> Result<bool, String> {
    let (_, references) = TABLE
        .iter()
        .find(|(key, _)| *key == construct)
        .ok_or_else(|| format!("construct `{construct}` has no realization row"))?;
    for reference in *references {
        if allocates(reference)? {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Whether a program requires heap allocation: it uses an allocating
/// element, or a type that contains itself in place.
#[must_use]
pub fn program_allocates(program: &Program) -> bool {
    let types = program_elements(program)
        .into_iter()
        .filter(|element| element.starts_with("type:"));
    if built_elements(program)
        .into_iter()
        .chain(types)
        .any(|element| allocates(&element).unwrap_or(true))
    {
        return true;
    }
    // An owner of in-place storage: an ADT, or a function type, whose
    // closures store their captures in place.
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    enum Owner {
        Adt(u64),
        Fn(Ty),
    }
    fn in_place(ty: &Ty, out: &mut BTreeSet<Owner>) {
        match ty {
            Ty::Option { value } => in_place(value, out),
            Ty::Result { ok, error } => {
                in_place(ok, out);
                in_place(error, out);
            }
            Ty::Pair { left, right } => {
                in_place(left, out);
                in_place(right, out);
            }
            Ty::Adt { index } => {
                out.insert(Owner::Adt(*index));
            }
            Ty::Fn { .. } => {
                out.insert(Owner::Fn(ty.clone()));
            }
            _ => {}
        }
    }
    fn closures(expr: &Expr, out: &mut Vec<(u64, usize)>) {
        fn each(exprs: &[Expr], out: &mut Vec<(u64, usize)>) {
            exprs.iter().for_each(|expr| closures(expr, out));
        }
        match expr {
            Expr::Value { .. } | Expr::Var { .. } => {}
            Expr::Let { bound, body, .. } => {
                closures(bound, out);
                closures(body, out);
            }
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => {
                closures(condition, out);
                closures(then_branch, out);
                closures(else_branch, out);
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                closures(scrutinee, out);
                arms.iter().for_each(|arm| closures(&arm.body, out));
            }
            Expr::Build { operands, .. }
            | Expr::Call { operands, .. }
            | Expr::Prim { operands, .. } => each(operands, out),
            Expr::Closure { function, captures } => {
                out.push((*function, captures.len()));
                each(captures, out);
            }
            Expr::Apply { target, operands } => {
                closures(target, out);
                each(operands, out);
            }
            Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
                closures(value, out);
            }
        }
    }
    // What each function type's closures capture, by the function type a
    // closure of `function` capturing `count` values has.
    let mut captured: BTreeMap<Ty, Vec<Ty>> = BTreeMap::new();
    let mut sites = Vec::new();
    for function in &program.functions {
        closures(&function.body, &mut sites);
    }
    for (function, count) in sites {
        let Some(declared) = usize::try_from(function)
            .ok()
            .and_then(|function| program.functions.get(function))
        else {
            continue;
        };
        let fn_type = Ty::Fn {
            parameters: declared.types.iter().skip(count).cloned().collect(),
            result: Box::new(declared.result.clone()),
        };
        captured
            .entry(fn_type)
            .or_default()
            .extend(declared.types.iter().take(count).cloned());
    }
    let stored = |owner: &Owner| -> Vec<Ty> {
        match owner {
            Owner::Adt(adt) => usize::try_from(*adt)
                .ok()
                .and_then(|adt| program.adts.get(adt))
                .map(|declared| declared.constructors.iter().flatten().cloned().collect())
                .unwrap_or_default(),
            Owner::Fn(ty) => captured.get(ty).cloned().unwrap_or_default(),
        }
    };
    let owners: Vec<Owner> = (0..program.adts.len() as u64)
        .map(Owner::Adt)
        .chain(captured.keys().cloned().map(Owner::Fn))
        .collect();
    owners.iter().any(|start| {
        let mut seen = BTreeSet::new();
        let mut pending = vec![start.clone()];
        while let Some(owner) = pending.pop() {
            let mut reached = BTreeSet::new();
            for ty in stored(&owner) {
                in_place(&ty, &mut reached);
            }
            if reached.contains(start) {
                return true;
            }
            pending.extend(reached.into_iter().filter(|next| seen.insert(next.clone())));
        }
        false
    })
}

/// Check the table against the runtime rows of the production registry.
///
/// # Errors
///
/// Returns the first runtime construct without a row, row without a runtime
/// construct, duplicate row, or reference to no element.
pub fn check_table(runtime: &BTreeSet<String>) -> Result<(), String> {
    let elements = all_elements()?;
    let mut seen = BTreeSet::new();
    for (construct, references) in TABLE {
        if !seen.insert(*construct) {
            return Err(format!("construct `{construct}` has two realization rows"));
        }
        if !runtime.contains(*construct) {
            return Err(format!(
                "realization row `{construct}` is not a runtime construct of the registry"
            ));
        }
        if references.is_empty() {
            return Err(format!("construct `{construct}` has no realization"));
        }
        for reference in *references {
            let known = elements.contains(*reference)
                || STRUCTURAL.contains(reference)
                || reference
                    .strip_prefix("template:")
                    .is_some_and(|name| Template::named(name).is_some());
            if !known {
                return Err(format!(
                    "construct `{construct}` names `{reference}`, which is no calculus element"
                ));
            }
        }
    }
    if let Some(missing) = runtime
        .iter()
        .find(|construct| !seen.contains(construct.as_str()))
    {
        return Err(format!(
            "runtime construct `{missing}` has no realization row"
        ));
    }
    Ok(())
}
