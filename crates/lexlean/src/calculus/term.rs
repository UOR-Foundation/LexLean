//! Target programs as LexLean semantic terms (SPEC.md §17.14). The fixture
//! module of the `compiler` project states, for each hand-constructed
//! program, that the LexLean evaluator produces the outcome the reference
//! interpreter computed; Lean's kernel checks each statement by reduction.

use serde_json::{json, Value as Json};

use super::{
    Adt, Arm, Expr, Fixture, Function, IntKind, OrderingValue, Outcome, Prim, Program, Shape, Ty,
    Value,
};

/// The module defining the syntax.
pub const SYNTAX: &str = "TargetSyntax";
/// The module defining the denotation.
pub const SEMANTICS: &str = "TargetSemantics";

/// A module member reference.
#[must_use]
pub fn member(module: &str, name: &str) -> Json {
    json!({"module": module, "name": name})
}

/// A named type with no arguments.
#[must_use]
pub fn named(module: &str, name: &str) -> Json {
    json!({"kind": "named", "member": member(module, name), "arguments": []})
}

fn ctor(module: &str, name: &str, arguments: Vec<Json>) -> Json {
    json!({"kind": "constructor", "constructor": member(module, name), "arguments": arguments})
}

fn syntax(name: &str, arguments: Vec<Json>) -> Json {
    ctor(SYNTAX, name, arguments)
}

/// A natural-number literal.
#[must_use]
pub fn nat(number: u64) -> Json {
    json!({"kind": "nat", "value": number.to_string()})
}

/// A list literal built from `cons` cells.
#[must_use]
pub fn list(element: &Json, items: Vec<Json>) -> Json {
    items.into_iter().rev().fold(
        json!({"kind": "nil", "element": element}),
        |tail, head| json!({"kind": "cons", "head": head, "tail": tail}),
    )
}

fn nat_list(numbers: &[u64]) -> Json {
    list(
        &json!({"kind": "nat"}),
        numbers.iter().map(|number| nat(*number)).collect(),
    )
}

fn kind(kind: IntKind) -> Json {
    syntax(&format!("IntKind.{}", kind.name()), Vec::new())
}

/// A type as a `TargetSyntax.Ty` term.
#[must_use]
pub fn ty(ty: &Ty) -> Json {
    let ty_type = named(SYNTAX, "Ty");
    match ty {
        Ty::Unit => syntax("Ty.unit", Vec::new()),
        Ty::Bool => syntax("Ty.bool", Vec::new()),
        Ty::Nat => syntax("Ty.nat", Vec::new()),
        Ty::Int => syntax("Ty.int", Vec::new()),
        Ty::Fixed { width } => syntax("Ty.fixed", vec![kind(*width)]),
        Ty::String => syntax("Ty.string", Vec::new()),
        Ty::Bytes => syntax("Ty.bytes", Vec::new()),
        Ty::Ordering => syntax("Ty.ordering", Vec::new()),
        Ty::Option { value } => syntax("Ty.option", vec![self::ty(value)]),
        Ty::Result { ok, error } => syntax("Ty.result", vec![self::ty(ok), self::ty(error)]),
        Ty::List { element } => syntax("Ty.list", vec![self::ty(element)]),
        Ty::Pair { left, right } => syntax("Ty.pair", vec![self::ty(left), self::ty(right)]),
        Ty::Adt { index } => syntax("Ty.adt", vec![nat(*index)]),
        Ty::Fn { parameters, result } => syntax(
            "Ty.fn",
            vec![
                list(&ty_type, parameters.iter().map(self::ty).collect()),
                self::ty(result),
            ],
        ),
    }
}

fn integer(representation: &str, value: &str) -> Json {
    json!({"kind": "integer", "representation": representation, "value": value})
}

/// A value as a `TargetSyntax.Value` term.
#[must_use]
pub fn value(value: &Value) -> Json {
    let value_type = named(SYNTAX, "Value");
    let fixed = |name: &str, representation: &str, text: &str| {
        syntax(
            &format!("Value.{name}"),
            vec![integer(representation, text)],
        )
    };
    let all = |items: &[Value]| items.iter().map(self::value).collect::<Vec<Json>>();
    match value {
        Value::Unit => syntax("Value.unit", Vec::new()),
        Value::Bool { value } => {
            syntax("Value.bool", vec![json!({"kind": "bool", "value": value})])
        }
        Value::Nat { value } => syntax("Value.nat", vec![json!({"kind": "nat", "value": value})]),
        Value::Int { value } => syntax("Value.int", vec![integer("int", value)]),
        Value::U8 { value } => fixed("u8", "uint8", value),
        Value::U16 { value } => fixed("u16", "uint16", value),
        Value::U32 { value } => fixed("u32", "uint32", value),
        Value::U64 { value } => fixed("u64", "uint64", value),
        Value::I8 { value } => fixed("i8", "int8", value),
        Value::I16 { value } => fixed("i16", "int16", value),
        Value::I32 { value } => fixed("i32", "int32", value),
        Value::I64 { value } => fixed("i64", "int64", value),
        Value::String { value } => syntax(
            "Value.string",
            vec![json!({"kind": "string", "value": value})],
        ),
        Value::Bytes { hex } => syntax("Value.bytes", vec![json!({"kind": "bytes", "hex": hex})]),
        Value::Ordering { value } => {
            let name = match value {
                OrderingValue::Lt => "Order.less",
                OrderingValue::Eq => "Order.same",
                OrderingValue::Gt => "Order.more",
            };
            syntax("Value.ordering", vec![syntax(name, Vec::new())])
        }
        Value::None => syntax("Value.none", Vec::new()),
        Value::Some { value } => syntax("Value.some", vec![self::value(value)]),
        Value::Ok { value } => syntax("Value.ok", vec![self::value(value)]),
        Value::Error { value } => syntax("Value.error", vec![self::value(value)]),
        Value::List { items } => syntax("Value.list", vec![list(&value_type, all(items))]),
        Value::Pair { left, right } => {
            syntax("Value.pair", vec![self::value(left), self::value(right)])
        }
        Value::Adt {
            constructor,
            fields,
        } => syntax(
            "Value.adt",
            vec![nat(*constructor), list(&value_type, all(fields))],
        ),
        Value::Closure { function, captures } => syntax(
            "Value.closure",
            vec![nat(*function), list(&value_type, all(captures))],
        ),
    }
}

