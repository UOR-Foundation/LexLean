//! Collection templates and key comparisons (SPEC.md §17.17.3).
//!
//! A collection primitive is lowered to a call of a library template's entry
//! function. Its relation is the template's lemma, applied at the call's
//! runtime functions: the lemma takes, as hypotheses, what each runtime
//! function does on an empty list and on each outcome of a key comparison
//! (`clauses.rs` proves them), the encoders of the call's types, and the
//! placement of the template's functions in the program.

use super::super::{element_of, instance_type};
use super::calls::FunctionArgument;
use super::clauses::{Arg, Form};
use super::{Pf, Scope, Writer};
use crate::calculus::library::Template;
use crate::calculus::{Expr, Ty};
use crate::ir::semantic::{SemanticPrimitive, SemanticTerm, SemanticType};
use crate::production::kernel::{Given, Term};
use crate::production::source::Site;

/// What a template call is made of: where it is read, the template's entry
/// function in the program, the call's types, and its operands.
struct Call {
    site: Site,
    entry: Term,
    tys: Vec<Ty>,
    instance: Vec<SemanticType>,
    types: Vec<SemanticType>,
    values: Vec<Term>,
    functions: Vec<FunctionArgument>,
}

impl Call {
    fn value(&self, index: usize, template: Template) -> Result<Term, String> {
        self.values
            .get(index)
            .copied()
            .ok_or_else(|| format!("template {} lacks operand {index}", template.name()))
    }

    fn operand_type(&self, index: usize, template: Template) -> Result<&SemanticType, String> {
        self.types
            .get(index)
            .ok_or_else(|| format!("template {} lacks operand {index}", template.name()))
    }

    fn step(&self, template: Template) -> Result<&FunctionArgument, String> {
        self.functions
            .first()
            .ok_or_else(|| format!("template {} lacks its step", template.name()))
    }

    fn at(&self, index: usize) -> Result<SemanticType, String> {
        instance_type(&self.instance, index)
    }
}

