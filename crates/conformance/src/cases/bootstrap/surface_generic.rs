//! `BS-24` (SPEC.md §17.23, §17.21): the generic notation as language-1.2
//! source, `LexLeanBoot.SurfaceGeneric`, held to the byte to
//! `lexlean::surface::generic::{parse, print}`.
//!
//! `parseGeneric` reads a text with `lexText` of `BS-22`, as `Reader::new`
//! does, and then reads the tokens as one value of the notation with an
//! explicit stack of frames, so that it admits no `recursion`; it returns the
//! value as the standard library's `Json` or the offset and message of the
//! first error. `printGeneric` builds the document of a value (`SurfaceDoc`,
//! `BS-23`) and lays it out at the printer's width; it walks the value by the
//! structural recursion the standard library's own writer uses.
//!
//! Each planted defect names the one vector that catches it. `BS-05` and
//! `BS-11` each lost a plant when their corpus grew, because a plant caught by
//! "some run of the corpus" is only as good as that corpus.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::artifact::canonical_json::Json;
use lexlean::calculus::Value;
use lexlean::surface::generic::{parse, print};
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{nat_val, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SurfaceGeneric";
const SOURCE: &str = "bootstrap/src/SurfaceGeneric.lex.tex";
const JSON_SOURCE: &str = "stdlib/src/Json.lex.tex";
const ROOTS: [(&str, &[&str]); 2] = [
    ("parseGeneric", &["allocation", "overflow"]),
    ("printGeneric", &["allocation", "overflow", "recursion"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The committed JSON files read, by size in bytes; the values of those whose
/// printed text is at most the first bound are run under the evaluator, one in
/// `EVALUATOR_EVERY` by a hash of the path, and those within the second bound
/// under rustc, one in `RUSTC_EVERY`.
const JSON_BYTES: usize = 6_000;
const EVALUATOR_TEXT: usize = 300;
const RUSTC_TEXT: usize = 6_000;
const EVALUATOR_EVERY: u64 = 4;
const RUSTC_EVERY: u64 = 3;
/// Seeded runs under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 400;
const RUSTC_SEEDED: usize = 1_800;

// ---------------------------------------------------------------- the sorts

/// The constructors of the standard library's `Json`, in declaration order,
/// read from its text, so that the encoder names a constructor and never
/// numbers it, and one moved in the module fails here.
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
        assert!(
            block.contains("name: Json,"),
            "the first inductive of Json is the Json sort"
        );
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

// ------------------------------------------------------------- the encoders

fn nat_u64(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

/// A value as the standard library's `Json`: an array is a chain of `JItem`
/// cells and an object a chain of `JMember` cells in ascending order of key.
fn enc_json(json: &Json) -> Value {
    match json {
        Json::Bool(flag) => adt("JBool", vec![Value::Bool { value: *flag }]),
        Json::Int(number) => adt(
            "JInt",
            vec![
                Value::Bool { value: *number < 0 },
                nat_u64(number.unsigned_abs()),
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

/// The run of `parseGeneric` on a text, and the oracle's message when it
/// refuses.
fn parse_run(source: &str) -> (Run, Option<String>) {
    let (value, refusal) = match parse(source) {
        Ok(json) => (
            Value::Ok {
                value: Box::new(enc_json(&json)),
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
            vec![text(source)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

/// The run of `printGeneric` on a value.
fn print_run(json: &Json) -> Run {
    (
        vec![enc_json(json)],
        serde_json::to_value(text(&print(json))).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ vectors

/// What a vector is a run of.
enum Subject {
    Parse(String),
    Print(Json),
}

/// One named vector.
struct Vector {
    label: String,
    subject: Subject,
}

impl Vector {
    fn printing(&self) -> bool {
        matches!(self.subject, Subject::Print(_))
    }

    fn run(&self) -> (Run, Option<String>) {
        match &self.subject {
            Subject::Parse(source) => parse_run(source),
            Subject::Print(json) => (print_run(json), None),
        }
    }
}

fn string(value: &str) -> Json {
    Json::Str(value.to_owned())
}

fn array(items: Vec<Json>) -> Json {
    Json::Arr(items)
}

fn object(pairs: Vec<(&str, Json)>) -> Json {
    Json::object(pairs)
}

fn nested(levels: usize, open: &str, inner: &str, close: &str) -> String {
    format!("{}{inner}{}", open.repeat(levels), close.repeat(levels))
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out: Vec<Vector> = Vec::new();
    let mut add = |label: &str, subject: Subject| {
        out.push(Vector {
            label: label.to_owned(),
            subject,
        });
    };
    let reading = |source: &str| Subject::Parse(source.to_owned());

    // Reading: what is accepted.
    for (label, source) in [
        ("the oracle's own object", "add{right: x, left: 1,}"),
        ("the oracle's own kind", "{kind: \"two words\", a: []}"),
        ("a true", "true"),
        ("a false", "false"),
        ("a zero", "0"),
        ("a minus zero", "-0"),
        ("an integer", "1234567890"),
        ("a negative integer", "-12"),
        ("an integer with leading zeros", "007"),
        ("a negative integer with leading zeros", "-007"),
        ("an integer of only zeros", "0000000000000000000000000"),
        ("the largest integer", "9223372036854775807"),
        ("the smallest integer", "-9223372036854775808"),
        (
            "the largest integer with leading zeros",
            "000009223372036854775807",
        ),
        ("a bare word", "word"),
        ("a bare word with every kind of character", "a.b'c/d-e_f9"),
        ("a bare word with a hyphen", "a-b"),
        ("a bare word with an underscore", "_x"),
        ("a bare word that begins like a keyword", "trueish"),
        ("a bare word of keyword and a digit", "true1"),
        ("a string", "\"two words\""),
        ("an empty string", "\"\""),
        ("a string with escapes", "\"a\\n\\u00e9\\\"\""),
        ("an empty array", "[]"),
        ("an array", "[1, 2, 3]"),
        ("an array with a trailing comma", "[1, 2,]"),
        ("an array of one with a trailing comma", "[x,]"),
        ("an array of arrays", "[[], [[]], [1, [2]]]"),
        (
            "an array of every kind of value",
            "[true, false, 0, -1, word, \"s\", [], {}, K{}]",
        ),
        ("an empty object", "{}"),
        ("an object", "{a: 1, b: 2}"),
        ("an object out of order", "{b: 2, a: 1, c: 3}"),
        ("an object with a trailing comma", "{a: 1,}"),
        (
            "an object with quoted keys",
            "{\"a b\": 1, \"\": 2, \"\\n\": 3}",
        ),
        (
            "an object with bare keys of every kind",
            "{a.b: 1, a'b: 2, a/b: 3, a-b: 4, _a: 5}",
        ),
        ("an object of objects", "{a: {b: {c: {}}}}"),
        ("a tagged object", "add{left: x, right: 1}"),
        ("a tagged empty object", "K{}"),
        ("a tagged object with a trailing comma", "K{a: 1,}"),
        ("a tagged object of tagged objects", "K{a: L{b: M{}}}"),
        ("a tagged object in an array", "[K{a: 1}, L{}]"),
        ("an object whose kind is its member", "{kind: x}"),
        ("a keyword before a brace", "true {}"),
        (
            "white space of every kind between tokens",
            "[ \n1\t,\u{a0}2\u{3000}]\r\n",
        ),
        ("white space around a value", "  \n x \n "),
        (
            "a member that follows its own value",
            "{a: 1, b: [x, y], c: {d: z}}",
        ),
    ] {
        add(label, reading(source));
    }

    // Reading: what is refused.
    for (label, source) in [
        ("the empty text", ""),
        ("white space only", " \n "),
        ("a value at the end of the text", "["),
        ("a value at the end of an array", "[1,"),
        ("a value at the end of an object", "{a:"),
        ("a closing parenthesis for a value", ")"),
        ("an opening parenthesis for a value", "("),
        ("a closing bracket for a value", "]"),
        ("a closing brace for a value", "}"),
        ("a comma for a value", ","),
        ("a colon for a value", ":"),
        ("a colon equals for a value", ":="),
        ("a double colon for a value", "::"),
        ("a semicolon for a value", ";"),
        ("a comma in an array for a value", "[,]"),
        ("a double comma in an array", "[1,,2]"),
        ("a colon in an array for a value", "[:]"),
        ("a name that is no name", "a b"),
        ("a hyphen alone", "-"),
        ("a word of digits and a letter", "1a"),
        ("a decimal number", "1.5"),
        ("an exponent", "1e3"),
        ("a plus sign", "+1"),
        ("an arrow", "->"),
        ("a word beyond ASCII", "\u{e9}"),
        ("a word of a hyphen and a digit", "-1a"),
        ("a word of two hyphens", "--1"),
        ("an integer one past the largest", "9223372036854775808"),
        ("an integer one past the smallest", "-9223372036854775809"),
        ("an integer of twenty digits", "10000000000000000000"),
        (
            "an integer of twenty-five digits",
            "9999999999999999999999999",
        ),
        (
            "a long integer with leading zeros past the largest",
            "0009223372036854775808",
        ),
        (
            "an integer past the largest in an array",
            "[1, 99999999999999999999]",
        ),
        ("a member written twice", "{a: 1, a: 2}"),
        ("a member written twice among others", "{a: 1, b: 2, a: 3}"),
        ("a member written twice after a tag", "K{kind: x}"),
        (
            "a member written twice with quoted keys",
            "{\"a\": 1, a: 2}",
        ),
        (
            "a member written twice after an error in its value",
            "{a: 1, a: )}",
        ),
        ("a member name that is a number", "{1: 2}"),
        ("a member name that is a bracket", "{[: 1}"),
        ("a member name that is no name", "{a b: 1}"),
        ("a member name at the end of the text", "{"),
        ("a member name after a comma", "{a: 1, ,}"),
        ("a member name that is a hyphen", "{-: 1}"),
        ("a member without a colon", "{a}"),
        ("a member with a value in place of the colon", "{a 1}"),
        ("a member with a colon equals", "{a := 1}"),
        ("a member name at the end of the text without a colon", "{a"),
        ("an array not closed", "[1"),
        ("an array closed by a brace", "[1 }"),
        ("an array without a comma", "[1 2]"),
        ("an array of a value and a colon", "[1: 2]"),
        ("an object not closed", "{a: 1"),
        ("an object closed by a bracket", "{a: 1]"),
        ("an object without a comma", "{a: 1 b: 2}"),
        ("two values", "1 2"),
        ("two arrays", "[] []"),
        ("two strings", "\"x\" \"y\""),
        ("a value and a closing bracket", "1 ]"),
        ("a tag and a word", "K x"),
        ("an unterminated string", "\"abc"),
        ("a string with a bad escape", "[\"\\q\"]"),
        ("a lexical error after a parse error", "{a} \"abc"),
        ("a lexical error before a parse error", "\"abc\\q\" )"),
    ] {
        add(label, reading(source));
    }

    // Reading: the limit on nesting.
    add("256 nested arrays", reading(&nested(256, "[", "1", "]")));
    add("257 nested arrays", reading(&nested(257, "[", "1", "]")));
    add(
        "a few more nested arrays",
        reading(&nested(300, "[", "", "")),
    );
    add(
        "256 nested tagged objects",
        reading(&nested(256, "K{a: ", "1", "}")),
    );
    add(
        "257 nested tagged objects",
        reading(&nested(257, "K{a: ", "1", "}")),
    );
    add(
        "257 nested objects",
        reading(&nested(257, "{a: ", "1", "}")),
    );
    add(
        "256 nested arrays of objects",
        reading(&nested(128, "[{a: ", "1", "}]")),
    );
    add(
        "257 nested arrays of objects",
        reading(&format!("[{}", nested(128, "{a: [", "1", "]}"))),
    );

    // Printing.
    let ints = [
        0i64,
        1,
        -1,
        255,
        1_234_567_890,
        i64::MAX,
        i64::MIN,
        i64::MIN + 1,
    ];
    add("a true value", Subject::Print(Json::Bool(true)));
    add("a false value", Subject::Print(Json::Bool(false)));
    for number in ints {
        add(
            &format!("the integer {number}"),
            Subject::Print(Json::Int(number)),
        );
    }
    add("a printed negative integer", Subject::Print(Json::Int(-42)));
    for (label, value) in [
        ("a bare string", "word"),
        ("the string true", "true"),
        ("the string false", "false"),
        ("the empty string", ""),
        ("a string with a space", "two words"),
        ("a string that begins with a digit", "1a"),
        ("a string that begins with a hyphen", "-a"),
        ("a string with a colon", "a:b"),
        ("a string beyond ASCII", "\u{e9}t\u{e9}"),
        ("a string with an underscore", "_x"),
        ("a string with every kind of bare character", "a.b'c/d-e_f9"),
        ("a string with a quote mark", "a\"b"),
        ("a string with a line feed", "a\nb"),
        ("a string with a backslash", "a\\b"),
        ("a string with a control character", "a\u{1}b"),
        ("a string of a keyword and a letter", "trueish"),
    ] {
        add(label, Subject::Print(string(value)));
    }
    add("a printed empty array", Subject::Print(array(Vec::new())));
    add("a printed empty object", Subject::Print(object(Vec::new())));
    add(
        "an array of three",
        Subject::Print(array(vec![Json::Int(1), Json::Int(2), Json::Int(3)])),
    );
    add(
        "an array of one",
        Subject::Print(array(vec![string("only")])),
    );
    add(
        "a printed array of arrays",
        Subject::Print(array(vec![array(Vec::new()), array(vec![Json::Int(1)])])),
    );
    add(
        "a long array that breaks",
        Subject::Print(array(
            (0..30)
                .map(|number| string(&format!("item{number}")))
                .collect(),
        )),
    );
    add(
        "an array of ninety columns",
        Subject::Print(array(
            (0..13)
                .map(|number| string(&format!("elem{number}")))
                .collect(),
        )),
    );
    add(
        "a printed object",
        Subject::Print(object(vec![("a", Json::Int(1)), ("b", Json::Bool(true))])),
    );
    add(
        "an object with every priority key",
        Subject::Print(object(vec![
            ("axioms", array(Vec::new())),
            ("body", string("b")),
            ("constructors", array(Vec::new())),
            ("fields", array(Vec::new())),
            ("name", string("n")),
            ("other", Json::Int(0)),
            ("parameters", array(Vec::new())),
            ("proof", string("p")),
            ("result", string("r")),
            ("statement", string("s")),
            ("type", string("t")),
            ("type_parameters", array(Vec::new())),
            ("arguments", array(Vec::new())),
            ("zeta", Json::Int(1)),
        ])),
    );
    add(
        "an object whose keys need quotes",
        Subject::Print(object(vec![
            ("a b", Json::Int(1)),
            ("", Json::Int(2)),
            ("1", Json::Int(3)),
            ("true", Json::Int(4)),
        ])),
    );
    add(
        "a printed tagged object",
        Subject::Print(object(vec![
            ("kind", string("add")),
            ("left", string("x")),
            ("right", Json::Int(1)),
        ])),
    );
    add(
        "a printed tagged empty object",
        Subject::Print(object(vec![("kind", string("K"))])),
    );
    add(
        "an object with a kind that is no name",
        Subject::Print(object(vec![
            ("kind", string("two words")),
            ("a", Json::Int(1)),
        ])),
    );
    add(
        "an object with a kind that is a keyword",
        Subject::Print(object(vec![("kind", string("true")), ("a", Json::Int(1))])),
    );
    add(
        "an object with a kind that is not a string",
        Subject::Print(object(vec![("kind", Json::Int(7)), ("a", Json::Int(1))])),
    );
    add(
        "tagged objects in a tagged object",
        Subject::Print(object(vec![
            ("kind", string("outer")),
            (
                "inner",
                object(vec![
                    ("kind", string("inner")),
                    ("x", array(vec![Json::Int(1)])),
                ]),
            ),
        ])),
    );
    add(
        "a long object that breaks",
        Subject::Print(object(
            (0..20)
                .map(|number| {
                    (
                        Box::leak(format!("key{number:02}").into_boxed_str()) as &str,
                        string("some value"),
                    )
                })
                .collect(),
        )),
    );
    add(
        "nested arrays that break",
        Subject::Print(array(vec![
            array(
                (0..12)
                    .map(|number| string(&format!("alpha{number}")))
                    .collect(),
            ),
            array(
                (0..12)
                    .map(|number| string(&format!("beta{number}")))
                    .collect(),
            ),
        ])),
    );
    out
}

// ------------------------------------------------------------------- seeded

/// Strings of each kind, bare and not.
const STRINGS: &[&str] = &[
    "",
    "a",
    "word",
    "two words",
    "true",
    "false",
    "kind",
    "name",
    "1a",
    "-x",
    "a.b",
    "_x",
    "\u{e9}",
    "a\nb",
    "a\"b",
    "a:b",
    "K",
    "add",
    "type_parameters",
];
/// Keys of each kind, priority keys and not.
const KEYS: &[&str] = &[
    "name",
    "type_parameters",
    "parameters",
    "type",
    "result",
    "statement",
    "fields",
    "constructors",
    "arguments",
    "body",
    "proof",
    "axioms",
    "kind",
    "a",
    "b",
    "zeta",
    "two words",
    "",
];

/// A value of at most `depth` levels.
fn drawn_json(seeded: &mut Seeded, depth: usize) -> Json {
    let kind = if depth == 0 {
        seeded.below(4)
    } else {
        seeded.below(7)
    };
    match kind {
        0 => Json::Bool(seeded.chance(50)),
        1 => Json::Int(*seeded.pick(&[0i64, 1, -1, 255, 1_234_567, i64::MAX, i64::MIN])),
        2 | 3 => Json::Str((*seeded.pick(STRINGS)).to_owned()),
        4 | 5 => Json::Arr(
            (0..seeded.below(5))
                .map(|_| drawn_json(seeded, depth - 1))
                .collect(),
        ),
        _ => {
            let mut members = BTreeMap::new();
            for _ in 0..seeded.below(5) {
                let key = (*seeded.pick(KEYS)).to_owned();
                members.insert(key, drawn_json(seeded, depth - 1));
            }
            Json::Obj(members)
        }
    }
}

/// A whole token or a part of one, of each shape the notation reads.
const PIECES: &[&str] = &[
    "{",
    "}",
    "[",
    "]",
    ",",
    ":",
    ":=",
    "::",
    ";",
    "(",
    ")",
    " ",
    "\n",
    "true",
    "false",
    "kind",
    "K",
    "word",
    "a-b",
    "0",
    "-0",
    "007",
    "-1",
    "9223372036854775807",
    "9223372036854775808",
    "99999999999999999999",
    "1a",
    "->",
    "\"",
    "\"s\"",
    "\"a b\"",
    "\"\\n\"",
    "\"\\q\"",
    "\u{e9}",
    "\u{a0}",
    "\\",
    "a: 1",
    "a: ",
];

/// One seeded edit of a text at a scalar boundary, up to three of them.
fn edited(seeded: &mut Seeded, source: &str) -> String {
    let mut scalars: Vec<char> = source.chars().collect();
    for _ in 0..=seeded.below(3) {
        let at = seeded.below(scalars.len() + 1);
        let piece: Vec<char> = seeded.pick(PIECES).chars().collect();
        match seeded.below(3) {
            0 if at < scalars.len() => {
                scalars.remove(at);
            }
            1 => {
                drop(scalars.splice(at..at, piece));
            }
            _ if at < scalars.len() => {
                drop(scalars.splice(at..=at, piece));
            }
            _ => scalars.extend(piece),
        }
    }
    scalars.into_iter().collect()
}

/// `count` seeded values, with the runs of `printGeneric` on them, and the
/// runs of `parseGeneric` on their printed texts and on edits of those.
fn seeded_runs(count: usize, bases: &[String]) -> (Vec<Run>, Vec<Run>) {
    let mut seeded = Seeded(0x0B55_0024);
    let mut printing = Vec::with_capacity(count);
    let mut reading = Vec::with_capacity(2 * count);
    while printing.len() < count {
        let depth = 1 + seeded.below(4);
        let json = drawn_json(&mut seeded, depth);
        let printed = print(&json);
        printing.push(print_run(&json));
        reading.push(parse_run(&printed).0);
        let base = if seeded.chance(30) && !bases.is_empty() {
            seeded.pick(bases).clone()
        } else {
            printed
        };
        reading.push(parse_run(&edited(&mut seeded, &base)).0);
    }
    (printing, reading)
}

/// A hash of a path (FNV-1a).
fn path_hash(path: &str) -> u64 {
    path.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// Every committed JSON file of at most `JSON_BYTES` bytes that parses, with
/// its path.
fn documents() -> Vec<(String, Json)> {
    let root = repo_root();
    let walker = walkdir::WalkDir::new(root.as_std_path())
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), "target" | ".git" | ".lexlean")
                    || support::holds_cache_tag(entry)))
        });
    let mut out = Vec::new();
    for entry in walker.flatten() {
        let path = entry.path();
        if !entry.file_type().is_file() || !path.to_string_lossy().ends_with(".json") {
            continue;
        }
        let Ok(bytes) = std::fs::read(path) else {
            continue;
        };
        if bytes.len() > JSON_BYTES {
            continue;
        }
        let Ok(value) = Json::parse(&bytes) else {
            continue;
        };
        let relative = path
            .strip_prefix(root.as_std_path())
            .expect("under the root")
            .to_string_lossy()
            .replace('\\', "/");
        out.push((relative, value));
    }
    out
}

// ------------------------------------------------------------------- plants

/// A defect planted in the source, the root it is in, and the named vector
/// that catches it.
struct Plant {
    label: &'static str,
    root: usize,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "the nesting limit admits one level more",
        root: 0,
        site: "(256 <? (project st depth))",
        replacement: "(257 <? (project st depth))",
        vector: "257 nested arrays",
    },
    Plant {
        label: "a container does not count as a level",
        root: 0,
        site: "depth := ((project st depth) + 1)",
        replacement: "depth := (project st depth)",
        vector: "257 nested arrays",
    },
    Plant {
        label: "a closing bracket first in an array is a value",
        root: 0,
        site: "(if ((codeAt toks (project st pos)) =? 3) then (closeTop st) else",
        replacement: "(if ((codeAt toks (project st pos)) =? 99) then (closeTop st) else",
        vector: "an empty array",
    },
    Plant {
        label: "a closing brace first in an object is a member",
        root: 0,
        site: "(if ((codeAt toks (project st pos)) =? 1) then (closeTop st) else (keyAt",
        replacement: "(if ((codeAt toks (project st pos)) =? 99) then (closeTop st) else (keyAt",
        vector: "an empty object",
    },
    Plant {
        label: "a member written twice is taken",
        root: 0,
        site: "(if (hasKey members key) then",
        replacement: "(if false then",
        vector: "a member written twice",
    },
    Plant {
        label: "a member written twice is found at the next token",
        root: 0,
        site: "(refuse st offset (cat (cat (lit \"the member `\") key)",
        replacement: "(refuse st (project st pos) (cat (cat (lit \"the member `\") key)",
        vector: "a member written twice",
    },
    Plant {
        label: "a tag leaves out its kind",
        root: 0,
        site: "(cons ((lit \"kind\"), (ctor LexLeanStd.Json::Json.JStr word)) (nil (Bytes * LexLeanStd.Json::Json)))",
        replacement: "(nil (Bytes * LexLeanStd.Json::Json))",
        vector: "a tagged object",
    },
    Plant {
        label: "a tag need not be a name",
        root: 0,
        site: "(if (isBare word) then (if ((codeAt toks (pos + 1)) =? 0)",
        replacement: "(if true then (if ((codeAt toks (pos + 1)) =? 0)",
        vector: "a name that is no name",
    },
    Plant {
        label: "the largest integer is one less",
        root: 0,
        site: "(magnitude <=? 9223372036854775807)",
        replacement: "(magnitude <=? 9223372036854775806)",
        vector: "the largest integer",
    },
    Plant {
        label: "the smallest integer is one more",
        root: 0,
        site: "(magnitude <=? 9223372036854775808)",
        replacement: "(magnitude <=? 9223372036854775807)",
        vector: "the smallest integer",
    },
    Plant {
        label: "leading zeros are read as digits",
        root: 0,
        site: "(let first_digit : Nat := (zerosEnd text start);",
        replacement: "(let first_digit : Nat := start;",
        vector: "an integer with leading zeros",
    },
    Plant {
        label: "minus zero is negative",
        root: 0,
        site: "(ctor Option.some [LexLeanStd.Json::Json] (ctor LexLeanStd.Json::Json.JInt false 0))",
        replacement: "(ctor Option.some [LexLeanStd.Json::Json] (ctor LexLeanStd.Json::Json.JInt negative 0))",
        vector: "a minus zero",
    },
    Plant {
        label: "an underscore does not begin a name",
        root: 0,
        site: "(value =? 95)",
        replacement: "(value =? 96)",
        vector: "a bare word with an underscore",
    },
    Plant {
        label: "a hyphen is no part of a name",
        root: 0,
        site: "(value =? 45)",
        replacement: "false",
        vector: "a bare word with a hyphen",
    },
    Plant {
        label: "a colon error is found at the member name",
        root: 0,
        site: "(refuse st (offsetAt toks size (pos + 1)) (lit \"expected `:` after a member name\"))",
        replacement: "(refuse st (offsetAt toks size pos) (lit \"expected `:` after a member name\"))",
        vector: "a member without a colon",
    },
    Plant {
        label: "the end of the text is found at its start",
        root: 0,
        site: "(refuse st size (lit \"expected a value, found the end of the text\"))",
        replacement: "(refuse st 0 (lit \"expected a value, found the end of the text\"))",
        vector: "a value at the end of an array",
    },
    Plant {
        label: "the text after a value is found at its end",
        root: 0,
        site: "(failed (offsetAt toks size (project final pos)) (lit \"unexpected text after the value\"))",
        replacement: "(failed size (lit \"unexpected text after the value\"))",
        vector: "two values",
    },
    Plant {
        label: "an array's comma is no comma in an object",
        root: 0,
        site: "(lit \"expected `,` or `}`\")",
        replacement: "(lit \"expected `,` or `]`\")",
        vector: "an object without a comma",
    },
    Plant {
        label: "the lexical errors come after the parse errors",
        root: 0,
        site: "| Result.error failure => (ctor Result.error [LexLeanStd.Json::Json, SurfaceLex::Failure] failure))",
        replacement: "| Result.error failure => (parseTokens (nil SurfaceLex::Spanned) (prim length text : Nat)))",
        vector: "a lexical error before a parse error",
    },
    Plant {
        label: "a priority key sorts as any other",
        root: 1,
        site: "(LexLeanStd.Order::sortByKey [SurfaceDoc::Doc] entries)",
        replacement: "(nil (Bytes * SurfaceDoc::Doc))",
        vector: "an object with every priority key",
    },
    Plant {
        label: "name is not the first key",
        root: 1,
        site: "(if (prim equal key 0x6e616d65 : Bool) then 0 else",
        replacement: "(if (prim equal key 0x6e616d65 : Bool) then 5 else",
        vector: "an object with every priority key",
    },
    Plant {
        label: "a field's colon has no space",
        root: 1,
        site: "(lit \": \")",
        replacement: "(lit \":\")",
        vector: "a printed object",
    },
    Plant {
        label: "a tag stays a field",
        root: 1,
        site: "(if (prim equal (first pair) 0x6b696e64 : Bool) then kept else",
        replacement: "(if false then kept else",
        vector: "a printed tagged object",
    },
    Plant {
        label: "a kind that is no name is a tag",
        root: 1,
        site: "(if (isBare tag) then (ctor Option.some [Bytes] tag)",
        replacement: "(if true then (ctor Option.some [Bytes] tag)",
        vector: "an object with a kind that is no name",
    },
    Plant {
        label: "an empty array is its closing bracket first",
        root: 1,
        site: "(cat open close)",
        replacement: "(cat close open)",
        vector: "a printed empty array",
    },
    Plant {
        label: "items are separated by a semicolon",
        root: 1,
        site: "(lit \",\")",
        replacement: "(lit \";\")",
        vector: "an array of three",
    },
    Plant {
        label: "a group is indented by four",
        root: 1,
        site: "(ctor SurfaceDoc::Doc.DNest 2",
        replacement: "(ctor SurfaceDoc::Doc.DNest 4",
        vector: "a long array that breaks",
    },
    Plant {
        label: "the printer's width is eighty",
        root: 1,
        site: "(SurfaceDoc::renderDoc node 100",
        replacement: "(SurfaceDoc::renderDoc node 80",
        vector: "an array of ninety columns",
    },
    Plant {
        label: "a negative integer has no sign",
        root: 1,
        site: "(LexLeanStd.Json::integerText negative magnitude)",
        replacement: "(LexLeanStd.Numeral::decimal magnitude)",
        vector: "a printed negative integer",
    },
    Plant {
        label: "any string is a name",
        root: 1,
        site: "(if (isBare text) then text else",
        replacement: "(if true then text else",
        vector: "a string with a space",
    },
    Plant {
        label: "a keyword is a name",
        root: 1,
        site: "(not (isKeyword text))",
        replacement: "true",
        vector: "the string true",
    },
    Plant {
        label: "the items of an array come last first",
        root: 1,
        site: "(LexLeanStd.Order::reverseList [SurfaceDoc::Doc] (docItems items))",
        replacement: "(docItems items)",
        vector: "an array of three",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        assert_eq!(
            vector.printing(),
            plant.root == 1,
            "{}: the vector is a run of the root planted",
            plant.label
        );
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/SurfaceGeneric.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/SurfaceGeneric.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, &[vector.run().0], 1).first() {
            Some(first) => eprintln!(
                "BS-24: plant detected: {}: {}",
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
            "BS-24: {}: largest certificate part {largest} bytes",
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

/// Every error `parse` can state, by a part of its message, so that a corpus
/// that never reached one fails here and not silently (R2).
const REFUSALS: &[&str] = &[
    "expected a value, found the end of the text",
    "expected a value, found `)`",
    "expected a value, found `:=`",
    "expected a member name",
    "expected `:` after a member name",
    "expected `,` or `]`",
    "expected `,` or `}`",
    "nested more deeply than the notation reads",
    "does not fit a 64-bit integer",
    "is not a name; quote it to write a string",
    "is written twice",
    "unexpected text after the value",
    "unterminated string",
    "is not an escape",
];

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("SurfaceGeneric")
            && checked_units.units.contains_key("SurfaceLex")
            && checked_units.units.contains_key("SurfaceDoc"),
        "SurfaceGeneric, SurfaceLex, and SurfaceDoc are linked modules of the project"
    );
    eligibility_of(&project, "SurfaceGeneric", MODULE, &ROOTS);
    assert_eq!(
        json_constructors().as_slice(),
        ["JBool", "JInt", "JStr", "JArr", "JObj", "JNil", "JItem", "JMember"],
        "the constructors of the standard library's Json"
    );

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut messages = Vec::new();
    let mut reading: Vec<Run> = Vec::new();
    let mut printing: Vec<Run> = Vec::new();
    for vector in &vectors {
        let (run, refusal) = vector.run();
        messages.extend(refusal);
        if vector.printing() {
            printing.push(run);
        } else {
            reading.push(run);
        }
    }
    for refusal in REFUSALS {
        assert!(
            messages.iter().any(|message| message.contains(refusal)),
            "no vector reaches the error `{refusal}`"
        );
    }

    // The committed JSON files: printed, and the printed text read back.
    let files = documents();
    let (mut evaluator_printing, mut evaluator_reading) = (Vec::new(), Vec::new());
    let (mut rustc_printing, mut rustc_reading) = (Vec::new(), Vec::new());
    let mut bases: Vec<String> = Vec::new();
    for (path, json) in &files {
        let printed = print(json);
        let hash = path_hash(path);
        if printed.len() <= EVALUATOR_TEXT && hash.is_multiple_of(EVALUATOR_EVERY) {
            evaluator_printing.push(print_run(json));
            evaluator_reading.push(parse_run(&printed).0);
            bases.push(printed.clone());
        }
        if printed.len() <= RUSTC_TEXT && hash.is_multiple_of(RUSTC_EVERY) {
            rustc_printing.push(print_run(json));
            rustc_reading.push(parse_run(&printed).0);
        }
    }
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, &bases),
            seeded_runs(RUSTC_SEEDED, &bases),
        )
    } else {
        ((Vec::new(), Vec::new()), (Vec::new(), Vec::new()))
    };
    eprintln!(
        "BS-24: {} vectors ({} read, {} printed), {} JSON files, {} and {} printed under the evaluator and rustc ({:?})",
        vectors.len(),
        reading.len(),
        printing.len(),
        files.len(),
        evaluator_printing.len(),
        rustc_printing.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        let mut read_corpus: Vec<Run> = reading.clone();
        read_corpus.append(&mut evaluator_reading);
        read_corpus.extend(evaluator_seeded.1);
        let mut print_corpus: Vec<Run> = printing.clone();
        print_corpus.append(&mut evaluator_printing);
        print_corpus.extend(evaluator_seeded.0);
        for (root, corpus) in [
            ("parseGeneric", &read_corpus),
            ("printGeneric", &print_corpus),
        ] {
            let disagreements = super::evaluate(&lowered[root], corpus, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-24: {root}: the evaluator agrees on {} runs ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-24: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled_reading = reading;
    compiled_reading.append(&mut rustc_reading);
    compiled_reading.extend(rustc_seeded.1);
    let mut compiled_printing = printing;
    compiled_printing.append(&mut rustc_printing);
    compiled_printing.extend(rustc_seeded.0);
    let total = rustc_differential(
        "BS-24",
        &[
            (
                "parseGeneric",
                &lowered["parseGeneric"],
                compiled_reading.as_slice(),
            ),
            (
                "printGeneric",
                &lowered["printGeneric"],
                compiled_printing.as_slice(),
            ),
        ],
        "oracle",
    );
    eprintln!("BS-24: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-24") {
        return;
    }
    export(&lowered);
    eprintln!("BS-24: passed in {:?}", started.elapsed());
}
