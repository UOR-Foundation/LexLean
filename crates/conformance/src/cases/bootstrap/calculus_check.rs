//! `BS-04` (SPEC.md §17.23, §17.14): the target calculus's checker as
//! language-1.2 source, held to `lexlean::calculus::check`.
//!
//! The roots take the program as calculus data, the constructors of the
//! module's inductives in declaration order, so the encoders below are the
//! one place that order is written down. Reading a program from its
//! canonical JSON is the oracle's `Program::parse`, which is not part of
//! this row: every vector is read by that reader and then handed over as
//! data.

use std::collections::BTreeSet;

use lexlean::calculus::{
    self as target, check, Arm, Expr, Fixture, Function, IntKind, OrderingValue, Prim, Program,
    Shape, Ty, Value, PROGRAM_SPEC,
};
use serde_json::{json, Value as Json};

use std::collections::BTreeMap;

use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::Profile;
use lexlean::production::lower::Lowered;

use super::{list, snake, text, Run, Seeded};
use crate::production_export as harness;
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support;

/// `kindIndex` of the module: the declaration order of `IntKind`.
fn kind_index(kind: IntKind) -> u64 {
    match kind {
        IntKind::U8 => 0,
        IntKind::U16 => 1,
        IntKind::U32 => 2,
        IntKind::U64 => 3,
        IntKind::I8 => 4,
        IntKind::I16 => 5,
        IntKind::I32 => 6,
        IntKind::I64 => 7,
    }
}

fn nat(number: u64) -> Value {
    Value::Nat {
        value: number.to_string(),
    }
}

fn adt(constructor: u64, fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor,
        fields,
    }
}

fn enc_kind(kind: IntKind) -> Value {
    adt(kind_index(kind), Vec::new())
}

/// A list as a chain of its type's own `Nil` and `Cons` constructors, which
/// are constructors `nil` and `cons` of each of the three recursive types:
/// certificate A cannot take, nor build, a list of a recursive type.
fn chain(nil: u64, cons: u64, items: Vec<Value>) -> Value {
    items
        .into_iter()
        .rev()
        .fold(adt(nil, Vec::new()), |tail, head| {
            adt(cons, vec![head, tail])
        })
}

fn ty_chain(items: Vec<Value>) -> Value {
    chain(14, 15, items)
}

fn enc_tys(tys: &[Ty]) -> Value {
    ty_chain(tys.iter().map(enc_ty).collect())
}

/// `Ty`: unit, bool, nat, int, fixed, string, bytes, ordering, option,
/// result, list, pair, adt, fn, nil, cons.
fn enc_ty(ty: &Ty) -> Value {
    match ty {
        Ty::Unit => adt(0, Vec::new()),
        Ty::Bool => adt(1, Vec::new()),
        Ty::Nat => adt(2, Vec::new()),
        Ty::Int => adt(3, Vec::new()),
        Ty::Fixed { width } => adt(4, vec![enc_kind(*width)]),
        Ty::String => adt(5, Vec::new()),
        Ty::Bytes => adt(6, Vec::new()),
        Ty::Ordering => adt(7, Vec::new()),
        Ty::Option { value } => adt(8, vec![enc_ty(value)]),
        Ty::Result { ok, error } => adt(9, vec![enc_ty(ok), enc_ty(error)]),
        Ty::List { element } => adt(10, vec![enc_ty(element)]),
        Ty::Pair { left, right } => adt(11, vec![enc_ty(left), enc_ty(right)]),
        Ty::Adt { index } => adt(12, vec![nat(*index)]),
        Ty::Fn { parameters, result } => adt(13, vec![enc_tys(parameters), enc_ty(result)]),
    }
}

fn enc_values(values: &[Value]) -> Value {
    chain(23, 24, values.iter().map(enc_value).collect())
}

/// `Value`: unit, bool, nat, int, u8..i64, string, bytes, ordering, none,
/// some, ok, error, list, pair, adt, closure. A text member is the UTF-8
/// bytes of the oracle's `String`.
fn enc_value(value: &Value) -> Value {
    let words = |index: u64, member: &str| adt(index, vec![text(member)]);
    match value {
        Value::Unit => adt(0, Vec::new()),
        Value::Bool { value } => adt(1, vec![Value::Bool { value: *value }]),
        Value::Nat { value } => words(2, value),
        Value::Int { value } => words(3, value),
        Value::U8 { value } => words(4, value),
        Value::U16 { value } => words(5, value),
        Value::U32 { value } => words(6, value),
        Value::U64 { value } => words(7, value),
        Value::I8 { value } => words(8, value),
        Value::I16 { value } => words(9, value),
        Value::I32 { value } => words(10, value),
        Value::I64 { value } => words(11, value),
        Value::String { value } => words(12, value),
        Value::Bytes { hex } => words(13, hex),
        Value::Ordering { value } => adt(
            14,
            vec![adt(
                match value {
                    OrderingValue::Lt => 0,
                    OrderingValue::Eq => 1,
                    OrderingValue::Gt => 2,
                },
                Vec::new(),
            )],
        ),
        Value::None => adt(15, Vec::new()),
        Value::Some { value } => adt(16, vec![enc_value(value)]),
        Value::Ok { value } => adt(17, vec![enc_value(value)]),
        Value::Error { value } => adt(18, vec![enc_value(value)]),
        Value::List { items } => adt(19, vec![enc_values(items)]),
        Value::Pair { left, right } => adt(20, vec![enc_value(left), enc_value(right)]),
        Value::Adt {
            constructor,
            fields,
        } => adt(21, vec![nat(*constructor), enc_values(fields)]),
        Value::Closure { function, captures } => {
            adt(22, vec![nat(*function), enc_values(captures)])
        }
    }
}

/// `Shape`: none, some, ok, error, nil, cons, zero, succ, pair, true, false,
/// unit, lt, eq, gt, adt.
fn enc_shape(shape: Shape) -> Value {
    let index = match shape {
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
        Shape::Adt { constructor } => return adt(15, vec![nat(constructor)]),
    };
    adt(index, Vec::new())
}

/// `Prim`: the forty primitives in the oracle's order, then `convert` and
/// `parse_decimal`.
fn enc_prim(operation: &Prim) -> Value {
    let index = match operation {
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
        Prim::Convert { target } => return adt(40, vec![enc_kind(*target)]),
        Prim::ParseDecimal { target } => return adt(41, vec![enc_ty(target)]),
    };
    adt(index, Vec::new())
}

fn enc_exprs(exprs: &[Expr]) -> Value {
    chain(13, 14, exprs.iter().map(enc_expr).collect())
}

fn enc_arm(arm: &Arm) -> Value {
    adt(
        15,
        vec![
            enc_shape(arm.shape),
            list(arm.binders.iter().map(|binder| nat(*binder)).collect()),
            enc_expr(&arm.body),
        ],
    )
}

/// `Expr`: value, var, let, cond, match, build, call, closure, apply, prim,
/// first, second, field.
fn enc_expr(expr: &Expr) -> Value {
    match expr {
        Expr::Value { ty, value } => adt(0, vec![enc_ty(ty), enc_value(value)]),
        Expr::Var { name } => adt(1, vec![nat(*name)]),
        Expr::Let {
            name,
            ty,
            bound,
            body,
        } => adt(
            2,
            vec![nat(*name), enc_ty(ty), enc_expr(bound), enc_expr(body)],
        ),
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => adt(
            3,
            vec![
                enc_expr(condition),
                enc_expr(then_branch),
                enc_expr(else_branch),
            ],
        ),
        Expr::Match {
            ty,
            scrutinee,
            arms,
        } => adt(
            4,
            vec![
                enc_ty(ty),
                enc_expr(scrutinee),
                chain(13, 14, arms.iter().map(enc_arm).collect()),
            ],
        ),
        Expr::Build {
            shape,
            ty,
            operands,
        } => adt(5, vec![enc_shape(*shape), enc_ty(ty), enc_exprs(operands)]),
        Expr::Call { function, operands } => adt(6, vec![nat(*function), enc_exprs(operands)]),
        Expr::Closure { function, captures } => adt(7, vec![nat(*function), enc_exprs(captures)]),
        Expr::Apply { target, operands } => adt(8, vec![enc_expr(target), enc_exprs(operands)]),
        Expr::Prim {
            operation,
            operands,
        } => adt(9, vec![enc_prim(operation), enc_exprs(operands)]),
        Expr::First { value } => adt(10, vec![enc_expr(value)]),
        Expr::Second { value } => adt(11, vec![enc_expr(value)]),
        Expr::Field { value, index } => adt(12, vec![enc_expr(value), nat(*index)]),
    }
}

/// The program as the roots take it: its spec, the constructors of its
/// declarations, and the parameters, types, result, and body of its functions
/// as parallel lists, the lists of recursive types as chains. Certificate A
/// cannot take a structure holding a list of a recursive type, so the module
/// rebuilds its `Program` inside the root from these.
fn program_arguments(program: &Program) -> Vec<Value> {
    vec![
        text(&program.spec),
        ty_chain(
            program
                .adts
                .iter()
                .map(|declared| {
                    ty_chain(
                        declared
                            .constructors
                            .iter()
                            .map(|fields| enc_tys(fields))
                            .collect(),
                    )
                })
                .collect(),
        ),
        list(
            program
                .functions
                .iter()
                .map(|function| list(function.parameters.iter().map(|name| nat(*name)).collect()))
                .collect(),
        ),
        ty_chain(
            program
                .functions
                .iter()
                .map(|function| enc_tys(&function.types))
                .collect(),
        ),
        ty_chain(
            program
                .functions
                .iter()
                .map(|function| enc_ty(&function.result))
                .collect(),
        ),
        chain(
            13,
            14,
            program
                .functions
                .iter()
                .map(|function| enc_expr(&function.body))
                .collect(),
        ),
    ]
}

// The encoding is a faithful image of the program: each chain is the list it
// stands for, so decoding the arguments returns the program. The case decodes
// every vector and compares.

