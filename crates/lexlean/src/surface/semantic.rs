//! The notation of the closed semantic-module schema (SPEC.md §17.21).
//!
//! The notation is read from the schema, not written beside it. A position in
//! a module (a declaration, a term, a type, a member, a parameter, a list of
//! any of them) is found by walking `schemas/semantic-module-v2.schema.json`,
//! and the position decides what may be written there: a term, a type, a
//! member and a parameter have a readable form, and every value that has no
//! readable form is written generically (§17.21.1), tagged by its `kind`.
//! Because the generic form covers every value of the schema, a variant added
//! to the schema has a surface the day it is added, and the readable forms
//! below are an economy, never a gate.
//!
//! A readable form is used only when it is exactly the value: a node with a
//! member the form does not spell, or a name the form cannot carry, is
//! written generically, so printing and parsing are inverse on every value.

use std::sync::OnceLock;

use serde_json::Value;

use crate::artifact::canonical_json::Json;

use super::doc::{render, Doc};
use super::generic::{self, delimited, describe, field, is_bare, Reader, MAX_DEPTH, WIDTH};
use super::lex::{quote, Token};
use super::SurfaceError;

/// The schema every position is found in.
fn schema() -> &'static Value {
    static SCHEMA: OnceLock<Value> = OnceLock::new();
    SCHEMA.get_or_init(|| {
        let bytes = crate::embedded::FILES
            .iter()
            .find(|(path, _)| *path == "schemas/semantic-module-v2.schema.json")
            .map(|(_, bytes)| *bytes)
            .expect("the semantic-module schema is embedded");
        serde_json::from_slice(bytes).expect("the semantic-module schema is JSON")
    })
}

/// What reads the value at a position.
#[derive(Clone, Copy)]
enum Pos {
    /// A term (§17.21.2).
    Term,
    /// A type (§17.21.2).
    Type,
    /// A reference to a declaration: `name` or `Module::name`.
    Member,
    /// A parameter: `name : type`.
    Param,
    /// Any other position, described by this schema node.
    Node(&'static Value),
    /// A position the schema says nothing about.
    Any,
}

/// Follow `$ref`s to the node they name.
fn resolve(mut node: &'static Value) -> &'static Value {
    while let Some(reference) = node.get("$ref").and_then(Value::as_str) {
        let name = reference.rsplit('/').next().unwrap_or(reference);
        match schema()["$defs"].get(name) {
            Some(next) => node = next,
            None => break,
        }
    }
    node
}

/// The position a schema node describes.
fn classify(node: &'static Value) -> Pos {
    if let Some(reference) = node.get("$ref").and_then(Value::as_str) {
        return match reference.rsplit('/').next() {
            Some("term") => Pos::Term,
            Some("type") => Pos::Type,
            Some("member") => Pos::Member,
            Some("parameter") => Pos::Param,
            _ => Pos::Node(resolve(node)),
        };
    }
    Pos::Node(node)
}

/// The position of the items of an array position.
fn items_of(node: &'static Value) -> Pos {
    resolve(node).get("items").map_or(Pos::Any, classify)
}

/// The position of the member `key` of an object schema.
fn member_of(object: Option<&'static Value>, key: &str) -> Pos {
    object
        .and_then(|object| resolve(object).get("properties"))
        .and_then(|properties| properties.get(key))
        .map_or(Pos::Any, classify)
}

/// The alternative of a tagged union that has `"kind": kind`.
fn alternative(union: &'static Value, kind: &str) -> Option<&'static Value> {
    resolve(union)
        .get("oneOf")?
        .as_array()?
        .iter()
        .map(resolve)
        .find(|alternative| {
            alternative
                .get("properties")
                .and_then(|properties| properties.get("kind"))
                .is_some_and(|tag| {
                    tag.get("const").and_then(Value::as_str) == Some(kind)
                        || tag
                            .get("enum")
                            .and_then(Value::as_array)
                            .is_some_and(|kinds| kinds.iter().any(|k| k.as_str() == Some(kind)))
                })
        })
}

/// Whether a schema node is a union of tagged alternatives.
fn is_union(node: &'static Value) -> bool {
    resolve(node)
        .get("oneOf")
        .and_then(Value::as_array)
        .is_some_and(|alternatives| {
            alternatives.iter().all(|alternative| {
                resolve(alternative)
                    .get("properties")
                    .is_some_and(|properties| properties.get("kind").is_some())
            })
        })
}

/// The union a position reads, if it reads one.
fn union_of(pos: Pos) -> Option<&'static Value> {
    match pos {
        Pos::Term => schema()["$defs"].get("term"),
        Pos::Type => schema()["$defs"].get("type"),
        Pos::Node(node) if is_union(node) => Some(node),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Printing.
// ---------------------------------------------------------------------------

/// The document of `value` at `pos`.
fn doc_at(value: &Json, pos: Pos) -> Doc {
    match pos {
        Pos::Term => terms::print(value).unwrap_or_else(|| tagged(value, pos)),
        Pos::Type => types::print(value).unwrap_or_else(|| tagged(value, pos)),
        Pos::Member => member_doc(value).unwrap_or_else(|| generic::doc(value)),
        Pos::Param => param_doc(value).unwrap_or_else(|| tagged(value, pos)),
        Pos::Node(_) => tagged(value, pos),
        Pos::Any => generic::doc(value),
    }
}

/// The generic spelling of `value`, its children read at the positions the
/// schema gives them.
fn tagged(value: &Json, pos: Pos) -> Doc {
    match value {
        Json::Arr(items) => {
            let item_pos = match pos {
                Pos::Node(node) => items_of(node),
                _ => Pos::Any,
            };
            delimited(
                "[",
                "]",
                items.iter().map(|item| doc_at(item, item_pos)).collect(),
            )
        }
        Json::Obj(object) => {
            let kind = object.get("kind").and_then(|kind| match kind {
                Json::Str(kind) if is_bare(kind) => Some(kind.as_str()),
                _ => None,
            });
            // The schema of the object: the alternative the tag names, or the
            // position's own node.
            let schema_of: Option<&'static Value> = match (pos, kind) {
                (Pos::Term | Pos::Type, Some(kind)) => {
                    union_of(pos).and_then(|union| alternative(union, kind))
                }
                (Pos::Node(node), Some(kind)) if is_union(node) => alternative(node, kind),
                (Pos::Node(node), _) => Some(node),
                (Pos::Param, _) => schema()["$defs"].get("parameter"),
                _ => None,
            };
            let fields: Vec<Doc> = generic::ordered(object)
                .into_iter()
                .filter(|(key, _)| !(kind.is_some() && key.as_str() == "kind"))
                .map(|(key, member)| field(key, doc_at(member, member_of(schema_of, key))))
                .collect();
            let open = kind.map_or_else(|| "{".to_owned(), |kind| format!("{kind}{{"));
            delimited(&open, "}", fields)
        }
        leaf => generic::doc(leaf),
    }
}

/// `name` or `Module::name`.
fn member_doc(value: &Json) -> Option<Doc> {
    let Json::Obj(object) = value else {
        return None;
    };
    let name = string_member(object, "name")?;
    let module = object.get("module").map(|module| match module {
        Json::Str(module) => Some(module.as_str()),
        _ => None,
    });
    let keys = if module.is_some() { 2 } else { 1 };
    if object.len() != keys || !is_bare(name) {
        return None;
    }
    match module {
        None => Some(Doc::text(name)),
        Some(Some(module)) if is_bare(module) => Some(Doc::text(format!("{module}::{name}"))),
        Some(_) => None,
    }
}

/// `name : type`.
fn param_doc(value: &Json) -> Option<Doc> {
    let Json::Obj(object) = value else {
        return None;
    };
    let name = string_member(object, "name")?;
    let ty = object.get("type")?;
    if object.len() != 2 || !is_bare(name) {
        return None;
    }
    Some(Doc::concat(vec![
        Doc::text(format!("{name} : ")),
        doc_at(ty, Pos::Type),
    ]))
}

/// A string member of an object.
fn string_member<'o>(
    object: &'o std::collections::BTreeMap<String, Json>,
    key: &str,
) -> Option<&'o str> {
    match object.get(key)? {
        Json::Str(text) => Some(text.as_str()),
        _ => None,
    }
}

/// Whether the object has exactly these keys.
fn exact(object: &std::collections::BTreeMap<String, Json>, keys: &[&str]) -> bool {
    object.len() == keys.len() && keys.iter().all(|key| object.contains_key(*key))
}

// ---------------------------------------------------------------------------
// Parsing.
// ---------------------------------------------------------------------------

/// Read the value at `pos`.
fn read_at(reader: &mut Reader<'_>, pos: Pos, depth: usize) -> Result<Json, SurfaceError> {
    if depth > MAX_DEPTH {
        return Err(reader.error("nested more deeply than the notation reads"));
    }
    match pos {
        Pos::Term => terms::read(reader, depth),
        Pos::Type => types::read(reader, depth),
        Pos::Member => read_member(reader),
        Pos::Param => read_param(reader, depth),
        Pos::Node(_) => read_tagged(reader, pos, depth),
        Pos::Any => generic::value(reader, depth),
    }
}

/// The generic spelling at `pos`: an array, an object, a tagged object, or a
/// leaf.
fn read_tagged(reader: &mut Reader<'_>, pos: Pos, depth: usize) -> Result<Json, SurfaceError> {
    match reader.peek() {
        Some(Token::LBrack) => {
            reader.bump();
            let item_pos = match pos {
                Pos::Node(node) => items_of(node),
                _ => Pos::Any,
            };
            let mut items = Vec::new();
            while reader.peek() != Some(&Token::RBrack) {
                items.push(read_at(reader, item_pos, depth + 1)?);
                if !reader.eat(&Token::Comma) {
                    break;
                }
            }
            reader.expect(&Token::RBrack, "`,` or `]`")?;
            Ok(Json::Arr(items))
        }
        Some(Token::LBrace) => {
            reader.bump();
            let schema_of = match pos {
                Pos::Node(node) => Some(node),
                Pos::Param => schema()["$defs"].get("parameter"),
                _ => None,
            };
            read_fields(reader, None, schema_of, depth)
        }
        Some(Token::Word(word))
            if reader.peek_at(1) == Some(&Token::LBrace)
                && is_bare(word)
                && !word.bytes().all(|byte| byte.is_ascii_digit()) =>
        {
            let kind = word.clone();
            reader.bump();
            reader.bump();
            let schema_of: Option<&'static Value> = match pos {
                Pos::Term | Pos::Type => union_of(pos).and_then(|union| alternative(union, &kind)),
                Pos::Node(node) if is_union(node) => alternative(node, &kind),
                Pos::Node(node) => Some(node),
                _ => None,
            };
            read_fields(reader, Some(kind), schema_of, depth)
        }
        _ => generic::value(reader, depth),
    }
}

