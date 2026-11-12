//! `BS-27` (SPEC.md §17.23, §17.20, §17.21): the authoring notation of a
//! tooling module as language-1.2 source, `LexLeanBoot.SurfaceTooling`, held to
//! the byte to `lexlean::surface::tooling::{print_module, parse_module,
//! declaration_spans}`.
//!
//! `printToolingModule` writes a module without the `spec` the language implies
//! with the printer of `BS-24`; `parseToolingModule` reads one object with the
//! reader of `BS-24` and adds the `spec`; `declarationSpans` reads the module's
//! members with the value reader of `BS-24` started at a depth and returns the
//! byte range of each element of `declarations`.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::artifact::canonical_json::Json;
use lexlean::calculus::Value;
use lexlean::surface::module::{locate, TOOLING};
use lexlean::surface::tooling::{declaration_spans, parse_module, print_module};
use lexlean::surface::SurfaceError;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, nat_val, pair, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.SurfaceTooling";
const JSON_SOURCE: &str = "stdlib/src/Json.lex.tex";
const SPEC: &str = "lexlean/tooling-module/1";
const ROOTS: [(&str, &[&str]); 3] = [
    (
        "printToolingModule",
        &["allocation", "overflow", "recursion"],
    ),
    ("parseToolingModule", &["allocation", "overflow"]),
    ("declarationSpans", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Committed JSON files read, by size; the values whose printed module is at
/// most the first bound run under the evaluator, one in `EVALUATOR_EVERY` by a
/// hash of the path, and those within the second bound under rustc, one in
/// `RUSTC_EVERY`.
const JSON_BYTES: usize = 6_000;
const EVALUATOR_TEXT: usize = 300;
const RUSTC_TEXT: usize = 6_000;
const EVALUATOR_EVERY: u64 = 4;
const RUSTC_EVERY: u64 = 2;
/// Committed tooling modules larger than this run under neither.
const TOOLING_BYTES: usize = 64_000;
/// Seeded edits under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 200;
const RUSTC_SEEDED: usize = 1_000;

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

fn nat_u64(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

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

fn failure(error: &SurfaceError) -> Value {
    Value::Error {
        value: Box::new(Value::Adt {
            constructor: 0,
            fields: vec![nat_val(error.offset), text(&error.message)],
        }),
    }
}

fn outcome<T>(
    result: Result<T, SurfaceError>,
    encode: impl Fn(&T) -> Value,
) -> (Value, Option<String>) {
    match result {
        Ok(value) => (
            Value::Ok {
                value: Box::new(encode(&value)),
            },
            None,
        ),
        Err(error) => (failure(&error), Some(error.message)),
    }
}

fn print_run(module: &Json) -> (Run, Option<String>) {
    let (value, refusal) = outcome(print_module(module), |printed: &String| text(printed));
    (
        (
            vec![enc_json(module)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

fn parse_run(source: &str, spec: &str) -> (Run, Option<String>) {
    let (value, refusal) = outcome(parse_module(source, spec), enc_json);
    (
        (
            vec![text(source), text(spec)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

fn spans_run(source: &str) -> (Run, Option<String>) {
    let (value, refusal) = outcome(declaration_spans(source), |spans: &Vec<(usize, usize)>| {
        list(
            spans
                .iter()
                .map(|(start, end)| pair(nat_val(*start), nat_val(*end)))
                .collect(),
        )
    });
    (
        (
            vec![text(source)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

// ------------------------------------------------------------------ vectors

enum Subject {
    Print(Json),
    Parse(String),
    Spans(String),
}

struct Vector {
    label: String,
    subject: Subject,
}

impl Vector {
    fn root(&self) -> usize {
        match self.subject {
            Subject::Print(_) => 0,
            Subject::Parse(_) => 1,
            Subject::Spans(_) => 2,
        }
    }

    fn run(&self) -> (Run, Option<String>) {
        match &self.subject {
            Subject::Print(module) => print_run(module),
            Subject::Parse(source) => parse_run(source, SPEC),
            Subject::Spans(source) => spans_run(source),
        }
    }
}

fn json(text: &str) -> Json {
    Json::parse(text.as_bytes()).unwrap_or_else(|error| panic!("`{text}` is JSON: {error:?}"))
}

fn nested(levels: usize, inner: &str) -> String {
    format!("{}{inner}{}", "[".repeat(levels), "]".repeat(levels))
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

    // Printing.
    for (label, text) in [
        ("a minimal module printed", r#"{"spec":"s"}"#),
        (
            "a module with members around spec printed",
            r#"{"a":1,"spec":"lexlean/tooling-module/1","z":[true,"x"]}"#,
        ),
        (
            "the oracle's own module printed",
            r#"{"authority":{"id":"A","lean_githash":"g","lean_version":"4.32.1"},"declarations":[{"constructors":[{"fields":[],"name":"false"}],"kind":"type","name":"Bool","parameters":[]},{"kind":"call","name":"f"}],"imports":[],"spec":"lexlean/tooling-module/1"}"#,
        ),
        ("a module whose spec is a boolean", r#"{"spec":true}"#),
        ("a module whose spec is an object", r#"{"spec":{"a":1}}"#),
        ("a module without a spec", r#"{"a":1}"#),
        ("an empty object is no module", "{}"),
        ("an array is no module", "[]"),
        ("a string is no module", r#""spec""#),
        ("an integer is no module", "-7"),
        ("a boolean is no module", "false"),
        (
            "a long module printed",
            r#"{"declarations":[{"kind":"call","name":"alpha"},{"kind":"call","name":"beta"},{"kind":"call","name":"gamma"},{"kind":"call","name":"delta"}],"spec":"x"}"#,
        ),
    ] {
        add(label, Subject::Print(json(text)));
    }

    // Reading.
    for (label, source) in [
        ("a minimal module read", "{}"),
        ("a module with members around spec read", "{a: 1, z: 2}"),
        (
            "a module of every kind of value read",
            "{a: [true, -1, x, \"s\"], b: K{c: {}}}",
        ),
        ("a module that writes its spec", "{spec: x}"),
        ("a spec written after another member", "{a: 1, spec: x}"),
        ("a quoted spec", "{a: 1, \"spec\": x}"),
        ("a spec in a value is no spec", "{a: spec}"),
        ("an array after white space", "  []"),
        ("an array after a no-break space", "\u{a0}[]"),
        ("an array after a line separator", "\u{2028}\n[1]"),
        ("a word is no module", "word"),
        ("a tagged object is a module", "K{a: 1}"),
        ("the empty text read", ""),
        ("an unterminated string read", "{a: \"x}"),
        ("trailing text read", "{} x"),
        ("a bad value read", "{a: )}"),
    ] {
        add(label, Subject::Parse(source.to_owned()));
    }

    // Spans.
    for (label, source) in [
        ("two declarations", "{declarations: [a, b]}"),
        (
            "declarations with white space",
            "{declarations: [ a,  b{c: 1} ]}",
        ),
        ("a declaration followed by a space", "{declarations: [a ]}"),
        ("declarations with a trailing comma", "{declarations: [a,]}"),
        ("empty declarations", "{declarations: []}"),
        (
            "declarations among other members",
            "{a: [1], declarations: [x], z: {}}",
        ),
        (
            "declarations twice",
            "{declarations: [a], declarations: [b, c]}",
        ),
        ("a quoted declarations key", "{\"declarations\": [a]}"),
        ("a declaration key", "{declaration: [a]}"),
        ("a module with a trailing comma", "{a: 1,}"),
        ("an empty module", "{}"),
        ("a module of a lexical error", "{a: \"x}"),
        ("no brace", "[a]"),
        ("the empty text", ""),
        ("a member without a name", "{: 1}"),
        ("a member at the end of the text", "{a: 1,"),
        ("a member without a colon", "{a 1}"),
        ("declarations that are no array", "{declarations: 1}"),
        ("declarations not closed", "{declarations: [a b]}"),
        ("a member list not closed", "{a: 1 b: 2}"),
        ("a module and trailing text", "{} x"),
        ("a bad declaration", "{declarations: [)]}"),
        ("a bad member value", "{a: )}"),
        (
            "a declaration nested 255 deep",
            &format!("{{declarations: [{}]}}", nested(255, "")),
        ),
        (
            "a declaration nested 256 deep",
            &format!("{{declarations: [{}]}}", nested(256, "")),
        ),
        (
            "a member nested 256 deep",
            &format!("{{a: {}}}", nested(256, "")),
        ),
        (
            "a member nested 257 deep",
            &format!("{{a: {}}}", nested(257, "")),
        ),
    ] {
        add(label, Subject::Spans(source.to_owned()));
    }
    out
}

// ------------------------------------------------------------------- corpora

fn path_hash(path: &str) -> u64 {
    path.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

fn walk() -> impl Iterator<Item = walkdir::DirEntry> {
    walkdir::WalkDir::new(repo_root().as_std_path())
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), "target" | ".git" | ".lexlean")
                    || support::holds_cache_tag(entry)))
        })
        .flatten()
}

fn relative(path: &std::path::Path) -> String {
    path.strip_prefix(repo_root().as_std_path())
        .expect("under the root")
        .to_string_lossy()
        .replace('\\', "/")
}

/// Every committed JSON file of at most `JSON_BYTES` bytes, with its path.
fn documents() -> Vec<(String, Json)> {
    let mut out = Vec::new();
    for entry in walk() {
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
        if let Ok(value) = Json::parse(&bytes) {
            out.push((relative(path), value));
        }
    }
    out
}

/// The notation of every committed tooling module written in it, with its
/// path.
fn tooling_sources() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for entry in walk() {
        let path = entry.path();
        if !entry.file_type().is_file() || !path.to_string_lossy().ends_with(".lex.tex") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let Some(found) = locate(&text) else {
            continue;
        };
        if found.kind.is(&TOOLING) && found.data.is_none() {
            out.push((relative(path), text[found.body.0..found.body.1].to_owned()));
        }
    }
    out
}

const PIECES: &[&str] = &[
    "{",
    "}",
    "[",
    "]",
    ",",
    ":",
    " ",
    "\n",
    "\"",
    "spec",
    "declarations",
    "x",
    "1",
    "K{",
    ")",
];

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

/// `count` seeded edits of the bases, read as a module and for their spans.
fn seeded_runs(count: usize, bases: &[String], seed: u64) -> (Vec<Run>, Vec<Run>) {
    let mut seeded = Seeded(seed);
    let mut reading = Vec::with_capacity(count);
    let mut spanning = Vec::with_capacity(count);
    if bases.is_empty() {
        return (reading, spanning);
    }
    while reading.len() < count {
        let base = seeded.pick(bases).clone();
        let source = edited(&mut seeded, &base);
        reading.push(parse_run(&source, SPEC).0);
        spanning.push(spans_run(&source).0);
    }
    (reading, spanning)
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    root: usize,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "the spec of a module is printed",
        root: 0,
        site: "(if (prim equal key (lit \"spec\") : Bool) then (withoutSpec rest) else",
        replacement: "(if false then (withoutSpec rest) else",
        vector: "a module with members around spec printed",
    },
    Plant {
        label: "a spec that is a boolean is a spec",
        root: 0,
        site: "      | LexLeanStd.Json::Json.JStr text => true\n      | LexLeanStd.Json::Json.JBool flag => false",
        replacement: "      | LexLeanStd.Json::Json.JStr text => true\n      | LexLeanStd.Json::Json.JBool flag => true",
        vector: "a module whose spec is a boolean",
    },
    Plant {
        label: "the final line feed is not written",
        root: 0,
        site: "0x0a : Bytes",
        replacement: "0x : Bytes",
        vector: "a minimal module printed",
    },
    Plant {
        label: "an array is a module without a spec",
        root: 0,
        site: "| LexLeanStd.Json::Json.JArr items => (ctor Result.error [Bytes, SurfaceLex::Failure] (failure 0 (lit \"a module is an object\")))",
        replacement: "| LexLeanStd.Json::Json.JArr items => (ctor Result.error [Bytes, SurfaceLex::Failure] (failure 0 (lit \"a module has a `spec`\")))",
        vector: "an array is no module",
    },
    Plant {
        label: "the spec may be written",
        root: 1,
        site: "(if (SurfaceGeneric::hasKey members (lit \"spec\")) then",
        replacement: "(if false then",
        vector: "a module that writes its spec",
    },
    Plant {
        label: "the written spec is found at the start",
        root: 1,
        site: "    | Option.some found => found\n",
        replacement: "    | Option.some found => 0\n",
        vector: "a spec written after another member",
    },
    Plant {
        label: "the implied spec is empty",
        root: 1,
        site: "(ctor LexLeanStd.Json::Json.JStr spec)) members)",
        replacement: "(ctor LexLeanStd.Json::Json.JStr 0x)) members)",
        vector: "a minimal module read",
    },
    Plant {
        label: "the spec goes first, unsorted",
        root: 1,
        site: "(SurfaceGeneric::memberChain (LexLeanStd.Order::sortByKey [LexLeanStd.Json::Json] (cons",
        replacement: "(SurfaceGeneric::memberChain (LexLeanStd.Order::reverseList [(Bytes * LexLeanStd.Json::Json)] (cons",
        vector: "a module with members around spec read",
    },
    Plant {
        label: "a module that is no object is found at the start",
        root: 1,
        site: "(if (reached <? n) then reached else 0)",
        replacement: "0",
        vector: "an array after white space",
    },
    Plant {
        label: "white space is one byte",
        root: 1,
        site: "(if (0 <? white) then (ctor Option.some [Nat] (position + white))",
        replacement: "(if (0 <? white) then (ctor Option.some [Nat] (position + 1))",
        vector: "an array after a no-break space",
    },
    Plant {
        label: "a span begins where the last token ended",
        root: 2,
        site: "(cons ((SurfaceGeneric::offsetAt toks size start), (lastEnd toks after))",
        replacement: "(cons ((lastEnd toks start), (lastEnd toks after))",
        vector: "declarations with white space",
    },
    Plant {
        label: "a span ends where the next token begins",
        root: 2,
        site: "(lastEnd toks after)) (project st spans))",
        replacement: "(SurfaceGeneric::offsetAt toks size after)) (project st spans))",
        vector: "a declaration followed by a space",
    },
    Plant {
        label: "a trailing comma is followed by a declaration",
        root: 2,
        site: "(if ((SurfaceGeneric::codeAt toks position) =? 3) then (spAt st position 3) else (spRead toks size st 2 true))",
        replacement: "(spRead toks size st 2 true)",
        vector: "declarations with a trailing comma",
    },
    Plant {
        label: "a declaration is read at the depth of a member",
        root: 2,
        site: "(spRead toks size st 2 true)",
        replacement: "(spRead toks size st 1 true)",
        vector: "a declaration nested 256 deep",
    },
    Plant {
        label: "a member is read at the depth of a declaration",
        root: 2,
        site: "(spRead toks size (spAt st (position + 2) 1) 1 false)",
        replacement: "(spRead toks size (spAt st (position + 2) 1) 2 false)",
        vector: "a member nested 256 deep",
    },
    Plant {
        label: "the declarations are another member",
        root: 2,
        site: "(lit \"declarations\")",
        replacement: "(lit \"declaration\")",
        vector: "two declarations",
    },
    Plant {
        label: "text after the module is taken",
        root: 2,
        site: "(if ((position + 1) <? (prim length toks : Nat)) then",
        replacement: "(if false then",
        vector: "a module and trailing text",
    },
    Plant {
        label: "a missing colon is found at the name",
        root: 2,
        site: "(spFail st (SurfaceGeneric::offsetAt toks size (position + 1)) (lit \"expected `:` after a member name\"))",
        replacement: "(spFail st (SurfaceGeneric::offsetAt toks size position) (lit \"expected `:` after a member name\"))",
        vector: "a member without a colon",
    },
    Plant {
        label: "a quoted name is no name",
        root: 2,
        site: "(if (if (code =? 11) then true else (code =? 12)) then",
        replacement: "(if (code =? 12) then",
        vector: "a quoted declarations key",
    },
    Plant {
        label: "a module may begin without a brace",
        root: 2,
        site: "(if ((SurfaceGeneric::codeAt toks position) =? 0) then (spAt st (position + 1) 1)",
        replacement: "(if true then (spAt st (position + 1) 1)",
        vector: "no brace",
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
            vector.root(),
            plant.root,
            "{}: the vector is a run of the root planted",
            plant.label
        );
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/SurfaceTooling.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/SurfaceTooling.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, &[vector.run().0], 1).first() {
            Some(first) => eprintln!(
                "BS-27: plant detected: {}: {}",
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
            "BS-27: {}: largest certificate part {largest} bytes",
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

/// Every error the three functions can state, by a part of its message (R2).
const REFUSALS: &[&str] = &[
    "a module has a `spec`",
    "a module is an object",
    "the `spec` of a module is implied by its language and is not written",
    "unexpected text after the value",
    "expected a value, found",
    "unterminated string",
    "expected `{`",
    "expected a member name",
    "expected `:` after a member name",
    "expected `[` after `declarations`",
    "expected `,` or `]`",
    "expected `,` or `}`",
    "unexpected text after the module",
    "nested more deeply than the notation reads",
];

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("SurfaceTooling")
            && checked_units.units.contains_key("SurfaceGeneric"),
        "SurfaceTooling and SurfaceGeneric are linked modules of the project"
    );
    eligibility_of(&project, "SurfaceTooling", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut messages = Vec::new();
    let mut runs: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for vector in &vectors {
        let (run, refusal) = vector.run();
        messages.extend(refusal);
        runs[vector.root()].push(run);
    }
    for refusal in REFUSALS {
        assert!(
            messages.iter().any(|message| message.contains(refusal)),
            "no vector reaches the error `{refusal}`"
        );
    }

    // The committed JSON files printed as modules, and each printed module read
    // back and spanned.
    let files = documents();
    let mut evaluator: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut compiled: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut bases: Vec<String> = Vec::new();
    for (path, value) in &files {
        let hash = path_hash(path);
        let printed = print_module(value).ok();
        let size = printed.as_ref().map_or(0, String::len);
        let spec = match value {
            Json::Obj(members) => match members.get("spec") {
                Some(Json::Str(spec)) => spec.clone(),
                _ => SPEC.to_owned(),
            },
            _ => SPEC.to_owned(),
        };
        for (sink, bound, every) in [
            (&mut evaluator, EVALUATOR_TEXT, EVALUATOR_EVERY),
            (&mut compiled, RUSTC_TEXT, RUSTC_EVERY),
        ] {
            if size <= bound && hash.is_multiple_of(every) {
                sink[0].push(print_run(value).0);
                if let Some(printed) = &printed {
                    sink[1].push(parse_run(printed, &spec).0);
                    sink[2].push(spans_run(printed).0);
                }
            }
        }
        if let Some(printed) = printed {
            if size <= EVALUATOR_TEXT {
                bases.push(printed);
            }
        }
    }
    // The committed tooling modules, read, reprinted, and spanned.
    let sources = tooling_sources();
    assert!(
        sources.len() >= 4,
        "the committed tooling modules in the notation are {}",
        sources.len()
    );
    let mut tooling = 0usize;
    for (path, body) in &sources {
        if body.len() > TOOLING_BYTES {
            continue;
        }
        let read = parse_module(body, SPEC).unwrap_or_else(|error| panic!("{path}: {error:?}"));
        let spans = declaration_spans(body).unwrap_or_else(|error| panic!("{path}: {error:?}"));
        assert!(!spans.is_empty(), "{path}: a module with declarations");
        compiled[0].push(print_run(&read).0);
        compiled[1].push(parse_run(body, SPEC).0);
        compiled[2].push(spans_run(body).0);
        tooling += 1;
    }
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, &bases, 0x0B55_0027),
            seeded_runs(RUSTC_SEEDED, &bases, 0x0B55_1027),
        )
    } else {
        ((Vec::new(), Vec::new()), (Vec::new(), Vec::new()))
    };
    eprintln!(
        "BS-27: {} vectors ({} printed, {} read, {} spanned), {} JSON files ({} and {} printed under the evaluator and rustc), {} tooling modules, {} and {} seeded edits ({:?})",
        vectors.len(),
        runs[0].len(),
        runs[1].len(),
        runs[2].len(),
        files.len(),
        evaluator[0].len(),
        compiled[0].len(),
        tooling,
        evaluator_seeded.0.len(),
        rustc_seeded.0.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        let [print_seed, read_seed, span_seed] = [
            Vec::new(),
            evaluator_seeded.0.clone(),
            evaluator_seeded.1.clone(),
        ];
        for (index, (root, _)) in ROOTS.iter().enumerate() {
            let mut corpus: Vec<Run> = runs[index].clone();
            corpus.extend(evaluator[index].iter().cloned());
            corpus.extend([&print_seed, &read_seed, &span_seed][index].iter().cloned());
            let disagreements = super::evaluate(&lowered[root], &corpus, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-27: {root}: the evaluator agrees on {} runs ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-27: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled_runs: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for index in 0..3 {
        compiled_runs[index].extend(runs[index].iter().cloned());
        compiled_runs[index].extend(compiled[index].iter().cloned());
    }
    compiled_runs[1].extend(rustc_seeded.0);
    compiled_runs[2].extend(rustc_seeded.1);
    let total = rustc_differential(
        "BS-27",
        &[
            (
                ROOTS[0].0,
                &lowered[ROOTS[0].0],
                compiled_runs[0].as_slice(),
            ),
            (
                ROOTS[1].0,
                &lowered[ROOTS[1].0],
                compiled_runs[1].as_slice(),
            ),
            (
                ROOTS[2].0,
                &lowered[ROOTS[2].0],
                compiled_runs[2].as_slice(),
            ),
        ],
        "oracle",
    );
    eprintln!("BS-27: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-27") {
        return;
    }
    export(&lowered);
    eprintln!("BS-27: passed in {:?}", started.elapsed());
}
