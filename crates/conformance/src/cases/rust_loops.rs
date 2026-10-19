//! The loops of the Rust backend (SPEC.md §17.13, §17.14, §17.16): `RB-14`.
//!
//! A library template whose self calls are all tail calls is rendered as a
//! loop, so the production rows of the primitives it realizes carry no
//! `recursion`; the rows are held to the rendering, not to the template.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::library::Template;
use lexlean::calculus::realization;
use lexlean::calculus::rust::ast::{
    Block, Callee, Crate, Ctor, Expr, Ident, ItemDef, Origin, Type,
};
use lexlean::calculus::rust::{self, Profile};

use crate::calculus::cases;
use crate::support::repo_root;

/// The templates whose instance holds a loop at the time of writing: every
/// template with a function whose self calls are all tail calls and change a
/// parameter, whether the template itself or a helper it carries (the graph
/// operations and the set operations carry a membership test or a fold, and
/// still recurse through their own non-tail calls). The set is stated so that
/// a template that stops holding a loop, or starts, is a change someone reads.
const LOOPS: &[&str] = &[
    "graph_reachable",
    "graph_successors",
    "graph_topological",
    "iterate",
    "iterate_until",
    "list_fold",
    "map_contains",
    "map_fold",
    "map_lookup",
    "set_contains",
    "set_difference",
    "set_fold",
    "set_intersection",
    "set_union",
];

/// Everything an expression of a crate names: the functions it calls and
/// closes over, and whether it holds a loop.
fn visit(block: &Block, calls: &mut BTreeSet<u64>, loops: &mut usize) {
    for binding in &block.lets {
        expr(&binding.value, calls, loops);
    }
    expr(&block.tail, calls, loops);
}

fn expr(node: &Expr, calls: &mut BTreeSet<u64>, loops: &mut usize) {
    match node {
        Expr::Call { callee, args, .. } => {
            if let Callee::Function(function) = callee {
                calls.insert(*function);
            }
            args.iter().for_each(|arg| expr(arg, calls, loops));
        }
        Expr::Construct { ctor, args, .. } => {
            if let Ctor::Closure { function, .. } = ctor {
                calls.insert(*function);
            }
            args.iter().for_each(|arg| expr(arg, calls, loops));
        }
        Expr::Apply { args, .. } => args.iter().for_each(|arg| expr(arg, calls, loops)),
        Expr::Not(inner, _)
        | Expr::Box(inner, _)
        | Expr::Widen(inner, _)
        | Expr::Succeed(inner, _) => expr(inner, calls, loops),
        Expr::Pair(left, right, _) => {
            expr(left, calls, loops);
            expr(right, calls, loops);
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            expr(condition, calls, loops);
            visit(then_branch, calls, loops);
            visit(else_branch, calls, loops);
        }
        Expr::Match {
            scrutinee, arms, ..
        } => {
            expr(scrutinee, calls, loops);
            arms.iter().for_each(|(_, body)| visit(body, calls, loops));
        }
        Expr::Block(inner) => visit(inner, calls, loops),
        Expr::Loop { body, .. } => {
            *loops += 1;
            visit(body, calls, loops);
        }
        Expr::Lit(..)
        | Expr::Move(..)
        | Expr::Clone(..)
        | Expr::Copy(..)
        | Expr::Deref(..)
        | Expr::Unbox(..)
        | Expr::Uncons(_)
        | Expr::IsZero(..)
        | Expr::NonZero(..)
        | Expr::Predecessor(_) => {}
    }
}

/// The call graph of the functions of a crate, and the number of loops each
/// holds.
fn graph(krate: &Crate) -> (BTreeMap<u64, BTreeSet<u64>>, usize) {
    let mut edges = BTreeMap::new();
    let mut loops = 0;
    for item in &krate.items {
        if let ItemDef::Function {
            name: Ident::Function(function),
            body,
            ..
        } = item
        {
            let mut calls = BTreeSet::new();
            visit(body, &mut calls, &mut loops);
            edges.insert(*function, calls);
        }
    }
    (edges, loops)
}

/// Whether some function of the crate is reachable from itself: the
/// rendering recurses.
fn recurses(krate: &Crate) -> bool {
    let (edges, _) = graph(krate);
    edges.keys().any(|start| {
        let mut seen = BTreeSet::new();
        let mut pending: Vec<u64> = edges[start].iter().copied().collect();
        while let Some(next) = pending.pop() {
            if next == *start {
                return true;
            }
            if seen.insert(next) {
                pending.extend(edges.get(&next).into_iter().flatten().copied());
            }
        }
        false
    })
}

/// A crate of one function with `body`.
fn one(body: Expr) -> Crate {
    Crate {
        profile: Profile::Std,
        items: vec![ItemDef::Function {
            name: Ident::Function(0),
            parameters: Vec::new(),
            result: Type::Unit,
            body: Block::of(body),
            at: Origin::of("function"),
        }],
    }
}

