//! One-time: give `FunOK` its third disjunct (`FunLoop`) and `funSem_of` the arm for it.
//! `funok corr <RustCorr.lex.tex>` renames the old `FunOK` to `FunOK0` and states
//! `FunOK := FunOK0 ∨ FunLoop`; `funok sound <RustSound.lex.tex>` makes the case split in
//! `funSem_of` on that disjunction, the old proof for the first arm and `funSem_loop` for the second.
use lexlean::ir::core::{CoreBinderInfo, CoreModule, CoreNode, CoreAxiomPolicy, CoreReducibility};
use lexlean::surface::core as notation;
use lexlean::surface::module::locate;
use std::collections::BTreeMap;

const FUNOK: &str = "LexLeanPreservation.Rust.FunOK";
const FUNOK0: &str = "LexLeanPreservation.Rust.FunOK0";
const FUNLOOP: &str = "LexLeanPreservation.Rust.FunLoop";
const CRATEOK: &str = "LexLeanPreservation.Rust.CrateOK";
const FUNSEM_OF: &str = "LexLeanPreservation.Rust.funSem_of";
const FUNSEM_LOOP: &str = "LexLeanPreservation.Rust.funSem_loop";

struct B {
    nodes: Vec<CoreNode>,
}

impl B {
    fn push(&mut self, n: CoreNode) -> usize {
        self.nodes.push(n);
        self.nodes.len() - 1
    }
    fn constant(&mut self, n: &str) -> usize {
        self.push(CoreNode::Const { n: n.to_owned(), u: vec![] })
    }
    fn app(&mut self, f: usize, args: &[usize]) -> usize {
        let mut node = f;
        for a in args {
            node = self.push(CoreNode::App { f: node, x: *a });
        }
        node
    }
    fn bvar(&mut self, i: usize) -> usize {
        self.push(CoreNode::BVar { i })
    }
    fn lam(&mut self, n: &str, t: usize, v: usize) -> usize {
        self.push(CoreNode::Lambda { n: n.to_owned(), b: CoreBinderInfo::Explicit, t, v })
    }
    /// Shift the loose variables of `i` at or above `cutoff` by one.
    fn shift(&mut self, i: usize, cutoff: usize, memo: &mut BTreeMap<(usize, usize), usize>) -> usize {
        if let Some(d) = memo.get(&(i, cutoff)) {
            return *d;
        }
        let out = match self.nodes[i].clone() {
            CoreNode::BVar { i: k } => {
                if k >= cutoff {
                    self.push(CoreNode::BVar { i: k + 1 })
                } else {
                    i
                }
            }
            CoreNode::App { f, x } => {
                let (nf, nx) = (self.shift(f, cutoff, memo), self.shift(x, cutoff, memo));
                self.push(CoreNode::App { f: nf, x: nx })
            }
            CoreNode::Lambda { n, b, t, v } => {
                let (nt, nv) = (self.shift(t, cutoff, memo), self.shift(v, cutoff + 1, memo));
                self.push(CoreNode::Lambda { n, b, t: nt, v: nv })
            }
            CoreNode::Forall { n, b, t, v } => {
                let (nt, nv) = (self.shift(t, cutoff, memo), self.shift(v, cutoff + 1, memo));
                self.push(CoreNode::Forall { n, b, t: nt, v: nv })
            }
            CoreNode::Let { n, t, v, body, d } => {
                let nt = self.shift(t, cutoff, memo);
                let nv = self.shift(v, cutoff, memo);
                let nb = self.shift(body, cutoff + 1, memo);
                self.push(CoreNode::Let { n, t: nt, v: nv, body: nb, d })
            }
            CoreNode::Projection { n, s, i: idx, x } => {
                let nx = self.shift(x, cutoff, memo);
                self.push(CoreNode::Projection { n, s, i: idx, x: nx })
            }
            CoreNode::Mdata { m, x } => {
                let nx = self.shift(x, cutoff, memo);
                self.push(CoreNode::Mdata { m, x: nx })
            }
            _ => i,
        };
        memo.insert((i, cutoff), out);
        out
    }
}

fn spine(nodes: &[CoreNode], mut i: usize) -> (usize, Vec<usize>) {
    let mut args = vec![];
    while let CoreNode::App { f, x } = &nodes[i] {
        args.push(*x);
        i = *f;
    }
    args.reverse();
    (i, args)
}

fn kids(n: &CoreNode) -> Vec<usize> {
    match n {
        CoreNode::App { f, x } => vec![*f, *x],
        CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => vec![*t, *v],
        CoreNode::Let { t, v, body, .. } => vec![*t, *v, *body],
        CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => vec![*x],
        _ => vec![],
    }
}

