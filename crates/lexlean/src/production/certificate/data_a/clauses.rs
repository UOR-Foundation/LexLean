//! The collections runtime as kernel terms (SPEC.md §17.17.3): its constants
//! and key instances, the denotation of a collection primitive, and the
//! clause hypotheses of the library's templates, each discharged by a proof
//! term.
//!
//! A clause states what one runtime function does when a comparison came out
//! one way: `∀ binders, compare k o = c → f args = rhs`. The runtime
//! function's body is a match on the comparison, so the statement follows by
//! rewriting the comparison in the one equation `F (compare k o) = F (compare
//! k o)`, where `F` is the function with the scrutinee abstracted. `F` is
//! written here with the eliminator of the scrutinee's type, or with the
//! runtime's own matcher where Lean compiled the match to a sparse eliminator,
//! whose branches an eliminator written by hand is not definitionally equal
//! to on an unknown scrutinee. The kernel then unfolds the runtime function
//! and takes the eliminator's step at the hypothesis's constructor.

use super::{Pf, Writer, EXPLICIT};
use crate::ir::core::{CoreBinderInfo, CoreLevel, CoreNode};
use crate::ir::semantic::SemanticType;
use crate::production::kernel::{Given, Term};
use crate::production::source::Site;

/// The universe level 1, of the types the runtime's functions return.
fn one() -> CoreLevel {
    CoreLevel::Succ(Box::new(CoreLevel::Zero))
}

/// An argument of a library lemma: a given one, or a clause proof over the
/// number of binders the clause quantifies before its hypothesis.
#[derive(Clone, Copy)]
pub(super) enum Arg {
    Pass(Given),
    Clause(usize, Form),
}

impl Arg {
    pub(super) fn term(term: Term) -> Self {
        Self::Pass(Given::Term(term))
    }

    pub(super) fn proof(pf: Pf) -> Self {
        Self::Pass(Given::Proof(pf.proof, pf.ty))
    }

    pub(super) const fn refl() -> Self {
        Self::Pass(Given::Refl)
    }

    /// `count` hypotheses discharged by computation.
    pub(super) fn refls(count: usize) -> Vec<Self> {
        vec![Self::refl(); count]
    }
}

/// The function a clause is about, with the terms its right sides name.
#[derive(Clone, Copy)]
pub(super) enum Form {
    /// `insertEntry κ ν inst`; binders key, value, other, stored, rest.
    InsertEntry {
        kappa: Term,
        nu: Term,
        function: Term,
    },
    /// `removeEntry κ ν inst`; binders key, other, stored, rest.
    RemoveEntry {
        kappa: Term,
        nu: Term,
        function: Term,
    },
    /// `lookupEntry κ ν inst`; binders key, other, stored, rest.
    LookupEntry { nu: Term, function: Term },
    /// `insertElement κ inst`; binders key, other, rest.
    InsertElement { kappa: Term, function: Term },
    /// `removeElement κ inst`; binders key, other, rest.
    RemoveElement { kappa: Term, function: Term },
    /// `containsElement κ inst`; binders key, other, rest.
    ContainsElement { function: Term },
    /// `compareCodes`; binders left, lefts, right, rights. The matcher is the
    /// runtime's `compareCodes.match_1`.
    CompareCodes { matcher: Term, function: Term },
    /// The pair key order; binders left, right. `second` compares the second
    /// components.
    PairKey {
        matcher: Term,
        alpha: Term,
        beta: Term,
        second: Term,
    },
    /// `lessThan`; binders left, right.
    LessThan { matcher: Term },
    /// `reachableFrom κ inst graph`; binders fuel, frontier, seen, and for a
    /// non-empty round also the head and tail. `insert` is `insertElement κ
    /// inst`.
    Reach {
        matcher: Term,
        kappa: Term,
        reach: Term,
        insert: Term,
    },
    /// `topological κ inst graph` at a frontier that is stuck; binders fuel,
    /// head, tail, order.
    TopoStuck {
        kappa: Term,
        topo: Term,
        remove: Term,
    },
    /// `topological κ inst graph` at a round that has a ready node; binders
    /// fuel, remaining, order, ready, rest.
    TopoNext {
        kappa: Term,
        topo: Term,
        remove: Term,
    },
    /// `iterateUntil σ step`; binders fuel, state, and for a step that
    /// continues also the next state.
    IterateUntil { sigma: Term, iterate: Term },
}

