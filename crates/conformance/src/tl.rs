//! Builders for compiler-tooling modules (`lexlean/tooling-module/1`,
//! SPEC.md §17.20), shared by the generators of tooling projects.
//!
//! Each builder returns exactly the object the schema names, with no optional
//! key set to a default: a committed module is compared byte for byte, so an
//! absent member and a defaulted one are different files. `serde_json` without
//! `preserve_order` keeps object members sorted, which is what makes the data
//! canonical (§21.7).

use serde_json::{json, Value as Json};

// --- types -----------------------------------------------------------------

/// A registered type applied to no argument.
pub fn ty(name: &str) -> Json {
    json!({"kind": "const", "name": name, "arguments": []})
}

/// A registered type applied to arguments.
pub fn ty_app(name: &str, arguments: Vec<Json>) -> Json {
    json!({"kind": "const", "name": name, "arguments": arguments})
}

/// A type variable.
pub fn tvar(name: &str) -> Json {
    json!({"kind": "parameter", "name": name, "arguments": []})
}

/// A type variable applied to arguments (a type constructor).
pub fn tvar_app(name: &str, arguments: Vec<Json>) -> Json {
    json!({"kind": "parameter", "name": name, "arguments": arguments})
}

/// A function type.
pub fn fn_ty(parameters: Vec<Json>, result: Json) -> Json {
    json!({"kind": "function", "parameters": parameters, "result": result})
}

/// An index value: a nullary constructor of an index type.
pub fn index(constructor: &str) -> Json {
    json!({"kind": "index", "constructor": constructor})
}

// --- terms -----------------------------------------------------------------

/// A local variable.
pub fn var(name: &str) -> Json {
    json!({"kind": "var", "name": name})
}

/// A natural-number literal; its value is text, so no width is implied.
pub fn nat(value: u64) -> Json {
    json!({"kind": "nat", "value": value.to_string()})
}

/// A string literal.
pub fn string(value: &str) -> Json {
    json!({"kind": "string", "value": value})
}

/// A character literal, by scalar value.
pub fn chr(value: char) -> Json {
    json!({"kind": "char", "value": u32::from(value)})
}

/// A name literal.
pub fn name(value: &str) -> Json {
    json!({"kind": "name", "value": value})
}

/// A call of a registered call or of a function of the module.
pub fn call(function: &str, type_arguments: Vec<Json>, arguments: Vec<Json>) -> Json {
    json!({"kind": "call", "function": function, "type_arguments": type_arguments, "arguments": arguments})
}

/// A constructor of a registered type.
pub fn construct(constructor: &str, type_arguments: Vec<Json>, arguments: Vec<Json>) -> Json {
    json!({"kind": "construct", "constructor": constructor, "type_arguments": type_arguments, "arguments": arguments})
}

/// The field of a value of a registered type with one constructor.
pub fn field(value: Json, field: &str) -> Json {
    json!({"kind": "field", "value": value, "field": field})
}

/// A list literal of one element type.
pub fn list(element: Json, elements: Vec<Json>) -> Json {
    json!({"kind": "list", "element": element, "elements": elements})
}

/// An array literal of one element type.
pub fn array(element: Json, elements: Vec<Json>) -> Json {
    json!({"kind": "array", "element": element, "elements": elements})
}

/// A local binding with its type.
pub fn let_in(binder: &str, ty: Json, value: Json, body: Json) -> Json {
    json!({"kind": "let", "binder": binder, "type": ty, "value": value, "body": body})
}

/// A function of typed parameters.
pub fn lambda(parameters: &[(&str, Json)], body: Json) -> Json {
    let parameters: Vec<Json> = parameters
        .iter()
        .map(|(name, ty)| json!({"name": name, "type": ty}))
        .collect();
    json!({"kind": "lambda", "parameters": parameters, "body": body})
}

/// The application of a value of function type.
pub fn apply(function: Json, arguments: Vec<Json>) -> Json {
    json!({"kind": "apply", "function": function, "arguments": arguments})
}

/// A conditional on a `Bool`.
pub fn if_then(condition: Json, then_branch: Json, else_branch: Json) -> Json {
    json!({"kind": "if", "condition": condition, "then_branch": then_branch, "else_branch": else_branch})
}