fn fields(value: &Value) -> (u64, &[Value]) {
    match value {
        Value::Adt {
            constructor,
            fields,
        } => (*constructor, fields),
        other => panic!("not a constructor: {other:?}"),
    }
}

fn unchain(value: &Value, nil: u64, cons: u64) -> Vec<&Value> {
    let mut out = Vec::new();
    let mut at = value;
    loop {
        let (constructor, parts) = fields(at);
        if constructor == nil {
            return out;
        }
        assert_eq!(constructor, cons, "a chain holds Nil and Cons only");
        out.push(&parts[0]);
        at = &parts[1];
    }
}

fn dec_text(value: &Value) -> String {
    let Value::Bytes { hex } = value else {
        panic!("not bytes: {value:?}")
    };
    let raw: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).expect("hex"))
        .collect();
    String::from_utf8(raw).expect("UTF-8")
}

fn dec_nat(value: &Value) -> u64 {
    let Value::Nat { value } = value else {
        panic!("not a nat: {value:?}")
    };
    value.parse().expect("a nat")
}

fn dec_kind(value: &Value) -> IntKind {
    IntKind::ALL[usize::try_from(fields(value).0).expect("index")]
}

fn dec_tys(value: &Value) -> Vec<Ty> {
    unchain(value, 14, 15).into_iter().map(dec_ty).collect()
}

fn dec_ty(value: &Value) -> Ty {
    let (constructor, parts) = fields(value);
    let boxed = |at: usize| Box::new(dec_ty(&parts[at]));
    match constructor {
        0 => Ty::Unit,
        1 => Ty::Bool,
        2 => Ty::Nat,
        3 => Ty::Int,
        4 => Ty::Fixed {
            width: dec_kind(&parts[0]),
        },
        5 => Ty::String,
        6 => Ty::Bytes,
        7 => Ty::Ordering,
        8 => Ty::Option { value: boxed(0) },
        9 => Ty::Result {
            ok: boxed(0),
            error: boxed(1),
        },
        10 => Ty::List { element: boxed(0) },
        11 => Ty::Pair {
            left: boxed(0),
            right: boxed(1),
        },
        12 => Ty::Adt {
            index: dec_nat(&parts[0]),
        },
        13 => Ty::Fn {
            parameters: dec_tys(&parts[0]),
            result: boxed(1),
        },
        other => panic!("not a type: {other}"),
    }
}

fn dec_values(value: &Value) -> Vec<Value> {
    unchain(value, 23, 24).into_iter().map(dec_value).collect()
}

fn dec_value(value: &Value) -> Value {
    let (constructor, parts) = fields(value);
    let boxed = |at: usize| Box::new(dec_value(&parts[at]));
    let words = || dec_text(&parts[0]);
    match constructor {
        0 => Value::Unit,
        1 => {
            let Value::Bool { value } = &parts[0] else {
                panic!("not a bool")
            };
            Value::Bool { value: *value }
        }
        2 => Value::Nat { value: words() },
        3 => Value::Int { value: words() },
        4 => Value::U8 { value: words() },
        5 => Value::U16 { value: words() },
        6 => Value::U32 { value: words() },
        7 => Value::U64 { value: words() },
        8 => Value::I8 { value: words() },
        9 => Value::I16 { value: words() },
        10 => Value::I32 { value: words() },
        11 => Value::I64 { value: words() },
        12 => Value::String { value: words() },
        13 => Value::Bytes { hex: words() },
        14 => Value::Ordering {
            value: match fields(&parts[0]).0 {
                0 => OrderingValue::Lt,
                1 => OrderingValue::Eq,
                _ => OrderingValue::Gt,
            },
        },
        15 => Value::None,
        16 => Value::Some { value: boxed(0) },
        17 => Value::Ok { value: boxed(0) },
        18 => Value::Error { value: boxed(0) },
        19 => Value::List {
            items: dec_values(&parts[0]),
        },
        20 => Value::Pair {
            left: boxed(0),
            right: boxed(1),
        },
        21 => Value::Adt {
            constructor: dec_nat(&parts[0]),
            fields: dec_values(&parts[1]),
        },
        22 => Value::Closure {
            function: dec_nat(&parts[0]),
            captures: dec_values(&parts[1]),
        },
        other => panic!("not a value: {other}"),
    }
}

fn dec_shape(value: &Value) -> Shape {
    let (constructor, parts) = fields(value);
    [
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
    ]
    .get(usize::try_from(constructor).expect("index"))
    .copied()
    .unwrap_or_else(|| Shape::Adt {
        constructor: dec_nat(&parts[0]),
    })
}

fn dec_prim(value: &Value) -> Prim {
    let (constructor, parts) = fields(value);
    match constructor {
        40 => Prim::Convert {
            target: dec_kind(&parts[0]),
        },
        41 => Prim::ParseDecimal {
            target: dec_ty(&parts[0]),
        },
        index => all_prims()
            .into_iter()
            .find(|candidate| fields(&enc_prim(candidate)).0 == index)
            .expect("a primitive"),
    }
}

fn dec_exprs(value: &Value) -> Vec<Expr> {
    unchain(value, 13, 14).into_iter().map(dec_expr).collect()
}

fn dec_expr(value: &Value) -> Expr {
    let (constructor, parts) = fields(value);
    let boxed = |at: usize| Box::new(dec_expr(&parts[at]));
    match constructor {
        0 => Expr::Value {
            ty: dec_ty(&parts[0]),
            value: dec_value(&parts[1]),
        },
        1 => Expr::Var {
            name: dec_nat(&parts[0]),
        },
        2 => Expr::Let {
            name: dec_nat(&parts[0]),
            ty: dec_ty(&parts[1]),
            bound: boxed(2),
            body: boxed(3),
        },
        3 => Expr::Cond {
            condition: boxed(0),
            then_branch: boxed(1),
            else_branch: boxed(2),
        },
        4 => Expr::Match {
            ty: dec_ty(&parts[0]),
            scrutinee: boxed(1),
            arms: unchain(&parts[2], 13, 14)
                .into_iter()
                .map(|arm| {
                    let (_, arm_parts) = fields(arm);
                    let Value::List { items } = &arm_parts[1] else {
                        panic!("binders are a list")
                    };
                    Arm {
                        shape: dec_shape(&arm_parts[0]),
                        binders: items.iter().map(dec_nat).collect(),
                        body: dec_expr(&arm_parts[2]),
                    }
                })
                .collect(),
        },
        5 => Expr::Build {
            shape: dec_shape(&parts[0]),
            ty: dec_ty(&parts[1]),
            operands: dec_exprs(&parts[2]),
        },
        6 => Expr::Call {
            function: dec_nat(&parts[0]),
            operands: dec_exprs(&parts[1]),
        },
        7 => Expr::Closure {
            function: dec_nat(&parts[0]),
            captures: dec_exprs(&parts[1]),
        },
        8 => Expr::Apply {
            target: boxed(0),
            operands: dec_exprs(&parts[1]),
        },
        9 => Expr::Prim {
            operation: dec_prim(&parts[0]),
            operands: dec_exprs(&parts[1]),
        },
        10 => Expr::First { value: boxed(0) },
        11 => Expr::Second { value: boxed(0) },
        12 => Expr::Field {
            value: boxed(0),
            index: dec_nat(&parts[1]),
        },
        other => panic!("not an expression: {other}"),
    }
}

/// The program the arguments of `program_arguments` stand for.
fn dec_program(arguments: &[Value]) -> Program {
    let chains = |value: &Value| -> Vec<Vec<Ty>> {
        unchain(value, 14, 15).into_iter().map(dec_tys).collect()
    };
    let constructors: Vec<Vec<Vec<Ty>>> = unchain(&arguments[1], 14, 15)
        .into_iter()
        .map(chains)
        .collect();
    let Value::List { items: names } = &arguments[2] else {
        panic!("parameters are a list")
    };
    let types = chains(&arguments[3]);
    let results = dec_tys(&arguments[4]);
    let bodies = dec_exprs(&arguments[5]);
    Program {
        spec: dec_text(&arguments[0]),
        adts: constructors
            .into_iter()
            .map(|constructors| target::Adt { constructors })
            .collect(),
        functions: bodies
            .into_iter()
            .enumerate()
            .map(|(at, body)| {
                let Value::List { items } = &names[at] else {
                    panic!("names are a list")
                };
                Function {
                    parameters: items.iter().map(dec_nat).collect(),
                    types: types[at].clone(),
                    result: results[at].clone(),
                    body,
                }
            })
            .collect(),
    }
}

/// What the oracle makes of a program, as the root returns it: `ok ()` or
/// the reason's text.
fn verdict(outcome: Result<(), String>) -> Json {
    serde_json::to_value(match outcome {
        Ok(()) => Value::Ok {
            value: Box::new(Value::Unit),
        },
        Err(reason) => Value::Error {
            value: Box::new(text(&reason)),
        },
    })
    .expect("a value is JSON")
}

/// A run of `checkProgram`.
fn program_run(program: &Program) -> Run {
    (program_arguments(program), verdict(check::check(program)))
}

/// A run of `checkArguments`.
fn arguments_run(program: &Program, entry: u64, arguments: &[Value]) -> Run {
    let mut all = program_arguments(program);
    all.push(nat(entry));
    all.push(enc_values(arguments));
    (
        all,
        verdict(check::check_arguments(program, entry, arguments)),
    )
}

// ---------------------------------------------------------------- vectors

/// A program from the repository with the entry and arguments it states, if
/// any.
struct Source {
    program: Program,
    entry: Option<(u64, Vec<Value>)>,
}

fn walk(dir: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            if !matches!(
                name.as_str(),
                "target" | ".git" | ".lexlean" | "node_modules"
            ) {
                walk(&path, found);
            }
        } else {
            found.push(path);
        }
    }
}

