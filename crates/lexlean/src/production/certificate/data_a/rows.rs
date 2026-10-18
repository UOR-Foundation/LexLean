//! The rows of certificate A: a width predicate and a relation theorem per
//! function of the closure, then `denote` and `root` (SPEC.md §17.17.4).

use std::collections::BTreeMap;

use super::super::{function_type, identifier, CLocal, Ctx, Hypotheses, Recursion, Subject};
use super::{CompTerms, LocalTerm, Pf, RowKind, Scope, Writer, EXPLICIT};
use crate::calculus::Expr;
use crate::ir::semantic::{MemberRef, SemanticParameter, SemanticType};
use crate::production::kernel::{Given, Term};
use crate::production::source::{Local, Site};

/// A binder of a row: its name, type, and the free variable standing for it.
#[derive(Clone)]
pub(super) struct Binder {
    pub(super) name: String,
    pub(super) ty: Term,
    pub(super) free: Term,
}

/// The binders, arguments, and statement of one function's relation.
#[derive(Clone)]
pub(super) struct Sig {
    pub(super) fits_binders: Vec<Binder>,
    pub(super) rel_binders: Vec<Binder>,
    /// The width predicate applied to its binders.
    pub(super) fits_applied: Term,
    /// The encoded argument list the function is called with.
    pub(super) arguments: Term,
    /// The encoded denotation of the call.
    pub(super) value: Term,
    pub(super) rel_params: Vec<Term>,
    /// `FunRel __prog index arguments (Rel fits value)`.
    pub(super) statement: Term,
}

/// The terms of the locals of a function: a free variable for each, and the
/// companions of a function-typed one, with the binders they make.
pub(super) struct LocalSet {
    pub(super) terms: Vec<LocalTerm>,
    pub(super) fits: Vec<Binder>,
    pub(super) rel: Vec<Binder>,
}

