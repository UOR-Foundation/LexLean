//! The corpora of the oracle differential (SPEC.md §17.19).
//!
//! The corpus is three kinds of project, all compiled by LexLean to Lean and
//! all handed to the oracle in the same way:
//!
//! - the four certified examples, every production root of which is a
//!   program written for a purpose;
//! - the probe project, one minimal root for each runtime construct of the
//!   production registry that those examples do not exercise alone, so that
//!   the oracle's verdict on a construct is a verdict on that construct and
//!   not on whatever else a large root happens to contain;
//! - the rejection projects: the committed negative fixtures that LexLean
//!   refuses as production roots, with the production marking removed so that
//!   LexLean still compiles them to Lean. LexLean's verdict on each is the
//!   diagnostic its fixture records; the oracle's is whatever it says of the
//!   same definition.

use serde_json::{json, Value as Json};

use crate::support::{repo_root, P};

/// The certified examples the differential runs over.
pub const EXAMPLES: [&str; 4] = ["production", "production-coverage", "models", "reasoning"];

/// The negative fixtures that LexLean refuses as production roots, and that
/// remain programs when the marking is removed.
pub const REJECTIONS: [&str; 7] = [
    "production-polymorphic-root",
    "production-higher-order-escape",
    "production-runtime-incompatible-type",
    "production-literal-width",
    "production-phantom-type-parameter",
    "production-named-type-boundary",
    "production-universe-boundary",
];

/// What a corpus is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// A committed certified example.
    Example,
    /// The probe project.
    Probes,
    /// A rejection fixture, by its name.
    Rejection(String),
}

/// One project of the corpus.
pub struct Corpus {
    /// The corpus name: the example, `probes`, or the fixture.
    pub name: String,
    /// What it is.
    pub kind: Kind,
    /// The project.
    pub project: P,
}

/// A probe: the minimal root that stands for the registry constructs `keys`.
pub struct Probe {
    /// The registry constructs the root exists to exercise.
    pub keys: &'static [&'static str],
    /// The module (and file) name.
    pub module: &'static str,
    /// The root's name.
    pub root: &'static str,
    /// The module's declarations.
    pub declarations: Vec<Json>,
    /// The targets the root claims.
    pub targets: &'static [&'static str],
}

/// A probe in tabular form: the constructs it is for, its module, its root,
/// its parameters, its result, and its body.
type Spec = (
    &'static [&'static str],
    &'static str,
    &'static str,
    Vec<Json>,
    Json,
    Json,
);

const BOTH: &[&str] = &["rust-core", "rust-std"];
const STD: &[&str] = &["rust-std"];

fn nat_t() -> Json {
    json!({"kind": "nat"})
}
fn bool_t() -> Json {
    json!({"kind": "bool"})
}
fn scalar_t(kind: &str) -> Json {
    json!({"kind": kind})
}
fn list_t(element: Json) -> Json {
    json!({"kind": "list", "element": element})
}
fn option_t(value: Json) -> Json {
    json!({"kind": "option", "value": value})
}
fn map_t(key: Json, value: Json) -> Json {
    json!({"kind": "map", "key": key, "value": value})
}
fn set_t(element: Json) -> Json {
    json!({"kind": "set", "element": element})
}
fn product_t(left: Json, right: Json) -> Json {
    json!({"kind": "product", "left": left, "right": right})
}
fn var(name: &str) -> Json {
    json!({"kind": "var", "name": name})
}
fn nat(value: u64) -> Json {
    json!({"kind": "nat", "value": value.to_string()})
}
fn param(name: &str, ty: Json) -> Json {
    json!({"name": name, "type": ty})
}
fn add(left: Json, right: Json) -> Json {
    json!({"kind": "add", "left": left, "right": right})
}
fn prim(operation: &str, arguments: Vec<Json>, result: Json) -> Json {
    json!({"kind": "primitive", "operation": operation, "arguments": arguments, "result": result})
}
fn lambda(parameters: Vec<Json>, captures: &[&str], body: Json) -> Json {
    json!({"kind": "lambda", "parameters": parameters, "captures": captures, "body": body})
}
fn call(name: &str, arguments: Vec<Json>) -> Json {
    json!({"kind": "call", "function": {"name": name}, "arguments": arguments})
}

