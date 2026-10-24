//! `BS-09`: the literal values of the Rust lowering of SPEC.md §17.16
//! (`Lowering::value`) as language-1.2 source (`LexLeanBoot.RustTerms`), the
//! first pass of the lowering of terms. The oracle's `value` is private to the
//! renderer, whose files the compiler-semantics digest frames, so the pass is
//! held to it by composition: a function whose body is a literal value lowers,
//! through every later pass of `rust::lower`, to a body whose tail is exactly
//! the expression `value` built, so the root takes a program, the tree of
//! `siteTrees`, and the index of such a function, and the case compares its
//! result with the tail of that function of the crate `rust::lower` returns.
//!
//! A program the oracle's checker refuses, and a literal the Rust lowering
//! refuses, are not vectors: the first is outside the roots of `BS-07`, and
//! the second is the root's `none`, which the corpora exercise on the
//! literals the checker accepts and `rust::lower` refuses.

use lexlean::calculus::rust::ast::{Ident, ItemDef};
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{self as target, Expr, Function, IntKind, OrderingValue, Ty, Value};
use serde_json::Value as Json;

use super::calculus_check::nat;
use super::calculus_check::{program_arguments, program_of};
use super::rust_lower::{assert_eligible, eligibility_report, enc_site};
use super::rust_print::enc_expr;
use super::{option, Run, Seeded};
use crate::production_export as harness;
use crate::support;
use lexlean::calculus::sites;

const MODULE: &str = "LexLeanBoot.RustTerms";

/// Three quarters of `max_file_bytes` (4,194,304), the most a part of a
/// certificate of this root may be.
const CERTIFICATE_BUDGET: u64 = 3_145_728;

/// The vectors of the explicit corpus, those the evaluator runs, and the
/// seeded ones each of the two runs.
const EXPLICIT: usize = 358;
const EVALUATOR_SEEDED: usize = 150;
const RUSTC_SEEDED: usize = 1_500;

/// The context a literal is lowered in: the ADTs of the program and the
/// functions before the one whose body is the literal.
struct Context {
    adts: Vec<Vec<Vec<Ty>>>,
    functions: Vec<Function>,
}

fn adt(index: u64) -> Ty {
    Ty::Adt { index }
}

fn option_of(ty: Ty) -> Ty {
    Ty::Option {
        value: Box::new(ty),
    }
}

fn list_of(ty: Ty) -> Ty {
    Ty::List {
        element: Box::new(ty),
    }
}

