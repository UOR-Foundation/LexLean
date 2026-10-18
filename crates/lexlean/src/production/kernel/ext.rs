//! Locally nameless construction on the kernel builder (SPEC.md §17.17).
//!
//! A certificate is built inside binders (the parameters of a function, the
//! fields of a matched constructor), so the generator holds terms that mention
//! them before the binder exists. A *free variable* is a placeholder constant
//! no declaration can be named: [`Builder::abstract_over`] turns it into the
//! bound variable of the lambda or Π that closes it, and nothing else ever
//! creates a loose variable, so the instantiation of [`super`] needs no
//! lifting.
//!
//! A lemma of the preservation library is applied with its implicit and
//! instance binders as *metavariables*, placeholders of the same kind. They
//! are solved by writing the binder types the library records against the
//! types of the arguments given for the explicit binders (a syntactic
//! comparison that assigns a placeholder where the pattern has one and never
//! reduces, searches for an instance, or inserts a coercion), by the hints the
//! generator holds from the statement it is proving, or, for a reflexivity
//! argument, by the left side of the equation it states. Nothing here decides
//! whether a term is well typed: the kernel does, on the finished module, and
//! a placeholder left unsolved is refused by the generator by name.

use std::collections::HashMap;

use super::{Applied, Builder, Term};
use crate::ir::core::{CoreBinderInfo, CoreNode};

/// The rewriter of `rewrite_apps`: given a constant's name and its rewritten
/// arguments, the replacement, or `None` to keep the application.
type Rewrite<'r> = dyn FnMut(&mut Builder, &str, &[Term]) -> Option<Term> + 'r;

const FREE: &str = "\u{1}free.";
const META: &str = "\u{1}meta.";

/// An explicit argument of a lemma application.
#[derive(Debug, Clone, Copy)]
pub enum Given {
    /// A term whose type the generator does not need.
    Term(Term),
    /// A term and its type, which solves the placeholders of the binder it
    /// fills.
    Proof(Term, Term),
    /// `Eq.refl` at the equation the binder states.
    Refl,
}

impl Builder {
    /// The nodes reachable from `roots`, child before parent, with the
    /// position each root has among them: the DAG a module keeps, without the
    /// intermediate terms the construction passed through.
    #[must_use]
    pub fn compact(&self, roots: &[Term]) -> (Vec<CoreNode>, Vec<Term>) {
        let mut position: HashMap<Term, Term> = HashMap::new();
        let mut out: Vec<CoreNode> = Vec::new();
        for &root in roots {
            let mut stack: Vec<(Term, bool)> = vec![(root, false)];
            while let Some((term, children_done)) = stack.pop() {
                if position.contains_key(&term) {
                    continue;
                }
                let node = &self.nodes[term];
                if !children_done {
                    stack.push((term, true));
                    match node {
                        CoreNode::BVar { .. }
                        | CoreNode::Sort { .. }
                        | CoreNode::Const { .. }
                        | CoreNode::Nat { .. }
                        | CoreNode::String { .. } => {}
                        CoreNode::App { f, x } => {
                            stack.push((*x, false));
                            stack.push((*f, false));
                        }
                        CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => {
                            stack.push((*v, false));
                            stack.push((*t, false));
                        }
                        CoreNode::Let { t, v, body, .. } => {
                            stack.push((*body, false));
                            stack.push((*v, false));
                            stack.push((*t, false));
                        }
                        CoreNode::Mdata { x, .. } | CoreNode::Projection { x, .. } => {
                            stack.push((*x, false));
                        }
                    }
                    continue;
                }
                let at = |term: &Term| position[term];
                let moved = match node.clone() {
                    CoreNode::App { f, x } => CoreNode::App {
                        f: at(&f),
                        x: at(&x),
                    },
                    CoreNode::Lambda { n, b, t, v } => CoreNode::Lambda {
                        n,
                        b,
                        t: at(&t),
                        v: at(&v),
                    },
                    CoreNode::Forall { n, b, t, v } => CoreNode::Forall {
                        n,
                        b,
                        t: at(&t),
                        v: at(&v),
                    },
                    CoreNode::Let { n, t, v, body, d } => CoreNode::Let {
                        n,
                        t: at(&t),
                        v: at(&v),
                        body: at(&body),
                        d,
                    },
                    CoreNode::Mdata { m, x } => CoreNode::Mdata { m, x: at(&x) },
                    CoreNode::Projection { n, s, i, x } => {
                        CoreNode::Projection { n, s, i, x: at(&x) }
                    }
                    atom @ (CoreNode::BVar { .. }
                    | CoreNode::Sort { .. }
                    | CoreNode::Const { .. }
                    | CoreNode::Nat { .. }
                    | CoreNode::String { .. }) => atom,
                };
                position.insert(term, out.len());
                out.push(moved);
            }
        }
        let mapped = roots.iter().map(|root| position[root]).collect();
        (out, mapped)
    }