/// Every committed target program, read by the oracle's reader and deduplicated
/// by content: the fixtures of `compiler/fixtures`, the program of every
/// committed Rust package manifest, and every lowered program a verification
/// record preserves.
fn committed() -> Vec<Source> {
    let root = support::repo_root();
    let root = root.as_std_path();
    let mut files = Vec::new();
    walk(root, &mut files);
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    let mut admit = |program: Program, entry: Option<(u64, Vec<Value>)>| {
        let key = serde_json::to_string(&program).expect("a program is JSON");
        if seen.insert(key) {
            out.push(Source { program, entry });
        }
    };
    for path in files {
        let relative = path
            .strip_prefix(root)
            .expect("under the root")
            .to_string_lossy()
            .replace('\\', "/");
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        if !name.ends_with(".json") {
            continue;
        }
        if relative.starts_with("compiler/fixtures/") {
            let bytes = std::fs::read(&path).expect("a fixture");
            let fixture: Fixture = serde_json::from_slice(&bytes)
                .unwrap_or_else(|error| panic!("{relative}: {error}"));
            admit(fixture.program, Some((fixture.entry, fixture.arguments)));
        } else if name == "package.json"
            && (relative.starts_with("compiler/rust/")
                || relative.starts_with("tests/production-export/"))
        {
            let json: Json =
                serde_json::from_slice(&std::fs::read(&path).expect("a package manifest"))
                    .expect("a manifest is JSON");
            let program: Program = serde_json::from_value(json["program"].clone())
                .unwrap_or_else(|error| panic!("{relative}: {error}"));
            admit(program, None);
        } else if relative.contains("/preserve/program/R") {
            let mut json: Json =
                serde_json::from_slice(&std::fs::read(&path).expect("a preserved program"))
                    .expect("a program is JSON");
            json["spec"] = json!(PROGRAM_SPEC);
            let program: Program =
                serde_json::from_value(json).unwrap_or_else(|error| panic!("{relative}: {error}"));
            admit(program, None);
        }
    }
    out
}

/// The nodes of an expression, counting itself.
fn size(expr: &Expr) -> usize {
    1 + match expr {
        Expr::Value { .. } | Expr::Var { .. } => 0,
        Expr::Let { bound, body, .. } => size(bound) + size(body),
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => size(condition) + size(then_branch) + size(else_branch),
        Expr::Match {
            scrutinee, arms, ..
        } => size(scrutinee) + arms.iter().map(|arm| size(&arm.body)).sum::<usize>(),
        Expr::Build { operands, .. }
        | Expr::Call { operands, .. }
        | Expr::Prim { operands, .. } => operands.iter().map(size).sum(),
        Expr::Closure { captures, .. } => captures.iter().map(size).sum(),
        Expr::Apply { target, operands } => size(target) + operands.iter().map(size).sum::<usize>(),
        Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => size(value),
    }
}

fn program_size(program: &Program) -> usize {
    program
        .functions
        .iter()
        .map(|function| size(&function.body))
        .sum()
}

// ---------------------------------------------------------------- corpora

fn program_of(adts: Vec<Vec<Vec<Ty>>>, functions: Vec<Function>) -> Program {
    Program {
        spec: PROGRAM_SPEC.to_owned(),
        adts: adts
            .into_iter()
            .map(|constructors| target::Adt { constructors })
            .collect(),
        functions,
    }
}

fn unit_expr() -> Expr {
    Expr::Value {
        ty: Ty::Unit,
        value: Value::Unit,
    }
}

fn constant(ty: Ty, body: Expr) -> Function {
    Function {
        parameters: Vec::new(),
        types: Vec::new(),
        result: ty,
        body,
    }
}

/// The two declarations the type and shape corpora refer to: a type with two
/// constructors, and a type with one.
fn declared() -> Vec<Vec<Vec<Ty>>> {
    vec![
        vec![vec![Ty::Nat, Ty::Bool], Vec::new()],
        vec![vec![Ty::Nat]],
    ]
}

fn kinds() -> [IntKind; 8] {
    IntKind::ALL
}

/// Decimal spellings at the edges of every realized integer and of the
/// grammar of a canonical decimal.
fn spellings() -> Vec<String> {
    let mut out: Vec<String> = [
        "0",
        "1",
        "-1",
        "-0",
        "00",
        "007",
        "+5",
        "",
        "-",
        "--1",
        "1.5",
        " 1",
        "1 ",
        "\u{663}",
        "1_000",
        "255",
        "256",
        "-128",
        "-129",
        "127",
        "128",
        "65535",
        "65536",
        "-32768",
        "-32769",
        "32767",
        "32768",
        "4294967295",
        "4294967296",
        "-2147483648",
        "-2147483649",
        "2147483647",
        "2147483648",
        "18446744073709551615",
        "18446744073709551616",
        "9223372036854775807",
        "9223372036854775808",
        "-9223372036854775808",
        "-9223372036854775809",
        "170141183460469231731687303715884105727",
        "170141183460469231731687303715884105728",
        "-170141183460469231731687303715884105728",
        "-170141183460469231731687303715884105729",
        "99999999999999999999999999999999999999999",
        "0123456789",
    ]
    .iter()
    .map(|spelling| (*spelling).to_owned())
    .collect();
    out.sort();
    out.dedup();
    out
}

/// Strings whose Debug spelling the module reproduces: ASCII with every
/// escape, the C1 controls, and printable scalars copied literally.
fn strings() -> Vec<String> {
    [
        "",
        "plain",
        "a\"b",
        "back\\slash",
        "line\nfeed",
        "carriage\rreturn",
        "tab\tstop",
        "nul\u{0}",
        "bell\u{7}",
        "unit\u{1f}",
        "del\u{7f}",
        "next\u{85}line",
        "c1\u{80}\u{9f}",
        "caf\u{e9}",
        "\u{65e5}\u{672c}",
        "\u{1f642}",
        "single'quote",
        "\u{a0}nbsp",
        "mixed \"q\" \\ \n \u{1}",
    ]
    .iter()
    .map(|string| (*string).to_owned())
    .collect()
}

fn hexes() -> Vec<String> {
    [
        "", "00", "0a", "ff", "abc", "ABCD", "zz", "0g", "\u{e9}e", "00 ", "a", "deadbeef",
    ]
    .iter()
    .map(|hex| (*hex).to_owned())
    .collect()
}

/// A literal of every constructor, with strings drawn from the corpora.
fn sample_values(seeded: &mut Seeded) -> Vec<Value> {
    let spellings = spellings();
    let strings = strings();
    let hexes = hexes();
    let digits = |seeded: &mut Seeded| seeded.pick(&spellings).clone();
    let mut out = vec![
        Value::Unit,
        Value::Bool { value: true },
        Value::Bool { value: false },
        Value::Ordering {
            value: OrderingValue::Lt,
        },
        Value::Ordering {
            value: OrderingValue::Gt,
        },
        Value::None,
    ];
    for _ in 0..3 {
        out.extend([
            Value::Nat {
                value: digits(seeded),
            },
            Value::Int {
                value: digits(seeded),
            },
            Value::U8 {
                value: digits(seeded),
            },
            Value::U16 {
                value: digits(seeded),
            },
            Value::U32 {
                value: digits(seeded),
            },
            Value::U64 {
                value: digits(seeded),
            },
            Value::I8 {
                value: digits(seeded),
            },
            Value::I16 {
                value: digits(seeded),
            },
            Value::I32 {
                value: digits(seeded),
            },
            Value::I64 {
                value: digits(seeded),
            },
            Value::String {
                value: seeded.pick(&strings).clone(),
            },
            Value::Bytes {
                hex: seeded.pick(&hexes).clone(),
            },
        ]);
    }
    out.extend([
        Value::Some {
            value: Box::new(Value::Nat { value: "1".into() }),
        },
        Value::Some {
            value: Box::new(Value::Unit),
        },
        Value::Ok {
            value: Box::new(Value::Nat { value: "1".into() }),
        },
        Value::Error {
            value: Box::new(Value::Bool { value: true }),
        },
        Value::List { items: Vec::new() },
        Value::List {
            items: vec![
                Value::Nat { value: "1".into() },
                Value::Nat { value: "2".into() },
            ],
        },
        Value::List {
            items: vec![Value::Nat { value: "1".into() }, Value::Unit],
        },
        Value::Pair {
            left: Box::new(Value::Nat { value: "1".into() }),
            right: Box::new(Value::Bool { value: false }),
        },
        Value::Pair {
            left: Box::new(Value::Unit),
            right: Box::new(Value::Unit),
        },
        Value::Adt {
            constructor: 0,
            fields: vec![
                Value::Nat { value: "1".into() },
                Value::Bool { value: true },
            ],
        },
        Value::Adt {
            constructor: 1,
            fields: Vec::new(),
        },
        Value::Adt {
            constructor: 0,
            fields: vec![Value::Unit],
        },
        Value::Adt {
            constructor: 9,
            fields: Vec::new(),
        },
        Value::Closure {
            function: 0,
            captures: Vec::new(),
        },
    ]);
    out
}

fn sample_tys() -> Vec<Ty> {
    let mut out = vec![
        Ty::Unit,
        Ty::Bool,
        Ty::Nat,
        Ty::Int,
        Ty::String,
        Ty::Bytes,
        Ty::Ordering,
    ];
    out.extend(kinds().iter().map(|width| Ty::Fixed { width: *width }));
    let nat = || Box::new(Ty::Nat);
    out.extend([
        Ty::Option { value: nat() },
        Ty::Option {
            value: Box::new(Ty::Unit),
        },
        Ty::Result {
            ok: nat(),
            error: Box::new(Ty::Bool),
        },
        Ty::List { element: nat() },
        Ty::List {
            element: Box::new(Ty::Unit),
        },
        Ty::Pair {
            left: nat(),
            right: Box::new(Ty::Bool),
        },
        Ty::Pair {
            left: Box::new(Ty::Unit),
            right: Box::new(Ty::Unit),
        },
        Ty::Adt { index: 0 },
        Ty::Adt { index: 1 },
        Ty::Adt { index: 9 },
        Ty::Fn {
            parameters: vec![Ty::Nat],
            result: nat(),
        },
        Ty::Fn {
            parameters: Vec::new(),
            result: nat(),
        },
    ]);
    out
}