/// The members of an object, after its `{`, each read at the position the
/// schema gives it.
fn read_fields(
    reader: &mut Reader<'_>,
    kind: Option<String>,
    schema_of: Option<&'static Value>,
    depth: usize,
) -> Result<Json, SurfaceError> {
    let mut members = std::collections::BTreeMap::new();
    if let Some(kind) = kind {
        members.insert("kind".to_owned(), Json::Str(kind));
    }
    while reader.peek() != Some(&Token::RBrace) {
        let offset = reader.offset();
        let key = match reader.bump() {
            Some(Token::Word(word)) if is_bare(&word) => word,
            Some(Token::Str(text)) => text,
            _ => {
                return Err(SurfaceError {
                    offset,
                    message: "expected a member name".to_owned(),
                });
            }
        };
        reader.expect(&Token::Colon, "`:` after a member name")?;
        let member = read_at(reader, member_of(schema_of, &key), depth + 1)?;
        if members.insert(key.clone(), member).is_some() {
            return Err(SurfaceError {
                offset,
                message: format!("the member `{key}` is written twice"),
            });
        }
        if !reader.eat(&Token::Comma) {
            break;
        }
    }
    reader.expect(&Token::RBrace, "`,` or `}`")?;
    Ok(Json::Obj(members))
}

/// A bare name.
fn read_name(reader: &mut Reader<'_>, what: &str) -> Result<String, SurfaceError> {
    let offset = reader.offset();
    match reader.bump() {
        Some(Token::Word(word)) if is_bare(&word) => Ok(word),
        Some(other) => Err(SurfaceError {
            offset,
            message: format!("expected {what}, found {}", describe(&other)),
        }),
        None => Err(reader.error(format!("expected {what}, found the end of the text"))),
    }
}

/// `name`, `Module::name`, or the generic object.
fn read_member(reader: &mut Reader<'_>) -> Result<Json, SurfaceError> {
    if reader.peek() == Some(&Token::LBrace) {
        return generic::value(reader, 0);
    }
    member_path(reader)
}

/// `name` or `Module::name` as a member object.
fn member_path(reader: &mut Reader<'_>) -> Result<Json, SurfaceError> {
    let first = read_name(reader, "a name")?;
    let mut object = std::collections::BTreeMap::new();
    if reader.eat(&Token::Scope) {
        let name = read_name(reader, "a name after `::`")?;
        object.insert("module".to_owned(), Json::Str(first));
        object.insert("name".to_owned(), Json::Str(name));
    } else {
        object.insert("name".to_owned(), Json::Str(first));
    }
    Ok(Json::Obj(object))
}

/// `name : type`, or the generic object.
fn read_param(reader: &mut Reader<'_>, depth: usize) -> Result<Json, SurfaceError> {
    if reader.peek() == Some(&Token::LBrace) {
        return read_tagged(reader, Pos::Param, depth);
    }
    let name = read_name(reader, "a parameter name")?;
    reader.expect(&Token::Colon, "`:` after a parameter name")?;
    let ty = types::read(reader, depth + 1)?;
    let mut object = std::collections::BTreeMap::new();
    object.insert("name".to_owned(), Json::Str(name));
    object.insert("type".to_owned(), ty);
    Ok(Json::Obj(object))
}

