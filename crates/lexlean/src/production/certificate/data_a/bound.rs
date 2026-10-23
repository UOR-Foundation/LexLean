//! Certificate A of a root with a bound (SPEC.md §17.13, §17.17, `SP-29`).
//!
//! The source names the bound `B`, a theorem of the root's module whose
//! statement linking fixed (`ir::semantic::bound`): under `x < 2^64` for each
//! natural parameter, each addition and multiplication of naturals of the
//! body is below 2^64, in evaluation order. The width predicate `__fits_0`
//! of the root is a conjunction, in that same order, of the checks
//! `Nat.blt v 2^64` the primitive lemmas state, under the `let`s of the
//! body. So the proof of `bounded`, that `__fits_0 x = true` under the
//! hypotheses, walks the predicate: `true` by reflexivity, `a && b` through
//! `Bool.and_eq_true_iff` from the proofs of its halves, a `let` by
//! instantiating it, and each check through `Nat.blt_eq` from the next
//! conjunct of `B x h`, projected out of the conjunction. Nothing here
//! decides a range: the kernel checks that each conjunct of the theorem is
//! the proposition the check decides, so a theorem stating another, fewer,
//! or reordered obligations leaves a step ill-typed, which the pinned Lean
//! refuses (`LLV7013`). `root_value` then rewrites `root` with `bounded`:
//! the lowered program converges to the encoded value of the root on every
//! argument in the width.

use super::rows::{Binder, Sig};
use super::{RowKind, Writer, EXPLICIT};
use crate::ir::core::{CoreLevel, CoreNode};
use crate::ir::semantic::SemanticType;
use crate::production::kernel::Term;
use crate::production::source::Local;

