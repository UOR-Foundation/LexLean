//! The reference interpreter: a step-for-step transcription of the LexLean
//! evaluator `TargetSemantics.eval` (SPEC.md §17.14). It is not the
//! definition of the calculus. It computes each fixture's expected outcome,
//! and the fixture theorems ask the kernel to confirm that outcome against
//! the LexLean evaluator, so any divergence fails verification.

use std::cmp::Ordering;

use super::{Arm, Expr, IntKind, OrderingValue, Outcome, Prim, Program, Shape, Ty, Value};

/// A runtime value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rv {
    Unit,
    Bool(bool),
    Nat(u64),
    Int(i64),
    Fixed(IntKind, i128),
    Str(String),
    Bytes(Vec<u8>),
    Ord(Ordering),
    None,
    Some(Box<Rv>),
    Ok(Box<Rv>),
    Err(Box<Rv>),
    List(Vec<Rv>),
    Pair(Box<Rv>, Box<Rv>),
    Adt(u64, Vec<Rv>),
    Closure(u64, Vec<Rv>),
}

/// A runtime outcome; steps are charged exactly as the denotation charges
/// them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Out {
    Value(Rv, u64),
    Overflow(u64),
    Stuck,
    Exhausted,
}

enum Outs {
    Values(Vec<Rv>, u64),
    Overflow(u64),
    Stuck,
    Exhausted,
}

fn add_steps(outcome: Out, extra: u64) -> Out {
    match outcome {
        Out::Value(value, steps) => Out::Value(value, steps + extra),
        Out::Overflow(steps) => Out::Overflow(steps + extra),
        other => other,
    }
}

/// `weight`: one per node, and a string's characters or a byte string's
/// bytes.
fn weight(value: &Rv) -> u64 {
    match value {
        Rv::Str(text) => 1 + text.chars().count() as u64,
        Rv::Bytes(octets) => 1 + octets.len() as u64,
        Rv::Some(inner) | Rv::Ok(inner) | Rv::Err(inner) => 1 + weight(inner),
        Rv::List(items) | Rv::Adt(_, items) | Rv::Closure(_, items) => 1 + weights(items),
        Rv::Pair(left, right) => 1 + weight(left) + weight(right),
        Rv::Unit
        | Rv::Bool(_)
        | Rv::Nat(_)
        | Rv::Int(_)
        | Rv::Fixed(..)
        | Rv::Ord(_)
        | Rv::None => 1,
    }
}

fn weights(values: &[Rv]) -> u64 {
    values.iter().map(weight).sum()
}

/// `chargeResult`: a primitive's outcome charged `extra` steps and its
/// value's weight.
fn charge_result(outcome: Out, extra: u64) -> Out {
    match outcome {
        Out::Value(value, steps) => {
            let charged = steps + extra + weight(&value);
            Out::Value(value, charged)
        }
        Out::Overflow(steps) => Out::Overflow(steps + extra),
        other => other,
    }
}

fn nat_result(number: u128) -> Out {
    u64::try_from(number).map_or(Out::Overflow(0), |number| Out::Value(Rv::Nat(number), 0))
}

fn int_result(number: i128) -> Out {
    i64::try_from(number).map_or(Out::Overflow(0), |number| Out::Value(Rv::Int(number), 0))
}

fn fixed_value(kind: IntKind, number: i128) -> Option<Rv> {
    let (low, high) = kind.range();
    (low..=high)
        .contains(&number)
        .then_some(Rv::Fixed(kind, number))
}

fn option(value: Option<Rv>) -> Out {
    Out::Value(value.map_or(Rv::None, |value| Rv::Some(Box::new(value))), 0)
}

/// Wrap a mathematical result into `kind` by two's complement truncation.
fn wrap(kind: IntKind, number: i128) -> i128 {
    let bits = kind.bits();
    let modulus: i128 = 1 << bits;
    let truncated = number.rem_euclid(modulus);
    if kind.signed() && truncated >= modulus / 2 {
        truncated - modulus
    } else {
        truncated
    }
}