    /// A natural-number literal of any size, from its digits.
    pub fn nat_text(&mut self, digits: &str) -> Term {
        self.push(CoreNode::Nat {
            v: digits.to_owned(),
        })
    }

    /// A fresh free variable.
    pub fn free(&mut self) -> Term {
        let id = self.fresh;
        self.fresh += 1;
        self.konst(&format!("{FREE}{id}"))
    }

    fn meta(&mut self) -> (usize, Term) {
        let id = self.fresh;
        self.fresh += 1;
        (id, self.konst(&format!("{META}{id}")))
    }

    fn meta_id(&self, term: Term) -> Option<usize> {
        match &self.nodes[term] {
            CoreNode::Const { n, .. } => n.strip_prefix(META).and_then(|id| id.parse().ok()),
            _ => None,
        }
    }

    /// `term` with the free variable `free` bound by the binder around it.
    pub fn abstract_over(&mut self, term: Term, free: Term) -> Term {
        let free_id = self.min_free[free];
        if self.min_free[term] > free_id {
            return term;
        }
        let mut memo = HashMap::new();
        self.abstract_memo(term, free, free_id, 0, &mut memo)
    }

    fn abstract_memo(
        &mut self,
        term: Term,
        free: Term,
        free_id: usize,
        depth: usize,
        memo: &mut HashMap<(Term, usize), Term>,
    ) -> Term {
        if term == free {
            return self.bvar(depth);
        }
        if self.min_free[term] > free_id {
            return term;
        }
        if let Some(&done) = memo.get(&(term, depth)) {
            return done;
        }
        let out = match self.nodes[term].clone() {
            CoreNode::BVar { .. }
            | CoreNode::Sort { .. }
            | CoreNode::Const { .. }
            | CoreNode::Nat { .. }
            | CoreNode::String { .. } => term,
            CoreNode::App { f, x } => {
                let f = self.abstract_memo(f, free, free_id, depth, memo);
                let x = self.abstract_memo(x, free, free_id, depth, memo);
                self.app(f, x)
            }
            CoreNode::Lambda { n, b, t, v } => {
                let t = self.abstract_memo(t, free, free_id, depth, memo);
                let v = self.abstract_memo(v, free, free_id, depth + 1, memo);
                self.lam(&n, b, t, v)
            }
            CoreNode::Forall { n, b, t, v } => {
                let t = self.abstract_memo(t, free, free_id, depth, memo);
                let v = self.abstract_memo(v, free, free_id, depth + 1, memo);
                self.pi(&n, b, t, v)
            }
            CoreNode::Let { n, t, v, body, d } => {
                let t = self.abstract_memo(t, free, free_id, depth, memo);
                let v = self.abstract_memo(v, free, free_id, depth, memo);
                let body = self.abstract_memo(body, free, free_id, depth + 1, memo);
                self.let_node(&n, t, v, body, d)
            }
            CoreNode::Mdata { m, x } => {
                let x = self.abstract_memo(x, free, free_id, depth, memo);
                self.mdata_node(m, x)
            }
            CoreNode::Projection { n, s, i, x } => {
                let x = self.abstract_memo(x, free, free_id, depth, memo);
                self.projection(&n, &s, i, x)
            }
        };
        memo.insert((term, depth), out);
        out
    }

    /// `fun (name : ty) => body` over the free variable `free` of `body`.
    pub fn lam_over(
        &mut self,
        name: &str,
        binder: CoreBinderInfo,
        ty: Term,
        free: Term,
        body: Term,
    ) -> Term {
        let body = self.abstract_over(body, free);
        self.lam(name, binder, ty, body)
    }

    /// `(name : ty) -> body` over the free variable `free` of `body`.
    pub fn pi_over(
        &mut self,
        name: &str,
        binder: CoreBinderInfo,
        ty: Term,
        free: Term,
        body: Term,
    ) -> Term {
        let body = self.abstract_over(body, free);
        self.pi(name, binder, ty, body)
    }

    /// `let name : ty := value; body` over the free variable `free` of `body`.
    pub fn let_over(&mut self, name: &str, ty: Term, value: Term, free: Term, body: Term) -> Term {
        let body = self.abstract_over(body, free);
        self.let_node(name, ty, value, body, false)
    }

    /// A `let` node whose body is already written over `#0`.
    pub fn let_node(&mut self, name: &str, ty: Term, value: Term, body: Term, dep: bool) -> Term {
        self.push(CoreNode::Let {
            n: name.to_owned(),
            t: ty,
            v: value,
            body,
            d: dep,
        })
    }

    fn mdata_node(&mut self, entries: Vec<crate::ir::core::CoreMetaEntry>, x: Term) -> Term {
        self.push(CoreNode::Mdata { m: entries, x })
    }

    /// A primitive projection.
    pub fn projection(&mut self, name: &str, structure: &str, index: usize, x: Term) -> Term {
        self.push(CoreNode::Projection {
            n: name.to_owned(),
            s: structure.to_owned(),
            i: index,
            x,
        })
    }

