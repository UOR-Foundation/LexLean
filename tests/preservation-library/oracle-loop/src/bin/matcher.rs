//! One-time: give the matcher `rc_litValue.match_1` an alternative for `Expr.loop`, the arm of its
//! `Expr.casesOn`, and its users (`rc_litValue._f`) the alternative they pass. The alternative and the arm for
//! `loop` are those for a donor constructor with the donor's one field replaced by `loop`'s two.
use lexlean::ir::core::{CoreBinderInfo, CoreMatcherAlt, CoreModule, CoreNode};
use lexlean::surface::core as notation;
use lexlean::surface::module::locate;
use std::collections::{BTreeMap, BTreeSet};

const MATCH: &str = "LexLeanPreservation.Rust.rc_litValue.match_1";
const CASES: &str = "LexLeanTarget.RustSyntax.Expr.casesOn";
const PREFIX: &str = "LexLeanTarget.RustSyntax.Expr.";
const ORDER: [&str; 21] = ["lit","move","clone","copy","deref","not","unbox","box","call","apply","construct","pair","cond","matchOn","block","uncons","isZero","nonZero","predecessor","widen","succeed"];

struct T { nodes: Vec<CoreNode>, donor: String, memo: BTreeMap<(usize, usize, usize), usize> }
impl T {
    fn push(&mut self, n: CoreNode) -> usize { self.nodes.push(n); self.nodes.len() - 1 }
    fn spine(&self, mut i: usize) -> (usize, Vec<usize>) { let mut a = vec![]; while let CoreNode::App { f, x } = &self.nodes[i] { a.push(*x); i = *f; } a.reverse(); (i, a) }
    fn constant(&mut self, n: &str) -> usize { self.push(CoreNode::Const { n: n.to_owned(), u: vec![] }) }
    /// shift every free variable of a term by `by`, and replace the donor's field by `loop`'s two (n = 1, m = 2) when `fields`.
    fn go(&mut self, i: usize, depth: usize, extra: usize, fields: bool) -> Result<usize, String> {
        let key = (i, depth, extra * 2 + usize::from(fields));
        if let Some(d) = self.memo.get(&key) { return Ok(*d); }
        let (n, m) = if fields { (1, 2) } else { (0, 0) };
        let out = match self.nodes[i].clone() {
            CoreNode::BVar { i: k } => {
                if k < depth { self.push(CoreNode::BVar { i: k }) }
                else if fields && k == depth { return Err(format!("the donor's field is used outside its constructor (depth {depth})")); }
                else { self.push(CoreNode::BVar { i: k - n + m + extra }) }
            }
            CoreNode::App { f, x } => {
                let (head, args) = self.spine(i);
                if fields && matches!(&self.nodes[head], CoreNode::Const { n, .. } if *n == format!("{PREFIX}{}", self.donor)) && args.len() == 1 {
                    if !matches!(&self.nodes[args[0]], CoreNode::BVar { i: k } if *k == depth) { return Err("the donor is applied to something other than its field".into()); }
                    let lp = self.constant(&format!("{PREFIX}loop"));
                    let s = self.push(CoreNode::BVar { i: depth + 1 });
                    let f1 = self.push(CoreNode::BVar { i: depth });
                    let a1 = self.push(CoreNode::App { f: lp, x: s });
                    self.push(CoreNode::App { f: a1, x: f1 })
                } else {
                    let (nf, nx) = (self.go(f, depth, extra, fields)?, self.go(x, depth, extra, fields)?);
                    self.push(CoreNode::App { f: nf, x: nx })
                }
            }
            CoreNode::Lambda { n: name, b, t, v } => { let (nt, nv) = (self.go(t, depth, extra, fields)?, self.go(v, depth + 1, extra, fields)?); self.push(CoreNode::Lambda { n: name, b, t: nt, v: nv }) }
            CoreNode::Forall { n: name, b, t, v } => { let (nt, nv) = (self.go(t, depth, extra, fields)?, self.go(v, depth + 1, extra, fields)?); self.push(CoreNode::Forall { n: name, b, t: nt, v: nv }) }
            CoreNode::Let { n: name, t, v, body, d } => { let (nt, nv, nb) = (self.go(t, depth, extra, fields)?, self.go(v, depth, extra, fields)?, self.go(body, depth + 1, extra, fields)?); self.push(CoreNode::Let { n: name, t: nt, v: nv, body: nb, d }) }
            CoreNode::Projection { n: name, s, i: idx, x } => { let nx = self.go(x, depth, extra, fields)?; self.push(CoreNode::Projection { n: name, s, i: idx, x: nx }) }
            CoreNode::Mdata { m: entries, x } => { let nx = self.go(x, depth, extra, fields)?; self.push(CoreNode::Mdata { m: entries, x: nx }) }
            other => self.push(other),
        };
        self.memo.insert(key, out);
        Ok(out)
    }
    fn lam(&mut self, name: &str, t: usize, v: usize) -> usize { self.push(CoreNode::Lambda { n: name.to_owned(), b: CoreBinderInfo::Explicit, t, v }) }
    fn pi(&mut self, name: &str, t: usize, v: usize) -> usize { self.push(CoreNode::Forall { n: name.to_owned(), b: CoreBinderInfo::Explicit, t, v }) }
}

