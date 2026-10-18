//! One-time repair of the authored proofs of the preservation library that a
//! constructor added to `Callee` and to `Lit` leaves one case short.
use lexlean::ir::core::{CoreBinderInfo, CoreDeclaration, CoreModule, CoreNode};
use lexlean::surface::core as notation;
use lexlean::surface::module::locate;
use std::collections::{BTreeMap, BTreeSet};

const MATCH: &str = "LexLeanTarget.RustSemantics.eval.match_3";
const CASES: &str = "LexLeanTarget.RustSyntax.Lit.casesOn";
const RUN_ITEM: &str = "LexLeanTarget.RustSemantics.runItem";
const RUN_MACHINE: &str = "LexLeanTarget.RustSemantics.runMachine";
const MACHINE: &str = "LexLeanTarget.RustSyntax.Machine";
const CALLEE_CASES: &str = "LexLeanTarget.RustSyntax.Callee.casesOn";
const CALLEE_RUNTIME: &str = "LexLeanTarget.RustSyntax.Callee.runtime";
const CALLEE_MACHINE: &str = "LexLeanTarget.RustSyntax.Callee.machine";
const ITEM: &str = "LexLeanTarget.RustSyntax.Item";
const LIT_UNIT: &str = "LexLeanTarget.RustSyntax.Lit.unit";
const LIT_OVERFLOW: &str = "LexLeanTarget.RustSyntax.Lit.overflow";

struct Patch<'a> {
    old: &'a [CoreNode],
    nodes: Vec<CoreNode>,
    memo: BTreeMap<usize, usize>,
    flagged: BTreeSet<usize>,
    proofs: &'a BTreeSet<usize>,
    matches: usize,
    cases: usize,
    callee_cases: usize,
}

