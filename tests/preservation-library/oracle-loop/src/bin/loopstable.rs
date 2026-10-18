//! One-time: graft the lemma `stable_loop` (authored in Lean, exported) into `RustBase` and give `stable` the arm for `Expr.loop`.
use lexlean::ir::core::{CoreBinderInfo, CoreDeclaration, CoreModule, CoreNode};
use lexlean::surface::core as notation;
use lexlean::surface::module::locate;
use std::collections::{BTreeMap, BTreeSet};

const CASES: &str = "LexLeanTarget.RustSyntax.Expr.casesOn";
const PREFIX: &str = "LexLeanTarget.RustSyntax.Expr.";
const STABLE_LOOP: &str = "LexLeanPreservation.Rust.stable_loop";
const ORDER: [&str; 21] = ["lit","move","clone","copy","deref","not","unbox","box","call","apply","construct","pair","cond","matchOn","block","uncons","isZero","nonZero","predecessor","widen","succeed"];

struct B { nodes: Vec<CoreNode> }
impl B {
    fn push(&mut self, n: CoreNode) -> usize { self.nodes.push(n); self.nodes.len() - 1 }
    fn spine(&self, mut i: usize) -> (usize, Vec<usize>) { let mut a = vec![]; while let CoreNode::App { f, x } = &self.nodes[i] { a.push(*x); i = *f; } a.reverse(); (i, a) }
}

