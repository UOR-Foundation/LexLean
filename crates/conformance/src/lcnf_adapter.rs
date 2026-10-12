//! The extraction adapter of SPEC.md §22.10 as a compiler-tooling module
//! (§17.20): the program that reads Lean's compiler front end for a verified
//! project and reports facts, written as data over a typed registry of the
//! pinned Lean's interface, and generated into Lean by the normal compiler.
//!
//! This file is the authoring form of that program. The committed source,
//! `language/lcnf-1.2/tooling/src/Extract.lex.tex`, is what the compiler
//! embeds; `cargo xtask check-calculus` requires it to equal what this file
//! renders over `tests/lcnf/registry.json`, so nothing here is authoritative
//! once rendered and an edit of the committed source is undone by the next
//! regeneration.

use serde_json::Value as Json;

use crate::tl::{
    branch, call, chr, construct, field, function, if_then, index, lambda, let_in, list, matching,
    name, nat, string, ty, ty_app, var, Flags, PARTIAL, TRAVERSAL,
};

/// The namespace of the adapter's own definitions; the registry check finds
/// them by it.
pub const NAMESPACE: &str = "LexLeanExtract";

pub(crate) fn own(local: &str) -> String {
    format!("{NAMESPACE}.{local}")
}

pub(crate) fn lcnf(local: &str) -> String {
    format!("Lean.Compiler.LCNF.{local}")
}

// --- types -----------------------------------------------------------------

pub(crate) fn t_string() -> Json {
    ty("String")
}
pub(crate) fn t_nat() -> Json {
    ty("Nat")
}
pub(crate) fn t_bool() -> Json {
    ty("Bool")
}
pub(crate) fn t_char() -> Json {
    ty("Char")
}
pub(crate) fn t_unit() -> Json {
    ty("Unit")
}
pub(crate) fn t_name() -> Json {
    ty("Lean.Name")
}
pub(crate) fn t_expr() -> Json {
    ty("Lean.Expr")
}
pub(crate) fn t_env() -> Json {
    ty("Lean.Environment")
}
pub(crate) fn t_list(element: Json) -> Json {
    ty_app("List", vec![element])
}
pub(crate) fn t_array(element: Json) -> Json {
    ty_app("Array", vec![element])
}
pub(crate) fn t_prod(left: Json, right: Json) -> Json {
    ty_app("Prod", vec![left, right])
}
pub(crate) fn t_pair() -> Json {
    t_prod(t_string(), t_string())
}
pub(crate) fn t_set() -> Json {
    ty_app("Std.HashSet", vec![t_name()])
}
pub(crate) fn pure_index() -> Json {
    index(&lcnf("Purity.pure"))
}
pub(crate) fn t_pure(local: &str) -> Json {
    ty_app(&lcnf(local), vec![pure_index()])
}
pub(crate) fn t_code() -> Json {
    t_pure("Code")
}
pub(crate) fn t_core(value: Json) -> Json {
    ty_app("Lean.Core.CoreM", vec![value])
}
pub(crate) fn t_command(value: Json) -> Json {
    ty_app("Lean.Elab.Command.CommandElabM", vec![value])
}
pub(crate) fn m_core() -> Json {
    ty("Lean.Core.CoreM")
}
pub(crate) fn m_command() -> Json {
    ty("Lean.Elab.Command.CommandElabM")
}
pub(crate) fn m_term() -> Json {
    ty("Lean.Elab.Term.TermElabM")
}

// --- terms -----------------------------------------------------------------

pub(crate) fn s(text: &str) -> Json {
    string(text)
}

pub(crate) fn append(left: Json, right: Json) -> Json {
    call("String.append", vec![], vec![left, right])
}

/// The concatenation of strings, nested to the right.
pub(crate) fn cat(mut parts: Vec<Json>) -> Json {
    let mut result = parts.pop().expect("a part");
    while let Some(part) = parts.pop() {
        result = append(part, result);
    }
    result
}

pub(crate) fn pair(key: &str, value: Json) -> Json {
    construct(
        "Prod.mk",
        vec![t_string(), t_string()],
        vec![string(key), value],
    )
}

pub(crate) fn str_of(text: Json) -> Json {
    call(&own("str"), vec![], vec![text])
}

pub(crate) fn name_of(value: Json) -> Json {
    call(&own("name"), vec![], vec![value])
}

pub(crate) fn arr_list(items: Json) -> Json {
    call(&own("arr"), vec![], vec![items])
}

pub(crate) fn bool_of(value: Json) -> Json {
    call(&own("bool"), vec![], vec![value])
}

pub(crate) fn fvar_of(value: Json) -> Json {
    call(&own("fvar"), vec![], vec![value])
}

