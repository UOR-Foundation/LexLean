//! The language-1.2 widening conversions (SPEC.md §17.12, *Widening
//! conversions*).
//!
//! `to_int` takes a value of any fixed-width integer type to the `Int` it
//! denotes, and `to_nat` an unsigned one to its `Nat`. Linking elaborates each
//! to ordinary terms, as it does a checked model application (§17.12 rule 8):
//! the operand is bound once by a `let`, each of its bits is read by
//! `bit_and` and `equal`, and the weights of the set bits are summed exactly.
//! Lean, the production analysis, the lowering, and the certificates read only
//! the elaboration, so every one of them meets constructs it already
//! realizes and proves, and no decimal text is involved.
//!
//! The conversion could instead be a primitive of its own all the way down;
//! the calculus has no fixed-width-to-`Int` primitive, though, and adding one
//! would need its own lemmas in the preservation library. Elaborating keeps
//! the realization and its proofs exactly those of the constructs it uses.

use std::collections::{BTreeMap, BTreeSet};

use super::model::{CheckStatement, Elaboration};
use super::reasoning::Formula;
use super::{
    branch_binder_types, elimination_binder_types, infer_term, Environment, SemanticDeclaration,
    SemanticFailure, SemanticInteger, SemanticParameter, SemanticPrimitive, SemanticProof,
    SemanticTerm, SemanticType,
};

type Locals = BTreeMap<String, SemanticType>;

/// The elaboration of one declaration so far: how many binders it has
/// generated, and which conversions it has elaborated.
#[derive(Default)]
struct State {
    binders: usize,
    found: BTreeSet<&'static str>,
}

/// Elaborate every widening conversion of a module's elaboration: its
/// ordinary declarations and the Lean statements generated from source
/// terms. Generated obligations keep their source terms: they are what the
/// canonical document states, and no backend renders them as Lean.
pub(super) fn elaboration(
    elaboration: &mut Elaboration,
    env: &Environment<'_>,
) -> Result<(), SemanticFailure> {
    for declaration in elaboration.lowered.iter_mut().flatten() {
        let found = self::declaration(declaration, env)?;
        if !found.is_empty() {
            elaboration
                .widened
                .insert(declaration.name().to_owned(), found);
        }
    }
    for check in elaboration.checks.iter_mut().flatten() {
        let mut counter = State::default();
        match &mut check.statement {
            CheckStatement::Helper {
                helper: _,
                arguments,
            } => {
                for argument in arguments {
                    term(argument, &Locals::new(), env, &mut counter)?;
                }
            }
            CheckStatement::Tensor { .. }
            | CheckStatement::Lines { .. }
            | CheckStatement::Bytes { .. } => {}
        }
    }
    for theorem in elaboration.theorems.iter_mut().flatten() {
        let locals = parameters(&theorem.parameters);
        let mut counter = State::default();
        formula(&mut theorem.statement, &locals, env, &mut counter)?;
    }
    Ok(())
}

fn formula(
    formula: &mut Formula,
    locals: &Locals,
    env: &Environment<'_>,
    counter: &mut State,
) -> Result<(), SemanticFailure> {
    match formula {
        Formula::Term { term: value } => term(value, locals, env, counter),
        Formula::Helper {
            helper: _,
            arguments,
        } => {
            for argument in arguments {
                term(argument, locals, env, counter)?;
            }
            Ok(())
        }
        Formula::Implies {
            premise,
            conclusion,
        } => {
            self::formula(premise, locals, env, counter)?;
            self::formula(conclusion, locals, env, counter)
        }
        Formula::True => Ok(()),
    }
}