    /// `term` with each application of a constant rewritten by `rewrite`,
    /// which is given the constant's name and its (already rewritten)
    /// arguments and returns the replacement, or `None` to keep the
    /// application.
    pub fn rewrite_apps(&mut self, term: Term, rewrite: &mut Rewrite<'_>) -> Term {
        let mut memo = HashMap::new();
        self.rewrite_memo(term, rewrite, &mut memo)
    }

    fn rewrite_memo(
        &mut self,
        term: Term,
        rewrite: &mut Rewrite<'_>,
        memo: &mut HashMap<Term, Term>,
    ) -> Term {
        if let Some(&done) = memo.get(&term) {
            return done;
        }
        let out = match self.nodes[term].clone() {
            CoreNode::BVar { .. }
            | CoreNode::Sort { .. }
            | CoreNode::Nat { .. }
            | CoreNode::String { .. } => term,
            CoreNode::Const { n, .. } => rewrite(self, &n, &[]).unwrap_or(term),
            CoreNode::App { .. } => {
                let (head, args) = self.spine(term);
                let args: Vec<Term> = args
                    .iter()
                    .map(|arg| self.rewrite_memo(*arg, rewrite, memo))
                    .collect();
                match self.nodes[head].clone() {
                    CoreNode::Const { n, .. } => match rewrite(self, &n, &args) {
                        Some(replaced) => replaced,
                        None => self.apps(head, &args),
                    },
                    _ => {
                        let head = self.rewrite_memo(head, rewrite, memo);
                        self.apps(head, &args)
                    }
                }
            }
            CoreNode::Lambda { n, b, t, v } => {
                let t = self.rewrite_memo(t, rewrite, memo);
                let v = self.rewrite_memo(v, rewrite, memo);
                self.lam(&n, b, t, v)
            }
            CoreNode::Forall { n, b, t, v } => {
                let t = self.rewrite_memo(t, rewrite, memo);
                let v = self.rewrite_memo(v, rewrite, memo);
                self.pi(&n, b, t, v)
            }
            CoreNode::Let { n, t, v, body, d } => {
                let t = self.rewrite_memo(t, rewrite, memo);
                let v = self.rewrite_memo(v, rewrite, memo);
                let body = self.rewrite_memo(body, rewrite, memo);
                self.let_node(&n, t, v, body, d)
            }
            CoreNode::Mdata { m, x } => {
                let x = self.rewrite_memo(x, rewrite, memo);
                self.mdata_node(m, x)
            }
            CoreNode::Projection { n, s, i, x } => {
                let x = self.rewrite_memo(x, rewrite, memo);
                self.projection(&n, &s, i, x)
            }
        };
        memo.insert(term, out);
        out
    }

    /// `term` with each node of `from` replaced by the node of `to` at its
    /// position, wherever it occurs (a free variable, in practice).
    pub fn substitute(&mut self, term: Term, from: &[Term], to: &[Term]) -> Term {
        let map: HashMap<Term, Term> = from.iter().copied().zip(to.iter().copied()).collect();
        if map.is_empty() {
            return term;
        }
        let max_free = if from.iter().all(|&f| self.min_free[f] != usize::MAX) {
            from.iter().map(|&f| self.min_free[f]).max()
        } else {
            None
        };
        let mut memo = HashMap::new();
        self.substitute_memo(term, &map, max_free, &mut memo)
    }

    fn substitute_memo(
        &mut self,
        term: Term,
        map: &HashMap<Term, Term>,
        max_free: Option<usize>,
        memo: &mut HashMap<Term, Term>,
    ) -> Term {
        if let Some(&replacement) = map.get(&term) {
            return replacement;
        }
        if let Some(limit) = max_free {
            if self.min_free[term] > limit {
                return term;
            }
        }
        if let Some(&done) = memo.get(&term) {
            return done;
        }
        let out = match self.nodes[term].clone() {
            CoreNode::BVar { .. }
            | CoreNode::Sort { .. }
            | CoreNode::Const { .. }
            | CoreNode::Nat { .. }
            | CoreNode::String { .. } => term,
            CoreNode::App { f, x } => {
                let f = self.substitute_memo(f, map, max_free, memo);
                let x = self.substitute_memo(x, map, max_free, memo);
                self.app(f, x)
            }
            CoreNode::Lambda { n, b, t, v } => {
                let t = self.substitute_memo(t, map, max_free, memo);
                let v = self.substitute_memo(v, map, max_free, memo);
                self.lam(&n, b, t, v)
            }
            CoreNode::Forall { n, b, t, v } => {
                let t = self.substitute_memo(t, map, max_free, memo);
                let v = self.substitute_memo(v, map, max_free, memo);
                self.pi(&n, b, t, v)
            }
            CoreNode::Let { n, t, v, body, d } => {
                let t = self.substitute_memo(t, map, max_free, memo);
                let v = self.substitute_memo(v, map, max_free, memo);
                let body = self.substitute_memo(body, map, max_free, memo);
                self.let_node(&n, t, v, body, d)
            }
            CoreNode::Mdata { m, x } => {
                let x = self.substitute_memo(x, map, max_free, memo);
                self.mdata_node(m, x)
            }
            CoreNode::Projection { n, s, i, x } => {
                let x = self.substitute_memo(x, map, max_free, memo);
                self.projection(&n, &s, i, x)
            }
        };
        memo.insert(term, out);
        out
    }

