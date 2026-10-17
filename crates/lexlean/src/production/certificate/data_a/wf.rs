//! Well-founded recursion (SPEC.md §17.12, §17.17.4): a definition whose
//! recursive calls the source justifies with a measure and decrease evidence.
//!
//! Its width predicate and relation theorem are each `WellFounded.fix` over
//! the measure `InvImage`-ordered on the packed parameters, a recursive call
//! being the induction hypothesis applied to the packed arguments and the
//! source's evidence that they decrease the measure. The evidence quantifies
//! the arms enclosing the call, so every `if` and `match` of the body is
//! written by cases on its scrutinee with the equation the arm was taken by
//! (the elaborator's `match h : e with`), and the call reads that equation
//! from the free variable that stands for it.
//!
//! A recursive call is first written as a marker applied to its arguments and
//! its evidence, and the marker becomes what each use of the derivation needs
//! when the rows are built: the induction hypothesis in the width predicate
//! and the relation theorem, the predicate itself where a type states it,
//! and the implementation record's direct recursion.

use super::structural::pack_values_with;
use super::{PathStep, Pf, RowKind, Scope, WfSite, WfState, Writer, EXPLICIT};
use crate::ir::core::CoreLevel;
use crate::ir::semantic::{
    SemanticParameter, SemanticTerm, SemanticTermination, SemanticType, Step,
};
use crate::production::certificate::{CLocal, Ctx, Gen, Hypotheses};
use crate::production::kernel::{Builder, Term};
use crate::production::source::{Local, Site};
use std::collections::BTreeSet;

fn one() -> CoreLevel {
    CoreLevel::Succ(Box::new(CoreLevel::Zero))
}

/// Whether two steps are the same arm of the same node.
fn same_step(a: Step, b: Step) -> bool {
    match (a, b) {
        (
            Step::If {
                node: left,
                polarity: first,
            },
            Step::If {
                node: right,
                polarity: second,
            },
        ) => left == right && first == second,
        (
            Step::Match {
                node: left,
                branch: first,
            },
            Step::Match {
                node: right,
                branch: second,
            },
        ) => left == right && first == second,
        (
            Step::If {
                node: _,
                polarity: _,
            },
            Step::Match { node: _, branch: _ },
        )
        | (
            Step::Match { node: _, branch: _ },
            Step::If {
                node: _,
                polarity: _,
            },
        ) => false,
    }
}

