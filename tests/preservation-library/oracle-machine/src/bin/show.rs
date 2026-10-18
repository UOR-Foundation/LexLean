use lexlean::ir::core::{CoreModule, CoreNode, CoreLevel};
fn lvl(l: &CoreLevel) -> String { format!("{l:?}") }
fn show(m: &CoreModule, i: usize, depth: usize, out: &mut String, budget: &mut usize) {
    if *budget == 0 { out.push_str("…"); return; }
    *budget -= 1;
    if depth == 0 { out.push_str(&format!("@{i}")); return; }
    match &m.nodes[i] {
        CoreNode::BVar { i } => out.push_str(&format!("#{i}")),
        CoreNode::Sort { l } => out.push_str(&format!("Sort({})", lvl(l))),
        CoreNode::Const { n, .. } => out.push_str(n.rsplit_once("LexLean").map(|(_, r)| r).unwrap_or(n)),
        CoreNode::App { f, x } => {
            // flatten spine
            let mut args = vec![*x]; let mut h = *f;
            while let CoreNode::App { f, x } = &m.nodes[h] { args.push(*x); h = *f; }
            args.reverse();
            out.push('('); show(m, h, depth - 1, out, budget);
            for a in args { out.push(' '); show(m, a, depth - 1, out, budget); }
            out.push(')');
        }
        CoreNode::Lambda { n, t, v, .. } => { out.push_str(&format!("(fun {n} : ")); show(m, *t, depth - 1, out, budget); out.push_str(" => "); show(m, *v, depth - 1, out, budget); out.push(')'); }
        CoreNode::Forall { n, t, v, .. } => { out.push_str(&format!("(Π {n} : ")); show(m, *t, depth - 1, out, budget); out.push_str(", "); show(m, *v, depth - 1, out, budget); out.push(')'); }
        CoreNode::Let { n, t, v, body, .. } => { out.push_str(&format!("(let {n} : ")); show(m, *t, depth - 1, out, budget); out.push_str(" := "); show(m, *v, depth - 1, out, budget); out.push_str("; "); show(m, *body, depth - 1, out, budget); out.push(')'); }
        CoreNode::Projection { n, i, x, .. } => { out.push_str(&format!("(proj {n} {i} ")); show(m, *x, depth - 1, out, budget); out.push(')'); }
        CoreNode::Mdata { x, .. } => show(m, *x, depth, out, budget),
        CoreNode::Nat { v } => out.push_str(v),
        CoreNode::String { .. } => out.push_str("\"…\""),
    }
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a[2] == "@arg" {
        let text = std::fs::read_to_string(&a[1]).unwrap();
        let m = CoreModule::from_source(&text, "1.2").unwrap();
        let mut i: usize = a[3].parse().unwrap();
        let mut args = vec![];
        while let CoreNode::App { f, x } = &m.nodes[i] { args.push(*x); i = *f; }
        args.reverse();
        let k: usize = a[4].parse().unwrap();
        let mut s = String::new(); let mut b = 4000;
        show(&m, args[k], a[5].parse().unwrap(), &mut s, &mut b);
        println!("{s}");
        return;
    }
    if a[2] == "@node" {
        let text = std::fs::read_to_string(&a[1]).unwrap();
        let m = CoreModule::from_source(&text, "1.2").unwrap();
        let mut s = String::new(); let mut b = 4000;
        show(&m, a[3].parse().unwrap(), a[4].parse().unwrap(), &mut s, &mut b);
        println!("{s}");
        return;
    }
    let text = std::fs::read_to_string(&a[1]).unwrap();
    let m = CoreModule::from_source(&text, "1.2").unwrap();
    let depth: usize = a[3].parse().unwrap();
    let which = a.get(4).map(String::as_str).unwrap_or("both");
    for row in &m.declarations {
        if row.name == a[2] || row.name.ends_with(&format!(".{}", a[2])) {
            println!("== {} kind {:?} levels {:?}", row.name, row.kind, row.levels);
            if which != "value" { let mut s = String::new(); let mut b = 4000; show(&m, row.r#type.unwrap(), depth, &mut s, &mut b); println!("TYPE: {s}"); }
            if which != "type" { if let Some(v) = row.value { let mut s = String::new(); let mut b = 6000; show(&m, v, depth, &mut s, &mut b); println!("VALUE: {s}"); } }
        }
    }
}
