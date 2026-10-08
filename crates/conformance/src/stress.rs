//! Families of production programs that grow one dimension at a time, to
//! measure what the certificates cost against what the lowering estimates
//! before it generates them (SPEC.md §17.17, SP-02).
//!
//! Each family is written as language-1.2 semantic data into a copy of the
//! `lowering-size-limit` project whose limits are lifted, so that the program
//! lowers and certifies at every size the family reaches; the estimate is
//! then compared with the certificates that were generated.

use crate::support::P;
use serde_json::{json, Value};

fn nat() -> Value {
    json!({"kind": "nat"})
}

fn var(name: &str) -> Value {
    json!({"kind": "var", "name": name})
}

fn literal(value: u64) -> Value {
    json!({"kind": "nat", "value": value.to_string()})
}

fn add(left: Value, right: Value) -> Value {
    json!({"kind": "add", "left": left, "right": right})
}

fn parameter(name: &str, ty: Value) -> Value {
    json!({"name": name, "type": ty})
}

fn named(name: &str) -> Value {
    json!({"arguments": [], "kind": "named", "member": {"name": name}})
}

fn root(
    name: &str,
    parameters: Vec<Value>,
    result: Value,
    body: Value,
    effects: &[&str],
    targets: &[&str],
) -> Value {
    json!({
        "body": body,
        "executable": true,
        "kind": "definition",
        "name": name,
        "parameters": parameters,
        "production": {"effects": effects, "targets": targets},
        "result": result,
    })
}

fn project(declarations: &[Value]) -> P {
    let project = P::negative("lowering-size-limit");
    let toml = project.read("lexlean.toml");
    project.write(
        "lexlean.toml",
        &toml
            .replace("max_file_bytes = 4194304", "max_file_bytes = 4000000000")
            .replace("max_ir_nodes = 2000000", "max_ir_nodes = 2000000000"),
    );
    let data = json!({"declarations": declarations, "spec": "lexlean/semantic-module/2"});
    project.write(
        "src/Main.lex.tex",
        &format!(
            "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\n\\semanticdata{{{data}}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
        ),
    );
    project.relock();
    project
}

const BOTH: [&str; 2] = ["rust-core", "rust-std"];

/// `x_i = x_{i-1} + 1` (or `x_{i-1} + x_{i-1}`) `count` times.
fn let_chain(count: usize, doubling: bool) -> Vec<Value> {
    let mut body = var(&format!("x{count}"));
    for index in (1..=count).rev() {
        let previous = if index == 1 {
            "n".to_owned()
        } else {
            format!("x{}", index - 1)
        };
        let value = if doubling {
            add(var(&previous), var(&previous))
        } else {
            add(var(&previous), literal(1))
        };
        body = json!({
            "binder": {"name": format!("x{index}"), "type": nat()},
            "body": body,
            "kind": "let",
            "value": value,
        });
    }
    vec![root(
        "r",
        vec![parameter("n", nat())],
        nat(),
        body,
        &["overflow"],
        &BOTH,
    )]
}

/// `f_i x = f_{i-1} x + 1`, called from the root.
fn call_chain(count: usize) -> Vec<Value> {
    let mut declarations = Vec::new();
    for index in 0..count {
        let callee = if index > 0 {
            json!({"kind": "call", "function": {"name": format!("f{}", index - 1)}, "arguments": [var("x")]})
        } else {
            var("x")
        };
        declarations.push(json!({
            "body": add(callee, literal(1)),
            "executable": true,
            "kind": "definition",
            "name": format!("f{index}"),
            "parameters": [parameter("x", nat())],
            "result": nat(),
        }));
    }
    declarations.push(root(
        "r",
        vec![parameter("n", nat())],
        nat(),
        json!({"kind": "call", "function": {"name": format!("f{}", count - 1)}, "arguments": [var("n")]}),
        &["overflow"],
        &BOTH,
    ));
    declarations
}

fn structure(fields: usize) -> Value {
    json!({
        "fields": (0..fields).map(|index| parameter(&format!("f{index}"), nat())).collect::<Vec<_>>(),
        "kind": "structure",
        "name": "S",
        "parameters": [],
        "type_parameters": [],
    })
}

/// A structure of `fields` fields whose root adds every one of them.
fn wide_projection(fields: usize) -> Vec<Value> {
    wide_projection_of(fields, fields)
}

/// A structure of `fields` fields whose root adds the first `uses`.
fn wide_projection_of(fields: usize, uses: usize) -> Vec<Value> {
    let mut body: Option<Value> = None;
    for index in 0..uses {
        let projection =
            json!({"field": format!("f{index}"), "kind": "project", "value": var("s")});
        body = Some(match body {
            Some(sum) => add(sum, projection),
            None => projection,
        });
    }
    vec![
        structure(fields),
        root(
            "r",
            vec![parameter("s", named("S"))],
            nat(),
            body.unwrap_or_else(|| literal(0)),
            &["overflow"],
            &BOTH,
        ),
    ]
}

