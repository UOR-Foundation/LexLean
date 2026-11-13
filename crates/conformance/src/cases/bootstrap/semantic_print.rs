//! `BS-36` (SPEC.md §17.23, §17.21): the printer of the semantic notation as
//! language-1.2 source, `LexLeanBoot.SemanticPrint`, held to the byte to
//! `lexlean::surface::semantic::print_at` and `print_module`.
//!
//! The printer walks a value by structural recursion over the standard
//! library's `Json`, at the position the schema gives each value: a term, a
//! type, a member, and a parameter in their readable forms when a form carries
//! the value exactly, and every other value in the generic notation of
//! `BS-24`, tagged by its `kind` and with its members at the positions the
//! schema gives them. The schema is the host's input, as the oracle reads it
//! from the embedded tree.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::artifact::canonical_json::Json;
use lexlean::calculus::Value;
use lexlean::surface::module::{locate, SEMANTIC};
use lexlean::surface::semantic::{parse_module, print_at, print_module, schema_samples, Position};
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{nat_val, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SemanticPrint";
const SOURCE: &str = "src/SemanticPrint.lex.tex";
const JSON_SOURCE: &str = "stdlib/src/Json.lex.tex";
const SCHEMA: &str = "schemas/semantic-module-v2.schema.json";
const ROOTS: [(&str, &[&str]); 2] = [
    ("printAt", &["allocation", "overflow", "recursion"]),
    (
        "printSemanticModule",
        &["allocation", "overflow", "recursion"],
    ),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Committed modules whose notation is at most this many bytes are printed
/// whole under the evaluator, and under rustc those at most the second.
const EVALUATOR_MODULE: usize = 2_500;
const RUSTC_MODULE: usize = 15_000;
/// Declarations of the committed modules printed one at a time: at most this
/// many bytes of notation, one in every so many, under the evaluator.
const EVALUATOR_DECLARATION: usize = 1_200;
const EVALUATOR_EVERY: usize = 25;
/// Under rustc, one declaration in every so many. Each run carries the
/// schema, so the corpus is sampled to stay within the memory of a host.
const RUSTC_EVERY: usize = 6;
/// Seeded edits of declarations under the evaluator and under rustc.
const EVALUATOR_SEEDED: usize = 200;
const RUSTC_SEEDED: usize = 800;

// ---------------------------------------------------------------- the sorts

fn json_constructors() -> &'static Vec<String> {
    static CONSTRUCTORS: OnceLock<Vec<String>> = OnceLock::new();
    CONSTRUCTORS.get_or_init(|| {
        let source = std::fs::read_to_string(repo_root().join(JSON_SOURCE).as_std_path())
            .expect("the standard library's Json");
        let block = source
            .split("inductive{")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the first inductive");
        assert!(block.contains("name: Json,"), "the Json sort");
        block
            .split("{name: ")
            .skip(1)
            .map(|rest| {
                rest.split(',')
                    .next()
                    .expect("a constructor")
                    .trim()
                    .to_owned()
            })
            .collect()
    })
}

fn adt(constructor: &str, fields: Vec<Value>) -> Value {
    let found = json_constructors()
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`Json` has no constructor `{constructor}`"));
    Value::Adt {
        constructor: u64::try_from(found).expect("an index"),
        fields,
    }
}

/// A value as the standard library's `Json`.
fn enc_json(json: &Json) -> Value {
    match json {
        Json::Bool(flag) => adt("JBool", vec![Value::Bool { value: *flag }]),
        Json::Int(number) => adt(
            "JInt",
            vec![
                Value::Bool { value: *number < 0 },
                Value::Nat {
                    value: number.unsigned_abs().to_string(),
                },
            ],
        ),
        Json::Str(value) => adt("JStr", vec![text(value)]),
        Json::Arr(items) => adt(
            "JArr",
            vec![items
                .iter()
                .rev()
                .fold(adt("JNil", Vec::new()), |rest, item| {
                    adt("JItem", vec![enc_json(item), rest])
                })],
        ),
        Json::Obj(members) => adt(
            "JObj",
            vec![members
                .iter()
                .rev()
                .fold(adt("JNil", Vec::new()), |rest, (key, value)| {
                    adt("JMember", vec![text(key), enc_json(value), rest])
                })],
        ),
    }
}

/// The schema the oracle reads, as the host gives it to the roots.
fn schema_value() -> &'static Value {
    static SCHEMA_VALUE: OnceLock<Value> = OnceLock::new();
    SCHEMA_VALUE.get_or_init(|| {
        let bytes = std::fs::read(repo_root().join(SCHEMA).as_std_path()).expect("the schema");
        let embedded = lexlean::embedded::FILES
            .iter()
            .find(|(path, _)| *path == SCHEMA)
            .map(|(_, bytes)| *bytes)
            .expect("the schema is embedded");
        assert_eq!(bytes, embedded, "the committed schema is the embedded one");
        enc_json(&Json::parse(&bytes).expect("the schema is canonical JSON's data model"))
    })
}

