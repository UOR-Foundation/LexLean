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

use super::{bytes, list, option, pair, text, Run, Seeded};
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

fn enc_tys(tys: &[Ty]) -> Value {
    list(tys.iter().map(enc_ty).collect())
}

/// `Ty`: unit, bool, nat, int, fixed, string, bytes, ordering, option,
/// result, list, pair, adt, fn.
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
    list(values.iter().map(enc_value).collect())
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
    list(exprs.iter().map(enc_expr).collect())
}

fn enc_arm(arm: &Arm) -> Value {
    adt(
        0,
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
                list(arms.iter().map(enc_arm).collect()),
            ],
        ),
        Expr::Build {
            shape,
            ty,
            operands,
        } => adt(5, vec![enc_shape(*shape), enc_ty(ty), enc_exprs(operands)]),
        Expr::Call { function, operands } => adt(6, vec![nat(*function), enc_exprs(operands)]),
        Expr::Closure { function, captures } => {
            adt(7, vec![nat(*function), enc_exprs(captures)])
        }
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

fn enc_function(function: &Function) -> Value {
    adt(
        0,
        vec![
            list(function.parameters.iter().map(|name| nat(*name)).collect()),
            enc_tys(&function.types),
            enc_ty(&function.result),
            enc_expr(&function.body),
        ],
    )
}

/// `Program`, a structure of its spec, its `AdtDecl`s, and its functions.
fn enc_program(program: &Program) -> Value {
    adt(
        0,
        vec![
            text(&program.spec),
            list(
                program
                    .adts
                    .iter()
                    .map(|declared| {
                        adt(
                            0,
                            vec![list(
                                declared
                                    .constructors
                                    .iter()
                                    .map(|fields| enc_tys(fields))
                                    .collect(),
                            )],
                        )
                    })
                    .collect(),
            ),
            list(program.functions.iter().map(enc_function).collect()),
        ],
    )
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
    (
        vec![enc_program(program)],
        verdict(check::check(program)),
    )
}

/// A run of `checkArguments`.
fn arguments_run(program: &Program, entry: u64, arguments: &[Value]) -> Run {
    (
        vec![
            enc_program(program),
            nat(entry),
            enc_values(arguments),
        ],
        verdict(check::check_arguments(program, entry, arguments)),
    )
}

// ---------------------------------------------------------------- vectors

/// A program from the repository with the entry and arguments it states, if
/// any.
struct Source {
    label: String,
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
            if !matches!(name.as_str(), "target" | ".git" | ".lexlean" | "node_modules") {
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
    let mut admit = |label: String, program: Program, entry: Option<(u64, Vec<Value>)>| {
        let key = serde_json::to_string(&program).expect("a program is JSON");
        if seen.insert(key) {
            out.push(Source {
                label,
                program,
                entry,
            });
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
            admit(
                relative,
                fixture.program,
                Some((fixture.entry, fixture.arguments)),
            );
        } else if name == "package.json"
            && (relative.starts_with("compiler/rust/")
                || relative.starts_with("tests/production-export/"))
        {
            let json: Json =
                serde_json::from_slice(&std::fs::read(&path).expect("a package manifest"))
                    .expect("a manifest is JSON");
            let program: Program = serde_json::from_value(json["program"].clone())
                .unwrap_or_else(|error| panic!("{relative}: {error}"));
            admit(relative, program, None);
        } else if relative.contains("/preserve/program/R") {
            let mut json: Json =
                serde_json::from_slice(&std::fs::read(&path).expect("a preserved program"))
                    .expect("a program is JSON");
            json["spec"] = json!(PROGRAM_SPEC);
            let program: Program = serde_json::from_value(json)
                .unwrap_or_else(|error| panic!("{relative}: {error}"));
            admit(relative, program, None);
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

const MODULE: &str = "LexLeanBoot.CalculusCheck";

pub(super) fn case() {
    let started = std::time::Instant::now();
    let sources = committed();
    let nodes: usize = sources.iter().map(|source| program_size(&source.program)).sum();
    let accepted = sources
        .iter()
        .filter(|source| check::check(&source.program).is_ok())
        .count();
    eprintln!(
        "BS-04: {} committed programs ({} nodes, {} accepted, {} refused)",
        sources.len(),
        nodes,
        accepted,
        sources.len() - accepted
    );
    let project = support::P::bootstrap();
    let checked = support::checked_project(&project);
    let program_root = super::lower(&checked, &project, MODULE, "checkProgram");
    eprintln!("BS-04: lowered ({:?})", started.elapsed());
    let runs: Vec<Run> = sources.iter().map(|source| program_run(&source.program)).collect();
    let disagreements = super::evaluate(&program_root, &runs, 5);
    eprintln!("BS-04: evaluator ({:?}): {disagreements:#?}", started.elapsed());
    let _ = (Seeded(1), bytes(&[]), option(None), pair(Value::Unit, Value::Unit), arguments_run as fn(&Program, u64, &[Value]) -> Run);
}