    /// `term` with every constant named in `map` replaced by its term.
    pub fn replace_consts(&mut self, term: Term, map: &HashMap<String, Term>) -> Term {
        if map.is_empty() {
            return term;
        }
        let mut memo = HashMap::new();
        self.replace_memo(term, map, &mut memo)
    }

    fn replace_memo(
        &mut self,
        term: Term,
        map: &HashMap<String, Term>,
        memo: &mut HashMap<Term, Term>,
    ) -> Term {
        if let Some(&done) = memo.get(&term) {
            return done;
        }
        let out = match self.nodes[term].clone() {
            CoreNode::Const { n, .. } => map.get(&n).copied().unwrap_or(term),
            CoreNode::BVar { .. }
            | CoreNode::Sort { .. }
            | CoreNode::Nat { .. }
            | CoreNode::String { .. } => term,
            CoreNode::App { f, x } => {
                let f = self.replace_memo(f, map, memo);
                let x = self.replace_memo(x, map, memo);
                self.app(f, x)
            }
            CoreNode::Lambda { n, b, t, v } => {
                let t = self.replace_memo(t, map, memo);
                let v = self.replace_memo(v, map, memo);
                self.lam(&n, b, t, v)
            }
            CoreNode::Forall { n, b, t, v } => {
                let t = self.replace_memo(t, map, memo);
                let v = self.replace_memo(v, map, memo);
                self.pi(&n, b, t, v)
            }
            CoreNode::Let { n, t, v, body, d } => {
                let t = self.replace_memo(t, map, memo);
                let v = self.replace_memo(v, map, memo);
                let body = self.replace_memo(body, map, memo);
                self.let_node(&n, t, v, body, d)
            }
            CoreNode::Mdata { m, x } => {
                let x = self.replace_memo(x, map, memo);
                self.mdata_node(m, x)
            }
            CoreNode::Projection { n, s, i, x } => {
                let x = self.replace_memo(x, map, memo);
                self.projection(&n, &s, i, x)
            }
        };
        memo.insert(term, out);
        out
    }

    /// `term` as a readable expression, for the generator's error messages.
    #[must_use]
    pub fn show(&self, term: Term) -> String {
        self.show_at(term, &mut Vec::new(), 0)
    }

    fn show_at(&self, term: Term, names: &mut Vec<String>, depth: usize) -> String {
        if depth > 4000 {
            return "…".to_owned();
        }
        match &self.nodes[term] {
            CoreNode::BVar { i } => names
                .len()
                .checked_sub(i + 1)
                .and_then(|at| names.get(at).cloned())
                .unwrap_or_else(|| format!("#{i}")),
            CoreNode::Sort { l } => format!("Sort({l:?})"),
            CoreNode::Const { n, .. } => n.replace(FREE, "$").replace(META, "?"),
            CoreNode::Nat { v } => v.clone(),
            CoreNode::String { v } => format!(
                "{:?}",
                v.iter()
                    .filter_map(|c| char::from_u32(*c))
                    .collect::<String>()
            ),
            CoreNode::App { .. } => {
                let (head, args) = self.spine(term);
                let mut out = format!("({}", self.show_at(head, names, depth + 1));
                for arg in args {
                    out.push(' ');
                    out.push_str(&self.show_at(arg, names, depth + 1));
                }
                out.push(')');
                out
            }
            CoreNode::Lambda { n, b, t, v } | CoreNode::Forall { n, b, t, v } => {
                let keyword = if matches!(self.nodes[term], CoreNode::Lambda { .. }) {
                    "fun"
                } else {
                    "∀"
                };
                let domain = self.show_at(*t, names, depth + 1);
                let label = if n.is_empty() { "_" } else { n.as_str() };
                names.push(label.to_owned());
                let body = self.show_at(*v, names, depth + 1);
                names.pop();
                let (open, close) = match b {
                    CoreBinderInfo::Explicit => ("(", ")"),
                    CoreBinderInfo::Implicit => ("{", "}"),
                    CoreBinderInfo::StrictImplicit => ("⦃", "⦄"),
                    CoreBinderInfo::Instance => ("[", "]"),
                };
                format!("({keyword} {open}{label} : {domain}{close}, {body})")
            }
            CoreNode::Let { n, t, v, body, .. } => {
                let value = self.show_at(*v, names, depth + 1);
                let ty = self.show_at(*t, names, depth + 1);
                names.push(n.clone());
                let rest = self.show_at(*body, names, depth + 1);
                names.pop();
                format!("(let {n} : {ty} := {value}; {rest})")
            }
            CoreNode::Mdata { x, .. } => self.show_at(*x, names, depth + 1),
            CoreNode::Projection { n, x, .. } => {
                format!("({}).{n}", self.show_at(*x, names, depth + 1))
            }
        }
    }

