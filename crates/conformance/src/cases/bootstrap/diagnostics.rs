//! `BS-29` (SPEC.md §17.23, §20.1, §24.5): the canonical diagnostic as
//! language-1.2 source, `LexLeanBoot.Diagnostics`, held to the byte to
//! `lexlean::diagnostic::{Diagnostic::to_json, sort_canonical}` and to
//! `LexLeanError::from_diagnostics(..).bounded(..)`.
//!
//! `diagnosticText` writes the canonical JSON text of one diagnostic (the
//! object of `to_json`, written as `to_canonical_string` writes it);
//! `sortDiagnostics` orders diagnostics as `sort_canonical` does, by a byte
//! key that preserves the order of the oracle's tuple (path, byte start, code,
//! message); `reportDiagnostics` gives the failure class of a set of
//! diagnostics as its exit code and the diagnostics a failure reports under the
//! configured `max_diagnostics`, with the `LLS8002` diagnostic the cap appends.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::Value;
use lexlean::diagnostic::{sort_canonical, Diagnostic, DiagnosticCode, Label, Note, Span};
use lexlean::LexLeanError;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, nat_val, option, pair, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.Diagnostics";
const ROOTS: [(&str, &[&str]); 3] = [
    ("diagnosticText", &["allocation", "overflow"]),
    ("sortDiagnostics", &["allocation", "overflow"]),
    ("reportDiagnostics", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The caps the committed diagnostics of each fixture are reported under.
const CAPS: [u64; 4] = [0, 1, 2, 256];
/// Seeded diagnostic sets under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 200;
const RUSTC_SEEDED: usize = 1_000;

// ------------------------------------------------------------- the encoders

fn code(text: &str) -> DiagnosticCode {
    let leaked: &'static str = Box::leak(text.to_owned().into_boxed_str());
    DiagnosticCode::from_validated(DiagnosticCode::validate(leaked))
}

fn enc_span(span: &Span) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(&span.path),
            nat_val(span.byte_start),
            nat_val(span.byte_end),
            nat_val(span.line_start),
            nat_val(span.column_start),
            nat_val(span.line_end),
            nat_val(span.column_end),
        ],
    }
}

fn enc(diagnostic: &Diagnostic) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(diagnostic.code.as_str()),
            text(&diagnostic.message),
            option(diagnostic.primary.as_ref().map(enc_span)),
            list(
                diagnostic
                    .labels
                    .iter()
                    .map(|label| Value::Adt {
                        constructor: 0,
                        fields: vec![text(&label.message), enc_span(&label.span)],
                    })
                    .collect(),
            ),
            list(
                diagnostic
                    .notes
                    .iter()
                    .map(|note| Value::Adt {
                        constructor: 0,
                        fields: vec![
                            text(&note.message),
                            option(note.span.as_ref().map(enc_span)),
                        ],
                    })
                    .collect(),
            ),
            list(diagnostic.help.iter().map(|item| text(item)).collect()),
            list(diagnostic.causes.iter().map(|item| text(item)).collect()),
        ],
    }
}

fn encs(diagnostics: &[Diagnostic]) -> Value {
    list(diagnostics.iter().map(enc).collect())
}

fn text_run(diagnostic: &Diagnostic) -> Run {
    (
        vec![enc(diagnostic)],
        serde_json::to_value(text(&diagnostic.to_json().to_canonical_string()))
            .expect("a value is JSON"),
    )
}

fn sort_run(diagnostics: &[Diagnostic]) -> Run {
    let mut sorted = diagnostics.to_vec();
    sort_canonical(&mut sorted);
    (
        vec![encs(diagnostics)],
        serde_json::to_value(encs(&sorted)).expect("a value is JSON"),
    )
}

/// The exit code and the reported diagnostics of a failure, as the oracle
/// makes them.
fn reported(diagnostics: &[Diagnostic], cap: u64, phase: &str) -> (i32, Vec<Diagnostic>) {
    let error = LexLeanError::from_diagnostics(diagnostics.to_vec()).bounded(cap, phase);
    (error.class.exit_code(), error.diagnostics)
}