/// A root: an executable definition with a production block that admits
/// every effect, since a declared effect may exceed the inferred ones.
fn root(name: &str, parameters: Vec<Json>, result: Json, body: Json, targets: &[&str]) -> Json {
    json!({
        "kind": "definition",
        "name": name,
        "executable": true,
        "parameters": parameters,
        "result": result,
        "body": body,
        "production": {"effects": ["allocation", "overflow", "recursion"], "targets": targets},
    })
}

/// A helper definition (not a root).
fn helper(name: &str, parameters: Vec<Json>, result: Json, body: Json) -> Json {
    json!({
        "kind": "definition",
        "name": name,
        "executable": true,
        "parameters": parameters,
        "result": result,
        "body": body,
    })
}

fn one(
    keys: &'static [&'static str],
    module: &'static str,
    name: &'static str,
    parameters: Vec<Json>,
    result: Json,
    body: Json,
    targets: &'static [&'static str],
) -> Probe {
    Probe {
        keys,
        module,
        root: name,
        declarations: vec![root(name, parameters, result, body, targets)],
        targets,
    }
}

fn nat_map() -> Json {
    map_t(nat_t(), nat_t())
}
fn nat_set() -> Json {
    set_t(nat_t())
}

/// The probes, in a fixed order.
#[must_use]
pub fn probes() -> Vec<Probe> {
    let mut out = Vec::new();
    out.push(one(
        &["type.unit", "term.unit"],
        "ProbeUnit",
        "unitRoot",
        vec![param("x", nat_t())],
        scalar_t("unit"),
        json!({"kind": "unit"}),
        BOTH,
    ));
    out.push(one(
        &["type.int", "primitive.negate"],
        "ProbeInt",
        "intRoot",
        vec![param("a", scalar_t("int"))],
        scalar_t("int"),
        prim("negate", vec![var("a")], scalar_t("int")),
        BOTH,
    ));
    for (module, kind, name) in [
        ("ProbeInt16", "int16", "int16Root"),
        ("ProbeInt32", "int32", "int32Root"),
    ] {
        let key: &'static [&'static str] = if kind == "int16" {
            &["type.int16"]
        } else {
            &["type.int32"]
        };
        out.push(one(
            key,
            module,
            name,
            vec![param("a", scalar_t(kind)), param("b", scalar_t(kind))],
            option_t(scalar_t(kind)),
            prim(
                "checked_add",
                vec![var("a"), var("b")],
                option_t(scalar_t(kind)),
            ),
            BOTH,
        ));
    }
    for (module, name, key, selector) in [
        ("ProbeFirst", "firstRoot", "term.first", "first"),
        ("ProbeSecond", "secondRoot", "term.second", "second"),
    ] {
        let keys: &'static [&'static str] = if selector == "first" {
            &["term.first"]
        } else {
            &["term.second"]
        };
        let _ = key;
        out.push(one(
            keys,
            module,
            name,
            vec![param("p", product_t(nat_t(), nat_t()))],
            nat_t(),
            json!({"kind": selector, "value": var("p")}),
            BOTH,
        ));
    }
    out.push(one(
        &["primitive.checked_convert"],
        "ProbeConvert",
        "convertRoot",
        vec![param("a", scalar_t("int8"))],
        option_t(scalar_t("uint16")),
        prim(
            "checked_convert",
            vec![var("a")],
            option_t(scalar_t("uint16")),
        ),
        BOTH,
    ));
    out.push(one(
        &["term.lambda", "term.apply"],
        "ProbeLambda",
        "lambdaRoot",
        vec![param("x", nat_t())],
        nat_t(),
        json!({
            "kind": "apply",
            "function": lambda(
                vec![param("y", nat_t())],
                &["x"],
                add(var("y"), var("x")),
            ),
            "arguments": [var("x")],
        }),
        BOTH,
    ));
    out.push(Probe {
        keys: &["term.function_ref", "type.function"],
        module: "ProbeFunctionRef",
        root: "referenceRoot",
        declarations: vec![
            helper(
                "double",
                vec![param("n", nat_t())],
                nat_t(),
                add(var("n"), var("n")),
            ),
            helper(
                "twice",
                vec![
                    param(
                        "step",
                        json!({"kind": "function", "parameters": [nat_t()], "result": nat_t()}),
                    ),
                    param("value", nat_t()),
                ],
                nat_t(),
                json!({
                    "kind": "apply",
                    "function": var("step"),
                    "arguments": [{
                        "kind": "apply",
                        "function": var("step"),
                        "arguments": [var("value")],
                    }],
                }),
            ),
            root(
                "referenceRoot",
                vec![param("x", nat_t())],
                nat_t(),
                call(
                    "twice",
                    vec![
                        json!({"kind": "function_ref", "function": {"name": "double"}}),
                        var("x"),
                    ],
                ),
                BOTH,
            ),
        ],
        targets: BOTH,
    });
    out.push(Probe {
        // A type parameter is realized only by instantiation, so no closure
        // holds the construct; the probe is a generic definition used at one
        // type, whose refusal by the oracle is the row of `type.parameter`.
        keys: &["declaration.definition"],
        module: "ProbeGeneric",
        root: "genericRoot",
        declarations: vec![
            json!({
                "kind": "definition",
                "name": "identity",
                "executable": true,
                "parameters": [param("value", json!({"kind": "parameter", "name": "Item"}))],
                "result": {"kind": "parameter", "name": "Item"},
                "body": var("value"),
                "type_parameters": ["Item"],
            }),
            root(
                "genericRoot",
                vec![param("x", nat_t())],
                nat_t(),
                json!({
                    "kind": "call",
                    "function": {"name": "identity"},
                    "arguments": [var("x")],
                    "type_arguments": [nat_t()],
                }),
                BOTH,
            ),
        ],
        targets: BOTH,
    });
    out.push(one(
        &["constructor.nat_succ"],
        "ProbeSucc",
        "successorRoot",
        vec![param("n", nat_t())],
        nat_t(),
        json!({"kind": "constructor", "constructor": {"name": "Nat.succ"}, "arguments": [var("n")]}),
        BOTH,
    ));
    out.push(Probe {
        keys: &[
            "declaration.class",
            "declaration.instance",
            "term.instance_value",
        ],
        module: "ProbeClass",
        root: "classRoot",
        declarations: vec![
            json!({
                "kind": "class",
                "name": "Weights",
                "parameters": [],
                "type_parameters": [],
                "fields": [param("base", nat_t()), param("scale", nat_t())],
            }),
            json!({
                "kind": "instance",
                "name": "defaultWeights",
                "class": {"name": "Weights"},
                "arguments": [],
                "priority": 1000,
                "fields": [
                    {"field": "base", "value": nat(2)},
                    {"field": "scale", "value": nat(3)},
                ],
            }),
            root(
                "classRoot",
                vec![param("x", nat_t())],
                nat_t(),
                add(
                    var("x"),
                    json!({
                        "kind": "project",
                        "field": "scale",
                        "value": {
                            "kind": "instance_value",
                            "class": {"name": "Weights"},
                            "arguments": [],
                            "resolved": {"name": "defaultWeights"},
                        },
                    }),
                ),
                BOTH,
            ),
        ],
        targets: BOTH,
    });
    out.push(one(
        &["primitive.parse_decimal"],
        "ProbeParse",
        "parseRoot",
        vec![param("s", scalar_t("string"))],
        option_t(scalar_t("uint32")),
        prim(
            "parse_decimal",
            vec![var("s")],
            option_t(scalar_t("uint32")),
        ),
        STD,
    ));
    out.push(one(
        &["primitive.format_decimal"],
        "ProbeFormat",
        "formatRoot",
        vec![param("a", scalar_t("uint8"))],
        scalar_t("string"),
        prim("format_decimal", vec![var("a")], scalar_t("string")),
        STD,
    ));
    // Maps.
    let map_probes: [Spec; 8] = [
        (
            &["primitive.map_insert"],
            "ProbeMapInsert",
            "mapInsertRoot",
            vec![
                param("m", nat_map()),
                param("k", nat_t()),
                param("v", nat_t()),
            ],
            nat_map(),
            prim("map_insert", vec![var("m"), var("k"), var("v")], nat_map()),
        ),
        (
            &["primitive.map_remove"],
            "ProbeMapRemove",
            "mapRemoveRoot",
            vec![param("m", nat_map()), param("k", nat_t())],
            nat_map(),
            prim("map_remove", vec![var("m"), var("k")], nat_map()),
        ),
        (
            &["primitive.map_lookup"],
            "ProbeMapLookup",
            "mapLookupRoot",
            vec![param("m", nat_map()), param("k", nat_t())],
            option_t(nat_t()),
            prim("map_lookup", vec![var("m"), var("k")], option_t(nat_t())),
        ),
        (
            &["primitive.map_contains"],
            "ProbeMapContains",
            "mapContainsRoot",
            vec![param("m", nat_map()), param("k", nat_t())],
            bool_t(),
            prim("map_contains", vec![var("m"), var("k")], bool_t()),
        ),
        (
            &["primitive.map_size"],
            "ProbeMapSize",
            "mapSizeRoot",
            vec![param("m", nat_map())],
            nat_t(),
            prim("map_size", vec![var("m")], nat_t()),
        ),
        (
            &["primitive.map_keys"],
            "ProbeMapKeys",
            "mapKeysRoot",
            vec![param("m", nat_map())],
            list_t(nat_t()),
            prim("map_keys", vec![var("m")], list_t(nat_t())),
        ),
        (
            &["primitive.map_values"],
            "ProbeMapValues",
            "mapValuesRoot",
            vec![param("m", nat_map())],
            list_t(nat_t()),
            prim("map_values", vec![var("m")], list_t(nat_t())),
        ),
        (
            &["primitive.map_entries"],
            "ProbeMapEntries",
            "mapEntriesRoot",
            vec![param("m", nat_map())],
            list_t(product_t(nat_t(), nat_t())),
            prim(
                "map_entries",
                vec![var("m")],
                list_t(product_t(nat_t(), nat_t())),
            ),
        ),
    ];
    for (keys, module, name, parameters, result, body) in map_probes {
        out.push(one(keys, module, name, parameters, result, body, STD));
    }
    out.push(one(
        &["primitive.map_fold"],
        "ProbeMapFold",
        "mapFoldRoot",
        vec![param("m", nat_map())],
        nat_t(),
        prim(
            "map_fold",
            vec![
                lambda(
                    vec![
                        param("acc", nat_t()),
                        param("key", nat_t()),
                        param("value", nat_t()),
                    ],
                    &[],
                    add(add(var("acc"), var("key")), var("value")),
                ),
                nat(0),
                var("m"),
            ],
            nat_t(),
        ),
        STD,
    ));
    // Sets.
    let set_probes: [Spec; 7] = [
        (
            &["primitive.set_insert"],
            "ProbeSetInsert",
            "setInsertRoot",
            vec![param("s", nat_set()), param("x", nat_t())],
            nat_set(),
            prim("set_insert", vec![var("s"), var("x")], nat_set()),
        ),
        (
            &["primitive.set_remove"],
            "ProbeSetRemove",
            "setRemoveRoot",
            vec![param("s", nat_set()), param("x", nat_t())],
            nat_set(),
            prim("set_remove", vec![var("s"), var("x")], nat_set()),
        ),
        (
            &["primitive.set_contains"],
            "ProbeSetContains",
            "setContainsRoot",
            vec![param("s", nat_set()), param("x", nat_t())],
            bool_t(),
            prim("set_contains", vec![var("s"), var("x")], bool_t()),
        ),
        (
            &["primitive.set_size"],
            "ProbeSetSize",
            "setSizeRoot",
            vec![param("s", nat_set())],
            nat_t(),
            prim("set_size", vec![var("s")], nat_t()),
        ),
        (
            &["primitive.set_elements"],
            "ProbeSetElements",
            "setElementsRoot",
            vec![param("s", nat_set())],
            list_t(nat_t()),
            prim("set_elements", vec![var("s")], list_t(nat_t())),
        ),
        (
            &["primitive.set_union"],
            "ProbeSetUnion",
            "setUnionRoot",
            vec![param("s", nat_set()), param("t", nat_set())],
            nat_set(),
            prim("set_union", vec![var("s"), var("t")], nat_set()),
        ),
        (
            &["primitive.set_intersection"],
            "ProbeSetIntersection",
            "setIntersectionRoot",
            vec![param("s", nat_set()), param("t", nat_set())],
            nat_set(),
            prim("set_intersection", vec![var("s"), var("t")], nat_set()),
        ),
    ];
    for (keys, module, name, parameters, result, body) in set_probes {
        out.push(one(keys, module, name, parameters, result, body, STD));
    }
    out.push(one(
        &["primitive.set_difference"],
        "ProbeSetDifference",
        "setDifferenceRoot",
        vec![param("s", nat_set()), param("t", nat_set())],
        nat_set(),
        prim("set_difference", vec![var("s"), var("t")], nat_set()),
        STD,
    ));
    out.push(one(
        &["primitive.set_fold"],
        "ProbeSetFold",
        "setFoldRoot",
        vec![param("s", nat_set())],
        nat_t(),
        prim(
            "set_fold",
            vec![
                lambda(
                    vec![param("acc", nat_t()), param("x", nat_t())],
                    &[],
                    add(var("acc"), var("x")),
                ),
                nat(0),
                var("s"),
            ],
            nat_t(),
        ),
        STD,
    ));
    out.push(one(
        &["primitive.list_fold"],
        "ProbeListFold",
        "listFoldRoot",
        vec![param("xs", list_t(nat_t()))],
        nat_t(),
        prim(
            "list_fold",
            vec![
                lambda(
                    vec![param("acc", nat_t()), param("x", nat_t())],
                    &[],
                    add(var("acc"), var("x")),
                ),
                nat(0),
                var("xs"),
            ],
            nat_t(),
        ),
        STD,
    ));
    out.push(one(
        &["primitive.iterate"],
        "ProbeIterate",
        "iterateRoot",
        vec![param("n", nat_t()), param("start", nat_t())],
        nat_t(),
        prim(
            "iterate",
            vec![
                lambda(vec![param("x", nat_t())], &[], add(var("x"), nat(1))),
                var("n"),
                var("start"),
            ],
            nat_t(),
        ),
        STD,
    ));
    out.push(one(
        &["primitive.iterate_until"],
        "ProbeIterateUntil",
        "iterateUntilRoot",
        vec![param("n", nat_t()), param("start", nat_t())],
        product_t(nat_t(), bool_t()),
        prim(
            "iterate_until",
            vec![
                lambda(
                    vec![param("x", nat_t())],
                    &[],
                    json!({
                        "kind": "if",
                        "condition": {"kind": "blt", "left": var("x"), "right": nat(10)},
                        "then_value": {
                            "kind": "constructor",
                            "constructor": {"name": "Option.some"},
                            "type_arguments": [nat_t()],
                            "arguments": [add(var("x"), nat(2))],
                        },
                        "else_value": {
                            "kind": "constructor",
                            "constructor": {"name": "Option.none"},
                            "type_arguments": [nat_t()],
                            "arguments": [],
                        },
                    }),
                ),
                var("n"),
                var("start"),
            ],
            product_t(nat_t(), bool_t()),
        ),
        STD,
    ));
    let graph = || map_t(nat_t(), nat_set());
    for (module, name, operation, key, result) in [
        (
            "ProbeGraphSuccessors",
            "successorsRoot",
            "graph_successors",
            "primitive.graph_successors",
            nat_set(),
        ),
        (
            "ProbeGraphReachable",
            "reachableRoot",
            "graph_reachable",
            "primitive.graph_reachable",
            nat_set(),
        ),
    ] {
        let keys: &'static [&'static str] = if key == "primitive.graph_successors" {
            &["primitive.graph_successors"]
        } else {
            &["primitive.graph_reachable"]
        };
        out.push(one(
            keys,
            module,
            name,
            vec![param("g", graph()), param("start", nat_t())],
            result.clone(),
            prim(operation, vec![var("g"), var("start")], result),
            STD,
        ));
    }
    out.push(one(
        &["primitive.graph_topological"],
        "ProbeGraphTopological",
        "topologicalRoot",
        vec![param("g", graph())],
        option_t(list_t(nat_t())),
        prim(
            "graph_topological",
            vec![var("g")],
            option_t(list_t(nat_t())),
        ),
        STD,
    ));
    out
}

