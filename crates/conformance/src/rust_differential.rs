//! The primitive differential of the Rust backend (SPEC.md §17.16).
//!
//! The fixtures exercise each primitive on a few chosen inputs, which leaves
//! the boundary of every width and every representation to whichever inputs
//! a fixture happens to need. The differential closes that: one program per
//! profile holds a function applying each primitive instance the profile
//! admits to its parameters, and each function is run on its boundary
//! values, their pairs, and seeded random inputs. The denotation's outcome
//! on every input is the expected output of the rendered package.

use lexlean::calculus::rust::package::{Errors, Export, Manifest, Passing, MANIFEST_SPEC};
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{
    check, interp, Expr, Function, IntKind, OrderingValue, Prim, Program, Ty, Value, PROGRAM_SPEC,
};

/// The program of one profile, its exported names, and the argument lists
/// each function is run on.
pub struct Differential {
    /// One function per primitive instance.
    pub program: Program,
    /// Each function's exported name.
    pub names: Vec<String>,
    /// Each function's argument lists.
    pub arguments: Vec<Vec<Vec<Value>>>,
}

/// A deterministic generator, so the inputs are the same on every run and
/// every host.
struct Seeded(u64);

impl Seeded {
    fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
    /// A value of a random bit length, so small and large magnitudes are
    /// equally likely.
    fn magnitude(&mut self) -> u64 {
        let shift = self.below(64);
        self.next() >> shift
    }
}

