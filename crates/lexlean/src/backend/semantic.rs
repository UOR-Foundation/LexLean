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
    /// The Lean module each imported module's key names; a module not in it
    /// lies under `prefix` (§10.5).
    lean_imports: &'a std::collections::BTreeMap<String, String>,
    /// §17.12: in a well-founded definition, each `if` (by node address)
    /// lowers to a dependent match binding its numbered hypothesis.
    hypotheses: std::collections::BTreeMap<usize, usize>,
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
pub(crate) fn hypothesis(index: usize) -> String {
    format!("__decrease{index}")
}

// Semantic names are validated data, not Lean tokens. Quoting a reserved
// segment preserves its exact Name identity instead of narrowing the source
// language to whatever the pinned parser happens to leave unreserved.
pub(crate) fn identifier(name: &str) -> String {
    name.split('.')
        .map(|segment| {
            if super::lean_tokens::is_reserved(segment)
                || crate::verify::source_audit::forbids_segment(segment)
            {
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
pub(crate) fn string_literal(value: &str) -> String {
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
        SemanticProof::Apply { arguments, .. } | SemanticProof::Law { arguments, .. } => {
            arguments.iter().any(|argument| term_uses(argument, local))
        }
    }
}

pub(crate) fn term_uses(term: &SemanticTerm, local: &str) -> bool {
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
pub(crate) fn violation_pattern(constructor: &MemberRef) -> Option<&'static str> {
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
            // §17.12 (text): a member of the laws library is written by the
            // library's own name; the module is the elaboration's import.
            (Some(module), _) if module == crate::text_laws::PREFIX => {
                identifier(&format!("{module}.{}", member.name))
            }
            (Some(module), _) => identifier(&format!(
                "{}.{}",
                self.lean_imports
                    .get(module)
                    .cloned()
                    .unwrap_or_else(|| format!("{}.{module}", self.prefix)),
                member.name
            )),
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
            lean_imports: self.lean_imports,
            hypotheses: std::collections::BTreeMap::new(),
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
        // Linking elaborates every widening conversion before Lean is rendered
        // (§17.12); the canonical document states the conversion itself, by
        // a name no Lean runtime declares.
        if operation.widening() {
            let name = match operation {
                SemanticPrimitive::ToNat => "toNat",
                _ => "toInt",
            };
            let mut out = format!("({name}");
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
                    .collect::<Vec<_>>();
                if self.document {
                    let numbers = values
                        .iter()
                        .map(u8::to_string)
                        .collect::<Vec<_>>()
                        .join(", ");
                    return format!("ByteArray.mk #[{numbers}]");
                }
                let mut counts = [0_usize; 256];
                for value in &values {
                    counts[usize::from(*value)] += 1;
                }
                // Keep sharing inside this literal: generated locals outside it
                // must not participate in the meaning of its octets.
                let mut bindings = String::new();
                for (value, count) in counts.iter().enumerate() {
                    if *count >= 4 {
                        bindings.push_str(&format!(
                            "let __byte{value} : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit {value}); "
                        ));
                    }
                }
                let octets = values
                    .iter()
                    .map(|value| {
                        if counts[usize::from(*value)] >= 4 {
                            format!("__byte{value}")
                        } else {
                            format!("(_root_.UInt8.ofNat (nat_lit {value}))")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let array = format!("_root_.ByteArray.mk #[{octets}]");
                if bindings.is_empty() {
                    array
                } else {
                    format!("({bindings}{array})")
                }
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
                let unfolded = definitions
                    .iter()
                    .map(|member| self.member(member))
                    .collect::<Vec<_>>();
                linear_arithmetic(&pad, &unfolded)
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
            SemanticProof::Law {
                name,
                type_arguments,
                arguments,
                premises,
            } => {
                // Linking resolved the law; a law it did not resolve never
                // reaches the backend.
                let lean = crate::ir::semantic::laws::law(name)
                    .map_or_else(|_| identifier(name), |law| law.lean.clone());
                let arguments = type_arguments
                    .iter()
                    .map(|argument| format!(" ({})", self.ty(argument)))
                    .chain(
                        arguments
                            .iter()
                            .map(|argument| format!(" ({})", self.term(argument))),
                    )
                    .chain(
                        premises
                            .iter()
                            .map(|premise| format!(" {}", self.member(premise))),
                    )
                    .collect::<String>();
                format!("{pad}exact @{lean}{arguments}\n")
            }
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

/// The fixed Boolean comparisons `linear_arithmetic` reflects into
/// propositions, and the runtime definitions it unfolds (§17.12).
const LINEAR_BRIDGES: &[&str] = &[
    "← Bool.not_eq_true",
    "Bool.and_eq_true",
    "Bool.or_eq_true",
    "Bool.not_eq_true'",
    "and_true",
    "true_and",
    "Option.some.injEq",
    "Nat.beq_eq",
    "Nat.blt_eq",
    "Nat.ble_eq",
    "LexLeanRuntime.subtract",
    "LexLeanRuntime.multiply",
    "LexLeanRuntime.negate",
    "LexLeanRuntime.equal",
    "beq_iff_eq",
    "decide_eq_true_eq",
    "decide_eq_false_iff_not",
    "Nat.add_assoc",
    "Int.add_assoc",
    "Int.neg_neg",
    "Int.emod_neg",
    "Int.neg_zero",
    "↓reduceIte",
];

/// The fixed-width integer types in the order `linear_arithmetic` names
/// their order lemmas, each with the projection to its mathematical value.
const FIXED_WIDTH: &[(&str, &str)] = &[
    ("UInt8", "toNat"),
    ("UInt16", "toNat"),
    ("UInt32", "toNat"),
    ("UInt64", "toNat"),
    ("Int8", "toInt"),
    ("Int16", "toInt"),
    ("Int32", "toInt"),
    ("Int64", "toInt"),
];

/// The fixed `linear_arithmetic` script (§17.12): introduce the goal, and
/// with named definitions substitute and unfold them; rewrite the runtime's
/// comparisons, arithmetic, and folds into linear arithmetic through the
/// bridges and the registered fold laws, with fixed-width order and
/// equality read on the mathematical values (literals evaluated first);
/// read every remaining fixed-width value as the bounded natural number of
/// its bits, so `omega` knows its range; split every `if` and `match` left
/// in the goal; and decide each case with `omega`. A goal the simplifier
/// closed leaves nothing for `omega`.
fn linear_arithmetic(pad: &str, definitions: &[String]) -> String {
    let rewrites = crate::ir::semantic::laws::registry()
        .map(|registry| registry.rewrites.as_slice())
        .unwrap_or_default();
    let mut lemmas: Vec<String> = definitions.to_vec();
    lemmas.extend(LINEAR_BRIDGES.iter().map(|lemma| (*lemma).to_owned()));
    lemmas.extend(rewrites.iter().cloned());
    for (ty, value) in FIXED_WIDTH {
        lemmas.push(format!("{ty}.le_iff_{value}_le"));
        lemmas.push(format!("{ty}.lt_iff_{value}_lt"));
    }
    for (ty, value) in FIXED_WIDTH {
        let reduce = if *value == "toNat" { "ToNat" } else { "ToInt" };
        lemmas.push(format!("{ty}.reduce{reduce}"));
        lemmas.push(format!("← {ty}.{value}_inj"));
    }
    let mut bits: Vec<String> = FIXED_WIDTH
        .iter()
        .map(|(ty, value)| format!("{ty}.{value}"))
        .collect();
    bits.push("BitVec.toInt_eq_toNat_cond".to_owned());
    let subst = if definitions.is_empty() {
        String::new()
    } else {
        format!("{pad}subst_vars\n")
    };
    format!(
        "{pad}intros\n{subst}{pad}try set_option linter.unusedSimpArgs false in simp only [{}] at *\n{pad}try set_option linter.unusedSimpArgs false in simp only [{}] at *\n{pad}all_goals (repeat' split)\n{pad}all_goals omega\n",
        lemmas.join(", "),
        bits.join(", ")
    )
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
        lean_imports: base.lean_imports,
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
/// module's data under the control `control` (`\semanticdata{`, or
/// `\toolingdata{` for a tooling module, SPEC.md §17.20) in the normalized
/// source, in order.
pub(crate) fn declaration_spans_in(source: &str, control: &str) -> Option<Vec<(usize, usize)>> {
    let bytes = source.as_bytes();
    let data = source.find(control)? + control.len();
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

/// The byte range in the normalized source of every declaration of a module
/// of the environment `environment` (`semanticmodule` or `toolingmodule`)
/// written in the authoring notation, in order; `None` for a module written
/// as data.
pub(crate) fn notation_declaration_spans(
    normalized: &str,
    environment: &str,
) -> Option<Vec<(usize, usize)>> {
    let begin = format!("\\begin{{{environment}}}");
    let end = format!("\\end{{{environment}}}");
    let body_start = normalized.find(&begin)? + begin.len();
    let body = normalized.get(body_start..normalized.rfind(&end)?)?;
    let tooling = environment == "toolingmodule";
    let data_control = if tooling {
        "\\toolingdata"
    } else {
        "\\semanticdata"
    };
    if body.trim_start().starts_with(data_control) {
        return None;
    }
    let spans = if tooling {
        crate::surface::tooling::declaration_spans(body)
    } else {
        crate::surface::semantic::declaration_spans(body)
    }
    .ok()?;
    Some(
        spans
            .into_iter()
            .map(|(start, end)| (body_start + start, body_start + end))
            .collect(),
    )
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
    emit_declarations_for(
        checked,
        text,
        kind,
        starts,
        end,
        &DeclarationSource {
            control: "\\semanticdata{",
            owner: "lexlean.core::semanticdata",
            environment: "semanticmodule",
            notation_owner: "lexlean.core::semanticsource",
            node: "semantic-declaration",
            skip_blank: false,
        },
    )
}

/// Which data control a module's declarations are written under, and what
/// their mapping rows are called.
pub(crate) struct DeclarationSource<'a> {
    /// The control that opens the data, with its brace.
    pub control: &'a str,
    /// The coverage owner of the metadata rows.
    pub owner: &'a str,
    /// The environment the module is written in, which the authoring
    /// notation of the same module fills instead of the control's argument.
    pub environment: &'a str,
    /// The coverage owner of the metadata rows of a module written in the
    /// notation.
    pub notation_owner: &'a str,
    /// The kind of each declaration's mapping node.
    pub node: &'a str,
    /// Whether a declaration whose rendering is blank has no piece. A
    /// semantic module's blank rendering is the separator its neighbours
    /// carry, and it is output like any other, so it keeps its piece.
    pub skip_blank: bool,
}

/// [`emit_declarations`] for any module kind whose declarations are the
/// elements of a `declarations` array under `source.control`. A declaration
/// whose rendering is empty (a registry row that has no Lean of its own)
/// has no piece.
pub(crate) fn emit_declarations_for(
    checked: &CheckedModule,
    text: &str,
    kind: &str,
    starts: &[usize],
    end: usize,
    source: &DeclarationSource<'_>,
) -> Result<Emitter, Diagnostic> {
    // A module written in the authoring notation (§17.21) maps each
    // declaration to its own text there, as one written as data maps it to
    // its object.
    let notation = notation_declaration_spans(&checked.normalized, source.environment);
    let owner = if notation.is_some() {
        source.notation_owner
    } else {
        source.owner
    };
    let spans = notation
        .or_else(|| declaration_spans_in(&checked.normalized, source.control))
        .filter(|spans| spans.len() == starts.len())
        .ok_or_else(|| {
            Diagnostic::new(
                code!("LLI9001"),
                format!("phase {kind}: the source declarations do not match the rendered ones"),
            )
        })?;
    let origin = || Origin::Metadata {
        owner: owner.to_owned(),
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
        if source.skip_blank && text[*start..stop].trim().is_empty() {
            continue;
        }
        let node = emitter.node(source.node);
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
    // declaration uses all three.
    let reasoning = module.declarations.iter().any(|declaration| {
        crate::ir::semantic::reasoning::declaration_construct(declaration).is_some()
    });
    let runtime = elaborated.contains("\"kind\":\"primitive\"") || reasoning;
    let document = &checked.document;
    let base_render = Render {
        prefix: module_prefix,
        lean_imports: &document.import_lean,
        hypotheses: std::collections::BTreeMap::new(),
        document: false,
        qualify: None,
    };
    // A declaration a model elaborates to is named `R.x`, and Lean resolves
    // a name inside it against the namespace `R` first, so every local
    // reference it makes is written qualified (§17.12, models).
    let model_render = Render {
        prefix: module_prefix,
        lean_imports: &document.import_lean,
        hypotheses: std::collections::BTreeMap::new(),
        document: false,
        qualify: Some(&document.lean_module),
    };
    // §17.12: the runtime a module uses. A language-1.2 module imports the
    // runtime modules, native modules of the embedded runtime project; a
    // language-1.1 module splices the frozen portable runtime inside its
    // namespace, as its released output did.
    let language_1_2 = module.spec == "lexlean/semantic-module/2";
    let collections = reasoning
        || uses_collections(
            &serde_json::to_value((&lowered, module.elaboration.all_checks()))
                .expect("elaborated declarations serialize"),
        );
    let models = module.declarations.iter().any(|declaration| {
        crate::ir::semantic::model::declaration_construct(declaration).is_some()
    });
    // §17.12 (fold laws): `linear_arithmetic` rewrites with the laws
    // module and `law` applies it, so a module with either proof uses it.
    let laws = elaborated.contains("\"kind\":\"linear_arithmetic\"")
        || elaborated.contains("\"kind\":\"law\"");
    let mut text = String::from("module\npublic import Init\n");
    if language_1_2 {
        use crate::backend::runtime::{COLLECTIONS, LAWS, MODELS, PORTABLE, REASONING};
        for (used, runtime_module) in [
            (runtime, PORTABLE),
            (collections, COLLECTIONS),
            (models, MODELS),
            (reasoning, REASONING),
            (laws, LAWS),
        ] {
            if used {
                text.push_str(&format!("public import {runtime_module}\n"));
            }
        }
    }
    for import in &document.imports {
        text.push_str(&format!(
            "public import {}\n",
            identifier(&document.lean_import(import))
        ));
    }
    // §17.12 (text): the elaboration's own imports — the sealed laws
    // library a text declaration means. They are written here, once, and
    // the laws themselves are never spliced into the module: the import is
    // what a text declaration elaborates to.
    for import in module.elaboration.imports() {
        text.push_str(&format!("public import {}\n", identifier(import)));
    }
    // Large closed byte constants incur Lean compiler work beyond its small
    // interactive defaults. Match the fixed finite closed-core budgets; the
    // project still bounds child elapsed time and captured output, and every
    // declaration is elaborated, replayed, and axiom-audited normally.
    text.push_str(
        "set_option autoImplicit false\nset_option maxRecDepth 100000\nset_option maxHeartbeats 1000000000\n",
    );
    // A parameter named like a nullary constructor of its own type (`zero :
    // Nat`, `red : Color`) is a valid binder, and Lean's linter reports it as
    // resembling the constructor. The constructors of a module are never
    // written bare, so nothing is hidden by silencing it.
    if language_1_2 {
        text.push_str("set_option linter.constructorNameAsVariable false\n");
    }
    text.push_str("namespace ");
    text.push_str(&identifier(&document.lean_module));
    text.push('\n');
    if runtime && !language_1_2 {
        text.push_str(crate::backend::runtime::frozen_portable_runtime());
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
                | SemanticDeclaration::Reasoner { .. }
                | SemanticDeclaration::Tokenizer { .. }
                | SemanticDeclaration::ContextWindow { .. }
                | SemanticDeclaration::TextGenerator { .. }
                | SemanticDeclaration::TextClassifier { .. } => {
                    return Err(Diagnostic::new(
                        code!("LLI9001"),
                        format!(
                            "phase lean-backend: model, reasoning, or text declaration `{}` was not elaborated",
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

pub(crate) fn tex_escape(text: &str) -> String {
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
        | SemanticDeclaration::Tokenizer { .. }
        | SemanticDeclaration::ContextWindow { .. }
        | SemanticDeclaration::TextGenerator { .. }
        | SemanticDeclaration::TextClassifier { .. }
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

/// The document lines of a text declaration (§17.12, *Text components*):
/// its closed members, the laws library its elaboration imports, exactly what
/// linking elaborated it to, and the theorems it generates. The pieces of the
/// vocabulary are never document text (rule 13 under *Models*): the artifact
/// is named, and the generated Lean carries its value.
fn latex_text(
    render: &Render<'_>,
    module: &SemanticModule,
    index: usize,
    declaration: &SemanticDeclaration,
    text: &mut String,
) {
    if let SemanticDeclaration::Tokenizer {
        scheme,
        vocabulary,
        axioms,
        ..
    } = declaration
    {
        latex_line(
            text,
            "Scheme",
            match scheme {
                crate::ir::semantic::TokenizerScheme::ByteLevel => "byte_level",
                crate::ir::semantic::TokenizerScheme::LongestMatch => "longest_match",
            },
        );
        latex_line(text, "Vocabulary", &latex_use(render, vocabulary));
        latex_line(
            text,
            "Laws",
            &format!(
                "imported from the native module {}",
                crate::text_laws::laws_module()
            ),
        );
        latex_line(text, "Axiom policy", &latex_policy(axioms));
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
            latex_line(
                text,
                "Elaborates to",
                &format!(
                    "{name}{} : {} := {}",
                    render.parameters(parameters),
                    render.ty(result),
                    render.term(body)
                ),
            );
        }
    }
    for theorem in module.elaboration.theorems(index) {
        let binders = if theorem.parameters.is_empty() {
            String::new()
        } else {
            format!("forall{}, ", render.parameters(&theorem.parameters))
        };
        latex_line(
            text,
            &format!(
                "Generates theorem ({} template)",
                theorem.template().replace('_', " ")
            ),
            &format!(
                "{} : {binders}{}",
                theorem.name,
                render.formula(&theorem.statement)
            ),
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
        lean_imports: &checked.document.import_lean,
        hypotheses: std::collections::BTreeMap::new(),
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
            SemanticDeclaration::Tokenizer { .. } => {
                latex_text(&render, module, index, declaration, &mut text);
            }
            SemanticDeclaration::ContextWindow { .. }
            | SemanticDeclaration::TextGenerator { .. }
            | SemanticDeclaration::TextClassifier { .. } => {
                return Err(Diagnostic::new(
                    code!("LLI9001"),
                    format!(
                        "phase latex-backend: text declaration `{}` was not elaborated",
                        declaration.name()
                    ),
                ));
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
mod byte_literal_tests {
    use super::{Render, SemanticParameter, SemanticTerm, SemanticType};

    fn render(document: bool, term: &SemanticTerm) -> String {
        let imports = std::collections::BTreeMap::new();
        Render {
            prefix: "ByteFixture",
            lean_imports: &imports,
            hypotheses: std::collections::BTreeMap::new(),
            document,
            qualify: None,
        }
        .term(term)
    }

    fn literal(values: &[u8]) -> SemanticTerm {
        SemanticTerm::Bytes {
            hex: values.iter().map(|value| format!("{value:02x}")).collect(),
        }
    }

    #[test]
    fn byte_literals_are_explicit_octets_without_changing_document_numbers() {
        for values in [vec![], vec![0, 255, 128], (0..=255).collect()] {
            let term = literal(&values);
            let explicit = values
                .iter()
                .map(|value| format!("(_root_.UInt8.ofNat (nat_lit {value}))"))
                .collect::<Vec<_>>()
                .join(", ");
            assert_eq!(
                render(false, &term),
                format!("_root_.ByteArray.mk #[{explicit}]")
            );
            let numbers = values
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            assert_eq!(render(true, &term), format!("ByteArray.mk #[{numbers}]"));
        }
    }

    #[test]
    fn repeated_octets_have_one_closed_binding_at_the_four_occurrence_boundary() {
        assert_eq!(
            render(false, &literal(&[0, 255, 0, 255, 0, 255, 0, 255, 128])),
            "(let __byte0 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 0); let __byte255 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 255); _root_.ByteArray.mk #[__byte0, __byte255, __byte0, __byte255, __byte0, __byte255, __byte0, __byte255, (_root_.UInt8.ofNat (nat_lit 128))])"
        );
        for count in 1..=3 {
            let text = render(false, &literal(&vec![255; count]));
            assert!(!text.contains("let "), "{count} occurrences: {text}");
            assert_eq!(text.matches("UInt8.ofNat").count(), count);
        }
        let all: Vec<u8> = (0..4).flat_map(|_| 0..=255).collect();
        let text = render(false, &literal(&all));
        assert_eq!(text.matches("let __byte").count(), 256);
        assert_eq!(text.matches("UInt8.ofNat").count(), 256);
        assert_eq!(text.matches("_root_.ByteArray.mk #[").count(), 1);
        let numbers = all.iter().map(u8::to_string).collect::<Vec<_>>().join(", ");
        assert_eq!(
            render(true, &literal(&all)),
            format!("ByteArray.mk #[{numbers}]")
        );
        assert_eq!(
            text,
            render(false, &literal(&all)),
            "deterministic literal-local scope"
        );
    }

    #[test]
    fn shared_octet_scope_cannot_capture_an_outer_generated_local() {
        let bytes = literal(&[0; 4]);
        let term = SemanticTerm::Let {
            binder: SemanticParameter {
                name: "__byte0".to_owned(),
                r#type: SemanticType::Nat,
            },
            value: Box::new(SemanticTerm::Nat {
                value: "9".to_owned(),
            }),
            body: Box::new(SemanticTerm::Pair {
                left: Box::new(SemanticTerm::Var {
                    name: "__byte0".to_owned(),
                }),
                right: Box::new(bytes),
            }),
        };
        assert_eq!(
            render(false, &term),
            "(let __byte0 : Nat := 9; (__byte0, (let __byte0 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 0); _root_.ByteArray.mk #[__byte0, __byte0, __byte0, __byte0])))"
        );
    }
}

#[cfg(test)]
mod declaration_span_tests {
    use super::declaration_spans_in;

    fn declaration_spans(source: &str) -> Option<Vec<(usize, usize)>> {
        declaration_spans_in(source, "\\semanticdata{")
    }

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
    fn language_1_2_runtime_is_native_and_language_1_1_keeps_its_frozen_text() {
        // Language 1.2: the portable runtime is a native module, whose rows the
        // decoder submits exposed (§17.10), with `noinline` kept as data.
        let portable = crate::backend::runtime::modules()
            .expect("the shipped runtime")
            .iter()
            .find(|module| module.name == crate::backend::runtime::PORTABLE)
            .expect("the portable runtime");
        for declaration in ["index", "magnitudeInt64", "append", "length"] {
            let name = format!("LexLeanRuntime.{declaration}");
            assert!(
                portable.declarations.iter().any(|(row, _)| *row == name),
                "{name}"
            );
        }
        // Language 1.1: the frozen text, unexposed except where it always was.
        let frozen = crate::backend::runtime::frozen_portable_runtime();
        assert!(frozen.contains("@[noinline] public def index "));
        assert!(!frozen.contains("@[expose, noinline]"));
        for declaration in ["append", "length"] {
            assert!(frozen.contains(&format!("@[expose] public def {declaration}")));
            assert!(!frozen.contains(&format!("@[noinline] public def {declaration}")));
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
