//! Certificate B (SPEC.md §17.17): the rendered crate of a lowered root
//! simulates the lowered program. The aligner walks each function's
//! calculus body beside the crate's Rust body and writes the derivation of
//! the correspondence `LexLeanPreservation.Rust.Corr` that relates them, one
//! rule application per construct; Lean checks every rule's side
//! conditions by evaluation, and the library's soundness theorem turns the
//! derivations into the simulation of every function. The aligner is a
//! search, not a proof: a derivation it writes that does not hold is
//! rejected by Lean, and a rendering it cannot relate is reported, never
//! certified.

use std::collections::BTreeMap;

use crate::calculus::check::Checker;
use crate::calculus::rust::ast::{
    Block, Callee, Crate, Ctor, Expr, Ident, ItemDef, Let, Lit, Pat, Type,
};
use crate::calculus::rust::runtime::Item;
use crate::calculus::{Arm, Expr as Term, Prim, Program, Shape, Ty};

use super::certificate::{render_program, render_ty};
use super::rust_term;

/// The library module whose soundness theorem certificate B applies.
pub const SOUNDNESS_MODULE: &str = "LexLeanPreservation.RustSound";

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

/// One certificate-B module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateB {
    /// The Lean module name.
    pub module: String,
    /// The module text.
    pub text: String,
    /// The fully qualified name of the simulation theorem.
    pub theorem: String,
}

impl CertificateB {
    /// The certificate as the preservation workspace stages and audits it.
    #[must_use]
    pub fn into_certificate(self) -> super::certificate::Certificate {
        super::certificate::Certificate {
            module: self.module,
            denote: self.theorem.clone(),
            theorem: self.theorem,
            text: self.text,
            imports: Vec::new(),
        }
    }
}

/// The module of a root's certificate B in `target`, below its certificate
/// A module.
#[must_use]
pub fn module_for(certificate_a: &str, target: &str) -> Option<String> {
    let suffix = match target {
        "rust-core" => "RustCore",
        "rust-std" => "RustStd",
        _ => return None,
    };
    Some(format!("{certificate_a}.{suffix}"))
}

// --- derivations -------------------------------------------------------------

/// One rule application: the rule, the implicit arguments its conclusion
/// does not determine, and its premises in order.
struct Rule {
    name: &'static str,
    named: Vec<(&'static str, String)>,
    premises: Vec<Premise>,
}

/// A premise: a sub-derivation, or a side condition Lean decides by
/// evaluation.
enum Premise {
    Rule(Rule),
    Decided,
}

use Premise::Decided;

fn rule(name: &'static str, named: Vec<(&'static str, String)>, premises: Vec<Premise>) -> Rule {
    Rule {
        name,
        named,
        premises,
    }
}

fn sub(derivation: Rule) -> Premise {
    Premise::Rule(derivation)
}

impl Rule {
    fn write(&self, out: &mut String) {
        out.push_str("(Corr.");
        out.push_str(self.name);
        for (name, value) in &self.named {
            out.push_str(" (");
            out.push_str(name);
            out.push_str(" := ");
            out.push_str(value);
            out.push(')');
        }
        for premise in &self.premises {
            out.push(' ');
            match premise {
                Premise::Rule(inner) => inner.write(out),
                Premise::Decided => out.push_str("rfl"),
            }
        }
        out.push(')');
    }

    fn text(&self) -> String {
        let mut out = String::new();
        self.write(&mut out);
        out
    }
}

// --- Lean terms --------------------------------------------------------------

fn list(items: impl IntoIterator<Item = String>) -> String {
    format!("[{}]", items.into_iter().collect::<Vec<_>>().join(", "))
}

fn lets_term(lets: &[Let]) -> String {
    list(lets.iter().map(rust_term::binding))
}

fn types_term(types: &[Ty]) -> String {
    list(types.iter().map(render_ty))
}

fn nats_term(items: &[u64]) -> String {
    list(items.iter().map(u64::to_string))
}

fn bools_term(items: &[bool]) -> String {
    list(items.iter().map(bool::to_string))
}

