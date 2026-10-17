//! Last-use moves: a read of a binding that nothing after it reads is a
//! move, and the projection of a pair component that nothing after it reads
//! moves that component out of the pair.
//!
//! The lowering reads every binding by cloning it, which is always sound and
//! never changes a value (§17.14: using a local clones a handle or a value of
//! fixed size). A clone of a handle is a reference count and a clone of a
//! pair of handles is one per component, so a program that threads a pair of
//! lists through a fold pays, at every read of every component, for the
//! whole pair. This pass runs over each function body after lowering and
//! before validation, backwards through the evaluation order, and replaces a
//! clone with a move exactly where the clone is the last read: the value is
//! the same, the machine does less. The denotation never sees the difference,
//! because `move`, `clone`, and `copy` all read the binding (`RustSemantics`),
//! and the correspondence row of every read is the same.
//!
//! Evaluation order is the printed order: a block's bindings then its tail,
//! a call's operands left to right, a conditional's condition then the branch
//! taken, a match's scrutinee then the arm taken. Branches are alternatives,
//! so what one branch reads is not read before its sibling; what the code
//! after the branch reads is read after both.

use super::ast::{Block, Expr, Ident, Let, Pat};
use std::collections::BTreeMap;

/// How a binding is read by the code after a point: as a whole, and by
/// component. A count is the most reads on any path, so that a component
/// read once on each of two alternative paths is read once.
#[derive(Debug, Clone, Default)]
struct Later {
    whole: usize,
    parts: BTreeMap<usize, usize>,
}

type Live = BTreeMap<Ident, Later>;

/// Replace every last read in `body` with a move.
pub fn apply(body: &mut Block) {
    let mut live = Live::new();
    block(body, &mut live);
}

/// The one component a projection pattern binds: `(h, _)` is `0`, `(_, h)`
/// is `1`, and a pattern binding more than one component is not a projection.
fn projected(parts: &[Pat]) -> Option<usize> {
    let mut bound = parts
        .iter()
        .enumerate()
        .filter(|(_, part)| **part != Pat::Wild);
    match (bound.next(), bound.next()) {
        (Some((index, _)), None) if parts.len() > 1 => Some(index),
        _ => None,
    }
}

fn read_whole(live: &mut Live, ident: &Ident) {
    live.entry(ident.clone()).or_default().whole += 1;
}

fn read_part(live: &mut Live, ident: &Ident, part: usize) {
    *live
        .entry(ident.clone())
        .or_default()
        .parts
        .entry(part)
        .or_default() += 1;
}

/// Whether a whole read here may move: nothing after it reads the binding.
fn may_move(live: &Live, ident: &Ident) -> bool {
    live.get(ident)
        .is_none_or(|later| later.whole == 0 && later.parts.is_empty())
}

/// Whether a projection of `part` here may move it out of the pair: nothing
/// after it reads the pair whole or reads `part`, and every other component
/// read after it is read once, so that each of those reads is itself a move
/// (a pair one component of which was moved cannot be cloned).
fn may_move_part(live: &Live, ident: &Ident, part: usize) -> bool {
    live.get(ident).is_none_or(|later| {
        later.whole == 0
            && !later.parts.contains_key(&part)
            && later.parts.values().all(|count| *count <= 1)
    })
}

/// The reads on either of two alternative paths.
fn merge(mut left: Live, right: Live) -> Live {
    for (ident, later) in right {
        let entry = left.entry(ident).or_default();
        entry.whole = entry.whole.max(later.whole);
        for (part, count) in later.parts {
            let slot = entry.parts.entry(part).or_default();
            *slot = (*slot).max(count);
        }
    }
    left
}

fn unbind(pattern: &Pat, live: &mut Live) {
    match pattern {
        Pat::Bind(ident) => {
            live.remove(ident);
        }
        Pat::Tuple(parts) | Pat::Adt { fields: parts, .. } => {
            parts.iter().for_each(|part| unbind(part, live));
        }
        Pat::Some(inner) | Pat::Ok(inner) | Pat::Err(inner) => unbind(inner, live),
        Pat::Wild | Pat::Unit | Pat::None | Pat::Ordering(_) => {}
    }
}

fn block(body: &mut Block, live: &mut Live) {
    expr(&mut body.tail, live);
    for binding in body.lets.iter_mut().rev() {
        unbind(&binding.pat, live);
        let Let { pat, value, .. } = binding;
        let projection = match (&*pat, &*value) {
            (Pat::Tuple(parts), Expr::Clone(ident, at)) => {
                projected(parts).map(|part| (part, ident.clone(), at.clone()))
            }
            _ => None,
        };
        if let Some((part, ident, at)) = projection {
            if may_move_part(live, &ident, part) {
                *value = Expr::Move(ident.clone(), at);
            }
            read_part(live, &ident, part);
            continue;
        }
        expr(value, live);
    }
}

fn exprs(items: &mut [Expr], live: &mut Live) {
    items.iter_mut().rev().for_each(|item| expr(item, live));
}

fn expr(value: &mut Expr, live: &mut Live) {
    match value {
        Expr::Clone(ident, at) => {
            let (ident, at) = (ident.clone(), at.clone());
            if may_move(live, &ident) {
                *value = Expr::Move(ident.clone(), at);
            }
            read_whole(live, &ident);
        }
        Expr::Move(ident, _)
        | Expr::Copy(ident, _)
        | Expr::Deref(ident, _)
        | Expr::Unbox(ident, _)
        | Expr::Uncons(ident)
        | Expr::IsZero(ident, _)
        | Expr::NonZero(ident, _)
        | Expr::Predecessor(ident) => read_whole(live, ident),
        Expr::Lit(..) => {}
        Expr::Not(inner, _)
        | Expr::Box(inner, _)
        | Expr::Widen(inner, _)
        | Expr::Succeed(inner, _) => {
            expr(inner, live);
        }
        Expr::Call { args, .. } | Expr::Construct { args, .. } => exprs(args, live),
        Expr::Apply { holder, args, .. } => {
            // The closure is borrowed for the whole application, so an
            // operand cannot move it: it is read before the operands.
            read_whole(live, holder);
            exprs(args, live);
        }
        Expr::Pair(left, right, _) => {
            expr(right, live);
            expr(left, live);
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            let mut taken = live.clone();
            block(then_branch, &mut taken);
            let mut other = std::mem::take(live);
            block(else_branch, &mut other);
            *live = merge(taken, other);
            expr(condition, live);
        }
        Expr::Match {
            scrutinee, arms, ..
        } => {
            let after = std::mem::take(live);
            let mut before = Live::new();
            for (pattern, body) in arms.iter_mut() {
                let mut path = after.clone();
                block(body, &mut path);
                unbind(pattern, &mut path);
                before = merge(before, path);
            }
            // A match without arms reads nothing after its scrutinee.
            *live = if arms.is_empty() { after } else { before };
            expr(scrutinee, live);
        }
        Expr::Block(inner) => block(inner, live),
    }
}