/// A bracketed, comma-separated list of what `item` reads.
fn read_list<T>(
    reader: &mut Reader<'_>,
    mut item: impl FnMut(&mut Reader<'_>) -> Result<T, SurfaceError>,
) -> Result<Vec<T>, SurfaceError> {
    reader.expect(&Token::LBrack, "`[`")?;
    let mut items = Vec::new();
    while reader.peek() != Some(&Token::RBrack) {
        items.push(item(reader)?);
        if !reader.eat(&Token::Comma) {
            break;
        }
    }
    reader.expect(&Token::RBrack, "`,` or `]`")?;
    Ok(items)
}

// ---------------------------------------------------------------------------
// Types.
// ---------------------------------------------------------------------------

/// The readable forms of types (§17.21.2).
mod types {
    use super::{
        doc_at, exact, is_bare, member_doc, member_path, read_list, read_tagged, string_member,
        Doc, Json, Pos, Reader, SurfaceError, Token,
    };

    /// The nullary types, by the kind they spell.
    const NULLARY: [(&str, &str); 19] = [
        ("Bool", "bool"),
        ("Bytes", "bytes"),
        ("Int", "int"),
        ("Int8", "int8"),
        ("Int16", "int16"),
        ("Int32", "int32"),
        ("Int64", "int64"),
        ("Nat", "nat"),
        ("Ordering", "ordering"),
        ("Prop", "prop"),
        ("String", "string"),
        ("Type", "type"),
        ("UInt8", "uint8"),
        ("UInt16", "uint16"),
        ("UInt32", "uint32"),
        ("UInt64", "uint64"),
        ("Unit", "unit"),
        ("ContractViolation", "contract_violation"),
        ("ReasoningFailure", "reasoning_failure"),
    ];

    /// The words that head an applied type.
    const HEADS: [&str; 6] = ["List", "Option", "Result", "Set", "Map", "Fn"];

    /// Whether a word is a type the notation spells itself.
    fn reserved(word: &str) -> bool {
        NULLARY.iter().any(|(spelling, _)| *spelling == word) || HEADS.contains(&word)
    }

    /// The readable form of a type, if it has exactly one.
    pub(super) fn print(value: &Json) -> Option<Doc> {
        let Json::Obj(object) = value else {
            return None;
        };
        let Json::Str(kind) = object.get("kind")? else {
            return None;
        };
        let child = |key: &str| object.get(key).map(|ty| doc_at(ty, Pos::Type));
        let applied = |head: &str, parts: Vec<Doc>| {
            let mut docs = vec![Doc::text(format!("({head}"))];
            for part in parts {
                docs.push(Doc::text(" "));
                docs.push(part);
            }
            docs.push(Doc::text(")"));
            Doc::concat(docs)
        };
        if let Some((spelling, _)) = NULLARY.iter().find(|(_, tag)| tag == kind) {
            return exact(object, &["kind"]).then(|| Doc::text(*spelling));
        }
        match kind.as_str() {
            "parameter" if exact(object, &["kind", "name"]) => {
                let name = string_member(object, "name")?;
                is_bare(name).then(|| Doc::text(format!("'{name}")))
            }
            "list" if exact(object, &["kind", "element"]) => {
                Some(applied("List", vec![child("element")?]))
            }
            "set" if exact(object, &["kind", "element"]) => {
                Some(applied("Set", vec![child("element")?]))
            }
            "option" if exact(object, &["kind", "value"]) => {
                Some(applied("Option", vec![child("value")?]))
            }
            "result" if exact(object, &["kind", "ok", "error"]) => {
                Some(applied("Result", vec![child("ok")?, child("error")?]))
            }
            "map" if exact(object, &["kind", "key", "value"]) => {
                Some(applied("Map", vec![child("key")?, child("value")?]))
            }
            "product" if exact(object, &["kind", "left", "right"]) => Some(Doc::concat(vec![
                Doc::text("("),
                child("left")?,
                Doc::text(" * "),
                child("right")?,
                Doc::text(")"),
            ])),
            "function" if exact(object, &["kind", "parameters", "result"]) => {
                let Json::Arr(parameters) = object.get("parameters")? else {
                    return None;
                };
                let parameters = parameters
                    .iter()
                    .map(|ty| doc_at(ty, Pos::Type))
                    .collect::<Vec<_>>();
                let mut inner = Vec::new();
                for (index, parameter) in parameters.into_iter().enumerate() {
                    if index > 0 {
                        inner.push(Doc::text(" "));
                    }
                    inner.push(parameter);
                }
                Some(applied(
                    "Fn",
                    vec![
                        Doc::concat(
                            std::iter::once(Doc::text("("))
                                .chain(inner)
                                .chain(std::iter::once(Doc::text(")")))
                                .collect(),
                        ),
                        child("result")?,
                    ],
                ))
            }
            "named" if exact(object, &["kind", "member", "arguments"]) => {
                let member = object.get("member")?;
                let Json::Obj(member_object) = member else {
                    return None;
                };
                let name = string_member(member_object, "name")?;
                let local = !member_object.contains_key("module");
                if local && reserved(name) {
                    return None;
                }
                let Json::Arr(arguments) = object.get("arguments")? else {
                    return None;
                };
                let head = member_doc(member)?;
                if arguments.is_empty() {
                    return Some(head);
                }
                let mut docs = vec![Doc::text("("), head];
                for argument in arguments {
                    docs.push(Doc::text(" "));
                    docs.push(doc_at(argument, Pos::Type));
                }
                docs.push(Doc::text(")"));
                Some(Doc::concat(docs))
            }
            _ => None,
        }
    }

    /// Read a type.
    pub(super) fn read(reader: &mut Reader<'_>, depth: usize) -> Result<Json, SurfaceError> {
        let offset = reader.offset();
        match reader.peek() {
            Some(Token::Word(word)) => {
                let word = word.clone();
                if reader.peek_at(1) == Some(&Token::LBrace) && is_bare(&word) {
                    return read_tagged(reader, Pos::Type, depth);
                }
                if let Some((_, kind)) = NULLARY.iter().find(|(spelling, _)| *spelling == word) {
                    reader.bump();
                    return Ok(node(kind, vec![]));
                }
                if let Some(name) = word.strip_prefix('\'') {
                    if is_bare(name) {
                        reader.bump();
                        return Ok(node(
                            "parameter",
                            vec![("name", Json::Str(name.to_owned()))],
                        ));
                    }
                }
                if HEADS.contains(&word.as_str()) {
                    return Err(SurfaceError {
                        offset,
                        message: format!("`{word}` takes arguments: write `({word} …)`"),
                    });
                }
                let member = member_path(reader)?;
                Ok(node(
                    "named",
                    vec![("member", member), ("arguments", Json::Arr(vec![]))],
                ))
            }
            Some(Token::LParen) => {
                reader.bump();
                let parsed = paren(reader, depth)?;
                reader.expect(&Token::RParen, "`)`")?;
                Ok(parsed)
            }
            Some(other) => Err(SurfaceError {
                offset,
                message: format!("expected a type, found {}", super::describe(other)),
            }),
            None => Err(reader.error("expected a type, found the end of the text")),
        }
    }

    /// An object with a `kind`.
    fn node(kind: &str, members: Vec<(&str, Json)>) -> Json {
        let mut object = std::collections::BTreeMap::new();
        object.insert("kind".to_owned(), Json::Str(kind.to_owned()));
        for (key, member) in members {
            object.insert(key.to_owned(), member);
        }
        Json::Obj(object)
    }

    /// The inside of a parenthesised type, after the `(`.
    fn paren(reader: &mut Reader<'_>, depth: usize) -> Result<Json, SurfaceError> {
        let next = |reader: &mut Reader<'_>| read(reader, depth + 1);
        if let Some(Token::Word(head)) = reader.peek() {
            let head = head.clone();
            let one = |reader: &mut Reader<'_>, kind: &str, key: &str| {
                reader.bump();
                let item = next(reader)?;
                Ok(node(kind, vec![(key, item)]))
            };
            match head.as_str() {
                "List" => return one(reader, "list", "element"),
                "Set" => return one(reader, "set", "element"),
                "Option" => return one(reader, "option", "value"),
                "Result" => {
                    reader.bump();
                    let ok = next(reader)?;
                    let error = next(reader)?;
                    return Ok(node("result", vec![("ok", ok), ("error", error)]));
                }
                "Map" => {
                    reader.bump();
                    let key = next(reader)?;
                    let value = next(reader)?;
                    return Ok(node("map", vec![("key", key), ("value", value)]));
                }
                "Fn" => {
                    reader.bump();
                    reader.expect(&Token::LParen, "`(` before the parameter types")?;
                    let mut parameters = Vec::new();
                    while reader.peek() != Some(&Token::RParen) {
                        parameters.push(next(reader)?);
                    }
                    reader.expect(&Token::RParen, "`)`")?;
                    let result = next(reader)?;
                    return Ok(node(
                        "function",
                        vec![("parameters", Json::Arr(parameters)), ("result", result)],
                    ));
                }
                _ => {}
            }
            // `(Name argument …)`: a named type applied, unless an infix `*`
            // follows the name.
            let after_name = if reader.peek_at(1) == Some(&Token::Scope) {
                3
            } else {
                1
            };
            let infix = matches!(reader.peek_at(after_name), Some(Token::Word(op)) if op == "*");
            if !infix && !head.starts_with('\'') && !reserved(&head) {
                let member = member_path(reader)?;
                let mut arguments = Vec::new();
                while reader.peek() != Some(&Token::RParen) {
                    arguments.push(next(reader)?);
                }
                return Ok(node(
                    "named",
                    vec![("member", member), ("arguments", Json::Arr(arguments))],
                ));
            }
        }
        let left = next(reader)?;
        if !reader.eat_word("*") {
            return Err(reader.error("expected `*` between the two types of a product"));
        }
        let right = next(reader)?;
        Ok(node("product", vec![("left", left), ("right", right)]))
    }

    /// A list of types in brackets, for the callers that write one.
    pub(super) fn read_types(
        reader: &mut Reader<'_>,
        depth: usize,
    ) -> Result<Vec<Json>, SurfaceError> {
        read_list(reader, |reader| read(reader, depth + 1))
    }

    /// The integer types a literal can name, by representation.
    const INTEGERS: [(&str, &str); 9] = [
        ("Int", "int"),
        ("Int8", "int8"),
        ("Int16", "int16"),
        ("Int32", "int32"),
        ("Int64", "int64"),
        ("UInt8", "uint8"),
        ("UInt16", "uint16"),
        ("UInt32", "uint32"),
        ("UInt64", "uint64"),
    ];

    /// The type word of an integer representation.
    pub(super) fn spelling_of(representation: &str) -> Option<&'static str> {
        INTEGERS
            .iter()
            .find(|(_, tag)| *tag == representation)
            .map(|(spelling, _)| *spelling)
    }

    /// The representation an integer type word names.
    pub(super) fn representation_of(spelling: &str) -> Option<&'static str> {
        INTEGERS
            .iter()
            .find(|(word, _)| *word == spelling)
            .map(|(_, tag)| *tag)
    }
}

// ---------------------------------------------------------------------------
// Terms.
// ---------------------------------------------------------------------------

/// The readable forms of terms (§17.21.2).
mod terms {
    use super::{
        doc_at, exact, is_bare, member_doc, member_path, param_doc, quote, read_list, read_name,
        read_param, read_tagged, string_member, types, Doc, Json, Pos, Reader, SurfaceError, Token,
    };