fn shape(shape: Shape) -> Json {
    let simple = |name: &str| syntax(&format!("Shape.{name}"), Vec::new());
    match shape {
        Shape::None => simple("none"),
        Shape::Some => simple("some"),
        Shape::Ok => simple("ok"),
        Shape::Error => simple("error"),
        Shape::Nil => simple("nil"),
        Shape::Cons => simple("cons"),
        Shape::Zero => simple("zero"),
        Shape::Succ => simple("succ"),
        Shape::Pair => simple("pair"),
        Shape::True => simple("true"),
        Shape::False => simple("false"),
        Shape::Unit => simple("unit"),
        Shape::Lt => simple("lt"),
        Shape::Eq => simple("eq"),
        Shape::Gt => simple("gt"),
        Shape::Adt { constructor } => syntax("Shape.adt", vec![nat(constructor)]),
    }
}

fn prim(prim: &Prim) -> Json {
    let simple = |name: &str| syntax(&format!("Prim.{name}"), Vec::new());
    match prim {
        Prim::NatAdd => simple("natAdd"),
        Prim::NatSub => simple("natSub"),
        Prim::NatMul => simple("natMul"),
        Prim::NatQuot => simple("natQuot"),
        Prim::NatRem => simple("natRem"),
        Prim::NatEq => simple("natEq"),
        Prim::NatLe => simple("natLe"),
        Prim::NatLt => simple("natLt"),
        Prim::IntAdd => simple("intAdd"),
        Prim::IntSub => simple("intSub"),
        Prim::IntMul => simple("intMul"),
        Prim::IntNeg => simple("intNeg"),
        Prim::IntQuot => simple("intQuot"),
        Prim::IntRem => simple("intRem"),
        Prim::CheckedAdd => simple("checkedAdd"),
        Prim::CheckedSub => simple("checkedSub"),
        Prim::CheckedMul => simple("checkedMul"),
        Prim::CheckedNeg => simple("checkedNeg"),
        Prim::CheckedQuot => simple("checkedQuot"),
        Prim::BitAnd => simple("bitAnd"),
        Prim::BitOr => simple("bitOr"),
        Prim::BitXor => simple("bitXor"),
        Prim::BitNot => simple("bitNot"),
        Prim::ShiftLeft => simple("shiftLeft"),
        Prim::ShiftRight => simple("shiftRight"),
        Prim::Equal => simple("equal"),
        Prim::BoolNot => simple("boolNot"),
        Prim::BoolAnd => simple("boolAnd"),
        Prim::BoolOr => simple("boolOr"),
        Prim::Append => simple("append"),
        Prim::Length => simple("length"),
        Prim::Index => simple("index"),
        Prim::Slice => simple("slice"),
        Prim::Utf8Encode => simple("utf8Encode"),
        Prim::Utf8Decode => simple("utf8Decode"),
        Prim::CompareBytes => simple("compareBytes"),
        Prim::SplitExact => simple("splitExact"),
        Prim::Join => simple("join"),
        Prim::FormatDecimal => simple("formatDecimal"),
        Prim::Compare => simple("compare"),
        Prim::Convert { target } => syntax("Prim.convert", vec![kind(*target)]),
        Prim::ParseDecimal { target } => syntax("Prim.parseDecimal", vec![ty(target)]),
    }
}

fn exprs(items: &[Expr]) -> Json {
    list(&named(SYNTAX, "Expr"), items.iter().map(expr).collect())
}

fn arm(arm: &Arm) -> Json {
    syntax(
        "Arm.arm",
        vec![shape(arm.shape), nat_list(&arm.binders), expr(&arm.body)],
    )
}