    /// Whether `term` mentions a free variable or a placeholder.
    #[must_use]
    pub fn has_placeholder(&self, term: Term) -> bool {
        self.min_free[term] != usize::MAX || !self.meta_clean(term)
    }

    /// Whether `term` mentions a lemma's placeholder, a binder nothing solved.
    #[must_use]
    pub fn has_unsolved(&self, term: Term) -> bool {
        !self.meta_clean(term)
    }

    /// Whether `term` mentions no lemma placeholder. A node that mentions
    /// none is remembered, so the big values a lemma's binders are solved to
    /// (the arms of a match, the lists a derivation threads) are walked once
    /// and not at every rule applied: without this a match of `n` arms costs
    /// `n²`.
    fn meta_clean(&self, root: Term) -> bool {
        if self.clean.borrow().contains(&root) {
            return true;
        }
        let mut status: HashMap<Term, bool> = HashMap::new();
        let mut stack: Vec<(Term, bool)> = vec![(root, false)];
        while let Some((term, expanded)) = stack.pop() {
            if status.contains_key(&term) {
                continue;
            }
            if self.clean.borrow().contains(&term) {
                status.insert(term, true);
                continue;
            }
            let children: Vec<Term> = match &self.nodes[term] {
                CoreNode::Const { n, .. } => {
                    status.insert(term, !n.starts_with(META));
                    continue;
                }
                CoreNode::BVar { .. }
                | CoreNode::Sort { .. }
                | CoreNode::Nat { .. }
                | CoreNode::String { .. } => {
                    status.insert(term, true);
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
            let all = children
                .iter()
                .all(|child| status.get(child).copied().unwrap_or(false));
            status.insert(term, all);
        }
        let mut clean = self.clean.borrow_mut();
        for (term, ok) in &status {
            if *ok {
                clean.insert(*term);
            }
        }
        status[&root]
    }

    /// Assign the placeholders of `pattern` that `actual` has a term at, by
    /// comparing the two node by node. A shape the two do not share is left
    /// to the kernel.
    fn solve(&self, pattern: Term, actual: Term, solved: &mut HashMap<usize, Term>) {
        if pattern == actual {
            return;
        }
        if let Some(id) = self.meta_id(pattern) {
            if !self.has_loose(actual, 0) {
                solved.entry(id).or_insert(actual);
            }
            return;
        }
        match (&self.nodes[pattern], &self.nodes[actual]) {
            (CoreNode::App { .. }, CoreNode::App { .. }) => {
                // Two applications are compared argument by argument only when
                // they apply the same head to as many arguments: otherwise one
                // is a definition applied that the other unfolds, which the
                // kernel compares.
                let (head, args) = self.spine(pattern);
                let (other_head, other_args) = self.spine(actual);
                let same_head = match (&self.nodes[head], &self.nodes[other_head]) {
                    (CoreNode::Const { n, .. }, CoreNode::Const { n: m, .. }) => n == m,
                    (CoreNode::BVar { i }, CoreNode::BVar { i: j }) => i == j,
                    _ => head == other_head,
                };
                if same_head && args.len() == other_args.len() {
                    for (a, b) in args.iter().zip(&other_args) {
                        self.solve(*a, *b, solved);
                    }
                }
            }
            (
                CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. },
                CoreNode::Lambda { t: u, v: w, .. } | CoreNode::Forall { t: u, v: w, .. },
            ) => {
                self.solve(*t, *u, solved);
                self.solve(*v, *w, solved);
            }
            (
                CoreNode::Let { t, v, body, .. },
                CoreNode::Let {
                    t: t2,
                    v: v2,
                    body: b2,
                    ..
                },
            ) => {
                self.solve(*t, *t2, solved);
                self.solve(*v, *v2, solved);
                self.solve(*body, *b2, solved);
            }
            (CoreNode::Projection { x, .. }, CoreNode::Projection { x: y, .. }) => {
                self.solve(*x, *y, solved);
            }
            _ => {}
        }
    }

    /// `body` with `#0` replaced by `arg`, a function applied to the bound
    /// variable reduced as the elaborator reduces it when it instantiates a
    /// binder: the type of `conv_cond` states `P b` and the type of the
    /// application states the predicate's body.
    pub fn instantiate_beta(&mut self, body: Term, arg: Term) -> Term {
        if self.loose_bound(body) == 0 {
            return body;
        }
        let mut memo = HashMap::new();
        self.beta_memo(body, arg, 0, &mut memo)
    }

    fn beta_memo(
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
                let f = self.beta_memo(f, arg, depth, memo);
                let x = self.beta_memo(x, arg, depth, memo);
                match self.nodes[f].clone() {
                    CoreNode::Lambda { v, .. } => self.instantiate_beta(v, x),
                    _ => self.app(f, x),
                }
            }
            CoreNode::Lambda { n, b, t, v } => {
                let t = self.beta_memo(t, arg, depth, memo);
                let v = self.beta_memo(v, arg, depth + 1, memo);
                self.lam(&n, b, t, v)
            }
            CoreNode::Forall { n, b, t, v } => {
                let t = self.beta_memo(t, arg, depth, memo);
                let v = self.beta_memo(v, arg, depth + 1, memo);
                self.pi(&n, b, t, v)
            }
            CoreNode::Let { n, t, v, body, d } => {
                let t = self.beta_memo(t, arg, depth, memo);
                let v = self.beta_memo(v, arg, depth, memo);
                let body = self.beta_memo(body, arg, depth + 1, memo);
                self.let_node(&n, t, v, body, d)
            }
            CoreNode::Mdata { m, x } => {
                let x = self.beta_memo(x, arg, depth, memo);
                self.mdata_node(m, x)
            }
            CoreNode::Projection { n, s, i, x } => {
                let x = self.beta_memo(x, arg, depth, memo);
                self.projection(&n, &s, i, x)
            }
        };
        memo.insert((term, depth), out);
        out
    }

    /// `term` with every solved placeholder replaced, transitively.
    fn resolve(&mut self, term: Term, solved: &HashMap<usize, Term>) -> Term {
        if solved.is_empty() || self.meta_clean(term) {
            return term;
        }
        let values_clean = solved.values().all(|&value| self.meta_clean(value));
        let mut out = term;
        for _ in 0..64 {
            let mut memo = HashMap::new();
            let next = self.resolve_memo(out, solved, &mut memo);
            if values_clean || next == out {
                return next;
            }
            out = next;
        }
        out
    }

    /// Replacement of placeholders that reduces a solved function applied to
    /// its arguments, as instantiating a binder does.
    fn resolve_memo(
        &mut self,
        term: Term,
        solved: &HashMap<usize, Term>,
        memo: &mut HashMap<Term, Term>,
    ) -> Term {
        if let Some(&done) = memo.get(&term) {
            return done;
        }
        if self.meta_clean(term) {
            return term;
        }
        let out = match self.nodes[term].clone() {
            CoreNode::Const { .. } => self
                .meta_id(term)
                .and_then(|id| solved.get(&id).copied())
                .unwrap_or(term),
            CoreNode::BVar { .. }
            | CoreNode::Sort { .. }
            | CoreNode::Nat { .. }
            | CoreNode::String { .. } => term,
            CoreNode::App { f, x } => {
                let (head, _) = self.spine(f);
                let replaced_head = self.meta_id(head).is_some();
                let f2 = self.resolve_memo(f, solved, memo);
                let x2 = self.resolve_memo(x, solved, memo);
                match self.nodes[f2].clone() {
                    CoreNode::Lambda { v, .. } if replaced_head => self.instantiate_beta(v, x2),
                    _ => self.app(f2, x2),
                }
            }
            CoreNode::Lambda { n, b, t, v } => {
                let t = self.resolve_memo(t, solved, memo);
                let v = self.resolve_memo(v, solved, memo);
                self.lam(&n, b, t, v)
            }
            CoreNode::Forall { n, b, t, v } => {
                let t = self.resolve_memo(t, solved, memo);
                let v = self.resolve_memo(v, solved, memo);
                self.pi(&n, b, t, v)
            }
            CoreNode::Let { n, t, v, body, d } => {
                let t = self.resolve_memo(t, solved, memo);
                let v = self.resolve_memo(v, solved, memo);
                let body = self.resolve_memo(body, solved, memo);
                self.let_node(&n, t, v, body, d)
            }
            CoreNode::Mdata { m, x } => {
                let x = self.resolve_memo(x, solved, memo);
                self.mdata_node(m, x)
            }
            CoreNode::Projection { n, s, i, x } => {
                let x = self.resolve_memo(x, solved, memo);
                self.projection(&n, &s, i, x)
            }
        };
        memo.insert(term, out);
        out
    }

    /// Open a lemma for application: every binder of `ty` becomes a
    /// placeholder, the hint of its name (when `hints` has one) is its
    /// solution, and the caller supplies the explicit ones, in order, with
    /// [`Open::give`] and reads what is already determined with
    /// [`Open::domain`] and [`Open::value`].
    ///
    /// # Errors
    ///
    /// None; the signature mirrors [`Open::finish`] for symmetry.
    pub fn open_lemma(
        &mut self,
        name: &str,
        head: Term,
        ty: Term,
        hints: &[(&str, Term)],
    ) -> Result<Open, String> {
        if let Some(template) = self.opened.get(name) {
            let mut open = template.clone();
            for binder in &open.binders {
                if let Some((_, hint)) = hints.iter().find(|(wanted, _)| *wanted == binder.name) {
                    open.solved.insert(binder.id, *hint);
                }
            }
            return Ok(open);
        }
        let mut open = Open {
            name: name.to_owned(),
            head,
            binders: Vec::new(),
            conclusion: ty,
            solved: HashMap::new(),
            next: 0,
        };
        let mut ty = ty;
        while let CoreNode::Forall { n, b, t, v } = &self.nodes[ty] {
            let (binder, info, domain, body) = (n.clone(), *b, *t, *v);
            let (id, placeholder) = self.meta();
            open.binders.push(OpenBinder {
                name: binder,
                explicit: info == CoreBinderInfo::Explicit,
                domain,
                id,
                placeholder,
            });
            ty = self.instantiate_beta(body, placeholder);
        }
        open.conclusion = ty;
        self.opened.insert(name.to_owned(), open.clone());
        for binder in &open.binders {
            if let Some((_, hint)) = hints.iter().find(|(wanted, _)| *wanted == binder.name) {
                open.solved.insert(binder.id, *hint);
            }
        }
        Ok(open)
    }

    /// `head`, whose type is `ty`, applied to `given` at its explicit binders.
    /// An implicit or instance binder takes the hint of its name when `hints`
    /// has one, else the term the types of the arguments determine.
    ///
    /// # Errors
    ///
    /// More arguments than explicit binders, a reflexivity argument at a
    /// binder that is not an equation, or a placeholder no argument, hint, or
    /// reflexivity determined.
    pub fn apply_lemma(
        &mut self,
        name: &str,
        head: Term,
        ty: Term,
        hints: &[(&str, Term)],
        given: &[Given],
    ) -> Result<Applied, String> {
        let mut open = self.open_lemma(name, head, ty, hints)?;
        for argument in given {
            open.give(self, *argument)?;
        }
        open.finish(self)
    }
}