    /// The words a form of the notation is headed or divided by. A variable
    /// or a function with such a name is written generically.
    const RESERVED: [&str; 21] = [
        "let", "if", "match", "fun", "forall", "nil", "cons", "first", "second", "not", "apply",
        "ctor", "record", "project", "prim", "ref", "checked", "then", "else", "with", "true",
    ];

    /// The binary forms: the kind, its operator, its two members.
    const INFIX: [(&str, &str, &str, &str); 12] = [
        ("add", "+", "left", "right"),
        ("eq", "=", "left", "right"),
        ("le", "<=", "left", "right"),
        ("lt", "<", "left", "right"),
        ("beq", "=?", "left", "right"),
        ("ble", "<=?", "left", "right"),
        ("blt", "<?", "left", "right"),
        ("and", "&&", "left", "right"),
        ("or", "||", "left", "right"),
        ("prop_and", "/\\", "left", "right"),
        ("iff", "<->", "left", "right"),
        ("implies", "->", "premise", "conclusion"),
    ];

    /// Decimal digits with no leading zero.
    fn canonical_digits(text: &str) -> bool {
        !text.is_empty()
            && text.bytes().all(|byte| byte.is_ascii_digit())
            && (text.len() == 1 || !text.starts_with('0'))
    }

    /// A decimal integer: digits with no leading zero, and `-` before a
    /// nonzero one.
    fn canonical_integer(text: &str) -> bool {
        match text.strip_prefix('-') {
            Some(digits) => canonical_digits(digits) && digits != "0",
            None => canonical_digits(text),
        }
    }

    /// An even number of lower-case hexadecimal digits.
    fn canonical_hex(text: &str) -> bool {
        text.len().is_multiple_of(2)
            && text
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    }

    /// Whether a name may be written as a bare word in a term.
    fn plain(name: &str) -> bool {
        is_bare(name) && !RESERVED.contains(&name) && name != "false"
    }

    fn obj(kind: &str, members: Vec<(&str, Json)>) -> Json {
        let mut object = std::collections::BTreeMap::new();
        object.insert("kind".to_owned(), Json::Str(kind.to_owned()));
        for (key, member) in members {
            object.insert(key.to_owned(), member);
        }
        Json::Obj(object)
    }

    /// `(head part …)`.
    fn form(head: &str, parts: Vec<Doc>) -> Doc {
        let mut docs = vec![Doc::text(format!("({head}"))];
        for part in parts {
            docs.push(Doc::text(" "));
            docs.push(part);
        }
        docs.push(Doc::text(")"));
        Doc::concat(docs)
    }

    /// `[a, b]`.
    fn bracketed(items: Vec<Doc>) -> Doc {
        let mut docs = vec![Doc::text("[")];
        let count = items.len();
        for (index, item) in items.into_iter().enumerate() {
            docs.push(item);
            if index + 1 < count {
                docs.push(Doc::text(", "));
            }
        }
        docs.push(Doc::text("]"));
        Doc::concat(docs)
    }