fn report_run(diagnostics: &[Diagnostic], cap: u64, phase: &str) -> Run {
    let (exit, kept) = reported(diagnostics, cap, phase);
    (
        vec![
            encs(diagnostics),
            Value::Nat {
                value: cap.to_string(),
            },
            text(phase),
        ],
        serde_json::to_value(pair(
            nat_val(usize::try_from(exit).expect("an exit code")),
            encs(&kept),
        ))
        .expect("a value is JSON"),
    )
}

// ------------------------------------------------------- reading the corpus

fn span_of(json: &Json2) -> Span {
    let n = |key: &str| usize::try_from(json[key].as_u64().expect("a count")).expect("usize");
    Span {
        path: json["path"].as_str().expect("a path").to_owned(),
        byte_start: n("byte_start"),
        byte_end: n("byte_end"),
        line_start: n("line_start"),
        column_start: n("column_start"),
        line_end: n("line_end"),
        column_end: n("column_end"),
    }
}

fn strings_of(json: &Json2) -> Vec<String> {
    json.as_array()
        .expect("an array")
        .iter()
        .map(|item| item.as_str().expect("a string").to_owned())
        .collect()
}

fn diagnostic_of(json: &Json2) -> Diagnostic {
    let mut diagnostic = Diagnostic::new(
        code(json["code"].as_str().expect("a code")),
        json["message"].as_str().expect("a message"),
    );
    if let Some(primary) = json.get("primary") {
        diagnostic.primary = Some(span_of(primary));
    }
    for label in json["labels"].as_array().expect("labels") {
        diagnostic.labels.push(Label {
            message: label["message"].as_str().expect("a message").to_owned(),
            span: span_of(&label["span"]),
        });
    }
    for note in json["notes"].as_array().expect("notes") {
        diagnostic.notes.push(Note {
            message: note["message"].as_str().expect("a message").to_owned(),
            span: note.get("span").map(span_of),
        });
    }
    diagnostic.help = strings_of(&json["help"]);
    diagnostic.causes = strings_of(&json["causes"]);
    diagnostic
}

