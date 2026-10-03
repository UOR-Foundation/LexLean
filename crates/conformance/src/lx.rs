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
/// A type with no arguments, named by its kind alone: `bytes`, `ordering`,
/// and the fixed widths `uint8`..`int64`.
pub fn scalar_t(kind: &str) -> Json {
    json!({"kind": kind})
}
/// A type declared in the module being generated. A type of another module
/// is `lexlean::calculus::term::named`, which always names its module.
pub fn local_t(name: &str) -> Json {
    json!({"kind": "named", "member": member(name), "arguments": []})
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
/// A natural literal beyond `u64`, such as `2^64`, the bound of the
/// calculus's `nat` realization.
pub fn nat_wide(number: u128) -> Json {
    json!({"kind": "nat", "value": number.to_string()})
}
pub fn string(text: &str) -> Json {
    json!({"kind": "string", "value": text})
}
pub fn bytes(hex: &str) -> Json {
    json!({"kind": "bytes", "hex": hex})
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
pub fn beq(left: Json, right: Json) -> Json {
    json!({"kind": "beq", "left": left, "right": right})
}
pub fn ble(left: Json, right: Json) -> Json {
    json!({"kind": "ble", "left": left, "right": right})
}
pub fn blt(left: Json, right: Json) -> Json {
    json!({"kind": "blt", "left": left, "right": right})
}
pub fn and(left: Json, right: Json) -> Json {
    json!({"kind": "and", "left": left, "right": right})
}
pub fn or(left: Json, right: Json) -> Json {
    json!({"kind": "or", "left": left, "right": right})
}
pub fn not(value: Json) -> Json {
    json!({"kind": "not", "value": value})
}
/// Propositional equality, the statement of a theorem.
pub fn eq(left: Json, right: Json) -> Json {
    json!({"kind": "eq", "left": left, "right": right})
}
pub fn ite(condition: Json, then_value: Json, else_value: Json) -> Json {
    json!({"kind": "if", "condition": condition, "then_value": then_value, "else_value": else_value})
}
pub fn let_in(name: &str, ty: Json, value: Json, body: Json) -> Json {
    json!({"kind": "let", "binder": parameter(name, ty), "value": value, "body": body})
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
/// A constructor whose type arguments elaboration infers from its operands.
pub fn construct(constructor: Json, arguments: Vec<Json>) -> Json {
    json!({"kind": "constructor", "constructor": constructor, "arguments": arguments})
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

/// A type parameter of the declaration being generated.
pub fn parameter_t(name: &str) -> Json {
    json!({"kind": "parameter", "name": name})
}
/// A function type.
pub fn function_t(parameters: Vec<Json>, result: Json) -> Json {
    json!({"kind": "function", "parameters": parameters, "result": result})
}
/// A call of a polymorphic definition at `type_arguments`.
pub fn call_at(function: Json, type_arguments: Vec<Json>, arguments: Vec<Json>) -> Json {
    json!({"kind": "call", "function": function, "arguments": arguments,
           "type_arguments": type_arguments})
}
/// A definition generic in `type_parameters`.
pub fn generic(type_parameters: &[&str], mut definition: Json) -> Json {
    definition["type_parameters"] = json!(type_parameters);
    definition
}
/// An application of a function-valued term.
pub fn apply(function: Json, arguments: Vec<Json>) -> Json {
    json!({"kind": "apply", "function": function, "arguments": arguments})
}
/// A definition as a value.
pub fn function_ref(function: Json) -> Json {
    json!({"kind": "function_ref", "function": function})
}
/// `∀ (name : ty), body`.
pub fn forall(name: &str, ty: Json, body: Json) -> Json {
    json!({"kind": "forall", "binder": parameter(name, ty), "body": body})
}

// --- proofs ----------------------------------------------------------------

pub fn decide() -> Json {
    json!({"kind": "decide"})
}

// --- declarations ----------------------------------------------------------

pub fn parameter(name: &str, ty: Json) -> Json {
    json!({"name": name, "type": ty})
}
/// A monomorphic inductive: `(constructor, field types)` in order.
pub fn inductive(name: &str, constructors: Vec<(&str, Vec<Json>)>) -> Json {
    let constructors: Vec<Json> = constructors
        .into_iter()
        .map(|(name, fields)| json!({"name": name, "fields": fields}))
        .collect();
    json!({"kind": "inductive", "name": name, "type_parameters": [], "parameters": [],
           "constructors": constructors})
}
/// A monomorphic structure: `(field, type)` in order.
pub fn structure(name: &str, fields: Vec<(&str, Json)>) -> Json {
    let fields: Vec<Json> = fields
        .into_iter()
        .map(|(name, ty)| json!({"name": name, "type": ty}))
        .collect();
    json!({"kind": "structure", "name": name, "type_parameters": [], "parameters": [],
           "fields": fields})
}
/// A nonrecursive definition; [`recursive`], [`mutual`], and [`axioms`]
/// add what the others state.
pub fn definition(name: &str, parameters: Vec<Json>, result: Json, body: Json) -> Json {
    json!({"kind": "definition", "name": name, "parameters": parameters, "result": result,
           "body": body})
}
/// A closed theorem.
pub fn theorem(name: &str, statement: Json, proof: Json) -> Json {
    json!({"kind": "theorem", "name": name, "parameters": [], "statement": statement,
           "proof": proof})
}
/// A definition by structural recursion on its parameter `argument`.
pub fn recursive(argument: &str, mut definition: Json) -> Json {
    definition["recursive_argument"] = json!(argument);
    definition
}
/// A declaration in the mutual group `group`.
pub fn mutual(group: &str, mut declaration: Json) -> Json {
    declaration["mutual"] = json!(group);
    declaration
}
/// A declaration whose Lean axiom audit must report exactly `names`.
pub fn axioms(names: &[&str], mut declaration: Json) -> Json {
    declaration["axioms"] = json!(names);
    declaration
}

// --- modules ---------------------------------------------------------------

/// A `.lex.tex` module of the `compiler` project: glossary, imports, and the
/// canonical semantic data. Every `compiler` module carries the same title,
/// so it is not a parameter.
///
/// # Panics
///
/// Panics only if `serde_json` cannot serialize a JSON value.
pub fn module_tex(name: &str, imports: &[&str], declarations: Vec<Json>) -> String {
    let mut out = format!("\\begin{{lexlean}}{{{name}}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n");
    for import in imports {
        out.push_str(&format!("\\importmodule{{{import}}}\n"));
    }
    let data = json!({"spec": "lexlean/semantic-module/2", "declarations": declarations});
    out.push_str(&format!(
        "\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\n\\semanticdata{{{}}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n",
        serde_json::to_string(&data).expect("module data serializes")
    ));
    out
}