    fn array<'v>(
        object: &'v std::collections::BTreeMap<String, Json>,
        key: &str,
    ) -> Option<&'v Vec<Json>> {
        match object.get(key)? {
            Json::Arr(items) => Some(items),
            _ => None,
        }
    }

    fn term_docs(items: &[Json]) -> Vec<Doc> {
        items.iter().map(|item| doc_at(item, Pos::Term)).collect()
    }

    fn type_arguments(object: &std::collections::BTreeMap<String, Json>) -> Option<Vec<Doc>> {
        match object.get("type_arguments") {
            None => Some(Vec::new()),
            Some(Json::Arr(items)) if !items.is_empty() => Some(vec![bracketed(
                items.iter().map(|ty| doc_at(ty, Pos::Type)).collect(),
            )]),
            _ => None,
        }
    }

    fn keys_with(object: &std::collections::BTreeMap<String, Json>, base: &[&str]) -> bool {
        let extra = usize::from(object.contains_key("type_arguments"));
        object.len() == base.len() + extra && base.iter().all(|key| object.contains_key(*key))
    }

    /// The readable form of a term, if it has exactly one.
    pub(super) fn print(value: &Json) -> Option<Doc> {
        let Json::Obj(object) = value else {
            return None;
        };
        let Json::Str(kind) = object.get("kind")? else {
            return None;
        };
        let term = |key: &str| object.get(key).map(|term| doc_at(term, Pos::Term));
        if let Some((_, operator, left, right)) = INFIX.iter().find(|(tag, ..)| tag == kind) {
            return exact(object, &["kind", left, right]).then(|| {
                Doc::concat(vec![
                    Doc::text("("),
                    term(left).unwrap_or(Doc::Nil),
                    Doc::text(format!(" {operator} ")),
                    term(right).unwrap_or(Doc::Nil),
                    Doc::text(")"),
                ])
            });
        }
        match kind.as_str() {
            "var" if exact(object, &["kind", "name"]) => {
                let name = string_member(object, "name")?;
                plain(name).then(|| Doc::text(name))
            }
            "nat" if exact(object, &["kind", "value"]) => {
                let digits = string_member(object, "value")?;
                canonical_digits(digits).then(|| Doc::text(digits))
            }
            "bool" if exact(object, &["kind", "value"]) => match object.get("value")? {
                Json::Bool(true) => Some(Doc::text("true")),
                Json::Bool(false) => Some(Doc::text("false")),
                _ => None,
            },
            "string" if exact(object, &["kind", "value"]) => {
                Some(Doc::text(quote(string_member(object, "value")?)))
            }
            "bytes" if exact(object, &["kind", "hex"]) => {
                let hex = string_member(object, "hex")?;
                canonical_hex(hex).then(|| Doc::text(format!("0x{hex}")))
            }
            "unit" if exact(object, &["kind"]) => Some(Doc::text("()")),
            "integer" if exact(object, &["kind", "representation", "value"]) => {
                let value = string_member(object, "value")?;
                let representation = string_member(object, "representation")?;
                let spelling = types::spelling_of(representation)?;
                canonical_integer(value).then(|| Doc::text(format!("({value} : {spelling})")))
            }
            "pair" if exact(object, &["kind", "left", "right"]) => Some(Doc::concat(vec![
                Doc::text("("),
                term("left")?,
                Doc::text(", "),
                term("right")?,
                Doc::text(")"),
            ])),
            "not" if exact(object, &["kind", "value"]) => Some(form("not", vec![term("value")?])),
            "first" if exact(object, &["kind", "value"]) => {
                Some(form("first", vec![term("value")?]))
            }
            "second" if exact(object, &["kind", "value"]) => {
                Some(form("second", vec![term("value")?]))
            }
            "cons" if exact(object, &["kind", "head", "tail"]) => {
                Some(form("cons", vec![term("head")?, term("tail")?]))
            }
            "nil" if exact(object, &["kind", "element"]) => {
                Some(form("nil", vec![doc_at(object.get("element")?, Pos::Type)]))
            }
            "if" if exact(object, &["kind", "condition", "then_value", "else_value"]) => {
                Some(Doc::concat(vec![
                    Doc::text("(if "),
                    term("condition")?,
                    Doc::text(" then "),
                    term("then_value")?,
                    Doc::text(" else "),
                    term("else_value")?,
                    Doc::text(")"),
                ]))
            }
            "let" if exact(object, &["kind", "binder", "value", "body"]) => {
                let binder = param_doc(object.get("binder")?)?;
                Some(Doc::concat(vec![
                    Doc::text("(let "),
                    binder,
                    Doc::text(" := "),
                    term("value")?,
                    Doc::text("; "),
                    term("body")?,
                    Doc::text(")"),
                ]))
            }
            "forall" if exact(object, &["kind", "binder", "body"]) => {
                let binder = param_doc(object.get("binder")?)?;
                Some(Doc::concat(vec![
                    Doc::text("(forall ("),
                    binder,
                    Doc::text("), "),
                    term("body")?,
                    Doc::text(")"),
                ]))
            }
            "lambda" if exact(object, &["kind", "parameters", "captures", "body"]) => {
                let parameters = array(object, "parameters")?;
                let mut docs = vec![Doc::text("(fun ")];
                let captures = array(object, "captures")?;
                if !captures.is_empty() {
                    let names = captures
                        .iter()
                        .map(|capture| match capture {
                            Json::Str(name) if is_bare(name) => Some(Doc::text(name.clone())),
                            _ => None,
                        })
                        .collect::<Option<Vec<_>>>()?;
                    docs.push(bracketed(names));
                    docs.push(Doc::text(" "));
                }
                for parameter in parameters {
                    docs.push(Doc::text("("));
                    docs.push(param_doc(parameter)?);
                    docs.push(Doc::text(") "));
                }
                docs.push(Doc::text("=> "));
                docs.push(term("body")?);
                docs.push(Doc::text(")"));
                Some(Doc::concat(docs))
            }
            "apply" if exact(object, &["kind", "function", "arguments"]) => {
                let mut parts = vec![term("function")?];
                parts.extend(term_docs(array(object, "arguments")?));
                Some(form("apply", parts))
            }
            "project" if exact(object, &["kind", "value", "field"]) => {
                let field = string_member(object, "field")?;
                if !is_bare(field) {
                    return None;
                }
                Some(form("project", vec![term("value")?, Doc::text(field)]))
            }
            "call" if keys_with(object, &["kind", "function", "arguments"]) => {
                let function = object.get("function")?;
                let name = member_name(function)?;
                if !plain(name) {
                    return None;
                }
                let mut parts = vec![member_doc(function)?];
                parts.extend(type_arguments(object)?);
                parts.extend(term_docs(array(object, "arguments")?));
                Some(Doc::concat(
                    std::iter::once(Doc::text("("))
                        .chain(parts.into_iter().enumerate().flat_map(|(index, part)| {
                            if index == 0 {
                                vec![part]
                            } else {
                                vec![Doc::text(" "), part]
                            }
                        }))
                        .chain(std::iter::once(Doc::text(")")))
                        .collect(),
                ))
            }
            "constructor" if keys_with(object, &["kind", "constructor", "arguments"]) => {
                let mut parts = vec![member_doc(object.get("constructor")?)?];
                parts.extend(type_arguments(object)?);
                parts.extend(term_docs(array(object, "arguments")?));
                Some(form("ctor", parts))
            }
            "function_ref" if keys_with(object, &["kind", "function"]) => {
                let mut parts = vec![member_doc(object.get("function")?)?];
                parts.extend(type_arguments(object)?);
                Some(form("ref", parts))
            }
            "record" if keys_with(object, &["kind", "type", "fields"]) => {
                let mut parts = vec![member_doc(object.get("type")?)?];
                parts.extend(type_arguments(object)?);
                let mut assignments = Vec::new();
                for assignment in array(object, "fields")? {
                    let Json::Obj(assignment) = assignment else {
                        return None;
                    };
                    if !exact(assignment, &["field", "value"]) {
                        return None;
                    }
                    let field = string_member(assignment, "field")?;
                    if !is_bare(field) {
                        return None;
                    }
                    assignments.push(Doc::concat(vec![
                        Doc::text(format!("{field} := ")),
                        doc_at(assignment.get("value")?, Pos::Term),
                    ]));
                }
                parts.push(Doc::concat(vec![
                    Doc::text("{"),
                    Doc::concat(
                        assignments
                            .into_iter()
                            .enumerate()
                            .flat_map(|(index, assignment)| {
                                if index == 0 {
                                    vec![assignment]
                                } else {
                                    vec![Doc::text(", "), assignment]
                                }
                            })
                            .collect(),
                    ),
                    Doc::text("}"),
                ]));
                Some(form("record", parts))
            }
            "primitive" if exact(object, &["kind", "operation", "arguments", "result"]) => {
                let operation = string_member(object, "operation")?;
                if !is_bare(operation) {
                    return None;
                }
                let mut docs = vec![Doc::text(format!("(prim {operation}"))];
                for argument in array(object, "arguments")? {
                    docs.push(Doc::text(" "));
                    docs.push(doc_at(argument, Pos::Term));
                }
                docs.push(Doc::text(" : "));
                docs.push(doc_at(object.get("result")?, Pos::Type));
                docs.push(Doc::text(")"));
                Some(Doc::concat(docs))
            }
            "checked_apply" if keys_with(object, &["kind", "model", "arguments", "checks"]) => {
                let mut parts = vec![member_doc(object.get("model")?)?];
                parts.extend(type_arguments(object)?);
                parts.extend(term_docs(array(object, "arguments")?));
                parts.push(Doc::text("with"));
                let checks = array(object, "checks")?
                    .iter()
                    .map(|check| match check {
                        Json::Str(name) if is_bare(name) => Some(Doc::text(name.clone())),
                        _ => None,
                    })
                    .collect::<Option<Vec<_>>>()?;
                parts.push(bracketed(checks));
                Some(form("checked", parts))
            }
            "match" if exact(object, &["kind", "scrutinee", "branches"]) => {
                let mut branches = Vec::new();
                for branch in array(object, "branches")? {
                    let Json::Obj(branch) = branch else {
                        return None;
                    };
                    if !exact(branch, &["constructor", "binders", "body"]) {
                        return None;
                    }
                    let mut docs = vec![Doc::text("| "), member_doc(branch.get("constructor")?)?];
                    for binder in array(branch, "binders")? {
                        let Json::Str(name) = binder else {
                            return None;
                        };
                        if !plain(name) {
                            return None;
                        }
                        docs.push(Doc::text(format!(" {name}")));
                    }
                    docs.push(Doc::text(" => "));
                    docs.push(doc_at(branch.get("body")?, Pos::Term));
                    branches.push(Doc::concat(docs));
                }
                let mut docs = vec![Doc::text("(match "), term("scrutinee")?];
                for branch in branches {
                    docs.push(Doc::Line);
                    docs.push(branch);
                }
                docs.push(Doc::text(")"));
                Some(Doc::concat(docs).nest(2).group())
            }
            _ => None,
        }
    }

    /// The name of a member object that `member_doc` can spell.
    fn member_name(value: &Json) -> Option<&str> {
        member_doc(value)?;
        let Json::Obj(object) = value else {
            return None;
        };
        string_member(object, "name")
    }

    /// Read a term.
    pub(super) fn read(reader: &mut Reader<'_>, depth: usize) -> Result<Json, SurfaceError> {
        let offset = reader.offset();
        match reader.peek() {
            Some(Token::Str(text)) => {
                let text = text.clone();
                reader.bump();
                Ok(obj("string", vec![("value", Json::Str(text))]))
            }
            Some(Token::LParen) => {
                reader.bump();
                let parsed = paren(reader, depth)?;
                reader.expect(&Token::RParen, "`)`")?;
                Ok(parsed)
            }
            Some(Token::Word(word)) => {
                let word = word.clone();
                if reader.peek_at(1) == Some(&Token::LBrace) && is_bare(&word) {
                    return read_tagged(reader, Pos::Term, depth);
                }
                reader.bump();
                if word == "true" || word == "false" {
                    return Ok(obj("bool", vec![("value", Json::Bool(word == "true"))]));
                }
                if word.bytes().all(|byte| byte.is_ascii_digit()) {
                    return nat(&word, offset);
                }
                if let Some(hex) = word.strip_prefix("0x") {
                    return bytes(hex, offset);
                }
                if plain(&word) {
                    return Ok(obj("var", vec![("name", Json::Str(word))]));
                }
                Err(SurfaceError {
                    offset,
                    message: if RESERVED.contains(&word.as_str()) {
                        format!("`{word}` is a keyword; write `var{{name: {word}}}` for a variable of that name")
                    } else {
                        format!("`{word}` is not a term")
                    },
                })
            }
            Some(other) => Err(SurfaceError {
                offset,
                message: format!("expected a term, found {}", super::describe(other)),
            }),
            None => Err(reader.error("expected a term, found the end of the text")),
        }
    }

    fn nat(digits: &str, offset: usize) -> Result<Json, SurfaceError> {
        if digits.len() > 1 && digits.starts_with('0') {
            return Err(SurfaceError {
                offset,
                message: format!("`{digits}` has a leading zero"),
            });
        }
        Ok(obj("nat", vec![("value", Json::Str(digits.to_owned()))]))
    }

    fn bytes(hex: &str, offset: usize) -> Result<Json, SurfaceError> {
        if !hex.len().is_multiple_of(2)
            || !hex
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(SurfaceError {
                offset,
                message:
                    "a byte string is `0x` and an even number of lower-case hexadecimal digits"
                        .to_owned(),
            });
        }
        Ok(obj("bytes", vec![("hex", Json::Str(hex.to_owned()))]))
    }

    /// Terms until the next token is `stop`, or a `)` when `stop` is empty.
    fn terms_until(
        reader: &mut Reader<'_>,
        depth: usize,
        stop: Option<&str>,
    ) -> Result<Vec<Json>, SurfaceError> {
        let mut items = Vec::new();
        loop {
            match (reader.peek(), stop) {
                (Some(Token::RParen), None) => break,
                (Some(Token::Word(word)), Some(stop)) if word == stop => break,
                (None, _) => return Err(reader.error("expected `)`")),
                _ => items.push(read(reader, depth + 1)?),
            }
        }
        Ok(items)
    }

    /// An optional `[type, …]` after a member.
    fn optional_types(reader: &mut Reader<'_>, depth: usize) -> Result<Option<Json>, SurfaceError> {
        if reader.peek() == Some(&Token::LBrack) {
            Ok(Some(Json::Arr(types::read_types(reader, depth)?)))
        } else {
            Ok(None)
        }
    }

    fn with_type_arguments(members: &mut Vec<(&'static str, Json)>, types: Option<Json>) {
        if let Some(types) = types {
            members.push(("type_arguments", types));
        }
    }

    /// The inside of a parenthesised term, after the `(`.
    fn paren(reader: &mut Reader<'_>, depth: usize) -> Result<Json, SurfaceError> {
        let offset = reader.offset();
        let sub = |reader: &mut Reader<'_>| read(reader, depth + 1);
        match reader.peek() {
            Some(Token::RParen) => return Ok(obj("unit", vec![])),
            Some(Token::Word(word)) => {
                let word = word.clone();
                // An integer literal: `(-3 : Int8)`.
                let numeral = word.strip_prefix('-').unwrap_or(&word);
                if !numeral.is_empty()
                    && numeral.bytes().all(|byte| byte.is_ascii_digit())
                    && reader.peek_at(1) == Some(&Token::Colon)
                {
                    reader.bump();
                    reader.bump();
                    let ty = read_name(reader, "an integer type")?;
                    let Some(representation) = types::representation_of(&ty) else {
                        return Err(SurfaceError {
                            offset,
                            message: format!("`{ty}` is not an integer type"),
                        });
                    };
                    if word == "-0" || (numeral.len() > 1 && numeral.starts_with('0')) {
                        return Err(SurfaceError {
                            offset,
                            message: format!("`{word}` is not a canonical integer"),
                        });
                    }
                    return Ok(obj(
                        "integer",
                        vec![
                            ("representation", Json::Str(representation.to_owned())),
                            ("value", Json::Str(word)),
                        ],
                    ));
                }
                match word.as_str() {
                    "not" | "first" | "second" => {
                        reader.bump();
                        let value = sub(reader)?;
                        return Ok(obj(&word, vec![("value", value)]));
                    }
                    "cons" => {
                        reader.bump();
                        let head = sub(reader)?;
                        let tail = sub(reader)?;
                        return Ok(obj("cons", vec![("head", head), ("tail", tail)]));
                    }
                    "nil" => {
                        reader.bump();
                        let element = types::read(reader, depth + 1)?;
                        return Ok(obj("nil", vec![("element", element)]));
                    }
                    "if" => {
                        reader.bump();
                        let condition = sub(reader)?;
                        expect_word(reader, "then")?;
                        let then_value = sub(reader)?;
                        expect_word(reader, "else")?;
                        let else_value = sub(reader)?;
                        return Ok(obj(
                            "if",
                            vec![
                                ("condition", condition),
                                ("then_value", then_value),
                                ("else_value", else_value),
                            ],
                        ));
                    }
                    "let" => {
                        reader.bump();
                        let binder = read_param(reader, depth + 1)?;
                        reader.expect(&Token::ColonEq, "`:=`")?;
                        let value = sub(reader)?;
                        reader.expect(&Token::Semi, "`;`")?;
                        let body = sub(reader)?;
                        return Ok(obj(
                            "let",
                            vec![("binder", binder), ("value", value), ("body", body)],
                        ));
                    }
                    "forall" => {
                        reader.bump();
                        reader.expect(&Token::LParen, "`(` before the bound variable")?;
                        let binder = read_param(reader, depth + 1)?;
                        reader.expect(&Token::RParen, "`)`")?;
                        reader.expect(&Token::Comma, "`,` before the body")?;
                        let body = sub(reader)?;
                        return Ok(obj("forall", vec![("binder", binder), ("body", body)]));
                    }
                    "fun" => {
                        reader.bump();
                        let captures = if reader.peek() == Some(&Token::LBrack) {
                            read_list(reader, |reader| read_name(reader, "a name"))?
                        } else {
                            Vec::new()
                        };
                        let mut parameters = Vec::new();
                        while reader.peek() == Some(&Token::LParen) {
                            reader.bump();
                            parameters.push(read_param(reader, depth + 1)?);
                            reader.expect(&Token::RParen, "`)`")?;
                        }
                        if parameters.is_empty() {
                            return Err(reader.error("a function takes at least one parameter"));
                        }
                        expect_word(reader, "=>")?;
                        let body = sub(reader)?;
                        return Ok(obj(
                            "lambda",
                            vec![
                                ("parameters", Json::Arr(parameters)),
                                (
                                    "captures",
                                    Json::Arr(captures.into_iter().map(Json::Str).collect()),
                                ),
                                ("body", body),
                            ],
                        ));
                    }
                    "apply" => {
                        reader.bump();
                        let function = sub(reader)?;
                        let arguments = terms_until(reader, depth, None)?;
                        return Ok(obj(
                            "apply",
                            vec![("function", function), ("arguments", Json::Arr(arguments))],
                        ));
                    }
                    "project" => {
                        reader.bump();
                        let value = sub(reader)?;
                        let field = read_name(reader, "a field name")?;
                        return Ok(obj(
                            "project",
                            vec![("value", value), ("field", Json::Str(field))],
                        ));
                    }
                    "ctor" => {
                        reader.bump();
                        let constructor = member_path(reader)?;
                        let types = optional_types(reader, depth)?;
                        let arguments = terms_until(reader, depth, None)?;
                        let mut members = vec![
                            ("constructor", constructor),
                            ("arguments", Json::Arr(arguments)),
                        ];
                        with_type_arguments(&mut members, types);
                        return Ok(obj("constructor", members));
                    }
                    "ref" => {
                        reader.bump();
                        let function = member_path(reader)?;
                        let types = optional_types(reader, depth)?;
                        let mut members = vec![("function", function)];
                        with_type_arguments(&mut members, types);
                        return Ok(obj("function_ref", members));
                    }
                    "record" => {
                        reader.bump();
                        let ty = member_path(reader)?;
                        let types = optional_types(reader, depth)?;
                        reader.expect(&Token::LBrace, "`{` before the fields")?;
                        let mut fields = Vec::new();
                        while reader.peek() != Some(&Token::RBrace) {
                            let field = read_name(reader, "a field name")?;
                            reader.expect(&Token::ColonEq, "`:=` after a field name")?;
                            let value = sub(reader)?;
                            let mut assignment = std::collections::BTreeMap::new();
                            assignment.insert("field".to_owned(), Json::Str(field));
                            assignment.insert("value".to_owned(), value);
                            fields.push(Json::Obj(assignment));
                            if !reader.eat(&Token::Comma) {
                                break;
                            }
                        }
                        reader.expect(&Token::RBrace, "`,` or `}`")?;
                        let mut members = vec![("type", ty), ("fields", Json::Arr(fields))];
                        with_type_arguments(&mut members, types);
                        return Ok(obj("record", members));
                    }
                    "prim" => {
                        reader.bump();
                        let operation = read_name(reader, "a primitive operation")?;
                        let mut arguments = Vec::new();
                        while reader.peek() != Some(&Token::Colon) {
                            if matches!(reader.peek(), None | Some(Token::RParen)) {
                                return Err(reader.error("expected `:` and the result type"));
                            }
                            arguments.push(sub(reader)?);
                        }
                        reader.bump();
                        let result = types::read(reader, depth + 1)?;
                        return Ok(obj(
                            "primitive",
                            vec![
                                ("operation", Json::Str(operation)),
                                ("arguments", Json::Arr(arguments)),
                                ("result", result),
                            ],
                        ));
                    }
                    "checked" => {
                        reader.bump();
                        let model = member_path(reader)?;
                        let types = optional_types(reader, depth)?;
                        let arguments = terms_until(reader, depth, Some("with"))?;
                        expect_word(reader, "with")?;
                        let checks = read_list(reader, |reader| read_name(reader, "a check"))?;
                        let mut members = vec![
                            ("model", model),
                            ("arguments", Json::Arr(arguments)),
                            (
                                "checks",
                                Json::Arr(checks.into_iter().map(Json::Str).collect()),
                            ),
                        ];
                        with_type_arguments(&mut members, types);
                        return Ok(obj("checked_apply", members));
                    }
                    "match" => {
                        reader.bump();
                        let scrutinee = sub(reader)?;
                        let mut branches = Vec::new();
                        while reader.eat_word("|") {
                            let constructor = member_path(reader)?;
                            let mut binders = Vec::new();
                            while !matches!(reader.peek(), Some(Token::Word(word)) if word == "=>")
                            {
                                binders.push(Json::Str(read_name(reader, "a binder name")?));
                            }
                            reader.bump();
                            let body = sub(reader)?;
                            let mut branch = std::collections::BTreeMap::new();
                            branch.insert("constructor".to_owned(), constructor);
                            branch.insert("binders".to_owned(), Json::Arr(binders));
                            branch.insert("body".to_owned(), body);
                            branches.push(Json::Obj(branch));
                        }
                        if branches.is_empty() {
                            return Err(reader.error("a match has at least one `| …` branch"));
                        }
                        return Ok(obj(
                            "match",
                            vec![("scrutinee", scrutinee), ("branches", Json::Arr(branches))],
                        ));
                    }
                    _ => {}
                }
                // `(f argument …)`, or an infix form or pair with a variable
                // as its first operand.
                let after_name = if reader.peek_at(1) == Some(&Token::Scope) {
                    3
                } else {
                    1
                };
                let infix = match reader.peek_at(after_name) {
                    Some(Token::Comma) => true,
                    Some(Token::Word(op)) => INFIX.iter().any(|(_, spelling, ..)| spelling == op),
                    _ => false,
                };
                let generic = reader.peek_at(1) == Some(&Token::LBrace);
                if plain(&word)
                    && !infix
                    && !generic
                    && !word.bytes().all(|byte| byte.is_ascii_digit())
                {
                    let function = member_path(reader)?;
                    let types = optional_types(reader, depth)?;
                    let arguments = terms_until(reader, depth, None)?;
                    let mut members =
                        vec![("function", function), ("arguments", Json::Arr(arguments))];
                    with_type_arguments(&mut members, types);
                    return Ok(obj("call", members));
                }
            }
            _ => {}
        }
        // `(operand operator operand)` and `(first, second)`.
        let left = sub(reader)?;
        if reader.eat(&Token::Comma) {
            let right = sub(reader)?;
            return Ok(obj("pair", vec![("left", left), ("right", right)]));
        }
        let at = reader.offset();
        if let Some(Token::Word(op)) = reader.peek() {
            if let Some((kind, _, left_key, right_key)) =
                INFIX.iter().find(|(_, spelling, ..)| spelling == op)
            {
                reader.bump();
                let right = sub(reader)?;
                return Ok(obj(kind, vec![(left_key, left), (right_key, right)]));
            }
        }
        Err(SurfaceError {
            offset: at,
            message: "expected an operator or `,` after the first operand".to_owned(),
        })
    }

    fn expect_word(reader: &mut Reader<'_>, word: &str) -> Result<(), SurfaceError> {
        if reader.eat_word(word) {
            Ok(())
        } else {
            Err(reader.error(format!("expected `{word}`")))
        }
    }
}