impl Writer<'_, '_> {
    // --- Constants --------------------------------------------------------

    /// The name of a declaration of the collections runtime of the module
    /// `site` is read in.
    pub(super) fn collections_name(&self, site: &Site, name: &str) -> Result<String, String> {
        let _ = site;
        Ok(format!("LexLeanCollections.{name}"))
    }

    /// A declaration of the collections runtime.
    pub(super) fn collections(&mut self, site: &Site, name: &str) -> Result<Term, String> {
        let full = self.collections_name(site, name)?;
        Ok(self.b.konst(&full))
    }

    /// A matcher of the collections runtime, which is universe polymorphic in
    /// its motive; the motives here return types in `Type`.
    fn matcher(&mut self, site: &Site, name: &str) -> Result<Term, String> {
        let full = self.collections_name(site, name)?;
        Ok(self.b.konst_at(&full, vec![one()]))
    }

    /// The runtime's `compareCodes.match_1`, its `lessThan.match_1`, and its
    /// `reachableFrom.match_1`.
    fn code_matcher(&mut self, site: &Site) -> Result<Term, String> {
        self.matcher(site, "compareCodes.match_1")
    }

    pub(super) fn less_matcher(&mut self, site: &Site) -> Result<Term, String> {
        self.matcher(site, "lessThan.match_1")
    }

    pub(super) fn reach_matcher(&mut self, site: &Site) -> Result<Term, String> {
        self.matcher(site, "reachableFrom.match_1")
    }

    // --- Key orders -------------------------------------------------------