/// One binder of a lemma being applied.
#[derive(Debug, Clone)]
struct OpenBinder {
    name: String,
    explicit: bool,
    /// Its type, over the placeholders of the earlier binders.
    domain: Term,
    id: usize,
    placeholder: Term,
}

/// A lemma being applied: its binders as placeholders, with what is solved
/// so far.
#[derive(Debug, Clone)]
pub struct Open {
    name: String,
    head: Term,
    binders: Vec<OpenBinder>,
    conclusion: Term,
    solved: HashMap<usize, Term>,
    /// The next explicit binder [`Open::give`] fills.
    next: usize,
}

impl Builder {
    /// `Eq.refl` at the equation `domain` states; for a statement under
    /// binders, the function from them. A placeholder on a side of the
    /// equation is solved by the other side.
    fn refl_proof(
        &mut self,
        name: &str,
        position: usize,
        domain: Term,
        solved: &mut HashMap<usize, Term>,
    ) -> Result<Term, String> {
        if let CoreNode::Forall { n, b: info, t, v } = self.nodes[domain].clone() {
            let free = self.free();
            let body = self.instantiate_beta(v, free);
            let inner = self.refl_proof(name, position, body, solved)?;
            return Ok(self.lam_over(&n, info, t, free, inner));
        }
        let (eq, parts) = self.spine(domain);
        let levels = match &self.nodes[eq] {
            CoreNode::Const { n, u } if n == "Eq" && parts.len() == 3 => u.clone(),
            _ => {
                return Err(format!(
                    "`{name}`: argument {position} is a reflexivity proof at {}, which is not an equation",
                    self.show(domain)
                ));
            }
        };
        if let Some(solution) = self.meta_id(parts[2]) {
            solved.entry(solution).or_insert(parts[1]);
        }
        if let Some(solution) = self.meta_id(parts[1]) {
            solved.entry(solution).or_insert(parts[2]);
        }
        let refl = self.konst_at("Eq.refl", levels);
        let sides = self.resolve(parts[1], solved);
        let carrier = self.resolve(parts[0], solved);
        Ok(self.apps(refl, &[carrier, sides]))
    }
}