impl<'a> Writer<'_, 'a> {
    /// `List.nil (Prod Nat Value)`: the environment of no bindings.
    pub(super) fn empty_env(&mut self) -> Term {
        let nat = self.b.konst("Nat");
        let value = self.value_ty();
        let entry = self.b.pair_ty(nat, value);
        let nil = self.b.konst0("List.nil");
        self.b.app(nil, entry)
    }

    /// Free variables for `locals`, with the binders of the width predicate
    /// and of the relation they make: a function-typed local brings the width
    /// predicate, index, captured values, and relation hypothesis of the
    /// closure it is bound to.
    pub(super) fn local_set(&mut self, index: u64, locals: &[CLocal]) -> Result<LocalSet, String> {
        let mut terms = Vec::new();
        let mut fits = Vec::new();
        let mut rel = Vec::new();
        for local in locals {
            let name = identifier(&local.local.name);
            let ty = self.lean_ty(&local.local.ty)?;
            let value = self.b.free();
            let binder = Binder {
                name: name.clone(),
                ty,
                free: value,
            };
            fits.push(binder.clone());
            rel.push(binder);
            let companion = match function_type(&local.local.ty) {
                None => None,
                Some((parameters, codomain)) => {
                    let (parameters, codomain) = (parameters.to_vec(), codomain.clone());
                    let mut domain = Vec::new();
                    for parameter in &parameters {
                        domain.push(self.lean_ty(parameter)?);
                    }
                    let bool = self.b.konst("Bool");
                    let mut fits_ty = bool;
                    for ty in domain.iter().rev() {
                        fits_ty = self.b.arrow(*ty, fits_ty);
                    }
                    let ff = self.b.free();
                    let fi = self.b.free();
                    let fc = self.b.free();
                    let fh = self.b.free();
                    let nat = self.b.konst("Nat");
                    let value_ty = self.value_ty();
                    let captures_ty = self.b.list_ty(value_ty);
                    // ∀ p…, FunRel __prog fi (captures ++ [enc p…]) (Rel (ff p…) (enc cod (x p…)))
                    let ps: Vec<Term> = domain.iter().map(|_| self.b.free()).collect();
                    let mut encoded = Vec::new();
                    for (parameter, p) in parameters.iter().zip(&ps) {
                        let enc = self.enc(parameter)?;
                        encoded.push(self.b.app(enc, *p));
                    }
                    let list = self.b.list(value_ty, &encoded);
                    let append = self.b.konst("LexLeanRuntime.append");
                    let instance = self.b.konst("LexLeanRuntime.instAppendableList");
                    let instance = self.b.app(instance, value_ty);
                    let arguments = self.b.apps(append, &[captures_ty, instance, fc, list]);
                    let fits_applied = self.b.apps(ff, &ps);
                    let applied = self.b.apps(value, &ps);
                    let codomain_enc = self.enc(&codomain)?;
                    let result = self.b.app(codomain_enc, applied);
                    let rel_value = self.rel(fits_applied, result);
                    let fun_rel = self.lp("FunRel");
                    let prog = self.prog;
                    let mut relation_ty = self.b.apps(fun_rel, &[prog, fi, arguments, rel_value]);
                    for (position, p) in ps.iter().enumerate().rev() {
                        relation_ty = self.b.pi_over(
                            &format!("__p{position}"),
                            EXPLICIT,
                            domain[position],
                            *p,
                            relation_ty,
                        );
                    }
                    let _ = index;
                    fits.push(Binder {
                        name: "__ff".to_owned(),
                        ty: fits_ty,
                        free: ff,
                    });
                    rel.push(Binder {
                        name: "__ff".to_owned(),
                        ty: fits_ty,
                        free: ff,
                    });
                    rel.push(Binder {
                        name: "__fi".to_owned(),
                        ty: nat,
                        free: fi,
                    });
                    rel.push(Binder {
                        name: "__fc".to_owned(),
                        ty: captures_ty,
                        free: fc,
                    });
                    rel.push(Binder {
                        name: "__fh".to_owned(),
                        ty: relation_ty,
                        free: fh,
                    });
                    Some(CompTerms {
                        relation_ty,
                        fits: ff,
                        index: fi,
                        captures: fc,
                        relation: fh,
                    })
                }
            };
            terms.push(LocalTerm { value, companion });
        }
        Ok(LocalSet { terms, fits, rel })
    }

    /// The evaluation environment of a function run on its locals: its
    /// parameters bound to the locals' encodings, as `bindAll` binds them.
    pub(super) fn entry_env(
        &mut self,
        index: u64,
        locals: &[CLocal],
        terms: &[LocalTerm],
    ) -> Result<Term, String> {
        let parameters = self
            .gen
            .program
            .functions
            .get(index as usize)
            .ok_or_else(|| format!("the program has no function {index}"))?
            .parameters
            .clone();
        if parameters.len() != locals.len() {
            return Err(format!(
                "function {index} takes {} parameters, the certificate binds {}",
                parameters.len(),
                locals.len()
            ));
        }
        let mut values = Vec::new();
        for (local, term) in locals.iter().zip(terms) {
            values.push(self.encoded_local(local, *term)?);
        }
        let empty = self.empty_env();
        self.extend_env(empty, &parameters, &values)
    }

    /// The signature of function `index` over `locals`, whose denotation
    /// applied to them is `denotation`.
    pub(super) fn signature(
        &mut self,
        index: u64,
        locals: &[CLocal],
        set: &LocalSet,
        result: &SemanticType,
        denotation: Term,
    ) -> Result<Sig, String> {
        let mut fits_args = Vec::new();
        let mut arguments = Vec::new();
        for ((local, terms), _) in locals.iter().zip(&set.terms).zip(0..) {
            fits_args.push(terms.value);
            if let Some(companion) = terms.companion {
                fits_args.push(companion.fits);
            }
            arguments.push(self.encoded_local(local, *terms)?);
        }
        let fits_head = self.own(&format!("__fits_{index}"));
        let fits_applied = self.b.apps(fits_head, &fits_args);
        let value_ty = self.value_ty();
        let arguments = self.b.list(value_ty, &arguments);
        let encoder = self.enc(result)?;
        let value = self.b.app(encoder, denotation);
        let rel_value = self.rel(fits_applied, value);
        let fun_rel = self.lp("FunRel");
        let prog = self.prog;
        let i = self.nat_u(index);
        let statement = self.b.apps(fun_rel, &[prog, i, arguments, rel_value]);
        Ok(Sig {
            fits_binders: set.fits.clone(),
            rel_binders: set.rel.clone(),
            fits_applied,
            arguments,
            value,
            rel_params: set.rel.iter().map(|binder| binder.free).collect(),
            statement,
        })
    }

    /// `fun binders => body`.
    pub(super) fn lambdas(&mut self, binders: &[Binder], body: Term) -> Term {
        let mut out = body;
        for binder in binders.iter().rev() {
            out = self
                .b
                .lam_over(&binder.name, EXPLICIT, binder.ty, binder.free, out);
        }
        out
    }

    /// `(binders) → body`.
    pub(super) fn pis(&mut self, binders: &[Binder], body: Term) -> Term {
        let mut out = body;
        for binder in binders.iter().rev() {
            out = self
                .b
                .pi_over(&binder.name, EXPLICIT, binder.ty, binder.free, out);
        }
        out
    }

    /// `funRel_intro rfl rfl body`: the relation of function `index`, whose
    /// body is derived by `body` in the environment `env`.
    pub(super) fn relation_proof(
        &mut self,
        index: u64,
        sig: &Sig,
        env: Term,
        body: Pf,
    ) -> Result<Pf, String> {
        let program = self.gen.program;
        let function = program
            .functions
            .get(index as usize)
            .ok_or_else(|| format!("the program has no function {index}"))?;
        let function = self.function_term(function)?;
        let (_, parts) = self.b.spine(sig.statement);
        let observation = parts
            .get(3)
            .copied()
            .ok_or("a statement with no observation")?;
        let p = self.prog;
        let fi = self.nat_u(index);
        self.apply(
            "funRel_intro",
            &[
                ("p", p),
                ("fi", fi),
                ("args", sig.arguments),
                ("o", observation),
                ("fn", function),
                ("env", env),
            ],
            &[Given::Refl, Given::Refl, Given::Proof(body.proof, body.ty)],
        )
    }

    /// The width predicate row and relation row of function `index`.
    pub(super) fn function_rows(
        &mut self,
        index: u64,
        sig: &Sig,
        body: Pf,
        env: Term,
    ) -> Result<(), String> {
        let bool = self.b.konst("Bool");
        let fits = self.fits_of(body)?;
        let fits_value = self.lambdas(&sig.fits_binders, fits);
        let fits_ty = self.pis(&sig.fits_binders, bool);
        self.push_row(
            &format!("__fits_{index}"),
            RowKind::Definition { compiled: true },
            fits_ty,
            fits_value,
        )?;
        let proof = self.relation_proof(index, sig, env, body)?;
        let rel_value = self.lambdas(&sig.rel_binders, proof.proof);
        let rel_ty = self.pis(&sig.rel_binders, sig.statement);
        self.push_row(
            &format!("__rel_{index}"),
            RowKind::Theorem,
            rel_ty,
            rel_value,
        )
    }

    /// One strongly connected component of the relation graph.
    pub(super) fn component(
        &mut self,
        component: &[u64],
        graph: &BTreeMap<u64, std::collections::BTreeSet<u64>>,
    ) -> Result<(), String> {
        let recursive = component.len() > 1
            || graph
                .get(&component[0])
                .is_some_and(|edges| edges.contains(&component[0]));
        let structural = component.iter().any(|index| {
            matches!(
                self.gen.subjects.get(index),
                Some(Subject::Definition {
                    recursion: Recursion::Structural { .. },
                    ..
                })
            )
        });
        if structural {
            return self.structural_component(component);
        }
        for index in component {
            let subject = self.gen.subject_of(*index)?;
            match subject {
                Subject::Definition {
                    module,
                    name,
                    type_arguments,
                    locals,
                    result,
                    body,
                    recursion,
                    site,
                    type_parameters,
                } => {
                    let clocals = Gen::scope(&locals);
                    match recursion {
                        Recursion::None => {
                            if recursive {
                                return Err(format!(
                                    "`{module}.{name}` recurses without recursion evidence"
                                ));
                            }
                            let set = self.local_set(*index, &clocals)?;
                            let denotation =
                                self.denotation(&module, &name, &type_arguments, &set)?;
                            let sig =
                                self.signature(*index, &clocals, &set, &result, denotation)?;
                            self.sigs.insert(*index, sig.clone());
                            let env = self.entry_env(*index, &clocals, &set.terms)?;
                            let scope = Scope {
                                ctx: Ctx::new(
                                    site,
                                    type_parameters,
                                    clocals,
                                    Hypotheses::new(),
                                    *index,
                                ),
                                terms: set.terms.clone(),
                                env,
                                path: Vec::new(),
                            };
                            let program = self.gen.program;
                            let body_expr = &program.functions[*index as usize].body;
                            let pf = self.prove(body, body_expr, &scope)?;
                            self.function_rows(*index, &sig, pf, env)?;
                        }
                        Recursion::WellFounded { termination, group } => {
                            self.wf_definition(
                                *index,
                                &module,
                                &name,
                                &type_arguments,
                                &locals,
                                &result,
                                body,
                                &termination,
                                &group,
                                site,
                                type_parameters,
                                recursive,
                            )?;
                        }
                        Recursion::Structural { .. } => {
                            return Err(format!(
                                "`{module}.{name}` is structurally recursive outside a structural component"
                            ));
                        }
                    }
                }
                Subject::Instance {
                    module,
                    name,
                    class,
                    fields,
                } => {
                    if recursive {
                        return Err(format!("instance `{module}.{name}` is recursive"));
                    }
                    let set = self.local_set(*index, &[])?;
                    let lean_name = self.gen.source.lean_name(
                        &module,
                        &MemberRef {
                            module: Some(module.clone()),
                            name: name.clone(),
                        },
                    );
                    let denotation = self.b.konst(&lean_name);
                    let sig = self.signature(*index, &[], &set, &class, denotation)?;
                    self.sigs.insert(*index, sig.clone());
                    let env = self.empty_env();
                    let scope = Scope {
                        ctx: Ctx::new(
                            Site {
                                module: module.clone(),
                                substitution: BTreeMap::new(),
                            },
                            Vec::new(),
                            Vec::new(),
                            Hypotheses::new(),
                            *index,
                        ),
                        terms: Vec::new(),
                        env,
                        path: Vec::new(),
                    };
                    let program = self.gen.program;
                    let body_expr = &program.functions[*index as usize].body;
                    let pf = self.record(&class, fields, body_expr, &scope)?;
                    self.function_rows(*index, &sig, pf, env)?;
                }
                Subject::Lambda {
                    site: _,
                    captures: _,
                    parameters: _,
                    body,
                    hypotheses: _,
                    type_parameters: _,
                } => {
                    if recursive {
                        return Err(format!("lambda function {index} is recursive"));
                    }
                    let ctx = self.gen.lambda_ctx(*index)?;
                    let clocals = ctx.locals.clone();
                    let set = self.local_set(*index, &clocals)?;
                    let result = self.gen.source.infer(body, &ctx.sources(), &ctx.site)?;
                    let env = self.entry_env(*index, &clocals, &set.terms)?;
                    let scope = Scope {
                        ctx,
                        terms: set.terms.clone(),
                        env,
                        path: Vec::new(),
                    };
                    let denotation = self.src(body, &scope)?;
                    let sig = self.signature(*index, &clocals, &set, &result, denotation)?;
                    self.sigs.insert(*index, sig.clone());
                    let program = self.gen.program;
                    let body_expr = &program.functions[*index as usize].body;
                    let pf = self.prove(body, body_expr, &scope)?;
                    self.function_rows(*index, &sig, pf, env)?;
                }
            }
        }
        Ok(())
    }

    /// The denotation of a definition instance applied to its locals.
    pub(super) fn denotation(
        &mut self,
        module: &str,
        name: &str,
        type_arguments: &[SemanticType],
        set: &LocalSet,
    ) -> Result<Term, String> {
        let lean_name = self.gen.source.lean_name(
            module,
            &MemberRef {
                module: Some(module.to_owned()),
                name: name.to_owned(),
            },
        );
        let head = self.b.konst(&lean_name);
        let mut args = Vec::new();
        for argument in type_arguments {
            args.push(self.lean_ty(argument)?);
        }
        args.extend(set.terms.iter().map(|terms| terms.value));
        Ok(self.b.apps(head, &args))
    }

    /// `denote`, `root`, and the boundary.
    pub(super) fn root(
        &mut self,
        root_module: &str,
        root_name: &str,
        report: &super::super::RootReport,
    ) -> Result<(), String> {
        let root_lean = self.gen.source.lean_name(
            root_module,
            &MemberRef {
                module: Some(root_module.to_owned()),
                name: root_name.to_owned(),
            },
        );
        if root_lean != report.root {
            return Err(format!(
                "the certificate root `{root_lean}` is not the report's `{}`",
                report.root
            ));
        }
        let (parameters, result) = self.gen.source.signature(root_module, root_name, &[])?;
        let locals: Vec<(Local, SemanticType)> = parameters
            .into_iter()
            .map(|SemanticParameter { name, r#type }| {
                (
                    Local {
                        name,
                        ty: r#type.clone(),
                    },
                    r#type,
                )
            })
            .collect();
        let clocals = Gen::scope(&locals);
        let set = self.local_set(0, &clocals)?;
        let head = self.b.konst(&root_lean);
        let values: Vec<Term> = set.terms.iter().map(|terms| terms.value).collect();
        let denotation = self.b.apps(head, &values);
        let sig = self.signature(0, &clocals, &set, &result, denotation)?;
        // The observation as a definition, so the differential evaluator can
        // compute the right-hand side the theorem states: `Rel` unfolded to the
        // lazy `cond`.
        let obs = self.lp("Obs");
        let wrap = self.lp("Obs.value");
        let overflow = self.lp("Obs.overflow");
        let valued = self.b.app(wrap, sig.value);
        let cond = self.b.konst_at(
            "cond",
            vec![crate::ir::core::CoreLevel::Succ(Box::new(
                crate::ir::core::CoreLevel::Zero,
            ))],
        );
        let observation = self
            .b
            .apps(cond, &[obs, sig.fits_applied, valued, overflow]);
        let denote_value = self.lambdas(&sig.fits_binders, observation);
        let denote_ty = self.pis(&sig.fits_binders, obs);
        self.push_row(
            "denote",
            RowKind::Definition { compiled: true },
            denote_ty,
            denote_value,
        )?;
        let rel0 = self.call_relation_at(0, &sig)?;
        let run_conv = self.lp("RunConv");
        let prog = self.prog;
        let entry = self.nat_u(0);
        let rel = self.rel(sig.fits_applied, sig.value);
        let root_ty = self.b.apps(run_conv, &[prog, entry, sig.arguments, rel]);
        let root = self.apply("run_of_funRel", &[], &[Given::Proof(rel0.proof, rel0.ty)])?;
        let root_value = self.lambdas(&sig.rel_binders, root.proof);
        let root_pi = self.pis(&sig.rel_binders, root_ty);
        self.push_row("root", RowKind::Theorem, root_pi, root_value)?;
        let parameter_types: Vec<SemanticType> = locals.iter().map(|(_, ty)| ty.clone()).collect();
        self.boundary(&sig, &parameter_types)
    }

    /// The relation of function `index` at its own binders.
    fn call_relation_at(&mut self, index: u64, sig: &Sig) -> Result<Pf, String> {
        let head = self.own(&format!("__rel_{index}"));
        let proof = self.b.apps(head, &sig.rel_params);
        Ok(Pf {
            proof,
            ty: sig.statement,
        })
    }

    /// A record (instance or document value) of closed type `ty`.
    pub(super) fn record(
        &mut self,
        ty: &SemanticType,
        fields: &[crate::ir::semantic::SemanticAssignment],
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        self.record_proof(ty, fields, expr, scope)
    }
}

use super::super::Gen;
