//! An independent model of the ordered collections (SPEC.md §17.12).
//!
//! The model is Rust's own `BTreeMap` and `BTreeSet` plus a direct
//! breadth-first search and Kahn's algorithm, none of which share code with
//! the compiler. Seeded operation sequences are run through the model, and
//! each outcome becomes a theorem stating that the generated LexLean term
//! equals the model's list, which pinned Lean then decides. Expected values
//! are written as plain lists rather than collection literals, so linking's
//! own canonical order never supplies the answer it is checked against.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{json, Value};

/// A fixed-seed generator (`SplitMix64`), so every run poses the same
/// sequences and a failure reproduces.
struct Seeded(u64);

impl Seeded {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut mixed = self.0;
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        mixed ^ (mixed >> 31)
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
}

fn nat_type() -> Value {
    json!({"kind": "nat"})
}

fn int_type() -> Value {
    json!({"kind": "int"})
}

fn string_type() -> Value {
    json!({"kind": "string"})
}

fn list_type(element: Value) -> Value {
    json!({"element": element, "kind": "list"})
}

fn set_type(element: Value) -> Value {
    json!({"element": element, "kind": "set"})
}

fn map_type(key: Value, value: Value) -> Value {
    json!({"key": key, "kind": "map", "value": value})
}

fn nat(value: u64) -> Value {
    json!({"kind": "nat", "value": value.to_string()})
}

fn int(value: i64) -> Value {
    json!({"kind": "integer", "representation": "int", "value": value.to_string()})
}

fn string(value: &str) -> Value {
    json!({"kind": "string", "value": value})
}

fn pair(left: Value, right: Value) -> Value {
    json!({"kind": "pair", "left": left, "right": right})
}

/// A plain `cons`/`nil` list, in exactly the model's order.
fn list(element: &Value, items: Vec<Value>) -> Value {
    items.into_iter().rev().fold(
        json!({"element": element, "kind": "nil"}),
        |tail, head| json!({"head": head, "kind": "cons", "tail": tail}),
    )
}

fn some(value_type: &Value, value: Value) -> Value {
    json!({
        "arguments": [value],
        "constructor": {"name": "Option.some"},
        "kind": "constructor",
        "type_arguments": [value_type],
    })
}

fn none(value_type: &Value) -> Value {
    json!({
        "arguments": [],
        "constructor": {"name": "Option.none"},
        "kind": "constructor",
        "type_arguments": [value_type],
    })
}

fn primitive(operation: &str, arguments: Vec<Value>, result: Value) -> Value {
    json!({
        "arguments": arguments,
        "kind": "primitive",
        "operation": operation,
        "result": result,
    })
}

/// A theorem decided by Lean; its exact axiom policy is what pinned Lean's
/// `decide` uses for the compared type.
fn theorem(name: String, left: Value, right: Value, axioms: &[&str]) -> Value {
    let mut declaration = json!({
        "kind": "theorem",
        "name": name,
        "parameters": [],
        "proof": {"kind": "decide"},
        "statement": {"kind": "eq", "left": left, "right": right},
    });
    if !axioms.is_empty() {
        declaration["axioms"] = json!(axioms);
    }
    declaration
}

/// Maps keyed by negative and positive integers: random insertion
/// (including replacement) and removal (including of absent keys), then the
/// entry list and a lookup, each against `BTreeMap`.
pub fn map_theorems() -> Vec<Value> {
    let mut seeded = Seeded(0x5eed_0028_0001);
    let ty = map_type(int_type(), nat_type());
    let entry = json!({"kind": "product", "left": int_type(), "right": nat_type()});
    let mut theorems = Vec::new();
    for case in 0..16 {
        let mut model = BTreeMap::<i64, u64>::new();
        let mut term =
            json!({"entries": [], "key": int_type(), "kind": "map_literal", "value": nat_type()});
        for _ in 0..4 + seeded.below(10) {
            let key = i64::try_from(seeded.below(15)).unwrap_or_default() - 7;
            if seeded.below(4) == 0 {
                model.remove(&key);
                term = primitive("map_remove", vec![term, int(key)], ty.clone());
            } else {
                let value = seeded.below(100);
                model.insert(key, value);
                term = primitive("map_insert", vec![term, int(key), nat(value)], ty.clone());
            }
        }
        let entries = model
            .iter()
            .map(|(key, value)| pair(int(*key), nat(*value)))
            .collect();
        theorems.push(theorem(
            format!("model_map_{case}"),
            primitive("map_entries", vec![term.clone()], list_type(entry.clone())),
            list(&entry, entries),
            &[],
        ));
        let probe = i64::try_from(seeded.below(15)).unwrap_or_default() - 7;
        let expected = model
            .get(&probe)
            .map_or_else(|| none(&nat_type()), |value| some(&nat_type(), nat(*value)));
        theorems.push(theorem(
            format!("model_map_lookup_{case}"),
            primitive(
                "map_lookup",
                vec![term, int(probe)],
                json!({"kind": "option", "value": nat_type()}),
            ),
            expected,
            &[],
        ));
    }
    theorems
}