impl Writer<'_, '_> {
    /// The rows `bounded` and `root_value` of a root whose report names a
    /// bound; nothing for any other root.
    pub(super) fn bounded_rows(
        &mut self,
        report: &crate::production::RootReport,
        sig: &Sig,
        locals: &[(Local, SemanticType)],
    ) -> Result<(), String> {
        let bound = match &report.bound {
            Some(bound) => bound.clone(),
            None => return Ok(()),
        };
        // A bound covers a root of scalar parameters, where the width
        // predicate and the relation bind exactly one variable per parameter
        // and the same one.
        if sig.fits_binders.len() != locals.len()
            || sig.rel_binders.len() != locals.len()
            || sig
                .fits_binders
                .iter()
                .zip(&sig.rel_binders)
                .any(|(fits, rel)| fits.free != rel.free)
        {
            return Err(format!(
                "the bound `{bound}` is stated for a root of scalar parameters, and the relation of `{}` binds more",
                report.root
            ));
        }
        let width = self.nat_lit(crate::ir::semantic::bound::NAT_WIDTH);
        let lt = self.b.konst_at("LT.lt", vec![CoreLevel::Zero]);
        let nat = self.b.konst("Nat");
        let inst_lt = self.b.konst("instLTNat");
        // One hypothesis per natural parameter, in parameter order: the
        // premises of the theorem, as linking stated them.
        let mut hypotheses: Vec<Binder> = Vec::new();
        for (binder, (local, ty)) in sig.fits_binders.iter().zip(locals) {
            if *ty == SemanticType::Nat {
                let ty = self.b.apps(lt, &[nat, inst_lt, binder.free, width]);
                let free = self.b.free();
                hypotheses.push(Binder {
                    name: format!("__h_{}", local.name),
                    ty,
                    free,
                });
            }
        }
        // `B x₁ … xₙ h₁ … hₖ : C₁ ∧ (C₂ ∧ … ∧ Cₘ)`.
        let theorem = self.b.konst(&bound);
        let mut arguments: Vec<Term> = sig.fits_binders.iter().map(|binder| binder.free).collect();
        arguments.extend(hypotheses.iter().map(|binder| binder.free));
        let applied_bound = self.b.apps(theorem, &arguments);
        // The width predicate of the root, unfolded at the parameters.
        let fits_name = format!("{}.__fits_0", self.name);
        let fits_value = match self.rows.iter().find(|row| row.name == fits_name) {
            Some(row) => row.value,
            None => return Err(format!("`{fits_name}` is not a row of the certificate")),
        };
        let mut predicate = fits_value;
        for binder in &sig.fits_binders {
            predicate = match self.as_lambda_body(predicate) {
                Some(body) => self.b.instantiate(body, binder.free),
                None => {
                    return Err(format!(
                        "`{fits_name}` binds fewer variables than the root has parameters"
                    ))
                }
            };
        }
        // The checks, in evaluation order, and the propositions they decide.
        let mut checks: Vec<(Term, Term)> = Vec::new();
        self.checks_of(predicate, &mut checks)?;
        let count = checks.len();
        if count == 0 {
            return Err(format!(
                "the bound `{bound}` is stated for a root whose width predicate has no check"
            ));
        }
        let propositions: Vec<Term> = checks
            .iter()
            .map(|(value, limit)| self.b.apps(lt, &[nat, inst_lt, *value, *limit]))
            .collect();
        // `Rᵢ = Cᵢ ∧ Rᵢ₊₁`, the conjunction from the i-th conjunct on.
        let and = self.b.konst("And");
        let mut suffixes = propositions.clone();
        for position in (0..count - 1).rev() {
            suffixes[position] = self
                .b
                .apps(and, &[propositions[position], suffixes[position + 1]]);
        }
        // The i-th conjunct, projected out of the applied theorem.
        let left = self.b.konst("And.left");
        let right = self.b.konst("And.right");
        let mut conjuncts = Vec::with_capacity(count);
        let mut rest = applied_bound;
        for position in 0..count {
            if position + 1 == count {
                conjuncts.push(rest);
            } else {
                let here = [propositions[position], suffixes[position + 1], rest];
                conjuncts.push(self.b.apps(left, &here));
                rest = self.b.apps(right, &here);
            }
        }
        let mut next = 0usize;
        let proof = self.predicate_true(predicate, &conjuncts, &mut next)?;
        if next != count {
            return Err(format!(
                "the width predicate of `{}` has {count} checks and its proof consumed {next}",
                report.root
            ));
        }
        let bool_ty = self.b.konst("Bool");
        let tru = self.tru();
        let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
        let eq_bool = self.b.konst_at("Eq", vec![one.clone()]);
        let bounded_ty = self.b.apps(eq_bool, &[bool_ty, sig.fits_applied, tru]);
        let mut binders = sig.fits_binders.clone();
        binders.extend(hypotheses.iter().cloned());
        let bounded_pi = self.pis(&binders, bounded_ty);
        let bounded_lam = self.lambdas(&binders, proof);
        self.push_row("bounded", RowKind::Theorem, bounded_pi, bounded_lam)?;
        // `root_value`: `root` with the predicate rewritten to `true`, whose
        // `Rel true v` is `Obs.value v` by unfolding.
        let bounded_head = self.own("bounded");
        let applied_bounded = self.b.apps(bounded_head, &arguments);
        let run_conv = self.lp("RunConv");
        let rel = self.lp("Rel");
        let prog = self.prog;
        let entry = self.nat_u(0);
        let rewritten = self.b.free();
        let rel_rewritten = self.b.apps(rel, &[rewritten, sig.value]);
        let motive_body = self
            .b
            .apps(run_conv, &[prog, entry, sig.arguments, rel_rewritten]);
        let motive = self
            .b
            .lam_over("__b", EXPLICIT, bool_ty, rewritten, motive_body);
        let prop = self.b.prop();
        let congr = self.b.konst_at("congrArg", vec![one.clone(), one]);
        let equal_observations = self.b.apps(
            congr,
            &[
                bool_ty,
                prop,
                sig.fits_applied,
                tru,
                motive,
                applied_bounded,
            ],
        );
        let rel_fits = self.rel(sig.fits_applied, sig.value);
        let stated = self
            .b
            .apps(run_conv, &[prog, entry, sig.arguments, rel_fits]);
        let rel_true = self.rel(tru, sig.value);
        let rewritten_ty = self
            .b
            .apps(run_conv, &[prog, entry, sig.arguments, rel_true]);
        let eq_mp = self.b.konst_at("Eq.mp", vec![CoreLevel::Zero]);
        let root_head = self.own("root");
        let applied_root = self.b.apps(root_head, &sig.rel_params);
        let value = self.b.apps(
            eq_mp,
            &[stated, rewritten_ty, equal_observations, applied_root],
        );
        let obs_value = self.lp("Obs.value");
        let valued = self.b.app(obs_value, sig.value);
        let ty = self.b.apps(run_conv, &[prog, entry, sig.arguments, valued]);
        let mut rel_binders = sig.rel_binders.clone();
        rel_binders.extend(hypotheses);
        let pi = self.pis(&rel_binders, ty);
        let lam = self.lambdas(&rel_binders, value);
        self.push_row("root_value", RowKind::Theorem, pi, lam)
    }

