//! `BS-06`: the Rust printer of SPEC.md §17.16 as language-1.2 source
//! (`LexLeanBoot.RustPrint`), held to `calculus::rust::ast::{print,
//! print_expr}` to the byte.
//!
//! The roots take the closed Rust AST as data with the payloads a printer
//! reads (`LexLeanBoot.RustAst`), the header of the library (the attributes
//! and the runtime, which the host reads from semantic modules) as an
//! argument, and a capacity for the output buffer.

use std::collections::BTreeMap;

use lexlean::calculus::rust::ast::{
    self, Block, Callee, CaptureRead, Crate, Ctor, Expr, Ident, ItemDef, Let, Lit, Origin, Pat,
};
use lexlean::calculus::rust::Profile;
use lexlean::calculus::{OrderingValue, Value};
use serde_json::Value as Json;

use super::rust_validate::{
    adt, boolean, chain, enc_ctor, enc_dispatch, enc_ident, enc_item, enc_kind, enc_origin,
    enc_row, enc_type, enc_types, nat,
};
use super::{bytes, text, Run};

/// The text a write past the capacity is refused with, as `Buffer.finish`
/// spells it.
pub(super) const REFUSAL: &str = "output exceeds the capacity of the buffer";

/// `OrderingKind`: less, equal, greater.
fn enc_ordering(value: OrderingValue) -> Value {
    adt(
        match value {
            OrderingValue::Lt => 0,
            OrderingValue::Eq => 1,
            OrderingValue::Gt => 2,
        },
        Vec::new(),
    )
}

/// A magnitude that a `Nat` of the target holds: the payloads of literals are
/// at most 64 bits wide, as the widths of the closed AST are.
fn magnitude(value: i128) -> Value {
    nat(u64::try_from(value.unsigned_abs()).expect("a literal of at most 64 bits"))
}

/// `Lit`: unit, bool, nat, int, fixed, str, bytes, ordering, overflow.
fn enc_lit(literal: &Lit) -> Value {
    match literal {
        Lit::Unit => adt(0, Vec::new()),
        Lit::Bool(value) => adt(1, vec![boolean(*value)]),
        Lit::Nat(value) => adt(2, vec![nat(*value)]),
        Lit::Int(value) => adt(3, vec![boolean(*value < 0), magnitude(i128::from(*value))]),
        Lit::Fixed(kind, value) => adt(
            4,
            vec![enc_kind(*kind), boolean(*value < 0), magnitude(*value)],
        ),
        Lit::Str(value) => adt(5, vec![bytes(value.as_bytes())]),
        Lit::Bytes(value) => adt(6, vec![bytes(value)]),
        Lit::Ordering(value) => adt(7, vec![enc_ordering(*value)]),
        Lit::Overflow => adt(8, Vec::new()),
    }
}

fn enc_pats(pats: &[Pat]) -> Value {
    chain(5, 6, pats.iter().map(enc_pat).collect())
}

/// `Pat`: a leaf (wild, unit, none), bind, tuple, a wrapper (some, ok, err),
/// adt, then the chain's nil and cons, then an ordering.
fn enc_pat(pat: &Pat) -> Value {
    let leaf = |kind: u64| adt(0, vec![adt(kind, Vec::new())]);
    let wrap = |kind: u64, inner: &Pat| adt(3, vec![adt(kind, Vec::new()), enc_pat(inner)]);
    match pat {
        Pat::Wild => leaf(0),
        Pat::Unit => leaf(1),
        Pat::None => leaf(2),
        Pat::Ordering(value) => adt(7, vec![enc_ordering(*value)]),
        Pat::Bind(ident) => adt(1, vec![enc_ident(ident)]),
        Pat::Tuple(items) => adt(2, vec![enc_pats(items)]),
        Pat::Some(inner) => wrap(0, inner),
        Pat::Ok(inner) => wrap(1, inner),
        Pat::Err(inner) => wrap(2, inner),
        Pat::Adt {
            adt: n,
            constructor,
            fields,
        } => adt(4, vec![nat(*n), nat(*constructor), enc_pats(fields)]),
    }
}

/// `Callee`: function, runtime row, machine primitive (family, width).
fn enc_callee(callee: &Callee) -> Value {
    match callee {
        Callee::Function(n) => adt(0, vec![nat(*n)]),
        Callee::Runtime(item) => adt(1, vec![enc_item(*item)]),
        Callee::Machine(machine) => adt(
            2,
            vec![
                nat(machine.family()),
                super::option(machine.width().map(enc_kind)),
            ],
        ),
    }
}

/// Expression chains share the sort's `ENil` (12) and `ECons` (13).
fn enc_args(args: &[Expr]) -> Value {
    chain(12, 13, args.iter().map(enc_expr).collect())
}

/// A block (11): lets as a chain of `ELet`, then the tail.
fn enc_block(block: &Block) -> Value {
    let lets = block
        .lets
        .iter()
        .rev()
        .fold(adt(12, Vec::new()), |rest, binding| enc_let(binding, rest));
    adt(11, vec![lets, enc_expr(&block.tail)])
}

/// `ELet` (14): the pattern, whether a type is stated, the type (a unit one
/// when none is), the value, the origin, and the bindings after.
fn enc_let(binding: &Let, rest: Value) -> Value {
    adt(
        14,
        vec![
            enc_pat(&binding.pat),
            boolean(binding.ty.is_some()),
            binding
                .ty
                .as_ref()
                .map_or_else(|| enc_type(&ast::Type::Unit), enc_type),
            enc_expr(&binding.value),
            enc_origin(&binding.at),
            rest,
        ],
    )
}

fn enc_arms(arms: &[(Pat, Block)]) -> Value {
    arms.iter()
        .rev()
        .fold(adt(12, Vec::new()), |rest, (pat, body)| {
            adt(15, vec![enc_pat(pat), enc_block(body), rest])
        })
}

