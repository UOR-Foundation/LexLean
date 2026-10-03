//! Static validity of target programs (SPEC.md §17.14): closed types, bound
//! locals, exact arities, exhaustive and irredundant matches, literals within
//! their realization, and well-typed primitives. A valid program never gets
//! stuck under the denotation; the only failures it can exhibit are
//! `overflow` and fuel exhaustion.

use std::collections::BTreeSet;

use super::{Adt, Arm, Expr, Function, IntKind, Prim, Program, Shape, Ty, Value};

/// The inclusive bounds of the realized `nat` and `int`.
const NAT_MAX: u128 = u64::MAX as u128;

pub(crate) struct Checker<'a> {
    pub(crate) program: &'a Program,
}

fn fail<T>(reason: impl Into<String>) -> Result<T, String> {
    Err(reason.into())
}

fn expect(actual: &Ty, expected: &Ty, what: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        fail(format!("{what} has type {actual:?}, expected {expected:?}"))
    }
}

/// Parse a canonical decimal (`0`, or no leading zero, optional `-`, never
/// `-0`).
fn decimal(text: &str) -> Result<i128, String> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    let canonical = !digits.is_empty()
        && digits.bytes().all(|byte| byte.is_ascii_digit())
        && (digits == "0" || !digits.starts_with('0'))
        && text != "-0";
    if !canonical {
        return fail(format!("`{text}` is not a canonical decimal"));
    }
    text.parse::<i128>()
        .map_err(|_| format!("`{text}` is outside every realized integer"))
}