/// `r s = S { f_i := s.f_i }`, `copied` of its `fields` fields copied from
/// the parameter and the rest set to zero: the family whose certificates grow
/// with the square of the arity.
fn record_copy(fields: usize, copied: usize) -> Vec<Value> {
    let record = json!({
        "kind": "record",
        "type": {"name": "S"},
        "fields": (0..fields).map(|index| json!({
            "field": format!("f{index}"),
            "value": if index < copied {
                json!({"field": format!("f{index}"), "kind": "project", "value": var("s")})
            } else {
                literal(0)
            },
        })).collect::<Vec<_>>(),
    });
    vec![
        structure(fields),
        root(
            "r",
            vec![parameter("s", named("S"))],
            named("S"),
            record,
            &[],
            &BOTH,
        ),
    ]
}

/// An enumeration of `constructors` constructors of `fields` fields each,
/// matched exhaustively.
fn enumeration(constructors: usize, fields: usize) -> Vec<Value> {
    let declaration = json!({
        "constructors": (0..constructors).map(|index| json!({
            "fields": vec![nat(); fields],
            "name": format!("c{index}"),
        })).collect::<Vec<_>>(),
        "kind": "inductive",
        "name": "E",
        "parameters": [],
        "type_parameters": [],
    });
    let branches: Vec<Value> = (0..constructors)
        .map(|index| {
            json!({
                "binders": (0..fields).map(|field| format!("b{index}_{field}")).collect::<Vec<_>>(),
                "body": if fields == 0 {
                    literal(index as u64)
                } else {
                    add(var(&format!("b{index}_0")), literal(index as u64))
                },
                "constructor": {"name": format!("E.c{index}")},
            })
        })
        .collect();
    vec![
        declaration,
        root(
            "r",
            vec![parameter("e", named("E"))],
            nat(),
            json!({"kind": "match", "scrutinee": var("e"), "branches": branches}),
            &["overflow"],
            &BOTH,
        ),
    ]
}

/// A function of `count` parameters that passes them all on, `depth` deep.
fn many_parameters(count: usize, depth: usize) -> Vec<Value> {
    let arguments = |count: usize| {
        (0..count)
            .map(|index| var(&format!("p{index}")))
            .collect::<Vec<_>>()
    };
    let parameters = |count: usize| {
        (0..count)
            .map(|index| parameter(&format!("p{index}"), nat()))
            .collect::<Vec<_>>()
    };
    let mut declarations = Vec::new();
    for level in 0..depth {
        let body = if level > 0 {
            json!({"kind": "call", "function": {"name": format!("h{}", level - 1)}, "arguments": arguments(count)})
        } else {
            var("p0")
        };
        declarations.push(json!({
            "body": body,
            "executable": true,
            "kind": "definition",
            "name": format!("h{level}"),
            "parameters": parameters(count),
            "result": nat(),
        }));
    }
    declarations.push(root(
        "r",
        parameters(count),
        nat(),
        json!({"kind": "call", "function": {"name": format!("h{}", depth - 1)}, "arguments": arguments(count)}),
        &[],
        &BOTH,
    ));
    declarations
}

/// `g_k<T> x = g_{k+1}<(T, T)> (x, x)` down to depth `depth`: the type of the
/// last instance has `2^depth` nodes.
fn generic_chain(depth: usize) -> Vec<Value> {
    let t = json!({"kind": "parameter", "name": "T"});
    let mut declarations = Vec::new();
    for level in (0..=depth).rev() {
        let body = if level == depth {
            literal(0)
        } else {
            json!({
                "arguments": [{"kind": "pair", "left": var("x"), "right": var("x")}],
                "function": {"name": format!("g{}", level + 1)},
                "kind": "call",
                "type_arguments": [{"kind": "product", "left": t, "right": t}],
            })
        };
        declarations.push(json!({
            "body": body,
            "executable": true,
            "kind": "definition",
            "name": format!("g{level}"),
            "parameters": [parameter("x", t.clone())],
            "result": nat(),
            "type_parameters": ["T"],
        }));
    }
    declarations.push(root(
        "chain",
        vec![parameter("n", nat())],
        nat(),
        json!({"arguments": [var("n")], "function": {"name": "g0"}, "kind": "call", "type_arguments": [nat()]}),
        &[],
        &["rust-std"],
    ));
    declarations
}

