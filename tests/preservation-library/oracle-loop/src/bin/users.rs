use lexlean::ir::core::{CoreModule, CoreNode};
use std::collections::BTreeSet;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let text = std::fs::read_to_string(&a[1]).unwrap();
    let m = CoreModule::from_source(&text, "1.2").unwrap();
    let target = &a[2];
    // which nodes mention the constant (transitively)
    let mut mentions = vec![false; m.nodes.len()];
    for (i, n) in m.nodes.iter().enumerate() {
        mentions[i] = match n {
            CoreNode::Const { n, .. } => n == target,
            CoreNode::App { f, x } => mentions[*f] || mentions[*x],
            CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => mentions[*t] || mentions[*v],
            CoreNode::Let { t, v, body, .. } => mentions[*t] || mentions[*v] || mentions[*body],
            CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => mentions[*x],
            _ => false,
        };
    }
    for row in &m.declarations {
        let hit = row.expression_roots().any(|r| mentions[r]) || row.eqn_info.as_ref().is_some_and(|e| mentions[e.r#type] || mentions[e.value]);
        if hit { println!("{} {:?} derived={}", row.name, row.kind, row.derived); }
    }
    let _: BTreeSet<usize> = BTreeSet::new();
}
