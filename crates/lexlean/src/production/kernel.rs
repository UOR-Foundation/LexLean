//! Kernel terms built as data (SPEC.md §17.10, §17.17).
//!
//! A certificate held as kernel-term data is a native core module: a shared
//! expression DAG and declaration rows the one core backend renders to the
//! Lean that verification compiles. This module is the builder of that DAG.
//! Every argument and universe level is written, as a kernel term has them: a
//! lemma of the preservation library is applied by reading its recorded
//! Π-type out of the native library data and instantiating one binder after
//! another with the argument given for it, so that the type of each further
//! binder, and of the application, is a substitution and never an inference.
//! Where the elaborator would have unified an `rfl`, the argument is written
//! as `Eq.refl` at the type the instantiated binder states.
//!
//! The builder shares equal nodes (the DAG of the schema is child before
//! parent), keeps the lemma names each term applies so a row can pin its
//! axioms from the library registry, and hands the finished nodes to a
//! [`CoreModule`].

use std::collections::{BTreeSet, HashMap, HashSet};

use crate::ir::core::{CoreBinderInfo, CoreLevel, CoreModule, CoreNode};

mod ext;

pub use ext::{Given, Open};

/// A node of the DAG under construction.
pub type Term = usize;

/// One argument of a lemma application: a term, or the reflexivity proof of
/// the equation the binder states.
#[derive(Debug, Clone, Copy)]
pub enum Arg {
    /// The term.
    Term(Term),
    /// `Eq.refl` at the type the instantiated binder states, which must be
    /// an equation.
    Refl,
}

/// A lemma applied: the term and its type after every argument.
#[derive(Debug, Clone, Copy)]
pub struct Applied {
    pub term: Term,
    pub ty: Term,
}

/// The builder of a core module's expression DAG.
#[derive(Debug, Default)]
pub struct Builder {
    nodes: Vec<CoreNode>,
    shared: HashMap<CoreNode, Term>,
    /// The library lemmas the terms built so far apply, by name.
    lemmas: BTreeSet<String>,
    /// The next placeholder number ([`ext`]).
    fresh: usize,
    /// The nodes known to mention no lemma placeholder ([`ext`]): a term the
    /// generator substitutes for a placeholder is never walked again.
    clean: std::cell::RefCell<HashSet<Term>>,
    /// The lemmas opened so far, by name: a lemma's placeholders are the same
    /// for every application, each of which keeps its own solutions, so its
    /// type is instantiated once.
    opened: HashMap<String, ext::Open>,
    /// The loose-variable bound of every node a walk has passed
    /// ([`Self::has_loose`]).
    loose: std::cell::RefCell<HashMap<Term, usize>>,
    /// The least free-variable id (`$id`) reachable from each node, or
    /// `usize::MAX` when the node mentions no free variable.
    min_free: Vec<usize>,
}

/// The empty binder name, which the notation writes as `_`.
const ANONYMOUS: &str = "";