/// One random natural-number set and its `BTreeSet` model.
fn random_set(seeded: &mut Seeded) -> (Value, BTreeSet<u64>) {
    let ty = set_type(nat_type());
    let mut model = BTreeSet::new();
    let mut term = json!({"element": nat_type(), "elements": [], "kind": "set_literal"});
    for _ in 0..2 + seeded.below(9) {
        let key = seeded.below(16);
        if seeded.below(5) == 0 {
            model.remove(&key);
            term = primitive("set_remove", vec![term, nat(key)], ty.clone());
        } else {
            model.insert(key);
            term = primitive("set_insert", vec![term, nat(key)], ty.clone());
        }
    }
    (term, model)
}

fn elements(term: Value, element: &Value) -> Value {
    primitive("set_elements", vec![term], list_type(element.clone()))
}

/// Natural-number sets under insertion, removal, union, intersection, and
/// difference, and string sets over non-ASCII keys, against `BTreeSet`,
/// whose string order is byte order and therefore Unicode scalar order.
pub fn set_theorems() -> Vec<Value> {
    let mut seeded = Seeded(0x5eed_0028_0002);
    let element = nat_type();
    let ty = set_type(nat_type());
    let mut theorems = Vec::new();
    let naturals = |model: &BTreeSet<u64>| model.iter().map(|key| nat(*key)).collect::<Vec<_>>();
    for case in 0..12 {
        let (left, left_model) = random_set(&mut seeded);
        let (right, right_model) = random_set(&mut seeded);
        let union: BTreeSet<u64> = left_model.union(&right_model).copied().collect();
        let intersection: BTreeSet<u64> = left_model.intersection(&right_model).copied().collect();
        let difference: BTreeSet<u64> = left_model.difference(&right_model).copied().collect();
        for (operation, model) in [
            ("set_union", union),
            ("set_intersection", intersection),
            ("set_difference", difference),
        ] {
            theorems.push(theorem(
                format!("model_{operation}_{case}"),
                elements(
                    primitive(operation, vec![left.clone(), right.clone()], ty.clone()),
                    &element,
                ),
                list(&element, naturals(&model)),
                &[],
            ));
        }
        theorems.push(theorem(
            format!("model_set_{case}"),
            elements(left, &element),
            list(&element, naturals(&left_model)),
            &[],
        ));
    }
    let alphabet = [
        "",
        "a",
        "aa",
        "ab",
        "b",
        "Z",
        "z",
        "é",
        "ü",
        "日本",
        "日",
        "\u{1F600}",
        "ß",
    ];
    let ty = set_type(string_type());
    let element = string_type();
    for case in 0..6 {
        let mut model = BTreeSet::<&str>::new();
        let mut term = json!({"element": string_type(), "elements": [], "kind": "set_literal"});
        for _ in 0..3 + seeded.below(8) {
            let key = alphabet[usize::try_from(seeded.below(13)).unwrap_or_default()];
            model.insert(key);
            term = primitive("set_insert", vec![term, string(key)], ty.clone());
        }
        theorems.push(theorem(
            format!("model_string_set_{case}"),
            elements(term, &element),
            list(&element, model.iter().map(|key| string(key)).collect()),
            &["Classical.choice", "Quot.sound", "propext"],
        ));
    }
    theorems
}

/// A graph model: its nodes are its keys and every successor (§17.12 item 4).
struct GraphModel(BTreeMap<u64, BTreeSet<u64>>);

impl GraphModel {
    fn nodes(&self) -> BTreeSet<u64> {
        self.0
            .iter()
            .flat_map(|(key, successors)| std::iter::once(*key).chain(successors.iter().copied()))
            .collect()
    }

    fn successors(&self, node: u64) -> BTreeSet<u64> {
        self.0.get(&node).cloned().unwrap_or_default()
    }