/// Lean's `String.toInt?` followed by the canonical-spelling check.
fn parse_decimal(text: &str) -> Option<Result<i128, ()>> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    let canonical = !digits.is_empty()
        && digits.bytes().all(|byte| byte.is_ascii_digit())
        && (digits == "0" || !digits.starts_with('0'))
        && text != "-0";
    if !canonical {
        return None;
    }
    // A canonical decimal beyond i128 is outside every realized integer.
    Some(text.parse::<i128>().map_err(|_| ()))
}

#[allow(clippy::too_many_lines)]
fn primitive(operation: &Prim, values: &[Rv]) -> Out {
    use Rv::{Bool, Bytes, Fixed, Int, List, Nat, Str};
    match (operation, values) {
        (Prim::NatAdd, [Nat(a), Nat(b)]) => nat_result(u128::from(*a) + u128::from(*b)),
        (Prim::NatSub, [Nat(a), Nat(b)]) => Out::Value(Nat(a.saturating_sub(*b)), 0),
        (Prim::NatMul, [Nat(a), Nat(b)]) => nat_result(u128::from(*a) * u128::from(*b)),
        (Prim::NatQuot, [Nat(a), Nat(b), Nat(z)]) => {
            Out::Value(Nat(if *b == 0 { *z } else { a / b }), 0)
        }
        (Prim::NatRem, [Nat(a), Nat(b), Nat(z)]) => {
            Out::Value(Nat(if *b == 0 { *z } else { a % b }), 0)
        }
        (Prim::NatEq, [Nat(a), Nat(b)]) => Out::Value(Bool(a == b), 0),
        (Prim::NatLe, [Nat(a), Nat(b)]) => Out::Value(Bool(a <= b), 0),
        (Prim::NatLt, [Nat(a), Nat(b)]) => Out::Value(Bool(a < b), 0),
        (Prim::IntAdd, [Int(a), Int(b)]) => int_result(i128::from(*a) + i128::from(*b)),
        (Prim::IntSub, [Int(a), Int(b)]) => int_result(i128::from(*a) - i128::from(*b)),
        (Prim::IntMul, [Int(a), Int(b)]) => int_result(i128::from(*a) * i128::from(*b)),
        (Prim::IntNeg, [Int(a)]) => int_result(-i128::from(*a)),
        (Prim::IntQuot, [Int(a), Int(b), Int(z)]) => {
            if *b == 0 {
                int_result(i128::from(*z))
            } else {
                int_result(i128::from(*a) / i128::from(*b))
            }
        }
        (Prim::IntRem, [Int(a), Int(b), Int(z)]) => {
            if *b == 0 {
                int_result(i128::from(*z))
            } else {
                int_result(i128::from(*a) % i128::from(*b))
            }
        }
        (Prim::CheckedAdd, [Fixed(k, a), Fixed(l, b)]) if k == l => option(fixed_value(*k, a + b)),
        (Prim::CheckedSub, [Fixed(k, a), Fixed(l, b)]) if k == l => option(fixed_value(*k, a - b)),
        // Two 64-bit magnitudes can multiply past `i128`; such a product is
        // outside every width.
        (Prim::CheckedMul, [Fixed(k, a), Fixed(l, b)]) if k == l => option(
            a.checked_mul(*b)
                .and_then(|product| fixed_value(*k, product)),
        ),
        (Prim::CheckedQuot, [Fixed(k, a), Fixed(l, b)]) if k == l => option(if *b == 0 {
            None
        } else {
            fixed_value(*k, a / b)
        }),
        (Prim::CheckedNeg, [Fixed(k, a)]) if k.signed() => option(fixed_value(*k, -a)),
        (Prim::BitAnd, [Fixed(k, a), Fixed(l, b)]) if k == l => {
            Out::Value(Fixed(*k, wrap(*k, a & b)), 0)
        }
        (Prim::BitOr, [Fixed(k, a), Fixed(l, b)]) if k == l => {
            Out::Value(Fixed(*k, wrap(*k, a | b)), 0)
        }
        (Prim::BitXor, [Fixed(k, a), Fixed(l, b)]) if k == l => {
            Out::Value(Fixed(*k, wrap(*k, a ^ b)), 0)
        }
        (Prim::BitNot, [Fixed(k, a)]) => Out::Value(Fixed(*k, wrap(*k, !a)), 0),
        (Prim::ShiftLeft, [Fixed(k, a), Fixed(IntKind::U32, amount)]) => {
            option((*amount < i128::from(k.bits())).then(|| Fixed(*k, wrap(*k, a << amount))))
        }
        (Prim::ShiftRight, [Fixed(k, a), Fixed(IntKind::U32, amount)]) => {
            // Signed kinds shift arithmetically, as Lean's `IntN.shiftRight`.
            option((*amount < i128::from(k.bits())).then(|| Fixed(*k, wrap(*k, a >> amount))))
        }
        // Equality is defined on the closed scalars only, each against its
        // own kind; anything else is stuck, as in the denotation.
        (Prim::Equal, [Nat(a), Nat(b)]) => Out::Value(Bool(a == b), 0),
        (Prim::Equal, [Bool(a), Bool(b)]) => Out::Value(Bool(a == b), 0),
        (Prim::Equal, [Fixed(k, a), Fixed(l, b)]) if k == l => Out::Value(Bool(a == b), 0),
        (Prim::Equal, [Str(a), Str(b)]) => Out::Value(Bool(a == b), 0),
        (Prim::Equal, [Bytes(a), Bytes(b)]) => Out::Value(Bool(a == b), 0),
        (Prim::Equal, [Rv::Ord(a), Rv::Ord(b)]) => Out::Value(Bool(a == b), 0),
        (Prim::BoolNot, [Bool(a)]) => Out::Value(Bool(!a), 0),
        (Prim::BoolAnd, [Bool(a), Bool(b)]) => Out::Value(Bool(*a && *b), 0),
        (Prim::BoolOr, [Bool(a), Bool(b)]) => Out::Value(Bool(*a || *b), 0),
        (Prim::Append, [List(a), List(b)]) => {
            Out::Value(List(a.iter().chain(b).cloned().collect()), 0)
        }
        (Prim::Append, [Bytes(a), Bytes(b)]) => {
            Out::Value(Bytes(a.iter().chain(b).copied().collect()), 0)
        }
        (Prim::Length, [List(a)]) => nat_result(a.len() as u128),
        (Prim::Length, [Bytes(a)]) => nat_result(a.len() as u128),
        (Prim::Length, [Str(a)]) => nat_result(a.chars().count() as u128),
        (Prim::Index, [List(a), Nat(i)]) => {
            option(usize::try_from(*i).ok().and_then(|i| a.get(i)).cloned())
        }
        (Prim::Index, [Bytes(a), Nat(i)]) => option(
            usize::try_from(*i)
                .ok()
                .and_then(|i| a.get(i))
                .map(|byte| Fixed(IntKind::U8, i128::from(*byte))),
        ),
        (Prim::Slice, [List(a), Nat(start), Nat(count)]) => {
            option(slice(a, *start, *count).map(|part| List(part.to_vec())))
        }
        (Prim::Slice, [Bytes(a), Nat(start), Nat(count)]) => {
            option(slice(a, *start, *count).map(|part| Bytes(part.to_vec())))
        }
        (Prim::Utf8Encode, [Str(a)]) => Out::Value(Bytes(a.as_bytes().to_vec()), 0),
        (Prim::Utf8Decode, [Bytes(a)]) => option(String::from_utf8(a.clone()).ok().map(Str)),
        (Prim::CompareBytes, [Bytes(a), Bytes(b)]) => Out::Value(Rv::Ord(a.cmp(b)), 0),
        (Prim::SplitExact, [Str(text), Str(delimiter), Fixed(IntKind::U32, maximum)]) => {
            let fields: Vec<Rv> = text
                .split(delimiter.as_str())
                .map(|field| Str(field.to_owned()))
                .collect();
            option(
                (!delimiter.is_empty() && fields.len() as i128 <= *maximum).then_some(List(fields)),
            )
        }
        (Prim::Join, [List(items), Str(delimiter)]) => {
            let mut texts = Vec::new();
            for item in items {
                let Str(text) = item else { return Out::Stuck };
                texts.push(text.as_str());
            }
            Out::Value(Str(texts.join(delimiter)), 0)
        }
        (Prim::FormatDecimal, [Int(a)]) => Out::Value(Str(a.to_string()), 0),
        (Prim::FormatDecimal, [Fixed(_, a)]) => Out::Value(Str(a.to_string()), 0),
        (Prim::Convert { target }, [Int(a)]) => option(fixed_value(*target, i128::from(*a))),
        (Prim::Convert { target }, [Fixed(_, a)]) => option(fixed_value(*target, *a)),
        (Prim::Compare, [left, right]) => {
            compare(left, right).map_or(Out::Stuck, |order| Out::Value(Rv::Ord(order), 0))
        }
        (Prim::ParseDecimal { target: Ty::Int }, [Str(text)]) => match parse_decimal(text) {
            None => Out::Value(Rv::None, 0),
            Some(Ok(number)) => i64::try_from(number).map_or(Out::Overflow(0), |number| {
                Out::Value(Rv::Some(Box::new(Int(number))), 0)
            }),
            Some(Err(())) => Out::Overflow(0),
        },
        (
            Prim::ParseDecimal {
                target: Ty::Fixed { width },
            },
            [Str(text)],
        ) => match parse_decimal(text) {
            Some(Ok(number)) => option(fixed_value(*width, number)),
            None | Some(Err(())) => Out::Value(Rv::None, 0),
        },
        _ => Out::Stuck,
    }
}

