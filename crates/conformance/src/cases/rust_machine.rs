//! The machine primitives and the overflow value in the Rust AST (SPEC.md
//! §17.14, §17.16): `RB-15`.
//!
//! Every machine primitive at every width is written into a crate as a
//! function of its operands, the crate is checked, printed, and compiled by
//! the pinned rustc, and a harness compares each function on the boundary
//! values of its width with an independent computation over `i128`.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use lexlean::calculus::rust::ast::{
    self, Block, Callee, Crate, Ctor, Expr, Ident, ItemDef, Lit, Machine, Origin, Pat, Type,
};
use lexlean::calculus::rust::{self, validate, Profile};
use lexlean::calculus::IntKind;

use crate::calculus::cases;
use crate::rust_build::{cargo_in, workspace, Built};

/// The operand types and the result type of a machine primitive.
fn signature(machine: Machine) -> (Vec<Type>, Type) {
    let fixed = Type::Fixed;
    match machine {
        Machine::CheckedAdd(k)
        | Machine::CheckedSub(k)
        | Machine::CheckedMul(k)
        | Machine::CheckedQuot(k) => (vec![fixed(k), fixed(k)], Type::Option(Box::new(fixed(k)))),
        Machine::CheckedNeg(k) => (vec![fixed(k)], Type::Option(Box::new(fixed(k)))),
        Machine::CheckedShl(k) | Machine::CheckedShr(k) => (
            vec![fixed(k), fixed(IntKind::U32)],
            Type::Option(Box::new(fixed(k))),
        ),
        Machine::TryFrom(k) => (vec![Type::Int], Type::Option(Box::new(fixed(k)))),
        Machine::BitAnd(k) | Machine::BitOr(k) | Machine::BitXor(k) => {
            (vec![fixed(k), fixed(k)], fixed(k))
        }
        Machine::BitNot(k) => (vec![fixed(k)], fixed(k)),
        Machine::NatEq | Machine::NatLe | Machine::NatLt => {
            (vec![Type::Nat, Type::Nat], Type::Bool)
        }
        Machine::NatRem => (vec![Type::Nat, Type::Nat, Type::Nat], Type::Nat),
        Machine::IntRem => (vec![Type::Int, Type::Int, Type::Int], Type::Int),
        Machine::CheckedNatAdd | Machine::CheckedNatMul => (
            vec![Type::Nat, Type::Nat],
            Type::Option(Box::new(Type::Nat)),
        ),
        Machine::NatSub => (vec![Type::Nat, Type::Nat], Type::Nat),
        Machine::NatQuot => (vec![Type::Nat, Type::Nat, Type::Nat], Type::Nat),
        Machine::CheckedIntAdd | Machine::CheckedIntSub | Machine::CheckedIntMul => (
            vec![Type::Int, Type::Int],
            Type::Option(Box::new(Type::Int)),
        ),
        Machine::ListLength => (vec![Type::List(Box::new(Type::Param(0)))], Type::Nat),
        Machine::ListAppend => (
            vec![
                Type::List(Box::new(Type::Param(0))),
                Type::List(Box::new(Type::Param(0))),
            ],
            Type::List(Box::new(Type::Param(0))),
        ),
        Machine::ListIndex => (
            vec![Type::List(Box::new(Type::Param(0))), Type::Nat],
            Type::Option(Box::new(Type::Param(0))),
        ),
        Machine::ListSlice => (
            vec![Type::List(Box::new(Type::Param(0))), Type::Nat, Type::Nat],
            Type::Option(Box::new(Type::List(Box::new(Type::Param(0))))),
        ),
        Machine::CheckedIntNeg => (vec![Type::Int], Type::Option(Box::new(Type::Int))),
        Machine::CheckedIntQuot => (
            vec![Type::Int, Type::Int, Type::Int],
            Type::Option(Box::new(Type::Int)),
        ),
    }
}

fn call(machine: Machine, operands: usize) -> Expr {
    Expr::Call {
        callee: Callee::Machine(machine),
        args: (0..operands)
            .map(|n| Expr::Copy(Ident::Param(n as u64), Origin::of("operand")))
            .collect(),
        propagate: false,
        at: Origin::of("machine"),
    }
}