    fn reachable(&self, start: u64) -> BTreeSet<u64> {
        let mut seen = BTreeSet::from([start]);
        let mut frontier = vec![start];
        while let Some(node) = frontier.pop() {
            for successor in self.successors(node) {
                if seen.insert(successor) {
                    frontier.push(successor);
                }
            }
        }
        seen
    }

    /// Kahn's order taking the least ready node first; a node is ready when
    /// no remaining node (itself included) has it as a successor.
    fn topological(&self) -> Option<Vec<u64>> {
        let mut remaining = self.nodes();
        let mut order = Vec::new();
        while !remaining.is_empty() {
            let ready = remaining.iter().copied().find(|node| {
                remaining
                    .iter()
                    .all(|other| !self.successors(*other).contains(node))
            })?;
            remaining.remove(&ready);
            order.push(ready);
        }
        Some(order)
    }

    /// The graph built by `map_insert`, in `insertions` order, from the
    /// empty map; a repeated key replaces its successors.
    fn term(insertions: &[(u64, BTreeSet<u64>)]) -> (Value, Self) {
        let ty = map_type(nat_type(), set_type(nat_type()));
        let mut model = BTreeMap::new();
        let mut term = json!({"entries": [], "key": nat_type(), "kind": "map_literal", "value": set_type(nat_type())});
        for (node, successors) in insertions {
            model.insert(*node, successors.clone());
            let set = json!({
                "element": nat_type(),
                "elements": successors.iter().map(|successor| nat(*successor)).collect::<Vec<_>>(),
                "kind": "set_literal",
            });
            term = primitive("map_insert", vec![term, nat(*node), set], ty.clone());
        }
        (term, Self(model))
    }
}

fn graph_queries(name: &str, term: &Value, model: &GraphModel, start: u64) -> Vec<Value> {
    let node = nat_type();
    let order = list_type(nat_type());
    let reachable = model.reachable(start).into_iter().map(nat).collect();
    let topological = model.topological().map_or_else(
        || none(&order),
        |nodes| some(&order, list(&node, nodes.into_iter().map(nat).collect())),
    );
    vec![
        theorem(
            format!("{name}_reachable"),
            elements(
                primitive(
                    "graph_reachable",
                    vec![term.clone(), nat(start)],
                    set_type(nat_type()),
                ),
                &node,
            ),
            list(&node, reachable),
            &["propext"],
        ),
        theorem(
            format!("{name}_topological"),
            primitive(
                "graph_topological",
                vec![term.clone()],
                json!({"kind": "option", "value": order}),
            ),
            topological,
            &[],
        ),
    ]
}

/// Random graphs, half of them acyclic, some with successors that have no
/// entry of their own; and a chain whose depth is its node count less one,
/// ending in such a successor, so a traversal bound one short of the node
/// count would truncate it.
pub fn graph_theorems() -> Vec<Value> {
    let mut seeded = Seeded(0x5eed_0030_0001);
    let mut theorems = Vec::new();
    for case in 0..12 {
        let acyclic = case % 2 == 0;
        let mut insertions = Vec::new();
        for _ in 0..3 + seeded.below(6) {
            let node = seeded.below(8);
            let successors: BTreeSet<u64> = (0..seeded.below(4))
                .map(|_| seeded.below(10))
                .filter(|successor| !acyclic || *successor > node)
                .collect();
            insertions.push((node, successors));
        }
        let (term, model) = GraphModel::term(&insertions);
        let start = seeded.below(10);
        theorems.extend(graph_queries(
            &format!("model_graph_{case}"),
            &term,
            &model,
            start,
        ));
    }
    let depth = 24;
    let chain: Vec<(u64, BTreeSet<u64>)> = (0..depth - 1)
        .rev()
        .map(|node| (node, BTreeSet::from([node + 1])))
        .collect();
    let (term, model) = GraphModel::term(&chain);
    assert_eq!(
        model.nodes().len(),
        usize::try_from(depth).unwrap_or_default()
    );
    theorems.extend(graph_queries("model_graph_chain", &term, &model, 0));
    theorems
}

/// The collections example with `theorems` appended to its `Main` module.
pub fn with_theorems(project: &crate::support::P, theorems: Vec<Value>) {
    let source = project.read("src/Main.lex.tex");
    let mut module = crate::support::module_json(&source);
    module["declarations"]
        .as_array_mut()
        .expect("declarations")
        .extend(theorems);
    project.write(
        "src/Main.lex.tex",
        &crate::support::with_module_json(&source, &module),
    );
}