/// `compareValue`: the key order, lexicographic through pairs and lists. A
/// decided prefix ends the comparison, so later components are never
/// examined, exactly as in the denotation.
fn compare(left: &Rv, right: &Rv) -> Option<Ordering> {
    match (left, right) {
        (Rv::Nat(a), Rv::Nat(b)) => Some(a.cmp(b)),
        (Rv::Int(a), Rv::Int(b)) => Some(a.cmp(b)),
        (Rv::Fixed(k, a), Rv::Fixed(l, b)) if k == l => Some(a.cmp(b)),
        (Rv::Bool(a), Rv::Bool(b)) => Some(a.cmp(b)),
        (Rv::Str(a), Rv::Str(b)) => Some(a.cmp(b)),
        (Rv::Pair(a, b), Rv::Pair(c, d)) => match compare(a, c)? {
            Ordering::Equal => compare(b, d),
            decided => Some(decided),
        },
        (Rv::List(left), Rv::List(right)) => {
            for (a, b) in left.iter().zip(right) {
                match compare(a, b)? {
                    Ordering::Equal => {}
                    decided => return Some(decided),
                }
            }
            Some(left.len().cmp(&right.len()))
        }
        _ => None,
    }
}

fn slice<T>(items: &[T], start: u64, count: u64) -> Option<&[T]> {
    let end = u128::from(start) + u128::from(count);
    if end > items.len() as u128 {
        return None;
    }
    let start = usize::try_from(start).ok()?;
    let end = usize::try_from(end).ok()?;
    Some(&items[start..end])
}

