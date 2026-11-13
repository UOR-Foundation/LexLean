//! `BS-37` (SPEC.md §17.23, §17.21): the reader of the semantic notation as
//! language-1.2 source, `LexLeanBoot.SemanticRead`, held to the byte to
//! `lexlean::surface::semantic::parse_at`, `parse_module`, and
//! `declaration_spans`.
//!
//! The reader lexes the text with `lexText` of `BS-22`, so that a lexical error
//! comes first, and reads the tokens with an explicit stack of frames under
//! `iterate_until`, one frame for each construct the oracle reads by recursion
//! (an array, an object's members, each readable form of a term and a type, a
//! parameter, a list of type arguments), at the positions the schema gives,
//! with the oracle's nesting depths; values with no readable form are read by
//! the generic reader of `BS-24`. It admits no `recursion`.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::artifact::canonical_json::Json;
use lexlean::calculus::Value;
use lexlean::surface::module::{locate, SEMANTIC};
use lexlean::surface::semantic::{
    declaration_spans, parse_at, parse_module, print_at, schema_samples, Position,
};
use lexlean::surface::SurfaceError;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, nat_val, pair, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SemanticRead";
const SOURCE: &str = "src/SemanticRead.lex.tex";
const JSON_SOURCE: &str = "stdlib/src/Json.lex.tex";
const SCHEMA: &str = "schemas/semantic-module-v2.schema.json";
/// The schema tag a module read here is made under.
const SPEC: &str = "lexlean/semantic-module/2";
const ROOTS: [(&str, &[&str]); 3] = [
    ("parseAt", &["allocation", "overflow"]),
    ("parseSemanticModule", &["allocation", "overflow"]),
    ("semanticSpans", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Committed modules whose notation is at most this many bytes are read and
/// spanned whole under the evaluator, and under rustc those at most the second.
const EVALUATOR_MODULE: usize = 1_200;
const RUSTC_MODULE: usize = 6_000;
/// Declarations of the committed modules printed and read back one at a time:
/// at most this many bytes of notation, one in every so many, under the
/// evaluator.
const EVALUATOR_DECLARATION: usize = 1_200;
const EVALUATOR_EVERY: usize = 25;
/// Under rustc, one declaration in every so many: each run carries the
/// schema, so the corpus is sampled to stay within the memory of a host.
const RUSTC_EVERY: usize = 12;
/// Seeded edits of printed declarations under the evaluator and under rustc.
const EVALUATOR_SEEDED: usize = 150;
const RUSTC_SEEDED: usize = 400;

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

fn failure_value(failure: &SurfaceError) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![nat_val(failure.offset), text(&failure.message)],
    }
}

fn outcome<T>(
    result: &Result<T, SurfaceError>,
    ok: impl Fn(&T) -> Value,
) -> (Value, Option<String>) {
    match result {
        Ok(value) => (
            Value::Ok {
                value: Box::new(ok(value)),
            },
            None,
        ),
        Err(failure) => (
            Value::Error {
                value: Box::new(failure_value(failure)),
            },
            Some(failure.message.clone()),
        ),
    }
}

