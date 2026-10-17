//! Primitives realized directly (SPEC.md §17.17.3): the library lemma of the
//! primitive at its operand types, applied to the operands' denotations.

use super::super::{
    element_of, equal_suffix, integer_suffix, numeric, option_value, sequence, width_suffix,
    Numeric, Sequence,
};
use super::{Pf, Scope, Writer};
use crate::calculus::Expr;
use crate::ir::semantic::{SemanticPrimitive, SemanticTerm, SemanticType};
use crate::production::kernel::{Given, Term};
use crate::production::source::Site;

/// A primitive's relation lemma, its leading hints and arguments, and the
/// library lemma's own name.
struct PrimRule {
    lemma: String,
    hints: Vec<(&'static str, Term)>,
    prefix: Vec<Given>,
}

impl<'a> Writer<'_, 'a> {
    /// The encoder bundle of a list, set, or map type.
    pub(super) fn list_bundle(&mut self, ty: &SemanticType) -> Result<Term, String> {
        match self.gen.nested.get(&self.gen.source.type_text(ty)) {
            Some(nested) => {
                let index = nested.index;
                Ok(self.own(&format!("__L_{index}")))
            }
            None => {
                let element = element_of(ty)?;
                let alpha = self.lean_ty(&element)?;
                let e = self.enc(&element)?;
                let head = self.lp("listEnc");
                Ok(self.b.apps(head, &[alpha, e]))
            }
        }
    }

    /// The encoder bundle of an option type.
    pub(super) fn option_bundle(&mut self, ty: &SemanticType) -> Result<Term, String> {
        match self.gen.nested.get(&self.gen.source.type_text(ty)) {
            Some(nested) => {
                let index = nested.index;
                Ok(self.own(&format!("__O_{index}")))
            }
            None => {
                let value = option_value(ty)?;
                let alpha = self.lean_ty(&value)?;
                let e = self.enc(&value)?;
                let head = self.lp("optEnc");
                Ok(self.b.apps(head, &[alpha, e]))
            }
        }
    }