fn destruct(shape: Shape, value: &Rv) -> Option<Vec<Rv>> {
    match (shape, value) {
        (Shape::None, Rv::None) | (Shape::Unit, Rv::Unit) => Some(Vec::new()),
        (Shape::Some, Rv::Some(inner))
        | (Shape::Ok, Rv::Ok(inner))
        | (Shape::Error, Rv::Err(inner)) => Some(vec![inner.as_ref().clone()]),
        (Shape::Nil, Rv::List(items)) if items.is_empty() => Some(Vec::new()),
        (Shape::Cons, Rv::List(items)) if !items.is_empty() => {
            Some(vec![items[0].clone(), Rv::List(items[1..].to_vec())])
        }
        (Shape::Zero, Rv::Nat(0)) => Some(Vec::new()),
        (Shape::Succ, Rv::Nat(number)) if *number > 0 => Some(vec![Rv::Nat(number - 1)]),
        (Shape::Pair, Rv::Pair(left, right)) => {
            Some(vec![left.as_ref().clone(), right.as_ref().clone()])
        }
        (Shape::True, Rv::Bool(true))
        | (Shape::False, Rv::Bool(false))
        | (Shape::Lt, Rv::Ord(Ordering::Less))
        | (Shape::Eq, Rv::Ord(Ordering::Equal))
        | (Shape::Gt, Rv::Ord(Ordering::Greater)) => Some(Vec::new()),
        (Shape::Adt { constructor }, Rv::Adt(tag, fields)) if *tag == constructor => {
            Some(fields.clone())
        }
        _ => None,
    }
}

