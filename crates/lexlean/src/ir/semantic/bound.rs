//! Language 1.2 (SPEC.md §17.13): the bound of a production root.
//!
//! A production root may name a *bound*: a prior theorem of its module whose
//! statement is exactly the obligation generated here. Under the hypothesis
//! that every natural-number parameter lies in the target width, each
//! addition and multiplication of naturals in the root's own body stays
//! below 2^64, in evaluation order. Eligibility (`production::eligibility`)
//! discharges `overflow` with the bound, and certificate A
//! (`production::certificate`) proves from the theorem that the root's width
//! predicate holds on every argument in the width, so the discharge rests on
//! the kernel and never on this generator: a statement this generator got
//! wrong leaves the certificate's step ill-typed, and the pinned Lean refuses
//! the module.
//!
//! The rule is small by decision. The body is read through `let`, variables,
//! literals, `+`, and the arithmetic primitives on naturals, where
//! subtraction, quotient, and remainder are total and contribute no check.
//! Any other construct, an integer operation, a successor, a call, a match,
//! or a conditional is refused as `LLT4017`, naming it, so an operation the
//! bound would not cover is never admitted by inference and never guarded
//! at run time.

use super::model::{fail, implies, require_statement, Obligation};
use super::{
    Environment, MemberRef, SemanticDeclaration, SemanticFailure, SemanticInteger,
    SemanticParameter, SemanticPrimitive, SemanticTerm, SemanticType,
};
use crate::code;
use crate::production::eligibility::{primitive_key, term_key};

/// `2^64`: the natural-number width of every registered target (§17.13).
pub const NAT_WIDTH: &str = "18446744073709551616";

/// The construct keys of the production registry a bound covers (§17.13):
/// the addition and the multiplication of naturals. Every other row that
/// names a representation keeps `overflow`.
pub const COVERED: [&str; 2] = ["term.add", "primitive.multiply"];

/// §17.13: a production root that names a bound names a prior theorem of
/// its module stating exactly the obligation of [`obligation`].
pub(super) fn check_production_bound(
    declaration: &SemanticDeclaration,
    env: &Environment<'_>,
) -> Result<(), SemanticFailure> {
    let SemanticDeclaration::Definition {
        name,
        parameters,
        body,
        production: Some(production),
        ..
    } = declaration
    else {
        return Ok(());
    };
    let Some(bound) = &production.bound else {
        return Ok(());
    };
    let obligation = obligation(name, parameters, body, bound)?;
    require_statement(env, bound, &obligation, code!("LLT4017"))
}

fn uncovered(root: &str, reason: String) -> SemanticFailure {
    fail(
        code!("LLT4017"),
        format!("production bound of `{root}`: {reason}"),
    )
}

/// The obligation the bound of `root` discharges: for the parameters of the
/// root, each natural parameter below the width implies that every addition
/// and multiplication of naturals of `body` is below the width, as a
/// conjunction in evaluation order with every `let` substituted.
///
/// # Errors
///
/// `LLT4017` when the body has a construct the rule does not cover, or no
/// operation that can overflow.
pub fn obligation(
    root: &str,
    parameters: &[SemanticParameter],
    body: &SemanticTerm,
    bound: &MemberRef,
) -> Result<Obligation, SemanticFailure> {
    let mut walk = Walk {
        root,
        parameters,
        lets: Vec::new(),
        conjuncts: Vec::new(),
    };
    walk.term(body)?;
    let Some(mut conclusion) = walk.conjuncts.pop() else {
        return Err(uncovered(
            root,
            "the body has no addition or multiplication of naturals, so the bound covers no operation"
                .to_owned(),
        ));
    };
    while let Some(conjunct) = walk.conjuncts.pop() {
        conclusion = SemanticTerm::PropAnd {
            left: Box::new(conjunct),
            right: Box::new(conclusion),
        };
    }
    let mut statement = conclusion;
    for parameter in parameters.iter().rev() {
        if parameter.r#type == SemanticType::Nat {
            statement = implies(
                below_width(SemanticTerm::Var {
                    name: parameter.name.clone(),
                }),
                statement,
            );
        }
    }
    Ok(Obligation {
        role: format!("production bound of `{root}`"),
        theorem: bound.clone(),
        type_parameters: Vec::new(),
        parameters: parameters.to_vec(),
        statement,
    })
}

/// `term < 2^64`.
fn below_width(term: SemanticTerm) -> SemanticTerm {
    SemanticTerm::Lt {
        left: Box::new(term),
        right: Box::new(SemanticTerm::Nat {
            value: NAT_WIDTH.to_owned(),
        }),
    }
}

const fn integer_type(representation: SemanticInteger) -> SemanticType {
    match representation {
        SemanticInteger::Int => SemanticType::Int,
        SemanticInteger::Int8 => SemanticType::Int8,
        SemanticInteger::Int16 => SemanticType::Int16,
        SemanticInteger::Int32 => SemanticType::Int32,
        SemanticInteger::Int64 => SemanticType::Int64,
        SemanticInteger::UInt8 => SemanticType::UInt8,
        SemanticInteger::UInt16 => SemanticType::UInt16,
        SemanticInteger::UInt32 => SemanticType::UInt32,
        SemanticInteger::UInt64 => SemanticType::UInt64,
    }
}

