//! Lowering of a valid target program to the closed Rust AST of a profile
//! (SPEC.md §17.16).
//!
//! Every construct lowering emits carries the [`Origin`] it realizes: the
//! calculus element of the term, shape, or literal it renders, derived from
//! that term and never from the construct chosen for it, so a construct that
//! does not realize its own term is refused by `validate::correspond`.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use super::super::check::Checker;
use super::super::realization;
use super::super::{Arm, Expr as Term, OrderingValue, Prim, Program, Shape, Ty, Value};
use super::ast::{
    self, Block, Callee, CaptureRead, Crate, Ctor, Dispatch, Expr, Ident, ItemDef, Let, Lit,
    Origin, Pat, Type,
};
use super::runtime::Item;
use super::Profile;

/// A type that can contain itself through in-place storage: an ADT, or a
/// function type through its closures' captures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Owner {
    Adt(u64),
    Fn(usize),
}

fn fail<T>(reason: impl Into<String>) -> Result<T, String> {
    Err(reason.into())
}

pub(super) fn index(number: u64) -> Result<usize, String> {
    usize::try_from(number).map_err(|error| error.to_string())
}

/// The realization element (§17.14) of a calculus syntax node of `class`.
fn element<T: Serialize>(class: &str, node: &T) -> String {
    format!("{class}:{}", realization::kind(node))
}

/// The origin of a construct realizing term `term`.
fn term_at(term: &Term) -> Origin {
    Origin::of(&element("expr", term))
}

/// The origin of a construct realizing shape `shape`.
fn shape_at(shape: &Shape) -> Origin {
    Origin::of(&element("shape", shape))
}

/// The origin of a literal `value` of type `ty`, with its width.
fn value_at(value: &Value, ty: &Ty) -> Origin {
    let width = match ty {
        Ty::Fixed { width } => Some(*width),
        _ => None,
    };
    Origin::at_width(&element("value", value), width)
}

/// The origin of primitive `operation` applied to operands of `types`: the
/// primitive's element and, for a primitive indexed by a width, the width
/// it works at: its operand width, or the target of a conversion or a
/// fixed-width parse. It is derived from the term alone, independently of
/// [`item`], which chooses the runtime function.
fn prim_at(operation: &Prim, types: &[Ty]) -> Origin {
    let operand = || match types.first() {
        Some(Ty::Fixed { width }) => Some(*width),
        _ => None,
    };
    let width = match operation {
        Prim::Convert { target } => Some(*target),
        Prim::ParseDecimal {
            target: Ty::Fixed { width },
        } => Some(*width),
        Prim::CheckedAdd
        | Prim::CheckedSub
        | Prim::CheckedMul
        | Prim::CheckedQuot
        | Prim::CheckedNeg
        | Prim::BitAnd
        | Prim::BitOr
        | Prim::BitXor
        | Prim::BitNot
        | Prim::ShiftLeft
        | Prim::ShiftRight
        | Prim::FormatDecimal => operand(),
        _ => None,
    };
    Origin::at_width(&element("prim", operation), width)
}

/// Whether local `name` occurs free in `expr`.
pub(super) fn mentions(expr: &Term, name: u64) -> bool {
    let any = |exprs: &[Term]| exprs.iter().any(|expr| mentions(expr, name));
    match expr {
        Term::Value { .. } => false,
        Term::Var { name: used } => *used == name,
        Term::Let {
            name: bound,
            bound: value,
            body,
            ..
        } => mentions(value, name) || (*bound != name && mentions(body, name)),
        Term::Cond {
            condition,
            then_branch,
            else_branch,
        } => {
            mentions(condition, name) || mentions(then_branch, name) || mentions(else_branch, name)
        }
        Term::Match {
            scrutinee, arms, ..
        } => {
            mentions(scrutinee, name)
                || arms
                    .iter()
                    .any(|arm| !arm.binders.contains(&name) && mentions(&arm.body, name))
        }
        Term::Build { operands, .. }
        | Term::Call { operands, .. }
        | Term::Prim { operands, .. } => any(operands),
        Term::Closure { captures, .. } => any(captures),
        Term::Apply { target, operands } => mentions(target, name) || any(operands),
        Term::First { value } | Term::Second { value } | Term::Field { value, .. } => {
            mentions(value, name)
        }
    }
}

/// The pattern binding local `name` of type `ty` in `body`: `_` when the
/// body never reads it, or when it is a unit, whose every read is the
/// literal `()`.
fn binder(name: u64, ty: &Ty, body: &Term) -> Pat {
    if *ty != Ty::Unit && mentions(body, name) {
        Pat::Bind(Ident::Local(name))
    } else {
        Pat::Wild
    }
}

/// An expression as a block, without a redundant pair of braces.
fn block_of(expr: Expr) -> Block {
    match expr {
        Expr::Block(block) => *block,
        other => Block::of(other),
    }
}

/// `expr` after `lets`, as one expression.
fn after(lets: Vec<Let>, expr: Expr) -> Expr {
    if lets.is_empty() {
        expr
    } else {
        let mut block = block_of(expr);
        let mut all = lets;
        all.append(&mut block.lets);
        block.lets = all;
        Expr::Block(Box::new(block))
    }
}

/// Make `expr` the value of `block`: a block's bindings join the block's,
/// whose names are unique in their function, so no scope changes meaning.
fn set_tail(block: &mut Block, expr: Expr) {
    match expr {
        Expr::Block(inner) => {
            let inner = *inner;
            block.lets.extend(inner.lets);
            block.tail = inner.tail;
        }
        other => block.tail = other,
    }
}

/// Fold every block that binds a value and then returns exactly that binding
/// into the value itself: Rust's lint gate refuses the binding, and the two
/// evaluate the same. Names are unique in their function, so a block the
/// value is joins the enclosing one without changing any name's meaning.
fn fold(block: &mut Block) {
    for binding in &mut block.lets {
        fold_expr(&mut binding.value);
    }
    fold_expr(&mut block.tail);
    loop {
        let returned = match (&block.tail, block.lets.last()) {
            (
                Expr::Move(read, _) | Expr::Copy(read, _),
                Some(Let {
                    pat: Pat::Bind(bound),
                    ty: None,
                    ..
                }),
            ) => read == bound,
            _ => false,
        };
        if !returned {
            return;
        }
        if let Some(binding) = block.lets.pop() {
            set_tail(block, binding.value);
        }
    }
}

fn fold_expr(expr: &mut Expr) {
    match expr {
        Expr::Block(block) => fold(block),
        Expr::Not(inner, _)
        | Expr::Box(inner, _)
        | Expr::Widen(inner, _)
        | Expr::Succeed(inner, _) => {
            fold_expr(inner);
        }
        Expr::Call { args, .. } | Expr::Apply { args, .. } | Expr::Construct { args, .. } => {
            args.iter_mut().for_each(fold_expr);
        }
        Expr::Pair(left, right, _) => {
            fold_expr(left);
            fold_expr(right);
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            fold_expr(condition);
            fold(then_branch);
            fold(else_branch);
        }
        Expr::Match {
            scrutinee, arms, ..
        } => {
            fold_expr(scrutinee);
            arms.iter_mut().for_each(|(_, body)| fold(body));
        }
        Expr::Lit(..)
        | Expr::Move(..)
        | Expr::Clone(..)
        | Expr::Copy(..)
        | Expr::Deref(..)
        | Expr::Unbox(..)
        | Expr::Uncons(_)
        | Expr::IsZero(..)
        | Expr::NonZero(..)
        | Expr::Predecessor(_) => {}
    }
}

/// The value a fallible function returns from `expr`: `Ok` is pushed into
/// every tail, and a fallible call or application in tail position returns
/// its result instead of propagating and rewrapping it. A computed unit is
/// bound before `Ok(())`, since Rust's lint gate refuses it as an argument.
fn succeed(expr: Expr, unit: bool) -> Expr {
    match expr {
        Expr::Block(mut block) => {
            succeed_tail(&mut block, unit);
            Expr::Block(block)
        }
        Expr::If {
            condition,
            mut then_branch,
            mut else_branch,
            at,
        } => {
            succeed_tail(&mut then_branch, unit);
            succeed_tail(&mut else_branch, unit);
            Expr::If {
                condition,
                then_branch,
                else_branch,
                at,
            }
        }
        Expr::Match {
            scrutinee,
            arms,
            at,
        } => Expr::Match {
            scrutinee,
            arms: arms
                .into_iter()
                .map(|(pattern, mut body)| {
                    succeed_tail(&mut body, unit);
                    (pattern, body)
                })
                .collect(),
            at,
        },
        Expr::Call {
            callee,
            args,
            propagate: true,
            at,
        } => Expr::Call {
            callee,
            args,
            propagate: false,
            at,
        },
        Expr::Apply {
            holder,
            args,
            propagate: true,
            at,
        } => Expr::Apply {
            holder,
            args,
            propagate: false,
            at,
        },
        Expr::Lit(Lit::Unit, at) => {
            Expr::Succeed(Box::new(Expr::Lit(Lit::Unit, at)), Origin::of("overflow"))
        }
        other if unit => {
            let at = Origin::of("overflow");
            Expr::Block(Box::new(Block {
                lets: vec![Let {
                    pat: Pat::Unit,
                    ty: None,
                    value: other,
                    at: at.clone(),
                }],
                tail: Expr::Succeed(Box::new(Expr::Lit(Lit::Unit, at.clone())), at),
            }))
        }
        other => Expr::Succeed(Box::new(other), Origin::of("overflow")),
    }
}