/// An object of a record: each value is already text.
pub(crate) fn obj(fields: Vec<(&str, Json)>) -> Json {
    call(
        &own("obj"),
        vec![],
        vec![list(
            t_pair(),
            fields
                .into_iter()
                .map(|(key, value)| pair(key, value))
                .collect(),
        )],
    )
}

pub(crate) fn kind_of(kind: &str) -> (&'static str, Json) {
    ("kind", str_of(string(kind)))
}

pub(crate) fn map_list(from: Json, to: Json, parameter: &str, body: Json, items: Json) -> Json {
    call(
        "List.map",
        vec![from.clone(), to],
        vec![lambda(&[(parameter, from)], body), items],
    )
}

pub(crate) fn array_to_list(element: Json, items: Json) -> Json {
    call("Array.toList", vec![element], vec![items])
}

pub(crate) fn to_text(value: Json, ty: Json) -> Json {
    call("ToString.toString", vec![ty], vec![value])
}

pub(crate) fn eq(ty: Json, left: Json, right: Json) -> Json {
    call("BEq.beq", vec![ty], vec![left, right])
}

pub(crate) fn and(left: Json, right: Json) -> Json {
    call("Bool.and", vec![], vec![left, right])
}

pub(crate) fn or(left: Json, right: Json) -> Json {
    call("Bool.or", vec![], vec![left, right])
}

pub(crate) fn not(value: Json) -> Json {
    call("Bool.not", vec![], vec![value])
}

pub(crate) fn add(left: Json, right: Json) -> Json {
    call("Nat.add", vec![], vec![left, right])
}

pub(crate) fn lt(left: Json, right: Json) -> Json {
    call("Nat.blt", vec![], vec![left, right])
}

pub(crate) fn nil(element: Json) -> Json {
    construct("List.nil", vec![element], vec![])
}

pub(crate) fn cons(element: Json, head: Json, tail: Json) -> Json {
    construct("List.cons", vec![element], vec![head, tail])
}

/// `Bind.bind` in a monad: run `action`, name its result, continue.
pub(crate) fn bind(
    monad: &Json,
    result: Json,
    next: Json,
    action: Json,
    binder: &str,
    rest: Json,
) -> Json {
    call(
        "Bind.bind",
        vec![monad.clone(), result.clone(), next],
        vec![action, lambda(&[(binder, result)], rest)],
    )
}

pub(crate) fn pure_in(monad: &Json, value_ty: Json, value: Json) -> Json {
    call("Pure.pure", vec![monad.clone(), value_ty], vec![value])
}

pub(crate) fn unit_value() -> Json {
    call("Unit.unit", vec![], vec![])
}

/// `throwError` with a message built from text.
pub(crate) fn throw(monad: &Json, result: Json, message: Json) -> Json {
    call(
        "Lean.throwError",
        vec![monad.clone(), result],
        vec![call("Lean.stringToMessageData", vec![], vec![message])],
    )
}

pub(crate) fn get_env(monad: &Json) -> Json {
    call("Lean.MonadEnv.getEnv", vec![monad.clone()], vec![])
}

pub(crate) fn field_of(value: Json, name: &str) -> Json {
    field(value, name)
}

/// A name that need not resolve to a constant (a double-backtick literal must).
pub(crate) fn simple_name(text: &str) -> Json {
    call("Lean.Name.mkSimple", vec![], vec![string(text)])
}

pub(crate) fn nat_text(value: Json) -> Json {
    to_text(value, t_nat())
}

// --- the functions ---------------------------------------------------------

