//! The proof term of a source term's relation (SPEC.md §17.17.4): one lemma
//! of the preservation library per construct, applied at the program
//! expression the lowering produced for it.

use super::super::element_of;
use super::{LocalTerm, Pf, Scope, Writer, EXPLICIT};
use crate::calculus::{Expr, Shape};
use crate::ir::core::CoreLevel;
use crate::ir::semantic::{SemanticEdge, SemanticMapEntry, SemanticTerm, SemanticType, Step};
use crate::production::kernel::{Given, Term};
use crate::production::source::{Constructor, Source};

/// What the proof of an argument contributes to its callee's relation.
pub(super) struct Passed {
    /// The arguments the callee's width predicate takes for it.
    pub(super) fits: Vec<Term>,
    /// The arguments the callee's relation takes for it.
    pub(super) relation: Vec<Term>,
    /// The proof of its operand.
    pub(super) proof: Pf,
}

/// How a conditional selects its value.
pub(super) enum Selection {
    /// `if c then t else e` at a result type.
    If(SemanticType),
    /// `c && right`.
    And,
    /// `c || right`.
    Or,
}

impl<'a> Writer<'_, 'a> {
    // --- Types of derivations ---------------------------------------------

    /// `Rel fits value`.
    pub(super) fn rel(&mut self, fits: Term, value: Term) -> Term {
        let rel = self.lp("Rel");
        self.b.apps(rel, &[fits, value])
    }

    /// The width predicate and value a derivation's type relates. A type
    /// under `let` bindings relates the bindings applied to each.
    pub(super) fn rel_parts(&mut self, ty: Term) -> Result<(Term, Term), String> {
        if let crate::ir::core::CoreNode::Let { n, t, v, body, d } = self.b.node(ty).clone() {
            let (fits, value) = self.rel_parts(body)?;
            let fits = self.b.let_node(&n, t, v, fits, d);
            let value = self.b.let_node(&n, t, v, value, d);
            return Ok((fits, value));
        }
        let (_, args) = self.b.spine(ty);
        let observation = args
            .get(3)
            .copied()
            .ok_or("a derivation type with no relation")?;
        let (_, parts) = self.b.spine(observation);
        match parts.as_slice() {
            [fits, value] => Ok((*fits, *value)),
            _ => Err(format!(
                "a derivation relates {}, not a Rel",
                self.b.show(observation)
            )),
        }
    }

    /// The width predicate of a derivation.
    pub(super) fn fits_of(&mut self, pf: Pf) -> Result<Term, String> {
        Ok(self.rel_parts(pf.ty)?.0)
    }

    /// The value of a derivation.
    pub(super) fn value_of(&mut self, pf: Pf) -> Result<Term, String> {
        Ok(self.rel_parts(pf.ty)?.1)
    }

    // --- Lists of operands ------------------------------------------------

    /// The proof that the operands of a call or construction evaluate: a
    /// `ConvL` chain over the proofs of each.
    pub(super) fn operands(&mut self, env: Term, proofs: &[Pf]) -> Result<Pf, String> {
        let p = self.prog;
        let mut out = self.apply("convL_nil", &[("p", p), ("env", env)], &[])?;
        for head in proofs.iter().rev() {
            out = self.apply(
                "convL_cons",
                &[],
                &[
                    Given::Proof(head.proof, head.ty),
                    Given::Proof(out.proof, out.ty),
                ],
            )?;
        }
        Ok(out)
    }

    /// A value built: `conv_build` over the operands with the construction
    /// lemma `construct`.
    fn built(
        &mut self,
        scope: &Scope,
        expr: &Expr,
        operands: &[Pf],
        construct: Pf,
    ) -> Result<Pf, String> {
        let (shape, ty, exprs) = match expr {
            Expr::Build {
                shape,
                ty,
                operands,
            } => (*shape, ty, operands),
            Expr::Value { .. }
            | Expr::Var { .. }
            | Expr::Let { .. }
            | Expr::Cond { .. }
            | Expr::Match { .. }
            | Expr::Call { .. }
            | Expr::Closure { .. }
            | Expr::Apply { .. }
            | Expr::Prim { .. }
            | Expr::First { .. }
            | Expr::Second { .. }
            | Expr::Field { .. } => {
                return Err("the lowering of a construction is not a build".to_owned())
            }
        };
        if exprs.len() != operands.len() {
            return Err(format!(
                "a construction of {} operands lowered to a build of {}",
                operands.len(),
                exprs.len()
            ));
        }
        let list = self.operands(scope.env, operands)?;
        let shape = self.shape_term(shape);
        let ty = self.ty_term(ty);
        let (p, env) = (self.prog, scope.env);
        self.apply(
            "conv_build",
            &[("p", p), ("env", env), ("shape", shape), ("t", ty)],
            &[
                Given::Proof(list.proof, list.ty),
                Given::Proof(construct.proof, construct.ty),
            ],
        )
    }

    /// A construction lemma that needs no argument.
    fn construct_plain(&mut self, name: &str) -> Result<Pf, String> {
        self.apply(name, &[], &[])
    }

    // --- Values of locals -------------------------------------------------

    /// The encoded value of a local: its encoding applied to it, or the closure
    /// value of a function-typed one.
    pub(super) fn encoded_local(
        &mut self,
        local: &super::super::CLocal,
        terms: LocalTerm,
    ) -> Result<Term, String> {
        match terms.companion {
            Some(companion) => {
                let closure = self.ts("Value.closure");
                Ok(self.b.apps(closure, &[companion.index, companion.captures]))
            }
            None => {
                let enc = self.enc(&local.local.ty)?;
                Ok(self.b.app(enc, terms.value))
            }
        }
    }

    // --- Derivations ------------------------------------------------------

    /// The proof of `term`, lowered to `expr`.
    #[allow(clippy::too_many_lines)]
    pub(super) fn prove(
        &mut self,
        term: &SemanticTerm,
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        let site = scope.ctx.site.clone();
        let (p, env) = (self.prog, scope.env);
        match term {
            SemanticTerm::Var { name } => {
                let number = match expr {
                    Expr::Var { name } => *name,
                    Expr::Value { .. }
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
                        return Err(format!("the lowering of `{name}` is not a variable"))
                    }
                };
                let (local, terms) = scope.lookup(name)?;
                let local = local.clone();
                let value = self.encoded_local(&local, terms)?;
                let x = self.nat_u(number);
                self.apply(
                    "conv_var",
                    &[("p", p), ("env", env), ("x", x), ("v", value)],
                    &[Given::Refl],
                )
            }
            SemanticTerm::Nat { value: _ }
            | SemanticTerm::Integer {
                representation: _,
                value: _,
            }
            | SemanticTerm::String { value: _ }
            | SemanticTerm::Bytes { hex: _ } => {
                let (ty, value) = match expr {
                    Expr::Value { ty, value } => (ty, value),
                    Expr::Var { .. }
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
                        return Err("the lowering of a literal is not a value".to_owned())
                    }
                };
                let ty = self.ty_term(ty);
                let value = self.value_term(value)?;
                self.apply(
                    "conv_value",
                    &[("p", p), ("env", env), ("t", ty), ("v", value)],
                    &[],
                )
            }
            SemanticTerm::Primitive {
                operation,
                arguments,
                result,
            } => self.primitive(term, *operation, arguments, result, expr, scope),
            SemanticTerm::Bool { value } => {
                let construct = self.construct_plain(if *value {
                    "construct_true"
                } else {
                    "construct_false"
                })?;
                self.built(scope, expr, &[], construct)
            }
            SemanticTerm::Unit => {
                let construct = self.construct_plain("construct_unit")?;
                self.built(scope, expr, &[], construct)
            }
            SemanticTerm::Nil { element: _ } => {
                let construct = self.construct_plain("construct_nil")?;
                self.built(scope, expr, &[], construct)
            }
            SemanticTerm::Cons { head, tail } => {
                let (head_expr, tail_expr) = match expr {
                    Expr::Build { operands, .. } if operands.len() == 2 => {
                        (&operands[0], &operands[1])
                    }
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
                        return Err("the lowering of a cons is not a build of two".to_owned())
                    }
                };
                let ty = self.infer(term, scope)?;
                let head_pf = self.prove(head, head_expr, scope)?;
                let tail_pf = self.prove(tail, tail_expr, scope)?;
                let tail_src = self.src(tail, scope)?;
                let construct = self.construct_cons(&ty, head_pf, tail_src)?;
                self.built(scope, expr, &[head_pf, tail_pf], construct)
            }
            SemanticTerm::Record {
                r#type: _,
                type_arguments: _,
                fields,
            } => {
                let ty = self.infer(term, scope)?;
                self.record_proof(&ty, fields, expr, scope)
            }
            SemanticTerm::Constructor {
                constructor,
                type_arguments,
                arguments,
            } => {
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.gen.source.close(argument, &site))
                    .collect();
                let (kind, _) = self.gen.source.constructor(constructor, &closed, &site)?;
                let operand_exprs = self.operand_exprs(expr)?;
                let mut proofs = Vec::new();
                match &kind {
                    Constructor::Violation(first, second) => {
                        for (value, child) in [*first, *second].into_iter().zip(operand_exprs) {
                            let construct = self.construct_plain(if value {
                                "construct_true"
                            } else {
                                "construct_false"
                            })?;
                            proofs.push(self.built(scope, child, &[], construct)?);
                        }
                    }
                    Constructor::Bool(_)
                    | Constructor::Zero
                    | Constructor::Succ
                    | Constructor::Nil
                    | Constructor::Cons
                    | Constructor::OptionNone
                    | Constructor::OptionSome
                    | Constructor::Ok
                    | Constructor::Error
                    | Constructor::Document { ty: _, index: _ } => {
                        for (argument, child) in arguments.iter().zip(operand_exprs) {
                            proofs.push(self.prove(argument, child, scope)?);
                        }
                    }
                }
                let construct = match &kind {
                    Constructor::Bool(true) => self.construct_plain("construct_true")?,
                    Constructor::Bool(false) => self.construct_plain("construct_false")?,
                    Constructor::Zero => self.construct_plain("construct_zero")?,
                    Constructor::Nil => self.construct_plain("construct_nil")?,
                    Constructor::OptionNone => self.construct_plain("construct_none")?,
                    Constructor::Succ => {
                        let n = self.src(&arguments[0], scope)?;
                        self.apply("construct_succ", &[("n", n)], &[])?
                    }
                    Constructor::Cons => {
                        let ty = self.infer(term, scope)?;
                        let tail = self.src(&arguments[1], scope)?;
                        self.construct_cons(&ty, proofs[0], tail)?
                    }
                    Constructor::OptionSome => {
                        let a = self.value_of(proofs[0])?;
                        self.apply("construct_some", &[("a", a)], &[])?
                    }
                    Constructor::Ok => {
                        let a = self.value_of(proofs[0])?;
                        self.apply("construct_ok", &[("a", a)], &[])?
                    }
                    Constructor::Error => {
                        let a = self.value_of(proofs[0])?;
                        self.apply("construct_error", &[("a", a)], &[])?
                    }
                    Constructor::Violation(_, _) => {
                        let a = self.value_of(proofs[0])?;
                        let b = self.value_of(proofs[1])?;
                        self.apply("construct_pair", &[("a", a), ("b", b)], &[])?
                    }
                    Constructor::Document { ty: _, index } => {
                        self.construct_adt(*index as u64, &proofs)?
                    }
                };
                self.built(scope, expr, &proofs, construct)
            }
            SemanticTerm::InstanceValue {
                class: _,
                arguments: _,
                resolved,
            } => {
                let module = Source::module_of(resolved, &site);
                let index = self.gen.instance_index(&module, &resolved.name, &[])?;
                let function = match expr {
                    Expr::Call { function, .. } => *function,
                    Expr::Value { .. }
                    | Expr::Var { .. }
                    | Expr::Let { .. }
                    | Expr::Cond { .. }
                    | Expr::Match { .. }
                    | Expr::Build { .. }
                    | Expr::Closure { .. }
                    | Expr::Apply { .. }
                    | Expr::Prim { .. }
                    | Expr::First { .. }
                    | Expr::Second { .. }
                    | Expr::Field { .. } => {
                        return Err("the lowering of an instance is not a call".to_owned())
                    }
                };
                if function != index {
                    return Err(format!(
                        "an instance at function {index} lowered to a call of {function}"
                    ));
                }
                let list = self.operands(env, &[])?;
                let rel = self.call_relation(index, &[])?;
                self.apply(
                    "conv_call",
                    &[("p", p), ("env", env)],
                    &[
                        Given::Proof(list.proof, list.ty),
                        Given::Proof(rel.proof, rel.ty),
                    ],
                )
            }
            SemanticTerm::Project { value, field } => {
                let (inner_expr, index) = match expr {
                    Expr::Field { value, index } => (value.as_ref(), *index),
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
                    | Expr::Second { .. } => {
                        return Err("the lowering of a projection is not a field".to_owned())
                    }
                };
                let ty = self.infer(value, scope)?;
                let shape = self.gen.source.document(&ty)?;
                let position = shape
                    .field_names
                    .iter()
                    .position(|name| name == field)
                    .ok_or_else(|| format!("`{field}` is not a field"))?;
                let inner = self.prove(value, inner_expr, scope)?;
                // The encoding of the document is `Value.adt 0` of its
                // fields' encodings; the projected field is the one at
                // `index`.
                let src = self.src(value, scope)?;
                let mut fields = Vec::new();
                for (name, field_ty) in shape.field_names.iter().zip(&shape.fields[0]) {
                    let projected = self.project_term(&ty, name, src)?;
                    let enc = self.enc(field_ty)?;
                    fields.push(self.b.app(enc, projected));
                }
                let value_ty = self.value_ty();
                let list = self.b.list(value_ty, &fields);
                let selected = fields[position];
                let ctor = self.nat_u(0);
                let k = self.nat_u(index);
                self.apply(
                    "conv_field",
                    &[
                        ("p", p),
                        ("env", env),
                        ("k", k),
                        ("fields", list),
                        ("ctor", ctor),
                        ("v", selected),
                    ],
                    &[Given::Proof(inner.proof, inner.ty), Given::Refl],
                )
            }
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } => {
                let module = Source::module_of(function, &site);
                let closed: Vec<SemanticType> = type_arguments
                    .iter()
                    .map(|argument| self.gen.source.close(argument, &site))
                    .collect();
                let index = self.gen.instance_index(&module, &function.name, &closed)?;
                let (callee, operand_exprs) = match expr {
                    Expr::Call { function, operands } => (*function, operands),
                    Expr::Value { .. }
                    | Expr::Var { .. }
                    | Expr::Let { .. }
                    | Expr::Cond { .. }
                    | Expr::Match { .. }
                    | Expr::Build { .. }
                    | Expr::Closure { .. }
                    | Expr::Apply { .. }
                    | Expr::Prim { .. }
                    | Expr::First { .. }
                    | Expr::Second { .. }
                    | Expr::Field { .. } => {
                        return Err("the lowering of a call is not a call".to_owned())
                    }
                };
                if callee != index {
                    return Err(format!("a call of function {index} lowered to {callee}"));
                }
                let mut proofs = Vec::new();
                let mut relation_args = Vec::new();
                let mut fits_args = Vec::new();
                for (argument, child) in arguments.iter().zip(operand_exprs) {
                    let passed = self.argument(argument, child, scope)?;
                    proofs.push(passed.proof);
                    relation_args.extend(passed.relation);
                    fits_args.extend(passed.fits);
                }
                let list = self.operands(env, &proofs)?;
                let recursive = self.wf.as_ref().is_some_and(|state| state.index == index);
                let rel = if recursive {
                    self.wf_call(index, arguments, &relation_args, scope)?
                } else {
                    self.call_relation(index, &relation_args)?
                };
                self.apply(
                    "conv_call",
                    &[("p", p), ("env", env)],
                    &[
                        Given::Proof(list.proof, list.ty),
                        Given::Proof(rel.proof, rel.ty),
                    ],
                )
            }
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => {
                let (c, t, e) = match expr {
                    Expr::Cond {
                        condition,
                        then_branch,
                        else_branch,
                    } => (
                        condition.as_ref(),
                        then_branch.as_ref(),
                        else_branch.as_ref(),
                    ),
                    Expr::Value { .. }
                    | Expr::Var { .. }
                    | Expr::Let { .. }
                    | Expr::Match { .. }
                    | Expr::Build { .. }
                    | Expr::Call { .. }
                    | Expr::Closure { .. }
                    | Expr::Apply { .. }
                    | Expr::Prim { .. }
                    | Expr::First { .. }
                    | Expr::Second { .. }
                    | Expr::Field { .. } => {
                        return Err("the lowering of a conditional is not a cond".to_owned())
                    }
                };
                self.conditional(term, condition, then_value, else_value, (c, t, e), scope)
            }
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => self.matching(term, scrutinee, branches, expr, scope),
            SemanticTerm::Add { left, right } => {
                self.binary_prim(left, right, expr, scope, |writer, a, b| {
                    writer.apply("prim_natAdd", &[], &[Given::Term(a), Given::Term(b)])
                })
            }
            SemanticTerm::Beq { left, right } => {
                self.binary_prim(left, right, expr, scope, |writer, a, b| {
                    writer.apply("prim_natEq", &[], &[Given::Term(a), Given::Term(b)])
                })
            }
            SemanticTerm::Ble { left, right } => {
                self.binary_prim(left, right, expr, scope, |writer, a, b| {
                    writer.apply("prim_natLe", &[], &[Given::Term(a), Given::Term(b)])
                })
            }
            SemanticTerm::Blt { left, right } => {
                self.binary_prim(left, right, expr, scope, |writer, a, b| {
                    writer.apply("prim_natLt", &[], &[Given::Term(a), Given::Term(b)])
                })
            }
            SemanticTerm::And { left, right } => {
                let (c, t, e) = match expr {
                    Expr::Cond {
                        condition,
                        then_branch,
                        else_branch,
                    } => (
                        condition.as_ref(),
                        then_branch.as_ref(),
                        else_branch.as_ref(),
                    ),
                    Expr::Value { .. }
                    | Expr::Var { .. }
                    | Expr::Let { .. }
                    | Expr::Match { .. }
                    | Expr::Build { .. }
                    | Expr::Call { .. }
                    | Expr::Closure { .. }
                    | Expr::Apply { .. }
                    | Expr::Prim { .. }
                    | Expr::First { .. }
                    | Expr::Second { .. }
                    | Expr::Field { .. } => {
                        return Err("the lowering of && is not a cond".to_owned())
                    }
                };
                let else_term = SemanticTerm::Bool { value: false };
                self.conditional_of(
                    Selection::And,
                    None,
                    left,
                    right,
                    &else_term,
                    (c, t, e),
                    scope,
                )
            }
            SemanticTerm::Or { left, right } => {
                let (c, t, e) = match expr {
                    Expr::Cond {
                        condition,
                        then_branch,
                        else_branch,
                    } => (
                        condition.as_ref(),
                        then_branch.as_ref(),
                        else_branch.as_ref(),
                    ),
                    Expr::Value { .. }
                    | Expr::Var { .. }
                    | Expr::Let { .. }
                    | Expr::Match { .. }
                    | Expr::Build { .. }
                    | Expr::Call { .. }
                    | Expr::Closure { .. }
                    | Expr::Apply { .. }
                    | Expr::Prim { .. }
                    | Expr::First { .. }
                    | Expr::Second { .. }
                    | Expr::Field { .. } => {
                        return Err("the lowering of || is not a cond".to_owned())
                    }
                };
                let then_term = SemanticTerm::Bool { value: true };
                self.conditional_of(
                    Selection::Or,
                    None,
                    left,
                    &then_term,
                    right,
                    (c, t, e),
                    scope,
                )
            }
            SemanticTerm::Not { value } => {
                let operand_exprs = match expr {
                    Expr::Prim { operands, .. } => operands,
                    Expr::Value { .. }
                    | Expr::Var { .. }
                    | Expr::Let { .. }
                    | Expr::Cond { .. }
                    | Expr::Match { .. }
                    | Expr::Build { .. }
                    | Expr::Call { .. }
                    | Expr::Closure { .. }
                    | Expr::Apply { .. }
                    | Expr::First { .. }
                    | Expr::Second { .. }
                    | Expr::Field { .. } => {
                        return Err("the lowering of ! is not a primitive".to_owned())
                    }
                };
                let inner = self.prove(value, &operand_exprs[0], scope)?;
                let src = self.src(value, scope)?;
                let list = self.operands(env, &[inner])?;
                let lemma = self.apply("prim_boolNot", &[], &[Given::Term(src)])?;
                self.apply(
                    "conv_prim",
                    &[("p", p), ("env", env)],
                    &[
                        Given::Proof(list.proof, list.ty),
                        Given::Proof(lemma.proof, lemma.ty),
                    ],
                )
            }
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => self.let_proof(binder, value, body, expr, scope),
            SemanticTerm::Pair { left, right } => {
                let operand_exprs = self.operand_exprs(expr)?;
                let l = self.prove(left, &operand_exprs[0], scope)?;
                let r = self.prove(right, &operand_exprs[1], scope)?;
                let a = self.value_of(l)?;
                let b = self.value_of(r)?;
                let construct = self.apply("construct_pair", &[("a", a), ("b", b)], &[])?;
                self.built(scope, expr, &[l, r], construct)
            }
            SemanticTerm::First { value } | SemanticTerm::Second { value } => {
                let inner_expr = match expr {
                    Expr::First { value } | Expr::Second { value } => value.as_ref(),
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
                    | Expr::Field { .. } => {
                        return Err("the lowering of a projection is not first or second".to_owned())
                    }
                };
                let ty = self.infer(value, scope)?;
                let (left, right) = match &ty {
                    SemanticType::Product { left, right } => (left.as_ref(), right.as_ref()),
                    SemanticType::Type
                    | SemanticType::Parameter { .. }
                    | SemanticType::Nat
                    | SemanticType::Bool
                    | SemanticType::Prop
                    | SemanticType::Unit
                    | SemanticType::Int
                    | SemanticType::Int8
                    | SemanticType::Int16
                    | SemanticType::Int32
                    | SemanticType::Int64
                    | SemanticType::UInt8
                    | SemanticType::UInt16
                    | SemanticType::UInt32
                    | SemanticType::UInt64
                    | SemanticType::String
                    | SemanticType::Bytes
                    | SemanticType::Ordering
                    | SemanticType::Option { .. }
                    | SemanticType::Result { .. }
                    | SemanticType::List { .. }
                    | SemanticType::Named { .. }
                    | SemanticType::Function { .. }
                    | SemanticType::Map { .. }
                    | SemanticType::Set { .. }
                    | SemanticType::ContractViolation
                    | SemanticType::ReasoningFailure => {
                        return Err("a projection of a non-product".to_owned())
                    }
                };
                let inner = self.prove(value, inner_expr, scope)?;
                // The pair's encoding unfolds to the pair of its components'.
                let src = self.src(value, scope)?;
                let (alpha, beta) = (self.lean_ty(left)?, self.lean_ty(right)?);
                let fst = self.b.konst00("Prod.fst");
                let fst = self.b.apps(fst, &[alpha, beta, src]);
                let snd = self.b.konst00("Prod.snd");
                let snd = self.b.apps(snd, &[alpha, beta, src]);
                let (enc_left, enc_right) = (self.enc(left)?, self.enc(right)?);
                let a = self.b.app(enc_left, fst);
                let b = self.b.app(enc_right, snd);
                let name = if matches!(term, SemanticTerm::First { value: _ }) {
                    "conv_first"
                } else {
                    "conv_second"
                };
                self.apply(
                    name,
                    &[("p", p), ("env", env), ("a", a), ("b", b)],
                    &[Given::Proof(inner.proof, inner.ty)],
                )
            }
            SemanticTerm::Apply {
                function,
                arguments,
            } => self.application(function, arguments, expr, scope),
            SemanticTerm::MapLiteral {
                key: _,
                value: _,
                entries,
            } => {
                let mut items = Vec::new();
                let mut cursor = Some(expr);
                for SemanticMapEntry { key, value } in entries {
                    let node = cursor.ok_or("a map literal lowered to a shorter chain")?;
                    let (item_expr, rest) = self.chain(node)?;
                    let operand_exprs = self.operand_exprs(item_expr)?;
                    let k = self.prove(key, &operand_exprs[0], scope)?;
                    let v = self.prove(value, &operand_exprs[1], scope)?;
                    let a = self.value_of(k)?;
                    let b = self.value_of(v)?;
                    let construct = self.apply("construct_pair", &[("a", a), ("b", b)], &[])?;
                    items.push(self.built(scope, item_expr, &[k, v], construct)?);
                    cursor = Some(rest);
                }
                self.cons_chain(scope, expr, &items)
            }
            SemanticTerm::SetLiteral {
                element: _,
                elements,
            } => {
                let mut items = Vec::new();
                let mut cursor = expr;
                for element in elements {
                    let (item_expr, rest) = self.chain(cursor)?;
                    items.push(self.prove(element, item_expr, scope)?);
                    cursor = rest;
                }
                self.cons_chain(scope, expr, &items)
            }
            SemanticTerm::GraphLiteral {
                node: _,
                nodes,
                edges,
            } => {
                let mut items = Vec::new();
                let mut cursor = expr;
                for source in nodes {
                    let (item_expr, rest) = self.chain(cursor)?;
                    let operand_exprs = self.operand_exprs(item_expr)?;
                    let source_pf = self.prove(source, &operand_exprs[0], scope)?;
                    let mut targets = Vec::new();
                    let mut target_cursor = &operand_exprs[1];
                    for SemanticEdge {
                        source: from,
                        target,
                    } in edges
                    {
                        if from == source {
                            let (target_expr, target_rest) = self.chain(target_cursor)?;
                            targets.push(self.prove(target, target_expr, scope)?);
                            target_cursor = target_rest;
                        }
                    }
                    let chain = self.cons_chain(scope, &operand_exprs[1], &targets)?;
                    let a = self.value_of(source_pf)?;
                    let b = self.value_of(chain)?;
                    let construct = self.apply("construct_pair", &[("a", a), ("b", b)], &[])?;
                    items.push(self.built(scope, item_expr, &[source_pf, chain], construct)?);
                    cursor = rest;
                }
                self.cons_chain(scope, expr, &items)
            }
            SemanticTerm::CheckedApply {
                model: _,
                type_arguments: _,
                arguments: _,
                checks: _,
            } => Err(
                "a checked model application is certified only through its elaboration".to_owned(),
            ),
            SemanticTerm::Lambda {
                parameters: _,
                captures: _,
                body: _,
            }
            | SemanticTerm::FunctionRef {
                function: _,
                type_arguments: _,
            } => Err("a function value outside an argument or application position".to_owned()),
            SemanticTerm::Eq { left: _, right: _ }
            | SemanticTerm::Le { left: _, right: _ }
            | SemanticTerm::Lt { left: _, right: _ }
            | SemanticTerm::PropAnd { left: _, right: _ }
            | SemanticTerm::Implies {
                premise: _,
                conclusion: _,
            }
            | SemanticTerm::Iff { left: _, right: _ }
            | SemanticTerm::Forall { binder: _, body: _ } => Err(format!(
                "the formal-only construct `{}` has no realization",
                super::super::super::eligibility::term_key(term)
            )),
        }
    }

    /// The proof of a record of closed type `ty`: a build of its fields in
    /// declaration order.
    pub(super) fn record_proof(
        &mut self,
        ty: &SemanticType,
        fields: &[crate::ir::semantic::SemanticAssignment],
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        let shape = self.gen.source.document(ty)?;
        let operand_exprs = self.operand_exprs(expr)?;
        let mut proofs = Vec::new();
        for (position, field) in shape.field_names.iter().enumerate() {
            let value = &fields
                .iter()
                .find(|assignment| &assignment.field == field)
                .ok_or_else(|| format!("field `{field}` is not assigned"))?
                .value;
            let child = operand_exprs
                .get(position)
                .ok_or("a record lowered to fewer operands than fields")?;
            proofs.push(self.prove(value, child, scope)?);
        }
        let construct = self.construct_adt(0, &proofs)?;
        self.built(scope, expr, &proofs, construct)
    }

    // --- Expression plumbing ----------------------------------------------

    /// The operands of a build or a primitive.
    fn operand_exprs(&self, expr: &'a Expr) -> Result<&'a [Expr], String> {
        match expr {
            Expr::Build { operands, .. } => Ok(operands),
            Expr::Value { .. }
            | Expr::Var { .. }
            | Expr::Let { .. }
            | Expr::Cond { .. }
            | Expr::Match { .. }
            | Expr::Call { .. }
            | Expr::Closure { .. }
            | Expr::Apply { .. }
            | Expr::Prim { .. }
            | Expr::First { .. }
            | Expr::Second { .. }
            | Expr::Field { .. } => Err("the lowering of a construction is not a build".to_owned()),
        }
    }

    /// The head and tail of a cons build in a chain.
    fn chain(&self, expr: &'a Expr) -> Result<(&'a Expr, &'a Expr), String> {
        match expr {
            Expr::Build {
                shape: Shape::Cons,
                operands,
                ..
            } if operands.len() == 2 => Ok((&operands[0], &operands[1])),
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
                Err("a literal collection lowered to something other than a cons chain".to_owned())
            }
        }
    }

    /// The proof of a literal collection: a cons chain of its items ending in
    /// a nil, over the expression `expr` the lowering gave it.
    fn cons_chain(&mut self, scope: &Scope, expr: &'a Expr, items: &[Pf]) -> Result<Pf, String> {
        let mut spine: Vec<&'a Expr> = Vec::new();
        let mut cursor = expr;
        for _ in items {
            spine.push(cursor);
            cursor = self.chain(cursor)?.1;
        }
        let nil = self.construct_plain("construct_nil")?;
        let mut out = self.built(scope, cursor, &[], nil)?;
        for (item, node) in items.iter().zip(spine).rev() {
            let h = self.value_of(*item)?;
            let tail_value = self.value_of(out)?;
            let t = self.list_payload(tail_value)?;
            let construct = self.apply("construct_cons", &[("h", h), ("t", t)], &[])?;
            out = self.built(scope, node, &[*item, out], construct)?;
        }
        Ok(out)
    }

    /// The list a `Value.list` wraps.
    fn list_payload(&mut self, value: Term) -> Result<Term, String> {
        let (head, args) = self.b.spine(value);
        let is_list = matches!(self.b.node(head),
            crate::ir::core::CoreNode::Const { n, .. } if n.ends_with("TargetSyntax.Value.list"));
        if is_list && args.len() == 1 {
            Ok(args[0])
        } else {
            Err(format!(
                "a list value {} is not a Value.list",
                self.b.show(value)
            ))
        }
    }

    /// `construct_cons` at the value of `head` and the payload of the
    /// encoding of the list `tail_src`.
    fn construct_cons(
        &mut self,
        ty: &SemanticType,
        head: Pf,
        tail_src: Term,
    ) -> Result<Pf, String> {
        let element = element_of(ty)?;
        let alpha = self.lean_ty(&element)?;
        let e = self.enc(&element)?;
        let h = self.value_of(head)?;
        let value = self.value_ty();
        let map = self.b.konst_at(
            "List.map",
            vec![
                crate::ir::core::CoreLevel::Zero,
                crate::ir::core::CoreLevel::Zero,
            ],
        );
        let t = self.b.apps(map, &[alpha, value, e, tail_src]);
        self.apply("construct_cons", &[("h", h), ("t", t)], &[])
    }

    /// `construct_adt` at the constructor `index` and the operands' values.
    fn construct_adt(&mut self, index: u64, proofs: &[Pf]) -> Result<Pf, String> {
        let values = proofs
            .iter()
            .map(|pf| self.value_of(*pf))
            .collect::<Result<Vec<_>, _>>()?;
        let ty = self.value_ty();
        let list = self.b.list(ty, &values);
        let k = self.nat_u(index);
        self.apply("construct_adt", &[("k", k), ("fs", list)], &[])
    }

    /// The projection of `field` out of `value`, a document of type `ty`.
    pub(super) fn project_term(
        &mut self,
        ty: &SemanticType,
        field: &str,
        value: Term,
    ) -> Result<Term, String> {
        let (member, arguments) = match ty {
            SemanticType::Named { member, arguments } => (member, arguments),
            SemanticType::Type
            | SemanticType::Parameter { .. }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Prop
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64
            | SemanticType::String
            | SemanticType::Bytes
            | SemanticType::Ordering
            | SemanticType::Option { .. }
            | SemanticType::Result { .. }
            | SemanticType::List { .. }
            | SemanticType::Product { .. }
            | SemanticType::Function { .. }
            | SemanticType::Map { .. }
            | SemanticType::Set { .. }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {
                return Err("a projection from a non-document".to_owned())
            }
        };
        let owner = self.document_name(member)?;
        let head = self.b.konst(&format!("{owner}.{field}"));
        let mut all = Vec::new();
        for argument in arguments {
            all.push(self.lean_ty(argument)?);
        }
        all.push(value);
        Ok(self.b.apps(head, &all))
    }

    /// A primitive over two natural operands, realized by `rule`.
    fn binary_prim(
        &mut self,
        left: &SemanticTerm,
        right: &SemanticTerm,
        expr: &'a Expr,
        scope: &Scope,
        rule: impl FnOnce(&mut Self, Term, Term) -> Result<Pf, String>,
    ) -> Result<Pf, String> {
        let operand_exprs = match expr {
            Expr::Prim { operands, .. } if operands.len() == 2 => operands,
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
                return Err("the lowering of an arithmetic term is not a primitive".to_owned())
            }
        };
        let l = self.prove(left, &operand_exprs[0], scope)?;
        let r = self.prove(right, &operand_exprs[1], scope)?;
        let list = self.operands(scope.env, &[l, r])?;
        let a = self.src(left, scope)?;
        let b = self.src(right, scope)?;
        let lemma = rule(self, a, b)?;
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

    /// A conditional: `conv_cond` with the predicate and value motives that
    /// select among the branches.
    fn conditional(
        &mut self,
        whole: &SemanticTerm,
        condition: &SemanticTerm,
        then_value: &SemanticTerm,
        else_value: &SemanticTerm,
        exprs: (&'a Expr, &'a Expr, &'a Expr),
        scope: &Scope,
    ) -> Result<Pf, String> {
        let result_ty = self.infer(whole, scope)?;
        let node = self.node_of(scope, whole);
        self.conditional_of(
            Selection::If(result_ty),
            node,
            condition,
            then_value,
            else_value,
            exprs,
            scope,
        )
    }

    /// A conditional whose value is selected as `kind` says. A numbered
    /// `if` of a well-founded body (`node`) is written by cases on its
    /// condition with the equation each arm was taken by.
    #[allow(clippy::too_many_arguments)]
    fn conditional_of(
        &mut self,
        kind: Selection,
        node: Option<usize>,
        condition: &SemanticTerm,
        then_value: &SemanticTerm,
        else_value: &SemanticTerm,
        exprs: (&'a Expr, &'a Expr, &'a Expr),
        scope: &Scope,
    ) -> Result<Pf, String> {
        let (p, env) = (self.prog, scope.env);
        let tested = self.prove(condition, exprs.0, scope)?;
        let equations = node.map(|_| (self.b.free(), self.b.free()));
        let (then_scope, else_scope) = match (node, equations) {
            (Some(node), Some((on_true, on_false))) => (
                Self::entering(
                    scope,
                    Step::If {
                        node,
                        polarity: true,
                    },
                    on_true,
                    &[],
                ),
                Self::entering(
                    scope,
                    Step::If {
                        node,
                        polarity: false,
                    },
                    on_false,
                    &[],
                ),
            ),
            _ => (scope.clone(), scope.clone()),
        };
        let then_pf = self.prove(then_value, exprs.1, &then_scope)?;
        let else_pf = self.prove(else_value, exprs.2, &else_scope)?;
        let condition_src = self.src(condition, scope)?;
        let bool = self.b.konst("Bool");
        let guard = self.b.free();
        let then_fits = self.fits_of(then_pf)?;
        let else_fits = self.fits_of(else_pf)?;
        let fits_body = match equations {
            None => self.ite(bool, guard, then_fits, else_fits),
            Some((on_true, on_false)) => {
                let (tru, fls) = (self.tru(), self.fls());
                let true_eq = self.eq(bool, guard, tru);
                let false_eq = self.eq(bool, guard, fls);
                let then_arm = self.arm_lambda(&[], (on_true, true_eq), then_fits);
                let else_arm = self.arm_lambda(&[], (on_false, false_eq), else_fits);
                let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
                self.equation_cases(
                    &SemanticType::Bool,
                    guard,
                    bool,
                    one,
                    vec![(0, else_arm), (1, then_arm)],
                )?
            }
        };
        let predicate = self.b.lam_over("__b", EXPLICIT, bool, guard, fits_body);
        let encoded = match &kind {
            Selection::If(result_ty) => {
                let lean = self.lean_ty(result_ty)?;
                let encoder = self.enc(result_ty)?;
                let then_src = self.src(then_value, scope)?;
                let else_src = self.src(else_value, scope)?;
                let selected = match equations {
                    None => self.ite(lean, guard, then_src, else_src),
                    Some(_) => {
                        let (tru, fls) = (self.tru(), self.fls());
                        let true_eq = self.eq(bool, guard, tru);
                        let false_eq = self.eq(bool, guard, fls);
                        let (unused_true, unused_false) = (self.b.free(), self.b.free());
                        let then_arm = self.arm_lambda(&[], (unused_true, true_eq), then_src);
                        let else_arm = self.arm_lambda(&[], (unused_false, false_eq), else_src);
                        let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
                        self.equation_cases(
                            &SemanticType::Bool,
                            guard,
                            lean,
                            one,
                            vec![(0, else_arm), (1, then_arm)],
                        )?
                    }
                };
                self.b.app(encoder, selected)
            }
            Selection::And => {
                let right = self.src(then_value, scope)?;
                let combined = self.and(guard, right);
                let ctor = self.ts("Value.bool");
                self.b.app(ctor, combined)
            }
            Selection::Or => {
                let right = self.src(else_value, scope)?;
                let head = self.b.konst("Bool.or");
                let combined = self.b.apps(head, &[guard, right]);
                let ctor = self.ts("Value.bool");
                self.b.app(ctor, combined)
            }
        };
        let value_fn = self.b.lam_over("__b", EXPLICIT, bool, guard, encoded);
        let tru = self.tru();
        let fls = self.fls();
        let then_branch =
            self.guarded(condition_src, tru, then_pf, equations.map(|pair| pair.0))?;
        let else_branch =
            self.guarded(condition_src, fls, else_pf, equations.map(|pair| pair.1))?;
        self.apply(
            "conv_cond",
            &[("p", p), ("env", env)],
            &[
                Given::Term(predicate),
                Given::Term(value_fn),
                Given::Term(condition_src),
                Given::Proof(tested.proof, tested.ty),
                Given::Proof(then_branch.0, then_branch.1),
                Given::Proof(else_branch.0, else_branch.1),
            ],
        )
    }

    /// `fun (h : cond = value) => proof`, with its type; `hypothesis` is the
    /// free variable the proof reads the equation by, when it does.
    fn guarded(
        &mut self,
        condition: Term,
        value: Term,
        pf: Pf,
        hypothesis: Option<Term>,
    ) -> Result<(Term, Term), String> {
        let bool = self.b.konst("Bool");
        let ty = self.eq(bool, condition, value);
        let hypothesis = match hypothesis {
            Some(hypothesis) => hypothesis,
            None => self.b.free(),
        };
        let proof = self.b.lam_over("_", EXPLICIT, ty, hypothesis, pf.proof);
        let result = self.b.pi_over("_", EXPLICIT, ty, hypothesis, pf.ty);
        Ok((proof, result))
    }

    // --- Let ----------------------------------------------------------------

    fn let_proof(
        &mut self,
        binder: &crate::ir::semantic::SemanticParameter,
        value: &SemanticTerm,
        body: &SemanticTerm,
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        let (number, ty, bound_expr, body_expr) = match expr {
            Expr::Let {
                name,
                ty,
                bound,
                body,
            } => (*name, ty, bound.as_ref(), body.as_ref()),
            Expr::Value { .. }
            | Expr::Var { .. }
            | Expr::Cond { .. }
            | Expr::Match { .. }
            | Expr::Build { .. }
            | Expr::Call { .. }
            | Expr::Closure { .. }
            | Expr::Apply { .. }
            | Expr::Prim { .. }
            | Expr::First { .. }
            | Expr::Second { .. }
            | Expr::Field { .. } => return Err("the lowering of a let is not a let".to_owned()),
        };
        let (local, generic) = self.gen.let_local(binder, &scope.ctx);
        let local_ty = local.ty.clone();
        let lean = self.lean_ty(&local_ty)?;
        let bound = self.prove(value, bound_expr, scope)?;
        let bound_src = self.src(value, scope)?;
        let free = self.b.free();
        let enc = self.enc(&local_ty)?;
        let encoded = self.b.app(enc, free);
        let name = self.nat_u(number);
        let env = self.extend_env(scope.env, &[number], &[encoded])?;
        let inner = self.bind(scope, vec![(local, generic)], &[free], env);
        let rest = self.prove(body, body_expr, &inner)?;
        let (p, outer) = (self.prog, scope.env);
        let target = self.ty_term(ty);
        let applied = self.apply(
            "conv_let",
            &[("p", p), ("env", outer), ("t", target), ("x", name)],
            &[
                Given::Proof(bound.proof, bound.ty),
                Given::Proof(rest.proof, rest.ty),
            ],
        )?;
        // The binding is substituted, not bound: the type of the derivation
        // stays the `Conv` of the lowered `let` for the derivation above it to
        // read, and the kernel sees the same term either way.
        let _ = lean;
        let proof = self.b.substitute(applied.proof, &[free], &[bound_src]);
        let ty = self.b.substitute(applied.ty, &[free], &[bound_src]);
        Ok(Pf { proof, ty })
    }
}