fn parameters(parameters: &[SemanticParameter]) -> Locals {
    parameters
        .iter()
        .map(|parameter| (parameter.name.clone(), parameter.r#type.clone()))
        .collect()
}

/// Elaborate every widening conversion of one elaborated declaration. Each
/// generated binder is `__widen<n>`, numbered within the declaration; no
/// source name begins with an underscore, so none is captured.
fn declaration(
    declaration: &mut SemanticDeclaration,
    env: &Environment<'_>,
) -> Result<BTreeSet<&'static str>, SemanticFailure> {
    let mut counter = State::default();
    elaborate(declaration, env, &mut counter)?;
    Ok(counter.found)
}

fn elaborate(
    declaration: &mut SemanticDeclaration,
    env: &Environment<'_>,
    counter: &mut State,
) -> Result<(), SemanticFailure> {
    match declaration {
        SemanticDeclaration::Definition {
            parameters: binders,
            body,
            termination,
            ..
        } => {
            let locals = parameters(binders);
            term(body, &locals, env, counter)?;
            if let Some(termination) = termination {
                term(&mut termination.measure, &locals, env, counter)?;
            }
            Ok(())
        }
        SemanticDeclaration::Theorem {
            parameters: binders,
            statement,
            proof: evidence,
            ..
        } => {
            let locals = parameters(binders);
            term(statement, &locals, env, counter)?;
            self::proof(evidence, &locals, env, counter)
        }
        SemanticDeclaration::Instance { fields, .. } => {
            for field in fields {
                term(&mut field.value, &Locals::new(), env, counter)?;
            }
            Ok(())
        }
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Inductive { .. } => Ok(()),
        // Linking replaces each of these by the declarations it elaborates
        // to, so none is ever an elaborated declaration itself.
        SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. }
        | SemanticDeclaration::Tokenizer { .. }
        | SemanticDeclaration::ContextWindow { .. }
        | SemanticDeclaration::TextGenerator { .. }
        | SemanticDeclaration::TextClassifier { .. } => {
            let mut widens = false;
            super::declaration_terms(declaration, &mut |node| {
                widens |= matches!(
                    node,
                    SemanticTerm::Primitive { operation, .. } if operation.widening()
                );
            });
            if widens {
                Err(format!(
                    "internal: `{}` reached the widening elaboration unelaborated",
                    declaration.name()
                )
                .into())
            } else {
                Ok(())
            }
        }
    }
}

fn proof(
    proof: &mut SemanticProof,
    locals: &Locals,
    env: &Environment<'_>,
    counter: &mut State,
) -> Result<(), SemanticFailure> {
    let (scrutinee, branches, induction) = match proof {
        SemanticProof::Reflexivity
        | SemanticProof::Decide
        | SemanticProof::Congruence
        | SemanticProof::LinearArithmetic { .. }
        | SemanticProof::Simplify { .. }
        | SemanticProof::BooleanReflection { .. } => return Ok(()),
        SemanticProof::Apply { arguments, .. } | SemanticProof::Law { arguments, .. } => {
            for argument in arguments {
                term(argument, locals, env, counter)?;
            }
            return Ok(());
        }
        SemanticProof::Constructor { branches } => {
            for branch in branches {
                self::proof(branch, locals, env, counter)?;
            }
            return Ok(());
        }
        SemanticProof::Cases {
            scrutinee,
            branches,
        } => (scrutinee, branches, false),
        SemanticProof::Induction {
            scrutinee,
            generalizing: _,
            branches,
        } => (scrutinee, branches, true),
    };
    let expected = elimination_binder_types(scrutinee, induction, locals, env)?;
    for branch in branches {
        let mut nested = locals.clone();
        if let Some(types) = expected.get(&branch.constructor) {
            for (binder, ty) in branch.binders.iter().zip(types) {
                nested.insert(binder.clone(), ty.clone());
            }
        }
        self::proof(&mut branch.proof, &nested, env, counter)?;
    }
    Ok(())
}

