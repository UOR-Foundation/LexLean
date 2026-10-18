use lexlean::ir::core::{CoreModule, CoreNode};
use std::collections::BTreeSet;
fn spine(m: &CoreModule, mut i: usize) -> (usize, Vec<usize>) {
    let mut args = vec![];
    while let CoreNode::App { f, x } = &m.nodes[i] { args.push(*x); i = *f; }
    args.reverse(); (i, args)
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let text = std::fs::read_to_string(&a[1]).unwrap();
    let m = CoreModule::from_source(&text, "1.2").unwrap();
    let target = &a[3];
    for row in &m.declarations {
        if row.name != a[2] && !row.name.ends_with(&format!(".{}", a[2])) { continue; }
        for (which, root) in [("type", row.r#type), ("value", row.value)] {
            let Some(root) = root else { continue };
            let mut seen = BTreeSet::new(); let mut work = vec![root]; let mut sites = vec![];
            while let Some(n) = work.pop() {
                if !seen.insert(n) { continue; }
                match &m.nodes[n] {
                    CoreNode::App { f, x } => {
                        let (h, args) = spine(&m, n);
                        if let CoreNode::Const { n: name, .. } = &m.nodes[h] { if name == target { sites.push((n, args.len())); } }
                        work.push(*f); work.push(*x);
                    }
                    CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => { work.push(*t); work.push(*v); }
                    CoreNode::Let { t, v, body, .. } => { work.push(*t); work.push(*v); work.push(*body); }
                    CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => work.push(*x),
                    _ => {}
                }
            }
            sites.sort();
            println!("{} {which}: {} sites {:?} ({} nodes)", row.name, sites.len(), sites, seen.len());
        }
    }
}
