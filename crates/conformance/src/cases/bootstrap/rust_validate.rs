//! `BS-05` (SPEC.md §17.23, §17.16): the Rust crate validator as language-1.2
//! source, held to `lexlean::calculus::rust::validate::{validate, correspond}`.
//!
//! The roots take the crate as module data, the constructors of the module's
//! inductives in declaration order, so the encoders below are the one place
//! that order is written down. The runtime function a call names is data of
//! the module's `Rt` sort and the module transcribes what `runtime::Item`
//! says of it; lowering, printing, and packaging are not part of this row.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::rust::ast::{
    Block, Callee, CaptureRead, Crate, Ctor, Dispatch, Expr, Ident, ItemDef, Let, Lit, Machine,
    Origin, Pat, Type,
};
use lexlean::calculus::rust::runtime::Item;
use lexlean::calculus::{IntKind, Value};

use super::{list, option, snake, text, Run, Seeded};
use crate::production_export as harness;
use crate::rust_build::cargo_in;
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support;
use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::{self as target};

pub(super) fn nat(number: u64) -> Value {
    Value::Nat {
        value: number.to_string(),
    }
}

pub(super) fn adt(constructor: u64, fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor,
        fields,
    }
}

pub(super) fn boolean(value: bool) -> Value {
    Value::Bool { value }
}