fn kids(n: &CoreNode) -> Vec<usize> { match n { CoreNode::App { f, x } => vec![*f, *x], CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => vec![*t, *v], CoreNode::Let { t, v, body, .. } => vec![*t, *v, *body], CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => vec![*x], _ => vec![] } }

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let path = &a[1];
    let donor = a[2].clone();
    let text = std::fs::read_to_string(path).unwrap();
    let located = locate(&text).unwrap();
    let body = &text[located.body.0..located.body.1];
    let mut module = CoreModule::from_source(&text, "1.2").unwrap();
    let mut t = T { nodes: module.nodes.clone(), donor: donor.clone(), memo: BTreeMap::new() };
    let alts = ORDER.len();
    // ---- the matcher
    let row_at = module.declarations.iter().position(|r| r.name == MATCH).expect("the matcher");
    let (ty, val) = (module.declarations[row_at].r#type.unwrap(), module.declarations[row_at].value.unwrap());
    // type: Π motive r v h h_1 … h_k. R
    let mut binders = vec![]; let mut cur = ty;
    for _ in 0..4 + alts { let CoreNode::Forall { n, t: bt, v, .. } = t.nodes[cur].clone() else { panic!("the matcher's type has fewer binders") }; binders.push((n, bt)); cur = v; }
    let result = cur;
    let donor_index = ORDER.iter().position(|c| *c == donor).unwrap();
    let donor_alt_type = binders[4 + donor_index].1; // in a context of 4 + donor_index binders
    // the donor alternative's type sits `alts - 1 - donor_index` binders before the end of the telescope: its new home has 4 + alts binders
    let moved = (4 + alts) - (4 + donor_index);
    let CoreNode::Forall { v: after_field, .. } = t.nodes[donor_alt_type].clone() else { panic!("the donor alternative is not a function") };
    let inner = t.go(after_field, 0, moved, true).unwrap_or_else(|e| panic!("{e}"));
    let ident = t.constant("LexLeanTarget.RustSyntax.Ident");
    let block = t.constant("LexLeanTarget.RustSyntax.Block");
    let p2 = t.pi("body", block, inner);
    let new_alt_type = t.pi("state", ident, p2);
    let new_result = t.go(result, 0, 1, false).unwrap();
    let mut new_ty = t.pi("h_new", new_alt_type, new_result);
    for (n, bt) in binders.iter().rev() { new_ty = t.pi(n, *bt, new_ty); }
    // value: fun motive r v h h_1 … h_k => Expr.casesOn …
    let mut lams = vec![]; let mut cur = val;
    for _ in 0..4 + alts { let CoreNode::Lambda { n, t: bt, v, .. } = t.nodes[cur].clone() else { panic!("the matcher's value has fewer binders") }; lams.push((n, bt)); cur = v; }
    let cases = cur;
    let (head, args) = t.spine(cases);
    assert!(matches!(&t.nodes[head], CoreNode::Const { n, .. } if n == CASES) && args.len() >= 2 + alts, "the value is an Expr.casesOn");
    let mut new_args = vec![];
    for (position, arg) in args.iter().enumerate() { let shifted = t.go(*arg, 0, 1, false).unwrap(); new_args.push((position, shifted)); }
    // the arm for loop from the donor's arm (args[2 + donor]): fun x (hh : …) => h_donor x v hh
    let donor_arm = args[2 + donor_index];
    let CoreNode::Lambda { v: after_x, .. } = t.nodes[donor_arm].clone() else { panic!("donor arm") };
    let CoreNode::Lambda { t: hh_type, v: call, .. } = t.nodes[after_x].clone() else { panic!("donor arm hypothesis") };
    let new_hh_type = t.go(hh_type, 0, 1, true).unwrap_or_else(|e| panic!("{e}"));
    // the call `h_donor x v hh` under (x, hh): head index k, args [x=#1, v, hh=#0]
    let (call_head, call_args) = t.spine(call);
    let CoreNode::BVar { i: head_k } = t.nodes[call_head].clone() else { panic!("the donor arm does not call an alternative") };
    assert_eq!(call_args.len(), 3, "the donor arm's call has the field, the value, and the hypothesis");
    let CoreNode::BVar { i: v_k } = t.nodes[call_args[1]].clone() else { panic!("the value is a variable") };
    // outer-relative positions at depth 2: the alternative sits at head_k - 2 from the end of the telescope; the new alternative is the innermost
    let alt_outer = head_k - 2;
    let expected = alts - 1 - donor_index;
    assert_eq!(alt_outer, expected, "the donor alternative is where the telescope says");
    let v_outer = v_k - 2;
    // new arm: fun state body hh => h_new state body v hh, depth 3 inside; h_new is the innermost lambda: outer position 0
    let mk = |t: &mut T, k: usize| t.push(CoreNode::BVar { i: k });
    let (h_new, st, bd, vv, hh) = (mk(&mut t, 3), mk(&mut t, 2), mk(&mut t, 1), mk(&mut t, v_outer + 1 + 3), mk(&mut t, 0));
    let mut c = h_new; for x in [st, bd, vv, hh] { c = t.push(CoreNode::App { f: c, x }); }
    let l3 = t.lam("hh", new_hh_type, c);
    let l2 = t.lam("body", block, l3);
    let loop_arm = t.lam("state", ident, l2);
    let mut cases_node = new_args[0].1; // placeholder to keep names; rebuilt below
    let casesc = { let CoreNode::Const { n, u } = t.nodes[head].clone() else { unreachable!() }; t.push(CoreNode::Const { n, u }) };
    cases_node = casesc;
    for (position, shifted) in &new_args {
        if *position == 2 + alts { cases_node = t.push(CoreNode::App { f: cases_node, x: loop_arm }); }
        cases_node = t.push(CoreNode::App { f: cases_node, x: *shifted });
    }
    // the telescope grows by h_new, whose type is `new_alt_type` shifted into place (it is already in its context)
    let mut new_val = t.lam("h_new", new_alt_type, cases_node);
    for (n, bt) in lams.iter().rev() { new_val = t.lam(n, *bt, new_val); }
    module.declarations[row_at].r#type = Some(new_ty);
    module.declarations[row_at].value = Some(new_val);
    if let Some(mt) = module.declarations[row_at].matcher.as_mut() { mt.alts.push(CoreMatcherAlt { fields: 2, overlaps: 0, unit_thunk: false }); }
    // ---- the users: every application of the matcher gets the alternative for loop
    let mut memo: BTreeMap<usize, usize> = BTreeMap::new();
    let mut applications = 0;
    let names: Vec<usize> = module.declarations.iter().enumerate().filter(|(_, r)| r.name != MATCH && !r.derived).map(|(i, _)| i).collect();
    for ri in names {
        let Some(root) = module.declarations[ri].value else { continue };
        let before = applications;
        let nr = users(&mut t, root, &mut memo, &mut applications, alts, donor_index).unwrap_or_else(|e| panic!("{}: {e}", module.declarations[ri].name));
        if applications != before { println!("{}: {} application(s) of the matcher", module.declarations[ri].name, applications - before); module.declarations[ri].value = Some(nr); }
    }
    module.nodes = t.nodes;
    // ---- prune, renumber, write
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
    std::fs::write(path, format!("{}{}{}{}{}", &text[..located.body.0], lead, printed.trim_matches('\n'), trail, &text[located.body.1..])).unwrap();
    println!("nodes -> {}", module.nodes.len());
}

