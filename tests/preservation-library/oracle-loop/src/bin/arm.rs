//! One-time repair of authored declarations that a new constructor of `Expr` leaves one arm short:
//! the arm for the new constructor is a donor constructor's arm with the donor's fields replaced by
//! the new constructor's, provided the donor's proof uses its fields only through the constructor.
use lexlean::ir::core::{CoreBinderInfo, CoreModule, CoreNode};
use lexlean::surface::core as notation;
use lexlean::surface::module::locate;
use std::collections::{BTreeMap, BTreeSet};

const CASES: &str = "LexLeanTarget.RustSyntax.Expr.casesOn";
const PREFIX: &str = "LexLeanTarget.RustSyntax.Expr.";
const ORDER: [&str; 21] = ["lit","move","clone","copy","deref","not","unbox","box","call","apply","construct","pair","cond","matchOn","block","uncons","isZero","nonZero","predecessor","widen","succeed"];
const NEW: &str = "loop";
const NEW_FIELDS: [&str; 2] = ["LexLeanTarget.RustSyntax.Ident", "LexLeanTarget.RustSyntax.Block"];

struct Arm<'a> {
    old: &'a [CoreNode],
    nodes: Vec<CoreNode>,
    memo: BTreeMap<usize, usize>,
    flagged: BTreeSet<usize>,
    proofs: &'a BTreeSet<usize>,
    donor: usize,
    donor_fields: usize,
    sites: usize,
    // memo of the donor transformation, keyed by (new node, depth)
    shift: BTreeMap<(usize, usize), usize>,
}

impl Arm<'_> {
    fn push(&mut self, n: CoreNode) -> usize { self.nodes.push(n); self.nodes.len() - 1 }
    fn spine(&self, mut i: usize) -> (usize, Vec<usize>) {
        let mut args = vec![];
        while let CoreNode::App { f, x } = &self.old[i] { args.push(*x); i = *f; }
        args.reverse(); (i, args)
    }
    fn spine_new(&self, mut i: usize) -> (usize, Vec<usize>) {
        let mut args = vec![];
        while let CoreNode::App { f, x } = &self.nodes[i] { args.push(*x); i = *f; }
        args.reverse(); (i, args)
    }
    fn is_const_new(&self, i: usize, name: &str) -> bool { matches!(&self.nodes[i], CoreNode::Const { n, .. } if n == name) }
    /// The body of the donor arm under its `n` field binders, with the donor constructor replaced by the new one.
    fn transform(&mut self, i: usize, depth: usize) -> Result<usize, String> {
        if let Some(done) = self.shift.get(&(i, depth)) { return Ok(*done); }
        let n = self.donor_fields; let m = NEW_FIELDS.len();
        let node = self.nodes[i].clone();
        let out = match node {
            CoreNode::BVar { i: k } => {
                if k < depth { self.push(CoreNode::BVar { i: k }) }
                else if k < depth + n { return Err(format!("a field of the donor is used outside its constructor (index {k} at depth {depth})")); }
                else { self.push(CoreNode::BVar { i: k - n + m }) }
            }
            CoreNode::App { f, x } => {
                let (head, args) = self.spine_new(i);
                let donor_name = format!("{PREFIX}{}", ORDER[self.donor]);
                if self.is_const_new(head, &donor_name) && args.len() == n {
                    // the spine must be the fields in order
                    for (position, a) in args.iter().enumerate() {
                        if !matches!(&self.nodes[*a], CoreNode::BVar { i: k } if *k == depth + n - 1 - position) { return Err("the donor constructor is applied to something other than its fields".to_owned()); }
                    }
                    let mut node = self.push(CoreNode::Const { n: format!("{PREFIX}{NEW}"), u: vec![] });
                    for position in 0..m {
                        let field = self.push(CoreNode::BVar { i: depth + m - 1 - position });
                        node = self.push(CoreNode::App { f: node, x: field });
                    }
                    node
                } else {
                    let (nf, nx) = (self.transform(f, depth)?, self.transform(x, depth)?);
                    self.push(CoreNode::App { f: nf, x: nx })
                }
            }
            CoreNode::Lambda { n: name, b, t, v } => { let (nt, nv) = (self.transform(t, depth)?, self.transform(v, depth + 1)?); self.push(CoreNode::Lambda { n: name, b, t: nt, v: nv }) }
            CoreNode::Forall { n: name, b, t, v } => { let (nt, nv) = (self.transform(t, depth)?, self.transform(v, depth + 1)?); self.push(CoreNode::Forall { n: name, b, t: nt, v: nv }) }
            CoreNode::Let { n: name, t, v, body, d } => { let (nt, nv, nb) = (self.transform(t, depth)?, self.transform(v, depth)?, self.transform(body, depth + 1)?); self.push(CoreNode::Let { n: name, t: nt, v: nv, body: nb, d }) }
            CoreNode::Projection { n: name, s, i: idx, x } => { let nx = self.transform(x, depth)?; self.push(CoreNode::Projection { n: name, s, i: idx, x: nx }) }
            CoreNode::Mdata { m: entries, x } => { let nx = self.transform(x, depth)?; self.push(CoreNode::Mdata { m: entries, x: nx }) }
            other => self.push(other),
        };
        self.shift.insert((i, depth), out);
        Ok(out)
    }
    fn rw(&mut self, i: usize) -> Result<usize, String> {
        if let Some(done) = self.memo.get(&i) { return Ok(*done); }
        let out = match self.old[i].clone() {
            CoreNode::App { f, x } => {
                let (head, args) = self.spine(i);
                if matches!(&self.old[head], CoreNode::Const { n, .. } if n == CASES) && args.len() >= 2 + ORDER.len() {
                    self.sites += 1;
                    let rewritten: Vec<usize> = args.iter().map(|a| self.rw(*a)).collect::<Result<_, _>>()?;
                    let h = self.rw(head)?;
                    // the donor arm: fun (fields) => body
                    let mut arm = rewritten[2 + self.donor];
                    let mut types = vec![];
                    let mut binder = None;
                    for _ in 0..self.donor_fields {
                        let CoreNode::Lambda { n, b, t, v } = self.nodes[arm].clone() else { return Err("the donor arm binds fewer fields than the constructor has".to_owned()) };
                        types.push(t); binder = Some((n, b)); arm = v;
                    }
                    let _ = (types, binder);
                    let body = self.transform(arm, 0)?;
                    let mut new_arm = body;
                    for (position, field) in NEW_FIELDS.iter().enumerate().rev() {
                        let t = self.push(CoreNode::Const { n: (*field).to_owned(), u: vec![] });
                        new_arm = self.push(CoreNode::Lambda { n: if position == 0 { "state".to_owned() } else { "body".to_owned() }, b: CoreBinderInfo::Explicit, t, v: new_arm });
                    }
                    let mut node = h;
                    for (position, a) in rewritten.iter().copied().enumerate() {
                        if position == 2 + ORDER.len() { node = self.push(CoreNode::App { f: node, x: new_arm }); }
                        node = self.push(CoreNode::App { f: node, x: a });
                    }
                    node
                } else {
                    let (nf, nx) = (self.rw(f)?, self.rw(x)?);
                    self.push(CoreNode::App { f: nf, x: nx })
                }
            }
            CoreNode::Lambda { n, b, t, v } => { let (nt, nv) = (self.rw(t)?, self.rw(v)?); self.push(CoreNode::Lambda { n, b, t: nt, v: nv }) }
            CoreNode::Forall { n, b, t, v } => { let (nt, nv) = (self.rw(t)?, self.rw(v)?); self.push(CoreNode::Forall { n, b, t: nt, v: nv }) }
            CoreNode::Let { n, t, v, body, d } => { let (nt, nv, nb) = (self.rw(t)?, self.rw(v)?, self.rw(body)?); self.push(CoreNode::Let { n, t: nt, v: nv, body: nb, d }) }
            CoreNode::Projection { n, s, i: idx, x } => { let nx = self.rw(x)?; self.push(CoreNode::Projection { n, s, i: idx, x: nx }) }
            CoreNode::Mdata { m, x } => { let nx = self.rw(x)?; self.push(CoreNode::Mdata { m, x: nx }) }
            other => self.push(other),
        };
        if self.proofs.contains(&i) { self.flagged.insert(out); }
        self.memo.insert(i, out);
        Ok(out)
    }
}