/// An expression as a `TargetSyntax.Expr` term.
#[must_use]
pub fn expr(expr: &Expr) -> Json {
    match expr {
        Expr::Value { ty: t, value: v } => syntax("Expr.value", vec![ty(t), value(v)]),
        Expr::Var { name } => syntax("Expr.var", vec![nat(*name)]),
        Expr::Let {
            name,
            ty: t,
            bound,
            body,
        } => syntax(
            "Expr.let",
            vec![nat(*name), ty(t), self::expr(bound), self::expr(body)],
        ),
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => syntax(
            "Expr.cond",
            vec![
                self::expr(condition),
                self::expr(then_branch),
                self::expr(else_branch),
            ],
        ),
        Expr::Match {
            ty: t,
            scrutinee,
            arms,
        } => syntax(
            "Expr.match",
            vec![
                ty(t),
                self::expr(scrutinee),
                list(&named(SYNTAX, "Arm"), arms.iter().map(arm).collect()),
            ],
        ),
        Expr::Build {
            shape: s,
            ty: t,
            operands,
        } => syntax("Expr.build", vec![shape(*s), ty(t), exprs(operands)]),
        Expr::Call { function, operands } => {
            syntax("Expr.call", vec![nat(*function), exprs(operands)])
        }
        Expr::Closure { function, captures } => {
            syntax("Expr.closure", vec![nat(*function), exprs(captures)])
        }
        Expr::Apply { target, operands } => {
            syntax("Expr.apply", vec![self::expr(target), exprs(operands)])
        }
        Expr::Prim {
            operation,
            operands,
        } => syntax("Expr.prim", vec![prim(operation), exprs(operands)]),
        Expr::First { value } => syntax("Expr.first", vec![self::expr(value)]),
        Expr::Second { value } => syntax("Expr.second", vec![self::expr(value)]),
        Expr::Field { value, index } => syntax("Expr.field", vec![self::expr(value), nat(*index)]),
    }
}

fn adt(adt: &Adt) -> Json {
    let ty_type = named(SYNTAX, "Ty");
    let list_of_tys = json!({"kind": "list", "element": ty_type});
    json!({"kind": "record", "type": member(SYNTAX, "Adt"), "fields": [
        {"field": "constructors", "value": list(&list_of_tys, adt.constructors.iter().map(|fields| list(&ty_type, fields.iter().map(ty).collect())).collect())}
    ]})
}

/// A function as a `TargetSyntax.Function` record.
#[must_use]
pub fn function(function: &Function) -> Json {
    json!({"kind": "record", "type": member(SYNTAX, "Function"), "fields": [
        {"field": "parameters", "value": nat_list(&function.parameters)},
        {"field": "types", "value": list(&named(SYNTAX, "Ty"), function.types.iter().map(ty).collect())},
        {"field": "result", "value": ty(&function.result)},
        {"field": "body", "value": expr(&function.body)}
    ]})
}

/// A whole program as a `TargetSyntax.Program` term.
#[must_use]
pub fn program(program: &Program) -> Json {
    json!({"kind": "record", "type": member(SYNTAX, "Program"), "fields": [
        {"field": "adts", "value": list(&named(SYNTAX, "Adt"), program.adts.iter().map(adt).collect())},
        {"field": "functions", "value": list(&named(SYNTAX, "Function"), program.functions.iter().map(function).collect())}
    ]})
}

/// An outcome as a `TargetSemantics.Outcome` term.
#[must_use]
pub fn outcome(outcome: &Outcome) -> Json {
    match outcome {
        Outcome::Value { value: v, steps } => {
            ctor(SEMANTICS, "Outcome.value", vec![value(v), nat(*steps)])
        }
        Outcome::Overflow { steps } => ctor(SEMANTICS, "Outcome.overflow", vec![nat(*steps)]),
        Outcome::Stuck => ctor(SEMANTICS, "Outcome.stuck", Vec::new()),
        Outcome::Exhausted => ctor(SEMANTICS, "Outcome.exhausted", Vec::new()),
    }
}

/// The declarations stating one fixture: its program, the evaluation of its
/// entry on its arguments (`<id>Run`), and the theorem that the evaluation
/// reduces to the expected outcome.
#[must_use]
pub fn fixture_declarations(fixture: &Fixture, identifier: &str) -> Vec<Json> {
    let program_name = format!("{identifier}Program");
    let run_name = format!("{identifier}Run");
    let run = json!({"kind": "call", "function": member(SEMANTICS, "run"), "arguments": [
        nat(fixture.fuel),
        {"kind": "call", "function": {"name": program_name}, "arguments": []},
        nat(fixture.entry),
        list(&named(SYNTAX, "Value"), fixture.arguments.iter().map(value).collect())
    ]});
    vec![
        json!({"kind": "definition", "name": program_name, "parameters": [], "result": named(SYNTAX, "Program"), "body": program(&fixture.program)}),
        json!({"kind": "definition", "name": run_name, "parameters": [], "result": named(SEMANTICS, "Outcome"), "body": run}),
        json!({"kind": "theorem", "name": format!("{identifier}Outcome"), "parameters": [],
               "statement": {"kind": "eq", "left": {"kind": "call", "function": {"name": run_name}, "arguments": []}, "right": outcome(&fixture.expected)},
               "proof": {"kind": "reflexivity"}}),
    ]
}