impl Checker<'_> {
    fn adt(&self, index: u64) -> Result<&Adt, String> {
        usize::try_from(index)
            .ok()
            .and_then(|index| self.program.adts.get(index))
            .ok_or_else(|| format!("ADT {index} is not declared"))
    }

    fn function(&self, index: u64) -> Result<&Function, String> {
        usize::try_from(index)
            .ok()
            .and_then(|index| self.program.functions.get(index))
            .ok_or_else(|| format!("function {index} is not declared"))
    }

    fn ty(&self, ty: &Ty) -> Result<(), String> {
        match ty {
            Ty::Unit
            | Ty::Bool
            | Ty::Nat
            | Ty::Int
            | Ty::Fixed { .. }
            | Ty::String
            | Ty::Bytes
            | Ty::Ordering => Ok(()),
            Ty::Option { value } | Ty::List { element: value } => self.ty(value),
            Ty::Result { ok, error } => {
                self.ty(ok)?;
                self.ty(error)
            }
            Ty::Pair { left, right } => {
                self.ty(left)?;
                self.ty(right)
            }
            Ty::Adt { index } => self.adt(*index).map(|_| ()),
            Ty::Fn { parameters, result } => {
                if parameters.is_empty() {
                    return fail("a function type takes at least one parameter");
                }
                parameters
                    .iter()
                    .try_for_each(|parameter| self.ty(parameter))?;
                self.ty(result)
            }
        }
    }

    /// A literal's conformance to its type. Closures and orderings have no
    /// literal form: they arise only from evaluation.
    fn value(&self, value: &Value, ty: &Ty) -> Result<(), String> {
        let fixed = |text: &str, kind: IntKind| -> Result<(), String> {
            let (low, high) = kind.range();
            let number = decimal(text)?;
            if (low..=high).contains(&number) {
                Ok(())
            } else {
                fail(format!("{text} is outside {}", kind.name()))
            }
        };
        match (value, ty) {
            (Value::Unit, Ty::Unit)
            | (Value::Bool { .. }, Ty::Bool)
            | (Value::String { .. }, Ty::String) => Ok(()),
            (Value::Nat { value }, Ty::Nat) => {
                let number = decimal(value)?;
                if number < 0 || number.unsigned_abs() > NAT_MAX {
                    return fail(format!(
                        "nat literal {value} is outside the 64-bit realization"
                    ));
                }
                Ok(())
            }
            (Value::Int { value }, Ty::Int) => fixed(value, IntKind::I64),
            (Value::U8 { value }, Ty::Fixed { width: IntKind::U8 }) => fixed(value, IntKind::U8),
            (
                Value::U16 { value },
                Ty::Fixed {
                    width: IntKind::U16,
                },
            ) => fixed(value, IntKind::U16),
            (
                Value::U32 { value },
                Ty::Fixed {
                    width: IntKind::U32,
                },
            ) => fixed(value, IntKind::U32),
            (
                Value::U64 { value },
                Ty::Fixed {
                    width: IntKind::U64,
                },
            ) => fixed(value, IntKind::U64),
            (Value::I8 { value }, Ty::Fixed { width: IntKind::I8 }) => fixed(value, IntKind::I8),
            (
                Value::I16 { value },
                Ty::Fixed {
                    width: IntKind::I16,
                },
            ) => fixed(value, IntKind::I16),
            (
                Value::I32 { value },
                Ty::Fixed {
                    width: IntKind::I32,
                },
            ) => fixed(value, IntKind::I32),
            (
                Value::I64 { value },
                Ty::Fixed {
                    width: IntKind::I64,
                },
            ) => fixed(value, IntKind::I64),
            (Value::Bytes { hex }, Ty::Bytes) => {
                if hex.len() % 2 == 0
                    && hex
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                {
                    Ok(())
                } else {
                    fail(format!(
                        "byte literal `{hex}` is not even-length lowercase hexadecimal"
                    ))
                }
            }
            (Value::None, Ty::Option { .. }) => Ok(()),
            (Value::Some { value }, Ty::Option { value: inner }) => self.value(value, inner),
            (Value::Ok { value }, Ty::Result { ok, .. }) => self.value(value, ok),
            (Value::Error { value }, Ty::Result { error, .. }) => self.value(value, error),
            (Value::List { items }, Ty::List { element }) => {
                items.iter().try_for_each(|item| self.value(item, element))
            }
            (
                Value::Pair { left, right },
                Ty::Pair {
                    left: lt,
                    right: rt,
                },
            ) => {
                self.value(left, lt)?;
                self.value(right, rt)
            }
            (
                Value::Adt {
                    constructor,
                    fields,
                },
                Ty::Adt { index },
            ) => {
                let field_types = self.constructor(*index, *constructor)?;
                if fields.len() != field_types.len() {
                    return fail(format!(
                        "constructor {constructor} of ADT {index} takes {} fields, the literal has {}",
                        field_types.len(),
                        fields.len()
                    ));
                }
                fields
                    .iter()
                    .zip(field_types)
                    .try_for_each(|(field, ty)| self.value(field, ty))
            }
            (Value::Ordering { .. }, Ty::Ordering) => Ok(()),
            (Value::Closure { .. }, _) => {
                fail("a closure has no literal form; build it with `closure`")
            }
            (value, ty) => fail(format!("the literal {value:?} does not have type {ty:?}")),
        }
    }

    fn constructor(&self, adt: u64, constructor: u64) -> Result<&Vec<Ty>, String> {
        let declaration = self.adt(adt)?;
        usize::try_from(constructor)
            .ok()
            .and_then(|index| declaration.constructors.get(index))
            .ok_or_else(|| format!("ADT {adt} has no constructor {constructor}"))
    }

    /// The field types a shape binds from a value of `ty`.
    pub(crate) fn shape_fields(&self, shape: Shape, ty: &Ty) -> Result<Vec<Ty>, String> {
        Ok(match (shape, ty) {
            (Shape::None, Ty::Option { .. })
            | (Shape::Nil, Ty::List { .. })
            | (Shape::Zero, Ty::Nat)
            | (Shape::True | Shape::False, Ty::Bool)
            | (Shape::Unit, Ty::Unit)
            | (Shape::Lt | Shape::Eq | Shape::Gt, Ty::Ordering) => Vec::new(),
            (Shape::Some, Ty::Option { value }) => vec![value.as_ref().clone()],
            (Shape::Ok, Ty::Result { ok, .. }) => vec![ok.as_ref().clone()],
            (Shape::Error, Ty::Result { error, .. }) => vec![error.as_ref().clone()],
            (Shape::Cons, Ty::List { element }) => vec![element.as_ref().clone(), ty.clone()],
            (Shape::Succ, Ty::Nat) => vec![Ty::Nat],
            (Shape::Pair, Ty::Pair { left, right }) => {
                vec![left.as_ref().clone(), right.as_ref().clone()]
            }
            (Shape::Adt { constructor }, Ty::Adt { index }) => {
                self.constructor(*index, constructor)?.clone()
            }
            (shape, ty) => return fail(format!("shape {shape:?} does not belong to type {ty:?}")),
        })
    }

    /// Every shape that covers a type, for exhaustiveness.
    fn shapes(&self, ty: &Ty) -> Result<Vec<Shape>, String> {
        Ok(match ty {
            Ty::Option { .. } => vec![Shape::None, Shape::Some],
            Ty::Result { .. } => vec![Shape::Ok, Shape::Error],
            Ty::List { .. } => vec![Shape::Nil, Shape::Cons],
            Ty::Nat => vec![Shape::Zero, Shape::Succ],
            Ty::Bool => vec![Shape::True, Shape::False],
            Ty::Pair { .. } => vec![Shape::Pair],
            Ty::Unit => vec![Shape::Unit],
            Ty::Ordering => vec![Shape::Lt, Shape::Eq, Shape::Gt],
            Ty::Adt { index } => (0..self.adt(*index)?.constructors.len() as u64)
                .map(|constructor| Shape::Adt { constructor })
                .collect(),
            other => return fail(format!("a value of type {other:?} cannot be matched")),
        })
    }

    fn exprs(&self, exprs: &[Expr], scope: &mut Vec<(u64, Ty)>) -> Result<Vec<Ty>, String> {
        exprs.iter().map(|expr| self.expr(expr, scope)).collect()
    }

    fn operands(
        &self,
        exprs: &[Expr],
        expected: &[Ty],
        scope: &mut Vec<(u64, Ty)>,
        what: &str,
    ) -> Result<(), String> {
        if exprs.len() != expected.len() {
            return fail(format!(
                "{what} takes {} operand(s), received {}",
                expected.len(),
                exprs.len()
            ));
        }
        for (index, (expr, ty)) in exprs.iter().zip(expected).enumerate() {
            let actual = self.expr(expr, scope)?;
            expect(&actual, ty, &format!("{what} operand {index}"))?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn expr(&self, expr: &Expr, scope: &mut Vec<(u64, Ty)>) -> Result<Ty, String> {
        match expr {
            Expr::Value { ty, value } => {
                self.ty(ty)?;
                self.value(value, ty)?;
                Ok(ty.clone())
            }
            Expr::Var { name } => scope
                .iter()
                .rev()
                .find(|(bound, _)| bound == name)
                .map(|(_, ty)| ty.clone())
                .ok_or_else(|| format!("local {name} is unbound")),
            Expr::Let {
                name,
                ty,
                bound,
                body,
            } => {
                self.ty(ty)?;
                let actual = self.expr(bound, scope)?;
                expect(&actual, ty, &format!("the value bound to {name}"))?;
                scope.push((*name, ty.clone()));
                let result = self.expr(body, scope);
                scope.pop();
                result
            }
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => {
                expect(&self.expr(condition, scope)?, &Ty::Bool, "a condition")?;
                let then_ty = self.expr(then_branch, scope)?;
                let else_ty = self.expr(else_branch, scope)?;
                expect(&else_ty, &then_ty, "the else branch")?;
                Ok(then_ty)
            }
            Expr::Match {
                ty,
                scrutinee,
                arms,
            } => {
                self.ty(ty)?;
                let scrutinee_ty = self.expr(scrutinee, scope)?;
                let required = self.shapes(&scrutinee_ty)?;
                let mut covered = BTreeSet::new();
                for arm in arms {
                    self.arm(arm, &scrutinee_ty, ty, scope, &mut covered)?;
                }
                if let Some(missing) = required.iter().find(|shape| !covered.contains(*shape)) {
                    return fail(format!(
                        "the match is not exhaustive: {missing:?} is not covered"
                    ));
                }
                Ok(ty.clone())
            }
            Expr::Build {
                shape,
                ty,
                operands,
            } => {
                self.ty(ty)?;
                let fields = match (shape, ty) {
                    (Shape::Zero, Ty::Nat) => Vec::new(),
                    _ => self.shape_fields(*shape, ty)?,
                };
                self.operands(operands, &fields, scope, &format!("constructor {shape:?}"))?;
                Ok(ty.clone())
            }
            Expr::Call { function, operands } => {
                let callee = self.function(*function)?;
                self.operands(
                    operands,
                    &callee.types,
                    scope,
                    &format!("function {function}"),
                )?;
                Ok(callee.result.clone())
            }
            Expr::Closure { function, captures } => {
                let callee = self.function(*function)?;
                if captures.len() >= callee.types.len() {
                    return fail(format!(
                        "closure of function {function} captures {} of its {} parameters; at least one must remain",
                        captures.len(),
                        callee.types.len()
                    ));
                }
                let (captured, remaining) = callee.types.split_at(captures.len());
                self.operands(
                    captures,
                    captured,
                    scope,
                    &format!("closure of function {function}"),
                )?;
                Ok(Ty::Fn {
                    parameters: remaining.to_vec(),
                    result: Box::new(callee.result.clone()),
                })
            }
            Expr::Apply { target, operands } => {
                let Ty::Fn { parameters, result } = self.expr(target, scope)? else {
                    return fail("only a function value can be applied");
                };
                self.operands(operands, &parameters, scope, "an application")?;
                Ok(*result)
            }
            Expr::Prim {
                operation,
                operands,
            } => {
                let types = self.exprs(operands, scope)?;
                prim_type(operation, &types)
            }
            Expr::First { value } => match self.expr(value, scope)? {
                Ty::Pair { left, .. } => Ok(*left),
                other => fail(format!("`first` of {other:?}")),
            },
            Expr::Second { value } => match self.expr(value, scope)? {
                Ty::Pair { right, .. } => Ok(*right),
                other => fail(format!("`second` of {other:?}")),
            },
            Expr::Field { value, index } => {
                let Ty::Adt { index: adt } = self.expr(value, scope)? else {
                    return fail("`field` requires an ADT value");
                };
                let declaration = self.adt(adt)?;
                if declaration.constructors.len() != 1 {
                    return fail(format!(
                        "`field` requires an ADT with exactly one constructor; ADT {adt} has {}",
                        declaration.constructors.len()
                    ));
                }
                usize::try_from(*index)
                    .ok()
                    .and_then(|position| declaration.constructors[0].get(position))
                    .cloned()
                    .ok_or_else(|| format!("ADT {adt} has no field {index}"))
            }
        }
    }

    fn arm(
        &self,
        arm: &Arm,
        scrutinee: &Ty,
        result: &Ty,
        scope: &mut Vec<(u64, Ty)>,
        covered: &mut BTreeSet<Shape>,
    ) -> Result<(), String> {
        let fields = self.shape_fields(arm.shape, scrutinee)?;
        if arm.binders.len() != fields.len() {
            return fail(format!(
                "shape {:?} binds {} value(s), the arm names {}",
                arm.shape,
                fields.len(),
                arm.binders.len()
            ));
        }
        let unique: BTreeSet<u64> = arm.binders.iter().copied().collect();
        if unique.len() != arm.binders.len() {
            return fail(format!("shape {:?} binds the same name twice", arm.shape));
        }
        if !covered.insert(arm.shape) {
            return fail(format!("the arm for {:?} is unreachable", arm.shape));
        }
        let depth = scope.len();
        scope.extend(arm.binders.iter().copied().zip(fields));
        let body = self.expr(&arm.body, scope);
        scope.truncate(depth);
        expect(&body?, result, "an arm")
    }
}

/// Whether a type has the fixed total order of map keys (§17.12): integers
/// numerically, `false` before `true`, strings by scalar sequence, and pairs
/// lexicographically.
fn orderable(ty: &Ty) -> bool {
    match ty {
        Ty::Nat | Ty::Int | Ty::Fixed { .. } | Ty::Bool | Ty::String => true,
        Ty::Pair { left, right } => orderable(left) && orderable(right),
        _ => false,
    }
}

/// The result type of `operation` applied to operands of `types`.
///
/// # Errors
///
/// Returns the reason the primitive does not apply to `types`.
pub fn primitive_type(operation: &Prim, types: &[Ty]) -> Result<Ty, String> {
    prim_type(operation, types)
}

/// The result type of a primitive at its operand types.
#[allow(clippy::too_many_lines)]
fn prim_type(operation: &Prim, types: &[Ty]) -> Result<Ty, String> {
    let fixed = |ty: &Ty| matches!(ty, Ty::Fixed { .. });
    let option = |ty: Ty| Ty::Option {
        value: Box::new(ty),
    };
    let exactly = |expected: &[Ty], result: Ty| -> Result<Ty, String> {
        if types == expected {
            Ok(result)
        } else {
            fail(format!(
                "primitive {operation:?} takes {expected:?}, received {types:?}"
            ))
        }
    };
    let first = types.first().cloned().unwrap_or(Ty::Unit);
    match operation {
        Prim::NatAdd | Prim::NatSub | Prim::NatMul => exactly(&[Ty::Nat, Ty::Nat], Ty::Nat),
        Prim::NatQuot | Prim::NatRem => exactly(&[Ty::Nat, Ty::Nat, Ty::Nat], Ty::Nat),
        Prim::NatEq | Prim::NatLe | Prim::NatLt => exactly(&[Ty::Nat, Ty::Nat], Ty::Bool),
        Prim::IntAdd | Prim::IntSub | Prim::IntMul => exactly(&[Ty::Int, Ty::Int], Ty::Int),
        Prim::IntNeg => exactly(&[Ty::Int], Ty::Int),
        Prim::IntQuot | Prim::IntRem => exactly(&[Ty::Int, Ty::Int, Ty::Int], Ty::Int),
        Prim::CheckedAdd | Prim::CheckedSub | Prim::CheckedMul | Prim::CheckedQuot
            if fixed(&first) =>
        {
            exactly(&[first.clone(), first.clone()], option(first))
        }
        Prim::CheckedNeg if matches!(first, Ty::Fixed { width } if width.signed()) => {
            exactly(std::slice::from_ref(&first), option(first.clone()))
        }
        Prim::BitAnd | Prim::BitOr | Prim::BitXor if fixed(&first) => {
            exactly(&[first.clone(), first.clone()], first)
        }
        Prim::BitNot if fixed(&first) => exactly(std::slice::from_ref(&first), first.clone()),
        Prim::ShiftLeft | Prim::ShiftRight if fixed(&first) => exactly(
            &[
                first.clone(),
                Ty::Fixed {
                    width: IntKind::U32,
                },
            ],
            option(first),
        ),
        Prim::Equal
            if matches!(
                first,
                Ty::Nat | Ty::Bool | Ty::Fixed { .. } | Ty::String | Ty::Bytes | Ty::Ordering
            ) =>
        {
            exactly(&[first.clone(), first], Ty::Bool)
        }
        Prim::BoolNot => exactly(&[Ty::Bool], Ty::Bool),
        Prim::BoolAnd | Prim::BoolOr => exactly(&[Ty::Bool, Ty::Bool], Ty::Bool),
        Prim::Append if matches!(first, Ty::List { .. } | Ty::Bytes) => {
            exactly(&[first.clone(), first.clone()], first)
        }
        Prim::Length if matches!(first, Ty::List { .. } | Ty::Bytes | Ty::String) => {
            exactly(&[first], Ty::Nat)
        }
        Prim::Index => match &first {
            Ty::List { element } => {
                exactly(&[first.clone(), Ty::Nat], option(element.as_ref().clone()))
            }
            Ty::Bytes => exactly(
                &[Ty::Bytes, Ty::Nat],
                option(Ty::Fixed { width: IntKind::U8 }),
            ),
            other => fail(format!("primitive Index of {other:?}")),
        },
        Prim::Slice if matches!(first, Ty::List { .. } | Ty::Bytes) => {
            exactly(&[first.clone(), Ty::Nat, Ty::Nat], option(first))
        }
        Prim::Utf8Encode => exactly(&[Ty::String], Ty::Bytes),
        Prim::Utf8Decode => exactly(&[Ty::Bytes], option(Ty::String)),
        Prim::CompareBytes => exactly(&[Ty::Bytes, Ty::Bytes], Ty::Ordering),
        Prim::SplitExact => exactly(
            &[
                Ty::String,
                Ty::String,
                Ty::Fixed {
                    width: IntKind::U32,
                },
            ],
            option(Ty::List {
                element: Box::new(Ty::String),
            }),
        ),
        Prim::Join => exactly(
            &[
                Ty::List {
                    element: Box::new(Ty::String),
                },
                Ty::String,
            ],
            Ty::String,
        ),
        Prim::FormatDecimal if matches!(first, Ty::Int | Ty::Fixed { .. }) => {
            exactly(&[first], Ty::String)
        }
        Prim::Convert { target } if matches!(first, Ty::Int | Ty::Fixed { .. }) => {
            exactly(&[first], option(Ty::Fixed { width: *target }))
        }
        Prim::Compare if orderable(&first) => exactly(&[first.clone(), first], Ty::Ordering),
        Prim::ParseDecimal { target } if matches!(target, Ty::Int | Ty::Fixed { .. }) => {
            exactly(&[Ty::String], option(target.clone()))
        }
        other => fail(format!("primitive {other:?} does not apply to {types:?}")),
    }
}

/// Primitive applications whose LexLean meaning a `reflexivity` proof
/// cannot establish from a module file under pinned Lean 4.32.1: Lean's
/// elaborator checks such a proof by definitional unfolding before the
/// kernel sees it, and does not unfold the definitions these reach
/// (`String.splitOn`, `String.toInt?`, `ByteArray.toList`, `String.toUTF8`,
/// `String.intercalate`, `Nat.repr`), which recurse by well-founded
/// recursion or are not exposed to module files. LexLean's proof language
/// has no kernel-only decision, so a fixture that applies one is decided by
/// Lean's evaluator instead (§17.14). Byte equality is not among them: the
/// kernel decides it.
pub const KERNEL_OPAQUE: [&str; 6] = [
    "split_exact",
    "parse_decimal",
    "format_decimal",
    "compare_bytes",
    "utf8_encode",
    "join",
];

/// The [`KERNEL_OPAQUE`] applications a valid program contains.
///
/// # Errors
///
/// Returns the reason the program is invalid.
pub fn kernel_opaque(program: &Program) -> Result<BTreeSet<&'static str>, String> {
    fn walk(
        checker: &Checker<'_>,
        expr: &Expr,
        scope: &mut Vec<(u64, Ty)>,
        out: &mut BTreeSet<&'static str>,
    ) -> Result<(), String> {
        let mut children: Vec<&Expr> = Vec::new();
        match expr {
            Expr::Value { .. } | Expr::Var { .. } => {}
            Expr::Let {
                name,
                ty,
                bound,
                body,
            } => {
                walk(checker, bound, scope, out)?;
                scope.push((*name, ty.clone()));
                let result = walk(checker, body, scope, out);
                scope.pop();
                return result;
            }
            Expr::Cond {
                condition,
                then_branch,
                else_branch,
            } => children.extend([
                condition.as_ref(),
                then_branch.as_ref(),
                else_branch.as_ref(),
            ]),
            Expr::Match {
                scrutinee, arms, ..
            } => {
                walk(checker, scrutinee, scope, out)?;
                let scrutinee_ty = checker.expr(scrutinee, scope)?;
                for arm in arms {
                    let fields = checker.shape_fields(arm.shape, &scrutinee_ty)?;
                    let depth = scope.len();
                    scope.extend(arm.binders.iter().copied().zip(fields));
                    let result = walk(checker, &arm.body, scope, out);
                    scope.truncate(depth);
                    result?;
                }
                return Ok(());
            }
            Expr::Build { operands, .. }
            | Expr::Call { operands, .. }
            | Expr::Closure {
                captures: operands, ..
            } => children.extend(operands),
            Expr::Apply { target, operands } => {
                children.push(target);
                children.extend(operands);
            }
            Expr::Prim {
                operation,
                operands,
            } => {
                let opaque = match operation {
                    Prim::SplitExact => Some("split_exact"),
                    Prim::ParseDecimal { .. } => Some("parse_decimal"),
                    Prim::FormatDecimal => Some("format_decimal"),
                    Prim::CompareBytes => Some("compare_bytes"),
                    Prim::Utf8Encode => Some("utf8_encode"),
                    Prim::Join => Some("join"),
                    _ => None,
                };
                out.extend(opaque);
                children.extend(operands);
            }
            Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
                children.push(value)
            }
        }
        children
            .into_iter()
            .try_for_each(|child| walk(checker, child, scope, out))
    }
    check(program)?;
    let checker = Checker { program };
    let mut out = BTreeSet::new();
    for function in &program.functions {
        let mut scope: Vec<(u64, Ty)> = function
            .parameters
            .iter()
            .copied()
            .zip(function.types.iter().cloned())
            .collect();
        walk(&checker, &function.body, &mut scope, &mut out)?;
    }
    Ok(out)
}