fn slot_term(slot: Option<u64>) -> String {
    slot.map_or_else(|| "none".to_owned(), |m| format!("(some {m})"))
}

/// Two Rust expressions are the same term when their Lean terms are, which
/// ignores the origins.
fn same(left: &Expr, right: &Expr) -> bool {
    rust_term::expr(left) == rust_term::expr(right)
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

// --- scopes ------------------------------------------------------------------

/// A scope: each program local with its type and the Rust local holding it,
/// innermost last.
#[derive(Debug, Clone, Default)]
struct Ctx(Vec<(u64, Ty, Option<u64>)>);

impl Ctx {
    fn with(&self, name: u64, ty: &Ty, slot: Option<u64>) -> Self {
        let mut out = self.clone();
        out.0.push((name, ty.clone(), slot));
        out
    }

    fn lookup(&self, name: u64) -> Option<&(u64, Ty, Option<u64>)> {
        self.0.iter().rev().find(|entry| entry.0 == name)
    }

    fn slot_free(&self, slot: u64) -> bool {
        self.0.iter().all(|entry| entry.2 != Some(slot))
    }

    fn scope(&self) -> Vec<(u64, Ty)> {
        self.0
            .iter()
            .map(|(name, ty, _)| (*name, ty.clone()))
            .collect()
    }

    /// The Lean list, innermost first.
    fn term(&self) -> String {
        list(
            self.0.iter().rev().map(|(name, ty, slot)| {
                format!("({name}, {}, {})", render_ty(ty), slot_term(*slot))
            }),
        )
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

/// The scope parameter patterns open, as `paramCtx`: each pattern a local
/// or nothing, every local free in the scope it extends.
fn param_ctx(base: &Ctx, names: &[u64], types: &[Ty], patterns: &[Pat]) -> Result<Ctx, String> {
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
        out = out.with(*name, ty, slot);
    }
    Ok(out)
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

struct Aligner<'a> {
    program: &'a Program,
    krate: &'a Crate,
    checker: Checker<'a>,
    /// Whether each function's rendering returns `R<T>`.
    fallible: Vec<bool>,
    /// The calculus function type of each rendered function type.
    fn_types: BTreeMap<u64, Ty>,
    /// The ADTs no value inhabits.
    empty: Vec<u64>,
}

type Found = Result<Rule, String>;

impl<'a> Aligner<'a> {
    fn new(program: &'a Program, krate: &'a Crate) -> Result<Self, String> {
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
        let mut aligner = Aligner {
            program,
            krate,
            checker: Checker { program },
            fallible,
            fn_types: BTreeMap::new(),
            empty: Vec::new(),
        };
        aligner.empty = aligner.empty_adts();
        Ok(aligner)
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

    fn uninh(&self, empty: &[u64], ty: &Ty) -> bool {
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

    fn function(&self, index: u64) -> Result<&'a crate::calculus::Function, String> {
        usize::try_from(index)
            .ok()
            .and_then(|position| self.program.functions.get(position))
            .ok_or_else(|| format!("function {index} is not declared"))
    }

    fn fallible_fn(&self, index: u64) -> bool {
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

    /// `.e term ty r`.
    fn e(&mut self, g: &Ctx, fl: bool, term: &Term, ty: &Ty, r: &Expr) -> Found {
        match (term, r) {
            (Term::Var { name }, _) => {
                let (_, bound, slot) = g
                    .lookup(*name)
                    .ok_or_else(|| format!("local {name} is unbound"))?;
                if *bound == Ty::Unit {
                    if !matches!(r, Expr::Lit(Lit::Unit, _)) {
                        return Err("a unit local is read as `()`".to_owned());
                    }
                    Ok(rule("varUnit", vec![], vec![Decided]))
                } else {
                    let read = match r {
                        Expr::Move(read, _)
                        | Expr::Clone(read, _)
                        | Expr::Copy(read, _)
                        | Expr::Deref(read, _)
                        | Expr::Unbox(read, _) => read,
                        _ => return Err("a local is not read".to_owned()),
                    };
                    if !matches!((read, slot), (Ident::Local(m), Some(held)) if m == held) {
                        return Err(format!("local {name} is read from another binding"));
                    }
                    Ok(rule("var", vec![], vec![Decided, Decided]))
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
                Ok(rule("value", vec![], vec![Decided, Decided]))
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
                Ok(rule("buildPair", vec![], vec![sub(operands_l)]))
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
                Ok(rule("buildConst", vec![], vec![Decided]))
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
                Ok(rule("buildSucc", vec![], vec![sub(operand)]))
            }
            (_, Expr::Block(block)) => {
                let inner = self.b(g, fl, false, term, ty, &block.lets, &block.tail)?;
                Ok(rule(
                    "eOfB",
                    vec![
                        ("lets", lets_term(&block.lets)),
                        ("tail", rust_term::expr(&block.tail)),
                    ],
                    vec![sub(inner)],
                ))
            }
            _ => {
                let inner = self.b(g, fl, false, term, ty, &[], r)?;
                Ok(rule("eOfBNil", vec![], vec![sub(inner)]))
            }
        }
    }

    /// `.l terms types rs`.
    fn l(&mut self, g: &Ctx, fl: bool, terms: &[Term], types: &[Ty], rs: &[Expr]) -> Found {
        match (terms, types, rs) {
            ([], [], []) => Ok(rule("lNil", vec![], vec![])),
            ([term, terms @ ..], [ty, types @ ..], [r, rs @ ..]) => {
                let head = self.e(g, fl, term, ty, r)?;
                let rest = self.l(g, fl, terms, types, rs)?;
                Ok(rule("lCons", vec![], vec![sub(head), sub(rest)]))
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
        if lets.is_empty() && !terms.is_empty() {
            let operands = self.l(g, fl, terms, types, args)?;
            return Ok(rule("opsOfL", vec![], vec![sub(operands)]));
        }
        match (terms, types, lets, args) {
            ([], [], [], []) => Ok(rule("opsNil", vec![], vec![])),
            ([term, terms @ ..], [ty, types @ ..], [binding, lets @ ..], [arg, args @ ..]) => {
                match (&binding.pat, arg) {
                    (Pat::Unit, Expr::Lit(Lit::Unit, _)) => {
                        let head = self.e(g, fl, term, &Ty::Unit, &binding.value)?;
                        let rest = self.ops(g, fl, terms, types, lets, args)?;
                        Ok(rule("opsUnit", vec![], vec![sub(head), sub(rest)]))
                    }
                    (Pat::Bind(temp), Expr::Move(read, _)) if temp == read => {
                        let head = self.e(g, fl, term, ty, &binding.value)?;
                        let rest = self.ops(g, fl, terms, types, lets, args)?;
                        Ok(rule(
                            "opsCons",
                            vec![],
                            vec![sub(head), sub(rest), Decided, Decided],
                        ))
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
                Ok(rule(
                    "build",
                    vec![("ts", types_term(&types)), ("mask", bools_term(&mask))],
                    vec![sub(operands_d), Decided],
                ))
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
                Ok(rule(
                    "callOps",
                    vec![("ts", types_term(&types))],
                    vec![sub(operands_d), Decided, Decided, Decided, Decided, Decided],
                ))
            }
            Term::Prim {
                operation,
                operands,
            } => self.prim(g, fl, false, operation, operands, lets, tail),
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
                Ok(rule(
                    "closure",
                    vec![
                        ("caps", types_term(&caps)),
                        ("mask", bools_term(&mask)),
                        ("af", af.to_string()),
                        ("ff", ff.to_string()),
                    ],
                    vec![
                        sub(operands_d),
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                    ],
                ))
            }
            Term::Apply { .. } => self.apply(g, fl, false, term, lets, tail),
            Term::First { value } | Term::Second { value } => {
                let first = matches!(term, Term::First { .. });
                let [binding] = lets else {
                    return Err("a projection binds one pair".to_owned());
                };
                let pair_ty = self.type_of(g, value)?;
                let Ty::Pair { left, right } = &pair_ty else {
                    return Err("a projection of a non-pair".to_owned());
                };
                let named = vec![("ta", render_ty(left)), ("tb", render_ty(right))];
                let scrutinee = self.e(g, fl, value, &pair_ty, &binding.value)?;
                Ok(rule(
                    if first { "first" } else { "second" },
                    named,
                    vec![sub(scrutinee), Decided],
                ))
            }
            Term::Var { .. } | Term::Value { .. } | Term::Build { .. } => {
                if !lets.is_empty() {
                    return Err("a value binds nothing".to_owned());
                }
                let value = self.e(g, fl, term, ty, tail)?;
                Ok(rule("bOfE", vec![], vec![sub(value)]))
            }
        }
    }

    /// `.b true term ty lets tail`: the tail of a fallible function.
    fn b_fallible(&mut self, g: &Ctx, term: &Term, ty: &Ty, lets: &[Let], tail: &Expr) -> Found {
        let mut failures = Vec::new();
        match self.b_structural(g, term, ty, lets, tail) {
            Ok(found) => return Ok(found),
            Err(reason) => failures.push(reason),
        }
        if let Expr::Succeed(inner, _) = tail {
            match self.b(g, true, false, term, ty, lets, inner) {
                Ok(value) => return Ok(rule("fOfB", vec![], vec![sub(value)])),
                Err(reason) => failures.push(reason),
            }
        }
        if let (Some((last, init)), true) = (lets.split_last(), *ty == Ty::Unit) {
            let unit_ok = matches!(tail, Expr::Succeed(inner, _) if matches!(inner.as_ref(), Expr::Lit(Lit::Unit, _)));
            if matches!(last.pat, Pat::Unit) && last.ty.is_none() && unit_ok {
                match self.b(g, true, false, term, ty, init, &last.value) {
                    Ok(value) => {
                        return Ok(rule(
                            "fUnit",
                            vec![("lets", lets_term(init))],
                            vec![sub(value)],
                        ));
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
                Ok(rule(
                    "callF",
                    vec![("ts", types_term(&types))],
                    vec![sub(operands_d), Decided, Decided, Decided, Decided],
                ))
            }
            Term::Prim {
                operation,
                operands,
            } => self.prim(g, true, true, operation, operands, lets, tail),
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
                Ok(rule("buildSuccF", vec![], vec![sub(operand)]))
            }
            Term::Apply { .. } => self.apply(g, true, true, term, lets, tail),
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
        let bound_d = self.e(g, fl, bound, bound_ty, &first.value)?;
        let inner = g.with(*name, bound_ty, slot);
        let body_d = self.b(&inner, fl, fm, body, ty, rest, tail)?;
        Ok(match slot {
            Some(_) => rule("letBind", vec![], vec![sub(bound_d), Decided, sub(body_d)]),
            None => rule("letWild", vec![], vec![sub(bound_d), sub(body_d)]),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn prim(
        &mut self,
        g: &Ctx,
        fl: bool,
        fm: bool,
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
            return Ok(rule(
                "primWiden",
                vec![("t0", render_ty(operand_ty))],
                vec![sub(operand_d), Decided, Decided],
            ));
        }
        let operands_d = self.ops(g, fl, operands, &types, lets, args)?;
        if fm {
            if *propagate {
                return Err("a fallible tail returns the primitive's result".to_owned());
            }
            Ok(rule(
                "primF",
                vec![("ts", types_term(&types))],
                vec![
                    sub(operands_d),
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                ],
            ))
        } else {
            Ok(rule(
                "prim",
                vec![("ts", types_term(&types))],
                vec![
                    sub(operands_d),
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                ],
            ))
        }
    }

    fn apply(
        &mut self,
        g: &Ctx,
        fl: bool,
        fm: bool,
        term: &Term,
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
        let target_d = self.e(g, fl, target, &target_ty, &first.value)?;
        let operands_d = self.ops(g, fl, operands, &parameters, rest, args)?;
        if fm {
            if *propagate {
                return Err("a fallible tail returns the application's result".to_owned());
            }
            Ok(rule(
                "applyF",
                vec![("ps", types_term(&parameters))],
                vec![sub(target_d), sub(operands_d), Decided, Decided, Decided],
            ))
        } else {
            Ok(rule(
                "apply",
                vec![
                    ("ps", types_term(&parameters)),
                    ("af", propagate.to_string()),
                ],
                vec![
                    sub(target_d),
                    sub(operands_d),
                    Decided,
                    Decided,
                    Decided,
                    Decided,
                ],
            ))
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
            for inverse in negations_of(tail) {
                candidates.push((lets.len(), inverse, Form::Neg));
            }
        }
        let mut failures = Vec::new();
        for (split, ce, form) in candidates {
            let (lc, lk) = lets.split_at(split);
            let attempt = (|| -> Found {
                let condition_d = self.b(g, fl, false, condition, &Ty::Bool, lc, &ce)?;
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
                Ok(rule(
                    "cond",
                    vec![
                        ("lc", lets_term(lc)),
                        ("ce", rust_term::expr(&ce)),
                        ("lk", lets_term(lk)),
                    ],
                    vec![sub(condition_d), sub(k_d)],
                ))
            })();
            match attempt {
                Ok(found) => return Ok(found),
                Err(reason) => failures.push(reason),
            }
        }
        // The branches are the Boolean literals: the conditional is its
        // condition.
        let attempt = (|| -> Found {
            let condition_d = self.b(g, fl, fm, condition, &Ty::Bool, lets, tail)?;
            let yes = self.b(g, fl, false, then_branch, &Ty::Bool, &[], &lit_bool(true))?;
            let no = self.b(g, fl, false, else_branch, &Ty::Bool, &[], &lit_bool(false))?;
            Ok(rule(
                "condId",
                vec![],
                vec![sub(condition_d), sub(yes), sub(no)],
            ))
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
                let inner = g.with(*binder, &Ty::Nat, *slot);
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
                match form {
                    Form::Held => {
                        let [held] = lets else {
                            return Err("a held condition binds one Boolean".to_owned());
                        };
                        if !same(&held.value, ce) {
                            return Err("the held condition is another".to_owned());
                        }
                    }
                    _ => {
                        if !lets.is_empty() || !same(condition, ce) {
                            return Err("the `if` tests another condition".to_owned());
                        }
                    }
                }
                let (else_ctx, else_rest, else_tail) = Self::else_side(g, kind, else_branch, b)?;
                let then_d = self.b(g, fl, fm, a, ty, &then_branch.lets, &then_branch.tail)?;
                let else_d = self.b(&else_ctx, fl, fm, b, ty, &else_rest, &else_tail)?;
                let named = vec![
                    ("la", lets_term(&then_branch.lets)),
                    ("ta", rust_term::expr(&then_branch.tail)),
                    ("lb", lets_term(&else_rest)),
                    ("tb", rust_term::expr(&else_tail)),
                ];
                Ok(if form == Form::Held {
                    rule(
                        "kHeld",
                        named,
                        vec![sub(then_d), sub(else_d), Decided, Decided],
                    )
                } else {
                    rule("kIf", named, vec![sub(then_d), sub(else_d)])
                })
            }
            Form::Same => {
                let Some((first, rest)) = lets.split_first() else {
                    return Err("the same branches follow the condition".to_owned());
                };
                if !matches!(first.pat, Pat::Wild) || !same(&first.value, ce) {
                    return Err("the condition is not evaluated first".to_owned());
                }
                let else_ctx = Self::else_side(g, kind, &Block::of(lit_unit()), b)?.0;
                let then_d = self.b(g, fl, fm, a, ty, rest, tail)?;
                let else_d = self.b(&else_ctx, fl, fm, b, ty, rest, tail)?;
                Ok(rule(
                    "kSame",
                    vec![],
                    vec![sub(then_d), sub(else_d), Decided],
                ))
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
                if !same(&wanted, tail) {
                    return Err("the conditional is not its condition".to_owned());
                }
                let else_ctx = Self::else_side(g, kind, &Block::of(lit_unit()), b)?.0;
                let then_d =
                    self.b(g, fl, false, a, &Ty::Bool, &[], &lit_bool(form == Form::Id))?;
                let else_d = self.b(
                    &else_ctx,
                    fl,
                    false,
                    b,
                    &Ty::Bool,
                    &[],
                    &lit_bool(form != Form::Id),
                )?;
                Ok(rule(
                    if form == Form::Id { "kId" } else { "kNeg" },
                    vec![],
                    vec![sub(then_d), sub(else_d), Decided],
                ))
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
                let scrutinee_d = self.e(g, fl, scrutinee, &Ty::Unit, &first.value)?;
                let body_d = self.b(g, fl, fm, &arm.body, ty, rest, tail)?;
                Ok(rule(
                    "matchUnit",
                    vec![],
                    vec![sub(scrutinee_d), sub(body_d)],
                ))
            }
            Ty::Nat => {
                let (held, first) = holder.ok_or("a match on a natural holds it")?;
                let (a, x, b) = nat_arms(arms)?;
                let scrutinee_d = self.e(g, fl, scrutinee, &Ty::Nat, &first.value)?;
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
                            return Ok(rule(
                                "matchNat",
                                vec![("sl", slot_term(slot))],
                                vec![sub(scrutinee_d), Decided, Decided, Decided, sub(k_d)],
                            ));
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
                        let scrutinee_d = self.e(g, fl, scrutinee, &Ty::Bool, &first.value)?;
                        let ce = Expr::Move(
                            held.clone(),
                            crate::calculus::rust::ast::Origin::of("expr:match"),
                        );
                        let k_d =
                            self.k_any(g, fl, &Kind::Plain, fm, &ce, a, b, ty, &lets[1..], tail)?;
                        Ok(rule(
                            "matchBool",
                            vec![],
                            vec![sub(scrutinee_d), Decided, Decided, sub(k_d)],
                        ))
                    })();
                    if attempt.is_ok() {
                        return attempt;
                    }
                }
                let scrutinee_d = self.b(g, fl, fm, scrutinee, &Ty::Bool, lets, tail)?;
                let yes = self.b(g, fl, false, a, &Ty::Bool, &[], &lit_bool(true))?;
                let no = self.b(g, fl, false, b, &Ty::Bool, &[], &lit_bool(false))?;
                Ok(rule(
                    "matchBoolId",
                    vec![],
                    vec![sub(scrutinee_d), Decided, sub(yes), sub(no)],
                ))
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
                let inner = param_ctx(g, &arm.binders, &types, parts)?;
                let scrutinee_d = self.e(g, fl, scrutinee, &st, &first.value)?;
                let body_d = self.b(&inner, fl, fm, &arm.body, ty, &lets[2..], tail)?;
                Ok(rule(
                    "matchPair",
                    vec![
                        ("Γ''", inner.term()),
                        ("ta", render_ty(left)),
                        ("tb", render_ty(right)),
                    ],
                    vec![sub(scrutinee_d), Decided, sub(body_d), Decided],
                ))
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
                            let scrutinee_d = self.e(g, fl, scrutinee, &st, &first.value)?;
                            let (arms_d, idx) = self.m(g, fl, &st, view, arms, fm, ty, rarms)?;
                            Ok(rule(
                                "matchArms",
                                vec![
                                    ("st", render_ty(&st)),
                                    ("U", nats_term(&self.empty)),
                                    ("view", view.to_string()),
                                    ("idx", nats_term(&idx)),
                                ],
                                vec![sub(scrutinee_d), sub(arms_d), Decided, Decided, Decided],
                            ))
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
                let scrutinee_d = self.b(g, fl, fm, scrutinee, &st, lets, tail)?;
                let (arms_d, idx) = self.m(g, fl, &st, false, arms, false, &st, &rebuilt)?;
                Ok(rule(
                    "matchRebuilt",
                    vec![
                        ("U", nats_term(&self.empty)),
                        ("idx", nats_term(&idx)),
                        (
                            "rarms",
                            list(rebuilt.iter().map(|(pattern, block)| {
                                format!(
                                    "(.mk {} {})",
                                    rust_term::pat(pattern),
                                    rust_term::block(block)
                                )
                            })),
                        ),
                    ],
                    vec![sub(scrutinee_d), sub(arms_d), Decided, Decided, Decided],
                ))
            }
            other => Err(format!("a match on {other:?}")),
        }
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
                        (Pat::Wild, lit_unit())
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
    ) -> Result<(Rule, Vec<u64>), String> {
        let Some(((pattern, block), rest)) = rarms.split_first() else {
            return Ok((rule("mNil", vec![], vec![]), Vec::new()));
        };
        let (rest_d, mut idx) = self.m(g, fl, st, view, arms, fm, tau, rest)?;
        let (shape, inner) = arm_outer(st, view, pattern)?;
        let j = arms
            .iter()
            .position(|arm| arm.shape == shape)
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
        let inner_ctx = param_ctx(g, &arm.binders, &types, &direct)?;
        let body_d = self.b(&inner_ctx, fl, fm, &arm.body, tau, &lets, &tail)?;
        idx.insert(0, j as u64);
        Ok((
            rule(
                "mCons",
                vec![
                    ("Γ'", inner_ctx.term()),
                    ("j", j.to_string()),
                    ("loads", lets_term(&loads)),
                    ("lets", lets_term(&lets)),
                    ("tail", rust_term::expr(&tail)),
                ],
                vec![sub(rest_d), Decided, Decided, Decided, sub(body_d)],
            ),
            idx,
        ))
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
        let [(Pat::Adt { adt, .. }, block)] = arms.as_slice() else {
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
        let named = vec![("ts", types_term(&types)), ("rd", rust_term::expr(&read))];
        match lets {
            [] => {
                let record_d = self.e(g, fl, value, &record_ty, scrutinee)?;
                Ok(rule(
                    "fieldRead",
                    named,
                    vec![
                        sub(record_d),
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                    ],
                ))
            }
            [binding] => {
                let record_d = self.e(g, fl, value, &record_ty, &binding.value)?;
                Ok(rule(
                    "fieldHeld",
                    named,
                    vec![
                        sub(record_d),
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                        Decided,
                    ],
                ))
            }
            _ => Err("a field binds at most its record".to_owned()),
        }
    }

    // --- functions --------------------------------------------------------------------

    /// The proof of `FunOK` for function `index`.
    fn function_proof(&mut self, index: u64) -> Result<String, String> {
        let function = self.function(index)?;
        let (parameters, body) = self
            .krate
            .items
            .iter()
            .find_map(|item| match item {
                ItemDef::Function {
                    name: Ident::Function(found),
                    parameters,
                    body,
                    ..
                } if *found == index => Some((parameters, body)),
                _ => None,
            })
            .ok_or_else(|| format!("the crate does not render function {index}"))?;
        if let Some(empty) = function.types.iter().find(|ty| self.uninh(&self.empty, ty)) {
            // No value of this parameter's type exists: the function is
            // never called.
            let position = function
                .types
                .iter()
                .position(|ty| ty == empty)
                .unwrap_or(0);
            let mut member = "(List.Mem.head _)".to_owned();
            for _ in 0..position {
                member = format!("(List.Mem.tail _ {member})");
            }
            return Ok(format!(
                "⟨_, rfl, .inr ⟨{}, {}, rfl, {member}, rfl⟩⟩",
                nats_term(&self.empty),
                render_ty(empty)
            ));
        }
        let patterns: Vec<Pat> = parameters
            .iter()
            .map(|(pattern, _)| pattern.clone())
            .collect();
        let scope = param_ctx(
            &Ctx::default(),
            &function.parameters,
            &function.types,
            &patterns,
        )?;
        let fallible = self.fallible_fn(index);
        let derivation = self.b(
            &scope,
            fallible,
            fallible,
            &function.body,
            &function.result,
            &body.lets,
            &body.tail,
        )?;
        Ok(format!(
            "⟨_, rfl, .inl ⟨_, _, {}, {}, {}, rfl, rfl,\n    {}⟩⟩",
            lets_term(&body.lets),
            rust_term::expr(&body.tail),
            scope.term(),
            derivation.text()
        ))
    }

    /// The `apply` flags of every function type the program closes over or
    /// applies.
    fn flags(&self) -> Result<String, String> {
        let mut entries = Vec::new();
        for (position, ty) in &self.fn_types {
            let Ty::Fn { parameters, result } = ty else {
                return Err("a function type is not a function".to_owned());
            };
            entries.push(format!(
                "(({}, {}), {})",
                types_term(parameters),
                render_ty(result),
                self.apply_fallible(*position)?
            ));
        }
        Ok(list(entries))
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

/// Every condition whose negation is `expr`.
fn negations_of(expr: &Expr) -> Vec<Expr> {
    let at = || crate::calculus::rust::ast::Origin::of("negate");
    let mut out = vec![Expr::Not(Box::new(expr.clone()), at())];
    match expr {
        Expr::Lit(Lit::Bool(value), _) => out.push(Expr::Lit(Lit::Bool(!value), at())),
        Expr::IsZero(ident, _) => out.push(Expr::NonZero(ident.clone(), at())),
        Expr::NonZero(ident, _) => out.push(Expr::IsZero(ident.clone(), at())),
        Expr::Not(inner, _) => out.push(inner.as_ref().clone()),
        _ => {}
    }
    out.into_iter()
        .filter(|candidate| same(&negate(candidate), expr))
        .collect()
}

/// Certificate B of one root's program rendered as `krate`, as module
/// `module`.
///
/// # Errors
///
/// Returns the reason the aligner cannot relate the crate to the program:
/// a rendering no rule of the correspondence derives.
pub fn certificate_b(
    program: &Program,
    krate: &Crate,
    module: &str,
) -> Result<CertificateB, String> {
    let mut aligner = Aligner::new(program, krate)?;
    let mut proofs = Vec::new();
    for index in 0..program.functions.len() as u64 {
        let proof = aligner
            .function_proof(index)
            .map_err(|reason| format!("function {index}: {reason}"))?;
        proofs.push(proof);
    }
    let count = proofs.len();
    let mut out = format!("import {SOUNDNESS_MODULE}\n");
    out.push_str("set_option autoImplicit false\n");
    out.push_str("set_option maxRecDepth 100000\n");
    out.push_str(&format!("namespace {module}\n"));
    out.push_str("open LexLeanTarget LexLeanPreservation.Rust\n\n");
    out.push_str(&format!(
        "def program : TargetSyntax.Program :=\n  {}\n\n",
        render_program(program)?
    ));
    out.push_str(&format!(
        "def krate : RustSyntax.Crate :=\n  {}\n\n",
        rust_term::crate_term(krate)
    ));
    out.push_str(&format!("def flags : Flags := {}\n\n", aligner.flags()?));
    for (index, proof) in proofs.iter().enumerate() {
        out.push_str(&format!(
            "theorem fun{index} : FunOK program krate flags {index} :=\n  {proof}\n\n"
        ));
    }
    out.push_str(
        "theorem crate_ok : CrateOK program krate flags := by\n  intro f fn h\n  match f with\n",
    );
    for index in 0..count {
        out.push_str(&format!("  | {index} => exact fun{index}\n"));
    }
    out.push_str(&format!(
        "  | _ + {count} => simp [program, index_eq] at h\n\n"
    ));
    out.push_str(
        "/-- Every function of the program is simulated by its rendering. -/\n\
         theorem root : ∀ n f, FunSem program krate flags n f := fun n f => simulate crate_ok n f\n\n",
    );
    out.push_str(&format!("end {module}\n"));
    Ok(CertificateB {
        module: module.to_owned(),
        theorem: format!("{module}.root"),
        text: out,
    })
}