/// The text escapes of the record.
fn text_functions() -> Vec<Json> {
    let escaped = |out: Json, text: &str| append(out, string(text));
    let c = || var("c");
    let out = || var("out");
    let to_nat = |value: Json| call("Char.toNat", vec![], vec![value]);
    let chain = if_then(
        eq(t_char(), c(), chr('"')),
        escaped(out(), "\\\""),
        if_then(
            eq(t_char(), c(), chr('\\')),
            escaped(out(), "\\\\"),
            if_then(
                lt(to_nat(c()), nat(32)),
                cat(vec![
                    out(),
                    string("\\u"),
                    call(
                        "String.ofList",
                        vec![],
                        vec![call(
                            "List.drop",
                            vec![t_char()],
                            vec![
                                nat(1),
                                call(
                                    "Nat.toDigits",
                                    vec![],
                                    vec![nat(16), add(to_nat(c()), nat(65536))],
                                ),
                            ],
                        )],
                    ),
                ]),
                call("String.push", vec![], vec![out(), c()]),
            ),
        ),
    );
    vec![
        function(
            &own("escape"),
            &[],
            &[("text", t_string())],
            t_string(),
            call(
                "String.foldl",
                vec![t_string()],
                vec![
                    lambda(&[("out", t_string()), ("c", t_char())], chain),
                    string(""),
                    var("text"),
                ],
            ),
            Flags::default(),
            None,
        ),
        function(
            &own("str"),
            &[],
            &[("text", t_string())],
            t_string(),
            cat(vec![
                string("\""),
                call(&own("escape"), vec![], vec![var("text")]),
                string("\""),
            ]),
            Flags::default(),
            None,
        ),
        function(
            &own("name"),
            &[],
            &[("n", t_name())],
            t_string(),
            str_of(call("Lean.Name.toString", vec![], vec![var("n")])),
            Flags::default(),
            None,
        ),
        function(
            &own("arr"),
            &[],
            &[("items", t_list(t_string()))],
            t_string(),
            cat(vec![
                string("["),
                call(
                    "String.intercalate",
                    vec![],
                    vec![string(","), var("items")],
                ),
                string("]"),
            ]),
            Flags::default(),
            None,
        ),
        function(
            &own("obj"),
            &[],
            &[("fields", t_list(t_pair()))],
            t_string(),
            cat(vec![
                string("{"),
                call(
                    "String.intercalate",
                    vec![],
                    vec![
                        string(","),
                        map_list(
                            t_pair(),
                            t_string(),
                            "kv",
                            cat(vec![
                                str_of(field_of(var("kv"), "fst")),
                                string(":"),
                                field_of(var("kv"), "snd"),
                            ]),
                            var("fields"),
                        ),
                    ],
                ),
                string("}"),
            ]),
            Flags::default(),
            None,
        ),
        function(
            &own("bool"),
            &[],
            &[("value", t_bool())],
            t_string(),
            if_then(var("value"), string("true"), string("false")),
            Flags::default(),
            None,
        ),
        function(
            &own("fvar"),
            &[],
            &[("id", ty("Lean.FVarId"))],
            t_string(),
            name_of(field_of(var("id"), "name")),
            Flags::default(),
            None,
        ),
    ]
}

/// A sorted list of distinct names: Lean's `Name.lt` order, by a merge sort.
fn sort_functions() -> Vec<Json> {
    let names = t_list(t_name());
    let halves = t_prod(names.clone(), names.clone());
    let merge_body = matching(
        var("a"),
        vec![
            branch("List.nil", &[], var("b")),
            branch(
                "List.cons",
                &["x", "xs"],
                matching(
                    var("b"),
                    vec![
                        branch("List.nil", &[], var("a")),
                        branch(
                            "List.cons",
                            &["y", "ys"],
                            if_then(
                                call("Lean.Name.lt", vec![], vec![var("y"), var("x")]),
                                cons(
                                    t_name(),
                                    var("y"),
                                    call(&own("mergeNames"), vec![], vec![var("a"), var("ys")]),
                                ),
                                cons(
                                    t_name(),
                                    var("x"),
                                    call(&own("mergeNames"), vec![], vec![var("xs"), var("b")]),
                                ),
                            ),
                        ),
                    ],
                ),
            ),
        ],
    );
    let split_body = matching(
        var("l"),
        vec![
            branch(
                "List.nil",
                &[],
                construct(
                    "Prod.mk",
                    vec![names.clone(), names.clone()],
                    vec![nil(t_name()), nil(t_name())],
                ),
            ),
            branch(
                "List.cons",
                &["x", "xs"],
                let_in(
                    "rest",
                    halves.clone(),
                    call(&own("splitNames"), vec![], vec![var("xs")]),
                    construct(
                        "Prod.mk",
                        vec![names.clone(), names.clone()],
                        vec![
                            cons(t_name(), var("x"), field_of(var("rest"), "snd")),
                            field_of(var("rest"), "fst"),
                        ],
                    ),
                ),
            ),
        ],
    );
    let sort_body = matching(
        var("l"),
        vec![
            branch("List.nil", &[], nil(t_name())),
            branch(
                "List.cons",
                &["x", "xs"],
                matching(
                    var("xs"),
                    vec![
                        branch("List.nil", &[], var("l")),
                        branch(
                            "List.cons",
                            &["y", "ys"],
                            let_in(
                                "halves",
                                halves.clone(),
                                call(&own("splitNames"), vec![], vec![var("l")]),
                                call(
                                    &own("mergeNames"),
                                    vec![],
                                    vec![
                                        call(
                                            &own("sortNames"),
                                            vec![],
                                            vec![field_of(var("halves"), "fst")],
                                        ),
                                        call(
                                            &own("sortNames"),
                                            vec![],
                                            vec![field_of(var("halves"), "snd")],
                                        ),
                                    ],
                                ),
                            ),
                        ),
                    ],
                ),
            ),
        ],
    );
    vec![
        function(
            &own("mergeNames"),
            &[],
            &[("a", names.clone()), ("b", names.clone())],
            names.clone(),
            merge_body,
            PARTIAL,
            None,
        ),
        function(
            &own("splitNames"),
            &[],
            &[("l", names.clone())],
            halves,
            split_body,
            PARTIAL,
            None,
        ),
        function(
            &own("sortNames"),
            &[],
            &[("l", names.clone())],
            names.clone(),
            sort_body,
            PARTIAL,
            None,
        ),
        function(
            &own("sorted"),
            &[],
            &[("names", t_set())],
            names,
            call(
                &own("sortNames"),
                vec![],
                vec![call(
                    "Std.HashSet.toList",
                    vec![t_name()],
                    vec![var("names")],
                )],
            ),
            Flags::default(),
            None,
        ),
        function(
            &own("insert"),
            &[],
            &[("names", t_set()), ("n", t_name())],
            t_set(),
            call(
                "Std.HashSet.insert",
                vec![t_name()],
                vec![var("names"), var("n")],
            ),
            Flags::default(),
            None,
        ),
    ]
}