impl Open {
    fn explicit_at(&self, position: usize) -> Option<usize> {
        self.binders
            .iter()
            .enumerate()
            .filter(|(_, binder)| binder.explicit)
            .nth(position)
            .map(|(index, _)| index)
    }

    /// Solve the placeholders of the lemma's conclusion that `expected`, the
    /// type its application is wanted at, has a term at.
    pub fn expect(&mut self, b: &Builder, expected: Term) {
        b.solve(self.conclusion, expected, &mut self.solved);
    }

    /// Solve the placeholders of a binder's type against the type of the
    /// argument it takes, without giving the argument.
    pub fn solve_domain(&mut self, b: &Builder, binder: &str, actual: Term) {
        if let Some(found) = self
            .binders
            .iter()
            .find(|candidate| candidate.name == binder)
        {
            let domain = found.domain;
            b.solve(domain, actual, &mut self.solved);
        }
    }

    /// The type of the next explicit binder, with what is solved written in.
    pub fn domain(&self, b: &mut Builder) -> Term {
        let at = self.explicit_at(self.next).map_or_else(
            || {
                self.binders
                    .iter()
                    .find(|binder| binder.explicit)
                    .map_or(self.conclusion, |binder| binder.domain)
            },
            |index| self.binders[index].domain,
        );
        b.resolve(at, &self.solved)
    }

