//! `BS-30` (SPEC.md §17.23, §20.6, §23): the human diagnostic output and the
//! argument pre-scans of the command line as language-1.2 source,
//! `LexLeanBoot.CliOutput`, held to the byte to `lexlean::cli::
//! {human_diagnostics_text, wants_json, named_command}`.
//!
//! `humanDiagnostics` writes the diagnostics of a failure as the command line
//! writes them to stderr, with the SGR colors or without; `wantsJson` finds
//! `--diagnostic-format json` in any of its two spellings before the arguments
//! are parsed, so a usage error is reported in the JSON mode; `namedCommand`
//! finds the first argument that names a command, for the `command` field of a
//! usage failure. The diagnostics are the sort of `BS-29`.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::Value;
use lexlean::cli::{human_diagnostics_text, named_command, wants_json};
use lexlean::diagnostic::{Diagnostic, DiagnosticCode, Label, Note, Span};
use lexlean::{ErrorClass, LexLeanError};
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, nat_val, option, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.CliOutput";
const ROOTS: [(&str, &[&str]); 3] = [
    ("humanDiagnostics", &["allocation"]),
    ("wantsJson", &["allocation"]),
    ("namedCommand", &["allocation"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded runs of each root under the evaluator, and under rustc.
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

/// The failure that carries `diagnostics` in the given order.
fn failure(diagnostics: &[Diagnostic]) -> LexLeanError {
    LexLeanError {
        class: ErrorClass::Language,
        diagnostics: diagnostics.to_vec(),
        source: None,
    }
}

fn human_run(diagnostics: &[Diagnostic], color: bool) -> Run {
    (
        vec![
            list(diagnostics.iter().map(enc).collect()),
            Value::Bool { value: color },
        ],
        serde_json::to_value(text(&human_diagnostics_text(&failure(diagnostics), color)))
            .expect("a value is JSON"),
    )
}

fn arguments_value(arguments: &[String]) -> Value {
    list(arguments.iter().map(|argument| text(argument)).collect())
}

fn json_run(arguments: &[String]) -> Run {
    (
        vec![arguments_value(arguments)],
        serde_json::to_value(Value::Bool {
            value: wants_json(arguments),
        })
        .expect("a value is JSON"),
    )
}

fn command_run(arguments: &[String]) -> Run {
    (
        vec![arguments_value(arguments)],
        serde_json::to_value(text(named_command(arguments))).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ corpora

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

/// The committed expected diagnostics of every negative fixture, and the
/// arguments of its command.
fn committed() -> Vec<(Vec<Diagnostic>, Vec<String>)> {
    let root = repo_root().join("tests/negative");
    let mut names: Vec<String> = std::fs::read_dir(root.as_std_path())
        .expect("the negative fixtures")
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    let mut out = Vec::new();
    for name in names {
        let expected = root.join(&name).join("expected");
        let Ok(bytes) = std::fs::read(expected.join("diagnostics.json").as_std_path()) else {
            continue;
        };
        let json: Json2 = serde_json::from_slice(&bytes).expect("diagnostics JSON");
        let diagnostics = json
            .as_array()
            .expect("an array")
            .iter()
            .map(diagnostic_of)
            .collect();
        let command: Json2 = std::fs::read(expected.join("command.json").as_std_path())
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or(Json2::Null);
        let mut arguments = vec!["lexlean".to_owned()];
        if let Some(argv) = command["argv"].as_array() {
            arguments = argv
                .iter()
                .filter_map(|item| item.as_str().map(str::to_owned))
                .collect();
        }
        out.push((diagnostics, arguments));
    }
    out
}

// ------------------------------------------------------------------ vectors

fn at(path: &str, line: usize, column: usize) -> Span {
    Span {
        path: path.to_owned(),
        byte_start: 0,
        byte_end: 1,
        line_start: line,
        column_start: column,
        line_end: line,
        column_end: column + 1,
    }
}

fn d(code_text: &str, message: &str) -> Diagnostic {
    Diagnostic::new(code(code_text), message)
}

fn argv(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

enum Subject {
    Human(Vec<Diagnostic>, bool),
    Json(Vec<String>),
    Command(Vec<String>),
}

struct Vector {
    label: String,
    subject: Subject,
}

impl Vector {
    fn root(&self) -> usize {
        match self.subject {
            Subject::Human(..) => 0,
            Subject::Json(_) => 1,
            Subject::Command(_) => 2,
        }
    }

    fn run(&self) -> Run {
        match &self.subject {
            Subject::Human(items, color) => human_run(items, *color),
            Subject::Json(arguments) => json_run(arguments),
            Subject::Command(arguments) => command_run(arguments),
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
    let everything = || {
        d("LLT4001", "phase link: m")
            .with_span(at("src/Main.lex.tex", 5, 1))
            .with_label("here", at("src/A.lex.tex", 2, 7))
            .with_note("a note")
            .with_located_note("a located note", at("b", 1, 1))
            .with_help("help me")
            .with_cause("because")
    };
    add("no diagnostics", Subject::Human(Vec::new(), false));
    add(
        "an unlocated diagnostic",
        Subject::Human(vec![d("LLC0102", "stale lock")], false),
    );
    add(
        "a located diagnostic",
        Subject::Human(
            vec![d("LLT4001", "m").with_span(at("src/Main.lex.tex", 11, 3))],
            false,
        ),
    );
    add("every part", Subject::Human(vec![everything()], false));
    add(
        "a span over two lines",
        Subject::Human(
            vec![d("LLT4001", "m").with_span(Span {
                path: "f".to_owned(),
                byte_start: 0,
                byte_end: 9,
                line_start: 2,
                column_start: 3,
                line_end: 9,
                column_end: 4,
            })],
            false,
        ),
    );
    add(
        "every part in color",
        Subject::Human(vec![everything()], true),
    );
    add(
        "an unlocated diagnostic in color",
        Subject::Human(vec![d("LLI9001", "internal")], true),
    );
    add(
        "two labels",
        Subject::Human(
            vec![d("LLP2001", "m")
                .with_label("first", at("x", 1, 2))
                .with_label("second", at("y", 3, 4))],
            false,
        ),
    );
    add(
        "two diagnostics in their order",
        Subject::Human(vec![d("LLT4002", "second"), d("LLT4001", "first")], false),
    );
    add(
        "a message with a newline",
        Subject::Human(vec![d("LLT4001", "one\ntwo")], false),
    );
    add(
        "help and causes in order",
        Subject::Human(
            vec![d("LLV7005", "m")
                .with_help("h1")
                .with_help("h2")
                .with_cause("c1")
                .with_cause("c2")],
            false,
        ),
    );
    // The JSON pre-scan.
    for (label, items) in [
        ("no arguments", vec![]),
        ("the program alone", vec!["lexlean"]),
        (
            "the joined spelling",
            vec!["lexlean", "check", "--diagnostic-format=json"],
        ),
        (
            "the separate spelling",
            vec!["lexlean", "--diagnostic-format", "json", "check"],
        ),
        (
            "the human mode",
            vec!["lexlean", "--diagnostic-format", "human"],
        ),
        ("json without the flag", vec!["lexlean", "json"]),
        (
            "the flag at the end",
            vec!["lexlean", "check", "--diagnostic-format"],
        ),
        (
            "the program named like the flag",
            vec!["--diagnostic-format=json"],
        ),
        (
            "the program before json",
            vec!["--diagnostic-format", "json"],
        ),
        (
            "the flag twice",
            vec![
                "lexlean",
                "--diagnostic-format",
                "--diagnostic-format",
                "json",
            ],
        ),
        (
            "a word between",
            vec!["lexlean", "--diagnostic-format", "x", "json"],
        ),
        (
            "another joined value",
            vec!["lexlean", "--diagnostic-format=human"],
        ),
    ] {
        add(label, Subject::Json(argv(&items)));
    }
    // The named command.
    for (label, items) in [
        ("no command", vec!["lexlean", "--version"]),
        ("a command", vec!["lexlean", "check", "--all"]),
        (
            "the first of two commands",
            vec!["lexlean", "build", "check"],
        ),
        (
            "a command after flags",
            vec!["lexlean", "--project", "p", "fmt"],
        ),
        ("the program named like a command", vec!["check", "--all"]),
        ("a word like a command", vec!["lexlean", "checks"]),
        ("every command", vec!["lexlean", "production"]),
        (
            "the last command of the table",
            vec!["lexlean", "explain", "LLT4001"],
        ),
        ("no arguments for a command", vec![]),
    ] {
        add(label, Subject::Command(argv(&items)));
    }
    out
}

// ------------------------------------------------------------------- seeded

const WORDS: &[&str] = &[
    "lexlean",
    "check",
    "build",
    "init",
    "lock",
    "verify",
    "fmt",
    "migrate",
    "clean",
    "explain",
    "production",
    "--diagnostic-format",
    "--diagnostic-format=json",
    "--diagnostic-format=human",
    "json",
    "human",
    "--all",
    "--project",
    "x",
];

fn drawn_arguments(seeded: &mut Seeded) -> Vec<String> {
    let count = seeded.below(6);
    (0..count)
        .map(|_| (*seeded.pick(WORDS)).to_owned())
        .collect()
}

fn seeded_runs(count: usize, sets: &[Vec<Diagnostic>], seed: u64) -> [Vec<Run>; 3] {
    let mut seeded = Seeded(seed);
    let mut out: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for _ in 0..count {
        let mut joined: Vec<Diagnostic> = Vec::new();
        for _ in 0..seeded.below(4) {
            joined.extend(seeded.pick(sets).iter().cloned());
        }
        out[0].push(human_run(&joined, seeded.chance(50)));
        let arguments = drawn_arguments(&mut seeded);
        out[1].push(json_run(&arguments));
        out[2].push(command_run(&arguments));
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
        label: "the bold red is plain red",
        root: 0,
        site: "boldRed := 0x1b5b313b33316d",
        replacement: "boldRed := 0x1b5b33316d",
        vector: "an unlocated diagnostic in color",
    },
    Plant {
        label: "color is always off",
        root: 0,
        site: "(if color then (record Palette",
        replacement: "(if false then (record Palette",
        vector: "every part in color",
    },
    Plant {
        label: "the location gives the end line",
        root: 0,
        site: "(LexLeanStd.Numeral::decimal (project span lineStart))",
        replacement: "(LexLeanStd.Numeral::decimal (project span lineEnd))",
        vector: "a span over two lines",
    },
    Plant {
        label: "the arrow has no indent",
        root: 0,
        site: "(cat 0x0a (lit \"  \"))",
        replacement: "0x0a",
        vector: "a located diagnostic",
    },
    Plant {
        label: "a diagnostic does not end its line",
        root: 0,
        site: "(cat head located) 0x0a)",
        replacement: "(cat head located) 0x)",
        vector: "two diagnostics in their order",
    },
    Plant {
        label: "notes are written last first",
        root: 0,
        site: "(cons (project item message) acc)",
        replacement: "(prim append acc (cons (project item message) (nil Bytes)) : (List Bytes))",
        vector: "every part",
    },
    Plant {
        label: "a cause is written as help",
        root: 0,
        site: "(lit \"  cause: \")",
        replacement: "(lit \"  help: \")",
        vector: "help and causes in order",
    },
    Plant {
        label: "a label has no location",
        root: 0,
        site: "(lit \" (\")) (place (project item span))) (cat (lit \")\") 0x0a))",
        replacement: "(lit \" (\")) 0x) (cat (lit \")\") 0x0a))",
        vector: "two labels",
    },
    Plant {
        label: "the joined spelling is not seen",
        root: 1,
        site: "(if (prim equal argument (lit \"--diagnostic-format=json\") : Bool) then",
        replacement: "(if false then",
        vector: "the joined spelling",
    },
    Plant {
        label: "any value after the flag is json",
        root: 1,
        site: "then (prim equal argument (lit \"json\") : Bool) else false)",
        replacement: "then true else false)",
        vector: "the human mode",
    },
    Plant {
        label: "the program is scanned",
        root: 1,
        site: "(false, (ctor Option.none [Bytes])) (afterFirst arguments)",
        replacement: "(false, (ctor Option.none [Bytes])) arguments",
        vector: "the program named like the flag",
    },
    Plant {
        label: "json counts anywhere",
        root: 1,
        site: "| Option.none => (false, (ctor Option.some [Bytes] argument))))),",
        replacement: "| Option.none => ((prim equal argument (lit \"json\") : Bool), (ctor Option.some [Bytes] argument))))),",
        vector: "the program before json",
    },
    Plant {
        label: "the last command is named",
        root: 2,
        site: "| Option.some known => found\n",
        replacement: "| Option.some known => (if (isCommand argument) then (ctor Option.some [Bytes] argument) else found)\n",
        vector: "the first of two commands",
    },
    Plant {
        label: "the program is a command",
        root: 2,
        site: "(ctor Option.none [Bytes]) (afterFirst arguments) : (Option Bytes))",
        replacement: "(ctor Option.none [Bytes]) arguments : (Option Bytes))",
        vector: "the program named like a command",
    },
    Plant {
        label: "the table ends before explain",
        root: 2,
        site: "(cons (lit \"explain\") (cons (lit \"production\") (nil Bytes)))",
        replacement: "(cons (lit \"production\") (nil Bytes))",
        vector: "the last command of the table",
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
        let source = planted.read_raw("src/CliOutput.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/CliOutput.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, &[vector.run()], 1).first() {
            Some(first) => eprintln!(
                "BS-30: plant detected: {}: {}",
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
            "BS-30: {}: largest certificate part {largest} bytes",
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
        checked_units.units.contains_key("CliOutput")
            && checked_units.units.contains_key("Diagnostics"),
        "CliOutput and Diagnostics are linked modules of the project"
    );
    eligibility_of(&project, "CliOutput", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut runs: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for vector in &vectors {
        runs[vector.root()].push(vector.run());
    }
    let json_outcomes: BTreeSet<bool> = vectors
        .iter()
        .filter_map(|vector| match &vector.subject {
            Subject::Json(arguments) => Some(wants_json(arguments)),
            _ => None,
        })
        .collect();
    assert_eq!(
        json_outcomes.len(),
        2,
        "the vectors reach both outcomes of the scan"
    );

    // The committed diagnostics of each fixture, in both modes, and the
    // arguments of each fixture's command.
    let fixtures = committed();
    let mut corpus: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut sets: Vec<Vec<Diagnostic>> = Vec::new();
    for (diagnostics, arguments) in &fixtures {
        corpus[0].push(human_run(diagnostics, false));
        corpus[0].push(human_run(diagnostics, true));
        corpus[1].push(json_run(arguments));
        corpus[2].push(command_run(arguments));
        sets.push(diagnostics.clone());
    }
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, &sets, 0x0B55_0030),
            seeded_runs(RUSTC_SEEDED, &sets, 0x0B55_1030),
        )
    } else {
        (
            [Vec::new(), Vec::new(), Vec::new()],
            [Vec::new(), Vec::new(), Vec::new()],
        )
    };
    eprintln!(
        "BS-30: {} vectors ({} human, {} scanned, {} named), {} fixtures, {} and {} seeded runs of each root ({:?})",
        vectors.len(),
        runs[0].len(),
        runs[1].len(),
        runs[2].len(),
        fixtures.len(),
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
                "BS-30: {root}: the evaluator agrees on {} runs ({:?})",
                all_runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-30: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: [Vec<Run>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for index in 0..3 {
        compiled[index].extend(runs[index].iter().cloned());
        compiled[index].extend(corpus[index].iter().cloned());
        compiled[index].extend(rustc_seeded[index].iter().cloned());
    }
    let total = rustc_differential(
        "BS-30",
        &[
            (ROOTS[0].0, &lowered[ROOTS[0].0], compiled[0].as_slice()),
            (ROOTS[1].0, &lowered[ROOTS[1].0], compiled[1].as_slice()),
            (ROOTS[2].0, &lowered[ROOTS[2].0], compiled[2].as_slice()),
        ],
        "oracle",
    );
    eprintln!("BS-30: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-30") {
        return;
    }
    export(&lowered);
    eprintln!("BS-30: passed in {:?}", started.elapsed());
}