/// The registry-independent record writers for Lean expressions and the
/// LCNF fragments that are not part of the code family.
fn record_functions() -> Vec<Json> {
    let expr_branch = |ctor: &str, binders: &[&str], body: Json| {
        branch(&format!("Lean.Expr.{ctor}"), binders, body)
    };
    let level_list = |us: Json| {
        arr_list(map_list(
            ty("Lean.Level"),
            t_string(),
            "u",
            str_of(to_text(var("u"), ty("Lean.Level"))),
            us,
        ))
    };
    let unsupported = |what: &str| {
        obj(vec![
            kind_of("unsupported"),
            ("expression", str_of(string(what))),
        ])
    };
    let type_body = matching(
        var("e"),
        vec![
            expr_branch(
                "const",
                &["n", "us"],
                if_then(
                    eq(t_name(), var("n"), name("lcErased")),
                    obj(vec![kind_of("erased")]),
                    if_then(
                        eq(t_name(), var("n"), name("lcAny")),
                        obj(vec![kind_of("any")]),
                        obj(vec![
                            kind_of("const"),
                            ("name", name_of(var("n"))),
                            ("levels", level_list(var("us"))),
                        ]),
                    ),
                ),
            ),
            expr_branch(
                "app",
                &["fn", "arg"],
                obj(vec![
                    kind_of("app"),
                    (
                        "head",
                        call(
                            &own("type"),
                            vec![],
                            vec![call("Lean.Expr.getAppFn", vec![], vec![var("e")])],
                        ),
                    ),
                    (
                        "arguments",
                        arr_list(map_list(
                            t_expr(),
                            t_string(),
                            "a",
                            call(&own("type"), vec![], vec![var("a")]),
                            array_to_list(
                                t_expr(),
                                call("Lean.Expr.getAppArgs", vec![], vec![var("e")]),
                            ),
                        )),
                    ),
                ]),
            ),
            expr_branch(
                "forallE",
                &["binderName", "domain", "body", "binderInfo"],
                obj(vec![
                    kind_of("arrow"),
                    ("domain", call(&own("type"), vec![], vec![var("domain")])),
                    ("codomain", call(&own("type"), vec![], vec![var("body")])),
                ]),
            ),
            expr_branch(
                "fvar",
                &["id"],
                obj(vec![kind_of("fvar"), ("id", fvar_of(var("id")))]),
            ),
            expr_branch(
                "bvar",
                &["position"],
                obj(vec![kind_of("bvar"), ("index", nat_text(var("position")))]),
            ),
            expr_branch(
                "sort",
                &["u"],
                obj(vec![
                    kind_of("sort"),
                    ("level", str_of(to_text(var("u"), ty("Lean.Level")))),
                ]),
            ),
            expr_branch("mvar", &["id"], unsupported("metavariable")),
            expr_branch(
                "lam",
                &["binderName", "domain", "body", "binderInfo"],
                unsupported("lambda"),
            ),
            expr_branch(
                "letE",
                &["declName", "t", "value", "body", "nonDep"],
                unsupported("let"),
            ),
            expr_branch("lit", &["literal"], unsupported("literal")),
            expr_branch(
                "mdata",
                &["data", "body"],
                matching(
                    call(
                        "Lean.annotation?",
                        vec![],
                        vec![simple_name("borrowed"), var("e")],
                    ),
                    vec![
                        branch(
                            "Option.some",
                            &["annotated"],
                            obj(vec![
                                kind_of("borrowed"),
                                ("type", call(&own("type"), vec![], vec![var("body")])),
                            ]),
                        ),
                        branch("Option.none", &[], unsupported("metadata")),
                    ],
                ),
            ),
            expr_branch(
                "proj",
                &["typeName", "idx", "struct"],
                unsupported("projection"),
            ),
        ],
    );
    let pure_arg = t_pure("Arg");
    let arg_body = matching(
        var("a"),
        vec![
            branch(&lcnf("Arg.erased"), &[], obj(vec![kind_of("erased")])),
            branch(
                &lcnf("Arg.fvar"),
                &["id"],
                obj(vec![kind_of("fvar"), ("id", fvar_of(var("id")))]),
            ),
            branch(
                &lcnf("Arg.type"),
                &["e"],
                obj(vec![
                    kind_of("type"),
                    ("type", call(&own("type"), vec![], vec![var("e")])),
                ]),
            ),
        ],
    );
    let literal_text = |tag: &str, converter: Option<&str>| {
        let value = match converter {
            Some(converter) => nat_text(call(converter, vec![], vec![var("v")])),
            None => nat_text(var("v")),
        };
        obj(vec![kind_of(tag), ("value", str_of(value))])
    };
    let literal_body = matching(
        var("value"),
        vec![
            branch(&lcnf("LitValue.nat"), &["v"], literal_text("nat", None)),
            branch(
                &lcnf("LitValue.str"),
                &["v"],
                obj(vec![kind_of("string"), ("value", str_of(var("v")))]),
            ),
            branch(
                &lcnf("LitValue.uint8"),
                &["v"],
                literal_text("uint8", Some("UInt8.toNat")),
            ),
            branch(
                &lcnf("LitValue.uint16"),
                &["v"],
                literal_text("uint16", Some("UInt16.toNat")),
            ),
            branch(
                &lcnf("LitValue.uint32"),
                &["v"],
                literal_text("uint32", Some("UInt32.toNat")),
            ),
            branch(
                &lcnf("LitValue.uint64"),
                &["v"],
                literal_text("uint64", Some("UInt64.toNat")),
            ),
            branch(
                &lcnf("LitValue.usize"),
                &["v"],
                literal_text("usize", Some("UInt64.toNat")),
            ),
        ],
    );
    let arguments = |args: Json| {
        arr_list(map_list(
            pure_arg.clone(),
            t_string(),
            "a",
            call(&own("arg"), vec![], vec![var("a")]),
            array_to_list(pure_arg.clone(), args),
        ))
    };
    let let_value_body = matching(
        var("v"),
        vec![
            branch(
                &lcnf("LetValue.lit"),
                &["value"],
                obj(vec![
                    kind_of("literal"),
                    ("literal", call(&own("literal"), vec![], vec![var("value")])),
                ]),
            ),
            branch(&lcnf("LetValue.erased"), &[], obj(vec![kind_of("erased")])),
            branch(
                &lcnf("LetValue.proj"),
                &["typeName", "position", "struct"],
                obj(vec![
                    kind_of("projection"),
                    ("type_name", name_of(var("typeName"))),
                    ("index", nat_text(var("position"))),
                    ("value", fvar_of(var("struct"))),
                ]),
            ),
            branch(
                &lcnf("LetValue.const"),
                &["declName", "us", "args"],
                obj(vec![
                    kind_of("const"),
                    ("name", name_of(var("declName"))),
                    ("levels", level_list(var("us"))),
                    ("arguments", arguments(var("args"))),
                ]),
            ),
            branch(
                &lcnf("LetValue.fvar"),
                &["id", "args"],
                obj(vec![
                    kind_of("apply"),
                    ("function", fvar_of(var("id"))),
                    ("arguments", arguments(var("args"))),
                ]),
            ),
        ],
    );
    let param_body = obj(vec![
        ("id", fvar_of(field_of(var("p"), "fvarId"))),
        (
            "type",
            call(&own("type"), vec![], vec![field_of(var("p"), "type")]),
        ),
        ("borrow", bool_of(field_of(var("p"), "borrow"))),
    ]);
    vec![
        function(
            &own("type"),
            &[],
            &[("e", t_expr())],
            t_string(),
            type_body,
            PARTIAL,
            None,
        ),
        function(
            &own("arg"),
            &[],
            &[("a", pure_arg.clone())],
            t_string(),
            arg_body,
            Flags::default(),
            None,
        ),
        function(
            &own("literal"),
            &[],
            &[("value", ty(&lcnf("LitValue")))],
            t_string(),
            literal_body,
            Flags::default(),
            None,
        ),
        function(
            &own("letValue"),
            &[],
            &[("v", t_pure("LetValue"))],
            t_string(),
            let_value_body,
            Flags::default(),
            None,
        ),
        function(
            &own("param"),
            &[],
            &[("p", t_pure("Param"))],
            t_string(),
            param_body,
            Flags::default(),
            None,
        ),
    ]
}