/// `Expr`, as the validator's sort with the payloads: lit, read, wrap, call,
/// apply, construct, pair, if, match, block, probe, then the block, the
/// chain's nil and cons, the let, the arm, and the loop over a state.
pub(super) fn enc_expr(expr: &Expr) -> Value {
    let read = |kind: u64, ident: &Ident, at: &Origin| {
        adt(
            1,
            vec![adt(kind, Vec::new()), enc_ident(ident), enc_origin(at)],
        )
    };
    let wrap = |kind: u64, inner: &Expr, at: &Origin| {
        adt(
            2,
            vec![adt(kind, Vec::new()), enc_expr(inner), enc_origin(at)],
        )
    };
    let probe = |kind: u64, ident: &Ident, at: &Origin| {
        adt(
            10,
            vec![adt(kind, Vec::new()), enc_ident(ident), enc_origin(at)],
        )
    };
    match expr {
        Expr::Lit(literal, at) => adt(0, vec![enc_lit(literal), enc_origin(at)]),
        Expr::Move(ident, at) => read(0, ident, at),
        Expr::Clone(ident, at) => read(1, ident, at),
        Expr::Copy(ident, at) => read(2, ident, at),
        Expr::Deref(ident, at) => read(3, ident, at),
        Expr::Unbox(ident, at) => read(4, ident, at),
        Expr::Not(inner, at) => wrap(0, inner, at),
        Expr::Box(inner, at) => wrap(1, inner, at),
        Expr::Widen(inner, at) => wrap(2, inner, at),
        Expr::Succeed(inner, at) => wrap(3, inner, at),
        Expr::Call {
            callee,
            args,
            propagate,
            at,
        } => adt(
            3,
            vec![
                enc_callee(callee),
                enc_args(args),
                boolean(*propagate),
                enc_origin(at),
            ],
        ),
        Expr::Apply {
            holder,
            args,
            propagate,
            at,
        } => adt(
            4,
            vec![
                enc_ident(holder),
                enc_args(args),
                boolean(*propagate),
                enc_origin(at),
            ],
        ),
        Expr::Construct { ctor, args, at } => {
            adt(5, vec![enc_ctor(ctor), enc_args(args), enc_origin(at)])
        }
        Expr::Pair(left, right, at) => {
            adt(6, vec![enc_expr(left), enc_expr(right), enc_origin(at)])
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
            at,
        } => adt(
            7,
            vec![
                enc_expr(condition),
                enc_block(then_branch),
                enc_block(else_branch),
                enc_origin(at),
            ],
        ),
        Expr::Match {
            scrutinee,
            arms,
            at,
        } => adt(8, vec![enc_expr(scrutinee), enc_arms(arms), enc_origin(at)]),
        Expr::Block(block) => adt(9, vec![enc_block(block)]),
        Expr::IsZero(ident, at) => probe(0, ident, at),
        Expr::NonZero(ident, at) => probe(1, ident, at),
        Expr::Uncons(ident) => probe(2, ident, &Origin::of("")),
        Expr::Predecessor(ident) => probe(3, ident, &Origin::of("")),
        Expr::Loop { state, body } => adt(16, vec![enc_ident(state), enc_block(body)]),
    }
}

/// `Par`: nil, cons (pattern, type, the rest).
fn enc_params(parameters: &[(Pat, ast::Type)]) -> Value {
    parameters
        .iter()
        .rev()
        .fold(adt(0, Vec::new()), |rest, (pat, ty)| {
            adt(1, vec![enc_pat(pat), enc_type(ty), rest])
        })
}

/// `Def`: enum, apply, function, nil, cons.
fn enc_item_def(item: &ItemDef) -> Value {
    match item {
        ItemDef::Enum { name, variants, at } => {
            adt(0, vec![enc_type(name), enc_row(variants), enc_origin(at)])
        }
        ItemDef::Apply {
            fn_type,
            parameters,
            result,
            fallible,
            arms,
            at,
        } => adt(
            1,
            vec![
                nat(*fn_type),
                enc_types(parameters),
                enc_type(result),
                boolean(*fallible),
                enc_dispatch(arms),
                enc_origin(at),
            ],
        ),
        ItemDef::Function {
            name,
            parameters,
            result,
            body,
            at,
        } => adt(
            2,
            vec![
                enc_ident(name),
                enc_params(parameters),
                enc_type(result),
                enc_block(body),
                enc_origin(at),
            ],
        ),
    }
}

/// `Crate`, a structure: whether the profile is `rust-core`, and the items
/// as a `Def` chain.
pub(super) fn enc_crate(krate: &Crate) -> Value {
    let items = krate
        .items
        .iter()
        .rev()
        .fold(adt(3, Vec::new()), |tail, item| {
            adt(4, vec![enc_item_def(item), tail])
        });
    adt(0, vec![boolean(krate.profile == Profile::Core), items])
}

// ------------------------------------------------------------------ oracle

/// What the root returns for `printed` in a buffer of `capacity`.
fn outcome(printed: &str, capacity: usize) -> Json {
    serde_json::to_value(if printed.len() > capacity {
        Value::Error {
            value: Box::new(text(REFUSAL)),
        }
    } else {
        Value::Ok {
            value: Box::new(text(printed)),
        }
    })
    .expect("a value is JSON")
}

/// A run of `printCrate`: the capacity, the header, and the crate; the
/// capacity is the printed length plus `slack` below or above it.
pub(super) fn print_run(krate: &Crate, header: &str, capacity: usize) -> Option<Run> {
    // The oracle indexes the operands of a machine call, so a crate whose
    // call has fewer than its primitive's is outside the claim.
    let printed = catch(|| {
        let body = print_body(krate);
        format!("{header}{body}")
    })?;
    Some((
        vec![
            nat(capacity as u64),
            bytes(header.as_bytes()),
            enc_crate(krate),
        ],
        outcome(&printed, capacity),
    ))
}

/// The items of `krate` as `ast::print` lays them out, after the header.
fn print_body(krate: &Crate) -> String {
    let mut out = String::new();
    for item in &krate.items {
        out.push('\n');
        out.push_str(&ast::print_item(item));
    }
    out
}

