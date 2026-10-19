//! The loop form of a function and its inversion (SPEC.md §17.17, `SP-20`).
//!
//! The rendering turns a function whose self calls are all tail calls into a
//! loop (§17.16): the parameters the calls change are replaced by renamed
//! ones, a holder takes them as a state, and the body, with the tail calls
//! turned into `Ok` of the next state and every other tail value into `Err`,
//! runs in `loop`. The preservation library states that form as `loopOf`
//! over an ordinary recursive body, and certificate B derives the
//! correspondence for the recursive body, which the crate no longer holds.
//! [`invert`] recovers that body from the loop form, and [`form`] is `loopOf`
//! together with the Boolean checks of `loopCheckOf`, transliterated from the
//! library, so that a crate whose loop is not the form of any recursive body
//! is refused here with the reason, not only by the kernel.
//!
//! Three things of the recursive body leave no trace in the loop form and are
//! chosen, not recovered: the form in which a kept parameter is read by a
//! self call (the library accepts every read of the parameter, and `Clone` is
//! the one the rendering writes for it), the source of a dropped parameter's
//! argument (recovered only when it is a temporary the block binds, which
//! the block keeps), and nothing else. Whatever is chosen is held to the
//! calculus by the derivation of the recursive body.

use crate::calculus::rust::ast::{Block, Callee, Ctor, Expr, Ident, Let, Origin, Pat, Type};

/// How a parameter position is carried by the loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Position {
    /// The self calls change it: it is part of the state.
    Changed,
    /// The self calls pass it on as it is.
    Kept(Ident),
    /// Nothing reads it.
    Dropped,
}

/// A recursive body, with what turns it into its loop form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recursive {
    /// One position per parameter.
    pub positions: Vec<Position>,
    /// The numbers of the renamed parameters, in parameter order.
    pub renamed: Vec<u64>,
    /// The number of the holder of the state.
    pub holder: u64,
    /// The type of the state.
    pub state: Type,
    /// The type of the value the function returns.
    pub done: Type,
    /// The parameters of the recursive function.
    pub parameters: Vec<(Pat, Type)>,
    /// Its body.
    pub body: Block,
}

fn origin() -> Origin {
    Origin::of("expr:call")
}

fn read_name(expr: &Expr) -> Option<&Ident> {
    match expr {
        Expr::Move(read, _) | Expr::Clone(read, _) | Expr::Copy(read, _) | Expr::Deref(read, _) => {
            Some(read)
        }
        _ => None,
    }
}

fn is_pure(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lit(..)
            | Expr::Move(..)
            | Expr::Clone(..)
            | Expr::Copy(..)
            | Expr::Deref(..)
            | Expr::Unbox(..)
    )
}

/// The arguments of a self call that the state carries, or none when a kept
/// parameter is not passed on or a dropped argument may have an effect.
fn carried(positions: &[Position], args: &[Expr]) -> Option<Vec<Expr>> {
    if positions.len() != args.len() {
        return None;
    }
    let mut state = Vec::new();
    for (position, arg) in positions.iter().zip(args) {
        match position {
            Position::Changed => state.push(arg.clone()),
            Position::Kept(name) => {
                if read_name(arg) != Some(name) {
                    return None;
                }
            }
            Position::Dropped => {
                if !is_pure(arg) {
                    return None;
                }
            }
        }
    }
    Some(state)
}

/// The components nested to the right as a pair, from the last.
fn pack(parts: Vec<Expr>, at: &Origin) -> Option<Expr> {
    let mut parts = parts;
    let mut state = parts.pop()?;
    while let Some(left) = parts.pop() {
        state = Expr::Pair(Box::new(left), Box::new(state), at.clone());
    }
    Some(state)
}

fn binders(pattern: &Pat, out: &mut Vec<Ident>) {
    match pattern {
        Pat::Wild | Pat::Unit | Pat::None | Pat::Ordering(_) => {}
        Pat::Bind(name) => out.push(name.clone()),
        Pat::Tuple(parts) | Pat::Adt { fields: parts, .. } => {
            parts.iter().for_each(|part| binders(part, out))
        }
        Pat::Some(inner) | Pat::Ok(inner) | Pat::Err(inner) => binders(inner, out),
    }
}