/// Code, function declarations, alternatives, and their `cases`, each a
/// structural traversal of the LCNF code family (SPEC.md §17.20).
fn code_functions() -> Vec<Json> {
    let params = |value: Json| {
        arr_list(map_list(
            t_pure("Param"),
            t_string(),
            "p",
            call(&own("param"), vec![], vec![var("p")]),
            array_to_list(t_pure("Param"), value),
        ))
    };
    let arguments = |value: Json| {
        arr_list(map_list(
            t_pure("Arg"),
            t_string(),
            "a",
            call(&own("arg"), vec![], vec![var("a")]),
            array_to_list(t_pure("Arg"), value),
        ))
    };
    let code_of = |value: Json| call(&own("code"), vec![], vec![value]);
    let fun_decl_of = |value: Json| call(&own("funDecl"), vec![], vec![value]);
    let code_body = matching(
        var("c"),
        vec![
            branch(
                &lcnf("Code.let"),
                &["decl", "k"],
                obj(vec![
                    kind_of("let"),
                    ("id", fvar_of(field_of(var("decl"), "fvarId"))),
                    (
                        "type",
                        call(&own("type"), vec![], vec![field_of(var("decl"), "type")]),
                    ),
                    (
                        "value",
                        call(
                            &own("letValue"),
                            vec![],
                            vec![field_of(var("decl"), "value")],
                        ),
                    ),
                    ("body", code_of(var("k"))),
                ]),
            ),
            branch(
                &lcnf("Code.fun"),
                &["decl", "k"],
                obj(vec![
                    kind_of("fun"),
                    ("declaration", fun_decl_of(var("decl"))),
                    ("body", code_of(var("k"))),
                ]),
            ),
            branch(
                &lcnf("Code.jp"),
                &["decl", "k"],
                obj(vec![
                    kind_of("join"),
                    ("declaration", fun_decl_of(var("decl"))),
                    ("body", code_of(var("k"))),
                ]),
            ),
            branch(
                &lcnf("Code.jmp"),
                &["id", "args"],
                obj(vec![
                    kind_of("jump"),
                    ("target", fvar_of(var("id"))),
                    ("arguments", arguments(var("args"))),
                ]),
            ),
            branch(
                &lcnf("Code.cases"),
                &["cs"],
                call(&own("cases"), vec![], vec![var("cs")]),
            ),
            branch(
                &lcnf("Code.return"),
                &["id"],
                obj(vec![kind_of("return"), ("id", fvar_of(var("id")))]),
            ),
            branch(
                &lcnf("Code.unreach"),
                &["t"],
                obj(vec![
                    kind_of("unreachable"),
                    ("type", call(&own("type"), vec![], vec![var("t")])),
                ]),
            ),
        ],
    );
    let fun_decl_body = matching(
        var("d"),
        vec![branch(
            &lcnf("FunDecl.mk"),
            &["fvarId", "binderName", "params", "type", "value"],
            obj(vec![
                ("id", fvar_of(var("fvarId"))),
                ("parameters", params(var("params"))),
                ("type", call(&own("type"), vec![], vec![var("type")])),
                ("value", code_of(var("value"))),
            ]),
        )],
    );
    let cases_body = matching(
        var("cs"),
        vec![branch(
            &lcnf("Cases.mk"),
            &["typeName", "resultType", "discr", "alts"],
            obj(vec![
                kind_of("cases"),
                ("type_name", name_of(var("typeName"))),
                (
                    "result_type",
                    call(&own("type"), vec![], vec![var("resultType")]),
                ),
                ("discriminant", fvar_of(var("discr"))),
                (
                    "alternatives",
                    arr_list(map_list(
                        t_pure("Alt"),
                        t_string(),
                        "a",
                        call(&own("alt"), vec![], vec![var("a")]),
                        array_to_list(t_pure("Alt"), var("alts")),
                    )),
                ),
            ]),
        )],
    );
    let alt_body = matching(
        var("a"),
        vec![
            branch(
                &lcnf("Alt.alt"),
                &["ctorName", "params", "k"],
                obj(vec![
                    kind_of("constructor"),
                    ("constructor", name_of(var("ctorName"))),
                    ("parameters", params(var("params"))),
                    ("code", code_of(var("k"))),
                ]),
            ),
            branch(
                &lcnf("Alt.default"),
                &["k"],
                obj(vec![kind_of("default"), ("code", code_of(var("k")))]),
            ),
        ],
    );
    let group = Some("Code");
    vec![
        function(
            &own("code"),
            &[],
            &[("c", t_code())],
            t_string(),
            code_body,
            TRAVERSAL,
            group,
        ),
        function(
            &own("funDecl"),
            &[],
            &[("d", t_pure("FunDecl"))],
            t_string(),
            fun_decl_body,
            TRAVERSAL,
            group,
        ),
        function(
            &own("cases"),
            &[],
            &[("cs", t_pure("Cases"))],
            t_string(),
            cases_body,
            TRAVERSAL,
            group,
        ),
        function(
            &own("alt"),
            &[],
            &[("a", t_pure("Alt"))],
            t_string(),
            alt_body,
            TRAVERSAL,
            group,
        ),
    ]
}