    /// The `Key` instance of a key type, as the pinned Lean selects it.
    pub(super) fn key_instance(&mut self, site: &Site, ty: &SemanticType) -> Result<Term, String> {
        let name = match ty {
            SemanticType::Nat => "instKeyNat",
            SemanticType::Int => "instKeyInt",
            SemanticType::Bool => "instKeyBool",
            SemanticType::Int8 => "instKeyInt8",
            SemanticType::Int16 => "instKeyInt16",
            SemanticType::Int32 => "instKeyInt32",
            SemanticType::Int64 => "instKeyInt64",
            SemanticType::UInt8 => "instKeyUInt8",
            SemanticType::UInt16 => "instKeyUInt16",
            SemanticType::UInt32 => "instKeyUInt32",
            SemanticType::UInt64 => "instKeyUInt64",
            SemanticType::String => "instKeyString",
            SemanticType::Product { left, right } => {
                let alpha = self.lean_ty(left)?;
                let beta = self.lean_ty(right)?;
                let first = self.key_instance(site, left)?;
                let second = self.key_instance(site, right)?;
                let head = self.collections(site, "instKeyProd")?;
                return Ok(self.b.apps(head, &[alpha, beta, first, second]));
            }
            SemanticType::Type
            | SemanticType::Parameter { name: _ }
            | SemanticType::Prop
            | SemanticType::Unit
            | SemanticType::Bytes
            | SemanticType::Ordering
            | SemanticType::Option { value: _ }
            | SemanticType::Result { ok: _, error: _ }
            | SemanticType::List { element: _ }
            | SemanticType::Named {
                member: _,
                arguments: _,
            }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::Set { element: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {
                return Err(format!(
                    "`{}` has no key order",
                    self.gen.source.type_text(ty)
                ));
            }
        };
        self.collections(site, name)
    }

    /// The source key order of `ty`: the runtime's `Key.compare` at its
    /// instance.
    pub(super) fn key_order(&mut self, site: &Site, ty: &SemanticType) -> Result<Term, String> {
        let lean = self.lean_ty(ty)?;
        let instance = self.key_instance(site, ty)?;
        let compare = self.collections(site, "Key.compare")?;
        Ok(self.b.apps(compare, &[lean, instance]))
    }

    /// A runtime function at the type arguments and instance the elaborator
    /// gives it.
    pub(super) fn collections_at(
        &mut self,
        site: &Site,
        name: &str,
        leading: &[Term],
    ) -> Result<Term, String> {
        let head = self.collections(site, name)?;
        Ok(self.b.apps(head, leading))
    }

    /// The proof that the realization's comparison agrees with the key order
    /// of `ty`.
    pub(super) fn key_spec(&mut self, site: &Site, ty: &SemanticType) -> Result<Pf, String> {
        let order = self.key_order(site, ty)?;
        let scalar = match ty {
            SemanticType::Nat => Some("keySpec_nat"),
            SemanticType::Int => Some("keySpec_int"),
            SemanticType::Bool => Some("keySpec_bool"),
            SemanticType::Int8 => Some("keySpec_i8"),
            SemanticType::Int16 => Some("keySpec_i16"),
            SemanticType::Int32 => Some("keySpec_i32"),
            SemanticType::Int64 => Some("keySpec_i64"),
            SemanticType::UInt8 => Some("keySpec_u8"),
            SemanticType::UInt16 => Some("keySpec_u16"),
            SemanticType::UInt32 => Some("keySpec_u32"),
            SemanticType::UInt64 => Some("keySpec_u64"),
            SemanticType::Type
            | SemanticType::Parameter { name: _ }
            | SemanticType::Prop
            | SemanticType::Unit
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
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::Set { element: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => None,
        };
        if let Some(lemma) = scalar {
            return self.lemma_args(lemma, &[], &[Arg::term(order), Arg::refl()]);
        }
        match ty {
            SemanticType::String => {
                let codes = self.collections(site, "compareCodes")?;
                let matcher = self.code_matcher(site)?;
                let clause = Arg::Clause(
                    4,
                    Form::CompareCodes {
                        matcher,
                        function: codes,
                    },
                );
                self.lemma_args(
                    "keySpec_string",
                    &[],
                    &[
                        Arg::term(codes),
                        Arg::refl(),
                        Arg::refl(),
                        Arg::refl(),
                        clause,
                        clause,
                        clause,
                        Arg::term(order),
                        Arg::refl(),
                    ],
                )
            }
            SemanticType::Product { left, right } => {
                let first = self.key_spec(site, left)?;
                let second = self.key_spec(site, right)?;
                let alpha = self.lean_ty(left)?;
                let beta = self.lean_ty(right)?;
                let compare = self.key_order(site, right)?;
                let matcher = self.code_matcher(site)?;
                let clause = Arg::Clause(
                    2,
                    Form::PairKey {
                        matcher,
                        alpha,
                        beta,
                        second: compare,
                    },
                );
                self.lemma_args(
                    "keySpec_pair",
                    &[],
                    &[
                        Arg::proof(first),
                        Arg::proof(second),
                        Arg::term(order),
                        clause,
                        clause,
                        clause,
                    ],
                )
            }
            SemanticType::Nat
            | SemanticType::Int
            | SemanticType::Bool
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64
            | SemanticType::Type
            | SemanticType::Parameter { name: _ }
            | SemanticType::Prop
            | SemanticType::Unit
            | SemanticType::Bytes
            | SemanticType::Ordering
            | SemanticType::Option { value: _ }
            | SemanticType::Result { ok: _, error: _ }
            | SemanticType::List { element: _ }
            | SemanticType::Named {
                member: _,
                arguments: _,
            }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::Set { element: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => Err(format!(
                "`{}` has no key order",
                self.gen.source.type_text(ty)
            )),
        }
    }

    // --- Operation records of the library ---------------------------------

    /// The set-runtime clauses of key type `ty`: `SetOps` at the runtime's
    /// insertion, removal, and membership.
    pub(super) fn set_ops(&mut self, site: &Site, ty: &SemanticType) -> Result<Pf, String> {
        let kappa = self.lean_ty(ty)?;
        let instance = self.key_instance(site, ty)?;
        let order = self.key_order(site, ty)?;
        let insert = self.collections_at(site, "insertElement", &[kappa, instance])?;
        let remove = self.collections_at(site, "removeElement", &[kappa, instance])?;
        let contains = self.collections_at(site, "containsElement", &[kappa, instance])?;
        let insertion = Form::InsertElement {
            kappa,
            function: insert,
        };
        let removal = Form::RemoveElement {
            kappa,
            function: remove,
        };
        let membership = Form::ContainsElement { function: contains };
        let mut args = vec![Arg::refl()];
        args.extend(vec![Arg::Clause(3, insertion); 3]);
        args.push(Arg::refl());
        args.extend(vec![Arg::Clause(3, removal); 3]);
        args.push(Arg::refl());
        args.extend(vec![Arg::Clause(3, membership); 3]);
        self.lemma_args(
            "SetOps.mk",
            &[
                ("κ", kappa),
                ("cmp", order),
                ("ins", insert),
                ("rem", remove),
                ("con", contains),
            ],
            &args,
        )
    }

    /// The lookup-runtime clauses of key type `key` and value type `value`.
    pub(super) fn look_ops(
        &mut self,
        site: &Site,
        key: &SemanticType,
        value: &SemanticType,
    ) -> Result<Pf, String> {
        let kappa = self.lean_ty(key)?;
        let nu = self.lean_ty(value)?;
        let instance = self.key_instance(site, key)?;
        let order = self.key_order(site, key)?;
        let lookup = self.collections_at(site, "lookupEntry", &[kappa, nu, instance])?;
        let form = Form::LookupEntry {
            nu,
            function: lookup,
        };
        let mut args = vec![Arg::refl()];
        args.extend(vec![Arg::Clause(4, form); 3]);
        self.lemma_args(
            "LookOps.mk",
            &[("κ", kappa), ("ν", nu), ("cmp", order), ("look", lookup)],
            &args,
        )
    }

    /// The topological-sort clauses of key type `ty` on a graph `graph`.
    pub(super) fn topo_ops(
        &mut self,
        site: &Site,
        ty: &SemanticType,
        graph: Term,
    ) -> Result<Pf, String> {
        let kappa = self.lean_ty(ty)?;
        let instance = self.key_instance(site, ty)?;
        let successors = self.collections_at(site, "graphSuccessors", &[kappa, instance, graph])?;
        let contains = self.collections_at(site, "containsElement", &[kappa, instance])?;
        let remove = self.collections_at(site, "removeElement", &[kappa, instance])?;
        let topo = self.collections_at(site, "topological", &[kappa, instance, graph])?;
        let stuck = Form::TopoStuck {
            kappa,
            topo,
            remove,
        };
        let next = Form::TopoNext {
            kappa,
            topo,
            remove,
        };
        self.lemma_args(
            "TopoOps.mk",
            &[
                ("κ", kappa),
                ("sx", successors),
                ("con", contains),
                ("rem", remove),
                ("topo", topo),
            ],
            &[
                Arg::refl(),
                Arg::refl(),
                Arg::refl(),
                Arg::Clause(4, stuck),
                Arg::Clause(5, next),
            ],
        )
    }

    // --- Applying a lemma with clause arguments ---------------------------

    /// A library lemma applied to arguments, a clause argument proved from the
    /// statement its binder carries once the arguments before it are known.
    pub(super) fn lemma_args(
        &mut self,
        name: &str,
        hints: &[(&str, Term)],
        args: &[Arg],
    ) -> Result<Pf, String> {
        let mut open = self.open(name, hints)?;
        for arg in args {
            match *arg {
                Arg::Pass(given) => open.give(&mut self.b, given)?,
                Arg::Clause(binders, form) => {
                    let domain = open.domain(&mut self.b);
                    let proof = self
                        .clause_proof(domain, binders, form)
                        .map_err(|error| format!("a clause of `{name}`: {error}"))?;
                    open.give(&mut self.b, Given::Term(proof))?;
                }
            }
        }
        let applied = open.finish(&mut self.b)?;
        Ok(Pf {
            proof: applied.term,
            ty: applied.ty,
        })
    }

    /// The proof of a clause statement `domain`: `binders` quantifiers, then
    /// the hypothesis `scrutinee = case`, then the equation `lhs = rhs`.
    ///
    /// The proof is `fun .. h => Eq.rec (F scrutinee = F scrutinee) h`, whose
    /// type `F scrutinee = F case` the kernel finds to be the statement: `F
    /// scrutinee` is the left side by unfolding the runtime function, and `F
    /// case` the right side by the eliminator's step on a constructor.
    fn clause_proof(&mut self, domain: Term, binders: usize, form: Form) -> Result<Term, String> {
        let mut opened: Vec<(String, CoreBinderInfo, Term, Term)> = Vec::new();
        let mut current = domain;
        for _ in 0..=binders {
            let (name, info, ty, body) = match self.b.node(current).clone() {
                CoreNode::Forall { n, b, t, v } => (n, b, t, v),
                CoreNode::BVar { .. }
                | CoreNode::Sort { .. }
                | CoreNode::Const { .. }
                | CoreNode::App { .. }
                | CoreNode::Lambda { .. }
                | CoreNode::Let { .. }
                | CoreNode::Mdata { .. }
                | CoreNode::Projection { .. }
                | CoreNode::Nat { .. }
                | CoreNode::String { .. } => {
                    return Err(format!(
                        "the statement {} quantifies fewer than {binders} variables and a hypothesis",
                        self.b.show(domain)
                    ));
                }
            };
            let free = self.b.free();
            current = self.b.instantiate_beta(body, free);
            opened.push((name, info, ty, free));
        }
        let (hypothesis_name, hypothesis_info, hypothesis, hypothesis_free) =
            opened.pop().ok_or("a clause without a hypothesis")?;
        let frees: Vec<Term> = opened.iter().map(|(_, _, _, free)| *free).collect();
        let (a_ty, scrutinee, case, hypothesis_levels) = self.equation(hypothesis)?;
        let (result_ty, _, _, conclusion_levels) = self.equation(current)?;
        let at_scrutinee = self.form_term(form, &frees, scrutinee)?;
        let variable = self.b.free();
        let transport = self.b.free();
        let at_variable = self.form_term(form, &frees, variable)?;
        let eq_head = self.b.konst_at("Eq", conclusion_levels.clone());
        let motive_body = self
            .b
            .apps(eq_head, &[result_ty, at_scrutinee, at_variable]);
        let hyp_head = self.b.konst_at("Eq", hypothesis_levels.clone());
        let hyp_at = self.b.apps(hyp_head, &[a_ty, scrutinee, variable]);
        let inner = self
            .b
            .lam_over("_", EXPLICIT, hyp_at, transport, motive_body);
        let motive = self.b.lam_over("c", EXPLICIT, a_ty, variable, inner);
        let refl = self.b.konst_at("Eq.refl", conclusion_levels);
        let refl = self.b.apps(refl, &[result_ty, at_scrutinee]);
        let sort = hypothesis_levels
            .first()
            .cloned()
            .ok_or("an equation without a universe")?;
        let rec = self.b.konst_at("Eq.rec", vec![CoreLevel::Zero, sort]);
        let mut term = self
            .b
            .apps(rec, &[a_ty, scrutinee, motive, refl, case, hypothesis_free]);
        term = self.b.lam_over(
            &hypothesis_name,
            hypothesis_info,
            hypothesis,
            hypothesis_free,
            term,
        );
        for (name, info, ty, free) in opened.into_iter().rev() {
            term = self.b.lam_over(&name, info, ty, free, term);
        }
        Ok(term)
    }

    /// The type, sides, and universe of an equation.
    fn equation(&self, equation: Term) -> Result<(Term, Term, Term, Vec<CoreLevel>), String> {
        let (head, parts) = self.b.spine(equation);
        match self.b.node(head) {
            CoreNode::Const { n, u } if n == "Eq" && parts.len() == 3 => {
                Ok((parts[0], parts[1], parts[2], u.clone()))
            }
            CoreNode::BVar { .. }
            | CoreNode::Sort { .. }
            | CoreNode::Const { .. }
            | CoreNode::App { .. }
            | CoreNode::Lambda { .. }
            | CoreNode::Forall { .. }
            | CoreNode::Let { .. }
            | CoreNode::Mdata { .. }
            | CoreNode::Projection { .. }
            | CoreNode::Nat { .. }
            | CoreNode::String { .. } => {
                Err(format!("{} is not an equation", self.b.show(equation)))
            }
        }
    }

    // --- The abstracted bodies -------------------------------------------

    fn cons(&mut self, element: Term, head: Term, tail: Term) -> Term {
        let cons = self.b.konst0("List.cons");
        self.b.apps(cons, &[element, head, tail])
    }

    /// `fun (name : ty) => body` over the free variable `free`.
    fn lam1(&mut self, name: &str, ty: Term, free: Term, body: Term) -> Term {
        self.b.lam_over(name, EXPLICIT, ty, free, body)
    }

    /// `fun (_ : Unit) => value`.
    fn thunk(&mut self, value: Term) -> Term {
        let unit = self.b.konst("Unit");
        self.b.lam("_", EXPLICIT, unit, value)
    }

    /// The eliminator of `Ordering` at the constant motive `result`.
    fn ordering_cases(&mut self, result: Term, scrutinee: Term, arms: [Term; 3]) -> Term {
        let ordering = self.b.konst("Ordering");
        let lifted = self.b.lift(result, 1, 0);
        let motive = self.b.lam("_", EXPLICIT, ordering, lifted);
        let head = self.b.konst_at("Ordering.casesOn", vec![one()]);
        self.b
            .apps(head, &[motive, scrutinee, arms[0], arms[1], arms[2]])
    }

    /// The eliminator of `List element` at the constant motive `result`.
    fn list_cases(
        &mut self,
        element: Term,
        result: Term,
        scrutinee: Term,
        nil: Term,
        cons: Term,
    ) -> Term {
        let list = self.b.list_ty(element);
        let lifted = self.b.lift(result, 1, 0);
        let motive = self.b.lam("_", EXPLICIT, list, lifted);
        let head = self
            .b
            .konst_at("List.casesOn", vec![one(), CoreLevel::Zero]);
        self.b.apps(head, &[element, motive, scrutinee, nil, cons])
    }

    /// The eliminator of `Option value` at the constant motive `result`.
    fn option_cases(
        &mut self,
        value: Term,
        result: Term,
        scrutinee: Term,
        none: Term,
        some: Term,
    ) -> Term {
        let option = self.b.konst0("Option");
        let option = self.b.app(option, value);
        let lifted = self.b.lift(result, 1, 0);
        let motive = self.b.lam("_", EXPLICIT, option, lifted);
        let head = self
            .b
            .konst_at("Option.casesOn", vec![one(), CoreLevel::Zero]);
        self.b.apps(head, &[value, motive, scrutinee, none, some])
    }

    fn none(&mut self, value: Term) -> Term {
        let none = self.b.konst0("Option.none");
        self.b.app(none, value)
    }

    fn some(&mut self, value: Term, item: Term) -> Term {
        let some = self.b.konst0("Option.some");
        self.b.apps(some, &[value, item])
    }

    fn projection_of(&mut self, name: &str, alpha: Term, beta: Term, pair: Term) -> Term {
        let head = self.b.konst00(name);
        self.b.apps(head, &[alpha, beta, pair])
    }

    /// `if remaining.isEmpty then some order.reverse else none`, as the
    /// runtime's `topological` writes it.
    fn topo_empty(&mut self, kappa: Term, remaining: Term, order: Term) -> Term {
        let is_empty = self.b.konst0("List.isEmpty");
        let empty = self.b.apps(is_empty, &[kappa, remaining]);
        let reverse = self.b.konst0("List.reverse");
        let reversed = self.b.apps(reverse, &[kappa, order]);
        let list = self.b.list_ty(kappa);
        let found = self.some(list, reversed);
        let missing = self.none(list);
        let result = self.option_of(list);
        self.ite(result, empty, found, missing)
    }

    fn option_of(&mut self, value: Term) -> Term {
        let option = self.b.konst0("Option");
        self.b.app(option, value)
    }

    /// The function `F` of a clause at the scrutinee `scrutinee`, over the
    /// clause's quantified variables `frees`.
    #[allow(clippy::too_many_lines)]
    fn form_term(&mut self, form: Form, frees: &[Term], scrutinee: Term) -> Result<Term, String> {
        let at = |position: usize| -> Result<Term, String> {
            frees.get(position).copied().ok_or_else(|| {
                format!(
                    "a clause quantifies {} variables, not {}",
                    frees.len(),
                    position + 1
                )
            })
        };
        match form {
            Form::InsertEntry {
                kappa,
                nu,
                function,
            } => {
                let (key, value, other, stored, rest) = (at(0)?, at(1)?, at(2)?, at(3)?, at(4)?);
                let entry = self.b.pair_ty(kappa, nu);
                let new = self.b.pair(kappa, nu, key, value);
                let old = self.b.pair(kappa, nu, other, stored);
                let tail = self.cons(entry, old, rest);
                let before = self.cons(entry, new, tail);
                let replaced = self.cons(entry, new, rest);
                let recursive = self.b.apps(function, &[key, value, rest]);
                let after = self.cons(entry, old, recursive);
                let result = self.b.list_ty(entry);
                Ok(self.ordering_cases(result, scrutinee, [before, replaced, after]))
            }
            Form::RemoveEntry {
                kappa,
                nu,
                function,
            } => {
                let (key, other, stored, rest) = (at(0)?, at(1)?, at(2)?, at(3)?);
                let entry = self.b.pair_ty(kappa, nu);
                let old = self.b.pair(kappa, nu, other, stored);
                let kept = self.cons(entry, old, rest);
                let recursive = self.b.apps(function, &[key, rest]);
                let after = self.cons(entry, old, recursive);
                let result = self.b.list_ty(entry);
                Ok(self.ordering_cases(result, scrutinee, [kept, rest, after]))
            }
            Form::LookupEntry { nu, function } => {
                let (key, stored, rest) = (at(0)?, at(2)?, at(3)?);
                let missing = self.none(nu);
                let found = self.some(nu, stored);
                let recursive = self.b.apps(function, &[key, rest]);
                let result = self.option_of(nu);
                Ok(self.ordering_cases(result, scrutinee, [missing, found, recursive]))
            }
            Form::InsertElement { kappa, function } => {
                let (key, other, rest) = (at(0)?, at(1)?, at(2)?);
                let tail = self.cons(kappa, other, rest);
                let before = self.cons(kappa, key, tail);
                let kept = self.cons(kappa, other, rest);
                let recursive = self.b.apps(function, &[key, rest]);
                let after = self.cons(kappa, other, recursive);
                let result = self.b.list_ty(kappa);
                Ok(self.ordering_cases(result, scrutinee, [before, kept, after]))
            }
            Form::RemoveElement { kappa, function } => {
                let (key, other, rest) = (at(0)?, at(1)?, at(2)?);
                let kept = self.cons(kappa, other, rest);
                let recursive = self.b.apps(function, &[key, rest]);
                let after = self.cons(kappa, other, recursive);
                let result = self.b.list_ty(kappa);
                Ok(self.ordering_cases(result, scrutinee, [kept, rest, after]))
            }
            Form::ContainsElement { function } => {
                let (key, rest) = (at(0)?, at(2)?);
                let no = self.fls();
                let yes = self.tru();
                let recursive = self.b.apps(function, &[key, rest]);
                let result = self.b.konst("Bool");
                Ok(self.ordering_cases(result, scrutinee, [no, yes, recursive]))
            }
            Form::CompareCodes { matcher, function } => {
                let (lefts, rights) = (at(1)?, at(3)?);
                let recursive = self.b.apps(function, &[lefts, rights]);
                let result = self.b.konst("Ordering");
                Ok(self.sparse_ordering(matcher, result, scrutinee, recursive, None))
            }
            Form::PairKey {
                matcher,
                alpha,
                beta,
                second,
            } => {
                let (left, right) = (at(0)?, at(1)?);
                let left_second = self.projection_of("Prod.snd", alpha, beta, left);
                let right_second = self.projection_of("Prod.snd", alpha, beta, right);
                let compared = self.b.apps(second, &[left_second, right_second]);
                let result = self.b.konst("Ordering");
                Ok(self.sparse_ordering(matcher, result, scrutinee, compared, None))
            }
            Form::LessThan { matcher } => {
                let yes = self.tru();
                let no = self.fls();
                let result = self.b.konst("Bool");
                Ok(self.sparse_ordering(matcher, result, scrutinee, yes, Some(no)))
            }
            Form::Reach {
                matcher,
                kappa,
                reach,
                insert,
            } => {
                let (fuel, seen) = (at(0)?, at(2)?);
                let list = self.b.list_ty(kappa);
                let stay = self.thunk(seen);
                let next = self.b.free();
                let folded = self.fold_insert(kappa, insert, seen, next);
                let recursive = self.b.apps(reach, &[fuel, next, folded]);
                let advance = self.lam1("next", list, next, recursive);
                let motive = self.constant_motive(list, list);
                Ok(self
                    .b
                    .apps(matcher, &[kappa, motive, scrutinee, stay, advance]))
            }
            Form::TopoStuck {
                kappa,
                topo,
                remove,
            } => {
                let (fuel, head, tail, order) = (at(0)?, at(1)?, at(2)?, at(3)?);
                let remaining = self.cons(kappa, head, tail);
                Ok(self.topo_cases(kappa, topo, remove, scrutinee, fuel, remaining, order))
            }
            Form::TopoNext {
                kappa,
                topo,
                remove,
            } => {
                let (fuel, remaining, order) = (at(0)?, at(1)?, at(2)?);
                Ok(self.topo_cases(kappa, topo, remove, scrutinee, fuel, remaining, order))
            }
            Form::IterateUntil { sigma, iterate } => {
                let (fuel, state) = (at(0)?, at(1)?);
                let bool = self.b.konst("Bool");
                let result = self.b.pair_ty(sigma, bool);
                let done = self.tru();
                let finished = self.b.pair(sigma, bool, state, done);
                let next = self.b.free();
                let continued = self.b.apps(iterate, &[fuel, next]);
                let continues = self.lam1("next", sigma, next, continued);
                Ok(self.option_cases(sigma, result, scrutinee, finished, continues))
            }
        }
    }

    /// The matcher of an `Ordering` match with a sparse default: its first
    /// alternative is the constructor the runtime names, the default is
    /// `other => other` unless `default` gives a constant.
    fn sparse_ordering(
        &mut self,
        matcher: Term,
        result: Term,
        scrutinee: Term,
        first: Term,
        default: Option<Term>,
    ) -> Term {
        let ordering = self.b.konst("Ordering");
        let motive = self.constant_motive(ordering, result);
        let thunk = self.thunk(first);
        let variable = self.b.free();
        let body = default.unwrap_or(variable);
        let other = self.lam1("other", ordering, variable, body);
        self.b.apps(matcher, &[motive, scrutinee, thunk, other])
    }

    /// `fun (_ : domain) => result`.
    fn constant_motive(&mut self, domain: Term, result: Term) -> Term {
        let lifted = self.b.lift(result, 1, 0);
        self.b.lam("_", EXPLICIT, domain, lifted)
    }

    /// `List.foldl (fun acc key => insert key acc) seen items`.
    fn fold_insert(&mut self, kappa: Term, insert: Term, seen: Term, items: Term) -> Term {
        let list = self.b.list_ty(kappa);
        let acc = self.b.free();
        let key = self.b.free();
        let inserted = self.b.apps(insert, &[key, acc]);
        let inner = self.lam1("key", kappa, key, inserted);
        let step = self.lam1("acc", list, acc, inner);
        let foldl = self.b.konst00("List.foldl");
        self.b.apps(foldl, &[list, kappa, step, seen, items])
    }

    /// The match of `topological` on the nodes ready in `remaining`.
    #[allow(clippy::too_many_arguments)]
    fn topo_cases(
        &mut self,
        kappa: Term,
        topo: Term,
        remove: Term,
        scrutinee: Term,
        fuel: Term,
        remaining: Term,
        order: Term,
    ) -> Term {
        let list = self.b.list_ty(kappa);
        let result = self.option_of(list);
        let empty = self.topo_empty(kappa, remaining, order);
        let ready = self.b.free();
        let tail = self.b.free();
        let rest = self.b.apps(remove, &[ready, remaining]);
        let extended = self.cons(kappa, ready, order);
        let call = self.b.apps(topo, &[fuel, rest, extended]);
        let inner = self.lam1("tail", list, tail, call);
        let step = self.lam1("ready", kappa, ready, inner);
        self.list_cases(kappa, result, scrutinee, empty, step)
    }
}