fn tex(module: &str, declarations: &[Json]) -> String {
    let data = json!({"declarations": declarations, "spec": "lexlean/semantic-module/2"});
    format!(
        "\\begin{{lexlean}}{{{module}}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\n\\semanticdata{{{data}}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
    )
}

/// The probe project, with a module per probe.
#[must_use]
pub fn probe_project(probes: &[Probe]) -> P {
    let project = P::negative("lowering-size-limit");
    let mut modules: Vec<&str> = probes.iter().map(|probe| probe.module).collect();
    modules.sort_unstable();
    let entrypoints: Vec<String> = modules
        .iter()
        .map(|module| format!("\"src/{module}.lex.tex\""))
        .collect();
    let toml = project
        .read("lexlean.toml")
        .replace(
            "module_prefix = \"Production\"",
            "module_prefix = \"OracleProbe\"",
        )
        .replace(
            "entrypoints = [\"src/Main.lex.tex\"]",
            &format!("entrypoints = [{}]", entrypoints.join(", ")),
        );
    project.write("lexlean.toml", &toml);
    let _ = std::fs::remove_file(project.root.join("src/Main.lex.tex").as_std_path());
    for probe in probes {
        project.write(
            &format!("src/{}.lex.tex", probe.module),
            &tex(probe.module, &probe.declarations),
        );
    }
    project.relock();
    project
}