impl<'a> Writer<'_, 'a> {
    /// A template call.
    pub(super) fn template_call(
        &mut self,
        template: Template,
        instance: &[SemanticType],
        arguments: &[SemanticTerm],
        types: &[SemanticType],
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        let tys = instance
            .iter()
            .map(|ty| self.gen.target_ty(ty))
            .collect::<Result<Vec<_>, _>>()?;
        let entry = *self
            .gen
            .templates
            .get(&(template.name().to_owned(), tys.clone()))
            .ok_or_else(|| format!("template {} has no instance in the layout", template.name()))?;
        let Expr::Call { function, operands } = expr else {
            return Err("the lowering of a template call is not a call".to_owned());
        };
        let (function, operand_exprs) = (*function, operands);
        if function != entry || operand_exprs.len() != arguments.len() {
            return Err(format!(
                "a call of template {} at function {entry} lowered to a call of {function} with {} operands",
                template.name(),
                operand_exprs.len()
            ));
        }
        let mut values = Vec::new();
        let mut proofs = Vec::new();
        let mut functions = Vec::new();
        for (argument, child) in arguments.iter().zip(operand_exprs) {
            match self.function_argument(argument, child, scope)? {
                Some(companion) => {
                    values.push(companion.value);
                    proofs.push(companion.proof);
                    functions.push(companion);
                }
                None => {
                    values.push(self.src(argument, scope)?);
                    proofs.push(self.prove(argument, child, scope)?);
                }
            }
        }
        let entry_term = self.nat_u(entry);
        let call = Call {
            site: scope.ctx.site.clone(),
            entry: entry_term,
            tys,
            instance: instance.to_vec(),
            types: types.to_vec(),
            values,
            functions,
        };
        let lemma = self.template_lemma(template, &call)?;
        let list = self.operands(scope.env, &proofs)?;
        let (p, env) = (self.prog, scope.env);
        self.apply(
            "conv_call",
            &[("p", p), ("env", env)],
            &[
                Given::Proof(list.proof, list.ty),
                Given::Proof(lemma.proof, lemma.ty),
            ],
        )
    }

    /// The hints every template lemma takes: the program, the entry function,
    /// and the target types the entry is instantiated at, under the names the
    /// lemma gives them.
    fn call_hints(&mut self, call: &Call, names: &[&'static str]) -> Vec<(&'static str, Term)> {
        let mut hints = vec![("p", self.prog), ("fi", call.entry)];
        for (name, ty) in names.iter().zip(&call.tys) {
            let term = self.ty_term(ty);
            hints.push((name, term));
        }
        hints
    }

    /// The template's lemma at the call.
    #[allow(clippy::too_many_lines)]
    fn template_lemma(&mut self, template: Template, call: &Call) -> Result<Pf, String> {
        match template {
            Template::MapInsert => {
                let (key, stored) = (call.at(0)?, call.at(1)?);
                let (kappa, nu) = (self.lean_ty(&key)?, self.lean_ty(&stored)?);
                let spec = self.key_spec(&call.site, &key)?;
                let instance = self.key_instance(&call.site, &key)?;
                let function =
                    self.collections_at(&call.site, "insertEntry", &[kappa, nu, instance])?;
                let form = Form::InsertEntry {
                    kappa,
                    nu,
                    function,
                };
                let (ev, list) = (
                    self.enc(&stored)?,
                    self.list_bundle(call.operand_type(0, template)?)?,
                );
                let mut hints = self.call_hints(call, &["tk", "tw"]);
                hints.push(("ν", nu));
                let mut args = vec![
                    Arg::proof(spec),
                    Arg::term(ev),
                    Arg::term(list),
                    Arg::refl(),
                    Arg::term(function),
                    Arg::refl(),
                    Arg::Clause(5, form),
                    Arg::Clause(5, form),
                    Arg::Clause(5, form),
                    Arg::refl(),
                ];
                args.extend(self.operand_args(call, template, 0..3)?);
                self.lemma_args("tpl_mapInsert", &hints, &args)
            }
            Template::MapRemove => {
                let (key, stored) = (call.at(0)?, call.at(1)?);
                let (kappa, nu) = (self.lean_ty(&key)?, self.lean_ty(&stored)?);
                let spec = self.key_spec(&call.site, &key)?;
                let instance = self.key_instance(&call.site, &key)?;
                let function =
                    self.collections_at(&call.site, "removeEntry", &[kappa, nu, instance])?;
                let form = Form::RemoveEntry {
                    kappa,
                    nu,
                    function,
                };
                let (ev, list) = (
                    self.enc(&stored)?,
                    self.list_bundle(call.operand_type(0, template)?)?,
                );
                let mut hints = self.call_hints(call, &["tk", "tw"]);
                hints.push(("ν", nu));
                let mut args = vec![
                    Arg::proof(spec),
                    Arg::term(ev),
                    Arg::term(list),
                    Arg::refl(),
                    Arg::term(function),
                    Arg::refl(),
                    Arg::Clause(4, form),
                    Arg::Clause(4, form),
                    Arg::Clause(4, form),
                    Arg::refl(),
                ];
                args.extend(self.operand_args(call, template, 0..2)?);
                self.lemma_args("tpl_mapRemove", &hints, &args)
            }
            Template::MapLookup => {
                let (key, stored) = (call.at(0)?, call.at(1)?);
                let (kappa, nu) = (self.lean_ty(&key)?, self.lean_ty(&stored)?);
                let spec = self.key_spec(&call.site, &key)?;
                let instance = self.key_instance(&call.site, &key)?;
                let function =
                    self.collections_at(&call.site, "lookupEntry", &[kappa, nu, instance])?;
                let form = Form::LookupEntry { nu, function };
                let ev = self.enc(&stored)?;
                let list = self.list_bundle(call.operand_type(0, template)?)?;
                let options = self.option_bundle(&SemanticType::Option {
                    value: Box::new(stored.clone()),
                })?;
                let mut hints = self.call_hints(call, &["tk", "tw"]);
                hints.push(("ν", nu));
                let mut args = vec![
                    Arg::proof(spec),
                    Arg::term(ev),
                    Arg::term(list),
                    Arg::refl(),
                    Arg::term(options),
                    Arg::refl(),
                    Arg::term(function),
                    Arg::refl(),
                    Arg::Clause(4, form),
                    Arg::Clause(4, form),
                    Arg::Clause(4, form),
                    Arg::refl(),
                ];
                args.extend(self.operand_args(call, template, 0..2)?);
                self.lemma_args("tpl_mapLookup", &hints, &args)
            }
            Template::MapContains => {
                let (key, stored) = (call.at(0)?, call.at(1)?);
                let (kappa, nu) = (self.lean_ty(&key)?, self.lean_ty(&stored)?);
                let spec = self.key_spec(&call.site, &key)?;
                let instance = self.key_instance(&call.site, &key)?;
                let function =
                    self.collections_at(&call.site, "lookupEntry", &[kappa, nu, instance])?;
                let form = Form::LookupEntry { nu, function };
                let ev = self.enc(&stored)?;
                let list = self.list_bundle(call.operand_type(0, template)?)?;
                let mut hints = self.call_hints(call, &["tk", "tw"]);
                hints.push(("ν", nu));
                let mut args = vec![
                    Arg::proof(spec),
                    Arg::term(ev),
                    Arg::term(list),
                    Arg::refl(),
                    Arg::term(function),
                    Arg::refl(),
                    Arg::Clause(4, form),
                    Arg::Clause(4, form),
                    Arg::Clause(4, form),
                ];
                args.extend(Arg::refls(2));
                args.extend(self.operand_args(call, template, 0..2)?);
                self.lemma_args("tpl_mapContains", &hints, &args)
            }
            Template::MapKeys | Template::MapValues => {
                let (key, stored) = (call.at(0)?, call.at(1)?);
                let (lemma, projected) = match template {
                    Template::MapKeys => ("tpl_mapKeys", key.clone()),
                    Template::MapValues => ("tpl_mapValues", stored.clone()),
                    Template::MapInsert
                    | Template::MapRemove
                    | Template::MapLookup
                    | Template::MapContains
                    | Template::MapFold
                    | Template::SetInsert
                    | Template::SetRemove
                    | Template::SetContains
                    | Template::SetUnion
                    | Template::SetIntersection
                    | Template::SetDifference
                    | Template::SetFold
                    | Template::ListFold
                    | Template::Iterate
                    | Template::IterateUntil
                    | Template::GraphSuccessors
                    | Template::GraphReachable
                    | Template::GraphTopological => {
                        return Err("a projection of a non-projection template".to_owned());
                    }
                };
                let (kappa, nu) = (self.lean_ty(&key)?, self.lean_ty(&stored)?);
                let (ek, ev) = (self.enc(&key)?, self.enc(&stored)?);
                let list = self.list_bundle(call.operand_type(0, template)?)?;
                let projections = self.list_bundle(&SemanticType::List {
                    element: Box::new(projected),
                })?;
                let mut hints = self.call_hints(call, &["tk", "tw"]);
                hints.extend([("κ", kappa), ("ν", nu), ("ek", ek), ("ev", ev)]);
                let mut args = vec![
                    Arg::term(list),
                    Arg::refl(),
                    Arg::term(projections),
                    Arg::refl(),
                    Arg::refl(),
                ];
                args.extend(self.operand_args(call, template, 0..1)?);
                self.lemma_args(lemma, &hints, &args)
            }
            Template::MapFold => {
                let (key, stored, state) = (call.at(0)?, call.at(1)?, call.at(2)?);
                let step = *call.step(template)?;
                let list = self.list_bundle(call.operand_type(2, template)?)?;
                let mut hints = self.call_hints(call, &["tk", "tw", "ts"]);
                let types = [
                    ("κ", self.lean_ty(&key)?),
                    ("ν", self.lean_ty(&stored)?),
                    ("σ", self.lean_ty(&state)?),
                    ("ek", self.enc(&key)?),
                    ("ev", self.enc(&stored)?),
                    ("es", self.enc(&state)?),
                ];
                hints.extend(types);
                let mut args = vec![Arg::term(list), Arg::refl()];
                args.extend(Self::step_args(step));
                args.push(Arg::refl());
                args.extend(self.operand_args(call, template, 1..3)?);
                self.lemma_args("tpl_mapFold", &hints, &args)
            }
            Template::SetInsert | Template::SetRemove | Template::SetContains => {
                let key = call.at(0)?;
                let kappa = self.lean_ty(&key)?;
                let instance = self.key_instance(&call.site, &key)?;
                let (lemma, name, form): (&str, &str, fn(Term, Term) -> Form) = match template {
                    Template::SetInsert => ("tpl_setInsert", "insertElement", |kappa, function| {
                        Form::InsertElement { kappa, function }
                    }),
                    Template::SetRemove => ("tpl_setRemove", "removeElement", |kappa, function| {
                        Form::RemoveElement { kappa, function }
                    }),
                    Template::SetContains => {
                        ("tpl_setContains", "containsElement", |_, function| {
                            Form::ContainsElement { function }
                        })
                    }
                    Template::MapInsert
                    | Template::MapRemove
                    | Template::MapLookup
                    | Template::MapContains
                    | Template::MapKeys
                    | Template::MapValues
                    | Template::MapFold
                    | Template::SetUnion
                    | Template::SetIntersection
                    | Template::SetDifference
                    | Template::SetFold
                    | Template::ListFold
                    | Template::Iterate
                    | Template::IterateUntil
                    | Template::GraphSuccessors
                    | Template::GraphReachable
                    | Template::GraphTopological => {
                        return Err("an element operation of a non-element template".to_owned());
                    }
                };
                let function = self.collections_at(&call.site, name, &[kappa, instance])?;
                let form = form(kappa, function);
                let spec = self.key_spec(&call.site, &key)?;
                let list = self.list_bundle(call.operand_type(0, template)?)?;
                let hints = self.call_hints(call, &["tk"]);
                let mut args = vec![
                    Arg::proof(spec),
                    Arg::term(list),
                    Arg::refl(),
                    Arg::term(function),
                    Arg::refl(),
                    Arg::Clause(3, form),
                    Arg::Clause(3, form),
                    Arg::Clause(3, form),
                    Arg::refl(),
                ];
                args.extend(self.operand_args(call, template, 0..2)?);
                self.lemma_args(lemma, &hints, &args)
            }
            Template::SetUnion => {
                let key = call.at(0)?;
                let kappa = self.lean_ty(&key)?;
                let instance = self.key_instance(&call.site, &key)?;
                let function =
                    self.collections_at(&call.site, "insertElement", &[kappa, instance])?;
                let form = Form::InsertElement { kappa, function };
                let spec = self.key_spec(&call.site, &key)?;
                let list = self.list_bundle(call.operand_type(0, template)?)?;
                let hints = self.call_hints(call, &["tk"]);
                let mut args = vec![
                    Arg::proof(spec),
                    Arg::term(list),
                    Arg::refl(),
                    Arg::term(function),
                    Arg::refl(),
                    Arg::Clause(3, form),
                    Arg::Clause(3, form),
                    Arg::Clause(3, form),
                ];
                args.extend(Arg::refls(2));
                // The lemma folds the right operand into the left.
                args.push(Arg::term(call.value(1, template)?));
                args.push(Arg::term(call.value(0, template)?));
                self.lemma_args("tpl_setUnion", &hints, &args)
            }
            Template::SetIntersection | Template::SetDifference => {
                let key = call.at(0)?;
                let kappa = self.lean_ty(&key)?;
                let instance = self.key_instance(&call.site, &key)?;
                let function =
                    self.collections_at(&call.site, "containsElement", &[kappa, instance])?;
                let form = Form::ContainsElement { function };
                let lemma = match template {
                    Template::SetIntersection => "tpl_setIntersection",
                    Template::SetDifference => "tpl_setDifference",
                    Template::MapInsert
                    | Template::MapRemove
                    | Template::MapLookup
                    | Template::MapContains
                    | Template::MapKeys
                    | Template::MapValues
                    | Template::MapFold
                    | Template::SetInsert
                    | Template::SetRemove
                    | Template::SetContains
                    | Template::SetUnion
                    | Template::SetFold
                    | Template::ListFold
                    | Template::Iterate
                    | Template::IterateUntil
                    | Template::GraphSuccessors
                    | Template::GraphReachable
                    | Template::GraphTopological => {
                        return Err("a filter of a non-filter template".to_owned());
                    }
                };
                let spec = self.key_spec(&call.site, &key)?;
                let list = self.list_bundle(call.operand_type(0, template)?)?;
                let hints = self.call_hints(call, &["tk"]);
                let mut args = vec![
                    Arg::proof(spec),
                    Arg::term(list),
                    Arg::refl(),
                    Arg::term(function),
                    Arg::refl(),
                    Arg::Clause(3, form),
                    Arg::Clause(3, form),
                    Arg::Clause(3, form),
                ];
                args.extend(Arg::refls(2));
                args.extend(self.operand_args(call, template, 0..2)?);
                self.lemma_args(lemma, &hints, &args)
            }
            Template::SetFold | Template::ListFold => {
                let (element, state) = (call.at(0)?, call.at(1)?);
                let step = *call.step(template)?;
                let list = self.list_bundle(call.operand_type(2, template)?)?;
                let mut hints = self.call_hints(call, &["ta", "ts"]);
                let types = [
                    ("α", self.lean_ty(&element)?),
                    ("σ", self.lean_ty(&state)?),
                    ("ea", self.enc(&element)?),
                    ("es", self.enc(&state)?),
                ];
                hints.extend(types);
                let mut args = vec![Arg::term(list), Arg::refl()];
                args.extend(Self::step_args(step));
                args.push(Arg::refl());
                args.extend(self.operand_args(call, template, 1..3)?);
                self.lemma_args("tpl_listFold", &hints, &args)
            }
            Template::Iterate => {
                let state = call.at(0)?;
                let step = *call.step(template)?;
                let sigma = self.lean_ty(&state)?;
                let iterate =
                    self.collections_at(&call.site, "iterate", &[sigma, call.value(0, template)?])?;
                let mut hints = self.call_hints(call, &["ts"]);
                hints.extend([("σ", sigma), ("es", self.enc(&state)?)]);
                let mut args = Self::step_args(step);
                args.push(Arg::term(iterate));
                args.extend(Arg::refls(2));
                args.push(Arg::refl());
                args.extend(self.operand_args(call, template, 1..3)?);
                self.lemma_args("tpl_iterate", &hints, &args)
            }
            Template::IterateUntil => {
                let state = call.at(0)?;
                let step = *call.step(template)?;
                let sigma = self.lean_ty(&state)?;
                let iterate = self.collections_at(
                    &call.site,
                    "iterateUntil",
                    &[sigma, call.value(0, template)?],
                )?;
                let options = self.option_bundle(&SemanticType::Option {
                    value: Box::new(state.clone()),
                })?;
                let pairs = self.enc(&SemanticType::Product {
                    left: Box::new(state.clone()),
                    right: Box::new(SemanticType::Bool),
                })?;
                let form = Form::IterateUntil { sigma, iterate };
                let mut hints = self.call_hints(call, &["ts"]);
                hints.extend([("σ", sigma), ("es", self.enc(&state)?)]);
                let mut args = vec![
                    Arg::term(options),
                    Arg::refl(),
                    Arg::term(pairs),
                    Arg::refl(),
                ];
                args.extend(Self::step_args(step));
                args.extend([
                    Arg::term(iterate),
                    Arg::refl(),
                    Arg::Clause(2, form),
                    Arg::Clause(3, form),
                    Arg::refl(),
                ]);
                args.extend(self.operand_args(call, template, 1..3)?);
                self.lemma_args("tpl_iterateUntil", &hints, &args)
            }
            Template::GraphSuccessors => {
                let node = call.at(0)?;
                let kappa = self.lean_ty(&node)?;
                let successors_ty = SemanticType::List {
                    element: Box::new(node.clone()),
                };
                let nodes = self.list_bundle(&successors_ty)?;
                let graph = self.list_bundle(call.operand_type(0, template)?)?;
                let instance = self.key_instance(&call.site, &node)?;
                let list = self.b.list_ty(kappa);
                let look =
                    self.collections_at(&call.site, "lookupEntry", &[kappa, list, instance])?;
                let form = Form::LookupEntry {
                    nu: list,
                    function: look,
                };
                let spec = self.key_spec(&call.site, &node)?;
                let hints = self.call_hints(call, &["tk"]);
                let mut args = vec![
                    Arg::proof(spec),
                    Arg::term(nodes),
                    Arg::term(graph),
                    Arg::refl(),
                    Arg::term(look),
                    Arg::refl(),
                    Arg::Clause(4, form),
                    Arg::Clause(4, form),
                    Arg::Clause(4, form),
                ];
                args.extend(Arg::refls(2));
                args.extend(self.operand_args(call, template, 0..2)?);
                self.lemma_args("tpl_graphSuccessors", &hints, &args)
            }
            Template::GraphReachable | Template::GraphTopological => {
                let node = call.at(0)?;
                let kappa = self.lean_ty(&node)?;
                let nodes_ty = SemanticType::List {
                    element: Box::new(node.clone()),
                };
                let graph = call.value(0, template)?;
                let instance = self.key_instance(&call.site, &node)?;
                let spec = self.key_spec(&call.site, &node)?;
                let nodes = self.list_bundle(&nodes_ty)?;
                let graph_bundle = self.list_bundle(call.operand_type(0, template)?)?;
                let successors =
                    self.collections_at(&call.site, "graphSuccessors", &[kappa, instance, graph])?;
                let sets = self.set_ops(&call.site, &node)?;
                let lookups = self.look_ops(&call.site, &node, &nodes_ty)?;
                let hints = self.call_hints(call, &["tk"]);
                let mut args = vec![
                    Arg::proof(spec),
                    Arg::term(nodes),
                    Arg::refl(),
                    Arg::term(graph_bundle),
                    Arg::refl(),
                ];
                match template {
                    Template::GraphReachable => {
                        let insert =
                            self.collections_at(&call.site, "insertElement", &[kappa, instance])?;
                        let reach = self.collections_at(
                            &call.site,
                            "reachableFrom",
                            &[kappa, instance, graph],
                        )?;
                        let matcher = self.reach_matcher(&call.site)?;
                        let form = Form::Reach {
                            matcher,
                            kappa,
                            reach,
                            insert,
                        };
                        args.extend([
                            Arg::proof(sets),
                            Arg::proof(lookups),
                            Arg::term(graph),
                            Arg::term(successors),
                            Arg::refl(),
                            Arg::term(reach),
                            Arg::refl(),
                            Arg::Clause(3, form),
                            Arg::Clause(5, form),
                        ]);
                        args.extend(Arg::refls(13));
                        args.push(Arg::term(call.value(1, template)?));
                        self.lemma_args("tpl_graphReachable", &hints, &args)
                    }
                    Template::GraphTopological => {
                        let options = self.option_bundle(&SemanticType::Option {
                            value: Box::new(nodes_ty.clone()),
                        })?;
                        let topo = self.topo_ops(&call.site, &node, graph)?;
                        args.extend([
                            Arg::term(options),
                            Arg::refl(),
                            Arg::proof(sets),
                            Arg::proof(lookups),
                            Arg::term(graph),
                            Arg::term(successors),
                            Arg::refl(),
                            Arg::proof(topo),
                        ]);
                        args.extend(Arg::refls(12));
                        self.lemma_args("tpl_graphTopological", &hints, &args)
                    }
                    Template::MapInsert
                    | Template::MapRemove
                    | Template::MapLookup
                    | Template::MapContains
                    | Template::MapKeys
                    | Template::MapValues
                    | Template::MapFold
                    | Template::SetInsert
                    | Template::SetRemove
                    | Template::SetContains
                    | Template::SetUnion
                    | Template::SetIntersection
                    | Template::SetDifference
                    | Template::SetFold
                    | Template::ListFold
                    | Template::Iterate
                    | Template::IterateUntil
                    | Template::GraphSuccessors => {
                        Err("a traversal of a non-traversal template".to_owned())
                    }
                }
            }
        }
    }

    /// The operands at `range` as the lemma's final explicit arguments.
    fn operand_args(
        &self,
        call: &Call,
        template: Template,
        range: std::ops::Range<usize>,
    ) -> Result<Vec<Arg>, String> {
        range
            .map(|index| call.value(index, template).map(Arg::term))
            .collect()
    }

    /// The step of a fold or iteration as the lemma takes it: the function,
    /// its width predicate, closure index, captured values, and relation.
    fn step_args(step: FunctionArgument) -> Vec<Arg> {
        vec![
            Arg::term(step.value),
            Arg::term(step.fits),
            Arg::term(step.index),
            Arg::term(step.captures),
            Arg::term(step.relation),
        ]
    }

    /// A key comparison.
    pub(super) fn less_than(
        &mut self,
        types: &[SemanticType],
        arguments: &[SemanticTerm],
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        let key = types.first().ok_or("a key comparison has no operand")?;
        let site = scope.ctx.site.clone();
        // The lowering is a match on the realization's comparison of the
        // operands (§17.14 `primitive.less_than`).
        let Expr::Match { scrutinee, .. } = expr else {
            return Err("the lowering of a key comparison is not a match".to_owned());
        };
        let Expr::Prim { operands, .. } = scrutinee.as_ref() else {
            return Err("the lowering of a comparison does not compare".to_owned());
        };
        if operands.len() != arguments.len() {
            return Err("a key comparison lowered to a comparison of other operands".to_owned());
        }
        let operand_exprs = operands;
        let mut proofs = Vec::new();
        for (argument, child) in arguments.iter().zip(operand_exprs) {
            proofs.push(self.prove(argument, child, scope)?);
        }
        let list = self.operands(scope.env, &proofs)?;
        let kappa = self.lean_ty(key)?;
        let instance = self.key_instance(&site, key)?;
        let less = self.collections_at(&site, "lessThan", &[kappa, instance])?;
        let spec = self.key_spec(&site, key)?;
        let matcher = self.less_matcher(&site)?;
        let clause = Arg::Clause(2, Form::LessThan { matcher });
        // The operands appear in the lemma under the key encoder, which
        // syntactic matching does not look through: they are given.
        let [left, right] = arguments else {
            return Err("a key comparison takes two operands".to_owned());
        };
        let (left, right) = (self.src(left, scope)?, self.src(right, scope)?);
        let (p, env) = (self.prog, scope.env);
        self.lemma_args(
            "conv_lessThan",
            &[("p", p), ("env", env), ("a", left), ("b", right)],
            &[
                Arg::proof(spec),
                Arg::term(less),
                clause,
                clause,
                clause,
                Arg::proof(list),
            ],
        )
    }

    /// A collection primitive's runtime call on operands of the given types,
    /// as the module elaborates `(LexLeanCollections.op args : result)`: the
    /// type arguments from the operands' types, and the `Key` instance of the
    /// key type for an operation that orders.
    pub(super) fn collection_call(
        &mut self,
        site: &Site,
        operation: SemanticPrimitive,
        types: &[SemanticType],
        args: &[Term],
    ) -> Result<Term, String> {
        let operand = |index: usize| -> Result<&SemanticType, String> {
            types
                .get(index)
                .ok_or_else(|| format!("{operation:?} lacks operand {index}"))
        };
        let map = |ty: &SemanticType| -> Result<(SemanticType, SemanticType), String> {
            match ty {
                SemanticType::Map { key, value } => {
                    Ok((key.as_ref().clone(), value.as_ref().clone()))
                }
                SemanticType::Type
                | SemanticType::Parameter { name: _ }
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
                | SemanticType::Option { value: _ }
                | SemanticType::Result { ok: _, error: _ }
                | SemanticType::List { element: _ }
                | SemanticType::Named {
                    member: _,
                    arguments: _,
                }
                | SemanticType::Product { left: _, right: _ }
                | SemanticType::Function {
                    parameters: _,
                    result: _,
                }
                | SemanticType::Set { element: _ }
                | SemanticType::ContractViolation
                | SemanticType::ReasoningFailure => Err(format!("{operation:?} of a non-map")),
            }
        };
        let (name, leading) = match operation {
            SemanticPrimitive::ToInt | SemanticPrimitive::ToNat => {
                return Err(format!(
                    "{operation:?} is certified only through its elaboration"
                ));
            }
            SemanticPrimitive::MapInsert
            | SemanticPrimitive::MapRemove
            | SemanticPrimitive::MapLookup
            | SemanticPrimitive::MapContains => {
                let (key, value) = map(operand(0)?)?;
                let instance = self.key_instance(site, &key)?;
                (
                    collection_name(operation)?,
                    vec![self.lean_ty(&key)?, self.lean_ty(&value)?, instance],
                )
            }
            SemanticPrimitive::MapSize
            | SemanticPrimitive::MapKeys
            | SemanticPrimitive::MapValues
            | SemanticPrimitive::MapEntries => {
                let (key, value) = map(operand(0)?)?;
                (
                    collection_name(operation)?,
                    vec![self.lean_ty(&key)?, self.lean_ty(&value)?],
                )
            }
            SemanticPrimitive::MapFold => {
                let (key, value) = map(operand(2)?)?;
                (
                    collection_name(operation)?,
                    vec![
                        self.lean_ty(&key)?,
                        self.lean_ty(&value)?,
                        self.lean_ty(operand(1)?)?,
                    ],
                )
            }
            SemanticPrimitive::SetInsert
            | SemanticPrimitive::SetRemove
            | SemanticPrimitive::SetContains
            | SemanticPrimitive::SetUnion
            | SemanticPrimitive::SetIntersection
            | SemanticPrimitive::SetDifference => {
                let element = element_of(operand(0)?)?;
                let instance = self.key_instance(site, &element)?;
                (
                    collection_name(operation)?,
                    vec![self.lean_ty(&element)?, instance],
                )
            }
            SemanticPrimitive::SetSize | SemanticPrimitive::SetElements => {
                let element = element_of(operand(0)?)?;
                (collection_name(operation)?, vec![self.lean_ty(&element)?])
            }
            SemanticPrimitive::SetFold | SemanticPrimitive::ListFold => {
                let element = element_of(operand(2)?)?;
                (
                    collection_name(operation)?,
                    vec![self.lean_ty(&element)?, self.lean_ty(operand(1)?)?],
                )
            }
            SemanticPrimitive::Iterate | SemanticPrimitive::IterateUntil => (
                collection_name(operation)?,
                vec![self.lean_ty(operand(2)?)?],
            ),
            SemanticPrimitive::GraphSuccessors
            | SemanticPrimitive::GraphReachable
            | SemanticPrimitive::GraphTopological => {
                let (key, _) = map(operand(0)?)?;
                let instance = self.key_instance(site, &key)?;
                (
                    collection_name(operation)?,
                    vec![self.lean_ty(&key)?, instance],
                )
            }
            SemanticPrimitive::LessThan => {
                let key = operand(0)?;
                let instance = self.key_instance(site, key)?;
                (
                    collection_name(operation)?,
                    vec![self.lean_ty(key)?, instance],
                )
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
            | SemanticPrimitive::FormatDecimal => {
                return Err(format!("{operation:?} is not a collection operation"));
            }
        };
        let mut all = leading;
        all.extend_from_slice(args);
        self.collections_at(site, name, &all)
    }
}

/// The name of a collection primitive's runtime function.
fn collection_name(operation: SemanticPrimitive) -> Result<&'static str, String> {
    let (namespace, name) = super::super::primitive_name(operation)?;
    if namespace == "LexLeanCollections" {
        Ok(name)
    } else {
        Err(format!("{operation:?} is not in the collections runtime"))
    }
}