/// A run of `printExpr`.
pub(super) fn expr_run(expr: &Expr, capacity: usize) -> Option<Run> {
    let printed = catch(|| ast::print_expr(expr))?;
    Some((
        vec![nat(capacity as u64), enc_expr(expr)],
        outcome(&printed, capacity),
    ))
}

/// The oracle's text of `body`, or `None` when it panics on an operand it
/// does not have: the claim leaves those crates out.
fn catch<T>(body: impl FnOnce() -> T + std::panic::UnwindSafe) -> Option<T> {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(body).ok();
    std::panic::set_hook(hook);
    result
}

// ------------------------------------------------------------- hand-built

fn at() -> Origin {
    Origin::of("hand")
}

fn local(n: u64) -> Ident {
    Ident::Local(n)
}

fn read(n: u64) -> Expr {
    Expr::Move(local(n), at())
}

fn unit() -> Expr {
    Expr::Lit(Lit::Unit, at())
}

fn block(lets: Vec<Let>, tail: Expr) -> Block {
    Block { lets, tail }
}

fn bind(n: u64, value: Expr) -> Let {
    Let {
        pat: Pat::Bind(local(n)),
        ty: None,
        value,
        at: at(),
    }
}

fn call(callee: Callee, args: Vec<Expr>, propagate: bool) -> Expr {
    Expr::Call {
        callee,
        args,
        propagate,
        at: at(),
    }
}

/// The strings of the escaper: every ASCII scalar, the quote and the
/// backslash on their own, the widths of UTF-8 at their edges, and a mix.
fn strings() -> Vec<String> {
    let mut out: Vec<String> = vec![
        String::new(),
        "a".to_owned(),
        "\"".to_owned(),
        "\\".to_owned(),
        "\\\"\\".to_owned(),
        (0u8..=127).map(char::from).collect(),
        "\u{7f}".to_owned(),
        "\u{80}".to_owned(),
        "\u{ff}".to_owned(),
        "\u{7ff}".to_owned(),
        "\u{800}".to_owned(),
        "\u{ffff}".to_owned(),
        "\u{10000}".to_owned(),
        "\u{10ffff}".to_owned(),
        "caf\u{e9} \u{65e5}\u{672c} \u{1f389}".to_owned(),
    ];
    out.extend((0u8..=127).map(|byte| char::from(byte).to_string()));
    out
}

/// Every literal: the edges of each width and sign, the strings, and the
/// byte strings.
fn literals() -> Vec<Lit> {
    let mut out = vec![
        Lit::Unit,
        Lit::Bool(true),
        Lit::Bool(false),
        Lit::Nat(0),
        Lit::Nat(9),
        Lit::Nat(10),
        Lit::Nat(u64::MAX),
        Lit::Nat(1 << 63),
        Lit::Int(0),
        Lit::Int(-1),
        Lit::Int(1),
        Lit::Int(i64::MAX),
        Lit::Int(i64::MIN),
        Lit::Int(i64::MIN + 1),
        Lit::Ordering(OrderingValue::Lt),
        Lit::Ordering(OrderingValue::Eq),
        Lit::Ordering(OrderingValue::Gt),
        Lit::Overflow,
        Lit::Bytes(Vec::new()),
        Lit::Bytes(vec![0]),
        Lit::Bytes(vec![255, 0, 16]),
        Lit::Bytes((0..=255).collect()),
    ];
    for kind in lexlean::calculus::IntKind::ALL {
        let (low, high) = kind.range();
        for value in [
            0,
            1,
            high,
            high - 1,
            low,
            low + 1,
            if kind.signed() { -1 } else { 2 },
        ] {
            out.push(Lit::Fixed(kind, value));
        }
    }
    out.extend(strings().into_iter().map(Lit::Str));
    out
}

fn idents() -> Vec<Ident> {
    vec![
        Ident::Local(0),
        Ident::Local(u64::MAX),
        Ident::Holder(3),
        Ident::Operand(12),
        Ident::Callee(7),
        Ident::Boxed(5),
        Ident::Part(1),
        Ident::Capture(2),
        Ident::Param(9),
        Ident::Function(100),
        Ident::Export("run".to_owned()),
        Ident::Export("a_name_with_digits_42".to_owned()),
    ]
}

fn types() -> Vec<ast::Type> {
    use ast::Type as T;
    let nat = || Box::new(T::Nat);
    let mut out = vec![
        T::Unit,
        T::Bool,
        T::Nat,
        T::Int,
        T::Ordering,
        T::Str,
        T::Bytes,
        T::Overflow,
        T::I128,
        T::Adt(0),
        T::Adt(31),
        T::Fn(0),
        T::Fn(7),
        T::Option(nat()),
        T::List(Box::new(T::Option(nat()))),
        T::Rc(Box::new(T::Adt(2))),
        T::Fallible(nat()),
        T::Fallible(Box::new(T::Unit)),
        T::Ref(Box::new(T::Bytes)),
        T::Result(nat(), Box::new(T::Str)),
        T::Pair(
            nat(),
            Box::new(T::Pair(Box::new(T::Bool), Box::new(T::Unit))),
        ),
    ];
    out.extend(lexlean::calculus::IntKind::ALL.map(T::Fixed));
    out
}

fn patterns() -> Vec<Pat> {
    let bind = |n| Pat::Bind(local(n));
    vec![
        Pat::Wild,
        Pat::Unit,
        Pat::None,
        bind(4),
        Pat::Tuple(Vec::new()),
        Pat::Tuple(vec![bind(1)]),
        Pat::Tuple(vec![
            bind(1),
            Pat::Wild,
            Pat::Tuple(vec![bind(2), Pat::Unit]),
        ]),
        Pat::Some(Box::new(bind(1))),
        Pat::Ok(Box::new(Pat::Wild)),
        Pat::Err(Box::new(Pat::Some(Box::new(bind(3))))),
        Pat::Ordering(OrderingValue::Lt),
        Pat::Ordering(OrderingValue::Eq),
        Pat::Ordering(OrderingValue::Gt),
        Pat::Adt {
            adt: 3,
            constructor: 0,
            fields: Vec::new(),
        },
        Pat::Adt {
            adt: 3,
            constructor: 2,
            fields: vec![bind(1), Pat::Wild],
        },
    ]
}