/// [`succeed`] on the value of `block`.
fn succeed_tail(block: &mut Block, unit: bool) {
    let tail = std::mem::replace(
        &mut block.tail,
        Expr::Lit(Lit::Unit, Origin::of("overflow")),
    );
    set_tail(block, succeed(tail, unit));
}

/// `expr` negated: a zero test becomes the nonzero test, a negation its
/// operand, and a Boolean literal the other one, so no negation of a
/// comparison, a negation, or a constant is written.
fn negate(expr: Expr, at: &Origin) -> Expr {
    match expr {
        Expr::Lit(Lit::Bool(value), _) => Expr::Lit(Lit::Bool(!value), at.clone()),
        Expr::IsZero(ident, origin) => Expr::NonZero(ident, origin),
        Expr::NonZero(ident, origin) => Expr::IsZero(ident, origin),
        Expr::Not(inner, _) => *inner,
        other => Expr::Not(Box::new(other), at.clone()),
    }
}

/// Renames every name a block binds to a canonical name, in binding order,
/// so two blocks that differ only in the names they bind print alike.
struct Renamer {
    names: BTreeMap<Ident, Ident>,
    next: u64,
}

impl Renamer {
    fn canonical(&mut self, ident: &Ident) -> Ident {
        // Far above any fresh number, so no canonical name meets a free one.
        let n = (1 << 40) + self.next;
        self.next += 1;
        match ident {
            Ident::Local(_) => Ident::Local(n),
            Ident::Holder(_) => Ident::Holder(n),
            Ident::Operand(_) => Ident::Operand(n),
            Ident::Callee(_) => Ident::Callee(n),
            Ident::Boxed(_) => Ident::Boxed(n),
            Ident::Part(_) => Ident::Part(n),
            Ident::Capture(_) => Ident::Capture(n),
            Ident::Param(_) => Ident::Param(n),
            Ident::Function(_) => Ident::Function(n),
            Ident::Export(name) => Ident::Export(name.clone()),
        }
    }

    fn bind(&mut self, pattern: &mut Pat) {
        match pattern {
            Pat::Bind(ident) => {
                let renamed = self.canonical(ident);
                self.names.insert(ident.clone(), renamed.clone());
                *ident = renamed;
            }
            Pat::Tuple(items) | Pat::Adt { fields: items, .. } => {
                items.iter_mut().for_each(|item| self.bind(item));
            }
            Pat::Some(inner) | Pat::Ok(inner) | Pat::Err(inner) => self.bind(inner),
            Pat::Wild | Pat::Unit | Pat::None | Pat::Ordering(_) => {}
        }
    }

    fn read(&self, ident: &mut Ident) {
        if let Some(renamed) = self.names.get(ident) {
            *ident = renamed.clone();
        }
    }

    fn block(&mut self, block: &mut Block) {
        for binding in &mut block.lets {
            self.expr(&mut binding.value);
            self.bind(&mut binding.pat);
        }
        self.expr(&mut block.tail);
    }

    fn expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Move(ident, _)
            | Expr::Clone(ident, _)
            | Expr::Copy(ident, _)
            | Expr::Deref(ident, _)
            | Expr::Unbox(ident, _)
            | Expr::Uncons(ident)
            | Expr::IsZero(ident, _)
            | Expr::NonZero(ident, _)
            | Expr::Predecessor(ident) => self.read(ident),
            Expr::Not(inner, _)
            | Expr::Box(inner, _)
            | Expr::Widen(inner, _)
            | Expr::Succeed(inner, _) => self.expr(inner),
            Expr::Call { args, .. } | Expr::Construct { args, .. } => {
                args.iter_mut().for_each(|arg| self.expr(arg));
            }
            Expr::Apply { holder, args, .. } => {
                self.read(holder);
                args.iter_mut().for_each(|arg| self.expr(arg));
            }
            Expr::Pair(left, right, _) => {
                self.expr(left);
                self.expr(right);
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.expr(condition);
                self.block(then_branch);
                self.block(else_branch);
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                self.expr(scrutinee);
                for (pattern, body) in arms {
                    self.bind(pattern);
                    self.block(body);
                }
            }
            Expr::Block(block) => self.block(block),
            Expr::Lit(..) => {}
        }
    }
}

/// Whether two blocks are the same code up to the names they bind. Such
/// blocks compute the same value, and Rust's lint gate refuses a branch
/// between them, so the rendering never states one.
fn same_code(left: &Block, right: &Block) -> bool {
    let canonical = |block: &Block| {
        let mut block = block.clone();
        Renamer {
            names: BTreeMap::new(),
            next: 0,
        }
        .block(&mut block);
        ast::print_block(&block)
    };
    canonical(left) == canonical(right)
}

/// Whether a match arm returns exactly the value it matched: the same
/// variant rebuilt from the same bindings, read without a copy of their
/// own.
fn rebuilds(pattern: &Pat, body: &Block) -> bool {
    let same = |pattern: &Pat, arg: &Expr| match (pattern, arg) {
        (Pat::Bind(bound), Expr::Copy(read, _) | Expr::Move(read, _)) => bound == read,
        (Pat::Wild, Expr::Lit(Lit::Unit, _)) => true,
        _ => false,
    };
    if !body.lets.is_empty() {
        return false;
    }
    match (pattern, &body.tail) {
        (
            Pat::None,
            Expr::Construct {
                ctor: Ctor::None(_),
                ..
            },
        ) => true,
        (
            Pat::Some(inner),
            Expr::Construct {
                ctor: Ctor::Some,
                args,
                ..
            },
        )
        | (
            Pat::Ok(inner),
            Expr::Construct {
                ctor: Ctor::Ok(..),
                args,
                ..
            },
        )
        | (
            Pat::Err(inner),
            Expr::Construct {
                ctor: Ctor::Err(..),
                args,
                ..
            },
        ) => args.len() == 1 && same(inner, &args[0]),
        (Pat::Ordering(matched), Expr::Lit(Lit::Ordering(built), _)) => matched == built,
        (
            Pat::Adt {
                adt,
                constructor,
                fields,
            },
            Expr::Construct {
                ctor:
                    Ctor::Adt {
                        adt: built_adt,
                        constructor: built,
                    },
                args,
                ..
            },
        ) => {
            adt == built_adt
                && constructor == built
                && fields.len() == args.len()
                && fields.iter().zip(args).all(|(field, arg)| same(field, arg))
        }
        _ => false,
    }
}

/// Whether a block does nothing and is the unit.
fn empty(block: &Block) -> bool {
    block.lets.is_empty() && matches!(block.tail, Expr::Lit(Lit::Unit, _))
}

/// The Boolean literal a block is, if it is one.
fn literal(block: &Block) -> Option<bool> {
    match (&block.lets[..], &block.tail) {
        ([], Expr::Lit(Lit::Bool(value), _)) => Some(*value),
        _ => None,
    }
}

/// The runtime function realizing `operation` at its operand types.
fn item(operation: &Prim, operands: &[Ty]) -> Result<Item, String> {
    let first = operands.first().cloned().unwrap_or(Ty::Unit);
    let fixed = || match &first {
        Ty::Fixed { width } => Ok(*width),
        other => fail(format!("{other:?} is not fixed-width")),
    };
    let sequence = |list: Item, bytes: Item, string: Option<Item>| match (&first, string) {
        (Ty::List { .. }, _) => Ok(list),
        (Ty::Bytes, _) => Ok(bytes),
        (Ty::String, Some(string)) => Ok(string),
        (other, _) => fail(format!("{operation:?} does not apply to {other:?}")),
    };
    Ok(match operation {
        Prim::NatAdd => Item::NatAdd,
        Prim::NatSub => Item::NatSub,
        Prim::NatMul => Item::NatMul,
        Prim::NatQuot => Item::NatQuot,
        Prim::NatRem => Item::NatRem,
        Prim::NatEq => Item::NatEq,
        Prim::NatLe => Item::NatLe,
        Prim::NatLt => Item::NatLt,
        Prim::IntAdd => Item::IntAdd,
        Prim::IntSub => Item::IntSub,
        Prim::IntMul => Item::IntMul,
        Prim::IntNeg => Item::IntNeg,
        Prim::IntQuot => Item::IntQuot,
        Prim::IntRem => Item::IntRem,
        Prim::CheckedAdd => Item::CheckedAdd(fixed()?),
        Prim::CheckedSub => Item::CheckedSub(fixed()?),
        Prim::CheckedMul => Item::CheckedMul(fixed()?),
        Prim::CheckedQuot => Item::CheckedQuot(fixed()?),
        Prim::CheckedNeg => Item::CheckedNeg(fixed()?),
        Prim::BitAnd => Item::BitAnd(fixed()?),
        Prim::BitOr => Item::BitOr(fixed()?),
        Prim::BitXor => Item::BitXor(fixed()?),
        Prim::BitNot => Item::BitNot(fixed()?),
        Prim::ShiftLeft => Item::ShiftLeft(fixed()?),
        Prim::ShiftRight => Item::ShiftRight(fixed()?),
        Prim::Equal => Item::Equal,
        Prim::BoolNot => Item::BoolNot,
        Prim::BoolAnd => Item::BoolAnd,
        Prim::BoolOr => Item::BoolOr,
        Prim::Append => sequence(Item::AppendList, Item::AppendBytes, None)?,
        Prim::Length => sequence(
            Item::LengthList,
            Item::LengthBytes,
            Some(Item::LengthString),
        )?,
        Prim::Index => sequence(Item::IndexList, Item::IndexBytes, None)?,
        Prim::Slice => sequence(Item::SliceList, Item::SliceBytes, None)?,
        Prim::Utf8Encode => Item::Utf8Encode,
        Prim::Utf8Decode => Item::Utf8Decode,
        Prim::CompareBytes => Item::CompareBytes,
        Prim::Compare => Item::Compare,
        Prim::SplitExact => Item::SplitExact,
        Prim::Join => Item::Join,
        Prim::FormatDecimal => match first {
            Ty::Int => Item::FormatInt,
            _ => Item::FormatFixed(fixed()?),
        },
        Prim::Convert { target } => Item::Convert(*target),
        Prim::ParseDecimal { target } => match target {
            Ty::Int => Item::ParseInt,
            Ty::Fixed { width } => Item::ParseFixed(*width),
            other => return fail(format!("parse_decimal does not target {other:?}")),
        },
    })
}