fn finish(module: &mut CoreModule, nodes: Vec<CoreNode>, text: &str, target: &str, located_body: (usize, usize)) {
    module.nodes = nodes;
    // prune and renumber
    let mut needed = vec![false; module.nodes.len()];
    let mut work: Vec<usize> = vec![];
    for row in &module.declarations {
        work.extend(row.expression_roots());
        if let Some(info) = &row.eqn_info {
            work.extend([info.r#type, info.value]);
        }
        if let Some(st) = row.inductive.as_ref().and_then(|i| i.structure.as_ref()) {
            work.extend(st.fields.iter().filter_map(|f| f.auto_param));
        }
    }
    while let Some(n) = work.pop() {
        if needed[n] {
            continue;
        }
        needed[n] = true;
        work.extend(kids(&module.nodes[n]));
    }
    let mut map = vec![usize::MAX; module.nodes.len()];
    let mut kept = Vec::new();
    for (i, node) in module.nodes.iter().enumerate() {
        if needed[i] {
            map[i] = kept.len();
            let mut node = node.clone();
            match &mut node {
                CoreNode::App { f, x } => {
                    *f = map[*f];
                    *x = map[*x];
                }
                CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => {
                    *t = map[*t];
                    *v = map[*v];
                }
                CoreNode::Let { t, v, body, .. } => {
                    *t = map[*t];
                    *v = map[*v];
                    *body = map[*body];
                }
                CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => *x = map[*x],
                _ => {}
            }
            kept.push(node);
        }
    }
    for row in &mut module.declarations {
        row.r#type = row.r#type.map(|r| map[r]);
        row.value = row.value.map(|r| map[r]);
        if let Some(info) = row.eqn_info.as_mut() {
            info.r#type = map[info.r#type];
            info.value = map[info.value];
        }
        if let Some(st) = row.inductive.as_mut().and_then(|i| i.structure.as_mut()) {
            for f in &mut st.fields {
                f.auto_param = f.auto_param.map(|r| map[r]);
            }
        }
    }
    module.proof_nodes = module
        .proof_nodes
        .iter()
        .filter(|p| needed[**p])
        .map(|p| map[*p])
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    module.nodes = kept;
    module.validate("1.2").unwrap_or_else(|e| panic!("{e}"));
    let printed = notation::print_module(module).unwrap();
    assert_eq!(notation::parse_module(&printed).unwrap(), *module);
    let body = &text[located_body.0..located_body.1];
    let lead = &text[located_body.0..located_body.0 + (body.len() - body.trim_start_matches('\n').len())];
    let trail_len = body.len() - body.trim_end_matches('\n').len();
    let trail = &body[body.len() - trail_len..];
    std::fs::write(
        target,
        format!("{}{}{}{}{}", &text[..located_body.0], lead, printed.trim_matches('\n'), trail, &text[located_body.1..]),
    )
    .unwrap();
    println!("nodes -> {}", module.nodes.len());
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (mode, target) = (&a[1], &a[2]);
    let text = std::fs::read_to_string(target).unwrap();
    let located = locate(&text).unwrap();
    let mut module = CoreModule::from_source(&text, "1.2").unwrap();
    let mut b = B { nodes: module.nodes.clone() };
    match mode.as_str() {
        "corr" => {
            let at = module.declarations.iter().position(|r| r.name == FUNOK).expect("FunOK");
            let old = module.declarations[at].clone();
            let height = |name: &str, module: &CoreModule| match module
                .declarations
                .iter()
                .find(|r| r.name == name)
                .and_then(|r| r.reducibility.clone())
            {
                Some(CoreReducibility::Regular(h)) => h,
                _ => panic!("no height for {name}"),
            };
            let loop_height = height(FUNLOOP, &module);
            let old_height = height(FUNOK, &module);
            // FunOK0 is the old FunOK
            let mut zero = old.clone();
            zero.name = FUNOK0.to_owned();
            zero.all = Some(vec![FUNOK0.to_owned()]);
            // FunOK := fun p c A f => Or (FunOK0 p c A f) (FunLoop p c A f)
            let mut types = vec![];
            let mut v = old.value.unwrap();
            for _ in 0..4 {
                let CoreNode::Lambda { t, v: inner, .. } = b.nodes[v].clone() else { panic!("FunOK is not a 4-ary lambda") };
                types.push(t);
                v = inner;
            }
            let args: Vec<usize> = [3usize, 2, 1, 0].iter().map(|i| b.bvar(*i)).collect();
            let c0 = b.constant(FUNOK0);
            let left = b.app(c0, &args);
            let c1 = b.constant(FUNLOOP);
            let args2: Vec<usize> = [3usize, 2, 1, 0].iter().map(|i| b.bvar(*i)).collect();
            let right = b.app(c1, &args2);
            let or = b.constant("Or");
            let mut body = b.app(or, &[left, right]);
            for (n, t) in ["f", "A", "c", "p"].iter().zip(types.iter().rev()) {
                body = b.lam(n, *t, body);
            }
            let mut new = old.clone();
            new.value = Some(body);
            new.reducibility = Some(CoreReducibility::Regular(old_height.max(loop_height) + 1));
            let loop_axioms = module.declarations.iter().find(|r| r.name == FUNLOOP).unwrap().policy.axioms().to_vec();
            let mut axioms: Vec<String> = old.policy.axioms().to_vec();
            axioms.extend(loop_axioms);
            axioms.sort();
            axioms.dedup();
            new.policy = CoreAxiomPolicy::Exact(axioms.clone());
            module.declarations[at] = new;
            module.declarations.insert(at, zero);
            let crate_at = module.declarations.iter().position(|r| r.name == CRATEOK).expect("CrateOK");
            module.declarations[crate_at].reducibility =
                Some(CoreReducibility::Regular(old_height.max(loop_height) + 2));
            module.declarations[crate_at].policy = CoreAxiomPolicy::Exact(axioms);
            let nodes = std::mem::take(&mut b.nodes);
            finish(&mut module, nodes, &text, target, located.body);
        }
        "sound" => {
            let at = module.declarations.iter().position(|r| r.name == FUNSEM_OF).expect("funSem_of");
            let root = module.declarations[at].value.unwrap();
            // peel the 18 lambdas
            let mut binders = vec![];
            let mut cur = root;
            for _ in 0..18 {
                let CoreNode::Lambda { n, b: bi, t, v } = b.nodes[cur].clone() else { panic!("funSem_of has fewer than 18 binders") };
                binders.push((n, bi, t));
                cur = v;
            }
            let (head, args) = spine(&b.nodes, cur);
            assert!(matches!(&b.nodes[head], CoreNode::Const { n, .. } if n == "Exists.casesOn"), "not Exists.casesOn");
            assert_eq!(args.len(), 5);
            // depth 18: hst=0, ho=1, o=2, hF=3, hb=4, hw=5, hidx=6, F=7, env=8, args=9, fn=10, f=11, hcrate=12, ih=13, n=14, A=15, c=16, p=17
            let mk = |b: &mut B, name: &str| {
                let c = b.constant(name);
                let a: Vec<usize> = [17usize, 16, 15, 11].iter().map(|i| b.bvar(*i)).collect();
                b.app(c, &a)
            };
            let a_ty = mk(&mut b, FUNOK0);
            let b_ty = mk(&mut b, FUNLOOP);
            let or = b.constant("Or");
            let or_ab = b.app(or, &[a_ty, b_ty]);
            // motive: fun _ : Or a b => G   (G is the old motive's body)
            let CoreNode::Lambda { v: goal, .. } = b.nodes[args[2]].clone() else { panic!("motive is not a lambda") };
            let motive = b.lam("x", or_ab, goal);
            // inl: fun h0 : FunOK0 .. => (Exists.casesOn .. h0 ..) shifted
            let mut memo = BTreeMap::new();
            let mut shifted_args = vec![];
            for (k, x) in args.iter().enumerate() {
                if k == 3 {
                    shifted_args.push(b.bvar(0));
                } else if k == 2 {
                    // the old motive ranges over the proof of the unfolded
                    // existential, which is now FunOK0's body, not FunOK's
                    let CoreNode::Lambda { n, b: bi, v, .. } = b.nodes[*x].clone() else { panic!("inner motive is not a lambda") };
                    let dom = mk(&mut b, FUNOK0);
                    let dom = b.shift(dom, 0, &mut memo);
                    let v2 = b.shift(v, 1, &mut memo);
                    let lam = b.push(CoreNode::Lambda { n, b: bi, t: dom, v: v2 });
                    shifted_args.push(lam);
                } else {
                    shifted_args.push(b.shift(*x, 0, &mut memo));
                }
            }
            let shifted_head = b.shift(head, 0, &mut memo);
            let inner = b.app(shifted_head, &shifted_args);
            let a_ty2 = mk(&mut b, FUNOK0);
            let inl = b.lam("h0", a_ty2, inner);
            // inr: fun hl : FunLoop .. => funSem_loop p c A n f ih hl fn args env F hidx hw hb hF o ho hst
            let b_ty2 = mk(&mut b, FUNLOOP);
            let fl = b.constant(FUNSEM_LOOP);
            let order = [18usize, 17, 16, 15, 12, 14, 0, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
            let args: Vec<usize> = order.iter().map(|i| b.bvar(*i)).collect();
            let call = b.app(fl, &args);
            let inr = b.lam("hl", b_ty2, call);
            let oc = b.constant("Or.casesOn");
            // the major of the old term, unshifted
            let major = {
                let old_args = spine(&b.nodes, cur).1;
                old_args[3]
            };
            let a3 = mk(&mut b, FUNOK0);
            let b3 = mk(&mut b, FUNLOOP);
            let new_body = b.app(oc, &[a3, b3, motive, major, inl, inr]);
            let mut result = new_body;
            for (n, bi, t) in binders.into_iter().rev() {
                result = b.push(CoreNode::Lambda { n, b: bi, t, v: result });
            }
            module.declarations[at].value = Some(result);
            let nodes = std::mem::take(&mut b.nodes);
            finish(&mut module, nodes, &text, target, located.body);
        }
        other => panic!("unknown mode {other}"),
    }
}
