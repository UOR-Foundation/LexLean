//! Deterministic Lean and LaTeX lowering for language-1.1 semantic modules.

use std::collections::BTreeSet;

use crate::artifact::source_map::MapRole;
use crate::backend::{EmitSource, Emitter};
use crate::code;
use crate::diagnostic::Diagnostic;
use crate::ir::semantic::model::{CheckProof, CheckStatement, CrossCheck};
use crate::ir::semantic::BUILTIN_CONSTRUCTOR_OWNERS;
use crate::ir::semantic::{
    mentions_type_parameter, MemberRef, SemanticAssignment, SemanticBranch, SemanticDeclaration,
    SemanticModule, SemanticParameter, SemanticPrimitive, SemanticProof, SemanticProofBranch,
    SemanticReflection, SemanticReflectionComparison, SemanticReflectionField, SemanticTerm,
    SemanticType,
};
use crate::link::CheckedModule;
use crate::source::coverage::Origin;

mod reasoning;

struct Render<'a> {
    prefix: &'a str,
    /// §17.12: in a well-founded definition, each `if` (by node address)
    /// lowers to a dependent match binding its numbered hypothesis.
    hypotheses: std::collections::BTreeMap<usize, usize>,
    /// Whether this module emits the portable runtime, whose Nat
    /// subtraction and multiplication `linear_arithmetic` unfolds.
    runtime: bool,
    /// Whether this renders the canonical document, which names a map and
    /// a set by their own type constructors rather than by the list
    /// encoding Lean receives (§17.12).
    document: bool,
    /// The module's own Lean name, when every local reference is written
    /// qualified: a cross-check named `E.thm` would otherwise resolve the
    /// theorem `thm` it restates to itself (§17.12, models).
    qualify: Option<&'a str>,
}

/// The generated name of well-founded hypothesis `index`. Semantic names
/// begin with an ASCII letter and an unused binder lowers as `_name`, so no
/// source binder can capture a name beginning with two underscores.
fn hypothesis(index: usize) -> String {
    format!("__decrease{index}")
}