/// Run the case of one `RB` conformance ID.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub fn run(id: &str) {
    assert_eq!(id, "RB-14");
    let registry: toml::Value = toml::from_str(
        &std::fs::read_to_string(
            repo_root()
                .join("language/production-1.2.toml")
                .as_std_path(),
        )
        .expect("registry"),
    )
    .expect("registry parses");
    // Each template as the fixtures instantiate it, rendered in the fixture's
    // own program: an instance alone has no closure of its step type, so no
    // value inhabits its parameter and nothing of it is rendered.
    let mut recursive: BTreeMap<&str, bool> = BTreeMap::new();
    let mut looped = BTreeSet::new();
    for case in cases() {
        if case.libraries.is_empty() {
            continue;
        }
        let program = &case.fixture.program;
        for profile in Profile::ALL {
            let krate = match rust::lower(program, profile) {
                Ok(krate) => krate,
                // `rust-core` has no heap: a fixture that allocates has no
                // rendering there, and every other refusal is a failure.
                Err(reason)
                    if profile == Profile::Core && reason.contains("requires heap allocation") =>
                {
                    continue
                }
                Err(reason) => panic!("{}: {reason}", case.fixture.name),
            };
            let (edges, _) = graph(&krate);
            for library in &case.libraries {
                let members: Vec<u64> = (library.at
                    ..library.at
                        + library
                            .template
                            .instantiate(&library.types, library.at)
                            .expect("instantiates")
                            .len() as u64)
                    .collect();
                // The instance recurses when one of its functions reaches
                // itself through the instance's own functions.
                let cycle = members.iter().any(|start| {
                    let mut seen = BTreeSet::new();
                    let mut pending: Vec<u64> = edges[start]
                        .iter()
                        .copied()
                        .filter(|next| members.contains(next))
                        .collect();
                    while let Some(next) = pending.pop() {
                        if next == *start {
                            return true;
                        }
                        if seen.insert(next) {
                            pending.extend(
                                edges[&next]
                                    .iter()
                                    .copied()
                                    .filter(|onward| members.contains(onward)),
                            );
                        }
                    }
                    false
                });
                let name = library.template.name();
                let entry = recursive.entry(name).or_insert(cycle);
                assert_eq!(*entry, cycle, "{name} recurses in every instance");
                // The calculus and the rendering agree on every function: one
                // the calculus calls a loop holds one, and one it does not
                // holds none.
                for position in &members {
                    let mut held = 0;
                    let mut calls = BTreeSet::new();
                    for item in &krate.items {
                        if let ItemDef::Function {
                            name: Ident::Function(at),
                            body,
                            ..
                        } = item
                        {
                            if at == position {
                                visit(body, &mut calls, &mut held);
                            }
                        }
                    }
                    let function = &program.functions[usize::try_from(*position).expect("index")];
                    assert_eq!(
                        rust::loops(*position, function),
                        held > 0,
                        "{}: {name} function {position}",
                        case.fixture.name
                    );
                    if held > 0 {
                        looped.insert(name);
                    }
                }
            }
        }
    }
    assert_eq!(
        recursive.len(),
        Template::ALL.len(),
        "every template is rendered in some fixture"
    );
    assert_eq!(
        looped.iter().copied().collect::<Vec<_>>(),
        LOOPS,
        "the templates rendered as loops"
    );
    // The registry says `recursion` exactly when the rendering of a
    // template the row names recurses, or the row names the structural
    // `recursion`.
    let mut disagreements = Vec::new();
    let mut compared = 0;
    for row in registry["construct"].as_array().expect("constructs") {
        if row["disposition"].as_str() != Some("runtime") {
            continue;
        }
        let key = row["key"].as_str().expect("key");
        let references = realization::TABLE
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, references)| *references)
            .expect("a realization row");
        let templates: Vec<&str> = references
            .iter()
            .filter_map(|reference| reference.strip_prefix("template:"))
            .collect();
        if templates.is_empty() {
            continue;
        }
        compared += 1;
        let rendered = references.contains(&"recursion")
            || templates.iter().any(|template| recursive[template]);
        let registered = row["recursion"].as_bool().expect("recursion");
        if registered != rendered {
            disagreements.push(format!(
                "{key}: registry {registered}, rendering {rendered}"
            ));
        }
    }
    assert!(
        compared >= 20,
        "{compared} template-realized rows are compared"
    );
    assert!(
        disagreements.is_empty(),
        "recursion disagreements: {disagreements:#?}"
    );
    // The detector is armed: a function that calls itself recurses, and one
    // that does not, does not.
    assert!(recurses(&one(Expr::Call {
        callee: Callee::Function(0),
        args: Vec::new(),
        propagate: false,
        at: Origin::of("expr:call"),
    })));
    assert!(!recurses(&one(Expr::Lit(
        lexlean::calculus::rust::ast::Lit::Unit,
        Origin::of("value:unit"),
    ))));
}