fn kept_names(positions: &[Position]) -> Vec<&Ident> {
    positions
        .iter()
        .filter_map(|position| match position {
            Position::Kept(name) => Some(name),
            Position::Changed | Position::Dropped => None,
        })
        .collect()
}

/// A pattern binds none of the kept parameters.
fn hygienic_pattern(kept: &[&Ident], pattern: &Pat) -> bool {
    let mut bound = Vec::new();
    binders(pattern, &mut bound);
    bound.iter().all(|name| !kept.contains(&name))
}

struct Retarget<'a> {
    me: u64,
    positions: &'a [Position],
    state: &'a Type,
    done: &'a Type,
}

impl Retarget<'_> {
    fn end(&self, value: Expr) -> Expr {
        Expr::Construct {
            ctor: Ctor::Err(self.state.clone(), self.done.clone()),
            args: vec![value],
            at: origin(),
        }
    }

    fn expr(&self, tail: &Expr) -> Option<Expr> {
        Some(match tail {
            Expr::Call {
                callee: Callee::Function(called),
                args,
                propagate: false,
                at,
            } if *called == self.me => {
                let state = pack(carried(self.positions, args)?, at)?;
                Expr::Construct {
                    ctor: Ctor::Ok(self.state.clone(), self.done.clone()),
                    args: vec![state],
                    at: at.clone(),
                }
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
                at,
            } => Expr::If {
                condition: condition.clone(),
                then_branch: Box::new(self.block(then_branch)?),
                else_branch: Box::new(self.block(else_branch)?),
                at: at.clone(),
            },
            Expr::Match {
                scrutinee,
                arms,
                at,
            } => {
                let kept = kept_names(self.positions);
                let mut retargeted = Vec::new();
                for (pattern, body) in arms {
                    if !hygienic_pattern(&kept, pattern) {
                        return None;
                    }
                    retargeted.push((pattern.clone(), self.block(body)?));
                }
                Expr::Match {
                    scrutinee: scrutinee.clone(),
                    arms: retargeted,
                    at: at.clone(),
                }
            }
            Expr::Block(inner) => Expr::Block(Box::new(self.block(inner)?)),
            other => self.end(other.clone()),
        })
    }

    fn block(&self, block: &Block) -> Option<Block> {
        let kept = kept_names(self.positions);
        if !block
            .lets
            .iter()
            .all(|binding| hygienic_pattern(&kept, &binding.pat))
        {
            return None;
        }
        Some(Block {
            lets: block.lets.clone(),
            tail: self.expr(&block.tail)?,
        })
    }
}

fn occurrences_block(block: &Block, out: &mut Vec<Ident>) {
    for binding in &block.lets {
        occurrences(&binding.value, out);
    }
    occurrences(&block.tail, out);
}

fn occurrences(expr: &Expr, out: &mut Vec<Ident>) {
    match expr {
        Expr::Lit(..) => {}
        Expr::Move(read, _)
        | Expr::Clone(read, _)
        | Expr::Copy(read, _)
        | Expr::Deref(read, _)
        | Expr::Unbox(read, _)
        | Expr::Uncons(read)
        | Expr::IsZero(read, _)
        | Expr::NonZero(read, _)
        | Expr::Predecessor(read) => out.push(read.clone()),
        Expr::Not(inner, _)
        | Expr::Box(inner, _)
        | Expr::Widen(inner, _)
        | Expr::Succeed(inner, _) => occurrences(inner, out),
        Expr::Call { args, .. } | Expr::Construct { args, .. } => {
            args.iter().for_each(|arg| occurrences(arg, out));
        }
        Expr::Apply { holder, args, .. } => {
            out.push(holder.clone());
            args.iter().for_each(|arg| occurrences(arg, out));
        }
        Expr::Pair(left, right, _) => {
            occurrences(left, out);
            occurrences(right, out);
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            occurrences(condition, out);
            occurrences_block(then_branch, out);
            occurrences_block(else_branch, out);
        }
        Expr::Match {
            scrutinee, arms, ..
        } => {
            occurrences(scrutinee, out);
            arms.iter()
                .for_each(|(_, body)| occurrences_block(body, out));
        }
        Expr::Block(inner) => occurrences_block(inner, out),
        Expr::Loop { state, body } => {
            out.push(state.clone());
            occurrences_block(body, out);
        }
    }
}