/// Copy a term, giving each application of the matcher the alternative for `loop`, made from the donor's.
fn users(t: &mut T, i: usize, memo: &mut BTreeMap<usize, usize>, count: &mut usize, alts: usize, donor_index: usize) -> Result<usize, String> {
    if let Some(d) = memo.get(&i) { return Ok(*d); }
    let out = match t.nodes[i].clone() {
        CoreNode::App { f, x } => {
            let (head, args) = t.spine(i);
            if matches!(&t.nodes[head], CoreNode::Const { n, .. } if n == MATCH) && args.len() == 4 + alts {
                *count += 1;
                let rewritten: Vec<usize> = args.iter().map(|a| users(t, *a, memo, count, alts, donor_index)).collect::<Result<_, _>>()?;
                let donor_alt = rewritten[4 + donor_index];
                let CoreNode::Lambda { v: after_field, .. } = t.nodes[donor_alt].clone() else { return Err("the donor alternative is not a lambda".into()) };
                let inner = t.go(after_field, 0, 0, true)?;
                let ident = t.constant("LexLeanTarget.RustSyntax.Ident");
                let block = t.constant("LexLeanTarget.RustSyntax.Block");
                let l2 = t.lam("body", block, inner);
                let alt = t.lam("state", ident, l2);
                let mut node = users(t, head, memo, count, alts, donor_index)?;
                for a in rewritten.iter().copied().chain([alt]) { node = t.push(CoreNode::App { f: node, x: a }); }
                node
            } else {
                let (nf, nx) = (users(t, f, memo, count, alts, donor_index)?, users(t, x, memo, count, alts, donor_index)?);
                t.push(CoreNode::App { f: nf, x: nx })
            }
        }
        CoreNode::Lambda { n, b, t: bt, v } => { let (nt, nv) = (users(t, bt, memo, count, alts, donor_index)?, users(t, v, memo, count, alts, donor_index)?); t.push(CoreNode::Lambda { n, b, t: nt, v: nv }) }
        CoreNode::Forall { n, b, t: bt, v } => { let (nt, nv) = (users(t, bt, memo, count, alts, donor_index)?, users(t, v, memo, count, alts, donor_index)?); t.push(CoreNode::Forall { n, b, t: nt, v: nv }) }
        CoreNode::Let { n, t: bt, v, body, d } => { let (nt, nv, nb) = (users(t, bt, memo, count, alts, donor_index)?, users(t, v, memo, count, alts, donor_index)?, users(t, body, memo, count, alts, donor_index)?); t.push(CoreNode::Let { n, t: nt, v: nv, body: nb, d }) }
        CoreNode::Projection { n, s, i: idx, x } => { let nx = users(t, x, memo, count, alts, donor_index)?; t.push(CoreNode::Projection { n, s, i: idx, x: nx }) }
        CoreNode::Mdata { m, x } => { let nx = users(t, x, memo, count, alts, donor_index)?; t.push(CoreNode::Mdata { m, x: nx }) }
        other => t.push(other),
    };
    memo.insert(i, out);
    Ok(out)
}