/// Expressions of every construct, with the forms the printer decides by the
/// shape of its parts: a negated comparison, an `else` that is empty, a
/// scrutinee that is a block, a loop, a machine call with extra operands.
#[allow(clippy::too_many_lines)]
fn expressions() -> Vec<Expr> {
    let mut out: Vec<Expr> = literals()
        .into_iter()
        .map(|lit| Expr::Lit(lit, at()))
        .collect();
    for ident in idents() {
        out.extend([
            Expr::Move(ident.clone(), at()),
            Expr::Clone(ident.clone(), at()),
            Expr::Copy(ident.clone(), at()),
            Expr::Deref(ident.clone(), at()),
            Expr::Unbox(ident.clone(), at()),
            Expr::IsZero(ident.clone(), at()),
            Expr::NonZero(ident.clone(), at()),
            Expr::Uncons(ident.clone()),
            Expr::Predecessor(ident.clone()),
            Expr::Not(Box::new(Expr::IsZero(ident.clone(), at())), at()),
            Expr::Not(Box::new(Expr::NonZero(ident.clone(), at())), at()),
            Expr::Not(Box::new(Expr::Predecessor(ident.clone())), at()),
            Expr::Not(Box::new(Expr::Uncons(ident.clone())), at()),
            Expr::Not(Box::new(Expr::Move(ident.clone(), at())), at()),
            Expr::Apply {
                holder: ident.clone(),
                args: vec![read(1), read(2)],
                propagate: true,
                at: at(),
            },
            Expr::Apply {
                holder: ident,
                args: Vec::new(),
                propagate: false,
                at: at(),
            },
        ]);
    }
    out.extend([
        Expr::Box(Box::new(read(1)), at()),
        Expr::Widen(Box::new(read(1)), at()),
        Expr::Succeed(Box::new(unit()), at()),
        Expr::Pair(
            Box::new(read(1)),
            Box::new(Expr::Pair(Box::new(read(2)), Box::new(unit()), at())),
            at(),
        ),
    ]);
    // Constructors, with and without operands.
    for ty in types().into_iter().take(12) {
        out.push(Expr::Construct {
            ctor: Ctor::None(ty.clone()),
            args: Vec::new(),
            at: at(),
        });
        out.push(Expr::Construct {
            ctor: Ctor::Nil(ty.clone()),
            args: Vec::new(),
            at: at(),
        });
        out.push(Expr::Construct {
            ctor: Ctor::Ok(ty.clone(), ast::Type::Str),
            args: vec![read(1)],
            at: at(),
        });
        out.push(Expr::Construct {
            ctor: Ctor::Err(ast::Type::Nat, ty),
            args: vec![read(1)],
            at: at(),
        });
    }
    for args in [Vec::new(), vec![read(1)], vec![read(1), read(2), read(3)]] {
        for ctor in [
            Ctor::Some,
            Ctor::Cons,
            Ctor::Adt {
                adt: 4,
                constructor: 1,
            },
            Ctor::Closure {
                fn_type: 2,
                function: 5,
            },
        ] {
            out.push(Expr::Construct {
                ctor,
                args: args.clone(),
                at: at(),
            });
        }
    }
    // Runtime rows and machine primitives, with their operands, with one too
    // many, propagating or not.
    for item in lexlean::calculus::rust::runtime::Item::all() {
        for count in 0..=2 {
            out.push(call(
                Callee::Runtime(item),
                (0..count).map(read).collect(),
                count == 1,
            ));
        }
    }
    for machine in ast::Machine::all() {
        let arity = machine.arity();
        for extra in 0..=1 {
            for propagate in [false, true] {
                out.push(call(
                    Callee::Machine(machine),
                    (0..(arity + extra) as u64).map(read).collect(),
                    propagate,
                ));
            }
        }
    }
    out.push(call(Callee::Function(3), vec![read(1), unit()], true));
    out.push(call(Callee::Function(0), Vec::new(), false));
    // Blocks, conditionals, matches, and loops.
    let unit_block = || block(Vec::new(), unit());
    let value_block = || block(vec![bind(1, read(0))], read(1));
    let typed = Let {
        pat: Pat::Tuple(vec![Pat::Bind(local(1)), Pat::Wild]),
        ty: Some(ast::Type::Pair(
            Box::new(ast::Type::Nat),
            Box::new(ast::Type::Bool),
        )),
        value: read(0),
        at: at(),
    };
    out.extend([
        Expr::Block(Box::new(unit_block())),
        Expr::Block(Box::new(value_block())),
        Expr::Block(Box::new(block(
            vec![typed.clone(), bind(2, unit())],
            unit(),
        ))),
        Expr::Block(Box::new(block(vec![typed], read(1)))),
        Expr::If {
            condition: Box::new(read(0)),
            then_branch: Box::new(value_block()),
            else_branch: Box::new(unit_block()),
            at: at(),
        },
        Expr::If {
            condition: Box::new(Expr::IsZero(local(0), at())),
            then_branch: Box::new(unit_block()),
            else_branch: Box::new(value_block()),
            at: at(),
        },
        Expr::If {
            condition: Box::new(read(0)),
            then_branch: Box::new(unit_block()),
            else_branch: Box::new(block(vec![bind(1, unit())], unit())),
            at: at(),
        },
    ]);
    let arms = |count: usize| -> Vec<(Pat, Block)> {
        patterns()
            .into_iter()
            .take(count)
            .map(|pattern| (pattern, value_block()))
            .collect()
    };
    let scrutinees = vec![
        read(0),
        Expr::Block(Box::new(value_block())),
        Expr::Loop {
            state: local(0),
            body: Box::new(value_block()),
        },
        call(
            Callee::Machine(ast::Machine::IntRem),
            vec![read(0), read(1), read(2)],
            false,
        ),
        call(
            Callee::Machine(ast::Machine::CheckedIntQuot),
            vec![read(0), read(1), read(2)],
            true,
        ),
    ];
    for scrutinee in scrutinees {
        for count in [0, 1, 4] {
            let inner = Expr::Match {
                scrutinee: Box::new(scrutinee.clone()),
                arms: arms(count),
                at: at(),
            };
            out.push(inner.clone());
            // A match that is itself a scrutinee, and one in a block.
            out.push(Expr::Match {
                scrutinee: Box::new(inner.clone()),
                arms: arms(2),
                at: at(),
            });
            out.push(Expr::Block(Box::new(block(vec![bind(1, inner)], read(1)))));
        }
    }
    out.push(Expr::Loop {
        state: local(0),
        body: Box::new(unit_block()),
    });
    out.push(Expr::Loop {
        state: Ident::Holder(2),
        body: Box::new(block(
            vec![bind(
                1,
                Expr::If {
                    condition: Box::new(Expr::IsZero(local(0), at())),
                    then_branch: Box::new(block(
                        Vec::new(),
                        Expr::Construct {
                            ctor: Ctor::Err(ast::Type::Nat, ast::Type::Nat),
                            args: vec![read(0)],
                            at: at(),
                        },
                    )),
                    else_branch: Box::new(block(
                        Vec::new(),
                        Expr::Construct {
                            ctor: Ctor::Ok(ast::Type::Nat, ast::Type::Nat),
                            args: vec![Expr::Predecessor(local(0))],
                            at: at(),
                        },
                    )),
                    at: at(),
                },
            )],
            read(1),
        )),
    });
    for pattern in patterns() {
        out.push(Expr::Match {
            scrutinee: Box::new(read(0)),
            arms: vec![(pattern, unit_block())],
            at: at(),
        });
    }
    out
}

