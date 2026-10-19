//! A function whose self calls are all tail calls, rendered as a loop
//! (SPEC.md §17.16, issue 105).
//!
//! The calculus has no loop, so a library template that iterates calls
//! itself (§17.14), and `rustc` does not promise to turn that call into a
//! jump: the stack depth of the rendering is the iteration count. A function
//! every self call of which is a tail call needs no stack for them. The
//! lowering states it as `RustSyntax`'s one repeating construct, over a state
//! made of the parameters the self calls change:
//!
//! ```text
//! fn f(a, b, c) { body }       fn f(a', b', c) {
//!                                 let s = (a', b');
//!                                 loop(s) { let (a, b) = s; body' }
//!                              }
//! ```
//!
//! where every tail self call `f(x, y, c)` of `body` is `Ok((x, y))`, which
//! continues with that state, and every other tail value `e` is `Err(e)`,
//! which ends the loop with it. A parameter every self call passes on as it
//! got it is no part of the state; the body reads it, which the validator
//! requires to be a clone. A function with a self call that is not a tail call
//! (a call whose result is used, or an argument of another call) recurses and
//! is left as it is, which is what the `recursion` effect of its realization
//! says (§17.13).

use super::ast::{Block, Callee, Ctor, Expr, Ident, Let, Origin, Pat, Type};

/// Whether `expr` calls function `me` anywhere.
fn occurs(me: u64, expr: &Expr) -> bool {
    match expr {
        Expr::Call { callee, args, .. } => {
            matches!(callee, Callee::Function(called) if *called == me)
                || args.iter().any(|arg| occurs(me, arg))
        }
        Expr::Apply { args, .. } | Expr::Construct { args, .. } => {
            args.iter().any(|arg| occurs(me, arg))
        }
        Expr::Not(inner, _)
        | Expr::Box(inner, _)
        | Expr::Widen(inner, _)
        | Expr::Succeed(inner, _) => occurs(me, inner),
        Expr::Pair(left, right, _) => occurs(me, left) || occurs(me, right),
        Expr::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => occurs(me, condition) || in_block(me, then_branch) || in_block(me, else_branch),
        Expr::Match {
            scrutinee, arms, ..
        } => occurs(me, scrutinee) || arms.iter().any(|(_, body)| in_block(me, body)),
        Expr::Block(block) | Expr::Loop { body: block, .. } => in_block(me, block),
        Expr::Lit(..)
        | Expr::Move(..)
        | Expr::Clone(..)
        | Expr::Copy(..)
        | Expr::Deref(..)
        | Expr::Unbox(..)
        | Expr::Uncons(_)
        | Expr::IsZero(..)
        | Expr::NonZero(..)
        | Expr::Predecessor(_) => false,
    }
}

fn in_block(me: u64, block: &Block) -> bool {
    block.lets.iter().any(|binding| occurs(me, &binding.value)) || occurs(me, &block.tail)
}

/// Collect the arguments of every self call in tail position of `block`, and
/// say whether those are the only self calls in it.
fn tails(me: u64, block: &Block, found: &mut Vec<Vec<Expr>>) -> bool {
    if block.lets.iter().any(|binding| occurs(me, &binding.value)) {
        return false;
    }
    tail(me, &block.tail, found)
}

fn tail(me: u64, expr: &Expr, found: &mut Vec<Vec<Expr>>) -> bool {
    match expr {
        Expr::Call {
            callee: Callee::Function(called),
            args,
            ..
        } if *called == me => {
            if args.iter().any(|arg| occurs(me, arg)) {
                return false;
            }
            found.push(args.clone());
            true
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            !occurs(me, condition) && tails(me, then_branch, found) && tails(me, else_branch, found)
        }
        Expr::Match {
            scrutinee, arms, ..
        } => !occurs(me, scrutinee) && arms.iter().all(|(_, body)| tails(me, body, found)),
        Expr::Block(block) => tails(me, block, found),
        other => !occurs(me, other),
    }
}

/// Whether `argument` passes parameter `name` on unchanged.
fn passes(argument: &Expr, name: &Ident) -> bool {
    matches!(
        argument,
        Expr::Move(read, _) | Expr::Clone(read, _) | Expr::Copy(read, _) | Expr::Deref(read, _)
            if read == name
    )
}