// Semantic names are validated data, not Lean tokens. Quoting a reserved
// segment preserves its exact Name identity instead of narrowing the source
// language to whatever the pinned parser happens to leave unreserved.
fn identifier(name: &str) -> String {
    name.split('.')
        .map(|segment| {
            if super::lean_tokens::is_reserved(segment) {
                format!("«{segment}»")
            } else {
                segment.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join(".")
}

// Lean's pinned parser accepts four-digit Unicode escapes, not Rust's
// zero escape or braced Unicode debug spelling. Escape characters, never
// substrings, so a literal backslash followed by `0` remains literal data.
fn string_literal(value: &str) -> String {
    use std::fmt::Write as _;

    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character.is_control() => {
                write!(output, "\\u{:04x}", u32::from(character)).expect("writing to a string");
            }
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

/// The Lean spelling of a binder: its identifier when its scope mentions
/// it, otherwise `_name`, which the unused-variable linter exempts.
fn bound_name(name: &str, used: bool) -> String {
    if used {
        identifier(name)
    } else {
        format!("_{name}")
    }
}

/// Whether a proof refers to the local `local` by name: as a case or
/// induction scrutinee, a generalized variable, a hypothesis it simplifies
/// with, a reflected value, or inside a term it applies a theorem to. A
/// branch binder of the same name shadows it.
fn proof_uses(proof: &SemanticProof, local: &str) -> bool {
    let branch_uses = |branches: &[crate::ir::semantic::SemanticProofBranch]| {
        branches.iter().any(|branch| {
            !branch.binders.iter().any(|binder| binder == local) && proof_uses(&branch.proof, local)
        })
    };
    match proof {
        SemanticProof::Reflexivity | SemanticProof::Decide | SemanticProof::Congruence => false,
        SemanticProof::LinearArithmetic { definitions }
        | SemanticProof::Simplify { definitions } => definitions
            .iter()
            .any(|member| member.module.is_none() && member.name == local),
        SemanticProof::Constructor { branches } => {
            branches.iter().any(|branch| proof_uses(branch, local))
        }
        SemanticProof::Cases {
            scrutinee,
            branches,
        } => scrutinee == local || branch_uses(branches),
        SemanticProof::Induction {
            scrutinee,
            generalizing,
            branches,
        } => {
            scrutinee == local
                || generalizing.iter().any(|name| name == local)
                || branch_uses(branches)
        }
        SemanticProof::BooleanReflection { reflection } => match reflection {
            SemanticReflection::List {
                parameter, values, ..
            } => parameter == local || values == local,
            SemanticReflection::Record { record, .. } => record == local,
        },
        SemanticProof::Apply { arguments, .. } => {
            arguments.iter().any(|argument| term_uses(argument, local))
        }
    }
}

fn term_uses(term: &SemanticTerm, local: &str) -> bool {
    let pair = |left: &SemanticTerm, right: &SemanticTerm| {
        term_uses(left, local) || term_uses(right, local)
    };
    match term {
        SemanticTerm::Var { name } => name == local,
        SemanticTerm::Nat { .. }
        | SemanticTerm::Integer { .. }
        | SemanticTerm::String { .. }
        | SemanticTerm::Bytes { .. }
        | SemanticTerm::Bool { .. }
        | SemanticTerm::Unit
        | SemanticTerm::Nil { .. }
        | SemanticTerm::InstanceValue { .. } => false,
        SemanticTerm::Cons { head, tail }
        | SemanticTerm::Eq {
            left: head,
            right: tail,
        }
        | SemanticTerm::Le {
            left: head,
            right: tail,
        }
        | SemanticTerm::Lt {
            left: head,
            right: tail,
        }
        | SemanticTerm::Add {
            left: head,
            right: tail,
        }
        | SemanticTerm::Beq {
            left: head,
            right: tail,
        }
        | SemanticTerm::Ble {
            left: head,
            right: tail,
        }
        | SemanticTerm::Blt {
            left: head,
            right: tail,
        }
        | SemanticTerm::And {
            left: head,
            right: tail,
        }
        | SemanticTerm::PropAnd {
            left: head,
            right: tail,
        }
        | SemanticTerm::Or {
            left: head,
            right: tail,
        }
        | SemanticTerm::Implies {
            premise: head,
            conclusion: tail,
        }
        | SemanticTerm::Iff {
            left: head,
            right: tail,
        } => pair(head, tail),
        SemanticTerm::Not { value }
        | SemanticTerm::Project { value, .. }
        | SemanticTerm::First { value }
        | SemanticTerm::Second { value } => term_uses(value, local),
        SemanticTerm::Pair { left, right } => term_uses(left, local) || term_uses(right, local),
        SemanticTerm::Lambda {
            parameters, body, ..
        } => !parameters.iter().any(|parameter| parameter.name == local) && term_uses(body, local),
        SemanticTerm::Apply {
            function,
            arguments,
        } => {
            term_uses(function, local)
                || arguments.iter().any(|argument| term_uses(argument, local))
        }
        SemanticTerm::FunctionRef { .. } => false,
        SemanticTerm::CheckedApply { arguments, .. } => {
            arguments.iter().any(|argument| term_uses(argument, local))
        }
        SemanticTerm::MapLiteral { entries, .. } => entries
            .iter()
            .any(|entry| term_uses(&entry.key, local) || term_uses(&entry.value, local)),
        SemanticTerm::SetLiteral { elements, .. } => {
            elements.iter().any(|element| term_uses(element, local))
        }
        SemanticTerm::GraphLiteral { nodes, edges, .. } => {
            nodes.iter().any(|node| term_uses(node, local))
                || edges
                    .iter()
                    .any(|edge| term_uses(&edge.source, local) || term_uses(&edge.target, local))
        }
        SemanticTerm::Record { fields, .. } => fields
            .iter()
            .any(|assignment| term_uses(&assignment.value, local)),
        SemanticTerm::Constructor { arguments, .. }
        | SemanticTerm::Call { arguments, .. }
        | SemanticTerm::Primitive { arguments, .. } => {
            arguments.iter().any(|argument| term_uses(argument, local))
        }
        SemanticTerm::If {
            condition,
            then_value,
            else_value,
        } => {
            term_uses(condition, local)
                || term_uses(then_value, local)
                || term_uses(else_value, local)
        }
        SemanticTerm::Match {
            scrutinee,
            branches,
        } => {
            term_uses(scrutinee, local)
                || branches.iter().any(|branch| term_uses(&branch.body, local))
        }
        SemanticTerm::Forall { binder, body } => binder.name != local && term_uses(body, local),
        SemanticTerm::Let {
            binder,
            value,
            body,
        } => term_uses(value, local) || (binder.name != local && term_uses(body, local)),
    }
}

/// The Lean pattern of a `ContractViolation` or `ReasoningFailure`
/// constructor: the pair of Booleans it is represented by (§17.12).
fn violation_pattern(constructor: &MemberRef) -> Option<&'static str> {
    if constructor.module.is_some() {
        return None;
    }
    match constructor.name.as_str() {
        "ContractViolation.precondition" => Some("(false, false)"),
        "ContractViolation.input_invariant" => Some("(false, true)"),
        "ContractViolation.postcondition" => Some("(true, false)"),
        "ContractViolation.output_invariant" => Some("(true, true)"),
        // §17.12 (reasoning): `(answered, replay)`.
        "ReasoningFailure.exhausted" => Some("(false, false)"),
        "ReasoningFailure.unsolved" => Some("(false, true)"),
        "ReasoningFailure.rejected" => Some("(true, false)"),
        "ReasoningFailure.invalid_step" => Some("(true, true)"),
        _ => None,
    }
}

fn reflected_projection(record: &str, field: &SemanticReflectionField) -> String {
    format!("({}).{}", identifier(record), identifier(&field.field))
}

fn reflected_bool(record: &str, fields: &[SemanticReflectionField]) -> String {
    let field = &fields[0];
    let comparison = format!(
        "Nat.beq ({}) ({})",
        reflected_projection(record, field),
        field.expected
    );
    if fields.len() == 1 {
        comparison
    } else {
        format!("({comparison} && {})", reflected_bool(record, &fields[1..]))
    }
}

fn reflected_prop(record: &str, fields: &[SemanticReflectionField]) -> String {
    let field = &fields[0];
    let comparison = format!(
        "({} = {})",
        reflected_projection(record, field),
        field.expected
    );
    if fields.len() == 1 {
        comparison
    } else {
        format!(
            "({comparison} /\\ {})",
            reflected_prop(record, &fields[1..])
        )
    }
}

fn reflected_iff(fields: &[SemanticReflectionField]) -> String {
    if fields.len() == 1 {
        "llBeqBridge _ _".to_owned()
    } else {
        format!(
            "Iff.trans (llAndBridge _ _) (and_congr (llBeqBridge _ _) ({}))",
            reflected_iff(&fields[1..])
        )
    }
}

impl Render<'_> {
    fn member(&self, member: &MemberRef) -> String {
        if member.module.is_none() {
            match member.name.as_str() {
                "Result.error" => return "Except.error".to_owned(),
                "Result.ok" => return "Except.ok".to_owned(),
                _ => {}
            }
        }
        let builtin = member
            .name
            .split_once('.')
            .is_some_and(|(owner, _)| BUILTIN_CONSTRUCTOR_OWNERS.contains(&owner));
        match (&member.module, self.qualify) {
            (Some(module), _) => identifier(&format!("{}.{}.{}", self.prefix, module, member.name)),
            (None, Some(own)) if !builtin => identifier(&format!("{own}.{}", member.name)),
            (None, _) => identifier(&member.name),
        }
    }

    fn ty(&self, ty: &SemanticType) -> String {
        match ty {
            SemanticType::Type => "Type".to_owned(),
            SemanticType::Parameter { name } => identifier(name),
            SemanticType::Nat => "Nat".to_owned(),
            SemanticType::Bool => "Bool".to_owned(),
            SemanticType::Prop => "Prop".to_owned(),
            SemanticType::Unit => "Unit".to_owned(),
            SemanticType::Int => "Int".to_owned(),
            SemanticType::Int8 => "Int8".to_owned(),
            SemanticType::Int16 => "Int16".to_owned(),
            SemanticType::Int32 => "Int32".to_owned(),
            SemanticType::Int64 => "Int64".to_owned(),
            SemanticType::UInt8 => "UInt8".to_owned(),
            SemanticType::UInt16 => "UInt16".to_owned(),
            SemanticType::UInt32 => "UInt32".to_owned(),
            SemanticType::UInt64 => "UInt64".to_owned(),
            SemanticType::String => "String".to_owned(),
            SemanticType::Bytes => "ByteArray".to_owned(),
            SemanticType::Ordering => "Ordering".to_owned(),
            SemanticType::Option { value } => format!("Option ({})", self.ty(value)),
            SemanticType::Result { ok, error } => {
                format!("Except ({}) ({})", self.ty(error), self.ty(ok))
            }
            SemanticType::List { element } => format!("List ({})", self.ty(element)),
            SemanticType::Named { member, arguments } => {
                let mut out = self.member(member);
                for argument in arguments {
                    out.push_str(" (");
                    out.push_str(&self.ty(argument));
                    out.push(')');
                }
                out
            }
            SemanticType::Product { left, right } => {
                format!("(Prod ({}) ({}))", self.ty(left), self.ty(right))
            }
            SemanticType::Function { parameters, result } => {
                let mut out = String::from("(");
                for parameter in parameters {
                    out.push_str(&format!("({}) -> ", self.ty(parameter)));
                }
                out.push_str(&format!("({}))", self.ty(result)));
                out
            }
            // §17.12: a map is its canonical ascending entry list and a set
            // its canonical ascending element list; the representation is
            // plain data, so values cross module boundaries unchanged.
            SemanticType::Map { key, value } if self.document => {
                format!("Map ({}) ({})", self.ty(key), self.ty(value))
            }
            SemanticType::Set { element } if self.document => format!("Set ({})", self.ty(element)),
            SemanticType::Map { key, value } => {
                format!("List (Prod ({}) ({}))", self.ty(key), self.ty(value))
            }
            SemanticType::Set { element } => format!("List ({})", self.ty(element)),
            // §17.12 (models): a refusal is plain data with exactly four
            // values, so it crosses module boundaries unchanged.
            SemanticType::ContractViolation if self.document => "ContractViolation".to_owned(),
            SemanticType::ContractViolation => "(Prod Bool Bool)".to_owned(),
            // §17.12 (reasoning): likewise for a reasoning failure.
            SemanticType::ReasoningFailure if self.document => "ReasoningFailure".to_owned(),
            SemanticType::ReasoningFailure => "(Prod Bool Bool)".to_owned(),
        }
    }

    /// Binders the scope never mentions lower as `_name`: Lean's
    /// unused-variable linter would otherwise warn, and verification admits
    /// no unexpected output. Semantic names begin with an ASCII letter, so
    /// `_name` captures no other binder.
    fn scoped_parameters(
        &self,
        parameters: &[SemanticParameter],
        used: impl Fn(&str) -> bool,
    ) -> String {
        parameters
            .iter()
            .map(|parameter| {
                format!(
                    " ({} : {})",
                    bound_name(&parameter.name, used(&parameter.name)),
                    self.ty(&parameter.r#type)
                )
            })
            .collect()
    }

    fn parameters(&self, parameters: &[SemanticParameter]) -> String {
        parameters
            .iter()
            .map(|parameter| {
                format!(
                    " ({} : {})",
                    identifier(&parameter.name),
                    self.ty(&parameter.r#type)
                )
            })
            .collect()
    }

    fn recursive_equations(
        &self,
        name: &str,
        type_binders: &str,
        parameters: &[SemanticParameter],
        result: &SemanticType,
        recursive_argument: &str,
        body: &SemanticTerm,
    ) -> Option<String> {
        let SemanticTerm::Match {
            scrutinee,
            branches,
        } = body
        else {
            return None;
        };
        if !matches!(scrutinee.as_ref(), SemanticTerm::Var { name } if name == recursive_argument) {
            return None;
        }
        let mut text = format!("@[expose] public def {}{type_binders} :", identifier(name));
        for parameter in parameters {
            text.push_str(&format!(
                " ({} : {}) ->",
                identifier(&parameter.name),
                self.ty(&parameter.r#type)
            ));
        }
        text.push_str(&format!(" {}\n", self.ty(result)));
        for branch in branches {
            let patterns = parameters
                .iter()
                .map(|parameter| {
                    if parameter.name == recursive_argument {
                        let binders = if branch.binders.is_empty() {
                            String::new()
                        } else {
                            format!(
                                " {}",
                                branch
                                    .binders
                                    .iter()
                                    .map(|binder| {
                                        if term_uses(&branch.body, binder) {
                                            identifier(binder)
                                        } else {
                                            "_".to_owned()
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            )
                        };
                        format!("{}{}", self.member(&branch.constructor), binders)
                    } else if term_uses(&branch.body, &parameter.name) {
                        identifier(&parameter.name)
                    } else {
                        format!("_{}", parameter.name)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            text.push_str(&format!("  | {patterns} => {}\n", self.term(&branch.body)));
        }
        Some(text)
    }

    /// A definition's or theorem's type parameters; one its declaration
    /// never mentions lowers as `_name`, like an unused value binder.
    fn scoped_type_parameters(
        &self,
        parameters: &[String],
        declaration: &SemanticDeclaration,
    ) -> String {
        parameters
            .iter()
            .map(|parameter| {
                format!(
                    " ({} : Type)",
                    bound_name(parameter, mentions_type_parameter(declaration, parameter))
                )
            })
            .collect()
    }

    fn type_parameters(&self, parameters: &[String]) -> String {
        parameters
            .iter()
            .map(|parameter| format!(" ({} : Type)", identifier(parameter)))
            .collect()
    }

    fn assignments(&self, assignments: &[SemanticAssignment]) -> String {
        assignments
            .iter()
            .map(|row| format!("{} := {}", identifier(&row.field), self.term(&row.value)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// One Lean-only cross-check (§17.12, models).
    fn cross_check(&self, check: &CrossCheck, module: &str) -> String {
        let qualified = Render {
            prefix: self.prefix,
            hypotheses: std::collections::BTreeMap::new(),
            runtime: self.runtime,
            document: false,
            qualify: Some(module),
        };
        let this = &qualified;
        let name = identifier(&check.name);
        let type_parameters = this.type_parameters(&check.type_parameters);
        let (statement, proof) = match &check.statement {
            CheckStatement::Helper { helper, arguments } => {
                let mut out = format!("LexLeanModels.{helper}");
                for argument in arguments {
                    out.push_str(&format!(" ({})", this.term(argument)));
                }
                (out, None)
            }
            CheckStatement::Tensor {
                artifact,
                element,
                shape,
            } => {
                let (width, signed) = crate::ir::semantic::model::element_encoding(*element);
                let value = identifier(artifact);
                // The flattened values decode from exactly the bytes, and
                // every level of nesting has exactly its declared length.
                let mut level = value.clone();
                let mut shape_checks = vec![format!("{value}.length == {}", shape[0])];
                let mut rows: u64 = 1;
                for (depth, dimension) in shape.iter().enumerate().skip(1) {
                    rows = rows.saturating_mul(shape[depth - 1]);
                    shape_checks.push(format!(
                        "{level}.map List.length == List.replicate {rows} {dimension}"
                    ));
                    level = format!("{level}.flatten");
                }
                (
                    format!(
                        "(LexLeanModels.tensorMatches {width} {signed} {level} {value}.bytes && {}) = true",
                        shape_checks
                            .iter()
                            .map(|check| format!("({check})"))
                            .collect::<Vec<_>>()
                            .join(" && ")
                    ),
                    Some("by decide"),
                )
            }
            CheckStatement::Lines { artifact } => {
                let value = identifier(artifact);
                (
                    format!("LexLeanModels.linesMatch {value} {value}.bytes = true"),
                    Some("by decide"),
                )
            }
            CheckStatement::Bytes { artifact } => {
                let value = identifier(artifact);
                (format!("{value} = {value}.bytes"), Some("rfl"))
            }
        };
        let proof = match (&check.proof, proof) {
            (_, Some(proof)) => proof.to_owned(),
            (CheckProof::Decide, None) => "by decide".to_owned(),
            (
                CheckProof::Theorem {
                    theorem,
                    type_arguments,
                },
                None,
            ) => {
                let mut out = this.member(theorem);
                for argument in type_arguments {
                    out.push_str(&format!(" ({})", this.ty(argument)));
                }
                out
            }
        };
        format!("public theorem {name}{type_parameters} : {statement} := {proof}\n")
    }

    fn branch(&self, branch: &SemanticBranch) -> String {
        let binders = if branch.binders.is_empty() {
            String::new()
        } else {
            format!(
                " {}",
                branch
                    .binders
                    .iter()
                    .map(|binder| {
                        if term_uses(&branch.body, binder) {
                            identifier(binder)
                        } else {
                            "_".to_owned()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };
        let pattern = match violation_pattern(&branch.constructor) {
            Some(pattern) if !self.document => pattern.to_owned(),
            _ => self.member(&branch.constructor),
        };
        format!("| {pattern}{binders} => {}", self.term(&branch.body))
    }

    fn primitive(
        &self,
        operation: SemanticPrimitive,
        arguments: &[SemanticTerm],
        result: &SemanticType,
    ) -> String {
        let int64_specialization = match operation {
            SemanticPrimitive::CheckedAdd => Some("checkedAddInt64"),
            SemanticPrimitive::CheckedSubtract => Some("checkedSubtractInt64"),
            SemanticPrimitive::CheckedMultiply => Some("checkedMultiplyInt64"),
            SemanticPrimitive::CheckedNegate => Some("checkedNegateInt64"),
            SemanticPrimitive::CheckedQuotient => Some("checkedQuotientInt64"),
            _ => None,
        };
        if matches!(result, SemanticType::Option { value } if **value == SemanticType::Int64) {
            if let Some(operation) = int64_specialization {
                let mut out = format!("(LexLeanRuntime.{operation}");
                for argument in arguments {
                    out.push_str(" (");
                    out.push_str(&self.term(argument));
                    out.push(')');
                }
                out.push_str(" : Option (Int64))");
                return out;
            }
        }
        let operation = match operation {
            SemanticPrimitive::Subtract => "subtract",
            SemanticPrimitive::Multiply => "multiply",
            SemanticPrimitive::Quotient => "quotient",
            SemanticPrimitive::Remainder => "remainder",
            SemanticPrimitive::Negate => "negate",
            SemanticPrimitive::CheckedConvert => "checkedConvert",
            SemanticPrimitive::CheckedAdd => "checkedAdd",
            SemanticPrimitive::CheckedSubtract => "checkedSubtract",
            SemanticPrimitive::CheckedMultiply => "checkedMultiply",
            SemanticPrimitive::CheckedNegate => "checkedNegate",
            SemanticPrimitive::CheckedQuotient => "checkedQuotient",
            SemanticPrimitive::BitAnd => "bitAnd",
            SemanticPrimitive::BitOr => "bitOr",
            SemanticPrimitive::BitXor => "bitXor",
            SemanticPrimitive::BitNot => "bitNot",
            SemanticPrimitive::ShiftLeft => "shiftLeft",
            SemanticPrimitive::ShiftRight => "shiftRight",
            SemanticPrimitive::Append => "append",
            SemanticPrimitive::Length => "length",
            SemanticPrimitive::Index => "index",
            SemanticPrimitive::Slice => "slice",
            SemanticPrimitive::Utf8Encode => "utf8Encode",
            SemanticPrimitive::Utf8Decode => "utf8Decode",
            SemanticPrimitive::CompareBytes => "compareBytes",
            SemanticPrimitive::Equal => "equal",
            SemanticPrimitive::SplitExact => "splitExact",
            SemanticPrimitive::Join => "join",
            SemanticPrimitive::ParseDecimal => "parseDecimal",
            SemanticPrimitive::FormatDecimal => "formatDecimal",
            collection => {
                let name = match collection {
                    SemanticPrimitive::MapInsert => "mapInsert",
                    SemanticPrimitive::MapRemove => "mapRemove",
                    SemanticPrimitive::MapLookup => "mapLookup",
                    SemanticPrimitive::MapContains => "mapContains",
                    SemanticPrimitive::MapSize => "mapSize",
                    SemanticPrimitive::MapKeys => "mapKeys",
                    SemanticPrimitive::MapValues => "mapValues",
                    SemanticPrimitive::MapEntries => "mapEntries",
                    SemanticPrimitive::MapFold => "mapFold",
                    SemanticPrimitive::SetInsert => "setInsert",
                    SemanticPrimitive::SetRemove => "setRemove",
                    SemanticPrimitive::SetContains => "setContains",
                    SemanticPrimitive::SetSize => "setSize",
                    SemanticPrimitive::SetElements => "setElements",
                    SemanticPrimitive::SetUnion => "setUnion",
                    SemanticPrimitive::SetIntersection => "setIntersection",
                    SemanticPrimitive::SetDifference => "setDifference",
                    SemanticPrimitive::SetFold => "setFold",
                    SemanticPrimitive::ListFold => "listFold",
                    SemanticPrimitive::Iterate => "iterate",
                    SemanticPrimitive::IterateUntil => "iterateUntil",
                    SemanticPrimitive::GraphSuccessors => "graphSuccessors",
                    SemanticPrimitive::GraphReachable => "graphReachable",
                    SemanticPrimitive::LessThan => "lessThan",
                    _ => "graphTopological",
                };
                let mut out = format!("(LexLeanCollections.{name}");
                for argument in arguments {
                    out.push_str(" (");
                    out.push_str(&self.term(argument));
                    out.push(')');
                }
                out.push_str(" : ");
                out.push_str(&self.ty(result));
                out.push(')');
                return out;
            }
        };
        let mut out = format!("(LexLeanRuntime.{operation}");
        for argument in arguments {
            out.push_str(" (");
            out.push_str(&self.term(argument));
            out.push(')');
        }
        out.push_str(" : ");
        out.push_str(&self.ty(result));
        out.push(')');
        out
    }

    #[allow(clippy::too_many_lines)]
    fn term(&self, term: &SemanticTerm) -> String {
        let binary = |operator: &str, left: &SemanticTerm, right: &SemanticTerm| {
            format!("({} {operator} {})", self.term(left), self.term(right))
        };
        match term {
            SemanticTerm::Var { name } => identifier(name),
            SemanticTerm::Nat { value } => value.clone(),
            SemanticTerm::Integer {
                representation,
                value,
            } => format!("({value} : {representation:?})"),
            SemanticTerm::String { value } => string_literal(value),
            SemanticTerm::Bytes { hex } => {
                let values = hex
                    .as_bytes()
                    .chunks_exact(2)
                    .map(|pair| {
                        let pair = core::str::from_utf8(pair).expect("validated byte literal");
                        u8::from_str_radix(pair, 16).expect("validated byte literal")
                    })
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("ByteArray.mk #[{values}]")
            }
            SemanticTerm::Primitive {
                operation,
                arguments,
                result,
            } => self.primitive(*operation, arguments, result),
            SemanticTerm::Bool { value } => value.to_string(),
            SemanticTerm::Unit => "()".to_owned(),
            SemanticTerm::Nil { element } => format!("([] : List ({}))", self.ty(element)),
            SemanticTerm::Cons { head, tail } => {
                format!("({} :: {})", self.term(head), self.term(tail))
            }
            SemanticTerm::Record {
                r#type,
                type_arguments,
                fields,
            } => {
                let annotation = SemanticType::Named {
                    member: r#type.clone(),
                    arguments: type_arguments.clone(),
                };
                format!(
                    "({{ {} }} : {})",
                    self.assignments(fields),
                    self.ty(&annotation)
                )
            }
            SemanticTerm::Constructor {
                constructor,
                type_arguments: _,
                arguments,
            } => {
                if let (false, Some(pattern)) = (self.document, violation_pattern(constructor)) {
                    return format!("({pattern} : Prod Bool Bool)");
                }
                let mut out = self.member(constructor);
                for argument in arguments {
                    out.push_str(" (");
                    out.push_str(&self.term(argument));
                    out.push(')');
                }
                out
            }
            SemanticTerm::InstanceValue { resolved, .. } => self.member(resolved),
            SemanticTerm::Project { value, field } => {
                format!("({}).{}", self.term(value), identifier(field))
            }
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } => {
                let mut out = self.member(function);
                for argument in type_arguments {
                    out.push_str(" (");
                    out.push_str(&self.ty(argument));
                    out.push(')');
                }
                for argument in arguments {
                    out.push_str(" (");
                    out.push_str(&self.term(argument));
                    out.push(')');
                }
                out
            }
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => match self.hypotheses.get(&(std::ptr::from_ref(term) as usize)) {
                Some(index) => format!(
                    "(match (generalizing := false) {} : {} with | true => {} | false => {})",
                    hypothesis(*index),
                    self.term(condition),
                    self.term(then_value),
                    self.term(else_value)
                ),
                None => format!(
                    "(if {} then {} else {})",
                    self.term(condition),
                    self.term(then_value),
                    self.term(else_value)
                ),
            },
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => {
                let branches = branches
                    .iter()
                    .map(|branch| self.branch(branch))
                    .collect::<Vec<_>>()
                    .join(" ");
                match self.hypotheses.get(&(std::ptr::from_ref(term) as usize)) {
                    Some(index) => format!(
                        "(match (generalizing := false) {} : {} with {branches})",
                        hypothesis(*index),
                        self.term(scrutinee)
                    ),
                    None => format!("(match {} with {branches})", self.term(scrutinee)),
                }
            }
            SemanticTerm::Eq { left, right } => binary("=", left, right),
            SemanticTerm::Le { left, right } => binary("<=", left, right),
            SemanticTerm::Lt { left, right } => binary("<", left, right),
            SemanticTerm::Add { left, right } => binary("+", left, right),
            SemanticTerm::Beq { left, right } => {
                format!("(Nat.beq ({}) ({}))", self.term(left), self.term(right))
            }
            SemanticTerm::Ble { left, right } => {
                format!("(Nat.ble ({}) ({}))", self.term(left), self.term(right))
            }
            SemanticTerm::Blt { left, right } => {
                format!("(Nat.blt ({}) ({}))", self.term(left), self.term(right))
            }
            SemanticTerm::And { left, right } => binary("&&", left, right),
            SemanticTerm::PropAnd { left, right } => binary("/\\", left, right),
            SemanticTerm::Or { left, right } => binary("||", left, right),
            SemanticTerm::Not { value } => format!("(!{})", self.term(value)),
            SemanticTerm::Implies {
                premise,
                conclusion,
            } => binary("->", premise, conclusion),
            SemanticTerm::Iff { left, right } => binary("<->", left, right),
            SemanticTerm::Forall { binder, body } => format!(
                "(forall ({} : {}), {})",
                bound_name(&binder.name, term_uses(body, &binder.name)),
                self.ty(&binder.r#type),
                self.term(body)
            ),
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => format!(
                "(let {} : {} := {}; {})",
                bound_name(&binder.name, term_uses(body, &binder.name)),
                self.ty(&binder.r#type),
                self.term(value),
                self.term(body)
            ),
            SemanticTerm::Pair { left, right } => {
                format!("({}, {})", self.term(left), self.term(right))
            }
            SemanticTerm::First { value } => format!("({}).1", self.term(value)),
            SemanticTerm::Second { value } => format!("({}).2", self.term(value)),
            SemanticTerm::Lambda {
                parameters, body, ..
            } => format!(
                "(fun{} => {})",
                self.scoped_parameters(parameters, |name| term_uses(body, name)),
                self.term(body)
            ),
            SemanticTerm::Apply {
                function,
                arguments,
            } => {
                let mut out = format!("({}", self.term(function));
                for argument in arguments {
                    out.push_str(&format!(" ({})", self.term(argument)));
                }
                out.push(')');
                out
            }
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                let mut out = format!("({}", self.member(function));
                for argument in type_arguments {
                    out.push_str(&format!(" ({})", self.ty(argument)));
                }
                out.push(')');
                out
            }
            SemanticTerm::MapLiteral {
                key,
                value,
                entries,
            } => format!(
                "([{}] : List (Prod ({}) ({})))",
                entries
                    .iter()
                    .map(|entry| format!(
                        "({}, {})",
                        self.term(&entry.key),
                        self.term(&entry.value)
                    ))
                    .collect::<Vec<_>>()
                    .join(", "),
                self.ty(key),
                self.ty(value)
            ),
            SemanticTerm::SetLiteral { element, elements } => format!(
                "([{}] : List ({}))",
                elements
                    .iter()
                    .map(|element| self.term(element))
                    .collect::<Vec<_>>()
                    .join(", "),
                self.ty(element)
            ),
            // Linking elaborates every checked application before Lean is
            // rendered (§17.12); the document states the application itself.
            SemanticTerm::CheckedApply {
                model,
                type_arguments,
                arguments,
                checks,
            } => {
                let mut out = format!("(checked {}", self.member(model));
                for argument in type_arguments {
                    out.push_str(&format!(" ({})", self.ty(argument)));
                }
                for argument in arguments {
                    out.push_str(&format!(" ({})", self.term(argument)));
                }
                out.push_str(&format!(
                    " [{}])",
                    checks
                        .iter()
                        .map(|check| crate::ir::semantic::model::check_name_of(*check))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
                out
            }
            SemanticTerm::GraphLiteral { node, nodes, edges } => format!(
                "([{}] : List (Prod ({}) (List ({}))))",
                nodes
                    .iter()
                    .map(|source| {
                        let targets = edges
                            .iter()
                            .filter(|edge| edge.source == *source)
                            .map(|edge| self.term(&edge.target))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("({}, [{targets}])", self.term(source))
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
                self.ty(node),
                self.ty(node)
            ),
        }
    }

    fn proof_branch(&self, branch: &SemanticProofBranch, indent: usize) -> String {
        let binders = if branch.binders.is_empty() {
            String::new()
        } else {
            format!(
                " {}",
                branch
                    .binders
                    .iter()
                    .map(|name| identifier(name))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };
        format!(
            "{}| {}{} =>\n{}",
            "  ".repeat(indent),
            identifier(&branch.constructor),
            binders,
            self.proof(&branch.proof, indent + 1)
        )
    }

    fn proof(&self, proof: &SemanticProof, indent: usize) -> String {
        let pad = "  ".repeat(indent);
        match proof {
            SemanticProof::Reflexivity => format!("{pad}rfl\n"),
            SemanticProof::Decide => format!("{pad}decide\n"),
            SemanticProof::LinearArithmetic { definitions } => {
                let runtime = if self.runtime {
                    ", LexLeanRuntime.subtract, LexLeanRuntime.multiply"
                } else {
                    ""
                };
                if definitions.is_empty() {
                    format!(
                        "{pad}intros\n{pad}try set_option linter.unusedSimpArgs false in simp only [← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq{runtime}] at *\n{pad}all_goals omega\n"
                    )
                } else {
                    format!(
                        "{pad}intros\n{pad}subst_vars\n{pad}try set_option linter.unusedSimpArgs false in simp only [{}, ← Bool.not_eq_true, Bool.and_eq_true, Bool.or_eq_true, Bool.not_eq_true', and_true, true_and, Option.some.injEq, Nat.beq_eq, Nat.blt_eq, Nat.ble_eq{runtime}] at *\n{pad}all_goals omega\n",
                        definitions
                            .iter()
                            .map(|member| self.member(member))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            SemanticProof::Simplify { definitions } => format!(
                "{pad}simp only [{}]\n",
                definitions
                    .iter()
                    .map(|member| self.member(member))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            SemanticProof::Constructor { branches } => {
                let mut out = format!("{pad}constructor\n");
                for branch in branches {
                    out.push_str(&format!("{pad}·\n"));
                    out.push_str(&self.proof(branch, indent + 1));
                }
                out
            }
            SemanticProof::Cases {
                scrutinee,
                branches,
            } => {
                let mut out = format!("{pad}cases {} with\n", identifier(scrutinee));
                for branch in branches {
                    out.push_str(&self.proof_branch(branch, indent));
                }
                out
            }
            SemanticProof::Induction {
                scrutinee,
                generalizing,
                branches,
            } => {
                let generalizing = if generalizing.is_empty() {
                    String::new()
                } else {
                    format!(
                        " generalizing {}",
                        generalizing
                            .iter()
                            .map(|name| identifier(name))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                };
                let mut out = format!(
                    "{pad}induction {}{generalizing} with\n",
                    identifier(scrutinee)
                );
                for branch in branches {
                    out.push_str(&self.proof_branch(branch, indent));
                }
                out
            }
            SemanticProof::Congruence => format!("{pad}congr 1\n"),
            SemanticProof::BooleanReflection { reflection } => match reflection {
                SemanticReflection::List {
                    parameter,
                    values,
                    boolean_definition,
                    proposition_definition,
                    comparison,
                } => {
                    let parameter = identifier(parameter);
                    let values = identifier(values);
                    let boolean = self.member(boolean_definition);
                    let proposition = self.member(proposition_definition);
                    let mut out = format!(
                        "{pad}have llAndBridge : ∀ left right : Bool, ((left && right) = true) ↔ left = true ∧ right = true := by\n{pad}  intro left right\n{pad}  cases left <;> cases right <;> decide\n"
                    );
                    if matches!(comparison, SemanticReflectionComparison::NatBeq) {
                        out.push_str(&format!(
                            "{pad}have llBeqRefl : ∀ value : Nat, Nat.beq value value = true := by\n{pad}  intro value\n{pad}  induction value with\n{pad}  | zero => rfl\n{pad}  | succ value ih => exact ih\n"
                        ));
                    }
                    out.push_str(&format!(
                        "{pad}induction {values} generalizing {parameter} with\n{pad}| nil => constructor <;> intro _ <;> rfl\n{pad}| cons llValue llRest llIH =>\n{pad}  constructor\n{pad}  · intro h\n{pad}    have hpair := (llAndBridge _ _).mp h\n"
                    ));
                    match comparison {
                        SemanticReflectionComparison::NatBeq => out.push_str(&format!(
                            "{pad}    exact And.intro (Nat.eq_of_beq_eq_true hpair.left) ((llIH ({parameter} + 1)).mp hpair.right)\n{pad}  · intro h\n{pad}    have hleft : Nat.beq {parameter} llValue = true := h.left ▸ llBeqRefl {parameter}\n{pad}    have hright : {boolean} ({parameter} + 1) llRest = true := (llIH ({parameter} + 1)).mpr h.right\n{pad}    exact (llAndBridge _ _).mpr (And.intro hleft hright)\n"
                        )),
                        SemanticReflectionComparison::NatBlt => out.push_str(&format!(
                            "{pad}    exact And.intro (Nat.le_of_ble_eq_true hpair.left) ((llIH {parameter}).mp hpair.right)\n{pad}  · intro h\n{pad}    have hleft : Nat.blt llValue {parameter} = true := Nat.ble_eq_true_of_le h.left\n{pad}    have hright : {boolean} {parameter} llRest = true := (llIH {parameter}).mpr h.right\n{pad}    exact (llAndBridge _ _).mpr (And.intro hleft hright)\n"
                        )),
                    }
                    let _ = proposition;
                    out
                }
                SemanticReflection::Record {
                    record,
                    boolean_definition,
                    proposition_definition,
                    fields,
                } => {
                    let _ = (
                        self.member(boolean_definition),
                        self.member(proposition_definition),
                    );
                    format!(
                        "{pad}have llAndBridge : ∀ left right : Bool, ((left && right) = true) ↔ left = true ∧ right = true := by\n{pad}  intro left right\n{pad}  cases left <;> cases right <;> decide\n{pad}have llBeqRefl : ∀ value : Nat, Nat.beq value value = true := by\n{pad}  intro value\n{pad}  induction value with\n{pad}  | zero => rfl\n{pad}  | succ value ih => exact ih\n{pad}have llBeqBridge : ∀ left right : Nat, Nat.beq left right = true ↔ left = right := by\n{pad}  intro left right\n{pad}  constructor\n{pad}  · exact Nat.eq_of_beq_eq_true\n{pad}  · intro h\n{pad}    cases h\n{pad}    exact llBeqRefl left\n{pad}change (({}) = true) ↔ {}\n{pad}exact {}\n",
                        reflected_bool(record, fields),
                        reflected_prop(record, fields),
                        reflected_iff(fields)
                    )
                }
            },
            SemanticProof::Apply {
                theorem,
                type_arguments,
                arguments,
            } => {
                let arguments = type_arguments
                    .iter()
                    .map(|argument| format!(" ({})", self.ty(argument)))
                    .chain(
                        arguments
                            .iter()
                            .map(|argument| format!(" ({})", self.term(argument))),
                    )
                    .collect::<String>();
                format!("{pad}exact {}{arguments}\n", self.member(theorem))
            }
        }
    }
}

/// §17.12 well-founded lowering: a semireducible definition whose `if`
/// nodes bind numbered hypotheses, the declared measure, and one explicit
/// evidence application per recursive call site.
fn well_founded_definition(
    base: &Render<'_>,
    group: &BTreeSet<String>,
    declaration: &SemanticDeclaration,
) -> Result<String, Diagnostic> {
    let SemanticDeclaration::Definition {
        name,
        type_parameters,
        parameters,
        result,
        body,
        termination: Some(termination),
        ..
    } = declaration
    else {
        return Err(Diagnostic::new(
            code!("LLI9001"),
            "phase lean-backend: a well-founded lowering of a declaration without termination evidence",
        ));
    };
    let plan =
        crate::ir::semantic::well_founded_plan(name, group, type_parameters, parameters, body)
            .map_err(|reason| {
                Diagnostic::new(
                    code!("LLI9001"),
                    format!("phase lean-backend: unplanned well-founded definition: {reason}"),
                )
            })?;
    let render = Render {
        prefix: base.prefix,
        runtime: base.runtime,
        hypotheses: plan
            .addresses
            .iter()
            .enumerate()
            .map(|(index, address)| (*address, index))
            .collect(),
        document: false,
        qualify: base.qualify,
    };
    // A parameter the measure alone mentions is used: `termination_by`
    // references it.
    let used = |local: &str| term_uses(body, local) || term_uses(&termination.measure, local);
    let mut text = format!(
        "@[expose, semireducible] public def {}{}{} : {} := {}\ntermination_by {}\ndecreasing_by all_goals first",
        identifier(name),
        render.type_parameters(type_parameters),
        render.scoped_parameters(parameters, used),
        render.ty(result),
        render.term(body),
        render.term(&termination.measure)
    );
    for (site, evidence) in plan.sites.iter().zip(&termination.evidence) {
        let mut application = format!(" | (have __evidence := {}", render.member(evidence));
        for parameter in type_parameters {
            application.push_str(&format!(" ({})", identifier(parameter)));
        }
        for parameter in parameters {
            application.push_str(&format!(
                " ({})",
                bound_name(&parameter.name, used(&parameter.name))
            ));
        }
        // An enclosing match binder appears in its match's hypothesis, so
        // Lean solves it by unification; naming it would make a binder the
        // branch body ignores a use the linter cannot see.
        for _ in plan.binders(site) {
            application.push_str(" _");
        }
        for step in &site.path {
            application.push_str(&format!(" ({})", hypothesis(step.node())));
        }
        application.push_str("; subst_vars; exact __evidence)");
        text.push_str(&application);
    }
    text.push('\n');
    Ok(text)
}

/// Whether a module needs the fixed ordered-collection runtime (§17.12): it
/// writes a collection type or literal, or applies any collection primitive.
/// The primitive alone suffices, because a module that only measures or
/// folds a collection imported from another module still calls the runtime,
/// and the importer's runtime lives in the importer's namespace.
fn uses_collections(module: &serde_json::Value) -> bool {
    fn walk(value: &serde_json::Value) -> bool {
        match value {
            serde_json::Value::Array(items) => items.iter().any(walk),
            serde_json::Value::Object(object) => {
                let kind = object.get("kind").and_then(serde_json::Value::as_str);
                let collection = match kind {
                    Some("map" | "set" | "map_literal" | "set_literal" | "graph_literal") => true,
                    Some("primitive") => object
                        .get("operation")
                        .and_then(|operation| {
                            serde_json::from_value::<SemanticPrimitive>(operation.clone()).ok()
                        })
                        .is_some_and(SemanticPrimitive::language_1_2),
                    _ => false,
                };
                collection || object.values().any(walk)
            }
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => false,
        }
    }
    walk(module)
}

/// The fixed ordered-collection runtime: canonical ascending entry and
/// element lists keyed by the closed total orders, ordered folds, bounded
/// iteration, and graph traversal bounded by the node count (§17.12). A
/// graph's nodes are its keys and every successor, so a successor that was
/// inserted without its own entry is a node with no successors rather than
/// a name the traversals disagree about.
const COLLECTIONS_RUNTIME: &str = r#"
namespace LexLeanCollections

public class Key (α : Type) where
  compare : α -> α -> Ordering

@[expose] public def compareCodes : List Char -> List Char -> Ordering
  | [], [] => .eq
  | [], _ :: _ => .lt
  | _ :: _, [] => .gt
  | left :: lefts, right :: rights =>
    match Ord.compare left.val.toNat right.val.toNat with
    | .eq => compareCodes lefts rights
    | other => other

public instance : Key Nat where compare := Ord.compare
public instance : Key Int where compare := Ord.compare
public instance : Key Bool where compare := Ord.compare
public instance : Key Int8 where compare left right := Ord.compare left.toInt right.toInt
public instance : Key Int16 where compare left right := Ord.compare left.toInt right.toInt
public instance : Key Int32 where compare left right := Ord.compare left.toInt right.toInt
public instance : Key Int64 where compare left right := Ord.compare left.toInt right.toInt
public instance : Key UInt8 where compare left right := Ord.compare left.toNat right.toNat
public instance : Key UInt16 where compare left right := Ord.compare left.toNat right.toNat
public instance : Key UInt32 where compare left right := Ord.compare left.toNat right.toNat
public instance : Key UInt64 where compare left right := Ord.compare left.toNat right.toNat
public instance : Key String where compare left right := compareCodes left.toList right.toList
public instance {α β : Type} [Key α] [Key β] : Key (Prod α β) where
  compare left right := match Key.compare left.1 right.1 with
    | .eq => Key.compare left.2 right.2
    | other => other

@[expose] public def insertEntry {κ ν : Type} [Key κ] (key : κ) (value : ν) : List (Prod κ ν) -> List (Prod κ ν)
  | [] => [(key, value)]
  | (other, stored) :: rest => match Key.compare key other with
    | .lt => (key, value) :: (other, stored) :: rest
    | .eq => (key, value) :: rest
    | .gt => (other, stored) :: insertEntry key value rest

@[expose] public def removeEntry {κ ν : Type} [Key κ] (key : κ) : List (Prod κ ν) -> List (Prod κ ν)
  | [] => []
  | (other, stored) :: rest => match Key.compare key other with
    | .lt => (other, stored) :: rest
    | .eq => rest
    | .gt => (other, stored) :: removeEntry key rest

@[expose] public def lookupEntry {κ ν : Type} [Key κ] (key : κ) : List (Prod κ ν) -> Option ν
  | [] => none
  | (other, stored) :: rest => match Key.compare key other with
    | .lt => none
    | .eq => some stored
    | .gt => lookupEntry key rest

@[expose] public def insertElement {κ : Type} [Key κ] (key : κ) : List κ -> List κ
  | [] => [key]
  | other :: rest => match Key.compare key other with
    | .lt => key :: other :: rest
    | .eq => other :: rest
    | .gt => other :: insertElement key rest

@[expose] public def removeElement {κ : Type} [Key κ] (key : κ) : List κ -> List κ
  | [] => []
  | other :: rest => match Key.compare key other with
    | .lt => other :: rest
    | .eq => rest
    | .gt => other :: removeElement key rest

@[expose] public def containsElement {κ : Type} [Key κ] (key : κ) : List κ -> Bool
  | [] => false
  | other :: rest => match Key.compare key other with
    | .lt => false
    | .eq => true
    | .gt => containsElement key rest

@[expose] public def mapInsert {κ ν : Type} [Key κ] (map : List (Prod κ ν)) (key : κ) (value : ν) : List (Prod κ ν) := insertEntry key value map
@[expose] public def mapRemove {κ ν : Type} [Key κ] (map : List (Prod κ ν)) (key : κ) : List (Prod κ ν) := removeEntry key map
@[expose] public def mapLookup {κ ν : Type} [Key κ] (map : List (Prod κ ν)) (key : κ) : Option ν := lookupEntry key map
@[expose] public def mapContains {κ ν : Type} [Key κ] (map : List (Prod κ ν)) (key : κ) : Bool := (lookupEntry key map).isSome
@[expose] public def mapSize {κ ν : Type} (map : List (Prod κ ν)) : Nat := map.length
@[expose] public def mapKeys {κ ν : Type} (map : List (Prod κ ν)) : List κ := map.map Prod.fst
@[expose] public def mapValues {κ ν : Type} (map : List (Prod κ ν)) : List ν := map.map Prod.snd
@[expose] public def mapEntries {κ ν : Type} (map : List (Prod κ ν)) : List (Prod κ ν) := map
@[expose] public def mapFold {κ ν β : Type} (step : β -> κ -> ν -> β) (initial : β) (map : List (Prod κ ν)) : β :=
  map.foldl (fun state entry => step state entry.1 entry.2) initial

@[expose] public def setInsert {κ : Type} [Key κ] (set : List κ) (key : κ) : List κ := insertElement key set
@[expose] public def setRemove {κ : Type} [Key κ] (set : List κ) (key : κ) : List κ := removeElement key set
@[expose] public def setContains {κ : Type} [Key κ] (set : List κ) (key : κ) : Bool := containsElement key set
@[expose] public def setSize {κ : Type} (set : List κ) : Nat := set.length
@[expose] public def setElements {κ : Type} (set : List κ) : List κ := set
@[expose] public def setUnion {κ : Type} [Key κ] (left right : List κ) : List κ := right.foldl (fun acc key => insertElement key acc) left
@[expose] public def setIntersection {κ : Type} [Key κ] (left right : List κ) : List κ := left.filter (fun key => containsElement key right)
@[expose] public def setDifference {κ : Type} [Key κ] (left right : List κ) : List κ := left.filter (fun key => !containsElement key right)

@[expose] public def graphSuccessors {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) (node : κ) : List κ :=
  (lookupEntry node graph).getD []

@[expose] public def graphNodes {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) : List κ :=
  graph.foldl (fun acc entry => entry.2.foldl (fun inner node => insertElement node inner) (insertElement entry.1 acc)) []

@[expose] public def reachableFrom {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) : Nat -> List κ -> List κ -> List κ
  | 0, _, seen => seen
  | Nat.succ fuel, frontier, seen =>
    let next := frontier.foldl (fun acc node =>
      (graphSuccessors graph node).foldl (fun acc2 succ =>
        if containsElement succ seen || containsElement succ acc2 then acc2 else insertElement succ acc2) acc) []
    match next with
    | [] => seen
    | _ => reachableFrom graph fuel next (next.foldl (fun acc key => insertElement key acc) seen)

@[expose] public def graphReachable {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) (start : κ) : List κ :=
  reachableFrom graph ((graphNodes graph).length + 1) [start] [start]

@[expose] public def topological {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) : Nat -> List κ -> List κ -> Option (List κ)
  | 0, remaining, order => if remaining.isEmpty then some order.reverse else none
  | Nat.succ fuel, remaining, order =>
    match remaining.filter (fun node => remaining.all (fun other => !containsElement node (graphSuccessors graph other))) with
    | [] => if remaining.isEmpty then some order.reverse else none
    | ready :: _ => topological graph fuel (removeElement ready remaining) (ready :: order)

@[expose] public def graphTopological {κ : Type} [Key κ] (graph : List (Prod κ (List κ))) : Option (List κ) :=
  let nodes := graphNodes graph
  topological graph (nodes.length + 1) nodes []
@[expose] public def listFold {α σ : Type} (step : σ -> α -> σ) (initial : σ) (values : List α) : σ :=
  values.foldl step initial

@[expose] public def setFold {κ σ : Type} (step : σ -> κ -> σ) (initial : σ) (set : List κ) : σ :=
  set.foldl step initial

@[expose] public def iterate {σ : Type} (step : σ -> σ) : Nat -> σ -> σ
  | 0, state => state
  | Nat.succ count, state => iterate step count (step state)

@[expose] public def iterateUntil {σ : Type} (step : σ -> Option σ) : Nat -> σ -> Prod σ Bool
  | 0, state => (state, false)
  | Nat.succ fuel, state => match step state with
    | none => (state, true)
    | some next => iterateUntil step fuel next

@[expose] public def lessThan {κ : Type} [Key κ] (left right : κ) : Bool :=
  match Key.compare left right with
  | .lt => true
  | _ => false
end LexLeanCollections
"#;

/// The fixed model runtime (§17.12, models): the meaning of every claim,
/// validator link, and artifact decoding, written once, independently of
/// the obligation generator. Each cross-check restates a user theorem, or
/// recomputes an artifact, against these definitions, so Lean refuses a
/// generator that states anything else. Plain predicates only: no type is
/// declared here, so nothing has a per-module identity.
const MODELS_RUNTIME: &str = r#"
namespace LexLeanModels

@[expose] public def Sound1 {α : Type} (v : α -> Bool) (p : α -> Prop) : Prop :=
  forall (a : α), v a = true -> p a
@[expose] public def Sound2 {α β : Type} (v : α -> β -> Bool) (p : α -> β -> Prop) : Prop :=
  forall (a : α) (b : β), v a b = true -> p a b
@[expose] public def Sound4 {α β γ δ : Type} (v : α -> β -> γ -> δ -> Bool) (p : α -> β -> γ -> δ -> Prop) : Prop :=
  forall (a : α) (b : β) (c : γ) (d : δ), v a b c d = true -> p a b c d
@[expose] public def Complete1 {α : Type} (v : α -> Bool) (p : α -> Prop) : Prop :=
  forall (a : α), p a -> v a = true
@[expose] public def Complete2 {α β : Type} (v : α -> β -> Bool) (p : α -> β -> Prop) : Prop :=
  forall (a : α) (b : β), p a b -> v a b = true
@[expose] public def Complete4 {α β γ δ : Type} (v : α -> β -> γ -> δ -> Bool) (p : α -> β -> γ -> δ -> Prop) : Prop :=
  forall (a : α) (b : β) (c : γ) (d : δ), p a b c d -> v a b c d = true

@[expose] public def Satisfies {α β : Type} (p : α -> Prop) (q : α -> β -> Prop) (r : α -> β) : Prop :=
  forall (a : α), p a -> q a (r a)
@[expose] public def SatisfiesTotal {α β : Type} (q : α -> β -> Prop) (r : α -> β) : Prop :=
  forall (a : α), q a (r a)
@[expose] public def SatisfiesStep {σ α β : Type} (j : σ -> Prop) (p : σ -> α -> Prop) (q : σ -> α -> σ -> β -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> p s a -> q s a (r s a).1 (r s a).2
@[expose] public def SatisfiesStepNoInvariant {σ α β : Type} (p : σ -> α -> Prop) (q : σ -> α -> σ -> β -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), p s a -> q s a (r s a).1 (r s a).2
@[expose] public def SatisfiesStepNoPrecondition {σ α β : Type} (j : σ -> Prop) (q : σ -> α -> σ -> β -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> q s a (r s a).1 (r s a).2
@[expose] public def SatisfiesStepTotal {σ α β : Type} (q : σ -> α -> σ -> β -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), q s a (r s a).1 (r s a).2
@[expose] public def Preserves {σ α β : Type} (j : σ -> Prop) (p : σ -> α -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> p s a -> j (r s a).1
@[expose] public def PreservesTotal {σ α β : Type} (j : σ -> Prop) (r : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> j (r s a).1
@[expose] public def Initial {σ : Type} (j : σ -> Prop) (i : σ) : Prop :=
  j i
@[expose] public def Equivalent {α β : Type} (p : α -> Prop) (r f : α -> β) : Prop :=
  forall (a : α), p a -> r a = f a
@[expose] public def EquivalentTotal {α β : Type} (r f : α -> β) : Prop :=
  forall (a : α), r a = f a
@[expose] public def EquivalentStep {σ α β : Type} (j : σ -> Prop) (p : σ -> α -> Prop) (r f : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> p s a -> r s a = f s a
@[expose] public def EquivalentStepNoInvariant {σ α β : Type} (p : σ -> α -> Prop) (r f : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), p s a -> r s a = f s a
@[expose] public def EquivalentStepNoPrecondition {σ α β : Type} (j : σ -> Prop) (r f : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), j s -> r s a = f s a
@[expose] public def EquivalentStepTotal {σ α β : Type} (r f : σ -> α -> Prod σ β) : Prop :=
  forall (s : σ) (a : α), r s a = f s a
@[expose, reducible] public def Agreement {α β : Type} (r : α -> β) (cmp : β -> β -> Bool) (d : List (Prod α β)) (examples agreements : Nat) : Prop :=
  d.length = examples /\ d.foldl (fun c e => if cmp (r e.1) e.2 then c + 1 else c) 0 = agreements

@[expose] public def inRange (width : Nat) (signed : Bool) (value : Int) : Bool :=
  if signed then
    decide (-((2 : Int) ^ (8 * width - 1)) <= value) && decide (value < (2 : Int) ^ (8 * width - 1))
  else
    decide (0 <= value) && decide (value < (2 : Int) ^ (8 * width))
@[expose] public def encodeInt (width : Nat) (value : Int) : List Nat :=
  let unsigned := if value < 0 then value + (2 : Int) ^ (8 * width) else value
  (List.range width).map (fun i => (unsigned.toNat / 256 ^ i) % 256)
@[expose] public def tensorMatches (width : Nat) (signed : Bool) (values : List Int) (bytes : ByteArray) : Bool :=
  values.all (inRange width signed) && values.flatMap (encodeInt width) == bytes.data.toList.map UInt8.toNat
@[expose] public def utf8Char (c : Char) : List Nat :=
  let n := c.toNat
  if n < 128 then [n]
  else if n < 2048 then [192 + n / 64, 128 + n % 64]
  else if n < 65536 then [224 + n / 4096, 128 + (n / 64) % 64, 128 + n % 64]
  else [240 + n / 262144, 128 + (n / 4096) % 64, 128 + (n / 64) % 64, 128 + n % 64]
@[expose] public def linesMatch (lines : List String) (bytes : ByteArray) : Bool :=
  lines.all (fun line => line.toList.all (fun c => c.toNat != 10 && c.toNat != 13)) &&
    lines.foldr (fun line rest => line.toList.foldr (fun c tail => utf8Char c ++ tail) [10] ++ rest) [] ==
      bytes.data.toList.map UInt8.toNat
end LexLeanModels
"#;

/// The fixed reasoning runtime (§17.12, reasoning): the propositions a
/// generated statement names and the lemmas every generated proof applies,
/// written once and independently of the elaborator. Every lemma is proved
/// by structural recursion without `simp` or a core list lemma, so it is
/// axiom-free; a generated proof applies only these and its declaration's
/// own theorems. Nothing here runs: the runtime is formal-only.
const REASONING_RUNTIME: &str = r#"
namespace LexLeanReasoning

public inductive Star {σ : Type} (r : σ -> σ -> Prop) : σ -> σ -> Prop where
  | refl (a : σ) : Star r a a
  | tail (a b c : σ) : Star r a b -> r b c -> Star r a c

public inductive All {α : Type} (p : α -> Prop) : List α -> Prop where
  | nil : All p []
  | cons (a : α) (rest : List α) : p a -> All p rest -> All p (a :: rest)

@[expose] public def Preserves {σ : Type} (r : σ -> σ -> Prop) (j : σ -> Prop) : Prop :=
  forall (s t : σ), j s -> r s t -> j t
@[expose] public def Sound {α β : Type} (v : α -> β -> Bool) (p : α -> β -> Prop) : Prop :=
  forall (a : α) (b : β), v a b = true -> p a b
@[expose] public def Complete {α β : Type} (v : α -> β -> Bool) (p : α -> β -> Prop) : Prop :=
  forall (a : α) (b : β), p a b -> v a b = true
@[expose] public def Reaches {σ ε : Type} (r : σ -> σ -> Prop) (a : σ) (acc : Except ε σ) : Prop :=
  forall (s : σ), acc = Except.ok s -> Star r a s
@[expose] public def Found {ν ρ : Type} (ok : ν -> Prop) (accept : ν -> Option ρ) : Option (Prod ρ ν) -> Prop
  | none => True
  | some hit => ok hit.2 /\ accept hit.2 = some hit.1
@[expose] public def SearchOk {ν ρ : Type} (ok : ν -> Prop) (accept : ν -> Option ρ) (frontier : List ν) (found : Option (Prod ρ ν)) : Prop :=
  All ok frontier /\ Found ok accept found

public theorem starPreserves {σ : Type} (r : σ -> σ -> Prop) (j : σ -> Prop) (h : Preserves r j) :
    forall (a b : σ), Star r a b -> j a -> j b := by
  intro a b hs
  induction hs with
  | refl => exact fun ha => ha
  | tail y z _ hyz ih => exact fun ha => h y z (ih ha) hyz

public theorem reachesStart {σ ε : Type} (r : σ -> σ -> Prop) (a : σ) : Reaches (ε := ε) r a (Except.ok a) := by
  intro s e
  cases e
  exact Star.refl a

public theorem guarded {σ : Type} (r : σ -> σ -> Prop) (s : σ) (g : Bool) (c : σ) (h : g = true -> r s c) (t : σ) :
    (if g then some c else none) = some t -> r s t := by
  cases g with
  | false => intro e; cases e
  | true => intro e; cases e; exact h rfl

public theorem guardedRank {σ : Type} (μ : σ -> Nat) (s : σ) (g : Bool) (c : σ) (h : g = true -> μ c < μ s) (t : σ) :
    (if g then some c else none) = some t -> μ t < μ s := by
  cases g with
  | false => intro e; cases e
  | true => intro e; cases e; exact h rfl

public theorem checked {ρ : Type} (g : Bool) (a b : ρ) : (if g then some a else none) = some b -> g = true /\ a = b := by
  cases g with
  | false => intro e; cases e
  | true => intro e; cases e; exact And.intro rfl rfl

public theorem foldInvariant {σ α : Type} (f : σ -> α -> σ) (p : σ -> Prop) (h : forall (a : σ) (x : α), p a -> p (f a x)) :
    forall (xs : List α) (a : σ), p a -> p (LexLeanCollections.listFold f a xs) := by
  intro xs
  induction xs with
  | nil => exact fun _ ha => ha
  | cons x rest ih => exact fun a ha => ih (f a x) (h a x ha)

public theorem foldSnoc {σ α : Type} (f : σ -> α -> σ) (a : σ) (xs : List α) (y : α) :
    LexLeanCollections.listFold f a (LexLeanRuntime.append xs (y :: [])) = f (LexLeanCollections.listFold f a xs) y := by
  induction xs generalizing a with
  | nil => rfl
  | cons x rest ih => exact ih (f a x)

public theorem allAppend {α : Type} (p : α -> Prop) : forall (xs ys : List α), All p xs -> All p ys -> All p (LexLeanRuntime.append xs ys) := by
  intro xs
  induction xs with
  | nil => exact fun _ _ h => h
  | cons x rest ih =>
    intro ys hx hy
    cases hx with
    | cons _ _ hp hr => exact All.cons x (LexLeanRuntime.append rest ys) hp (ih ys hr hy)

public theorem allSingle {α : Type} (p : α -> Prop) (a : α) (h : p a) : All p (a :: []) :=
  All.cons a [] h All.nil

public theorem allHead {α : Type} (p : α -> Prop) (a : α) (rest : List α) (h : All p (a :: rest)) : p a := by
  cases h with
  | cons _ _ hp _ => exact hp

public theorem allTail {α : Type} (p : α -> Prop) (a : α) (rest : List α) (h : All p (a :: rest)) : All p rest := by
  cases h with
  | cons _ _ _ hr => exact hr

public theorem allFold {σ α : Type} (f : σ -> α -> σ) (p : σ -> Prop) (q : α -> Prop) (h : forall (a : σ) (x : α), q x -> p a -> p (f a x)) :
    forall (xs : List α) (a : σ), All q xs -> p a -> p (LexLeanCollections.listFold f a xs) := by
  intro xs
  induction xs with
  | nil => exact fun _ _ ha => ha
  | cons x rest ih =>
    intro a hx ha
    cases hx with
    | cons _ _ hq hr => exact ih (f a x) hr (h a x hq ha)

public theorem capAll {α : Type} (p : α -> Prop) (k : Nat) (xs : List α) (h : All p xs) :
    All p (LexLeanCollections.listFold (fun (acc : List α) (n : α) => if Nat.blt (LexLeanRuntime.length acc) k then LexLeanRuntime.append acc (n :: []) else acc) [] xs) :=
  allFold (fun (acc : List α) (n : α) => if Nat.blt (LexLeanRuntime.length acc) k then LexLeanRuntime.append acc (n :: []) else acc) (All p) p
    (fun a x hq ha => by
      show All p (if Nat.blt (LexLeanRuntime.length a) k then LexLeanRuntime.append a (x :: []) else a)
      cases Nat.blt (LexLeanRuntime.length a) k with
      | false => exact ha
      | true => exact allAppend p a (x :: []) ha (allSingle p x hq)) xs [] h All.nil

public theorem lengthSnoc {α : Type} (xs : List α) (y : α) : LexLeanRuntime.length (LexLeanRuntime.append xs (y :: [])) = LexLeanRuntime.length xs + 1 := by
  induction xs with
  | nil => rfl
  | cons x rest ih => exact congrArg Nat.succ ih

public theorem capBound {α : Type} (k : Nat) (xs : List α) :
    LexLeanRuntime.length (LexLeanCollections.listFold (fun (acc : List α) (n : α) => if Nat.blt (LexLeanRuntime.length acc) k then LexLeanRuntime.append acc (n :: []) else acc) [] xs) <= k :=
  foldInvariant (fun (acc : List α) (n : α) => if Nat.blt (LexLeanRuntime.length acc) k then LexLeanRuntime.append acc (n :: []) else acc) (fun (acc : List α) => LexLeanRuntime.length acc <= k)
    (fun a x ha => by
      show LexLeanRuntime.length (if Nat.blt (LexLeanRuntime.length a) k then LexLeanRuntime.append a (x :: []) else a) <= k
      cases e : Nat.blt (LexLeanRuntime.length a) k with
      | false => exact ha
      | true =>
        show LexLeanRuntime.length (LexLeanRuntime.append a (x :: [])) <= k
        rw [lengthSnoc]
        exact Nat.le_of_ble_eq_true e) xs [] (Nat.zero_le k)

public theorem freshAll {ν κ : Type} [LexLeanCollections.Key κ] (key : ν -> κ) (p : ν -> Prop) (v : List κ) (ns : List ν) (h : All p ns) :
    All p (LexLeanCollections.listFold (fun (acc : Prod (List ν) (List κ)) (n : ν) => if LexLeanCollections.setContains acc.2 (key n) then acc else (LexLeanRuntime.append acc.1 (n :: []), LexLeanCollections.setInsert acc.2 (key n))) (([] : List ν), v) ns).1 :=
  allFold (fun (acc : Prod (List ν) (List κ)) (n : ν) => if LexLeanCollections.setContains acc.2 (key n) then acc else (LexLeanRuntime.append acc.1 (n :: []), LexLeanCollections.setInsert acc.2 (key n))) (fun (acc : Prod (List ν) (List κ)) => All p acc.1) p
    (fun a x hq ha => by
      show All p (if LexLeanCollections.setContains a.2 (key x) then a else (LexLeanRuntime.append a.1 (x :: []), LexLeanCollections.setInsert a.2 (key x))).1
      cases LexLeanCollections.setContains a.2 (key x) with
      | true => exact ha
      | false => exact allAppend p a.1 (x :: []) ha (allSingle p x hq)) ns (([] : List ν), v) h All.nil

public theorem searchStart {ν ρ : Type} (ok : ν -> Prop) (accept : ν -> Option ρ) (frontier : List ν) (h : All ok frontier) :
    SearchOk ok accept frontier none :=
  And.intro h True.intro

public theorem peakBound (p len k : Nat) (hp : p <= k) (hl : len <= k) : (if Nat.blt p len then len else p) <= k := by
  cases Nat.blt p len with
  | false => exact hp
  | true => exact hl

public theorem iterateUntilInvariant {σ : Type} (step : σ -> Option σ) (p : σ -> Prop) (h : forall (a b : σ), step a = some b -> p a -> p b) :
    forall (n : Nat) (a : σ), p a -> p (LexLeanCollections.iterateUntil step n a).1 := by
  intro n
  induction n with
  | zero => exact fun _ ha => ha
  | succ n ih =>
    intro a ha
    show p (match step a with | none => (a, true) | some next => LexLeanCollections.iterateUntil step n next).1
    cases e : step a with
    | none => exact ha
    | some b => exact ih b (h a b e ha)

public theorem iterateUntilSimulate {σ τ : Type} (f : σ -> Option σ) (g : τ -> Option τ) (π : σ -> τ)
    (hnone : forall (a : σ), f a = none -> g (π a) = none)
    (hsome : forall (a b : σ), f a = some b -> g (π a) = some (π b)) :
    forall (n : Nat) (a : σ), π (LexLeanCollections.iterateUntil f n a).1 = (LexLeanCollections.iterateUntil g n (π a)).1 /\ (LexLeanCollections.iterateUntil f n a).2 = (LexLeanCollections.iterateUntil g n (π a)).2 := by
  intro n
  induction n with
  | zero => exact fun _ => And.intro rfl rfl
  | succ n ih =>
    intro a
    show π (match f a with | none => (a, true) | some next => LexLeanCollections.iterateUntil f n next).1 = (match g (π a) with | none => (π a, true) | some next => LexLeanCollections.iterateUntil g n next).1 /\ (match f a with | none => (a, true) | some next => LexLeanCollections.iterateUntil f n next).2 = (match g (π a) with | none => (π a, true) | some next => LexLeanCollections.iterateUntil g n next).2
    cases e : f a with
    | none => rw [hnone a e]; exact And.intro rfl rfl
    | some b => rw [hsome a b e]; exact ih b

public theorem iterateUntilCount {σ : Type} (step : σ -> Option σ) (c : σ -> Nat) (h : forall (a b : σ), step a = some b -> c b = c a + 1) :
    forall (n : Nat) (a : σ), c a = 0 -> c (LexLeanCollections.iterateUntil step n a).1 <= n := by
  have general : forall (n : Nat) (a : σ), c (LexLeanCollections.iterateUntil step n a).1 <= c a + n := by
    intro n
    induction n with
    | zero => exact fun a => Nat.le_refl (c a)
    | succ n ih =>
      intro a
      show c (match step a with | none => (a, true) | some next => LexLeanCollections.iterateUntil step n next).1 <= c a + (n + 1)
      cases e : step a with
      | none => exact Nat.le_add_right (c a) (n + 1)
      | some b =>
        have hb := ih b
        rw [h a b e] at hb
        rw [Nat.add_right_comm] at hb
        exact (Nat.add_assoc (c a) n 1) ▸ (Nat.add_right_comm (c a) 1 n) ▸ hb
  intro n a h0
  have hg := general n a
  rw [h0, Nat.zero_add] at hg
  exact hg

public theorem iterateUntilGrowth {σ : Type} (step : σ -> Option σ) (c : σ -> Nat) (h : forall (a b : σ), step a = some b -> c b <= c a + 1) :
    forall (n : Nat) (a : σ), c a = 0 -> c (LexLeanCollections.iterateUntil step n a).1 <= n := by
  have general : forall (n : Nat) (a : σ), c (LexLeanCollections.iterateUntil step n a).1 <= c a + n := by
    intro n
    induction n with
    | zero => exact fun a => Nat.le_refl (c a)
    | succ n ih =>
      intro a
      show c (match step a with | none => (a, true) | some next => LexLeanCollections.iterateUntil step n next).1 <= c a + (n + 1)
      cases e : step a with
      | none => exact Nat.le_add_right (c a) (n + 1)
      | some b =>
        have hb := Nat.le_trans (ih b) (Nat.add_le_add_right (h a b e) n)
        rw [Nat.add_right_comm] at hb
        exact hb
  intro n a h0
  have hg := general n a
  rw [h0, Nat.zero_add] at hg
  exact hg

public theorem iterateUntilStops {σ : Type} (step : σ -> Option σ) (p : σ -> Prop) (μ : σ -> Nat)
    (h : forall (a b : σ), step a = some b -> p a -> p b /\ μ b < μ a) :
    forall (n : Nat) (a : σ), p a -> μ a < n -> (LexLeanCollections.iterateUntil step n a).2 = true := by
  intro n
  induction n with
  | zero => intro a _ hm; exact absurd hm (Nat.not_lt_zero (μ a))
  | succ n ih =>
    intro a ha hm
    show (match step a with | none => (a, true) | some next => LexLeanCollections.iterateUntil step n next).2 = true
    cases e : step a with
    | none => rfl
    | some b =>
      have hb := h a b e ha
      exact ih b hb.left (Nat.lt_of_lt_of_le hb.right (Nat.le_of_lt_succ hm))

public theorem iterateUntilBound {σ : Type} (step : σ -> Option σ) (c : σ -> Nat) (k : Nat) (h : forall (a b : σ), step a = some b -> c a <= k -> c b <= k) :
    forall (n : Nat) (a : σ), c a <= k -> c (LexLeanCollections.iterateUntil step n a).1 <= k :=
  iterateUntilInvariant step (fun (a : σ) => c a <= k) h
public theorem noneSome {ρ : Type} {q : Prop} (v : ρ) (h : (none : Option ρ) = some v) : q := by
  cases h

public theorem zeroLe (n : Nat) : 0 <= n :=
  Nat.zero_le n

public theorem bltSucc (a b : Nat) (h : Nat.blt a b = true) : a + 1 <= b :=
  Nat.le_of_ble_eq_true h

end LexLeanReasoning
"#;

fn emit(checked: &CheckedModule, text: &str, kind: &str) -> Emitter {
    let mut emitter = Emitter::new();
    let node = emitter.node(kind);
    emitter.piece(
        text,
        kind,
        Origin::Metadata {
            owner: "lexlean.core::semanticdata".to_owned(),
        },
        EmitSource::File(0, checked.normalized.len()),
        MapRole::Declaration,
        node,
    );
    emitter
}

fn skip_whitespace(bytes: &[u8], mut position: usize) -> usize {
    while bytes
        .get(position)
        .is_some_and(|byte| matches!(byte, b' ' | b'\n' | b'\r' | b'\t'))
    {
        position += 1;
    }
    position
}

/// The end of the JSON value starting at `position`.
fn skip_value(bytes: &[u8], mut position: usize) -> Option<usize> {
    let (mut depth, mut in_string, mut escaped) = (0usize, false, false);
    loop {
        let byte = *bytes.get(position)?;
        if in_string {
            position += 1;
            match byte {
                _ if escaped => escaped = false,
                b'\\' => escaped = true,
                b'"' => {
                    in_string = false;
                    if depth == 0 {
                        return Some(position);
                    }
                }
                _ => {}
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => depth += 1,
            // A closing bracket at depth zero ends the scalar before it.
            b'}' | b']' if depth == 0 => return Some(position),
            b'}' | b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(position + 1);
                }
            }
            b',' | b' ' | b'\n' | b'\r' | b'\t' if depth == 0 => return Some(position),
            _ => {}
        }
        position += 1;
    }
}

/// The byte range of every element of the `declarations` array of the
/// module's semantic data in the normalized source, in order.
fn declaration_spans(source: &str) -> Option<Vec<(usize, usize)>> {
    let bytes = source.as_bytes();
    let data = source.find("\\semanticdata{")? + "\\semanticdata{".len();
    let mut position = skip_whitespace(bytes, data);
    if bytes.get(position) != Some(&b'{') {
        return None;
    }
    position += 1;
    loop {
        position = skip_whitespace(bytes, position);
        let key_end = skip_value(bytes, position)?;
        let key = source.get(position..key_end)?;
        position = skip_whitespace(bytes, key_end);
        if bytes.get(position) != Some(&b':') {
            return None;
        }
        position = skip_whitespace(bytes, position + 1);
        if key == "\"declarations\"" {
            break;
        }
        position = skip_whitespace(bytes, skip_value(bytes, position)?);
        if bytes.get(position) != Some(&b',') {
            return None;
        }
        position += 1;
    }
    if bytes.get(position) != Some(&b'[') {
        return None;
    }
    position += 1;
    let mut spans = Vec::new();
    loop {
        position = skip_whitespace(bytes, position);
        match bytes.get(position)? {
            b']' => return Some(spans),
            b',' if !spans.is_empty() => position = skip_whitespace(bytes, position + 1),
            _ if spans.is_empty() => {}
            _ => return None,
        }
        let end = skip_value(bytes, position)?;
        spans.push((position, end));
        position = end;
    }
}

/// Language 1.2: one mapping node per declaration, relating its generated
/// text to its own object in the source, inside the module node that
/// relates the preamble and closing to the whole module.
fn emit_declarations(
    checked: &CheckedModule,
    text: &str,
    kind: &str,
    starts: &[usize],
    end: usize,
) -> Result<Emitter, Diagnostic> {
    let spans = declaration_spans(&checked.normalized)
        .filter(|spans| spans.len() == starts.len())
        .ok_or_else(|| {
            Diagnostic::new(
                code!("LLI9001"),
                format!("phase {kind}: the source declarations do not match the rendered ones"),
            )
        })?;
    let origin = || Origin::Metadata {
        owner: "lexlean.core::semanticdata".to_owned(),
    };
    let whole = EmitSource::File(0, checked.normalized.len());
    let mut emitter = Emitter::new();
    let module_node = emitter.node(kind);
    let first = starts.first().copied().unwrap_or(end);
    emitter.piece(
        &text[..first],
        kind,
        origin(),
        whole.clone(),
        MapRole::Declaration,
        module_node,
    );
    for (index, (start, span)) in starts.iter().zip(&spans).enumerate() {
        let stop = starts.get(index + 1).copied().unwrap_or(end);
        let node = emitter.node("semantic-declaration");
        emitter.piece(
            &text[*start..stop],
            kind,
            origin(),
            EmitSource::File(span.0, span.1),
            MapRole::Declaration,
            node,
        );
    }
    emitter.piece(
        &text[end..],
        kind,
        origin(),
        whole,
        MapRole::Declaration,
        module_node,
    );
    Ok(emitter)
}

/// The language-1.2 portable runtime: the frozen runtime with every
/// definition exposed (§17.12). An unexposed definition's body is invisible
/// to every other module under Lean's module system, so a definition
/// imported from another module could not reduce through the primitives it
/// applies. `noinline` is kept on every definition that had it.
fn portable_runtime_1_2() -> &'static str {
    static RUNTIME: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    RUNTIME.get_or_init(|| {
        portable_runtime()
            .lines()
            .map(|line| {
                if let Some(rest) = line.strip_prefix("@[noinline] public def ") {
                    format!("@[expose, noinline] public def {rest}\n")
                } else if let Some(rest) = line.strip_prefix("public def ") {
                    format!("@[expose] public def {rest}\n")
                } else {
                    format!("{line}\n")
                }
            })
            .collect()
    })
}

fn portable_runtime() -> &'static str {
    r#"
namespace LexLeanRuntime

public class ToMathInt (α : Type) where
  toInt : α -> Int

public class Fixed (α : Type) extends ToMathInt α where
  fromInt : Int -> α
  minimum : Int
  maximum : Int
  bitAnd : α -> α -> α
  bitOr : α -> α -> α
  bitXor : α -> α -> α
  bitNot : α -> α
  shiftLeft : α -> UInt32 -> Option α
  shiftRight : α -> UInt32 -> Option α

public instance : ToMathInt Int where toInt := fun value => value

public instance : Fixed Int8 where
  toInt := Int8.toInt
  fromInt := Int8.ofInt
  minimum := -128
  maximum := 127
  bitAnd := Int8.land
  bitOr := Int8.lor
  bitXor := Int8.xor
  bitNot := Int8.complement
  shiftLeft := fun value amount => if amount.toNat < 8 then some (Int8.shiftLeft value (Int8.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 8 then some (Int8.shiftRight value (Int8.ofNat amount.toNat)) else none

public instance : Fixed Int16 where
  toInt := Int16.toInt
  fromInt := Int16.ofInt
  minimum := -32768
  maximum := 32767
  bitAnd := Int16.land
  bitOr := Int16.lor
  bitXor := Int16.xor
  bitNot := Int16.complement
  shiftLeft := fun value amount => if amount.toNat < 16 then some (Int16.shiftLeft value (Int16.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 16 then some (Int16.shiftRight value (Int16.ofNat amount.toNat)) else none

public instance : Fixed Int32 where
  toInt := Int32.toInt
  fromInt := Int32.ofInt
  minimum := -2147483648
  maximum := 2147483647
  bitAnd := Int32.land
  bitOr := Int32.lor
  bitXor := Int32.xor
  bitNot := Int32.complement
  shiftLeft := fun value amount => if amount.toNat < 32 then some (Int32.shiftLeft value (Int32.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 32 then some (Int32.shiftRight value (Int32.ofNat amount.toNat)) else none

public instance : Fixed Int64 where
  toInt := Int64.toInt
  fromInt := Int64.ofInt
  minimum := -9223372036854775808
  maximum := 9223372036854775807
  bitAnd := Int64.land
  bitOr := Int64.lor
  bitXor := Int64.xor
  bitNot := Int64.complement
  shiftLeft := fun value amount => if amount.toNat < 64 then some (Int64.shiftLeft value (Int64.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 64 then some (Int64.shiftRight value (Int64.ofNat amount.toNat)) else none

public instance : Fixed UInt8 where
  toInt := fun value => Int.ofNat value.toNat
  fromInt := UInt8.ofInt
  minimum := 0
  maximum := 255
  bitAnd := UInt8.land
  bitOr := UInt8.lor
  bitXor := UInt8.xor
  bitNot := UInt8.complement
  shiftLeft := fun value amount => if amount.toNat < 8 then some (UInt8.shiftLeft value (UInt8.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 8 then some (UInt8.shiftRight value (UInt8.ofNat amount.toNat)) else none

public instance : Fixed UInt16 where
  toInt := fun value => Int.ofNat value.toNat
  fromInt := UInt16.ofInt
  minimum := 0
  maximum := 65535
  bitAnd := UInt16.land
  bitOr := UInt16.lor
  bitXor := UInt16.xor
  bitNot := UInt16.complement
  shiftLeft := fun value amount => if amount.toNat < 16 then some (UInt16.shiftLeft value (UInt16.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 16 then some (UInt16.shiftRight value (UInt16.ofNat amount.toNat)) else none

public instance : Fixed UInt32 where
  toInt := fun value => Int.ofNat value.toNat
  fromInt := UInt32.ofInt
  minimum := 0
  maximum := 4294967295
  bitAnd := UInt32.land
  bitOr := UInt32.lor
  bitXor := UInt32.xor
  bitNot := UInt32.complement
  shiftLeft := fun value amount => if amount.toNat < 32 then some (UInt32.shiftLeft value amount) else none
  shiftRight := fun value amount => if amount.toNat < 32 then some (UInt32.shiftRight value amount) else none

public instance : Fixed UInt64 where
  toInt := fun value => Int.ofNat value.toNat
  fromInt := UInt64.ofInt
  minimum := 0
  maximum := 18446744073709551615
  bitAnd := UInt64.land
  bitOr := UInt64.lor
  bitXor := UInt64.xor
  bitNot := UInt64.complement
  shiftLeft := fun value amount => if amount.toNat < 64 then some (UInt64.shiftLeft value (UInt64.ofNat amount.toNat)) else none
  shiftRight := fun value amount => if amount.toNat < 64 then some (UInt64.shiftRight value (UInt64.ofNat amount.toNat)) else none

@[expose] public def checkedFromInt {α : Type} [Fixed α] (value : Int) : Option α :=
  if value < Fixed.minimum (α := α) then none else if Fixed.maximum (α := α) < value then none else some (Fixed.fromInt value)

@[expose] public def checkedConvert {α β : Type} [ToMathInt α] [Fixed β] (value : α) : Option β :=
  checkedFromInt (ToMathInt.toInt value)

@[expose] public def checkedAdd {α : Type} [Fixed α] (left right : α) : Option α :=
  checkedFromInt (ToMathInt.toInt left + ToMathInt.toInt right)

@[expose] public def checkedSubtract {α : Type} [Fixed α] (left right : α) : Option α :=
  checkedFromInt (ToMathInt.toInt left - ToMathInt.toInt right)

@[expose] public def checkedMultiply {α : Type} [Fixed α] (left right : α) : Option α :=
  checkedFromInt (ToMathInt.toInt left * ToMathInt.toInt right)

@[expose] public def checkedNegate {α : Type} [Fixed α] (value : α) : Option α :=
  checkedFromInt (-ToMathInt.toInt value)

@[expose] public def checkedQuotient {α : Type} [Fixed α] (left right : α) : Option α :=
  if ToMathInt.toInt right = 0 then none else checkedFromInt (Int.tdiv (ToMathInt.toInt left) (ToMathInt.toInt right))

@[expose] public def checkedAddInt64 (left right : Int64) : Option Int64 :=
  let value := left + right
  if (0 < right && value < left) || (right < 0 && left < value) then none else some value

@[expose] public def checkedSubtractInt64 (left right : Int64) : Option Int64 :=
  let value := left - right
  if (0 < right && left < value) || (right < 0 && value < left) then none else some value

@[expose] public def checkedNegateInt64 (value : Int64) : Option Int64 :=
  if value == (-9223372036854775808 : Int64) then none else some (-value)

public def magnitudeInt64 (value : Int64) : UInt64 :=
  let bits := value.toUInt64
  if value < 0 then 0 - bits else bits

public def signedMagnitudeInt64 (negative : Bool) (value : UInt64) : Int64 :=
  (if negative then 0 - value else value).toInt64

public def divideMagnitudeInt64 : Nat -> UInt64 -> UInt64 -> UInt64 -> UInt64 -> UInt64
  | 0, _, _, _, quotient => quotient
  | Nat.succ fuel, source, divisor, remainder, quotient =>
      let high := 9223372036854775808 <= source
      let source := source + source
      let remainder := remainder + remainder + if high then 1 else 0
      let quotient := quotient + quotient
      if divisor <= remainder then
        divideMagnitudeInt64 fuel source divisor (remainder - divisor) (quotient + 1)
      else
        divideMagnitudeInt64 fuel source divisor remainder quotient

@[expose] public def checkedQuotientInt64 (left right : Int64) : Option Int64 :=
  if right == 0 then none
  else if left == (-9223372036854775808 : Int64) && right == (-1 : Int64) then none
  else
    let negative := (left < 0) != (right < 0)
    some (signedMagnitudeInt64 negative
      (divideMagnitudeInt64 64 (magnitudeInt64 left) (magnitudeInt64 right) 0 0))

public def multiplyMagnitudeInt64 : Nat -> UInt64 -> UInt64 -> UInt64 -> Bool -> Option UInt64
  | 0, _, _, accumulator, _ => some accumulator
  | Nat.succ fuel, source, multiplicand, accumulator, negative =>
      let high := 9223372036854775808 <= source
      let limit := if negative then 9223372036854775808 else 9223372036854775807
      let halfLimit := if negative then 4611686018427387904 else 4611686018427387903
      if halfLimit < accumulator then none
      else
        let doubled := accumulator + accumulator
        if high then
          if limit < multiplicand || limit - multiplicand < doubled then none
          else multiplyMagnitudeInt64 fuel (source + source) multiplicand
            (doubled + multiplicand) negative
        else
          multiplyMagnitudeInt64 fuel (source + source) multiplicand doubled negative

@[expose] public def checkedMultiplyInt64 (left right : Int64) : Option Int64 :=
  let negative := (left < 0) != (right < 0)
  match multiplyMagnitudeInt64 64 (magnitudeInt64 right) (magnitudeInt64 left) 0 negative with
  | none => none
  | some value => some (signedMagnitudeInt64 negative value)

@[noinline] public def subtract {α : Type} [Sub α] (left right : α) : α := left - right
@[noinline] public def multiply {α : Type} [Mul α] (left right : α) : α := left * right
@[noinline] public def negate {α : Type} [Neg α] (value : α) : α := -value

public class Quotient (α : Type) where
  quotient : α -> α -> α
  remainder : α -> α -> α
  isZero : α -> Bool

public instance : Quotient Nat where
  quotient := Nat.div
  remainder := Nat.mod
  isZero := fun value => value == 0

public instance : Quotient Int where
  quotient := Int.tdiv
  remainder := Int.tmod
  isZero := fun value => value == 0

@[noinline] public def quotient {α : Type} [Quotient α] (left right zeroCase : α) : α :=
  if Quotient.isZero right then zeroCase else Quotient.quotient left right

@[noinline] public def remainder {α : Type} [Quotient α] (left right zeroCase : α) : α :=
  if Quotient.isZero right then zeroCase else Quotient.remainder left right

@[expose] public def bitAnd {α : Type} [Fixed α] (left right : α) : α := Fixed.bitAnd left right
@[expose] public def bitOr {α : Type} [Fixed α] (left right : α) : α := Fixed.bitOr left right
@[expose] public def bitXor {α : Type} [Fixed α] (left right : α) : α := Fixed.bitXor left right
@[expose] public def bitNot {α : Type} [Fixed α] (value : α) : α := Fixed.bitNot value
@[expose] public def shiftLeft {α : Type} [Fixed α] (value : α) (amount : UInt32) : Option α := Fixed.shiftLeft value amount
@[expose] public def shiftRight {α : Type} [Fixed α] (value : α) (amount : UInt32) : Option α := Fixed.shiftRight value amount

public class Appendable (α : Type) where append : α -> α -> α
public instance {α : Type} : Appendable (List α) where append := List.append
public instance : Appendable ByteArray where append := ByteArray.append
@[expose] public def append {α : Type} [Appendable α] (left right : α) : α := Appendable.append left right

public class Lengthable (α : Type) where length : α -> Nat
public instance {α : Type} : Lengthable (List α) where length := List.length
public instance : Lengthable ByteArray where length := ByteArray.size
public instance : Lengthable String where length := String.length
@[expose] public def length {α : Type} [Lengthable α] (value : α) : Nat := Lengthable.length value

@[expose] public def listIndex {α : Type} : List α -> Nat -> Option α
  | [], _ => none
  | head :: _, 0 => some head
  | _ :: tail, index + 1 => listIndex tail index

public class Indexable (α β : Type) where index : α -> Nat -> Option β
public instance {α : Type} : Indexable (List α) α where index := listIndex
public instance : Indexable ByteArray UInt8 where index := fun value offset => value.data[offset]?
@[noinline] public def index {α β : Type} [Indexable α β] (value : α) (offset : Nat) : Option β := Indexable.index value offset

public class Sliceable (α : Type) where slice : α -> Nat -> Nat -> Option α
public instance {α : Type} : Sliceable (List α) where
  slice := fun value start count => if start + count <= value.length then some ((value.drop start).take count) else none
public instance : Sliceable ByteArray where
  slice := fun value start count => if start + count <= value.size then some (value.extract start (start + count)) else none
@[noinline] public def slice {α : Type} [Sliceable α] (value : α) (start count : Nat) : Option α := Sliceable.slice value start count

@[noinline] public def utf8Encode (value : String) : ByteArray := value.toUTF8
@[noinline] public def utf8Decode (value : ByteArray) : Option String := String.fromUTF8? value
@[noinline] public def compareBytes (left right : ByteArray) : Ordering := compare left.toList right.toList
@[expose] public def equal {α : Type} [BEq α] (left right : α) : Bool := left == right

@[noinline] public def splitExact (value delimiter : String) (maximum : UInt32) : Option (List String) :=
  let fields := value.splitOn delimiter
  if delimiter.isEmpty || maximum.toNat < fields.length then none else some fields

@[noinline] public def join (values : List String) (delimiter : String) : String := delimiter.intercalate values

public class Decimal (α : Type) where
  parse : String -> Option α
  format : α -> String

public instance : Decimal Int where
  parse := fun value => match value.toInt? with | some parsed => if toString parsed = value then some parsed else none | none => none
  format := toString

public instance {α : Type} [Fixed α] [ToString α] : Decimal α where
  parse := fun value => match value.toInt? with | some parsed => if toString parsed = value then checkedFromInt parsed else none | none => none
  format := toString

@[noinline] public def parseDecimal {α : Type} [Decimal α] (value : String) : Option α := Decimal.parse value
@[noinline] public def formatDecimal {α : Type} [Decimal α] (value : α) : String := Decimal.format value

end LexLeanRuntime
"#
}

fn contains_lean_comment_outside_string(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut index = 0_usize;
    let mut in_string = false;
    let mut escaped = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
        } else if byte == b'"' {
            in_string = true;
        } else if index + 1 < bytes.len()
            && matches!((byte, bytes[index + 1]), (b'-', b'-') | (b'/', b'-'))
        {
            return true;
        }
        index += 1;
    }
    false
}

/// Render one semantic module as prose-free Lean.
pub fn render_lean(
    checked: &CheckedModule,
    module: &SemanticModule,
    module_prefix: &str,
) -> Result<Emitter, Diagnostic> {
    // The module means its elaboration (§17.12): a model declaration is the
    // ordinary declarations it elaborates to, and a checked application is
    // the ordinary term it means. Lean is generated from exactly those.
    let lowered: Vec<&SemanticDeclaration> = module.lowered_declarations();
    let elaborated = serde_json::to_string(&lowered).expect("semantic declarations serialize")
        + &serde_json::to_string(&module.elaboration.all_checks()).expect("cross-checks serialize")
        + &serde_json::to_string(&module.elaboration.all_theorems())
            .expect("generated theorems serialize");
    if elaborated.contains("\"kind\":\"checked_apply\"") {
        return Err(Diagnostic::new(
            code!("LLI9001"),
            "phase lean-backend: a checked model application was not elaborated",
        ));
    }
    // §17.12 (reasoning): the reasoning runtime states its lemmas over the
    // portable and collection runtimes, so a module with a reasoning
    // declaration emits all three.
    let reasoning = module.declarations.iter().any(|declaration| {
        crate::ir::semantic::reasoning::declaration_construct(declaration).is_some()
    });
    let runtime = elaborated.contains("\"kind\":\"primitive\"") || reasoning;
    let document = &checked.document;
    let base_render = Render {
        prefix: module_prefix,
        hypotheses: std::collections::BTreeMap::new(),
        runtime,
        document: false,
        qualify: None,
    };
    // A declaration a model elaborates to is named `R.x`, and Lean resolves
    // a name inside it against the namespace `R` first, so every local
    // reference it makes is written qualified (§17.12, models).
    let model_render = Render {
        prefix: module_prefix,
        hypotheses: std::collections::BTreeMap::new(),
        runtime,
        document: false,
        qualify: Some(&document.lean_module),
    };
    let mut text = String::from("module\npublic import Init\n");
    for import in &document.imports {
        text.push_str(&format!(
            "public import {}\n",
            identifier(&format!("{module_prefix}.{import}"))
        ));
    }
    // Large closed byte constants incur Lean compiler work beyond its small
    // interactive defaults. Match the fixed finite closed-core budgets; the
    // project still bounds child elapsed time and captured output, and every
    // declaration is elaborated, replayed, and axiom-audited normally.
    text.push_str(
        "set_option autoImplicit false\nset_option maxRecDepth 100000\nset_option maxHeartbeats 1000000000\nnamespace ",
    );
    text.push_str(&identifier(&document.lean_module));
    text.push('\n');
    if runtime {
        if module.spec == "lexlean/semantic-module/2" {
            text.push_str(portable_runtime_1_2());
        } else {
            text.push_str(portable_runtime());
        }
    }
    if reasoning
        || uses_collections(
            &serde_json::to_value((&lowered, module.elaboration.all_checks()))
                .expect("elaborated declarations serialize"),
        )
    {
        text.push_str(COLLECTIONS_RUNTIME);
    }
    if module
        .declarations
        .iter()
        .any(|declaration| crate::ir::semantic::model::declaration_construct(declaration).is_some())
    {
        text.push_str(MODELS_RUNTIME);
    }
    if reasoning {
        text.push_str(REASONING_RUNTIME);
    }
    let group_of = |index: usize| match module.declarations.get(index) {
        Some(
            SemanticDeclaration::Inductive {
                mutual: Some(label),
                ..
            }
            | SemanticDeclaration::Definition {
                mutual: Some(label),
                ..
            },
        ) => Some(label.as_str()),
        _ => None,
    };
    let mut starts = Vec::with_capacity(module.declarations.len());
    for (index, source) in module.declarations.iter().enumerate() {
        starts.push(text.len());
        text.push('\n');
        // §17.12: one contiguous mutual group is one Lean `mutual` block.
        let group = group_of(index);
        if group.is_some() && (index == 0 || group_of(index - 1) != group) {
            text.push_str("mutual\n");
        }
        let declarations: Vec<&SemanticDeclaration> = match module.elaborated(index) {
            Some(elaborated) => elaborated.iter().collect(),
            None => vec![source],
        };
        let render = if crate::ir::semantic::model::elaborated_construct(source).is_some() {
            &model_render
        } else {
            &base_render
        };
        for declaration in declarations {
            match declaration {
                SemanticDeclaration::Structure {
                    name,
                    type_parameters,
                    parameters,
                    fields,
                } => {
                    let name = identifier(name);
                    text.push_str(&format!(
                        "public structure {name}{}{} where\n",
                        render.type_parameters(type_parameters),
                        render.parameters(parameters)
                    ));
                    for field in fields {
                        text.push_str(&format!(
                            "  {} : {}\n",
                            identifier(&field.name),
                            render.ty(&field.r#type)
                        ));
                    }
                }
                SemanticDeclaration::Class {
                    name,
                    type_parameters,
                    parameters,
                    fields,
                } => {
                    let name = identifier(name);
                    text.push_str(&format!(
                        "public class {name}{}{} where\n",
                        render.type_parameters(type_parameters),
                        render.parameters(parameters)
                    ));
                    for field in fields {
                        text.push_str(&format!(
                            "  {} : {}\n",
                            identifier(&field.name),
                            render.ty(&field.r#type)
                        ));
                    }
                }
                SemanticDeclaration::Instance {
                    name,
                    class,
                    arguments,
                    priority,
                    fields,
                } => {
                    let name = identifier(name);
                    let arguments = arguments
                        .iter()
                        .map(|argument| format!(" ({})", render.ty(argument)))
                        .collect::<String>();
                    text.push_str(&format!(
                        "public instance (priority := {priority}) {name} : {}{arguments} where\n",
                        render.member(class)
                    ));
                    for field in fields {
                        text.push_str(&format!(
                            "  {} := {}\n",
                            identifier(&field.field),
                            render.term(&field.value)
                        ));
                    }
                }
                SemanticDeclaration::Inductive {
                    name,
                    type_parameters,
                    parameters,
                    constructors,
                    ..
                } => {
                    let name = identifier(name);
                    text.push_str(&format!(
                        "public inductive {name}{}{} where\n",
                        render.type_parameters(type_parameters),
                        render.parameters(parameters)
                    ));
                    for constructor in constructors {
                        let fields = constructor
                            .fields
                            .iter()
                            .map(|field| format!(" (_ : {})", render.ty(field)))
                            .collect::<String>();
                        text.push_str(&format!("  | {}{fields}\n", identifier(&constructor.name)));
                    }
                    if group_of(index).is_some() && group_of(index + 1) != group_of(index) {
                        text.push_str("end\n");
                    }
                }
                SemanticDeclaration::Definition {
                    name,
                    type_parameters,
                    parameters,
                    result,
                    body,
                    recursive_argument,
                    mutual,
                    termination,
                    ..
                } => {
                    let type_binders = render.scoped_type_parameters(type_parameters, declaration);
                    if termination.is_some() {
                        // The recursion is the definition alone, or every member
                        // of its mutual group.
                        let group: BTreeSet<String> = match mutual {
                            Some(label) => module
                                .declarations
                                .iter()
                                .filter(|other| {
                                    matches!(other, SemanticDeclaration::Definition {
                                    mutual: Some(other_label),
                                    ..
                                } if other_label == label)
                                })
                                .map(|other| other.name().to_owned())
                                .collect(),
                            None => BTreeSet::from([name.clone()]),
                        };
                        // Every type parameter is passed to the evidence in
                        // `decreasing_by`, so each is used and keeps its name.
                        text.push_str(&well_founded_definition(render, &group, declaration)?);
                        if mutual.is_some() && group_of(index + 1) != group_of(index) {
                            text.push_str("end\n");
                        }
                    } else if let Some(recursive_argument) = recursive_argument {
                        let equations = render
                        .recursive_equations(
                            name,
                            &type_binders,
                            parameters,
                            result,
                            recursive_argument,
                            body,
                        )
                        .ok_or_else(|| {
                            Diagnostic::new(
                                code!("LLI9001"),
                                "phase lean-backend: recursive definition is not a top-level structural match",
                            )
                        })?;
                        text.push_str(&equations);
                        if mutual.is_some() {
                            // §17.12: a mutual group is structurally recursive by
                            // construction; Lean must confirm exactly that.
                            // Only the decreasing parameter is used after `=>`;
                            // naming the others would trip Lean's unused-variable
                            // linter, which fails verification.
                            let binders = parameters
                                .iter()
                                .map(|parameter| {
                                    if &parameter.name == recursive_argument {
                                        identifier(&parameter.name)
                                    } else {
                                        "_".to_owned()
                                    }
                                })
                                .collect::<Vec<_>>()
                                .join(" ");
                            text.push_str(&format!(
                                "termination_by structural {binders} => {}\n",
                                identifier(recursive_argument)
                            ));
                            if group_of(index + 1) != group_of(index) {
                                text.push_str("end\n");
                            }
                        }
                    } else {
                        let name = identifier(name);
                        // §17.12: a language-1.2 proposition-valued definition
                        // is reducible, so a proposition it names is decidable
                        // exactly when its body is, as a contract predicate
                        // over a finite domain must be.
                        let attributes = if module.spec == "lexlean/semantic-module/2"
                            && *result == SemanticType::Prop
                        {
                            "@[expose, reducible]"
                        } else {
                            "@[expose]"
                        };
                        text.push_str(&format!(
                            "{attributes} public def {name}{type_binders}{} : {} := {}\n",
                            render.scoped_parameters(parameters, |local| term_uses(body, local)),
                            render.ty(result),
                            render.term(body)
                        ));
                    }
                }
                SemanticDeclaration::Theorem {
                    name,
                    type_parameters,
                    parameters,
                    statement,
                    proof,
                    ..
                } => {
                    let name = identifier(name);
                    // A parameter neither the statement nor the proof mentions
                    // (as a termination-evidence theorem's may be) is bound as
                    // `_name`, so Lean's unused-variable linter stays quiet;
                    // every other binder keeps its exact name.
                    let binders = render.scoped_parameters(parameters, |local| {
                        term_uses(statement, local) || proof_uses(proof, local)
                    });
                    text.push_str(&format!(
                        "public theorem {name}{}{binders} : {} := by\n{}",
                        render.scoped_type_parameters(type_parameters, declaration),
                        render.term(statement),
                        render.proof(proof, 1)
                    ));
                }
                SemanticDeclaration::Artifact { .. }
                | SemanticDeclaration::Contract { .. }
                | SemanticDeclaration::Realization { .. }
                | SemanticDeclaration::Evidence { .. }
                | SemanticDeclaration::Model { .. }
                | SemanticDeclaration::Logic { .. }
                | SemanticDeclaration::InferenceRule { .. }
                | SemanticDeclaration::Verifier { .. }
                | SemanticDeclaration::Reasoner { .. } => {
                    return Err(Diagnostic::new(
                        code!("LLI9001"),
                        format!(
                            "phase lean-backend: model or reasoning declaration `{}` was not elaborated",
                            declaration.name()
                        ),
                    ));
                }
            }
        }
        for theorem in module.elaboration.theorems(index) {
            text.push_str(&render.generated_theorem(theorem));
        }
        for check in module.elaboration.checks(index) {
            text.push_str(&render.cross_check(check, &document.lean_module));
        }
    }
    let end = text.len();
    text.push_str("\nend ");
    text.push_str(&identifier(&document.lean_module));
    text.push('\n');
    if contains_lean_comment_outside_string(&text) {
        return Err(Diagnostic::new(
            code!("LLI9001"),
            "phase lean-backend: semantic lowering produced a comment token",
        ));
    }
    if module.spec == "lexlean/semantic-module/2" {
        emit_declarations(checked, &text, "semantic-lean-module", &starts, end)
    } else {
        Ok(emit(checked, &text, "semantic-lean-module"))
    }
}

fn tex_escape(text: &str) -> String {
    let mut out = String::new();
    for character in text.chars() {
        match character {
            '\\' => out.push_str("\\textbackslash{}"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '#' => out.push_str("\\#"),
            '$' => out.push_str("\\$"),
            '%' => out.push_str("\\%"),
            '&' => out.push_str("\\&"),
            '_' => out.push_str("\\_"),
            '^' => out.push_str("\\^{}"),
            '~' => out.push_str("\\~{}"),
            other if other.is_ascii() => out.push(other),
            other => out.push_str(&format!("<U+{:04X}>", u32::from(other))),
        }
    }
    out
}

/// Language 1.2: a declaration's value parameters with their types, so a
/// function-typed parameter is visible in the document.
fn latex_parameters(render: &Render<'_>, parameters: &[SemanticParameter], text: &mut String) {
    if !parameters.is_empty() {
        text.push_str(&format!(
            "\\noindent Parameters: \\texttt{{{}}}.\\par\n",
            tex_escape(render.parameters(parameters).trim_start())
        ));
    }
}

/// Language 1.2: every closure a definition forms, in pre-order, with the
/// locals it captures (explicit in the semantics, implicit in Lean's `fun`).
fn latex_closures(body: &SemanticTerm, text: &mut String) {
    let mut index = 0;
    crate::ir::semantic::visit_terms(body, &mut |term| {
        if let SemanticTerm::Lambda {
            parameters,
            captures,
            ..
        } = term
        {
            index += 1;
            let names: Vec<&str> = parameters
                .iter()
                .map(|parameter| parameter.name.as_str())
                .collect();
            text.push_str(&format!(
                "\\noindent Closure {index}: binds \\texttt{{({})}}, captures \\texttt{{({})}}.\\par\n",
                tex_escape(&names.join(", ")),
                tex_escape(&captures.join(", "))
            ));
        }
    });
}

fn latex_line(text: &mut String, label: &str, value: &str) {
    text.push_str(&format!(
        "\\noindent {label}: \\texttt{{{}}}.\\par\n",
        tex_escape(value)
    ));
}

fn latex_use(render: &Render<'_>, model_use: &crate::ir::semantic::ModelUse) -> String {
    let mut out = render.member(&model_use.member);
    for argument in &model_use.type_arguments {
        out.push_str(&format!(" ({})", render.ty(argument)));
    }
    out
}

fn latex_policy(axioms: &[String]) -> String {
    if axioms.is_empty() {
        "none".to_owned()
    } else {
        format!("exact [{}]", axioms.join(", "))
    }
}

/// Language 1.2 (models): what a model declaration states, then exactly
/// what linking elaborated it to, its generated obligations with the
/// theorems that discharge them, and the Lean restatements. The document
/// never calls a claim verified: verification is the attestation's (§22.9).
#[allow(clippy::too_many_lines)]
fn latex_model(
    render: &Render<'_>,
    module: &SemanticModule,
    index: usize,
    declaration: &SemanticDeclaration,
    text: &mut String,
) {
    use crate::ir::semantic::{
        ArtifactSchema, CompositeForm, CompositeJunction, EvidenceClaim, NeuralDecoder,
        NeuralLayer, RealizationDescriptor,
    };
    let names = |members: &[String]| members.join(", ");
    let junction = |junction: &CompositeJunction| match junction {
        CompositeJunction::Unconditional => "unconditional".to_owned(),
        CompositeJunction::Proved { evidence } => format!("proved by {}", render.member(evidence)),
        CompositeJunction::Checked => "checked at run time".to_owned(),
    };
    match declaration {
        SemanticDeclaration::Artifact {
            role,
            sha256,
            length,
            schema,
            r#type,
            axioms,
            ..
        } => {
            latex_line(text, "Role", &format!("{role:?}").to_lowercase());
            latex_line(
                text,
                "Schema",
                &match schema {
                    ArtifactSchema::Bytes => "bytes".to_owned(),
                    ArtifactSchema::IntTensor { element, shape } => format!(
                        "{} integer tensor, little-endian, row-major, shape ({})",
                        serde_json::to_value(element)
                            .ok()
                            .and_then(|value| value.as_str().map(str::to_owned))
                            .unwrap_or_default(),
                        shape
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    ArtifactSchema::Utf8Lines => "UTF-8 lines, each LF-terminated".to_owned(),
                },
            );
            latex_line(text, "Value type", &render.ty(r#type));
            latex_line(
                text,
                "Content",
                &format!("{length} bytes, SHA-256 {sha256}"),
            );
            latex_line(text, "Axiom policy", &latex_policy(axioms));
        }
        SemanticDeclaration::Contract {
            type_parameters,
            input,
            output,
            state,
            precondition,
            postcondition,
            invariant,
            validators,
            axioms,
            ..
        } => {
            if !type_parameters.is_empty() {
                latex_line(
                    text,
                    "Type parameters",
                    &format!("({})", names(type_parameters)),
                );
            }
            let mut interface = format!(
                "input {} : {}, output {} : {}",
                input.name,
                render.ty(&input.r#type),
                output.name,
                render.ty(&output.r#type)
            );
            if let Some(state) = state {
                interface.push_str(&format!(
                    ", state {} : {} stepping to {}",
                    state.name,
                    render.ty(&state.r#type),
                    state.next
                ));
            }
            latex_line(text, "Interface", &interface);
            for (label, predicate) in [
                ("Precondition", precondition),
                ("Postcondition", postcondition),
                ("Invariant", invariant),
            ] {
                if let Some(predicate) = predicate {
                    latex_line(text, label, &render.member(predicate));
                }
            }
            for validator in validators {
                let mut line = format!(
                    "{} validates the {}, sound by {}",
                    render.member(&validator.validator),
                    serde_json::to_value(validator.predicate)
                        .ok()
                        .and_then(|value| value.as_str().map(str::to_owned))
                        .unwrap_or_default(),
                    render.member(&validator.sound)
                );
                if let Some(complete) = &validator.complete {
                    line.push_str(&format!(", complete by {}", render.member(complete)));
                }
                latex_line(text, "Runtime validator", &line);
            }
            latex_line(text, "Axiom policy", &latex_policy(axioms));
        }
        SemanticDeclaration::Realization {
            type_parameters,
            input,
            output,
            state,
            descriptor,
            executable,
            axioms,
            ..
        } => {
            if !type_parameters.is_empty() {
                latex_line(
                    text,
                    "Type parameters",
                    &format!("({})", names(type_parameters)),
                );
            }
            let mut interface = format!(
                "input {} : {}, output {}",
                input.name,
                render.ty(&input.r#type),
                render.ty(output)
            );
            if let Some(state) = state {
                interface.push_str(&format!(
                    ", state {} : {} from {}",
                    state.name,
                    render.ty(&state.r#type),
                    render.term(&state.initial)
                ));
            }
            latex_line(text, "Interface", &interface);
            match descriptor {
                RealizationDescriptor::Deterministic { body } => {
                    latex_line(text, "Deterministic denotation", &render.term(body));
                }
                RealizationDescriptor::Rule { rules, default } => {
                    for (position, rule) in rules.iter().enumerate() {
                        latex_line(
                            text,
                            &format!("Rule {} ({})", position + 1, tex_escape(&rule.name)),
                            &format!(
                                "if {} then {}",
                                render.term(&rule.guard),
                                render.term(&rule.action)
                            ),
                        );
                    }
                    latex_line(text, "Otherwise", &render.term(default));
                }
                RealizationDescriptor::Statistical {
                    features,
                    width,
                    width_evidence,
                    weights,
                    bias,
                    labels,
                    ..
                } => {
                    latex_line(
                        text,
                        "Statistical scheme",
                        "linear scoring over mathematical integers, first maximal label",
                    );
                    latex_line(
                        text,
                        "Features",
                        &format!(
                            "{} of width {width}, by {}",
                            render.term(features),
                            render.member(width_evidence)
                        ),
                    );
                    latex_line(
                        text,
                        "Parameters",
                        &format!(
                            "weights {}, bias {}",
                            render.member(weights),
                            render.member(bias)
                        ),
                    );
                    latex_line(
                        text,
                        "Labels",
                        &labels
                            .iter()
                            .map(|label| render.term(label))
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                }
                RealizationDescriptor::Neural {
                    encoder,
                    width,
                    width_evidence,
                    layers,
                    decoder,
                    ..
                } => {
                    latex_line(
                        text,
                        "Neural architecture",
                        "exact integer feed-forward over mathematical integers",
                    );
                    latex_line(
                        text,
                        "Encoder",
                        &format!(
                            "{} of width {width}, by {}",
                            render.term(encoder),
                            render.member(width_evidence)
                        ),
                    );
                    for (position, layer) in layers.iter().enumerate() {
                        latex_line(
                            text,
                            &format!("Layer {}", position + 1),
                            &match layer {
                                NeuralLayer::Dense {
                                    inputs,
                                    outputs,
                                    weights,
                                    bias,
                                } => format!(
                                    "dense {inputs} to {outputs}, weights {}, bias {}",
                                    render.member(weights),
                                    render.member(bias)
                                ),
                                NeuralLayer::Relu => "rectified linear".to_owned(),
                                NeuralLayer::Requantize {
                                    shift,
                                    minimum,
                                    maximum,
                                } => format!(
                                    "truncating division by 2^{shift}, clamped to [{minimum}, {maximum}]"
                                ),
                            },
                        );
                    }
                    latex_line(
                        text,
                        "Decoder",
                        &match decoder {
                            NeuralDecoder::Argmax { labels } => format!(
                                "first maximal of {}",
                                labels
                                    .iter()
                                    .map(|label| render.term(label))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ),
                            NeuralDecoder::Function { binder, body } => {
                                format!("{binder} => {}", render.term(body))
                            }
                        },
                    );
                }
                RealizationDescriptor::Composite { form } => {
                    let line = match form {
                        CompositeForm::Sequence { stages, junctions } => format!(
                            "sequence {}; junctions {}",
                            stages
                                .iter()
                                .map(|stage| latex_use(render, stage))
                                .collect::<Vec<_>>()
                                .join(", "),
                            junctions
                                .iter()
                                .map(junction)
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                        CompositeForm::Fanout { left, right } => {
                            format!(
                                "fan-out of {} and {}",
                                latex_use(render, left),
                                latex_use(render, right)
                            )
                        }
                        CompositeForm::Product { left, right } => {
                            format!(
                                "product of {} and {}",
                                latex_use(render, left),
                                latex_use(render, right)
                            )
                        }
                        CompositeForm::Branch {
                            guard,
                            then,
                            r#else,
                        } => format!(
                            "if {} then {} else {}",
                            render.term(guard),
                            latex_use(render, then),
                            latex_use(render, r#else)
                        ),
                        CompositeForm::Scan {
                            stage,
                            junction: scan,
                        } => {
                            format!(
                                "scan of {}; junction {}",
                                latex_use(render, stage),
                                junction(scan)
                            )
                        }
                    };
                    latex_line(text, "Composite", &line);
                }
            }
            if *executable {
                text.push_str(
                    "\\noindent Execution: executable, non-escaping closures only.\\par\n",
                );
            }
            latex_line(text, "Axiom policy", &latex_policy(axioms));
        }
        SemanticDeclaration::Evidence {
            type_parameters,
            contract,
            realization,
            claims,
            axioms,
            ..
        } => {
            if !type_parameters.is_empty() {
                latex_line(
                    text,
                    "Type parameters",
                    &format!("({})", names(type_parameters)),
                );
            }
            latex_line(
                text,
                "About",
                &format!(
                    "realization {} against contract {}",
                    latex_use(render, realization),
                    latex_use(render, contract)
                ),
            );
            for claim in claims {
                let line = match claim {
                    EvidenceClaim::SatisfiesContract { theorem } => format!(
                        "satisfies the postcondition under the contract's premises, discharged by {}",
                        render.member(theorem)
                    ),
                    EvidenceClaim::PreservesInvariant { theorem } => format!(
                        "every step preserves the invariant, discharged by {}",
                        render.member(theorem)
                    ),
                    EvidenceClaim::InitialInvariant { theorem } => format!(
                        "the initial state satisfies the invariant, discharged by {}",
                        render.member(theorem)
                    ),
                    EvidenceClaim::EquivalentTo { reference, theorem } => format!(
                        "equals {} under the contract's premises, discharged by {}",
                        latex_use(render, reference),
                        render.member(theorem)
                    ),
                    EvidenceClaim::DatasetAgreement {
                        dataset,
                        comparison,
                        comparison_sound,
                        examples,
                        agreements,
                        theorem,
                    } => format!(
                        "agrees with the expected output on exactly {agreements} of the {examples} examples of {} under {}, sound for equality by {}, discharged by {}",
                        render.member(dataset),
                        render.member(comparison),
                        render.member(comparison_sound),
                        render.member(theorem)
                    ),
                };
                latex_line(text, "Claim", &line);
            }
            latex_line(text, "Axiom policy", &latex_policy(axioms));
        }
        SemanticDeclaration::Model {
            type_parameters,
            contract,
            realization,
            evidence,
            entry,
            axioms,
            ..
        } => {
            if !type_parameters.is_empty() {
                latex_line(
                    text,
                    "Type parameters",
                    &format!("({})", names(type_parameters)),
                );
            }
            latex_line(
                text,
                "Binds",
                &format!(
                    "contract {} to realization {}",
                    latex_use(render, contract),
                    latex_use(render, realization)
                ),
            );
            latex_line(
                text,
                "Evidence",
                &if evidence.is_empty() {
                    "none".to_owned()
                } else {
                    evidence
                        .iter()
                        .map(|item| latex_use(render, item))
                        .collect::<Vec<_>>()
                        .join(", ")
                },
            );
            if !entry.is_empty() {
                latex_line(
                    text,
                    "Entry",
                    &entry
                        .iter()
                        .map(|theorem| render.member(theorem))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
            }
            let required = module.elaboration.required(index);
            latex_line(
                text,
                "Runtime checks in executable code",
                &if required.is_empty() {
                    "none".to_owned()
                } else {
                    required
                        .iter()
                        .map(|check| crate::ir::semantic::model::check_name_of(*check))
                        .collect::<Vec<_>>()
                        .join(", ")
                },
            );
            latex_line(text, "Axiom policy", &latex_policy(axioms));
        }
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. }
        | SemanticDeclaration::Theorem { .. } => {}
    }
    for obligation in module.elaboration.obligations(index) {
        // A closed obligation is stated bare; the role's code quotes are
        // diagnostic spelling, not document text.
        let binders = if obligation.parameters.is_empty() {
            String::new()
        } else {
            format!("forall{}, ", render.parameters(&obligation.parameters))
        };
        latex_line(
            text,
            &format!(
                "Obligation ({})",
                tex_escape(&obligation.role.replace('`', ""))
            ),
            &format!(
                "{binders}{}; stated exactly by {}",
                render.term(&obligation.statement),
                render.member(&obligation.theorem)
            ),
        );
    }
    for derived in module.elaboration.lowered(index) {
        if let SemanticDeclaration::Definition {
            name,
            parameters,
            result,
            body,
            ..
        } = derived
        {
            // §17.12 rule 13: an artifact's bytes and decoded value are never
            // document text; the document states what they are (role, schema,
            // length, digest) and the generated Lean carries them.
            let shown = if matches!(declaration, SemanticDeclaration::Artifact { .. }) {
                format!(
                    "{name} : {}, {}",
                    render.ty(result),
                    if name.ends_with(".bytes") {
                        "the configured bytes"
                    } else {
                        "their decoding under the schema"
                    }
                )
            } else {
                format!(
                    "{name}{} : {} := {}",
                    render.parameters(parameters),
                    render.ty(result),
                    render.term(body)
                )
            };
            latex_line(text, "Elaborates to", &shown);
        }
    }
    for check in module.elaboration.checks(index) {
        latex_line(
            text,
            "Restated in Lean against the fixed model semantics as",
            &check.name,
        );
    }
}

/// Render the same semantic module as canonical explanatory LaTeX.
pub fn render_latex(
    checked: &CheckedModule,
    module: &SemanticModule,
    module_prefix: &str,
) -> Result<Emitter, Diagnostic> {
    let render = Render {
        prefix: module_prefix,
        hypotheses: std::collections::BTreeMap::new(),
        runtime: false,
        document: true,
        qualify: None,
    };
    let version_2 = module.spec == "lexlean/semantic-module/2";
    let mut text = String::from(
        "\\documentclass[11pt]{article}\n\\usepackage[T1]{fontenc}\n\\usepackage{amsmath,amssymb}\n\\begin{document}\n\\section*{Semantic declarations}\n",
    );
    let mut starts = Vec::with_capacity(module.declarations.len());
    for (index, declaration) in module.declarations.iter().enumerate() {
        starts.push(text.len());
        text.push_str(&format!(
            "\\subsection*{{\\texttt{{{}}}}}\n\\noindent Kind: \\texttt{{{}}}.\\par\n",
            tex_escape(declaration.name()),
            declaration.kind()
        ));
        // Language 1.2: a parameterized data declaration states the type
        // parameters its constructor and field types mention.
        if version_2 {
            if let SemanticDeclaration::Inductive {
                type_parameters, ..
            }
            | SemanticDeclaration::Structure {
                type_parameters, ..
            }
            | SemanticDeclaration::Class {
                type_parameters, ..
            } = declaration
            {
                if !type_parameters.is_empty() {
                    text.push_str(&format!(
                        "\\noindent Type parameters: \\texttt{{({})}}.\\par\n",
                        tex_escape(&type_parameters.join(", "))
                    ));
                }
            }
        }
        match declaration {
            SemanticDeclaration::Definition {
                type_parameters,
                parameters,
                result,
                body,
                axioms,
                executable,
                ..
            } => {
                let policy = if axioms.is_empty() {
                    "none".to_owned()
                } else {
                    format!("exact [{}]", axioms.join(", "))
                };
                if version_2 && !type_parameters.is_empty() {
                    text.push_str(&format!(
                        "\\noindent Type parameters: \\texttt{{({})}}.\\par\n",
                        tex_escape(&type_parameters.join(", "))
                    ));
                }
                if let (
                    true,
                    SemanticDeclaration::Definition {
                        mutual: Some(label),
                        recursive_argument: Some(argument),
                        ..
                    },
                ) = (version_2, declaration)
                {
                    text.push_str(&format!(
                        "\\noindent Mutual recursion group \\texttt{{{}}}, decreasing on \\texttt{{{}}}.\\par\n",
                        tex_escape(label),
                        tex_escape(argument)
                    ));
                }
                if let (
                    true,
                    SemanticDeclaration::Definition {
                        termination: Some(termination),
                        ..
                    },
                ) = (version_2, declaration)
                {
                    text.push_str(&format!(
                        "\\noindent Well-founded measure: \\texttt{{{}}}; decrease evidence per call: \\texttt{{{}}}.\\par\n",
                        tex_escape(&render.term(&termination.measure)),
                        tex_escape(
                            &termination
                                .evidence
                                .iter()
                                .map(|evidence| render.member(evidence))
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    ));
                }
                if version_2 {
                    latex_parameters(&render, parameters, &mut text);
                    latex_closures(body, &mut text);
                }
                if version_2 && *executable {
                    text.push_str(
                        "\\noindent Execution: executable, non-escaping closures only.\\par\n",
                    );
                }
                if let (
                    true,
                    SemanticDeclaration::Definition {
                        production: Some(production),
                        ..
                    },
                ) = (version_2, declaration)
                {
                    let effects = if production.effects.is_empty() {
                        "none".to_owned()
                    } else {
                        production.effects.join(", ")
                    };
                    text.push_str(&format!(
                        "\\noindent Production root for targets \\texttt{{{}}}; admitted effects: \\texttt{{{}}}.\\par\n",
                        tex_escape(&production.targets.join(", ")),
                        tex_escape(&effects)
                    ));
                }
                text.push_str(&format!(
                    "\\noindent Type: \\texttt{{{}}}.\\par\n\\noindent Definition: \\texttt{{{}}}.\\par\n\\noindent Axiom policy: \\texttt{{{}}}.\\par\n",
                    tex_escape(&render.ty(result)),
                    tex_escape(&render.term(body)),
                    tex_escape(&policy)
                ));
            }
            SemanticDeclaration::Theorem {
                statement,
                axioms,
                type_parameters,
                parameters,
                ..
            } => {
                let policy = if axioms.is_empty() {
                    "none".to_owned()
                } else {
                    format!("exact [{}]", axioms.join(", "))
                };
                if version_2 && !type_parameters.is_empty() {
                    text.push_str(&format!(
                        "\\noindent Type parameters: \\texttt{{({})}}.\\par\n",
                        tex_escape(&type_parameters.join(", "))
                    ));
                }
                if version_2 {
                    latex_parameters(&render, parameters, &mut text);
                }
                text.push_str(&format!(
                    "\\noindent Statement: \\texttt{{{}}}.\\par\n\\noindent Axiom policy: \\texttt{{{}}}.\\par\n",
                    tex_escape(&render.term(statement)),
                    tex_escape(&policy)
                ));
            }
            // Language 1.2 renders the data shape that 1.1 leaves implicit;
            // 1.1 documents keep their historical bytes.
            SemanticDeclaration::Inductive {
                constructors,
                mutual,
                ..
            } if version_2 => {
                if let Some(label) = mutual {
                    text.push_str(&format!(
                        "\\noindent Mutual group: \\texttt{{{}}}.\\par\n",
                        tex_escape(label)
                    ));
                }
                for constructor in constructors {
                    let fields = constructor
                        .fields
                        .iter()
                        .map(|field| render.ty(field))
                        .collect::<Vec<_>>()
                        .join(", ");
                    text.push_str(&format!(
                        "\\noindent Constructor \\texttt{{{}}}: \\texttt{{({})}}.\\par\n",
                        tex_escape(&constructor.name),
                        tex_escape(&fields)
                    ));
                }
            }
            SemanticDeclaration::Structure { fields, .. }
            | SemanticDeclaration::Class { fields, .. }
                if version_2 =>
            {
                for field in fields {
                    text.push_str(&format!(
                        "\\noindent Field \\texttt{{{}}}: \\texttt{{{}}}.\\par\n",
                        tex_escape(&field.name),
                        tex_escape(&render.ty(&field.r#type))
                    ));
                }
            }
            SemanticDeclaration::Structure { .. }
            | SemanticDeclaration::Class { .. }
            | SemanticDeclaration::Instance { .. }
            | SemanticDeclaration::Inductive { .. } => {}
            SemanticDeclaration::Artifact { .. }
            | SemanticDeclaration::Contract { .. }
            | SemanticDeclaration::Realization { .. }
            | SemanticDeclaration::Evidence { .. }
            | SemanticDeclaration::Model { .. } => {
                latex_model(&render, module, index, declaration, &mut text);
            }
            SemanticDeclaration::Logic { .. }
            | SemanticDeclaration::InferenceRule { .. }
            | SemanticDeclaration::Verifier { .. }
            | SemanticDeclaration::Reasoner { .. } => {
                reasoning::latex_reasoning(&render, module, index, declaration, &mut text);
            }
        }
    }
    let end = text.len();
    text.push_str("\\end{document}\n");
    if version_2 {
        emit_declarations(checked, &text, "semantic-latex-module", &starts, end)
    } else {
        Ok(emit(checked, &text, "semantic-latex-module"))
    }
}

#[cfg(test)]
mod declaration_span_tests {
    use super::declaration_spans;

    #[test]
    fn spans_are_the_declaration_objects_whatever_the_layout() {
        let compact = r#"\semanticdata{{"declarations":[{"name":"a\"}"},{"name":"b","x":[1,{"y":"]"}]}],"spec":"s"}}"#;
        let spans = declaration_spans(compact).expect("spans");
        let objects: Vec<&str> = spans
            .iter()
            .map(|(start, end)| &compact[*start..*end])
            .collect();
        assert_eq!(
            objects,
            [r#"{"name":"a\"}"}"#, r#"{"name":"b","x":[1,{"y":"]"}]}"#]
        );
        let spaced = "\\semanticdata{{\n  \"version\": 2,\n  \"declarations\": [\n    {\"name\":\"a\"} ,\n    {\"name\":\"b\"}\n  ]\n}}";
        let spans = declaration_spans(spaced).expect("spans");
        let objects: Vec<&str> = spans
            .iter()
            .map(|(start, end)| &spaced[*start..*end])
            .collect();
        assert_eq!(objects, [r#"{"name":"a"}"#, r#"{"name":"b"}"#]);
        assert_eq!(declaration_spans("\\semanticdata{{\"spec\":1}}"), None);
        assert_eq!(declaration_spans("no data"), None);
    }
}

#[cfg(test)]
mod comment_tests {
    #[test]
    fn string_literals_use_the_pinned_lean_escape_grammar() {
        for code in (0..=0x1f).chain(0x7f..=0x9f) {
            let character = char::from_u32(code).expect("control scalar");
            let expected = match character {
                '\n' => "\"\\n\"".to_owned(),
                '\r' => "\"\\r\"".to_owned(),
                '\t' => "\"\\t\"".to_owned(),
                _ => format!("\"\\u{code:04x}\""),
            };
            assert_eq!(super::string_literal(&character.to_string()), expected);
        }
        for value in ["", "ordinary ASCII", "literal \\0 \\u{80}", "\"quote\""] {
            assert_eq!(super::string_literal(value), format!("{value:?}"));
        }
        for value in ["\u{301}", "\u{200b}", "\u{10000}", "\u{10ffff}"] {
            assert_eq!(super::string_literal(value), format!("\"{value}\""));
        }
    }

    #[test]
    fn language_1_2_runtime_exposes_every_definition_and_keeps_1_1_frozen() {
        let frozen = super::portable_runtime();
        let exposed = super::portable_runtime_1_2();
        for line in exposed.lines() {
            assert!(!line.starts_with("public def "), "{line}");
            assert!(!line.starts_with("@[noinline] public def "), "{line}");
        }
        assert!(exposed.contains("@[expose, noinline] public def index "));
        assert!(exposed.contains("@[expose] public def magnitudeInt64 "));
        assert!(frozen.contains("@[noinline] public def index "));
        assert_eq!(
            exposed
                .replace("@[expose, noinline] public def", "@[noinline] public def")
                .replace("@[expose] public def magnitude", "public def magnitude")
                .replace(
                    "@[expose] public def signedMagnitude",
                    "public def signedMagnitude"
                )
                .replace(
                    "@[expose] public def divideMagnitude",
                    "public def divideMagnitude"
                )
                .replace(
                    "@[expose] public def multiplyMagnitude",
                    "public def multiplyMagnitude"
                ),
            if frozen.ends_with('\n') {
                frozen.to_owned()
            } else {
                format!("{frozen}\n")
            }
        );
    }

    #[test]
    fn imported_list_construction_remains_kernel_reducible() {
        let runtime = super::portable_runtime();
        for declaration in ["append", "length"] {
            assert!(runtime.contains(&format!("@[expose] public def {declaration}")));
            assert!(!runtime.contains(&format!("@[noinline] public def {declaration}")));
        }
    }

    #[test]
    fn comment_tokens_in_generated_string_literals_are_data() {
        assert!(!super::contains_lean_comment_outside_string(
            r#"def value := "--config=locked /- literal""#
        ));
        assert!(!super::contains_lean_comment_outside_string(
            r#"def value := "escaped \" /- literal \\""#
        ));
        assert!(super::contains_lean_comment_outside_string(
            r#"def value := "safe \\" -- generated comment"#
        ));
        assert!(super::contains_lean_comment_outside_string(
            "def value := true -- generated comment\n"
        ));
        assert!(super::contains_lean_comment_outside_string(
            "def value := /- generated comment -/ true\n"
        ));
    }
}