/// A program of one constant: a literal against a type.
fn literal_program(ty: &Ty, value: &Value) -> Program {
    program_of(
        declared(),
        vec![constant(
            ty.clone(),
            Expr::Value {
                ty: ty.clone(),
                value: value.clone(),
            },
        )],
    )
}

/// The literal programs: every sample value against every sample type with a
/// stride, so each constructor meets each type, and every decimal spelling
/// against each integer type.
fn literal_corpus(seeded: &mut Seeded, count: usize) -> Vec<Program> {
    let values = sample_values(seeded);
    let tys = sample_tys();
    let mut out = Vec::new();
    for spelling in spellings() {
        for kind in kinds() {
            let (fixed, value) = (
                Ty::Fixed { width: kind },
                match kind {
                    IntKind::U8 => Value::U8 {
                        value: spelling.clone(),
                    },
                    IntKind::U16 => Value::U16 {
                        value: spelling.clone(),
                    },
                    IntKind::U32 => Value::U32 {
                        value: spelling.clone(),
                    },
                    IntKind::U64 => Value::U64 {
                        value: spelling.clone(),
                    },
                    IntKind::I8 => Value::I8 {
                        value: spelling.clone(),
                    },
                    IntKind::I16 => Value::I16 {
                        value: spelling.clone(),
                    },
                    IntKind::I32 => Value::I32 {
                        value: spelling.clone(),
                    },
                    IntKind::I64 => Value::I64 {
                        value: spelling.clone(),
                    },
                },
            );
            out.push(literal_program(&fixed, &value));
        }
        out.push(literal_program(
            &Ty::Nat,
            &Value::Nat {
                value: spelling.clone(),
            },
        ));
        out.push(literal_program(
            &Ty::Int,
            &Value::Int {
                value: spelling.clone(),
            },
        ));
    }
    for hex in hexes() {
        out.push(literal_program(&Ty::Bytes, &Value::Bytes { hex }));
    }
    for string in strings() {
        out.push(literal_program(
            &Ty::String,
            &Value::String {
                value: string.clone(),
            },
        ));
        out.push(literal_program(&Ty::Nat, &Value::String { value: string }));
    }
    for value in &values {
        for ty in &tys {
            out.push(literal_program(ty, value));
        }
    }
    // A stride through the cross product keeps the order stable and the
    // count bounded.
    if out.len() > count {
        let step = out.len() / count + 1;
        out = out.into_iter().step_by(step).collect();
    }
    out
}

fn all_prims() -> Vec<Prim> {
    let mut out = vec![
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
    ];
    out.extend(
        kinds()
            .iter()
            .map(|target| Prim::Convert { target: *target }),
    );
    out.extend(
        [Ty::Int, Ty::Nat, Ty::Bool, Ty::String]
            .into_iter()
            .map(|target| Prim::ParseDecimal { target }),
    );
    out.extend(kinds().iter().map(|width| Prim::ParseDecimal {
        target: Ty::Fixed { width: *width },
    }));
    out
}

fn prim_pool() -> Vec<Ty> {
    let nat = || Box::new(Ty::Nat);
    let mut out = vec![
        Ty::Unit,
        Ty::Bool,
        Ty::Nat,
        Ty::Int,
        Ty::String,
        Ty::Bytes,
        Ty::Ordering,
        Ty::Fixed { width: IntKind::U8 },
        Ty::Fixed {
            width: IntKind::U32,
        },
        Ty::Fixed { width: IntKind::I8 },
        Ty::Fixed {
            width: IntKind::I64,
        },
        Ty::List { element: nat() },
        Ty::List {
            element: Box::new(Ty::String),
        },
        Ty::Option { value: nat() },
        Ty::Pair {
            left: nat(),
            right: nat(),
        },
        Ty::Pair {
            left: nat(),
            right: Box::new(Ty::String),
        },
        Ty::Fn {
            parameters: vec![Ty::Nat],
            result: nat(),
        },
    ];
    out.push(Ty::Pair {
        left: Box::new(Ty::Pair {
            left: nat(),
            right: Box::new(Ty::Bool),
        }),
        right: Box::new(Ty::Fixed { width: IntKind::U8 }),
    });
    out
}

/// A function applying `operation` to parameters of `types`, declared to
/// return what the oracle's rule gives it.
fn prim_program(operation: &Prim, types: &[Ty]) -> Program {
    let result = check::primitive_type(operation, types).unwrap_or(Ty::Unit);
    program_of(
        Vec::new(),
        vec![Function {
            parameters: (0..types.len() as u64).collect(),
            types: types.to_vec(),
            result,
            body: Expr::Prim {
                operation: operation.clone(),
                operands: (0..types.len() as u64)
                    .map(|name| Expr::Var { name })
                    .collect(),
            },
        }],
    )
}

/// The signature of each primitive, so some of the corpus is well typed.
fn signature(operation: &Prim, pool: &[Ty], seeded: &mut Seeded) -> Vec<Ty> {
    let fixed = || Ty::Fixed { width: IntKind::U8 };
    let nat = || Ty::Nat;
    let int = || Ty::Int;
    let string = || Ty::String;
    let bytes = || Ty::Bytes;
    let list_of = |ty: Ty| Ty::List {
        element: Box::new(ty),
    };
    match operation {
        Prim::NatAdd | Prim::NatSub | Prim::NatMul | Prim::NatEq | Prim::NatLe | Prim::NatLt => {
            vec![nat(), nat()]
        }
        Prim::NatQuot | Prim::NatRem => vec![nat(), nat(), nat()],
        Prim::IntAdd | Prim::IntSub | Prim::IntMul => vec![int(), int()],
        Prim::IntNeg => vec![int()],
        Prim::IntQuot | Prim::IntRem => vec![int(), int(), int()],
        Prim::CheckedAdd
        | Prim::CheckedSub
        | Prim::CheckedMul
        | Prim::CheckedQuot
        | Prim::BitAnd
        | Prim::BitOr
        | Prim::BitXor => vec![fixed(), fixed()],
        Prim::CheckedNeg | Prim::BitNot => vec![fixed()],
        Prim::ShiftLeft | Prim::ShiftRight => vec![
            fixed(),
            Ty::Fixed {
                width: IntKind::U32,
            },
        ],
        Prim::Equal | Prim::Compare | Prim::Append => {
            let ty = seeded.pick(pool).clone();
            vec![ty.clone(), ty]
        }
        Prim::BoolNot => vec![Ty::Bool],
        Prim::BoolAnd | Prim::BoolOr => vec![Ty::Bool, Ty::Bool],
        Prim::Length => vec![list_of(nat())],
        Prim::Index => vec![bytes(), nat()],
        Prim::Slice => vec![bytes(), nat(), nat()],
        Prim::Utf8Encode => vec![string()],
        Prim::Utf8Decode => vec![bytes()],
        Prim::CompareBytes => vec![bytes(), bytes()],
        Prim::SplitExact => vec![
            string(),
            string(),
            Ty::Fixed {
                width: IntKind::U32,
            },
        ],
        Prim::Join => vec![list_of(string()), string()],
        Prim::FormatDecimal | Prim::Convert { .. } => vec![seeded.pick(pool).clone()],
        Prim::ParseDecimal { .. } => vec![string()],
    }
}

/// Every primitive at its own signature, at shifted signatures, and at
/// operand types drawn from the pool.
fn prim_corpus(seeded: &mut Seeded, rounds: usize) -> Vec<Program> {
    let pool = prim_pool();
    let mut out = Vec::new();
    for operation in all_prims() {
        for round in 0..rounds {
            let mut types = signature(&operation, &pool, seeded);
            match round % 4 {
                0 => {}
                1 => {
                    let at = seeded.below(types.len().max(1));
                    if let Some(slot) = types.get_mut(at) {
                        *slot = seeded.pick(&pool).clone();
                    }
                }
                2 => {
                    types.push(seeded.pick(&pool).clone());
                }
                _ => {
                    types.pop();
                }
            }
            out.push(prim_program(&operation, &types));
        }
    }
    out
}

fn all_shapes() -> Vec<Shape> {
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
        Shape::Adt { constructor: 1 },
        Shape::Adt { constructor: 7 },
    ]
}

/// Each shape built and matched at each type: its fields, its operand count,
/// its binders, and the coverage of a match with that one arm.
fn shape_corpus(seeded: &mut Seeded) -> Vec<Program> {
    let tys = sample_tys();
    let mut out = Vec::new();
    for shape in all_shapes() {
        for ty in &tys {
            out.push(program_of(
                declared(),
                vec![constant(
                    ty.clone(),
                    Expr::Build {
                        shape,
                        ty: ty.clone(),
                        operands: Vec::new(),
                    },
                )],
            ));
            let binders: Vec<u64> = (0..seeded.below(3) as u64).collect();
            out.push(program_of(
                declared(),
                vec![Function {
                    parameters: vec![0],
                    types: vec![ty.clone()],
                    result: Ty::Unit,
                    body: Expr::Match {
                        ty: Ty::Unit,
                        scrutinee: Box::new(Expr::Var { name: 0 }),
                        arms: vec![Arm {
                            shape,
                            binders,
                            body: unit_expr(),
                        }],
                    },
                }],
            ));
        }
    }
    out
}