/// The committed expected diagnostics of every negative fixture, each file's
/// set with its path; each is first checked to be what the oracle writes.
fn committed() -> Vec<(String, Vec<Diagnostic>)> {
    let root = repo_root().join("tests/negative");
    let mut names: Vec<String> = std::fs::read_dir(root.as_std_path())
        .expect("the negative fixtures")
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    let mut out = Vec::new();
    for name in names {
        let path = root.join(&name).join("expected/diagnostics.json");
        let Ok(bytes) = std::fs::read(path.as_std_path()) else {
            continue;
        };
        let json: Json2 = serde_json::from_slice(&bytes).expect("diagnostics JSON");
        let diagnostics: Vec<Diagnostic> = json
            .as_array()
            .expect("an array of diagnostics")
            .iter()
            .map(diagnostic_of)
            .collect();
        let written = format!(
            "[{}]",
            diagnostics
                .iter()
                .map(|diagnostic| diagnostic.to_json().to_canonical_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        assert_eq!(
            written.as_bytes(),
            String::from_utf8_lossy(&bytes).trim_end().as_bytes(),
            "{name}: the committed diagnostics are what the oracle writes"
        );
        out.push((name, diagnostics));
    }
    out
}

// ------------------------------------------------------------------ vectors

fn at(path: &str, start: usize) -> Span {
    Span {
        path: path.to_owned(),
        byte_start: start,
        byte_end: start + 3,
        line_start: 2,
        column_start: 5,
        line_end: 4,
        column_end: 7,
    }
}

fn d(code_text: &str, message: &str) -> Diagnostic {
    Diagnostic::new(code(code_text), message)
}

fn located(code_text: &str, message: &str, path: &str, start: usize) -> Diagnostic {
    d(code_text, message).with_span(at(path, start))
}

enum Subject {
    Text(Box<Diagnostic>),
    Sort(Vec<Diagnostic>),
    Report(Vec<Diagnostic>, u64, String),
}

struct Vector {
    label: String,
    subject: Subject,
}

impl Vector {
    fn root(&self) -> usize {
        match self.subject {
            Subject::Text(_) => 0,
            Subject::Sort(_) => 1,
            Subject::Report(..) => 2,
        }
    }

    fn run(&self) -> Run {
        match &self.subject {
            Subject::Text(diagnostic) => text_run(diagnostic),
            Subject::Sort(items) => sort_run(items),
            Subject::Report(items, cap, phase) => report_run(items, *cap, phase),
        }
    }
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
    // The text of one diagnostic.
    add(
        "a minimal diagnostic",
        Subject::Text(Box::new(d("LLT4001", "m"))),
    );
    add(
        "a located diagnostic",
        Subject::Text(Box::new(located(
            "LLT4001",
            "phase link: m",
            "src/Main.lex.tex",
            113,
        ))),
    );
    add(
        "a span of distinct coordinates",
        Subject::Text(Box::new(d("LLL1001", "m").with_span(Span {
            path: "a".to_owned(),
            byte_start: 1,
            byte_end: 2,
            line_start: 3,
            column_start: 4,
            line_end: 5,
            column_end: 6,
        }))),
    );
    add(
        "a note without a span",
        Subject::Text(Box::new(d("LLV7005", "m").with_note("unlocated"))),
    );
    add(
        "a note with a span",
        Subject::Text(Box::new(
            d("LLV7005", "m").with_located_note("located", at("b", 9)),
        )),
    );
    add(
        "two labels",
        Subject::Text(Box::new(
            d("LLP2001", "m")
                .with_label("first", at("x", 1))
                .with_label("second", at("y", 2)),
        )),
    );
    add(
        "help with a quote",
        Subject::Text(Box::new(
            d("LLC0102", "m").with_help("run `lexlean lock` \"now\""),
        )),
    );
    add(
        "causes in order",
        Subject::Text(Box::new(
            d("LLI9001", "m")
                .with_cause("inner")
                .with_cause("innermost"),
        )),
    );
    add(
        "a message with a newline",
        Subject::Text(Box::new(d(
            "LLT4001",
            "line one\nline two\ttab \\ slash \u{1} \u{e9}",
        ))),
    );
    add(
        "every part at once",
        Subject::Text(Box::new(
            located("LLB6004", "m", "p/q.lex.tex", 7)
                .with_label("l", at("r", 1))
                .with_note("n")
                .with_located_note("o", at("s", 2))
                .with_help("h")
                .with_cause("c"),
        )),
    );
    add(
        "a path beyond ASCII",
        Subject::Text(Box::new(located(
            "LLL1001",
            "m",
            "src/\u{e9}t\u{e9}.lex.tex",
            0,
        ))),
    );
    // Sorting.
    add(
        "two files",
        Subject::Sort(vec![
            located("LLT4001", "m", "src/B.lex.tex", 1),
            located("LLT4001", "m", "src/A.lex.tex", 9),
        ]),
    );
    add(
        "a path and its extension",
        Subject::Sort(vec![
            located("LLT4001", "m", "ab", 0),
            located("LLT4001", "m", "a", 18_446_744_073_709_551_000),
        ]),
    );
    add(
        "one file, two offsets",
        Subject::Sort(vec![
            located("LLT4001", "m", "f", 256),
            located("LLT4001", "m", "f", 1),
        ]),
    );
    add(
        "same place, two codes",
        Subject::Sort(vec![
            located("LLT4002", "a", "f", 5),
            located("LLT4001", "z", "f", 5),
        ]),
    );
    add(
        "same place and code, two messages",
        Subject::Sort(vec![
            located("LLT4001", "b", "f", 5),
            located("LLT4001", "a", "f", 5),
        ]),
    );
    add(
        "unlocated first",
        Subject::Sort(vec![located("LLT4001", "m", "a", 0), d("LLC0102", "m")]),
    );
    add(
        "a path with a nul",
        Subject::Sort(vec![
            located("LLT4001", "m", "a\u{0}", 0),
            located("LLT4001", "m", "a", 9),
            located("LLT4001", "m", "a\u{1}", 0),
        ]),
    );
    add(
        "equal keys keep their order",
        Subject::Sort(vec![
            d("LLT4001", "m").with_note("first"),
            d("LLT4001", "m").with_note("second"),
        ]),
    );
    add("no diagnostics to sort", Subject::Sort(Vec::new()));
    // Reporting.
    let phase = || "check".to_owned();
    add("no diagnostics", Subject::Report(Vec::new(), 256, phase()));
    add(
        "a language failure",
        Subject::Report(vec![d("LLT4001", "m")], 256, phase()),
    );
    add(
        "a configuration failure",
        Subject::Report(vec![d("LLC0102", "m")], 256, phase()),
    );
    add(
        "an environment failure",
        Subject::Report(vec![d("LLV7001", "m"), d("LLT4001", "m")], 256, phase()),
    );
    add(
        "an internal and a limit failure",
        Subject::Report(vec![d("LLS8001", "m"), d("LLI9001", "m")], 256, phase()),
    );
    add(
        "three diagnostics capped at two",
        Subject::Report(
            vec![
                located("LLT4001", "c", "f", 3),
                located("LLT4001", "a", "f", 1),
                located("LLT4001", "b", "f", 2),
            ],
            2,
            "link".to_owned(),
        ),
    );
    add(
        "three language failures capped at one",
        Subject::Report(
            vec![d("LLT4001", "a"), d("LLT4002", "b"), d("LLT4003", "c")],
            1,
            phase(),
        ),
    );
    add(
        "capped at zero",
        Subject::Report(vec![d("LLT4001", "a"), d("LLT4002", "b")], 0, phase()),
    );
    add(
        "exactly at the cap",
        Subject::Report(vec![d("LLT4001", "a"), d("LLT4002", "b")], 2, phase()),
    );
    out
}

// ------------------------------------------------------------------- seeded

const PATHS: &[&str] = &[
    "",
    "a",
    "ab",
    "src/Main.lex.tex",
    "src/\u{e9}.lex.tex",
    "a\u{0}b",
];
const MESSAGES: &[&str] = &[
    "",
    "m",
    "phase link: expected `x`",
    "quote \" and slash \\",
    "line\nbreak",
    "control \u{1f} and \u{7f}",
    "\u{1f600} emoji",
];

fn drawn_span(seeded: &mut Seeded) -> Span {
    let start = *seeded.pick(&[0usize, 1, 9, 255, 256, 65_536, 4_294_967_296]);
    Span {
        path: (*seeded.pick(PATHS)).to_owned(),
        byte_start: start,
        byte_end: start + seeded.below(5),
        line_start: 1 + seeded.below(9),
        column_start: 1 + seeded.below(9),
        line_end: 1 + seeded.below(9),
        column_end: 1 + seeded.below(9),
    }
}

fn drawn(seeded: &mut Seeded, codes: &[String]) -> Diagnostic {
    let mut diagnostic = d(seeded.pick(codes), seeded.pick(MESSAGES));
    if seeded.chance(70) {
        diagnostic.primary = Some(drawn_span(seeded));
    }
    for _ in 0..seeded.below(3) {
        let span = drawn_span(seeded);
        diagnostic = diagnostic.with_label(*seeded.pick(MESSAGES), span);
    }
    for _ in 0..seeded.below(3) {
        if seeded.chance(50) {
            let span = drawn_span(seeded);
            diagnostic = diagnostic.with_located_note(*seeded.pick(MESSAGES), span);
        } else {
            diagnostic = diagnostic.with_note(*seeded.pick(MESSAGES));
        }
    }
    for _ in 0..seeded.below(2) {
        diagnostic = diagnostic.with_help(*seeded.pick(MESSAGES));
    }
    for _ in 0..seeded.below(2) {
        diagnostic = diagnostic.with_cause(*seeded.pick(MESSAGES));
    }
    diagnostic
}

/// `count` seeded sets: the text of each diagnostic, the sort of each set,
/// and the report of each set under a drawn cap.
fn seeded_runs(count: usize, codes: &[String], seed: u64) -> [Vec<Run>; 3] {
    let mut seeded = Seeded(seed);
    let mut out: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for _ in 0..count {
        let size = seeded.below(5);
        let set: Vec<Diagnostic> = (0..size).map(|_| drawn(&mut seeded, codes)).collect();
        if let Some(first) = set.first() {
            out[0].push(text_run(first));
        }
        out[1].push(sort_run(&set));
        let cap = u64::try_from(seeded.below(5)).expect("a cap");
        out[2].push(report_run(
            &set,
            cap,
            seeded.pick(&["check", "link", "build"]),
        ));
    }
    out
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
        label: "the severity is a warning",
        root: 0,
        site: "(jsonString (lit \"error\"))",
        replacement: "(jsonString (lit \"warning\"))",
        vector: "a minimal diagnostic",
    },
    Plant {
        label: "the primary span is left out",
        root: 0,
        site: "| Option.some span => (cons (field (lit \"primary\") (spanText span)) tail)",
        replacement: "| Option.some span => tail",
        vector: "a located diagnostic",
    },
    Plant {
        label: "the byte end is the byte start",
        root: 0,
        site: "(field (lit \"byte_end\") (num (project span byteEnd)))",
        replacement: "(field (lit \"byte_end\") (num (project span byteStart)))",
        vector: "a located diagnostic",
    },
    Plant {
        label: "the start column is the start line",
        root: 0,
        site: "(field (lit \"column_start\") (num (project span columnStart)))",
        replacement: "(field (lit \"column_start\") (num (project span lineStart)))",
        vector: "a span of distinct coordinates",
    },
    Plant {
        label: "a note without a span writes a null span",
        root: 0,
        site: "| Option.none => (objectOf (cons (field (lit \"message\") (jsonString (project note message))) (nil Bytes)))",
        replacement: "| Option.none => (objectOf (cons (field (lit \"message\") (jsonString (project note message))) (cons (field (lit \"span\") (lit \"null\")) (nil Bytes))))",
        vector: "a note without a span",
    },
    Plant {
        label: "labels are written last first",
        root: 0,
        site: "(nil Bytes) (LexLeanStd.Order::reverseList [Label] items) : (List Bytes)",
        replacement: "(nil Bytes) items : (List Bytes)",
        vector: "two labels",
    },
    Plant {
        label: "help and causes are not escaped",
        root: 0,
        site: "(cons (jsonString item) acc)",
        replacement: "(cons (cat (cat (lit \"\\\"\") item) (lit \"\\\"\")) acc)",
        vector: "help with a quote",
    },
    Plant {
        label: "the message is not escaped",
        root: 0,
        site: "(field (lit \"message\") (jsonString (project given message)))",
        replacement: "(field (lit \"message\") (cat (cat (lit \"\\\"\") (project given message)) (lit \"\\\"\")))",
        vector: "a message with a newline",
    },
    Plant {
        label: "the path does not order",
        root: 1,
        site: "(cat (cat (cat (orderedPath (first place)) (bigEndian (second place))) (project given code)) (project given message))",
        replacement: "(cat (cat (bigEndian (second place)) (project given code)) (project given message))",
        vector: "two files",
    },
    Plant {
        label: "a shorter path is not ended",
        root: 1,
        site: "0x0000))",
        replacement: "0x))",
        vector: "a path and its extension",
    },
    Plant {
        label: "the byte start is little-endian",
        root: 1,
        site: "(cat (byteText (prim remainder (first (second state)) 256 0 : Nat)) (second (second state)))",
        replacement: "(cat (second (second state)) (byteText (prim remainder (first (second state)) 256 0 : Nat)))",
        vector: "one file, two offsets",
    },
    Plant {
        label: "the code does not order",
        root: 1,
        site: "(bigEndian (second place))) (project given code))",
        replacement: "(bigEndian (second place))) 0x)",
        vector: "same place, two codes",
    },
    Plant {
        label: "the sort is descending",
        root: 1,
        site: "(LexLeanStd.Order::reverseList [(Bytes * Diagnostic)] (LexLeanStd.Order::sortByKey [Diagnostic]",
        replacement: "(LexLeanStd.Order::sortByKey [Diagnostic] (LexLeanStd.Order::sortByKey [Diagnostic]",
        vector: "two files",
    },
    Plant {
        label: "equal keys are reversed",
        root: 1,
        site: "(LexLeanStd.Order::sortByKey [Diagnostic] (LexLeanStd.Order::reverseList [(Bytes * Diagnostic)] (prim list_fold",
        replacement: "(LexLeanStd.Order::sortByKey [Diagnostic] (LexLeanStd.Order::sortByKey [Diagnostic] (prim list_fold",
        vector: "equal keys keep their order",
    },
    Plant {
        label: "a nul in a path is no byte",
        root: 1,
        site: "(cat (second state) 0x0001)",
        replacement: "(second state)",
        vector: "a path with a nul",
    },
    Plant {
        label: "the cap admits one more",
        root: 2,
        site: "(if (observed <=? limit) then",
        replacement: "(if (observed <=? (limit + 1)) then",
        vector: "three diagnostics capped at two",
    },
    Plant {
        label: "an environment code is a language failure",
        root: 2,
        site: "(if (isEnvironmentCode code) then 3 else",
        replacement: "(if false then 3 else",
        vector: "an environment failure",
    },
    Plant {
        label: "an internal code is a language failure",
        root: 2,
        site: "(if (value =? 73) then 70 else 1)",
        replacement: "(if (value =? 73) then 1 else 1)",
        vector: "an internal and a limit failure",
    },
    Plant {
        label: "no diagnostics is a language failure",
        root: 2,
        site: "| List.nil => empty",
        replacement: "| List.nil => 1",
        vector: "no diagnostics",
    },
    Plant {
        label: "the cap reports the observed count as kept",
        root: 2,
        site: "(lit \"; the first \")) (num limit))",
        replacement: "(lit \"; the first \")) (num observed))",
        vector: "three diagnostics capped at two",
    },
    Plant {
        label: "the cap's diagnostic comes first",
        root: 2,
        site: "(LexLeanStd.Order::reverseList [Diagnostic] (cons (exceeded phase limit observed) (LexLeanStd.Order::reverseList [Diagnostic] (firstOf sorted limit))))",
        replacement: "(cons (exceeded phase limit observed) (firstOf sorted limit))",
        vector: "three diagnostics capped at two",
    },
    Plant {
        label: "the class leaves out the cap's diagnostic",
        root: 2,
        site: "((worstExit kept 70), kept)",
        replacement: "((worstExit (firstOf sorted limit) 70), kept)",
        vector: "three language failures capped at one",
    },
    Plant {
        label: "a cap of zero keeps everything",
        root: 2,
        site: "(if ((first state) =? 0) then (ctor Option.none [(Nat * ((List Diagnostic) * (List Diagnostic)))])",
        replacement: "(if false then (ctor Option.none [(Nat * ((List Diagnostic) * (List Diagnostic)))])",
        vector: "capped at zero",
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
        let source = planted.read_raw("src/Diagnostics.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/Diagnostics.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, &[vector.run()], 1).first() {
            Some(first) => eprintln!(
                "BS-29: plant detected: {}: {}",
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
            "BS-29: {}: largest certificate part {largest} bytes",
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
        checked_units.units.contains_key("Diagnostics"),
        "Diagnostics is a linked module of the project"
    );
    eligibility_of(&project, "Diagnostics", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    // Every class of failure and both sides of the cap are reached.
    let mut exits = BTreeSet::new();
    let mut capped = false;
    let mut uncapped = false;
    for vector in &vectors {
        if let Subject::Report(items, cap, phase) = &vector.subject {
            let (exit, kept) = reported(items, *cap, phase);
            exits.insert(exit);
            if kept.iter().any(|item| item.code.as_str() == "LLS8002") {
                capped = true;
            } else {
                uncapped = true;
            }
        }
    }
    assert_eq!(
        exits.into_iter().collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 70],
        "the vectors reach every class of failure"
    );
    assert!(
        capped && uncapped,
        "the vectors reach both sides of the cap"
    );
    let mut runs: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for vector in &vectors {
        runs[vector.root()].push(vector.run());
    }

    // The committed diagnostics: each written, each fixture's set sorted and
    // reported under each cap, and all of them sorted together.
    let fixtures = committed();
    let mut all: Vec<Diagnostic> = Vec::new();
    let mut corpus: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for (_, set) in &fixtures {
        for diagnostic in set {
            corpus[0].push(text_run(diagnostic));
        }
        corpus[1].push(sort_run(set));
        for cap in CAPS {
            corpus[2].push(report_run(set, cap, "check"));
        }
        all.extend(set.iter().cloned());
    }
    corpus[1].push(sort_run(&all));
    corpus[2].push(report_run(&all, 256, "check"));
    let codes: Vec<String> = {
        let mut seen: BTreeSet<String> = all
            .iter()
            .map(|item| item.code.as_str().to_owned())
            .collect();
        for extra in [
            "LLC0102", "LLS8002", "LLI9001", "LLV7001", "LLB6004", "LLF5001", "LLR3004",
        ] {
            seen.insert(extra.to_owned());
        }
        seen.into_iter().collect()
    };
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, &codes, 0x0B55_0029),
            seeded_runs(RUSTC_SEEDED, &codes, 0x0B55_1029),
        )
    } else {
        (
            [Vec::new(), Vec::new(), Vec::new()],
            [Vec::new(), Vec::new(), Vec::new()],
        )
    };
    eprintln!(
        "BS-29: {} vectors ({} written, {} sorted, {} reported), {} committed diagnostics in {} fixtures, {} codes, {} and {} seeded sets ({:?})",
        vectors.len(),
        runs[0].len(),
        runs[1].len(),
        runs[2].len(),
        all.len(),
        fixtures.len(),
        codes.len(),
        EVALUATOR_SEEDED,
        RUSTC_SEEDED,
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        for (index, (root, _)) in ROOTS.iter().enumerate() {
            let mut all_runs: Vec<Run> = runs[index].clone();
            all_runs.extend(corpus[index].iter().cloned());
            all_runs.extend(evaluator_seeded[index].iter().cloned());
            let disagreements = super::evaluate(&lowered[root], &all_runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-29: {root}: the evaluator agrees on {} runs ({:?})",
                all_runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-29: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for index in 0..3 {
        compiled[index].extend(runs[index].iter().cloned());
        compiled[index].extend(corpus[index].iter().cloned());
        compiled[index].extend(rustc_seeded[index].iter().cloned());
    }
    let total = rustc_differential(
        "BS-29",
        &[
            (ROOTS[0].0, &lowered[ROOTS[0].0], compiled[0].as_slice()),
            (ROOTS[1].0, &lowered[ROOTS[1].0], compiled[1].as_slice()),
            (ROOTS[2].0, &lowered[ROOTS[2].0], compiled[2].as_slice()),
        ],
        "oracle",
    );
    eprintln!("BS-29: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-29") {
        return;
    }
    export(&lowered);
    eprintln!("BS-29: passed in {:?}", started.elapsed());
}