fn pair_of(left: Ty, right: Ty) -> Ty {
    Ty::Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn result_of(ok: Ty, error: Ty) -> Ty {
    Ty::Result {
        ok: Box::new(ok),
        error: Box::new(error),
    }
}

fn fixed(width: IntKind) -> Ty {
    Ty::Fixed { width }
}

fn function_type() -> Ty {
    Ty::Fn {
        parameters: vec![Ty::Nat],
        result: Box::new(Ty::Nat),
    }
}

/// The contexts of the corpora: no ADT, a list that holds itself, ADTs that
/// hold an option, a result, and a pair and two that hold each other through
/// an option, and a program that states a function type.
fn contexts() -> Vec<Context> {
    let identity = Function {
        parameters: vec![0],
        types: vec![Ty::Nat],
        result: Ty::Nat,
        body: Expr::Var { name: 0 },
    };
    let closure = Function {
        parameters: Vec::new(),
        types: Vec::new(),
        result: function_type(),
        body: Expr::Closure {
            function: 0,
            captures: Vec::new(),
        },
    };
    vec![
        Context {
            adts: Vec::new(),
            functions: Vec::new(),
        },
        Context {
            adts: vec![vec![vec![], vec![Ty::Nat, adt(0)]]],
            functions: Vec::new(),
        },
        Context {
            adts: vec![vec![
                vec![option_of(Ty::Nat), pair_of(Ty::Bool, fixed(IntKind::U8))],
                vec![result_of(Ty::String, Ty::Bytes)],
                vec![],
            ]],
            functions: Vec::new(),
        },
        Context {
            adts: vec![
                vec![vec![adt(1)], vec![]],
                vec![vec![option_of(adt(0))], vec![Ty::Ordering]],
            ],
            functions: Vec::new(),
        },
        Context {
            adts: Vec::new(),
            functions: vec![identity, closure],
        },
    ]
}

fn nat_value(text: &str) -> Value {
    Value::Nat {
        value: text.to_owned(),
    }
}

fn int_value(text: &str) -> Value {
    Value::Int {
        value: text.to_owned(),
    }
}

fn some(value: Value) -> Value {
    Value::Some {
        value: Box::new(value),
    }
}

fn pair(left: Value, right: Value) -> Value {
    Value::Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn items(values: Vec<Value>) -> Value {
    Value::List { items: values }
}

/// The literal of a fixed width, which the value kind of the width names.
fn fixed_value(width: IntKind, text: &str) -> Value {
    let value = text.to_owned();
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

const WIDTHS: [IntKind; 8] = [
    IntKind::U8,
    IntKind::U16,
    IntKind::U32,
    IntKind::U64,
    IntKind::I8,
    IntKind::I16,
    IntKind::I32,
    IntKind::I64,
];

/// The edge literals of a width: zero, one, and the two ends of its range.
fn fixed_edges(width: IntKind) -> [&'static str; 4] {
    match width {
        IntKind::U8 => ["0", "1", "128", "255"],
        IntKind::U16 => ["0", "1", "32768", "65535"],
        IntKind::U32 => ["0", "1", "2147483648", "4294967295"],
        IntKind::U64 => ["0", "1", "9223372036854775808", "18446744073709551615"],
        IntKind::I8 => ["0", "-1", "-128", "127"],
        IntKind::I16 => ["0", "-1", "-32768", "32767"],
        IntKind::I32 => ["0", "-1", "-2147483648", "2147483647"],
        IntKind::I64 => ["0", "-1", "-9223372036854775808", "9223372036854775807"],
    }
}

/// The scalar literals the corpora write, with the type of each: the zero,
/// the ends of every range, and the texts of every kind the checker accepts,
/// which are the canonical ones (no sign but the minus of a negative, no
/// leading zero, hexadecimal in lower case and of even length). Every
/// non-canonical text the checker refuses is no vector of this root.
fn scalars() -> Vec<(Ty, Value)> {
    let mut out = vec![
        (Ty::Unit, Value::Unit),
        (Ty::Bool, Value::Bool { value: true }),
        (Ty::Bool, Value::Bool { value: false }),
    ];
    for text in [
        "0",
        "1",
        "7",
        "10",
        "255",
        "65535",
        "4294967295",
        "18446744073709551615",
    ] {
        out.push((Ty::Nat, nat_value(text)));
    }
    for text in [
        "0",
        "1",
        "-1",
        "42",
        "-10",
        "9223372036854775807",
        "-9223372036854775808",
    ] {
        out.push((Ty::Int, int_value(text)));
    }
    for width in WIDTHS {
        for text in fixed_edges(width) {
            out.push((fixed(width), fixed_value(width, text)));
        }
    }
    for text in ["", "a", "abc", "a\"b\\c", "é€", "line\nnext", "\u{0}\u{1f}"] {
        out.push((
            Ty::String,
            Value::String {
                value: text.to_owned(),
            },
        ));
    }
    for hex in ["", "00", "ff", "00ff10", "deadbeef", "0123456789abcdef"] {
        out.push((
            Ty::Bytes,
            Value::Bytes {
                hex: hex.to_owned(),
            },
        ));
    }
    for value in [OrderingValue::Lt, OrderingValue::Eq, OrderingValue::Gt] {
        out.push((Ty::Ordering, Value::Ordering { value }));
    }
    out
}

/// The oracle's lowering of the literal `value` of type `ty` as the body of
/// the last function of `context`: the arguments of the root and what it must
/// return, `none` when the lowering refuses the literal.
fn run(context: &Context, ty: &Ty, value: &Value) -> Option<Run> {
    let mut functions = context.functions.clone();
    let at = functions.len();
    functions.push(Function {
        parameters: Vec::new(),
        types: Vec::new(),
        result: ty.clone(),
        body: Expr::Value {
            ty: ty.clone(),
            value: value.clone(),
        },
    });
    let program = program_of(context.adts.clone(), functions);
    // The root is given a program the checker accepts, as in `BS-07`.
    target::check::check(&program).ok()?;
    let tree = enc_site(&sites::trees(&program).ok()?);
    let mut arguments = program_arguments(&program);
    arguments.push(tree);
    arguments.push(nat(at as u64));
    let expected = match rust::lower(&program, Profile::Std) {
        Ok(krate) => {
            let tail = krate.items.iter().find_map(|item| match item {
                ItemDef::Function {
                    name: Ident::Function(number),
                    body,
                    ..
                } if usize::try_from(*number).ok() == Some(at) => Some(body),
                _ => None,
            })?;
            assert!(
                tail.lets.is_empty(),
                "the body of a literal is its expression"
            );
            Some(enc_expr(&tail.tail))
        }
        Err(_) => None,
    };
    Some((
        arguments,
        serde_json::to_value(option(expected)).expect("a value is JSON"),
    ))
}

/// The explicit corpus: every scalar literal in every context, and the
/// composite literals built from them, in every context that can hold them.
fn explicit() -> Vec<Run> {
    let contexts = contexts();
    let scalars = scalars();
    let mut out = Vec::new();
    for context in &contexts {
        for (ty, value) in &scalars {
            out.extend(run(context, ty, value));
        }
    }
    let plain = &contexts[0];
    for (ty, value) in composites() {
        out.extend(run(plain, &ty, &value));
    }
    // The literals of the types the ADTs of a context give.
    let list = &contexts[1];
    let cons = |head: &str, tail: Value| Value::Adt {
        constructor: 1,
        fields: vec![nat_value(head), tail],
    };
    let nil = Value::Adt {
        constructor: 0,
        fields: Vec::new(),
    };
    out.extend(run(list, &adt(0), &nil));
    out.extend(run(list, &adt(0), &cons("1", nil.clone())));
    out.extend(run(list, &adt(0), &cons("1", cons("2", nil.clone()))));
    out.extend(run(list, &list_of(adt(0)), &items(vec![nil.clone()])));
    out.extend(run(list, &option_of(adt(0)), &some(cons("3", nil))));
    let holder = &contexts[2];
    let first = Value::Adt {
        constructor: 0,
        fields: vec![
            some(nat_value("9")),
            pair(Value::Bool { value: true }, fixed_value(IntKind::U8, "200")),
        ],
    };
    let second = Value::Adt {
        constructor: 1,
        fields: vec![Value::Ok {
            value: Box::new(Value::String {
                value: "ok".to_owned(),
            }),
        }],
    };
    let third = Value::Adt {
        constructor: 2,
        fields: Vec::new(),
    };
    for value in [first, second, third] {
        out.extend(run(holder, &adt(0), &value));
    }
    let mutual = &contexts[3];
    let inner = Value::Adt {
        constructor: 0,
        fields: vec![Value::Adt {
            constructor: 0,
            fields: vec![some(Value::Adt {
                constructor: 1,
                fields: Vec::new(),
            })],
        }],
    };
    out.extend(run(
        mutual,
        &adt(0),
        &Value::Adt {
            constructor: 1,
            fields: Vec::new(),
        },
    ));
    out.extend(run(
        mutual,
        &adt(1),
        &Value::Adt {
            constructor: 1,
            fields: vec![Value::Ordering {
                value: OrderingValue::Gt,
            }],
        },
    ));
    out.extend(run(
        mutual,
        &adt(1),
        &Value::Adt {
            constructor: 0,
            fields: vec![some(inner)],
        },
    ));
    // A function type in a literal: the absent option, the empty list.
    let functions = &contexts[4];
    out.extend(run(functions, &option_of(function_type()), &Value::None));
    out.extend(run(
        functions,
        &list_of(function_type()),
        &items(Vec::new()),
    ));
    out.extend(run(
        functions,
        &pair_of(Ty::Nat, option_of(function_type())),
        &pair(nat_value("1"), Value::None),
    ));
    out
}

/// The composite literals: the wrappers, the lists, and the pairs.
fn composites() -> Vec<(Ty, Value)> {
    let nat = || nat_value("3");
    vec![
        (option_of(Ty::Nat), Value::None),
        (option_of(Ty::Nat), some(nat())),
        (
            option_of(option_of(Ty::Bool)),
            some(some(Value::Bool { value: true })),
        ),
        (option_of(option_of(Ty::Bool)), some(Value::None)),
        (
            result_of(Ty::Nat, Ty::Bool),
            Value::Ok {
                value: Box::new(nat()),
            },
        ),
        (
            result_of(Ty::Nat, Ty::Bool),
            Value::Error {
                value: Box::new(Value::Bool { value: false }),
            },
        ),
        (
            result_of(fixed(IntKind::I16), option_of(Ty::Unit)),
            Value::Error {
                value: Box::new(some(Value::Unit)),
            },
        ),
        (list_of(Ty::Nat), items(Vec::new())),
        (list_of(Ty::Nat), items(vec![nat()])),
        (
            list_of(Ty::Nat),
            items(vec![nat_value("1"), nat_value("2"), nat_value("3")]),
        ),
        (
            list_of(fixed(IntKind::U8)),
            items(vec![
                fixed_value(IntKind::U8, "1"),
                fixed_value(IntKind::U8, "2"),
            ]),
        ),
        (
            list_of(list_of(Ty::Bool)),
            items(vec![
                items(Vec::new()),
                items(vec![Value::Bool { value: true }]),
            ]),
        ),
        (
            pair_of(Ty::Nat, Ty::Bool),
            pair(nat(), Value::Bool { value: true }),
        ),
        (
            pair_of(fixed(IntKind::I64), pair_of(Ty::String, Ty::Unit)),
            pair(
                fixed_value(IntKind::I64, "-5"),
                pair(
                    Value::String {
                        value: "x".to_owned(),
                    },
                    Value::Unit,
                ),
            ),
        ),
    ]
}

/// A type of the seeded corpora and a literal of it, or none when the type
/// has no literal.
fn random_ty(seeded: &mut Seeded, adts: usize, depth: usize) -> Ty {
    let kinds = if depth == 0 { 8 } else { 13 };
    match seeded.below(kinds) {
        0 => Ty::Unit,
        1 => Ty::Bool,
        2 => Ty::Nat,
        3 => Ty::Int,
        4 => fixed(*seeded.pick(&WIDTHS)),
        5 => Ty::String,
        6 => Ty::Bytes,
        7 => {
            if adts > 0 && seeded.chance(50) {
                adt(seeded.below(adts) as u64)
            } else {
                Ty::Ordering
            }
        }
        8 => option_of(random_ty(seeded, adts, depth - 1)),
        9 => result_of(
            random_ty(seeded, adts, depth - 1),
            random_ty(seeded, adts, depth - 1),
        ),
        10 => list_of(random_ty(seeded, adts, depth - 1)),
        11 => pair_of(
            random_ty(seeded, adts, depth - 1),
            random_ty(seeded, adts, depth - 1),
        ),
        _ => {
            if adts > 0 {
                adt(seeded.below(adts) as u64)
            } else {
                Ty::Unit
            }
        }
    }
}

fn random_text(seeded: &mut Seeded, alphabet: &[&str], longest: usize) -> String {
    (0..seeded.below(longest + 1))
        .map(|_| *seeded.pick(alphabet))
        .collect()
}

/// A literal of `ty`; a few of them do not fit it, so the refusals of the
/// checker and of the renderer are drawn on too.
fn random_value(seeded: &mut Seeded, context: &Context, ty: &Ty, depth: usize) -> Value {
    match ty {
        Ty::Unit => Value::Unit,
        Ty::Bool => Value::Bool {
            value: seeded.chance(50),
        },
        Ty::Nat => nat_value(&(seeded.next() >> seeded.below(64)).to_string()),
        Ty::Int => int_value(&((seeded.next() >> seeded.below(64)) as i64).to_string()),
        Ty::Fixed { width } => {
            let bits = match width {
                IntKind::U8 | IntKind::I8 => 8,
                IntKind::U16 | IntKind::I16 => 16,
                IntKind::U32 | IntKind::I32 => 32,
                IntKind::U64 | IntKind::I64 => 64,
            };
            let word = seeded.next() >> (64 - bits);
            let text = match width {
                IntKind::U8 | IntKind::U16 | IntKind::U32 | IntKind::U64 => word.to_string(),
                IntKind::I8 => (word as u8 as i8).to_string(),
                IntKind::I16 => (word as u16 as i16).to_string(),
                IntKind::I32 => (word as u32 as i32).to_string(),
                IntKind::I64 => (word as i64).to_string(),
            };
            fixed_value(*width, &text)
        }
        Ty::String => Value::String {
            value: random_text(seeded, &["a", "\"", "\\", "é", "\n", "\u{7f}", " "], 6),
        },
        Ty::Bytes => Value::Bytes {
            hex: (0..seeded.below(6))
                .map(|_| format!("{:02x}", seeded.next() >> 56))
                .collect(),
        },
        Ty::Ordering => Value::Ordering {
            value: *seeded.pick(&[OrderingValue::Lt, OrderingValue::Eq, OrderingValue::Gt]),
        },
        Ty::Option { value } => {
            if seeded.chance(40) {
                Value::None
            } else {
                some(random_value(seeded, context, value, depth))
            }
        }
        Ty::Result { ok, error } => {
            if seeded.chance(50) {
                Value::Ok {
                    value: Box::new(random_value(seeded, context, ok, depth)),
                }
            } else {
                Value::Error {
                    value: Box::new(random_value(seeded, context, error, depth)),
                }
            }
        }
        Ty::List { element } => items(
            (0..seeded.below(4))
                .map(|_| random_value(seeded, context, element, depth))
                .collect(),
        ),
        Ty::Pair { left, right } => pair(
            random_value(seeded, context, left, depth),
            random_value(seeded, context, right, depth),
        ),
        Ty::Adt { index } => {
            let constructors = &context.adts[*index as usize];
            // Past the depth, the first constructor whose fields name no ADT.
            let flat =
                |fields: &Vec<Ty>| !fields.iter().any(|field| matches!(field, Ty::Adt { .. }));
            let chosen = if depth == 0 {
                constructors.iter().position(flat).unwrap_or(0)
            } else {
                seeded.below(constructors.len())
            };
            Value::Adt {
                constructor: chosen as u64,
                fields: constructors[chosen]
                    .iter()
                    .map(|field| random_value(seeded, context, field, depth.saturating_sub(1)))
                    .collect(),
            }
        }
        Ty::Fn { .. } => Value::Unit,
    }
}

/// `count` seeded vectors: a context, a type, and a literal of it.
fn seeded_runs(count: usize, seed: u64) -> Vec<Run> {
    let contexts = contexts();
    let mut seeded = Seeded(seed);
    let mut out = Vec::new();
    while out.len() < count {
        let context = &contexts[seeded.below(contexts.len() - 1)];
        let ty = random_ty(&mut seeded, context.adts.len(), 3);
        let value = random_value(&mut seeded, context, &ty, 3);
        out.extend(run(context, &ty, &value));
    }
    out
}

/// A defect planted in the source of `RustTerms`, caught on the runs of the
/// root.
struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
}

const PLANTS: [Plant; 10] = [
    Plant {
        label: "the minus of an integer is dropped",
        site: "(ctor Option.some [(Bool * Nat)] (true, magnitude))",
        replacement: "(ctor Option.some [(Bool * Nat)] (false, magnitude))",
    },
    Plant {
        label: "a digit is worth nine times the place before it",
        site: "(prim multiply (second state) 10 : Nat)",
        replacement: "(prim multiply (second state) 9 : Nat)",
    },
    Plant {
        label: "the letters of a hexadecimal digit are one less",
        site: "(prim subtract n 87 : Nat)",
        replacement: "(prim subtract n 88 : Nat)",
    },
    Plant {
        label: "the width of a fixed literal is not in its origin",
        site: "(ctor Option.some [RustValidate::IntKind] (RustLower::kindOf width))",
        replacement: "(ctor Option.none [RustValidate::IntKind])",
    },
    Plant {
        label: "the origin of an error is that of an err",
        site: "(prim utf8_encode \"value:error\" : Bytes)",
        replacement: "(prim utf8_encode \"value:err\" : Bytes)",
    },
    Plant {
        label: "the type of an absent option is the option's own",
        site: "(ctor RustValidate::Con.ConNone (RustLower::vty known inner))",
        replacement: "(ctor RustValidate::Con.ConNone (RustLower::vty known ty))",
    },
    Plant {
        label: "the types of an error constructor are swapped",
        site: "(ctor RustValidate::Con.ConErr (RustLower::vty known (first parts)) (RustLower::vty known (second parts)))",
        replacement: "(ctor RustValidate::Con.ConErr (RustLower::vty known (second parts)) (RustLower::vty known (first parts)))",
    },
    Plant {
        label: "the head and the tail of a list are swapped",
        site: "(ctor RustAst::Expr.ECons one (ctor RustAst::Expr.ECons two (ctor RustAst::Expr.ENil)))",
        replacement: "(ctor RustAst::Expr.ECons two (ctor RustAst::Expr.ECons one (ctor RustAst::Expr.ENil)))",
    },
    Plant {
        label: "a field that holds its ADT is not boxed",
        site: "(if (RustLower::cyclic known edges owner head) then",
        replacement: "(if false then",
    },
    Plant {
        label: "every field of an ADT is boxed",
        site: "(if (RustLower::cyclic known edges owner head) then",
        replacement: "(if true then",
    },
];

/// Each defect planted in a copy of `RustTerms` is a disagreement with the
/// oracle on the runs of the root.
fn plants(detection: &[Run]) {
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/RustTerms.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/RustTerms.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, "lowerValueAt");
        let disagreements = super::evaluate(&low, detection, 1);
        assert!(
            !disagreements.is_empty(),
            "{}: the planted defect is detected by the differential",
            plant.label
        );
        eprintln!(
            "BS-09: plant detected: {}: {}",
            plant.label, disagreements[0]
        );
    }
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("RustTerms"),
        "RustTerms is a linked module of the project"
    );
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report = eligibility_report(&project, &build_id, MODULE);
    let rows = report["roots"].as_array().expect("roots");
    assert_eq!(rows.len(), 1, "lowerValueAt is the one root of RustTerms");
    assert_eq!(rows[0]["root"], format!("{MODULE}.lowerValueAt"));
    assert_eligible(
        &rows[0],
        "lowerValueAt",
        &["allocation", "overflow", "recursion"],
    );
    let runs = explicit();
    let evaluator_seeded = seeded_runs(EVALUATOR_SEEDED, 0x0B59_0103);
    let rustc_seeded = seeded_runs(RUSTC_SEEDED, 0x0B59_0203);
    eprintln!(
        "BS-09: {} explicit runs, {} + {} seeded ({:?})",
        runs.len(),
        evaluator_seeded.len(),
        rustc_seeded.len(),
        started.elapsed()
    );
    assert_eq!(runs.len(), EXPLICIT, "the explicit runs");
    // The checker refuses every literal the lowering refuses, so no run
    // expects a refusal: the root's `none` is a claim about programs the
    // checker accepts, and the corpora measure that it is never the answer.
    let refusal = serde_json::to_value(option(None)).expect("a value is JSON");
    for group in [&runs, &evaluator_seeded, &rustc_seeded] {
        assert!(
            group.iter().all(|(_, expected)| *expected != refusal),
            "the oracle refuses no literal the checker accepts"
        );
    }
    let checked = support::checked_project(&project);
    let values = super::lower(&checked, &project, MODULE, "lowerValueAt");
    super::rust_validate::rustc_differential(
        "BS-09",
        &[("lowerValueAt", &values, &runs)],
        "explicit",
    );
    if !support::normative_host() {
        eprintln!("BS-09: platform-bound host: the evaluator, the seeded corpus, and the planted defects run on the normative host");
        return;
    }
    for (label, group) in [("explicit", &runs), ("seeded", &evaluator_seeded)] {
        let disagreements = super::evaluate(&values, group, 5);
        assert!(
            disagreements.is_empty(),
            "the evaluator and the oracle disagree on the {label} runs: {disagreements:?}"
        );
    }
    super::rust_validate::rustc_differential(
        "BS-09",
        &[("lowerValueAt", &values, &rustc_seeded)],
        "seeded",
    );
    let detection: Vec<Run> = runs.iter().chain(&evaluator_seeded).cloned().collect();
    plants(&detection);
    if !support::lean_backed("BS-09") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        if exported_root.root != format!("{MODULE}.lowerValueAt") {
            continue;
        }
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-09: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}, three quarters of max_file_bytes",
            exported_root.root
        );
        for output in &exported_root.outputs {
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
                serde_json::to_value(&values.program).expect("a program is JSON"),
                "lowerValueAt: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, 1, "one certified package for the root");
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
    eprintln!("BS-09: passed in {:?}", started.elapsed());
}