fn construct(shape: Shape, values: Vec<Rv>) -> Out {
    let mut values = values;
    match (shape, values.len()) {
        (Shape::None, 0) => Out::Value(Rv::None, 0),
        (Shape::Some, 1) => Out::Value(Rv::Some(Box::new(values.remove(0))), 0),
        (Shape::Ok, 1) => Out::Value(Rv::Ok(Box::new(values.remove(0))), 0),
        (Shape::Error, 1) => Out::Value(Rv::Err(Box::new(values.remove(0))), 0),
        (Shape::Nil, 0) => Out::Value(Rv::List(Vec::new()), 0),
        (Shape::Cons, 2) => match values.remove(1) {
            Rv::List(mut items) => {
                items.insert(0, values.remove(0));
                Out::Value(Rv::List(items), 0)
            }
            _ => Out::Stuck,
        },
        (Shape::Zero, 0) => Out::Value(Rv::Nat(0), 0),
        (Shape::Succ, 1) => match values[0] {
            Rv::Nat(number) => nat_result(u128::from(number) + 1),
            _ => Out::Stuck,
        },
        (Shape::Pair, 2) => {
            let right = values.remove(1);
            Out::Value(Rv::Pair(Box::new(values.remove(0)), Box::new(right)), 0)
        }
        (Shape::True, 0) => Out::Value(Rv::Bool(true), 0),
        (Shape::False, 0) => Out::Value(Rv::Bool(false), 0),
        (Shape::Unit, 0) => Out::Value(Rv::Unit, 0),
        (Shape::Lt, 0) => Out::Value(Rv::Ord(Ordering::Less), 0),
        (Shape::Eq, 0) => Out::Value(Rv::Ord(Ordering::Equal), 0),
        (Shape::Gt, 0) => Out::Value(Rv::Ord(Ordering::Greater), 0),
        (Shape::Adt { constructor }, _) => Out::Value(Rv::Adt(constructor, values), 0),
        _ => Out::Stuck,
    }
}

type Env = Vec<(u64, Rv)>;

/// The innermost binding of `name` and the number of bindings examined
/// before it (`depth`).
fn lookup(env: &Env, name: u64) -> Option<(Rv, u64)> {
    env.iter()
        .rev()
        .enumerate()
        .find(|(_, (bound, _))| *bound == name)
        .map(|(depth, (_, value))| (value.clone(), depth as u64))
}

fn bind_all(names: &[u64], values: Vec<Rv>, env: &Env) -> Option<Env> {
    if names.len() != values.len() {
        return None;
    }
    let mut out = env.clone();
    out.extend(names.iter().copied().zip(values));
    Some(out)
}

struct Interp<'a> {
    program: &'a Program,
}