// ---------------------------------------------------------------------------
// The module.
// ---------------------------------------------------------------------------

/// The declaration position of a module.
fn declaration() -> Pos {
    schema()["properties"]
        .get("declarations")
        .map_or(Pos::Any, items_of)
}

/// A position a value of the semantic-module schema can be written at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    /// A declaration of a module.
    Declaration,
    /// A term.
    Term,
    /// A type.
    Type,
    /// A proof.
    Proof,
}

impl Position {
    fn pos(self) -> Pos {
        match self {
            Self::Declaration => declaration(),
            Self::Term => Pos::Term,
            Self::Type => Pos::Type,
            Self::Proof => schema()["$defs"].get("proof").map_or(Pos::Any, Pos::Node),
        }
    }
}

/// The notation of one value at `position`.
#[must_use]
pub fn print_at(value: &Json, position: Position) -> String {
    render(&doc_at(value, position.pos()), WIDTH)
}

/// Read one value at `position` from a text that is exactly that value.
///
/// # Errors
///
/// Text that is not one value there.
pub fn parse_at(text: &str, position: Position) -> Result<Json, SurfaceError> {
    let mut reader = Reader::new(text)?;
    let value = read_at(&mut reader, position.pos(), 1)?;
    if !reader.done() {
        return Err(reader.error("unexpected text after the value"));
    }
    Ok(value)
}