/// Crates of every item: enums of a data type and of a function type, the
/// dispatch of every capture read with and without a failure and with a unit
/// result, and functions with their parameters.
fn crates() -> Vec<Crate> {
    use ast::Type as T;
    let mut items = Vec::new();
    items.push(ItemDef::Enum {
        name: T::Adt(0),
        variants: vec![
            (0, Vec::new()),
            (1, vec![T::Nat]),
            (2, vec![T::Str, T::List(Box::new(T::Bool))]),
        ],
        at: at(),
    });
    items.push(ItemDef::Enum {
        name: T::Fn(0),
        variants: vec![
            (3, Vec::new()),
            (4, vec![T::Rc(Box::new(T::Fn(0))), T::Unit]),
        ],
        at: at(),
    });
    items.push(ItemDef::Enum {
        name: T::Fn(1),
        variants: Vec::new(),
        at: at(),
    });
    let captures = vec![
        CaptureRead::Copy,
        CaptureRead::Clone,
        CaptureRead::Unbox,
        CaptureRead::Unit,
    ];
    for (parameters, result, fallible) in [
        (vec![T::Nat, T::Bool], T::Nat, false),
        (vec![T::Nat], T::Unit, true),
        (vec![T::Nat], T::Bool, true),
        (Vec::new(), T::Unit, false),
        (Vec::new(), T::Str, true),
    ] {
        for function_fallible in [false, true] {
            items.push(ItemDef::Apply {
                fn_type: 0,
                parameters: parameters.clone(),
                result: result.clone(),
                fallible,
                arms: vec![
                    ast::Dispatch {
                        function: 3,
                        captures: Vec::new(),
                        function_fallible,
                    },
                    ast::Dispatch {
                        function: 4,
                        captures: captures.clone(),
                        function_fallible: !function_fallible,
                    },
                ],
                at: at(),
            });
        }
    }
    items.push(ItemDef::Apply {
        fn_type: 1,
        parameters: vec![T::Nat, T::Str],
        result: T::Unit,
        fallible: false,
        arms: Vec::new(),
        at: at(),
    });
    items.push(ItemDef::Apply {
        fn_type: 1,
        parameters: Vec::new(),
        result: T::Unit,
        fallible: true,
        arms: Vec::new(),
        at: at(),
    });
    for result in [T::Unit, T::Nat, T::Fallible(Box::new(T::Unit))] {
        for parameters in [
            Vec::new(),
            vec![(Pat::Bind(Ident::Param(0)), T::Nat)],
            vec![
                (Pat::Wild, T::Unit),
                (Pat::Bind(Ident::Param(1)), T::Ref(Box::new(T::Bytes))),
            ],
            vec![(
                Pat::Tuple(vec![Pat::Wild, Pat::Bind(local(2))]),
                T::Pair(Box::new(T::Nat), Box::new(T::Nat)),
            )],
        ] {
            for body in [
                block(Vec::new(), unit()),
                block(vec![bind(1, read(0))], unit()),
                block(vec![bind(1, read(0))], read(1)),
            ] {
                items.push(ItemDef::Function {
                    name: Ident::Export("run".to_owned()),
                    parameters: parameters.clone(),
                    result: result.clone(),
                    body,
                    at: at(),
                });
            }
        }
    }
    items.push(ItemDef::Function {
        name: Ident::Function(0),
        parameters: vec![(Pat::Bind(local(0)), T::Nat)],
        result: T::Nat,
        body: block(
            Vec::new(),
            Expr::Loop {
                state: local(0),
                body: Box::new(block(Vec::new(), read(0))),
            },
        ),
        at: at(),
    });
    // Each profile with no item, with one, and with all.
    let mut out = Vec::new();
    for profile in [Profile::Core, Profile::Std] {
        out.push(Crate {
            profile,
            items: Vec::new(),
        });
        out.push(Crate {
            profile,
            items: items[..1].to_vec(),
        });
        out.push(Crate {
            profile,
            items: items.clone(),
        });
    }
    // The expressions of the hand-built list, one function each, so a crate
    // of them is the printer's text of an item and not only of an expression.
    for chunk in expressions().chunks(40) {
        let body = block(
            chunk
                .iter()
                .enumerate()
                .map(|(at_, expr)| bind(at_ as u64, expr.clone()))
                .collect(),
            unit(),
        );
        out.push(Crate {
            profile: Profile::Std,
            items: vec![ItemDef::Function {
                name: Ident::Function(1),
                parameters: Vec::new(),
                result: T::Unit,
                body,
                at: at(),
            }],
        });
    }
    out
}