impl Interp<'_> {
    fn call(&self, remaining: u64, function: u64, values: Vec<Rv>, steps: u64) -> Out {
        let Some(callee) = usize::try_from(function)
            .ok()
            .and_then(|index| self.program.functions.get(index))
        else {
            return Out::Stuck;
        };
        let Some(env) = bind_all(&callee.parameters, values, &Vec::new()) else {
            return Out::Stuck;
        };
        add_steps(self.eval(remaining, &env, &callee.body), steps)
    }

    fn propagate(outcome: Out, then: impl FnOnce(Rv, u64) -> Out) -> Out {
        match outcome {
            Out::Value(value, steps) => then(value, steps),
            Out::Overflow(steps) => Out::Overflow(steps + 1),
            other => other,
        }
    }

    fn propagate_list(outcomes: Outs, then: impl FnOnce(Vec<Rv>, u64) -> Out) -> Out {
        match outcomes {
            Outs::Values(values, steps) => then(values, steps),
            Outs::Overflow(steps) => Out::Overflow(steps + 1),
            Outs::Stuck => Out::Stuck,
            Outs::Exhausted => Out::Exhausted,
        }
    }

    fn eval(&self, fuel: u64, env: &Env, expr: &Expr) -> Out {
        let Some(remaining) = fuel.checked_sub(1) else {
            return Out::Exhausted;
        };
        match expr {
            // A literal is charged its weight each time it is read, as a
            // primitive's operands are, so a decoded constant is accounted
            // by its size; a scalar still costs one step (§17.14 *Cost*).
            Expr::Value { value, .. } => to_runtime(value).map_or(Out::Stuck, |value| {
                let steps = weight(&value);
                Out::Value(value, steps)
            }),
            Expr::Var { name } => {
                lookup(env, *name).map_or(Out::Stuck, |(value, depth)| Out::Value(value, 1 + depth))
            }
            Expr::Let {
                name, bound, body, ..
            } => Self::propagate(self.eval(remaining, env, bound), |value, steps| {
                let mut extended = env.clone();
                extended.push((*name, value));
                add_steps(self.eval(remaining, &extended, body), steps + 1)
            }),
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => Self::propagate(
                self.eval(remaining, env, condition),
                |value, steps| match value {
                    Rv::Bool(flag) => add_steps(
                        self.eval(remaining, env, if flag { then_branch } else { else_branch }),
                        steps + 1,
                    ),
                    _ => Out::Stuck,
                },
            ),
            Expr::Match {
                scrutinee, arms, ..
            } => Self::propagate(self.eval(remaining, env, scrutinee), |value, steps| {
                add_steps(self.eval_arms(remaining, env, &value, arms), steps + 1)
            }),
            Expr::Build {
                shape, operands, ..
            } => Self::propagate_list(self.eval_list(remaining, env, operands), |values, steps| {
                add_steps(construct(*shape, values), steps + 1)
            }),
            Expr::Call { function, operands } => {
                Self::propagate_list(self.eval_list(remaining, env, operands), |values, steps| {
                    self.call(remaining, *function, values, steps + 1)
                })
            }
            Expr::Closure { function, captures } => {
                Self::propagate_list(self.eval_list(remaining, env, captures), |values, steps| {
                    Out::Value(Rv::Closure(*function, values), steps + 1)
                })
            }
            Expr::Apply { target, operands } => {
                Self::propagate(self.eval(remaining, env, target), |value, target_steps| {
                    let Rv::Closure(function, captured) = value else {
                        return Out::Stuck;
                    };
                    Self::propagate_list(
                        self.eval_list(remaining, env, operands),
                        |values, steps| {
                            let passed = captured.len() as u64;
                            let mut all = captured;
                            all.extend(values);
                            self.call(remaining, function, all, target_steps + steps + 1 + passed)
                        },
                    )
                })
            }
            Expr::Prim {
                operation,
                operands,
            } => Self::propagate_list(self.eval_list(remaining, env, operands), |values, steps| {
                charge_result(primitive(operation, &values), steps + 1 + weights(&values))
            }),
            Expr::First { value } => Self::propagate(
                self.eval(remaining, env, value),
                |value, steps| match value {
                    Rv::Pair(left, _) => Out::Value(*left, steps + 1),
                    _ => Out::Stuck,
                },
            ),
            Expr::Second { value } => Self::propagate(
                self.eval(remaining, env, value),
                |value, steps| match value {
                    Rv::Pair(_, right) => Out::Value(*right, steps + 1),
                    _ => Out::Stuck,
                },
            ),
            Expr::Field { value, index } => Self::propagate(
                self.eval(remaining, env, value),
                |value, steps| match value {
                    Rv::Adt(_, fields) => usize::try_from(*index)
                        .ok()
                        .and_then(|position| fields.get(position))
                        .map_or(Out::Stuck, |selected| {
                            Out::Value(selected.clone(), steps + 1 + index)
                        }),
                    _ => Out::Stuck,
                },
            ),
        }
    }

    fn eval_list(&self, fuel: u64, env: &Env, exprs: &[Expr]) -> Outs {
        let Some(remaining) = fuel.checked_sub(1) else {
            return Outs::Exhausted;
        };
        let Some((head, rest)) = exprs.split_first() else {
            return Outs::Values(Vec::new(), 0);
        };
        match self.eval(remaining, env, head) {
            Out::Value(value, head_steps) => match self.eval_list(remaining, env, rest) {
                Outs::Values(mut values, rest_steps) => {
                    values.insert(0, value);
                    Outs::Values(values, head_steps + rest_steps)
                }
                Outs::Overflow(rest_steps) => Outs::Overflow(head_steps + rest_steps),
                other => other,
            },
            Out::Overflow(steps) => Outs::Overflow(steps),
            Out::Stuck => Outs::Stuck,
            Out::Exhausted => Outs::Exhausted,
        }
    }

    fn eval_arms(&self, fuel: u64, env: &Env, value: &Rv, arms: &[Arm]) -> Out {
        let Some(remaining) = fuel.checked_sub(1) else {
            return Out::Exhausted;
        };
        let Some((arm, rest)) = arms.split_first() else {
            return Out::Stuck;
        };
        // Each arm tried is one step.
        match destruct(arm.shape, value) {
            None => add_steps(self.eval_arms(remaining, env, value, rest), 1),
            Some(fields) => match bind_all(&arm.binders, fields, env) {
                None => Out::Stuck,
                Some(extended) => add_steps(self.eval(remaining, &extended, &arm.body), 1),
            },
        }
    }
}