/// Check a whole program.
///
/// # Errors
///
/// Returns the first violated rule, naming the function.
pub fn check(program: &Program) -> Result<(), String> {
    if program.spec != super::PROGRAM_SPEC {
        return fail(format!("target program has spec `{}`", program.spec));
    }
    let checker = Checker { program };
    for (index, adt) in program.adts.iter().enumerate() {
        if adt.constructors.is_empty() {
            return fail(format!("ADT {index} has no constructor"));
        }
        adt.constructors
            .iter()
            .flatten()
            .try_for_each(|ty| checker.ty(ty))
            .map_err(|reason| format!("ADT {index}: {reason}"))?;
    }
    for (index, function) in program.functions.iter().enumerate() {
        let context = |reason: String| format!("function {index}: {reason}");
        if function.parameters.len() != function.types.len() {
            return Err(context(format!(
                "{} parameter names for {} parameter types",
                function.parameters.len(),
                function.types.len()
            )));
        }
        let unique: BTreeSet<u64> = function.parameters.iter().copied().collect();
        if unique.len() != function.parameters.len() {
            return Err(context("a parameter name repeats".to_owned()));
        }
        function
            .types
            .iter()
            .try_for_each(|ty| checker.ty(ty))
            .map_err(context)?;
        checker.ty(&function.result).map_err(context)?;
        let mut scope: Vec<(u64, Ty)> = function
            .parameters
            .iter()
            .copied()
            .zip(function.types.iter().cloned())
            .collect();
        let body = checker.expr(&function.body, &mut scope).map_err(context)?;
        expect(&body, &function.result, "the body").map_err(context)?;
    }
    Ok(())
}

/// Check that `arguments` are a valid input to function `entry`.
///
/// # Errors
///
/// Returns the first violated rule.
pub fn check_arguments(program: &Program, entry: u64, arguments: &[Value]) -> Result<(), String> {
    let checker = Checker { program };
    let function = checker.function(entry)?;
    if arguments.len() != function.types.len() {
        return fail(format!(
            "function {entry} takes {} argument(s), received {}",
            function.types.len(),
            arguments.len()
        ));
    }
    arguments
        .iter()
        .zip(&function.types)
        .try_for_each(|(argument, ty)| checker.value(argument, ty))
}