fn distinct(names: &[Ident]) -> bool {
    names
        .iter()
        .enumerate()
        .all(|(at, name)| !names[at + 1..].contains(name))
}

/// `loopOf` and `loopCheckOf` of the library on the recursive body of
/// function `me`: the parameters and the body of its loop form, or none when
/// the library would not state them or its checks refuse them.
#[must_use]
pub fn form(me: u64, recursive: &Recursive) -> Option<(Vec<(Pat, Type)>, Block)> {
    let renamed: Vec<Ident> = recursive.renamed.iter().map(|k| Ident::Param(*k)).collect();
    if recursive.positions.len() != recursive.parameters.len() {
        return None;
    }
    let mut fresh = renamed.iter();
    let mut parameters = Vec::new();
    let mut originals = Vec::new();
    for (position, (pattern, ty)) in recursive.positions.iter().zip(&recursive.parameters) {
        match (position, pattern) {
            (Position::Changed, Pat::Bind(binder)) => {
                parameters.push((Pat::Bind(fresh.next()?.clone()), ty.clone()));
                originals.push(binder.clone());
            }
            (Position::Kept(name), Pat::Bind(binder)) if name == binder => {
                parameters.push((pattern.clone(), ty.clone()));
            }
            (Position::Dropped, Pat::Wild) => parameters.push((pattern.clone(), ty.clone())),
            _ => return None,
        }
    }
    if fresh.next().is_some() {
        return None;
    }
    let walk = Retarget {
        me,
        positions: &recursive.positions,
        state: &recursive.state,
        done: &recursive.done,
    };
    let retargeted = walk.block(&recursive.body)?;
    let at = origin();
    let mut reads: Vec<Expr> = renamed
        .iter()
        .map(|name| Expr::Move(name.clone(), at.clone()))
        .collect();
    let start = pack(std::mem::take(&mut reads), &at)?;
    let mut patterns: Vec<Pat> = originals
        .iter()
        .map(|name| Pat::Bind(name.clone()))
        .collect();
    let mut tuple = patterns.pop()?;
    while let Some(left) = patterns.pop() {
        tuple = Pat::Tuple(vec![left, tuple]);
    }
    let held = Ident::Holder(recursive.holder);
    let mut lets = vec![Let {
        pat: tuple,
        ty: None,
        value: Expr::Move(held.clone(), at.clone()),
        at: at.clone(),
    }];
    lets.extend(retargeted.lets);
    let looping = Block {
        lets,
        tail: retargeted.tail,
    };
    // `loopCheck`: the renamed parameters and the kept ones are distinct, the
    // originals of the changed parameters are none of the kept ones, the kept
    // ones are locals, and the loop body does not mention a renamed
    // parameter.
    let kept: Vec<Ident> = kept_names(&recursive.positions)
        .into_iter()
        .cloned()
        .collect();
    let mut mentioned = Vec::new();
    occurrences_block(&looping, &mut mentioned);
    let lawful = distinct(&renamed)
        && distinct(&kept)
        && originals.iter().all(|name| !kept.contains(name))
        && kept.iter().all(|name| matches!(name, Ident::Local(_)))
        && renamed.iter().all(|name| !mentioned.contains(name));
    if !lawful {
        return None;
    }
    Some((
        parameters,
        Block {
            lets: vec![Let {
                pat: Pat::Bind(held.clone()),
                ty: None,
                value: start,
                at,
            }],
            tail: Expr::Loop {
                state: held,
                body: Box::new(looping),
            },
        },
    ))
}

/// The right-nested components of a pair, `count` of them.
fn unpack(state: &Expr, count: usize) -> Option<Vec<Expr>> {
    let mut parts = Vec::new();
    let mut rest = state;
    for _ in 1..count {
        let Expr::Pair(left, right, _) = rest else {
            return None;
        };
        parts.push(left.as_ref().clone());
        rest = right;
    }
    parts.push(rest.clone());
    Some(parts)
}

