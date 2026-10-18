//! Certificate B (SPEC.md §17.17): the rendered crate of a lowered root
//! simulates the lowered program. The aligner walks each function's
//! calculus body beside the crate's Rust body and builds the derivation of
//! the correspondence `LexLeanPreservation.Rust.Corr` that relates them, one
//! rule application per construct, as a kernel term: every implicit argument
//! of a rule is written, the ones its conclusion states by the judgment the
//! aligner derives and the others by the hints the aligner holds from the
//! constructs it is relating. Lean checks every rule's side conditions by
//! evaluation in the kernel, and the library's soundness theorem turns the
//! derivations into the simulation of every function. The aligner is a
//! search, not a proof: a derivation it builds that does not hold is
//! rejected by the kernel, and a rendering it cannot relate is reported,
//! never certified.

use std::collections::{BTreeMap, HashSet};

use super::super::data_a::{Pf, Writer};
use crate::calculus::check::Checker;
use crate::calculus::rust::ast::{
    Block, Callee, Crate, Ctor, Expr, Ident, ItemDef, Let, Lit, Pat, Type,
};
use crate::calculus::rust::runtime::Item;
use crate::calculus::{Arm, Expr as Term, Prim, Program, Shape, Ty};
use crate::ir::core::CoreNode;
use crate::production::kernel::{Given, Term as K};

/// The rules the aligner writes: exactly the constructors of the library's
/// correspondence `Corr`, each a case of its soundness theorem. The rule-set
/// audit (`repo_model::correspondence`) holds this list, the library's
/// inductive, and the theorem's induction to one another, so a rule the
/// library does not prove cannot be written here unnoticed.
pub const RULES: [&str; 49] = [
    "var",
    "varUnit",
    "bOfE",
    "eOfB",
    "lNil",
    "lCons",
    "opsNil",
    "opsCons",
    "prim",
    "fOfB",
    "value",
    "eOfBNil",
    "opsOfL",
    "opsUnit",
    "build",
    "buildPair",
    "buildConst",
    "letBind",
    "letWild",
    "buildSucc",
    "primWiden",
    "primF",
    "callOps",
    "callF",
    "buildSuccF",
    "fUnit",
    "cond",
    "condId",
    "kId",
    "kNeg",
    "kSame",
    "kIf",
    "kHeld",
    "matchNat",
    "matchBool",
    "matchBoolId",
    "matchArms",
    "matchRebuilt",
    "mNil",
    "mCons",
    "matchPair",
    "matchUnit",
    "first",
    "second",
    "fieldRead",
    "fieldHeld",
    "closure",
    "apply",
    "applyF",
];

/// A premise of a rule: the derivation of a sub-judgment, or a side
/// condition the kernel decides by evaluation.
#[derive(Clone, Copy)]
pub(super) enum Prem {
    Sub(Pf),
    Decided,
}

use Prem::Decided;

fn sub(derivation: Pf) -> Prem {
    Prem::Sub(derivation)
}

/// The least bytes one node of the DAG takes in the record of a certificate:
/// a node is a row of the notation, which is never shorter. A refusal on it
/// is a refusal of a certificate that is certainly too large.
const NODE_BYTES: u64 = 12;

const TARGET: &str = "LexLeanTarget.TargetSyntax";
const RUST: &str = "LexLeanPreservation.Rust";

// --- scopes ------------------------------------------------------------------

/// A scope: each program local with its type and the Rust local holding it,
/// innermost last, and the `Ctx` term the correspondence states it as.
#[derive(Debug, Clone)]
pub(super) struct Ctx {
    entries: Vec<(u64, Ty, Option<u64>)>,
    term: K,
}

impl Ctx {
    pub(super) fn term(&self) -> K {
        self.term
    }

    fn lookup(&self, name: u64) -> Option<&(u64, Ty, Option<u64>)> {
        self.entries.iter().rev().find(|entry| entry.0 == name)
    }

    fn slot_free(&self, slot: u64) -> bool {
        self.entries.iter().all(|entry| entry.2 != Some(slot))
    }

    fn scope(&self) -> Vec<(u64, Ty)> {
        self.entries
            .iter()
            .map(|(name, ty, _)| (*name, ty.clone()))
            .collect()
    }
}

/// A binder pattern's slot: `v<m>`, or nothing.
fn slot_of(pattern: &Pat) -> Result<Option<u64>, String> {
    match pattern {
        Pat::Wild => Ok(None),
        Pat::Bind(Ident::Local(m)) => Ok(Some(*m)),
        other => Err(format!("binder pattern {other:?} binds no local")),
    }
}

/// How a conditional is entered.
#[derive(Debug, Clone)]
enum Kind {
    Plain,
    Nat {
        holder: Ident,
        binder: u64,
        slot: Option<u64>,
    },
}

/// Which form a conditional's rendering takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Form {
    Held,
    If,
    Same,
    Id,
    Neg,
}

// --- the aligner -------------------------------------------------------------

pub(super) struct Aligner<'g, 'a> {
    pub(super) w: Writer<'g, 'a>,
    program: &'a Program,
    krate: &'a Crate,
    checker: Checker<'a>,
    /// Whether each function's rendering returns `R<T>`.
    fallible: Vec<bool>,
    /// The calculus function type of each rendered function type.
    fn_types: BTreeMap<u64, Ty>,
    /// The ADTs no value inhabits.
    pub(super) empty: Vec<u64>,
    /// The bytes the certificate may take, `max_file_bytes`.
    budget: u64,
    /// Set when a derivation passed the limit: the aligner tries other rules
    /// when one fails, and would carry on to the same wall again, so after
    /// this every derivation is refused at once and the refusal, not the
    /// failure of whatever rule was being tried, is what is reported.
    pub(super) over: Option<String>,
    /// The nodes the finished rule applications and the two literals reach,
    /// each once: what the record will hold, which the nodes built on the way
    /// (a rule's instantiated type, an attempt a failed form discards) are not.
    counted: HashSet<K>,
    /// `module.program`, `module.krate`, `module.flags`.
    pub(super) p: K,
    pub(super) c: K,
    pub(super) a: K,
}

type Found = Result<Pf, String>;

impl<'g, 'a> Aligner<'g, 'a> {
    pub(super) fn new(
        w: Writer<'g, 'a>,
        program: &'a Program,
        krate: &'a Crate,
        budget: u64,
        module: &str,
    ) -> Result<Self, String> {
        let mut fallible = vec![false; program.functions.len()];
        for item in &krate.items {
            if let ItemDef::Function {
                name: Ident::Function(index),
                result,
                ..
            } = item
            {
                let slot = usize::try_from(*index)
                    .ok()
                    .and_then(|position| fallible.get_mut(position))
                    .ok_or_else(|| format!("the crate renders an undeclared function {index}"))?;
                *slot = matches!(result, Type::Fallible(_));
            }
        }
        let mut w = w;
        let p = w.b.konst(&format!("{module}.program"));
        let c = w.b.konst(&format!("{module}.krate"));
        let a = w.b.konst(&format!("{module}.flags"));
        let mut aligner = Aligner {
            w,
            program,
            krate,
            checker: Checker { program },
            fallible,
            fn_types: BTreeMap::new(),
            empty: Vec::new(),
            budget,
            over: None,
            counted: HashSet::new(),
            p,
            c,
            a,
        };
        aligner.empty = aligner.empty_adts();
        Ok(aligner)
    }

    // --- terms ----------------------------------------------------------------

    fn rust(&mut self, name: &str) -> K {
        self.w.b.konst(&format!("{RUST}.{name}"))
    }

    fn target(&mut self, name: &str) -> K {
        self.w.b.konst(&format!("{TARGET}.{name}"))
    }

    fn nat(&mut self, n: u64) -> K {
        self.w.nat_u(n)
    }

    fn boolean(&mut self, value: bool) -> K {
        self.w.boolean(value)
    }

    fn nats(&mut self, items: &[u64]) -> K {
        let nat = self.w.b.konst("Nat");
        let items: Vec<K> = items.iter().map(|n| self.w.nat_u(*n)).collect();
        self.w.b.list(nat, &items)
    }

    fn bools(&mut self, items: &[bool]) -> K {
        let bool = self.w.b.konst("Bool");
        let items: Vec<K> = items.iter().map(|flag| self.w.boolean(*flag)).collect();
        self.w.b.list(bool, &items)
    }

    fn ty(&mut self, ty: &Ty) -> K {
        self.w.ty_term(ty)
    }

    fn types(&mut self, types: &[Ty]) -> K {
        let element = self.target("Ty");
        let items: Vec<K> = types.iter().map(|ty| self.w.ty_term(ty)).collect();
        self.w.b.list(element, &items)
    }

    fn slot(&mut self, slot: Option<u64>) -> K {
        let nat = self.w.b.konst("Nat");
        match slot {
            Some(m) => {
                let some = self.w.b.konst0("Option.some");
                let m = self.w.nat_u(m);
                self.w.b.apps(some, &[nat, m])
            }
            None => {
                let none = self.w.b.konst0("Option.none");
                self.w.b.app(none, nat)
            }
        }
    }

    /// A calculus expression as a `TargetSyntax.Expr` term.
    fn calc(&mut self, term: &Term) -> Result<K, String> {
        self.w.expr_term(term)
    }

    /// `List Expr` of calculus expressions.
    fn calcs(&mut self, terms: &[Term]) -> Result<K, String> {
        let element = self.target("Expr");
        let mut items = Vec::with_capacity(terms.len());
        for term in terms {
            items.push(self.w.expr_term(term)?);
        }
        Ok(self.w.b.list(element, &items))
    }

    /// The `Ctx` entry `(x, (t, slot))`.
    fn ctx_entry(&mut self, name: u64, ty: &Ty, slot: Option<u64>) -> K {
        let nat = self.w.b.konst("Nat");
        let ty_ty = self.target("Ty");
        let option_nat = {
            let option = self.w.b.konst0("Option");
            self.w.b.app(option, nat)
        };
        let ty = self.w.ty_term(ty);
        let slot = self.slot(slot);
        let inner = self.w.b.pair(ty_ty, option_nat, ty, slot);
        let inner_ty = self.w.b.pair_ty(ty_ty, option_nat);
        let name = self.w.nat_u(name);
        self.w.b.pair(nat, inner_ty, name, inner)
    }

    fn ctx_elem_ty(&mut self) -> K {
        let nat = self.w.b.konst("Nat");
        let ty_ty = self.target("Ty");
        let option = self.w.b.konst0("Option");
        let option_nat = self.w.b.app(option, nat);
        let inner = self.w.b.pair_ty(ty_ty, option_nat);
        self.w.b.pair_ty(nat, inner)
    }

    pub(super) fn ctx_empty(&mut self) -> Ctx {
        let element = self.ctx_elem_ty();
        Ctx {
            entries: Vec::new(),
            term: self.w.b.list(element, &[]),
        }
    }

