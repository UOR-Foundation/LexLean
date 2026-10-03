//! The hand-constructed target programs of the production realization
//! calculus (SPEC.md §17.14), and the generated `compiler` project files that
//! state their meaning to Lean.
//!
//! Every fixture is written here, by hand, against the calculus syntax; its
//! expected outcome is computed by the reference interpreter. The committed
//! `compiler/fixtures/<name>.json` files and `compiler/src/TargetFixtures`
//! module, together with the calculus modules they are stated against
//! ([`crate::calculus_source`]), are exactly what [`files`] renders, which
//! `cargo xtask check-calculus` enforces. In the module, each fixture has a
//! `<id>Run` definition evaluating the LexLean denotation, a theorem that
//! the kernel reduces it to the expected outcome wherever every primitive it
//! reaches reduces in the kernel, and, for a library fixture, a theorem that
//! the realization computes exactly the value LexLean's own collection
//! primitive computes.

use std::collections::BTreeMap;
use std::path::Path;

use lexlean::calculus::library::Template;
use lexlean::calculus::{
    interp, term, Adt, Arm, Expr, Fixture, Function, IntKind, OrderingValue, Outcome, Prim,
    Program, Shape, Ty, Value, FIXTURE_SPEC, PROGRAM_SPEC,
};
use serde_json::{json, Value as Json};

use crate::lx;

/// The library instance a fixture exercises.
#[derive(Debug, Clone)]
pub struct LibraryUse {
    /// The template.
    pub template: Template,
    /// Its type arguments.
    pub types: Vec<Ty>,
    /// The function index of the instance's entry.
    pub at: u64,
}

/// One fixture with what the `compiler` project states about it.
#[derive(Debug, Clone)]
pub struct Case {
    /// The fixture.
    pub fixture: Fixture,
    /// The library instance it exercises.
    pub library: Option<LibraryUse>,
    /// For a library fixture, the LexLean term of type `Value` that
    /// LexLean's own primitive computes for the same input.
    pub oracle: Option<Json>,
}

/// Whether the kernel decides a fixture by reduction: no primitive
/// application in it is kernel-opaque (`check::KERNEL_OPAQUE`). Lean's
/// evaluator decides every fixture, these included.
///
/// # Panics
///
/// Panics on an invalid fixture program, which no generated fixture is.
#[must_use]
pub fn kernel_reducible(fixture: &Fixture) -> bool {
    lexlean::calculus::check::kernel_opaque(&fixture.program)
        .unwrap_or_else(|reason| panic!("fixture {}: {reason}", fixture.name))
        .is_empty()
}

/// The Lean identifier of a fixture: its kebab-case name in camel case.
#[must_use]
pub fn identifier(name: &str) -> String {
    let mut out = String::new();
    let mut upper = false;
    for character in name.chars() {
        if character == '-' {
            upper = true;
        } else if upper {
            out.extend(character.to_uppercase());
            upper = false;
        } else {
            out.push(character);
        }
    }
    out
}

// --- target syntax ---------------------------------------------------------

fn nat_t() -> Ty {
    Ty::Nat
}
fn fixed_t(width: IntKind) -> Ty {
    Ty::Fixed { width }
}
fn opt_t(value: Ty) -> Ty {
    Ty::Option {
        value: Box::new(value),
    }
}
fn res_t(ok: Ty, error: Ty) -> Ty {
    Ty::Result {
        ok: Box::new(ok),
        error: Box::new(error),
    }
}
fn list_t(element: Ty) -> Ty {
    Ty::List {
        element: Box::new(element),
    }
}
fn pair_t(left: Ty, right: Ty) -> Ty {
    Ty::Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}
fn fn_t(parameters: Vec<Ty>, result: Ty) -> Ty {
    Ty::Fn {
        parameters,
        result: Box::new(result),
    }
}