fn function(name: String, machine: Machine, operands: usize) -> ItemDef {
    let (parameters, result) = signature(machine);
    ItemDef::Function {
        name: Ident::Export(name),
        parameters: parameters
            .into_iter()
            .enumerate()
            .map(|(n, ty)| (Pat::Bind(Ident::Param(n as u64)), ty))
            .collect(),
        result,
        body: Block::of(call(machine, operands)),
        at: Origin::of("function"),
    }
}

/// The boundary values of a width, as `i128`.
fn boundary(kind: IntKind) -> Vec<i128> {
    let (low, high) = kind.range();
    let mut values = vec![low, low + 1, 0, 1, high - 1, high];
    if kind.signed() {
        values.push(-1);
    }
    values.sort_unstable();
    values.dedup();
    values
}

fn harness(machines: &[Machine]) -> String {
    let name = |machine: Machine| {
        format!(
            "m_{}",
            machines
                .iter()
                .position(|one| *one == machine)
                .expect("listed")
        )
    };
    let mut out = String::from("use machine::*;\nfn main() {\n");
    for kind in IntKind::ALL {
        let k = kind.name();
        let (low, high) = kind.range();
        let values = boundary(kind)
            .iter()
            .map(|v| format!("{v}i128"))
            .collect::<Vec<_>>()
            .join(", ");
        let bits = kind.bits();
        writeln!(
            out,
            "    {{\n        let fit = |v: i128| if v >= {low}i128 && v <= {high}i128 {{ Some(v as {k}) }} else {{ None }};\n        let vals: &[i128] = &[{values}];\n        for &a in vals {{\n            for &b in vals {{\n                let (x, y) = (a as {k}, b as {k});\n                assert_eq!({add}(x, y), fit(a + b), \"add {k} {{a}} {{b}}\");\n                assert_eq!({sub}(x, y), fit(a - b), \"sub {k} {{a}} {{b}}\");\n                assert_eq!({mul}(x, y), a.checked_mul(b).and_then(fit), \"mul {k} {{a}} {{b}}\");\n                assert_eq!({quot}(x, y), if b == 0 {{ None }} else {{ fit(a / b) }}, \"quot {k} {{a}} {{b}}\");\n                assert_eq!({and}(x, y), (a & b) as {k}, \"and {k} {{a}} {{b}}\");\n                assert_eq!({or}(x, y), (a | b) as {k}, \"or {k} {{a}} {{b}}\");\n                assert_eq!({xor}(x, y), (a ^ b) as {k}, \"xor {k} {{a}} {{b}}\");\n            }}\n            let x = a as {k};\n            assert_eq!({not}(x), (!a) as {k}, \"not {k} {{a}}\");\n            for amount in [0u32, 1, {bits} - 1, {bits}, {bits} + 1, 200] {{\n                let shl = if amount >= {bits} {{ None }} else {{ Some(((a as u128) << amount) as {k}) }};\n                let shr = if amount >= {bits} {{ None }} else {{ Some((a >> amount) as {k}) }};\n                assert_eq!({shl_}(x, amount), shl, \"shl {k} {{a}} {{amount}}\");\n                assert_eq!({shr_}(x, amount), shr, \"shr {k} {{a}} {{amount}}\");\n            }}\n        }}\n        for w in [i64::MIN, -129, -128, -1, 0, 1, 127, 128, 255, 256, 65535, 65536, 2147483647, 2147483648, 4294967295, 4294967296, i64::MAX] {{\n            assert_eq!({from}(w), fit(w as i128), \"try_from {k} {{w}}\");\n        }}\n{neg}    }}",
            add = name(Machine::CheckedAdd(kind)),
            sub = name(Machine::CheckedSub(kind)),
            mul = name(Machine::CheckedMul(kind)),
            quot = name(Machine::CheckedQuot(kind)),
            and = name(Machine::BitAnd(kind)),
            or = name(Machine::BitOr(kind)),
            xor = name(Machine::BitXor(kind)),
            not = name(Machine::BitNot(kind)),
            shl_ = name(Machine::CheckedShl(kind)),
            shr_ = name(Machine::CheckedShr(kind)),
            from = name(Machine::TryFrom(kind)),
            neg = if kind.signed() {
                format!(
                    "        for &a in vals {{ assert_eq!({}(a as {k}), fit(-a), \"neg {k} {{a}}\"); }}\n",
                    name(Machine::CheckedNeg(kind))
                )
            } else {
                String::new()
            },
        )
        .expect("write");
    }
    let (eq, le, lt, nat_rem, int_rem) = (
        name(Machine::NatEq),
        name(Machine::NatLe),
        name(Machine::NatLt),
        name(Machine::NatRem),
        name(Machine::IntRem),
    );
    let (nat_add, nat_sub, nat_mul, nat_quot) = (
        name(Machine::CheckedNatAdd),
        name(Machine::NatSub),
        name(Machine::CheckedNatMul),
        name(Machine::NatQuot),
    );
    let (int_add, int_sub, int_mul, int_neg, int_quot) = (
        name(Machine::CheckedIntAdd),
        name(Machine::CheckedIntSub),
        name(Machine::CheckedIntMul),
        name(Machine::CheckedIntNeg),
        name(Machine::CheckedIntQuot),
    );
    write!(
        out,
        "    let nats = [0u64, 1, 2, u64::MAX - 1, u64::MAX];\n    for &a in &nats {{\n        for &b in &nats {{\n            assert_eq!({eq}(a, b), a == b);\n            assert_eq!({le}(a, b), a <= b);\n            assert_eq!({lt}(a, b), a < b);\n            assert_eq!({nat_rem}(a, b, 7), if b == 0 {{ 7 }} else {{ a % b }}, \"nat_rem {{a}} {{b}}\");\n            let fit = |v: u128| if v <= u64::MAX as u128 {{ Some(v as u64) }} else {{ None }};\n            assert_eq!({nat_add}(a, b), fit(a as u128 + b as u128), \"nat_add {{a}} {{b}}\");\n            assert_eq!({nat_mul}(a, b), fit(a as u128 * b as u128), \"nat_mul {{a}} {{b}}\");\n            assert_eq!({nat_sub}(a, b), if a >= b {{ a - b }} else {{ 0 }}, \"nat_sub {{a}} {{b}}\");\n            assert_eq!({nat_quot}(a, b, 7), if b == 0 {{ 7 }} else {{ a / b }}, \"nat_quot {{a}} {{b}}\");\n        }}\n    }}\n    let ints = [i64::MIN, i64::MIN + 1, -7, -1, 0, 1, 2, i64::MAX];\n    for &a in &ints {{\n        for &b in &ints {{\n            let expected = if b == 0 {{ 7 }} else {{ ((a as i128) % (b as i128)) as i64 }};\n            assert_eq!({int_rem}(a, b, 7), expected, \"int_rem {{a}} {{b}}\");\n            let fit = |v: i128| if v >= i64::MIN as i128 && v <= i64::MAX as i128 {{ Some(v as i64) }} else {{ None }};\n            let (x, y) = (a as i128, b as i128);\n            assert_eq!({int_add}(a, b), fit(x + y), \"int_add {{a}} {{b}}\");\n            assert_eq!({int_sub}(a, b), fit(x - y), \"int_sub {{a}} {{b}}\");\n            assert_eq!({int_mul}(a, b), fit(x * y), \"int_mul {{a}} {{b}}\");\n            assert_eq!({int_quot}(a, b, 7), if b == 0 {{ Some(7) }} else {{ fit(x / y) }}, \"int_quot {{a}} {{b}}\");\n        }}\n        assert_eq!({int_neg}(a), if a == i64::MIN {{ None }} else {{ Some(-a) }}, \"int_neg {{a}}\");\n    }}\n    println!(\"machine\");\n}}\n"
    )
    .expect("write");
    out
}