/// The `index`th node of `expr` in preorder, counting itself.
fn nth<'a>(expr: &'a mut Expr, index: &mut usize) -> Option<&'a mut Expr> {
    if *index == 0 {
        return Some(expr);
    }
    *index -= 1;
    match expr {
        Expr::Value { .. } | Expr::Var { .. } => None,
        Expr::Let { bound, body, .. } => {
            let [bound, body] = [bound, body];
            nth(bound, index).or_else(|| nth(body, index))
        }
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => {
            if *index < size(condition) {
                return nth(condition, index);
            }
            *index -= size(condition);
            if *index < size(then_branch) {
                return nth(then_branch, index);
            }
            *index -= size(then_branch);
            nth(else_branch, index)
        }
        Expr::Match {
            scrutinee, arms, ..
        } => {
            if *index < size(scrutinee) {
                return nth(scrutinee, index);
            }
            *index -= size(scrutinee);
            for arm in arms {
                if *index < size(&arm.body) {
                    return nth(&mut arm.body, index);
                }
                *index -= size(&arm.body);
            }
            None
        }
        Expr::Build { operands, .. }
        | Expr::Call { operands, .. }
        | Expr::Prim { operands, .. } => {
            for operand in operands {
                if *index < size(operand) {
                    return nth(operand, index);
                }
                *index -= size(operand);
            }
            None
        }
        Expr::Closure { captures, .. } => {
            for operand in captures {
                if *index < size(operand) {
                    return nth(operand, index);
                }
                *index -= size(operand);
            }
            None
        }
        Expr::Apply { target, operands } => {
            if *index < size(target) {
                return nth(target, index);
            }
            *index -= size(target);
            for operand in operands {
                if *index < size(operand) {
                    return nth(operand, index);
                }
                *index -= size(operand);
            }
            None
        }
        Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
            nth(value, index)
        }
    }
}

fn random_ty(seeded: &mut Seeded, pool: &[Ty]) -> Ty {
    seeded.pick(pool).clone()
}

fn random_literal(seeded: &mut Seeded, values: &[Value], pool: &[Ty]) -> Expr {
    Expr::Value {
        ty: random_ty(seeded, pool),
        value: seeded.pick(values).clone(),
    }
}

/// A spelling changed at one place, as a mutated literal.
fn tweak(spelling: &str, seeded: &mut Seeded) -> String {
    match seeded.below(6) {
        0 => format!("0{spelling}"),
        1 => format!("-{spelling}"),
        2 => format!("{spelling}0"),
        3 => spelling
            .chars()
            .rev()
            .skip(1)
            .collect::<String>()
            .chars()
            .rev()
            .collect(),
        4 => format!("{spelling}x"),
        _ => spellings()[seeded.below(spellings().len())].clone(),
    }
}

/// One mutation of a program at a seeded place: each of them breaks, or
/// keeps, one of the rules the checker enforces.
#[allow(clippy::too_many_lines)]
fn mutate(program: &Program, seeded: &mut Seeded, values: &[Value], pool: &[Ty]) -> Program {
    let mut out = program.clone();
    let function_count = out.functions.len();
    if function_count == 0 {
        out.spec = "lexlean/target-program/0".to_owned();
        return out;
    }
    let function = seeded.below(function_count);
    let nodes = size(&out.functions[function].body);
    match seeded.below(24) {
        0 => out.spec = format!("{}{}", PROGRAM_SPEC, seeded.below(10)),
        1 => out.adts.push(target::Adt {
            constructors: Vec::new(),
        }),
        2 => {
            let ty = random_ty(seeded, pool);
            if out.adts.is_empty() {
                out.adts.push(target::Adt {
                    constructors: vec![vec![ty]],
                });
            } else {
                let at = seeded.below(out.adts.len());
                out.adts[at].constructors.push(vec![ty]);
            }
        }
        3 => {
            let at = seeded.below(out.functions[function].types.len().max(1));
            let ty = random_ty(seeded, pool);
            if let Some(slot) = out.functions[function].types.get_mut(at) {
                *slot = ty;
            }
        }
        4 => out.functions[function].result = random_ty(seeded, pool),
        5 => {
            let parameters = &mut out.functions[function].parameters;
            if parameters.len() > 1 && seeded.chance(50) {
                parameters[0] = parameters[1];
            } else {
                parameters.pop();
            }
        }
        6 => out.functions[function].types.push(Ty::Nat),
        7 => {
            let mut at = seeded.below(nodes);
            if let Some(node) = nth(&mut out.functions[function].body, &mut at) {
                *node = random_literal(seeded, values, pool);
            }
        }
        8 => {
            let mut at = seeded.below(nodes);
            let name = [0, 1, 2, 3, 4, 9999][seeded.below(6)];
            if let Some(node) = nth(&mut out.functions[function].body, &mut at) {
                *node = Expr::Var { name };
            }
        }
        9 => {
            // The place of a literal: its spelling changes.
            for _ in 0..8 {
                let mut at = seeded.below(nodes);
                if let Some(Expr::Value { value, .. }) =
                    nth(&mut out.functions[function].body, &mut at)
                {
                    match value {
                        Value::Nat { value }
                        | Value::Int { value }
                        | Value::U8 { value }
                        | Value::U16 { value }
                        | Value::U32 { value }
                        | Value::U64 { value }
                        | Value::I8 { value }
                        | Value::I16 { value }
                        | Value::I32 { value }
                        | Value::I64 { value } => *value = tweak(value, seeded),
                        Value::Bytes { hex } => *hex = tweak(hex, seeded),
                        Value::String { value } => value.push_str("\"\\"),
                        _ => continue,
                    }
                    break;
                }
            }
        }
        10..=13 => {
            let mut at = seeded.below(nodes);
            let ty = random_ty(seeded, pool);
            if let Some(node) = nth(&mut out.functions[function].body, &mut at) {
                match node {
                    Expr::Value { ty: slot, .. }
                    | Expr::Let { ty: slot, .. }
                    | Expr::Match { ty: slot, .. }
                    | Expr::Build { ty: slot, .. } => *slot = ty,
                    Expr::Call { function, .. } | Expr::Closure { function, .. } => {
                        *function = seeded.below(function_count + 2) as u64;
                    }
                    Expr::Prim { operation, .. } => {
                        let prims = all_prims();
                        *operation = seeded.pick(&prims).clone();
                    }
                    Expr::Field { index, .. } => *index = seeded.below(4) as u64,
                    Expr::Cond {
                        then_branch,
                        else_branch,
                        ..
                    } => std::mem::swap(then_branch, else_branch),
                    Expr::Var { name } => *name = seeded.below(8) as u64,
                    _ => {}
                }
            }
        }
        14..=17 => {
            let mut at = seeded.below(nodes);
            let literal = random_literal(seeded, values, pool);
            if let Some(node) = nth(&mut out.functions[function].body, &mut at) {
                match node {
                    Expr::Build { operands, .. }
                    | Expr::Call { operands, .. }
                    | Expr::Prim { operands, .. } => match seeded.below(3) {
                        0 => {
                            operands.pop();
                        }
                        1 => operands.push(literal),
                        _ => {
                            if let Some(slot) = operands.first_mut() {
                                *slot = literal;
                            }
                        }
                    },
                    Expr::Closure { captures, .. } => match seeded.below(2) {
                        0 => captures.push(literal),
                        _ => {
                            captures.pop();
                        }
                    },
                    Expr::Apply { target, operands } => match seeded.below(3) {
                        0 => {
                            operands.pop();
                        }
                        1 => operands.push(literal),
                        _ => **target = literal,
                    },
                    Expr::Match { arms, .. } => match seeded.below(5) {
                        0 => {
                            arms.pop();
                        }
                        1 => {
                            if let Some(arm) = arms.first().cloned() {
                                arms.push(arm);
                            }
                        }
                        2 => {
                            if let Some(arm) = arms.first_mut() {
                                arm.binders.push(7);
                            }
                        }
                        3 => {
                            if let Some(arm) = arms.last_mut() {
                                arm.binders.pop();
                            }
                        }
                        _ => {
                            let shapes = all_shapes();
                            let shape = *seeded.pick(&shapes);
                            if let Some(arm) = arms.first_mut() {
                                arm.shape = shape;
                            }
                        }
                    },
                    Expr::Let { name, .. } => *name = seeded.below(4) as u64,
                    _ => {}
                }
            }
        }
        18 => {
            let mut at = seeded.below(nodes);
            let index = seeded.below(3) as u64;
            if let Some(node) = nth(&mut out.functions[function].body, &mut at) {
                let inner = std::mem::replace(node, unit_expr());
                *node = match seeded.below(3) {
                    0 => Expr::First {
                        value: Box::new(inner),
                    },
                    1 => Expr::Second {
                        value: Box::new(inner),
                    },
                    _ => Expr::Field {
                        value: Box::new(inner),
                        index,
                    },
                };
            }
        }
        19 => {
            let mut at = seeded.below(nodes);
            if let Some(node) = nth(&mut out.functions[function].body, &mut at) {
                let inner = std::mem::replace(node, unit_expr());
                *node = Expr::Apply {
                    target: Box::new(inner),
                    operands: vec![unit_expr()],
                };
            }
        }
        20 => {
            let mut at = seeded.below(nodes);
            if let Some(node) = nth(&mut out.functions[function].body, &mut at) {
                let inner = std::mem::replace(node, unit_expr());
                *node = Expr::Let {
                    name: seeded.below(3) as u64,
                    ty: Ty::Nat,
                    bound: Box::new(inner),
                    body: Box::new(unit_expr()),
                };
            }
        }
        21 => {
            let mut at = seeded.below(nodes);
            if let Some(node) = nth(&mut out.functions[function].body, &mut at) {
                let inner = std::mem::replace(node, unit_expr());
                *node = Expr::Cond {
                    condition: Box::new(inner),
                    then_branch: Box::new(unit_expr()),
                    else_branch: Box::new(unit_expr()),
                };
            }
        }
        22 => {
            out.functions.remove(function);
        }
        _ => {
            let mut at = seeded.below(nodes);
            if let Some(node) = nth(&mut out.functions[function].body, &mut at) {
                let inner = std::mem::replace(node, unit_expr());
                *node = Expr::Closure {
                    function: seeded.below(function_count) as u64,
                    captures: vec![inner],
                };
            }
        }
    }
    out
}