/// A fallible chain, `x_i = x_{i-1} - 1`, whose operations may refuse.
fn subtraction_chain(count: usize) -> Vec<Value> {
    let mut body = var(&format!("x{count}"));
    for index in (1..=count).rev() {
        let previous = if index == 1 {
            "n".to_owned()
        } else {
            format!("x{}", index - 1)
        };
        body = json!({
            "binder": {"name": format!("x{index}"), "type": nat()},
            "body": body,
            "kind": "let",
            "value": json!({"arguments": [var(&previous), literal(1)], "kind": "primitive", "operation": "subtract", "result": nat()}),
        });
    }
    vec![root(
        "r",
        vec![parameter("n", nat())],
        nat(),
        body,
        &["overflow"],
        &BOTH,
    )]
}

/// A root returning one literal of `length` characters (control characters,
/// which a certificate spells with six).
fn string_literal(length: usize) -> Vec<Value> {
    vec![root(
        "r",
        vec![parameter("n", nat())],
        json!({"kind": "string"}),
        json!({"kind": "string", "value": "\u{1}".repeat(length)}),
        &["allocation"],
        &["rust-std"],
    )]
}

/// A root returning one bytes literal of `length` bytes.
fn bytes_literal(length: usize) -> Vec<Value> {
    vec![root(
        "r",
        vec![parameter("n", nat())],
        json!({"kind": "bytes"}),
        json!({"kind": "bytes", "hex": "ab".repeat(length)}),
        &["allocation"],
        &["rust-std"],
    )]
}

/// `depth` matches nested in the node arm of one another, each on its own
/// parameter and binding `fields` fields.
fn nested_match(depth: usize, fields: usize) -> Vec<Value> {
    let tree = json!({
        "constructors": [
            {"fields": [], "name": "leaf"},
            {"fields": vec![nat(); fields], "name": "node"},
        ],
        "kind": "inductive",
        "name": "T",
        "parameters": [],
        "type_parameters": [],
    });
    let mut body = literal(0);
    for level in (0..depth).rev() {
        let binders: Vec<String> = (0..fields)
            .map(|field| format!("b{level}_{field}"))
            .collect();
        let inner = if level + 1 == depth {
            var(&format!("b{level}_0"))
        } else {
            body
        };
        body = json!({
            "kind": "match",
            "scrutinee": var(&format!("t{level}")),
            "branches": [
                {"binders": [], "body": literal(0), "constructor": {"name": "T.leaf"}},
                {"binders": binders, "body": inner, "constructor": {"name": "T.node"}},
            ],
        });
    }
    vec![
        tree,
        root(
            "r",
            (0..depth)
                .map(|level| parameter(&format!("t{level}"), named("T")))
                .collect(),
            nat(),
            body,
            &[],
            &BOTH,
        ),
    ]
}

/// The identity at `T`, passed down `depth` generic definitions and
/// instantiated at a structure of `fields` fields.
fn wide_instances(depth: usize, fields: usize) -> Vec<Value> {
    let t = json!({"kind": "parameter", "name": "T"});
    let mut declarations = vec![structure(fields)];
    for level in (0..depth).rev() {
        let body = if level + 1 == depth {
            var("x")
        } else {
            json!({
                "arguments": [var("x")],
                "function": {"name": format!("i{}", level + 1)},
                "kind": "call",
                "type_arguments": [t.clone()],
            })
        };
        declarations.push(json!({
            "body": body,
            "executable": true,
            "kind": "definition",
            "name": format!("i{level}"),
            "parameters": [parameter("x", t.clone())],
            "result": t.clone(),
            "type_parameters": ["T"],
        }));
    }
    declarations.push(root(
        "r",
        vec![parameter("s", named("S"))],
        named("S"),
        json!({"arguments": [var("s")], "function": {"name": "i0"}, "kind": "call", "type_arguments": [named("S")]}),
        &[],
        &["rust-std"],
    ));
    declarations
}

/// A structure whose `fields` fields have names of `length` characters, all
/// read by the root.
fn long_names(fields: usize, length: usize) -> Vec<Value> {
    let name = |index: usize| format!("{}{index:03}", "f".repeat(length - 3));
    let record = json!({
        "fields": (0..fields).map(|index| parameter(&name(index), nat())).collect::<Vec<_>>(),
        "kind": "structure",
        "name": "S",
        "parameters": [],
        "type_parameters": [],
    });
    let mut body: Option<Value> = None;
    for index in 0..fields {
        let projection = json!({"field": name(index), "kind": "project", "value": var("s")});
        body = Some(match body {
            Some(sum) => add(sum, projection),
            None => projection,
        });
    }
    vec![
        record,
        root(
            "r",
            vec![parameter("s", named("S"))],
            nat(),
            body.unwrap_or_else(|| literal(0)),
            &["overflow"],
            &BOTH,
        ),
    ]
}