fn kids(n: &CoreNode) -> Vec<usize> { match n { CoreNode::App { f, x } => vec![*f, *x], CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => vec![*t, *v], CoreNode::Let { t, v, body, .. } => vec![*t, *v, *body], CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => vec![*x], _ => vec![] } }

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let path = &a[1];
    // arguments: <module> <donor>:<declaration> ...
    let text = std::fs::read_to_string(path).unwrap();
    let located = locate(&text).unwrap();
    let body = &text[located.body.0..located.body.1];
    let mut module = CoreModule::from_source(&text, "1.2").unwrap();
    let proofs: BTreeSet<usize> = module.proof_nodes.iter().copied().collect();
    let old = module.nodes.clone();
    let mut nodes = module.nodes.clone();
    let mut flagged = BTreeSet::new();
    for spec in &a[2..] {
        let (donor_name, declaration) = spec.split_once(':').expect("donor:declaration");
        let donor = ORDER.iter().position(|c| *c == donor_name).expect("a donor constructor");
        let fields = match donor_name { "lit" | "move" | "clone" | "copy" | "deref" | "not" | "unbox" | "box" | "uncons" | "isZero" | "nonZero" | "predecessor" | "widen" | "succeed" | "block" => 1, "pair" | "matchOn" => 2, "call" | "apply" | "construct" | "cond" => 3, _ => unreachable!() };
        let row = module.declarations.iter_mut().find(|r| r.name.ends_with(&format!(".{declaration}"))).unwrap_or_else(|| panic!("no declaration {declaration}"));
        let mut arm = Arm { old: &old, nodes: std::mem::take(&mut nodes), memo: BTreeMap::new(), flagged: std::mem::take(&mut flagged), proofs: &proofs, donor, donor_fields: fields, sites: 0, shift: BTreeMap::new() };
        let root = row.value.expect("a theorem or definition");
        match arm.rw(root) {
            Ok(new_root) => { row.value = Some(new_root); println!("{}: donor {donor_name}, {} Expr.casesOn sites", row.name, arm.sites); }
            Err(e) => { println!("{}: donor {donor_name} REFUSED: {e}", row.name); }
        }
        nodes = arm.nodes; flagged = arm.flagged;
    }
    module.nodes = nodes;
    module.proof_nodes = module.proof_nodes.iter().copied().chain(flagged).collect::<BTreeSet<_>>().into_iter().collect();
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