/// The hypotheses a body's recursive calls are justified under.
impl<'a> Writer<'_, 'a> {
    /// The number of the `if` or `match` `term` is, when it is a node of the
    /// well-founded body under derivation.
    pub(super) fn node_of(&self, scope: &Scope, term: &SemanticTerm) -> Option<usize> {
        scope
            .ctx
            .hypotheses
            .get(&(std::ptr::from_ref(term) as usize))
            .copied()
    }

    /// `scope` inside the arm `step`, taken by the equation `hypothesis`.
    pub(super) fn entering(scope: &Scope, step: Step, hypothesis: Term, fields: &[Term]) -> Scope {
        let mut out = scope.clone();
        out.path.push(PathStep {
            step,
            hypothesis,
            fields: fields.to_vec(),
        });
        out
    }

    /// `fun fields (hypothesis : ty) => body`: a minor premise of a cases
    /// that carries the equation its arm was taken by.
    pub(super) fn arm_lambda(
        &mut self,
        binders: &[(String, Term, Term)],
        hypothesis: (Term, Term),
        body: Term,
    ) -> Term {
        let (free, ty) = hypothesis;
        let mut out = self.b.lam_over("__h", EXPLICIT, ty, free, body);
        for (name, ty, free) in binders.iter().rev() {
            out = self.b.lam_over(name, EXPLICIT, *ty, *free, out);
        }
        out
    }

    /// `match h : scrutinee with …`: the eliminator of the scrutinee's type
    /// at the motive `fun a => scrutinee = a → result`, applied to the
    /// minors (by constructor position, from [`Self::arm_lambda`]) and to
    /// `rfl`.
    pub(super) fn equation_cases(
        &mut self,
        ty: &SemanticType,
        scrutinee: Term,
        result: Term,
        level: CoreLevel,
        mut minors: Vec<(usize, Term)>,
    ) -> Result<Term, String> {
        minors.sort_by_key(|(position, _)| *position);
        let minors: Vec<Term> = minors.into_iter().map(|(_, term)| term).collect();
        let carrier = self.lean_ty(ty)?;
        let bound = self.b.free();
        let equation = self.eq(carrier, scrutinee, bound);
        let arrow = self.b.arrow(equation, result);
        let motive = self.b.lam_over("__a", EXPLICIT, carrier, bound, arrow);
        let cases = self.cases_on(ty, scrutinee, motive, level, &minors)?;
        let refl = self.b.konst1("Eq.refl");
        let proof = self.b.apps(refl, &[carrier, scrutinee]);
        Ok(self.b.app(cases, proof))
    }

    /// The relation of a recursive call of the well-founded definition under
    /// derivation: a marker for the theorem at its arguments and the
    /// evidence that they decrease the measure, whose statement writes the
    /// width predicate as the matching marker.
    pub(super) fn wf_call(
        &mut self,
        index: u64,
        arguments: &[SemanticTerm],
        relation_args: &[Term],
        scope: &Scope,
    ) -> Result<Pf, String> {
        let state = self
            .wf
            .clone()
            .ok_or("a recursive call outside a well-founded definition")?;
        let path: Vec<Step> = scope.path.iter().map(|step| step.step).collect();
        let site = state
            .sites
            .iter()
            .find(|site| {
                site.arguments.as_slice() == arguments
                    && site.path.len() == path.len()
                    && site
                        .path
                        .iter()
                        .zip(&path)
                        .all(|(left, right)| same_step(*left, *right))
            })
            .ok_or("a recursive call with no decrease obligation in the plan")?;
        let lemma = self.member_const(&site.evidence, &scope.ctx.site)?;
        let mut evidence_args = state.type_arguments.clone();
        evidence_args.extend(state.parameters.iter().copied());
        for step in &scope.path {
            evidence_args.extend(step.fields.iter().copied());
        }
        for step in &scope.path {
            evidence_args.push(step.hypothesis);
        }
        let evidence = self.b.apps(lemma, &evidence_args);
        let plain = self.call_relation(index, relation_args)?;
        let fits_name = format!("{}.__fits_{index}", self.name);
        let call_name = format!("{}.__wfcall_{index}", self.name);
        let ty = self.b.rewrite_apps(
            plain.ty,
            &mut |b: &mut Builder, name: &str, args: &[Term]| {
                if name != fits_name {
                    return None;
                }
                let head = b.konst(&call_name);
                let mut all = args.to_vec();
                all.push(evidence);
                Some(b.apps(head, &all))
            },
        );
        let proof_head = self.own(&format!("__wfrel_{index}"));
        let mut all = relation_args.to_vec();
        all.push(evidence);
        let proof = self.b.apps(proof_head, &all);
        Ok(Pf { proof, ty })
    }

    /// `FunRel __prog index arguments (Rel fits value)`.
    fn statement_of(&mut self, index: u64, arguments: Term, fits: Term, value: Term) -> Term {
        let fun_rel = self.lp("FunRel");
        let prog = self.prog;
        let i = self.nat_u(index);
        let observation = self.rel(fits, value);
        self.b.apps(fun_rel, &[prog, i, arguments, observation])
    }

    /// The rows of one well-founded definition: its width predicate and its
    /// relation theorem by `WellFounded.fix`, and the implementation record
    /// of the predicate by direct recursion.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(super) fn wf_definition(
        &mut self,
        index: u64,
        module: &str,
        name: &str,
        type_arguments: &[SemanticType],
        locals: &[(Local, SemanticType)],
        result: &SemanticType,
        body: &'a SemanticTerm,
        termination: &SemanticTermination,
        group: &BTreeSet<String>,
        site: Site,
        type_parameters: Vec<(String, SemanticType)>,
        recursive: bool,
    ) -> Result<(), String> {
        if group.len() > 1 {
            return Err(format!(
                "`{module}.{name}` is one of a mutual well-founded group"
            ));
        }
        if !recursive {
            return Err(format!(
                "`{module}.{name}` names a termination measure and does not recurse"
            ));
        }
        let clocals: Vec<CLocal> = Gen::scope(locals);
        let declared: Vec<String> = type_parameters.iter().map(|(n, _)| n.clone()).collect();
        let parameters: Vec<SemanticParameter> = locals
            .iter()
            .map(|(local, _)| SemanticParameter {
                name: local.name.clone(),
                r#type: local.ty.clone(),
            })
            .collect();
        let plan =
            crate::ir::semantic::well_founded_plan(name, group, &declared, &parameters, body)?;
        if plan.sites.len() != termination.evidence.len() {
            return Err(format!(
                "`{module}.{name}` has {} recursive calls and {} decrease theorems",
                plan.sites.len(),
                termination.evidence.len()
            ));
        }
        let hypotheses: Hypotheses = plan
            .addresses
            .iter()
            .enumerate()
            .map(|(index, address)| (*address, index))
            .collect();
        let sites: Vec<WfSite> = plan
            .sites
            .iter()
            .zip(&termination.evidence)
            .map(|(site, evidence)| WfSite {
                path: site.path.clone(),
                arguments: site.arguments.clone(),
                evidence: evidence.clone(),
            })
            .collect();
        let set = self.local_set(index, &clocals)?;
        if set.terms.iter().any(|terms| terms.companion.is_some()) {
            return Err(format!(
                "`{module}.{name}` is well-founded with a function parameter"
            ));
        }
        let denotation = self.denotation(module, name, type_arguments, &set)?;
        let sig = self.signature(index, &clocals, &set, result, denotation)?;
        self.sigs.insert(index, sig.clone());
        let env = self.entry_env(index, &clocals, &set.terms)?;
        let scope = Scope {
            ctx: Ctx::new(site, type_parameters, clocals, hypotheses, index),
            terms: set.terms.clone(),
            env,
            path: Vec::new(),
        };
        let mut lean_arguments = Vec::new();
        for argument in type_arguments {
            lean_arguments.push(self.lean_ty(argument)?);
        }
        let state = WfState {
            index,
            sites,
            type_arguments: lean_arguments,
            parameters: set.terms.iter().map(|terms| terms.value).collect(),
        };
        self.wf = Some(state.clone());
        let program = self.gen.program;
        let body_expr = &program.functions[index as usize].body;
        let proved = self.prove(body, body_expr, &scope);
        self.wf = None;
        let pf = proved?;
        let measure = self.src(&termination.measure, &scope)?;
        self.wf_rows(index, &sig, pf, env, &state, measure, result, denotation)
    }

    /// The rows themselves.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn wf_rows(
        &mut self,
        index: u64,
        sig: &super::rows::Sig,
        body: Pf,
        env: Term,
        state: &WfState,
        measure: Term,
        result: &SemanticType,
        denotation: Term,
    ) -> Result<(), String> {
        let params: Vec<Term> = sig.fits_binders.iter().map(|binder| binder.free).collect();
        let tys: Vec<Term> = sig.fits_binders.iter().map(|binder| binder.ty).collect();
        if params.is_empty() {
            return Err("a well-founded definition with no parameters".to_owned());
        }
        let bool = self.b.konst("Bool");
        let domain = self.pack_types(&tys, false);
        let x = self.b.free();
        let projections: Vec<Term> = (0..tys.len())
            .map(|position| self.unpack(&tys, position, x, false))
            .collect();
        // The relation the induction hypothesis is over: the measure's order.
        let measured = self.b.substitute(measure, &params, &projections);
        let measure_fn = self.b.lam_over("__x", EXPLICIT, domain, x, measured);
        let measure_head = self.b.konst_at("measure", vec![one()]);
        let order = self.b.apps(measure_head, &[domain, measure_fn]);
        let rel_head = self.b.konst_at("WellFoundedRelation.rel", vec![one()]);
        let rel = self.b.apps(rel_head, &[domain, order]);
        let wf_head = self.b.konst_at("WellFoundedRelation.wf", vec![one()]);
        let well_founded = self.b.apps(wf_head, &[domain, order]);
        let packed = self.pack_values(&tys, &params, false);
        let call_name = format!("{}.__wfcall_{index}", self.name);
        let rel_name = format!("{}.__wfrel_{index}", self.name);
        let fits_name = format!("{}.__fits_{index}", self.name);
        let unsafe_name = format!("{}.__fits_{index}._unsafe_rec", self.name);
        let fits_raw = self.fits_of(body)?;
        // The width predicate: the body with the induction hypothesis for
        // each recursive call.
        let ih = self.b.free();
        let fits_with_ih = {
            let (call_name, tys) = (call_name.clone(), tys.clone());
            self.b.rewrite_apps(
                fits_raw,
                &mut |b: &mut Builder, name: &str, args: &[Term]| {
                    if name != call_name {
                        return None;
                    }
                    let (evidence, rest) = args.split_last()?;
                    let packed = pack_values_with(b, &tys, rest, false);
                    Some(b.apps(ih, &[packed, *evidence]))
                },
            )
        };
        let fits_body = self.b.substitute(fits_with_ih, &params, &projections);
        let y = self.b.free();
        let below = self.b.apps(rel, &[y, x]);
        let ih_body = self.b.arrow(below, bool);
        let ih_ty = self.b.pi_over("y", EXPLICIT, domain, y, ih_body);
        let step_fn = self.b.lam_over("ih", EXPLICIT, ih_ty, ih, fits_body);
        let fits_step = self.b.lam_over("x", EXPLICIT, domain, x, step_fn);
        let fits_motive = self.b.lam("_", EXPLICIT, domain, bool);
        let fix = self.b.konst_at("WellFounded.fix", vec![one(), one()]);
        let fix_at = |b: &mut Builder, at: Term| {
            b.apps(
                fix,
                &[domain, fits_motive, rel, well_founded, fits_step, at],
            )
        };
        let fits_fix = fix_at(&mut self.b, packed);
        let fits_value = self.lambdas(&sig.fits_binders, fits_fix);
        let fits_ty = self.pis(&sig.fits_binders, bool);
        self.push_row(
            &format!("__fits_{index}"),
            RowKind::Definition { compiled: true },
            fits_ty,
            fits_value,
        )?;
        // The relation theorem. Its body derives the relation of the
        // definition's own body; the equations `fix_eq` and the definition's
        // `eq_def` carry it to the statement.
        let (body_fits, body_value) = self.rel_parts(body.ty)?;
        let (head, parts) = self.b.spine(sig.statement);
        let observation = self.rel(body_fits, body_value);
        let mut inner_sig = sig.clone();
        inner_sig.statement = self
            .b
            .apps(head, &[parts[0], parts[1], parts[2], observation]);
        let inner = self.relation_proof(index, &inner_sig, env, body)?;
        let fits_var = self.b.free();
        let moved_fits_statement = self.statement_of(index, sig.arguments, fits_var, body_value);
        let fits_motive_eq = self
            .b
            .lam_over("__f", EXPLICIT, bool, fits_var, moved_fits_statement);
        let again = self.b.free();
        let below_again = self.b.apps(rel, &[again, packed]);
        let recur_again = fix_at(&mut self.b, again);
        let lifted = self.b.lift(recur_again, 1, 0);
        let inside = self.b.lam("_", EXPLICIT, below_again, lifted);
        let hypothesis_fn = self.b.lam_over("y", EXPLICIT, domain, again, inside);
        let unfolded = self.b.apps(fits_step, &[packed, hypothesis_fn]);
        let fix_eq = self.b.konst_at("WellFounded.fix_eq", vec![one(), one()]);
        let equation = self.b.apps(
            fix_eq,
            &[domain, fits_motive, rel, well_founded, fits_step, packed],
        );
        let symm = self.b.konst1("Eq.symm");
        let flipped = self.b.apps(symm, &[bool, fits_fix, unfolded, equation]);
        let ndrec = self.b.konst_at("Eq.ndrec", vec![CoreLevel::Zero, one()]);
        let by_fits = self.b.apps(
            ndrec,
            &[
                bool,
                body_fits,
                fits_motive_eq,
                inner.proof,
                sig.fits_applied,
                flipped,
            ],
        );
        // The denotation's equation.
        let encoder = self.enc(result)?;
        let value_node = self.b.node(body_value).clone();
        let denoted = match value_node {
            crate::ir::core::CoreNode::App { f, x: denoted } if f == encoder => denoted,
            crate::ir::core::CoreNode::BVar { .. }
            | crate::ir::core::CoreNode::Sort { .. }
            | crate::ir::core::CoreNode::Const { .. }
            | crate::ir::core::CoreNode::App { .. }
            | crate::ir::core::CoreNode::Lambda { .. }
            | crate::ir::core::CoreNode::Forall { .. }
            | crate::ir::core::CoreNode::Let { .. }
            | crate::ir::core::CoreNode::Nat { .. }
            | crate::ir::core::CoreNode::String { .. }
            | crate::ir::core::CoreNode::Mdata { .. }
            | crate::ir::core::CoreNode::Projection { .. } => {
                return Err("the value of a well-founded body is not an encoding".to_owned())
            }
        };
        let lean_result = self.lean_ty(result)?;
        let value_var = self.b.free();
        let encoded_var = self.b.app(encoder, value_var);
        let moved_value_statement =
            self.statement_of(index, sig.arguments, sig.fits_applied, encoded_var);
        let value_motive = self.b.lam_over(
            "__d",
            EXPLICIT,
            lean_result,
            value_var,
            moved_value_statement,
        );
        let (definition, definition_args) = self.b.spine(denotation);
        let definition_name = match self.b.node(definition).clone() {
            crate::ir::core::CoreNode::Const { n, .. } => n,
            crate::ir::core::CoreNode::BVar { .. }
            | crate::ir::core::CoreNode::Sort { .. }
            | crate::ir::core::CoreNode::App { .. }
            | crate::ir::core::CoreNode::Lambda { .. }
            | crate::ir::core::CoreNode::Forall { .. }
            | crate::ir::core::CoreNode::Let { .. }
            | crate::ir::core::CoreNode::Nat { .. }
            | crate::ir::core::CoreNode::String { .. }
            | crate::ir::core::CoreNode::Mdata { .. }
            | crate::ir::core::CoreNode::Projection { .. } => {
                return Err("the denotation of a definition is not a constant".to_owned())
            }
        };
        let eq_def_head = self.b.konst(&format!("{definition_name}.eq_def"));
        let eq_def = self.b.apps(eq_def_head, &definition_args);
        let flipped_def = self
            .b
            .apps(symm, &[lean_result, denotation, denoted, eq_def]);
        let by_value = self.b.apps(
            ndrec,
            &[
                lean_result,
                denoted,
                value_motive,
                by_fits,
                denotation,
                flipped_def,
            ],
        );
        // The markers: the predicate where a type states it, the induction
        // hypothesis where a derivation uses the theorem.
        let ihr = self.b.free();
        let transported = {
            let tys = tys.clone();
            let fits_name = fits_name.clone();
            let (call_name, rel_name) = (call_name.clone(), rel_name.clone());
            self.b.rewrite_apps(
                by_value,
                &mut |b: &mut Builder, name: &str, args: &[Term]| {
                    if name == call_name {
                        let (_, rest) = args.split_last()?;
                        let head = b.konst(&fits_name);
                        return Some(b.apps(head, rest));
                    }
                    if name == rel_name {
                        let (evidence, rest) = args.split_last()?;
                        let packed = pack_values_with(b, &tys, rest, false);
                        return Some(b.apps(ihr, &[packed, *evidence]));
                    }
                    None
                },
            )
        };
        let rel_body = self.b.substitute(transported, &params, &projections);
        let statement_at = |writer: &mut Self, at: Term| -> Vec<Term> {
            (0..tys.len())
                .map(|position| writer.unpack(&tys, position, at, false))
                .collect()
        };
        let below_projections = statement_at(self, y);
        let below_statement = self
            .b
            .substitute(sig.statement, &params, &below_projections);
        let ihr_body = self.b.arrow(below, below_statement);
        let ihr_ty = self.b.pi_over("y", EXPLICIT, domain, y, ihr_body);
        let rel_step_fn = self.b.lam_over("ih", EXPLICIT, ihr_ty, ihr, rel_body);
        let rel_step = self.b.lam_over("x", EXPLICIT, domain, x, rel_step_fn);
        let motive_var = self.b.free();
        let motive_projections = statement_at(self, motive_var);
        let motive_body = self
            .b
            .substitute(sig.statement, &params, &motive_projections);
        let rel_motive = self
            .b
            .lam_over("__x", EXPLICIT, domain, motive_var, motive_body);
        let rel_fix = self
            .b
            .konst_at("WellFounded.fix", vec![one(), CoreLevel::Zero]);
        let rel_fixed = self.b.apps(
            rel_fix,
            &[domain, rel_motive, rel, well_founded, rel_step, packed],
        );
        let rel_value = self.lambdas(&sig.rel_binders, rel_fixed);
        let rel_ty = self.pis(&sig.rel_binders, sig.statement);
        self.push_row(
            &format!("__rel_{index}"),
            RowKind::Theorem,
            rel_ty,
            rel_value,
        )?;
        // The implementation record of the predicate: direct recursion.
        let direct = {
            let (call_name, unsafe_name) = (call_name, unsafe_name);
            self.b.rewrite_apps(
                fits_raw,
                &mut |b: &mut Builder, name: &str, args: &[Term]| {
                    if name != call_name {
                        return None;
                    }
                    let (_, rest) = args.split_last()?;
                    let head = b.konst(&unsafe_name);
                    Some(b.apps(head, rest))
                },
            )
        };
        let unsafe_value = self.lambdas(&sig.fits_binders, direct);
        let unsafe_ty = self.pis(&sig.fits_binders, bool);
        let short = format!("__fits_{index}._unsafe_rec");
        self.push_row(&short, RowKind::Partial, unsafe_ty, unsafe_value)?;
        self.group_rows(&[short]);
        let _ = state;
        Ok(())
    }
}