/// What one function's body can do that makes it fail: a failing primitive
/// or successor of its own, a call, or an application of a function type.
#[derive(Default)]
struct Reaches {
    own: bool,
    calls: BTreeSet<usize>,
    applies: BTreeSet<usize>,
}

/// The lowering context: the program, its profile, its function types, and
/// fresh names.
pub(super) struct Lowering<'a> {
    pub(super) program: &'a Program,
    pub(super) profile: Profile,
    checker: Checker<'a>,
    /// Every function type of the program, numbered in first-use order.
    pub(super) fn_types: Vec<Ty>,
    /// For each function type, the closures of live functions that inhabit
    /// it: function index and captured types.
    pub(super) closures: BTreeMap<usize, Vec<(u64, Vec<Ty>)>>,
    /// The function types a closure term of the program has, live or not.
    closure_types: BTreeSet<usize>,
    /// For each function type, the closures that inhabit it in any function,
    /// live or not: what a value of it can store, as the realization
    /// (§17.14) counts storage.
    stored: BTreeMap<usize, Vec<(u64, Vec<Ty>)>>,
    /// For each owner, the owners its in-place storage reaches.
    reaches: BTreeMap<Owner, BTreeSet<Owner>>,
    /// The owners some Rust value of the rendered type inhabits, as Rust's
    /// exhaustiveness sees them: a box always holds a value.
    inhabited: BTreeSet<Owner>,
    /// The functions one of whose parameters has a type no value inhabits:
    /// no call of one is ever reached, and its body is an empty match.
    pub(super) dead: Vec<bool>,
    /// Whether each function can overflow.
    pub(super) fallible: Vec<bool>,
    /// Whether each function type's application can overflow.
    pub(super) apply_fallible: BTreeMap<usize, bool>,
    fresh: u64,
}

impl<'a> Lowering<'a> {
    /// Analyse a valid program: its function types, closures, boxes,
    /// inhabitation, and failures.
    pub(super) fn new(program: &'a Program, profile: Profile) -> Result<Self, String> {
        super::super::check::check(program)?;
        let mut lowering = Lowering {
            program,
            profile,
            checker: Checker { program },
            fn_types: Vec::new(),
            closures: BTreeMap::new(),
            closure_types: BTreeSet::new(),
            stored: BTreeMap::new(),
            reaches: BTreeMap::new(),
            inhabited: BTreeSet::new(),
            dead: vec![false; program.functions.len()],
            fallible: vec![false; program.functions.len()],
            apply_fallible: BTreeMap::new(),
            fresh: 0,
        };
        for function in &program.functions {
            for ty in function.types.iter().chain([&function.result]) {
                lowering.register(ty);
            }
            lowering.collect(&function.body, &mut Self::scope(function))?;
        }
        for adt in &program.adts {
            for ty in adt.constructors.iter().flatten() {
                lowering.register(ty);
            }
        }
        lowering.stored = lowering.closures.clone();
        lowering.reaches = lowering.reachability();
        // A function with an uninhabited parameter is never called, so its
        // closures inhabit nothing, which can leave more types uninhabited:
        // the dead functions only grow, so this ends.
        loop {
            lowering.inhabited = lowering.inhabitation();
            let dead: Vec<bool> = program
                .functions
                .iter()
                .map(|function| function.types.iter().any(|ty| !lowering.inhabits(ty)))
                .collect();
            if dead == lowering.dead {
                break;
            }
            lowering.dead = dead;
            lowering.closures.clear();
            for (position, function) in program.functions.iter().enumerate() {
                if !lowering.dead[position] {
                    lowering.collect(&function.body, &mut Self::scope(function))?;
                }
            }
        }
        lowering.failures()?;
        Ok(lowering)
    }

    fn scope(function: &super::super::Function) -> Vec<(u64, Ty)> {
        function
            .parameters
            .iter()
            .copied()
            .zip(function.types.iter().cloned())
            .collect()
    }

    fn fn_index(&mut self, ty: &Ty) -> usize {
        if let Some(position) = self.fn_types.iter().position(|known| known == ty) {
            return position;
        }
        self.fn_types.push(ty.clone());
        self.fn_types.len() - 1
    }