/// The named declarations of a semantic module's text that carry a
/// production block, and the text with every block removed.
fn strip_production(text: &str) -> (String, Vec<String>) {
    let Ok(mut data) = crate::support::try_module_json(text) else {
        return (text.to_owned(), Vec::new());
    };
    let mut roots = Vec::new();
    if let Some(declarations) = data.get_mut("declarations").and_then(Json::as_array_mut) {
        for declaration in declarations {
            if let Some(object) = declaration.as_object_mut() {
                if object.remove("production").is_some() {
                    if let Some(name) = object.get("name").and_then(Json::as_str) {
                        roots.push(name.to_owned());
                    }
                }
            }
        }
    }
    (crate::support::with_module_json(text, &data), roots)
}

/// The rejection project of a negative fixture: its sources with the
/// production marking removed, relocked, and the roots that carried one.
///
/// # Panics
///
/// Panics when the fixture is not a project of semantic modules.
#[must_use]
pub fn rejection_project(fixture: &str) -> (P, Vec<(String, String)>) {
    let project = P::negative(fixture);
    let mut roots = Vec::new();
    let sources = project.root.join("src");
    for entry in std::fs::read_dir(sources.as_std_path()).expect("sources") {
        let path = entry.expect("entry").path();
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned());
        let Some(name) = name.filter(|n| n.ends_with(".lex.tex")) else {
            continue;
        };
        let text = std::fs::read_to_string(&path).expect("source");
        let (stripped, marked) = strip_production(&text);
        std::fs::write(&path, stripped).expect("write");
        let module = name.trim_end_matches(".lex.tex").to_owned();
        roots.extend(marked.into_iter().map(|root| (module.clone(), root)));
    }
    project.relock();
    (project, roots)
}

