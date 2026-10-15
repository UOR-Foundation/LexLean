//! Builders for LexLean semantic-module JSON (`lexlean/semantic-module/2`),
//! shared by every generator of a `compiler` module.
//!
//! Each builder returns exactly the object the schema names, with no
//! optional key set to a default: the committed modules are compared byte
//! for byte, so an absent key and an empty one are different files.
//! `serde_json` without `preserve_order` keeps object keys sorted, which is
//! what makes [`module_tex`] canonical.

use serde_json::{json, Value as Json};

// --- types -----------------------------------------------------------------

pub fn nat_t() -> Json {
    json!({"kind": "nat"})
}
pub fn string_t() -> Json {
    json!({"kind": "string"})
}
pub fn int_t() -> Json {
    json!({"kind": "int"})
}
pub fn int8_t() -> Json {
    json!({"kind": "int8"})
}
pub fn bool_t() -> Json {
    json!({"kind": "bool"})
}
pub fn list_t(element: Json) -> Json {
    json!({"kind": "list", "element": element})
}
pub fn product_t(left: Json, right: Json) -> Json {
    json!({"kind": "product", "left": left, "right": right})
}
pub fn option_t(value: Json) -> Json {
    json!({"kind": "option", "value": value})
}
pub fn map_t(key: Json, value: Json) -> Json {
    json!({"kind": "map", "key": key, "value": value})
}
pub fn set_t(element: Json) -> Json {
    json!({"kind": "set", "element": element})
}

// --- references ------------------------------------------------------------

/// A member of the module being generated, or a builtin such as
/// `Option.some`; a member of another module is
/// `lexlean::calculus::term::member`.
pub fn member(name: &str) -> Json {
    json!({"name": name})
}

// --- terms -----------------------------------------------------------------

pub fn nat(number: u64) -> Json {
    json!({"kind": "nat", "value": number.to_string()})
}
pub fn string(text: &str) -> Json {
    json!({"kind": "string", "value": text})
}
pub fn integer(representation: &str, number: i64) -> Json {
    json!({"kind": "integer", "representation": representation, "value": number.to_string()})
}
pub fn boolean(value: bool) -> Json {
    json!({"kind": "bool", "value": value})
}
pub fn pair(left: Json, right: Json) -> Json {
    json!({"kind": "pair", "left": left, "right": right})
}
pub fn var(name: &str) -> Json {
    json!({"kind": "var", "name": name})
}
pub fn add(left: Json, right: Json) -> Json {
    json!({"kind": "add", "left": left, "right": right})
}
pub fn blt(left: Json, right: Json) -> Json {
    json!({"kind": "blt", "left": left, "right": right})
}
pub fn ite(condition: Json, then_value: Json, else_value: Json) -> Json {
    json!({"kind": "if", "condition": condition, "then_value": then_value, "else_value": else_value})
}
pub fn first(value: Json) -> Json {
    json!({"kind": "first", "value": value})
}
pub fn second(value: Json) -> Json {
    json!({"kind": "second", "value": value})
}
pub fn project(value: Json, field: &str) -> Json {
    json!({"kind": "project", "value": value, "field": field})
}
pub fn prim(operation: &str, arguments: Vec<Json>, result: Json) -> Json {
    json!({"kind": "primitive", "operation": operation, "arguments": arguments, "result": result})
}
pub fn call(function: Json, arguments: Vec<Json>) -> Json {
    json!({"kind": "call", "function": function, "arguments": arguments})
}
pub fn some(ty: Json, value: Json) -> Json {
    json!({"kind": "constructor", "constructor": {"name": "Option.some"}, "arguments": [value], "type_arguments": [ty]})
}
pub fn none(ty: Json) -> Json {
    json!({"kind": "constructor", "constructor": {"name": "Option.none"}, "arguments": [], "type_arguments": [ty]})
}
pub fn record(ty: Json, fields: Vec<(&str, Json)>) -> Json {
    let fields: Vec<Json> = fields
        .into_iter()
        .map(|(field, value)| json!({"field": field, "value": value}))
        .collect();
    json!({"kind": "record", "type": ty, "fields": fields})
}
/// One branch of a [`matching`]: a constructor and the names it binds.
pub fn branch(constructor: Json, binders: Vec<String>, body: Json) -> Json {
    json!({"constructor": constructor, "binders": binders, "body": body})
}
pub fn matching(scrutinee: Json, branches: Vec<Json>) -> Json {
    json!({"kind": "match", "scrutinee": scrutinee, "branches": branches})
}
pub fn lambda(parameters: &[(&str, Json)], body: Json) -> Json {
    let parameters: Vec<Json> = parameters
        .iter()
        .map(|(name, ty)| parameter(name, ty.clone()))
        .collect();
    json!({"kind": "lambda", "parameters": parameters, "captures": [], "body": body})
}
pub fn nil(element: Json) -> Json {
    json!({"kind": "nil", "element": element})
}
pub fn cons(head: Json, tail: Json) -> Json {
    json!({"kind": "cons", "head": head, "tail": tail})
}
pub fn list(element: Json, items: Vec<Json>) -> Json {
    items
        .into_iter()
        .rev()
        .fold(nil(element), |tail, head| cons(head, tail))
}
pub fn map_literal(key: Json, value: Json, entries: Vec<(Json, Json)>) -> Json {
    let entries: Vec<Json> = entries
        .into_iter()
        .map(|(key, value)| json!({"key": key, "value": value}))
        .collect();
    json!({"kind": "map_literal", "key": key, "value": value, "entries": entries})
}
pub fn set_literal(element: Json, elements: Vec<Json>) -> Json {
    json!({"kind": "set_literal", "element": element, "elements": elements})
}
pub fn graph_literal(nodes: &[u64], edges: &[(u64, u64)]) -> Json {
    let nodes: Vec<Json> = nodes.iter().map(|node| nat(*node)).collect();
    let edges: Vec<Json> = edges
        .iter()
        .map(|(source, target)| json!({"source": nat(*source), "target": nat(*target)}))
        .collect();
    json!({"kind": "graph_literal", "node": nat_t(), "nodes": nodes, "edges": edges})
}
/// A `TargetSyntax.Value` constructor.
pub fn value(name: &str, arguments: Vec<Json>) -> Json {
    json!({"kind": "constructor", "constructor": {"module": "TargetSyntax", "name": format!("Value.{name}")}, "arguments": arguments})
}
/// A `TargetOracle` encoder applied to a list.
pub fn encode(encoder: &str, items: Json) -> Json {
    value(
        "list",
        vec![
            json!({"kind": "call", "function": {"module": "TargetOracle", "name": encoder}, "arguments": [items]}),
        ],
    )
}
/// `match option with none => Value.none | some found => Value.some (wrap found)`.
pub fn encode_option(option: Json, wrap: impl FnOnce(Json) -> Json) -> Json {
    json!({"kind": "match", "scrutinee": option, "branches": [
        {"constructor": {"name": "Option.none"}, "binders": [], "body": value("none", Vec::new())},
        {"constructor": {"name": "Option.some"}, "binders": ["found"], "body": value("some", vec![wrap(var("found"))])}
    ]})
}