    /// The solution of the binder named `name`, when it has one.
    pub fn value(&self, b: &mut Builder, name: &str) -> Option<Term> {
        let binder = self.binders.iter().find(|binder| binder.name == name)?;
        let value = *self.solved.get(&binder.id)?;
        Some(b.resolve(value, &self.solved))
    }

    /// Give the next explicit binder its argument.
    ///
    /// # Errors
    ///
    /// A reflexivity argument at a binder that is not an equation, or more
    /// arguments than explicit binders.
    pub fn give(&mut self, b: &mut Builder, argument: Given) -> Result<(), String> {
        let Some(index) = self.explicit_at(self.next) else {
            return Err(format!(
                "`{}`: more arguments than explicit binders ({})",
                self.name, self.next
            ));
        };
        let domain = self.binders[index].domain;
        let id = self.binders[index].id;
        let position = self.next;
        self.next += 1;
        let value = match argument {
            Given::Term(value) => value,
            Given::Proof(value, actual) => {
                b.solve(domain, actual, &mut self.solved);
                value
            }
            Given::Refl => {
                let domain = b.resolve(domain, &self.solved);
                b.refl_proof(&self.name, position, domain, &mut self.solved)?
            }
        };
        self.solved.insert(id, value);
        Ok(())
    }

    /// The application, once every explicit binder has its argument.
    ///
    /// # Errors
    ///
    /// A binder no argument, hint, or expected type determined, by name.
    pub fn finish(self, b: &mut Builder) -> Result<Applied, String> {
        let mut term = self.head;
        for binder in &self.binders {
            let value = match self.solved.get(&binder.id) {
                Some(value) => *value,
                None => binder.placeholder,
            };
            term = b.app(term, value);
        }
        let term = b.resolve(term, &self.solved);
        let ty = b.resolve(self.conclusion, &self.solved);
        if b.has_unsolved(term) || b.has_unsolved(ty) {
            let open: Vec<&str> = self
                .binders
                .iter()
                .filter(|binder| !self.solved.contains_key(&binder.id))
                .map(|binder| binder.name.as_str())
                .collect();
            return Err(format!(
                "`{}`: the binders {open:?} are determined by no argument, hint, or expected type",
                self.name
            ));
        }
        Ok(Applied { term, ty })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_free_variable_becomes_the_bound_variable_of_its_lambda() {
        let mut b = Builder::new();
        let nat = b.konst("Nat");
        let x = b.free();
        let f = b.konst("f");
        let body = b.apps(f, &[x, x]);
        let lam = b.lam_over("x", CoreBinderInfo::Explicit, nat, x, body);
        let bound = b.bvar(0);
        let applied = b.apps(f, &[bound, bound]);
        assert!(!b.has_placeholder(lam));
        match b.node(lam) {
            CoreNode::Lambda { v, .. } => assert_eq!(*v, applied),
            other => panic!("not a lambda: {other:?}"),
        }
    }

    #[test]
    fn implicit_binders_are_solved_from_the_types_of_the_arguments() {
        // lemma : {a : Nat} -> (h : P a) -> Q a
        let mut b = Builder::new();
        let nat = b.konst("Nat");
        let p = b.konst("P");
        let q = b.konst("Q");
        let a0 = b.bvar(0);
        let pa = b.app(p, a0);
        let a1 = b.bvar(1);
        let qa = b.app(q, a1);
        let inner = b.pi("h", CoreBinderInfo::Explicit, pa, qa);
        let ty = b.pi("a", CoreBinderInfo::Implicit, nat, inner);
        let lemma = b.konst("lemma");
        let five = b.nat(5);
        let proof = b.konst("proof");
        let p5 = b.app(p, five);
        let applied = b
            .apply_lemma("lemma", lemma, ty, &[], &[Given::Proof(proof, p5)])
            .expect("a is determined by the argument's type");
        let q5 = b.app(q, five);
        assert_eq!(applied.ty, q5);
        let (_, args) = b.spine(applied.term);
        assert_eq!(args, vec![five, proof]);
    }

    #[test]
    fn an_undetermined_implicit_binder_is_named() {
        let mut b = Builder::new();
        let nat = b.konst("Nat");
        let prop = b.prop();
        let lifted = b.lift(prop, 1, 0);
        let ty = b.pi("a", CoreBinderInfo::Implicit, nat, lifted);
        let lemma = b.konst("lemma");
        let error = b
            .apply_lemma("lemma", lemma, ty, &[], &[])
            .expect_err("nothing determines a");
        assert!(error.contains("[\"a\"]"), "{error}");
    }
}