/// Elaborate every widening conversion of `term`, innermost first, under
/// `locals`, the types of the locals in scope.
fn term(
    term: &mut SemanticTerm,
    locals: &Locals,
    env: &Environment<'_>,
    counter: &mut State,
) -> Result<(), SemanticFailure> {
    let each = |children: &mut [&mut SemanticTerm], counter: &mut State| {
        children
            .iter_mut()
            .try_for_each(|child| self::term(child, locals, env, counter))
    };
    match term {
        SemanticTerm::Var { .. }
        | SemanticTerm::Nat { .. }
        | SemanticTerm::Integer { .. }
        | SemanticTerm::String { .. }
        | SemanticTerm::Bytes { .. }
        | SemanticTerm::Bool { .. }
        | SemanticTerm::Unit
        | SemanticTerm::Nil { .. }
        | SemanticTerm::InstanceValue { .. }
        | SemanticTerm::FunctionRef { .. } => Ok(()),
        SemanticTerm::Primitive {
            operation,
            arguments,
            result: _,
        } => {
            for argument in arguments.iter_mut() {
                self::term(argument, locals, env, counter)?;
            }
            if !operation.widening() {
                return Ok(());
            }
            let operation = *operation;
            let [operand] = arguments.as_mut_slice() else {
                return Err(format!(
                    "internal: primitive {operation:?} reached its elaboration with {} operands",
                    arguments.len()
                )
                .into());
            };
            let source = infer_term(operand, locals, env)?.ok_or_else(|| {
                format!("primitive {operation:?} requires a statically typed operand")
            })?;
            let value = std::mem::replace(operand, SemanticTerm::Unit);
            *term = widen(operation, &source, value, counter)?;
            Ok(())
        }
        SemanticTerm::Constructor { arguments, .. }
        | SemanticTerm::Call { arguments, .. }
        | SemanticTerm::CheckedApply { arguments, .. } => {
            each(&mut arguments.iter_mut().collect::<Vec<_>>(), counter)
        }
        SemanticTerm::Record { fields, .. } => each(
            &mut fields
                .iter_mut()
                .map(|field| &mut field.value)
                .collect::<Vec<_>>(),
            counter,
        ),
        SemanticTerm::Cons { head, tail } => each(&mut [head.as_mut(), tail.as_mut()], counter),
        SemanticTerm::Eq { left, right }
        | SemanticTerm::Le { left, right }
        | SemanticTerm::Lt { left, right }
        | SemanticTerm::Add { left, right }
        | SemanticTerm::Beq { left, right }
        | SemanticTerm::Ble { left, right }
        | SemanticTerm::Blt { left, right }
        | SemanticTerm::And { left, right }
        | SemanticTerm::PropAnd { left, right }
        | SemanticTerm::Or { left, right }
        | SemanticTerm::Iff { left, right }
        | SemanticTerm::Pair { left, right } => each(&mut [left.as_mut(), right.as_mut()], counter),
        SemanticTerm::Implies {
            premise,
            conclusion,
        } => each(&mut [premise.as_mut(), conclusion.as_mut()], counter),
        SemanticTerm::Project { value, .. }
        | SemanticTerm::Not { value }
        | SemanticTerm::First { value }
        | SemanticTerm::Second { value } => each(&mut [value.as_mut()], counter),
        SemanticTerm::If {
            condition,
            then_value,
            else_value,
        } => each(
            &mut [condition.as_mut(), then_value.as_mut(), else_value.as_mut()],
            counter,
        ),
        SemanticTerm::Match {
            scrutinee,
            branches,
        } => {
            self::term(scrutinee, locals, env, counter)?;
            let scrutinee_type = infer_term(scrutinee, locals, env)?;
            for branch in branches {
                let mut nested = locals.clone();
                if let Some(scrutinee_type) = &scrutinee_type {
                    let types = branch_binder_types(scrutinee_type, branch, env)?;
                    for (binder, ty) in branch.binders.iter().zip(types) {
                        nested.insert(binder.clone(), ty);
                    }
                }
                self::term(&mut branch.body, &nested, env, counter)?;
            }
            Ok(())
        }
        SemanticTerm::Forall { binder, body } => {
            let mut nested = locals.clone();
            nested.insert(binder.name.clone(), binder.r#type.clone());
            self::term(body, &nested, env, counter)
        }
        SemanticTerm::Let {
            binder,
            value,
            body,
        } => {
            self::term(value, locals, env, counter)?;
            let mut nested = locals.clone();
            nested.insert(binder.name.clone(), binder.r#type.clone());
            self::term(body, &nested, env, counter)
        }
        SemanticTerm::Lambda {
            parameters: binders,
            body,
            ..
        } => {
            let mut nested = locals.clone();
            nested.extend(parameters(binders));
            self::term(body, &nested, env, counter)
        }
        SemanticTerm::Apply {
            function,
            arguments,
        } => {
            let mut children = vec![function.as_mut()];
            children.extend(arguments.iter_mut());
            each(&mut children, counter)
        }
        SemanticTerm::MapLiteral { entries, .. } => each(
            &mut entries
                .iter_mut()
                .flat_map(|entry| [&mut entry.key, &mut entry.value])
                .collect::<Vec<_>>(),
            counter,
        ),
        SemanticTerm::SetLiteral { elements, .. } => {
            each(&mut elements.iter_mut().collect::<Vec<_>>(), counter)
        }
        SemanticTerm::GraphLiteral { nodes, edges, .. } => {
            let mut children: Vec<&mut SemanticTerm> = nodes.iter_mut().collect();
            children.extend(
                edges
                    .iter_mut()
                    .flat_map(|edge| [&mut edge.source, &mut edge.target]),
            );
            each(&mut children, counter)
        }
    }
}

/// The ordinary term `operation` of `value`, a value of the fixed-width
/// type `source`, means.
///
/// The operand is bound once, so it is evaluated once whatever it is. Bit
/// `i` of a `w`-bit value weighs `2^i`, except that the top bit of a signed
/// value weighs `-2^(w-1)` (two's complement); a bit is set exactly when the
/// value masked by the bit's own pattern is that pattern. The weights of the
/// set bits are summed as a balanced tree, most significant bit first, so the
/// term's depth grows with the logarithm of the width, and every literal the
/// term writes is within the 64-bit range of the production targets:
///
/// - over `Int`, `a + b` is `subtract a (negate b)`, the integer kernels'
///   own addition (§17.12, rule 4 under *Models*). The negative weight of a
///   signed top bit is always the leftmost leaf and so never negated, and a
///   partial sum of the other weights is at most `2^63 - 1`, so neither the
///   negation nor the subtraction can leave the 64-bit range unless the value
///   itself does;
/// - the weight `2^63` of an unsigned 64-bit value's top bit, which no
///   64-bit integer literal spells, is `2^62 + 2^62`; it leaves the range
///   exactly when that bit is set, which is exactly when the value does;
/// - over `Nat`, the weights are added by the natural-number addition, and
///   every weight of an unsigned 64-bit value is below `2^64`.
fn widen(
    operation: SemanticPrimitive,
    source: &SemanticType,
    value: SemanticTerm,
    counter: &mut State,
) -> Result<SemanticTerm, SemanticFailure> {
    let (representation, signed, width) = fixed(source).ok_or_else(|| {
        format!("internal: primitive {operation:?} reached its elaboration with {source}")
    })?;
    let natural = match operation {
        SemanticPrimitive::ToInt => false,
        SemanticPrimitive::ToNat if !signed => true,
        other => {
            return Err(format!(
                "internal: primitive {other:?} of {source} is not a widening conversion"
            )
            .into())
        }
    };
    let binder = format!("__widen{}", counter.binders);
    counter.binders += 1;
    counter.found.insert(if natural {
        "primitive.to_nat"
    } else {
        "primitive.to_int"
    });
    let local = || SemanticTerm::Var {
        name: binder.clone(),
    };
    let pattern = |value: String| SemanticTerm::Integer {
        representation,
        value,
    };
    let zero = || {
        if natural {
            nat("0".to_owned())
        } else {
            int("0".to_owned())
        }
    };
    let leaves: Vec<SemanticTerm> = (0..width)
        .rev()
        .map(|bit| {
            let top = bit == width - 1;
            let (mask, weight) = if top && signed {
                let least = -(1_i128 << bit);
                (least.to_string(), int(least.to_string()))
            } else {
                let power = 1_u128 << bit;
                let weight = if natural {
                    nat(power.to_string())
                } else if bit == 63 {
                    let half = (1_u128 << 62).to_string();
                    int_add(int(half.clone()), int(half))
                } else {
                    int(power.to_string())
                };
                (power.to_string(), weight)
            };
            let masked = primitive(
                SemanticPrimitive::BitAnd,
                vec![local(), pattern(mask.clone())],
                source.clone(),
            );
            SemanticTerm::If {
                condition: Box::new(primitive(
                    SemanticPrimitive::Equal,
                    vec![masked, pattern(mask)],
                    SemanticType::Bool,
                )),
                then_value: Box::new(weight),
                else_value: Box::new(zero()),
            }
        })
        .collect();
    let sum = balanced(leaves, &|left, right| {
        if natural {
            SemanticTerm::Add {
                left: Box::new(left),
                right: Box::new(right),
            }
        } else {
            int_add(left, right)
        }
    });
    Ok(SemanticTerm::Let {
        binder: SemanticParameter {
            name: binder.clone(),
            r#type: source.clone(),
        },
        value: Box::new(value),
        body: Box::new(sum),
    })
}

/// The representation, signedness, and width of a fixed-width type.
const fn fixed(ty: &SemanticType) -> Option<(SemanticInteger, bool, u32)> {
    Some(match ty {
        SemanticType::UInt8 => (SemanticInteger::UInt8, false, 8),
        SemanticType::Int8 => (SemanticInteger::Int8, true, 8),
        SemanticType::UInt16 => (SemanticInteger::UInt16, false, 16),
        SemanticType::Int16 => (SemanticInteger::Int16, true, 16),
        SemanticType::UInt32 => (SemanticInteger::UInt32, false, 32),
        SemanticType::Int32 => (SemanticInteger::Int32, true, 32),
        SemanticType::UInt64 => (SemanticInteger::UInt64, false, 64),
        SemanticType::Int64 => (SemanticInteger::Int64, true, 64),
        _ => return None,
    })
}

/// The sum of `terms`, a nonempty list, as a balanced tree in their order.
fn balanced(
    mut terms: Vec<SemanticTerm>,
    add: &dyn Fn(SemanticTerm, SemanticTerm) -> SemanticTerm,
) -> SemanticTerm {
    if terms.len() == 1 {
        return terms.remove(0);
    }
    let right = terms.split_off(terms.len() / 2);
    add(balanced(terms, add), balanced(right, add))
}

fn primitive(
    operation: SemanticPrimitive,
    arguments: Vec<SemanticTerm>,
    result: SemanticType,
) -> SemanticTerm {
    SemanticTerm::Primitive {
        operation,
        arguments,
        result,
    }
}

const fn nat(value: String) -> SemanticTerm {
    SemanticTerm::Nat { value }
}

const fn int(value: String) -> SemanticTerm {
    SemanticTerm::Integer {
        representation: SemanticInteger::Int,
        value,
    }
}

/// `left + right` over mathematical integers.
fn int_add(left: SemanticTerm, right: SemanticTerm) -> SemanticTerm {
    primitive(
        SemanticPrimitive::Subtract,
        vec![
            left,
            primitive(SemanticPrimitive::Negate, vec![right], SemanticType::Int),
        ],
        SemanticType::Int,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The value a closed elaboration denotes, for the constructs it writes.
    fn eval(term: &SemanticTerm, local: (&str, i128)) -> i128 {
        let literal = |value: &str| value.parse::<i128>().expect("canonical decimal");
        match term {
            SemanticTerm::Var { name } if name == local.0 => local.1,
            SemanticTerm::Nat { value } | SemanticTerm::Integer { value, .. } => literal(value),
            SemanticTerm::Add { left, right } => eval(left, local) + eval(right, local),
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => {
                if eval(condition, local) == 1 {
                    eval(then_value, local)
                } else {
                    eval(else_value, local)
                }
            }
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => eval(body, (binder.name.as_str(), eval(value, local))),
            SemanticTerm::Primitive {
                operation,
                arguments,
                ..
            } => {
                let values: Vec<i128> = arguments.iter().map(|a| eval(a, local)).collect();
                match operation {
                    SemanticPrimitive::BitAnd => values[0] & values[1],
                    SemanticPrimitive::Equal => i128::from(values[0] == values[1]),
                    SemanticPrimitive::Subtract => values[0] - values[1],
                    SemanticPrimitive::Negate => -values[0],
                    other => panic!("unexpected {other:?}"),
                }
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    /// Every literal the elaboration writes, as a decimal.
    fn literals(term: &SemanticTerm, out: &mut Vec<(SemanticInteger, i128)>) {
        super::super::visit_terms(term, &mut |node| match node {
            SemanticTerm::Integer {
                representation,
                value,
            } => out.push((*representation, value.parse().expect("decimal"))),
            SemanticTerm::Nat { value } => {
                out.push((SemanticInteger::UInt64, value.parse().expect("decimal")));
            }
            _ => {}
        });
    }

    #[test]
    fn every_width_denotes_the_value_at_its_bounds_and_samples() {
        let types = [
            SemanticType::UInt8,
            SemanticType::Int8,
            SemanticType::UInt16,
            SemanticType::Int16,
            SemanticType::UInt32,
            SemanticType::Int32,
            SemanticType::UInt64,
            SemanticType::Int64,
        ];
        for ty in types {
            let (_, signed, width) = fixed(&ty).expect("fixed");
            let (least, greatest) = if signed {
                (-(1_i128 << (width - 1)), (1_i128 << (width - 1)) - 1)
            } else {
                (0, (1_i128 << width) - 1)
            };
            let mut samples = vec![least, greatest, 0, 1, least + 1, greatest - 1];
            let mut state = 0x9e37_79b9_7f4a_7c15_u64 ^ u64::from(width);
            for _ in 0..64 {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let span = u128::try_from(greatest - least).expect("span") + 1;
                let offset = i128::try_from(u128::from(state) % span).expect("offset");
                samples.push(least + offset);
            }
            let mut operations = vec![SemanticPrimitive::ToInt];
            if !signed {
                operations.push(SemanticPrimitive::ToNat);
            }
            for operation in operations {
                let mut counter = State::default();
                let operand = SemanticTerm::Var {
                    name: "x".to_owned(),
                };
                let term = widen(operation, &ty, operand, &mut counter).expect("widens");
                assert_eq!(counter.binders, 1);
                for sample in &samples {
                    // A fixed-width bit pattern, read as the machine reads it.
                    assert_eq!(eval(&term, ("x", *sample)), *sample, "{ty} {operation:?}");
                }
                let mut found = Vec::new();
                literals(&term, &mut found);
                for (representation, value) in found {
                    let (least, greatest) = match representation {
                        SemanticInteger::Int => (i128::from(i64::MIN), i128::from(i64::MAX)),
                        SemanticInteger::UInt64 => (0, i128::from(u64::MAX)),
                        _ => (least, greatest),
                    };
                    assert!(
                        (least..=greatest).contains(&value),
                        "{ty} {operation:?} writes {value} outside its type's range"
                    );
                }
            }
        }
    }

    #[test]
    fn a_signed_value_has_no_natural_widening() {
        let mut counter = State::default();
        let operand = SemanticTerm::Var {
            name: "x".to_owned(),
        };
        assert!(widen(
            SemanticPrimitive::ToNat,
            &SemanticType::Int8,
            operand.clone(),
            &mut counter
        )
        .is_err());
        assert!(widen(
            SemanticPrimitive::ToInt,
            &SemanticType::Int,
            operand,
            &mut counter
        )
        .is_err());
    }
}