// --- polymorphism ----------------------------------------------------------

/// A call of a polymorphic definition at `type_arguments`.
pub fn call_at(function: Json, type_arguments: Vec<Json>, arguments: Vec<Json>) -> Json {
    json!({"kind": "call", "function": function, "arguments": arguments,
           "type_arguments": type_arguments})
}

// --- proofs ----------------------------------------------------------------

// --- declarations ----------------------------------------------------------

pub fn parameter(name: &str, ty: Json) -> Json {
    json!({"name": name, "type": ty})
}
/// A nonrecursive definition; [`recursive`], [`mutual`], and [`axioms`]
/// add what the others state.
pub fn definition(name: &str, parameters: Vec<Json>, result: Json, body: Json) -> Json {
    json!({"kind": "definition", "name": name, "parameters": parameters, "result": result,
           "body": body})
}
/// A definition by structural recursion on its parameter `argument`.
pub fn recursive(argument: &str, mut definition: Json) -> Json {
    definition["recursive_argument"] = json!(argument);
    definition
}

// --- modules ---------------------------------------------------------------

/// A `.lex.tex` module of the `compiler` project: glossary, imports, and the
/// semantic module in the authoring notation (SPEC.md §17.21), which is the
/// only form a product source is written in. Every `compiler` module carries
/// the same title, so it is not a parameter.
///
/// # Panics
///
/// Panics if the declarations are not a module of the closed schema.
pub fn module_tex(name: &str, imports: &[&str], declarations: Vec<Json>) -> String {
    let data = json!({"spec": "lexlean/semantic-module/2", "declarations": declarations});
    lexlean::sdk::assemble(
        &lexlean::sdk::SourceHeader {
            name,
            glossaries: &["lexlean.std.nat@1.2.0"],
            imports,
            title: "Natural number addition",
        },
        lexlean::sdk::ModuleKind::Semantic,
        &data,
    )
    .unwrap_or_else(|error| panic!("{name}: {error}"))
}
