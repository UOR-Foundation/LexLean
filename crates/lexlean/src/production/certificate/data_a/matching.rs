//! The proof of a `match`: `conv_matchV` over the scrutinee, whose arms are
//! the eliminator of the scrutinee's type applied to one derivation of the
//! program's arm list per constructor (SPEC.md §17.17.4).

use super::{Pf, Scope, Writer, EXPLICIT};
use crate::calculus::{Arm, Expr};
use crate::ir::core::CoreLevel;
use crate::ir::semantic::{SemanticBranch, SemanticTerm, SemanticType, Step};
use crate::production::kernel::{Given, Term};
use crate::production::source::Constructor;

/// The precomputed terms of one match arm, with the suffix of the arms after
/// it.
#[derive(Clone, Copy)]
pub(super) struct PreArm {
    shape: Term,
    binders: Term,
    body: Term,
    rest: Term,
}

impl<'a> Writer<'_, 'a> {
    /// Precompute the terms of each arm in `arms`, and the list of all arms
    /// with the suffix after each position, right to left in $O(n)$.
    pub(super) fn prepare_arms(&mut self, arms: &[Arm]) -> Result<(Term, Vec<PreArm>), String> {
        let ty = self.ts("Arm");
        let head = self.ts("Arm.arm");
        let nil = self.b.konst0("List.nil");
        let cons = self.b.konst0("List.cons");
        let mut parts = Vec::with_capacity(arms.len());
        let mut item_terms = Vec::with_capacity(arms.len());
        for arm in arms {
            let shape = self.shape_term(arm.shape);
            let binders = self.numbers_term(&arm.binders);
            let body = self.expr_term(&arm.body)?;
            parts.push((shape, binders, body));
            item_terms.push(self.b.apps(head, &[shape, binders, body]));
        }
        let mut list = self.b.app(nil, ty);
        let mut pre = vec![
            PreArm {
                shape: 0,
                binders: 0,
                body: 0,
                rest: 0,
            };
            arms.len()
        ];
        for index in (0..arms.len()).rev() {
            let (shape, binders, body) = parts[index];
            pre[index] = PreArm {
                shape,
                binders,
                body,
                rest: list,
            };
            list = self.b.apps(cons, &[ty, item_terms[index], list]);
        }
        Ok((list, pre))
    }

    /// The derivation that the arm at `position` of `arms` is taken for the
    /// value `encoded`: the arm matches, binds `fields` over `env`, and its
    /// body is derived by `body`.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn arm_hit(
        &mut self,
        env: Term,
        encoded: Term,
        arms: &[PreArm],
        position: usize,
        fields: &[Term],
        extended: Term,
        body: Pf,
    ) -> Result<Pf, String> {
        let arm = arms.get(position).ok_or("a match lowered to fewer arms")?;
        let value_ty = self.value_ty();
        let list = self.b.list(value_ty, fields);
        let p = self.prog;
        self.apply(
            "convA_hit",
            &[
                ("p", p),
                ("env", env),
                ("v", encoded),
                ("shape", arm.shape),
                ("binders", arm.binders),
                ("body", arm.body),
                ("rest", arm.rest),
                ("fields", list),
                ("env'", extended),
            ],
            &[Given::Refl, Given::Refl, Given::Proof(body.proof, body.ty)],
        )
    }

    /// The derivation that the arm at `position` of `arms` is missed for the
    /// value `encoded`, and a later arm is taken.
    pub(super) fn arm_miss(
        &mut self,
        env: Term,
        encoded: Term,
        arms: &[PreArm],
        position: usize,
        later: Pf,
    ) -> Result<Pf, String> {
        let arm = arms.get(position).ok_or("a match lowered to fewer arms")?;
        let p = self.prog;
        self.apply(
            "convA_miss",
            &[
                ("p", p),
                ("env", env),
                ("v", encoded),
                ("shape", arm.shape),
                ("binders", arm.binders),
                ("body", arm.body),
                ("rest", arm.rest),
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
            Expr::Value { .. }
            | Expr::Var { .. }
            | Expr::Let { .. }
            | Expr::Cond { .. }
            | Expr::Build { .. }
            | Expr::Call { .. }
            | Expr::Closure { .. }
            | Expr::Apply { .. }
            | Expr::Prim { .. }
            | Expr::First { .. }
            | Expr::Second { .. }
            | Expr::Field { .. } => return Err("the lowering of a match is not a match".to_owned()),
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
        let node = self.node_of(scope, whole);
        // Per constructor, in declaration order: the arm's width predicate,
        // value, and derivation, each abstracted over the arm's fields.
        let (arms_term, pre_arms) = self.prepare_arms(arms)?;
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
            let mut inner = self.bind(scope, locals.clone(), &frees, arm_env);
            let equation_free = self.b.free();
            if let Some(node) = node {
                inner = Self::entering(
                    &inner,
                    Step::Match {
                        node,
                        branch: position,
                    },
                    equation_free,
                    &frees,
                );
            }
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
            let mut proof =
                self.arm_hit(env, encoded, &pre_arms, position, &fields, arm_env, body_pf)?;
            for earlier in (0..position).rev() {
                proof = self.arm_miss(env, encoded, &pre_arms, earlier, proof)?;
            }
            let ctor_position = self.constructor_position(&constructor)?;
            let binders: Vec<(String, Term, Term)> = locals
                .iter()
                .enumerate()
                .map(|(index, (local, _))| {
                    (
                        super::super::identifier(&local.name),
                        types[index],
                        frees[index],
                    )
                })
                .collect();
            if node.is_some() {
                // The arm is taken by an equation the decrease evidence of a
                // recursive call within it reads: each of the predicate, the
                // value, and the derivation carries it as a hypothesis.
                let lean_value =
                    self.constructor_value(&constructor, &scrutinee_ty, branch, &frees, scope)?;
                let of_bound = self.eq(lean_scrutinee, bound, lean_value);
                let of_scrutinee = self.eq(lean_scrutinee, scrutinee_src, lean_value);
                let unused = self.b.free();
                let fits_arm = self.arm_lambda(&binders, (equation_free, of_bound), fits);
                let value_arm = self.arm_lambda(&binders, (unused, of_bound), body_src);
                let proof_arm =
                    self.arm_lambda(&binders, (equation_free, of_scrutinee), proof.proof);
                fits_minors.push((ctor_position, fits_arm));
                value_minors.push((ctor_position, value_arm));
                proof_minors.push((ctor_position, proof_arm));
                continue;
            }
            let wrap = |writer: &mut Self, term: Term| {
                let mut out = term;
                for (name, ty, free) in binders.iter().rev() {
                    out = writer.b.lam_over(name, EXPLICIT, *ty, *free, out);
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
        let sorted_with = |minors: &Vec<(usize, Term)>| minors.clone();
        let (fits_terms, value_terms, proof_terms) = (
            sorted(fits_minors.clone()),
            sorted(value_minors.clone()),
            sorted(proof_minors.clone()),
        );
        // G a = the arms' predicates by cases on a; W a = the encoding of the
        // arms' values by cases on a.
        let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
        let g_body = if node.is_some() {
            self.equation_cases(
                &scrutinee_ty,
                bound,
                bool,
                one.clone(),
                sorted_with(&fits_minors),
            )?
        } else {
            self.cases_on_const(&scrutinee_ty, bound, bool, &fits_terms)?
        };
        let g = self
            .b
            .lam_over("__a", EXPLICIT, lean_scrutinee, bound, g_body);
        let w_inner = if node.is_some() {
            self.equation_cases(
                &scrutinee_ty,
                bound,
                result_lean,
                one,
                sorted_with(&value_minors),
            )?
        } else {
            self.cases_on_const(&scrutinee_ty, bound, result_lean, &value_terms)?
        };
        let w_body = self.b.app(result_enc, w_inner);
        let w = self
            .b
            .lam_over("__a", EXPLICIT, lean_scrutinee, bound, w_body);
        // ha : ∀ a, sv = a → ConvA p env (enc a) arms (Rel (G a) (W a)),
        // proved by cases on a with the arms' derivations as minors.
        let encoded_bound = self.b.app(scrutinee_enc, bound);
        let g_a = self.b.app(g, bound);
        let w_a = self.b.app(w, bound);
        let rel = self.rel(g_a, w_a);
        let conv_a = self.lp("ConvA");
        let target = self
            .b
            .apps(conv_a, &[p, env, encoded_bound, arms_term, rel]);
        let equation = self.eq(lean_scrutinee, scrutinee_src, bound);
        let cases = if node.is_some() {
            let dependent = self.b.arrow(equation, target);
            let motive = self
                .b
                .lam_over("__a", EXPLICIT, lean_scrutinee, bound, dependent);
            let cases =
                self.cases_on(&scrutinee_ty, bound, motive, CoreLevel::Zero, &proof_terms)?;
            self.b.app(cases, hypothesis)
        } else {
            let motive = self
                .b
                .lam_over("__a", EXPLICIT, lean_scrutinee, bound, target);
            self.cases_on(&scrutinee_ty, bound, motive, CoreLevel::Zero, &proof_terms)?
        };
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
    /// Boolean in turn (`lower::violation_match`). The derivation is by cases
    /// on the pair and on each Boolean, so that each of the four
    /// constructors is a closed run of the program's arms.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
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
        let (p, env) = (self.prog, scope.env);
        let scrutinee_ty = self.infer(scrutinee, scope)?;
        let pair_arm = arms.first().ok_or("a violation match lowered to no arms")?;
        let (first_var, second_var) = match pair_arm.binders.as_slice() {
            [first, second] => (*first, *second),
            _ => return Err("a violation match binds other than its pair".to_owned()),
        };
        let (halves_ty, first_expr, halves) = match &pair_arm.body {
            Expr::Match {
                ty,
                scrutinee,
                arms,
            } => (ty, scrutinee.as_ref(), arms.as_slice()),
            Expr::Value { .. }
            | Expr::Var { .. }
            | Expr::Let { .. }
            | Expr::Cond { .. }
            | Expr::Build { .. }
            | Expr::Call { .. }
            | Expr::Closure { .. }
            | Expr::Apply { .. }
            | Expr::Prim { .. }
            | Expr::First { .. }
            | Expr::Second { .. }
            | Expr::Field { .. } => {
                return Err("a violation match's pair arm is not a match".to_owned())
            }
        };
        let _ = first_expr;
        let bool = self.b.konst("Bool");
        let value_ty = self.value_ty();
        let result_lean = self.lean_ty(result_ty)?;
        let result_enc = self.enc(result_ty)?;
        let scrutinee_enc = self.enc(&scrutinee_ty)?;
        let lean_scrutinee = self.lean_ty(&scrutinee_ty)?;
        let tested = self.prove(scrutinee, scrutinee_expr, scope)?;
        let scrutinee_src = self.src(scrutinee, scope)?;
        // The body of each of the four constructors.
        let mut leaves: Vec<((bool, bool), &SemanticBranch)> = Vec::new();
        for branch in branches {
            let (constructor, _) =
                self.gen
                    .source
                    .branch(branch, &scrutinee_ty, &scope.ctx.site)?;
            match constructor {
                crate::production::source::Constructor::Violation(first, second) => {
                    leaves.push(((first, second), branch));
                }
                crate::production::source::Constructor::Bool(_)
                | crate::production::source::Constructor::Zero
                | crate::production::source::Constructor::Succ
                | crate::production::source::Constructor::Nil
                | crate::production::source::Constructor::Cons
                | crate::production::source::Constructor::OptionNone
                | crate::production::source::Constructor::OptionSome
                | crate::production::source::Constructor::Ok
                | crate::production::source::Constructor::Error
                | crate::production::source::Constructor::Document { ty: _, index: _ } => {
                    return Err("a violation match over another constructor".to_owned());
                }
            }
        }
        let (arms_term, pre_arms) = self.prepare_arms(arms)?;
        let (halves_term, pre_halves) = self.prepare_arms(halves)?;
        // Per (first, second): the derivation of the program's arms, the
        // arm's predicate and its value.
        let mut proofs: Vec<[Term; 2]> = vec![[Term::default(); 2]; 2];
        let mut fits_leaves: Vec<[Term; 2]> = vec![[Term::default(); 2]; 2];
        let mut value_leaves: Vec<[Term; 2]> = vec![[Term::default(); 2]; 2];
        let first_ty = self.ty_term(halves_ty);
        for (first_value, second_value) in
            [(false, false), (false, true), (true, false), (true, true)]
        {
            let branch = leaves
                .iter()
                .find(|(key, _)| *key == (first_value, second_value))
                .map(|(_, branch)| *branch)
                .ok_or("a violation match misses a constructor")?;
            let first_bool = self.boolean(first_value);
            let second_bool = self.boolean(second_value);
            let bool_enc = self.ts("Value.bool");
            let first_encoded = self.b.app(bool_enc, first_bool);
            let second_encoded = self.b.app(bool_enc, second_bool);
            let pair_env = self.extend_env(
                env,
                &[first_var, second_var],
                &[first_encoded, second_encoded],
            )?;
            let inner_scope = Scope {
                env: pair_env,
                ..scope.clone()
            };
            let second_arms = &halves
                .get(usize::from(first_value))
                .ok_or("a violation match lowered to fewer halves")?;
            let (second_ty, leaf_arms) = match &second_arms.body {
                Expr::Match { ty, arms, .. } => (ty, arms.as_slice()),
                Expr::Value { .. }
                | Expr::Var { .. }
                | Expr::Let { .. }
                | Expr::Cond { .. }
                | Expr::Build { .. }
                | Expr::Call { .. }
                | Expr::Closure { .. }
                | Expr::Apply { .. }
                | Expr::Prim { .. }
                | Expr::First { .. }
                | Expr::Second { .. }
                | Expr::Field { .. } => return Err("a violation half is not a match".to_owned()),
            };
            let leaf = leaf_arms
                .get(usize::from(second_value))
                .ok_or("a violation half lowered to fewer arms")?;
            let body_pf = self.prove(&branch.body, &leaf.body, &inner_scope)?;
            let fits = self.fits_of(body_pf)?;
            let value = self.src(&branch.body, &inner_scope)?;
            // The innermost Boolean, then the first, then the pair.
            let (second_arms_term, pre_leaf_arms) = self.prepare_arms(leaf_arms)?;
            let leaf_hit = self.arm_hit(
                pair_env,
                second_encoded,
                &pre_leaf_arms,
                usize::from(second_value),
                &[],
                pair_env,
                body_pf,
            )?;
            let mut leaf_chain = leaf_hit;
            for earlier in (0..usize::from(second_value)).rev() {
                leaf_chain = self.arm_miss(
                    pair_env,
                    second_encoded,
                    &pre_leaf_arms,
                    earlier,
                    leaf_chain,
                )?;
            }
            let second_var_term = self.nat_u(second_var);
            let second_test = self.apply(
                "conv_var",
                &[
                    ("p", p),
                    ("env", pair_env),
                    ("x", second_var_term),
                    ("v", second_encoded),
                ],
                &[Given::Refl],
            )?;
            let second_match_ty = self.ty_term(second_ty);
            let inner = self.apply(
                "conv_match",
                &[
                    ("p", p),
                    ("env", pair_env),
                    ("t", second_match_ty),
                    ("arms", second_arms_term),
                ],
                &[
                    Given::Proof(second_test.proof, second_test.ty),
                    Given::Proof(leaf_chain.proof, leaf_chain.ty),
                ],
            )?;
            let half_hit = self.arm_hit(
                pair_env,
                first_encoded,
                &pre_halves,
                usize::from(first_value),
                &[],
                pair_env,
                inner,
            )?;
            let mut half_chain = half_hit;
            for earlier in (0..usize::from(first_value)).rev() {
                half_chain =
                    self.arm_miss(pair_env, first_encoded, &pre_halves, earlier, half_chain)?;
            }
            let first_var_term = self.nat_u(first_var);
            let first_test = self.apply(
                "conv_var",
                &[
                    ("p", p),
                    ("env", pair_env),
                    ("x", first_var_term),
                    ("v", first_encoded),
                ],
                &[Given::Refl],
            )?;
            let outer = self.apply(
                "conv_match",
                &[
                    ("p", p),
                    ("env", pair_env),
                    ("t", first_ty),
                    ("arms", halves_term),
                ],
                &[
                    Given::Proof(first_test.proof, first_test.ty),
                    Given::Proof(half_chain.proof, half_chain.ty),
                ],
            )?;
            let pair_value = self.b.pair(bool, bool, first_bool, second_bool);
            let encoded = self.b.app(scrutinee_enc, pair_value);
            let top = self.arm_hit(
                env,
                encoded,
                &pre_arms,
                0,
                &[first_encoded, second_encoded],
                pair_env,
                outer,
            )?;
            proofs[usize::from(first_value)][usize::from(second_value)] = top.proof;
            fits_leaves[usize::from(first_value)][usize::from(second_value)] = fits;
            value_leaves[usize::from(first_value)][usize::from(second_value)] = value;
        }
        let _ = (value_ty, match_ty);
        // G a and W a: the leaves by cases on the pair, then on each Boolean.
        let bound = self.b.free();
        let first = self.b.free();
        let second = self.b.free();
        let by_cases =
            |writer: &mut Self, leaves: &[[Term; 2]], result: Term| -> Result<Term, String> {
                let mut rows = Vec::new();
                for row in leaves {
                    rows.push(writer.cases_on_const(&SemanticType::Bool, second, result, row)?);
                }
                let by_first = writer.cases_on_const(&SemanticType::Bool, first, result, &rows)?;
                let inner = writer
                    .b
                    .lam_over("__second", EXPLICIT, bool, second, by_first);
                let minor = writer.b.lam_over("__first", EXPLICIT, bool, first, inner);
                writer.cases_on_const(&scrutinee_ty, bound, result, &[minor])
            };
        let g_body = by_cases(self, &fits_leaves, bool)?;
        let g = self
            .b
            .lam_over("__a", EXPLICIT, lean_scrutinee, bound, g_body);
        let w_inner = by_cases(self, &value_leaves, result_lean)?;
        let w_body = self.b.app(result_enc, w_inner);
        let w = self
            .b
            .lam_over("__a", EXPLICIT, lean_scrutinee, bound, w_body);
        // ha: by cases on the pair, then on each Boolean, the closed runs.
        let conv_a = self.lp("ConvA");
        let target_at = |writer: &mut Self, value: Term| -> Term {
            let encoded = writer.b.app(scrutinee_enc, value);
            let g_at = writer.b.app(g, value);
            let w_at = writer.b.app(w, value);
            let rel = writer.rel(g_at, w_at);
            writer.b.apps(conv_a, &[p, env, encoded, arms_term, rel])
        };
        let zero = CoreLevel::Zero;
        let mut rows = Vec::new();
        for first_value in [false, true] {
            let first_bool = self.boolean(first_value);
            let free_second = self.b.free();
            let pair_at = self.b.pair(bool, bool, first_bool, free_second);
            let target = target_at(self, pair_at);
            let motive = self
                .b
                .lam_over("__second", EXPLICIT, bool, free_second, target);
            let minors: Vec<Term> = proofs[usize::from(first_value)].to_vec();
            rows.push(self.cases_on(&SemanticType::Bool, second, motive, zero.clone(), &minors)?);
        }
        let pair_at = self.b.pair(bool, bool, first, second);
        let first_target = target_at(self, pair_at);
        let first_motive = self
            .b
            .lam_over("__first", EXPLICIT, bool, first, first_target);
        let by_first = self.cases_on(
            &SemanticType::Bool,
            first,
            first_motive,
            zero.clone(),
            &rows,
        )?;
        let minor = {
            let inner = self
                .b
                .lam_over("__second", EXPLICIT, bool, second, by_first);
            self.b.lam_over("__first", EXPLICIT, bool, first, inner)
        };
        let target = target_at(self, bound);
        let motive = self
            .b
            .lam_over("__a", EXPLICIT, lean_scrutinee, bound, target);
        let cases = self.cases_on(&scrutinee_ty, bound, motive, zero, &[minor])?;
        let hypothesis = self.b.free();
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
}