/// The constants a type, a signature, and a body of code name.
fn constant_functions() -> Vec<Json> {
    let type_constants =
        |value: Json, out: Json| call(&own("typeConstants"), vec![], vec![value, out]);
    let expr_branch = |ctor: &str, binders: &[&str], body: Json| {
        branch(&format!("Lean.Expr.{ctor}"), binders, body)
    };
    let insert = |out: Json, n: Json| call(&own("insert"), vec![], vec![out, n]);
    let type_constants_body = matching(
        var("e"),
        vec![
            expr_branch(
                "const",
                &["n", "us"],
                if_then(
                    or(
                        eq(t_name(), var("n"), name("lcErased")),
                        eq(t_name(), var("n"), name("lcAny")),
                    ),
                    var("out"),
                    insert(var("out"), var("n")),
                ),
            ),
            expr_branch(
                "app",
                &["f", "a"],
                type_constants(var("a"), type_constants(var("f"), var("out"))),
            ),
            expr_branch(
                "forallE",
                &["binderName", "d", "b", "binderInfo"],
                type_constants(var("b"), type_constants(var("d"), var("out"))),
            ),
            expr_branch("bvar", &["position"], var("out")),
            expr_branch("fvar", &["id"], var("out")),
            expr_branch("mvar", &["id"], var("out")),
            expr_branch("sort", &["u"], var("out")),
            expr_branch("lit", &["literal"], var("out")),
            expr_branch(
                "lam",
                &["binderName", "d", "b", "binderInfo"],
                type_constants(var("b"), type_constants(var("d"), var("out"))),
            ),
            expr_branch(
                "letE",
                &["declName", "t", "v", "b", "nonDep"],
                type_constants(
                    var("b"),
                    type_constants(var("v"), type_constants(var("t"), var("out"))),
                ),
            ),
            expr_branch(
                "mdata",
                &["data", "b"],
                type_constants(var("b"), var("out")),
            ),
            expr_branch(
                "proj",
                &["typeName", "idx", "b"],
                type_constants(var("b"), insert(var("out"), var("typeName"))),
            ),
        ],
    );
    let arg_constants_body = matching(
        var("a"),
        vec![
            branch(
                &lcnf("Arg.type"),
                &["e"],
                type_constants(var("e"), var("out")),
            ),
            branch(&lcnf("Arg.erased"), &[], var("out")),
            branch(&lcnf("Arg.fvar"), &["id"], var("out")),
        ],
    );
    let fold_args = |args: Json, start: Json| {
        call(
            "Array.foldl",
            vec![t_pure("Arg"), t_set()],
            vec![
                lambda(
                    &[("acc", t_set()), ("a", t_pure("Arg"))],
                    call(&own("argConstants"), vec![], vec![var("a"), var("acc")]),
                ),
                start,
                args,
            ],
        )
    };
    let param_constants_body = call(
        "Array.foldl",
        vec![t_pure("Param"), t_set()],
        vec![
            lambda(
                &[("acc", t_set()), ("p", t_pure("Param"))],
                type_constants(field_of(var("p"), "type"), var("acc")),
            ),
            var("out"),
            var("ps"),
        ],
    );
    let let_value_constants_body = matching(
        var("v"),
        vec![
            branch(
                &lcnf("LetValue.const"),
                &["n", "us", "args"],
                fold_args(var("args"), insert(var("out"), var("n"))),
            ),
            branch(
                &lcnf("LetValue.fvar"),
                &["id", "args"],
                fold_args(var("args"), var("out")),
            ),
            branch(
                &lcnf("LetValue.proj"),
                &["typeName", "position", "struct"],
                insert(var("out"), var("typeName")),
            ),
            branch(&lcnf("LetValue.lit"), &["value"], var("out")),
            branch(&lcnf("LetValue.erased"), &[], var("out")),
        ],
    );
    let code_constants =
        |value: Json, out: Json| call(&own("codeConstants"), vec![], vec![value, out]);
    let fun_constants =
        |value: Json, out: Json| call(&own("funConstants"), vec![], vec![value, out]);
    let alt_constants =
        |value: Json, out: Json| call(&own("altConstants"), vec![], vec![value, out]);
    let param_constants =
        |value: Json, out: Json| call(&own("paramConstants"), vec![], vec![value, out]);
    let code_constants_body = matching(
        var("c"),
        vec![
            branch(
                &lcnf("Code.let"),
                &["decl", "k"],
                code_constants(
                    var("k"),
                    call(
                        &own("letValueConstants"),
                        vec![],
                        vec![
                            field_of(var("decl"), "value"),
                            type_constants(field_of(var("decl"), "type"), var("out")),
                        ],
                    ),
                ),
            ),
            branch(
                &lcnf("Code.fun"),
                &["decl", "k"],
                code_constants(var("k"), fun_constants(var("decl"), var("out"))),
            ),
            branch(
                &lcnf("Code.jp"),
                &["decl", "k"],
                code_constants(var("k"), fun_constants(var("decl"), var("out"))),
            ),
            branch(
                &lcnf("Code.jmp"),
                &["id", "args"],
                fold_args(var("args"), var("out")),
            ),
            branch(
                &lcnf("Code.cases"),
                &["cs"],
                call(&own("casesConstants"), vec![], vec![var("cs"), var("out")]),
            ),
            branch(&lcnf("Code.return"), &["id"], var("out")),
            branch(
                &lcnf("Code.unreach"),
                &["t"],
                type_constants(var("t"), var("out")),
            ),
        ],
    );
    let fun_constants_body = matching(
        var("d"),
        vec![branch(
            &lcnf("FunDecl.mk"),
            &["fvarId", "binderName", "params", "type", "value"],
            code_constants(
                var("value"),
                type_constants(var("type"), param_constants(var("params"), var("out"))),
            ),
        )],
    );
    let cases_constants_body = matching(
        var("cs"),
        vec![branch(
            &lcnf("Cases.mk"),
            &["typeName", "resultType", "discr", "alts"],
            call(
                "Array.foldl",
                vec![t_pure("Alt"), t_set()],
                vec![
                    lambda(
                        &[("acc", t_set()), ("a", t_pure("Alt"))],
                        alt_constants(var("a"), var("acc")),
                    ),
                    type_constants(var("resultType"), insert(var("out"), var("typeName"))),
                    var("alts"),
                ],
            ),
        )],
    );
    let alt_constants_body = matching(
        var("a"),
        vec![
            branch(
                &lcnf("Alt.alt"),
                &["ctorName", "ps", "k"],
                code_constants(
                    var("k"),
                    param_constants(var("ps"), insert(var("out"), var("ctorName"))),
                ),
            ),
            branch(
                &lcnf("Alt.default"),
                &["k"],
                code_constants(var("k"), var("out")),
            ),
        ],
    );
    let group = Some("CodeConstants");
    vec![
        function(
            &own("typeConstants"),
            &[],
            &[("e", t_expr()), ("out", t_set())],
            t_set(),
            type_constants_body,
            TRAVERSAL,
            None,
        ),
        function(
            &own("argConstants"),
            &[],
            &[("a", t_pure("Arg")), ("out", t_set())],
            t_set(),
            arg_constants_body,
            Flags::default(),
            None,
        ),
        function(
            &own("paramConstants"),
            &[],
            &[("ps", t_array(t_pure("Param"))), ("out", t_set())],
            t_set(),
            param_constants_body,
            Flags::default(),
            None,
        ),
        function(
            &own("letValueConstants"),
            &[],
            &[("v", t_pure("LetValue")), ("out", t_set())],
            t_set(),
            let_value_constants_body,
            Flags::default(),
            None,
        ),
        function(
            &own("codeConstants"),
            &[],
            &[("c", t_code()), ("out", t_set())],
            t_set(),
            code_constants_body,
            TRAVERSAL,
            group,
        ),
        function(
            &own("funConstants"),
            &[],
            &[("d", t_pure("FunDecl")), ("out", t_set())],
            t_set(),
            fun_constants_body,
            TRAVERSAL,
            group,
        ),
        function(
            &own("casesConstants"),
            &[],
            &[("cs", t_pure("Cases")), ("out", t_set())],
            t_set(),
            cases_constants_body,
            TRAVERSAL,
            group,
        ),
        function(
            &own("altConstants"),
            &[],
            &[("a", t_pure("Alt")), ("out", t_set())],
            t_set(),
            alt_constants_body,
            TRAVERSAL,
            group,
        ),
    ]
}

/// Every function of the adapter, in the order a definition precedes its
/// uses.
pub fn functions() -> Vec<Json> {
    let mut all = Vec::new();
    all.extend(text_functions());
    all.extend(sort_functions());
    all.extend(record_functions());
    all.extend(code_functions());
    all.extend(constant_functions());
    all.extend(crate::lcnf_extract::info_functions());
    all.extend(crate::lcnf_extract::signature_functions());
    all.extend(crate::lcnf_extract::listing_functions());
    all.extend(crate::lcnf_extract::authority_functions());
    all.extend(crate::lcnf_extract::run_functions());
    all.extend(crate::lcnf_extract::entry_functions());
    all
}