/// The refusals of `TC-02`, written as the edits that test makes to the
/// committed fixture `adt-evaluation` and to constant programs, each with the
/// text the oracle's reason must contain. A program that no longer reads is
/// not a vector of a checker.
fn refusals() -> Vec<(Program, &'static str)> {
    let root = support::repo_root();
    let base: Json = serde_json::from_slice(
        &std::fs::read(
            root.join("compiler/fixtures/adt-evaluation.json")
                .as_std_path(),
        )
        .expect("the committed fixture"),
    )
    .expect("a fixture is JSON");
    let base = base["program"].clone();
    let closures: Json = serde_json::from_slice(
        &std::fs::read(root.join("compiler/fixtures/closures.json").as_std_path())
            .expect("the committed fixture"),
    )
    .expect("a fixture is JSON");
    let closures = closures["program"].clone();
    let mut out: Vec<(Json, &'static str)> = Vec::new();
    let mut edit = |source: &Json, change: &dyn Fn(&mut Json), expected: &'static str| {
        let mut json = source.clone();
        change(&mut json);
        out.push((json, expected));
    };
    edit(
        &base,
        &|json| json["spec"] = json!("lexlean/target-program/9"),
        "has spec `lexlean/target-program/9`",
    );
    edit(
        &base,
        &|json| json["functions"][0]["body"] = json!({"kind": "var", "name": 99}),
        "function 0: local 99 is unbound",
    );
    edit(
        &base,
        &|json| {
            json["functions"][1]["body"] = json!({"kind": "value", "type": {"kind": "bool"}, "value": {"kind": "bool", "value": true}});
        },
        "function 1: the body has type Bool, expected Nat",
    );
    edit(
        &base,
        &|json| {
            json["functions"][0]["body"]["arms"]
                .as_array_mut()
                .expect("arms")
                .pop();
        },
        "the match is not exhaustive: Adt { constructor: 2 } is not covered",
    );
    edit(
        &base,
        &|json| {
            let arm = json["functions"][0]["body"]["arms"][0].clone();
            json["functions"][0]["body"]["arms"]
                .as_array_mut()
                .expect("arms")
                .push(arm);
        },
        "the arm for Adt { constructor: 0 } is unreachable",
    );
    edit(
        &base,
        &|json| {
            json["functions"][1]["body"]["operands"]
                .as_array_mut()
                .expect("operands")
                .push(json!({"kind": "var", "name": 0}));
        },
        "function 0 takes 1 operand(s), received 2",
    );
    edit(
        &base,
        &|json| json["functions"][1]["body"]["function"] = json!(9),
        "function 9 is not declared",
    );
    edit(
        &base,
        &|json| json["functions"][0]["types"][0] = json!({"kind": "adt", "index": 9}),
        "ADT 9 is not declared",
    );
    edit(
        &base,
        &|json| {
            json["adts"]
                .as_array_mut()
                .expect("adts")
                .push(json!({"constructors": []}));
        },
        "ADT 2 has no constructor",
    );
    edit(
        &base,
        &|json| {
            json["functions"][0]["parameters"] = json!([0, 0]);
            json["functions"][0]["types"]
                .as_array_mut()
                .expect("types")
                .push(json!({"kind": "nat"}));
        },
        "a parameter name repeats",
    );
    edit(
        &base,
        &|json| {
            json["functions"][0]["body"]["arms"][2]["body"]["operands"][0]["operands"][0] =
                json!({"kind": "field", "value": {"kind": "var", "name": 5}, "index": 0});
        },
        "`field` requires an ADT with exactly one constructor; ADT 0 has 3",
    );
    let constant = |ty: Json, value: Json| {
        json!({"spec": PROGRAM_SPEC, "adts": [], "functions": [
            {"parameters": [], "types": [], "result": ty, "body": {"kind": "value", "type": ty, "value": value}}
        ]})
    };
    for (ty, value, expected) in [
        (
            json!({"kind": "fixed", "width": "u8"}),
            json!({"kind": "u8", "value": "256"}),
            "256 is outside u8",
        ),
        (
            json!({"kind": "nat"}),
            json!({"kind": "nat", "value": "007"}),
            "`007` is not a canonical decimal",
        ),
        (
            json!({"kind": "int"}),
            json!({"kind": "int", "value": "-0"}),
            "`-0` is not a canonical decimal",
        ),
        (
            json!({"kind": "nat"}),
            json!({"kind": "nat", "value": "18446744073709551616"}),
            "outside the 64-bit realization",
        ),
        (
            json!({"kind": "bytes"}),
            json!({"kind": "bytes", "hex": "abc"}),
            "byte literal `abc` is not even-length lowercase hexadecimal",
        ),
        (
            json!({"kind": "nat"}),
            json!({"kind": "int", "value": "1"}),
            "does not have type Nat",
        ),
        (
            json!({"kind": "fn", "parameters": [], "result": {"kind": "nat"}}),
            json!({"kind": "unit"}),
            "a function type takes at least one parameter",
        ),
        (
            json!({"kind": "nat"}),
            json!({"kind": "closure", "function": 0, "captures": []}),
            "a closure has no literal form",
        ),
    ] {
        out.push((constant(ty, value), expected));
    }
    let mut capture_all = closures;
    capture_all["functions"][0]["body"]["operands"][0]["captures"]
        .as_array_mut()
        .expect("captures")
        .push(json!({"kind": "var", "name": 0}));
    out.push((
        capture_all,
        "closure of function 2 captures 2 of its 2 parameters; at least one must remain",
    ));
    let nats = json!({"kind": "list", "element": {"kind": "nat"}});
    for (operation, operand_ty, result, expected) in [
        (
            json!({"kind": "compare"}),
            nats,
            json!({"kind": "ordering"}),
            "primitive Compare does not apply to",
        ),
        (
            json!({"kind": "equal"}),
            json!({"kind": "int"}),
            json!({"kind": "bool"}),
            "primitive Equal does not apply to [Int, Int]",
        ),
        (
            json!({"kind": "nat_add"}),
            json!({"kind": "int"}),
            json!({"kind": "int"}),
            "primitive NatAdd takes [Nat, Nat], received [Int, Int]",
        ),
    ] {
        out.push((
            json!({"spec": PROGRAM_SPEC, "adts": [], "functions": [
                {"parameters": [0], "types": [operand_ty], "result": result,
                 "body": {"kind": "prim", "operation": operation, "operands": [{"kind": "var", "name": 0}, {"kind": "var", "name": 0}]}}
            ]}),
            expected,
        ));
    }
    out.into_iter()
        .map(|(json, expected)| {
            let program: Program = serde_json::from_value(json)
                .unwrap_or_else(|reason| panic!("{expected}: {reason}"));
            let reason = check::check(&program).expect_err(expected);
            assert!(
                reason.contains(expected),
                "the oracle's reason `{reason}` no longer contains `{expected}`"
            );
            (program, expected)
        })
        .collect()
}
/// Runs of `checkArguments`: the arguments each committed fixture states, as
/// stated and as mutated (dropped, added, replaced, at another entry).
fn argument_corpus(sources: &[Source], seeded: &mut Seeded, count: usize) -> Vec<Run> {
    let values = sample_values(seeded);
    let stated: Vec<&Source> = sources
        .iter()
        .filter(|source| source.entry.is_some())
        .collect();
    let mut out = Vec::new();
    for source in &stated {
        let (entry, arguments) = source.entry.clone().expect("an entry");
        out.push(arguments_run(&source.program, entry, &arguments));
    }
    for _ in 0..count {
        let source = seeded.pick(&stated);
        let (mut entry, mut arguments) = source.entry.clone().expect("an entry");
        match seeded.below(6) {
            0 => {
                arguments.pop();
            }
            1 => arguments.push(seeded.pick(&values).clone()),
            2 => {
                if !arguments.is_empty() {
                    let at = seeded.below(arguments.len());
                    arguments[at] = seeded.pick(&values).clone();
                }
            }
            3 => entry = seeded.below(source.program.functions.len() + 2) as u64,
            4 => {
                if !arguments.is_empty() {
                    let at = seeded.below(arguments.len());
                    arguments[at] = Value::Some {
                        value: Box::new(seeded.pick(&values).clone()),
                    };
                }
            }
            _ => {
                arguments.reverse();
            }
        }
        out.push(arguments_run(&source.program, entry, &arguments));
    }
    out
}

/// Whether the oracle's message spells a scalar the claim leaves out: Rust's
/// `Debug` for `str` escapes by Unicode tables, and the module reproduces the
/// escapes of ASCII and of U+0080..U+009F only. The message shows exactly
/// the scalars Rust escaped, as `\u{..}`; any beyond U+009F is outside.
fn outside_claim(verdict: &Json) -> bool {
    let Some(message) = verdict["value"]["hex"].as_str() else {
        return false;
    };
    let raw: Vec<u8> = (0..message.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&message[at..at + 2], 16).expect("hex digit"))
        .collect();
    let message = String::from_utf8_lossy(&raw);
    let mut characters = message.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            continue;
        }
        if characters.next() == Some('u') {
            let mut code = String::new();
            for next in characters.by_ref() {
                if next == '{' {
                    continue;
                }
                if next == '}' {
                    break;
                }
                code.push(next);
            }
            if u32::from_str_radix(&code, 16).is_ok_and(|code| code > 0x9f) {
                return true;
            }
        }
    }
    false
}