/// The diagnostic LexLean's fixture records for a rejection: its code and
/// message.
///
/// # Panics
///
/// Panics when the fixture records no diagnostic.
#[must_use]
pub fn recorded_rejection(fixture: &str) -> (String, String) {
    let path = repo_root()
        .join("tests/negative")
        .join(fixture)
        .join("expected/diagnostics.json");
    let diagnostics: Json =
        serde_json::from_slice(&std::fs::read(path.as_std_path()).expect("diagnostics"))
            .expect("json");
    let first = diagnostics
        .as_array()
        .and_then(|all| all.first())
        .expect("a recorded diagnostic");
    (
        first["code"].as_str().expect("code").to_owned(),
        first["message"].as_str().expect("message").to_owned(),
    )
}

/// Every project of the corpus.
#[must_use]
pub fn corpora() -> Vec<Corpus> {
    let mut out: Vec<Corpus> = EXAMPLES
        .iter()
        .map(|name| Corpus {
            name: (*name).to_owned(),
            kind: Kind::Example,
            project: P::copy_example(name),
        })
        .collect();
    out.push(Corpus {
        name: "probes".to_owned(),
        kind: Kind::Probes,
        project: probe_project(&probes()),
    });
    out.extend(REJECTIONS.iter().map(|name| Corpus {
        name: (*name).to_owned(),
        kind: Kind::Rejection((*name).to_owned()),
        project: rejection_project(name).0,
    }));
    out
}