fn refused(krate: &Crate, message: &str) {
    let error = validate::validate(krate).expect_err("the planted crate is refused");
    assert!(error.contains(message), "{error}");
}

/// Run the case of `RB-15`.
pub fn run(id: &str) {
    assert_eq!(id, "RB-15");
    // The list machine calls are std ones: the Core harness takes the others.
    let machines: Vec<Machine> = Machine::all()
        .into_iter()
        .filter(|machine| {
            !matches!(
                machine,
                Machine::ListLength | Machine::ListAppend | Machine::ListIndex | Machine::ListSlice
            )
        })
        .collect();
    // Seventeen constructors, eleven of them at each of eight widths and one
    // at the four signed ones: the list is the Lean `Machine` type's.
    assert_eq!(machines.len(), 5 + 9 + 8 * 11 + 4);
    let items: Vec<ItemDef> = machines
        .iter()
        .enumerate()
        .map(|(n, machine)| function(format!("m_{n}"), *machine, signature(*machine).0.len()))
        .collect();
    let krate = Crate {
        profile: Profile::Core,
        items,
    };
    validate::validate(&krate).expect("the machine calls are valid");
    let text = ast::print(&krate).expect("prints");
    for needle in [
        "u8::checked_add(p0, p1)",
        "i64::checked_neg(p0)",
        "u16::checked_shl(p0, p1)",
        "u32::try_from(p0).ok()",
        "core::ops::BitAnd::bitand(p0, p1)",
        "core::ops::Not::not(p0)",
        "core::cmp::PartialOrd::le(&p0, &p1)",
        "u64::checked_rem(p0, p1, p2)",
    ] {
        let wanted = needle.replace(
            "u64::checked_rem(p0, p1, p2)",
            "u64::checked_rem(p0, p1).unwrap_or(p2)",
        );
        assert!(text.contains(&wanted), "the rendering lacks `{wanted}`");
    }
    // The pinned rustc compiles the crate and the harness on the boundary
    // values.
    let directory = tempfile::tempdir().expect("a directory");
    let files: BTreeMap<String, Vec<u8>> = BTreeMap::from([
        (
            "Cargo.toml".to_owned(),
            b"[package]\nname = \"machine\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[lints.rust]\nwarnings = \"deny\"\n".to_vec(),
        ),
        ("src/lib.rs".to_owned(), text.into_bytes()),
    ]);
    workspace(
        directory.path(),
        &[Built {
            name: "machine".to_owned(),
            files,
            harness: harness(&machines),
        }],
    );
    let output = cargo_in(
        directory.path(),
        &["run", "--quiet", "--offline", "-p", "run_machine"],
    );
    assert!(
        output.status.success(),
        "the machine calls do not compute as stated:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "machine");
    // A call with another number of operands than its primitive takes, and
    // an overflow value outside the payload of an `Err`, are refused.
    let one = |body: Expr, result: Type| Crate {
        profile: Profile::Core,
        items: vec![ItemDef::Function {
            name: Ident::Export("planted".to_owned()),
            parameters: vec![(Pat::Bind(Ident::Param(0)), Type::Fixed(IntKind::U8))],
            result,
            body: Block::of(body),
            at: Origin::of("function"),
        }],
    };
    refused(
        &one(
            call(Machine::CheckedAdd(IntKind::U8), 1),
            Type::Option(Box::new(Type::Fixed(IntKind::U8))),
        ),
        "machine: `checked_add` receives 1 operands of 2",
    );
    refused(
        &one(Expr::Lit(Lit::Overflow, Origin::of("overflow")), Type::Unit),
        "overflow: the overflow value stands outside the payload of an `Err`",
    );
    let err = Expr::Construct {
        ctor: Ctor::Err(Type::Unit, Type::Unit),
        args: vec![Expr::Lit(Lit::Overflow, Origin::of("overflow"))],
        at: Origin::of("overflow"),
    };
    let accepted = one(err, Type::Unit);
    // A machine call in a lowered crate has no row: `correspond` refuses the
    // construct, whatever element it claims to realize.
    let sum = cases()
        .into_iter()
        .find(|case| case.fixture.name == "sum-to")
        .expect("fixture sum-to")
        .fixture
        .program;
    let mut lowered = rust::lower(&sum, Profile::Std).expect("lowers");
    let elements = rust::realized(&sum, Profile::Std).expect("valid");
    validate::correspond(&lowered, &elements).expect("the lowered crate corresponds");
    let planted = super::rust_backend::edit_first(&mut lowered, &mut |expr| {
        if let Expr::Call { callee, .. } = expr {
            if matches!(callee, Callee::Runtime(_)) {
                *callee = Callee::Machine(Machine::NatEq);
                return true;
            }
        }
        false
    });
    assert!(planted, "the plant site exists");
    let error = validate::correspond(&lowered, &elements).expect_err("a machine call has no row");
    assert!(
        error.contains("the construct `call:machine` has no target-semantics correspondence"),
        "{error}"
    );
    validate::validate(&accepted).expect("an overflow value in the payload of an Err is valid");
}