    fn ctx_with(&mut self, g: &Ctx, name: u64, ty: &Ty, slot: Option<u64>) -> Ctx {
        let element = self.ctx_elem_ty();
        let entry = self.ctx_entry(name, ty, slot);
        let cons = self.w.b.konst0("List.cons");
        let term = self.w.b.apps(cons, &[element, entry, g.term]);
        let mut entries = g.entries.clone();
        entries.push((name, ty.clone(), slot));
        Ctx { entries, term }
    }

    fn lets(&mut self, lets: &[Let]) -> K {
        self.w.rs_lets(lets)
    }

    fn rexpr(&mut self, expr: &Expr) -> K {
        self.w.rs_expr(expr)
    }

    /// Two Rust expressions are the same term when their kernel terms are,
    /// which ignores the origins.
    fn same(&mut self, left: &Expr, right: &Expr) -> bool {
        self.w.rs_expr(left) == self.w.rs_expr(right)
    }

    fn lit_unit() -> Expr {
        Expr::Lit(Lit::Unit, crate::calculus::rust::ast::Origin::of("unit"))
    }

    fn lit_bool(value: bool) -> Expr {
        Expr::Lit(
            Lit::Bool(value),
            crate::calculus::rust::ast::Origin::of("value"),
        )
    }

    // --- judgments ------------------------------------------------------------

    fn jd(&mut self, constructor: &str, args: &[K]) -> K {
        let head = self.rust(&format!("Jd.{constructor}"));
        self.w.b.apps(head, args)
    }

    fn jd_e(&mut self, term: &Term, ty: &Ty, r: &Expr) -> Result<K, String> {
        let e = self.calc(term)?;
        let t = self.ty(ty);
        let r = self.rexpr(r);
        Ok(self.jd("e", &[e, t, r]))
    }

    fn jd_b(
        &mut self,
        fm: bool,
        term: &Term,
        ty: &Ty,
        lets: &[Let],
        tail: &Expr,
    ) -> Result<K, String> {
        let fm = self.boolean(fm);
        let e = self.calc(term)?;
        let t = self.ty(ty);
        let lets = self.lets(lets);
        let tail = self.rexpr(tail);
        Ok(self.jd("b", &[fm, e, t, lets, tail]))
    }

    fn jd_l(&mut self, terms: &[Term], types: &[Ty], rs: &[Expr]) -> Result<K, String> {
        let es = self.calcs(terms)?;
        let ts = self.types(types);
        let rs = self.rexprs(rs);
        Ok(self.jd("l", &[es, ts, rs]))
    }

    fn jd_ops(
        &mut self,
        terms: &[Term],
        types: &[Ty],
        lets: &[Let],
        args: &[Expr],
    ) -> Result<K, String> {
        let es = self.calcs(terms)?;
        let ts = self.types(types);
        let lets = self.lets(lets);
        let args = self.rexprs(args);
        Ok(self.jd("ops", &[es, ts, lets, args]))
    }

    fn rexprs(&mut self, exprs: &[Expr]) -> K {
        self.w.rs_exprs(exprs)
    }