/// A literal as a runtime value; `None` for a literal with no runtime form.
#[must_use]
pub fn to_runtime(value: &Value) -> Option<Rv> {
    let number = |text: &str| text.parse::<i128>().ok();
    Some(match value {
        Value::Unit => Rv::Unit,
        Value::Bool { value } => Rv::Bool(*value),
        Value::Nat { value } => Rv::Nat(value.parse().ok()?),
        Value::Int { value } => Rv::Int(value.parse().ok()?),
        Value::U8 { value } => fixed_value(IntKind::U8, number(value)?)?,
        Value::U16 { value } => fixed_value(IntKind::U16, number(value)?)?,
        Value::U32 { value } => fixed_value(IntKind::U32, number(value)?)?,
        Value::U64 { value } => fixed_value(IntKind::U64, number(value)?)?,
        Value::I8 { value } => fixed_value(IntKind::I8, number(value)?)?,
        Value::I16 { value } => fixed_value(IntKind::I16, number(value)?)?,
        Value::I32 { value } => fixed_value(IntKind::I32, number(value)?)?,
        Value::I64 { value } => fixed_value(IntKind::I64, number(value)?)?,
        Value::String { value } => Rv::Str(value.clone()),
        Value::Bytes { hex } => Rv::Bytes(
            (0..hex.len())
                .step_by(2)
                .map(|at| u8::from_str_radix(hex.get(at..at + 2)?, 16).ok())
                .collect::<Option<Vec<u8>>>()?,
        ),
        Value::Ordering { value } => Rv::Ord(match value {
            OrderingValue::Lt => Ordering::Less,
            OrderingValue::Eq => Ordering::Equal,
            OrderingValue::Gt => Ordering::Greater,
        }),
        Value::None => Rv::None,
        Value::Some { value } => Rv::Some(Box::new(to_runtime(value)?)),
        Value::Ok { value } => Rv::Ok(Box::new(to_runtime(value)?)),
        Value::Error { value } => Rv::Err(Box::new(to_runtime(value)?)),
        Value::List { items } => Rv::List(items.iter().map(to_runtime).collect::<Option<_>>()?),
        Value::Pair { left, right } => {
            Rv::Pair(Box::new(to_runtime(left)?), Box::new(to_runtime(right)?))
        }
        Value::Adt {
            constructor,
            fields,
        } => Rv::Adt(
            *constructor,
            fields.iter().map(to_runtime).collect::<Option<_>>()?,
        ),
        Value::Closure { function, captures } => Rv::Closure(
            *function,
            captures.iter().map(to_runtime).collect::<Option<_>>()?,
        ),
    })
}