fn position_code(position: Position) -> usize {
    match position {
        Position::Declaration => 0,
        Position::Term => 1,
        Position::Type => 2,
        Position::Proof => 3,
    }
}

fn at_run(value: &Json, position: Position) -> Run {
    (
        vec![
            schema_value().clone(),
            enc_json(value),
            nat_val(position_code(position)),
        ],
        serde_json::to_value(text(&print_at(value, position))).expect("a value is JSON"),
    )
}

fn module_run(module: &Json) -> (Run, Option<String>) {
    let (value, refusal) = match print_module(module) {
        Ok(printed) => (
            Value::Ok {
                value: Box::new(text(&printed)),
            },
            None,
        ),
        Err(failure) => (
            Value::Error {
                value: Box::new(Value::Adt {
                    constructor: 0,
                    fields: vec![nat_val(failure.offset), text(&failure.message)],
                }),
            },
            Some(failure.message),
        ),
    };
    (
        (
            vec![schema_value().clone(), enc_json(module)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

// ------------------------------------------------------------------ vectors

fn json(text: &str) -> Json {
    Json::parse(text.as_bytes()).unwrap_or_else(|error| panic!("{text}: {error}"))
}

/// One named vector: a value printed at a position, or a module printed.
struct Vector {
    label: String,
    value: Json,
    /// `None` for a module.
    position: Option<Position>,
}

impl Vector {
    fn root(&self) -> usize {
        usize::from(self.position.is_none())
    }

    fn run(&self) -> Run {
        match self.position {
            Some(position) => at_run(&self.value, position),
            None => module_run(&self.value).0,
        }
    }
}

const TERMS: &[(&str, &str)] = &[
    ("a variable", r#"{"kind":"var","name":"x"}"#),
    (
        "a variable named like a keyword",
        r#"{"kind":"var","name":"match"}"#,
    ),
    ("a variable named false", r#"{"kind":"var","name":"false"}"#),
    ("a variable named with", r#"{"kind":"var","name":"with"}"#),
    (
        "a variable that is no name",
        r#"{"kind":"var","name":"a b"}"#,
    ),
    (
        "a variable with a member more",
        r#"{"kind":"var","name":"x","type":{"kind":"nat"}}"#,
    ),
    ("a natural", r#"{"kind":"nat","value":"42"}"#),
    ("a natural zero", r#"{"kind":"nat","value":"0"}"#),
    (
        "a natural with a leading zero",
        r#"{"kind":"nat","value":"042"}"#,
    ),
    ("a natural that is empty", r#"{"kind":"nat","value":""}"#),
    ("a natural with a sign", r#"{"kind":"nat","value":"-1"}"#),
    ("true", r#"{"kind":"bool","value":true}"#),
    ("false", r#"{"kind":"bool","value":false}"#),
    (
        "a Boolean that is a string",
        r#"{"kind":"bool","value":"true"}"#,
    ),
    ("a string", r#"{"kind":"string","value":"a \"b\"\n"}"#),
    (
        "a string that is a number",
        r#"{"kind":"string","value":3}"#,
    ),
    ("bytes", r#"{"kind":"bytes","hex":"4a4b"}"#),
    ("empty bytes", r#"{"kind":"bytes","hex":""}"#),
    ("bytes of odd length", r#"{"kind":"bytes","hex":"4a4"}"#),
    ("bytes in capitals", r#"{"kind":"bytes","hex":"4A"}"#),
    ("unit", r#"{"kind":"unit"}"#),
    ("unit with a member", r#"{"kind":"unit","value":1}"#),
    (
        "an integer",
        r#"{"kind":"integer","representation":"int8","value":"-3"}"#,
    ),
    (
        "an unsigned integer",
        r#"{"kind":"integer","representation":"uint64","value":"18"}"#,
    ),
    (
        "an integer minus zero",
        r#"{"kind":"integer","representation":"int","value":"-0"}"#,
    ),
    (
        "an integer with a leading zero",
        r#"{"kind":"integer","representation":"int","value":"07"}"#,
    ),
    (
        "an integer of no representation",
        r#"{"kind":"integer","representation":"int128","value":"1"}"#,
    ),
    (
        "a sum",
        r#"{"kind":"add","left":{"kind":"var","name":"a"},"right":{"kind":"nat","value":"1"}}"#,
    ),
    (
        "an implication",
        r#"{"kind":"implies","premise":{"kind":"var","name":"p"},"conclusion":{"kind":"var","name":"q"}}"#,
    ),
    (
        "an implication with left and right",
        r#"{"kind":"implies","left":{"kind":"var","name":"p"},"right":{"kind":"var","name":"q"}}"#,
    ),
    (
        "a conjunction of propositions",
        r#"{"kind":"prop_and","left":{"kind":"var","name":"p"},"right":{"kind":"var","name":"q"}}"#,
    ),
    (
        "each comparison",
        r#"{"kind":"or","left":{"kind":"and","left":{"kind":"beq","left":{"kind":"var","name":"a"},"right":{"kind":"var","name":"b"}},"right":{"kind":"ble","left":{"kind":"var","name":"a"},"right":{"kind":"var","name":"b"}}},"right":{"kind":"iff","left":{"kind":"blt","left":{"kind":"var","name":"a"},"right":{"kind":"var","name":"b"}},"right":{"kind":"le","left":{"kind":"lt","left":{"kind":"var","name":"a"},"right":{"kind":"var","name":"b"}},"right":{"kind":"eq","left":{"kind":"var","name":"a"},"right":{"kind":"var","name":"b"}}}}}"#,
    ),
    (
        "a sum with a member more",
        r#"{"kind":"add","left":{"kind":"var","name":"a"},"right":{"kind":"var","name":"b"},"z":1}"#,
    ),
    (
        "a pair",
        r#"{"kind":"pair","left":{"kind":"var","name":"a"},"right":{"kind":"unit"}}"#,
    ),
    (
        "not, first and second",
        r#"{"kind":"not","value":{"kind":"first","value":{"kind":"second","value":{"kind":"var","name":"p"}}}}"#,
    ),
    (
        "a list",
        r#"{"kind":"cons","head":{"kind":"var","name":"h"},"tail":{"kind":"nil","element":{"kind":"nat"}}}"#,
    ),
    (
        "an if",
        r#"{"kind":"if","condition":{"kind":"var","name":"c"},"then_value":{"kind":"nat","value":"1"},"else_value":{"kind":"nat","value":"2"}}"#,
    ),
    (
        "a let",
        r#"{"kind":"let","binder":{"name":"x","type":{"kind":"nat"}},"value":{"kind":"nat","value":"1"},"body":{"kind":"var","name":"x"}}"#,
    ),
    (
        "a let whose binder has a member more",
        r#"{"kind":"let","binder":{"name":"x","type":{"kind":"nat"},"z":1},"value":{"kind":"nat","value":"1"},"body":{"kind":"var","name":"x"}}"#,
    ),
    (
        "a let whose binder is no name",
        r#"{"kind":"let","binder":{"name":"true","type":{"kind":"nat"}},"value":{"kind":"nat","value":"1"},"body":{"kind":"var","name":"x"}}"#,
    ),
    (
        "a forall",
        r#"{"kind":"forall","binder":{"name":"n","type":{"kind":"nat"}},"body":{"kind":"eq","left":{"kind":"var","name":"n"},"right":{"kind":"var","name":"n"}}}"#,
    ),
    (
        "a lambda",
        r#"{"kind":"lambda","parameters":[{"name":"x","type":{"kind":"nat"}},{"name":"y","type":{"kind":"bool"}}],"captures":[],"body":{"kind":"var","name":"x"}}"#,
    ),
    (
        "a lambda with captures",
        r#"{"kind":"lambda","parameters":[{"name":"x","type":{"kind":"nat"}}],"captures":["a","b"],"body":{"kind":"var","name":"a"}}"#,
    ),
    (
        "a lambda capturing no name",
        r#"{"kind":"lambda","parameters":[],"captures":["a b"],"body":{"kind":"unit"}}"#,
    ),
    (
        "a lambda with a parameter that is no name",
        r#"{"kind":"lambda","parameters":[{"name":"a b","type":{"kind":"nat"}}],"captures":[],"body":{"kind":"unit"}}"#,
    ),
    (
        "a lambda without parameters",
        r#"{"kind":"lambda","parameters":[],"captures":[],"body":{"kind":"unit"}}"#,
    ),
    (
        "an application",
        r#"{"kind":"apply","function":{"kind":"var","name":"f"},"arguments":[{"kind":"var","name":"a"},{"kind":"var","name":"b"}]}"#,
    ),
    (
        "an application of nothing",
        r#"{"kind":"apply","function":{"kind":"var","name":"f"},"arguments":[]}"#,
    ),
    (
        "a projection",
        r#"{"kind":"project","value":{"kind":"var","name":"r"},"field":"size"}"#,
    ),
    (
        "a projection of no name",
        r#"{"kind":"project","value":{"kind":"var","name":"r"},"field":"a-"}"#,
    ),
    (
        "a call",
        r#"{"kind":"call","function":{"name":"f"},"arguments":[{"kind":"var","name":"a"}]}"#,
    ),
    (
        "a call into a module",
        r#"{"kind":"call","function":{"module":"M","name":"f"},"arguments":[]}"#,
    ),
    (
        "a call with type arguments",
        r#"{"kind":"call","function":{"name":"f"},"type_arguments":[{"kind":"nat"},{"kind":"bool"}],"arguments":[{"kind":"var","name":"a"}]}"#,
    ),
    (
        "a call with no type arguments written",
        r#"{"kind":"call","function":{"name":"f"},"type_arguments":[],"arguments":[]}"#,
    ),
    (
        "a call of a keyword",
        r#"{"kind":"call","function":{"name":"let"},"arguments":[]}"#,
    ),
    (
        "a call whose module is no string",
        r#"{"kind":"call","function":{"module":1,"name":"f"},"arguments":[]}"#,
    ),
    (
        "a call whose function has a member more",
        r#"{"kind":"call","function":{"name":"f","z":1},"arguments":[]}"#,
    ),
    (
        "a constructor",
        r#"{"kind":"constructor","constructor":{"name":"Option.some"},"type_arguments":[{"kind":"nat"}],"arguments":[{"kind":"nat","value":"1"}]}"#,
    ),
    (
        "a constructor without arguments",
        r#"{"kind":"constructor","constructor":{"module":"M","name":"T.C"},"arguments":[]}"#,
    ),
    (
        "a function reference",
        r#"{"kind":"function_ref","function":{"name":"f"},"type_arguments":[{"kind":"nat"}]}"#,
    ),
    (
        "a function reference alone",
        r#"{"kind":"function_ref","function":{"name":"f"}}"#,
    ),
    (
        "a record",
        r#"{"kind":"record","type":{"name":"R"},"fields":[{"field":"a","value":{"kind":"nat","value":"1"}},{"field":"b","value":{"kind":"unit"}}]}"#,
    ),
    (
        "a record of no fields",
        r#"{"kind":"record","type":{"name":"R"},"type_arguments":[{"kind":"nat"}],"fields":[]}"#,
    ),
    (
        "a record whose field is no name",
        r#"{"kind":"record","type":{"name":"R"},"fields":[{"field":"a b","value":{"kind":"unit"}}]}"#,
    ),
    (
        "a record whose assignment has a member more",
        r#"{"kind":"record","type":{"name":"R"},"fields":[{"field":"a","value":{"kind":"unit"},"z":1}]}"#,
    ),
    (
        "a primitive",
        r#"{"kind":"primitive","operation":"append","arguments":[{"kind":"var","name":"a"},{"kind":"var","name":"b"}],"result":{"kind":"bytes"}}"#,
    ),
    (
        "a primitive of no arguments",
        r#"{"kind":"primitive","operation":"zero","arguments":[],"result":{"kind":"nat"}}"#,
    ),
    (
        "a checked application",
        r#"{"kind":"checked_apply","model":{"name":"M"},"type_arguments":[{"kind":"nat"}],"arguments":[{"kind":"var","name":"a"}],"checks":["c1","c2"]}"#,
    ),
    (
        "a checked application of no checks",
        r#"{"kind":"checked_apply","model":{"name":"M"},"arguments":[],"checks":[]}"#,
    ),
    (
        "a checked application whose check is no name",
        r#"{"kind":"checked_apply","model":{"name":"M"},"arguments":[],"checks":["1x"]}"#,
    ),
    (
        "a match",
        r#"{"kind":"match","scrutinee":{"kind":"var","name":"o"},"branches":[{"constructor":{"name":"Option.some"},"binders":["v"],"body":{"kind":"var","name":"v"}},{"constructor":{"name":"Option.none"},"binders":[],"body":{"kind":"nat","value":"0"}}]}"#,
    ),
    (
        "a match too long for a line",
        r#"{"kind":"match","scrutinee":{"kind":"var","name":"scrutineeWithALongName"},"branches":[{"constructor":{"name":"Option.some"},"binders":["valueWithAVeryLongName"],"body":{"kind":"var","name":"valueWithAVeryLongName"}},{"constructor":{"name":"Option.none"},"binders":[],"body":{"kind":"nat","value":"1234567890"}}]}"#,
    ),
    (
        "a match whose binder is a keyword",
        r#"{"kind":"match","scrutinee":{"kind":"var","name":"o"},"branches":[{"constructor":{"name":"C"},"binders":["then"],"body":{"kind":"unit"}}]}"#,
    ),
    (
        "a match of no branches",
        r#"{"kind":"match","scrutinee":{"kind":"var","name":"o"},"branches":[]}"#,
    ),
    (
        "a term of no readable form",
        r#"{"kind":"map_literal","entries":[]}"#,
    ),
    ("a term without a kind", r#"{"name":"x"}"#),
    (
        "a term whose kind is no name",
        r#"{"kind":"a b","name":"x"}"#,
    ),
    ("a term that is a string", r#""text""#),
    ("a term that is an array", r#"[{"kind":"var","name":"x"}]"#),
];

const TYPES: &[(&str, &str)] = &[
    (
        "every nullary type",
        r#"{"kind":"function","parameters":[{"kind":"bool"},{"kind":"bytes"},{"kind":"int"},{"kind":"int8"},{"kind":"int16"},{"kind":"int32"},{"kind":"int64"},{"kind":"nat"},{"kind":"ordering"},{"kind":"prop"},{"kind":"string"},{"kind":"type"},{"kind":"uint8"},{"kind":"uint16"},{"kind":"uint32"},{"kind":"uint64"},{"kind":"unit"},{"kind":"contract_violation"},{"kind":"reasoning_failure"}],"result":{"kind":"unit"}}"#,
    ),
    (
        "a nullary type with a member more",
        r#"{"kind":"nat","z":1}"#,
    ),
    ("a type parameter", r#"{"kind":"parameter","name":"T"}"#),
    (
        "a type parameter that is no name",
        r#"{"kind":"parameter","name":"1"}"#,
    ),
    (
        "a list and a set",
        r#"{"kind":"product","left":{"kind":"list","element":{"kind":"nat"}},"right":{"kind":"set","element":{"kind":"bool"}}}"#,
    ),
    (
        "an option, a result and a map",
        r#"{"kind":"result","ok":{"kind":"option","value":{"kind":"nat"}},"error":{"kind":"map","key":{"kind":"string"},"value":{"kind":"bytes"}}}"#,
    ),
    (
        "a function of no parameters",
        r#"{"kind":"function","parameters":[],"result":{"kind":"nat"}}"#,
    ),
    (
        "a named type",
        r#"{"kind":"named","member":{"name":"Doc"},"arguments":[]}"#,
    ),
    (
        "a named type with arguments",
        r#"{"kind":"named","member":{"module":"M","name":"Pair"},"arguments":[{"kind":"nat"},{"kind":"parameter","name":"T"}]}"#,
    ),
    (
        "a named type spelled like a type",
        r#"{"kind":"named","member":{"name":"List"},"arguments":[]}"#,
    ),
    (
        "a named type spelled Map",
        r#"{"kind":"named","member":{"name":"Map"},"arguments":[]}"#,
    ),
    (
        "a named type spelled like a nullary type",
        r#"{"kind":"named","member":{"name":"Nat"},"arguments":[]}"#,
    ),
    (
        "a named type of a module spelled like a type",
        r#"{"kind":"named","member":{"module":"M","name":"List"},"arguments":[]}"#,
    ),
    (
        "a named type whose member has a member more",
        r#"{"kind":"named","member":{"name":"Doc","z":1},"arguments":[]}"#,
    ),
    (
        "a list with a member more",
        r#"{"kind":"list","element":{"kind":"nat"},"z":1}"#,
    ),
    (
        "a type of no readable form",
        r#"{"kind":"graph","node":{"kind":"nat"}}"#,
    ),
];

/// The committed semantic modules written in the notation, read.
fn committed() -> Vec<(String, Json, usize)> {
    let spec = lexlean::ir::semantic::semantic_module_spec(lexlean::LANGUAGE_1_2).expect("a spec");
    let mut out = Vec::new();
    for (path, content) in super::scan::sources() {
        let Some(found) = locate(&content) else {
            continue;
        };
        if !found.kind.is(&SEMANTIC) || found.data.is_some() {
            continue;
        }
        let body = &content[found.body.0..found.body.1];
        if let Ok(module) = parse_module(body, spec) {
            out.push((path, module, body.len()));
        }
    }
    out
}

fn named() -> Vec<Vector> {
    let mut out = Vec::new();
    for (label, source) in TERMS {
        out.push(Vector {
            label: (*label).to_owned(),
            value: json(source),
            position: Some(Position::Term),
        });
    }
    for (label, source) in TYPES {
        out.push(Vector {
            label: (*label).to_owned(),
            value: json(source),
            position: Some(Position::Type),
        });
    }
    for (label, source, position) in [
        (
            "a declaration",
            r#"{"kind":"definition","name":"f","parameters":[{"name":"x","type":{"kind":"nat"}}],"result":{"kind":"nat"},"body":{"kind":"var","name":"x"},"executable":true}"#,
            Position::Declaration,
        ),
        (
            "a declaration of no kind",
            r#"{"name":"f"}"#,
            Position::Declaration,
        ),
        (
            "a declaration whose parameter is generic",
            r#"{"kind":"definition","name":"f","parameters":[{"name":"x","type":{"kind":"nat"},"z":1}],"result":{"kind":"nat"},"body":{"kind":"unit"}}"#,
            Position::Declaration,
        ),
        ("a proof", r#"{"kind":"decide"}"#, Position::Proof),
        (
            "a term at the proof position",
            r#"{"kind":"var","name":"x"}"#,
            Position::Proof,
        ),
    ] {
        out.push(Vector {
            label: label.to_owned(),
            value: json(source),
            position: Some(position),
        });
    }
    for (label, source) in [
        (
            "a module",
            r#"{"spec":"s","declarations":[{"kind":"definition","name":"a"},{"kind":"definition","name":"b"}]}"#,
        ),
        (
            "a module of no declarations",
            r#"{"spec":"s","declarations":[]}"#,
        ),
        ("a module that is no object", r#"[1]"#),
        ("a module without declarations", r#"{"spec":"s"}"#),
        (
            "a module whose declarations are no array",
            r#"{"spec":"s","declarations":{}}"#,
        ),
        (
            "a module with a member more",
            r#"{"spec":"s","declarations":[],"z":1}"#,
        ),
        (
            "a module whose spec is no string",
            r#"{"spec":1,"declarations":[]}"#,
        ),
        ("a module without a spec", r#"{"declarations":[],"z":1}"#),
    ] {
        out.push(Vector {
            label: label.to_owned(),
            value: json(source),
            position: None,
        });
    }
    // A union may list one kind twice (an `enum` alternative beside a
    // `const` one); the second sample is labelled by its place.
    let mut seen = BTreeSet::new();
    for (position, label, value) in schema_samples() {
        let mut label = format!("the schema sample {label}");
        let mut place = 2;
        while !seen.insert(label.clone()) {
            label = format!("{label} ({place})");
            place += 1;
        }
        out.push(Vector {
            label,
            value,
            position: Some(position),
        });
    }
    out
}

const REFUSALS: &[&str] = &[
    "a module is an object",
    "a module has a `declarations` array",
    "a module has exactly a `spec` and its `declarations`",
];

// ------------------------------------------------------------------- seeded

const NAMES: &[&str] = &[
    "x", "if", "M", "a b", "List", "Nat", "1", "", "f.g", "false",
];

fn edit(seeded: &mut Seeded, value: &Json, depth: usize) -> Json {
    match value {
        Json::Obj(members) => {
            let mut members = members.clone();
            let keys: Vec<String> = members.keys().cloned().collect();
            if !keys.is_empty() && seeded.chance(25) {
                let key = seeded.pick(&keys).clone();
                match seeded.below(4) {
                    0 => {
                        members.remove(&key);
                    }
                    1 => {
                        members.insert(key, Json::Str((*seeded.pick(NAMES)).to_owned()));
                    }
                    2 => {
                        members.insert("z".to_owned(), Json::Int(1));
                    }
                    _ => {
                        members.insert(key, Json::Arr(Vec::new()));
                    }
                }
            } else if !keys.is_empty() && depth < 40 {
                let key = seeded.pick(&keys).clone();
                let inner = edit(seeded, &members[&key], depth + 1);
                members.insert(key, inner);
            }
            Json::Obj(members)
        }
        Json::Arr(items) if !items.is_empty() && depth < 40 => {
            let mut items = items.clone();
            let at = seeded.below(items.len());
            if seeded.chance(20) {
                items.remove(at);
            } else {
                items[at] = edit(seeded, &items[at], depth + 1);
            }
            Json::Arr(items)
        }
        Json::Str(_) if seeded.chance(50) => Json::Str((*seeded.pick(NAMES)).to_owned()),
        other => other.clone(),
    }
}

fn seeded_runs(count: usize, bases: &[Json], seed: u64) -> Vec<Run> {
    let mut seeded = Seeded(seed);
    let positions = [
        Position::Declaration,
        Position::Term,
        Position::Type,
        Position::Proof,
    ];
    (0..count)
        .map(|_| {
            let base = seeded.pick(bases).clone();
            let edited = edit(&mut seeded, &base, 0);
            let position = if seeded.chance(80) {
                Position::Declaration
            } else {
                *seeded.pick(&positions)
            };
            at_run(&edited, position)
        })
        .collect()
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "a keyword may name a variable",
        site: "(both (if (listed name (reservedTerm)) then false else true)",
        replacement: "(both true",
        vector: "a variable named like a keyword",
    },
    Plant {
        label: "a leading zero is canonical",
        site: "(if (LexLeanStd.Text::startsWith text (lit \"0\")) then false else true)",
        replacement: "true",
        vector: "a natural with a leading zero",
    },
    Plant {
        label: "minus zero is canonical",
        site: "(both (canonicalDigits digits) (if (same digits (lit \"0\")) then false else true))",
        replacement: "(canonicalDigits digits)",
        vector: "an integer minus zero",
    },
    Plant {
        label: "capitals are hexadecimal",
        site: "(both (97 <=? code) (code <=? 102))",
        replacement: "(both (65 <=? code) (code <=? 102))",
        vector: "bytes in capitals",
    },
    Plant {
        label: "odd hexadecimal is canonical",
        site: "(both ((prim remainder (prim length text : Nat) 2 0 : Nat) =? 0) (allBytes text 0 true))",
        replacement: "(allBytes text 0 true)",
        vector: "bytes of odd length",
    },
    Plant {
        label: "the integer spelling of int8",
        site: "(ctor Option.some [Bytes] (lit \"Int8\")) else (if (same representation (lit \"int16\"))",
        replacement: "(ctor Option.some [Bytes] (lit \"Int\")) else (if (same representation (lit \"int16\"))",
        vector: "an integer",
    },
    Plant {
        label: "an implication of left and right",
        site: "(if (same kind (lit \"implies\")) then (lit \"premise\") else (lit \"left\"))",
        replacement: "(lit \"left\")",
        vector: "an implication",
    },
    Plant {
        label: "the operator of a conjunction of propositions",
        site: "(lit \"/\\\\\")",
        replacement: "(lit \"&&\")",
        vector: "a conjunction of propositions",
    },
    Plant {
        label: "an extra member keeps the readable form",
        site: "(both ((countOf chain) =? (prim length keys : Nat)) (hasAll chain keys))",
        replacement: "(hasAll chain keys)",
        vector: "a sum with a member more",
    },
    Plant {
        label: "empty type arguments are readable",
        site: "| Option.some items => (0 <? (countOf items))",
        replacement: "| Option.some items => true",
        vector: "a call with no type arguments written",
    },
    Plant {
        label: "a call may name a keyword",
        site: "| Option.some name => (plain name)\n          | Option.none => false)",
        replacement: "| Option.some name => true\n          | Option.none => false)",
        vector: "a call of a keyword",
    },
    Plant {
        label: "the captures are always written",
        site: "(if (0 <? (prim length captures : Nat)) then",
        replacement: "(if true then",
        vector: "a lambda",
    },
    Plant {
        label: "a lambda parameter needs no name",
        site: "(everyItem chain (lit \"parameters\") (fun (parameter : LexLeanStd.Json::Json) => (paramOk parameter)))",
        replacement: "true",
        vector: "a lambda with a parameter that is no name",
    },
    Plant {
        label: "a let binder may have a member more",
        site: "(both ((countOf chain) =? 2) (SurfaceGeneric::isBare name))",
        replacement: "(SurfaceGeneric::isBare name)",
        vector: "a let whose binder has a member more",
    },
    Plant {
        label: "the module of a member is dropped",
        site: "(ctor Option.some [Bytes] (cat (cat prefix (lit \"::\")) name))",
        replacement: "(ctor Option.some [Bytes] name)",
        vector: "a call into a module",
    },
    Plant {
        label: "a local member spelled like a type is readable",
        site: "(if (both (if (present (memberIn inner (lit \"module\"))) then false else true) (reservedType name)) then false else true)",
        replacement: "true",
        vector: "a named type spelled like a type",
    },
    Plant {
        label: "a qualified member spelled like a type is generic",
        site: "(if (present (memberIn inner (lit \"module\"))) then false else true)",
        replacement: "true",
        vector: "a named type of a module spelled like a type",
    },
    Plant {
        label: "Map is no applied type",
        site: "(cons (lit \"Map\") (cons (lit \"Fn\")",
        replacement: "(cons (lit \"Mapx\") (cons (lit \"Fn\")",
        vector: "a named type spelled Map",
    },
    Plant {
        label: "the product is spelled with a comma",
        site: "(tx (lit \" * \"))",
        replacement: "(tx (lit \", \"))",
        vector: "a list and a set",
    },
    Plant {
        label: "the parameter types of a function are comma separated",
        site: "| Pos.PFnTypes => (seq (cons (tx (lit \"(\")) (prim append (interleave docs (lit \" \"))",
        replacement: "| Pos.PFnTypes => (seq (cons (tx (lit \"(\")) (prim append (interleave docs (lit \", \"))",
        vector: "every nullary type",
    },
    Plant {
        label: "a match is not grouped",
        site: "(ctor SurfaceDoc::Doc.DGroup (ctor SurfaceDoc::Doc.DNest 2 (seq (cons (tx (lit \"(match \"))",
        replacement: "(ctor SurfaceDoc::Doc.DNest 2 (ctor SurfaceDoc::Doc.DNest 0 (seq (cons (tx (lit \"(match \"))",
        vector: "a match",
    },
    Plant {
        label: "a match branch is not indented",
        site: "(ctor SurfaceDoc::Doc.DNest 2 (seq (cons (tx (lit \"(match \"))",
        replacement: "(ctor SurfaceDoc::Doc.DNest 0 (seq (cons (tx (lit \"(match \"))",
        vector: "a match too long for a line",
    },
    Plant {
        label: "a record field is written with a colon",
        site: "(tx (cat (strAt chain (lit \"field\")) (lit \" := \")))",
        replacement: "(tx (cat (strAt chain (lit \"field\")) (lit \": \")))",
        vector: "a record",
    },
    Plant {
        label: "the checks are written without with",
        site: "(cons (tx (lit \" with \")) (cons (tx (lit \"[\"))",
        replacement: "(cons (tx (lit \" \")) (cons (tx (lit \"[\"))",
        vector: "a checked application",
    },
    Plant {
        label: "a declaration's members are at no position",
        site: "(itemsOf schema declarations)",
        replacement: "(ctor Pos.PAny)",
        vector: "a declaration",
    },
    Plant {
        label: "the proof position is any",
        site: "| Option.some proof => (ctor Pos.PNode proof)",
        replacement: "| Option.some proof => (ctor Pos.PAny)",
        vector: "the schema sample proof apply (every member)",
    },
    Plant {
        label: "modules are not separated by an empty line",
        site: "(cat (second state) (lit \"\\n\"))",
        replacement: "(second state)",
        vector: "a module",
    },
    Plant {
        label: "the spec need not be a string",
        site: "| Option.some spec => true\n        | Option.none => false)",
        replacement: "| Option.some spec => true\n        | Option.none => true)",
        vector: "a module whose spec is no string",
    },
    Plant {
        label: "a module may have a member more",
        site: "(both ((countOf chain) =? 2) (match (strOf (memberIn chain (lit \"spec\")))",
        replacement: "(both true (match (strOf (memberIn chain (lit \"spec\")))",
        vector: "a module with a member more",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let planted = support::P::bootstrap();
        let source = planted.read_raw(SOURCE);
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(SOURCE, &source.replacen(plant.site, plant.replacement, 1));
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[vector.root()].0);
        match super::evaluate(&low, &[vector.run()], 1).first() {
            Some(first) => eprintln!(
                "BS-36: plant detected: {}: {}",
                plant.label,
                first.chars().take(160).collect::<String>()
            ),
            None => missed.push(plant.label),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects their own vector does not detect: {missed:?}"
    );
}

fn export(lowered: &BTreeMap<&str, super::Lowered>) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some((member, low)) = exported_root
            .root
            .strip_prefix(&format!("{MODULE}."))
            .and_then(|member| lowered.get(member).map(|low| (member, low)))
        else {
            continue;
        };
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-36: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
            let package: Json2 = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&output.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("package JSON");
            assert_eq!(
                package["program"],
                serde_json::to_value(&low.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    let packages = harness::tree(&bundle.join("packages"));
    let committed_packages = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed_packages.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed_packages.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("SemanticPrint"),
        "SemanticPrint is a linked module of the project"
    );
    eligibility_of(&project, "SemanticPrint", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut runs: [Vec<Run>; 2] = [Vec::new(), Vec::new()];
    let mut messages = Vec::new();
    for vector in &vectors {
        if vector.position.is_none() {
            messages.extend(module_run(&vector.value).1);
        }
        runs[vector.root()].push(vector.run());
    }
    for refusal in REFUSALS {
        assert!(
            messages.iter().any(|message| message == refusal),
            "no vector reaches the refusal `{refusal}`"
        );
    }

    let modules = committed();
    assert!(
        modules.len() >= 50,
        "the committed semantic modules in the notation are {}",
        modules.len()
    );
    let mut evaluator: [Vec<Run>; 2] = [Vec::new(), Vec::new()];
    let mut compiled: [Vec<Run>; 2] = [Vec::new(), Vec::new()];
    let mut bases: Vec<Json> = Vec::new();
    let mut declarations = 0usize;
    for (path, module, size) in &modules {
        if *size <= EVALUATOR_MODULE {
            evaluator[1].push(module_run(module).0);
        }
        if *size <= RUSTC_MODULE {
            compiled[1].push(module_run(module).0);
        }
        let Json::Obj(members) = module else {
            panic!("{path}: a module is an object");
        };
        let Some(Json::Arr(items)) = members.get("declarations") else {
            panic!("{path}: a module has declarations");
        };
        for (index, declaration) in items.iter().enumerate() {
            let printed = print_at(declaration, Position::Declaration).len();
            declarations += 1;
            if printed <= EVALUATOR_DECLARATION && (index + path.len()) % EVALUATOR_EVERY == 0 {
                evaluator[0].push(at_run(declaration, Position::Declaration));
                bases.push(declaration.clone());
            }
            if printed <= RUSTC_MODULE && (index + path.len()) % RUSTC_EVERY == 0 {
                compiled[0].push(at_run(declaration, Position::Declaration));
            }
        }
    }
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, &bases, 0x0B55_0036),
            seeded_runs(RUSTC_SEEDED, &bases, 0x0B55_1036),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-36: {} vectors ({} at a position, {} modules), {} committed modules with {} declarations ({} and {} declarations, {} and {} modules under the evaluator and rustc), {} and {} seeded edits ({:?})",
        vectors.len(),
        runs[0].len(),
        runs[1].len(),
        modules.len(),
        declarations,
        evaluator[0].len(),
        compiled[0].len(),
        evaluator[1].len(),
        compiled[1].len(),
        evaluator_seeded.len(),
        rustc_seeded.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        for (index, (root, _)) in ROOTS.iter().enumerate() {
            let mut corpus: Vec<Run> = runs[index].clone();
            corpus.extend(evaluator[index].iter().cloned());
            if index == 0 {
                corpus.extend(evaluator_seeded.iter().cloned());
            }
            let disagreements = super::evaluate(&lowered[root], &corpus, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-36: {root}: the evaluator agrees on {} runs ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-36: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut all: [Vec<Run>; 2] = [runs[0].clone(), runs[1].clone()];
    all[0].extend(compiled[0].iter().cloned());
    all[1].extend(compiled[1].iter().cloned());
    all[0].extend(rustc_seeded);
    let total = rustc_differential(
        "BS-36",
        &[
            (ROOTS[0].0, &lowered[ROOTS[0].0], all[0].as_slice()),
            (ROOTS[1].0, &lowered[ROOTS[1].0], all[1].as_slice()),
        ],
        "oracle",
    );
    eprintln!("BS-36: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-36") {
        return;
    }
    export(&lowered);
    eprintln!("BS-36: passed in {:?}", started.elapsed());
}