/// The state the loop carries: the changing parameters, nested to the right
/// as the calculus nests a tuple of pairs.
struct State {
    /// Positions of the parameters the self calls change.
    changed: Vec<usize>,
    /// The type of the state.
    ty: Type,
    /// The type of the value the function returns.
    done: Type,
}

impl State {
    /// The expression of the next state, from the arguments of a self call.
    fn next(&self, args: &[Expr], at: &Origin) -> Expr {
        let mut parts: Vec<Expr> = self
            .changed
            .iter()
            .map(|position| args[*position].clone())
            .collect();
        let mut state = parts.pop().expect("a loop changes a parameter");
        while let Some(left) = parts.pop() {
            state = Expr::Pair(Box::new(left), Box::new(state), at.clone());
        }
        Expr::Construct {
            ctor: Ctor::Ok(self.ty.clone(), self.done.clone()),
            args: vec![state],
            at: at.clone(),
        }
    }

    /// The expression that ends the loop with `value`.
    fn end(&self, value: Expr, at: &Origin) -> Expr {
        Expr::Construct {
            ctor: Ctor::Err(self.ty.clone(), self.done.clone()),
            args: vec![value],
            at: at.clone(),
        }
    }
}

/// Make every tail self call of `block` the next state, and every other
/// tail value the end of the loop.
fn retarget(me: u64, state: &State, block: &mut Block, at: &Origin) {
    let tail = std::mem::replace(
        &mut block.tail,
        Expr::Lit(super::ast::Lit::Unit, at.clone()),
    );
    block.tail = match tail {
        Expr::Call {
            callee: Callee::Function(called),
            args,
            at: call,
            ..
        } if called == me => state.next(&args, &call),
        Expr::If {
            condition,
            mut then_branch,
            mut else_branch,
            at: here,
        } => {
            retarget(me, state, &mut then_branch, at);
            retarget(me, state, &mut else_branch, at);
            Expr::If {
                condition,
                then_branch,
                else_branch,
                at: here,
            }
        }
        Expr::Match {
            scrutinee,
            mut arms,
            at: here,
        } => {
            for (_, body) in &mut arms {
                retarget(me, state, body, at);
            }
            Expr::Match {
                scrutinee,
                arms,
                at: here,
            }
        }
        Expr::Block(mut inner) => {
            retarget(me, state, &mut inner, at);
            Expr::Block(inner)
        }
        other => state.end(other, at),
    };
}

/// The function `me`, whose parameters are `parameters` and whose value has
/// type `done`, as a loop when all its self calls are tail calls and some
/// of them change a parameter: its new parameters and body. `fresh` names
/// the parameters that the state replaces.
pub(super) fn looped(
    me: u64,
    parameters: &[(Pat, Type)],
    done: &Type,
    body: &Block,
    fresh: &mut dyn FnMut() -> u64,
) -> Option<(Vec<(Pat, Type)>, Block)> {
    let mut calls = Vec::new();
    if !tails(me, body, &mut calls) || calls.is_empty() {
        return None;
    }
    let mut changed = Vec::new();
    for (position, (pattern, _)) in parameters.iter().enumerate() {
        // A parameter nothing reads cannot be recovered from the loop's
        // state, so a function with one stays a call.
        let Pat::Bind(name) = pattern else {
            return None;
        };
        if calls.iter().any(|args| !passes(&args[position], name)) {
            changed.push(position);
        }
    }
    if changed.is_empty() {
        return None;
    }
    // The state nests its components to the right, as a tuple of pairs.
    let ty = changed
        .iter()
        .rev()
        .map(|position| parameters[*position].1.clone())
        .reduce(|right, left| Type::Pair(Box::new(left), Box::new(right)))?;
    let state = State {
        changed: changed.clone(),
        ty,
        done: done.clone(),
    };
    let at = Origin::of("expr:call");
    let mut inner = body.clone();
    retarget(me, &state, &mut inner, &at);
    let mut renamed = parameters.to_vec();
    let outer = {
        // The state is held by one binding the function makes from the
        // renamed parameters, whether there is one or several: a single
        // parameter is the state itself, several are a tuple.
        let held = Ident::Holder(fresh());
        let mut patterns = Vec::new();
        let mut values = Vec::new();
        for position in &changed {
            let name = Ident::Param(fresh());
            patterns.push(parameters[*position].0.clone());
            values.push(Expr::Move(name.clone(), at.clone()));
            renamed[*position].0 = Pat::Bind(name);
        }
        let mut tuple = patterns.pop().expect("a changed parameter");
        let mut start = values.pop().expect("a changed parameter");
        while let (Some(left), Some(read)) = (patterns.pop(), values.pop()) {
            tuple = Pat::Tuple(vec![left, tuple]);
            start = Expr::Pair(Box::new(read), Box::new(start), at.clone());
        }
        let mut looping = inner;
        looping.lets.insert(
            0,
            Let {
                pat: tuple,
                ty: None,
                value: Expr::Move(held.clone(), at.clone()),
                at: at.clone(),
            },
        );
        Block {
            lets: vec![Let {
                pat: Pat::Bind(held.clone()),
                ty: None,
                value: start,
                at: at.clone(),
            }],
            tail: Expr::Loop {
                state: held,
                body: Box::new(looping),
            },
        }
    };
    Some((renamed, outer))
}