/// A runtime value in the exact JSON form.
#[must_use]
pub fn from_runtime(value: &Rv) -> Value {
    match value {
        Rv::Unit => Value::Unit,
        Rv::Bool(value) => Value::Bool { value: *value },
        Rv::Nat(value) => Value::Nat {
            value: value.to_string(),
        },
        Rv::Int(value) => Value::Int {
            value: value.to_string(),
        },
        Rv::Fixed(kind, number) => {
            let value = number.to_string();
            match kind {
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
        Rv::Str(value) => Value::String {
            value: value.clone(),
        },
        Rv::Bytes(bytes) => Value::Bytes {
            hex: bytes.iter().map(|byte| format!("{byte:02x}")).collect(),
        },
        Rv::Ord(ordering) => Value::Ordering {
            value: match ordering {
                Ordering::Less => OrderingValue::Lt,
                Ordering::Equal => OrderingValue::Eq,
                Ordering::Greater => OrderingValue::Gt,
            },
        },
        Rv::None => Value::None,
        Rv::Some(inner) => Value::Some {
            value: Box::new(from_runtime(inner)),
        },
        Rv::Ok(inner) => Value::Ok {
            value: Box::new(from_runtime(inner)),
        },
        Rv::Err(inner) => Value::Error {
            value: Box::new(from_runtime(inner)),
        },
        Rv::List(items) => Value::List {
            items: items.iter().map(from_runtime).collect(),
        },
        Rv::Pair(left, right) => Value::Pair {
            left: Box::new(from_runtime(left)),
            right: Box::new(from_runtime(right)),
        },
        Rv::Adt(constructor, fields) => Value::Adt {
            constructor: *constructor,
            fields: fields.iter().map(from_runtime).collect(),
        },
        Rv::Closure(function, captures) => Value::Closure {
            function: *function,
            captures: captures.iter().map(from_runtime).collect(),
        },
    }
}

/// Run function `entry` on `arguments` with `fuel`, as `TargetSemantics.run`.
#[must_use]
pub fn run(program: &Program, fuel: u64, entry: u64, arguments: &[Value]) -> Outcome {
    let Some(values) = arguments
        .iter()
        .map(to_runtime)
        .collect::<Option<Vec<Rv>>>()
    else {
        return Outcome::Stuck;
    };
    let interp = Interp { program };
    let Some(function) = usize::try_from(entry)
        .ok()
        .and_then(|index| program.functions.get(index))
    else {
        return Outcome::Stuck;
    };
    let Some(env) = bind_all(&function.parameters, values, &Vec::new()) else {
        return Outcome::Stuck;
    };
    match interp.eval(fuel, &env, &function.body) {
        Out::Value(value, steps) => Outcome::Value {
            value: from_runtime(&value),
            steps,
        },
        Out::Overflow(steps) => Outcome::Overflow { steps },
        Out::Stuck => Outcome::Stuck,
        Out::Exhausted => Outcome::Exhausted,
    }
}