/// The walk of a root's body in evaluation order: operands left to right,
/// then the operation; a `let`'s value, then its body with the value
/// substituted.
struct Walk<'a> {
    root: &'a str,
    parameters: &'a [SemanticParameter],
    /// The `let`s in scope: name, type, and substituted value.
    lets: Vec<(String, SemanticType, SemanticTerm)>,
    /// The checks recorded so far, in evaluation order.
    conjuncts: Vec<SemanticTerm>,
}

impl Walk<'_> {
    /// The term with its `let`s substituted, and its type.
    fn term(
        &mut self,
        term: &SemanticTerm,
    ) -> Result<(SemanticTerm, SemanticType), SemanticFailure> {
        match term {
            SemanticTerm::Var { name } => {
                if let Some((_, ty, value)) = self
                    .lets
                    .iter()
                    .rev()
                    .find(|(bound, _, _)| bound == name)
                {
                    return Ok((value.clone(), ty.clone()));
                }
                let parameter = self
                    .parameters
                    .iter()
                    .find(|parameter| &parameter.name == name)
                    .ok_or_else(|| {
                        uncovered(
                            self.root,
                            format!("`{name}` is neither a parameter nor a `let` of the root"),
                        )
                    })?;
                Ok((term.clone(), parameter.r#type.clone()))
            }
            SemanticTerm::Nat { value: _ } => Ok((term.clone(), SemanticType::Nat)),
            SemanticTerm::Integer {
                representation,
                value: _,
            } => Ok((term.clone(), integer_type(*representation))),
            SemanticTerm::Add { left, right } => {
                let (left, _) = self.term(left)?;
                let (right, _) = self.term(right)?;
                let sum = SemanticTerm::Add {
                    left: Box::new(left),
                    right: Box::new(right),
                };
                self.conjuncts.push(below_width(sum.clone()));
                Ok((sum, SemanticType::Nat))
            }
            SemanticTerm::Primitive {
                operation,
                arguments,
                result,
            } => {
                let mut substituted = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    substituted.push(self.term(argument)?.0);
                }
                let applied = SemanticTerm::Primitive {
                    operation: *operation,
                    arguments: substituted,
                    result: result.clone(),
                };
                match (operation, result) {
                    (SemanticPrimitive::Multiply, SemanticType::Nat) => {
                        self.conjuncts.push(below_width(applied.clone()));
                        Ok((applied, SemanticType::Nat))
                    }
                    (
                        SemanticPrimitive::Subtract
                        | SemanticPrimitive::Quotient
                        | SemanticPrimitive::Remainder,
                        SemanticType::Nat,
                    ) => Ok((applied, SemanticType::Nat)),
                    (operation, result) => Err(uncovered(
                        self.root,
                        format!(
                            "the primitive `{}` at `{result}` is not covered: a bound covers the additions and multiplications of naturals of the root's own body, read through `let` and the total subtraction, quotient, and remainder of naturals",
                            primitive_key(*operation)
                        ),
                    )),
                }
            }
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => {
                let (value, _) = self.term(value)?;
                self.lets
                    .push((binder.name.clone(), binder.r#type.clone(), value));
                let body = self.term(body);
                self.lets.pop();
                body
            }
            SemanticTerm::String { .. }
            | SemanticTerm::Bytes { .. }
            | SemanticTerm::Bool { .. }
            | SemanticTerm::Unit
            | SemanticTerm::Nil { .. }
            | SemanticTerm::Cons { .. }
            | SemanticTerm::Record { .. }
            | SemanticTerm::Constructor { .. }
            | SemanticTerm::InstanceValue { .. }
            | SemanticTerm::Project { .. }
            | SemanticTerm::Call { .. }
            | SemanticTerm::If { .. }
            | SemanticTerm::Match { .. }
            | SemanticTerm::Eq { .. }
            | SemanticTerm::Le { .. }
            | SemanticTerm::Lt { .. }
            | SemanticTerm::Beq { .. }
            | SemanticTerm::Ble { .. }
            | SemanticTerm::Blt { .. }
            | SemanticTerm::And { .. }
            | SemanticTerm::PropAnd { .. }
            | SemanticTerm::Or { .. }
            | SemanticTerm::Not { .. }
            | SemanticTerm::Implies { .. }
            | SemanticTerm::Iff { .. }
            | SemanticTerm::Forall { .. }
            | SemanticTerm::Pair { .. }
            | SemanticTerm::First { .. }
            | SemanticTerm::Second { .. }
            | SemanticTerm::Lambda { .. }
            | SemanticTerm::Apply { .. }
            | SemanticTerm::FunctionRef { .. }
            | SemanticTerm::MapLiteral { .. }
            | SemanticTerm::SetLiteral { .. }
            | SemanticTerm::GraphLiteral { .. }
            | SemanticTerm::CheckedApply { .. } => Err(uncovered(
                self.root,
                format!(
                    "the construct `{}` is not covered: a bound covers the additions and multiplications of naturals of the root's own body, read through `let`, parameters, literals, and the total subtraction, quotient, and remainder of naturals, so an operation under this construct keeps `overflow`",
                    term_key(term)
                ),
            )),
        }
    }
}