// ----------------------------------------------------------------- vectors

use std::collections::BTreeSet;

use lexlean::calculus::rust::package::{self, Manifest};

use crate::support;

/// A committed package: its crate as the package renderer lowers it (the
/// exports included), and the library it committed.
struct Packaged {
    krate: Crate,
    library: String,
}

/// The crate and the committed `src/lib.rs` of every committed package
/// manifest under `compiler/rust` and `tests/production-export`.
fn packaged() -> Vec<Packaged> {
    let root = support::repo_root();
    let mut files = Vec::new();
    super::rust_validate::walk(root.as_std_path(), &mut files);
    let mut out = Vec::new();
    for path in files {
        let relative = path
            .strip_prefix(root.as_std_path())
            .expect("under the root")
            .to_string_lossy()
            .replace('\\', "/");
        if path.file_name().is_none_or(|name| name != "package.json")
            || !(relative.starts_with("compiler/rust/")
                || relative.starts_with("tests/production-export/"))
        {
            continue;
        }
        let manifest = Manifest::parse(&std::fs::read(&path).expect("a manifest"))
            .unwrap_or_else(|error| panic!("{relative}: {error}"));
        let (krate, _, _) =
            package::lower(&manifest).unwrap_or_else(|error| panic!("{relative}: {error}"));
        let library = std::fs::read_to_string(path.with_file_name("src").join("lib.rs"))
            .unwrap_or_else(|error| panic!("{relative}: {error}"));
        out.push(Packaged { krate, library });
    }
    out
}

/// The header a profile's library begins with: the attributes and the
/// runtime, as `ast::print` writes them before any item.
fn header(profile: Profile) -> String {
    ast::print(&Crate {
        profile,
        items: Vec::new(),
    })
    .expect("the header of a profile")
}

/// The refusals the vectors show for a capacity one short of the output.
fn tight(krate: &Crate, header: &str) -> Option<Run> {
    let printed = catch(|| format!("{header}{}", print_body(krate)))?;
    print_run(krate, header, printed.len().saturating_sub(1))
}

/// A run of `printCrate` with a capacity of exactly the output.
fn exact(krate: &Crate, header: &str) -> Option<Run> {
    let printed = catch(|| format!("{header}{}", print_body(krate)))?;
    print_run(krate, header, printed.len())
}

/// The function bodies of a crate as expressions: the block of each.
fn bodies(krate: &Crate) -> Vec<Expr> {
    krate
        .items
        .iter()
        .filter_map(|item| match item {
            ItemDef::Function { body, .. } => Some(Expr::Block(Box::new(body.clone()))),
            _ => None,
        })
        .collect()
}

/// `printExpr` with a capacity of exactly the output.
fn exact_expr(expr: &Expr) -> Option<Run> {
    let printed = catch(|| ast::print_expr(expr))?;
    expr_run(expr, printed.len())
}

/// A defect planted in the source and the root whose differential catches it.
const PLANTS: [super::Plant; 12] = [
    super::Plant {
        label: "the quote is not escaped",
        site: "(if (code =? 34) then (putS (second state) \"\\\\\\\"\")",
        replacement: "(if (code =? 34) then (putS (second state) \"\\\"\")",
        root: "printExpr",
    },
    super::Plant {
        label: "the hexadecimal of a scalar is in upper case",
        site: "0x30313233343536373839616263646566",
        replacement: "0x30313233343536373839414243444546",
        root: "printExpr",
    },
    super::Plant {
        label: "the delete character is printed as it is",
        site: "((32 <=? code) && (code <=? 126))",
        replacement: "((32 <=? code) && (code <=? 127))",
        root: "printExpr",
    },
    super::Plant {
        label: "the least i64 is spelled as a negative literal",
        site: "(if (negative && (magnitude =? 9223372036854775808)) then (putS out \"i64::MIN\")",
        replacement: "(if (negative && (magnitude =? 9223372036854775809)) then (putS out \"i64::MIN\")",
        root: "printExpr",
    },
    super::Plant {
        label: "a width's least value is spelled as a literal",
        site: "(if (negative && (magnitude =? (minimum kind))) then",
        replacement: "(if (negative && (magnitude =? 0)) then",
        root: "printExpr",
    },
    super::Plant {
        label: "the byte string separator has no space",
        site: "(putS (if ((first state) =? 0) then (second state) else (putS (second state) \", \")) \"0x\")",
        replacement: "(putS (if ((first state) =? 0) then (second state) else (putS (second state) \",\")) \"0x\")",
        root: "printExpr",
    },
    super::Plant {
        label: "a unit result is printed as an arrow",
        site: "(if (tyIsUnit result) then out else (putTy (putS out \" -> \") result))",
        replacement: "(if (tyIsUnit result) then (putS out \" -> ()\") else (putTy (putS out \" -> \") result))",
        root: "printCrate",
    },
    super::Plant {
        label: "the indentation is three spaces",
        site: "(putS (second state) \"    \")",
        replacement: "(putS (second state) \"   \")",
        root: "printExpr",
    },
    super::Plant {
        label: "a comparison under a negation is not parenthesized",
        site: "(((probeCode inner) =? 0) || (((probeCode inner) =? 1) || ((probeCode inner) =? 3)))",
        replacement: "(((probeCode inner) =? 0) || ((probeCode inner) =? 1))",
        root: "printExpr",
    },
    super::Plant {
        label: "an empty else is written out",
        site: "(if (isUnitBlock elseBranch) then head else",
        replacement: "(if false then head else",
        root: "printExpr",
    },
    super::Plant {
        label: "the blank line between items is dropped",
        site: "(putDefs (putItemDef (putS out \"\\n\") item) rest)",
        replacement: "(putDefs (putItemDef out item) rest)",
        root: "printCrate",
    },
    super::Plant {
        label: "the empty dispatch is matched with arms",
        site: "(if unread then (putS opened \"        match *self {}\\n\")",
        replacement: "(if false then (putS opened \"        match *self {}\\n\")",
        root: "printCrate",
    },
];