use super::super::{Expr as Term, Function, Ty};

/// Whether the calculus expression calls or closes over function `me`.
fn names(me: u64, expr: &Term) -> bool {
    let any = |exprs: &[Term]| exprs.iter().any(|expr| names(me, expr));
    match expr {
        Term::Value { .. } | Term::Var { .. } => false,
        Term::Let { bound, body, .. } => names(me, bound) || names(me, body),
        Term::Cond {
            condition,
            then_branch,
            else_branch,
        } => names(me, condition) || names(me, then_branch) || names(me, else_branch),
        Term::Match {
            scrutinee, arms, ..
        } => names(me, scrutinee) || arms.iter().any(|arm| names(me, &arm.body)),
        Term::Build { operands, .. } | Term::Prim { operands, .. } => any(operands),
        Term::Call { function, operands } => *function == me || any(operands),
        Term::Closure { function, captures } => *function == me || any(captures),
        Term::Apply { target, operands } => names(me, target) || any(operands),
        Term::First { value } | Term::Second { value } | Term::Field { value, .. } => {
            names(me, value)
        }
    }
}

/// Collect the operands of every self call in tail position of `expr`, and
/// say whether those are the only mentions of `me` in it.
fn calls_in_tail<'a>(me: u64, expr: &'a Term, found: &mut Vec<&'a [Term]>) -> bool {
    match expr {
        Term::Call { function, operands } if *function == me => {
            if operands.iter().any(|operand| names(me, operand)) {
                return false;
            }
            found.push(operands);
            true
        }
        Term::Let { bound, body, .. } => !names(me, bound) && calls_in_tail(me, body, found),
        Term::Cond {
            condition,
            then_branch,
            else_branch,
        } => {
            !names(me, condition)
                && calls_in_tail(me, then_branch, found)
                && calls_in_tail(me, else_branch, found)
        }
        Term::Match {
            scrutinee, arms, ..
        } => !names(me, scrutinee) && arms.iter().all(|arm| calls_in_tail(me, &arm.body, found)),
        other => !names(me, other),
    }
}

/// Whether the rendering of function `me` is a loop: it calls itself, every
/// such call is a tail call, and some call changes a parameter the body
/// reads. This is [`looped`] decided on the calculus, so that the effect of a
/// realization (`recursion`, §17.13) can be derived from the program and the
/// lowering is held to it (it fails when the two disagree).
#[must_use]
pub fn loops(me: u64, function: &Function) -> bool {
    let mut calls = Vec::new();
    if !calls_in_tail(me, &function.body, &mut calls) || calls.is_empty() {
        return false;
    }
    // A parameter nothing reads cannot be recovered from the loop's state, so
    // a function with one stays a call.
    let unread =
        |name: u64, ty: &Ty| *ty == Ty::Unit || !super::lower::mentions(&function.body, name);
    if function
        .parameters
        .iter()
        .zip(&function.types)
        .any(|(name, ty)| unread(*name, ty))
    {
        return false;
    }
    function
        .parameters
        .iter()
        .zip(&function.types)
        .enumerate()
        .any(|(position, (name, ty))| {
            !unread(*name, ty)
                && calls.iter().any(|operands| {
                    !matches!(&operands[position], Term::Var { name: passed } if passed == name)
                })
        })
}