    /// Number every function type `ty` mentions.
    fn register(&mut self, ty: &Ty) {
        match ty {
            Ty::Option { value } => self.register(value),
            Ty::Result { ok, error } => {
                self.register(ok);
                self.register(error);
            }
            Ty::List { element } => self.register(element),
            Ty::Pair { left, right } => {
                self.register(left);
                self.register(right);
            }
            Ty::Fn { parameters, result } => {
                let _ = self.fn_index(ty);
                for parameter in parameters {
                    self.register(parameter);
                }
                self.register(result);
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

    /// The owners whose storage a value of `ty` holds in place: a list's
    /// cells are behind a handle, so its element is not in place.
    fn in_place(&self, ty: &Ty, out: &mut BTreeSet<Owner>) {
        match ty {
            Ty::Option { value } => self.in_place(value, out),
            Ty::Result { ok, error } => {
                self.in_place(ok, out);
                self.in_place(error, out);
            }
            Ty::Pair { left, right } => {
                self.in_place(left, out);
                self.in_place(right, out);
            }
            Ty::Adt { index } => {
                out.insert(Owner::Adt(*index));
            }
            Ty::Fn { .. } => {
                if let Some(position) = self.fn_types.iter().position(|known| known == ty) {
                    out.insert(Owner::Fn(position));
                }
            }
            Ty::Unit
            | Ty::Bool
            | Ty::Nat
            | Ty::Int
            | Ty::Fixed { .. }
            | Ty::String
            | Ty::Bytes
            | Ty::Ordering
            | Ty::List { .. } => {}
        }
    }

    /// The variants of an owner, each as its field types: of a function
    /// type, its live closures, or with `stored` every closure of it.
    fn variants(&self, owner: Owner, stored: bool) -> Vec<Vec<Ty>> {
        let closures = if stored { &self.stored } else { &self.closures };
        match owner {
            Owner::Adt(adt) => usize::try_from(adt)
                .ok()
                .and_then(|adt| self.program.adts.get(adt))
                .map(|adt| adt.constructors.clone())
                .unwrap_or_default(),
            Owner::Fn(position) => closures
                .get(&position)
                .map(|variants| {
                    variants
                        .iter()
                        .map(|(_, captured)| captured.clone())
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    fn owners(&self) -> Vec<Owner> {
        (0..self.program.adts.len() as u64)
            .map(Owner::Adt)
            .chain((0..self.fn_types.len()).map(Owner::Fn))
            .collect()
    }

    /// For every owner, every owner its in-place storage reaches.
    fn reachability(&self) -> BTreeMap<Owner, BTreeSet<Owner>> {
        let owners = self.owners();
        let direct: BTreeMap<Owner, BTreeSet<Owner>> = owners
            .iter()
            .map(|owner| {
                let mut out = BTreeSet::new();
                for ty in self.variants(*owner, true).iter().flatten() {
                    self.in_place(ty, &mut out);
                }
                (*owner, out)
            })
            .collect();
        owners
            .iter()
            .map(|owner| {
                let mut seen = BTreeSet::new();
                let mut pending: Vec<Owner> = direct
                    .get(owner)
                    .map(|reached| reached.iter().copied().collect())
                    .unwrap_or_default();
                while let Some(next) = pending.pop() {
                    if seen.insert(next) {
                        if let Some(onward) = direct.get(&next) {
                            pending.extend(onward.iter().copied());
                        }
                    }
                }
                (*owner, seen)
            })
            .collect()
    }

    /// The owners a rendered value inhabits: the least set such that an
    /// owner is in it when one of its variants has only fields that are
    /// boxed or of an inhabited type.
    fn inhabitation(&self) -> BTreeSet<Owner> {
        let owners = self.owners();
        let mut inhabited = BTreeSet::new();
        loop {
            let mut changed = false;
            for owner in &owners {
                if inhabited.contains(owner) {
                    continue;
                }
                if self.variants(*owner, false).iter().any(|fields| {
                    fields
                        .iter()
                        .all(|ty| self.cyclic(*owner, ty) || self.visible(ty, &inhabited))
                }) {
                    inhabited.insert(*owner);
                    changed = true;
                }
            }
            if !changed {
                return inhabited;
            }
        }
    }

    /// Whether some value of the rendering of `ty` exists, given the
    /// inhabited owners.
    fn visible(&self, ty: &Ty, inhabited: &BTreeSet<Owner>) -> bool {
        match ty {
            Ty::Pair { left, right } => {
                self.visible(left, inhabited) && self.visible(right, inhabited)
            }
            Ty::Result { ok, error } => {
                self.visible(ok, inhabited) || self.visible(error, inhabited)
            }
            Ty::Adt { index } => inhabited.contains(&Owner::Adt(*index)),
            Ty::Fn { .. } => self
                .fn_types
                .iter()
                .position(|known| known == ty)
                .is_some_and(|position| inhabited.contains(&Owner::Fn(position))),
            Ty::Unit
            | Ty::Bool
            | Ty::Nat
            | Ty::Int
            | Ty::Fixed { .. }
            | Ty::String
            | Ty::Bytes
            | Ty::Ordering
            | Ty::Option { .. }
            | Ty::List { .. } => true,
        }
    }

    /// Whether some value of type `ty` exists in the rendering.
    pub(super) fn inhabits(&self, ty: &Ty) -> bool {
        self.visible(ty, &self.inhabited)
    }

    /// Refuse a live computation of a value no type inhabits: Rust would
    /// make everything after it unreachable code.
    fn inhabited(&self, ty: &Ty) -> Result<(), String> {
        if self.inhabits(ty) {
            Ok(())
        } else {
            fail(format!(
                "unsupported type: the program computes a value of {ty:?}, which no value inhabits, so its rendering would be unreachable code"
            ))
        }
    }

    /// Whether any type of the program holds itself, so some field is boxed.
    pub(super) fn indirect(&self) -> bool {
        self.reaches
            .iter()
            .any(|(owner, reached)| reached.contains(owner))
    }

    /// Whether a field of type `ty` stored in `owner` reaches `owner` again
    /// through in-place storage.
    fn cyclic(&self, owner: Owner, ty: &Ty) -> bool {
        let mut held = BTreeSet::new();
        self.in_place(ty, &mut held);
        held.iter().any(|reached| {
            *reached == owner
                || self
                    .reaches
                    .get(reached)
                    .is_some_and(|onward| onward.contains(&owner))
        })
    }

    /// Whether a field of type `ty` stored in `owner` must be boxed: its
    /// in-place storage reaches `owner` again, so the type has no fixed
    /// size. `rust-core` cannot box, so it refuses the program.
    pub(super) fn boxed(&self, owner: Owner, ty: &Ty) -> Result<bool, String> {
        let cyclic = self.cyclic(owner, ty);
        if cyclic && self.profile == Profile::Core {
            let named = match owner {
                Owner::Adt(adt) => format!("ADT {adt}"),
                Owner::Fn(position) => format!("function type {position}"),
            };
            return fail(format!(
                "{named} contains itself and requires heap allocation, which {} does not provide",
                self.profile.target()
            ));
        }
        Ok(cyclic)
    }

    /// Which functions and function types can overflow: a function can when
    /// its body reaches a failing primitive, a successor, a call to a
    /// function that can, or an application of a function type one of whose
    /// closures can. The least fixed point, by a worklist over the reverse
    /// edges, so each edge is followed once.
    fn failures(&mut self) -> Result<(), String> {
        let count = self.program.functions.len();
        let mut reached = Vec::with_capacity(count);
        for function in &self.program.functions {
            let mut found = Reaches::default();
            self.reach(&function.body, &mut Self::scope(function), &mut found)?;
            reached.push(found);
        }
        let mut callers: Vec<Vec<usize>> = vec![Vec::new(); count];
        let mut appliers: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for (caller, found) in reached.iter().enumerate() {
            for callee in &found.calls {
                if let Some(list) = callers.get_mut(*callee) {
                    list.push(caller);
                }
            }
            for fn_type in &found.applies {
                appliers.entry(*fn_type).or_default().push(caller);
            }
        }
        let mut inhabits: Vec<Vec<usize>> = vec![Vec::new(); count];
        for (fn_type, variants) in &self.closures {
            for (function, _) in variants {
                if let Some(list) = inhabits.get_mut(index(*function)?) {
                    list.push(*fn_type);
                }
            }
        }
        for position in 0..self.fn_types.len() {
            self.apply_fallible.insert(position, false);
        }
        let mut pending: Vec<usize> = (0..count).filter(|at| reached[*at].own).collect();
        for at in &pending {
            self.fallible[*at] = true;
        }
        while let Some(function) = pending.pop() {
            let mut newly = callers[function].clone();
            for fn_type in &inhabits[function] {
                if self.apply_fallible.insert(*fn_type, true) != Some(true) {
                    newly.extend(appliers.get(fn_type).into_iter().flatten().copied());
                }
            }
            for next in newly {
                if !self.fallible[next] {
                    self.fallible[next] = true;
                    pending.push(next);
                }
            }
        }
        Ok(())
    }

    /// Record what `expr` can do that fails.
    fn reach(
        &mut self,
        expr: &Term,
        scope: &mut Vec<(u64, Ty)>,
        found: &mut Reaches,
    ) -> Result<(), String> {
        let mut children: Vec<&Term> = Vec::new();
        match expr {
            Term::Value { .. } | Term::Var { .. } => {}
            Term::Let {
                name,
                ty,
                bound,
                body,
            } => {
                self.reach(bound, scope, found)?;
                scope.push((*name, ty.clone()));
                let body = self.reach(body, scope, found);
                scope.pop();
                return body;
            }
            Term::Match {
                scrutinee, arms, ..
            } => {
                self.reach(scrutinee, scope, found)?;
                let scrutinee_ty = self.checker.expr(scrutinee, scope)?;
                for arm in arms {
                    let fields = self.checker.shape_fields(arm.shape, &scrutinee_ty)?;
                    let depth = scope.len();
                    scope.extend(arm.binders.iter().copied().zip(fields));
                    let reached = self.reach(&arm.body, scope, found);
                    scope.truncate(depth);
                    reached?;
                }
                return Ok(());
            }
            Term::Cond {
                condition,
                then_branch,
                else_branch,
            } => {
                children.extend([
                    condition.as_ref(),
                    then_branch.as_ref(),
                    else_branch.as_ref(),
                ]);
            }
            Term::Build {
                shape, operands, ..
            } => {
                children.extend(operands);
                found.own |= *shape == Shape::Succ;
            }
            Term::Call { function, operands } => {
                children.extend(operands);
                found.calls.insert(index(*function)?);
            }
            Term::Closure { captures, .. } => children.extend(captures),
            Term::Apply { target, operands } => {
                children.push(target);
                children.extend(operands);
                let ty = self.checker.expr(target, scope)?;
                let position = self.fn_index(&ty);
                found.applies.insert(position);
            }
            Term::Prim {
                operation,
                operands,
            } => {
                children.extend(operands);
                let types = operands
                    .iter()
                    .map(|operand| self.checker.expr(operand, scope))
                    .collect::<Result<Vec<_>, _>>()?;
                found.own |= item(operation, &types)?.fallible();
            }
            Term::First { value } | Term::Second { value } | Term::Field { value, .. } => {
                children.push(value);
            }
        }
        for child in children {
            self.reach(child, scope, found)?;
        }
        Ok(())
    }

    fn fresh(&mut self) -> u64 {
        self.fresh += 1;
        self.fresh
    }

    fn heap(&self, what: &str) -> Result<(), String> {
        if self.profile == Profile::Core {
            return fail(format!(
                "{what} requires heap allocation, which {} does not provide",
                self.profile.target()
            ));
        }
        Ok(())
    }

    pub(super) fn ty(&mut self, ty: &Ty) -> Result<Type, String> {
        Ok(match ty {
            Ty::Unit => Type::Unit,
            Ty::Bool => Type::Bool,
            Ty::Nat => Type::Nat,
            Ty::Int => Type::Int,
            Ty::Fixed { width } => Type::Fixed(*width),
            Ty::String => {
                self.heap("a string")?;
                Type::Str
            }
            Ty::Bytes => {
                self.heap("a byte string")?;
                Type::Bytes
            }
            Ty::Ordering => Type::Ordering,
            Ty::Option { value } => Type::Option(Box::new(self.ty(value)?)),
            Ty::Result { ok, error } => {
                Type::Result(Box::new(self.ty(ok)?), Box::new(self.ty(error)?))
            }
            Ty::List { element } => {
                self.heap("a list")?;
                Type::List(Box::new(self.ty(element)?))
            }
            Ty::Pair { left, right } => {
                Type::Pair(Box::new(self.ty(left)?), Box::new(self.ty(right)?))
            }
            Ty::Adt { index } => Type::Adt(*index),
            Ty::Fn { .. } => Type::Fn(self.fn_index(ty) as u64),
        })
    }

    /// A field of `owner`: its type, boxed where it must be.
    fn field_ty(&mut self, owner: Owner, ty: &Ty) -> Result<Type, String> {
        let lowered = self.ty(ty)?;
        Ok(if self.boxed(owner, ty)? {
            Type::Rc(Box::new(lowered))
        } else {
            lowered
        })
    }

    /// A value stored as a field of `owner`.
    fn store(&self, owner: Owner, ty: &Ty, value: Expr) -> Result<Expr, String> {
        Ok(if self.boxed(owner, ty)? {
            Expr::Box(Box::new(value), Origin::of("indirection"))
        } else {
            value
        })
    }

    fn adt_fields(&self, adt: u64, constructor: u64) -> Result<Vec<Ty>, String> {
        self.program
            .adts
            .get(index(adt)?)
            .and_then(|declared| declared.constructors.get(index(constructor).ok()?))
            .cloned()
            .ok_or_else(|| format!("ADT {adt} has no constructor {constructor}"))
    }

    /// Collect every closure site, so each function type's enum is closed,
    /// and number every function type the program states.
    fn collect(&mut self, expr: &Term, scope: &mut Vec<(u64, Ty)>) -> Result<(), String> {
        let mut children: Vec<&Term> = Vec::new();
        match expr {
            Term::Value { ty, .. } => self.register(ty),
            Term::Var { .. } => {}
            Term::Let {
                name,
                ty,
                bound,
                body,
            } => {
                self.register(ty);
                self.collect(bound, scope)?;
                scope.push((*name, ty.clone()));
                let collected = self.collect(body, scope);
                scope.pop();
                return collected;
            }
            Term::Cond {
                condition,
                then_branch,
                else_branch,
            } => {
                children.extend([
                    condition.as_ref(),
                    then_branch.as_ref(),
                    else_branch.as_ref(),
                ]);
            }
            Term::Match {
                ty,
                scrutinee,
                arms,
            } => {
                self.register(ty);
                self.collect(scrutinee, scope)?;
                let scrutinee_ty = self.checker.expr(scrutinee, scope)?;
                for arm in arms {
                    let fields = self.checker.shape_fields(arm.shape, &scrutinee_ty)?;
                    let depth = scope.len();
                    scope.extend(arm.binders.iter().copied().zip(fields));
                    let collected = self.collect(&arm.body, scope);
                    scope.truncate(depth);
                    collected?;
                }
                return Ok(());
            }
            Term::Build { ty, operands, .. } => {
                self.register(ty);
                children.extend(operands);
            }
            Term::Call { operands, .. } | Term::Prim { operands, .. } => children.extend(operands),
            Term::Closure { function, captures } => {
                children.extend(captures);
                let ty = self.checker.expr(expr, scope)?;
                self.register(&ty);
                let position = self.fn_index(&ty);
                self.closure_types.insert(position);
                let captured = self
                    .program
                    .functions
                    .get(index(*function)?)
                    .and_then(|callee| callee.types.get(..captures.len()))
                    .ok_or_else(|| format!("closure of function {function} is malformed"))?
                    .to_vec();
                let entry = self.closures.entry(position).or_default();
                if !entry.iter().any(|(known, _)| known == function) {
                    entry.push((*function, captured));
                }
            }
            Term::Apply { target, operands } => {
                children.push(target);
                children.extend(operands);
            }
            Term::First { value } | Term::Second { value } | Term::Field { value, .. } => {
                children.push(value);
            }
        }
        for child in children {
            self.collect(child, scope)?;
        }
        Ok(())
    }

    /// The Rust expression of a literal `value` of type `ty`.
    #[allow(clippy::too_many_lines)]
    pub(super) fn value(&mut self, value: &Value, ty: &Ty) -> Result<Expr, String> {
        let at = value_at(value, ty);
        let number = |text: &str| {
            text.parse::<i128>()
                .map_err(|error| format!("literal `{text}`: {error}"))
        };
        Ok(match (value, ty) {
            (Value::Unit, _) => Expr::Lit(Lit::Unit, at),
            (Value::Bool { value }, _) => Expr::Lit(Lit::Bool(*value), at),
            (Value::Nat { value }, _) => Expr::Lit(
                Lit::Nat(
                    value
                        .parse()
                        .map_err(|error| format!("literal `{value}`: {error}"))?,
                ),
                at,
            ),
            (Value::Int { value }, _) => Expr::Lit(
                Lit::Int(
                    value
                        .parse()
                        .map_err(|error| format!("literal `{value}`: {error}"))?,
                ),
                at,
            ),
            (
                Value::U8 { value }
                | Value::U16 { value }
                | Value::U32 { value }
                | Value::U64 { value }
                | Value::I8 { value }
                | Value::I16 { value }
                | Value::I32 { value }
                | Value::I64 { value },
                Ty::Fixed { width },
            ) => Expr::Lit(Lit::Fixed(*width, number(value)?), at),
            (Value::String { value }, _) => {
                self.heap("a string")?;
                Expr::Lit(Lit::Str(value.clone()), at)
            }
            (Value::Bytes { hex }, _) => {
                self.heap("a byte string")?;
                let octets = (0..hex.len())
                    .step_by(2)
                    .map(|position| {
                        hex.get(position..position + 2)
                            .and_then(|digits| u8::from_str_radix(digits, 16).ok())
                            .ok_or_else(|| format!("byte literal `{hex}` is malformed"))
                    })
                    .collect::<Result<_, _>>()?;
                Expr::Lit(Lit::Bytes(octets), at)
            }
            (Value::Ordering { value }, _) => Expr::Lit(Lit::Ordering(*value), at),
            (Value::None, Ty::Option { value: inner }) => Expr::Construct {
                ctor: Ctor::None(self.ty(inner)?),
                args: Vec::new(),
                at,
            },
            (Value::Some { value }, Ty::Option { value: inner }) => Expr::Construct {
                ctor: Ctor::Some,
                args: vec![self.value(value, inner)?],
                at,
            },
            (Value::Ok { value }, Ty::Result { ok, error }) => Expr::Construct {
                ctor: Ctor::Ok(self.ty(ok)?, self.ty(error)?),
                args: vec![self.value(value, ok)?],
                at,
            },
            (Value::Error { value }, Ty::Result { ok, error }) => Expr::Construct {
                ctor: Ctor::Err(self.ty(ok)?, self.ty(error)?),
                args: vec![self.value(value, error)?],
                at,
            },
            (Value::List { items }, Ty::List { element }) => {
                let mut out = Expr::Construct {
                    ctor: Ctor::Nil(self.ty(element)?),
                    args: Vec::new(),
                    at: at.clone(),
                };
                for item in items.iter().rev() {
                    out = Expr::Construct {
                        ctor: Ctor::Cons,
                        args: vec![self.value(item, element)?, out],
                        at: at.clone(),
                    };
                }
                out
            }
            (
                Value::Pair { left, right },
                Ty::Pair {
                    left: left_ty,
                    right: right_ty,
                },
            ) => Expr::Pair(
                Box::new(self.value(left, left_ty)?),
                Box::new(self.value(right, right_ty)?),
                at,
            ),
            (
                Value::Adt {
                    constructor,
                    fields,
                },
                Ty::Adt { index: adt },
            ) => {
                let types = self.adt_fields(*adt, *constructor)?;
                let mut args = Vec::new();
                for (field, ty) in fields.iter().zip(&types) {
                    let lowered = self.value(field, ty)?;
                    args.push(self.store(Owner::Adt(*adt), ty, lowered)?);
                }
                Expr::Construct {
                    ctor: Ctor::Adt {
                        adt: *adt,
                        constructor: *constructor,
                    },
                    args,
                    at,
                }
            }
            (value, ty) => {
                return fail(format!(
                    "the literal {value:?} cannot be rendered at type {ty:?}"
                ))
            }
        })
    }

    fn exprs(&mut self, exprs: &[Term], scope: &mut Vec<(u64, Ty)>) -> Result<Vec<Expr>, String> {
        exprs.iter().map(|expr| self.expr(expr, scope)).collect()
    }

    /// The operands of a call, an application, a closure, or a constructor,
    /// of `types`, realizing `at`. Rust's lint gate refuses a unit argument
    /// computed by anything but the literal `()`, so when one is, every
    /// operand is first bound in order, keeping the denotation's evaluation
    /// order, and the unit is passed as `()`.
    fn operands(
        &mut self,
        terms: &[Term],
        types: &[Ty],
        at: &Origin,
        scope: &mut Vec<(u64, Ty)>,
    ) -> Result<(Vec<Let>, Vec<Expr>), String> {
        let rendered = self.exprs(terms, scope)?;
        let computed_unit = rendered
            .iter()
            .zip(types)
            .any(|(value, ty)| *ty == Ty::Unit && !matches!(value, Expr::Lit(Lit::Unit, _)));
        if !computed_unit {
            return Ok((Vec::new(), rendered));
        }
        let mut lets = Vec::new();
        let mut args = Vec::new();
        for (value, ty) in rendered.into_iter().zip(types) {
            if *ty == Ty::Unit {
                lets.push(Let {
                    pat: Pat::Unit,
                    ty: None,
                    value,
                    at: at.clone(),
                });
                args.push(Expr::Lit(Lit::Unit, at.clone()));
            } else {
                let name = Ident::Operand(self.fresh());
                lets.push(Let {
                    pat: Pat::Bind(name.clone()),
                    ty: None,
                    value,
                    at: at.clone(),
                });
                args.push(Expr::Move(name, at.clone()));
            }
        }
        Ok((lets, args))
    }

    /// A conditional on `condition` realizing `at`: the condition itself, or
    /// its negation, when the branches are the Boolean literals, and the
    /// branch evaluated after the condition when both are the same code.
    fn choose(
        &mut self,
        condition: Expr,
        then_branch: Block,
        else_branch: Block,
        at: &Origin,
    ) -> Expr {
        // Rust's lint gate refuses an `if` without `else` holding only
        // another, which an empty unit branch makes, so the inner one binds
        // its condition first.
        let mut then_branch = then_branch;
        if empty(&else_branch)
            && then_branch.lets.is_empty()
            && matches!(&then_branch.tail, Expr::If { else_branch: inner, .. } if empty(inner))
        {
            self.hold_condition(&mut then_branch);
        }
        match (literal(&then_branch), literal(&else_branch)) {
            (Some(true), Some(false)) => condition,
            (Some(false), Some(true)) => negate(condition, at),
            _ if same_code(&then_branch, &else_branch) => {
                let mut block = then_branch;
                block.lets.insert(
                    0,
                    Let {
                        pat: Pat::Wild,
                        ty: None,
                        value: condition,
                        at: at.clone(),
                    },
                );
                Expr::Block(Box::new(block))
            }
            // Rust's lint gate evaluates a conditional on a literal as the
            // constant it chooses and compares it with the branches around
            // it, so a literal condition is bound first.
            _ if matches!(condition, Expr::Lit(Lit::Bool(_), _)) => {
                let held = Ident::Holder(self.fresh());
                Expr::Block(Box::new(Block {
                    lets: vec![Let {
                        pat: Pat::Bind(held.clone()),
                        ty: None,
                        value: condition,
                        at: at.clone(),
                    }],
                    tail: Expr::If {
                        condition: Box::new(Expr::Move(held, at.clone())),
                        then_branch: Box::new(then_branch),
                        else_branch: Box::new(else_branch),
                        at: at.clone(),
                    },
                }))
            }
            _ => Expr::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
                at: at.clone(),
            },
        }
    }

    /// Bind the condition of the `if` that is `block`'s value before it.
    fn hold_condition(&mut self, block: &mut Block) {
        if let Expr::If { condition, at, .. } = &mut block.tail {
            let held = Ident::Holder(self.fresh());
            let tested =
                std::mem::replace(condition.as_mut(), Expr::Move(held.clone(), at.clone()));
            block.lets.push(Let {
                pat: Pat::Bind(held),
                ty: None,
                value: tested,
                at: at.clone(),
            });
        }
    }

    /// The arms of a match on `scrutinee_ty` over the bound `holder`. An arm
    /// whose shape holds a value of an uninhabited type is never taken and
    /// is omitted, as Rust's exhaustiveness admits.
    #[allow(clippy::too_many_lines)]
    fn arms(
        &mut self,
        holder: &Ident,
        scrutinee_ty: &Ty,
        arms: &[Arm],
        scope: &mut Vec<(u64, Ty)>,
    ) -> Result<Expr, String> {
        let matched = Origin::of("expr:match");
        let mut bodies: BTreeMap<Shape, (Vec<Pat>, Block)> = BTreeMap::new();
        let mut unreachable = BTreeSet::new();
        for arm in arms {
            let fields = self.checker.shape_fields(arm.shape, scrutinee_ty)?;
            if !fields.iter().all(|ty| self.inhabits(ty)) {
                unreachable.insert(arm.shape);
                continue;
            }
            let depth = scope.len();
            scope.extend(arm.binders.iter().copied().zip(fields.iter().cloned()));
            let body = self.expr(&arm.body, scope);
            scope.truncate(depth);
            let patterns = arm
                .binders
                .iter()
                .zip(&fields)
                .map(|(name, ty)| binder(*name, ty, &arm.body))
                .collect();
            bodies.insert(arm.shape, (patterns, block_of(body?)));
        }
        let moved = || Box::new(Expr::Move(holder.clone(), matched.clone()));
        let mut take = |shape: Shape| -> Result<Option<(Vec<Pat>, Block)>, String> {
            if unreachable.contains(&shape) {
                return Ok(None);
            }
            bodies
                .remove(&shape)
                .map(Some)
                .ok_or_else(|| format!("the match lacks {shape:?}"))
        };
        let first = |patterns: &[Pat], position: usize| {
            patterns
                .get(position)
                .cloned()
                .ok_or_else(|| format!("an arm lacks binder {position}"))
        };
        let lacks = |shape: &str| format!("the match lacks {shape}");
        let mut out: Vec<(Pat, Block)> = Vec::new();
        let rendered = match scrutinee_ty {
            Ty::Option { .. } => {
                if let Some((_, none)) = take(Shape::None)? {
                    out.push((Pat::None, none));
                }
                if let Some((patterns, some)) = take(Shape::Some)? {
                    out.push((Pat::Some(Box::new(first(&patterns, 0)?)), some));
                }
                Self::rebuilt(out, moved(), shape_at(&Shape::None))
            }
            Ty::Result { .. } => {
                if let Some((patterns, ok)) = take(Shape::Ok)? {
                    out.push((Pat::Ok(Box::new(first(&patterns, 0)?)), ok));
                }
                if let Some((patterns, error)) = take(Shape::Error)? {
                    out.push((Pat::Err(Box::new(first(&patterns, 0)?)), error));
                }
                Self::rebuilt(out, moved(), shape_at(&Shape::Ok))
            }
            Ty::List { .. } => {
                if let Some((_, nil)) = take(Shape::Nil)? {
                    out.push((Pat::None, nil));
                }
                if let Some((patterns, cons)) = take(Shape::Cons)? {
                    out.push((
                        Pat::Some(Box::new(Pat::Tuple(vec![
                            first(&patterns, 0)?,
                            first(&patterns, 1)?,
                        ]))),
                        cons,
                    ));
                }
                Expr::Match {
                    scrutinee: Box::new(Expr::Uncons(holder.clone())),
                    arms: out,
                    at: shape_at(&Shape::Nil),
                }
            }
            Ty::Nat => {
                let (_, zero) = take(Shape::Zero)?.ok_or_else(|| lacks("Zero"))?;
                let (patterns, mut succ) = take(Shape::Succ)?.ok_or_else(|| lacks("Succ"))?;
                let predecessor = first(&patterns, 0)?;
                if predecessor != Pat::Wild {
                    succ.lets.insert(
                        0,
                        Let {
                            pat: predecessor,
                            ty: None,
                            value: Expr::Predecessor(holder.clone()),
                            at: shape_at(&Shape::Succ),
                        },
                    );
                }
                let at = shape_at(&Shape::Zero);
                self.choose(Expr::IsZero(holder.clone(), at.clone()), zero, succ, &at)
            }
            Ty::Bool => {
                let (_, yes) = take(Shape::True)?.ok_or_else(|| lacks("True"))?;
                let (_, no) = take(Shape::False)?.ok_or_else(|| lacks("False"))?;
                self.choose(*moved(), yes, no, &shape_at(&Shape::True))
            }
            Ty::Pair { .. } => {
                let (patterns, mut body) = take(Shape::Pair)?.ok_or_else(|| lacks("Pair"))?;
                body.lets.insert(
                    0,
                    Let {
                        pat: Pat::Tuple(vec![first(&patterns, 0)?, first(&patterns, 1)?]),
                        ty: None,
                        value: *moved(),
                        at: shape_at(&Shape::Pair),
                    },
                );
                Expr::Block(Box::new(body))
            }
            Ty::Ordering => {
                for (shape, value) in [
                    (Shape::Lt, OrderingValue::Lt),
                    (Shape::Eq, OrderingValue::Eq),
                    (Shape::Gt, OrderingValue::Gt),
                ] {
                    let (_, body) = take(shape)?.ok_or_else(|| lacks("an ordering"))?;
                    out.push((Pat::Ordering(value), body));
                }
                Self::rebuilt(out, moved(), shape_at(&Shape::Lt))
            }
            Ty::Adt { index: adt } => {
                let count = self
                    .program
                    .adts
                    .get(index(*adt)?)
                    .map_or(0, |declared| declared.constructors.len())
                    as u64;
                for constructor in 0..count {
                    let Some((patterns, mut body)) = take(Shape::Adt { constructor })? else {
                        continue;
                    };
                    let types = self.adt_fields(*adt, constructor)?;
                    let mut fields = Vec::new();
                    let mut loads = Vec::new();
                    for (pattern, ty) in patterns.into_iter().zip(&types) {
                        if pattern != Pat::Wild && self.boxed(Owner::Adt(*adt), ty)? {
                            let held = Ident::Boxed(self.fresh());
                            loads.push(Let {
                                pat: pattern,
                                ty: None,
                                value: Expr::Unbox(held.clone(), Origin::of("indirection")),
                                at: Origin::of("indirection"),
                            });
                            fields.push(Pat::Bind(held));
                        } else {
                            fields.push(pattern);
                        }
                    }
                    loads.append(&mut body.lets);
                    body.lets = loads;
                    out.push((
                        Pat::Adt {
                            adt: *adt,
                            constructor,
                            fields,
                        },
                        body,
                    ));
                }
                Self::rebuilt(out, moved(), shape_at(&Shape::Adt { constructor: 0 }))
            }
            other => return fail(format!("cannot render a match on {other:?}")),
        };
        if !bodies.is_empty() {
            return fail("a match arm has no rendering");
        }
        Ok(rendered)
    }

    /// A match of `arms` on the moved scrutinee realizing `at`; when every
    /// arm rebuilds exactly what it matched, the match is its scrutinee.
    fn rebuilt(arms: Vec<(Pat, Block)>, scrutinee: Box<Expr>, at: Origin) -> Expr {
        if !arms.is_empty() && arms.iter().all(|(pattern, body)| rebuilds(pattern, body)) {
            *scrutinee
        } else {
            Expr::Match {
                scrutinee,
                arms,
                at,
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    fn expr(&mut self, expr: &Term, scope: &mut Vec<(u64, Ty)>) -> Result<Expr, String> {
        let at = term_at(expr);
        Ok(match expr {
            Term::Value { ty, value } => self.value(value, ty)?,
            Term::Var { name } => {
                let ty = scope
                    .iter()
                    .rev()
                    .find(|(bound, _)| bound == name)
                    .map(|(_, ty)| ty.clone())
                    .ok_or_else(|| format!("local {name} is unbound"))?;
                if ty == Ty::Unit {
                    Expr::Lit(Lit::Unit, at)
                } else if self.ty(&ty)?.copy() {
                    Expr::Copy(Ident::Local(*name), at)
                } else {
                    Expr::Clone(Ident::Local(*name), at)
                }
            }
            Term::Let {
                name,
                ty,
                bound,
                body,
            } => {
                self.inhabited(ty)?;
                let bound = self.expr(bound, scope)?;
                let lowered = self.ty(ty)?;
                scope.push((*name, ty.clone()));
                let rendered = self.expr(body, scope);
                scope.pop();
                let mut block = block_of(rendered?);
                block.lets.insert(
                    0,
                    Let {
                        pat: binder(*name, ty, body),
                        ty: Some(lowered),
                        value: bound,
                        at,
                    },
                );
                Expr::Block(Box::new(block))
            }
            Term::Cond {
                condition,
                then_branch,
                else_branch,
            } => {
                // The condition's bindings come before the `if`, so the
                // condition itself is one expression.
                let mut block = block_of(self.expr(condition, scope)?);
                let condition =
                    std::mem::replace(&mut block.tail, Expr::Lit(Lit::Unit, at.clone()));
                let then_branch = block_of(self.expr(then_branch, scope)?);
                let else_branch = block_of(self.expr(else_branch, scope)?);
                let chosen = self.choose(condition, then_branch, else_branch, &at);
                set_tail(&mut block, chosen);
                if block.lets.is_empty() {
                    block.tail
                } else {
                    Expr::Block(Box::new(block))
                }
            }
            Term::Match {
                ty,
                scrutinee,
                arms,
            } => {
                self.inhabited(ty)?;
                let scrutinee_ty = self.checker.expr(scrutinee, scope)?;
                let rendered = self.expr(scrutinee, scope)?;
                if scrutinee_ty == Ty::Unit {
                    // A unit has one shape and nothing to bind.
                    let arm = arms.first().ok_or("a match on unit has no arm")?;
                    let mut block = block_of(self.expr(&arm.body, scope)?);
                    block.lets.insert(
                        0,
                        Let {
                            pat: Pat::Unit,
                            ty: None,
                            value: rendered,
                            at: shape_at(&Shape::Unit),
                        },
                    );
                    return Ok(Expr::Block(Box::new(block)));
                }
                let holder = Ident::Holder(self.fresh());
                let body = self.arms(&holder, &scrutinee_ty, arms, scope)?;
                if matches!(&body, Expr::Move(moved, _) if *moved == holder) {
                    // Every arm rebuilds what it matched: the value is the
                    // scrutinee itself.
                    return Ok(rendered);
                }
                let mut block = block_of(body);
                block.lets.insert(
                    0,
                    Let {
                        pat: Pat::Bind(holder),
                        ty: None,
                        value: rendered,
                        at,
                    },
                );
                Expr::Block(Box::new(block))
            }
            Term::Build {
                shape,
                ty,
                operands,
            } => {
                self.inhabited(ty)?;
                self.build(shape, ty, operands, scope)?
            }
            Term::Call { function, operands } => {
                let callee = self
                    .program
                    .functions
                    .get(index(*function)?)
                    .ok_or_else(|| format!("function {function} is not declared"))?;
                let (types, result) = (callee.types.clone(), callee.result.clone());
                self.inhabited(&result)?;
                let (lets, args) = self.operands(operands, &types, &at, scope)?;
                after(
                    lets,
                    Expr::Call {
                        callee: Callee::Function(*function),
                        args,
                        propagate: self
                            .fallible
                            .get(index(*function)?)
                            .copied()
                            .unwrap_or(false),
                        at,
                    },
                )
            }
            Term::Closure { function, captures } => {
                let ty = self.checker.expr(expr, scope)?;
                let position = self.fn_index(&ty);
                let types: Vec<Ty> = self
                    .program
                    .functions
                    .get(index(*function)?)
                    .and_then(|callee| callee.types.get(..captures.len()))
                    .ok_or_else(|| format!("closure of function {function} is malformed"))?
                    .to_vec();
                let (lets, rendered) = self.operands(captures, &types, &at, scope)?;
                let mut args = Vec::new();
                for (value, ty) in rendered.into_iter().zip(&types) {
                    args.push(self.store(Owner::Fn(position), ty, value)?);
                }
                after(
                    lets,
                    Expr::Construct {
                        ctor: Ctor::Closure {
                            fn_type: position as u64,
                            function: *function,
                        },
                        args,
                        at,
                    },
                )
            }
            Term::Apply { target, operands } => {
                let ty = self.checker.expr(target, scope)?;
                let Ty::Fn { parameters, result } = &ty else {
                    return fail("`apply` of a value that is not a function");
                };
                self.inhabited(result)?;
                let position = self.fn_index(&ty);
                let callee = Ident::Callee(self.fresh());
                let target = self.expr(target, scope)?;
                let (mut lets, args) = self.operands(operands, parameters, &at, scope)?;
                lets.insert(
                    0,
                    Let {
                        pat: Pat::Bind(callee.clone()),
                        ty: Some(Type::Fn(position as u64)),
                        value: target,
                        at: at.clone(),
                    },
                );
                Expr::Block(Box::new(Block {
                    lets,
                    tail: Expr::Apply {
                        holder: callee,
                        args,
                        propagate: self.apply_fallible.get(&position).copied().unwrap_or(false),
                        at,
                    },
                }))
            }
            Term::Prim {
                operation,
                operands,
            } => {
                let types = operands
                    .iter()
                    .map(|operand| self.checker.expr(operand, scope))
                    .collect::<Result<Vec<_>, _>>()?;
                let realized = prim_at(operation, &types);
                let runtime = item(operation, &types)?;
                if runtime.heap() {
                    self.heap(&format!("the primitive {operation:?}"))?;
                }
                let rendered = self.exprs(operands, scope)?;
                // Operands are bound left to right before the call, so the
                // evaluation order is the denotation's.
                let mut lets = Vec::new();
                let mut args = Vec::new();
                for value in rendered {
                    let name = Ident::Operand(self.fresh());
                    lets.push(Let {
                        pat: Pat::Bind(name.clone()),
                        ty: None,
                        value,
                        at: at.clone(),
                    });
                    args.push(if matches!(operation, Prim::Convert { .. }) {
                        Expr::Widen(
                            Box::new(Expr::Move(name, at.clone())),
                            Origin::of(&realized.element),
                        )
                    } else {
                        Expr::Move(name, at.clone())
                    });
                }
                Expr::Block(Box::new(Block {
                    lets,
                    tail: Expr::Call {
                        callee: Callee::Runtime(runtime),
                        args,
                        propagate: runtime.fallible(),
                        at: realized,
                    },
                }))
            }
            Term::First { value } | Term::Second { value } => {
                let held = Ident::Part(self.fresh());
                let parts = if matches!(expr, Term::First { .. }) {
                    vec![Pat::Bind(held.clone()), Pat::Wild]
                } else {
                    vec![Pat::Wild, Pat::Bind(held.clone())]
                };
                Expr::Block(Box::new(Block {
                    lets: vec![Let {
                        pat: Pat::Tuple(parts),
                        ty: None,
                        value: self.expr(value, scope)?,
                        at: at.clone(),
                    }],
                    tail: Expr::Move(held, at),
                }))
            }
            Term::Field {
                value,
                index: position,
            } => {
                let Ty::Adt { index: adt } = self.checker.expr(value, scope)? else {
                    return fail("`field` of a non-ADT value");
                };
                let types = self.adt_fields(adt, 0)?;
                let selected = index(*position)?;
                let ty = types
                    .get(selected)
                    .cloned()
                    .ok_or_else(|| format!("ADT {adt} has no field {position}"))?;
                let held = Ident::Part(self.fresh());
                let fields: Vec<Pat> = (0..types.len())
                    .map(|field| {
                        if field == selected {
                            Pat::Bind(held.clone())
                        } else {
                            Pat::Wild
                        }
                    })
                    .collect();
                let read = if self.boxed(Owner::Adt(adt), &ty)? {
                    Expr::Unbox(held, Origin::of("indirection"))
                } else {
                    Expr::Move(held, at.clone())
                };
                // A computed record is bound first: Rust's lint gate refuses
                // a block as a scrutinee.
                let record = self.expr(value, scope)?;
                let (lets, scrutinee) = match record {
                    Expr::Copy(..) | Expr::Clone(..) | Expr::Move(..) => (Vec::new(), record),
                    computed => {
                        let holder = Ident::Holder(self.fresh());
                        (
                            vec![Let {
                                pat: Pat::Bind(holder.clone()),
                                ty: None,
                                value: computed,
                                at: at.clone(),
                            }],
                            Expr::Move(holder, at.clone()),
                        )
                    }
                };
                after(
                    lets,
                    Expr::Match {
                        scrutinee: Box::new(scrutinee),
                        arms: vec![(
                            Pat::Adt {
                                adt,
                                constructor: 0,
                                fields,
                            },
                            Block::of(read),
                        )],
                        at,
                    },
                )
            }
        })
    }

    /// A value built by `shape` at `ty` from `operands`.
    fn build(
        &mut self,
        shape: &Shape,
        ty: &Ty,
        operands: &[Term],
        scope: &mut Vec<(u64, Ty)>,
    ) -> Result<Expr, String> {
        let at = shape_at(shape);
        Ok(match (shape, ty) {
            (Shape::None, Ty::Option { value }) => Expr::Construct {
                ctor: Ctor::None(self.ty(value)?),
                args: Vec::new(),
                at,
            },
            (Shape::Some, Ty::Option { value }) => {
                let (lets, args) =
                    self.operands(operands, &[value.as_ref().clone()], &at, scope)?;
                after(
                    lets,
                    Expr::Construct {
                        ctor: Ctor::Some,
                        args,
                        at,
                    },
                )
            }
            (Shape::Ok | Shape::Error, Ty::Result { ok, error }) => {
                let held = if *shape == Shape::Ok { ok } else { error };
                let (lets, args) = self.operands(operands, &[held.as_ref().clone()], &at, scope)?;
                let (ok, error) = (self.ty(ok)?, self.ty(error)?);
                after(
                    lets,
                    Expr::Construct {
                        ctor: if *shape == Shape::Ok {
                            Ctor::Ok(ok, error)
                        } else {
                            Ctor::Err(ok, error)
                        },
                        args,
                        at,
                    },
                )
            }
            (Shape::Nil, Ty::List { element }) => Expr::Construct {
                ctor: Ctor::Nil(self.ty(element)?),
                args: Vec::new(),
                at,
            },
            (Shape::Cons, Ty::List { element }) => {
                self.heap("a list cell")?;
                let types = [element.as_ref().clone(), ty.clone()];
                let (lets, args) = self.operands(operands, &types, &at, scope)?;
                after(
                    lets,
                    Expr::Construct {
                        ctor: Ctor::Cons,
                        args,
                        at,
                    },
                )
            }
            (Shape::Zero, _) => Expr::Lit(Lit::Nat(0), at),
            (Shape::Succ, _) => Expr::Call {
                callee: Callee::Runtime(Item::NatSucc),
                args: self.exprs(operands, scope)?,
                propagate: true,
                at,
            },
            (Shape::Pair, _) => {
                let mut rendered = self.exprs(operands, scope)?;
                let (Some(right), Some(left), true) =
                    (rendered.pop(), rendered.pop(), rendered.is_empty())
                else {
                    return fail("a pair has two components");
                };
                Expr::Pair(Box::new(left), Box::new(right), at)
            }
            (Shape::True, _) => Expr::Lit(Lit::Bool(true), at),
            (Shape::False, _) => Expr::Lit(Lit::Bool(false), at),
            (Shape::Unit, _) => Expr::Lit(Lit::Unit, at),
            (Shape::Lt, _) => Expr::Lit(Lit::Ordering(OrderingValue::Lt), at),
            (Shape::Eq, _) => Expr::Lit(Lit::Ordering(OrderingValue::Eq), at),
            (Shape::Gt, _) => Expr::Lit(Lit::Ordering(OrderingValue::Gt), at),
            (Shape::Adt { constructor }, Ty::Adt { index: adt }) => {
                let types = self.adt_fields(*adt, *constructor)?;
                let (lets, rendered) = self.operands(operands, &types, &at, scope)?;
                let mut args = Vec::new();
                for (value, ty) in rendered.into_iter().zip(&types) {
                    args.push(self.store(Owner::Adt(*adt), ty, value)?);
                }
                after(
                    lets,
                    Expr::Construct {
                        ctor: Ctor::Adt {
                            adt: *adt,
                            constructor: *constructor,
                        },
                        args,
                        at,
                    },
                )
            }
            (shape, ty) => return fail(format!("cannot render {shape:?} at {ty:?}")),
        })
    }

    /// The crate: the ADTs, the function types and their dispatch, and the
    /// functions.
    #[allow(clippy::too_many_lines)]
    pub(super) fn lower(&mut self) -> Result<Crate, String> {
        let mut items = Vec::new();
        for (position, adt) in self.program.adts.iter().enumerate() {
            let owner = Owner::Adt(position as u64);
            let mut variants = Vec::new();
            for (constructor, fields) in adt.constructors.iter().enumerate() {
                let lowered = fields
                    .iter()
                    .map(|ty| self.field_ty(owner, ty))
                    .collect::<Result<Vec<_>, _>>()?;
                variants.push((constructor as u64, lowered));
            }
            items.push(ItemDef::Enum {
                name: Type::Adt(position as u64),
                variants,
                at: Origin::of("type:adt"),
            });
        }
        let fn_types = self.fn_types.clone();
        for (position, fn_ty) in fn_types.iter().enumerate() {
            let owner = Owner::Fn(position);
            let Ty::Fn { parameters, result } = fn_ty else {
                return fail("a function type is not a function");
            };
            // A function type is realized by its closures where the
            // program has one, and by the stated type otherwise.
            let at = Origin::of(if self.closure_types.contains(&position) {
                "expr:closure"
            } else {
                "type:fn"
            });
            let closures = self.closures.get(&position).cloned().unwrap_or_default();
            let mut variants = Vec::new();
            let mut arms = Vec::new();
            for (function, captured) in &closures {
                let lowered = captured
                    .iter()
                    .map(|ty| self.field_ty(owner, ty))
                    .collect::<Result<Vec<_>, _>>()?;
                variants.push((*function, lowered));
                let captures = captured
                    .iter()
                    .map(|ty| {
                        Ok(if *ty == Ty::Unit {
                            CaptureRead::Unit
                        } else if self.boxed(owner, ty)? {
                            CaptureRead::Unbox
                        } else if self.ty(ty)?.copy() {
                            CaptureRead::Copy
                        } else {
                            CaptureRead::Clone
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                arms.push(Dispatch {
                    function: *function,
                    captures,
                    function_fallible: self
                        .fallible
                        .get(index(*function)?)
                        .copied()
                        .unwrap_or(false),
                });
            }
            items.push(ItemDef::Enum {
                name: Type::Fn(position as u64),
                variants,
                at: at.clone(),
            });
            let parameters = parameters
                .iter()
                .map(|ty| self.ty(ty))
                .collect::<Result<Vec<_>, _>>()?;
            items.push(ItemDef::Apply {
                fn_type: position as u64,
                parameters,
                result: self.ty(result)?,
                fallible: self.apply_fallible.get(&position).copied().unwrap_or(false),
                arms,
                at,
            });
        }
        for (position, function) in self.program.functions.iter().enumerate() {
            let result = self.ty(&function.result)?;
            let fallible = self.fallible.get(position).copied().unwrap_or(false);
            let result = if fallible {
                Type::Fallible(Box::new(result))
            } else {
                result
            };
            let empty = function.types.iter().position(|ty| !self.inhabits(ty));
            let (parameters, body) = if let Some(empty) = empty {
                // No value of this parameter exists, so the function is never
                // called: its body is the empty match on that parameter.
                let mut parameters = Vec::new();
                for (at, (name, ty)) in function.parameters.iter().zip(&function.types).enumerate()
                {
                    let pattern = if at == empty {
                        Pat::Bind(Ident::Local(*name))
                    } else {
                        Pat::Wild
                    };
                    parameters.push((pattern, self.ty(ty)?));
                }
                let name = *function
                    .parameters
                    .get(empty)
                    .ok_or("the uninhabited parameter is not declared")?;
                let realized = Origin::of(&element("type", &function.types[empty]));
                (
                    parameters,
                    Block::of(Expr::Match {
                        scrutinee: Box::new(Expr::Move(Ident::Local(name), realized.clone())),
                        arms: Vec::new(),
                        at: realized,
                    }),
                )
            } else {
                let mut scope = Self::scope(function);
                let parameters = function
                    .parameters
                    .iter()
                    .zip(&function.types)
                    .map(|(name, ty)| Ok((binder(*name, ty, &function.body), self.ty(ty)?)))
                    .collect::<Result<Vec<_>, String>>()?;
                let body = self.expr(&function.body, &mut scope)?;
                let mut body = if fallible {
                    block_of(succeed(body, function.result == Ty::Unit))
                } else {
                    block_of(body)
                };
                fold(&mut body);
                (parameters, body)
            };
            items.push(ItemDef::Function {
                name: Ident::Function(position as u64),
                parameters,
                result,
                body,
                at: Origin::of("function"),
            });
        }
        Ok(Crate {
            profile: self.profile,
            items,
        })
    }
}