    /// The checks `Nat.blt v w` of a width predicate, in evaluation order,
    /// read through `&&`, `true`, and `let`.
    fn checks_of(&mut self, predicate: Term, out: &mut Vec<(Term, Term)>) -> Result<(), String> {
        match self.as_let(predicate) {
            Some((value, body)) => {
                let inner = self.b.instantiate(body, value);
                return self.checks_of(inner, out);
            }
            None => {}
        }
        let (head, args) = self.b.spine(predicate);
        match (self.head_name(head).as_str(), args.as_slice()) {
            ("Bool.true", []) => Ok(()),
            ("Bool.and" | "and", [a, b]) => {
                let (a, b) = (*a, *b);
                self.checks_of(a, out)?;
                self.checks_of(b, out)
            }
            ("Nat.blt", [value, limit]) => {
                out.push((*value, *limit));
                Ok(())
            }
            (_, _) => Err(self.uncovered_predicate(predicate)),
        }
    }

    /// A proof of `predicate = true` from the proofs of the propositions its
    /// checks decide, consumed in evaluation order from `next`.
    fn predicate_true(
        &mut self,
        predicate: Term,
        conjuncts: &[Term],
        next: &mut usize,
    ) -> Result<Term, String> {
        match self.as_let(predicate) {
            Some((value, body)) => {
                let inner = self.b.instantiate(body, value);
                return self.predicate_true(inner, conjuncts, next);
            }
            None => {}
        }
        let bool_ty = self.b.konst("Bool");
        let tru = self.tru();
        let one = CoreLevel::Succ(Box::new(CoreLevel::Zero));
        let eq_bool = self.b.konst_at("Eq", vec![one.clone()]);
        let (head, args) = self.b.spine(predicate);
        match (self.head_name(head).as_str(), args.as_slice()) {
            ("Bool.true", []) => {
                let refl = self.b.konst_at("Eq.refl", vec![one]);
                Ok(self.b.apps(refl, &[bool_ty, tru]))
            }
            ("Bool.and" | "and", [a, b]) => {
                let (a, b) = (*a, *b);
                let proof_a = self.predicate_true(a, conjuncts, next)?;
                let proof_b = self.predicate_true(b, conjuncts, next)?;
                let a_true = self.b.apps(eq_bool, &[bool_ty, a, tru]);
                let b_true = self.b.apps(eq_bool, &[bool_ty, b, tru]);
                let and_prop = self.b.konst("And");
                let both = self.b.apps(and_prop, &[a_true, b_true]);
                let intro = self.b.konst("And.intro");
                let both_proof = self.b.apps(intro, &[a_true, b_true, proof_a, proof_b]);
                let whole = self.b.apps(eq_bool, &[bool_ty, predicate, tru]);
                let iff = self.b.konst("Bool.and_eq_true_iff");
                let iff = self.b.apps(iff, &[a, b]);
                let mpr = self.b.konst("Iff.mpr");
                Ok(self.b.apps(mpr, &[whole, both, iff, both_proof]))
            }
            ("Nat.blt", [value, limit]) => {
                let (value, limit) = (*value, *limit);
                let conjunct = match conjuncts.get(*next) {
                    Some(conjunct) => *conjunct,
                    None => {
                        return Err(
                            "the width predicate has more checks than the bound states".to_owned()
                        )
                    }
                };
                *next += 1;
                let lt = self.b.konst_at("LT.lt", vec![CoreLevel::Zero]);
                let nat = self.b.konst("Nat");
                let inst_lt = self.b.konst("instLTNat");
                let proposition = self.b.apps(lt, &[nat, inst_lt, value, limit]);
                let decided = self.b.apps(eq_bool, &[bool_ty, predicate, tru]);
                let blt_eq = self.b.konst("Nat.blt_eq");
                let blt_eq = self.b.apps(blt_eq, &[value, limit]);
                let mpr = self.b.konst_at("Eq.mpr", vec![CoreLevel::Zero]);
                Ok(self.b.apps(mpr, &[decided, proposition, blt_eq, conjunct]))
            }
            (_, _) => Err(self.uncovered_predicate(predicate)),
        }
    }