fn kids(n: &CoreNode) -> Vec<usize> { match n { CoreNode::App { f, x } => vec![*f, *x], CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => vec![*t, *v], CoreNode::Let { t, v, body, .. } => vec![*t, *v, *body], CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => vec![*x], _ => vec![] } }

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (target, source) = (&a[1], &a[2]);
    // candidate bvar indices at the body of the inner proof of the block arm, found by trial
    let ihe: usize = a[3].parse().unwrap();
    let text = std::fs::read_to_string(target).unwrap();
    let located = locate(&text).unwrap();
    let body = &text[located.body.0..located.body.1];
    let mut module = CoreModule::from_source(&text, "1.2").unwrap();
    let src = CoreModule::from_source(&std::fs::read_to_string(source).unwrap(), "1.2").unwrap();
    let mut b = B { nodes: module.nodes.clone() };
    // 1. graft the nodes reachable from stable_loop
    let lemma = src.declarations.iter().find(|r| r.name == STABLE_LOOP).expect("stable_loop");
    let mut needed = vec![false; src.nodes.len()];
    let mut work: Vec<usize> = lemma.expression_roots().collect();
    while let Some(n) = work.pop() { if needed[n] { continue; } needed[n] = true; work.extend(kids(&src.nodes[n])); }
    let mut map = vec![usize::MAX; src.nodes.len()];
    for (i, node) in src.nodes.iter().enumerate() {
        if !needed[i] { continue; }
        let mut node = node.clone();
        match &mut node {
            CoreNode::App { f, x } => { *f = map[*f]; *x = map[*x]; }
            CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => { *t = map[*t]; *v = map[*v]; }
            CoreNode::Let { t, v, body, .. } => { *t = map[*t]; *v = map[*v]; *body = map[*body]; }
            CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => { *x = map[*x]; }
            _ => {}
        }
        map[i] = b.push(node);
    }
    let mut grafted = lemma.clone();
    grafted.r#type = grafted.r#type.map(|r| map[r]);
    grafted.value = grafted.value.map(|r| map[r]);
    let derived_src = src.declarations.iter().find(|r| r.name == "LexLeanTarget.RustSemantics.eval.eq_23").expect("eval.eq_23");
    let mut eq23: CoreDeclaration = derived_src.clone();
    eq23.derived = true; eq23.levels = None; eq23.r#type = None; eq23.value = None; eq23.reducibility = None; eq23.all = None;
    let at = module.declarations.iter().position(|r| r.name == "LexLeanPreservation.Rust.stable").expect("stable");
    module.declarations.insert(at, grafted);
    module.declarations.insert(at, eq23);
    // 2. the arm of `stable`
    let stable_at = module.declarations.iter().position(|r| r.name == "LexLeanPreservation.Rust.stable").unwrap();
    let root = module.declarations[stable_at].value.unwrap();
    let mut memo: BTreeMap<usize, usize> = BTreeMap::new();
    let mut sites = 0;
    let new_root = rewrite(&mut b, root, &mut memo, &mut sites, ihe).unwrap_or_else(|e| panic!("{e}"));
    module.declarations[stable_at].value = Some(new_root);
    println!("stable: {sites} Expr.casesOn sites");
    // 3. prune, renumber, write
    module.nodes = b.nodes;
    let proofs: BTreeSet<usize> = BTreeSet::new();
    let _ = proofs;
    let mut needed = vec![false; module.nodes.len()];
    let mut work: Vec<usize> = vec![];
    for row in &module.declarations {
        work.extend(row.expression_roots());
        if let Some(info) = &row.eqn_info { work.extend([info.r#type, info.value]); }
        if let Some(st) = row.inductive.as_ref().and_then(|i| i.structure.as_ref()) { work.extend(st.fields.iter().filter_map(|f| f.auto_param)); }
    }
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
    std::fs::write(target, format!("{}{}{}{}{}", &text[..located.body.0], lead, printed.trim_matches('\n'), trail, &text[located.body.1..])).unwrap();
    println!("nodes -> {}", module.nodes.len());
}

fn rewrite(b: &mut B, i: usize, memo: &mut BTreeMap<usize, usize>, sites: &mut usize, ihe: usize) -> Result<usize, String> {
    if let Some(d) = memo.get(&i) { return Ok(*d); }
    let out = match b.nodes[i].clone() {
        CoreNode::App { f, x } => {
            let (head, args) = b.spine(i);
            if matches!(&b.nodes[head], CoreNode::Const { n, .. } if n == CASES) && args.len() >= 2 + ORDER.len() && !matches!(&b.nodes[args[0]], CoreNode::BVar { .. } ) && *sites == 0 {
                *sites += 1;
                let rewritten: Vec<usize> = args.iter().map(|a| rewrite(b, *a, memo, sites, ihe)).collect::<Result<_, _>>()?;
                let arm = build_arm(b, rewritten[2 + 14], ihe)?;
                let mut node = rewrite(b, head, memo, sites, ihe)?;
                for (position, a) in rewritten.iter().copied().enumerate() {
                    if position == 2 + ORDER.len() { node = b.push(CoreNode::App { f: node, x: arm }); }
                    node = b.push(CoreNode::App { f: node, x: a });
                }
                node
            } else {
                let (nf, nx) = (rewrite(b, f, memo, sites, ihe)?, rewrite(b, x, memo, sites, ihe)?);
                b.push(CoreNode::App { f: nf, x: nx })
            }
        }
        CoreNode::Lambda { n, b: bi, t, v } => { let (nt, nv) = (rewrite(b, t, memo, sites, ihe)?, rewrite(b, v, memo, sites, ihe)?); b.push(CoreNode::Lambda { n, b: bi, t: nt, v: nv }) }
        CoreNode::Forall { n, b: bi, t, v } => { let (nt, nv) = (rewrite(b, t, memo, sites, ihe)?, rewrite(b, v, memo, sites, ihe)?); b.push(CoreNode::Forall { n, b: bi, t: nt, v: nv }) }
        CoreNode::Let { n, t, v, body, d } => { let (nt, nv, nb) = (rewrite(b, t, memo, sites, ihe)?, rewrite(b, v, memo, sites, ihe)?, rewrite(b, body, memo, sites, ihe)?); b.push(CoreNode::Let { n, t: nt, v: nv, body: nb, d }) }
        CoreNode::Projection { n, s, i: idx, x } => { let nx = rewrite(b, x, memo, sites, ihe)?; b.push(CoreNode::Projection { n, s, i: idx, x: nx }) }
        CoreNode::Mdata { m, x } => { let nx = rewrite(b, x, memo, sites, ihe)?; b.push(CoreNode::Mdata { m, x: nx }) }
        other => b.push(other),
    };
    memo.insert(i, out);
    Ok(out)
}

/// The arm for `loop` from the arm for `block`: `fun b hEq => Eq.ndrec _ (block b) M (fun h => P) e (Eq.symm _ _ _ hEq) h0`
/// becomes `fun s b hEq => Eq.ndrec _ (loop s b) M' (fun h => stable_loop n ihE ihB c env s b h) e (...) h0`.
fn build_arm(b: &mut B, block_arm: usize, ihe: usize) -> Result<usize, String> {
    let CoreNode::Lambda { v: after_field, .. } = b.nodes[block_arm].clone() else { return Err("the block arm is not a lambda".into()) };
    let CoreNode::Lambda { v: body, t: heq_type, n: heq_name, b: heq_binder } = b.nodes[after_field].clone() else { return Err("no equation hypothesis".into()) };
    let (head, args) = b.spine(body);
    if !matches!(&b.nodes[head], CoreNode::Const { n, .. } if n == "Eq.ndrec") { return Err("the block arm does not end in Eq.ndrec".into()); }
    let mut shift = Shift { b, memo: BTreeMap::new() };
    let new_heq_type = shift.map(heq_type, 0)?;
    // Eq.ndrec α a motive m e h  [extra…]
    let mut new_args = vec![];
    for (position, arg) in args.iter().enumerate() {
        let new = if position == 3 {
            // m = fun h => P, at depth 1 inside the equation hypothesis
            let CoreNode::Lambda { n: hn, b: hb, t: ht, .. } = shift.b.nodes[*arg].clone() else { return Err("the minor premise of Eq.ndrec is not a lambda".into()) };
            let new_ht = shift.map(ht, 1)?;
            // inside `fun h`: h=#0 hEq=#1 body=#2 state=#3; outer o -> o+1
            let mut c = |k: usize| shift.b.push(CoreNode::BVar { i: k });
            let (n_, ihe_, ihb_, c_, env_, s_, b_, h_) = (c(17), c(ihe), c(11), c(7), c(6), c(3), c(2), c(0));
            let lemma = shift.b.push(CoreNode::Const { n: STABLE_LOOP.to_owned(), u: vec![] });
            let mut t = lemma;
            for x in [n_, ihe_, ihb_, c_, env_, s_, b_, h_] { t = shift.b.push(CoreNode::App { f: t, x }); }
            shift.b.push(CoreNode::Lambda { n: hn, b: hb, t: new_ht, v: t })
        } else {
            shift.map(*arg, 1)?
        };
        new_args.push(new);
    }
    let mut node = shift.map(head, 1)?;
    for a in new_args { node = shift.b.push(CoreNode::App { f: node, x: a }); }
    let with_heq = shift.b.push(CoreNode::Lambda { n: heq_name, b: heq_binder, t: new_heq_type, v: node });
    let ident = shift.b.push(CoreNode::Const { n: "LexLeanTarget.RustSyntax.Ident".to_owned(), u: vec![] });
    let block = shift.b.push(CoreNode::Const { n: "LexLeanTarget.RustSyntax.Block".to_owned(), u: vec![] });
    let l2 = shift.b.push(CoreNode::Lambda { n: "body".to_owned(), b: CoreBinderInfo::Explicit, t: block, v: with_heq });
    Ok(shift.b.push(CoreNode::Lambda { n: "state".to_owned(), b: CoreBinderInfo::Explicit, t: ident, v: l2 }))
}

/// The block arm's terms under the loop arm: the spine `block f` (f the field) is `loop s f`, and a
/// variable of the context outside the arm moves one place; the field is used only in that spine.
struct Shift<'a> { b: &'a mut B, memo: BTreeMap<(usize, usize), usize> }
impl Shift<'_> {
    /// `depth`: binders entered since the arm's field binder, the equation hypothesis included.
    fn map(&mut self, i: usize, depth: usize) -> Result<usize, String> {
        if let Some(d) = self.memo.get(&(i, depth)) { return Ok(*d); }
        let out = match self.b.nodes[i].clone() {
            CoreNode::BVar { i: k } => {
                if k < depth { self.b.push(CoreNode::BVar { i: k }) }
                else if k == depth { return Err(format!("the field is used outside `block` at depth {depth}")); }
                else { self.b.push(CoreNode::BVar { i: k + 1 }) }
            }
            CoreNode::App { f, x } => {
                let (head, args) = self.b.spine(i);
                if matches!(&self.b.nodes[head], CoreNode::Const { n, .. } if *n == format!("{PREFIX}block")) && args.len() == 1 {
                    if !matches!(&self.b.nodes[args[0]], CoreNode::BVar { i: k } if *k == depth) { return Err("`block` is applied to something other than the field".into()); }
                    let lp = self.b.push(CoreNode::Const { n: format!("{PREFIX}loop"), u: vec![] });
                    let s = self.b.push(CoreNode::BVar { i: depth + 1 });
                    let f1 = self.b.push(CoreNode::BVar { i: depth });
                    let a1 = self.b.push(CoreNode::App { f: lp, x: s });
                    self.b.push(CoreNode::App { f: a1, x: f1 })
                } else {
                    let (nf, nx) = (self.map(f, depth)?, self.map(x, depth)?);
                    self.b.push(CoreNode::App { f: nf, x: nx })
                }
            }
            CoreNode::Lambda { n, b: bi, t, v } => { let (nt, nv) = (self.map(t, depth)?, self.map(v, depth + 1)?); self.b.push(CoreNode::Lambda { n, b: bi, t: nt, v: nv }) }
            CoreNode::Forall { n, b: bi, t, v } => { let (nt, nv) = (self.map(t, depth)?, self.map(v, depth + 1)?); self.b.push(CoreNode::Forall { n, b: bi, t: nt, v: nv }) }
            CoreNode::Let { n, t, v, body, d } => { let (nt, nv, nb) = (self.map(t, depth)?, self.map(v, depth)?, self.map(body, depth + 1)?); self.b.push(CoreNode::Let { n, t: nt, v: nv, body: nb, d }) }
            CoreNode::Projection { n, s, i: idx, x } => { let nx = self.map(x, depth)?; self.b.push(CoreNode::Projection { n, s, i: idx, x: nx }) }
            CoreNode::Mdata { m, x } => { let nx = self.map(x, depth)?; self.b.push(CoreNode::Mdata { m, x: nx }) }
            other => self.b.push(other),
        };
        self.memo.insert((i, depth), out);
        Ok(out)
    }
}