/// The rendered `rust-std` packages of the roots, compiled by the pinned
/// rustc, run on `runs` of each root: one outcome per run, each equal to the
/// oracle's. Returns the number of runs.
fn rustc_differential(roots: &[(&str, &Lowered, &[Run])], label: &str) -> usize {
    let started = std::time::Instant::now();
    let dir = tempfile::Builder::new()
        .prefix("lexlean-bootstrap-calculus-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    for (root, low, runs) in roots {
        let manifest = rust_packages::manifest(
            &format!("boot_{}", snake(root)),
            &low.program,
            low.entry(),
            Profile::Std,
        );
        let package = target::package(&manifest.to_file_bytes().expect("canonical manifest"))
            .unwrap_or_else(|failure| panic!("{root}: {failure:?}"));
        let arguments: Vec<Vec<Value>> = runs
            .iter()
            .map(|(arguments, _)| arguments.clone())
            .collect();
        let arity = low.program.functions[usize::try_from(low.entry()).expect("entry")]
            .types
            .len();
        let passing = vec![Passing::Own; arity];
        let harness_source = rust_harness::render_calls(
            &low.program,
            Profile::Std,
            &manifest.name,
            &[Calls {
                entry: low.entry(),
                function: "run",
                passing: &passing,
                arguments: &arguments,
            }],
        )
        .expect("the differential harness");
        expected.insert(
            manifest.name.clone(),
            runs.iter().map(|(_, expected)| expected.clone()).collect(),
        );
        crates.push(Built {
            name: manifest.name,
            files: package.files,
            harness: harness_source,
        });
    }
    workspace(dir.path(), &crates);
    let build = cargo_in(
        dir.path(),
        &["build", "--offline", "--workspace", "--quiet"],
    );
    assert!(
        build.status.success(),
        "a root's package does not build:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    eprintln!(
        "BS-04: {label}: rustc built the packages ({:?})",
        started.elapsed()
    );
    let mut total = 0;
    for (name, outcomes) in &expected {
        let binary = dir
            .path()
            .join("target/debug")
            .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
        let ran = std::process::Command::new(&binary)
            .output()
            .expect("the differential runs");
        assert!(
            ran.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        let stdout = String::from_utf8(ran.stdout).expect("utf8");
        let printed: Vec<Json> = stdout
            .lines()
            .filter(|line| !line.starts_with("work "))
            .map(|line| {
                serde_json::from_str(line).unwrap_or_else(|error| panic!("{name}: {error}: {line}"))
            })
            .collect();
        assert_eq!(printed.len(), outcomes.len(), "{name}: one outcome per run");
        let disagreements: Vec<String> = printed
            .iter()
            .zip(outcomes)
            .enumerate()
            .filter(|(_, (printed, expected))| printed != expected)
            .map(|(run, (printed, expected))| format!("run {run}: {printed} != {expected}"))
            .collect();
        assert!(
            disagreements.is_empty(),
            "{name}: the rendering and the Rust oracle disagree on {} of {} runs: {:?}",
            disagreements.len(),
            outcomes.len(),
            &disagreements[..disagreements.len().min(3)]
        );
        eprintln!(
            "BS-04: {label}: {name}: {} rustc runs agree with the oracle ({:?})",
            outcomes.len(),
            started.elapsed()
        );
        total += outcomes.len();
    }
    total
}

const MODULE: &str = "LexLeanBoot.CalculusCheck";

/// Mutants of the smaller committed programs: the mutation corpus.
fn mutants(sources: &[Source], seeded: &mut Seeded, count: usize, limit: usize) -> Vec<Program> {
    let values = sample_values(seeded);
    let pool = sample_tys();
    let small: Vec<&Source> = sources
        .iter()
        .filter(|source| program_size(&source.program) <= limit)
        .collect();
    (0..count)
        .map(|_| {
            let source = seeded.pick(&small);
            let mut program = mutate(&source.program, seeded, &values, &pool);
            if seeded.chance(25) {
                program = mutate(&program, seeded, &values, &pool);
            }
            program
        })
        .collect()
}

/// The roots in the order the module declares them, each with the effects
/// its declaration admits, which are exactly the effects its closure realizes.
const ROOTS: [(&str, &[&str]); 2] = [
    ("checkProgram", &["allocation", "overflow", "recursion"]),
    ("checkArguments", &["allocation", "overflow", "recursion"]),
];

/// Seeded mutants of the committed programs, and the other corpora, per root,
/// on the evaluator and under rustc. Counts are measured, not guessed: the
/// evaluator interprets the lowered program, which is what bounds it.
const EVALUATOR_LITERALS: usize = 1_500;
const EVALUATOR_PRIM_ROUNDS: usize = 12;
const EVALUATOR_MUTANTS: usize = 3_000;
const EVALUATOR_ARGUMENTS: usize = 300;
const RUSTC_LITERALS: usize = 1_500;
const RUSTC_PRIM_ROUNDS: usize = 24;
const RUSTC_MUTANTS: usize = 6_000;
const RUSTC_ARGUMENTS: usize = 750;
/// Runs per compiled harness: a harness spells every argument as a Rust
/// expression, and rustc's memory grows with their number.
const RUSTC_CHUNK: usize = 1_000;
/// A run of `checkArguments` spells a whole fixture program beside its
/// arguments, so its harnesses hold fewer.
const RUSTC_ARGUMENT_CHUNK: usize = 250;

/// A defect planted in the source: the check it removes, the text of the
/// source it is made at, and the root whose differential must catch it.
const PLANTS: [super::Plant; 12] = [
    super::Plant {
        label: "the exhaustiveness of a match is not checked",
        site: "(firstMissing required seen)",
        replacement: "(ctor Option.none [Shape])",
        root: "checkProgram",
    },
    super::Plant {
        label: "a second arm for one shape is accepted",
        site: "(if (shapeIn covered shape) then",
        replacement: "(if false then",
        root: "checkProgram",
    },
    super::Plant {
        label: "a closure may capture every parameter",
        site: "(total <=? given)",
        replacement: "(total <? given)",
        root: "checkProgram",
    },
    super::Plant {
        label: "a nat literal may be one past the largest u64",
        site: "(text \"18446744073709551615\")",
        replacement: "(text \"18446744073709551616\")",
        root: "checkProgram",
    },
    super::Plant {
        label: "`-0` is a canonical decimal",
        site: "(prim equal literal (text \"-0\") : Bool)",
        replacement: "(prim equal literal (text \"-1\") : Bool)",
        root: "checkProgram",
    },
    super::Plant {
        label: "an arm may bind one name twice",
        site: "(hasRepeat binders)",
        replacement: "false",
        root: "checkProgram",
    },
    super::Plant {
        label: "`field` is accepted on a type with several constructors",
        site: "(tyChainLength constructors) =? 1",
        replacement: "(tyChainLength constructors) =? (tyChainLength constructors)",
        root: "checkProgram",
    },
    super::Plant {
        label: "a byte literal may have an odd length",
        site: "(prim remainder (prim length hex : Nat) 2 0 : Nat)",
        replacement: "0",
        root: "checkProgram",
    },
    super::Plant {
        label: "a shift takes a u64 count",
        site: "(ctor Ty.TCons (ctor Ty.TFixed (ctor IntKind.U32)) (ctor Ty.TNil))) types (ctor Ty.TOption lead)); (let no104",
        replacement: "(ctor Ty.TCons (ctor Ty.TFixed (ctor IntKind.U64)) (ctor Ty.TNil))) types (ctor Ty.TOption lead)); (let no104",
        root: "checkProgram",
    },
    super::Plant {
        label: "the tail of a list is typed as its element",
        site: "(ctor Ty.TCons (tyFirst ty) (ctor Ty.TCons ty",
        replacement: "(ctor Ty.TCons (tyFirst ty) (ctor Ty.TCons (tyFirst ty)",
        root: "checkProgram",
    },
    super::Plant {
        label: "DEL (U+007F) is not escaped in a spelled string",
        site: "(byte =? 127)",
        replacement: "(byte =? 128)",
        root: "checkProgram",
    },
    super::Plant {
        label: "the argument count of an entry is not checked",
        site: "(valueChainLength arguments) =? (tyChainLength declared)",
        replacement: "0 =? 0",
        root: "checkArguments",
    },
];

/// Programs aimed at one rule each, so that every check the planted defects
/// remove is exercised by name whatever the seeds draw: a repeated binder, a
/// field of a type with several constructors, the count of a shift, the tail
/// of a cons, and a string with a control character in a message.
fn targeted() -> Vec<Program> {
    let var = |name: u64| Expr::Var { name };
    let function = |types: Vec<Ty>, result: Ty, body: Expr| Function {
        parameters: (0..types.len() as u64).collect(),
        types,
        result,
        body,
    };
    let pair = Ty::Pair {
        left: Box::new(Ty::Nat),
        right: Box::new(Ty::Nat),
    };
    let list = Ty::List {
        element: Box::new(Ty::Nat),
    };
    let mut out = Vec::new();
    for binders in [vec![1, 1], vec![1, 2]] {
        out.push(program_of(
            Vec::new(),
            vec![function(
                vec![pair.clone()],
                Ty::Nat,
                Expr::Match {
                    ty: Ty::Nat,
                    scrutinee: Box::new(var(0)),
                    arms: vec![Arm {
                        shape: Shape::Pair,
                        binders,
                        body: var(1),
                    }],
                },
            )],
        ));
    }
    for adt in [0, 1] {
        out.push(program_of(
            declared(),
            vec![function(
                vec![Ty::Adt { index: adt }],
                Ty::Nat,
                Expr::Field {
                    value: Box::new(var(0)),
                    index: 0,
                },
            )],
        ));
    }
    for count in [IntKind::U32, IntKind::U64] {
        let narrow = Ty::Fixed { width: IntKind::U8 };
        out.push(program_of(
            Vec::new(),
            vec![function(
                vec![narrow.clone(), Ty::Fixed { width: count }],
                Ty::Option {
                    value: Box::new(narrow),
                },
                Expr::Prim {
                    operation: Prim::ShiftLeft,
                    operands: vec![var(0), var(1)],
                },
            )],
        ));
    }
    out.push(program_of(
        Vec::new(),
        vec![function(
            vec![list.clone()],
            Ty::Nat,
            Expr::Match {
                ty: Ty::Nat,
                scrutinee: Box::new(var(0)),
                arms: vec![
                    Arm {
                        shape: Shape::Nil,
                        binders: Vec::new(),
                        body: Expr::Value {
                            ty: Ty::Nat,
                            value: Value::Nat { value: "0".into() },
                        },
                    },
                    Arm {
                        shape: Shape::Cons,
                        binders: vec![1, 2],
                        body: Expr::Prim {
                            operation: Prim::Length,
                            operands: vec![var(2)],
                        },
                    },
                ],
            },
        )],
    ));
    for string in ["del\u{7f}", "esc\u{1b}[0m", "bell\u{7}"] {
        out.push(literal_program(
            &Ty::Nat,
            &Value::String {
                value: string.to_owned(),
            },
        ));
    }
    out
}

/// The deterministic vectors every host runs: the committed programs, the
/// refusals of `TC-02`, and the corpora at the fixed seed.
fn vectors(sources: &[Source]) -> (Vec<Run>, Vec<Run>) {
    let mut seeded = Seeded(0x00C0_FFEE_0B54);
    // The small vectors first and the committed programs last: a defect
    // is found at the first run that disagrees, and the committed programs
    // are the slow ones.
    let mut programs: Vec<Program> = refusals().into_iter().map(|(program, _)| program).collect();
    programs.extend(targeted());
    programs.extend(literal_corpus(&mut seeded, 400));
    programs.extend(prim_corpus(&mut seeded, 4));
    programs.extend(shape_corpus(&mut seeded));
    programs.extend(sources.iter().map(|source| source.program.clone()));
    let mut program_runs: Vec<Run> = programs.iter().map(program_run).collect();
    program_runs.retain(|run| !outside_claim(&run.1));
    let mut argument_runs = argument_corpus(sources, &mut seeded, 200);
    argument_runs.retain(|run| !outside_claim(&run.1));
    (program_runs, argument_runs)
}

/// The corpora of the normative host: the vectors, then the seeded programs.
fn corpora(
    sources: &[Source],
    seed: u64,
    literals: usize,
    prim_rounds: usize,
    mutant_count: usize,
    argument_count: usize,
) -> (Vec<Run>, Vec<Run>) {
    let mut seeded = Seeded(seed);
    let mut programs = literal_corpus(&mut seeded, literals);
    programs.extend(prim_corpus(&mut seeded, prim_rounds));
    programs.extend(mutants(sources, &mut seeded, mutant_count, 80));
    let mut program_runs: Vec<Run> = programs.iter().map(program_run).collect();
    program_runs.retain(|run| !outside_claim(&run.1));
    let mut argument_runs = argument_corpus(sources, &mut seeded, argument_count);
    argument_runs.retain(|run| !outside_claim(&run.1));
    (program_runs, argument_runs)
}

fn refused(runs: &[Run]) -> usize {
    runs.iter().filter(|run| run.1["kind"] == "error").count()
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    // `Text` is the standard library's own module, imported as a source
    // package (§10.5); the checker is a module of the same project.
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("CalculusCheck")
            && checked_units.units.contains_key("LexLeanStd.Text"),
        "CalculusCheck and LexLeanStd.Text are linked modules of the project"
    );
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanBoot/CalculusCheck.eligibility.json")
                .as_std_path(),
        )
        .expect("the eligibility report"),
    )
    .expect("the report is JSON");
    let reported = report["roots"].as_array().expect("roots");
    assert_eq!(
        reported
            .iter()
            .map(|row| row["root"].as_str().expect("root").to_owned())
            .collect::<Vec<_>>(),
        ROOTS
            .iter()
            .map(|(root, _)| format!("{MODULE}.{root}"))
            .collect::<Vec<_>>(),
        "the two roots are the module's production roots, in order"
    );
    for (row, (root, effects)) in reported.iter().zip(ROOTS) {
        let declared: Vec<&str> = row["declared_effects"]
            .as_array()
            .expect("declared effects")
            .iter()
            .map(|effect| effect.as_str().expect("an effect"))
            .collect();
        assert_eq!(declared, effects, "{root}: the declared effects");
        let targets = row["targets"].as_array().expect("targets");
        assert_eq!(targets.len(), 1, "{root}: one target");
        let target = &targets[0];
        assert_eq!(target["target"], "rust-std", "{root}: the target");
        assert_eq!(target["status"], "eligible", "{root}: eligible");
        let realized: Vec<&str> = target["effects"]
            .as_array()
            .expect("effects")
            .iter()
            .map(|effect| effect["effect"].as_str().expect("an effect"))
            .collect();
        assert_eq!(
            realized, effects,
            "{root}: the declaration admits exactly the realized effects"
        );
    }

    let sources = committed();
    let nodes: usize = sources
        .iter()
        .map(|source| program_size(&source.program))
        .sum();
    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    let (vector_programs, vector_arguments) = vectors(&sources);
    // The chain is a faithful image of the list: decoding the arguments of
    // every vector returns the program the oracle read.
    let every: Vec<Program> = sources
        .iter()
        .map(|source| source.program.clone())
        .collect();
    for program in &every {
        assert_eq!(
            &dec_program(&program_arguments(program)),
            program,
            "list to chain and back"
        );
    }
    for (arguments, _) in vector_programs.iter().chain(&vector_arguments) {
        let program = dec_program(&arguments[..6]);
        assert_eq!(
            program_arguments(&program),
            arguments[..6],
            "chain to list and back"
        );
        if let Some(extra) = arguments.get(7) {
            assert_eq!(
                enc_values(&dec_values(extra)),
                *extra,
                "argument chain round trip"
            );
        }
    }
    eprintln!(
        "BS-04: {} committed programs ({nodes} nodes) and {} vectors ({} refused) of checkProgram, {} of checkArguments ({} refused) ({:?})",
        sources.len(),
        vector_programs.len(),
        refused(&vector_programs),
        vector_arguments.len(),
        refused(&vector_arguments),
        started.elapsed()
    );
    // On every host: the rendered rust-std packages of both roots, compiled
    // by that host's pinned rustc, agree with the oracle on the vectors.
    rustc_differential(
        &[
            ("checkProgram", &lowered["checkProgram"], &vector_programs),
            (
                "checkArguments",
                &lowered["checkArguments"],
                &vector_arguments,
            ),
        ],
        "vectors",
    );
    if !support::normative_host() {
        eprintln!(
            "BS-04: platform-bound host: the evaluator, the seeded corpora, and the planted defects run on the normative host ({:?})",
            started.elapsed()
        );
        return;
    }
    // On the normative host: the evaluator agrees on the vectors and the
    // seeded corpora, and the larger corpus runs under rustc.
    for (root, runs) in [
        ("checkProgram", &vector_programs),
        ("checkArguments", &vector_arguments),
    ] {
        let disagreements = super::evaluate(&lowered[root], runs, 5);
        assert!(
            disagreements.is_empty(),
            "{root}: the evaluator and the Rust checker disagree on the vectors: {disagreements:?}"
        );
    }
    eprintln!(
        "BS-04: the evaluator agrees on {} + {} vectors ({:?})",
        vector_programs.len(),
        vector_arguments.len(),
        started.elapsed()
    );
    let (seeded_programs, seeded_arguments) = corpora(
        &sources,
        0x0B54_0004,
        EVALUATOR_LITERALS,
        EVALUATOR_PRIM_ROUNDS,
        EVALUATOR_MUTANTS,
        EVALUATOR_ARGUMENTS,
    );
    for (root, runs) in [
        ("checkProgram", &seeded_programs),
        ("checkArguments", &seeded_arguments),
    ] {
        let disagreements = super::evaluate(&lowered[root], runs, 5);
        assert!(
            disagreements.is_empty(),
            "{root}: the evaluator and the Rust checker disagree: {disagreements:?}"
        );
    }
    eprintln!(
        "BS-04: the evaluator agrees on {} seeded programs ({} refused) and {} seeded argument runs ({} refused) ({:?})",
        seeded_programs.len(),
        refused(&seeded_programs),
        seeded_arguments.len(),
        refused(&seeded_arguments),
        started.elapsed()
    );
    let (large_programs, large_arguments) = corpora(
        &sources,
        0x0B54_0044,
        RUSTC_LITERALS,
        RUSTC_PRIM_ROUNDS,
        RUSTC_MUTANTS,
        RUSTC_ARGUMENTS,
    );
    let mut total = 0;
    for (at, programs) in large_programs.chunks(RUSTC_CHUNK).enumerate() {
        total += rustc_differential(
            &[("checkProgram", &lowered["checkProgram"], programs)],
            &format!("seeded program chunk {at}"),
        );
    }
    for (at, arguments) in large_arguments.chunks(RUSTC_ARGUMENT_CHUNK).enumerate() {
        total += rustc_differential(
            &[("checkArguments", &lowered["checkArguments"], arguments)],
            &format!("seeded argument chunk {at}"),
        );
    }
    eprintln!(
        "BS-04: {total} seeded rustc runs agree with the oracle ({:?})",
        started.elapsed()
    );
    // The gate is armed: each defect planted in a copy of the source is a
    // disagreement with the oracle.
    let mut detection: BTreeMap<&str, Vec<Run>> = BTreeMap::new();
    // The cheap runs first: a planted defect is found at the first run that
    // disagrees, and the committed programs are the slow ones.
    detection.insert("checkProgram", {
        let mut runs = vector_programs.clone();
        runs.extend(seeded_programs.iter().cloned());
        runs
    });
    detection.insert("checkArguments", {
        let mut runs = vector_arguments.clone();
        runs.extend(seeded_arguments.iter().cloned());
        runs
    });
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/CalculusCheck.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/CalculusCheck.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, plant.root);
        let disagreements = super::evaluate(&low, &detection[plant.root], 1);
        assert!(
            !disagreements.is_empty(),
            "{}: the planted defect is detected by the differential",
            plant.label
        );
        eprintln!(
            "BS-04: plant detected: {}: {}",
            plant.label, disagreements[0]
        );
    }
    // Under the pinned Lean, the production export certifies both roots;
    // its packages are the committed ones and realize the programs the
    // differential ran.
    if !support::lean_backed("BS-04") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let packages = harness::tree(&bundle.join("packages"));
    let committed_packages = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed_packages.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed_packages.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
    let mut realized = 0usize;
    for root in &exported.output.result.roots {
        let Some(member) = root.root.strip_prefix(&format!("{MODULE}.")) else {
            continue;
        };
        for output in &root.outputs {
            let package: Json = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&output.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("package JSON");
            assert_eq!(
                package["program"],
                serde_json::to_value(&lowered[member].program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    eprintln!("BS-04: passed in {:?}", started.elapsed());
}