impl Patch<'_> {
    fn push(&mut self, node: CoreNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }
    fn spine(&self, mut i: usize) -> (usize, Vec<usize>) {
        let mut args = vec![];
        while let CoreNode::App { f, x } = &self.old[i] {
            args.push(*x);
            i = *f;
        }
        args.reverse();
        (i, args)
    }
    fn is_const(&self, i: usize, name: &str) -> bool {
        matches!(&self.old[i], CoreNode::Const { n, .. } if n == name)
    }
    fn constant(&mut self, name: &str) -> usize {
        self.push(CoreNode::Const { n: name.to_owned(), u: vec![] })
    }
    /// A copy of the old node `i` with the constant `from` replaced by `to`.
    fn rename_unit(&mut self, i: usize, memo: &mut BTreeMap<usize, usize>) -> usize {
        self.rename(i, memo, LIT_UNIT, LIT_OVERFLOW)
    }
    fn rename(&mut self, i: usize, memo: &mut BTreeMap<usize, usize>, from: &str, to: &str) -> usize {
        if let Some(done) = memo.get(&i) { return *done; }
        let node = self.nodes[i].clone();
        let out = match node {
            CoreNode::Const { n, u } if n == from => self.push(CoreNode::Const { n: to.to_owned(), u }),
            CoreNode::App { f, x } => { let (f, x) = (self.rename(f, memo, from, to), self.rename(x, memo, from, to)); self.push(CoreNode::App { f, x }) }
            CoreNode::Lambda { n, b, t, v } => { let (t, v) = (self.rename(t, memo, from, to), self.rename(v, memo, from, to)); self.push(CoreNode::Lambda { n, b, t, v }) }
            CoreNode::Forall { n, b, t, v } => { let (t, v) = (self.rename(t, memo, from, to), self.rename(v, memo, from, to)); self.push(CoreNode::Forall { n, b, t, v }) }
            CoreNode::Let { n, t, v, body, d } => { let (t, v, body) = (self.rename(t, memo, from, to), self.rename(v, memo, from, to), self.rename(body, memo, from, to)); self.push(CoreNode::Let { n, t, v, body, d }) }
            CoreNode::Projection { n, s, i, x } => { let x = self.rename(x, memo, from, to); self.push(CoreNode::Projection { n, s, i, x }) }
            CoreNode::Mdata { m, x } => { let x = self.rename(x, memo, from, to); self.push(CoreNode::Mdata { m, x }) }
            other => self.push(other),
        };
        memo.insert(i, out);
        out
    }
    fn rw(&mut self, i: usize) -> Result<usize, String> {
        if let Some(done) = self.memo.get(&i) { return Ok(*done); }
        let out = match self.old[i].clone() {
            CoreNode::App { f, x } => {
                let (head, args) = self.spine(i);
                if self.is_const(head, MATCH) && args.len() == 4 {
                    self.matches += 1;
                    let rewritten: Vec<usize> = args.iter().map(|a| self.rw(*a)).collect::<Result<_, _>>()?;
                    let h = self.rw(head)?;
                    // alt2 = fun item => runItem profile item operands
                    let CoreNode::Lambda { v: body, .. } = self.nodes[rewritten[3]].clone() else { return Err(format!("site {i}: the runtime alternative is not a lambda")) };
                    let CoreNode::App { f: partial, x: operands } = self.nodes[body].clone() else { return Err(format!("site {i}: the runtime alternative is not a call")) };
                    let CoreNode::App { f: call_head, .. } = self.nodes[partial].clone() else { return Err(format!("site {i}: the runtime alternative has no profile")) };
                    let CoreNode::App { f: run_item, .. } = self.nodes[call_head].clone() else { return Err(format!("site {i}: unexpected call shape")) };
                    if !matches!(&self.nodes[run_item], CoreNode::Const { n, .. } if n == RUN_ITEM) { return Err(format!("site {i}: the runtime alternative does not call runItem")); }
                    let machine_type = self.constant(MACHINE);
                    let run_machine = self.constant(RUN_MACHINE);
                    let primitive = self.push(CoreNode::BVar { i: 0 });
                    let applied = self.push(CoreNode::App { f: run_machine, x: primitive });
                    let call = self.push(CoreNode::App { f: applied, x: operands });
                    let alt3 = self.push(CoreNode::Lambda { n: "primitive".to_owned(), b: CoreBinderInfo::Explicit, t: machine_type, v: call });
                    let mut node = h;
                    for a in rewritten.iter().copied().chain([alt3]) { node = self.push(CoreNode::App { f: node, x: a }); }
                    node
                } else if self.is_const(head, CALLEE_CASES) && args.len() == 5 {
                    self.callee_cases += 1;
                    // the arm for `machine` is the arm for `runtime` over a primitive instead of an item
                    let mut memo = BTreeMap::new();
                    let rewritten: Vec<usize> = args.iter().map(|a| self.rw(*a)).collect::<Result<_, _>>()?;
                    let CoreNode::Lambda { n, b, t, v } = self.nodes[rewritten[3]].clone() else { return Err(format!("site {i}: the runtime arm is not a lambda")) };
                    if !matches!(&self.nodes[t], CoreNode::Const { n, .. } if n == ITEM) { return Err(format!("site {i}: the runtime arm does not bind an item")) };
                    let machine_type = self.constant(MACHINE);
                    let body = self.rename(v, &mut memo, CALLEE_RUNTIME, CALLEE_MACHINE);
                    let machine_arm = self.push(CoreNode::Lambda { n, b, t: machine_type, v: body });
                    let h = self.rw(head)?;
                    let mut node = h;
                    for (position, a) in rewritten.iter().copied().enumerate() {
                        if position == 4 { node = self.push(CoreNode::App { f: node, x: machine_arm }); }
                        node = self.push(CoreNode::App { f: node, x: a });
                    }
                    node
                } else if self.is_const(head, CASES) && args.len() == 11 {
                    self.cases += 1;
                    let mut memo = BTreeMap::new();
                    let rewritten: Vec<usize> = args.iter().map(|a| self.rw(*a)).collect::<Result<_, _>>()?;
                    let overflow_arm = self.rename_unit(rewritten[2], &mut memo);
                    // the arm for `overflow` is the arm for `unit` with the constructor changed; it comes after the 8 arms
                    let h = self.rw(head)?;
                    let mut node = h;
                    for (position, a) in rewritten.iter().copied().enumerate() {
                        if position == 10 { node = self.push(CoreNode::App { f: node, x: overflow_arm }); }
                        node = self.push(CoreNode::App { f: node, x: a });
                    }
                    node
                } else {
                    let (nf, nx) = (self.rw(f)?, self.rw(x)?);
                    self.push(CoreNode::App { f: nf, x: nx })
                }
            }
            CoreNode::Lambda { n, b, t, v } => { let (nt, nv) = (self.rw(t)?, self.rw(v)?); self.rebuild(i, &[t, v], &[nt, nv], CoreNode::Lambda { n, b, t: nt, v: nv }) }
            CoreNode::Forall { n, b, t, v } => { let (nt, nv) = (self.rw(t)?, self.rw(v)?); self.rebuild(i, &[t, v], &[nt, nv], CoreNode::Forall { n, b, t: nt, v: nv }) }
            CoreNode::Let { n, t, v, body, d } => { let (nt, nv, nb) = (self.rw(t)?, self.rw(v)?, self.rw(body)?); self.rebuild(i, &[t, v, body], &[nt, nv, nb], CoreNode::Let { n, t: nt, v: nv, body: nb, d }) }
            CoreNode::Projection { n, s, i: idx, x } => { let nx = self.rw(x)?; self.rebuild(i, &[x], &[nx], CoreNode::Projection { n, s, i: idx, x: nx }) }
            CoreNode::Mdata { m, x } => { let nx = self.rw(x)?; self.rebuild(i, &[x], &[nx], CoreNode::Mdata { m, x: nx }) }
            other => self.push(other),
        };
        if self.proofs.contains(&i) { self.flagged.insert(out); }
        self.memo.insert(i, out);
        Ok(out)
    }
    fn rebuild(&mut self, _i: usize, _old: &[usize], _new: &[usize], node: CoreNode) -> usize { self.push(node) }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let path = &a[1];
    let text = std::fs::read_to_string(path).unwrap();
    let located = locate(&text).unwrap();
    let body = &text[located.body.0..located.body.1];
    let mut module = CoreModule::from_source(&text, "1.2").unwrap();
    let proofs: BTreeSet<usize> = module.proof_nodes.iter().copied().collect();
    let wanted: BTreeSet<&str> = a[2..].iter().map(String::as_str).collect();
    let mut patch = Patch { old: &module.nodes.clone(), nodes: vec![], memo: BTreeMap::new(), flagged: BTreeSet::new(), proofs: &proofs, matches: 0, cases: 0, callee_cases: 0 };
    // copy the whole table first so every old node has a counterpart, then rewrite the wanted values
    // (a simple scheme: the rewritten graph is appended after the old table)
    patch.nodes = module.nodes.clone();
    let base = patch.nodes.len();
    for row in module.declarations.iter_mut() {
        if !wanted.iter().any(|w| row.name.ends_with(&format!(".{w}"))) { continue; }
        let before = (patch.matches, patch.cases);
        let root = row.value.expect("a theorem");
        let mut fresh = Patch { old: patch.old, nodes: std::mem::take(&mut patch.nodes), memo: BTreeMap::new(), flagged: std::mem::take(&mut patch.flagged), proofs: patch.proofs, matches: 0, cases: 0, callee_cases: 0 };
        let new_root = fresh.rw(root).unwrap_or_else(|e| panic!("{}: {e}", row.name));
        row.value = Some(new_root);
        patch.nodes = fresh.nodes; patch.flagged = fresh.flagged;
        patch.matches += fresh.matches; patch.cases += fresh.cases; patch.callee_cases += fresh.callee_cases;
        println!("{}: {} eval.match_3 sites, {} Lit.casesOn sites, {} Callee.casesOn sites", row.name, patch.matches - before.0, patch.cases - before.1, fresh.callee_cases);
    }
    let _ = (base, wanted);
    let flagged = patch.flagged.clone();
    module.nodes = patch.nodes;
    module.proof_nodes = module.proof_nodes.iter().copied().chain(flagged).collect::<BTreeSet<_>>().into_iter().collect();
    // prune what nothing reaches, renumbering in order
    let mut needed = vec![false; module.nodes.len()];
    let mut work: Vec<usize> = vec![];
    for row in &module.declarations {
        work.extend(row.expression_roots());
        if let Some(info) = &row.eqn_info { work.extend([info.r#type, info.value]); }
        if let Some(st) = row.inductive.as_ref().and_then(|i| i.structure.as_ref()) { work.extend(st.fields.iter().filter_map(|f| f.auto_param)); }
    }
    fn kids(n: &CoreNode) -> Vec<usize> { match n { CoreNode::App { f, x } => vec![*f, *x], CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => vec![*t, *v], CoreNode::Let { t, v, body, .. } => vec![*t, *v, *body], CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => vec![*x], _ => vec![] } }
    while let Some(n) = work.pop() { if needed[n] { continue; } needed[n] = true; work.extend(kids(&module.nodes[n])); }
    let mut map = vec![usize::MAX; module.nodes.len()];
    let mut kept = Vec::new();
    for (i, node) in module.nodes.iter().enumerate() {
        if needed[i] {
            map[i] = kept.len();
            let mut node = node.clone();
            match &mut node {
                CoreNode::App { f, x } => { *f = map[*f]; *x = map[*x]; }
                CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => { *t = map[*t]; *v = map[*v]; }
                CoreNode::Let { t, v, body, .. } => { *t = map[*t]; *v = map[*v]; *body = map[*body]; }
                CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => { *x = map[*x]; }
                _ => {}
            }
            kept.push(node);
        }
    }
    for row in &mut module.declarations {
        row.r#type = row.r#type.map(|r| map[r]);
        row.value = row.value.map(|r| map[r]);
        if let Some(info) = row.eqn_info.as_mut() { info.r#type = map[info.r#type]; info.value = map[info.value]; }
        if let Some(st) = row.inductive.as_mut().and_then(|i| i.structure.as_mut()) { for f in &mut st.fields { f.auto_param = f.auto_param.map(|r| map[r]); } }
    }
    module.proof_nodes = module.proof_nodes.iter().filter(|p| needed[**p]).map(|p| map[*p]).collect::<BTreeSet<_>>().into_iter().collect();
    module.nodes = kept;
    module.validate("1.2").unwrap_or_else(|e| panic!("{e}"));
    let printed = notation::print_module(&module).unwrap();
    assert_eq!(notation::parse_module(&printed).unwrap(), module);
    let lead = &text[located.body.0..located.body.0 + (body.len() - body.trim_start_matches('\n').len())];
    let trail_len = body.len() - body.trim_end_matches('\n').len();
    let trail = &body[body.len() - trail_len..];
    std::fs::write(path, format!("{}{}{}{}{}", &text[..located.body.0], lead, printed.trim_matches('\n'), trail, &text[located.body.1..])).unwrap();
    println!("nodes -> {}", module.nodes.len());
    let _: Option<&CoreDeclaration> = None;
}