struct Restore<'a> {
    me: u64,
    positions: &'a [Position],
    changed: usize,
    state: Option<(Type, Type)>,
}

impl Restore<'_> {
    fn note(&mut self, state: &Type, done: &Type) -> Result<(), String> {
        match &self.state {
            None => {
                self.state = Some((state.clone(), done.clone()));
                Ok(())
            }
            Some((held_state, held_done)) if held_state == state && held_done == done => Ok(()),
            Some(_) => Err("the loop's results are of two types".to_owned()),
        }
    }

    fn expr(&mut self, tail: &Expr, lets: &[Let]) -> Result<Expr, String> {
        match tail {
            Expr::Construct {
                ctor: Ctor::Ok(state, done),
                args,
                at,
            } => {
                self.note(state, done)?;
                let [packed] = args.as_slice() else {
                    return Err("a continuing result holds one state".to_owned());
                };
                let mut next = unpack(packed, self.changed)
                    .ok_or("a state is not the pair of the changed parameters")?
                    .into_iter();
                let mut call = Vec::new();
                for (position, place) in self.positions.iter().zip(0..) {
                    call.push(match position {
                        Position::Changed => next.next().ok_or("a state is short")?,
                        Position::Kept(name) => Expr::Clone(name.clone(), at.clone()),
                        Position::Dropped => {
                            // A dropped argument is read, never run; it
                            // survives as the temporary the block binds for
                            // the operand of that position.
                            let from = lets
                                .len()
                                .checked_sub(self.positions.len())
                                .map(|first| &lets[first + place]);
                            match from {
                                Some(Let {
                                    pat: Pat::Bind(temp),
                                    ..
                                }) => Expr::Move(temp.clone(), at.clone()),
                                _ => {
                                    return Err(
                                        "the argument of a dropped parameter is not a temporary the block binds"
                                            .to_owned(),
                                    )
                                }
                            }
                        }
                    });
                }
                Ok(Expr::Call {
                    callee: Callee::Function(self.me),
                    args: call,
                    propagate: false,
                    at: at.clone(),
                })
            }
            Expr::Construct {
                ctor: Ctor::Err(state, done),
                args,
                ..
            } => {
                self.note(state, done)?;
                let [value] = args.as_slice() else {
                    return Err("an ending result holds one value".to_owned());
                };
                Ok(value.clone())
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
                at,
            } => Ok(Expr::If {
                condition: condition.clone(),
                then_branch: Box::new(self.block(then_branch)?),
                else_branch: Box::new(self.block(else_branch)?),
                at: at.clone(),
            }),
            Expr::Match {
                scrutinee,
                arms,
                at,
            } => {
                let mut restored = Vec::new();
                for (pattern, body) in arms {
                    restored.push((pattern.clone(), self.block(body)?));
                }
                Ok(Expr::Match {
                    scrutinee: scrutinee.clone(),
                    arms: restored,
                    at: at.clone(),
                })
            }
            Expr::Block(inner) => Ok(Expr::Block(Box::new(self.block(inner)?))),
            _ => Err("a tail of the loop is neither a continuation nor an end".to_owned()),
        }
    }

    fn block(&mut self, block: &Block) -> Result<Block, String> {
        Ok(Block {
            lets: block.lets.clone(),
            tail: self.expr(&block.tail, &block.lets)?,
        })
    }
}

/// The components of a right-nested tuple pattern or pair of reads.
fn flatten<T>(mut next: impl FnMut(&T) -> Option<(T, Option<T>)>, start: T) -> Option<Vec<T>> {
    let mut parts = Vec::new();
    let mut at = start;
    loop {
        let (head, rest) = next(&at)?;
        parts.push(head);
        match rest {
            Some(rest) => at = rest,
            None => return Some(parts),
        }
    }
}

