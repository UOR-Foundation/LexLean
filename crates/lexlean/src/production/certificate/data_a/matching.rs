//! The proof of a `match`: `conv_matchV` over the scrutinee, whose arms are
//! the eliminator of the scrutinee's type applied to one derivation of the
//! program's arm list per constructor (SPEC.md §17.17.4).

use super::{Pf, Scope, Writer, EXPLICIT};
use crate::calculus::{Arm, Expr};
use crate::ir::core::CoreLevel;
use crate::ir::semantic::{SemanticBranch, SemanticTerm, SemanticType};
use crate::production::kernel::{Given, Term};
use crate::production::source::Constructor;

impl<'a> Writer<'_, 'a> {
    /// The derivation that the arm at `position` of `arms` is taken for the
    /// value `encoded`: the arm matches, binds `fields` over `env`, and its
    /// body is derived by `body`.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn arm_hit(
        &mut self,
        env: Term,
        encoded: Term,
        arms: &'a [Arm],
        position: usize,
        fields: &[Term],
        extended: Term,
        body: Pf,
    ) -> Result<Pf, String> {
        let arm = arms.get(position).ok_or("a match lowered to fewer arms")?;
        let shape = self.shape_term(arm.shape);
        let binders = self.numbers_term(&arm.binders);
        let body_expr = self.expr_term(&arm.body)?;
        let rest = self.arms_list(&arms[position + 1..])?;
        let value_ty = self.value_ty();
        let list = self.b.list(value_ty, fields);
        let p = self.prog;
        self.apply(
            "convA_hit",
            &[
                ("p", p),
                ("env", env),
                ("v", encoded),
                ("shape", shape),
                ("binders", binders),
                ("body", body_expr),
                ("rest", rest),
                ("fields", list),
                ("env'", extended),
            ],
            &[
                Given::Refl,
                Given::Refl,
                Given::Proof(body.proof, body.ty),
            ],
        )
    }

    /// The derivation that the arm at `position` of `arms` is missed for the
    /// value `encoded`, and a later arm is taken.
    pub(super) fn arm_miss(
        &mut self,
        env: Term,
        encoded: Term,
        arms: &'a [Arm],
        position: usize,
        later: Pf,
    ) -> Result<Pf, String> {
        let arm = arms.get(position).ok_or("a match lowered to fewer arms")?;
        let shape = self.shape_term(arm.shape);
        let binders = self.numbers_term(&arm.binders);
        let body_expr = self.expr_term(&arm.body)?;
        let rest = self.arms_list(&arms[position + 1..])?;
        let p = self.prog;
        self.apply(
            "convA_miss",
            &[
                ("p", p),
                ("env", env),
                ("v", encoded),
                ("shape", shape),
                ("binders", binders),
                ("body", body_expr),
                ("rest", rest),
            ],
            &[Given::Refl, Given::Proof(later.proof, later.ty)],
        )
    }

    /// `(name, value) :: env` for each binder in order.
    pub(super) fn extend_env(
        &mut self,
        env: Term,
        binders: &[u64],
        fields: &[Term],
    ) -> Result<Term, String> {
        if binders.len() != fields.len() {
            return Err(format!(
                "an arm binds {} names to {} fields",
                binders.len(),
                fields.len()
            ));
        }
        let nat = self.b.konst("Nat");
        let value_ty = self.value_ty();
        let entry_ty = self.b.pair_ty(nat, value_ty);
        let cons = self.b.konst0("List.cons");
        let mut out = env;
        for (number, field) in binders.iter().zip(fields) {
            let name = self.nat_u(*number);
            let entry = self.b.pair(nat, value_ty, name, *field);
            out = self.b.apps(cons, &[entry_ty, entry, out]);
        }
        Ok(out)
    }

    /// The value of the constructor a branch matches, on the free variables
    /// of its fields.
    pub(super) fn constructor_value(
        &mut self,
        constructor: &Constructor,
        scrutinee_ty: &SemanticType,
        branch: &SemanticBranch,
        frees: &[Term],
        scope: &Scope,
    ) -> Result<Term, String> {
        self.constructed(
            constructor,
            scrutinee_ty,
            &branch.constructor,
            &scope.ctx.site,
            frees,
        )
    }

    /// The proof of a `match` the lowering made a `match` of.
    #[allow(clippy::too_many_lines)]
    pub(super) fn matching(
        &mut self,
        whole: &SemanticTerm,
        scrutinee: &SemanticTerm,
        branches: &[SemanticBranch],
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        let (match_ty, scrutinee_expr, arms) = match expr {
            Expr::Match {
                ty,
                scrutinee,
                arms,
            } => (ty, scrutinee.as_ref(), arms.as_slice()),
            _ => return Err("the lowering of a match is not a match".to_owned()),
        };
        let scrutinee_ty = self.infer(scrutinee, scope)?;
        let result_ty = self.infer(whole, scope)?;
        if matches!(
            scrutinee_ty,
            SemanticType::ContractViolation | SemanticType::ReasoningFailure
        ) {
            return self.violation_matching(
                &result_ty,
                match_ty,
                scrutinee,
                branches,
                arms,
                scrutinee_expr,
                scope,
            );
        }
        let (p, env) = (self.prog, scope.env);
        let lean_scrutinee = self.lean_ty(&scrutinee_ty)?;
        let result_lean = self.lean_ty(&result_ty)?;
        let scrutinee_enc = self.enc(&scrutinee_ty)?;
        let result_enc = self.enc(&result_ty)?;
        let tested = self.prove(scrutinee, scrutinee_expr, scope)?;
        let scrutinee_src = self.src(scrutinee, scope)?;
        let bound = self.b.free();
        let hypothesis = self.b.free();
        let bool = self.b.konst("Bool");
        // Per constructor, in declaration order: the arm's width predicate,
        // value, and derivation, each abstracted over the arm's fields.
        let mut fits_minors: Vec<(usize, Term)> = Vec::new();
        let mut value_minors: Vec<(usize, Term)> = Vec::new();
        let mut proof_minors: Vec<(usize, Term)> = Vec::new();
        for (position, branch) in branches.iter().enumerate() {
            let locals = self.gen.branch_locals(branch, scrutinee, &scope.ctx)?;
            let mut frees = Vec::new();
            let mut types = Vec::new();
            for (local, _) in &locals {
                frees.push(self.b.free());
                types.push(self.lean_ty(&local.ty.clone())?);
            }
            let arm = arms.get(position).ok_or("a match lowered to fewer arms")?;
            let mut fields = Vec::new();
            for ((local, _), free) in locals.iter().zip(&frees) {
                let enc = self.enc(&local.ty.clone())?;
                fields.push(self.b.app(enc, *free));
            }
            let arm_env = self.extend_env(env, &arm.binders, &fields)?;
            let inner = self.bind(scope, locals.clone(), &frees, arm_env);
            let body_pf = self.prove(&branch.body, &arm.body, &inner)?;
            let fits = self.fits_of(body_pf)?;
            let body_src = self.src(&branch.body, &inner)?;
            let (constructor, _) =
                self.gen
                    .source
                    .branch(branch, &scrutinee_ty, &scope.ctx.site)?;
            let value =
                self.constructor_value(&constructor, &scrutinee_ty, branch, &frees, scope)?;
            let encoded = self.b.app(scrutinee_enc, value);
            let mut proof = self.arm_hit(env, encoded, arms, position, &fields, arm_env, body_pf)?;
            for earlier in (0..position).rev() {
                proof = self.arm_miss(env, encoded, arms, earlier, proof)?;
            }
            let ctor_position = self.constructor_position(&constructor)?;
            let wrap = |writer: &mut Self, term: Term| {
                let mut out = term;
                for (index, (local, _)) in locals.iter().enumerate().rev() {
                    out = writer.b.lam_over(
                        &super::super::identifier(&local.name),
                        EXPLICIT,
                        types[index],
                        frees[index],
                        out,
                    );
                }
                out
            };
            fits_minors.push((ctor_position, wrap(self, fits)));
            value_minors.push((ctor_position, wrap(self, body_src)));
            proof_minors.push((ctor_position, wrap(self, proof.proof)));
        }
        let sorted = |mut minors: Vec<(usize, Term)>| -> Vec<Term> {
            minors.sort_by_key(|(position, _)| *position);
            minors.into_iter().map(|(_, term)| term).collect()
        };
        let (fits_terms, value_terms, proof_terms) = (
            sorted(fits_minors),
            sorted(value_minors),
            sorted(proof_minors),
        );
        // G a = the arms' predicates by cases on a; W a = the encoding of the
        // arms' values by cases on a.
        let g_body = self.cases_on_const(&scrutinee_ty, bound, bool, &fits_terms)?;
        let g = self.b.lam_over("__a", EXPLICIT, lean_scrutinee, bound, g_body);
        let w_inner = self.cases_on_const(&scrutinee_ty, bound, result_lean, &value_terms)?;
        let w_body = self.b.app(result_enc, w_inner);
        let w = self.b.lam_over("__a", EXPLICIT, lean_scrutinee, bound, w_body);
        // ha : ∀ a, sv = a → ConvA p env (enc a) arms (Rel (G a) (W a)),
        // proved by cases on a with the arms' derivations as minors.
        let arms_term = self.arms_list(arms)?;
        let encoded_bound = self.b.app(scrutinee_enc, bound);
        let g_a = self.b.app(g, bound);
        let w_a = self.b.app(w, bound);
        let rel = self.rel(g_a, w_a);
        let conv_a = self.lp("ConvA");
        let target = self.b.apps(conv_a, &[p, env, encoded_bound, arms_term, rel]);
        let motive = self
            .b
            .lam_over("__a", EXPLICIT, lean_scrutinee, bound, target);
        let cases = self.cases_on(
            &scrutinee_ty,
            bound,
            motive,
            CoreLevel::Zero,
            &proof_terms,
        )?;
        let equation = self.eq(lean_scrutinee, scrutinee_src, bound);
        let with_hypothesis = self
            .b
            .lam_over("__h", EXPLICIT, equation, hypothesis, cases);
        let ha = self
            .b
            .lam_over("__a", EXPLICIT, lean_scrutinee, bound, with_hypothesis);
        let scrutinee_expr_term = self.expr_term(scrutinee_expr)?;
        let ty = self.ty_term(match_ty);
        self.apply(
            "conv_matchV",
            &[
                ("p", p),
                ("env", env),
                ("α", lean_scrutinee),
                ("t", ty),
                ("s", scrutinee_expr_term),
                ("arms", arms_term),
            ],
            &[
                Given::Term(scrutinee_enc),
                Given::Term(g),
                Given::Term(w),
                Given::Term(scrutinee_src),
                Given::Proof(tested.proof, tested.ty),
                Given::Term(ha),
            ],
        )
    }

    /// The proof of a match over a violation: its pair of Booleans, then each
    /// Boolean in turn (`lower::violation_match`).
    #[allow(clippy::too_many_arguments)]
    fn violation_matching(
        &mut self,
        result_ty: &SemanticType,
        match_ty: &crate::calculus::Ty,
        scrutinee: &SemanticTerm,
        branches: &[SemanticBranch],
        arms: &'a [Arm],
        scrutinee_expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        let _ = (result_ty, match_ty, scrutinee, branches, arms, scrutinee_expr, scope);
        Err("violation matches".to_owned())
    }
}