    /// The relation lemma of a directly realized primitive at its operand
    /// types.
    #[allow(clippy::too_many_lines)]
    fn prim_rule(
        &mut self,
        operation: SemanticPrimitive,
        types: &[SemanticType],
        result: &SemanticType,
        site: &Site,
    ) -> Result<PrimRule, String> {
        let first = types
            .first()
            .ok_or_else(|| format!("{operation:?} has no operand"))?;
        let plain = |lemma: String| PrimRule {
            lemma,
            hints: Vec::new(),
            prefix: Vec::new(),
        };
        let numeric_rule = |nat: &str, int: &str| -> Result<PrimRule, String> {
            Ok(match numeric(first)? {
                Numeric::Nat => plain(nat.to_owned()),
                Numeric::Int => plain(int.to_owned()),
            })
        };
        Ok(match operation {
            SemanticPrimitive::ToInt | SemanticPrimitive::ToNat => {
                return Err(format!(
                    "{operation:?} is certified only through its elaboration"
                ));
            }
            SemanticPrimitive::Subtract => numeric_rule("prim_natSub", "prim_intSub")?,
            SemanticPrimitive::Multiply => numeric_rule("prim_natMul", "prim_intMul")?,
            SemanticPrimitive::Quotient => numeric_rule("prim_natQuot", "prim_intQuot")?,
            SemanticPrimitive::Remainder => numeric_rule("prim_natRem", "prim_intRem")?,
            SemanticPrimitive::Negate => plain("prim_intNeg".to_owned()),
            SemanticPrimitive::CheckedConvert => plain(format!(
                "prim_convert_{}_{}",
                integer_suffix(first)?,
                width_suffix(&option_value(result)?)?
            )),
            SemanticPrimitive::CheckedAdd => {
                plain(format!("prim_checkedAdd_{}", width_suffix(first)?))
            }
            SemanticPrimitive::CheckedSubtract => {
                plain(format!("prim_checkedSub_{}", width_suffix(first)?))
            }
            // The 64-bit signed loops are taken by their clauses (§17.17.3).
            SemanticPrimitive::CheckedMultiply => {
                let suffix = width_suffix(first)?;
                let mut rule = plain(format!("prim_checkedMul_{suffix}"));
                if suffix == "i64" {
                    let step = self.runtime_const(site, "multiplyMagnitudeInt64")?;
                    rule.prefix = vec![Given::Term(step), Given::Refl, Given::Refl];
                }
                rule
            }
            SemanticPrimitive::CheckedNegate => {
                plain(format!("prim_checkedNeg_{}", width_suffix(first)?))
            }
            SemanticPrimitive::CheckedQuotient => {
                let suffix = width_suffix(first)?;
                let mut rule = plain(format!("prim_checkedQuot_{suffix}"));
                if suffix == "i64" {
                    let step = self.runtime_const(site, "divideMagnitudeInt64")?;
                    rule.prefix = vec![Given::Term(step), Given::Refl, Given::Refl];
                }
                rule
            }
            SemanticPrimitive::BitAnd => plain(format!("prim_bitAnd_{}", width_suffix(first)?)),
            SemanticPrimitive::BitOr => plain(format!("prim_bitOr_{}", width_suffix(first)?)),
            SemanticPrimitive::BitXor => plain(format!("prim_bitXor_{}", width_suffix(first)?)),
            SemanticPrimitive::BitNot => plain(format!("prim_bitNot_{}", width_suffix(first)?)),
            SemanticPrimitive::ShiftLeft => {
                plain(format!("prim_shiftLeft_{}", width_suffix(first)?))
            }
            SemanticPrimitive::ShiftRight => {
                plain(format!("prim_shiftRight_{}", width_suffix(first)?))
            }
            SemanticPrimitive::Append => match sequence(first)? {
                Sequence::List(element) => {
                    let bundle = self.list_bundle(first)?;
                    let alpha = self.lean_ty(&element)?;
                    PrimRule {
                        lemma: "prim_append_list".to_owned(),
                        hints: vec![("α", alpha)],
                        prefix: vec![Given::Term(bundle)],
                    }
                }
                Sequence::Bytes => plain("prim_append_bytes".to_owned()),
                Sequence::String => return Err("Append of strings".to_owned()),
            },
            SemanticPrimitive::Length => match sequence(first)? {
                Sequence::List(element) => {
                    let bundle = self.list_bundle(first)?;
                    let alpha = self.lean_ty(&element)?;
                    PrimRule {
                        lemma: "prim_length_list".to_owned(),
                        hints: vec![("α", alpha)],
                        prefix: vec![Given::Term(bundle)],
                    }
                }
                Sequence::Bytes => plain("prim_length_bytes".to_owned()),
                Sequence::String => plain("prim_length_string".to_owned()),
            },
            SemanticPrimitive::MapSize | SemanticPrimitive::SetSize => {
                let bundle = self.list_bundle(first)?;
                let alpha = self.lean_ty(&element_of(first)?)?;
                PrimRule {
                    lemma: "prim_length_list".to_owned(),
                    hints: vec![("α", alpha)],
                    prefix: vec![Given::Term(bundle)],
                }
            }
            SemanticPrimitive::Index => match sequence(first)? {
                Sequence::List(element) => {
                    let bundle = self.list_bundle(first)?;
                    let options = self.option_bundle(&SemanticType::Option {
                        value: Box::new(element.clone()),
                    })?;
                    let alpha = self.lean_ty(&element)?;
                    let index = self.runtime_const(site, "listIndex")?;
                    let index = self.b.app(index, alpha);
                    PrimRule {
                        lemma: "prim_index_list".to_owned(),
                        hints: vec![("α", alpha)],
                        prefix: vec![
                            Given::Term(bundle),
                            Given::Term(options),
                            Given::Refl,
                            Given::Term(index),
                            Given::Refl,
                            Given::Refl,
                            Given::Refl,
                        ],
                    }
                }
                Sequence::Bytes => plain("prim_index_bytes".to_owned()),
                Sequence::String => return Err("Index of a string".to_owned()),
            },
            SemanticPrimitive::Slice => match sequence(first)? {
                Sequence::List(_) => {
                    let bundle = self.list_bundle(first)?;
                    let options = self.option_bundle(&SemanticType::Option {
                        value: Box::new(first.clone()),
                    })?;
                    let alpha = self.lean_ty(&element_of(first)?)?;
                    PrimRule {
                        lemma: "prim_slice_list".to_owned(),
                        hints: vec![("α", alpha)],
                        prefix: vec![Given::Term(bundle), Given::Term(options), Given::Refl],
                    }
                }
                Sequence::Bytes => plain("prim_slice_bytes".to_owned()),
                Sequence::String => return Err("Slice of a string".to_owned()),
            },
            SemanticPrimitive::Utf8Encode => plain("prim_utf8Encode".to_owned()),
            SemanticPrimitive::Utf8Decode => plain("prim_utf8Decode".to_owned()),
            SemanticPrimitive::CompareBytes => plain("prim_compareBytes".to_owned()),
            SemanticPrimitive::Equal => plain(format!("prim_equal_{}", equal_suffix(first)?)),
            SemanticPrimitive::SplitExact => plain("prim_splitExact".to_owned()),
            SemanticPrimitive::Join => plain("prim_join".to_owned()),
            SemanticPrimitive::ParseDecimal => {
                let target = option_value(result)?;
                match numeric(&target) {
                    Ok(Numeric::Int) => plain("prim_parseDecimal_int".to_owned()),
                    Ok(Numeric::Nat) | Err(_) => {
                        plain(format!("prim_parseDecimal_{}", width_suffix(&target)?))
                    }
                }
            }
            SemanticPrimitive::FormatDecimal => {
                plain(format!("prim_formatDecimal_{}", integer_suffix(first)?))
            }
            SemanticPrimitive::MapEntries
            | SemanticPrimitive::SetElements
            | SemanticPrimitive::MapInsert
            | SemanticPrimitive::MapRemove
            | SemanticPrimitive::MapLookup
            | SemanticPrimitive::MapContains
            | SemanticPrimitive::MapKeys
            | SemanticPrimitive::MapValues
            | SemanticPrimitive::MapFold
            | SemanticPrimitive::SetInsert
            | SemanticPrimitive::SetRemove
            | SemanticPrimitive::SetContains
            | SemanticPrimitive::SetUnion
            | SemanticPrimitive::SetIntersection
            | SemanticPrimitive::SetDifference
            | SemanticPrimitive::SetFold
            | SemanticPrimitive::ListFold
            | SemanticPrimitive::Iterate
            | SemanticPrimitive::IterateUntil
            | SemanticPrimitive::GraphSuccessors
            | SemanticPrimitive::GraphReachable
            | SemanticPrimitive::GraphTopological
            | SemanticPrimitive::LessThan => {
                return Err(format!("{operation:?} is not a direct primitive"));
            }
        })
    }