    /// The value and body of a `let` node.
    fn as_let(&self, term: Term) -> Option<(Term, Term)> {
        match self.b.node(term) {
            CoreNode::Let {
                n: _,
                t: _,
                v,
                body,
                d: _,
            } => Some((*v, *body)),
            CoreNode::BVar { i: _ }
            | CoreNode::Sort { l: _ }
            | CoreNode::Const { n: _, u: _ }
            | CoreNode::App { f: _, x: _ }
            | CoreNode::Lambda {
                n: _,
                b: _,
                t: _,
                v: _,
            }
            | CoreNode::Forall {
                n: _,
                b: _,
                t: _,
                v: _,
            }
            | CoreNode::Nat { v: _ }
            | CoreNode::String { v: _ }
            | CoreNode::Mdata { m: _, x: _ }
            | CoreNode::Projection {
                n: _,
                s: _,
                i: _,
                x: _,
            } => None,
        }
    }

    /// The body of a lambda node.
    fn as_lambda_body(&self, term: Term) -> Option<Term> {
        match self.b.node(term) {
            CoreNode::Lambda {
                n: _,
                b: _,
                t: _,
                v,
            } => Some(*v),
            CoreNode::BVar { i: _ }
            | CoreNode::Sort { l: _ }
            | CoreNode::Const { n: _, u: _ }
            | CoreNode::App { f: _, x: _ }
            | CoreNode::Forall {
                n: _,
                b: _,
                t: _,
                v: _,
            }
            | CoreNode::Let {
                n: _,
                t: _,
                v: _,
                body: _,
                d: _,
            }
            | CoreNode::Nat { v: _ }
            | CoreNode::String { v: _ }
            | CoreNode::Mdata { m: _, x: _ }
            | CoreNode::Projection {
                n: _,
                s: _,
                i: _,
                x: _,
            } => None,
        }
    }

    /// The name of a constant node, or nothing for any other node.
    fn head_name(&self, head: Term) -> String {
        match self.b.node(head) {
            CoreNode::Const { n, u: _ } => n.clone(),
            CoreNode::BVar { i: _ }
            | CoreNode::Sort { l: _ }
            | CoreNode::App { f: _, x: _ }
            | CoreNode::Lambda {
                n: _,
                b: _,
                t: _,
                v: _,
            }
            | CoreNode::Forall {
                n: _,
                b: _,
                t: _,
                v: _,
            }
            | CoreNode::Let {
                n: _,
                t: _,
                v: _,
                body: _,
                d: _,
            }
            | CoreNode::Nat { v: _ }
            | CoreNode::String { v: _ }
            | CoreNode::Mdata { m: _, x: _ }
            | CoreNode::Projection {
                n: _,
                s: _,
                i: _,
                x: _,
            } => String::new(),
        }
    }

    fn uncovered_predicate(&self, predicate: Term) -> String {
        format!(
            "the width predicate has a form the bound rule does not cover: {}",
            self.b.show(predicate)
        )
    }
}