// ------------------------------------------------------------------- case

const MODULE: &str = "LexLeanBoot.RustPrint";
const ROOTS: [(&str, &[&str]); 2] = [
    ("printCrate", &["allocation", "overflow", "recursion"]),
    ("printExpr", &["allocation", "overflow", "recursion"]),
];

// The counts SPEC.md §17.23, the register statement of BS-06, its scenario,
// and VERIFICATION.md state, over the tree with the packages of the two roots
// committed: the case fails when the tree no longer gives them.
const COMMITTED_MANIFESTS: usize = 256;
const COMMITTED_FIXTURES: usize = 122;
const COMMITTED_PROGRAMS: usize = 173;
const COMMITTED_CRATES: usize = 232;
const COMMITTED_PACKAGES: usize = 256;
const VECTORS_CRATE: usize = 607;
const COMMITTED_DISTINCT: usize = 464;
const REFUSED_CRATE: usize = 57;
const VECTORS_EXPR: usize = 1_955;
const HAND_EXPR: usize = 1_404;
const EVALUATOR_CRATE: usize = 542;
const SEEDED_CRATE: usize = 400;
const SEEDED_EXPR: usize = 399;
const RUSTC_SEEDED_RUNS: usize = 2_984;

/// The crates the evaluator is given in full: those of at most this many
/// expressions.
const EVALUATOR_CAP: usize = 400;
/// Crates a mutation corpus draws its bases from.
const BASE_CAP: usize = 80;
/// Seeded mutants per root under the evaluator.
const EVALUATOR_MUTANTS: usize = 400;
/// Further seeded mutants per root under rustc.
const RUSTC_MUTANTS: usize = 1_500;

fn eligibility(project: &support::P) {
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanBoot/RustPrint.eligibility.json")
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
}

