//! One-time: graft the declarations of an exported core module (authored in Lean, exported with
//! the exporter of checkpoint `d54dbf85`) into a library module, before an anchor declaration.
//! `graft <target.lex.tex> <exported.lex.tex> <anchor declaration>`
use lexlean::ir::core::{CoreModule, CoreNode};
use lexlean::surface::core as notation;
use lexlean::surface::module::locate;
use std::collections::BTreeSet;

fn kids(n: &CoreNode) -> Vec<usize> {
    match n {
        CoreNode::App { f, x } => vec![*f, *x],
        CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => vec![*t, *v],
        CoreNode::Let { t, v, body, .. } => vec![*t, *v, *body],
        CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => vec![*x],
        _ => vec![],
    }
}

fn remap(node: &mut CoreNode, map: &[usize]) {
    match node {
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
}

fn roots(row: &lexlean::ir::core::CoreDeclaration) -> Vec<usize> {
    let mut work: Vec<usize> = row.expression_roots().collect();
    if let Some(info) = &row.eqn_info {
        work.extend([info.r#type, info.value]);
    }
    if let Some(st) = row.inductive.as_ref().and_then(|i| i.structure.as_ref()) {
        work.extend(st.fields.iter().filter_map(|f| f.auto_param));
    }
    work
}

fn remap_row(row: &mut lexlean::ir::core::CoreDeclaration, map: &[usize]) {
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

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (target, source, anchor) = (&a[1], &a[2], &a[3]);
    let text = std::fs::read_to_string(target).unwrap();
    let located = locate(&text).unwrap();
    let body = &text[located.body.0..located.body.1];
    let mut module = CoreModule::from_source(&text, "1.2").unwrap();
    // Private auxiliary names embed the module they were elaborated in; the
    // rows now live in the host module, so the prefix must name the host.
    let stem = |p: &str| std::path::Path::new(p).file_name().unwrap().to_string_lossy().split('.').next().unwrap().to_string();
    let source_text = std::fs::read_to_string(source).unwrap().replace(
        &format!("_private.LexLeanPreservation.{}.#0", stem(source)),
        &format!("_private.LexLeanPreservation.{}.#0", stem(target)),
    );
    let src = CoreModule::from_source(&source_text, "1.2").unwrap();
    let existing: BTreeSet<&str> = module.declarations.iter().map(|r| r.name.as_str()).collect();
    let fresh: Vec<_> = src
        .declarations
        .iter()
        .filter(|r| !existing.contains(r.name.as_str()))
        .cloned()
        .collect();
    println!("{} of {} source declarations are new", fresh.len(), src.declarations.len());
    let mut needed = vec![false; src.nodes.len()];
    let mut work: Vec<usize> = fresh.iter().flat_map(roots).collect();
    while let Some(n) = work.pop() {
        if needed[n] {
            continue;
        }
        needed[n] = true;
        work.extend(kids(&src.nodes[n]));
    }
    let mut map = vec![usize::MAX; src.nodes.len()];
    let mut nodes = module.nodes.clone();
    for (i, node) in src.nodes.iter().enumerate() {
        if needed[i] {
            let mut node = node.clone();
            remap(&mut node, &map);
            map[i] = nodes.len();
            nodes.push(node);
        }
    }
    for p in &src.proof_nodes {
        if needed[*p] {
            module.proof_nodes.push(map[*p]);
        }
    }
    module.proof_nodes.sort();
    module.proof_nodes.dedup();
    let at = module
        .declarations
        .iter()
        .position(|r| r.name == *anchor)
        .unwrap_or_else(|| panic!("no declaration `{anchor}`"));
    for (k, mut row) in fresh.into_iter().enumerate() {
        remap_row(&mut row, &map);
        module.declarations.insert(at + k, row);
    }
    module.nodes = nodes;
    module.validate("1.2").unwrap_or_else(|e| panic!("{e}"));
    let printed = notation::print_module(&module).unwrap();
    assert_eq!(notation::parse_module(&printed).unwrap(), module);
    let lead = &text[located.body.0..located.body.0 + (body.len() - body.trim_start_matches('\n').len())];
    let trail_len = body.len() - body.trim_end_matches('\n').len();
    let trail = &body[body.len() - trail_len..];
    std::fs::write(
        target,
        format!("{}{}{}{}{}", &text[..located.body.0], lead, printed.trim_matches('\n'), trail, &text[located.body.1..]),
    )
    .unwrap();
    println!("nodes -> {}", module.nodes.len());
}