/// The recursive body that function `me`, rendered with `parameters` and
/// `body`, is the loop form of: the shape [`form`] makes, read back, or the
/// reason it has none.
///
/// # Errors
/// The reason the body is not in the image of the rendering of loops.
pub fn invert(me: u64, parameters: &[(Pat, Type)], body: &Block) -> Result<Recursive, String> {
    let shape = "the body of a loop is not a holder bound to the state and a loop over it";
    let [entry] = body.lets.as_slice() else {
        return Err(shape.to_owned());
    };
    let (Pat::Bind(Ident::Holder(holder)), None) = (&entry.pat, &entry.ty) else {
        return Err(shape.to_owned());
    };
    let Expr::Loop {
        state: Ident::Holder(looped),
        body: looping,
    } = &body.tail
    else {
        return Err(shape.to_owned());
    };
    if looped != holder {
        return Err(
            "the loop runs over another holder than the one its state is bound to".to_owned(),
        );
    }
    let renamed: Vec<u64> = flatten(
        |expr: &Expr| match expr {
            Expr::Pair(left, right, _) => {
                Some((left.as_ref().clone(), Some(right.as_ref().clone())))
            }
            other => Some((other.clone(), None)),
        },
        entry.value.clone(),
    )
    .ok_or("the state is not a pair of reads")?
    .iter()
    .map(|read| match read {
        Expr::Move(Ident::Param(k), _) => Ok(*k),
        _ => Err("the state is not made of renamed parameters".to_owned()),
    })
    .collect::<Result<_, _>>()?;
    let [destructure, kept_lets @ ..] = looping.lets.as_slice() else {
        return Err("the loop body does not begin by taking the state apart".to_owned());
    };
    if !matches!(&destructure.value, Expr::Move(Ident::Holder(read), _) if read == holder)
        || destructure.ty.is_some()
    {
        return Err("the loop body does not begin by taking the state apart".to_owned());
    }
    let originals: Vec<Ident> = flatten(
        |pattern: &Pat| match pattern {
            Pat::Tuple(parts) if parts.len() == 2 => {
                Some((parts[0].clone(), Some(parts[1].clone())))
            }
            other => Some((other.clone(), None)),
        },
        destructure.pat.clone(),
    )
    .ok_or("the state is not taken apart into its parts")?
    .iter()
    .map(|part| match part {
        Pat::Bind(name) => Ok(name.clone()),
        _ => Err("a part of the state is not bound to a name".to_owned()),
    })
    .collect::<Result<_, _>>()?;
    if originals.len() != renamed.len() {
        return Err("the state and the pattern that takes it apart differ in number".to_owned());
    }
    let mut positions = Vec::new();
    let mut recursive_parameters = Vec::new();
    let mut state_types = Vec::new();
    let mut taken = 0;
    for (pattern, ty) in parameters {
        match pattern {
            Pat::Bind(Ident::Param(k)) if renamed.get(taken) == Some(k) => {
                positions.push(Position::Changed);
                recursive_parameters.push((Pat::Bind(originals[taken].clone()), ty.clone()));
                state_types.push(ty.clone());
                taken += 1;
            }
            Pat::Bind(name) => {
                positions.push(Position::Kept(name.clone()));
                recursive_parameters.push((pattern.clone(), ty.clone()));
            }
            Pat::Wild => {
                positions.push(Position::Dropped);
                recursive_parameters.push((pattern.clone(), ty.clone()));
            }
            _ => return Err("a parameter of a loop is neither a name nor a wildcard".to_owned()),
        }
    }
    if taken != renamed.len() {
        return Err("the state holds a parameter the function does not have".to_owned());
    }
    let state = state_types
        .iter()
        .rev()
        .cloned()
        .reduce(|right, left| Type::Pair(Box::new(left), Box::new(right)))
        .ok_or("a loop has no state")?;
    let mut restore = Restore {
        me,
        positions: &positions,
        changed: renamed.len(),
        state: None,
    };
    let restored = restore.block(&Block {
        lets: kept_lets.to_vec(),
        tail: looping.tail.clone(),
    })?;
    let (held_state, done) = restore.state.ok_or("a loop never continues or ends")?;
    if held_state != state {
        return Err(
            "the type of the state is not the pair of the types of the changed parameters"
                .to_owned(),
        );
    }
    Ok(Recursive {
        positions,
        renamed,
        holder: *holder,
        state,
        done,
        parameters: recursive_parameters,
        body: restored,
    })
}