fn fixed(width: IntKind) -> Ty {
    Ty::Fixed { width }
}
fn list(element: Ty) -> Ty {
    Ty::List {
        element: Box::new(element),
    }
}
fn pair(left: Ty, right: Ty) -> Ty {
    Ty::Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn nat(number: u128) -> Value {
    Value::Nat {
        value: number.to_string(),
    }
}
fn int(number: i128) -> Value {
    Value::Int {
        value: number.to_string(),
    }
}
fn fixed_value(width: IntKind, number: i128) -> Value {
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
fn string(text: &str) -> Value {
    Value::String {
        value: text.to_owned(),
    }
}
fn bytes(content: &[u8]) -> Value {
    Value::Bytes {
        hex: content.iter().map(|byte| format!("{byte:02x}")).collect(),
    }
}

/// Every primitive instance: its name, operation, and operand types. The
/// result type is the checker's.
fn instances() -> Vec<(String, Prim, Vec<Ty>)> {
    let mut out: Vec<(String, Prim, Vec<Ty>)> = Vec::new();
    let mut push = |name: String, operation: Prim, types: Vec<Ty>| {
        out.push((name, operation, types));
    };
    for (name, operation) in [
        ("nat_add", Prim::NatAdd),
        ("nat_sub", Prim::NatSub),
        ("nat_mul", Prim::NatMul),
        ("nat_eq", Prim::NatEq),
        ("nat_le", Prim::NatLe),
        ("nat_lt", Prim::NatLt),
    ] {
        push(name.to_owned(), operation, vec![Ty::Nat, Ty::Nat]);
    }
    for (name, operation) in [("nat_quot", Prim::NatQuot), ("nat_rem", Prim::NatRem)] {
        push(name.to_owned(), operation, vec![Ty::Nat; 3]);
    }
    for (name, operation) in [
        ("int_add", Prim::IntAdd),
        ("int_sub", Prim::IntSub),
        ("int_mul", Prim::IntMul),
    ] {
        push(name.to_owned(), operation, vec![Ty::Int, Ty::Int]);
    }
    push("int_neg".to_owned(), Prim::IntNeg, vec![Ty::Int]);
    for (name, operation) in [("int_quot", Prim::IntQuot), ("int_rem", Prim::IntRem)] {
        push(name.to_owned(), operation, vec![Ty::Int; 3]);
    }
    push("bool_not".to_owned(), Prim::BoolNot, vec![Ty::Bool]);
    push(
        "bool_and".to_owned(),
        Prim::BoolAnd,
        vec![Ty::Bool, Ty::Bool],
    );
    push("bool_or".to_owned(), Prim::BoolOr, vec![Ty::Bool, Ty::Bool]);
    for width in IntKind::ALL {
        let w = width.name();
        let ty = fixed(width);
        for (name, operation) in [
            ("checked_add", Prim::CheckedAdd),
            ("checked_sub", Prim::CheckedSub),
            ("checked_mul", Prim::CheckedMul),
            ("checked_quot", Prim::CheckedQuot),
            ("bit_and", Prim::BitAnd),
            ("bit_or", Prim::BitOr),
            ("bit_xor", Prim::BitXor),
            ("equal", Prim::Equal),
            ("compare", Prim::Compare),
        ] {
            push(
                format!("{name}_{w}"),
                operation,
                vec![ty.clone(), ty.clone()],
            );
        }
        if width.signed() {
            push(
                format!("checked_neg_{w}"),
                Prim::CheckedNeg,
                vec![ty.clone()],
            );
        }
        push(format!("bit_not_{w}"), Prim::BitNot, vec![ty.clone()]);
        for (name, operation) in [
            ("shift_left", Prim::ShiftLeft),
            ("shift_right", Prim::ShiftRight),
        ] {
            push(
                format!("{name}_{w}"),
                operation,
                vec![ty.clone(), fixed(IntKind::U32)],
            );
        }
        push(
            format!("format_decimal_{w}"),
            Prim::FormatDecimal,
            vec![ty.clone()],
        );
        push(
            format!("parse_decimal_{w}"),
            Prim::ParseDecimal { target: ty.clone() },
            vec![Ty::String],
        );
        for target in IntKind::ALL {
            push(
                format!("convert_{w}_{}", target.name()),
                Prim::Convert { target },
                vec![ty.clone()],
            );
        }
    }
    push(
        "compare_int".to_owned(),
        Prim::Compare,
        vec![Ty::Int, Ty::Int],
    );
    push(
        "format_decimal_int".to_owned(),
        Prim::FormatDecimal,
        vec![Ty::Int],
    );
    push(
        "parse_decimal_int".to_owned(),
        Prim::ParseDecimal { target: Ty::Int },
        vec![Ty::String],
    );
    for target in IntKind::ALL {
        push(
            format!("convert_int_{}", target.name()),
            Prim::Convert { target },
            vec![Ty::Int],
        );
    }
    for (name, ty) in [
        ("nat", Ty::Nat),
        ("bool", Ty::Bool),
        ("string", Ty::String),
        ("bytes", Ty::Bytes),
        ("ordering", Ty::Ordering),
    ] {
        push(format!("equal_{name}"), Prim::Equal, vec![ty.clone(), ty]);
    }
    for (name, ty) in [
        ("nat", Ty::Nat),
        ("bool", Ty::Bool),
        ("string", Ty::String),
        ("pair_nat_int", pair(Ty::Nat, Ty::Int)),
        ("pair_string_bool", pair(Ty::String, Ty::Bool)),
    ] {
        push(
            format!("compare_{name}"),
            Prim::Compare,
            vec![ty.clone(), ty],
        );
    }
    let nats = list(Ty::Nat);
    push(
        "append_list".to_owned(),
        Prim::Append,
        vec![nats.clone(), nats.clone()],
    );
    push(
        "append_bytes".to_owned(),
        Prim::Append,
        vec![Ty::Bytes, Ty::Bytes],
    );
    for (name, ty) in [
        ("list", nats.clone()),
        ("bytes", Ty::Bytes),
        ("string", Ty::String),
    ] {
        push(format!("length_{name}"), Prim::Length, vec![ty]);
    }
    for (name, ty) in [("list", nats), ("bytes", Ty::Bytes)] {
        push(
            format!("index_{name}"),
            Prim::Index,
            vec![ty.clone(), Ty::Nat],
        );
        push(
            format!("slice_{name}"),
            Prim::Slice,
            vec![ty, Ty::Nat, Ty::Nat],
        );
    }
    push("utf8_encode".to_owned(), Prim::Utf8Encode, vec![Ty::String]);
    push("utf8_decode".to_owned(), Prim::Utf8Decode, vec![Ty::Bytes]);
    push(
        "compare_bytes".to_owned(),
        Prim::CompareBytes,
        vec![Ty::Bytes, Ty::Bytes],
    );
    push(
        "split_exact".to_owned(),
        Prim::SplitExact,
        vec![Ty::String, Ty::String, fixed(IntKind::U32)],
    );
    push(
        "join".to_owned(),
        Prim::Join,
        vec![list(Ty::String), Ty::String],
    );
    out
}

/// Whether values of `ty` need the heap, so only `rust-std` admits them.
fn heap(ty: &Ty) -> bool {
    match ty {
        Ty::String | Ty::Bytes | Ty::List { .. } => true,
        Ty::Option { value } => heap(value),
        Ty::Pair { left, right } => heap(left) || heap(right),
        _ => false,
    }
}

/// The boundary values of `ty`: the extremes, the values next to them, and
/// the values where a representation changes.
fn boundary(ty: &Ty) -> Vec<Value> {
    match ty {
        Ty::Nat => [
            0_u128,
            1,
            2,
            3,
            7,
            8,
            63,
            64,
            255,
            256,
            65_535,
            65_536,
            u128::from(u32::MAX),
            1 << 32,
            (1 << 63) - 1,
            1 << 63,
            u128::from(u64::MAX) - 1,
            u128::from(u64::MAX),
        ]
        .into_iter()
        .map(nat)
        .collect(),
        Ty::Int => [
            i128::from(i64::MIN),
            i128::from(i64::MIN) + 1,
            -(1 << 32),
            -65_536,
            -256,
            -129,
            -128,
            -9,
            -2,
            -1,
            0,
            1,
            2,
            9,
            127,
            128,
            255,
            256,
            65_535,
            1 << 32,
            1 << 62,
            i128::from(i64::MAX) - 1,
            i128::from(i64::MAX),
        ]
        .into_iter()
        .map(int)
        .collect(),
        Ty::Fixed { width } => {
            let (low, high) = width.range();
            let bits = i128::from(width.bits());
            let mut values = vec![
                low,
                low + 1,
                0,
                1,
                2,
                7,
                8,
                bits - 1,
                bits,
                bits + 1,
                high / 2,
                high / 2 + 1,
                high - 1,
                high,
            ];
            if width.signed() {
                values.extend([-1, -2, low / 2, low / 2 - 1]);
            }
            if *width == IntKind::U32 {
                values.extend([31, 32, 33, 63, 64, 65]);
            }
            let mut values: Vec<i128> = values
                .into_iter()
                .filter(|value| (low..=high).contains(value))
                .collect();
            values.sort_unstable();
            values.dedup();
            values
                .into_iter()
                .map(|value| fixed_value(*width, value))
                .collect()
        }
        Ty::Bool => vec![Value::Bool { value: false }, Value::Bool { value: true }],
        Ty::Ordering => [OrderingValue::Lt, OrderingValue::Eq, OrderingValue::Gt]
            .into_iter()
            .map(|value| Value::Ordering { value })
            .collect(),
        Ty::String => {
            let wide = format!("1{}", "0".repeat(40));
            let negative_wide = format!("-{wide}");
            [
                "",
                "a",
                "ab",
                ",",
                "a,b",
                "a,,b",
                ",a,",
                "é",
                "日本",
                "😀x",
                "0",
                "-0",
                "00",
                "007",
                "+5",
                " 1",
                "1 ",
                "12a",
                "-",
                "12",
                "-12",
                "127",
                "128",
                "-128",
                "-129",
                "255",
                "256",
                "32767",
                "32768",
                "65535",
                "65536",
                "2147483647",
                "2147483648",
                "-2147483648",
                "-2147483649",
                "4294967295",
                "4294967296",
                "9223372036854775807",
                "9223372036854775808",
                "-9223372036854775808",
                "-9223372036854775809",
                "18446744073709551615",
                "18446744073709551616",
                "170141183460469231731687303715884105727",
                "170141183460469231731687303715884105728",
                wide.as_str(),
                negative_wide.as_str(),
            ]
            .into_iter()
            .map(string)
            .collect()
        }
        Ty::Bytes => [
            &[][..],
            &[0x00],
            &[0xff],
            b"abc",
            b"a,b",
            &[0xc3, 0xa9],
            &[0xe6, 0x97, 0xa5],
            &[0xf0, 0x9f, 0x98, 0x80],
            &[0x80],
            &[0xc3],
            &[0xc0, 0xaf],
            &[0xed, 0xa0, 0x80],
            &[0xf4, 0x90, 0x80, 0x80],
            &[0x61, 0xff, 0x62],
        ]
        .into_iter()
        .map(bytes)
        .collect(),
        Ty::List { element } => {
            let items = boundary(element);
            let mut out = vec![Value::List { items: Vec::new() }];
            for length in 1..=4 {
                out.push(Value::List {
                    items: items.iter().take(length).cloned().collect(),
                });
            }
            out
        }
        Ty::Pair { left, right } => {
            let lefts = boundary(left);
            let rights = boundary(right);
            let mut out = Vec::new();
            for left in lefts.iter().take(4) {
                for right in rights.iter().take(4) {
                    out.push(Value::Pair {
                        left: Box::new(left.clone()),
                        right: Box::new(right.clone()),
                    });
                }
            }
            out
        }
        Ty::Unit | Ty::Option { .. } | Ty::Result { .. } | Ty::Adt { .. } | Ty::Fn { .. } => {
            Vec::new()
        }
    }
}

/// A random value of `ty`, drawn from the boundary values and from the whole
/// range.
fn random(ty: &Ty, seeded: &mut Seeded) -> Value {
    let edges = boundary(ty);
    if !edges.is_empty() && seeded.below(4) == 0 {
        return edges[usize::try_from(seeded.below(edges.len() as u64)).unwrap_or(0)].clone();
    }
    match ty {
        Ty::Nat => nat(u128::from(seeded.magnitude())),
        Ty::Int => {
            let magnitude = seeded.magnitude() >> 1;
            let value = i128::from(magnitude);
            int(if seeded.below(2) == 0 {
                value
            } else {
                -value - i128::from(seeded.below(2))
            })
        }
        Ty::Fixed { width } => {
            let (low, high) = width.range();
            let span = u128::try_from(high - low + 1).unwrap_or(1);
            let draw = u128::from(seeded.next()) % span;
            fixed_value(*width, low + i128::try_from(draw).unwrap_or(0))
        }
        Ty::Bool => Value::Bool {
            value: seeded.below(2) == 0,
        },
        Ty::String => {
            const ALPHABET: [&str; 10] = ["a", "b", ",", "é", "😀", "0", "1", "9", "-", ",,"];
            let length = seeded.below(7);
            string(
                &(0..length)
                    .map(|_| ALPHABET[usize::try_from(seeded.below(10)).unwrap_or(0)])
                    .collect::<String>(),
            )
        }
        Ty::Bytes => {
            const POOL: [u8; 12] = [
                0x00, 0x2c, 0x41, 0x7f, 0x80, 0xbf, 0xc3, 0xa9, 0xe6, 0xf0, 0x9f, 0xff,
            ];
            let length = seeded.below(7);
            bytes(
                &(0..length)
                    .map(|_| POOL[usize::try_from(seeded.below(12)).unwrap_or(0)])
                    .collect::<Vec<u8>>(),
            )
        }
        Ty::List { element } => {
            let length = seeded.below(5);
            Value::List {
                items: (0..length).map(|_| random(element, seeded)).collect(),
            }
        }
        Ty::Pair { left, right } => Value::Pair {
            left: Box::new(random(left, seeded)),
            right: Box::new(random(right, seeded)),
        },
        Ty::Ordering => edges[usize::try_from(seeded.below(3)).unwrap_or(0)].clone(),
        Ty::Unit | Ty::Option { .. } | Ty::Result { .. } | Ty::Adt { .. } | Ty::Fn { .. } => {
            Value::Unit
        }
    }
}

/// The argument lists of a function of `types`: each boundary value of
/// each operand, every pair of boundary values of every two operands, the
/// others held at a boundary value next to an extreme, and seeded random
/// lists.
fn argument_lists(types: &[Ty], seed: u64) -> Vec<Vec<Value>> {
    let edges: Vec<Vec<Value>> = types.iter().map(boundary).collect();
    let held: Vec<Value> = edges
        .iter()
        .map(|values| {
            values
                .get(1)
                .or_else(|| values.first())
                .cloned()
                .unwrap_or(Value::Unit)
        })
        .collect();
    let mut out: Vec<Vec<Value>> = Vec::new();
    for (position, operand) in edges.iter().enumerate() {
        for value in operand {
            let mut list = held.clone();
            list[position] = value.clone();
            out.push(list);
        }
        for (other, second) in edges.iter().enumerate().skip(position + 1) {
            for value in operand {
                for paired in second {
                    let mut list = held.clone();
                    list[position] = value.clone();
                    list[other] = paired.clone();
                    out.push(list);
                }
            }
        }
    }
    let mut seeded = Seeded(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    for _ in 0..48 {
        out.push(types.iter().map(|ty| random(ty, &mut seeded)).collect());
    }
    out.sort();
    out.dedup();
    out
}

/// The differential of `profile`: every primitive instance it admits.
///
/// # Panics
///
/// Panics if an instance is ill-typed.
#[must_use]
pub fn differential(profile: Profile) -> Differential {
    let mut functions = Vec::new();
    let mut names = Vec::new();
    let mut arguments = Vec::new();
    for (seed, (name, operation, types)) in instances().into_iter().enumerate() {
        let result = check::primitive_type(&operation, &types)
            .unwrap_or_else(|reason| panic!("{name}: {reason}"));
        if profile == Profile::Core && (types.iter().any(heap) || heap(&result)) {
            continue;
        }
        functions.push(Function {
            parameters: (0..types.len() as u64).collect(),
            types: types.clone(),
            result,
            body: Expr::Prim {
                operation,
                operands: (0..types.len() as u64)
                    .map(|name| Expr::Var { name })
                    .collect(),
            },
        });
        names.push(format!("op_{name}"));
        arguments.push(argument_lists(&types, seed as u64 + 1));
    }
    Differential {
        program: Program {
            spec: PROGRAM_SPEC.to_owned(),
            adts: Vec::new(),
            functions,
        },
        names,
        arguments,
    }
}

/// The package manifest exporting every function of `differential`.
///
/// # Panics
///
/// Panics if the program is invalid.
#[must_use]
pub fn manifest(differential: &Differential, profile: Profile) -> Manifest {
    let fallible = rust::fallible_functions(&differential.program).expect("a valid program");
    Manifest {
        spec: MANIFEST_SPEC.to_owned(),
        name: format!(
            "primitives_{}",
            match profile {
                Profile::Core => "core",
                Profile::Std => "std",
            }
        ),
        version: "1.0.0".to_owned(),
        profile: profile.target().to_owned(),
        program: differential.program.clone(),
        exports: differential
            .names
            .iter()
            .enumerate()
            .map(|(function, name)| Export {
                function: function as u64,
                name: name.clone(),
                parameters: vec![
                    Passing::Own;
                    differential.program.functions[function].types.len()
                ],
                errors: if fallible[function] {
                    Errors::Overflow
                } else {
                    Errors::None
                },
            })
            .collect(),
        sources: Vec::new(),
    }
}

/// The observable outcome the denotation gives every run of `differential`,
/// in harness order.
///
/// # Panics
///
/// Panics if a run is stuck or exhausts its fuel.
#[must_use]
pub fn expected(differential: &Differential) -> Vec<serde_json::Value> {
    let mut out = Vec::new();
    for (function, lists) in differential.arguments.iter().enumerate() {
        for list in lists {
            let outcome = interp::run(&differential.program, 1_000_000, function as u64, list);
            out.push(rust::observable(&outcome).unwrap_or_else(|| {
                panic!("{} on {list:?}: {outcome:?}", differential.names[function])
            }));
        }
    }
    out
}