pub(super) fn kind_index(kind: IntKind) -> u64 {
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

pub(super) fn enc_kind(kind: IntKind) -> Value {
    adt(kind_index(kind), Vec::new())
}

/// A list as a chain of its sort's own `Nil` and `Cons` constructors: the
/// module authors no list of a recursive sort (`#208`).
pub(super) fn chain(nil: u64, cons: u64, items: Vec<Value>) -> Value {
    items
        .into_iter()
        .rev()
        .fold(adt(nil, Vec::new()), |tail, head| {
            adt(cons, vec![head, tail])
        })
}

/// `Id`: local, holder, operand, callee, boxed, part, capture, param,
/// function, export.
pub(super) fn enc_ident(ident: &Ident) -> Value {
    match ident {
        Ident::Local(n) => adt(0, vec![nat(*n)]),
        Ident::Holder(n) => adt(1, vec![nat(*n)]),
        Ident::Operand(n) => adt(2, vec![nat(*n)]),
        Ident::Callee(n) => adt(3, vec![nat(*n)]),
        Ident::Boxed(n) => adt(4, vec![nat(*n)]),
        Ident::Part(n) => adt(5, vec![nat(*n)]),
        Ident::Capture(n) => adt(6, vec![nat(*n)]),
        Ident::Param(n) => adt(7, vec![nat(*n)]),
        Ident::Function(n) => adt(8, vec![nat(*n)]),
        Ident::Export(name) => adt(9, vec![text(name)]),
    }
}

/// `Origin`, a structure: the element and the width.
pub(super) fn enc_origin(origin: &Origin) -> Value {
    adt(
        0,
        vec![text(&origin.element), option(origin.width.map(enc_kind))],
    )
}

pub(super) fn enc_types(types: &[Type]) -> Value {
    chain(5, 6, types.iter().map(enc_type).collect())
}

/// `Ty`: a leaf (unit, bool, nat, int, ordering, str, bytes, overflow, i128), fixed, a unary
/// (option, list, rc, fallible, ref), a binary (result, pair), a name (adt,
/// fn), then the chain's nil and cons.
pub(super) fn enc_type(ty: &Type) -> Value {
    let leaf = |kind: u64| adt(0, vec![adt(kind, Vec::new())]);
    let unary = |kind: u64, inner: &Type| adt(2, vec![adt(kind, Vec::new()), enc_type(inner)]);
    let binary = |kind: u64, left: &Type, right: &Type| {
        adt(
            3,
            vec![adt(kind, Vec::new()), enc_type(left), enc_type(right)],
        )
    };
    let named = |kind: u64, n: u64| adt(4, vec![adt(kind, Vec::new()), nat(n)]);
    match ty {
        Type::Unit => leaf(0),
        Type::Bool => leaf(1),
        Type::Nat => leaf(2),
        Type::Int => leaf(3),
        Type::Ordering => leaf(4),
        Type::Str => leaf(5),
        Type::Bytes => leaf(6),
        Type::Overflow => leaf(7),
        Type::I128 => leaf(8),
        Type::Fixed(kind) => adt(1, vec![enc_kind(*kind)]),
        Type::Option(inner) => unary(0, inner),
        Type::List(element) => unary(1, element),
        Type::Rc(inner) => unary(2, inner),
        Type::Fallible(inner) => unary(3, inner),
        Type::Ref(inner) => unary(4, inner),
        Type::Result(ok, error) => binary(0, ok, error),
        Type::Pair(left, right) => binary(1, left, right),
        Type::Adt(n) => named(0, *n),
        Type::Fn(n) => named(1, *n),
    }
}

/// `Lit`: unit, bool, nat, int, fixed, str, bytes, ordering, overflow. The checker
/// reads a literal's kind and, for a fixed one, its width; no payload.
fn enc_lit(literal: &Lit) -> Value {
    match literal {
        Lit::Unit => adt(0, Vec::new()),
        Lit::Bool(_) => adt(1, Vec::new()),
        Lit::Nat(_) => adt(2, Vec::new()),
        Lit::Int(_) => adt(3, Vec::new()),
        Lit::Fixed(kind, _) => adt(4, vec![enc_kind(*kind)]),
        Lit::Str(_) => adt(5, Vec::new()),
        Lit::Bytes(_) => adt(6, Vec::new()),
        Lit::Ordering(_) => adt(7, Vec::new()),
        Lit::Overflow => adt(8, Vec::new()),
    }
}

/// `Con`: none, some, ok, err, adt, closure, cons, nil.
pub(super) fn enc_ctor(ctor: &Ctor) -> Value {
    match ctor {
        Ctor::None(ty) => adt(0, vec![enc_type(ty)]),
        Ctor::Some => adt(1, Vec::new()),
        Ctor::Ok(ok, error) => adt(2, vec![enc_type(ok), enc_type(error)]),
        Ctor::Err(ok, error) => adt(3, vec![enc_type(ok), enc_type(error)]),
        Ctor::Adt {
            adt: n,
            constructor,
        } => adt(4, vec![nat(*n), nat(*constructor)]),
        Ctor::Closure { fn_type, function } => adt(5, vec![nat(*fn_type), nat(*function)]),
        Ctor::Cons => adt(6, Vec::new()),
        Ctor::Nil(ty) => adt(7, vec![enc_type(ty)]),
    }
}

fn enc_pats(pats: &[Pat]) -> Value {
    chain(5, 6, pats.iter().map(enc_pat).collect())
}

/// `Pat`: a leaf (wild, unit, none, ordering), bind, tuple, a wrapper (some,
/// ok, err), adt, then the chain's nil and cons.
fn enc_pat(pat: &Pat) -> Value {
    let leaf = |kind: u64| adt(0, vec![adt(kind, Vec::new())]);
    let wrap = |kind: u64, inner: &Pat| adt(3, vec![adt(kind, Vec::new()), enc_pat(inner)]);
    match pat {
        Pat::Wild => leaf(0),
        Pat::Unit => leaf(1),
        Pat::None => leaf(2),
        Pat::Ordering(_) => leaf(3),
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

/// `Rt`, a structure: the variant of `runtime::Item` in its declaration
/// order, and the width of a fixed-width one.
pub(super) fn enc_item(item: Item) -> Value {
    let family =
        |index: u64, kind: Option<IntKind>| adt(0, vec![nat(index), option(kind.map(enc_kind))]);
    match item {
        Item::NatAdd => family(0, None),
        Item::NatSub => family(1, None),
        Item::NatMul => family(2, None),
        Item::NatQuot => family(3, None),
        Item::NatRem => family(4, None),
        Item::NatEq => family(5, None),
        Item::NatLe => family(6, None),
        Item::NatLt => family(7, None),
        Item::NatSucc => family(8, None),
        Item::IntAdd => family(9, None),
        Item::IntSub => family(10, None),
        Item::IntMul => family(11, None),
        Item::IntNeg => family(12, None),
        Item::IntQuot => family(13, None),
        Item::IntRem => family(14, None),
        Item::BoolNot => family(15, None),
        Item::BoolAnd => family(16, None),
        Item::BoolOr => family(17, None),
        Item::Equal => family(18, None),
        Item::Compare => family(19, None),
        Item::CheckedAdd(kind) => family(20, Some(kind)),
        Item::CheckedSub(kind) => family(21, Some(kind)),
        Item::CheckedMul(kind) => family(22, Some(kind)),
        Item::CheckedQuot(kind) => family(23, Some(kind)),
        Item::CheckedNeg(kind) => family(24, Some(kind)),
        Item::BitAnd(kind) => family(25, Some(kind)),
        Item::BitOr(kind) => family(26, Some(kind)),
        Item::BitXor(kind) => family(27, Some(kind)),
        Item::BitNot(kind) => family(28, Some(kind)),
        Item::ShiftLeft(kind) => family(29, Some(kind)),
        Item::ShiftRight(kind) => family(30, Some(kind)),
        Item::Convert(kind) => family(31, Some(kind)),
        Item::AppendList => family(32, None),
        Item::AppendBytes => family(33, None),
        Item::LengthList => family(34, None),
        Item::LengthBytes => family(35, None),
        Item::LengthString => family(36, None),
        Item::IndexList => family(37, None),
        Item::IndexBytes => family(38, None),
        Item::SliceList => family(39, None),
        Item::SliceBytes => family(40, None),
        Item::Utf8Encode => family(41, None),
        Item::Utf8Decode => family(42, None),
        Item::CompareBytes => family(43, None),
        Item::SplitExact => family(44, None),
        Item::Join => family(45, None),
        Item::FormatInt => family(46, None),
        Item::FormatFixed(kind) => family(47, Some(kind)),
        Item::ParseInt => family(48, None),
        Item::ParseFixed(kind) => family(49, Some(kind)),
    }
}

/// `Callee`: function, runtime, machine (by family).
fn enc_callee(callee: &Callee) -> Value {
    match callee {
        Callee::Function(n) => adt(0, vec![nat(*n)]),
        Callee::Runtime(item) => adt(1, vec![enc_item(*item)]),
        Callee::Machine(machine) => adt(2, vec![nat(machine.family())]),
    }
}

/// `Cap`: copy, clone, unbox, unit.
pub(super) fn enc_capture(read: CaptureRead) -> Value {
    adt(
        match read {
            CaptureRead::Copy => 0,
            CaptureRead::Clone => 1,
            CaptureRead::Unbox => 2,
            CaptureRead::Unit => 3,
        },
        Vec::new(),
    )
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
                .map_or_else(|| enc_type(&Type::Unit), enc_type),
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

/// `Expr`: lit, read (move, clone, copy, deref, unbox), wrap (not, box,
/// widen, succeed), call, apply, construct, pair, if, match, block, probe
/// (is-zero, non-zero, uncons, predecessor), then the block, the chain's nil
/// and cons, the let, the arm, and the loop over a state.
fn enc_expr(expr: &Expr) -> Value {
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
        // Neither carries an origin of its own; the module ignores the one given.
        Expr::Uncons(ident) => probe(2, ident, &Origin::of("")),
        Expr::Predecessor(ident) => probe(3, ident, &Origin::of("")),
        Expr::Loop { state, body } => adt(16, vec![enc_ident(state), enc_block(body)]),
    }
}

/// `Row`: nil, cons (the variant number, its field types as a `Ty` chain).
pub(super) fn enc_row(variants: &[(u64, Vec<Type>)]) -> Value {
    variants
        .iter()
        .rev()
        .fold(adt(0, Vec::new()), |rest, (number, fields)| {
            adt(1, vec![nat(*number), enc_types(fields), rest])
        })
}

/// `Dis`: nil, cons (function, captures, the function's failure, the rest).
pub(super) fn enc_dispatch(arms: &[Dispatch]) -> Value {
    arms.iter().rev().fold(adt(0, Vec::new()), |rest, arm| {
        adt(
            1,
            vec![
                nat(arm.function),
                Value::List {
                    items: arm.captures.iter().map(|read| enc_capture(*read)).collect(),
                },
                boolean(arm.function_fallible),
                rest,
            ],
        )
    })
}

/// `Par`: nil, cons (pattern, type, the rest).
fn enc_params(parameters: &[(Pat, Type)]) -> Value {
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
fn enc_crate(krate: &Crate) -> Value {
    let items = krate
        .items
        .iter()
        .rev()
        .fold(adt(3, Vec::new()), |tail, item| {
            adt(4, vec![enc_item_def(item), tail])
        });
    adt(
        0,
        vec![
            boolean(krate.profile == lexlean::calculus::rust::Profile::Core),
            items,
        ],
    )
}

// ------------------------------------------------------------------ oracle

use lexlean::calculus::rust::{self, validate, Profile};
use lexlean::calculus::{Fixture, Program};
use lexlean::production::lower::Lowered;
use serde_json::Value as Json;

/// What the oracle makes of a crate, as the root returns it: `ok ()` or the
/// reason's text.
pub(super) fn verdict(outcome: Result<(), String>) -> Json {
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

/// A run of `validateCrate`.
fn validate_run(krate: &Crate) -> Run {
    (vec![enc_crate(krate)], verdict(validate::validate(krate)))
}

/// A run of `correspondCrate`: the crate and the elements the program uses.
fn correspond_run(krate: &Crate, elements: &BTreeSet<String>) -> Run {
    (
        vec![
            enc_crate(krate),
            list(elements.iter().map(|element| text(element)).collect()),
        ],
        verdict(validate::correspond(krate, elements)),
    )
}

// ----------------------------------------------------------------- vectors

/// A committed target program and where it was read from.
pub(super) struct Source {
    pub(super) program: Program,
}

pub(super) fn walk(dir: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
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

/// Every committed target program the Rust renderer is shown: the program of
/// each package manifest under `compiler/rust` and `tests/production-export`
/// and each fixture of `compiler/fixtures`, deduplicated by canonical bytes.
pub(super) fn committed() -> (Vec<Source>, usize, usize, usize) {
    let root = support::repo_root();
    let root = root.as_std_path();
    let mut files = Vec::new();
    walk(root, &mut files);
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    let (mut manifests, mut fixtures) = (0, 0);
    let mut admit = |program: Program| {
        let key = serde_json::to_string(&program).expect("a program is JSON");
        if seen.insert(key) {
            out.push(Source { program });
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
            fixtures += 1;
            admit(fixture.program);
        } else if name == "package.json"
            && (relative.starts_with("compiler/rust/")
                || relative.starts_with("tests/production-export/"))
        {
            let json: Json =
                serde_json::from_slice(&std::fs::read(&path).expect("a package manifest"))
                    .expect("a manifest is JSON");
            let program: Program = serde_json::from_value(json["program"].clone())
                .unwrap_or_else(|error| panic!("{relative}: {error}"));
            manifests += 1;
            admit(program);
        }
    }
    let distinct = out.len();
    (out, manifests, fixtures, distinct)
}

/// A crate the oracle lowers from a committed program, with the elements the
/// program uses, and the program it came from.
pub(super) struct Rendered {
    pub(super) krate: Crate,
    pub(super) elements: BTreeSet<String>,
}

pub(super) fn rendered(sources: &[Source]) -> Vec<Rendered> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for source in sources {
        for profile in [Profile::Core, Profile::Std] {
            let Ok(krate) = rust::lower(&source.program, profile) else {
                continue;
            };
            let elements =
                rust::realized(&source.program, profile).expect("a lowered program is realized");
            if seen.insert(format!("{krate:?}")) {
                out.push(Rendered { krate, elements });
            }
        }
    }
    out
}

const MODULE: &str = "LexLeanBoot.RustValidate";
const ROOTS: [(&str, &[&str]); 2] = [
    ("validateCrate", &["allocation", "overflow", "recursion"]),
    ("correspondCrate", &["allocation", "overflow", "recursion"]),
];

// ----------------------------------------------------------------- mutants

/// A mutable reference to one node of the closed AST, of one class.
enum Node<'a> {
    Item,
    Block(&'a mut Block),
    Let(&'a mut Let),
    Expr(&'a mut Expr),
    Pat(&'a mut Pat),
    Ty(&'a mut Type),
    Ident(&'a mut Ident),
    Origin(&'a mut Origin),
    Ctor(&'a mut Ctor),
    Callee(&'a mut Callee),
    Dispatch(&'a mut Dispatch),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Class {
    Item,
    Block,
    Let,
    Expr,
    Pat,
    Ty,
    Ident,
    Origin,
    Ctor,
    Callee,
    Dispatch,
}

fn class_of(node: &Node<'_>) -> Class {
    match node {
        Node::Item => Class::Item,
        Node::Block(_) => Class::Block,
        Node::Let(_) => Class::Let,
        Node::Expr(_) => Class::Expr,
        Node::Pat(_) => Class::Pat,
        Node::Ty(_) => Class::Ty,
        Node::Ident(_) => Class::Ident,
        Node::Origin(_) => Class::Origin,
        Node::Ctor(_) => Class::Ctor,
        Node::Callee(_) => Class::Callee,
        Node::Dispatch(_) => Class::Dispatch,
    }
}

type Visitor<'v> = dyn FnMut(Node<'_>) + 'v;

fn visit_ty(ty: &mut Type, f: &mut Visitor<'_>) {
    f(Node::Ty(ty));
    match ty {
        Type::Option(inner)
        | Type::List(inner)
        | Type::Rc(inner)
        | Type::Fallible(inner)
        | Type::Ref(inner) => visit_ty(inner, f),
        Type::Result(left, right) | Type::Pair(left, right) => {
            visit_ty(left, f);
            visit_ty(right, f);
        }
        _ => {}
    }
}

fn visit_pat(pat: &mut Pat, f: &mut Visitor<'_>) {
    f(Node::Pat(pat));
    match pat {
        Pat::Bind(ident) => f(Node::Ident(ident)),
        Pat::Tuple(items) => items.iter_mut().for_each(|item| visit_pat(item, f)),
        Pat::Some(inner) | Pat::Ok(inner) | Pat::Err(inner) => visit_pat(inner, f),
        Pat::Adt { fields, .. } => fields.iter_mut().for_each(|field| visit_pat(field, f)),
        Pat::Wild | Pat::Unit | Pat::None | Pat::Ordering(_) => {}
    }
}

fn visit_block(block: &mut Block, f: &mut Visitor<'_>) {
    f(Node::Block(block));
    for binding in &mut block.lets {
        f(Node::Let(binding));
        visit_pat(&mut binding.pat, f);
        if let Some(ty) = &mut binding.ty {
            visit_ty(ty, f);
        }
        visit_expr(&mut binding.value, f);
        f(Node::Origin(&mut binding.at));
    }
    visit_expr(&mut block.tail, f);
}

fn visit_expr(expr: &mut Expr, f: &mut Visitor<'_>) {
    f(Node::Expr(expr));
    match expr {
        Expr::Lit(_, at) => f(Node::Origin(at)),
        Expr::Move(ident, at)
        | Expr::Clone(ident, at)
        | Expr::Copy(ident, at)
        | Expr::Deref(ident, at)
        | Expr::Unbox(ident, at)
        | Expr::IsZero(ident, at)
        | Expr::NonZero(ident, at) => {
            f(Node::Ident(ident));
            f(Node::Origin(at));
        }
        Expr::Uncons(ident) | Expr::Predecessor(ident) => f(Node::Ident(ident)),
        Expr::Not(inner, at)
        | Expr::Box(inner, at)
        | Expr::Widen(inner, at)
        | Expr::Succeed(inner, at) => {
            visit_expr(inner, f);
            f(Node::Origin(at));
        }
        Expr::Call {
            callee, args, at, ..
        } => {
            f(Node::Callee(callee));
            args.iter_mut().for_each(|arg| visit_expr(arg, f));
            f(Node::Origin(at));
        }
        Expr::Apply {
            holder, args, at, ..
        } => {
            f(Node::Ident(holder));
            args.iter_mut().for_each(|arg| visit_expr(arg, f));
            f(Node::Origin(at));
        }
        Expr::Construct { ctor, args, at } => {
            f(Node::Ctor(ctor));
            match ctor {
                Ctor::None(ty) | Ctor::Nil(ty) => visit_ty(ty, f),
                Ctor::Ok(ok, error) | Ctor::Err(ok, error) => {
                    visit_ty(ok, f);
                    visit_ty(error, f);
                }
                _ => {}
            }
            args.iter_mut().for_each(|arg| visit_expr(arg, f));
            f(Node::Origin(at));
        }
        Expr::Pair(left, right, at) => {
            visit_expr(left, f);
            visit_expr(right, f);
            f(Node::Origin(at));
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
            at,
        } => {
            visit_expr(condition, f);
            visit_block(then_branch, f);
            visit_block(else_branch, f);
            f(Node::Origin(at));
        }
        Expr::Match {
            scrutinee,
            arms,
            at,
        } => {
            visit_expr(scrutinee, f);
            for (pat, body) in arms {
                visit_pat(pat, f);
                visit_block(body, f);
            }
            f(Node::Origin(at));
        }
        Expr::Block(block) => visit_block(block, f),
        Expr::Loop { state, body } => {
            f(Node::Ident(state));
            visit_block(body, f);
        }
    }
}

fn visit_crate(krate: &mut Crate, f: &mut Visitor<'_>) {
    for item in &mut krate.items {
        f(Node::Item);
        match item {
            ItemDef::Enum { name, variants, at } => {
                visit_ty(name, f);
                for (_, fields) in variants {
                    fields.iter_mut().for_each(|ty| visit_ty(ty, f));
                }
                f(Node::Origin(at));
            }
            ItemDef::Apply {
                parameters,
                result,
                arms,
                at,
                ..
            } => {
                parameters.iter_mut().for_each(|ty| visit_ty(ty, f));
                visit_ty(result, f);
                arms.iter_mut().for_each(|arm| f(Node::Dispatch(arm)));
                f(Node::Origin(at));
            }
            ItemDef::Function {
                name,
                parameters,
                result,
                body,
                at,
            } => {
                f(Node::Ident(name));
                for (pat, ty) in parameters {
                    visit_pat(pat, f);
                    visit_ty(ty, f);
                }
                visit_ty(result, f);
                visit_block(body, f);
                f(Node::Origin(at));
            }
        }
    }
}

fn sites(krate: &mut Crate, class: Class) -> usize {
    let mut count = 0;
    visit_crate(krate, &mut |node| {
        if class_of(&node) == class {
            count += 1;
        }
    });
    count
}

/// Edit the `index`-th node of `class`, in visiting order.
fn edit_at(krate: &mut Crate, class: Class, index: usize, edit: &mut dyn FnMut(Node<'_>)) {
    let mut count = 0;
    visit_crate(krate, &mut |node| {
        if class_of(&node) == class {
            if count == index {
                edit(node);
            }
            count += 1;
        }
    });
}

/// What a mutation draws on: the identifiers, expressions, and numbers of the
/// crate it mutates, and the elements of the correspondence table.
pub(super) struct Pool {
    idents: Vec<Ident>,
    exprs: Vec<Expr>,
    types: Vec<Type>,
    functions: Vec<u64>,
    elements: Vec<String>,
}

pub(super) fn pool_of(krate: &Crate) -> Pool {
    let mut copy = krate.clone();
    let mut idents = Vec::new();
    let mut exprs = Vec::new();
    let mut types = vec![
        Type::Unit,
        Type::Bool,
        Type::Nat,
        Type::Int,
        Type::Str,
        Type::Bytes,
        Type::Ordering,
        Type::Fixed(IntKind::U8),
        Type::Fixed(IntKind::I64),
        Type::Adt(0),
        Type::Adt(97),
        Type::Fn(0),
        Type::Fn(97),
        Type::List(Box::new(Type::Nat)),
        Type::Option(Box::new(Type::Str)),
        Type::Rc(Box::new(Type::Nat)),
        Type::Fallible(Box::new(Type::Nat)),
        Type::Fallible(Box::new(Type::Unit)),
        Type::Ref(Box::new(Type::Nat)),
        Type::Pair(Box::new(Type::Nat), Box::new(Type::Bytes)),
        Type::Result(Box::new(Type::Nat), Box::new(Type::Bool)),
    ];
    let mut functions = vec![0, 1, 97];
    visit_crate(&mut copy, &mut |node| match node {
        Node::Ident(ident) => {
            if let Ident::Function(n) = ident {
                functions.push(*n);
            }
            idents.push(ident.clone());
        }
        Node::Expr(expr) => {
            if exprs.len() < 400 {
                exprs.push(expr.clone());
            }
        }
        Node::Ty(ty) if types.len() < 80 => types.push(ty.clone()),
        _ => {}
    });
    idents.push(Ident::Local(99));
    idents.push(Ident::Export("x".to_owned()));
    idents.push(Ident::Export("v1".to_owned()));
    let mut elements: Vec<String> = validate::CORRESPONDENCE
        .iter()
        .flat_map(|(_, realized)| realized.iter().map(|element| (*element).to_owned()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    elements.push("prim:nope".to_owned());
    Pool {
        idents,
        exprs,
        types,
        functions,
        elements,
    }
}

fn read_origin(expr: &Expr) -> Origin {
    expr.origin().cloned().unwrap_or_else(|| Origin::of("read"))
}

/// One mutation of a randomly chosen class; false when the class has no site.
#[allow(clippy::too_many_lines)]
fn mutate_once(krate: &mut Crate, seeded: &mut Seeded, pool: &Pool) -> bool {
    let kinds = IntKind::ALL;
    match seeded.below(20) {
        // An identifier read or bound is another one of the crate.
        0 => {
            let count = sites(krate, Class::Ident);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let other = seeded.pick(&pool.idents).clone();
            edit_at(krate, Class::Ident, at, &mut |node| {
                if let Node::Ident(ident) = node {
                    *ident = other.clone();
                }
            });
            true
        }
        // A name is shifted: its number, or its kind.
        1 => {
            let count = sites(krate, Class::Ident);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let step = seeded.below(3) as u64;
            edit_at(krate, Class::Ident, at, &mut |node| {
                if let Node::Ident(ident) = node {
                    *ident = match ident.clone() {
                        Ident::Local(n) => Ident::Part(n + step),
                        Ident::Holder(n) => Ident::Local(n + step),
                        Ident::Operand(n) => Ident::Operand(n + step),
                        Ident::Callee(n) => Ident::Callee(n + step),
                        Ident::Boxed(n) => Ident::Boxed(n + step),
                        Ident::Part(n) => Ident::Local(n + step),
                        Ident::Capture(n) => Ident::Capture(n + step),
                        Ident::Param(n) => Ident::Param(n + step),
                        Ident::Function(n) => Ident::Function(n + step),
                        Ident::Export(name) => Ident::Export(format!("{name}1")),
                    };
                }
            });
            true
        }
        // A read is another kind of read.
        2 => {
            let count = sites(krate, Class::Expr);
            let at = seeded.below(count.max(1));
            let pick = seeded.below(5);
            let mut changed = false;
            edit_at(krate, Class::Expr, at, &mut |node| {
                if let Node::Expr(expr) = node {
                    let (ident, origin) = match expr {
                        Expr::Move(ident, at)
                        | Expr::Clone(ident, at)
                        | Expr::Copy(ident, at)
                        | Expr::Deref(ident, at)
                        | Expr::Unbox(ident, at) => (ident.clone(), at.clone()),
                        _ => return,
                    };
                    *expr = match pick {
                        0 => Expr::Move(ident, origin),
                        1 => Expr::Clone(ident, origin),
                        2 => Expr::Copy(ident, origin),
                        3 => Expr::Deref(ident, origin),
                        _ => Expr::Unbox(ident, origin),
                    };
                    changed = true;
                }
            });
            changed
        }
        // A call or an application propagates, or does not.
        3 => {
            let count = sites(krate, Class::Expr);
            let at = seeded.below(count.max(1));
            let mut changed = false;
            edit_at(krate, Class::Expr, at, &mut |node| {
                if let Node::Expr(Expr::Call { propagate, .. } | Expr::Apply { propagate, .. }) =
                    node
                {
                    *propagate = !*propagate;
                    changed = true;
                }
            });
            changed
        }
        // A callee is another function or another runtime function.
        4 => {
            let count = sites(krate, Class::Callee);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let all = Item::all();
            let function = *seeded.pick(&pool.functions);
            let item = *seeded.pick(&all);
            let to_runtime = seeded.chance(50);
            edit_at(krate, Class::Callee, at, &mut |node| {
                if let Node::Callee(callee) = node {
                    *callee = if to_runtime {
                        Callee::Runtime(item)
                    } else {
                        Callee::Function(function)
                    };
                }
            });
            true
        }
        // A constructor names another variant.
        5 => {
            let count = sites(krate, Class::Ctor);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let number = seeded.below(4) as u64;
            let which = seeded.below(6);
            let ty = seeded.pick(&pool.types).clone();
            edit_at(krate, Class::Ctor, at, &mut |node| {
                if let Node::Ctor(ctor) = node {
                    *ctor = match (which, ctor.clone()) {
                        (0, Ctor::Adt { adt, constructor }) => Ctor::Adt {
                            adt: adt + number,
                            constructor,
                        },
                        (1, Ctor::Adt { adt, constructor }) => Ctor::Adt {
                            adt,
                            constructor: constructor + number + 1,
                        },
                        (0, Ctor::Closure { fn_type, function }) => Ctor::Closure {
                            fn_type: fn_type + number,
                            function,
                        },
                        (1, Ctor::Closure { fn_type, function }) => Ctor::Closure {
                            fn_type,
                            function: function + number + 1,
                        },
                        (2, Ctor::Some) => Ctor::Cons,
                        (2, Ctor::Cons) => Ctor::Some,
                        (3, Ctor::None(_)) => Ctor::Nil(ty.clone()),
                        (3, Ctor::Nil(_)) => Ctor::None(ty.clone()),
                        (4, Ctor::Ok(ok, error)) => Ctor::Err(ok, error),
                        (4, Ctor::Err(ok, error)) => Ctor::Ok(ok, error),
                        (5, Ctor::None(_)) => Ctor::None(ty.clone()),
                        (5, Ctor::Nil(_)) => Ctor::Nil(ty.clone()),
                        (_, other) => other,
                    };
                }
            });
            true
        }
        // A type is another type.
        6 => {
            let count = sites(krate, Class::Ty);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let other = seeded.pick(&pool.types).clone();
            edit_at(krate, Class::Ty, at, &mut |node| {
                if let Node::Ty(ty) = node {
                    *ty = other.clone();
                }
            });
            true
        }
        // A pattern changes.
        7 => {
            let count = sites(krate, Class::Pat);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let ident = seeded.pick(&pool.idents).clone();
            let which = seeded.below(6);
            let number = seeded.below(3) as u64;
            edit_at(krate, Class::Pat, at, &mut |node| {
                if let Node::Pat(pat) = node {
                    *pat = match (which, pat.clone()) {
                        (0, Pat::Bind(_) | Pat::Wild) => Pat::Bind(ident.clone()),
                        (1, Pat::Bind(_)) => Pat::Wild,
                        (2, Pat::Tuple(mut items)) => {
                            if items.len() > 1 && number == 0 {
                                items.pop();
                            } else {
                                items.push(Pat::Wild);
                            }
                            Pat::Tuple(items)
                        }
                        (
                            3,
                            Pat::Adt {
                                adt,
                                constructor,
                                mut fields,
                            },
                        ) => {
                            if fields.is_empty() || number == 0 {
                                fields.push(Pat::Wild);
                            } else {
                                fields.pop();
                            }
                            Pat::Adt {
                                adt,
                                constructor,
                                fields,
                            }
                        }
                        (
                            4,
                            Pat::Adt {
                                adt,
                                constructor,
                                fields,
                            },
                        ) => Pat::Adt {
                            adt: adt + number,
                            constructor: constructor + number,
                            fields,
                        },
                        (5, Pat::Some(inner)) => Pat::Ok(inner),
                        (5, Pat::Ok(inner)) => Pat::Err(inner),
                        (5, Pat::None) => Pat::Unit,
                        (_, other) => other,
                    };
                }
            });
            true
        }
        // An origin names another element, or another width.
        8 => {
            let count = sites(krate, Class::Origin);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let element = seeded.pick(&pool.elements).clone();
            let width = seeded.chance(60).then(|| *seeded.pick(&kinds));
            let both = seeded.chance(30);
            let width_only = seeded.chance(50);
            edit_at(krate, Class::Origin, at, &mut |node| {
                if let Node::Origin(origin) = node {
                    if width_only {
                        origin.width = width;
                    } else {
                        origin.element.clone_from(&element);
                        if both {
                            origin.width = width;
                        }
                    }
                }
            });
            true
        }
        // An item is dropped, doubled, swapped, or restated.
        9 => {
            if krate.items.is_empty() {
                return false;
            }
            let at = seeded.below(krate.items.len());
            let other = seeded.below(krate.items.len());
            let number = seeded.below(3) as u64;
            match seeded.below(6) {
                0 => {
                    krate.items.remove(at);
                }
                1 => {
                    let copy = krate.items[at].clone();
                    krate.items.insert(at, copy);
                }
                2 => krate.items.swap(at, other),
                3 => match &mut krate.items[at] {
                    ItemDef::Enum { name, .. } => {
                        *name = match name.clone() {
                            Type::Adt(n) => Type::Fn(n),
                            Type::Fn(n) => Type::Adt(n + number),
                            _ => Type::Adt(number),
                        };
                    }
                    ItemDef::Apply { fn_type, .. } => *fn_type += number + 1,
                    ItemDef::Function { name, .. } => *name = Ident::Function(number),
                },
                4 => {
                    if let ItemDef::Apply { fallible, .. } = &mut krate.items[at] {
                        *fallible = !*fallible;
                    } else if let ItemDef::Function { result, .. } = &mut krate.items[at] {
                        *result = match result.clone() {
                            Type::Fallible(inner) => *inner,
                            other => Type::Fallible(Box::new(other)),
                        };
                    } else if let ItemDef::Enum { variants, .. } = &mut krate.items[at] {
                        if let Some((variant, _)) = variants.first_mut() {
                            *variant += number;
                        }
                    }
                }
                _ => {
                    if let ItemDef::Enum { variants, .. } = &mut krate.items[at] {
                        if let Some((_, fields)) = variants.first_mut() {
                            if fields.is_empty() || number == 0 {
                                fields.push(Type::Nat);
                            } else {
                                fields.pop();
                            }
                        }
                    }
                }
            }
            true
        }
        // A dispatch arm misstates its function, or captures a box.
        10 => {
            let count = sites(krate, Class::Dispatch);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let function = *seeded.pick(&pool.functions);
            let which = seeded.below(4);
            edit_at(krate, Class::Dispatch, at, &mut |node| {
                if let Node::Dispatch(arm) = node {
                    match which {
                        0 => arm.function_fallible = !arm.function_fallible,
                        1 => arm.function = function,
                        2 => arm.captures.push(CaptureRead::Unbox),
                        _ => {
                            arm.captures.pop();
                        }
                    }
                }
            });
            true
        }
        // The profile is the other one.
        11 => {
            krate.profile = if krate.profile == Profile::Core {
                Profile::Std
            } else {
                Profile::Core
            };
            true
        }
        // An expression is another expression of the crate.
        12 => {
            let count = sites(krate, Class::Expr);
            if count == 0 || pool.exprs.is_empty() {
                return false;
            }
            let at = seeded.below(count);
            let other = seeded.pick(&pool.exprs).clone();
            edit_at(krate, Class::Expr, at, &mut |node| {
                if let Node::Expr(expr) = node {
                    *expr = other.clone();
                }
            });
            true
        }
        // An expression is wrapped, or unwrapped.
        13 => {
            let count = sites(krate, Class::Expr);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let which = seeded.below(6);
            edit_at(krate, Class::Expr, at, &mut |node| {
                if let Node::Expr(expr) = node {
                    let origin = read_origin(expr);
                    *expr = match (which, expr.clone()) {
                        (0, inner) => Expr::Succeed(Box::new(inner), Origin::of("overflow")),
                        (1, inner) => Expr::Not(Box::new(inner), origin),
                        (2, inner) => Expr::Box(Box::new(inner), Origin::of("indirection")),
                        (3, inner) => Expr::Widen(Box::new(inner), origin),
                        (
                            _,
                            Expr::Succeed(inner, _)
                            | Expr::Box(inner, _)
                            | Expr::Not(inner, _)
                            | Expr::Widen(inner, _),
                        ) => *inner,
                        (_, other) => other,
                    };
                }
            });
            true
        }
        // A binding is dropped, doubled, or swapped.
        14 => {
            let count = sites(krate, Class::Block);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let which = seeded.below(3);
            let pick = seeded.below(8);
            let other = seeded.below(8);
            let mut changed = false;
            edit_at(krate, Class::Block, at, &mut |node| {
                if let Node::Block(block) = node {
                    if block.lets.is_empty() {
                        return;
                    }
                    let index = pick % block.lets.len();
                    let second = other % block.lets.len();
                    match which {
                        0 => {
                            block.lets.remove(index);
                        }
                        1 => {
                            let copy = block.lets[index].clone();
                            block.lets.insert(index, copy);
                        }
                        _ => block.lets.swap(index, second),
                    }
                    changed = true;
                }
            });
            changed
        }
        // A match loses its arms, one arm, or doubles one.
        15 => {
            let count = sites(krate, Class::Expr);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let which = seeded.below(3);
            let pick = seeded.below(6);
            let mut changed = false;
            edit_at(krate, Class::Expr, at, &mut |node| {
                if let Node::Expr(Expr::Match { arms, .. }) = node {
                    if arms.is_empty() {
                        return;
                    }
                    let index = pick % arms.len();
                    match which {
                        0 => arms.clear(),
                        1 => {
                            arms.remove(index);
                        }
                        _ => {
                            let copy = arms[index].clone();
                            arms.insert(index, copy);
                        }
                    }
                    changed = true;
                }
            });
            changed
        }
        // A literal is of another kind.
        16 => {
            let count = sites(krate, Class::Expr);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let which = seeded.below(6);
            let kind = *seeded.pick(&kinds);
            let mut changed = false;
            edit_at(krate, Class::Expr, at, &mut |node| {
                if let Node::Expr(Expr::Lit(literal, _)) = node {
                    *literal = match which {
                        0 => Lit::Str(String::new()),
                        1 => Lit::Bytes(Vec::new()),
                        2 => Lit::Fixed(kind, 0),
                        3 => Lit::Nat(0),
                        4 => Lit::Unit,
                        _ => Lit::Bool(true),
                    };
                    changed = true;
                }
            });
            changed
        }
        // A block is replaced by a tail expression of the crate.
        17 => {
            let count = sites(krate, Class::Block);
            if count == 0 || pool.exprs.is_empty() {
                return false;
            }
            let at = seeded.below(count);
            let other = seeded.pick(&pool.exprs).clone();
            edit_at(krate, Class::Block, at, &mut |node| {
                if let Node::Block(block) = node {
                    block.tail = other.clone();
                }
            });
            true
        }
        // A block runs in a loop over a name of the crate: as it is, with its
        // value an `Err`, or after it takes a name or a component of one.
        19 => {
            let count = sites(krate, Class::Block);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let state = seeded.pick(&pool.idents).clone();
            let taken = seeded.pick(&pool.idents).clone();
            let which = seeded.below(4);
            let fresh = Ident::Local(300 + seeded.below(50) as u64);
            edit_at(krate, Class::Block, at, &mut |node| {
                if let Node::Block(block) = node {
                    let mut body = Block {
                        lets: std::mem::take(&mut block.lets),
                        tail: block.tail.clone(),
                    };
                    match which {
                        1 => {
                            body.tail = Expr::Construct {
                                ctor: Ctor::Err(Type::Nat, Type::Nat),
                                args: vec![body.tail.clone()],
                                at: Origin::of("construct:err"),
                            };
                        }
                        2 => body.lets.insert(
                            0,
                            Let {
                                pat: Pat::Wild,
                                ty: None,
                                value: Expr::Move(taken.clone(), Origin::of("read")),
                                at: Origin::of("let"),
                            },
                        ),
                        3 => body.lets.insert(
                            0,
                            Let {
                                pat: Pat::Tuple(vec![Pat::Bind(fresh.clone()), Pat::Wild]),
                                ty: None,
                                value: Expr::Move(taken.clone(), Origin::of("read")),
                                at: Origin::of("destructure:pair"),
                            },
                        ),
                        _ => {}
                    }
                    block.tail = Expr::Loop {
                        state: state.clone(),
                        body: Box::new(body),
                    };
                }
            });
            true
        }
        // A binding's pattern becomes a tuple of the identifier.
        _ => {
            let count = sites(krate, Class::Let);
            if count == 0 {
                return false;
            }
            let at = seeded.below(count);
            let ident = seeded.pick(&pool.idents).clone();
            let wild_first = seeded.chance(50);
            edit_at(krate, Class::Let, at, &mut |node| {
                if let Node::Let(binding) = node {
                    binding.pat = if wild_first {
                        Pat::Tuple(vec![Pat::Bind(ident.clone()), Pat::Wild])
                    } else {
                        Pat::Tuple(vec![Pat::Wild, Pat::Bind(ident.clone())])
                    };
                }
            });
            true
        }
    }
}

/// A mutant of `krate`: one to three mutations.
pub(super) fn mutant(krate: &Crate, seeded: &mut Seeded, pool: &Pool) -> Crate {
    let mut out = krate.clone();
    let rounds = 1 + usize::from(seeded.chance(30)) + usize::from(seeded.chance(10));
    for _ in 0..rounds {
        let mut tries = 0;
        while !mutate_once(&mut out, seeded, pool) && tries < 20 {
            tries += 1;
        }
    }
    out
}

// ---------------------------------------------------------------- templates

/// Every refusal the oracle reports, by the sentence it is built from. The
/// vectors are held to cover each one that a crate of the closed AST can
/// reach; `no-row` cannot be reached, because every construct the AST emits
/// has a row of the table, and the row-less path is the one the module
/// transcribes without a vector to reach it.
const REACHABLE: &[&str] = &[
    "enum-not-declared",
    "enum-declared-twice",
    "apply-declared-twice",
    "function-declared-twice",
    "heap-type",
    "type-adt-undeclared",
    "type-fn-undeclared",
    "heap-string-literal",
    "heap-byte-literal",
    "heap-list-match",
    "heap-boxed-read",
    "heap-box",
    "heap-runtime",
    "heap-empty-list",
    "heap-list-cell",
    "heap-boxed-capture",
    "bound-twice",
    "pattern-undeclared",
    "pattern-arity",
    "not-bound",
    "read-after-move",
    "read-after-part",
    "move-after-part",
    "moved-twice",
    "part-after-move",
    "part-moved-twice",
    "call-no-propagate",
    "call-spurious-propagate",
    "tail-not-fallible",
    "negated-zero",
    "loop-tail-not-fallible",
    "loop-moved",
    "loop-part-moved",
    "function-undeclared",
    "ctor-adt-undeclared",
    "ctor-adt-arity",
    "ctor-closure-undeclared",
    "ctor-closure-arity",
    "apply-untyped",
    "apply-undeclared",
    "apply-misstated",
    "dispatch-undeclared",
    "dispatch-misstated",
    "propagates-undeclared",
    "wrong-origin",
    "unused-element",
    "unused-type",
    "width",
];

/// The refusals only the hand-built crates reach: the committed crates hold no
/// machine call and no overflow value, so their mutants do not either (the
/// rule vectors are drawn until [`REACHABLE`] is, and these are counted
/// beside them).
const HAND_BUILT: &[&str] = &["machine-arity", "overflow-stray", "no-row"];

fn classify(message: &str) -> &'static str {
    let has = |needle: &str| message.contains(needle);
    if message.starts_with("machine: ") {
        "machine-arity"
    } else if message.starts_with("overflow: ") {
        "overflow-stray"
    } else if has("is not a declared type") {
        "enum-not-declared"
    } else if has("::apply is declared twice") {
        "apply-declared-twice"
    } else if message.starts_with("hygiene: ") && message.ends_with(" is declared twice") {
        if message.starts_with("hygiene: f") {
            "function-declared-twice"
        } else {
            "enum-declared-twice"
        }
    } else if has("rust-core renders the heap type") {
        "heap-type"
    } else if message.starts_with("type Adt") && message.ends_with(" is not declared") {
        "type-adt-undeclared"
    } else if message.starts_with("type Fn") && message.ends_with(" is not declared") {
        "type-fn-undeclared"
    } else if has("renders a string literal, which requires") {
        "heap-string-literal"
    } else if has("renders a byte string literal, which requires") {
        "heap-byte-literal"
    } else if has("renders a list match, which requires") {
        "heap-list-match"
    } else if has("renders a boxed read, which requires") {
        "heap-boxed-read"
    } else if has("renders a box, which requires") {
        "heap-box"
    } else if has("renders the runtime function") {
        "heap-runtime"
    } else if has("renders an empty list, which requires") {
        "heap-empty-list"
    } else if has("renders a list cell, which requires") {
        "heap-list-cell"
    } else if has("renders a boxed capture, which requires") {
        "heap-boxed-capture"
    } else if has(" is bound twice in one function") {
        "bound-twice"
    } else if has("the value a fallible loop ends with") {
        "loop-tail-not-fallible"
    } else if has("ownership: a component of ") && has(" in the body of a loop that runs again") {
        "loop-part-moved"
    } else if has("ownership: `") && has(" is moved in the body of a loop that runs again") {
        "loop-moved"
    } else if message.starts_with("pattern Adt") && has(" is not declared") {
        "pattern-undeclared"
    } else if message.starts_with("pattern Adt") && has(" binds ") {
        "pattern-arity"
    } else if has(" is read after it was moved") {
        "read-after-move"
    } else if has(" is read after a component was moved out of it") {
        "read-after-part"
    } else if has(" is moved after a component was moved out of it") {
        "move-after-part"
    } else if has("ownership: component ") && has(" is moved twice") {
        "part-moved-twice"
    } else if has(" is moved twice") {
        "moved-twice"
    } else if has("ownership: a component of ") {
        "part-after-move"
    } else if has("does not propagate its overflow") {
        "call-no-propagate"
    } else if has("propagates a failure it cannot have") {
        "call-spurious-propagate"
    } else if has("is not a fallible value") {
        "tail-not-fallible"
    } else if has("a negated zero test or predecessor") {
        "negated-zero"
    } else if has("is applied without a function type") {
        "apply-untyped"
    } else if has("::apply is not declared") {
        "apply-undeclared"
    } else if has("arithmetic: an application of Fn") {
        "apply-misstated"
    } else if has("dispatches to undeclared") {
        "dispatch-undeclared"
    } else if has("::apply misstates the failure of") {
        "dispatch-misstated"
    } else if has("propagates a failure its result type does not carry") {
        "propagates-undeclared"
    } else if has("has no target-semantics correspondence") {
        "no-row"
    } else if has("the element it was lowered from") {
        "wrong-origin"
    } else if has("realizes none of") {
        "unused-type"
    } else if has("which the program does not use") {
        "unused-element"
    } else if has("works at width") {
        "width"
    } else if message.starts_with("Adt") && has("::C") && has(" is not declared") {
        "ctor-adt-undeclared"
    } else if message.starts_with("Adt") && has(" receives ") {
        "ctor-adt-arity"
    } else if message.starts_with("Fn") && has("::F") && has(" is not declared") {
        "ctor-closure-undeclared"
    } else if message.starts_with("Fn") && has(" captures ") {
        "ctor-closure-arity"
    } else if message.ends_with(" is not declared") && message.starts_with('f') {
        "function-undeclared"
    } else if message.ends_with(" is not bound") {
        "not-bound"
    } else {
        panic!("the oracle's refusal belongs to no template: {message}")
    }
}

/// The refusal a run expects, by template.
fn template_of(run: &Run) -> Option<&'static str> {
    if run.1["kind"] != "error" {
        return None;
    }
    let hex = run.1["value"]["hex"].as_str().expect("hex");
    let raw: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).expect("hex digit"))
        .collect();
    Some(classify(
        &String::from_utf8(raw).expect("a refusal is text"),
    ))
}

/// How often each template occurs among `runs`.
fn coverage(runs: &[Run]) -> BTreeMap<&'static str, usize> {
    let mut out = BTreeMap::new();
    for run in runs {
        if let Some(template) = template_of(run) {
            *out.entry(template).or_insert(0) += 1;
        }
    }
    out
}

fn uncovered(counts: &BTreeMap<&'static str, usize>) -> Vec<&'static str> {
    REACHABLE
        .iter()
        .chain(HAND_BUILT)
        .copied()
        .filter(|template| !counts.contains_key(template))
        .collect()
}

// ----------------------------------------------------------------- corpora

/// The expression count of a crate: its size for the evaluator's budget.
pub(super) fn weight(krate: &Crate) -> usize {
    let mut copy = krate.clone();
    sites(&mut copy, Class::Expr)
}

/// The crates with at most `limit` expressions: the evaluator's share.
pub(super) fn small(all: &[Rendered], limit: usize) -> Vec<&Rendered> {
    all.iter()
        .filter(|rendered| weight(&rendered.krate) <= limit)
        .collect()
}

/// The elements a correspondence run is given: the program's, with one
/// removed or a stranger added now and then, so the element checks fire.
fn tweak(elements: &BTreeSet<String>, seeded: &mut Seeded) -> BTreeSet<String> {
    let mut out = elements.clone();
    if seeded.chance(30) && !out.is_empty() {
        let pick = seeded.below(out.len());
        let gone = out.iter().nth(pick).cloned().expect("an element");
        out.remove(&gone);
    }
    if seeded.chance(5) {
        out.insert("prim:stranger".to_owned());
    }
    out
}

/// `count` mutants of the crates of `bases`, each as a run of both roots.
fn mutants(bases: &[&Rendered], seeded: &mut Seeded, count: usize) -> (Vec<Run>, Vec<Run>) {
    let pools: Vec<Pool> = bases.iter().map(|base| pool_of(&base.krate)).collect();
    let mut validated = Vec::new();
    let mut corresponded = Vec::new();
    for _ in 0..count {
        let which = seeded.below(bases.len());
        let base = bases[which];
        let krate = mutant(&base.krate, seeded, &pools[which]);
        let elements = tweak(&base.elements, seeded);
        validated.push(validate_run(&krate));
        corresponded.push(correspond_run(&krate, &elements));
    }
    (validated, corresponded)
}

/// The reachable refusals that fewer than `each` runs of either root show.
fn unreached(
    validated: &BTreeMap<&'static str, usize>,
    corresponded: &BTreeMap<&'static str, usize>,
    each: usize,
) -> Vec<&'static str> {
    REACHABLE
        .iter()
        .copied()
        .filter(|template| {
            let reached =
                |seen: &BTreeMap<&'static str, usize>| seen.get(template).copied().unwrap_or(0);
            reached(validated).max(reached(corresponded)) < each
        })
        .collect()
}

/// Mutants of small crates, taken until every reachable refusal has been
/// seen `each` times, from the fixed seed: the rule vectors. Returns the
/// runs of both roots.
fn rule_vectors(bases: &[&Rendered], each: usize) -> (Vec<Run>, Vec<Run>) {
    let mut seeded = Seeded(0x05A1_1CE0_BE55_0005);
    let pools: Vec<Pool> = bases.iter().map(|base| pool_of(&base.krate)).collect();
    let mut validated: Vec<Run> = Vec::new();
    let mut corresponded: Vec<Run> = Vec::new();
    let mut seen_validate: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut seen_correspond: BTreeMap<&'static str, usize> = BTreeMap::new();
    for _ in 0..400_000 {
        let which = seeded.below(bases.len());
        let base = bases[which];
        let krate = mutant(&base.krate, &mut seeded, &pools[which]);
        let elements = tweak(&base.elements, &mut seeded);
        let run = validate_run(&krate);
        if let Some(template) = template_of(&run) {
            let seen = seen_validate.entry(template).or_insert(0);
            if *seen < each {
                *seen += 1;
                validated.push(run);
            }
        }
        let run = correspond_run(&krate, &elements);
        if let Some(template) = template_of(&run) {
            let seen = seen_correspond.entry(template).or_insert(0);
            if *seen < each {
                *seen += 1;
                corresponded.push(run);
            }
        }
        if unreached(&seen_validate, &seen_correspond, each).is_empty() {
            break;
        }
    }
    // The target is asserted, not hoped for: a generator that used its draws
    // up first would otherwise hand the case fewer runs than it states.
    let missing = unreached(&seen_validate, &seen_correspond, each);
    assert!(
        missing.is_empty(),
        "the rule corpus gave up before it reached {each} runs of: {missing:?}"
    );
    (validated, corresponded)
}

/// Crates by hand for the merge of moved sets at a branch: a name moved on
/// one arm of a match, or one branch of an `if`, and used after it. The
/// mutants reach this rarely, so these make it a vector.
pub(super) fn branch_crates() -> Vec<Crate> {
    let at = || Origin::of("expr:match");
    let local = |n: u64| Ident::Local(n);
    let unit = || Expr::Lit(Lit::Unit, Origin::of("value:unit"));
    let mut out = Vec::new();
    for arms in 2..=3usize {
        for moving in 0..arms {
            for after in 0..3usize {
                let mut cases: Vec<(Pat, Block)> = Vec::new();
                for arm in 0..arms {
                    let pat = if arm == 0 {
                        Pat::Some(Box::new(Pat::Wild))
                    } else if arm + 1 == arms {
                        Pat::None
                    } else {
                        Pat::Some(Box::new(Pat::Bind(local(10 + arm as u64))))
                    };
                    let body = if arm == moving {
                        Block::of(Expr::Move(local(0), Origin::of("read")))
                    } else {
                        Block::of(unit())
                    };
                    cases.push((pat, body));
                }
                let later = match after {
                    0 => Expr::Move(local(0), Origin::of("read")),
                    1 => Expr::Clone(local(0), Origin::of("read")),
                    _ => unit(),
                };
                let body = Block {
                    lets: vec![
                        Let {
                            pat: Pat::Wild,
                            ty: None,
                            value: Expr::Match {
                                scrutinee: Box::new(Expr::Copy(local(1), Origin::of("read"))),
                                arms: cases,
                                at: at(),
                            },
                            at: Origin::of("let"),
                        },
                        Let {
                            pat: Pat::Wild,
                            ty: None,
                            value: later,
                            at: Origin::of("let"),
                        },
                    ],
                    tail: unit(),
                };
                out.push(Crate {
                    profile: Profile::Std,
                    items: vec![ItemDef::Function {
                        name: Ident::Function(0),
                        parameters: vec![
                            (Pat::Bind(local(0)), Type::Str),
                            (Pat::Bind(local(1)), Type::Option(Box::new(Type::Nat))),
                        ],
                        result: Type::Unit,
                        body,
                        at: Origin::of("function"),
                    }],
                });
            }
        }
    }
    // The same on the two branches of an `if`.
    for moving in 0..2usize {
        let branch = |arm: usize| {
            if arm == moving {
                Block::of(Expr::Move(local(0), Origin::of("read")))
            } else {
                Block::of(unit())
            }
        };
        out.push(Crate {
            profile: Profile::Std,
            items: vec![ItemDef::Function {
                name: Ident::Function(0),
                parameters: vec![
                    (Pat::Bind(local(0)), Type::Str),
                    (Pat::Bind(local(1)), Type::Bool),
                ],
                result: Type::Unit,
                body: Block {
                    lets: vec![
                        Let {
                            pat: Pat::Wild,
                            ty: None,
                            value: Expr::If {
                                condition: Box::new(Expr::Copy(local(1), Origin::of("read"))),
                                then_branch: Box::new(branch(0)),
                                else_branch: Box::new(branch(1)),
                                at: Origin::of("expr:cond"),
                            },
                            at: Origin::of("let"),
                        },
                        Let {
                            pat: Pat::Wild,
                            ty: None,
                            value: Expr::Move(local(0), Origin::of("read")),
                            at: Origin::of("let"),
                        },
                    ],
                    tail: unit(),
                },
                at: Origin::of("function"),
            }],
        });
    }
    out
}

/// The elements a hand-built crate realizes: the origins of its constructs
/// and the rows its types name, so a correspondence run of it asks the oracle
/// about the loop and not only about an element the crate never used.
/// Crates by hand for the machine callee and the overflow value (`RB-15`,
/// `SP-18`): a call of each primitive family with one operand more than it
/// takes (the arity and the name of every row of the module's table),
/// a call of each arity with the right count and with a propagation it cannot
/// have, the overflow value outside an `Err` (refused) and in one (accepted).
/// No lowering emits any of them, so correspondence refuses each of the calls
/// as a construct with no row.
pub(super) fn machine_crates() -> Vec<Crate> {
    let at = Origin::of;
    let function = |body: Expr| Crate {
        profile: lexlean::calculus::rust::Profile::Core,
        items: vec![ItemDef::Function {
            name: Ident::Function(0),
            parameters: Vec::new(),
            result: Type::Unit,
            body: Block::of(body),
            at: at("function"),
        }],
    };
    let call = |machine: Machine, operands: usize, propagate: bool| Expr::Call {
        callee: Callee::Machine(machine),
        args: (0..operands)
            .map(|n| Expr::Lit(Lit::Nat(n as u64), at("value:nat")))
            .collect(),
        propagate,
        at: at("prim:machine"),
    };
    let mut crates = Vec::new();
    for machine in Machine::families() {
        crates.push(function(call(machine, machine.arity() + 1, false)));
    }
    for machine in [
        Machine::CheckedNeg(IntKind::U8),
        Machine::CheckedAdd(IntKind::U8),
        Machine::NatRem,
    ] {
        crates.push(function(call(machine, machine.arity(), false)));
    }
    crates.push(function(call(Machine::CheckedAdd(IntKind::U8), 2, true)));
    crates.push(function(Expr::Lit(Lit::Overflow, at("overflow"))));
    crates.push(function(Expr::Construct {
        ctor: Ctor::Err(Type::Unit, Type::Unit),
        args: vec![Expr::Lit(Lit::Overflow, at("overflow"))],
        at: at("construct:err"),
    }));
    // The types `Overflow` and `i128` have no row of the correspondence: a
    // function that takes one is valid and is refused by `correspond`.
    for ty in [Type::Overflow, Type::I128] {
        crates.push(Crate {
            profile: lexlean::calculus::rust::Profile::Core,
            items: vec![ItemDef::Function {
                name: Ident::Function(0),
                parameters: vec![(Pat::Bind(Ident::Param(0)), ty.clone())],
                result: ty,
                body: Block::of(Expr::Copy(Ident::Param(0), at("read:param"))),
                at: at("function"),
            }],
        });
    }
    // The runtime fails with `Err::<T, Overflow>(Overflow)` at the tail of a
    // fallible function: the one `Err` the arithmetic rule reads as a
    // fallible value. The same `Err` with another payload, and the overflow
    // value under another error type, are not.
    for (error, payload) in [
        (Type::Overflow, Lit::Overflow),
        (Type::Overflow, Lit::Nat(0)),
        (Type::Nat, Lit::Overflow),
    ] {
        crates.push(Crate {
            profile: lexlean::calculus::rust::Profile::Core,
            items: vec![ItemDef::Function {
                name: Ident::Function(0),
                parameters: vec![(Pat::Bind(Ident::Param(0)), Type::Nat)],
                result: Type::Fallible(Box::new(Type::Nat)),
                body: Block::of(Expr::Construct {
                    ctor: Ctor::Err(Type::Nat, error),
                    args: vec![Expr::Lit(payload, at("overflow"))],
                    at: at("construct:err"),
                }),
                at: at("function"),
            }],
        });
    }
    crates
}

pub(super) fn elements_of(krate: &Crate) -> BTreeSet<String> {
    let (instances, types) = validate::instances(krate);
    let mut out: BTreeSet<String> = instances
        .iter()
        .map(|instance| instance.origin.element.clone())
        .collect();
    for construct in types {
        for (name, realized) in validate::CORRESPONDENCE {
            if *name == construct {
                out.extend(realized.iter().map(|element| (*element).to_owned()));
            }
        }
    }
    out
}

/// Crates by hand for the loop rules of `calculus::rust::validate`: a loop's
/// state is moved by its body and nothing else outside the body is (both
/// refusals, with the names the oracle reports first), the value a fallible
/// loop ends with is a fallible value, and the body is checked as a block that
/// is not a tail. The mutants reach these rarely, so these make each rule a
/// vector in both directions.
// The crates are listed in the order the rules are described, each one pushed beside
// the reason it is there.
#[allow(clippy::too_many_lines, clippy::vec_init_then_push)]
pub(super) fn loop_crates() -> Vec<Crate> {
    let at = Origin::of;
    let local = Ident::Local;
    let read = |ident: Ident| Expr::Move(ident, at("read"));
    let copy = |ident: Ident| Expr::Copy(ident, at("read"));
    let succeed = |inner: Expr| Expr::Succeed(Box::new(inner), at("overflow"));
    let err = |inner: Expr| Expr::Construct {
        ctor: Ctor::Err(Type::Nat, Type::Nat),
        args: vec![inner],
        at: at("construct:err"),
    };
    let ok = |inner: Expr| Expr::Construct {
        ctor: Ctor::Ok(Type::Nat, Type::Nat),
        args: vec![inner],
        at: at("construct:ok"),
    };
    let nat = || Expr::Lit(Lit::Nat(0), at("value:nat"));
    let unit = || Expr::Lit(Lit::Unit, at("value:unit"));
    let discard = |value: Expr| Let {
        pat: Pat::Wild,
        ty: None,
        value,
        at: at("let"),
    };
    let bind = |ident: Ident, value: Expr| Let {
        pat: Pat::Bind(ident),
        ty: None,
        value,
        at: at("let"),
    };
    let part = |ident: Ident, which: usize, value: Expr| Let {
        pat: Pat::Tuple(if which == 0 {
            vec![Pat::Bind(ident), Pat::Wild]
        } else {
            vec![Pat::Wild, Pat::Bind(ident)]
        }),
        ty: None,
        value,
        at: at("destructure:pair"),
    };
    // A loop over `state` whose body runs `lets` and ends with `end` when the
    // state is zero and with the next state otherwise.
    let counting = |state: Ident, lets: Vec<Let>, end: Expr| Expr::Loop {
        state: state.clone(),
        body: Box::new(Block {
            lets,
            tail: Expr::If {
                condition: Box::new(Expr::IsZero(state.clone(), at("match:nat"))),
                then_branch: Box::new(Block::of(end)),
                else_branch: Box::new(Block::of(ok(Expr::Predecessor(state)))),
                at: at("if"),
            },
        }),
    };
    let parameters = || {
        vec![
            (Pat::Bind(local(0)), Type::Nat),
            (Pat::Bind(local(1)), Type::Str),
            (
                Pat::Bind(local(2)),
                Type::Pair(Box::new(Type::Nat), Box::new(Type::Str)),
            ),
            (Pat::Bind(local(3)), Type::Option(Box::new(Type::Nat))),
            (
                Pat::Bind(local(4)),
                Type::Pair(Box::new(Type::Nat), Box::new(Type::Str)),
            ),
        ]
    };
    let fallible = || Type::Fallible(Box::new(Type::Nat));
    let function = |parameters: Vec<(Pat, Type)>, result: Type, lets: Vec<Let>, tail: Expr| Crate {
        profile: Profile::Std,
        items: vec![ItemDef::Function {
            name: Ident::Function(0),
            parameters,
            result,
            body: Block { lets, tail },
            at: at("function"),
        }],
    };
    let plain =
        |result: Type, lets: Vec<Let>, tail: Expr| function(parameters(), result, lets, tail);
    let body_of = |lets: Vec<Let>, tail: Expr| Expr::Loop {
        state: local(0),
        body: Box::new(Block { lets, tail }),
    };
    let mut out = Vec::new();
    // The fallible-tail rule: accepted ends, refused ends, and the first
    // refused end named.
    out.push(plain(
        Type::Nat,
        vec![],
        counting(local(0), vec![], err(copy(local(0)))),
    ));
    out.push(plain(
        fallible(),
        vec![],
        counting(local(0), vec![], err(succeed(copy(local(0))))),
    ));
    out.push(plain(
        fallible(),
        vec![],
        counting(local(0), vec![], err(copy(local(0)))),
    ));
    out.push(plain(
        fallible(),
        vec![],
        counting(local(0), vec![], err(nat())),
    ));
    out.push(plain(
        fallible(),
        vec![],
        counting(
            local(0),
            vec![],
            err(Expr::If {
                condition: Box::new(Expr::IsZero(local(0), at("match:nat"))),
                then_branch: Box::new(Block::of(unit())),
                else_branch: Box::new(Block::of(unit())),
                at: at("if"),
            }),
        ),
    ));
    let branching = |first: Expr, second: Expr| {
        body_of(
            vec![],
            Expr::If {
                condition: Box::new(Expr::IsZero(local(0), at("match:nat"))),
                then_branch: Box::new(Block::of(err(first))),
                else_branch: Box::new(Block::of(Expr::If {
                    condition: Box::new(Expr::NonZero(local(0), at("match:nat"))),
                    then_branch: Box::new(Block::of(ok(Expr::Predecessor(local(0))))),
                    else_branch: Box::new(Block::of(err(second))),
                    at: at("if"),
                })),
                at: at("if"),
            },
        )
    };
    out.push(plain(
        fallible(),
        vec![],
        branching(succeed(copy(local(0))), nat()),
    ));
    out.push(plain(fallible(), vec![], branching(copy(local(0)), nat())));
    out.push(plain(
        fallible(),
        vec![],
        branching(succeed(copy(local(0))), succeed(nat())),
    ));
    let matching = |first: Expr, second: Expr| {
        body_of(
            vec![],
            Expr::Match {
                scrutinee: Box::new(copy(local(3))),
                arms: vec![
                    (Pat::Some(Box::new(Pat::Wild)), Block::of(err(first))),
                    (Pat::None, Block::of(err(second))),
                ],
                at: at("expr:match"),
            },
        )
    };
    out.push(plain(
        fallible(),
        vec![],
        matching(succeed(copy(local(0))), copy(local(0))),
    ));
    out.push(plain(
        fallible(),
        vec![],
        matching(succeed(copy(local(0))), succeed(nat())),
    ));
    out.push(plain(
        fallible(),
        vec![],
        body_of(
            vec![],
            Expr::Block(Box::new(Block::of(err(copy(local(0)))))),
        ),
    ));
    // Not the value of the function, or not of a fallible one: unchecked.
    out.push(plain(
        fallible(),
        vec![discard(counting(local(0), vec![], err(copy(local(0)))))],
        succeed(unit()),
    ));
    out.push(plain(
        fallible(),
        vec![],
        Expr::If {
            condition: Box::new(copy(local(3))),
            then_branch: Box::new(Block::of(counting(local(0), vec![], err(copy(local(0)))))),
            else_branch: Box::new(Block::of(succeed(nat()))),
            at: at("if"),
        },
    ));
    out.push(plain(
        Type::Unit,
        vec![discard(counting(local(0), vec![], err(copy(local(0)))))],
        unit(),
    ));
    // Ownership across the body: refused moves, the name the oracle reports
    // first, and the moves a body may make.
    out.push(plain(
        Type::Nat,
        vec![],
        counting(local(0), vec![discard(read(local(1)))], err(copy(local(0)))),
    ));
    out.push(plain(
        Type::Nat,
        vec![],
        counting(
            local(0),
            vec![discard(read(local(2))), discard(read(local(1)))],
            err(copy(local(0))),
        ),
    ));
    // The names order by kind and then by number or bytes, whatever order
    // the body moves them in.
    for moved in [
        vec![Ident::Export("e".to_owned()), local(6), Ident::Param(5)],
        vec![Ident::Export("e".to_owned()), Ident::Param(5)],
        vec![Ident::Export("b".to_owned()), Ident::Export("a".to_owned())],
    ] {
        let mut parameters = vec![(Pat::Bind(local(0)), Type::Nat)];
        parameters.extend(
            moved
                .iter()
                .map(|ident| (Pat::Bind(ident.clone()), Type::Str)),
        );
        out.push(function(
            parameters,
            Type::Nat,
            vec![],
            counting(
                local(0),
                moved
                    .into_iter()
                    .map(|ident| discard(read(ident)))
                    .collect(),
                err(copy(local(0))),
            ),
        ));
    }
    out.push(plain(
        Type::Nat,
        vec![],
        counting(
            local(0),
            vec![
                bind(local(8), Expr::Clone(local(1), at("read"))),
                discard(read(local(8))),
            ],
            err(copy(local(0))),
        ),
    ));
    out.push(plain(
        Type::Nat,
        vec![],
        body_of(vec![], ok(read(local(0)))),
    ));
    out.push(plain(
        Type::Nat,
        vec![discard(read(local(1)))],
        counting(local(0), vec![discard(read(local(1)))], err(copy(local(0)))),
    ));
    out.push(plain(
        Type::Nat,
        vec![],
        counting(
            local(0),
            vec![discard(counting(local(0), vec![], err(copy(local(0)))))],
            err(copy(local(0))),
        ),
    ));
    // Components of a pair: moved in the body (refused), moved before the
    // loop and not in it, moved again, a body's own pair.
    out.push(plain(
        Type::Nat,
        vec![],
        counting(
            local(0),
            vec![part(Ident::Part(1), 0, read(local(2)))],
            err(copy(local(0))),
        ),
    ));
    out.push(plain(
        Type::Nat,
        vec![part(Ident::Part(1), 0, read(local(2)))],
        counting(local(0), vec![], err(copy(local(0)))),
    ));
    out.push(plain(
        Type::Nat,
        vec![part(Ident::Part(1), 0, read(local(2)))],
        counting(
            local(0),
            vec![part(Ident::Part(2), 1, read(local(2)))],
            err(copy(local(0))),
        ),
    ));
    out.push(plain(
        Type::Nat,
        vec![part(Ident::Part(1), 0, read(local(2)))],
        counting(
            local(0),
            vec![part(Ident::Part(2), 0, read(local(2)))],
            err(copy(local(0))),
        ),
    ));
    out.push(plain(
        Type::Nat,
        vec![],
        counting(
            local(0),
            vec![
                bind(local(8), Expr::Clone(local(2), at("read"))),
                part(Ident::Part(1), 0, read(local(8))),
            ],
            err(copy(local(0))),
        ),
    ));
    out.push(plain(
        Type::Nat,
        vec![],
        counting(
            local(0),
            vec![
                part(Ident::Part(1), 0, read(local(4))),
                part(Ident::Part(2), 0, read(local(2))),
            ],
            err(copy(local(0))),
        ),
    ));
    // A state the block did not bind, and an inner loop that moves what the
    // outer body does not own.
    out.push(plain(
        Type::Nat,
        vec![],
        counting(local(9), vec![], err(nat())),
    ));
    out.push(plain(
        Type::Nat,
        vec![],
        counting(
            local(0),
            vec![
                bind(local(8), Expr::Clone(local(1), at("read"))),
                discard(counting(
                    local(8),
                    vec![discard(read(local(1)))],
                    err(nat()),
                )),
            ],
            err(copy(local(0))),
        ),
    ));
    out.push(plain(
        Type::Nat,
        vec![],
        counting(
            local(0),
            vec![
                bind(local(8), Expr::Copy(local(0), at("read"))),
                discard(counting(local(8), vec![], err(copy(local(8))))),
            ],
            err(copy(local(0))),
        ),
    ));
    // The body is checked as a block that is not a tail: an application of a
    // fallible function value that drops its failure is refused there even
    // when the loop is the value of a fallible function.
    for propagate in [false, true] {
        let mut parameters = parameters();
        parameters.push((Pat::Bind(local(5)), Type::Fn(0)));
        out.push(Crate {
            profile: Profile::Std,
            items: vec![
                ItemDef::Enum {
                    name: Type::Fn(0),
                    variants: vec![(0, vec![])],
                    at: at("enum:closures"),
                },
                ItemDef::Apply {
                    fn_type: 0,
                    parameters: vec![],
                    result: Type::Nat,
                    fallible: true,
                    arms: vec![],
                    at: at("apply"),
                },
                ItemDef::Function {
                    name: Ident::Function(0),
                    parameters,
                    result: fallible(),
                    body: Block::of(body_of(
                        vec![],
                        Expr::Apply {
                            holder: local(5),
                            args: vec![],
                            propagate,
                            at: at("apply"),
                        },
                    )),
                    at: at("function"),
                },
            ],
        });
    }
    out
}

fn refused(runs: &[Run]) -> usize {
    runs.iter().filter(|run| run.1["kind"] == "error").count()
}

/// A run's arguments printed back as the crate they encode is not needed: the
/// encoders are checked against the oracle by the differential itself.
fn merged(parts: &[&[Run]]) -> Vec<Run> {
    parts.iter().flat_map(|part| part.iter().cloned()).collect()
}

/// The nodes of a value: what a literal of it costs rustc.
pub(super) fn value_nodes(value: &Value) -> usize {
    1 + match value {
        Value::Adt { fields, .. } => fields.iter().map(value_nodes).sum(),
        Value::List { items } => items.iter().map(value_nodes).sum(),
        Value::Pair { left, right } => value_nodes(left) + value_nodes(right),
        Value::Some { value } | Value::Ok { value } | Value::Error { value } => value_nodes(value),
        _ => 0,
    }
}

/// The nodes of the arguments one harness binary may hold. A crate is data
/// of thousands of nodes, which rustc compiles as one literal each, so the
/// runs are spread over several harness binaries of one compiled package.
pub(super) const HARNESS_NODES: usize = 60_000;

/// The rendered `rust-std` packages of the roots, compiled by the pinned
/// rustc, run on `runs` of each root: one outcome per run, each equal to the
/// oracle's. Returns the number of runs.
pub(super) fn rustc_differential(
    id: &str,
    roots: &[(&str, &Lowered, &[Run])],
    label: &str,
) -> usize {
    let started = std::time::Instant::now();
    let dir = tempfile::Builder::new()
        .prefix(&format!("lexlean-bootstrap-{}-", id.to_ascii_lowercase()))
        .tempdir()
        .expect("tempdir");
    let mut members = Vec::new();
    // (binary, expected outcomes) in order
    let mut expected: Vec<(String, String, Vec<Json>)> = Vec::new();
    for (root, low, runs) in roots {
        let manifest = rust_packages::manifest(
            &format!("boot_{}", snake(root)),
            &low.program,
            low.entry(),
            Profile::Std,
        );
        let package = target::package(&manifest.to_file_bytes().expect("canonical manifest"))
            .unwrap_or_else(|failure| panic!("{root}: {failure:?}"));
        let name = manifest.name.clone();
        for (path, bytes) in &package.files {
            let file = dir.path().join("p").join(&name).join(path);
            std::fs::create_dir_all(file.parent().expect("parent")).expect("package directory");
            std::fs::write(file, bytes).expect("package file");
        }
        members.push(format!("\"p/{name}\""));
        let arity = low.program.functions[usize::try_from(low.entry()).expect("entry")]
            .types
            .len();
        let passing = vec![Passing::Own; arity];
        let mut chunks: Vec<Vec<&Run>> = Vec::new();
        let mut weight = 0;
        for run in *runs {
            let nodes: usize = run.0.iter().map(value_nodes).sum();
            if chunks.is_empty() || weight + nodes > HARNESS_NODES {
                chunks.push(Vec::new());
                weight = 0;
            }
            weight += nodes;
            chunks.last_mut().expect("a chunk").push(run);
        }
        for (number, chunk) in chunks.iter().enumerate() {
            let arguments: Vec<Vec<Value>> = chunk.iter().map(|run| run.0.clone()).collect();
            let source = rust_harness::render_calls(
                &low.program,
                Profile::Std,
                &name,
                &[Calls {
                    entry: low.entry(),
                    function: "run",
                    passing: &passing,
                    arguments: &arguments,
                }],
            )
            .expect("the differential harness");
            let binary = format!("run_{name}_{number}");
            let runner = dir.path().join("h").join(&binary);
            std::fs::create_dir_all(runner.join("src")).expect("harness directory");
            std::fs::write(
                runner.join("Cargo.toml"),
                format!(
                    "[package]\nname = \"{binary}\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{name} = {{ path = \"../../p/{name}\" }}\n"
                ),
            )
            .expect("harness manifest");
            std::fs::write(runner.join("src/main.rs"), source).expect("harness source");
            members.push(format!("\"h/{binary}\""));
            expected.push((
                binary,
                name.clone(),
                chunk.iter().map(|run| run.1.clone()).collect(),
            ));
        }
    }
    std::fs::write(
        dir.path().join("Cargo.toml"),
        format!(
            "[workspace]\nresolver = \"2\"\nmembers = [{}]\n",
            members.join(", ")
        ),
    )
    .expect("workspace manifest");
    let build = cargo_in(
        dir.path(),
        &[
            "build",
            "--offline",
            "--workspace",
            "--quiet",
            "--jobs",
            "2",
        ],
    );
    assert!(
        build.status.success(),
        "a root's package does not build:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    eprintln!(
        "{id}: {label}: rustc built the packages and {} harness binaries ({:?})",
        expected.len(),
        started.elapsed()
    );
    let mut total = 0;
    let mut bounds: Vec<(String, std::path::PathBuf)> = Vec::new();
    for (binary, package, outcomes) in &expected {
        let path = dir
            .path()
            .join("target/debug")
            .join(format!("{binary}{}", std::env::consts::EXE_SUFFIX));
        let ran = std::process::Command::new(&path)
            .output()
            .expect("the differential runs");
        assert!(
            ran.status.success(),
            "{binary}: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        bounds.push((package.clone(), path.clone()));
        let stdout = String::from_utf8(ran.stdout).expect("utf8");
        let printed: Vec<Json> = stdout
            .lines()
            .filter(|line| !line.starts_with("work "))
            .map(|line| {
                serde_json::from_str(line)
                    .unwrap_or_else(|error| panic!("{binary}: {error}: {line}"))
            })
            .collect();
        assert_eq!(
            printed.len(),
            outcomes.len(),
            "{binary}: one outcome per run"
        );
        let disagreements: Vec<String> = printed
            .iter()
            .zip(outcomes)
            .enumerate()
            .filter(|(_, (printed, expected))| printed != expected)
            .map(|(run, (printed, expected))| format!("run {run}: {printed} != {expected}"))
            .collect();
        assert!(
            disagreements.is_empty(),
            "{binary}: the rendering and the Rust oracle disagree on {} of {} runs: {:?}",
            disagreements.len(),
            outcomes.len(),
            &disagreements[..disagreements.len().min(3)]
        );
        total += outcomes.len();
    }
    eprintln!(
        "{id}: {label}: {total} rustc runs agree with the oracle ({:?})",
        started.elapsed()
    );
    // PD-08: the roots recurse with the depth of the crate, so the harness
    // that ran them records (measuring) or demands (otherwise) their stack
    // bound; the row keeps the largest need of the corpora. The bounds are
    // taken after every outcome is compared, so a package without a recorded
    // bound never hides a disagreement.
    for (package, path) in &bounds {
        crate::stack_bounds::observe(&crate::stack_bounds::subject("bootstrap", package), path);
    }
    total
}

/// A defect planted in the source: the check it removes, the text of the
/// source it is made at, and the root whose differential must catch it.
const PLANTS: [super::Plant; 20] = [
    // The number of operands of a machine call is not compared with the
    // primitive's.
    super::Plant {
        label: "machine arity not checked",
        site: "(match ((exprLen args) =? (machineArity m))",
        replacement: "(match true",
        root: "validateCrate",
    },
    // The overflow value outside the payload of an `Err` is accepted.
    super::Plant {
        label: "stray overflow value accepted",
        site: "| Lit.LitOverflow => (ctor Result.error [St, Bytes] (text \"overflow: the overflow value stands outside the payload of an `Err`\"))",
        replacement: "| Lit.LitOverflow => (ctor Result.ok [St, Bytes] st)",
        root: "validateCrate",
    },
    // The overflow value in the payload of an `Err` is refused too.
    super::Plant {
        label: "overflow value in an Err refused",
        site: "(let skip : Bool := (if err then (isOverflowExpr head) else false);",
        replacement: "(let skip : Bool := false;",
        root: "validateCrate",
    },
    // A machine call is not an instance of correspondence, so no row is
    // asked for it.
    super::Plant {
        label: "machine call not an instance of correspondence",
        site: "| Callee.CalMachine m => (foundAt found (text \"call:machine\") org)",
        replacement: "| Callee.CalMachine m => found",
        root: "correspondCrate",
    },
    // The profile check of a type is skipped: a heap type passes under
    // `rust-core`.
    super::Plant {
        label: "heap type admitted under rust-core",
        site: "(let blocked : Bool := (if (project cx core) then heapy else false);",
        replacement: "(let blocked : Bool := (if (project cx core) then false else false);",
        root: "validateCrate",
    },
    // A name bound twice in one function is accepted.
    super::Plant {
        label: "name bound twice accepted",
        site: "| Pat.PBind who => (match (idHas (project st bound) who)",
        replacement: "| Pat.PBind who => (match (idHas (nil Id) who)",
        root: "validateCrate",
    },
    // The moved sets of the branches of a match are not merged: only the
    // last branch's survive, so a use after a move on an earlier branch is
    // not seen.
    super::Plant {
        label: "moves on earlier match branches forgotten",
        site: "moved := (unionIds (project acc moved) (project bArm3 moved))",
        replacement: "moved := (project bArm3 moved)",
        root: "validateCrate",
    },
    // A component moved after its whole was moved is accepted.
    super::Plant {
        label: "component moved after its whole accepted",
        site: "(match (idHas (project st moved) who)\n      | Bool.true => (ctor Result.error [St, Bytes] (cat (text \"ownership: a component of \")",
        replacement: "(match false\n      | Bool.true => (ctor Result.error [St, Bytes] (cat (text \"ownership: a component of \")",
        root: "validateCrate",
    },
    // A type naming an undeclared ADT is accepted.
    super::Plant {
        label: "undeclared ADT type accepted",
        site: "(match (hasVariants (project (project cx decl) adts) n)",
        replacement: "(match true",
        root: "validateCrate",
    },
    // A `?` in a function whose result carries no failure is accepted.
    super::Plant {
        label: "propagation in an infallible function accepted",
        site: "(if (project s2 propagates) then (not fallible) else false)",
        replacement: "false",
        root: "validateCrate",
    },
    // A fallible call that does not propagate is accepted.
    super::Plant {
        label: "fallible call without propagation accepted",
        site: "(let excused : Bool := (if tail then (project cx fallible) else false);",
        replacement: "(let excused : Bool := true;",
        root: "validateCrate",
    },
    // An `apply` arm that misstates its function's failure is accepted
    // when the function can fail and the arm says it cannot, or the reverse.
    super::Plant {
        label: "dispatch failure not compared with the function's",
        site: "(let differs : Bool := (if isFunction then (if functionFallible then false else true) else functionFallible);",
        replacement: "(let differs : Bool := false;",
        root: "validateCrate",
    },
    // A correspondence row loses an element it names (`expr:var`, which
    // `expr:match` replaces).
    super::Plant {
        label: "correspondence row narrowed",
        site: "(cons ((text \"read\"), (cons (text \"expr:var\")",
        replacement: "(cons ((text \"read\"), (cons (text \"expr:match\")",
        root: "correspondCrate",
    },
    // The width of a runtime call is compared only when the call is
    // fixed-width, not when its origin is.
    super::Plant {
        label: "width of a runtime call not compared",
        site: "(LexLeanStd.Text::startsWith construct (text \"call:runtime:\"))",
        replacement: "false",
        root: "correspondCrate",
    },
    // A loop's body may move any name outside it.
    super::Plant {
        label: "name moved in a loop body accepted",
        site: "(let offends : Bool := (if same then false else (if kept then false else seen));",
        replacement: "(let offends : Bool := false;",
        root: "validateCrate",
    },
    // A loop's body may move components of any name outside it.
    super::Plant {
        label: "component moved in a loop body accepted",
        site: "(let offends : Bool := (if seen then (if same then false else true) else false);",
        replacement: "(let offends : Bool := false;",
        root: "validateCrate",
    },
    // The value a fallible loop ends with is not looked at.
    super::Plant {
        label: "end of a fallible loop not checked",
        site: "| Bool.true => (endsBlock (project cx items) body)\n",
        replacement: "| Bool.true => (ctor Result.ok [Unit, Bytes] ())\n",
        root: "validateCrate",
    },
    // A loop does not move its state into its own binding.
    super::Plant {
        label: "loop state not moved into the loop",
        site: "| Result.ok done0 => (match (consume st state)",
        replacement: "| Result.ok done0 => (match (ctor Result.ok [St, Bytes] st)",
        root: "validateCrate",
    },
    // The name a loop's refusal reports is the earliest moved, not the least
    // in the oracle's order of names.
    super::Plant {
        label: "name reported is not the least",
        site: "| Option.some seen => (match (idLess who seen)",
        replacement: "| Option.some seen => (match true",
        root: "validateCrate",
    },
    // A loop's body is checked as the tail of the function it is the value of.
    super::Plant {
        label: "loop body checked as a tail",
        site: "| Expr.ELoop state body => (applyBlock types decl body false)",
        replacement: "| Expr.ELoop state body => (applyBlock types decl body tail)",
        root: "validateCrate",
    },
];

/// The roots' eligibility reports and declared effects.
fn eligibility(project: &support::P) {
    eligibility_of(project, "RustValidate", MODULE, &ROOTS);
}

/// The eligibility reports and declared effects of the production roots of
/// one module of the bootstrap project, in order.
pub(super) fn eligibility_of(
    project: &support::P,
    file: &str,
    module: &str,
    roots: &[(&str, &[&str])],
) {
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join(format!("production/LexLeanBoot/{file}.eligibility.json"))
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
        roots
            .iter()
            .map(|(root, _)| format!("{module}.{root}"))
            .collect::<Vec<_>>(),
        "the roots are the module's production roots, in order"
    );
    for (row, (root, effects)) in reported.iter().zip(roots) {
        let declared: Vec<&str> = row["declared_effects"]
            .as_array()
            .expect("declared effects")
            .iter()
            .map(|effect| effect.as_str().expect("an effect"))
            .collect();
        assert_eq!(declared, *effects, "{root}: the declared effects");
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
            realized, *effects,
            "{root}: the declaration admits exactly the realized effects"
        );
    }
}

/// The crates the evaluator is given in full: those of at most this many
/// expressions. The larger ones, which are the compiler's own packages, are
/// held to the oracle by the compiled differential.
const EVALUATOR_CAP: usize = 400;
/// Crates a mutation corpus draws its bases from.
const BASE_CAP: usize = 80;
const EVALUATOR_MUTANTS: usize = 500;
const RUSTC_MUTANTS: usize = 2_000;
const RULES_EACH: usize = 3;

// The counts the specification states, over the committed tree: SPEC.md
// §17.23, the register statement of BS-05, its scenario, and VERIFICATION.md
// restate them, and the case fails when the tree no longer gives them.
const COMMITTED_MANIFESTS: usize = 256;
const COMMITTED_FIXTURES: usize = 122;
const COMMITTED_PROGRAMS: usize = 173;
const COMMITTED_CRATES: usize = 232;
const RULE_VALIDATE_RUNS: usize = 132;
const RULE_CORRESPOND_RUNS: usize = 12;
const BRANCH_CRATES: usize = 17;
const LOOP_CRATES: usize = 34;
const MACHINE_CRATES: usize = 26 + 3 + 1 + 2 + 2 + 3;
const VECTORS_VALIDATE: usize = 452;
const REFUSED_VALIDATE: usize = 195;
const VECTORS_CORRESPOND: usize = 315;
const REFUSED_CORRESPOND: usize = 82;
const EVALUATOR_CRATES: usize = 201;
const EVALUATOR_VALIDATE: usize = 421;
const EVALUATOR_CORRESPOND: usize = 284;
const SEEDED_REFUSED_VALIDATE: usize = 271;
const SEEDED_REFUSED_CORRESPOND: usize = 244;

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("RustValidate")
            && checked_units.units.contains_key("LexLeanStd.Text"),
        "RustValidate and LexLeanStd.Text are linked modules of the project"
    );
    eligibility(&project);

    let (sources, manifests, fixtures, distinct) = committed();
    let all = rendered(&sources);
    // The counts SPEC.md §17.23, the register statement, and the scenario
    // state are asserted here, so a repository that grows fails the case
    // instead of leaving a stale statement behind.
    assert_eq!(
        (manifests, fixtures, distinct, all.len()),
        (
            COMMITTED_MANIFESTS,
            COMMITTED_FIXTURES,
            COMMITTED_PROGRAMS,
            COMMITTED_CRATES
        ),
        "the committed manifests, fixtures, distinct programs, and distinct crates"
    );
    let bases = small(&all, BASE_CAP);
    let (mut rule_validate, rule_correspond) = rule_vectors(&bases, RULES_EACH);
    let by_hand: Vec<Crate> = branch_crates()
        .into_iter()
        .chain(loop_crates())
        .chain(machine_crates())
        .collect();
    assert_eq!(
        (rule_validate.len(), rule_correspond.len()),
        (RULE_VALIDATE_RUNS, RULE_CORRESPOND_RUNS),
        "the rule runs: each reachable refusal {RULES_EACH} times"
    );
    assert_eq!(
        (
            branch_crates().len(),
            loop_crates().len(),
            machine_crates().len()
        ),
        (BRANCH_CRATES, LOOP_CRATES, MACHINE_CRATES),
        "the hand-built crates"
    );
    rule_validate.extend(by_hand.iter().map(validate_run));
    let mut rule_correspond = rule_correspond;
    rule_correspond.extend(
        loop_crates()
            .iter()
            .chain(machine_crates().iter())
            .map(|krate| correspond_run(krate, &elements_of(krate))),
    );
    let mut counts = coverage(&rule_validate);
    for (template, count) in coverage(&rule_correspond) {
        *counts.entry(template).or_insert(0) += count;
    }
    assert!(
        uncovered(&counts).is_empty(),
        "refusals no rule vector reaches: {:?}",
        uncovered(&counts)
    );
    let accepted_validate: Vec<Run> = all
        .iter()
        .map(|rendered| validate_run(&rendered.krate))
        .collect();
    let accepted_correspond: Vec<Run> = all
        .iter()
        .map(|rendered| correspond_run(&rendered.krate, &rendered.elements))
        .collect();
    assert!(
        refused(&accepted_validate) == 0 && refused(&accepted_correspond) == 0,
        "the crates the oracle lowers are accepted by the oracle"
    );
    let vectors_validate = merged(&[&accepted_validate, &rule_validate]);
    let vectors_correspond = merged(&[&accepted_correspond, &rule_correspond]);
    assert_eq!(
        (
            vectors_validate.len(),
            refused(&vectors_validate),
            vectors_correspond.len(),
            refused(&vectors_correspond)
        ),
        (
            VECTORS_VALIDATE,
            REFUSED_VALIDATE,
            VECTORS_CORRESPOND,
            REFUSED_CORRESPOND
        ),
        "the vectors of validateCrate and correspondCrate, and the refused among them"
    );
    eprintln!(
        "BS-05: the vectors are the counted ones ({:?})",
        started.elapsed()
    );
    let checked = support::checked_project(&project);
    let validate_root = super::lower(&checked, &project, MODULE, "validateCrate");
    let correspond_root = super::lower(&checked, &project, MODULE, "correspondCrate");
    // On every host: the rendered rust-std packages of both roots, compiled
    // by that host's pinned rustc, agree with the oracle on the vectors.
    rustc_differential(
        "BS-05",
        &[
            ("validateCrate", &validate_root, &vectors_validate),
            ("correspondCrate", &correspond_root, &vectors_correspond),
        ],
        "vectors",
    );
    if !support::normative_host() {
        eprintln!(
            "BS-05: platform-bound host: the evaluator, the seeded corpora, and the planted defects run on the normative host ({:?})",
            started.elapsed()
        );
        return;
    }
    // On the normative host: the evaluator agrees on the vectors within its
    // cap and on the seeded mutants; the larger corpus runs under rustc.
    let within: Vec<usize> = all
        .iter()
        .enumerate()
        .filter(|(_, rendered)| weight(&rendered.krate) <= EVALUATOR_CAP)
        .map(|(index, _)| index)
        .collect();
    let evaluator_validate: Vec<Run> = within
        .iter()
        .map(|index| accepted_validate[*index].clone())
        .chain(rule_validate.iter().cloned())
        .collect();
    let evaluator_correspond: Vec<Run> = within
        .iter()
        .map(|index| accepted_correspond[*index].clone())
        .chain(rule_correspond.iter().cloned())
        .collect();
    for (root, low, runs) in [
        ("validateCrate", &validate_root, &evaluator_validate),
        ("correspondCrate", &correspond_root, &evaluator_correspond),
    ]
    .into_iter()
    {
        let disagreements = super::evaluate(low, runs, 5);
        assert!(
            disagreements.is_empty(),
            "{root}: the evaluator and the Rust validator disagree on the vectors: {disagreements:?}"
        );
    }
    assert_eq!(
        (
            within.len(),
            evaluator_validate.len(),
            evaluator_correspond.len()
        ),
        (EVALUATOR_CRATES, EVALUATOR_VALIDATE, EVALUATOR_CORRESPOND),
        "the crates of at most {EVALUATOR_CAP} expressions and the vectors the evaluator is given"
    );
    eprintln!(
        "BS-05: the evaluator agrees on the vectors ({:?})",
        started.elapsed()
    );
    let mut seeded = Seeded(0x0B55_0005);
    let (seeded_validate, seeded_correspond) = mutants(&bases, &mut seeded, EVALUATOR_MUTANTS);
    for (root, low, runs) in [
        ("validateCrate", &validate_root, &seeded_validate),
        ("correspondCrate", &correspond_root, &seeded_correspond),
    ]
    .into_iter()
    {
        let disagreements = super::evaluate(low, runs, 5);
        assert!(
            disagreements.is_empty(),
            "{root}: the evaluator and the Rust validator disagree on the seeded mutants: {disagreements:?}"
        );
    }
    assert_eq!(
        (
            seeded_validate.len(),
            seeded_correspond.len(),
            refused(&seeded_validate),
            refused(&seeded_correspond)
        ),
        (
            EVALUATOR_MUTANTS,
            EVALUATOR_MUTANTS,
            SEEDED_REFUSED_VALIDATE,
            SEEDED_REFUSED_CORRESPOND
        ),
        "the seeded mutants the evaluator is given, and the refused among them"
    );
    eprintln!(
        "BS-05: the evaluator agrees on the seeded mutants ({:?})",
        started.elapsed()
    );
    let mut large = Seeded(0x0B55_0055);
    let (large_validate, large_correspond) = mutants(&bases, &mut large, RUSTC_MUTANTS);
    rustc_differential(
        "BS-05",
        &[
            ("validateCrate", &validate_root, &large_validate),
            ("correspondCrate", &correspond_root, &large_correspond),
        ],
        "seeded",
    );
    // The gate is armed: each defect planted in a copy of the source is a
    // disagreement with the oracle.
    let mut detection: BTreeMap<&str, Vec<Run>> = BTreeMap::new();
    detection.insert("validateCrate", merged(&[&rule_validate, &seeded_validate]));
    detection.insert(
        "correspondCrate",
        merged(&[&rule_correspond, &seeded_correspond]),
    );
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/RustValidate.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/RustValidate.lex.tex",
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
            "BS-05: plant detected: {}: {}",
            plant.label, disagreements[0]
        );
    }
    // Under the pinned Lean, the production export certifies both roots;
    // its packages are the committed ones and realize the programs the
    // differential ran.
    if !support::lean_backed("BS-05") {
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
        let low = if member == "validateCrate" {
            &validate_root
        } else {
            &correspond_root
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
    eprintln!("BS-05: passed in {:?}", started.elapsed());
}