    /// A runtime constant of the module `site` is read in.
    fn runtime_const(&mut self, site: &Site, name: &str) -> Result<Term, String> {
        let _ = site;
        Ok(self.b.konst(&format!("LexLeanRuntime.{name}")))
    }

    /// The proof of a primitive term.
    pub(super) fn primitive(
        &mut self,
        whole: &SemanticTerm,
        operation: SemanticPrimitive,
        arguments: &[SemanticTerm],
        result: &SemanticType,
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        let site = scope.ctx.site.clone();
        let types = arguments
            .iter()
            .map(|argument| self.infer(argument, scope))
            .collect::<Result<Vec<_>, _>>()?;
        let result = self.gen.source.close(result, &site);
        if let Some((template, instance)) =
            super::super::super::lower::template_of(operation, &types)?
        {
            return self.template_call(template, &instance, arguments, &types, expr, scope);
        }
        match operation {
            SemanticPrimitive::ToInt | SemanticPrimitive::ToNat => {
                return Err(format!(
                    "{operation:?} is certified only through its elaboration"
                ));
            }
            SemanticPrimitive::MapEntries | SemanticPrimitive::SetElements => {
                if arguments.len() != 1 {
                    return Err(format!("{operation:?} takes one operand"));
                }
                return self.prove(&arguments[0], expr, scope);
            }
            SemanticPrimitive::LessThan => {
                return self.less_than(&types, arguments, expr, scope);
            }
            SemanticPrimitive::Subtract
            | SemanticPrimitive::Multiply
            | SemanticPrimitive::Quotient
            | SemanticPrimitive::Remainder
            | SemanticPrimitive::Negate
            | SemanticPrimitive::CheckedConvert
            | SemanticPrimitive::CheckedAdd
            | SemanticPrimitive::CheckedSubtract
            | SemanticPrimitive::CheckedMultiply
            | SemanticPrimitive::CheckedNegate
            | SemanticPrimitive::CheckedQuotient
            | SemanticPrimitive::BitAnd
            | SemanticPrimitive::BitOr
            | SemanticPrimitive::BitXor
            | SemanticPrimitive::BitNot
            | SemanticPrimitive::ShiftLeft
            | SemanticPrimitive::ShiftRight
            | SemanticPrimitive::Append
            | SemanticPrimitive::Length
            | SemanticPrimitive::Index
            | SemanticPrimitive::Slice
            | SemanticPrimitive::Utf8Encode
            | SemanticPrimitive::Utf8Decode
            | SemanticPrimitive::CompareBytes
            | SemanticPrimitive::Equal
            | SemanticPrimitive::SplitExact
            | SemanticPrimitive::Join
            | SemanticPrimitive::ParseDecimal
            | SemanticPrimitive::FormatDecimal
            | SemanticPrimitive::MapSize
            | SemanticPrimitive::SetSize
            | SemanticPrimitive::MapInsert
            | SemanticPrimitive::MapRemove
            | SemanticPrimitive::MapLookup
            | SemanticPrimitive::MapContains
            | SemanticPrimitive::MapKeys
            | SemanticPrimitive::MapValues
            | SemanticPrimitive::MapFold
            | SemanticPrimitive::SetInsert
            | SemanticPrimitive::SetRemove
            | SemanticPrimitive::SetContains
            | SemanticPrimitive::SetUnion
            | SemanticPrimitive::SetIntersection
            | SemanticPrimitive::SetDifference
            | SemanticPrimitive::SetFold
            | SemanticPrimitive::ListFold
            | SemanticPrimitive::Iterate
            | SemanticPrimitive::IterateUntil
            | SemanticPrimitive::GraphSuccessors
            | SemanticPrimitive::GraphReachable
            | SemanticPrimitive::GraphTopological => {}
        }
        let _ = whole;
        let operand_exprs = match expr {
            Expr::Prim { operands, .. } if operands.len() == arguments.len() => operands,
            Expr::Value { .. }
            | Expr::Var { .. }
            | Expr::Let { .. }
            | Expr::Cond { .. }
            | Expr::Match { .. }
            | Expr::Build { .. }
            | Expr::Call { .. }
            | Expr::Closure { .. }
            | Expr::Apply { .. }
            | Expr::Prim { .. }
            | Expr::First { .. }
            | Expr::Second { .. }
            | Expr::Field { .. } => {
                return Err("the lowering of a primitive is not a primitive".to_owned())
            }
        };
        let rule = self.prim_rule(operation, &types, &result, &site)?;
        let mut proofs = Vec::new();
        let mut sources = Vec::new();
        for (argument, child) in arguments.iter().zip(operand_exprs) {
            proofs.push(self.prove(argument, child, scope)?);
            sources.push(self.src(argument, scope)?);
        }
        let list = self.operands(scope.env, &proofs)?;
        let mut given = rule.prefix;
        given.extend(sources.into_iter().map(Given::Term));
        let hints: Vec<(&str, Term)> = rule.hints;
        let lemma = self.apply(&rule.lemma, &hints, &given)?;
        let (p, env) = (self.prog, scope.env);
        self.apply(
            "conv_prim",
            &[("p", p), ("env", env)],
            &[
                Given::Proof(list.proof, list.ty),
                Given::Proof(lemma.proof, lemma.ty),
            ],
        )
    }
}