fn natv(number: u64) -> Value {
    Value::Nat {
        value: number.to_string(),
    }
}
fn intv(number: i64) -> Value {
    Value::Int {
        value: number.to_string(),
    }
}
fn fxv(width: IntKind, number: i128) -> Value {
    let value = number.to_string();
    match width {
        IntKind::U8 => Value::U8 { value },
        IntKind::U16 => Value::U16 { value },
        IntKind::U32 => Value::U32 { value },
        IntKind::U64 => Value::U64 { value },
        IntKind::I8 => Value::I8 { value },
        IntKind::I16 => Value::I16 { value },
        IntKind::I32 => Value::I32 { value },
        IntKind::I64 => Value::I64 { value },
    }
}
fn strv(text: &str) -> Value {
    Value::String {
        value: text.to_owned(),
    }
}
fn listv(items: Vec<Value>) -> Value {
    Value::List { items }
}
fn pairv(left: Value, right: Value) -> Value {
    Value::Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn v(name: u64) -> Expr {
    Expr::Var { name }
}
fn lit(ty: Ty, value: Value) -> Expr {
    Expr::Value { ty, value }
}
fn nat(number: u64) -> Expr {
    lit(Ty::Nat, natv(number))
}
fn int(number: i64) -> Expr {
    lit(Ty::Int, intv(number))
}
fn fx(width: IntKind, number: i128) -> Expr {
    lit(fixed_t(width), fxv(width, number))
}
fn string(text: &str) -> Expr {
    lit(Ty::String, strv(text))
}
fn bytes(hex: &str) -> Expr {
    lit(
        Ty::Bytes,
        Value::Bytes {
            hex: hex.to_owned(),
        },
    )
}
fn p(operation: Prim, operands: Vec<Expr>) -> Expr {
    Expr::Prim {
        operation,
        operands,
    }
}
fn call(function: u64, operands: Vec<Expr>) -> Expr {
    Expr::Call { function, operands }
}
fn closure(function: u64, captures: Vec<Expr>) -> Expr {
    Expr::Closure { function, captures }
}
fn apply(target: Expr, operands: Vec<Expr>) -> Expr {
    Expr::Apply {
        target: Box::new(target),
        operands,
    }
}
fn arm(shape: Shape, binders: Vec<u64>, body: Expr) -> Arm {
    Arm {
        shape,
        binders,
        body,
    }
}
fn matching(ty: Ty, scrutinee: Expr, arms: Vec<Arm>) -> Expr {
    Expr::Match {
        ty,
        scrutinee: Box::new(scrutinee),
        arms,
    }
}
fn build(shape: Shape, ty: Ty, operands: Vec<Expr>) -> Expr {
    Expr::Build {
        shape,
        ty,
        operands,
    }
}
fn let_in(name: u64, ty: Ty, bound: Expr, body: Expr) -> Expr {
    Expr::Let {
        name,
        ty,
        bound: Box::new(bound),
        body: Box::new(body),
    }
}
fn cond(condition: Expr, then_branch: Expr, else_branch: Expr) -> Expr {
    Expr::Cond {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
    }
}
fn first(value: Expr) -> Expr {
    Expr::First {
        value: Box::new(value),
    }
}
fn second(value: Expr) -> Expr {
    Expr::Second {
        value: Box::new(value),
    }
}
fn field(value: Expr, index: u64) -> Expr {
    Expr::Field {
        value: Box::new(value),
        index,
    }
}
fn boolean(value: bool) -> Expr {
    build(
        if value { Shape::True } else { Shape::False },
        Ty::Bool,
        Vec::new(),
    )
}
/// A list built by `cons` cells, so construction is exercised.
fn list_of(element: &Ty, items: Vec<Expr>) -> Expr {
    let ty = list_t(element.clone());
    items
        .into_iter()
        .rev()
        .fold(build(Shape::Nil, ty.clone(), Vec::new()), |tail, head| {
            build(Shape::Cons, ty.clone(), vec![head, tail])
        })
}
fn function(types: Vec<Ty>, result: Ty, body: Expr) -> Function {
    Function {
        parameters: (0..types.len() as u64).collect(),
        types,
        result,
        body,
    }
}

fn program(adts: Vec<Vec<Vec<Ty>>>, functions: Vec<Function>) -> Program {
    Program {
        spec: PROGRAM_SPEC.to_owned(),
        adts: adts
            .into_iter()
            .map(|constructors| Adt { constructors })
            .collect(),
        functions,
    }
}

/// A fixture whose expected outcome the reference interpreter computes. The
/// program must be valid and is stored in canonical form.
fn case(name: &str, program: Program, entry: u64, arguments: Vec<Value>, fuel: u64) -> Case {
    lexlean::calculus::check::check(&program)
        .unwrap_or_else(|reason| panic!("fixture {name}: {reason}"));
    lexlean::calculus::check::check_arguments(&program, entry, &arguments)
        .unwrap_or_else(|reason| panic!("fixture {name}: {reason}"));
    let program = program
        .canonical()
        .unwrap_or_else(|reason| panic!("fixture {name}: {reason}"));
    let expected = interp::run(&program, fuel, entry, &arguments);
    Case {
        fixture: Fixture {
            spec: FIXTURE_SPEC.to_owned(),
            name: name.to_owned(),
            program,
            entry,
            arguments,
            fuel,
            expected,
        },
        library: None,
        oracle: None,
    }
}

// --- core fixtures ---------------------------------------------------------

#[allow(clippy::too_many_lines)]
fn core_cases() -> Vec<Case> {
    use IntKind::{I16, I32, I64, I8, U16, U32, U64, U8};
    let mut out = Vec::new();

    // Structural recursion over `nat`: 0 + 1 + ... + n.
    let sum_to = || {
        program(
            Vec::new(),
            vec![function(
                vec![nat_t()],
                nat_t(),
                matching(
                    nat_t(),
                    v(0),
                    vec![
                        arm(Shape::Zero, Vec::new(), nat(0)),
                        arm(
                            Shape::Succ,
                            vec![1],
                            p(
                                Prim::NatAdd,
                                vec![build(Shape::Succ, nat_t(), vec![v(1)]), call(0, vec![v(1)])],
                            ),
                        ),
                    ],
                ),
            )],
        )
    };
    out.push(case("sum-to", sum_to(), 0, vec![natv(10)], 1000));
    // Too little fuel: the denotation reports exhaustion, never a value.
    out.push(case("sum-to-exhausted", sum_to(), 0, vec![natv(10)], 12));

    let constant =
        |result: Ty, body: Expr| program(Vec::new(), vec![function(Vec::new(), result, body)]);
    let max = u64::MAX;
    out.push(case(
        "nat-overflow-add",
        constant(nat_t(), p(Prim::NatAdd, vec![nat(max), nat(1)])),
        0,
        Vec::new(),
        100,
    ));
    out.push(case(
        "nat-overflow-mul",
        constant(
            nat_t(),
            p(Prim::NatMul, vec![nat(4_294_967_296), nat(4_294_967_296)]),
        ),
        0,
        Vec::new(),
        100,
    ));
    out.push(case(
        "nat-overflow-succ",
        constant(nat_t(), build(Shape::Succ, nat_t(), vec![nat(max)])),
        0,
        Vec::new(),
        100,
    ));
    // An overflow in the left operand ends evaluation before the right is
    // considered; the step count records exactly the work done.
    out.push(case(
        "nat-overflow-nested",
        constant(
            nat_t(),
            p(
                Prim::NatAdd,
                vec![p(Prim::NatMul, vec![nat(max), nat(2)]), nat(1)],
            ),
        ),
        0,
        Vec::new(),
        100,
    ));

    out.push(case(
        "nat-arithmetic",
        constant(
            pair_t(list_t(nat_t()), list_t(Ty::Bool)),
            build(
                Shape::Pair,
                pair_t(list_t(nat_t()), list_t(Ty::Bool)),
                vec![
                    list_of(
                        &nat_t(),
                        vec![
                            p(Prim::NatSub, vec![nat(3), nat(5)]),
                            p(Prim::NatSub, vec![nat(9), nat(4)]),
                            p(Prim::NatMul, vec![nat(6), nat(7)]),
                            p(Prim::NatQuot, vec![nat(17), nat(5), nat(0)]),
                            p(Prim::NatRem, vec![nat(17), nat(5), nat(0)]),
                            p(Prim::NatQuot, vec![nat(1), nat(0), nat(7)]),
                            p(Prim::NatRem, vec![nat(1), nat(0), nat(8)]),
                            p(Prim::NatAdd, vec![nat(max - 1), nat(1)]),
                        ],
                    ),
                    list_of(
                        &Ty::Bool,
                        vec![
                            p(Prim::NatEq, vec![nat(3), nat(3)]),
                            p(Prim::NatEq, vec![nat(3), nat(4)]),
                            p(Prim::NatLe, vec![nat(4), nat(3)]),
                            p(Prim::NatLe, vec![nat(3), nat(3)]),
                            p(Prim::NatLt, vec![nat(3), nat(4)]),
                            p(Prim::NatLt, vec![nat(4), nat(4)]),
                        ],
                    ),
                ],
            ),
        ),
        0,
        Vec::new(),
        200,
    ));

    let min = i64::MIN;
    out.push(case(
        "int-arithmetic",
        constant(
            list_t(Ty::Int),
            list_of(
                &Ty::Int,
                vec![
                    p(Prim::IntAdd, vec![int(-5), int(3)]),
                    p(Prim::IntSub, vec![int(-5), int(3)]),
                    p(Prim::IntMul, vec![int(-4), int(6)]),
                    p(Prim::IntNeg, vec![int(7)]),
                    p(Prim::IntQuot, vec![int(-7), int(2), int(0)]),
                    p(Prim::IntRem, vec![int(-7), int(2), int(0)]),
                    p(Prim::IntQuot, vec![int(7), int(-2), int(0)]),
                    p(Prim::IntRem, vec![int(7), int(-2), int(0)]),
                    p(Prim::IntQuot, vec![int(5), int(0), int(-9)]),
                    p(Prim::IntRem, vec![int(5), int(0), int(4)]),
                    // The defaults differ from the answers, so a realization
                    // that returns the default for a nonzero divisor is seen.
                    p(Prim::IntQuot, vec![int(min), int(1), int(5)]),
                    p(Prim::IntRem, vec![int(min), int(-1), int(7)]),
                    p(Prim::IntAdd, vec![int(i64::MAX), int(min)]),
                ],
            ),
        ),
        0,
        Vec::new(),
        200,
    ));
    out.push(case(
        "int-overflow-neg",
        constant(Ty::Int, p(Prim::IntNeg, vec![int(min)])),
        0,
        Vec::new(),
        100,
    ));
    out.push(case(
        "int-overflow-mul",
        constant(Ty::Int, p(Prim::IntMul, vec![int(i64::MAX), int(2)])),
        0,
        Vec::new(),
        100,
    ));
    out.push(case(
        "int-overflow-quot",
        constant(Ty::Int, p(Prim::IntQuot, vec![int(min), int(-1), int(0)])),
        0,
        Vec::new(),
        100,
    ));
    out.push(case(
        "int-overflow-sub",
        constant(Ty::Int, p(Prim::IntSub, vec![int(min), int(1)])),
        0,
        Vec::new(),
        100,
    ));

    let checked = |operation: Prim, width: IntKind, operands: &[i128]| {
        p(
            operation,
            operands.iter().map(|value| fx(width, *value)).collect(),
        )
    };
    out.push(case(
        "fixed-checked-signed",
        constant(
            list_t(opt_t(fixed_t(I8))),
            list_of(
                &opt_t(fixed_t(I8)),
                vec![
                    checked(Prim::CheckedAdd, I8, &[127, 1]),
                    checked(Prim::CheckedAdd, I8, &[100, 27]),
                    checked(Prim::CheckedSub, I8, &[-128, 1]),
                    checked(Prim::CheckedMul, I8, &[-64, 2]),
                    checked(Prim::CheckedMul, I8, &[16, 16]),
                    checked(Prim::CheckedNeg, I8, &[-128]),
                    checked(Prim::CheckedNeg, I8, &[5]),
                    checked(Prim::CheckedQuot, I8, &[-128, -1]),
                    checked(Prim::CheckedQuot, I8, &[7, 0]),
                    checked(Prim::CheckedQuot, I8, &[-7, 2]),
                ],
            ),
        ),
        0,
        Vec::new(),
        200,
    ));
    out.push(case(
        "fixed-checked-wide",
        constant(
            pair_t(list_t(opt_t(fixed_t(I64))), list_t(opt_t(fixed_t(U64)))),
            build(
                Shape::Pair,
                pair_t(list_t(opt_t(fixed_t(I64))), list_t(opt_t(fixed_t(U64)))),
                vec![
                    list_of(
                        &opt_t(fixed_t(I64)),
                        vec![
                            checked(Prim::CheckedAdd, I64, &[i128::from(i64::MAX), 1]),
                            checked(Prim::CheckedSub, I64, &[i128::from(i64::MIN), 1]),
                            checked(Prim::CheckedMul, I64, &[i128::from(i64::MIN), -1]),
                            checked(Prim::CheckedMul, I64, &[-3_037_000_499, 3_037_000_499]),
                            checked(Prim::CheckedQuot, I64, &[i128::from(i64::MIN), -1]),
                            checked(Prim::CheckedQuot, I64, &[i128::from(i64::MIN), 7]),
                            checked(Prim::CheckedNeg, I64, &[i128::from(i64::MIN)]),
                        ],
                    ),
                    list_of(
                        &opt_t(fixed_t(U64)),
                        vec![
                            checked(Prim::CheckedAdd, U64, &[i128::from(u64::MAX), 1]),
                            checked(Prim::CheckedSub, U64, &[0, 1]),
                            checked(Prim::CheckedMul, U64, &[4_294_967_296, 4_294_967_296]),
                            checked(Prim::CheckedMul, U64, &[4_294_967_295, 4_294_967_297]),
                            checked(Prim::CheckedQuot, U64, &[i128::from(u64::MAX), 16]),
                        ],
                    ),
                ],
            ),
        ),
        0,
        Vec::new(),
        200,
    ));
    out.push(case(
        "fixed-checked-narrow",
        constant(
            pair_t(
                list_t(opt_t(fixed_t(U8))),
                pair_t(
                    list_t(opt_t(fixed_t(U16))),
                    pair_t(
                        list_t(opt_t(fixed_t(I16))),
                        pair_t(list_t(opt_t(fixed_t(U32))), list_t(opt_t(fixed_t(I32)))),
                    ),
                ),
            ),
            build(
                Shape::Pair,
                pair_t(
                    list_t(opt_t(fixed_t(U8))),
                    pair_t(
                        list_t(opt_t(fixed_t(U16))),
                        pair_t(
                            list_t(opt_t(fixed_t(I16))),
                            pair_t(list_t(opt_t(fixed_t(U32))), list_t(opt_t(fixed_t(I32)))),
                        ),
                    ),
                ),
                vec![
                    list_of(
                        &opt_t(fixed_t(U8)),
                        vec![
                            checked(Prim::CheckedAdd, U8, &[255, 1]),
                            checked(Prim::CheckedMul, U8, &[15, 17]),
                        ],
                    ),
                    build(
                        Shape::Pair,
                        pair_t(
                            list_t(opt_t(fixed_t(U16))),
                            pair_t(
                                list_t(opt_t(fixed_t(I16))),
                                pair_t(list_t(opt_t(fixed_t(U32))), list_t(opt_t(fixed_t(I32)))),
                            ),
                        ),
                        vec![
                            list_of(
                                &opt_t(fixed_t(U16)),
                                vec![
                                    checked(Prim::CheckedSub, U16, &[0, 1]),
                                    checked(Prim::CheckedAdd, U16, &[65_000, 535]),
                                ],
                            ),
                            build(
                                Shape::Pair,
                                pair_t(
                                    list_t(opt_t(fixed_t(I16))),
                                    pair_t(
                                        list_t(opt_t(fixed_t(U32))),
                                        list_t(opt_t(fixed_t(I32))),
                                    ),
                                ),
                                vec![
                                    list_of(
                                        &opt_t(fixed_t(I16)),
                                        vec![
                                            checked(Prim::CheckedNeg, I16, &[-32_768]),
                                            checked(Prim::CheckedQuot, I16, &[-32_767, -1]),
                                        ],
                                    ),
                                    build(
                                        Shape::Pair,
                                        pair_t(
                                            list_t(opt_t(fixed_t(U32))),
                                            list_t(opt_t(fixed_t(I32))),
                                        ),
                                        vec![
                                            list_of(
                                                &opt_t(fixed_t(U32)),
                                                vec![
                                                    checked(
                                                        Prim::CheckedMul,
                                                        U32,
                                                        &[65_536, 65_536],
                                                    ),
                                                    checked(Prim::CheckedQuot, U32, &[9, 4]),
                                                ],
                                            ),
                                            list_of(
                                                &opt_t(fixed_t(I32)),
                                                vec![
                                                    checked(
                                                        Prim::CheckedSub,
                                                        I32,
                                                        &[-2_147_483_648, 1],
                                                    ),
                                                    checked(
                                                        Prim::CheckedNeg,
                                                        I32,
                                                        &[-2_147_483_647],
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ),
        0,
        Vec::new(),
        300,
    ));

    let shift = |operation: Prim, width: IntKind, value: i128, amount: i128| {
        p(operation, vec![fx(width, value), fx(U32, amount)])
    };
    out.push(case(
        "fixed-bits-unsigned",
        constant(
            pair_t(list_t(fixed_t(U8)), list_t(opt_t(fixed_t(U8)))),
            build(
                Shape::Pair,
                pair_t(list_t(fixed_t(U8)), list_t(opt_t(fixed_t(U8)))),
                vec![
                    list_of(
                        &fixed_t(U8),
                        vec![
                            checked(Prim::BitAnd, U8, &[0xF0, 0x3C]),
                            checked(Prim::BitOr, U8, &[0xF0, 0x0C]),
                            checked(Prim::BitXor, U8, &[0xFF, 0x0F]),
                            checked(Prim::BitNot, U8, &[0]),
                        ],
                    ),
                    list_of(
                        &opt_t(fixed_t(U8)),
                        vec![
                            shift(Prim::ShiftLeft, U8, 0x81, 1),
                            shift(Prim::ShiftLeft, U8, 1, 8),
                            shift(Prim::ShiftRight, U8, 0x80, 7),
                            shift(Prim::ShiftRight, U8, 0x80, 9),
                        ],
                    ),
                ],
            ),
        ),
        0,
        Vec::new(),
        200,
    ));
    out.push(case(
        "fixed-bits-signed",
        constant(
            pair_t(list_t(fixed_t(I8)), list_t(opt_t(fixed_t(I8)))),
            build(
                Shape::Pair,
                pair_t(list_t(fixed_t(I8)), list_t(opt_t(fixed_t(I8)))),
                vec![
                    list_of(
                        &fixed_t(I8),
                        vec![
                            checked(Prim::BitNot, I8, &[0]),
                            checked(Prim::BitAnd, I8, &[-1, 0x55]),
                            checked(Prim::BitXor, I8, &[-128, -1]),
                        ],
                    ),
                    list_of(
                        &opt_t(fixed_t(I8)),
                        vec![
                            shift(Prim::ShiftRight, I8, -128, 1),
                            shift(Prim::ShiftRight, I8, -1, 7),
                            shift(Prim::ShiftLeft, I8, 64, 1),
                            shift(Prim::ShiftLeft, I8, 1, 7),
                            shift(Prim::ShiftLeft, I8, 1, 8),
                        ],
                    ),
                ],
            ),
        ),
        0,
        Vec::new(),
        200,
    ));

    out.push(case(
        "equality",
        constant(
            list_t(Ty::Bool),
            list_of(
                &Ty::Bool,
                vec![
                    p(Prim::Equal, vec![nat(4), nat(4)]),
                    p(Prim::Equal, vec![boolean(true), boolean(false)]),
                    p(Prim::Equal, vec![string("é"), string("é")]),
                    p(
                        Prim::Equal,
                        vec![
                            p(Prim::Compare, vec![nat(1), nat(2)]),
                            lit(
                                Ty::Ordering,
                                Value::Ordering {
                                    value: OrderingValue::Lt,
                                },
                            ),
                        ],
                    ),
                    p(
                        Prim::Equal,
                        vec![
                            build(Shape::Gt, Ty::Ordering, Vec::new()),
                            lit(
                                Ty::Ordering,
                                Value::Ordering {
                                    value: OrderingValue::Lt,
                                },
                            ),
                        ],
                    ),
                    p(Prim::Equal, vec![fx(U16, 65_535), fx(U16, 65_535)]),
                    p(Prim::Equal, vec![fx(I32, -1), fx(I32, 1)]),
                ],
            ),
        ),
        0,
        Vec::new(),
        200,
    ));

    out.push(case(
        "booleans",
        program(
            Vec::new(),
            vec![function(
                vec![Ty::Bool, Ty::Bool],
                list_t(Ty::Bool),
                list_of(
                    &Ty::Bool,
                    vec![
                        p(Prim::BoolNot, vec![v(0)]),
                        p(Prim::BoolAnd, vec![v(0), v(1)]),
                        p(Prim::BoolOr, vec![v(0), v(1)]),
                        cond(v(0), boolean(false), boolean(true)),
                        matching(
                            Ty::Bool,
                            v(1),
                            vec![
                                arm(Shape::True, Vec::new(), v(0)),
                                arm(Shape::False, Vec::new(), p(Prim::BoolNot, vec![v(0)])),
                            ],
                        ),
                    ],
                ),
            )],
        ),
        0,
        vec![Value::Bool { value: true }, Value::Bool { value: false }],
        200,
    ));

    out.push(case(
        "lists",
        program(
            Vec::new(),
            vec![function(
                vec![list_t(nat_t())],
                pair_t(
                    list_t(nat_t()),
                    pair_t(
                        nat_t(),
                        pair_t(
                            opt_t(nat_t()),
                            pair_t(
                                opt_t(nat_t()),
                                pair_t(opt_t(list_t(nat_t())), opt_t(list_t(nat_t()))),
                            ),
                        ),
                    ),
                ),
                build(
                    Shape::Pair,
                    pair_t(
                        list_t(nat_t()),
                        pair_t(
                            nat_t(),
                            pair_t(
                                opt_t(nat_t()),
                                pair_t(
                                    opt_t(nat_t()),
                                    pair_t(opt_t(list_t(nat_t())), opt_t(list_t(nat_t()))),
                                ),
                            ),
                        ),
                    ),
                    vec![
                        p(Prim::Append, vec![v(0), list_of(&nat_t(), vec![nat(9)])]),
                        build(
                            Shape::Pair,
                            pair_t(
                                nat_t(),
                                pair_t(
                                    opt_t(nat_t()),
                                    pair_t(
                                        opt_t(nat_t()),
                                        pair_t(opt_t(list_t(nat_t())), opt_t(list_t(nat_t()))),
                                    ),
                                ),
                            ),
                            vec![
                                p(Prim::Length, vec![v(0)]),
                                build(
                                    Shape::Pair,
                                    pair_t(
                                        opt_t(nat_t()),
                                        pair_t(
                                            opt_t(nat_t()),
                                            pair_t(opt_t(list_t(nat_t())), opt_t(list_t(nat_t()))),
                                        ),
                                    ),
                                    vec![
                                        p(Prim::Index, vec![v(0), nat(1)]),
                                        build(
                                            Shape::Pair,
                                            pair_t(
                                                opt_t(nat_t()),
                                                pair_t(
                                                    opt_t(list_t(nat_t())),
                                                    opt_t(list_t(nat_t())),
                                                ),
                                            ),
                                            vec![
                                                p(Prim::Index, vec![v(0), nat(3)]),
                                                build(
                                                    Shape::Pair,
                                                    pair_t(
                                                        opt_t(list_t(nat_t())),
                                                        opt_t(list_t(nat_t())),
                                                    ),
                                                    vec![
                                                        p(Prim::Slice, vec![v(0), nat(1), nat(2)]),
                                                        p(Prim::Slice, vec![v(0), nat(2), nat(2)]),
                                                    ],
                                                ),
                                            ],
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ],
                ),
            )],
        ),
        0,
        vec![listv(vec![natv(4), natv(5), natv(6)])],
        300,
    ));

    // Every fixed-width primitive at every width it admits (§17.14): one
    // fixture of arithmetic, bits, shifts, conversion, equality, and order
    // per width, in nested pairs so `rust-core` renders it too, and one of
    // decimal formatting and parsing.
    for width in IntKind::ALL {
        let (low, high) = width.range();
        let ty = fixed_t(width);
        let (a, b, c) = (v(0), v(1), v(2));
        let opt = || opt_t(fixed_t(width));
        let mut items: Vec<(Ty, Expr)> = vec![
            (opt(), p(Prim::CheckedAdd, vec![a.clone(), b.clone()])),
            (opt(), p(Prim::CheckedSub, vec![b.clone(), a.clone()])),
            (opt(), p(Prim::CheckedMul, vec![a.clone(), b.clone()])),
            (opt(), p(Prim::CheckedQuot, vec![a.clone(), b.clone()])),
        ];
        if width.signed() {
            items.push((opt(), p(Prim::CheckedNeg, vec![b.clone()])));
        }
        items.extend([
            (ty.clone(), p(Prim::BitAnd, vec![a.clone(), b.clone()])),
            (ty.clone(), p(Prim::BitOr, vec![a.clone(), b.clone()])),
            (ty.clone(), p(Prim::BitXor, vec![a.clone(), b.clone()])),
            (ty.clone(), p(Prim::BitNot, vec![b.clone()])),
            (opt(), p(Prim::ShiftLeft, vec![b.clone(), c.clone()])),
            (opt(), p(Prim::ShiftRight, vec![a.clone(), c.clone()])),
            (
                opt(),
                p(
                    Prim::ShiftLeft,
                    vec![b.clone(), fx(IntKind::U32, i128::from(width.bits()))],
                ),
            ),
            (opt(), p(Prim::Convert { target: width }, vec![int(300)])),
            (Ty::Bool, p(Prim::Equal, vec![a.clone(), b.clone()])),
            (Ty::Ordering, p(Prim::Compare, vec![a.clone(), b.clone()])),
        ]);
        let (result, body) = items
            .into_iter()
            .rev()
            .reduce(|(right_ty, right), (left_ty, left)| {
                let ty = pair_t(left_ty, right_ty);
                (ty.clone(), build(Shape::Pair, ty, vec![left, right]))
            })
            .expect("items");
        out.push(case(
            &format!("fixed-width-{}", width.name()),
            program(
                Vec::new(),
                vec![function(
                    vec![ty.clone(), ty.clone(), fixed_t(IntKind::U32)],
                    result,
                    body,
                )],
            ),
            0,
            vec![
                fxv(width, high - 1),
                fxv(width, if width.signed() { low + 1 } else { 3 }),
                fxv(IntKind::U32, 3),
            ],
            400,
        ));
        out.push(case(
            &format!("fixed-decimal-{}", width.name()),
            program(
                Vec::new(),
                vec![function(
                    vec![ty.clone(), Ty::String],
                    pair_t(Ty::String, opt_t(ty.clone())),
                    build(
                        Shape::Pair,
                        pair_t(Ty::String, opt_t(ty.clone())),
                        vec![
                            p(Prim::FormatDecimal, vec![v(0)]),
                            p(Prim::ParseDecimal { target: ty.clone() }, vec![v(1)]),
                        ],
                    ),
                )],
            ),
            0,
            vec![fxv(width, low), strv(&high.to_string())],
            100,
        ));
    }

    // Appending two long lists: the charge grows with the operands, and the
    // Rust rendering's work stays within it (the work bound of TC-07).
    out.push(case(
        "list-append-long",
        program(
            Vec::new(),
            vec![function(
                vec![list_t(nat_t()), list_t(nat_t())],
                list_t(nat_t()),
                p(Prim::Append, vec![v(0), v(1)]),
            )],
        ),
        0,
        vec![
            listv((0..20).map(natv).collect()),
            listv((20..40).map(natv).collect()),
        ],
        100,
    ));

    // Byte order alone, over literals: the kernel decides it.
    out.push(case(
        "byte-compare",
        constant(
            list_t(Ty::Ordering),
            list_of(
                &Ty::Ordering,
                vec![
                    p(Prim::CompareBytes, vec![bytes("0102"), bytes("0103")]),
                    p(Prim::CompareBytes, vec![bytes("ff"), bytes("00ff")]),
                    p(Prim::CompareBytes, vec![bytes("0a0b"), bytes("0a0b")]),
                    p(Prim::CompareBytes, vec![bytes(""), bytes("00")]),
                ],
            ),
        ),
        0,
        Vec::new(),
        100,
    ));

    // Byte equality and byte order: Lean's evaluator decides these.
    out.push(case(
        "byte-order",
        constant(
            pair_t(list_t(Ty::Bool), pair_t(Ty::Bytes, list_t(Ty::Ordering))),
            build(
                Shape::Pair,
                pair_t(list_t(Ty::Bool), pair_t(Ty::Bytes, list_t(Ty::Ordering))),
                vec![
                    list_of(
                        &Ty::Bool,
                        vec![
                            p(Prim::Equal, vec![bytes("00ff"), bytes("00fe")]),
                            p(Prim::Equal, vec![bytes(""), bytes("")]),
                            p(
                                Prim::Equal,
                                vec![
                                    p(Prim::CompareBytes, vec![bytes("01"), bytes("02")]),
                                    lit(
                                        Ty::Ordering,
                                        Value::Ordering {
                                            value: OrderingValue::Lt,
                                        },
                                    ),
                                ],
                            ),
                        ],
                    ),
                    build(
                        Shape::Pair,
                        pair_t(Ty::Bytes, list_t(Ty::Ordering)),
                        vec![
                            p(Prim::Utf8Encode, vec![string("hé€")]),
                            list_of(
                                &Ty::Ordering,
                                vec![
                                    p(Prim::CompareBytes, vec![bytes("01"), bytes("0100")]),
                                    p(Prim::CompareBytes, vec![bytes("ff"), bytes("0100")]),
                                    p(Prim::CompareBytes, vec![bytes(""), bytes("")]),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ),
        0,
        Vec::new(),
        300,
    ));
    out.push(case(
        "bytes-and-text",
        constant(
            pair_t(
                Ty::Bytes,
                pair_t(
                    list_t(nat_t()),
                    pair_t(list_t(opt_t(fixed_t(U8))), list_t(opt_t(Ty::String))),
                ),
            ),
            build(
                Shape::Pair,
                pair_t(
                    Ty::Bytes,
                    pair_t(
                        list_t(nat_t()),
                        pair_t(list_t(opt_t(fixed_t(U8))), list_t(opt_t(Ty::String))),
                    ),
                ),
                vec![
                    p(Prim::Append, vec![bytes("0102"), bytes("ff")]),
                    build(
                        Shape::Pair,
                        pair_t(
                            list_t(nat_t()),
                            pair_t(list_t(opt_t(fixed_t(U8))), list_t(opt_t(Ty::String))),
                        ),
                        vec![
                            list_of(
                                &nat_t(),
                                vec![
                                    p(Prim::Length, vec![bytes("010203")]),
                                    p(Prim::Length, vec![string("héllo€")]),
                                ],
                            ),
                            build(
                                Shape::Pair,
                                pair_t(list_t(opt_t(fixed_t(U8))), list_t(opt_t(Ty::String))),
                                vec![
                                    list_of(
                                        &opt_t(fixed_t(U8)),
                                        vec![
                                            p(Prim::Index, vec![bytes("0aff"), nat(1)]),
                                            p(Prim::Index, vec![bytes("0aff"), nat(2)]),
                                        ],
                                    ),
                                    list_of(
                                        &opt_t(Ty::String),
                                        vec![
                                            p(Prim::Utf8Decode, vec![bytes("68c3a9")]),
                                            p(Prim::Utf8Decode, vec![bytes("c328")]),
                                        ],
                                    ),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
        ),
        0,
        Vec::new(),
        300,
    ));
    out.push(case(
        "byte-slices",
        constant(
            list_t(opt_t(Ty::Bytes)),
            list_of(
                &opt_t(Ty::Bytes),
                vec![
                    p(Prim::Slice, vec![bytes("00112233"), nat(1), nat(2)]),
                    p(Prim::Slice, vec![bytes("00112233"), nat(3), nat(2)]),
                    p(Prim::Slice, vec![bytes("00112233"), nat(4), nat(0)]),
                ],
            ),
        ),
        0,
        Vec::new(),
        200,
    ));

    out.push(case(
        "split-and-join",
        constant(
            pair_t(list_t(opt_t(list_t(Ty::String))), Ty::String),
            build(
                Shape::Pair,
                pair_t(list_t(opt_t(list_t(Ty::String))), Ty::String),
                vec![
                    list_of(
                        &opt_t(list_t(Ty::String)),
                        vec![
                            p(
                                Prim::SplitExact,
                                vec![string("a,b,,c"), string(","), fx(U32, 10)],
                            ),
                            p(
                                Prim::SplitExact,
                                vec![string("a,b"), string(""), fx(U32, 10)],
                            ),
                            p(
                                Prim::SplitExact,
                                vec![string("a,b,c"), string(","), fx(U32, 2)],
                            ),
                            p(
                                Prim::SplitExact,
                                vec![string("aaa"), string("aa"), fx(U32, 5)],
                            ),
                            p(
                                Prim::SplitExact,
                                vec![string("ababcab"), string("abc"), fx(U32, 5)],
                            ),
                            p(
                                Prim::SplitExact,
                                vec![string("aabaab"), string("ab"), fx(U32, 5)],
                            ),
                            p(Prim::SplitExact, vec![string(""), string(";"), fx(U32, 1)]),
                        ],
                    ),
                    p(
                        Prim::Join,
                        vec![
                            list_of(&Ty::String, vec![string("x"), string(""), string("z")]),
                            string("--"),
                        ],
                    ),
                ],
            ),
        ),
        0,
        Vec::new(),
        300,
    ));

    out.push(case(
        "decimal",
        constant(
            pair_t(
                list_t(Ty::String),
                pair_t(list_t(opt_t(Ty::Int)), list_t(opt_t(fixed_t(U8)))),
            ),
            build(
                Shape::Pair,
                pair_t(
                    list_t(Ty::String),
                    pair_t(list_t(opt_t(Ty::Int)), list_t(opt_t(fixed_t(U8)))),
                ),
                vec![
                    list_of(
                        &Ty::String,
                        vec![
                            p(Prim::FormatDecimal, vec![int(-42)]),
                            p(Prim::FormatDecimal, vec![fx(I8, -128)]),
                            p(Prim::FormatDecimal, vec![fx(U64, i128::from(u64::MAX))]),
                        ],
                    ),
                    build(
                        Shape::Pair,
                        pair_t(list_t(opt_t(Ty::Int)), list_t(opt_t(fixed_t(U8)))),
                        vec![
                            list_of(
                                &opt_t(Ty::Int),
                                [
                                    "123",
                                    "-0",
                                    "007",
                                    "+5",
                                    "",
                                    "-",
                                    "-9223372036854775808",
                                    "12a",
                                ]
                                .iter()
                                .map(|text| {
                                    p(Prim::ParseDecimal { target: Ty::Int }, vec![string(text)])
                                })
                                .collect(),
                            ),
                            list_of(
                                &opt_t(fixed_t(U8)),
                                ["255", "256", "-1", "0"]
                                    .iter()
                                    .map(|text| {
                                        p(
                                            Prim::ParseDecimal {
                                                target: fixed_t(U8),
                                            },
                                            vec![string(text)],
                                        )
                                    })
                                    .collect(),
                            ),
                        ],
                    ),
                ],
            ),
        ),
        0,
        Vec::new(),
        400,
    ));
    // A canonical decimal outside the realized `int` is an overflow, not
    // `none`: the mathematical value exists and does not fit.
    out.push(case(
        "decimal-overflow",
        constant(
            opt_t(Ty::Int),
            p(
                Prim::ParseDecimal { target: Ty::Int },
                vec![string("9223372036854775808")],
            ),
        ),
        0,
        Vec::new(),
        100,
    ));
    // A canonical decimal beyond every machine integer a realization may
    // parse through is still an overflow.
    out.push(case(
        "decimal-overflow-wide",
        constant(
            opt_t(Ty::Int),
            p(
                Prim::ParseDecimal { target: Ty::Int },
                vec![string(&format!("1{}", "0".repeat(40)))],
            ),
        ),
        0,
        Vec::new(),
        100,
    ));

    // Conversions, gathered in a record so every target width is a field.
    let converted = vec![
        opt_t(fixed_t(U8)),
        opt_t(fixed_t(U16)),
        opt_t(fixed_t(U32)),
        opt_t(fixed_t(U64)),
        opt_t(fixed_t(I8)),
        opt_t(fixed_t(I16)),
        opt_t(fixed_t(I32)),
        opt_t(fixed_t(I64)),
    ];
    out.push(case(
        "convert",
        program(
            vec![vec![converted]],
            vec![function(
                Vec::new(),
                Ty::Adt { index: 0 },
                build(
                    Shape::Adt { constructor: 0 },
                    Ty::Adt { index: 0 },
                    vec![
                        p(Prim::Convert { target: U8 }, vec![int(300)]),
                        p(Prim::Convert { target: U16 }, vec![int(65_535)]),
                        p(Prim::Convert { target: U32 }, vec![fx(I8, -1)]),
                        p(Prim::Convert { target: U64 }, vec![fx(U32, 4_294_967_295)]),
                        p(Prim::Convert { target: I8 }, vec![fx(U8, 128)]),
                        p(Prim::Convert { target: I16 }, vec![fx(I64, -32_768)]),
                        p(
                            Prim::Convert { target: I32 },
                            vec![fx(U64, i128::from(u64::MAX))],
                        ),
                        p(Prim::Convert { target: I64 }, vec![int(i64::MIN)]),
                    ],
                ),
            )],
        ),
        0,
        Vec::new(),
        200,
    ));

    // The key order: a three-way match over `compare` at every key type.
    let code = |order: Expr| {
        matching(
            nat_t(),
            order,
            vec![
                arm(Shape::Lt, Vec::new(), nat(0)),
                arm(Shape::Eq, Vec::new(), nat(1)),
                arm(Shape::Gt, Vec::new(), nat(2)),
            ],
        )
    };
    let compare = |left: Expr, right: Expr| p(Prim::Compare, vec![left, right]);
    let ns_pair = |number: i64, text: &str| {
        build(
            Shape::Pair,
            pair_t(Ty::Int, Ty::String),
            vec![int(number), string(text)],
        )
    };
    out.push(case(
        "ordering",
        constant(
            pair_t(list_t(nat_t()), list_t(Ty::Ordering)),
            build(
                Shape::Pair,
                pair_t(list_t(nat_t()), list_t(Ty::Ordering)),
                vec![
                    list_of(
                        &nat_t(),
                        vec![
                            code(compare(nat(2), nat(10))),
                            code(compare(int(-3), int(-3))),
                            code(compare(fx(I16, 5), fx(I16, -5))),
                            code(compare(fx(U64, 0), fx(U64, 1))),
                            code(compare(boolean(false), boolean(true))),
                            code(compare(string("z"), string("é"))),
                            code(compare(string("ab"), string("a"))),
                            code(compare(ns_pair(1, "b"), ns_pair(1, "a"))),
                            code(compare(ns_pair(0, "z"), ns_pair(1, "a"))),
                            code(lit(
                                Ty::Ordering,
                                Value::Ordering {
                                    value: OrderingValue::Gt,
                                },
                            )),
                        ],
                    ),
                    list_of(
                        &Ty::Ordering,
                        vec![
                            build(Shape::Lt, Ty::Ordering, Vec::new()),
                            build(Shape::Eq, Ty::Ordering, Vec::new()),
                            build(Shape::Gt, Ty::Ordering, Vec::new()),
                        ],
                    ),
                ],
            ),
        ),
        0,
        Vec::new(),
        400,
    ));

    // A recursive ADT and a record: ADT 0 is `literal(nat) | plus(e, e) |
    // scaled(point, e)`, ADT 1 the record `point(nat, nat)`.
    let expression = Ty::Adt { index: 0 };
    let point = Ty::Adt { index: 1 };
    let literal = |number: u64| {
        build(
            Shape::Adt { constructor: 0 },
            expression.clone(),
            vec![nat(number)],
        )
    };
    let tree = build(
        Shape::Adt { constructor: 1 },
        expression.clone(),
        vec![
            literal(2),
            build(
                Shape::Adt { constructor: 2 },
                expression.clone(),
                vec![
                    build(
                        Shape::Adt { constructor: 0 },
                        point.clone(),
                        vec![nat(3), nat(4)],
                    ),
                    literal(5),
                ],
            ),
        ],
    );
    out.push(case(
        "adt-evaluation",
        program(
            vec![
                vec![
                    vec![nat_t()],
                    vec![expression.clone(), expression.clone()],
                    vec![point.clone(), expression.clone()],
                ],
                vec![vec![nat_t(), nat_t()]],
            ],
            vec![
                function(
                    vec![expression.clone()],
                    nat_t(),
                    matching(
                        nat_t(),
                        v(0),
                        vec![
                            arm(Shape::Adt { constructor: 0 }, vec![1], v(1)),
                            arm(
                                Shape::Adt { constructor: 1 },
                                vec![2, 3],
                                p(Prim::NatAdd, vec![call(0, vec![v(2)]), call(0, vec![v(3)])]),
                            ),
                            arm(
                                Shape::Adt { constructor: 2 },
                                vec![4, 5],
                                p(
                                    Prim::NatMul,
                                    vec![
                                        p(Prim::NatAdd, vec![field(v(4), 0), field(v(4), 1)]),
                                        call(0, vec![v(5)]),
                                    ],
                                ),
                            ),
                        ],
                    ),
                ),
                function(Vec::new(), nat_t(), call(0, vec![tree])),
            ],
        ),
        1,
        Vec::new(),
        400,
    ));
    // A record of three fields read in an order no permutation of the
    // indices preserves.
    out.push(case(
        "record-fields",
        program(
            vec![vec![vec![nat_t(), nat_t(), nat_t()]]],
            vec![function(
                vec![Ty::Adt { index: 0 }],
                pair_t(nat_t(), pair_t(nat_t(), nat_t())),
                build(
                    Shape::Pair,
                    pair_t(nat_t(), pair_t(nat_t(), nat_t())),
                    vec![
                        p(Prim::NatSub, vec![field(v(0), 0), field(v(0), 2)]),
                        build(
                            Shape::Pair,
                            pair_t(nat_t(), nat_t()),
                            vec![field(v(0), 1), field(v(0), 2)],
                        ),
                    ],
                ),
            )],
        ),
        0,
        vec![Value::Adt {
            constructor: 0,
            fields: vec![natv(20), natv(5), natv(3)],
        }],
        100,
    ));

    // Higher-order: a closure capturing an offset, applied by a map over a
    // list, and a closure returned as a value.
    out.push(case(
        "closures",
        program(
            Vec::new(),
            vec![
                function(
                    vec![list_t(nat_t()), nat_t()],
                    list_t(nat_t()),
                    call(1, vec![closure(2, vec![v(1)]), v(0)]),
                ),
                function(
                    vec![fn_t(vec![nat_t()], nat_t()), list_t(nat_t())],
                    list_t(nat_t()),
                    matching(
                        list_t(nat_t()),
                        v(1),
                        vec![
                            arm(
                                Shape::Nil,
                                Vec::new(),
                                build(Shape::Nil, list_t(nat_t()), Vec::new()),
                            ),
                            arm(
                                Shape::Cons,
                                vec![2, 3],
                                build(
                                    Shape::Cons,
                                    list_t(nat_t()),
                                    vec![apply(v(0), vec![v(2)]), call(1, vec![v(0), v(3)])],
                                ),
                            ),
                        ],
                    ),
                ),
                function(
                    vec![nat_t(), nat_t()],
                    nat_t(),
                    p(Prim::NatAdd, vec![v(1), v(0)]),
                ),
            ],
        ),
        0,
        vec![listv(vec![natv(1), natv(2), natv(3)]), natv(10)],
        400,
    ));
    out.push(case(
        "closure-value",
        program(
            Vec::new(),
            vec![
                function(
                    vec![nat_t()],
                    fn_t(vec![nat_t()], nat_t()),
                    closure(1, vec![v(0)]),
                ),
                function(
                    vec![nat_t(), nat_t()],
                    nat_t(),
                    p(Prim::NatMul, vec![v(0), v(1)]),
                ),
            ],
        ),
        0,
        vec![natv(6)],
        100,
    ));
    // A closure of two captures, applied twice through a function value,
    // computing `x * k0 - k1`: exchanging the captures changes the result.
    out.push(case(
        "closure-captures",
        program(
            Vec::new(),
            vec![
                function(
                    vec![nat_t(), nat_t(), nat_t()],
                    nat_t(),
                    call(1, vec![closure(2, vec![v(0), v(1)]), v(2)]),
                ),
                function(
                    vec![fn_t(vec![nat_t()], nat_t()), nat_t()],
                    nat_t(),
                    apply(v(0), vec![apply(v(0), vec![v(1)])]),
                ),
                function(
                    vec![nat_t(), nat_t(), nat_t()],
                    nat_t(),
                    p(Prim::NatSub, vec![p(Prim::NatMul, vec![v(2), v(0)]), v(1)]),
                ),
            ],
        ),
        0,
        vec![natv(10), natv(3), natv(2)],
        100,
    ));

    // Binding and the remaining constructor shapes: `let`, pairs, unit,
    // options, and results.
    let result_t = res_t(nat_t(), Ty::String);
    let classify = |value: Expr| {
        matching(
            Ty::String,
            value,
            vec![
                arm(Shape::Ok, vec![3], string("ok")),
                arm(Shape::Error, vec![4], v(4)),
            ],
        )
    };
    out.push(case(
        "binding-and-shapes",
        program(
            Vec::new(),
            vec![function(
                vec![nat_t()],
                pair_t(
                    pair_t(Ty::String, nat_t()),
                    pair_t(nat_t(), pair_t(Ty::String, Ty::String)),
                ),
                let_in(
                    1,
                    pair_t(nat_t(), Ty::String),
                    build(
                        Shape::Pair,
                        pair_t(nat_t(), Ty::String),
                        vec![v(0), string("left")],
                    ),
                    let_in(
                        2,
                        opt_t(nat_t()),
                        build(Shape::Some, opt_t(nat_t()), vec![first(v(1))]),
                        build(
                            Shape::Pair,
                            pair_t(
                                pair_t(Ty::String, nat_t()),
                                pair_t(nat_t(), pair_t(Ty::String, Ty::String)),
                            ),
                            vec![
                                build(
                                    Shape::Pair,
                                    pair_t(Ty::String, nat_t()),
                                    vec![second(v(1)), first(v(1))],
                                ),
                                build(
                                    Shape::Pair,
                                    pair_t(nat_t(), pair_t(Ty::String, Ty::String)),
                                    vec![
                                        matching(
                                            nat_t(),
                                            v(2),
                                            vec![
                                                arm(Shape::None, Vec::new(), nat(0)),
                                                arm(Shape::Some, vec![5], v(5)),
                                            ],
                                        ),
                                        build(
                                            Shape::Pair,
                                            pair_t(Ty::String, Ty::String),
                                            vec![
                                                classify(build(
                                                    Shape::Ok,
                                                    result_t.clone(),
                                                    vec![v(0)],
                                                )),
                                                matching(
                                                    Ty::String,
                                                    build(Shape::Unit, Ty::Unit, Vec::new()),
                                                    vec![arm(
                                                        Shape::Unit,
                                                        Vec::new(),
                                                        classify(build(
                                                            Shape::Error,
                                                            result_t.clone(),
                                                            vec![string("failed")],
                                                        )),
                                                    )],
                                                ),
                                            ],
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ),
                ),
            )],
        ),
        0,
        vec![natv(8)],
        300,
    ));
    out.push(case(
        "pair-and-list-match",
        program(
            Vec::new(),
            vec![function(
                vec![pair_t(list_t(nat_t()), opt_t(nat_t()))],
                nat_t(),
                matching(
                    nat_t(),
                    v(0),
                    vec![arm(
                        Shape::Pair,
                        vec![1, 2],
                        matching(
                            nat_t(),
                            v(1),
                            vec![
                                arm(Shape::Nil, Vec::new(), nat(0)),
                                arm(
                                    Shape::Cons,
                                    vec![3, 4],
                                    matching(
                                        nat_t(),
                                        v(2),
                                        vec![
                                            arm(Shape::None, Vec::new(), v(3)),
                                            arm(
                                                Shape::Some,
                                                vec![5],
                                                p(Prim::NatAdd, vec![v(3), v(5)]),
                                            ),
                                        ],
                                    ),
                                ),
                            ],
                        ),
                    )],
                ),
            )],
        ),
        0,
        vec![pairv(
            listv(vec![natv(4), natv(9)]),
            Value::Some {
                value: Box::new(natv(30)),
            },
        )],
        200,
    ));

    // Every literal form, gathered in one record.
    let all_types = vec![
        Ty::Unit,
        Ty::Bool,
        nat_t(),
        Ty::Int,
        fixed_t(U8),
        fixed_t(U16),
        fixed_t(U32),
        fixed_t(U64),
        fixed_t(I8),
        fixed_t(I16),
        fixed_t(I32),
        fixed_t(I64),
        Ty::String,
        Ty::Bytes,
        Ty::Ordering,
        opt_t(nat_t()),
        opt_t(Ty::Bool),
        res_t(nat_t(), Ty::String),
        res_t(nat_t(), Ty::String),
        list_t(nat_t()),
        pair_t(nat_t(), Ty::String),
        Ty::Adt { index: 1 },
    ];
    let all_values = vec![
        Value::Unit,
        Value::Bool { value: true },
        natv(u64::MAX),
        intv(i64::MIN),
        fxv(U8, 255),
        fxv(U16, 65_535),
        fxv(U32, 4_294_967_295),
        fxv(U64, i128::from(u64::MAX)),
        fxv(I8, -128),
        fxv(I16, -32_768),
        fxv(I32, -2_147_483_648),
        fxv(I64, i128::from(i64::MIN)),
        strv("snow ☃"),
        Value::Bytes {
            hex: "00ff7f".to_owned(),
        },
        Value::Ordering {
            value: OrderingValue::Eq,
        },
        Value::None,
        Value::Some {
            value: Box::new(Value::Bool { value: false }),
        },
        Value::Ok {
            value: Box::new(natv(1)),
        },
        Value::Error {
            value: Box::new(strv("no")),
        },
        listv(vec![natv(1), natv(2)]),
        pairv(natv(3), strv("three")),
        Value::Adt {
            constructor: 1,
            fields: vec![natv(5)],
        },
    ];
    out.push(case(
        "literals",
        program(
            vec![vec![all_types.clone()], vec![Vec::new(), vec![nat_t()]]],
            vec![function(
                Vec::new(),
                Ty::Adt { index: 0 },
                build(
                    Shape::Adt { constructor: 0 },
                    Ty::Adt { index: 0 },
                    all_types
                        .into_iter()
                        .zip(all_values)
                        .map(|(ty, value)| lit(ty, value))
                        .collect(),
                ),
            )],
        ),
        0,
        Vec::new(),
        200,
    ));
    out.extend(renderer_cases());
    out
}

/// A right-nested pair of `items`, and its type.
fn tuple_of(mut items: Vec<(Expr, Ty)>) -> (Expr, Ty) {
    let (mut value, mut ty) = items.pop().expect("an item");
    while let Some((left, left_ty)) = items.pop() {
        let both = pair_t(left_ty, ty);
        value = build(Shape::Pair, both.clone(), vec![left, value]);
        ty = both;
    }
    (value, ty)
}

/// Fixtures for the shapes of the Rust renderings (§17.16): every kind of
/// closure dispatch, an uninhabited function type, a boxed field read, a
/// string that needs escaping, and each program shape whose rendering Rust's
/// lint gate constrains.
#[allow(clippy::too_many_lines)]
fn renderer_cases() -> Vec<Case> {
    let mut out = Vec::new();
    let unit = || build(Shape::Unit, Ty::Unit, Vec::new());
    let some = |ty: Ty, value: Expr| build(Shape::Some, opt_t(ty), vec![value]);
    let none = |ty: Ty| build(Shape::None, opt_t(ty), Vec::new());
    let nat_fn = || fn_t(vec![nat_t()], nat_t());

    // A function type inhabited by two closures: one capturing a copied
    // number and a cloned string, whose function can overflow, and one
    // capturing a unit, whose function cannot.
    out.push(case(
        "closure-dispatch",
        program(
            Vec::new(),
            vec![
                function(
                    vec![nat_t(), Ty::String],
                    pair_t(nat_t(), nat_t()),
                    build(
                        Shape::Pair,
                        pair_t(nat_t(), nat_t()),
                        vec![
                            call(3, vec![closure(1, vec![v(0), v(1)]), nat(10)]),
                            call(3, vec![closure(2, vec![unit()]), v(0)]),
                        ],
                    ),
                ),
                function(
                    vec![nat_t(), Ty::String, nat_t()],
                    nat_t(),
                    p(
                        Prim::NatAdd,
                        vec![
                            p(Prim::NatAdd, vec![v(0), p(Prim::Length, vec![v(1)])]),
                            v(2),
                        ],
                    ),
                ),
                function(
                    vec![Ty::Unit, nat_t()],
                    nat_t(),
                    p(Prim::NatSub, vec![v(1), nat(1)]),
                ),
                function(vec![nat_fn(), nat_t()], nat_t(), apply(v(0), vec![v(1)])),
            ],
        ),
        0,
        vec![natv(7), strv("abc")],
        200,
    ));
    // A closure applied where it is built, so its function type is stated
    // nowhere.
    out.push(case(
        "closure-inline",
        program(
            Vec::new(),
            vec![
                function(
                    vec![nat_t(), nat_t()],
                    nat_t(),
                    apply(closure(1, vec![v(0), v(1)]), vec![nat(100)]),
                ),
                function(
                    vec![nat_t(), nat_t(), nat_t()],
                    nat_t(),
                    p(Prim::NatSub, vec![p(Prim::NatMul, vec![v(2), v(0)]), v(1)]),
                ),
            ],
        ),
        0,
        vec![natv(3), natv(10)],
        100,
    ));
    // A closure capturing a value of its own function type, so the capture
    // is boxed.
    out.push(case(
        "closure-recursive",
        program(
            Vec::new(),
            vec![
                function(
                    vec![nat_t()],
                    nat_t(),
                    let_in(
                        1,
                        nat_fn(),
                        closure(2, Vec::new()),
                        let_in(2, nat_fn(), closure(1, vec![v(1)]), apply(v(2), vec![v(0)])),
                    ),
                ),
                function(
                    vec![nat_fn(), nat_t()],
                    nat_t(),
                    apply(v(0), vec![p(Prim::NatMul, vec![v(1), nat(2)])]),
                ),
                function(vec![nat_t()], nat_t(), p(Prim::NatSub, vec![v(0), nat(1)])),
            ],
        ),
        0,
        vec![natv(5)],
        100,
    ));
    // A function type no closure inhabits: the function taking one is never
    // called, and the arm binding one is never taken.
    out.push(case(
        "closure-uninhabited",
        program(
            Vec::new(),
            vec![
                function(vec![nat_t()], nat_t(), call(2, vec![none(nat_fn()), v(0)])),
                function(vec![nat_fn(), nat_t()], nat_t(), apply(v(0), vec![v(1)])),
                function(
                    vec![opt_t(nat_fn()), nat_t()],
                    nat_t(),
                    matching(
                        nat_t(),
                        v(0),
                        vec![
                            arm(Shape::None, Vec::new(), v(1)),
                            arm(Shape::Some, vec![2], apply(v(2), vec![v(1)])),
                        ],
                    ),
                ),
            ],
        ),
        0,
        vec![natv(4)],
        100,
    ));
    // The field of a record that holds its own type, read through its box.
    let node = Ty::Adt { index: 0 };
    out.push(case(
        "boxed-field",
        program(
            vec![vec![vec![nat_t(), opt_t(node.clone())]]],
            vec![function(
                vec![node.clone()],
                nat_t(),
                matching(
                    nat_t(),
                    field(v(0), 1),
                    vec![
                        arm(Shape::None, Vec::new(), nat(0)),
                        arm(Shape::Some, vec![1], field(v(1), 0)),
                    ],
                ),
            )],
        ),
        0,
        vec![Value::Adt {
            constructor: 0,
            fields: vec![
                natv(1),
                Value::Some {
                    value: Box::new(Value::Adt {
                        constructor: 0,
                        fields: vec![natv(2), Value::None],
                    }),
                },
            ],
        }],
        100,
    ));
    // A string literal of characters Rust writes escaped.
    // Every scalar a LexLean source admits once its semantic data escapes
    // the controls: no DEL, and in NFC, so the combining mark follows a
    // letter it does not compose with.
    let escaped = "quote\" backslash\\ newline\n tab\t nul\u{0} \u{e9} x\u{301} \u{2603} \u{1f600}";
    out.push(case(
        "string-escapes",
        program(
            Vec::new(),
            vec![function(
                vec![nat_t()],
                pair_t(Ty::String, nat_t()),
                build(
                    Shape::Pair,
                    pair_t(Ty::String, nat_t()),
                    vec![
                        string(escaped),
                        p(
                            Prim::NatAdd,
                            vec![p(Prim::Length, vec![string(escaped)]), v(0)],
                        ),
                    ],
                ),
            )],
        ),
        0,
        vec![natv(1)],
        100,
    ));
    // Units: computed, passed, bound, matched, returned by a function that
    // can overflow, and chosen by a conditional without an alternative.
    out.push(case(
        "unit-values",
        program(
            Vec::new(),
            vec![
                function(
                    vec![Ty::Unit, nat_t()],
                    pair_t(Ty::Unit, opt_t(Ty::Unit)),
                    let_in(
                        2,
                        Ty::Unit,
                        call(1, vec![v(0)]),
                        let_in(
                            3,
                            Ty::Unit,
                            call(4, vec![v(1)]),
                            let_in(
                                4,
                                Ty::Unit,
                                call(
                                    5,
                                    vec![
                                        p(Prim::NatLt, vec![v(1), nat(9)]),
                                        p(Prim::NatLt, vec![v(1), nat(2)]),
                                    ],
                                ),
                                build(
                                    Shape::Pair,
                                    pair_t(Ty::Unit, opt_t(Ty::Unit)),
                                    vec![call(1, vec![v(2)]), some(Ty::Unit, call(2, vec![v(1)]))],
                                ),
                            ),
                        ),
                    ),
                ),
                function(
                    vec![Ty::Unit],
                    Ty::Unit,
                    matching(Ty::Unit, v(0), vec![arm(Shape::Unit, Vec::new(), v(0))]),
                ),
                function(
                    vec![nat_t()],
                    Ty::Unit,
                    matching(
                        Ty::Unit,
                        v(0),
                        vec![
                            arm(Shape::Zero, Vec::new(), unit()),
                            arm(Shape::Succ, vec![1], call(3, vec![v(1)])),
                        ],
                    ),
                ),
                function(
                    vec![nat_t()],
                    Ty::Unit,
                    let_in(1, nat_t(), p(Prim::NatAdd, vec![v(0), nat(1)]), unit()),
                ),
                function(
                    vec![nat_t()],
                    Ty::Unit,
                    cond(
                        p(Prim::NatLt, vec![v(0), nat(3)]),
                        call(3, vec![v(0)]),
                        unit(),
                    ),
                ),
                function(
                    vec![Ty::Bool, Ty::Bool],
                    Ty::Unit,
                    cond(v(0), cond(v(1), call(1, vec![unit()]), unit()), unit()),
                ),
            ],
        ),
        0,
        vec![Value::Unit, natv(5)],
        200,
    ));
    // Conditionals and Boolean matches whose rendering Rust's lint gate
    // constrains: literal arms, a negated conditional, equal branches, a
    // chain repeating a branch, and a zero test as a value.
    let (shapes, shapes_t) = tuple_of(vec![
        (
            matching(
                Ty::Bool,
                v(0),
                vec![
                    arm(Shape::True, Vec::new(), boolean(false)),
                    arm(Shape::False, Vec::new(), boolean(true)),
                ],
            ),
            Ty::Bool,
        ),
        (
            matching(
                Ty::Bool,
                v(1),
                vec![
                    arm(Shape::True, Vec::new(), boolean(true)),
                    arm(Shape::False, Vec::new(), boolean(false)),
                ],
            ),
            Ty::Bool,
        ),
        (
            matching(
                Ty::Bool,
                v(2),
                vec![
                    arm(Shape::Zero, Vec::new(), boolean(false)),
                    arm(Shape::Succ, vec![3], boolean(true)),
                ],
            ),
            Ty::Bool,
        ),
        (
            matching(
                Ty::Bool,
                v(2),
                vec![
                    arm(Shape::Zero, Vec::new(), boolean(true)),
                    arm(Shape::Succ, vec![4], boolean(false)),
                ],
            ),
            Ty::Bool,
        ),
        (
            cond(
                cond(v(0), v(1), boolean(false)),
                boolean(false),
                boolean(true),
            ),
            Ty::Bool,
        ),
        (
            cond(p(Prim::NatLt, vec![v(2), nat(3)]), nat(4), nat(4)),
            nat_t(),
        ),
        (cond(v(0), nat(1), cond(v(1), nat(1), nat(2))), nat_t()),
        (
            matching(
                nat_t(),
                v(2),
                vec![
                    arm(Shape::Zero, Vec::new(), nat(7)),
                    arm(Shape::Succ, vec![5], nat(7)),
                ],
            ),
            nat_t(),
        ),
        (
            matching(
                nat_t(),
                v(1),
                vec![
                    arm(Shape::True, Vec::new(), nat(8)),
                    arm(Shape::False, Vec::new(), nat(8)),
                ],
            ),
            nat_t(),
        ),
    ]);
    out.push(case(
        "boolean-shapes",
        program(
            Vec::new(),
            vec![function(
                vec![Ty::Bool, Ty::Bool, nat_t()],
                shapes_t,
                shapes,
            )],
        ),
        0,
        vec![
            Value::Bool { value: true },
            Value::Bool { value: false },
            natv(2),
        ],
        200,
    ));
    // Matches whose arms rebuild what they matched, are all one value, or
    // map one variant: each renders as Rust's lint gate admits.
    let tagged = Ty::Adt { index: 0 };
    let maybe = opt_t(nat_t());
    let outcome = res_t(nat_t(), Ty::Bool);
    let (matched, matched_t) = tuple_of(vec![
        (
            matching(
                maybe.clone(),
                v(0),
                vec![
                    arm(Shape::None, Vec::new(), none(nat_t())),
                    arm(Shape::Some, vec![5], some(nat_t(), v(5))),
                ],
            ),
            maybe.clone(),
        ),
        (
            matching(
                outcome.clone(),
                v(1),
                vec![
                    arm(
                        Shape::Ok,
                        vec![6],
                        build(Shape::Ok, outcome.clone(), vec![v(6)]),
                    ),
                    arm(
                        Shape::Error,
                        vec![7],
                        build(Shape::Error, outcome.clone(), vec![v(7)]),
                    ),
                ],
            ),
            outcome.clone(),
        ),
        (
            matching(
                tagged.clone(),
                v(2),
                vec![
                    arm(
                        Shape::Adt { constructor: 0 },
                        vec![8],
                        build(Shape::Adt { constructor: 0 }, tagged.clone(), vec![v(8)]),
                    ),
                    arm(
                        Shape::Adt { constructor: 1 },
                        vec![9],
                        build(Shape::Adt { constructor: 1 }, tagged.clone(), vec![v(9)]),
                    ),
                ],
            ),
            tagged.clone(),
        ),
        (
            matching(
                Ty::Ordering,
                p(Prim::Compare, vec![v(3), v(4)]),
                vec![
                    arm(
                        Shape::Lt,
                        Vec::new(),
                        build(Shape::Lt, Ty::Ordering, Vec::new()),
                    ),
                    arm(
                        Shape::Eq,
                        Vec::new(),
                        build(Shape::Eq, Ty::Ordering, Vec::new()),
                    ),
                    arm(
                        Shape::Gt,
                        Vec::new(),
                        build(Shape::Gt, Ty::Ordering, Vec::new()),
                    ),
                ],
            ),
            Ty::Ordering,
        ),
        (
            matching(
                maybe.clone(),
                v(0),
                vec![
                    arm(Shape::None, Vec::new(), none(nat_t())),
                    arm(
                        Shape::Some,
                        vec![10],
                        some(nat_t(), p(Prim::NatSub, vec![v(10), nat(1)])),
                    ),
                ],
            ),
            maybe.clone(),
        ),
        (
            matching(
                maybe.clone(),
                v(1),
                vec![
                    arm(Shape::Ok, vec![11], some(nat_t(), v(11))),
                    arm(Shape::Error, vec![12], none(nat_t())),
                ],
            ),
            maybe.clone(),
        ),
        (
            matching(
                nat_t(),
                v(0),
                vec![
                    arm(Shape::None, Vec::new(), nat(5)),
                    arm(Shape::Some, vec![13], nat(5)),
                ],
            ),
            nat_t(),
        ),
        (call(1, vec![v(0)]), Ty::Unit),
    ]);
    out.push(case(
        "match-shapes",
        program(
            vec![vec![vec![nat_t()], vec![Ty::Bool]]],
            vec![
                function(
                    vec![maybe.clone(), outcome, tagged, nat_t(), nat_t()],
                    matched_t,
                    matched,
                ),
                function(
                    vec![maybe],
                    Ty::Unit,
                    matching(
                        Ty::Unit,
                        v(0),
                        vec![
                            arm(Shape::None, Vec::new(), unit()),
                            arm(Shape::Some, vec![1], call(2, vec![v(1)])),
                        ],
                    ),
                ),
                function(
                    vec![nat_t()],
                    Ty::Unit,
                    let_in(1, Ty::Bool, p(Prim::NatEq, vec![v(0), nat(1)]), unit()),
                ),
            ],
        ),
        0,
        vec![
            Value::Some {
                value: Box::new(natv(3)),
            },
            Value::Ok {
                value: Box::new(natv(4)),
            },
            Value::Adt {
                constructor: 1,
                fields: vec![Value::Bool { value: true }],
            },
            natv(2),
            natv(5),
        ],
        200,
    ));
    // Units through a function type whose closures differ in failure, as
    // an argument computed by a call that cannot fail, and as the value of
    // a function that can.
    let unit_fn = || fn_t(vec![nat_t()], Ty::Unit);
    out.push(case(
        "unit-closures",
        program(
            Vec::new(),
            vec![
                function(
                    vec![nat_t()],
                    pair_t(
                        opt_t(Ty::Unit),
                        pair_t(Ty::Unit, pair_t(Ty::Unit, Ty::Unit)),
                    ),
                    build(
                        Shape::Pair,
                        pair_t(
                            opt_t(Ty::Unit),
                            pair_t(Ty::Unit, pair_t(Ty::Unit, Ty::Unit)),
                        ),
                        vec![
                            some(Ty::Unit, call(2, vec![v(0)])),
                            build(
                                Shape::Pair,
                                pair_t(Ty::Unit, pair_t(Ty::Unit, Ty::Unit)),
                                vec![
                                    call(3, vec![closure(1, Vec::new()), v(0)]),
                                    build(
                                        Shape::Pair,
                                        pair_t(Ty::Unit, Ty::Unit),
                                        vec![
                                            call(3, vec![closure(2, Vec::new()), v(0)]),
                                            call(4, vec![v(0)]),
                                        ],
                                    ),
                                ],
                            ),
                        ],
                    ),
                ),
                function(
                    vec![nat_t()],
                    Ty::Unit,
                    let_in(1, nat_t(), p(Prim::NatAdd, vec![v(0), nat(1)]), unit()),
                ),
                function(
                    vec![nat_t()],
                    Ty::Unit,
                    let_in(1, Ty::Bool, p(Prim::NatEq, vec![v(0), nat(1)]), unit()),
                ),
                function(vec![unit_fn(), nat_t()], Ty::Unit, apply(v(0), vec![v(1)])),
                function(
                    vec![nat_t()],
                    Ty::Unit,
                    let_in(
                        1,
                        nat_t(),
                        p(Prim::NatAdd, vec![v(0), nat(2)]),
                        call(2, vec![v(0)]),
                    ),
                ),
            ],
        ),
        0,
        vec![natv(5)],
        200,
    ));
    // Each shape whose rendering Rust's lint gate constrains as the value of
    // a function: a match rebuilding its scrutinee, mapping one variant,
    // defaulting one, returning its binding, a chain through a literal
    // condition, a field of a computed record, and a unit match with an
    // empty arm.
    let swapped = Ty::Adt { index: 0 };
    let maybe_nat = opt_t(nat_t());
    let checked = res_t(nat_t(), Ty::Bool);
    let shaped: Vec<(Ty, Ty, Expr)> = vec![
        (
            maybe_nat.clone(),
            maybe_nat.clone(),
            matching(
                maybe_nat.clone(),
                v(0),
                vec![
                    arm(Shape::None, Vec::new(), none(nat_t())),
                    arm(Shape::Some, vec![1], some(nat_t(), v(1))),
                ],
            ),
        ),
        (
            maybe_nat.clone(),
            maybe_nat.clone(),
            matching(
                maybe_nat.clone(),
                v(0),
                vec![
                    arm(Shape::None, Vec::new(), none(nat_t())),
                    arm(
                        Shape::Some,
                        vec![1],
                        some(nat_t(), p(Prim::NatSub, vec![v(1), nat(1)])),
                    ),
                ],
            ),
        ),
        (
            checked.clone(),
            maybe_nat.clone(),
            matching(
                maybe_nat.clone(),
                v(0),
                vec![
                    arm(Shape::Ok, vec![1], some(nat_t(), v(1))),
                    arm(Shape::Error, vec![2], none(nat_t())),
                ],
            ),
        ),
        (
            maybe_nat.clone(),
            nat_t(),
            matching(
                nat_t(),
                v(0),
                vec![
                    arm(Shape::None, Vec::new(), nat(0)),
                    arm(Shape::Some, vec![1], v(1)),
                ],
            ),
        ),
        (
            maybe_nat.clone(),
            nat_t(),
            matching(
                nat_t(),
                v(0),
                vec![
                    arm(Shape::None, Vec::new(), nat(7)),
                    arm(Shape::Some, vec![1], v(1)),
                ],
            ),
        ),
        (
            nat_t(),
            nat_t(),
            matching(
                nat_t(),
                v(0),
                vec![
                    arm(Shape::Zero, Vec::new(), nat(0)),
                    arm(Shape::Succ, vec![1], v(1)),
                ],
            ),
        ),
        (
            Ty::Bool,
            nat_t(),
            cond(v(0), nat(200), cond(boolean(false), nat(1), nat(200))),
        ),
        (
            swapped.clone(),
            nat_t(),
            field(
                matching(
                    swapped.clone(),
                    v(0),
                    vec![arm(
                        Shape::Adt { constructor: 0 },
                        vec![1, 2],
                        build(
                            Shape::Adt { constructor: 0 },
                            swapped.clone(),
                            vec![v(2), v(1)],
                        ),
                    )],
                ),
                0,
            ),
        ),
        (
            maybe_nat.clone(),
            Ty::Unit,
            matching(
                Ty::Unit,
                v(0),
                vec![
                    arm(Shape::None, Vec::new(), call(10, vec![nat(1)])),
                    arm(Shape::Some, vec![1], unit()),
                ],
            ),
        ),
    ];
    let shaped_arguments = [
        Value::Some {
            value: Box::new(natv(3)),
        },
        Value::Some {
            value: Box::new(natv(4)),
        },
        Value::Ok {
            value: Box::new(natv(5)),
        },
        Value::None,
        Value::Some {
            value: Box::new(natv(6)),
        },
        natv(9),
        Value::Bool { value: false },
        Value::Adt {
            constructor: 0,
            fields: vec![natv(1), natv(2)],
        },
        Value::Some {
            value: Box::new(natv(1)),
        },
    ];
    let (results, results_t) = tuple_of(
        shaped
            .iter()
            .zip(&shaped_arguments)
            .enumerate()
            .map(|(at, ((parameter, result, _), argument))| {
                (
                    call(
                        at as u64 + 1,
                        vec![lit(parameter.clone(), argument.clone())],
                    ),
                    result.clone(),
                )
            })
            .collect(),
    );
    let mut functions = vec![function(Vec::new(), results_t, results)];
    functions.extend(
        shaped
            .into_iter()
            .map(|(parameter, result, body)| function(vec![parameter], result, body)),
    );
    functions.push(function(
        vec![nat_t()],
        Ty::Unit,
        let_in(1, Ty::Bool, p(Prim::NatEq, vec![v(0), nat(1)]), unit()),
    ));
    out.push(case(
        "function-shapes",
        program(vec![vec![vec![nat_t(), nat_t()]]], functions),
        0,
        Vec::new(),
        400,
    ));
    // A function of eight parameters, an enum one of whose variants is
    // large, and results whose error is that enum or a unit.
    let large = Ty::Adt { index: 0 };
    let eight: Vec<Ty> = vec![nat_t(); 8];
    out.push(case(
        "wide-shapes",
        program(
            vec![vec![Vec::new(), vec![nat_t(); 40]]],
            vec![
                function(
                    vec![nat_t()],
                    pair_t(
                        nat_t(),
                        pair_t(res_t(nat_t(), large.clone()), res_t(nat_t(), Ty::Unit)),
                    ),
                    build(
                        Shape::Pair,
                        pair_t(
                            nat_t(),
                            pair_t(res_t(nat_t(), large.clone()), res_t(nat_t(), Ty::Unit)),
                        ),
                        vec![
                            call(
                                1,
                                (0..8)
                                    .map(|at| if at == 0 { v(0) } else { nat(at) })
                                    .collect(),
                            ),
                            build(
                                Shape::Pair,
                                pair_t(res_t(nat_t(), large.clone()), res_t(nat_t(), Ty::Unit)),
                                vec![call(2, vec![v(0)]), call(3, vec![v(0)])],
                            ),
                        ],
                    ),
                ),
                function(eight, nat_t(), p(Prim::NatSub, vec![v(7), v(0)])),
                function(
                    vec![nat_t()],
                    res_t(nat_t(), large.clone()),
                    cond(
                        p(Prim::NatLt, vec![v(0), nat(5)]),
                        build(Shape::Ok, res_t(nat_t(), large.clone()), vec![v(0)]),
                        build(
                            Shape::Error,
                            res_t(nat_t(), large.clone()),
                            vec![build(Shape::Adt { constructor: 0 }, large, Vec::new())],
                        ),
                    ),
                ),
                function(
                    vec![nat_t()],
                    res_t(nat_t(), Ty::Unit),
                    cond(
                        p(Prim::NatLt, vec![v(0), nat(1)]),
                        build(Shape::Ok, res_t(nat_t(), Ty::Unit), vec![v(0)]),
                        build(Shape::Error, res_t(nat_t(), Ty::Unit), vec![unit()]),
                    ),
                ),
            ],
        ),
        0,
        vec![natv(3)],
        200,
    ));
    out
}

// --- library fixtures ------------------------------------------------------

/// A library fixture: `functions` come first (the entry is function 0), the
/// template instance follows them.
fn library_case(
    name: &str,
    template: Template,
    types: Vec<Ty>,
    functions: Vec<Function>,
    arguments: Vec<Value>,
    oracle: Json,
) -> Case {
    let at = functions.len() as u64;
    let mut all = functions;
    all.extend(
        template
            .instantiate(&types, at)
            .unwrap_or_else(|reason| panic!("fixture {name}: {reason}")),
    );
    let mut out = case(name, program(Vec::new(), all), 0, arguments, 4000);
    out.library = Some(LibraryUse {
        template,
        types,
        at,
    });
    out.oracle = Some(oracle);
    out
}

/// The entry of a library fixture: pass every parameter to the instance.
fn forward(types: Vec<Ty>, result: Ty, at: u64) -> Function {
    let operands = (0..types.len() as u64).map(v).collect();
    function(types, result, call(at, operands))
}

fn nat_string_map(entries: &[(u64, &str)]) -> (Value, Json) {
    let target = listv(
        entries
            .iter()
            .map(|(key, value)| pairv(natv(*key), strv(value)))
            .collect(),
    );
    let source = lx::map_literal(
        lx::nat_t(),
        lx::string_t(),
        entries
            .iter()
            .map(|(key, value)| (lx::nat(*key), lx::string(value)))
            .collect(),
    );
    (target, source)
}

fn nat_set(elements: &[u64]) -> (Value, Json) {
    (
        listv(elements.iter().map(|element| natv(*element)).collect()),
        lx::set_literal(
            lx::nat_t(),
            elements.iter().map(|element| lx::nat(*element)).collect(),
        ),
    )
}

fn nat_graph(nodes: &[u64], edges: &[(u64, u64)]) -> (Value, Json) {
    let target = listv(
        nodes
            .iter()
            .map(|node| {
                let mut successors: Vec<u64> = edges
                    .iter()
                    .filter(|(source, _)| source == node)
                    .map(|(_, target)| *target)
                    .collect();
                successors.sort_unstable();
                pairv(
                    natv(*node),
                    listv(successors.into_iter().map(natv).collect()),
                )
            })
            .collect(),
    );
    (target, lx::graph_literal(nodes, edges))
}

#[allow(clippy::too_many_lines)]
fn library_cases() -> Vec<Case> {
    let mut out = Vec::new();
    let string = Ty::String;
    let entry = pair_t(nat_t(), string.clone());
    let entries = list_t(entry);
    let nats = list_t(nat_t());
    let map_nat_string = || lx::map_t(lx::nat_t(), lx::string_t());
    let encode_map = |map: Json| {
        lx::encode(
            "encodeNatStrings",
            lx::prim(
                "map_entries",
                vec![map],
                lx::list_t(lx::product_t(lx::nat_t(), lx::string_t())),
            ),
        )
    };
    let encode_set = |set: Json| {
        lx::encode(
            "encodeNats",
            lx::prim("set_elements", vec![set], lx::list_t(lx::nat_t())),
        )
    };

    let (map, map_source) = nat_string_map(&[(1, "a"), (3, "c")]);
    for (name, key, value) in [
        ("map-insert", 2, "b"),
        ("map-insert-replace", 3, "z"),
        ("map-insert-last", 9, "i"),
    ] {
        out.push(library_case(
            name,
            Template::MapInsert,
            vec![nat_t(), string.clone()],
            vec![forward(
                vec![entries.clone(), nat_t(), string.clone()],
                entries.clone(),
                1,
            )],
            vec![map.clone(), natv(key), strv(value)],
            encode_map(lx::prim(
                "map_insert",
                vec![map_source.clone(), lx::nat(key), lx::string(value)],
                map_nat_string(),
            )),
        ));
    }
    for (name, key) in [("map-remove", 1), ("map-remove-absent", 2)] {
        out.push(library_case(
            name,
            Template::MapRemove,
            vec![nat_t(), string.clone()],
            vec![forward(vec![entries.clone(), nat_t()], entries.clone(), 1)],
            vec![map.clone(), natv(key)],
            encode_map(lx::prim(
                "map_remove",
                vec![map_source.clone(), lx::nat(key)],
                map_nat_string(),
            )),
        ));
    }
    for (name, key) in [("map-lookup", 3), ("map-lookup-absent", 2)] {
        out.push(library_case(
            name,
            Template::MapLookup,
            vec![nat_t(), string.clone()],
            vec![forward(
                vec![entries.clone(), nat_t()],
                opt_t(string.clone()),
                1,
            )],
            vec![map.clone(), natv(key)],
            lx::encode_option(
                lx::prim(
                    "map_lookup",
                    vec![map_source.clone(), lx::nat(key)],
                    lx::option_t(lx::string_t()),
                ),
                |found| lx::value("string", vec![found]),
            ),
        ));
    }
    out.push(library_case(
        "map-contains",
        Template::MapContains,
        vec![nat_t(), string.clone()],
        vec![forward(vec![entries.clone(), nat_t()], Ty::Bool, 1)],
        vec![map.clone(), natv(3)],
        lx::value(
            "bool",
            vec![lx::prim(
                "map_contains",
                vec![map_source.clone(), lx::nat(3)],
                lx::bool_t(),
            )],
        ),
    ));
    out.push(library_case(
        "map-keys",
        Template::MapKeys,
        vec![nat_t(), string.clone()],
        vec![forward(vec![entries.clone()], nats.clone(), 1)],
        vec![map.clone()],
        lx::encode(
            "encodeNats",
            lx::prim(
                "map_keys",
                vec![map_source.clone()],
                lx::list_t(lx::nat_t()),
            ),
        ),
    ));
    out.push(library_case(
        "map-values",
        Template::MapValues,
        vec![nat_t(), string.clone()],
        vec![forward(vec![entries.clone()], list_t(string.clone()), 1)],
        vec![map.clone()],
        lx::encode(
            "encodeStrings",
            lx::prim(
                "map_values",
                vec![map_source.clone()],
                lx::list_t(lx::string_t()),
            ),
        ),
    ));
    // map_fold with the step `state + key + length value`.
    out.push(library_case(
        "map-fold",
        Template::MapFold,
        vec![nat_t(), string.clone(), nat_t()],
        vec![
            function(
                vec![entries.clone()],
                nat_t(),
                call(2, vec![closure(1, Vec::new()), nat(100), v(0)]),
            ),
            function(
                vec![nat_t(), nat_t(), string.clone()],
                nat_t(),
                p(
                    Prim::NatAdd,
                    vec![
                        p(Prim::NatAdd, vec![v(0), v(1)]),
                        p(Prim::Length, vec![v(2)]),
                    ],
                ),
            ),
        ],
        vec![map.clone()],
        lx::value(
            "nat",
            vec![lx::prim(
                "map_fold",
                vec![
                    lx::lambda(
                        &[
                            ("state", lx::nat_t()),
                            ("key", lx::nat_t()),
                            ("text", lx::string_t()),
                        ],
                        lx::add(
                            lx::add(lx::var("state"), lx::var("key")),
                            lx::prim("length", vec![lx::var("text")], lx::nat_t()),
                        ),
                    ),
                    lx::nat(100),
                    map_source.clone(),
                ],
                lx::nat_t(),
            )],
        ),
    ));

    // Insertion at every key type: the realization's `compare` must agree
    // with LexLean's own key order.
    let insert_case = |name: &str, key: Ty, elements: Vec<Value>, inserted: Value, oracle: Json| {
        let set = list_t(key.clone());
        library_case(
            name,
            Template::SetInsert,
            vec![key.clone()],
            vec![forward(vec![set.clone(), key], set, 1)],
            vec![listv(elements), inserted],
            oracle,
        )
    };
    let (set, set_source) = nat_set(&[2, 4, 8]);
    out.push(insert_case(
        "set-insert-nat",
        nat_t(),
        vec![natv(2), natv(4), natv(8)],
        natv(5),
        encode_set(lx::prim(
            "set_insert",
            vec![set_source.clone(), lx::nat(5)],
            lx::set_t(lx::nat_t()),
        )),
    ));
    out.push(insert_case(
        "set-insert-int",
        Ty::Int,
        vec![intv(-5), intv(3)],
        intv(-7),
        lx::encode(
            "encodeInts",
            lx::prim(
                "set_elements",
                vec![lx::prim(
                    "set_insert",
                    vec![
                        lx::set_literal(
                            lx::int_t(),
                            vec![lx::integer("int", -5), lx::integer("int", 3)],
                        ),
                        lx::integer("int", -7),
                    ],
                    lx::set_t(lx::int_t()),
                )],
                lx::list_t(lx::int_t()),
            ),
        ),
    ));
    out.push(insert_case(
        "set-insert-int8",
        fixed_t(IntKind::I8),
        vec![fxv(IntKind::I8, -128), fxv(IntKind::I8, 5)],
        fxv(IntKind::I8, -1),
        lx::encode(
            "encodeI8s",
            lx::prim(
                "set_elements",
                vec![lx::prim(
                    "set_insert",
                    vec![
                        lx::set_literal(
                            lx::int8_t(),
                            vec![lx::integer("int8", -128), lx::integer("int8", 5)],
                        ),
                        lx::integer("int8", -1),
                    ],
                    lx::set_t(lx::int8_t()),
                )],
                lx::list_t(lx::int8_t()),
            ),
        ),
    ));
    out.push(insert_case(
        "set-insert-bool",
        Ty::Bool,
        vec![Value::Bool { value: true }],
        Value::Bool { value: false },
        lx::encode(
            "encodeBools",
            lx::prim(
                "set_elements",
                vec![lx::prim(
                    "set_insert",
                    vec![
                        lx::set_literal(lx::bool_t(), vec![lx::boolean(true)]),
                        lx::boolean(false),
                    ],
                    lx::set_t(lx::bool_t()),
                )],
                lx::list_t(lx::bool_t()),
            ),
        ),
    ));
    out.push(insert_case(
        "set-insert-string",
        string.clone(),
        vec![strv("b"), strv("é")],
        strv("z"),
        lx::encode(
            "encodeStrings",
            lx::prim(
                "set_elements",
                vec![lx::prim(
                    "set_insert",
                    vec![
                        lx::set_literal(lx::string_t(), vec![lx::string("b"), lx::string("é")]),
                        lx::string("z"),
                    ],
                    lx::set_t(lx::string_t()),
                )],
                lx::list_t(lx::string_t()),
            ),
        ),
    ));
    let product = lx::product_t(lx::nat_t(), lx::string_t());
    out.push(insert_case(
        "set-insert-product",
        pair_t(nat_t(), string.clone()),
        vec![pairv(natv(1), strv("b")), pairv(natv(2), strv("a"))],
        pairv(natv(1), strv("a")),
        lx::encode(
            "encodeNatStrings",
            lx::prim(
                "set_elements",
                vec![lx::prim(
                    "set_insert",
                    vec![
                        lx::set_literal(
                            product.clone(),
                            vec![
                                lx::pair(lx::nat(1), lx::string("b")),
                                lx::pair(lx::nat(2), lx::string("a")),
                            ],
                        ),
                        lx::pair(lx::nat(1), lx::string("a")),
                    ],
                    lx::set_t(product.clone()),
                )],
                lx::list_t(product),
            ),
        ),
    ));

    for (name, element) in [("set-remove", 4), ("set-remove-absent", 5)] {
        out.push(library_case(
            name,
            Template::SetRemove,
            vec![nat_t()],
            vec![forward(vec![nats.clone(), nat_t()], nats.clone(), 1)],
            vec![set.clone(), natv(element)],
            encode_set(lx::prim(
                "set_remove",
                vec![set_source.clone(), lx::nat(element)],
                lx::set_t(lx::nat_t()),
            )),
        ));
    }
    for (name, element) in [("set-contains", 8), ("set-contains-absent", 3)] {
        out.push(library_case(
            name,
            Template::SetContains,
            vec![nat_t()],
            vec![forward(vec![nats.clone(), nat_t()], Ty::Bool, 1)],
            vec![set.clone(), natv(element)],
            lx::value(
                "bool",
                vec![lx::prim(
                    "set_contains",
                    vec![set_source.clone(), lx::nat(element)],
                    lx::bool_t(),
                )],
            ),
        ));
    }
    let (other, other_source) = nat_set(&[1, 4, 9]);
    for (name, template, operation) in [
        ("set-union", Template::SetUnion, "set_union"),
        (
            "set-intersection",
            Template::SetIntersection,
            "set_intersection",
        ),
        ("set-difference", Template::SetDifference, "set_difference"),
    ] {
        out.push(library_case(
            name,
            template,
            vec![nat_t()],
            vec![forward(vec![nats.clone(), nats.clone()], nats.clone(), 1)],
            vec![set.clone(), other.clone()],
            encode_set(lx::prim(
                operation,
                vec![set_source.clone(), other_source.clone()],
                lx::set_t(lx::nat_t()),
            )),
        ));
    }
    // set_fold with the step `state * 10 + element`.
    out.push(library_case(
        "set-fold",
        Template::SetFold,
        vec![nat_t(), nat_t()],
        vec![
            function(
                vec![nats.clone()],
                nat_t(),
                call(2, vec![closure(1, Vec::new()), nat(0), v(0)]),
            ),
            function(
                vec![nat_t(), nat_t()],
                nat_t(),
                p(
                    Prim::NatAdd,
                    vec![p(Prim::NatMul, vec![v(0), nat(10)]), v(1)],
                ),
            ),
        ],
        vec![set.clone()],
        lx::value(
            "nat",
            vec![lx::prim(
                "set_fold",
                vec![
                    lx::lambda(
                        &[("state", lx::nat_t()), ("element", lx::nat_t())],
                        lx::add(
                            lx::prim("multiply", vec![lx::var("state"), lx::nat(10)], lx::nat_t()),
                            lx::var("element"),
                        ),
                    ),
                    lx::nat(0),
                    set_source.clone(),
                ],
                lx::nat_t(),
            )],
        ),
    ));
    // list_fold with a captured weight: `state + weight * length text`.
    out.push(library_case(
        "list-fold",
        Template::ListFold,
        vec![string.clone(), nat_t()],
        vec![
            function(
                vec![list_t(string.clone())],
                nat_t(),
                call(2, vec![closure(1, vec![nat(3)]), nat(1), v(0)]),
            ),
            function(
                vec![nat_t(), nat_t(), string.clone()],
                nat_t(),
                p(
                    Prim::NatAdd,
                    vec![
                        v(1),
                        p(Prim::NatMul, vec![v(0), p(Prim::Length, vec![v(2)])]),
                    ],
                ),
            ),
        ],
        vec![listv(vec![strv("ab"), strv("é"), strv("")])],
        lx::value(
            "nat",
            vec![lx::prim(
                "list_fold",
                vec![
                    lx::lambda(
                        &[("state", lx::nat_t()), ("text", lx::string_t())],
                        lx::add(
                            lx::var("state"),
                            lx::prim(
                                "multiply",
                                vec![
                                    lx::nat(3),
                                    lx::prim("length", vec![lx::var("text")], lx::nat_t()),
                                ],
                                lx::nat_t(),
                            ),
                        ),
                    ),
                    lx::nat(1),
                    lx::list(
                        lx::string_t(),
                        vec![lx::string("ab"), lx::string("é"), lx::string("")],
                    ),
                ],
                lx::nat_t(),
            )],
        ),
    ));
    out.push(library_case(
        "iterate",
        Template::Iterate,
        vec![nat_t()],
        vec![
            function(
                vec![nat_t()],
                nat_t(),
                call(2, vec![closure(1, Vec::new()), v(0), nat(1)]),
            ),
            function(vec![nat_t()], nat_t(), p(Prim::NatAdd, vec![v(0), nat(3)])),
        ],
        vec![natv(4)],
        lx::value(
            "nat",
            vec![lx::prim(
                "iterate",
                vec![
                    lx::lambda(
                        &[("value", lx::nat_t())],
                        lx::add(lx::var("value"), lx::nat(3)),
                    ),
                    lx::nat(4),
                    lx::nat(1),
                ],
                lx::nat_t(),
            )],
        ),
    ));
    for (name, bound) in [("iterate-until", 10), ("iterate-until-bounded", 2)] {
        let until = pair_t(nat_t(), Ty::Bool);
        out.push(library_case(
            name,
            Template::IterateUntil,
            vec![nat_t()],
            vec![
                function(
                    vec![nat_t()],
                    until,
                    call(2, vec![closure(1, Vec::new()), v(0), nat(0)]),
                ),
                function(
                    vec![nat_t()],
                    opt_t(nat_t()),
                    cond(
                        p(Prim::NatLt, vec![v(0), nat(3)]),
                        build(
                            Shape::Some,
                            opt_t(nat_t()),
                            vec![p(Prim::NatAdd, vec![v(0), nat(1)])],
                        ),
                        build(Shape::None, opt_t(nat_t()), Vec::new()),
                    ),
                ),
            ],
            vec![natv(bound)],
            {
                let result = lx::prim(
                    "iterate_until",
                    vec![
                        lx::lambda(
                            &[("value", lx::nat_t())],
                            lx::ite(
                                lx::blt(lx::var("value"), lx::nat(3)),
                                lx::some(lx::nat_t(), lx::add(lx::var("value"), lx::nat(1))),
                                lx::none(lx::nat_t()),
                            ),
                        ),
                        lx::nat(bound),
                        lx::nat(0),
                    ],
                    lx::product_t(lx::nat_t(), lx::bool_t()),
                );
                lx::value(
                    "pair",
                    vec![
                        lx::value("nat", vec![lx::first(result.clone())]),
                        lx::value("bool", vec![lx::second(result)]),
                    ],
                )
            },
        ));
    }

    let adjacency = list_t(pair_t(nat_t(), nats.clone()));
    let (dag, dag_source) = nat_graph(&[1, 2, 3, 4], &[(1, 2), (1, 3), (2, 4), (3, 4)]);
    let (cycle, cycle_source) = nat_graph(&[1, 2, 3], &[(1, 2), (2, 3), (3, 2)]);
    for (name, node) in [("graph-successors", 1), ("graph-successors-absent", 9)] {
        out.push(library_case(
            name,
            Template::GraphSuccessors,
            vec![nat_t()],
            vec![forward(vec![adjacency.clone(), nat_t()], nats.clone(), 1)],
            vec![dag.clone(), natv(node)],
            encode_set(lx::prim(
                "graph_successors",
                vec![dag_source.clone(), lx::nat(node)],
                lx::set_t(lx::nat_t()),
            )),
        ));
    }
    for (name, node) in [
        ("graph-reachable", 1),
        ("graph-reachable-inner", 2),
        ("graph-reachable-absent", 7),
    ] {
        out.push(library_case(
            name,
            Template::GraphReachable,
            vec![nat_t()],
            vec![forward(vec![adjacency.clone(), nat_t()], nats.clone(), 1)],
            vec![dag.clone(), natv(node)],
            encode_set(lx::prim(
                "graph_reachable",
                vec![dag_source.clone(), lx::nat(node)],
                lx::set_t(lx::nat_t()),
            )),
        ));
    }
    for (name, graph, source) in [
        ("graph-topological", dag.clone(), dag_source.clone()),
        ("graph-topological-cycle", cycle, cycle_source),
    ] {
        out.push(library_case(
            name,
            Template::GraphTopological,
            vec![nat_t()],
            vec![forward(vec![adjacency.clone()], opt_t(nats.clone()), 1)],
            vec![graph],
            lx::encode_option(
                lx::prim(
                    "graph_topological",
                    vec![source],
                    lx::option_t(lx::list_t(lx::nat_t())),
                ),
                |order| lx::encode("encodeNats", order),
            ),
        ));
    }
    out
}

/// Every fixture, sorted by name.
#[must_use]
pub fn cases() -> Vec<Case> {
    let mut out = core_cases();
    out.extend(library_cases());
    out.sort_by(|left, right| left.fixture.name.cmp(&right.fixture.name));
    out
}

/// The axioms Lean reports for every declaration that reaches the evaluator.
const RUN_AXIOMS: [&str; 3] = ["Classical.choice", "Quot.sound", "propext"];

/// The `TargetFixtures` module.
#[must_use]
pub fn fixtures_module(cases: &[Case]) -> String {
    let mut declarations = Vec::new();
    for case in cases {
        let id = identifier(&case.fixture.name);
        let mut items = term::fixture_declarations(&case.fixture, &id);
        if !kernel_reducible(&case.fixture) {
            // Lean's evaluator, not the kernel, decides this fixture.
            items.truncate(items.len() - 1);
        }
        for item in &mut items {
            if item["kind"] == "theorem"
                || item["name"]
                    .as_str()
                    .is_some_and(|name| name.ends_with("Run"))
            {
                item["axioms"] = json!(RUN_AXIOMS);
            }
        }
        declarations.extend(items);
        if let Some(oracle) = &case.oracle {
            let Outcome::Value { steps, .. } = &case.fixture.expected else {
                panic!(
                    "library fixture {} does not produce a value",
                    case.fixture.name
                );
            };
            declarations.push(json!({
                "kind": "theorem",
                "name": format!("{id}Agrees"),
                "parameters": [],
                "axioms": RUN_AXIOMS,
                "statement": {"kind": "eq",
                    "left": {"kind": "call", "function": {"name": format!("{id}Run")}, "arguments": []},
                    "right": {"kind": "constructor", "constructor": {"module": term::SEMANTICS, "name": "Outcome.value"},
                              "arguments": [oracle, {"kind": "nat", "value": steps.to_string()}]}},
                "proof": {"kind": "reflexivity"}
            }));
        }
    }
    lx::module_tex(
        "TargetFixtures",
        &[term::SYNTAX, term::SEMANTICS, "TargetOracle"],
        declarations,
    )
}

/// Every generated file of the `compiler` project, by path relative to the
/// repository root: the fixtures, their module, and the calculus itself.
#[must_use]
pub fn files() -> BTreeMap<String, Vec<u8>> {
    let cases = cases();
    let mut out = BTreeMap::new();
    for case in &cases {
        out.insert(
            format!("compiler/fixtures/{}.json", case.fixture.name),
            case.fixture.to_file_bytes(),
        );
    }
    out.insert(
        "compiler/src/TargetFixtures.lex.tex".to_owned(),
        fixtures_module(&cases).into_bytes(),
    );
    // Each fixture's package in each Rust profile that admits it, and the
    // negative package manifests, so a change to a rendering is a reviewed
    // change to committed bytes.
    out.extend(crate::rust_packages::files());
    out.extend(crate::calculus_source::files());
    out
}

/// The directories whose every file, at any depth, is generated.
const GENERATED_DIRECTORIES: [&str; 3] = ["compiler/fixtures", "compiler/gnaf", "compiler/rust"];

/// Compare (or, with `write`, rewrite) the generated files of the calculus
/// (§17.14), of the GNAF model and requests over it (§17.15), and of the
/// fixtures' Rust packages and the negative package manifests (§17.16).
///
/// # Errors
///
/// Returns the first generated file whose committed bytes differ, or a
/// committed fixture no generator produces.
pub fn check(root: &Path, write: bool) -> Result<usize, String> {
    // A drifted shipped module also changes the compiler-semantics ID every
    // provenance binds, so it is compared first, where its report names it.
    let shipped = if write {
        0
    } else {
        shipped_modules(root, false)?
    };
    let mut files = files();
    files.extend(crate::gnaf::files());
    for directory in GENERATED_DIRECTORIES {
        for entry in walkdir::WalkDir::new(root.join(directory))
            .into_iter()
            .flatten()
            .filter(|entry| entry.file_type().is_file())
        {
            let relative = entry
                .path()
                .strip_prefix(root)
                .map_err(|error| error.to_string())?
                .to_string_lossy()
                .replace('\\', "/");
            if !files.contains_key(&relative) {
                if write {
                    std::fs::remove_file(entry.path())
                        .map_err(|error| format!("{relative}: {error}"))?;
                } else {
                    return Err(format!("{relative} is not a generated file"));
                }
            }
        }
    }
    for (relative, bytes) in &files {
        let path = root.join(relative);
        if write {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|error| format!("{relative}: {error}"))?;
            }
            std::fs::write(&path, bytes).map_err(|error| format!("{relative}: {error}"))?;
        } else {
            let committed = std::fs::read(&path).map_err(|error| format!("{relative}: {error}"))?;
            if &committed != bytes {
                return Err(format!("{relative} differs from its generator; run `cargo xtask check-calculus --write`"));
            }
        }
    }
    let shipped = if write {
        shipped_modules(root, true)?
    } else {
        shipped
    };
    Ok(files.len() + shipped)
}

/// §17.17: the calculus modules shipped for certificates are byte-equal to
/// the compiler project's golden modules; `write` copies the golden.
///
/// # Errors
///
/// Returns the first shipped module that differs or is missing.
pub fn shipped_modules(root: &Path, write: bool) -> Result<usize, String> {
    use lexlean::production::preserve::{module_path, MODULES_DIR, TARGET_MODULES};
    for module in TARGET_MODULES {
        let golden = root
            .join("compiler/expected/build/modules")
            .join(module_path(module));
        let shipped = root.join(MODULES_DIR).join(module_path(module));
        let bytes =
            std::fs::read(&golden).map_err(|error| format!("{}: {error}", golden.display()))?;
        if write {
            std::fs::write(&shipped, &bytes)
                .map_err(|error| format!("{}: {error}", shipped.display()))?;
        } else if std::fs::read(&shipped).ok().as_deref() != Some(bytes.as_slice()) {
            return Err(format!(
                "{} differs from the compiler golden {}; run `cargo xtask check-calculus --write`",
                shipped.display(),
                golden.display()
            ));
        }
    }
    Ok(TARGET_MODULES.len())
}