fn at_run(source: &str, position: Position) -> (Run, Option<String>) {
    let (value, refusal) = outcome(&parse_at(source, position), enc_json);
    (
        (
            vec![
                schema_value().clone(),
                text(source),
                nat_val(position_code(position)),
            ],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

fn module_run(source: &str) -> (Run, Option<String>) {
    let (value, refusal) = outcome(&parse_module(source, SPEC), enc_json);
    (
        (
            vec![schema_value().clone(), text(source), text(SPEC)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

fn spans_run(source: &str) -> (Run, Option<String>) {
    let (value, refusal) = outcome(&declaration_spans(source), |spans| {
        list(
            spans
                .iter()
                .map(|(start, end)| pair(nat_val(*start), nat_val(*end)))
                .collect(),
        )
    });
    (
        (
            vec![schema_value().clone(), text(source)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

// ------------------------------------------------------------------ vectors

/// What a vector runs: a value at a position, a module, or the spans of one.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Root {
    At(Position),
    Module,
    Spans,
}

/// One named vector: a text and the root it is read by.
struct Vector {
    label: String,
    source: String,
    root: Root,
}

impl Vector {
    fn index(&self) -> usize {
        match self.root {
            Root::At(_) => 0,
            Root::Module => 1,
            Root::Spans => 2,
        }
    }

    fn run(&self) -> (Run, Option<String>) {
        match self.root {
            Root::At(position) => at_run(&self.source, position),
            Root::Module => module_run(&self.source),
            Root::Spans => spans_run(&self.source),
        }
    }
}

const TERMS: &[(&str, &str)] = &[
    ("a variable", "x"),
    ("a string", "\"a\\nb\""),
    ("true", "true"),
    ("a numeral", "42"),
    ("a numeral with a leading zero", "042"),
    ("bytes", "0x4a4b"),
    ("empty bytes", "0x"),
    ("bytes of odd length", "0x4a4"),
    ("bytes in capitals", "0x4A"),
    ("a keyword as a term", "match"),
    ("a word that is no term", "#"),
    ("a delimiter as a term", "]"),
    ("nothing as a term", ""),
    ("text after a term", "x y"),
    ("unit", "()"),
    ("an integer", "(-3 : Int8)"),
    ("an integer minus zero", "(-0 : Int)"),
    ("an integer with a leading zero", "(07 : Int)"),
    ("an integer of no integer type", "(3 : Nat)"),
    ("an integer of no type", "(3 : )"),
    ("a sum", "(a + 1)"),
    (
        "each operator",
        "((a = b) || ((a <= b) && ((a < b) <-> ((a =? b) /\\ ((a <=? b) -> (a <? b))))))",
    ),
    ("an implication", "(p -> q)"),
    ("a pair", "(a, b)"),
    ("a pair of variables", "(a, (b, c))"),
    ("a call of two arguments", "(a b c)"),
    ("an operand and a bad operator", "(1 * 2)"),
    ("not, first and second", "(not (first (second p)))"),
    ("a list", "(cons h (nil Nat))"),
    ("an if", "(if c then a else b)"),
    ("an if without then", "(if c a else b)"),
    ("an if without else", "(if c then a b)"),
    ("a let", "(let x : Nat := 1; x)"),
    ("a let without its value", "(let x : Nat 1; x)"),
    ("a let without its semicolon", "(let x : Nat := 1 x)"),
    (
        "a let with a generic binder",
        "(let {name: x, type: nat{}} := 1; x)",
    ),
    ("a let whose binder has no type", "(let x := 1; x)"),
    ("a let whose binder is no name", "(let 1 : Nat := 1; x)"),
    ("a forall", "(forall (n : Nat), (n = n))"),
    ("a forall without its parenthesis", "(forall n : Nat, n)"),
    ("a forall without its close", "(forall (n : Nat, n)"),
    ("a forall without its comma", "(forall (n : Nat) n)"),
    ("a function", "(fun (x : Nat) (y : Bool) => x)"),
    ("a function with captures", "(fun [a, b] (x : Nat) => a)"),
    ("a function without parameters", "(fun => x)"),
    ("a function without its arrow", "(fun (x : Nat) x)"),
    (
        "a function with an unclosed parameter",
        "(fun (x : Nat => x)",
    ),
    ("a function capturing a numeral", "(fun [1] (x : Nat) => x)"),
    (
        "a function with unclosed captures",
        "(fun [a (x : Nat) => x)",
    ),
    ("an application", "(apply f a b)"),
    ("an application without its close", "(apply f a"),
    ("a projection", "(project r size)"),
    ("a projection of no field", "(project r 1)"),
    ("a call", "(f a b)"),
    ("a call into a module", "(M::f a)"),
    ("a call with a bad module path", "(M:: a)"),
    ("a call with type arguments", "(f [Nat, Bool] a)"),
    ("a call with no type arguments", "(f [] a)"),
    ("a call with unclosed type arguments", "(f [Nat a)"),
    ("a constructor", "(ctor Option.some [Nat] 1)"),
    ("a constructor of no name", "(ctor 1)"),
    ("a function reference", "(ref f [Nat])"),
    ("a bare function reference", "(ref M::f)"),
    ("a record", "(record R [Nat] {a := 1, b := ()})"),
    ("a record with a trailing comma", "(record R {a := 1,})"),
    ("a record without fields", "(record R {})"),
    ("a record without its brace", "(record R a := 1)"),
    ("a record field without its assignment", "(record R {a 1})"),
    ("a record of no field name", "(record R {1 := 1})"),
    ("a record not closed", "(record R {a := 1 b := 2})"),
    ("a primitive", "(prim append a b : Bytes)"),
    ("a primitive without its result", "(prim append a)"),
    ("a primitive at the end", "(prim append a"),
    ("a primitive of no operation", "(prim 1 : Nat)"),
    (
        "a checked application",
        "(checked M [Nat] a b with [c1, c2])",
    ),
    ("a checked application without with", "(checked M a)"),
    (
        "a checked application without its checks",
        "(checked M a with c)",
    ),
    (
        "a checked application of a numeral check",
        "(checked M a with [1])",
    ),
    (
        "a match",
        "(match o | Option.some v => v | Option.none => 0)",
    ),
    ("a match without branches", "(match o)"),
    ("a match with a numeral binder", "(match o | C 1 => x)"),
    ("a match without its arrow", "(match o | C x"),
    (
        "a match written generically",
        "(match{scrutinee: x, branches: []} + 1)",
    ),
    ("a tagged term", "var{name: if}"),
    ("a tagged term with a member twice", "var{name: x, name: y}"),
    ("a tagged term without a member name", "var{1: x}"),
    ("a tagged term without a colon", "var{name x}"),
    ("a tagged term not closed", "var{name: x y: z}"),
    (
        "a member written generically",
        "call{function: {name: f}, arguments: []}",
    ),
    ("a term deeper than the notation reads", ""),
    ("a term at the limit of the notation", ""),
    ("a string that is not closed", "\"abc"),
];

const TYPES: &[(&str, &str)] = &[
    ("the type Nat", "Nat"),
    ("every nullary type", "(Fn (Bool Bytes Int Int8 Int16 Int32 Int64 Nat Ordering Prop String Type UInt8 UInt16 UInt32 UInt64 Unit ContractViolation ReasoningFailure) Unit)"),
    ("a type parameter", "'T"),
    ("a type parameter that is no name", "'1"),
    ("a head without arguments", "List"),
    ("a named type", "Doc"),
    ("a named type of a module", "M::Doc"),
    ("a named type with arguments", "(Pair Nat 'T)"),
    ("a list, a set and an option", "((List Nat) * ((Set Bool) * (Option Nat)))"),
    ("a result type", "(Result Nat String)"),
    ("a map type", "(Map String Bytes)"),
    ("a function type", "(Fn (Nat Bool) Nat)"),
    ("a function type without its parameters", "(Fn Nat Nat)"),
    ("a product of named types", "(Doc * M::Doc)"),
    ("a product without its star", "(Nat Nat)"),
    ("a type not closed", "(List Nat"),
    ("a delimiter as a type", ","),
    ("nothing as a type", ""),
    ("a tagged type", "nat{}"),
    ("a type deeper than the notation reads", ""),
];

/// `(not ` `depth` times around `x`, closed.
fn nested_term(depth: usize) -> String {
    format!("{}x{}", "(not ".repeat(depth), ")".repeat(depth))
}

fn named() -> Vec<Vector> {
    let mut out = Vec::new();
    for (label, source) in TERMS {
        let source = match *label {
            "a term deeper than the notation reads" => nested_term(300),
            "a term at the limit of the notation" => nested_term(255),
            _ => (*source).to_owned(),
        };
        out.push(Vector {
            label: (*label).to_owned(),
            source,
            root: Root::At(Position::Term),
        });
    }
    for (label, source) in TYPES {
        let source = if *label == "a type deeper than the notation reads" {
            format!("{}Nat{}", "(List ".repeat(300), ")".repeat(300))
        } else {
            (*source).to_owned()
        };
        out.push(Vector {
            label: (*label).to_owned(),
            source,
            root: Root::At(Position::Type),
        });
    }
    for (label, source) in [
        ("a module and `::` without a name", "(List M::)"),
        ("a module and `::` at the end", "M::"),
    ] {
        out.push(Vector {
            label: label.to_owned(),
            source: source.to_owned(),
            root: Root::At(Position::Type),
        });
    }
    let declaration =
        "definition{name: f, parameters: [x : Nat], result: Nat, body: x, executable: true}";
    for (label, source, root) in [
        (
            "a declaration",
            declaration.to_owned(),
            Root::At(Position::Declaration),
        ),
        (
            "a parameter written generically",
            "definition{name: f, parameters: [{name: x, type: Nat}], result: Nat, body: x}"
                .to_owned(),
            Root::At(Position::Declaration),
        ),
        (
            "a declaration with an array of items",
            "definition{name: f, parameters: [x : Nat, y : Bool,], result: Nat, body: x}"
                .to_owned(),
            Root::At(Position::Declaration),
        ),
        (
            "an array not closed",
            "definition{name: f, parameters: [x : Nat y : Bool]}".to_owned(),
            Root::At(Position::Declaration),
        ),
        (
            "a declaration without a kind",
            "{name: f}".to_owned(),
            Root::At(Position::Declaration),
        ),
        (
            "a declaration of a big integer",
            "definition{name: f, size: 99999999999999999999}".to_owned(),
            Root::At(Position::Declaration),
        ),
        (
            "a declaration of a word that is no name",
            "definition{name: 1x}".to_owned(),
            Root::At(Position::Declaration),
        ),
        (
            "a declaration of an empty value",
            "definition{name: )}".to_owned(),
            Root::At(Position::Declaration),
        ),
        (
            "a declaration cut short",
            "definition{name:".to_owned(),
            Root::At(Position::Declaration),
        ),
        (
            "a parameter without its colon",
            "definition{name: f, parameters: [x Nat]}".to_owned(),
            Root::At(Position::Declaration),
        ),
        (
            "a parameter of no name",
            "definition{name: f, parameters: [1 : Nat]}".to_owned(),
            Root::At(Position::Declaration),
        ),
        ("a proof", "decide{}".to_owned(), Root::At(Position::Proof)),
        (
            "a term at the proof position",
            "x".to_owned(),
            Root::At(Position::Proof),
        ),
        (
            "two declarations",
            format!("{declaration}\n\n{declaration}\n"),
            Root::Module,
        ),
        ("no declarations", "  \n".to_owned(), Root::Module),
        (
            "a module with a lexical error",
            "definition{name: \"f}".to_owned(),
            Root::Module,
        ),
        (
            "a module with a bad declaration",
            format!("{declaration}\nx"),
            Root::Module,
        ),
        (
            "two declarations spanned",
            format!("{declaration}\n\n  {declaration}  \n"),
            Root::Spans,
        ),
        (
            "a declaration after white space spanned",
            format!("\n\n {declaration}"),
            Root::Spans,
        ),
        ("nothing spanned", String::new(), Root::Spans),
        (
            "a bad declaration spanned",
            format!("{declaration} ]"),
            Root::Spans,
        ),
    ] {
        out.push(Vector {
            label: label.to_owned(),
            source,
            root,
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
            source: print_at(&value, position),
            root: Root::At(position),
        });
    }
    out
}

/// The refusals a vector must reach, each by a part of its message.
const REFUSALS: &[&str] = &[
    "nested more deeply than the notation reads",
    "expected a member name",
    "expected `:` after a member name",
    "is written twice",
    "expected `,` or `}`",
    "expected `,` or `]`",
    "does not fit a 64-bit integer",
    "is not a name; quote it to write a string",
    "expected a value, found",
    "expected a name, found",
    "expected a name after `::`, found",
    "expected a parameter name, found",
    "expected a field name, found",
    "expected a primitive operation, found",
    "expected a binder name, found",
    "expected an integer type, found",
    "expected a check, found",
    "expected `:` after a parameter name",
    "expected a type, found",
    "takes arguments: write",
    "expected `(` before the parameter types",
    "expected `*` between the two types of a product",
    "expected `)`",
    "has a leading zero",
    "a byte string is `0x`",
    "is a keyword; write `var{name:",
    "is not a term",
    "expected a term, found",
    "is not an integer type",
    "is not a canonical integer",
    "expected `then`",
    "expected `else`",
    "expected `:=`",
    "expected `;`",
    "expected `(` before the bound variable",
    "expected `,` before the body",
    "a function takes at least one parameter",
    "expected `=>`",
    "expected `{` before the fields",
    "expected `:=` after a field name",
    "expected `:` and the result type",
    "a match has at least one `| …` branch",
    "expected an operator or `,` after the first operand",
    "expected `[`",
    "unexpected text after the value",
    "unterminated string",
];

/// The committed semantic modules written in the notation: their bodies.
fn committed() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (path, content) in super::scan::sources() {
        let Some(found) = locate(&content) else {
            continue;
        };
        if !found.kind.is(&SEMANTIC) || found.data.is_some() {
            continue;
        }
        out.push((path, content[found.body.0..found.body.1].to_owned()));
    }
    out
}

// ------------------------------------------------------------------- seeded

const PIECES: &[&str] = &[
    "(", ")", "[", "]", "{", "}", ",", ":", ":=", "::", ";", " ", "x", "Nat", "if", "then",
    "match", "|", "=>", "fun", "let", "*", "+", "->", "\"s\"", "0x", "01", "-0", "nat{}", "List",
];

fn edited(seeded: &mut Seeded, source: &str) -> String {
    let mut scalars: Vec<char> = source.chars().collect();
    for _ in 0..=seeded.below(2) {
        let at = seeded.below(scalars.len() + 1);
        let piece: Vec<char> = seeded.pick(PIECES).chars().collect();
        match seeded.below(3) {
            0 if at < scalars.len() => {
                let to = (at + 1 + seeded.below(4)).min(scalars.len());
                drop(scalars.drain(at..to));
            }
            1 => drop(scalars.splice(at..at, piece)),
            _ if at < scalars.len() => drop(scalars.splice(at..=at, piece)),
            _ => scalars.extend(piece),
        }
    }
    scalars.into_iter().collect()
}

fn seeded_runs(count: usize, bases: &[String], seed: u64) -> Vec<Run> {
    let mut seeded = Seeded(seed);
    (0..count)
        .map(|_| {
            let base = seeded.pick(bases).clone();
            at_run(&edited(&mut seeded, &base), Position::Declaration).0
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
        label: "terms nest past the bound",
        site: "(if (256 <? depth) then (haltAt toks size st position (lit \"nested more deeply than the notation reads\")) else (let token : Nat := (code toks position); (let word : Bytes := (txt toks position); (if (token =? 11)",
        replacement: "(if (512 <? depth) then (haltAt toks size st position (lit \"nested more deeply than the notation reads\")) else (let token : Nat := (code toks position); (let word : Bytes := (txt toks position); (if (token =? 11)",
        vector: "a term deeper than the notation reads",
    },
    Plant {
        label: "the bound on terms is one short",
        site: "(if (256 <? depth) then (haltAt toks size st position (lit \"nested more deeply than the notation reads\")) else (let token : Nat := (code toks position); (let word : Bytes := (txt toks position); (if (token =? 11)",
        replacement: "(if (255 <? depth) then (haltAt toks size st position (lit \"nested more deeply than the notation reads\")) else (let token : Nat := (code toks position); (let word : Bytes := (txt toks position); (if (token =? 11)",
        vector: "a term at the limit of the notation",
    },
    Plant {
        label: "text after a value is read",
        site: "(haltAt toks size st position (lit \"unexpected text after the value\"))",
        replacement: "(finished st value (nil (Nat * Nat)))",
        vector: "text after a term",
    },
    Plant {
        label: "a member may be written twice",
        site: "(if (SurfaceGeneric::hasKey (project top members) key) then",
        replacement: "(if false then",
        vector: "a tagged term with a member twice",
    },
    Plant {
        label: "a tag is no member",
        site: "(cons ((lit \"kind\"), (str word)) (nil (Bytes * LexLeanStd.Json::Json)))",
        replacement: "(nil (Bytes * LexLeanStd.Json::Json))",
        vector: "a tagged term",
    },
    Plant {
        label: "a generic parameter has no schema",
        site: "| Pos.PParam => (defNamed schema (lit \"parameter\"))",
        replacement: "| Pos.PParam => (ctor Option.none [LexLeanStd.Json::Json])",
        vector: "a parameter written generically",
    },
    Plant {
        label: "array items are at no position",
        site: "| Pos.PNode node => (itemsOf schema node)",
        replacement: "| Pos.PNode node => (ctor Pos.PAny)",
        vector: "a declaration",
    },
    Plant {
        label: "a trailing comma in an array is refused",
        site: "(if ((code toks (position + 1)) =? 3) then (give st (position + 2) (arr (project collected values)))",
        replacement: "(if false then (give st (position + 2) (arr (project collected values)))",
        vector: "a declaration with an array of items",
    },
    Plant {
        label: "a reference to a term is no term",
        site: "(if (same segment (lit \"term\")) then (ctor Pos.PTerm)",
        replacement: "(if (same segment (lit \"term\")) then (ctor Pos.PAny)",
        vector: "a declaration",
    },
    Plant {
        label: "a member written generically is read deeper",
        site: "(startGeneric toks size st 0)",
        replacement: "(startGeneric toks size st 257)",
        vector: "a member written generically",
    },
    Plant {
        label: "Nat reads as int",
        site: "(if (same word (lit \"Nat\")) then (ctor Option.some [Bytes] (lit \"nat\"))",
        replacement: "(if (same word (lit \"Nat\")) then (ctor Option.some [Bytes] (lit \"int\"))",
        vector: "the type Nat",
    },
    Plant {
        label: "a head needs no arguments",
        site: "(if (listed word (heads)) then",
        replacement: "(if false then",
        vector: "a head without arguments",
    },
    Plant {
        label: "a type parameter is a named type",
        site: "(both (LexLeanStd.Text::startsWith word (lit \"'\")) (SurfaceGeneric::isBare (tailFrom word 1)))",
        replacement: "false",
        vector: "a type parameter",
    },
    Plant {
        label: "a product needs no star",
        site: "(if (wordIs toks position (lit \"*\")) then",
        replacement: "(if true then",
        vector: "a product without its star",
    },
    Plant {
        label: "the parameter types begin at their parenthesis",
        site: "(fnParams toks size st (frame 14 depth) (position + 2))",
        replacement: "(fnParams toks size st (frame 14 depth) (position + 1))",
        vector: "a function type",
    },
    Plant {
        label: "a named type may be the left of a product",
        site: "(let infixAfter : Bool := (wordIs toks (position + after) (lit \"*\"));",
        replacement: "(let infixAfter : Bool := false;",
        vector: "a product of named types",
    },
    Plant {
        label: "ok and error are swapped",
        site: "(kindNode (lit \"result\") (two \"ok\" (project top held) \"error\" value))",
        replacement: "(kindNode (lit \"result\") (two \"error\" (project top held) \"ok\" value))",
        vector: "a result type",
    },
    Plant {
        label: "a leading zero is a numeral",
        site: "(if (both (1 <? (prim length word : Nat)) (LexLeanStd.Text::startsWith word (lit \"0\"))) then (halt st offset",
        replacement: "(if false then (halt st offset",
        vector: "a numeral with a leading zero",
    },
    Plant {
        label: "odd hexadecimal is bytes",
        site: "(both (stop =? (prim length text : Nat)) ((prim remainder (prim length text : Nat) 2 0 : Nat) =? 0))",
        replacement: "(stop =? (prim length text : Nat))",
        vector: "bytes of odd length",
    },
    Plant {
        label: "a keyword is no term",
        site: "(if (listed word (reservedTerm)) then (cat",
        replacement: "(if false then (cat",
        vector: "a keyword as a term",
    },
    Plant {
        label: "a let binder is never generic",
        site: "(both (same word (lit \"let\")) (wordIs toks (position + 2) (lit \"name\")))",
        replacement: "false",
        vector: "a let with a generic binder",
    },
    Plant {
        label: "minus zero is canonical",
        site: "(if (if (same word (lit \"-0\")) then true else",
        replacement: "(if (if false then true else",
        vector: "an integer minus zero",
    },
    Plant {
        label: "a comma after a name is a call",
        site: "(if ((code toks (position + after)) =? 6) then true else",
        replacement: "(if false then true else",
        vector: "a pair of variables",
    },
    Plant {
        label: "then may be any word",
        site: "(if (wordIs toks position (lit \"then\")) then",
        replacement: "(if ((code toks position) =? 12) then",
        vector: "an if without then",
    },
    Plant {
        label: "the binder and the value of a let are swapped",
        site: "(kindNode (lit \"let\") (cons ((lit \"body\"), value) (two \"binder\" (project top held) \"value\" (project top heldMore))))",
        replacement: "(kindNode (lit \"let\") (cons ((lit \"body\"), value) (two \"binder\" (project top heldMore) \"value\" (project top held))))",
        vector: "a let",
    },
    Plant {
        label: "the captures are dropped",
        site: "(funParams toks size st (holding (frame 37 depth) (project captures value) (ctor LexLeanStd.Json::Json.JNil)) (project captures pos))",
        replacement: "(funParams toks size st (holding (frame 37 depth) (arr (nil LexLeanStd.Json::Json)) (ctor LexLeanStd.Json::Json.JNil)) (project captures pos))",
        vector: "a function with captures",
    },
    Plant {
        label: "a match needs no branch",
        site: "| List.nil => (haltAt toks size st position (lit \"a match has at least one `| …` branch\"))",
        replacement: "| List.nil => (give st position (kindNode (lit \"match\") (nil (Bytes * LexLeanStd.Json::Json))))",
        vector: "a match without branches",
    },
    Plant {
        label: "a record field needs no comma",
        site: "(if ((code toks position) =? 1) then (recordNext toks size st assigned position)",
        replacement: "(if true then (recordNext toks size st assigned position)",
        vector: "a record not closed",
    },
    Plant {
        label: "a primitive may end at its close",
        site: "(if (if (token =? 99) then true else (token =? 5)) then",
        replacement: "(if (token =? 99) then",
        vector: "a primitive without its result",
    },
    Plant {
        label: "a checked application reads to its close",
        site: "(if (same keyword (lit \"checked\")) then (lit \"with\") else (lit \"\"))",
        replacement: "(lit \"\")",
        vector: "a checked application",
    },
    Plant {
        label: "an unclosed application is refused at the start",
        site: "(halt st size (lit \"expected `)`\"))",
        replacement: "(halt st 0 (lit \"expected `)`\"))",
        vector: "an application without its close",
    },
    Plant {
        label: "an implication has a left and a right",
        site: "(if implication then (lit \"premise\") else (lit \"left\"))",
        replacement: "(lit \"left\")",
        vector: "an implication",
    },
    Plant {
        label: "the module of a member is dropped",
        site: "(objectOf (two \"module\" (project head value) \"name\" (project name value)))",
        replacement: "(objectOf (one \"name\" (project name value)))",
        vector: "a call into a module",
    },
    Plant {
        label: "a span begins at the previous end",
        site: "(push st (worded collected (lit \"\") (SurfaceGeneric::offsetAt toks size position)) position 0 (project top place) 1)",
        replacement: "(push st (worded collected (lit \"\") (lastEnd toks position)) position 0 (project top place) 1)",
        vector: "two declarations spanned",
    },
    Plant {
        label: "a span ends at the next token",
        site: "((project top offset), (lastEnd toks position))",
        replacement: "((project top offset), (SurfaceGeneric::offsetAt toks size position))",
        vector: "two declarations spanned",
    },
    Plant {
        label: "the first span begins at zero",
        site: "(SurfaceGeneric::offsetAt toks size 0)",
        replacement: "0",
        vector: "a declaration after white space spanned",
    },
    Plant {
        label: "empty type arguments are read as none",
        site: "(if ((code toks position) =? 3) then (give st (position + 1) (arr (project top values))) else (wantType st top position))",
        replacement: "(wantType st top position)",
        vector: "a call with no type arguments",
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
        let low = super::lower(&checked, &planted, MODULE, ROOTS[vector.index()].0);
        match super::evaluate(&low, &[vector.run().0], 1).first() {
            Some(first) => eprintln!(
                "BS-37: plant detected: {}: {}",
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
            "BS-37: {}: largest certificate part {largest} bytes",
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
        checked_units.units.contains_key("SemanticRead"),
        "SemanticRead is a linked module of the project"
    );
    eligibility_of(&project, "SemanticRead", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut runs: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut messages = Vec::new();
    for vector in &vectors {
        let (run, refusal) = vector.run();
        messages.extend(refusal);
        runs[vector.index()].push(run);
    }
    for refusal in REFUSALS {
        assert!(
            messages.iter().any(|message| message.contains(refusal)),
            "no vector reaches the refusal `{refusal}`"
        );
    }

    // The committed modules read and spanned whole, and each declaration
    // printed and read back at the declaration position.
    assert_eq!(
        lexlean::ir::semantic::semantic_module_spec(lexlean::LANGUAGE_1_2),
        Some(SPEC),
        "the schema tag of a language-1.2 semantic module"
    );
    let modules = committed();
    assert!(
        modules.len() >= 50,
        "the committed semantic modules in the notation are {}",
        modules.len()
    );
    let mut evaluator: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut compiled: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut bases: Vec<String> = Vec::new();
    let mut declarations = 0usize;
    for (path, body) in &modules {
        let module = parse_module(body, SPEC).unwrap_or_else(|error| panic!("{path}: {error:?}"));
        if body.len() <= EVALUATOR_MODULE {
            evaluator[1].push(module_run(body).0);
            evaluator[2].push(spans_run(body).0);
        }
        if body.len() <= RUSTC_MODULE {
            compiled[1].push(module_run(body).0);
            compiled[2].push(spans_run(body).0);
        }
        let Json::Obj(members) = &module else {
            panic!("{path}: a module is an object");
        };
        let Some(Json::Arr(items)) = members.get("declarations") else {
            panic!("{path}: a module has declarations");
        };
        for (index, declaration) in items.iter().enumerate() {
            let printed = print_at(declaration, Position::Declaration);
            declarations += 1;
            if printed.len() <= EVALUATOR_DECLARATION
                && (index + path.len()).is_multiple_of(EVALUATOR_EVERY)
            {
                evaluator[0].push(at_run(&printed, Position::Declaration).0);
                bases.push(printed.clone());
            }
            if printed.len() <= RUSTC_MODULE && (index + path.len()).is_multiple_of(RUSTC_EVERY) {
                compiled[0].push(at_run(&printed, Position::Declaration).0);
            }
        }
    }
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, &bases, 0x0B55_0037),
            seeded_runs(RUSTC_SEEDED, &bases, 0x0B55_1037),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-37: {} vectors ({} at a position, {} modules, {} spanned), {} committed modules with {} declarations ({} and {} declarations, {} and {} modules under the evaluator and rustc), {} and {} seeded edits ({:?})",
        vectors.len(),
        runs[0].len(),
        runs[1].len(),
        runs[2].len(),
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
                "BS-37: {root}: the evaluator agrees on {} runs ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-37: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut all: [Vec<Run>; 3] = [runs[0].clone(), runs[1].clone(), runs[2].clone()];
    for (sink, more) in all.iter_mut().zip(&compiled) {
        sink.extend(more.iter().cloned());
    }
    all[0].extend(rustc_seeded);
    let total = rustc_differential(
        "BS-37",
        &[
            (ROOTS[0].0, &lowered[ROOTS[0].0], all[0].as_slice()),
            (ROOTS[1].0, &lowered[ROOTS[1].0], all[1].as_slice()),
            (ROOTS[2].0, &lowered[ROOTS[2].0], all[2].as_slice()),
        ],
        "oracle",
    );
    eprintln!("BS-37: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-37") {
        return;
    }
    export(&lowered);
    eprintln!("BS-37: passed in {:?}", started.elapsed());
}