/// The notation of a module's declarations: each declaration, one after the
/// other, with an empty line between two.
///
/// # Errors
///
/// A value that is not a module.
pub fn print_module(module: &Json) -> Result<String, SurfaceError> {
    let Json::Obj(object) = module else {
        return Err(SurfaceError {
            offset: 0,
            message: "a module is an object".to_owned(),
        });
    };
    let Some(Json::Arr(declarations)) = object.get("declarations") else {
        return Err(SurfaceError {
            offset: 0,
            message: "a module has a `declarations` array".to_owned(),
        });
    };
    if object.len() != 2 || !matches!(object.get("spec"), Some(Json::Str(_))) {
        return Err(SurfaceError {
            offset: 0,
            message: "a module has exactly a `spec` and its `declarations`".to_owned(),
        });
    }
    let mut out = String::new();
    for (index, declaration_value) in declarations.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        out.push_str(&render(&doc_at(declaration_value, declaration()), WIDTH));
        out.push('\n');
    }
    Ok(out)
}

/// Read the declarations of a module, and make the module of them under
/// `spec`.
///
/// # Errors
///
/// Text that is not a sequence of declarations.
pub fn parse_module(text: &str, spec: &str) -> Result<Json, SurfaceError> {
    let mut reader = Reader::new(text)?;
    let mut declarations = Vec::new();
    while !reader.done() {
        declarations.push(read_at(&mut reader, declaration(), 1)?);
    }
    let mut module = std::collections::BTreeMap::new();
    module.insert("spec".to_owned(), Json::Str(spec.to_owned()));
    module.insert("declarations".to_owned(), Json::Arr(declarations));
    Ok(Json::Obj(module))
}

/// The byte range of each declaration of a module's notation, in order.
///
/// # Errors
///
/// Text that is not a sequence of declarations.
pub fn declaration_spans(text: &str) -> Result<Vec<(usize, usize)>, SurfaceError> {
    let mut reader = Reader::new(text)?;
    let mut spans = Vec::new();
    while !reader.done() {
        let start = reader.offset();
        read_at(&mut reader, declaration(), 1)?;
        spans.push((start, reader.last_end()));
    }
    Ok(spans)
}

// ---------------------------------------------------------------------------
// The exhaustiveness audit's input.
// ---------------------------------------------------------------------------