/// Every stress family at the sizes the estimate is checked on, with a name.
#[must_use]
pub fn families() -> Vec<(String, P)> {
    let mut out: Vec<(String, P)> = Vec::new();
    let mut push = |name: String, declarations: Vec<Value>| {
        out.push((name, project(&declarations)));
    };
    for count in [4, 8, 12] {
        push(
            format!("doubling let chain {count}"),
            let_chain(count, true),
        );
    }
    for count in [25, 100] {
        push(format!("let chain {count}"), let_chain(count, false));
        push(
            format!("subtraction chain {count}"),
            subtraction_chain(count),
        );
        push(format!("call chain {count}"), call_chain(count));
    }
    for count in [10, 40] {
        push(format!("wide projection {count}"), wide_projection(count));
        push(format!("enumeration {count}"), enumeration(count, 3));
    }
    if std::env::var("STRESS_GRID").is_ok() {
        for fields in [10, 20, 40, 80] {
            for uses in [1, 5, 20].into_iter().filter(|uses| *uses <= fields) {
                push(
                    format!("grid {fields} {uses}"),
                    wide_projection_of(fields, uses),
                );
            }
        }
    }
    for fields in [25, 50, 100, 200] {
        push(format!("record copy {fields}"), record_copy(fields, fields));
    }
    push(
        "record built from one field 100".to_owned(),
        record_copy(100, 1),
    );
    for count in [25, 100] {
        push(
            format!("many parameters {count}"),
            many_parameters(count, 2),
        );
    }
    for length in [2_000, 20_000] {
        push(format!("string literal {length}"), string_literal(length));
    }
    push("bytes literal 10000".to_owned(), bytes_literal(10_000));
    for constructors in [60, 200] {
        push(
            format!("enumeration of one field {constructors}"),
            enumeration(constructors, 1),
        );
    }
    push(
        "enumeration of three fields 100".to_owned(),
        enumeration(100, 3),
    );
    for (depth, fields) in [(12, 3), (20, 30)] {
        push(
            format!("nested match {depth} of {fields}"),
            nested_match(depth, fields),
        );
    }
    push(
        "wide instances 20 of 100".to_owned(),
        wide_instances(20, 100),
    );
    push("long names 20 of 100".to_owned(), long_names(20, 100));
    for depth in [6, 9, 10, 11] {
        push(format!("generic chain {depth}"), generic_chain(depth));
    }
    out
}

/// Roots whose parameters, fields, and locals are spelled like the names the
/// generator binds in certificate E (`hrep`, `ro`, `h`, `hr`, `hc`, `n`) and
/// like the tokens the certificate audit forbids (`kernel`, `prefix`,
/// `macro`, ...). They are valid programs and must verify.
#[must_use]
pub fn names() -> P {
    let sum = |names: &[&str]| -> Value {
        let mut out = var(names[0]);
        for name in &names[1..] {
            out = add(out, var(name));
        }
        out
    };
    let plain = ["hrep", "ro", "h", "hr", "hc", "n", "f", "x", "P"];
    let words = [
        "kernel", "prefix", "notation", "infix", "elab", "macro", "syntax", "extern",
    ];
    let set = json!({"element": nat(), "kind": "set"});
    let size = |name: &str| json!({"arguments": [var(name)], "kind": "primitive", "operation": "set_size", "result": nat()});
    let words_record = json!({
        "fields": [parameter("macro", nat()), parameter("syntax", nat())],
        "kind": "structure",
        "name": "Words",
        "parameters": [],
        "type_parameters": [],
    });
    let declarations = vec![
        root(
            "plain",
            plain.iter().map(|name| parameter(name, nat())).collect(),
            nat(),
            sum(&plain),
            &["overflow"],
            &BOTH,
        ),
        root(
            "entered",
            vec![
                parameter("h", set.clone()),
                parameter("hr", set),
                parameter("hc", nat()),
                parameter("ro", nat()),
                parameter("hrep", nat()),
            ],
            nat(),
            add(
                add(add(size("h"), size("hr")), var("hc")),
                add(var("ro"), var("hrep")),
            ),
            &["allocation", "overflow"],
            &["rust-std"],
        ),
        root(
            "words",
            words.iter().map(|name| parameter(name, nat())).collect(),
            nat(),
            sum(&words),
            &["overflow"],
            &BOTH,
        ),
        words_record,
        root(
            "fielded",
            vec![parameter("w", named("Words"))],
            nat(),
            json!({
                "binder": {"name": "elab", "type": nat()},
                "body": add(var("elab"), json!({"field": "syntax", "kind": "project", "value": var("w")})),
                "kind": "let",
                "value": json!({"field": "macro", "kind": "project", "value": var("w")}),
            }),
            &["overflow"],
            &BOTH,
        ),
    ];
    project(&declarations)
}

/// A root matching a value of an enumeration of `constructors` constructors
/// without fields, one arm each, under limits lifted far enough for it.
#[must_use]
pub fn wide_match(constructors: usize) -> P {
    project(&enumeration(constructors, 0))
}