/// `count` seeded mutants of the crates of `bases`, each as a run of
/// `printCrate`, and the same crates' first function body as a run of
/// `printExpr`.
fn mutants(
    bases: &[&super::rust_validate::Rendered],
    seeded: &mut super::Seeded,
    count: usize,
) -> (Vec<Run>, Vec<Run>) {
    let pools: Vec<_> = bases
        .iter()
        .map(|base| super::rust_validate::pool_of(&base.krate))
        .collect();
    let (mut crates, mut exprs) = (Vec::new(), Vec::new());
    for _ in 0..count {
        let which = seeded.below(bases.len());
        let krate = super::rust_validate::mutant(&bases[which].krate, seeded, &pools[which]);
        if let Some(run) = exact(&krate, "") {
            crates.push(run);
        }
        if let Some(run) = bodies(&krate).first().and_then(exact_expr) {
            exprs.push(run);
        }
    }
    (crates, exprs)
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("RustPrint")
            && checked_units.units.contains_key("RustAst")
            && checked_units.units.contains_key("LexLeanStd.Buffer"),
        "RustPrint, RustAst, and LexLeanStd.Buffer are linked modules of the project"
    );
    eligibility(&project);

    let (sources, manifests, fixtures, distinct) = super::rust_validate::committed();
    let all = super::rust_validate::rendered(&sources);
    let packages = packaged();
    // The committed library of every package ends with the items as the oracle
    // prints them. The header before them is the runtime of the committed
    // export, which the regeneration of the records moves with the runtime, so
    // the items are what the committed libraries pin; the header is held by
    // the vectors that give the roots the oracle's own header.
    let (core_header, std_header) = (header(Profile::Core), header(Profile::Std));
    for package in &packages {
        assert!(
            package.library.ends_with(&print_body(&package.krate)),
            "the committed library ends with the printed items"
        );
    }
    assert_eq!(
        (manifests, fixtures, distinct, all.len(), packages.len()),
        (
            COMMITTED_MANIFESTS,
            COMMITTED_FIXTURES,
            COMMITTED_PROGRAMS,
            COMMITTED_CRATES,
            COMMITTED_PACKAGES
        ),
        "the committed manifests, fixtures, distinct programs, distinct crates, and packages"
    );
    eprintln!(
        "BS-06: {manifests} manifests, {fixtures} fixtures, {distinct} programs, {} crates, {} packages ({:?})",
        all.len(),
        packages.len(),
        started.elapsed()
    );

    // Vectors of printCrate: the crates the renderer lowers, the crates of
    // the committed packages, the hand-built crates, each with the capacity
    // of its output, a few one byte short, and the real headers.
    let mut seen = BTreeSet::new();
    let mut crate_runs: Vec<Run> = Vec::new();
    let committed_crates = all
        .iter()
        .map(|rendered| &rendered.krate)
        .chain(packages.iter().map(|package| &package.krate));
    for krate in committed_crates {
        if seen.insert(format!("{krate:?}")) {
            crate_runs.extend(exact(krate, ""));
        }
    }
    let committed_count = crate_runs.len();
    for (at_, krate) in crates().iter().enumerate() {
        crate_runs.extend(exact(krate, ""));
        crate_runs.extend(exact(krate, "// header\n"));
        if at_ % 2 == 0 {
            crate_runs.extend(tight(krate, "// header\n"));
        }
    }
    for rendered in all.iter().step_by(7) {
        crate_runs.extend(tight(&rendered.krate, ""));
    }
    for (profile, header) in [(Profile::Core, &core_header), (Profile::Std, &std_header)] {
        let krate = &crates()[if profile == Profile::Core { 2 } else { 5 }];
        crate_runs.extend(exact(krate, header));
        crate_runs.extend(tight(krate, header));
    }
    let mut expr_runs: Vec<Run> = expressions().iter().filter_map(exact_expr).collect();
    let hand_exprs = expr_runs.len();
    for rendered in small_crates(&all, EVALUATOR_CAP) {
        expr_runs.extend(bodies(&rendered.krate).iter().filter_map(exact_expr));
    }
    let refused = |runs: &[Run]| {
        runs.iter()
            .filter(|(_, outcome)| outcome["kind"] == "error")
            .count()
    };
    eprintln!(
        "BS-06: {} printCrate vectors ({} committed distinct, {} refused), {} printExpr vectors ({hand_exprs} hand-built, {} refused) ({:?})",
        crate_runs.len(),
        committed_count,
        refused(&crate_runs),
        expr_runs.len(),
        refused(&expr_runs),
        started.elapsed()
    );

    assert_eq!(
        (
            crate_runs.len(),
            committed_count,
            refused(&crate_runs),
            expr_runs.len(),
            hand_exprs,
            refused(&expr_runs)
        ),
        (
            VECTORS_CRATE,
            COMMITTED_DISTINCT,
            REFUSED_CRATE,
            VECTORS_EXPR,
            HAND_EXPR,
            0
        ),
        "the vectors of printCrate and printExpr, the committed crates among them, and the refused"
    );
    // The corpora of the normative host are made here, so that every count
    // the row states is asserted before the first leg runs.
    let evaluator_crates: Vec<Run> = crate_runs
        .iter()
        .filter(|(arguments, _)| value_nodes_of(arguments) <= EVALUATOR_NODES)
        .cloned()
        .collect();
    let evaluator_exprs: Vec<Run> = expr_runs
        .iter()
        .filter(|(arguments, _)| value_nodes_of(arguments) <= EVALUATOR_NODES)
        .cloned()
        .collect();
    let bases = super::rust_validate::small(&all, BASE_CAP);
    let mut seeded = super::Seeded(0x0B55_0006);
    let (seeded_crates, seeded_exprs) = mutants(&bases, &mut seeded, EVALUATOR_MUTANTS);
    let mut large = super::Seeded(0x0B55_0066);
    let (large_crates, large_exprs) = mutants(&bases, &mut large, RUSTC_MUTANTS);
    assert_eq!(
        (
            evaluator_crates.len(),
            evaluator_exprs.len(),
            seeded_crates.len(),
            seeded_exprs.len(),
            large_crates.len() + large_exprs.len()
        ),
        (
            EVALUATOR_CRATE,
            VECTORS_EXPR,
            SEEDED_CRATE,
            SEEDED_EXPR,
            RUSTC_SEEDED_RUNS
        ),
        "the vectors and the seeded mutants of the evaluator, and the further mutants of rustc"
    );
    let checked = support::checked_project(&project);
    let crate_root = super::lower(&checked, &project, MODULE, "printCrate");
    let expr_root = super::lower(&checked, &project, MODULE, "printExpr");
    // On every host: the rendered rust-std packages of both roots, compiled by
    // that host's pinned rustc, agree with the oracle on the vectors.
    super::rust_validate::rustc_differential(
        "BS-06",
        &[
            ("printCrate", &crate_root, &crate_runs),
            ("printExpr", &expr_root, &expr_runs),
        ],
        "vectors",
    );
    if !support::normative_host() {
        eprintln!("BS-06: platform-bound host: the evaluator, the seeded corpora, and the planted defects run on the normative host");
        return;
    }
    for (root, low, runs) in [
        ("printCrate", &crate_root, &evaluator_crates),
        ("printExpr", &expr_root, &evaluator_exprs),
    ] {
        let disagreements = super::evaluate(low, runs, 5);
        assert!(
            disagreements.is_empty(),
            "{root}: the evaluator and the Rust printer disagree on the vectors: {disagreements:?}"
        );
    }
    eprintln!(
        "BS-06: the evaluator agrees on {} + {} vectors ({:?})",
        evaluator_crates.len(),
        evaluator_exprs.len(),
        started.elapsed()
    );
    for (root, low, runs) in [
        ("printCrate", &crate_root, &seeded_crates),
        ("printExpr", &expr_root, &seeded_exprs),
    ] {
        let disagreements = super::evaluate(low, runs, 5);
        assert!(
            disagreements.is_empty(),
            "{root}: the evaluator and the Rust printer disagree on the seeded mutants: {disagreements:?}"
        );
    }
    eprintln!(
        "BS-06: the evaluator agrees on {} + {} seeded mutants ({:?})",
        seeded_crates.len(),
        seeded_exprs.len(),
        started.elapsed()
    );
    let large_runs = super::rust_validate::rustc_differential(
        "BS-06",
        &[
            ("printCrate", &crate_root, &large_crates),
            ("printExpr", &expr_root, &large_exprs),
        ],
        "seeded",
    );
    assert_eq!(large_runs, RUSTC_SEEDED_RUNS);
    // The gate is armed: each defect planted in a copy of the source is a
    // disagreement with the oracle.
    let mut detection: BTreeMap<&str, Vec<Run>> = BTreeMap::new();
    detection.insert(
        "printCrate",
        evaluator_crates
            .iter()
            .chain(&seeded_crates)
            .cloned()
            .collect(),
    );
    detection.insert(
        "printExpr",
        evaluator_exprs
            .iter()
            .chain(&seeded_exprs)
            .cloned()
            .collect(),
    );
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/RustPrint.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/RustPrint.lex.tex",
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
            "BS-06: plant detected: {}: {}",
            plant.label, disagreements[0]
        );
    }
    // Under the pinned Lean, the production export certifies both roots; its
    // packages are the committed ones and realize the programs the
    // differential ran.
    if !support::lean_backed("BS-06") {
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
        let low = if member == "printCrate" {
            &crate_root
        } else {
            &expr_root
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
                serde_json::to_value(&low.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    eprintln!("BS-06: passed in {:?}", started.elapsed());
}

/// The nodes of the evaluator's share of a run: a crate or an expression of
/// at most this many nodes of data.
const EVALUATOR_NODES: usize = 6_000;

fn value_nodes_of(arguments: &[Value]) -> usize {
    arguments
        .iter()
        .map(super::rust_validate::value_nodes)
        .sum()
}

fn small_crates(
    all: &[super::rust_validate::Rendered],
    limit: usize,
) -> Vec<&super::rust_validate::Rendered> {
    super::rust_validate::small(all, limit)
}

use crate::production_export as harness;
