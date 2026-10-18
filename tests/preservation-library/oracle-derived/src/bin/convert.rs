//! One-time conversion of the preservation library's reserved-name and sparse
//! casesOn rows to derived rows, pruning the nodes nothing else reaches.
use lexlean::ir::core::{CoreDeclaration, CoreModule, CoreNode};
use lexlean::surface::core as notation;
use lexlean::surface::module::locate;

fn migrating(row: &CoreDeclaration) -> bool {
    let last = row.name.rsplit('.').next().unwrap_or(&row.name);
    let reserved = last == "splitter"
        || last == "else_eq"
        || last == "eq_def"
        || (last.starts_with("eq_") && last.len() > 3 && last[3..].chars().all(|c| c.is_ascii_digit()));
    reserved || row.sparse.is_some()
}

fn remap(node: &CoreNode, m: &[usize]) -> CoreNode {
    let mut node = node.clone();
    match &mut node {
        CoreNode::App { f, x } => { *f = m[*f]; *x = m[*x]; }
        CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => { *t = m[*t]; *v = m[*v]; }
        CoreNode::Let { t, v, body, .. } => { *t = m[*t]; *v = m[*v]; *body = m[*body]; }
        CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => { *x = m[*x]; }
        _ => {}
    }
    node
}

fn children(node: &CoreNode) -> Vec<usize> {
    match node {
        CoreNode::App { f, x } => vec![*f, *x],
        CoreNode::Lambda { t, v, .. } | CoreNode::Forall { t, v, .. } => vec![*t, *v],
        CoreNode::Let { t, v, body, .. } => vec![*t, *v, *body],
        CoreNode::Projection { x, .. } | CoreNode::Mdata { x, .. } => vec![*x],
        _ => vec![],
    }
}

fn main() {
    let dir = std::env::args().nth(1).unwrap();
    let mut total = (0usize, 0usize, 0usize, 0usize);
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if !path.to_string_lossy().ends_with(".lex.tex") { continue; }
        let text = std::fs::read_to_string(&path).unwrap();
        let located = locate(&text).unwrap();
        let body = &text[located.body.0..located.body.1];
        let mut module = CoreModule::from_source(&text, "1.2").unwrap_or_else(|e| panic!("{path:?}: {e}"));
        // the canonical printing of the unconverted module is the body: nothing else changes
        let printed = notation::print_module(&module).unwrap();
        assert_eq!(printed.trim_matches('\n'), body.trim_matches('\n'), "{path:?} is not canonical");
        let before_nodes = module.nodes.len();
        let mut converted = 0;
        for row in &mut module.declarations {
            if migrating(row) {
                row.derived = true;
                row.levels = None;
                row.r#type = None;
                row.value = None;
                row.reducibility = None;
                row.all = None;
                converted += 1;
            }
        }
        // reachability
        let mut needed = vec![false; module.nodes.len()];
        let mut work: Vec<usize> = Vec::new();
        for row in &module.declarations {
            work.extend(row.expression_roots());
            if let Some(info) = &row.eqn_info { work.extend([info.r#type, info.value]); }
            if let Some(st) = row.inductive.as_ref().and_then(|i| i.structure.as_ref()) {
                work.extend(st.fields.iter().filter_map(|f| f.auto_param));
            }
        }
        while let Some(n) = work.pop() {
            if needed[n] { continue; }
            needed[n] = true;
            work.extend(children(&module.nodes[n]));
        }
        let mut map = vec![usize::MAX; module.nodes.len()];
        let mut nodes = Vec::new();
        for (i, node) in module.nodes.iter().enumerate() {
            if needed[i] { map[i] = nodes.len(); nodes.push(remap(node, &map)); }
        }
        for row in &mut module.declarations {
            row.r#type = row.r#type.map(|r| map[r]);
            row.value = row.value.map(|r| map[r]);
            if let Some(info) = row.eqn_info.as_mut() { info.r#type = map[info.r#type]; info.value = map[info.value]; }
            if let Some(st) = row.inductive.as_mut().and_then(|i| i.structure.as_mut()) {
                for f in &mut st.fields { f.auto_param = f.auto_param.map(|r| map[r]); }
            }
        }
        module.proof_nodes = module.proof_nodes.iter().filter(|p| needed[**p]).map(|p| map[*p]).collect();
        module.nodes = nodes;
        module.validate("1.2").unwrap_or_else(|e| panic!("{path:?}: {e}"));
        let printed = notation::print_module(&module).unwrap();
        let reread = notation::parse_module(&printed).unwrap();
        assert_eq!(reread, module, "{path:?} reads back");
        // keep the exact shape of the original around the body
        let new_text = {
            let lead = &text[located.body.0..located.body.0 + (body.len() - body.trim_start_matches('\n').len())];
            let trail_len = body.len() - body.trim_end_matches('\n').len();
            let trail = &body[body.len() - trail_len..];
            format!("{}{}{}{}{}", &text[..located.body.0], lead, printed.trim_matches('\n'), trail, &text[located.body.1..])
        };
        std::fs::write(&path, new_text).unwrap();
        println!("{}: {} rows derived, nodes {} -> {}", path.file_name().unwrap().to_string_lossy(), converted, before_nodes, module.nodes.len());
        total.0 += converted; total.1 += before_nodes; total.2 += module.nodes.len();
    }
    println!("total: {} rows derived, nodes {} -> {}", total.0, total.1, total.2);
}