impl Builder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The nodes built so far.
    #[must_use]
    pub fn nodes(&self) -> &[CoreNode] {
        &self.nodes
    }

    /// The lemma names applied since the last [`Self::take_lemmas`].
    pub fn take_lemmas(&mut self) -> BTreeSet<String> {
        std::mem::take(&mut self.lemmas)
    }

    /// Record a lemma name among the applied ones.
    pub fn note_lemma(&mut self, name: &str) {
        self.lemmas.insert(name.to_owned());
    }

    fn push(&mut self, node: CoreNode) -> Term {
        if let Some(&index) = self.shared.get(&node) {
            return index;
        }
        let index = self.nodes.len();
        let (is_clean, loose_bound, min_free) = match &node {
            CoreNode::BVar { i } => (true, Some(i + 1), usize::MAX),
            CoreNode::Sort { .. } | CoreNode::Nat { .. } | CoreNode::String { .. } => {
                (true, Some(0), usize::MAX)
            }
            CoreNode::Const { n, .. } => {
                let free = n
                    .strip_prefix("\u{1}free.")
                    .and_then(|id| id.parse().ok())
                    .unwrap_or(usize::MAX);
                (!n.starts_with("\u{1}meta."), Some(0), free)
            }
            CoreNode::App { f, x } => {
                let clean = self.clean.borrow();
                let loose = self.loose.borrow();
                (
                    clean.contains(f) && clean.contains(x),
                    loose.get(f).zip(loose.get(x)).map(|(&a, &b)| a.max(b)),
                    self.min_free[*f].min(self.min_free[*x]),
                )
            }
            CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => {
                let clean = self.clean.borrow();
                let loose = self.loose.borrow();
                (
                    clean.contains(t) && clean.contains(v),
                    loose
                        .get(t)
                        .zip(loose.get(v))
                        .map(|(&a, &b)| a.max(b.saturating_sub(1))),
                    self.min_free[*t].min(self.min_free[*v]),
                )
            }
            CoreNode::Let { t, v, body, .. } => {
                let clean = self.clean.borrow();
                let loose = self.loose.borrow();
                (
                    clean.contains(t) && clean.contains(v) && clean.contains(body),
                    loose
                        .get(t)
                        .zip(loose.get(v))
                        .zip(loose.get(body))
                        .map(|((&a, &b), &c)| a.max(b).max(c.saturating_sub(1))),
                    self.min_free[*t]
                        .min(self.min_free[*v])
                        .min(self.min_free[*body]),
                )
            }
            CoreNode::Mdata { x, .. } | CoreNode::Projection { x, .. } => {
                let clean = self.clean.borrow();
                let loose = self.loose.borrow();
                (clean.contains(x), loose.get(x).copied(), self.min_free[*x])
            }
        };
        if is_clean {
            self.clean.borrow_mut().insert(index);
        }
        if let Some(bound) = loose_bound {
            self.loose.borrow_mut().insert(index, bound);
        }
        self.min_free.push(min_free);
        self.nodes.push(node.clone());
        self.shared.insert(node, index);
        index
    }

    /// The node at `term`.
    #[must_use]
    pub fn node(&self, term: Term) -> &CoreNode {
        &self.nodes[term]
    }

    // --- Atoms ------------------------------------------------------------

    pub fn bvar(&mut self, i: usize) -> Term {
        self.push(CoreNode::BVar { i })
    }

    pub fn sort(&mut self, level: CoreLevel) -> Term {
        self.push(CoreNode::Sort { l: level })
    }

    /// `Prop`.
    pub fn prop(&mut self) -> Term {
        self.sort(CoreLevel::Zero)
    }

    /// `Type`.
    pub fn type0(&mut self) -> Term {
        self.sort(CoreLevel::Succ(Box::new(CoreLevel::Zero)))
    }

    /// A constant with no universe levels.
    pub fn konst(&mut self, name: &str) -> Term {
        self.push(CoreNode::Const {
            n: name.to_owned(),
            u: Vec::new(),
        })
    }

    /// A constant at the given universe levels.
    pub fn konst_at(&mut self, name: &str, levels: Vec<CoreLevel>) -> Term {
        self.push(CoreNode::Const {
            n: name.to_owned(),
            u: levels,
        })
    }

    /// A constant at universe level 0 (`List.{0}`, `Option.{0}`).
    pub fn konst0(&mut self, name: &str) -> Term {
        self.konst_at(name, vec![CoreLevel::Zero])
    }

    /// A constant at universe level 1 (`Eq.{1}` over a type in `Type`).
    pub fn konst1(&mut self, name: &str) -> Term {
        self.konst_at(name, vec![CoreLevel::Succ(Box::new(CoreLevel::Zero))])
    }

    /// A constant at universe levels 0, 0 (`Prod.{0, 0}`, `Except.{0, 0}`).
    pub fn konst00(&mut self, name: &str) -> Term {
        self.konst_at(name, vec![CoreLevel::Zero, CoreLevel::Zero])
    }

    /// A natural-number literal.
    pub fn nat(&mut self, value: u64) -> Term {
        self.push(CoreNode::Nat {
            v: value.to_string(),
        })
    }

    /// A string literal.
    pub fn string(&mut self, value: &str) -> Term {
        self.push(CoreNode::String {
            v: value.chars().map(u32::from).collect(),
        })
    }

    // --- Compound nodes ---------------------------------------------------

    pub fn app(&mut self, f: Term, x: Term) -> Term {
        self.push(CoreNode::App { f, x })
    }

    /// `f x₁ … xₙ`.
    pub fn apps(&mut self, f: Term, args: &[Term]) -> Term {
        args.iter().fold(f, |f, &x| self.app(f, x))
    }

    /// `fun (name : ty) => body`, `body` written over `#0` for `name`.
    pub fn lam(&mut self, name: &str, binder: CoreBinderInfo, ty: Term, body: Term) -> Term {
        self.push(CoreNode::Lambda {
            n: name.to_owned(),
            b: binder,
            t: ty,
            v: body,
        })
    }

    /// `(name : ty) -> body`, `body` written over `#0` for `name`.
    pub fn pi(&mut self, name: &str, binder: CoreBinderInfo, ty: Term, body: Term) -> Term {
        self.push(CoreNode::Forall {
            n: name.to_owned(),
            b: binder,
            t: ty,
            v: body,
        })
    }

    /// `a -> b` with the anonymous explicit binder; `b` must not mention `#0`,
    /// which the caller ensures by lifting it.
    pub fn arrow(&mut self, a: Term, b: Term) -> Term {
        let lifted = self.lift(b, 1, 0);
        self.pi(ANONYMOUS, CoreBinderInfo::Explicit, a, lifted)
    }

    // --- Lists and standard constructions ---------------------------------

    /// `[x₁, …, xₙ] : List α` at universe level 0.
    pub fn list(&mut self, element: Term, items: &[Term]) -> Term {
        let nil = self.konst0("List.nil");
        let cons = self.konst0("List.cons");
        let mut out = self.app(nil, element);
        for &item in items.iter().rev() {
            out = self.apps(cons, &[element, item, out]);
        }
        out
    }

    /// `List α` at universe level 0.
    pub fn list_ty(&mut self, element: Term) -> Term {
        let list = self.konst0("List");
        self.app(list, element)
    }

    /// `(a, b) : Prod α β` at universe levels 0, 0.
    pub fn pair(&mut self, alpha: Term, beta: Term, a: Term, b: Term) -> Term {
        let mk = self.konst00("Prod.mk");
        self.apps(mk, &[alpha, beta, a, b])
    }

    /// `Prod α β` at universe levels 0, 0.
    pub fn pair_ty(&mut self, alpha: Term, beta: Term) -> Term {
        let prod = self.konst00("Prod");
        self.apps(prod, &[alpha, beta])
    }

    // --- Substitution -----------------------------------------------------

    /// `term` with every loose variable at or above `cutoff` raised by `n`.
    pub fn lift(&mut self, term: Term, n: usize, cutoff: usize) -> Term {
        if n == 0 {
            return term;
        }
        let mut memo = HashMap::new();
        self.lift_memo(term, n, cutoff, &mut memo)
    }

    fn lift_memo(
        &mut self,
        term: Term,
        n: usize,
        cutoff: usize,
        memo: &mut HashMap<(Term, usize), Term>,
    ) -> Term {
        if let Some(&done) = memo.get(&(term, cutoff)) {
            return done;
        }
        if self.loose_bound(term) <= cutoff {
            return term;
        }
        let out = match self.nodes[term].clone() {
            CoreNode::BVar { i } => {
                if i >= cutoff {
                    self.bvar(i + n)
                } else {
                    term
                }
            }
            CoreNode::Sort { .. }
            | CoreNode::Const { .. }
            | CoreNode::Nat { .. }
            | CoreNode::String { .. } => term,
            CoreNode::App { f, x } => {
                let f = self.lift_memo(f, n, cutoff, memo);
                let x = self.lift_memo(x, n, cutoff, memo);
                self.app(f, x)
            }
            CoreNode::Lambda { n: name, b, t, v } => {
                let t = self.lift_memo(t, n, cutoff, memo);
                let v = self.lift_memo(v, n, cutoff + 1, memo);
                self.lam(&name, b, t, v)
            }
            CoreNode::Forall { n: name, b, t, v } => {
                let t = self.lift_memo(t, n, cutoff, memo);
                let v = self.lift_memo(v, n, cutoff + 1, memo);
                self.pi(&name, b, t, v)
            }
            CoreNode::Let {
                n: name,
                t,
                v,
                body,
                d,
            } => {
                let t = self.lift_memo(t, n, cutoff, memo);
                let v = self.lift_memo(v, n, cutoff, memo);
                let body = self.lift_memo(body, n, cutoff + 1, memo);
                self.push(CoreNode::Let {
                    n: name,
                    t,
                    v,
                    body,
                    d,
                })
            }
            CoreNode::Mdata { m, x } => {
                let x = self.lift_memo(x, n, cutoff, memo);
                self.push(CoreNode::Mdata { m, x })
            }
            CoreNode::Projection { n: name, s, i, x } => {
                let x = self.lift_memo(x, n, cutoff, memo);
                self.push(CoreNode::Projection { n: name, s, i, x })
            }
        };
        memo.insert((term, cutoff), out);
        out
    }

    /// `body` with `#0` replaced by `arg` and every other loose variable
    /// lowered by one: the body of a binder instantiated at `arg`.
    pub fn instantiate(&mut self, body: Term, arg: Term) -> Term {
        if self.loose_bound(body) == 0 {
            return body;
        }
        let mut memo = HashMap::new();
        self.instantiate_memo(body, arg, 0, &mut memo)
    }

    fn instantiate_memo(
        &mut self,
        term: Term,
        arg: Term,
        depth: usize,
        memo: &mut HashMap<(Term, usize), Term>,
    ) -> Term {
        if let Some(&done) = memo.get(&(term, depth)) {
            return done;
        }
        if self.loose_bound(term) <= depth {
            return term;
        }
        let out = match self.nodes[term].clone() {
            CoreNode::BVar { i } => {
                if i == depth {
                    self.lift(arg, depth, 0)
                } else if i > depth {
                    self.bvar(i - 1)
                } else {
                    term
                }
            }
            CoreNode::Sort { .. }
            | CoreNode::Const { .. }
            | CoreNode::Nat { .. }
            | CoreNode::String { .. } => term,
            CoreNode::App { f, x } => {
                let f = self.instantiate_memo(f, arg, depth, memo);
                let x = self.instantiate_memo(x, arg, depth, memo);
                self.app(f, x)
            }
            CoreNode::Lambda { n, b, t, v } => {
                let t = self.instantiate_memo(t, arg, depth, memo);
                let v = self.instantiate_memo(v, arg, depth + 1, memo);
                self.lam(&n, b, t, v)
            }
            CoreNode::Forall { n, b, t, v } => {
                let t = self.instantiate_memo(t, arg, depth, memo);
                let v = self.instantiate_memo(v, arg, depth + 1, memo);
                self.pi(&n, b, t, v)
            }
            CoreNode::Let { n, t, v, body, d } => {
                let t = self.instantiate_memo(t, arg, depth, memo);
                let v = self.instantiate_memo(v, arg, depth, memo);
                let body = self.instantiate_memo(body, arg, depth + 1, memo);
                self.push(CoreNode::Let { n, t, v, body, d })
            }
            CoreNode::Mdata { m, x } => {
                let x = self.instantiate_memo(x, arg, depth, memo);
                self.push(CoreNode::Mdata { m, x })
            }
            CoreNode::Projection { n, s, i, x } => {
                let x = self.instantiate_memo(x, arg, depth, memo);
                self.push(CoreNode::Projection { n, s, i, x })
            }
        };
        memo.insert((term, depth), out);
        out
    }

    /// Whether `term` mentions a loose variable at or above `cutoff`.
    #[must_use]
    pub fn has_loose(&self, term: Term, cutoff: usize) -> bool {
        self.loose_bound(term) > cutoff
    }

    /// The least `k` such that every loose variable of `term` is below `k`
    /// (zero for a closed term), remembered for every node it passes: the
    /// DAG is shared, and a walk that does not remember is as long as the
    /// tree it unfolds to.
    fn loose_bound(&self, root: Term) -> usize {
        if let Some(&known) = self.loose.borrow().get(&root) {
            return known;
        }
        let mut stack: Vec<(Term, bool)> = vec![(root, false)];
        let mut found: HashMap<Term, usize> = HashMap::new();
        while let Some((term, expanded)) = stack.pop() {
            if found.contains_key(&term) {
                continue;
            }
            if let Some(&known) = self.loose.borrow().get(&term) {
                found.insert(term, known);
                continue;
            }
            let children: Vec<Term> = match &self.nodes[term] {
                CoreNode::BVar { i } => {
                    found.insert(term, i + 1);
                    continue;
                }
                CoreNode::Sort { .. }
                | CoreNode::Const { .. }
                | CoreNode::Nat { .. }
                | CoreNode::String { .. } => {
                    found.insert(term, 0);
                    continue;
                }
                CoreNode::App { f, x } => vec![*f, *x],
                CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => vec![*t, *v],
                CoreNode::Let { t, v, body, .. } => vec![*t, *v, *body],
                CoreNode::Mdata { x, .. } | CoreNode::Projection { x, .. } => vec![*x],
            };
            if !expanded {
                stack.push((term, true));
                for child in children {
                    stack.push((child, false));
                }
                continue;
            }
            let bound = |child: Term| found.get(&child).copied().unwrap_or(0);
            let value = match &self.nodes[term] {
                CoreNode::App { f, x } => bound(*f).max(bound(*x)),
                CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => {
                    bound(*t).max(bound(*v).saturating_sub(1))
                }
                CoreNode::Let { t, v, body, .. } => {
                    bound(*t).max(bound(*v)).max(bound(*body).saturating_sub(1))
                }
                CoreNode::Mdata { x, .. } | CoreNode::Projection { x, .. } => bound(*x),
                CoreNode::BVar { .. }
                | CoreNode::Sort { .. }
                | CoreNode::Const { .. }
                | CoreNode::Nat { .. }
                | CoreNode::String { .. } => 0,
            };
            found.insert(term, value);
        }
        let result = found[&root];
        self.loose.borrow_mut().extend(found);
        result
    }

    // --- Importing from another module ------------------------------------

    /// The node `root` of `source`, copied into this DAG with its children.
    pub fn import(&mut self, source: &CoreModule, root: usize) -> Term {
        let mut memo = HashMap::new();
        self.import_memo(source, root, &mut memo)
    }

    fn import_memo(
        &mut self,
        source: &CoreModule,
        index: usize,
        memo: &mut HashMap<usize, Term>,
    ) -> Term {
        if let Some(&done) = memo.get(&index) {
            return done;
        }
        let out = match &source.nodes[index] {
            node @ (CoreNode::BVar { .. }
            | CoreNode::Sort { .. }
            | CoreNode::Const { .. }
            | CoreNode::Nat { .. }
            | CoreNode::String { .. }) => self.push(node.clone()),
            CoreNode::App { f, x } => {
                let f = self.import_memo(source, *f, memo);
                let x = self.import_memo(source, *x, memo);
                self.app(f, x)
            }
            CoreNode::Lambda { n, b, t, v } => {
                let t = self.import_memo(source, *t, memo);
                let v = self.import_memo(source, *v, memo);
                self.lam(n, *b, t, v)
            }
            CoreNode::Forall { n, b, t, v } => {
                let t = self.import_memo(source, *t, memo);
                let v = self.import_memo(source, *v, memo);
                self.pi(n, *b, t, v)
            }
            CoreNode::Let { n, t, v, body, d } => {
                let t = self.import_memo(source, *t, memo);
                let v = self.import_memo(source, *v, memo);
                let body = self.import_memo(source, *body, memo);
                self.push(CoreNode::Let {
                    n: n.clone(),
                    t,
                    v,
                    body,
                    d: *d,
                })
            }
            // An annotation is kernel-irrelevant; a statement states the term.
            CoreNode::Mdata { m: _, x } => self.import_memo(source, *x, memo),
            CoreNode::Projection { n, s, i, x } => {
                let x = self.import_memo(source, *x, memo);
                self.push(CoreNode::Projection {
                    n: n.clone(),
                    s: s.clone(),
                    i: *i,
                    x,
                })
            }
        };
        memo.insert(index, out);
        out
    }

    // --- Application against a Π-type -------------------------------------

    /// The head and arguments of an application spine.
    #[must_use]
    pub fn spine(&self, term: Term) -> (Term, Vec<Term>) {
        let mut args = Vec::new();
        let mut head = term;
        while let CoreNode::App { f, x } = &self.nodes[head] {
            args.push(*x);
            head = *f;
        }
        args.reverse();
        (head, args)
    }

    /// `head`, whose type is `ty`, applied to `args` binder by binder: each
    /// explicit term instantiates the binder it fills, and a [`Arg::Refl`]
    /// is `Eq.refl` at the equation the binder states. The result's type is
    /// the body after the last argument.
    ///
    /// # Errors
    ///
    /// More arguments than binders, or a reflexivity argument at a binder
    /// whose type is not an equation.
    pub fn apply_pi(&mut self, head: Term, ty: Term, args: &[Arg]) -> Result<Applied, String> {
        let mut term = head;
        let mut ty = ty;
        for (position, arg) in args.iter().enumerate() {
            let (domain, body) = match &self.nodes[ty] {
                CoreNode::Forall { t, v, .. } => (*t, *v),
                _ => {
                    return Err(format!(
                        "argument {position} applied to a term whose type is not a binder"
                    ));
                }
            };
            let value = match arg {
                Arg::Term(value) => *value,
                Arg::Refl => {
                    let (head, parts) = self.spine(domain);
                    let levels = match &self.nodes[head] {
                        CoreNode::Const { n, u } if n == "Eq" && parts.len() == 3 => u.clone(),
                        _ => {
                            return Err(format!(
                                "argument {position}: a reflexivity proof at a binder that is not an equation"
                            ));
                        }
                    };
                    let refl = self.konst_at("Eq.refl", levels);
                    self.apps(refl, &[parts[0], parts[1]])
                }
            };
            term = self.app(term, value);
            ty = self.instantiate(body, value);
        }
        Ok(Applied { term, ty })
    }

    /// The binders of a Π-type, outermost first, with the body after them:
    /// each binder's type is written over the earlier binders.
    #[must_use]
    pub fn binders(&self, ty: Term) -> (Vec<(String, CoreBinderInfo, Term)>, Term) {
        let mut out = Vec::new();
        let mut body = ty;
        while let CoreNode::Forall { n, b, t, v } = &self.nodes[body] {
            out.push((n.clone(), *b, *t));
            body = *v;
        }
        (out, body)
    }

    /// The finished DAG.
    #[must_use]
    pub fn finish(self) -> Vec<CoreNode> {
        self.nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiating_a_binder_substitutes_and_lowers() {
        let mut b = Builder::new();
        let nat = b.konst("Nat");
        // (x : Nat) -> (y : Nat) -> Eq Nat x y, written over #1 and #0.
        let x = b.bvar(1);
        let y = b.bvar(0);
        let eq = b.konst1("Eq");
        let body = b.apps(eq, &[nat, x, y]);
        let inner = b.pi("y", CoreBinderInfo::Explicit, nat, body);
        let ty = b.pi("x", CoreBinderInfo::Explicit, nat, inner);
        let zero = b.nat(0);
        let f = b.konst("f");
        let applied = b.apply_pi(f, ty, &[Arg::Term(zero)]).expect("one argument");
        let (binders, result) = b.binders(applied.ty);
        assert_eq!(binders.len(), 1);
        let (head, parts) = b.spine(result);
        assert!(matches!(b.node(head), CoreNode::Const { n, .. } if n == "Eq"));
        assert_eq!(parts[1], zero, "x became the literal");
        assert!(
            matches!(b.node(parts[2]), CoreNode::BVar { i: 0 }),
            "y is still #0"
        );
        assert_eq!(applied.term, b.app(f, zero));
    }

    #[test]
    fn a_reflexivity_argument_is_eq_refl_at_the_stated_equation() {
        let mut b = Builder::new();
        let nat = b.konst("Nat");
        let one = b.nat(1);
        let eq = b.konst1("Eq");
        let equation = b.apps(eq, &[nat, one, one]);
        let prop = b.prop();
        let lifted = b.lift(prop, 1, 0);
        let ty = b.pi("h", CoreBinderInfo::Explicit, equation, lifted);
        let f = b.konst("f");
        let applied = b.apply_pi(f, ty, &[Arg::Refl]).expect("one argument");
        let (_, args) = b.spine(applied.term);
        let (refl_head, refl_args) = b.spine(args[0]);
        assert!(matches!(b.node(refl_head), CoreNode::Const { n, .. } if n == "Eq.refl"));
        assert_eq!(refl_args, vec![nat, one]);
        assert_eq!(applied.ty, prop);
    }

    #[test]
    fn equal_nodes_are_shared() {
        let mut b = Builder::new();
        let a = b.konst("Nat");
        let c = b.konst("Nat");
        assert_eq!(a, c);
        let list1 = b.list(a, &[]);
        let list2 = b.list(c, &[]);
        assert_eq!(list1, list2);
        assert!(b.nodes().len() < 6);
    }
}