/// Whether the loop form of the function rendered with `parameters` and
/// `body` is the form the library states of the recursive body [`invert`]
/// recovers from it.
///
/// # Errors
/// The reason the rendering is not in the image of the rendering of loops.
pub fn recover(me: u64, parameters: &[(Pat, Type)], body: &Block) -> Result<Recursive, String> {
    let recursive = invert(me, parameters, body)?;
    match form(me, &recursive) {
        Some((made_parameters, made_body))
            if made_parameters == parameters && made_body == *body =>
        {
            Ok(recursive)
        }
        Some(_) => {
            Err("the loop is not the loop form of the recursive body it restores to".to_owned())
        }
        None => Err(
            "the library would not state the recursive body the loop restores to as a loop"
                .to_owned(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::rust::ast::Lit;

    fn local(n: u64) -> Ident {
        Ident::Local(n)
    }

    /// `fn f(n, k) -> nat { if n == 0 { k } else { f(n - 1, k) } }`, whose
    /// first parameter the self call changes and whose second it keeps.
    fn countdown() -> Recursive {
        let at = origin();
        let call = Expr::Call {
            callee: Callee::Function(7),
            args: vec![
                Expr::Predecessor(local(0)),
                Expr::Clone(local(1), at.clone()),
            ],
            propagate: false,
            at: at.clone(),
        };
        Recursive {
            positions: vec![Position::Changed, Position::Kept(local(1))],
            renamed: vec![3],
            holder: 9,
            state: Type::Nat,
            done: Type::Nat,
            parameters: vec![
                (Pat::Bind(local(0)), Type::Nat),
                (Pat::Bind(local(1)), Type::Nat),
            ],
            body: Block::of(Expr::If {
                condition: Box::new(Expr::IsZero(local(0), at.clone())),
                then_branch: Box::new(Block::of(Expr::Clone(local(1), at.clone()))),
                else_branch: Box::new(Block::of(call)),
                at,
            }),
        }
    }

    #[test]
    fn a_loop_restores_to_the_body_it_is_the_form_of() {
        let recursive = countdown();
        let (parameters, body) = form(7, &recursive).expect("the form of a countdown");
        assert_eq!(parameters[0].0, Pat::Bind(Ident::Param(3)));
        assert_eq!(parameters[1].0, Pat::Bind(local(1)));
        assert_eq!(recover(7, &parameters, &body), Ok(recursive));
    }

    #[test]
    fn a_loop_outside_the_image_of_the_rendering_is_refused() {
        let (parameters, body) = form(7, &countdown()).expect("the form of a countdown");
        let looping = |body: &Block| match &body.tail {
            Expr::Loop { body, .. } => (**body).clone(),
            other => panic!("not a loop: {other:?}"),
        };
        let with = |changed: Block| Block {
            lets: body.lets.clone(),
            tail: Expr::Loop {
                state: Ident::Holder(9),
                body: Box::new(changed),
            },
        };
        let mut untaken = looping(&body);
        untaken.lets.remove(0);
        let mut rebound = looping(&body);
        let binding = rebound.lets[0].clone();
        rebound.lets.push(Let {
            pat: Pat::Bind(local(1)),
            ..binding
        });
        let mut unwrapped = looping(&body);
        if let Expr::If { else_branch, .. } = &mut unwrapped.tail {
            else_branch.tail = Expr::Lit(Lit::Unit, origin());
        }
        for (what, refused) in [
            ("a state no one takes apart", with(untaken)),
            ("a pattern that rebinds a kept parameter", with(rebound)),
            (
                "a tail that is neither a continuation nor an end",
                with(unwrapped),
            ),
        ] {
            assert!(recover(7, &parameters, &refused).is_err(), "{what}");
        }
        // The renderer's parameters, not the library's: a parameter the state
        // does not hold.
        let mut stray = parameters.clone();
        stray[1].0 = Pat::Bind(Ident::Param(4));
        assert!(recover(7, &stray, &body).is_err());
    }
}