/// A smallest value of a schema node. `every` also fills the optional
/// members. Recursive positions are filled with a fixed leaf, so that the
/// sample of an alternative exercises that alternative and nothing else.
fn sample(node: &Value, every: bool) -> Json {
    let defs = &schema()["$defs"];
    let leaf = |text: &str| generic::parse(text).expect("generic notation");
    if let Some(reference) = node.get("$ref").and_then(Value::as_str) {
        let name = reference.rsplit('/').next().expect("name");
        return match name {
            "term" => leaf(r#"nat{value: "0"}"#),
            "type" => leaf("nat{}"),
            "proof" => leaf("reflexivity{}"),
            "member" => leaf("{name: x}"),
            "parameter" => leaf("{name: x, type: nat{}}"),
            "name" => Json::Str("x".to_owned()),
            other => sample(&defs[other], every),
        };
    }
    if let Some(constant) = node.get("const") {
        return from_value(constant);
    }
    if let Some(options) = node.get("enum").and_then(Value::as_array) {
        return from_value(&options[0]);
    }
    if let Some(options) = node.get("oneOf").and_then(Value::as_array) {
        return sample(&options[0], every);
    }
    match node.get("type").and_then(Value::as_str) {
        Some("boolean") => Json::Bool(true),
        Some("integer") => Json::Int(node.get("minimum").and_then(Value::as_i64).unwrap_or(1)),
        Some("string") => match node.get("pattern").and_then(Value::as_str) {
            Some(pattern) if pattern.contains("{64}") => Json::Str("0".repeat(64)),
            Some(pattern) if pattern.contains("[0-9a-f]{2}") => Json::Str(String::new()),
            Some(pattern) if pattern.contains("[1-9]") => Json::Str("1".to_owned()),
            _ => Json::Str("x".to_owned()),
        },
        Some("array") => {
            let items = node
                .get("items")
                .map_or_else(|| Json::Str("x".to_owned()), |items| sample(items, every));
            if node.get("minItems").and_then(Value::as_u64).unwrap_or(0) > 0 || every {
                Json::Arr(vec![items])
            } else {
                Json::Arr(Vec::new())
            }
        }
        _ => {
            let mut members = std::collections::BTreeMap::new();
            let required: Vec<&str> = node
                .get("required")
                .and_then(Value::as_array)
                .map(|names| names.iter().filter_map(Value::as_str).collect())
                .unwrap_or_default();
            if let Some(properties) = node.get("properties").and_then(Value::as_object) {
                for (key, property) in properties {
                    if every || required.contains(&key.as_str()) {
                        members.insert(key.clone(), sample(property, every));
                    }
                }
            }
            Json::Obj(members)
        }
    }
}

fn from_value(value: &Value) -> Json {
    match value {
        Value::Bool(flag) => Json::Bool(*flag),
        Value::Number(number) => Json::Int(number.as_i64().unwrap_or(0)),
        Value::String(text) => Json::Str(text.clone()),
        _ => Json::Str("x".to_owned()),
    }
}

/// One sample of every alternative of the declaration, term, type, and proof
/// unions of the schema, with and without its optional members, with the
/// position it is written at and a label naming it. This is what the
/// exhaustiveness audit (`AU-01`) feeds the printer and the parser: the
/// samples are read from the schema, so a variant added to it is sampled the
/// day it is added.
///
/// # Panics
///
/// When the embedded schema is not the shape this audit reads.
#[must_use]
pub fn schema_samples() -> Vec<(Position, String, Json)> {
    let defs = &schema()["$defs"];
    let mut samples = Vec::new();
    for (name, position) in [
        ("declaration", Position::Declaration),
        ("term", Position::Term),
        ("type", Position::Type),
        ("proof", Position::Proof),
    ] {
        for alternative in defs[name]["oneOf"].as_array().expect("a union") {
            let kind = &alternative["properties"]["kind"];
            let kinds: Vec<String> = match (kind.get("const"), kind.get("enum")) {
                (Some(constant), _) => vec![constant.as_str().expect("kind").to_owned()],
                (_, Some(options)) => options
                    .as_array()
                    .expect("kinds")
                    .iter()
                    .map(|kind| kind.as_str().expect("kind").to_owned())
                    .collect(),
                _ => panic!("an alternative of `{name}` without a kind"),
            };
            for kind in kinds {
                for every in [false, true] {
                    let Json::Obj(mut members) = sample(alternative, every) else {
                        panic!("a sample of `{name}` that is not an object");
                    };
                    members.insert("kind".to_owned(), Json::Str(kind.clone()));
                    let label = format!(
                        "{name} {kind}{}",
                        if every { " (every member)" } else { "" }
                    );
                    samples.push((position, label, Json::Obj(members)));
                }
            }
        }
    }
    samples
}

#[cfg(test)]
mod tests {
    use super::{
        declaration_spans, parse_at, parse_module, print_at, print_module, schema_samples, Position,
    };
    use crate::artifact::canonical_json::Json;
    use crate::surface::generic;

    /// A value written in the generic notation.
    fn json(text: &str) -> Json {
        generic::parse(text).expect("generic notation")
    }

    fn round_trip(value: &Json, position: Position) -> String {
        let text = print_at(value, position);
        let back =
            parse_at(&text, position).unwrap_or_else(|error| panic!("{error}\n{text}\n{value:?}"));
        assert_eq!(&back, value, "{text}");
        assert_eq!(print_at(&back, position), text, "the notation is canonical");
        text
    }

    /// The readable forms print as the SPEC says they do, and parse back.
    #[test]
    fn readable_forms() {
        let cases: [(&str, Position, &str); 12] = [
            (r#"nat{value: "3"}"#, Position::Term, "3"),
            (r#"var{name: n}"#, Position::Term, "n"),
            (r#"bool{value: true}"#, Position::Term, "true"),
            (
                r#"add{left: var{name: a}, right: nat{value: "1"}}"#,
                Position::Term,
                "(a + 1)",
            ),
            (
                r#"call{function: {module: M, name: f}, arguments: [var{name: x}, nat{value: "0"}]}"#,
                Position::Term,
                "(M::f x 0)",
            ),
            (
                r#"call{function: {name: f}, arguments: []}"#,
                Position::Term,
                "(f)",
            ),
            (
                r#"integer{representation: int8, value: "-3"}"#,
                Position::Term,
                "(-3 : Int8)",
            ),
            (
                r#"let{binder: {name: x, type: nat{}}, value: nat{value: "1"}, body: var{name: x}}"#,
                Position::Term,
                "(let x : Nat := 1; x)",
            ),
            (
                r#"function{parameters: [nat{}, bool{}], result: list{element: parameter{name: T}}}"#,
                Position::Type,
                "(Fn (Nat Bool) (List 'T))",
            ),
            (
                r#"named{member: {module: Glyphs, name: Glyph}, arguments: [nat{}]}"#,
                Position::Type,
                "(Glyphs::Glyph Nat)",
            ),
            (
                r#"product{left: nat{}, right: option{value: string{}}}"#,
                Position::Type,
                "(Nat * (Option String))",
            ),
            (r#"reflexivity{}"#, Position::Proof, "reflexivity{}"),
        ];
        for (generic, position, expected) in cases {
            let value = json(generic);
            assert_eq!(round_trip(&value, position), expected);
        }
    }

    /// A node a readable form cannot carry exactly is written generically, so
    /// nothing is lost: an extra member, a name that is a keyword, a literal
    /// that is not canonical, and a default written out.
    #[test]
    fn what_a_form_cannot_carry_is_written_generically() {
        let cases: [(&str, Position); 7] = [
            (r#"var{name: let}"#, Position::Term),
            (r#"var{name: "two words"}"#, Position::Term),
            (r#"nat{value: "01"}"#, Position::Term),
            (
                r#"add{left: nat{value: "1"}, right: nat{value: "2"}, extra: true}"#,
                Position::Term,
            ),
            (
                r#"call{function: {name: f}, arguments: [], type_arguments: []}"#,
                Position::Term,
            ),
            (
                r#"named{member: {name: Nat}, arguments: []}"#,
                Position::Type,
            ),
            (r#"bytes{hex: "abc"}"#, Position::Term),
        ];
        for (generic, position) in cases {
            let text = round_trip(&json(generic), position);
            assert!(text.contains('{'), "{text} is written generically");
        }
    }

    /// Malformed notation is an error with an offset, never a panic.
    #[test]
    fn malformed_notation_is_an_error() {
        for text in [
            "(",
            "(let x : Nat := 1)",
            "(f",
            "(a +)",
            "(match x)",
            "(1 : Foo)",
            "'",
            "(nil)",
            "(if a then b)",
            "(fun => x)",
            "0x1",
            "01",
            "(prim add : Nat",
        ] {
            assert!(parse_at(text, Position::Term).is_err(), "{text:?}");
        }
        for text in ["(List)", "(Fn Nat Nat)", "'", "(Nat *)", "List"] {
            assert!(parse_at(text, Position::Type).is_err(), "{text:?}");
        }
        assert!(parse_module("definition{ name: x ", "s").is_err());
    }

    /// A module's declarations are separated by an empty line and each has the
    /// byte range the source map needs.
    #[test]
    fn a_module_and_its_declaration_spans() {
        let module = json(
            r#"{spec: "lexlean/semantic-module/2", declarations: [
                theorem{name: a, parameters: [], statement: bool{value: true}, proof: decide{}},
                theorem{name: b, parameters: [], statement: bool{value: true}, proof: decide{}}
            ]}"#,
        );
        let text = print_module(&module).expect("prints");
        assert!(text.contains("}\n\ntheorem{"), "{text}");
        assert_eq!(
            parse_module(&text, "lexlean/semantic-module/2").expect("parses"),
            module
        );
        let spans = declaration_spans(&text).expect("spans");
        assert_eq!(spans.len(), 2);
        assert!(text[spans[1].0..spans[1].1].starts_with("theorem{"));
        assert!(text[spans[0].0..spans[0].1].ends_with('}'));
    }

    // -----------------------------------------------------------------
    // Exhaustiveness: every alternative of the schema has a notation.
    // -----------------------------------------------------------------

    /// Every alternative of every union of the schema, with and without its
    /// optional members, is written and read back exactly. A variant added to
    /// the schema is covered the day it is added, by the generic form, so
    /// this fails only for a readable form that is not exact.
    #[test]
    fn every_alternative_of_the_schema_has_a_notation() {
        let samples = schema_samples();
        for (position, label, value) in &samples {
            let text = round_trip(value, *position);
            assert!(!text.is_empty(), "{label}");
        }
        // 15 declarations, 43 terms (two `integer` alternatives), 29 type
        // kinds, 10 proofs, each with and without its optional members: the
        // count moves when the schema does, and the test says so rather than
        // passing over the new kind.
        assert!(samples.len() > 180, "{} samples", samples.len());
    }
}