    fn kind_term(&mut self, kind: &Kind) -> K {
        match kind {
            Kind::Plain => self.rust("Kind.plain"),
            Kind::Nat {
                holder,
                binder,
                slot,
            } => {
                let holder = self.w.rs_ident(holder);
                let binder = self.nat(*binder);
                let slot = self.slot(*slot);
                let head = self.rust("Kind.nat");
                self.w.b.apps(head, &[holder, binder, slot])
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn jd_k(
        &mut self,
        kind: &Kind,
        fm: bool,
        ce: &Expr,
        a: &Term,
        b: &Term,
        ty: &Ty,
        lets: &[Let],
        tail: &Expr,
    ) -> Result<K, String> {
        let kind = self.kind_term(kind);
        let fm = self.boolean(fm);
        let ce = self.rexpr(ce);
        let a = self.calc(a)?;
        let b = self.calc(b)?;
        let t = self.ty(ty);
        let lets = self.lets(lets);
        let tail = self.rexpr(tail);
        Ok(self.jd("k", &[kind, fm, ce, a, b, t, lets, tail]))
    }

    /// `Corr p c A Γ fl jd`, the type of a derivation.
    fn corr_ty(&mut self, g: &Ctx, fl: bool, jd: K) -> K {
        let corr = self.rust("Corr");
        let fl = self.boolean(fl);
        self.w
            .b
            .apps(corr, &[self.p, self.c, self.a, g.term, fl, jd])
    }

    /// One rule applied: the conclusion `Corr p c A Γ fl jd`, the hints for
    /// the implicit binders the conclusion does not state, and the premises
    /// in order.
    fn rule(
        &mut self,
        name: &'static str,
        g: &Ctx,
        fl: bool,
        jd: K,
        hints: &[(&'static str, K)],
        premises: &[Prem],
    ) -> Found {
        let conclusion = self.corr_ty(g, fl, jd);
        let flag = self.boolean(fl);
        let mut all: Vec<(&str, K)> = vec![
            ("p", self.p),
            ("c", self.c),
            ("A", self.a),
            ("Γ", g.term),
            ("fl", flag),
        ];
        all.extend(hints.iter().copied());
        let lemma = format!("Rust.Corr.{name}");
        let mut open = self.w.open(&lemma, &all)?;
        open.expect(&self.w.b, conclusion);
        for premise in premises {
            let given = match premise {
                Prem::Sub(derivation) => Given::Proof(derivation.proof, derivation.ty),
                Decided => Given::Refl,
            };
            open.give(&mut self.w.b, given)?;
        }
        let applied = open.finish(&mut self.w.b)?;
        self.count_new(applied.term);
        Ok(Pf {
            proof: applied.term,
            ty: conclusion,
        })
    }

    // --- emptiness, as `uninh` and `validU` ----------------------------------

    fn no_closure(&self, parameters: &[Ty], result: &Ty) -> bool {
        self.krate.items.iter().all(|item| match item {
            ItemDef::Apply { arms, .. } => arms.iter().all(|arm| {
                usize::try_from(arm.function)
                    .ok()
                    .and_then(|position| self.program.functions.get(position))
                    .is_none_or(|function| {
                        !(function.types.get(arm.captures.len()..) == Some(parameters)
                            && function.result == *result)
                    })
            }),
            ItemDef::Enum { .. } | ItemDef::Function { .. } => true,
        })
    }

    pub(super) fn uninh(&self, empty: &[u64], ty: &Ty) -> bool {
        match ty {
            Ty::Adt { index } => empty.contains(index),
            Ty::Pair { left, right } => self.uninh(empty, left) || self.uninh(empty, right),
            Ty::Result { ok, error } => self.uninh(empty, ok) && self.uninh(empty, error),
            Ty::Fn { parameters, result } => self.no_closure(parameters, result),
            Ty::Unit
            | Ty::Bool
            | Ty::Nat
            | Ty::Int
            | Ty::Fixed { .. }
            | Ty::String
            | Ty::Bytes
            | Ty::Ordering
            | Ty::Option { .. }
            | Ty::List { .. } => false,
        }
    }

    /// The largest set of ADTs each of whose constructors holds a field of
    /// an empty type: no value has a type it claims empty.
    fn empty_adts(&self) -> Vec<u64> {
        let mut empty: Vec<u64> = (0..self.program.adts.len() as u64).collect();
        loop {
            let kept: Vec<u64> = empty
                .iter()
                .copied()
                .filter(|index| {
                    self.program.adts[*index as usize]
                        .constructors
                        .iter()
                        .all(|fields| fields.iter().any(|ty| self.uninh(&empty, ty)))
                })
                .collect();
            if kept == empty {
                return empty;
            }
            empty = kept;
        }
    }

    // --- types ----------------------------------------------------------------

    fn type_of(&self, g: &Ctx, term: &Term) -> Result<Ty, String> {
        self.checker.expr(term, &mut g.scope())
    }

    pub(super) fn function(&self, index: u64) -> Result<&'a crate::calculus::Function, String> {
        usize::try_from(index)
            .ok()
            .and_then(|position| self.program.functions.get(position))
            .ok_or_else(|| format!("function {index} is not declared"))
    }

    pub(super) fn fallible_fn(&self, index: u64) -> bool {
        usize::try_from(index)
            .ok()
            .and_then(|position| self.fallible.get(position))
            .copied()
            .unwrap_or(false)
    }

    fn apply_fallible(&self, fn_type: u64) -> Result<bool, String> {
        self.krate
            .items
            .iter()
            .find_map(|item| match item {
                ItemDef::Apply {
                    fn_type: position,
                    fallible,
                    ..
                } if *position == fn_type => Some(*fallible),
                _ => None,
            })
            .ok_or_else(|| format!("function type {fn_type} has no apply"))
    }

    fn record_fn_type(&mut self, position: u64, ty: &Ty) -> Result<(), String> {
        if let Some(known) = self.fn_types.get(&position) {
            if known != ty {
                return Err(format!(
                    "function type {position} realizes two calculus types"
                ));
            }
        }
        self.fn_types.insert(position, ty.clone());
        Ok(())
    }

    // --- value position -------------------------------------------------------

    /// The derivation of a sub-term is refused as soon as the certificate
    /// built so far is larger than a certificate may be: the derivation of a
    /// node repeats the types and the patterns of the nodes it derives, so it
    /// can grow with the square of a record's arity or of the depth of a
    /// nesting, which no count of the program's nodes bounds. A rule is
    /// checked as the derivation returns through it, so the work done before
    /// the refusal is the size the limit allows, once.
    fn within_budget(&mut self, derivation: Pf) -> Found {
        let total = self.weighed();
        if total > self.budget {
            return Err(self.refuse(total));
        }
        Ok(derivation)
    }

    /// Record that a derivation of at least `total` bytes passed the limit.
    fn refuse(&mut self, total: u64) -> String {
        let reason = format!(
            "{}the derivation of a function is at least {total} bytes, beyond the {} bytes of max_file_bytes",
            crate::production::lower::LIMIT,
            self.budget
        );
        self.over.get_or_insert_with(|| reason.clone());
        reason
    }

    /// The refusal already made, if any.
    fn refused(&self) -> Result<(), String> {
        self.over.clone().map_or(Ok(()), Err)
    }

    /// `e`, refused as soon as its derivation passes the limit.
    fn eg(&mut self, g: &Ctx, fl: bool, term: &Term, ty: &Ty, r: &Expr) -> Found {
        self.refused()?;
        let derivation = self.e(g, fl, term, ty, r)?;
        self.within_budget(derivation)
    }

    /// `b`, refused as soon as its derivation passes the limit.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn bg(
        &mut self,
        g: &Ctx,
        fl: bool,
        fm: bool,
        term: &Term,
        ty: &Ty,
        lets: &[Let],
        tail: &Expr,
    ) -> Found {
        self.refused()?;
        let derivation = self.b(g, fl, fm, term, ty, lets, tail)?;
        self.within_budget(derivation)
    }

    /// `m`, refused as soon as its derivation passes the limit.
    #[allow(clippy::too_many_arguments)]
    fn mg(
        &mut self,
        g: &Ctx,
        fl: bool,
        st: &Ty,
        view: bool,
        arms: &[Arm],
        fm: bool,
        tau: &Ty,
        rarms: &[(Pat, Block)],
    ) -> Result<(Pf, Vec<u64>), String> {
        self.refused()?;
        let (derivation, indices) = self.m(g, fl, st, view, arms, fm, tau, rarms)?;
        Ok((self.within_budget(derivation)?, indices))
    }

    /// `.e term ty r`.
    fn e(&mut self, g: &Ctx, fl: bool, term: &Term, ty: &Ty, r: &Expr) -> Found {
        match (term, r) {
            (Term::Var { name }, _) => {
                let (_, bound, slot) = g
                    .lookup(*name)
                    .ok_or_else(|| format!("local {name} is unbound"))?;
                let (bound, slot) = (bound.clone(), *slot);
                let jd = self.jd_e(term, ty, r)?;
                if bound == Ty::Unit {
                    if !matches!(r, Expr::Lit(Lit::Unit, _)) {
                        return Err("a unit local is read as `()`".to_owned());
                    }
                    let s = self.slot(slot);
                    self.rule("varUnit", g, fl, jd, &[("s", s)], &[Decided])
                } else {
                    let read = match r {
                        Expr::Move(read, _)
                        | Expr::Clone(read, _)
                        | Expr::Copy(read, _)
                        | Expr::Deref(read, _)
                        | Expr::Unbox(read, _) => read,
                        _ => return Err("a local is not read".to_owned()),
                    };
                    let Some(held) = (match (read, slot) {
                        (Ident::Local(m), Some(held)) if *m == held => Some(held),
                        _ => None,
                    }) else {
                        return Err(format!("local {name} is read from another binding"));
                    };
                    let m = self.nat(held);
                    self.rule("var", g, fl, jd, &[("m", m)], &[Decided, Decided])
                }
            }
            (Term::Value { value, .. }, _) => {
                let fits = match (value, r) {
                    (crate::calculus::Value::Bool { value }, Expr::Lit(Lit::Bool(written), _)) => {
                        value == written
                    }
                    (crate::calculus::Value::Bool { .. }, _) => false,
                    (crate::calculus::Value::Unit, written) => {
                        matches!(written, Expr::Lit(Lit::Unit, _))
                    }
                    _ => !matches!(r, Expr::Block(_) | Expr::If { .. } | Expr::Match { .. }),
                };
                if !fits {
                    return Err("the value is not its literal".to_owned());
                }
                let jd = self.jd_e(term, ty, r)?;
                self.rule("value", g, fl, jd, &[], &[Decided, Decided])
            }
            (
                Term::Build {
                    shape: Shape::Pair,
                    ty: Ty::Pair { left, right },
                    operands,
                },
                Expr::Pair(ra, rb, _),
            ) => {
                let types = [left.as_ref().clone(), right.as_ref().clone()];
                let rendered = [ra.as_ref().clone(), rb.as_ref().clone()];
                let operands_l = self.l(g, fl, operands, &types, &rendered)?;
                let jd = self.jd_e(term, ty, r)?;
                self.rule("buildPair", g, fl, jd, &[], &[sub(operands_l)])
            }
            (
                Term::Build {
                    shape:
                        shape @ (Shape::True
                        | Shape::False
                        | Shape::Unit
                        | Shape::Lt
                        | Shape::Eq
                        | Shape::Gt
                        | Shape::Zero),
                    ..
                },
                Expr::Lit(written, _),
            ) => {
                let fits = match (shape, written) {
                    (Shape::True, Lit::Bool(value)) => *value,
                    (Shape::False, Lit::Bool(value)) => !*value,
                    (Shape::Unit, Lit::Unit) => true,
                    (Shape::Lt, Lit::Ordering(crate::calculus::OrderingValue::Lt))
                    | (Shape::Eq, Lit::Ordering(crate::calculus::OrderingValue::Eq))
                    | (Shape::Gt, Lit::Ordering(crate::calculus::OrderingValue::Gt)) => true,
                    (Shape::Zero, Lit::Nat(value)) => *value == 0,
                    _ => false,
                };
                if !fits {
                    return Err("the constant is not its literal".to_owned());
                }
                let jd = self.jd_e(term, ty, r)?;
                self.rule("buildConst", g, fl, jd, &[], &[Decided])
            }
            (
                Term::Build {
                    shape: Shape::Succ,
                    operands,
                    ..
                },
                Expr::Call {
                    callee: Callee::Runtime(Item::NatSucc),
                    args,
                    propagate: true,
                    ..
                },
            ) => {
                let operand = self.l(g, fl, operands, &[Ty::Nat], args)?;
                let jd = self.jd_e(term, ty, r)?;
                self.rule("buildSucc", g, fl, jd, &[], &[sub(operand)])
            }
            (_, Expr::Block(block)) => {
                let inner = self.bg(g, fl, false, term, ty, &block.lets, &block.tail)?;
                let jd = self.jd_e(term, ty, r)?;
                let lets = self.lets(&block.lets);
                let tail = self.rexpr(&block.tail);
                self.rule(
                    "eOfB",
                    g,
                    fl,
                    jd,
                    &[("lets", lets), ("tail", tail)],
                    &[sub(inner)],
                )
            }
            _ => {
                let inner = self.bg(g, fl, false, term, ty, &[], r)?;
                let jd = self.jd_e(term, ty, r)?;
                self.rule("eOfBNil", g, fl, jd, &[], &[sub(inner)])
            }
        }
    }

    /// `.l terms types rs`.
    fn l(&mut self, g: &Ctx, fl: bool, terms: &[Term], types: &[Ty], rs: &[Expr]) -> Found {
        let jd = self.jd_l(terms, types, rs)?;
        match (terms, types, rs) {
            ([], [], []) => self.rule("lNil", g, fl, jd, &[], &[]),
            ([term, terms @ ..], [ty, types @ ..], [r, rs @ ..]) => {
                let head = self.eg(g, fl, term, ty, r)?;
                let rest = self.l(g, fl, terms, types, rs)?;
                self.rule("lCons", g, fl, jd, &[], &[sub(head), sub(rest)])
            }
            _ => Err("operands, types, and renderings differ in number".to_owned()),
        }
    }

    /// `.ops terms types lets args`.
    fn ops(
        &mut self,
        g: &Ctx,
        fl: bool,
        terms: &[Term],
        types: &[Ty],
        lets: &[Let],
        args: &[Expr],
    ) -> Found {
        let jd = self.jd_ops(terms, types, lets, args)?;
        if lets.is_empty() && !terms.is_empty() {
            let operands = self.l(g, fl, terms, types, args)?;
            return self.rule("opsOfL", g, fl, jd, &[], &[sub(operands)]);
        }
        match (terms, types, lets, args) {
            ([], [], [], []) => self.rule("opsNil", g, fl, jd, &[], &[]),
            ([term, terms @ ..], [ty, types @ ..], [binding, lets @ ..], [arg, args @ ..]) => {
                match (&binding.pat, arg) {
                    (Pat::Unit, Expr::Lit(Lit::Unit, _)) => {
                        let head = self.eg(g, fl, term, &Ty::Unit, &binding.value)?;
                        let rest = self.ops(g, fl, terms, types, lets, args)?;
                        self.rule("opsUnit", g, fl, jd, &[], &[sub(head), sub(rest)])
                    }
                    (Pat::Bind(temp), Expr::Move(read, _)) if temp == read => {
                        let head = self.eg(g, fl, term, ty, &binding.value)?;
                        let rest = self.ops(g, fl, terms, types, lets, args)?;
                        self.rule(
                            "opsCons",
                            g,
                            fl,
                            jd,
                            &[],
                            &[sub(head), sub(rest), Decided, Decided],
                        )
                    }
                    _ => Err("an operand is not bound to the temporary it is read from".to_owned()),
                }
            }
            _ => Err("operands and their bindings differ in number".to_owned()),
        }
    }

    // --- blocks ---------------------------------------------------------------

    /// `.b fm term ty lets tail`.
    #[allow(clippy::too_many_arguments)]
    fn b(
        &mut self,
        g: &Ctx,
        fl: bool,
        fm: bool,
        term: &Term,
        ty: &Ty,
        lets: &[Let],
        tail: &Expr,
    ) -> Found {
        if fm {
            return self.b_fallible(g, term, ty, lets, tail);
        }
        match term {
            Term::Let { .. } => self.let_(g, fl, false, term, ty, lets, tail),
            Term::Cond { .. } => self.cond(g, fl, false, term, ty, lets, tail),
            Term::Match { .. } => self.match_(g, fl, false, term, ty, lets, tail),
            Term::Field { .. } => self.field(g, fl, false, term, ty, lets, tail),
            Term::Build {
                shape:
                    shape @ (Shape::None
                    | Shape::Some
                    | Shape::Ok
                    | Shape::Error
                    | Shape::Nil
                    | Shape::Cons
                    | Shape::Adt { .. }),
                ty: built,
                operands,
            } => {
                let Expr::Construct { args, .. } = tail else {
                    return Err("a constructed value is not a construction".to_owned());
                };
                let types = self.checker.shape_fields(*shape, built)?;
                let (mask, plain) = unboxed(args);
                let operands_d = self.ops(g, fl, operands, &types, lets, &plain)?;
                let jd = self.jd_b(false, term, ty, lets, tail)?;
                let ts = self.types(&types);
                let mask = self.bools(&mask);
                self.rule(
                    "build",
                    g,
                    fl,
                    jd,
                    &[("ts", ts), ("mask", mask)],
                    &[sub(operands_d), Decided],
                )
            }
            Term::Call { function, operands } => {
                let Expr::Call {
                    callee: Callee::Function(called),
                    args,
                    ..
                } = tail
                else {
                    return Err("a call is not rendered as a call".to_owned());
                };
                if called != function {
                    return Err("a call renders another function".to_owned());
                }
                let types = self.function(*function)?.types.clone();
                let operands_d = self.ops(g, fl, operands, &types, lets, args)?;
                let jd = self.jd_b(false, term, ty, lets, tail)?;
                let ts = self.types(&types);
                let fn_term = self.function_literal(*function)?;
                self.rule(
                    "callOps",
                    g,
                    fl,
                    jd,
                    &[("ts", ts), ("fn", fn_term)],
                    &[sub(operands_d), Decided, Decided, Decided, Decided, Decided],
                )
            }
            Term::Prim {
                operation,
                operands,
            } => self.prim(g, fl, false, term, ty, operation, operands, lets, tail),
            Term::Closure { function, captures } => {
                let Expr::Construct {
                    ctor:
                        Ctor::Closure {
                            fn_type,
                            function: built,
                        },
                    args,
                    ..
                } = tail
                else {
                    return Err("a closure is not rendered as one".to_owned());
                };
                if built != function {
                    return Err("a closure renders another function".to_owned());
                }
                let closure_ty = self.type_of(g, term)?;
                self.record_fn_type(*fn_type, &closure_ty)?;
                let callee = self.function(*function)?;
                let caps: Vec<Ty> = callee
                    .types
                    .get(..captures.len())
                    .ok_or("a closure captures more than its function takes")?
                    .to_vec();
                let (mask, plain) = unboxed(args);
                let operands_d = self.ops(g, fl, captures, &caps, lets, &plain)?;
                let af = self.apply_fallible(*fn_type)?;
                let ff = self.fallible_fn(*function);
                let jd = self.jd_b(false, term, ty, lets, tail)?;
                let caps_term = self.types(&caps);
                let mask = self.bools(&mask);
                let af = self.boolean(af);
                let ff = self.boolean(ff);
                let fn_term = self.function_literal(*function)?;
                self.rule(
                    "closure",
                    g,
                    fl,
                    jd,
                    &[
                        ("caps", caps_term),
                        ("mask", mask),
                        ("af", af),
                        ("ff", ff),
                        ("fn", fn_term),
                    ],
                    &[
                        sub(operands_d),
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                    ],
                )
            }
            Term::Apply { .. } => self.apply(g, fl, false, term, ty, lets, tail),
            Term::First { value } | Term::Second { value } => {
                let first = matches!(term, Term::First { .. });
                let [binding] = lets else {
                    return Err("a projection binds one pair".to_owned());
                };
                let pair_ty = self.type_of(g, value)?;
                let Ty::Pair { left, right } = &pair_ty else {
                    return Err("a projection of a non-pair".to_owned());
                };
                let ta = self.ty(left);
                let tb = self.ty(right);
                let scrutinee = self.eg(g, fl, value, &pair_ty, &binding.value)?;
                let jd = self.jd_b(false, term, ty, lets, tail)?;
                self.rule(
                    if first { "first" } else { "second" },
                    g,
                    fl,
                    jd,
                    &[("ta", ta), ("tb", tb)],
                    &[sub(scrutinee), Decided],
                )
            }
            Term::Var { .. } | Term::Value { .. } | Term::Build { .. } => {
                if !lets.is_empty() {
                    return Err("a value binds nothing".to_owned());
                }
                let value = self.eg(g, fl, term, ty, tail)?;
                let jd = self.jd_b(false, term, ty, lets, tail)?;
                self.rule("bOfE", g, fl, jd, &[], &[sub(value)])
            }
        }
    }

    /// The `TargetSyntax.Function` term of function `index`, which the
    /// program literal states.
    fn function_literal(&mut self, index: u64) -> Result<K, String> {
        let function = self.function(index)?;
        self.w.function_term(function)
    }

    /// `.b true term ty lets tail`: the tail of a fallible function.
    fn b_fallible(&mut self, g: &Ctx, term: &Term, ty: &Ty, lets: &[Let], tail: &Expr) -> Found {
        let mut failures = Vec::new();
        match self.b_structural(g, term, ty, lets, tail) {
            Ok(found) => return Ok(found),
            Err(reason) => failures.push(reason),
        }
        if let Expr::Succeed(inner, _) = tail {
            match self.bg(g, true, false, term, ty, lets, inner) {
                Ok(value) => {
                    let jd = self.jd_b(true, term, ty, lets, tail)?;
                    return self.rule("fOfB", g, true, jd, &[], &[sub(value)]);
                }
                Err(reason) => failures.push(reason),
            }
        }
        if let (Some((last, init)), true) = (lets.split_last(), *ty == Ty::Unit) {
            let unit_ok = matches!(tail, Expr::Succeed(inner, _) if matches!(inner.as_ref(), Expr::Lit(Lit::Unit, _)));
            if matches!(last.pat, Pat::Unit) && last.ty.is_none() && unit_ok {
                match self.bg(g, true, false, term, ty, init, &last.value) {
                    Ok(value) => {
                        let jd = self.jd_b(true, term, ty, lets, tail)?;
                        let init_term = self.lets(init);
                        return self.rule(
                            "fUnit",
                            g,
                            true,
                            jd,
                            &[("lets", init_term)],
                            &[sub(value)],
                        );
                    }
                    Err(reason) => failures.push(reason),
                }
            }
        }
        Err(format!(
            "no fallible tail rule fits: {}",
            failures.join("; ")
        ))
    }

    /// The rules that keep a fallible tail inside a construct.
    fn b_structural(&mut self, g: &Ctx, term: &Term, ty: &Ty, lets: &[Let], tail: &Expr) -> Found {
        match term {
            Term::Let { .. } => self.let_(g, true, true, term, ty, lets, tail),
            Term::Cond { .. } => self.cond(g, true, true, term, ty, lets, tail),
            Term::Match { .. } => self.match_(g, true, true, term, ty, lets, tail),
            Term::Field { .. } => self.field(g, true, true, term, ty, lets, tail),
            Term::Call { function, operands } => {
                let Expr::Call {
                    callee: Callee::Function(called),
                    args,
                    propagate: false,
                    ..
                } = tail
                else {
                    return Err("not a returned call".to_owned());
                };
                if called != function || !self.fallible_fn(*function) {
                    return Err("not a returned fallible call".to_owned());
                }
                let types = self.function(*function)?.types.clone();
                let operands_d = self.ops(g, true, operands, &types, lets, args)?;
                let jd = self.jd_b(true, term, ty, lets, tail)?;
                let ts = self.types(&types);
                let fn_term = self.function_literal(*function)?;
                self.rule(
                    "callF",
                    g,
                    true,
                    jd,
                    &[("ts", ts), ("fn", fn_term)],
                    &[sub(operands_d), Decided, Decided, Decided, Decided],
                )
            }
            Term::Prim {
                operation,
                operands,
            } => self.prim(g, true, true, term, ty, operation, operands, lets, tail),
            Term::Build {
                shape: Shape::Succ,
                operands,
                ..
            } => {
                let Expr::Call {
                    callee: Callee::Runtime(Item::NatSucc),
                    args,
                    propagate: false,
                    ..
                } = tail
                else {
                    return Err("not a returned successor".to_owned());
                };
                if !lets.is_empty() {
                    return Err("a successor binds nothing".to_owned());
                }
                let operand = self.l(g, true, operands, &[Ty::Nat], args)?;
                let jd = self.jd_b(true, term, ty, lets, tail)?;
                self.rule("buildSuccF", g, true, jd, &[], &[sub(operand)])
            }
            Term::Apply { .. } => self.apply(g, true, true, term, ty, lets, tail),
            _ => Err("no construct keeps the tail".to_owned()),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn let_(
        &mut self,
        g: &Ctx,
        fl: bool,
        fm: bool,
        term: &Term,
        ty: &Ty,
        lets: &[Let],
        tail: &Expr,
    ) -> Found {
        let Term::Let {
            name,
            ty: bound_ty,
            bound,
            body,
        } = term
        else {
            return Err("not a let".to_owned());
        };
        let Some((first, rest)) = lets.split_first() else {
            return Err("a let binds nothing".to_owned());
        };
        let slot = slot_of(&first.pat)?;
        let bound_d = self.eg(g, fl, bound, bound_ty, &first.value)?;
        let inner = self.ctx_with(g, *name, bound_ty, slot);
        let body_d = self.bg(&inner, fl, fm, body, ty, rest, tail)?;
        let jd = self.jd_b(fm, term, ty, lets, tail)?;
        let fm_term = self.boolean(fm);
        match slot {
            Some(m) => {
                let m = self.nat(m);
                self.rule(
                    "letBind",
                    g,
                    fl,
                    jd,
                    &[("fm", fm_term), ("m", m)],
                    &[sub(bound_d), Decided, sub(body_d)],
                )
            }
            None => self.rule(
                "letWild",
                g,
                fl,
                jd,
                &[("fm", fm_term)],
                &[sub(bound_d), sub(body_d)],
            ),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn prim(
        &mut self,
        g: &Ctx,
        fl: bool,
        fm: bool,
        term: &Term,
        ty: &Ty,
        operation: &Prim,
        operands: &[Term],
        lets: &[Let],
        tail: &Expr,
    ) -> Found {
        let Expr::Call {
            callee: Callee::Runtime(_),
            args,
            propagate,
            ..
        } = tail
        else {
            return Err("a primitive is not a runtime call".to_owned());
        };
        let types = operands
            .iter()
            .map(|operand| self.type_of(g, operand))
            .collect::<Result<Vec<_>, _>>()?;
        if let Prim::Convert { .. } = operation {
            let ([operand], [operand_ty], [Expr::Widen(arg, _)]) =
                (operands, types.as_slice(), args.as_slice())
            else {
                return Err("a conversion takes one widened operand".to_owned());
            };
            let operand_d = self.ops(
                g,
                fl,
                std::slice::from_ref(operand),
                std::slice::from_ref(operand_ty),
                lets,
                std::slice::from_ref(arg.as_ref()),
            )?;
            if fm {
                return Err("a conversion does not fail".to_owned());
            }
            let jd = self.jd_b(false, term, ty, lets, tail)?;
            let t0 = self.ty(operand_ty);
            return self.rule(
                "primWiden",
                g,
                fl,
                jd,
                &[("t0", t0)],
                &[sub(operand_d), Decided, Decided],
            );
        }
        let operands_d = self.ops(g, fl, operands, &types, lets, args)?;
        let ts = self.types(&types);
        if fm {
            if *propagate {
                return Err("a fallible tail returns the primitive's result".to_owned());
            }
            let jd = self.jd_b(true, term, ty, lets, tail)?;
            self.rule(
                "primF",
                g,
                true,
                jd,
                &[("ts", ts)],
                &[
                    sub(operands_d),
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                ],
            )
        } else {
            let jd = self.jd_b(false, term, ty, lets, tail)?;
            self.rule(
                "prim",
                g,
                fl,
                jd,
                &[("ts", ts)],
                &[
                    sub(operands_d),
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                ],
            )
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn apply(
        &mut self,
        g: &Ctx,
        fl: bool,
        fm: bool,
        term: &Term,
        ty: &Ty,
        lets: &[Let],
        tail: &Expr,
    ) -> Found {
        let Term::Apply { target, operands } = term else {
            return Err("not an application".to_owned());
        };
        let Some((first, rest)) = lets.split_first() else {
            return Err("an application binds its closure".to_owned());
        };
        let (Pat::Bind(callee), Some(Type::Fn(position))) = (&first.pat, &first.ty) else {
            return Err("an application binds its closure first".to_owned());
        };
        let Expr::Apply {
            holder,
            args,
            propagate,
            ..
        } = tail
        else {
            return Err("an application is not rendered as one".to_owned());
        };
        if holder != callee {
            return Err("an application applies another binding".to_owned());
        }
        let target_ty = self.type_of(g, target)?;
        let Ty::Fn { parameters, .. } = &target_ty else {
            return Err("an application of a non-function".to_owned());
        };
        let parameters = parameters.clone();
        self.record_fn_type(*position, &target_ty)?;
        let target_d = self.eg(g, fl, target, &target_ty, &first.value)?;
        let operands_d = self.ops(g, fl, operands, &parameters, rest, args)?;
        let jd = self.jd_b(fm, term, ty, lets, tail)?;
        let ps = self.types(&parameters);
        if fm {
            if *propagate {
                return Err("a fallible tail returns the application's result".to_owned());
            }
            self.rule(
                "applyF",
                g,
                true,
                jd,
                &[("ps", ps)],
                &[sub(target_d), sub(operands_d), Decided, Decided, Decided],
            )
        } else {
            let af = self.boolean(*propagate);
            self.rule(
                "apply",
                g,
                fl,
                jd,
                &[("ps", ps), ("af", af)],
                &[
                    sub(target_d),
                    sub(operands_d),
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                ],
            )
        }
    }

    // --- conditionals -----------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    fn cond(
        &mut self,
        g: &Ctx,
        fl: bool,
        fm: bool,
        term: &Term,
        ty: &Ty,
        lets: &[Let],
        tail: &Expr,
    ) -> Found {
        let Term::Cond {
            condition,
            then_branch,
            else_branch,
        } = term
        else {
            return Err("not a conditional".to_owned());
        };
        let mut candidates: Vec<(usize, Expr, Form)> = Vec::new();
        if let Expr::If {
            condition: tested, ..
        } = tail
        {
            if let (Some(last), Expr::Move(read, _)) = (lets.last(), tested.as_ref()) {
                if matches!(&last.pat, Pat::Bind(bound) if bound == read) {
                    candidates.push((lets.len() - 1, last.value.clone(), Form::Held));
                }
            }
            candidates.push((lets.len(), tested.as_ref().clone(), Form::If));
        }
        for (position, binding) in lets.iter().enumerate().rev() {
            if matches!(binding.pat, Pat::Wild) {
                candidates.push((position, binding.value.clone(), Form::Same));
            }
        }
        if !fm {
            candidates.push((lets.len(), tail.clone(), Form::Id));
            for inverse in self.negations_of(tail) {
                candidates.push((lets.len(), inverse, Form::Neg));
            }
        }
        let mut failures = Vec::new();
        for (split, ce, form) in candidates {
            let (lc, lk) = lets.split_at(split);
            let attempt = (|| -> Found {
                let condition_d = self.bg(g, fl, false, condition, &Ty::Bool, lc, &ce)?;
                let k_d = self.k(
                    g,
                    fl,
                    &Kind::Plain,
                    fm,
                    &ce,
                    then_branch,
                    else_branch,
                    ty,
                    lk,
                    tail,
                    form,
                )?;
                let jd = self.jd_b(fm, term, ty, lets, tail)?;
                let fm_term = self.boolean(fm);
                let lc_term = self.lets(lc);
                let ce_term = self.rexpr(&ce);
                let lk_term = self.lets(lk);
                let c0 = self.calc(condition)?;
                let tail_term = self.rexpr(tail);
                self.rule(
                    "cond",
                    g,
                    fl,
                    jd,
                    &[
                        ("fm", fm_term),
                        ("c0", c0),
                        ("lc", lc_term),
                        ("ce", ce_term),
                        ("lk", lk_term),
                        ("tk", tail_term),
                    ],
                    &[sub(condition_d), sub(k_d)],
                )
            })();
            match attempt {
                Ok(found) => return Ok(found),
                Err(reason) => failures.push(reason),
            }
        }
        // The branches are the Boolean literals: the conditional is its
        // condition.
        let attempt = (|| -> Found {
            let condition_d = self.bg(g, fl, fm, condition, &Ty::Bool, lets, tail)?;
            let yes = self.bg(
                g,
                fl,
                false,
                then_branch,
                &Ty::Bool,
                &[],
                &Self::lit_bool(true),
            )?;
            let no = self.bg(
                g,
                fl,
                false,
                else_branch,
                &Ty::Bool,
                &[],
                &Self::lit_bool(false),
            )?;
            let jd = self.jd_b(fm, term, ty, lets, tail)?;
            let fm_term = self.boolean(fm);
            self.rule(
                "condId",
                g,
                fl,
                jd,
                &[("fm", fm_term)],
                &[sub(condition_d), sub(yes), sub(no)],
            )
        })();
        match attempt {
            Ok(found) => Ok(found),
            Err(reason) => {
                failures.push(reason);
                Err(format!("no conditional form fits: {}", failures.join("; ")))
            }
        }
    }

    /// The scope and rendering of a branch taken when the condition fails.
    fn else_side(
        &mut self,
        g: &Ctx,
        kind: &Kind,
        block: &Block,
        body: &Term,
    ) -> Result<(Ctx, Vec<Let>, Expr), String> {
        match kind {
            Kind::Plain => Ok((g.clone(), block.lets.clone(), block.tail.clone())),
            Kind::Nat {
                holder,
                binder,
                slot,
            } => {
                let inner = self.ctx_with(g, *binder, &Ty::Nat, *slot);
                match slot {
                    None => Ok((inner, block.lets.clone(), block.tail.clone())),
                    Some(m) => {
                        let is_pred = |binding: &Let| {
                            matches!(&binding.pat, Pat::Bind(Ident::Local(bound)) if bound == m)
                                && matches!(&binding.value, Expr::Predecessor(read) if read == holder)
                        };
                        if let Some((first, rest)) = block.lets.split_first() {
                            if is_pred(first) {
                                return Ok((inner, rest.to_vec(), block.tail.clone()));
                            }
                        }
                        // The fold took `let v = m - 1; v` to `m - 1`.
                        if block.lets.is_empty()
                            && matches!(&block.tail, Expr::Predecessor(read) if read == holder)
                            && matches!(body, Term::Var { name } if name == m)
                        {
                            return Ok((
                                inner,
                                Vec::new(),
                                Expr::Copy(
                                    Ident::Local(*m),
                                    crate::calculus::rust::ast::Origin::of("var"),
                                ),
                            ));
                        }
                        Err("the successor branch does not bind the predecessor".to_owned())
                    }
                }
            }
        }
    }

    /// `.k kind fm ce a b ty lets tail` in `form`.
    #[allow(clippy::too_many_arguments)]
    fn k(
        &mut self,
        g: &Ctx,
        fl: bool,
        kind: &Kind,
        fm: bool,
        ce: &Expr,
        a: &Term,
        b: &Term,
        ty: &Ty,
        lets: &[Let],
        tail: &Expr,
        form: Form,
    ) -> Found {
        let fm_term = self.boolean(fm);
        let kind_term = self.kind_term(kind);
        let ce_term = self.rexpr(ce);
        match form {
            Form::Held | Form::If => {
                let Expr::If {
                    condition,
                    then_branch,
                    else_branch,
                    ..
                } = tail
                else {
                    return Err("not an `if`".to_owned());
                };
                let mut held_ident = None;
                match form {
                    Form::Held => {
                        let [held] = lets else {
                            return Err("a held condition binds one Boolean".to_owned());
                        };
                        if !self.same(&held.value, ce) {
                            return Err("the held condition is another".to_owned());
                        }
                        held_ident = Some(held.clone());
                    }
                    _ => {
                        if !lets.is_empty() || !self.same(condition, ce) {
                            return Err("the `if` tests another condition".to_owned());
                        }
                    }
                }
                let (else_ctx, else_rest, else_tail) = self.else_side(g, kind, else_branch, b)?;
                let then_d = self.bg(g, fl, fm, a, ty, &then_branch.lets, &then_branch.tail)?;
                let else_d = self.bg(&else_ctx, fl, fm, b, ty, &else_rest, &else_tail)?;
                let jd = self.jd_k(kind, fm, ce, a, b, ty, lets, tail)?;
                let la = self.lets(&then_branch.lets);
                let ta = self.rexpr(&then_branch.tail);
                let lb = self.lets(&else_rest);
                let tb = self.rexpr(&else_tail);
                let mut hints = vec![
                    ("fm", fm_term),
                    ("kind", kind_term),
                    ("ce", ce_term),
                    ("la", la),
                    ("ta", ta),
                    ("lb", lb),
                    ("tb", tb),
                ];
                if form == Form::Held {
                    let Some(Let {
                        pat: Pat::Bind(held),
                        ..
                    }) = held_ident
                    else {
                        return Err("a held condition is bound to a temporary".to_owned());
                    };
                    let (k, j) = self
                        .w
                        .rs_ident_parts(&held)
                        .ok_or("a held condition is bound to a generated name")?;
                    hints.push(("k", k));
                    hints.push(("j", j));
                    self.rule(
                        "kHeld",
                        g,
                        fl,
                        jd,
                        &hints,
                        &[sub(then_d), sub(else_d), Decided, Decided],
                    )
                } else {
                    self.rule("kIf", g, fl, jd, &hints, &[sub(then_d), sub(else_d)])
                }
            }
            Form::Same => {
                let Some((first, rest)) = lets.split_first() else {
                    return Err("the same branches follow the condition".to_owned());
                };
                if !matches!(first.pat, Pat::Wild) || !self.same(&first.value, ce) {
                    return Err("the condition is not evaluated first".to_owned());
                }
                let else_ctx = self.else_side(g, kind, &Block::of(Self::lit_unit()), b)?.0;
                let then_d = self.bg(g, fl, fm, a, ty, rest, tail)?;
                let else_d = self.bg(&else_ctx, fl, fm, b, ty, rest, tail)?;
                let jd = self.jd_k(kind, fm, ce, a, b, ty, lets, tail)?;
                self.rule(
                    "kSame",
                    g,
                    fl,
                    jd,
                    &[("fm", fm_term), ("kind", kind_term), ("ce", ce_term)],
                    &[sub(then_d), sub(else_d), Decided],
                )
            }
            Form::Id | Form::Neg => {
                if !lets.is_empty() || fm {
                    return Err("a literal conditional binds nothing".to_owned());
                }
                let wanted = if form == Form::Id {
                    ce.clone()
                } else {
                    negate(ce)
                };
                if !self.same(&wanted, tail) {
                    return Err("the conditional is not its condition".to_owned());
                }
                let else_ctx = self.else_side(g, kind, &Block::of(Self::lit_unit()), b)?.0;
                let then_d = self.bg(
                    g,
                    fl,
                    false,
                    a,
                    &Ty::Bool,
                    &[],
                    &Self::lit_bool(form == Form::Id),
                )?;
                let else_d = self.bg(
                    &else_ctx,
                    fl,
                    false,
                    b,
                    &Ty::Bool,
                    &[],
                    &Self::lit_bool(form != Form::Id),
                )?;
                let jd = self.jd_k(kind, false, ce, a, b, &Ty::Bool, lets, tail)?;
                self.rule(
                    if form == Form::Id { "kId" } else { "kNeg" },
                    g,
                    fl,
                    jd,
                    &[("kind", kind_term), ("ce", ce_term)],
                    &[sub(then_d), sub(else_d), Decided],
                )
            }
        }
    }

    /// `.k` in whichever form the rendering takes.
    #[allow(clippy::too_many_arguments)]
    fn k_any(
        &mut self,
        g: &Ctx,
        fl: bool,
        kind: &Kind,
        fm: bool,
        ce: &Expr,
        a: &Term,
        b: &Term,
        ty: &Ty,
        lets: &[Let],
        tail: &Expr,
    ) -> Found {
        let mut failures = Vec::new();
        for form in [Form::Held, Form::If, Form::Same, Form::Id, Form::Neg] {
            match self.k(g, fl, kind, fm, ce, a, b, ty, lets, tail, form) {
                Ok(found) => return Ok(found),
                Err(reason) => failures.push(reason),
            }
        }
        Err(failures.join("; "))
    }

    // --- matches ------------------------------------------------------------------

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn match_(
        &mut self,
        g: &Ctx,
        fl: bool,
        fm: bool,
        term: &Term,
        ty: &Ty,
        lets: &[Let],
        tail: &Expr,
    ) -> Found {
        let Term::Match {
            scrutinee, arms, ..
        } = term
        else {
            return Err("not a match".to_owned());
        };
        let st = self.type_of(g, scrutinee)?;
        let fm_term = self.boolean(fm);
        let holder = lets.first().and_then(|first| match &first.pat {
            Pat::Bind(held @ Ident::Holder(_)) => Some((held.clone(), first)),
            _ => None,
        });
        match &st {
            Ty::Unit => {
                let [arm] = arms.as_slice() else {
                    return Err("a match on unit has one arm".to_owned());
                };
                let Some((first, rest)) = lets.split_first() else {
                    return Err("a match on unit binds its scrutinee".to_owned());
                };
                if !matches!(first.pat, Pat::Unit) {
                    return Err("a match on unit binds `()`".to_owned());
                }
                let scrutinee_d = self.eg(g, fl, scrutinee, &Ty::Unit, &first.value)?;
                let body_d = self.bg(g, fl, fm, &arm.body, ty, rest, tail)?;
                let jd = self.jd_b(fm, term, ty, lets, tail)?;
                self.rule(
                    "matchUnit",
                    g,
                    fl,
                    jd,
                    &[("fm", fm_term)],
                    &[sub(scrutinee_d), sub(body_d)],
                )
            }
            Ty::Nat => {
                let (held, first) = holder.ok_or("a match on a natural holds it")?;
                let (a, x, b) = nat_arms(arms)?;
                let scrutinee_d = self.eg(g, fl, scrutinee, &Ty::Nat, &first.value)?;
                let ce = Expr::IsZero(
                    held.clone(),
                    crate::calculus::rust::ast::Origin::of("shape:zero"),
                );
                let rest = &lets[1..];
                let mut failures = Vec::new();
                for form in [Form::If, Form::Held, Form::Same, Form::Id, Form::Neg] {
                    let slot = match (form, tail) {
                        (Form::If | Form::Held, Expr::If { else_branch, .. }) => {
                            pred_slot(&held, else_branch, b, x)
                        }
                        _ => None,
                    };
                    let kind = Kind::Nat {
                        holder: held.clone(),
                        binder: x,
                        slot,
                    };
                    match self.k(g, fl, &kind, fm, &ce, a, b, ty, rest, tail, form) {
                        Ok(k_d) => {
                            let jd = self.jd_b(fm, term, ty, lets, tail)?;
                            let sl = self.slot(slot);
                            let a_term = self.calc(a)?;
                            let x_term = self.nat(x);
                            let b_term = self.calc(b)?;
                            return self.rule(
                                "matchNat",
                                g,
                                fl,
                                jd,
                                &[
                                    ("fm", fm_term),
                                    ("sl", sl),
                                    ("a", a_term),
                                    ("x", x_term),
                                    ("b", b_term),
                                ],
                                &[sub(scrutinee_d), Decided, Decided, Decided, sub(k_d)],
                            );
                        }
                        Err(reason) => failures.push(reason),
                    }
                }
                Err(format!(
                    "no form of a match on a natural fits: {}",
                    failures.join("; ")
                ))
            }
            Ty::Bool => {
                let (a, b) = bool_arms(arms)?;
                if let Some((held, first)) = holder {
                    let attempt = (|| -> Found {
                        let scrutinee_d = self.eg(g, fl, scrutinee, &Ty::Bool, &first.value)?;
                        let ce = Expr::Move(
                            held.clone(),
                            crate::calculus::rust::ast::Origin::of("expr:match"),
                        );
                        let k_d =
                            self.k_any(g, fl, &Kind::Plain, fm, &ce, a, b, ty, &lets[1..], tail)?;
                        let jd = self.jd_b(fm, term, ty, lets, tail)?;
                        let a_term = self.calc(a)?;
                        let b_term = self.calc(b)?;
                        self.rule(
                            "matchBool",
                            g,
                            fl,
                            jd,
                            &[("fm", fm_term), ("a", a_term), ("b", b_term)],
                            &[sub(scrutinee_d), Decided, Decided, sub(k_d)],
                        )
                    })();
                    if attempt.is_ok() {
                        return attempt;
                    }
                }
                let scrutinee_d = self.bg(g, fl, fm, scrutinee, &Ty::Bool, lets, tail)?;
                let yes = self.bg(g, fl, false, a, &Ty::Bool, &[], &Self::lit_bool(true))?;
                let no = self.bg(g, fl, false, b, &Ty::Bool, &[], &Self::lit_bool(false))?;
                let jd = self.jd_b(fm, term, ty, lets, tail)?;
                let a_term = self.calc(a)?;
                let b_term = self.calc(b)?;
                self.rule(
                    "matchBoolId",
                    g,
                    fl,
                    jd,
                    &[("fm", fm_term), ("a", a_term), ("b", b_term)],
                    &[sub(scrutinee_d), Decided, sub(yes), sub(no)],
                )
            }
            Ty::Pair { left, right } => {
                let [arm] = arms.as_slice() else {
                    return Err("a match on a pair has one arm".to_owned());
                };
                let (held, first) = holder.ok_or("a match on a pair holds it")?;
                let Some(second) = lets.get(1) else {
                    return Err("a match on a pair splits it".to_owned());
                };
                let Pat::Tuple(parts) = &second.pat else {
                    return Err("a match on a pair splits it".to_owned());
                };
                if !matches!(&second.value, Expr::Move(read, _) if *read == held) {
                    return Err("a match on a pair splits its holder".to_owned());
                }
                let types = [left.as_ref().clone(), right.as_ref().clone()];
                let inner = self.param_ctx(g, &arm.binders, &types, parts)?;
                let scrutinee_d = self.eg(g, fl, scrutinee, &st, &first.value)?;
                let body_d = self.bg(&inner, fl, fm, &arm.body, ty, &lets[2..], tail)?;
                let jd = self.jd_b(fm, term, ty, lets, tail)?;
                let ta = self.ty(left);
                let tb = self.ty(right);
                self.rule(
                    "matchPair",
                    g,
                    fl,
                    jd,
                    &[("fm", fm_term), ("Γ''", inner.term), ("ta", ta), ("tb", tb)],
                    &[sub(scrutinee_d), Decided, sub(body_d), Decided],
                )
            }
            Ty::Option { .. }
            | Ty::Result { .. }
            | Ty::List { .. }
            | Ty::Ordering
            | Ty::Adt { .. } => {
                let view = matches!(st, Ty::List { .. });
                if let (
                    Some((held, first)),
                    [_],
                    Expr::Match {
                        scrutinee: matched,
                        arms: rarms,
                        ..
                    },
                ) = (holder, lets, tail)
                {
                    let reads_holder = match matched.as_ref() {
                        Expr::Move(read, _) => !view && *read == held,
                        Expr::Uncons(read) => view && *read == held,
                        _ => false,
                    };
                    if reads_holder {
                        let attempt = (|| -> Found {
                            let scrutinee_d = self.eg(g, fl, scrutinee, &st, &first.value)?;
                            let (arms_d, idx) = self.mg(g, fl, &st, view, arms, fm, ty, rarms)?;
                            let jd = self.jd_b(fm, term, ty, lets, tail)?;
                            let st_term = self.ty(&st);
                            let empty = self.empty.clone();
                            let u = self.nats(&empty);
                            let view_term = self.boolean(view);
                            let idx = self.nats(&idx);
                            self.rule(
                                "matchArms",
                                g,
                                fl,
                                jd,
                                &[
                                    ("fm", fm_term),
                                    ("st", st_term),
                                    ("U", u),
                                    ("view", view_term),
                                    ("idx", idx),
                                ],
                                &[sub(scrutinee_d), sub(arms_d), Decided, Decided, Decided],
                            )
                        })();
                        if attempt.is_ok() {
                            return attempt;
                        }
                    }
                }
                if view || *ty != st {
                    return Err("the match is neither rendered nor its scrutinee".to_owned());
                }
                // Every arm rebuilds what it matched: the match is its
                // scrutinee, and the rebuilding arms are stated here.
                let rebuilt = self.rebuilt_arms(&st, arms)?;
                let scrutinee_d = self.bg(g, fl, fm, scrutinee, &st, lets, tail)?;
                let (arms_d, idx) = self.mg(g, fl, &st, false, arms, false, &st, &rebuilt)?;
                let jd = self.jd_b(fm, term, ty, lets, tail)?;
                let empty = self.empty.clone();
                let u = self.nats(&empty);
                let idx = self.nats(&idx);
                let rarms = self.w.rs_arms(&rebuilt);
                self.rule(
                    "matchRebuilt",
                    g,
                    fl,
                    jd,
                    &[("fm", fm_term), ("U", u), ("idx", idx), ("rarms", rarms)],
                    &[sub(scrutinee_d), sub(arms_d), Decided, Decided, Decided],
                )
            }
            other => Err(format!("a match on {other:?}")),
        }
    }

    /// The scope parameter patterns open, as `paramCtx`: each pattern a local
    /// or nothing, every local free in the scope it extends.
    pub(super) fn param_ctx(
        &mut self,
        base: &Ctx,
        names: &[u64],
        types: &[Ty],
        patterns: &[Pat],
    ) -> Result<Ctx, String> {
        if names.len() != types.len() || types.len() != patterns.len() {
            return Err("binders, types, and patterns differ in number".to_owned());
        }
        let mut out = base.clone();
        for ((name, ty), pattern) in names.iter().zip(types).zip(patterns) {
            let slot = slot_of(pattern)?;
            if let Some(m) = slot {
                if !out.slot_free(m) {
                    return Err(format!("local v{m} is already held"));
                }
            }
            out = self.ctx_with(&out, *name, ty, slot);
        }
        Ok(out)
    }

    /// The arms of a rebuilding match, as the rendering would state them.
    fn rebuilt_arms(&self, st: &Ty, arms: &[Arm]) -> Result<Vec<(Pat, Block)>, String> {
        let origin = || crate::calculus::rust::ast::Origin::of("rebuilt");
        let mut out = Vec::new();
        for arm in arms {
            let fields = self.checker.shape_fields(arm.shape, st)?;
            if fields.iter().any(|ty| self.uninh(&self.empty, ty)) {
                continue;
            }
            let binders: Vec<(Pat, Expr)> = arm
                .binders
                .iter()
                .zip(&fields)
                .map(|(name, ty)| {
                    if *ty == Ty::Unit {
                        (Pat::Wild, Self::lit_unit())
                    } else {
                        (
                            Pat::Bind(Ident::Local(*name)),
                            Expr::Copy(Ident::Local(*name), origin()),
                        )
                    }
                })
                .collect();
            let (patterns, args): (Vec<Pat>, Vec<Expr>) = binders.into_iter().unzip();
            let one = |patterns: &[Pat]| patterns.first().cloned().ok_or("an arm lacks its binder");
            let (pattern, tail) = match (arm.shape, st) {
                (Shape::None, Ty::Option { .. }) => (
                    Pat::None,
                    Expr::Construct {
                        ctor: Ctor::None(Type::Unit),
                        args: vec![],
                        at: origin(),
                    },
                ),
                (Shape::Some, Ty::Option { .. }) => (
                    Pat::Some(Box::new(one(&patterns)?)),
                    Expr::Construct {
                        ctor: Ctor::Some,
                        args,
                        at: origin(),
                    },
                ),
                (Shape::Ok, Ty::Result { .. }) => (
                    Pat::Ok(Box::new(one(&patterns)?)),
                    Expr::Construct {
                        ctor: Ctor::Ok(Type::Unit, Type::Unit),
                        args,
                        at: origin(),
                    },
                ),
                (Shape::Error, Ty::Result { .. }) => (
                    Pat::Err(Box::new(one(&patterns)?)),
                    Expr::Construct {
                        ctor: Ctor::Err(Type::Unit, Type::Unit),
                        args,
                        at: origin(),
                    },
                ),
                (Shape::Lt | Shape::Eq | Shape::Gt, Ty::Ordering) => {
                    let value = match arm.shape {
                        Shape::Lt => crate::calculus::OrderingValue::Lt,
                        Shape::Eq => crate::calculus::OrderingValue::Eq,
                        _ => crate::calculus::OrderingValue::Gt,
                    };
                    (
                        Pat::Ordering(value),
                        Expr::Lit(Lit::Ordering(value), origin()),
                    )
                }
                (Shape::Adt { constructor }, Ty::Adt { index }) => (
                    Pat::Adt {
                        adt: *index,
                        constructor,
                        fields: patterns,
                    },
                    Expr::Construct {
                        ctor: Ctor::Adt {
                            adt: *index,
                            constructor,
                        },
                        args,
                        at: origin(),
                    },
                ),
                (shape, ty) => return Err(format!("{shape:?} at {ty:?} does not rebuild")),
            };
            out.push((pattern, Block::of(tail)));
        }
        Ok(out)
    }

    /// `.m st view arms idx fm τ rarms`, and `idx`.
    #[allow(clippy::too_many_arguments)]
    fn m(
        &mut self,
        g: &Ctx,
        fl: bool,
        st: &Ty,
        view: bool,
        arms: &[Arm],
        fm: bool,
        tau: &Ty,
        rarms: &[(Pat, Block)],
    ) -> Result<(Pf, Vec<u64>), String> {
        // The arms are derived last to first, each wrapped around the
        // derivation of those after it; a loop and not a recursion, because
        // a match has as many arms as its source states, which a configured
        // limit allows to be as many as the stack could not hold.
        let st_term = self.ty(st);
        let view_term = self.boolean(view);
        let fm_term = self.boolean(fm);
        let tau_term = self.ty(tau);
        let arms_term = self.w.arms_list(arms)?;
        let nat = self.w.b.konst("Nat");
        let arm_ty = self.rs_arm_ty();
        let mut idx_term = self.w.b.list(nat, &[]);
        let mut rarms_term = self.w.b.list(arm_ty, &[]);
        let nil_jd = self.jd(
            "m",
            &[
                st_term, view_term, arms_term, idx_term, fm_term, tau_term, rarms_term,
            ],
        );
        let mut derivation = self.rule("mNil", g, fl, nil_jd, &[], &[])?;
        let mut indices = Vec::with_capacity(rarms.len());
        let mut cursor = arms.len();
        for (pattern, block) in rarms.iter().rev() {
            self.refused()?;
            let (next, index, arm_term) = self.m_arm(
                g,
                fl,
                st,
                view,
                arms,
                &mut cursor,
                fm,
                tau,
                pattern,
                block,
                derivation,
                [st_term, view_term, arms_term, fm_term, tau_term],
                (idx_term, rarms_term),
            )?;
            derivation = self.within_budget(next)?;
            let index_term = self.nat(index);
            let cons = self.w.b.konst0("List.cons");
            idx_term = self.w.b.apps(cons, &[nat, index_term, idx_term]);
            rarms_term = self.w.b.apps(cons, &[arm_ty, arm_term, rarms_term]);
            indices.push(index);
        }
        indices.reverse();
        Ok((derivation, indices))
    }

    fn rs_arm_ty(&mut self) -> K {
        self.w.rs("Arm")
    }

    /// One arm of `.m`: its derivation wrapped around `rest_d`, the
    /// derivation of the arms after it, the index of its source arm, and the
    /// Rust arm term it states.
    #[allow(clippy::too_many_arguments)]
    fn m_arm(
        &mut self,
        g: &Ctx,
        fl: bool,
        st: &Ty,
        view: bool,
        arms: &[Arm],
        cursor: &mut usize,
        fm: bool,
        tau: &Ty,
        pattern: &Pat,
        block: &Block,
        rest_d: Pf,
        fixed: [K; 5],
        suffix: (K, K),
    ) -> Result<(Pf, u64, K), String> {
        let [st_term, view_term, arms_term, fm_term, tau_term] = fixed;
        let (idx_rest, rarms_rest) = suffix;
        let (shape, inner) = arm_outer(st, view, pattern)?;
        let j = arms[..*cursor]
            .iter()
            .rposition(|arm| arm.shape == shape)
            .inspect(|&found| *cursor = found)
            .or_else(|| arms.iter().position(|arm| arm.shape == shape))
            .ok_or_else(|| format!("no arm has shape {shape:?}"))?;
        let arm = &arms[j];
        let types = self.checker.shape_fields(shape, st)?;
        let boxed = inner
            .iter()
            .filter(|pattern| matches!(pattern, Pat::Bind(Ident::Boxed(_))))
            .count();
        // The loads the arm begins with, the last one perhaps folded into
        // the arm's value.
        let mut loads: Vec<Let> = block.lets.iter().take(boxed).cloned().collect();
        let mut lets: Vec<Let> = block
            .lets
            .get(boxed..)
            .map(<[Let]>::to_vec)
            .unwrap_or_default();
        let mut tail = block.tail.clone();
        if loads.len() < boxed || !loads.iter().all(is_load) {
            loads = block
                .lets
                .iter()
                .take(boxed.saturating_sub(1))
                .cloned()
                .collect();
            lets = Vec::new();
            let Expr::Unbox(Ident::Boxed(last), _) = &block.tail else {
                return Err("an arm does not load its boxed fields".to_owned());
            };
            let Term::Var { name } = &arm.body else {
                return Err("only a variable folds into its load".to_owned());
            };
            if block.lets.len() != boxed - 1 {
                return Err("an arm's loads are not all there".to_owned());
            }
            loads.push(Let {
                pat: Pat::Bind(Ident::Local(*name)),
                ty: None,
                value: Expr::Unbox(
                    Ident::Boxed(*last),
                    crate::calculus::rust::ast::Origin::of("indirection"),
                ),
                at: crate::calculus::rust::ast::Origin::of("indirection"),
            });
            tail = Expr::Copy(
                Ident::Local(*name),
                crate::calculus::rust::ast::Origin::of("var"),
            );
        }
        let direct = direct_patterns(&inner, &loads)?;
        let inner_ctx = self.param_ctx(g, &arm.binders, &types, &direct)?;
        let body_d = self.bg(&inner_ctx, fl, fm, &arm.body, tau, &lets, &tail)?;
        // The judgment: this arm consed onto those after it. The lemma states
        // the arm folded, which the kernel computes from the arm the crate
        // states.
        let mut joined = loads.clone();
        joined.extend(lets.iter().cloned());
        let q = self.w.rs_pat(pattern);
        let written = self.w.rs_arm(pattern, block);
        let nat = self.w.b.konst("Nat");
        let arm_ty = self.rs_arm_ty();
        let index_term = self.nat(j as u64);
        let cons = self.w.b.konst0("List.cons");
        let idx_all = self.w.b.apps(cons, &[nat, index_term, idx_rest]);
        let rarms_all = self.w.b.apps(cons, &[arm_ty, written, rarms_rest]);
        let jd = self.jd(
            "m",
            &[
                st_term, view_term, arms_term, idx_all, fm_term, tau_term, rarms_all,
            ],
        );
        let gamma_inner = inner_ctx.term;
        let j_term = self.nat(j as u64);
        let shape_term = self.w.shape_term(shape);
        let xs = self.nats(&arm.binders);
        let body = self.calc(&arm.body)?;
        let loads_term = self.lets(&loads);
        let lets_term = self.lets(&lets);
        let tail_term = self.rexpr(&tail);
        let rule = self.rule(
            "mCons",
            g,
            fl,
            jd,
            &[
                ("Γ'", gamma_inner),
                ("fm", fm_term),
                ("j", j_term),
                ("shape", shape_term),
                ("xs", xs),
                ("body", body),
                ("q", q),
                ("loads", loads_term),
                ("lets", lets_term),
                ("tail", tail_term),
                ("rarms", rarms_rest),
                ("idx", idx_rest),
                ("st", st_term),
                ("view", view_term),
                ("arms", arms_term),
                ("τ", tau_term),
            ],
            &[sub(rest_d), Decided, Decided, Decided, sub(body_d)],
        )?;
        Ok((rule, j as u64, written))
    }

    // --- record fields ------------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    fn field(
        &mut self,
        g: &Ctx,
        fl: bool,
        fm: bool,
        term: &Term,
        ty: &Ty,
        lets: &[Let],
        tail: &Expr,
    ) -> Found {
        let Term::Field { value, .. } = term else {
            return Err("not a field".to_owned());
        };
        let Expr::Match {
            scrutinee, arms, ..
        } = tail
        else {
            return Err("a field is read by a match".to_owned());
        };
        let [(Pat::Adt { adt, fields, .. }, block)] = arms.as_slice() else {
            return Err("a field is read by one arm".to_owned());
        };
        let record_ty = Ty::Adt { index: *adt };
        let types = self
            .checker
            .shape_fields(Shape::Adt { constructor: 0 }, &record_ty)?;
        let read = if fm {
            match (block.lets.as_slice(), &block.tail) {
                ([binding], Expr::Succeed(..)) if *ty == Ty::Unit => binding.value.clone(),
                ([], Expr::Succeed(inner, _)) => inner.as_ref().clone(),
                _ => return Err("a field in a fallible tail is `Ok` of the read".to_owned()),
            }
        } else {
            block.tail.clone()
        };
        // The part the pattern binds names the read.
        let part = fields
            .iter()
            .find_map(|field| match field {
                Pat::Bind(held @ Ident::Part(_)) => Some(held.clone()),
                _ => None,
            })
            .or_else(|| {
                fields.iter().find_map(|field| match field {
                    Pat::Bind(held) => Some(held.clone()),
                    _ => None,
                })
            });
        let Some(part) = part else {
            return Err("a field read binds no part".to_owned());
        };
        let (kind_part, index_part) = self
            .w
            .rs_ident_parts(&part)
            .ok_or("a field is read into a generated name")?;
        let ts = self.types(&types);
        let rd = self.rexpr(&read);
        let fm_term = self.boolean(fm);
        let jd = self.jd_b(fm, term, ty, lets, tail)?;
        let mut hints = vec![
            ("fm", fm_term),
            ("ts", ts),
            ("rd", rd),
            ("k", kind_part),
            ("j", index_part),
        ];
        match lets {
            [] => {
                let record_d = self.eg(g, fl, value, &record_ty, scrutinee)?;
                self.rule(
                    "fieldRead",
                    g,
                    fl,
                    jd,
                    &hints,
                    &[
                        sub(record_d),
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                    ],
                )
            }
            [binding] => {
                let record_d = self.eg(g, fl, value, &record_ty, &binding.value)?;
                if let Pat::Bind(held) = &binding.pat {
                    if let Some((kind_held, index_held)) = self.w.rs_ident_parts(held) {
                        hints.push(("k'", kind_held));
                        hints.push(("j'", index_held));
                    }
                }
                self.rule(
                    "fieldHeld",
                    g,
                    fl,
                    jd,
                    &hints,
                    &[
                        sub(record_d),
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                    ],
                )
            }
            _ => Err("a field binds at most its record".to_owned()),
        }
    }

    /// The `apply` flags of every function type the program closes over or
    /// applies, as `Flags`.
    pub(super) fn flags(&mut self) -> Result<K, String> {
        let ty_ty = self.target("Ty");
        let list_ty = self.w.b.list_ty(ty_ty);
        let inner_ty = self.w.b.pair_ty(list_ty, ty_ty);
        let bool = self.w.b.konst("Bool");
        let element_ty = self.w.b.pair_ty(inner_ty, bool);
        let mut entries = Vec::new();
        let fn_types: Vec<(u64, Ty)> = self
            .fn_types
            .iter()
            .map(|(position, ty)| (*position, ty.clone()))
            .collect();
        for (position, ty) in fn_types {
            let Ty::Fn { parameters, result } = &ty else {
                return Err("a function type is not a function".to_owned());
            };
            let parameters = self.types(parameters);
            let result = self.ty(result);
            let signature = self.w.b.pair(list_ty, ty_ty, parameters, result);
            let flag = self.apply_fallible(position)?;
            let flag = self.boolean(flag);
            entries.push(self.w.b.pair(inner_ty, bool, signature, flag));
        }
        Ok(self.w.b.list(element_ty, &entries))
    }

    /// The bytes the certificate is certainly at least, by its nodes.
    pub(super) fn weighed(&self) -> u64 {
        (self.counted.len() as u64).saturating_mul(NODE_BYTES)
    }

    /// Count the nodes `root` reaches that no finished rule or literal has
    /// reached: each node once, so the whole count is linear.
    pub(super) fn count_new(&mut self, root: K) {
        let mut stack = vec![root];
        while let Some(term) = stack.pop() {
            if !self.counted.insert(term) {
                continue;
            }
            match self.w.b.node(term) {
                CoreNode::App { f, x } => {
                    stack.push(*f);
                    stack.push(*x);
                }
                CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => {
                    stack.push(*t);
                    stack.push(*v);
                }
                CoreNode::Let { t, v, body, .. } => {
                    stack.push(*t);
                    stack.push(*v);
                    stack.push(*body);
                }
                CoreNode::Mdata { x, .. } | CoreNode::Projection { x, .. } => stack.push(*x),
                CoreNode::BVar { .. }
                | CoreNode::Sort { .. }
                | CoreNode::Const { .. }
                | CoreNode::Nat { .. }
                | CoreNode::String { .. } => {}
            }
        }
    }

    pub(super) fn krate(&self) -> &'a Crate {
        self.krate
    }

    /// Every condition whose negation is `expr`.
    fn negations_of(&mut self, expr: &Expr) -> Vec<Expr> {
        let at = || crate::calculus::rust::ast::Origin::of("negate");
        let mut out = vec![Expr::Not(Box::new(expr.clone()), at())];
        match expr {
            Expr::Lit(Lit::Bool(value), _) => out.push(Expr::Lit(Lit::Bool(!value), at())),
            Expr::IsZero(ident, _) => out.push(Expr::NonZero(ident.clone(), at())),
            Expr::NonZero(ident, _) => out.push(Expr::IsZero(ident.clone(), at())),
            Expr::Not(inner, _) => out.push(inner.as_ref().clone()),
            _ => {}
        }
        let mut kept = Vec::new();
        for candidate in out {
            let negated = negate(&candidate);
            if self.same(&negated, expr) {
                kept.push(candidate);
            }
        }
        kept
    }
}

// --- helpers ---------------------------------------------------------------------

/// A construction's arguments with each boxed one read through its box, and
/// which were boxed, as `boxWith`.
fn unboxed(args: &[Expr]) -> (Vec<bool>, Vec<Expr>) {
    args.iter()
        .map(|arg| match arg {
            Expr::Box(inner, _) => (true, inner.as_ref().clone()),
            plain => (false, plain.clone()),
        })
        .unzip()
}

fn is_load(binding: &Let) -> bool {
    matches!(&binding.pat, Pat::Bind(Ident::Local(_)))
        && binding.ty.is_none()
        && matches!(&binding.value, Expr::Unbox(Ident::Boxed(_), _))
}

/// The field patterns with each boxed field read back by its load, as
/// `directPat`.
fn direct_patterns(inner: &[Pat], loads: &[Let]) -> Result<Vec<Pat>, String> {
    let mut loads = loads.iter();
    let mut out = Vec::new();
    for pattern in inner {
        match pattern {
            Pat::Wild | Pat::Bind(Ident::Local(_)) => out.push(pattern.clone()),
            Pat::Bind(Ident::Boxed(b)) => {
                let load = loads.next().ok_or("a boxed field is not loaded")?;
                match (&load.pat, &load.value) {
                    (Pat::Bind(Ident::Local(m)), Expr::Unbox(Ident::Boxed(read), _))
                        if read == b =>
                    {
                        out.push(Pat::Bind(Ident::Local(*m)));
                    }
                    _ => return Err("a load reads another field".to_owned()),
                }
            }
            other => return Err(format!("field pattern {other:?}")),
        }
    }
    if loads.next().is_some() {
        return Err("an arm loads more than its boxed fields".to_owned());
    }
    Ok(out)
}

/// The shape a Rust arm's pattern realizes and its field patterns, as
/// `armOuter`.
fn arm_outer(st: &Ty, view: bool, pattern: &Pat) -> Result<(Shape, Vec<Pat>), String> {
    Ok(match (st, view, pattern) {
        (Ty::Option { .. }, false, Pat::None) => (Shape::None, vec![]),
        (Ty::Option { .. }, false, Pat::Some(inner)) => (Shape::Some, vec![inner.as_ref().clone()]),
        (Ty::Result { .. }, false, Pat::Ok(inner)) => (Shape::Ok, vec![inner.as_ref().clone()]),
        (Ty::Result { .. }, false, Pat::Err(inner)) => (Shape::Error, vec![inner.as_ref().clone()]),
        (Ty::Ordering, false, Pat::Ordering(value)) => (
            match value {
                crate::calculus::OrderingValue::Lt => Shape::Lt,
                crate::calculus::OrderingValue::Eq => Shape::Eq,
                crate::calculus::OrderingValue::Gt => Shape::Gt,
            },
            vec![],
        ),
        (Ty::List { .. }, true, Pat::None) => (Shape::Nil, vec![]),
        (Ty::List { .. }, true, Pat::Some(inner)) => match inner.as_ref() {
            Pat::Tuple(parts) if parts.len() == 2 => (Shape::Cons, parts.clone()),
            other => return Err(format!("a list cell pattern {other:?}")),
        },
        (
            Ty::Adt { index },
            false,
            Pat::Adt {
                adt,
                constructor,
                fields,
            },
        ) if adt == index => (
            Shape::Adt {
                constructor: *constructor,
            },
            fields.clone(),
        ),
        (ty, _, pattern) => return Err(format!("pattern {pattern:?} at {ty:?}")),
    })
}

fn nat_arms(arms: &[Arm]) -> Result<(&Term, u64, &Term), String> {
    match arms {
        [zero, succ] | [succ, zero] if zero.shape == Shape::Zero && succ.shape == Shape::Succ => {
            match succ.binders.as_slice() {
                [binder] if zero.binders.is_empty() => Ok((&zero.body, *binder, &succ.body)),
                _ => Err("the arms of a natural bind one predecessor".to_owned()),
            }
        }
        _ => Err("a match on a natural has a zero arm and a successor arm".to_owned()),
    }
}

fn bool_arms(arms: &[Arm]) -> Result<(&Term, &Term), String> {
    match arms {
        [yes, no] | [no, yes] if yes.shape == Shape::True && no.shape == Shape::False => {
            Ok((&yes.body, &no.body))
        }
        _ => Err("a match on a Boolean has a true arm and a false arm".to_owned()),
    }
}

/// The local the successor branch binds the predecessor to, if it does.
fn pred_slot(holder: &Ident, branch: &Block, body: &Term, binder: u64) -> Option<u64> {
    if let Some(first) = branch.lets.first() {
        if let (Pat::Bind(Ident::Local(m)), Expr::Predecessor(read)) = (&first.pat, &first.value) {
            if read == holder {
                return Some(*m);
            }
        }
    }
    if branch.lets.is_empty()
        && matches!(&branch.tail, Expr::Predecessor(read) if read == holder)
        && matches!(body, Term::Var { name } if *name == binder)
    {
        return Some(binder);
    }
    None
}

/// The negation the rendering writes, as `negateR`.
fn negate(expr: &Expr) -> Expr {
    let at = || crate::calculus::rust::ast::Origin::of("negate");
    match expr {
        Expr::Lit(Lit::Bool(value), _) => Expr::Lit(Lit::Bool(!value), at()),
        Expr::IsZero(ident, _) => Expr::NonZero(ident.clone(), at()),
        Expr::NonZero(ident, _) => Expr::IsZero(ident.clone(), at()),
        Expr::Not(inner, _) => inner.as_ref().clone(),
        other => Expr::Not(Box::new(other.clone()), at()),
    }
}