/// A flat match on an inductive type.
pub fn matching(scrutinee: Json, branches: Vec<Json>) -> Json {
    json!({"kind": "match", "scrutinee": scrutinee, "branches": branches})
}

/// A branch for one constructor, by its full name.
pub fn branch(constructor: &str, binders: &[&str], body: Json) -> Json {
    json!({"constructor": constructor, "binders": binders, "body": body})
}

/// The default branch.
pub fn otherwise(body: Json) -> Json {
    json!({"binders": [], "body": body})
}

// --- declarations ----------------------------------------------------------

/// How a function is declared.
#[derive(Clone, Copy, Default)]
pub struct Flags {
    /// The function may be called from outside the module's own text.
    pub public: bool,
    /// The function is `partial`.
    pub partial: bool,
    /// The function is a traversal, checked for complete recursion.
    pub traversal: bool,
}

/// A function the rest of the program may be built on.
pub const PUBLIC: Flags = Flags {
    public: true,
    partial: false,
    traversal: false,
};
/// A function whose recursion Lean cannot see terminate.
pub const PARTIAL: Flags = Flags {
    public: false,
    partial: true,
    traversal: false,
};
/// A partial function the checker holds to the complete-traversal rule.
pub const TRAVERSAL: Flags = Flags {
    public: false,
    partial: true,
    traversal: true,
};

/// A function of the tool.
pub fn function(
    name: &str,
    type_parameters: &[&str],
    parameters: &[(&str, Json)],
    result: Json,
    body: Json,
    flags: Flags,
    mutual: Option<&str>,
) -> Json {
    let parameters: Vec<Json> = parameters
        .iter()
        .map(|(name, ty)| json!({"name": name, "type": ty}))
        .collect();
    let mut declaration = json!({
        "kind": "function",
        "name": name,
        "public": flags.public,
        "partial": flags.partial,
        "type_parameters": type_parameters,
        "parameters": parameters,
        "result": result,
        "body": body,
    });
    if let Some(label) = mutual {
        declaration["mutual"] = json!(label);
    }
    if flags.traversal {
        declaration["traversal"] = json!(true);
    }
    declaration
}

/// The registration of a command.
pub fn command(name: &str, atoms: Vec<Json>, elaborator: &str) -> Json {
    json!({"kind": "command", "name": name, "atoms": atoms, "elaborator": elaborator})
}

/// A keyword atom of a command's syntax.
pub fn token(text: &str) -> Json {
    json!({"kind": "token", "text": text})
}

/// An identifier atom of a command's syntax.
pub fn ident_atom() -> Json {
    json!({"kind": "ident"})
}

/// A type atom of a command's syntax.
pub fn type_atom() -> Json {
    json!({"kind": "type"})
}

/// The invocation of a registered command.
pub fn invoke(command: &str, arguments: Vec<Json>) -> Json {
    json!({"kind": "invoke", "command": command, "arguments": arguments})
}

/// An identifier argument of an invocation.
pub fn ident_argument(value: &str) -> Json {
    json!({"kind": "ident", "value": value})
}

/// A type argument of an invocation, as a registered signature.
pub fn type_argument(signature: Json) -> Json {
    json!({"kind": "type", "value": signature})
}

/// The evaluation of a function of the tool.
pub fn evaluate(function: &str, arguments: Vec<Json>) -> Json {
    json!({"kind": "evaluate", "function": function, "arguments": arguments})
}

/// The module around its declarations, citing the compiler's pinned Lean.
pub fn module(authority: &str, imports: Vec<String>, declarations: Vec<Json>) -> Json {
    json!({
        "spec": "lexlean/tooling-module/1",
        "authority": {
            "id": authority,
            "lean_version": lexlean::LEAN_VERSION,
            "lean_githash": lexlean::LEAN_GITHASH,
        },
        "imports": imports,
        "declarations": declarations,
    })
}

/// The canonical LaTeX source of a project's one tooling module.
pub fn module_tex(module: &str, title: &str, glossary: &str, data: &Json) -> String {
    format!(
        "\\begin{{lexlean}}{{{module}}}\n\\useglossary{{{glossary}}}\n\\title{{{title}}}\n\\begin{{toolingmodule}}\n\\toolingdata{{{}}}\n\\end{{toolingmodule}}\n\\end{{lexlean}}\n",
        serde_json::to_string(data).expect("serializes")
    )
}
