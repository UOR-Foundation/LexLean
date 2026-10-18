//! The bootstrap project (SPEC.md §17.23): the compiler's own components as
//! language-1.2 source, each held to the Rust it replaces. The first is the
//! axiom-output parser (§22.5), whose oracle is `verify::axiom` and whose
//! closed Lean-name check is `ir::core::is_closed_lean_name`.

use std::collections::BTreeMap;
use std::time::Instant;

use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{self as target, interp, Value};
use lexlean::production::lower::{linked_modules, lower_root, roots, Lowered};
use serde_json::Value as Json;

use crate::production_export as harness;
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support::{self, P};

/// The module the first component lives in.
const MODULE: &str = "LexLeanBoot.AxiomOutput";

/// The production roots in the order the module declares them, each with the
/// effects its declaration admits, which are exactly the effects its closure
/// realizes.
const ROOTS: [(&str, &[&str]); 2] = [
    ("isClosedLeanName", &["allocation", "overflow", "recursion"]),
    ("parseAuditOutput", &["allocation", "overflow", "recursion"]),
];

/// Audit outputs per root under the evaluator, after the edge cases and the
/// committed vectors.
const EVALUATOR_AUDITS: usize = 400;
/// Names under the evaluator.
const EVALUATOR_NAMES: usize = 2_000;
/// Audit outputs under rustc.
const RUSTC_AUDITS: usize = 12_000;
/// Names under rustc.
const RUSTC_NAMES: usize = 40_000;
/// The evaluator's fuel; a run that exhausts it fails the case.
const FUEL: u64 = 2_000_000_000;

/// The constructor of `Rejection`, the one constructor of a structure.
const REJECTION: u64 = 0;

/// White space in the sense of `char::is_whitespace`, one of each width.
const WHITE: &[&str] = &[
    " ", "\t", "\u{b}", "\u{c}", "\r", "\u{85}", "\u{a0}", "\u{1680}", "\u{2000}", "\u{2003}",
    "\u{200a}", "\u{2028}", "\u{2029}", "\u{202f}", "\u{205f}", "\u{3000}",
];
/// Characters that look like white space and are none.
const LOOKALIKES: &[&str] = &[
    "\u{200b}", "\u{180e}", "\u{feff}", "\u{2060}", "\u{1f}", "\u{a1}",
];
/// Characters that break a payload at one place or another.
const TRICKY: &[&str] = &[
    "'", "]", "[", ",", ":", ".", " ", "\n", "\r\n", "\u{85}", "\r", "-", "\u{e9}", "info: ", "#",
    "\"", "\\", "\u{0}", "\u{65e5}",
];
/// Axioms that occur in practice.
const AXIOMS: &[&str] = &[
    "propext",
    "Classical.choice",
    "Quot.sound",
    "Lean.ofReduceBool",
    "Lean.trustCompiler",
    "sorryAx",
];
/// Where a located envelope says the message came from.
const PATHS: &[&str] = &[
    "$STAGING/audit/A.lean",
    "A.lean",
    "",
    "a b",
    "..",
    "/x/y/z.lean",
    "C",
];
/// Numbers a located envelope carries: canonical, signed, padded, at the
/// edges of `usize`, and none.
const COUNTS: &[&str] = &[
    "0",
    "3",
    "14",
    "+3",
    "+",
    "",
    " 5 ",
    "\u{a0}7\u{2003}",
    "0003",
    "000000000000000000000000000012",
    "18446744073709551615",
    "18446744073709551616",
    "18446744073709551614",
    "99999999999999999999",
    "100000000000000000000",
    "00018446744073709551615",
    "-1",
    "1.5",
    "1e3",
    "x",
    "\u{663}",
    "++1",
    "1 2",
];

/// A deterministic generator (xorshift64*), so the corpus is the same on
/// every host.
struct Seeded(u64);

impl Seeded {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, bound: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(bound).expect("a bound")).expect("below")
    }
    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }
    fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn bytes(value: &[u8]) -> Value {
    Value::Bytes { hex: hex(value) }
}
fn text(value: &str) -> Value {
    bytes(value.as_bytes())
}
fn list(items: Vec<Value>) -> Value {
    Value::List { items }
}
fn pair(left: Value, right: Value) -> Value {
    Value::Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}
fn option(value: Option<Value>) -> Value {
    match value {
        Some(value) => Value::Some {
            value: Box::new(value),
        },
        None => Value::None,
    }
}
fn texts(items: &[String]) -> Value {
    list(items.iter().map(|item| text(item)).collect())
}

/// One run: the root's arguments and the oracle's observable outcome.
type Run = (Vec<Value>, Json);

/// What the Rust parser `verify::axiom` makes of an audit output, as the
/// value the root returns: the observed axioms per declaration in ascending
/// order of the declaration, or the rejection's message and declaration.
fn audit_value(stdout: &str, expected: &[String]) -> Value {
    match lexlean::verify::axiom::parse_audit_output(stdout, expected) {
        Ok(observed) => Value::Ok {
            value: Box::new(list(
                observed
                    .iter()
                    .map(|(declaration, axioms)| pair(text(declaration), texts(axioms)))
                    .collect(),
            )),
        },
        Err(failure) => {
            assert_eq!(
                failure.diagnostic.code.as_str(),
                "LLV7004",
                "the parser's one diagnostic"
            );
            Value::Error {
                value: Box::new(Value::Adt {
                    constructor: REJECTION,
                    fields: vec![
                        text(&failure.diagnostic.message),
                        option(failure.declaration.as_deref().map(text)),
                    ],
                }),
            }
        }
    }
}

fn audit_run(stdout: &str, expected: &[String]) -> Run {
    (
        vec![text(stdout), texts(expected)],
        serde_json::to_value(audit_value(stdout, expected)).expect("a value is JSON"),
    )
}

/// The name before either exact axiom-payload suffix, as the committed
/// verification case spells it.
fn declared(line: &str) -> String {
    let payload = line.split("info: ").last().unwrap_or(line);
    let body = payload.strip_prefix('\'').unwrap_or(payload);
    body.strip_suffix("' does not depend on any axioms")
        .or_else(|| {
            body.strip_suffix(']')
                .and_then(|body| body.rsplit_once("' depends on axioms: ["))
                .map(|(name, _)| name)
        })
        .unwrap_or("Demo.M.unknown")
        .to_owned()
}

fn golden(file: &str) -> String {
    std::fs::read_to_string(
        support::repo_root()
            .join("tests/golden/axiom-parser")
            .join(file)
            .as_std_path(),
    )
    .unwrap_or_else(|error| panic!("{file}: {error}"))
}

/// The committed vectors as runs, with the number of those the oracle
/// accepts and rejects: every line of `accepted.txt` and `rejected.txt`, each
/// blank-line separated record of `accepted-wrapped.txt`, and each file whole
/// against every declaration its lines name.
fn vectors() -> (Vec<Run>, usize, usize) {
    let mut runs = Vec::new();
    let mut accepted = 0;
    let mut rejected = 0;
    let mut count = |runs: &mut Vec<Run>, stdout: &str, expected: Vec<String>| {
        let run = audit_run(stdout, &expected);
        if run.1["kind"] == "ok" {
            accepted += 1;
        } else {
            rejected += 1;
        }
        runs.push(run);
    };
    for file in ["accepted.txt", "rejected.txt"] {
        let content = golden(file);
        let lines: Vec<&str> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        for line in &lines {
            count(&mut runs, line, vec![declared(line)]);
        }
        count(
            &mut runs,
            &content,
            lines.iter().map(|line| declared(line)).collect(),
        );
    }
    let wrapped = golden("accepted-wrapped.txt");
    let records: Vec<&str> = wrapped
        .split("\n\n")
        .map(str::trim_end)
        .filter(|record| !record.is_empty())
        .collect();
    for record in &records {
        count(&mut runs, record, vec![declared(record)]);
    }
    count(
        &mut runs,
        &wrapped,
        records.iter().map(|record| declared(record)).collect(),
    );
    (runs, accepted, rejected)
}

/// A segment of the closed grammar, drawn from the characters at its edges.
fn segment(seeded: &mut Seeded) -> String {
    const FIRST: &[char] = &['A', 'Z', 'a', 'z', '_', 'm'];
    const LATER: &[char] = &['A', 'Z', 'a', 'z', '0', '9', '_', '?', '\'', 'm'];
    let mut out = String::new();
    out.push(*seeded.pick(FIRST));
    for _ in 0..seeded.below(6) {
        out.push(*seeded.pick(LATER));
    }
    out
}

/// A name: usually in the closed grammar, otherwise one step outside it.
fn name(seeded: &mut Seeded) -> String {
    let segments = 1 + seeded.below(4);
    let mut parts: Vec<String> = (0..segments).map(|_| segment(seeded)).collect();
    if seeded.chance(12) {
        let at = seeded.below(parts.len());
        match seeded.below(9) {
            0 => parts[at] = String::new(),
            1 => parts[at].insert(0, *seeded.pick(&['0', '9', '?', '\'', '-'])),
            2 => parts[at].push_str(seeded.pick(&["-", " ", "\u{e9}", "$", "!", "`", "\u{3b1}"])),
            3 => parts[at].push('.'),
            4 => parts.insert(at, String::new()),
            5 => return String::new(),
            6 => parts[at] = format!("{}\u{a0}", parts[at]),
            7 => parts[at].insert(0, '.'),
            _ => parts[at] = "9".to_owned(),
        }
    }
    parts.join(".")
}

/// An axiom: usually one that occurs, sometimes one invented.
fn axiom(seeded: &mut Seeded) -> String {
    if seeded.chance(70) {
        (*seeded.pick(AXIOMS)).to_owned()
    } else {
        name(seeded)
    }
}

/// White space of the class `char::is_whitespace` accepts, `0..=max` items.
fn white(seeded: &mut Seeded, max: usize) -> String {
    (0..seeded.below(max + 1))
        .map(|_| *seeded.pick(WHITE))
        .collect()
}

/// The payload of one record, one line.
fn payload(seeded: &mut Seeded, declaration: &str) -> String {
    let quote = if seeded.chance(3) { "\"" } else { "'" };
    let mut out = if seeded.chance(35) {
        format!("{quote}{declaration}' does not depend on any axioms")
    } else {
        let mut pieces: Vec<String> = (0..1 + seeded.below(5)).map(|_| axiom(seeded)).collect();
        if seeded.chance(6) {
            let again = pieces[seeded.below(pieces.len())].clone();
            pieces.push(again);
        }
        if seeded.chance(4) {
            let at = seeded.below(pieces.len() + 1);
            pieces.insert(at, String::new());
        }
        if seeded.chance(2) {
            pieces.clear();
        }
        let separator = match seeded.below(10) {
            0 => ",".to_owned(),
            1 => format!("{},{}", white(seeded, 2), white(seeded, 2)),
            _ => ", ".to_owned(),
        };
        let padded: Vec<String> = pieces
            .iter()
            .map(|piece| {
                if seeded.chance(8) {
                    format!("{}{piece}{}", white(seeded, 2), white(seeded, 2))
                } else {
                    piece.clone()
                }
            })
            .collect();
        format!(
            "{quote}{declaration}' depends on axioms: [{}]",
            padded.join(&separator)
        )
    };
    if seeded.chance(4) {
        out.push_str(seeded.pick(&[
            " extra",
            "]",
            "\u{2003}",
            ".",
            " does not depend on any axioms",
        ]));
    }
    if seeded.chance(2) {
        out.pop();
    }
    out
}

/// Break one payload the way the pinned toolchain's format width does: after
/// a comma, the tail on continuation lines of a space of indent, or, on
/// occasion, anywhere and with another indent.
fn wrap(seeded: &mut Seeded, payload: &str, width: usize) -> String {
    if payload.chars().count() <= width {
        return payload.to_owned();
    }
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    let careless = seeded.chance(6);
    let mut pieces: Vec<String> = Vec::new();
    if careless {
        let chars: Vec<char> = payload.chars().collect();
        pieces = chars
            .chunks(width.max(1))
            .map(|chunk| chunk.iter().collect())
            .collect();
    } else {
        let mut piece = String::new();
        for character in payload.chars() {
            piece.push(character);
            if character == ',' {
                pieces.push(std::mem::take(&mut piece));
            }
        }
        if !piece.is_empty() {
            pieces.push(piece);
        }
    }
    for piece in pieces {
        let candidate = if current.is_empty() {
            piece.clone()
        } else {
            format!("{current}{piece}")
        };
        if !current.is_empty() && candidate.chars().count() > width {
            lines.push(std::mem::take(&mut current));
            current = piece.trim_start().to_owned();
        } else {
            current = candidate;
        }
    }
    lines.push(current);
    let indent = if seeded.chance(8) {
        white(seeded, 2) + " "
    } else {
        " ".to_owned()
    };
    let mut out = lines[0].clone();
    for line in &lines[1..] {
        out.push('\n');
        out.push_str(&indent);
        out.push_str(line.trim_start());
    }
    out
}

/// The text before a payload: nothing, a label, or a location, each in its
/// right form and at one step outside it.
fn envelope(seeded: &mut Seeded) -> String {
    match seeded.below(100) {
        0..=54 => String::new(),
        55..=64 => "info: ".to_owned(),
        65..=76 => format!(
            "{}:{}:{}: info: ",
            seeded.pick(PATHS),
            seeded.pick(COUNTS),
            seeded.pick(COUNTS)
        ),
        77..=88 => format!(
            "{}:{}:{}-{}:{}: info: ",
            seeded.pick(PATHS),
            seeded.pick(COUNTS),
            seeded.pick(COUNTS),
            seeded.pick(COUNTS),
            seeded.pick(COUNTS)
        ),
        _ => (*seeded.pick(&[
            "warning: ",
            "error: ",
            "info:",
            "info:  ",
            "INFO: ",
            "information: ",
            "note: ",
            "A.lean:3:0: warning: ",
            "A.lean:3:0 info: ",
            "A.lean:3: info: ",
            "A.lean:3:0-4: info: ",
            "A.lean:3:0-4:5:6: info: ",
            "A.lean:3:0:  info: ",
            "A.lean:3:0:info: ",
            "A.lean:3:0: info:",
            ":3:0: info: ",
            "a:b:c:d: info: ",
        ]))
        .to_owned(),
    }
}

/// One record: an envelope, a payload, wrapped at `width`, and indented.
fn record(seeded: &mut Seeded, declaration: &str, width: usize) -> String {
    let body = payload(seeded, declaration);
    let wrapped = wrap(seeded, &body, width);
    format!("{}{}{}", white(seeded, 1), envelope(seeded), wrapped)
}

/// Declarations in the sorted order of their commands (§18.9): a few
/// distinct names, now and then none.
fn declarations(seeded: &mut Seeded) -> Vec<String> {
    let mut names: Vec<String> = (0..seeded.below(6)).map(|_| name(seeded)).collect();
    names.sort();
    names.dedup();
    names
}

/// A line that is not a record.
fn noise(seeded: &mut Seeded) -> String {
    (*seeded.pick(&[
        "some unrelated informational text",
        "warning: 'Demo.M.x' does not depend on any axioms",
        "error(lean.unknownIdentifier): Unknown constant `Demo.M.missing`",
        "'",
        "]",
        "info: ",
        "",
        "   ",
        "\u{3000}",
        "\u{200b}",
        " continuation without a record",
    ]))
    .to_owned()
}

/// One audit output and its declarations: the records, separated and ended
/// in one of the ways a stream can be, then mutated about half the time.
#[allow(clippy::too_many_lines)]
fn audit(seeded: &mut Seeded) -> (String, Vec<String>) {
    let mut expected = declarations(seeded);
    let width = match seeded.below(4) {
        0 => 1 + seeded.below(30),
        1 => 60 + seeded.below(80),
        2 => 120,
        _ => 10_000,
    };
    let mut records: Vec<String> = expected
        .iter()
        .map(|declaration| record(seeded, declaration, width))
        .collect();
    if seeded.chance(60) && !records.is_empty() {
        match seeded.below(12) {
            0 => {
                records.remove(seeded.below(records.len()));
            }
            1 => {
                let at = seeded.below(records.len());
                records.insert(at, records[at].clone());
            }
            2 => {
                let extra = name(seeded);
                let line = record(seeded, &extra, width);
                let at = seeded.below(records.len() + 1);
                records.insert(at, line);
            }
            3 if records.len() > 1 => {
                let (a, b) = (seeded.below(records.len()), seeded.below(records.len()));
                records.swap(a, b);
            }
            4 => {
                let at = seeded.below(records.len() + 1);
                records.insert(at, noise(seeded));
            }
            5 => {
                let at = seeded.below(records.len());
                let long = format!(
                    "{}.{}",
                    "a".repeat(2_000 + seeded.below(500)),
                    segment(seeded)
                );
                records[at] = record(seeded, &long, width);
            }
            6 => {
                let at = seeded.below(records.len());
                let many: Vec<String> = (0..200 + seeded.below(100))
                    .map(|index| format!("Ax.n{index}"))
                    .collect();
                records[at] = wrap(
                    seeded,
                    &format!("'D' depends on axioms: [{}]", many.join(", ")),
                    width,
                );
            }
            7 => {
                let at = seeded.below(records.len());
                let cut: Vec<char> = records[at].chars().collect();
                records[at] = cut[..seeded.below(cut.len() + 1)].iter().collect();
            }
            8 => {
                let at = seeded.below(records.len());
                let mut chars: Vec<char> = records[at].chars().collect();
                if !chars.is_empty() {
                    let place = seeded.below(chars.len());
                    chars.splice(place..=place, seeded.pick(TRICKY).chars());
                }
                records[at] = chars.into_iter().collect();
            }
            9 => {
                let at = seeded.below(records.len());
                let mut chars: Vec<char> = records[at].chars().collect();
                chars.splice(
                    seeded.below(chars.len() + 1)..,
                    seeded.pick(LOOKALIKES).chars(),
                );
                records[at] = chars.into_iter().collect();
            }
            _ => {
                let at = seeded.below(records.len());
                let mut chars: Vec<char> = records[at].chars().collect();
                if !chars.is_empty() {
                    chars.remove(seeded.below(chars.len()));
                }
                records[at] = chars.into_iter().collect();
            }
        }
    }
    let mut stdout = String::new();
    for (index, line) in records.iter().enumerate() {
        stdout.push_str(line);
        if index + 1 < records.len() || seeded.chance(80) {
            stdout.push_str(if seeded.chance(10) { "\r\n" } else { "\n" });
        }
        if seeded.chance(8) {
            stdout.push_str(&white(seeded, 2).replace('\n', ""));
            stdout.push('\n');
        }
        if seeded.chance(3) {
            stdout.push('\n');
        }
    }
    if seeded.chance(8) {
        stdout.push_str(&noise(seeded));
        stdout.push('\n');
    }
    if seeded.chance(25) && !expected.is_empty() {
        match seeded.below(5) {
            0 => {
                expected.remove(seeded.below(expected.len()));
            }
            1 => {
                let again = expected[seeded.below(expected.len())].clone();
                expected.push(again);
            }
            2 => expected.push(name(seeded)),
            3 if expected.len() > 1 => {
                let (a, b) = (seeded.below(expected.len()), seeded.below(expected.len()));
                expected.swap(a, b);
            }
            _ => expected.reverse(),
        }
    }
    (stdout, expected)
}

/// Fixed audit outputs at the boundaries the grammar has: white space of every
/// width around a record, the edges of `usize` in a location, the order of
/// axioms, and a wrapped list that never closes.
fn audit_edges() -> Vec<(String, Vec<String>)> {
    let one = |stdout: &str, name: &str| (stdout.to_owned(), vec![name.to_owned()]);
    let mut out = vec![
        one("", "A.b"),
        (String::new(), Vec::new()),
        ("\n\n  \n".to_owned(), Vec::new()),
        one("'A.b' does not depend on any axioms", "A.b"),
        one("'A.b' does not depend on any axioms\n", "A.b"),
        one("'A.b' does not depend on any axioms\r\n", "A.b"),
        one("\u{85}'A.b' does not depend on any axioms\u{a0}\n", "A.b"),
        one(
            "\u{3000}\u{2003}'A.b' does not depend on any axioms\u{2028}\n",
            "A.b",
        ),
        one("\u{200b}'A.b' does not depend on any axioms\n", "A.b"),
        one("\u{180e}'A.b' does not depend on any axioms\n", "A.b"),
        one("\u{b}'A.b' does not depend on any axioms\u{c}\n", "A.b"),
        one("'A.b' depends on axioms: [propext,\n Quot.sound]", "A.b"),
        one(
            "'A.b' depends on axioms: [propext,\n\u{a0}Quot.sound]",
            "A.b",
        ),
        one(
            "'A.b' depends on axioms: [propext,\n\u{200b}Quot.sound]",
            "A.b",
        ),
        one("'A.b' depends on axioms: [propext,\n \n Quot.sound]", "A.b"),
        one("'A.b' depends on axioms: [propext,\n\n Quot.sound]", "A.b"),
        one("'A.b' depends on axioms: [propext,\nQuot.sound]", "A.b"),
        one("'A.b' depends on axioms: [propext,", "A.b"),
        one("'A.b' depends on axioms: [propext,\n", "A.b"),
        one("'A.b' depends on axioms: [propext,\n   ", "A.b"),
        one(
            "'A.b' depends on axioms: [propext,\n  'C.d' does not depend on any axioms",
            "A.b",
        ),
        one(
            "'A.b' depends on axioms: [Quot.sound, propext, Classical.choice]",
            "A.b",
        ),
        one("'A.b' depends on axioms: [propext,Quot.sound]", "A.b"),
        one("'A.b' depends on axioms: [ propext , Quot.sound ]", "A.b"),
        one("'A.b' depends on axioms: [propext, propext]", "A.b"),
        one("'A.b' depends on axioms: []", "A.b"),
        one("'A.b' depends on axioms: [,]", "A.b"),
        one("'A.b' depends on axioms: [propext,]", "A.b"),
        one("'A.b' depends on axioms: [propext", "A.b"),
        one("'A.b' depends on axioms: propext]", "A.b"),
        one("'A.b' depends on axioms [propext]", "A.b"),
        one(
            "'A.b' depends on axioms: [a.b' depends on axioms: [propext]",
            "A.b",
        ),
        one(
            "'A.b' depends on axioms: ['x' depends on axioms: [propext]]",
            "A.b",
        ),
        one(
            "'A' depends on axioms: [Q]' depends on axioms: [propext]",
            "A' depends on axioms: [Q]",
        ),
        one("'A.b'' does not depend on any axioms", "A.b'"),
        one("'A.b' does not depend on any axioms extra", "A.b"),
        one("'' does not depend on any axioms", ""),
        one("' ' does not depend on any axioms", " "),
        one("'A..b' does not depend on any axioms", "A..b"),
        one("'.A' does not depend on any axioms", ".A"),
        one("'A.' does not depend on any axioms", "A."),
        one("'9a' does not depend on any axioms", "9a"),
        one("'a9?'' does not depend on any axioms", "a9?'"),
        one("'a-b' does not depend on any axioms", "a-b"),
        one("'\u{e9}' does not depend on any axioms", "\u{e9}"),
        one("'A.b' does not depend on any axioms", "A.c"),
        one("'A.b' does not depend on any axioms", ""),
        ("'A.b' does not depend on any axioms".to_owned(), Vec::new()),
        (
            "'A.b' does not depend on any axioms\n'A.b' does not depend on any axioms\n".to_owned(),
            vec!["A.b".to_owned(), "A.b".to_owned()],
        ),
        (
            "'A.b' does not depend on any axioms\n'A.b' does not depend on any axioms\n".to_owned(),
            vec!["A.b".to_owned()],
        ),
        (
            "'B.b' does not depend on any axioms\n'A.b' does not depend on any axioms\n".to_owned(),
            vec!["A.b".to_owned(), "B.b".to_owned()],
        ),
        (
            "'B.b' does not depend on any axioms\n'A.b' does not depend on any axioms\n".to_owned(),
            vec!["B.b".to_owned(), "A.b".to_owned()],
        ),
        (
            "'a' does not depend on any axioms\n'B' does not depend on any axioms\n".to_owned(),
            vec!["B".to_owned(), "a".to_owned()],
        ),
        (
            "'A.b' does not depend on any axioms\n'A.b' does not depend on any axioms\n".to_owned(),
            vec!["A.b".to_owned(), "A.b".to_owned(), "A.b".to_owned()],
        ),
        (
            "some text\n'A.b' does not depend on any axioms".to_owned(),
            vec!["A.b".to_owned()],
        ),
        (
            "'A.b' does not depend on any axioms\nsome text".to_owned(),
            vec!["A.b".to_owned()],
        ),
    ];
    for number in COUNTS {
        for label in ["info: ", "warning: "] {
            out.push(one(
                &format!("A.lean:{number}:0: {label}'A.b' does not depend on any axioms"),
                "A.b",
            ));
            out.push(one(
                &format!("A.lean:1:{number}-{number}:{number}: {label}'A.b' does not depend on any axioms"),
                "A.b",
            ));
            out.push(one(
                &format!("A.lean:{number}:{number}: {label}'A.b' does not depend on any axioms"),
                "A.b",
            ));
        }
    }
    out
}

/// The deterministic audit-output vectors: the committed golden vectors and the
/// boundary edges.
fn audit_vectors() -> Vec<Run> {
    let (mut out, _, _) = vectors();
    for (stdout, expected) in audit_edges() {
        out.push(audit_run(&stdout, &expected));
    }
    out
}

/// The corpus of audit outputs: the committed vectors, the edges, then seeded
/// random outputs up to `runs`.
fn audit_corpus(runs: usize) -> Vec<Run> {
    let mut seeded = Seeded(0x9e37_79b9_7f4a_7c15 ^ 0x0061_7564_6974);
    let mut out = audit_vectors();
    while out.len() < runs {
        let (stdout, expected) = audit(&mut seeded);
        out.push(audit_run(&stdout, &expected));
    }
    out
}

/// The deterministic closed-Lean-name vectors: the grammar's edges and every
/// single-byte UTF-8 character in first and in later position.
fn name_vectors() -> Vec<Run> {
    let mut out = Vec::new();
    let one = |out: &mut Vec<Run>, raw: &str| {
        out.push((
            vec![text(raw)],
            serde_json::to_value(Value::Bool {
                value: lexlean::ir::core::is_closed_lean_name(raw),
            })
            .expect("a value is JSON"),
        ));
    };
    for raw in [
        "",
        ".",
        "a",
        "a.",
        ".a",
        "a..b",
        "a.b",
        "_",
        "_.",
        "_a",
        "a_",
        "a?",
        "a'",
        "a?'",
        "?a",
        "'a",
        "9",
        "a9",
        "9a",
        "a.9",
        "a.9b",
        "A.b.C.d",
        "a b",
        "a\u{a0}",
        "\u{e9}",
        "a\u{e9}",
        "a.\u{e9}",
        "a-b",
        "a$b",
        "a\n",
        "\na",
        "a\0",
        "Z",
        "z",
        "@",
        "[",
        "`",
        "{",
        "/",
        ":",
        "a.b.c.d.e.f.g.h",
        "\u{3b1}",
    ] {
        one(&mut out, raw);
    }
    for byte in 0..=255_u8 {
        if let Ok(character) = std::str::from_utf8(&[byte]) {
            one(&mut out, character);
            one(&mut out, &format!("a{character}"));
            one(&mut out, &format!("a.{character}"));
            one(&mut out, &format!("{character}a"));
        }
    }
    out
}

/// The corpus of names: the grammar's edges, every byte in first and in later
/// position, then seeded random names.
fn name_corpus(runs: usize) -> Vec<Run> {
    let mut seeded = Seeded(0x0123_4567_89ab_cdef);
    let mut out = name_vectors();
    while out.len() < runs {
        let raw = name(&mut seeded);
        out.push((
            vec![text(&raw)],
            serde_json::to_value(Value::Bool {
                value: lexlean::ir::core::is_closed_lean_name(&raw),
            })
            .expect("a value is JSON"),
        ));
    }
    out
}

/// The lowered program of one root of the checked project.
fn lower(
    checked: &lexlean::link::CheckedProject,
    project: &P,
    module: &str,
    root: &str,
) -> Lowered {
    let linked = linked_modules(checked);
    let roots = roots(checked).expect("the production roots");
    let root = format!("{module}.{root}");
    let found = roots
        .iter()
        .find(|candidate| candidate.report.root == root)
        .unwrap_or_else(|| panic!("`{root}` is a production root"));
    lower_root(
        &linked,
        &found.module,
        &found.name,
        found.report,
        &support::limits(project),
    )
    .expect("the root lowers")
}

/// A value of the calculus as a person reads it: bytes as text, a pair or a
/// list as itself, a constructor by its fields.
fn readable(value: &Json) -> String {
    match value {
        Json::Object(map) => match map.get("kind").and_then(Json::as_str) {
            Some("bytes") => {
                let hex = map["hex"].as_str().expect("hex");
                let raw: Vec<u8> = (0..hex.len())
                    .step_by(2)
                    .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).expect("hex digit"))
                    .collect();
                format!("{:?}", String::from_utf8_lossy(&raw))
            }
            Some("pair") => format!("({}, {})", readable(&map["left"]), readable(&map["right"])),
            Some("list") => {
                let items: Vec<String> = map["items"]
                    .as_array()
                    .expect("items")
                    .iter()
                    .map(readable)
                    .collect();
                format!("[{}]", items.join(", "))
            }
            Some("adt") => {
                let constructor = map.get("constructor").and_then(Json::as_u64).unwrap_or(0);
                let fields: Vec<String> = map["fields"]
                    .as_array()
                    .expect("fields")
                    .iter()
                    .map(readable)
                    .collect();
                format!("#{constructor}{{{}}}", fields.join(", "))
            }
            Some("none") => "none".to_owned(),
            Some("unit") => "()".to_owned(),
            Some(kind @ ("ok" | "error" | "some")) => {
                format!("{kind} {}", readable(&map["value"]))
            }
            Some("bool") => map["value"].to_string(),
            _ => value.to_string(),
        },
        other => other.to_string(),
    }
}

/// Every run of `corpus` on which the evaluator and the oracle disagree, up
/// to `limit`, each as the input and the two outcomes.
fn evaluate(lowered: &Lowered, corpus: &[Run], limit: usize) -> Vec<String> {
    let mut out = Vec::new();
    for (run, (arguments, expected)) in corpus.iter().enumerate() {
        let outcome = interp::run(&lowered.program, FUEL, lowered.entry(), arguments);
        let observed = rust::observable(&outcome)
            .unwrap_or_else(|| panic!("run {run} on {arguments:?}: {outcome:?}"));
        if &observed != expected {
            let input: Vec<String> = arguments
                .iter()
                .map(|argument| readable(&serde_json::to_value(argument).expect("JSON")))
                .collect();
            out.push(format!(
                "run {run} on {}: the program gives {}, the oracle {}",
                input.join(" "),
                readable(&observed),
                readable(expected)
            ));
            if out.len() >= limit {
                break;
            }
        }
    }
    out
}

fn snake(root: &str) -> String {
    let mut out = String::new();
    for character in root.chars() {
        if character.is_ascii_uppercase() {
            out.push('_');
            out.push(character.to_ascii_lowercase());
        } else {
            out.push(character);
        }
    }
    out
}

/// A defect planted in the source, the root whose differential must catch it,
/// and the corpus it must be caught on.
struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    root: &'static str,
}

const PLANTS: [Plant; 7] = [
    Plant {
        label: "the continuation joins without the space the unwrapped form has",
        site: "(extend records remaining (cat (cat payload (text \" \")) more) about)",
        replacement: "(extend records remaining (cat payload more) about)",
        root: "parseAuditOutput",
    },
    Plant {
        label: "the next-line control (U+0085) is not white space",
        site: "(byteIs subject (position + 1) (133 : UInt8))",
        replacement: "(byteIs subject (position + 1) (134 : UInt8))",
        root: "parseAuditOutput",
    },
    Plant {
        label: "a location number may be one past the largest `usize`",
        site: "18446744073709551615",
        replacement: "18446744073709551616",
        root: "parseAuditOutput",
    },
    Plant {
        label: "the axioms are ordered descending",
        site: "(prim equal (prim compare_bytes left right : Ordering) (prim compare_bytes 0x 0x01 : Ordering) : Bool)",
        replacement: "(prim equal (prim compare_bytes right left : Ordering) (prim compare_bytes 0x 0x01 : Ordering) : Bool)",
        root: "parseAuditOutput",
    },
    Plant {
        label: "a rejected line is not attributed to the declaration expected there",
        site: "(stop (cat (text \"unrecognized audit line: \") (trimmed line)) (headOf remaining))",
        replacement: "(stop (cat (text \"unrecognized audit line: \") (trimmed line)) (ctor Option.none [Bytes]))",
        root: "parseAuditOutput",
    },
    Plant {
        label: "a name segment may begin with a digit",
        site: "(if (inRange byte (65 : UInt8) (90 : UInt8)) then true else (if (inRange byte (97 : UInt8) (122 : UInt8)) then true else (prim equal byte (95 : UInt8) : Bool)))",
        replacement: "(if (inRange byte (48 : UInt8) (57 : UInt8)) then true else (if (inRange byte (65 : UInt8) (90 : UInt8)) then true else (if (inRange byte (97 : UInt8) (122 : UInt8)) then true else (prim equal byte (95 : UInt8) : Bool))))",
        root: "isClosedLeanName",
    },
    Plant {
        label: "the duplicate record check is dropped",
        site: "(refuse (cat (text \"duplicate record for \") (quoted (project entry declaration))) (ctor Option.some [Bytes] want))",
        replacement: "(ctor Result.ok [(List (Bytes * (List Bytes))), Rejection] entries)",
        root: "parseAuditOutput",
    },
];

const TOKEN_MODULE: &str = "LexLeanBoot.TokenAudit";

const TOKEN_ROOTS: [(&str, &[&str]); 2] = [
    ("lex", &["allocation", "overflow", "recursion"]),
    ("auditTokens", &["allocation", "overflow", "recursion"]),
];

const EVALUATOR_TOKEN_TEXTS: usize = 300;
const RUSTC_TOKEN_TEXTS: usize = 4_000;

fn nat_val(n: usize) -> Value {
    Value::Nat {
        value: n.to_string(),
    }
}

fn bool_val(b: bool) -> Value {
    Value::Bool { value: b }
}

fn lex_value(input: &str) -> Value {
    match lexlean::production::preserve::lex(input) {
        Ok(lexemes) => Value::Ok {
            value: Box::new(list(
                lexemes
                    .iter()
                    .map(|lexeme| {
                        let kind = match &lexeme.kind {
                            lexlean::production::preserve::LexemeKind::Ident(segments) => {
                                Value::Adt {
                                    constructor: 0,
                                    fields: vec![list(
                                        segments
                                            .iter()
                                            .map(|seg| Value::Adt {
                                                constructor: 0,
                                                fields: vec![text(&seg.text), bool_val(seg.quoted)],
                                            })
                                            .collect(),
                                    )],
                                }
                            }
                            lexlean::production::preserve::LexemeKind::Number => Value::Adt {
                                constructor: 1,
                                fields: vec![],
                            },
                            lexlean::production::preserve::LexemeKind::Literal => Value::Adt {
                                constructor: 2,
                                fields: vec![],
                            },
                            lexlean::production::preserve::LexemeKind::Command(cmd) => Value::Adt {
                                constructor: 3,
                                fields: vec![text(cmd)],
                            },
                            lexlean::production::preserve::LexemeKind::Symbol(ch) => {
                                let mut buf = [0u8; 4];
                                Value::Adt {
                                    constructor: 4,
                                    fields: vec![text(ch.encode_utf8(&mut buf))],
                                }
                            }
                        };
                        Value::Adt {
                            constructor: 0,
                            fields: vec![
                                kind,
                                nat_val(lexeme.line),
                                bool_val(lexeme.first_on_line),
                                nat_val(lexeme.start),
                            ],
                        }
                    })
                    .collect(),
            )),
        },
        Err(msg) => Value::Error {
            value: Box::new(text(&msg)),
        },
    }
}

fn token_audit_value(input: &str, imports: &[String]) -> Value {
    let set: std::collections::BTreeSet<String> = imports.iter().cloned().collect();
    match lexlean::production::preserve::audit_tokens(input, &set) {
        Ok(()) => Value::Ok {
            value: Box::new(Value::Unit),
        },
        Err(msg) => Value::Error {
            value: Box::new(text(&msg)),
        },
    }
}

fn lex_run(input: &str) -> Run {
    (
        vec![text(input)],
        serde_json::to_value(lex_value(input)).expect("a value is JSON"),
    )
}

fn token_audit_run(input: &str, imports: &[String]) -> Run {
    (
        vec![text(input), texts(imports)],
        serde_json::to_value(token_audit_value(input, imports)).expect("a value is JSON"),
    )
}

/// The committed unit-test vectors, number-lexing prefixes, and shipped
/// preservation library and calculus modules for `TokenAudit`.
fn token_vectors() -> (Vec<Run>, Vec<Run>) {
    let mut lex_runs = Vec::new();
    let mut audit_runs = Vec::new();
    let default_imports: Vec<String> = lexlean::production::preserve::TARGET_MODULES
        .iter()
        .map(|m| (*m).to_owned())
        .collect();
    let empty_imports: Vec<String> = Vec::new();

    let mut add = |source: &str, imports: &[String]| {
        lex_runs.push(lex_run(source));
        audit_runs.push(token_audit_run(source, imports));
    };

    // 1. All unit vectors from `preserve.rs`.
    for word in ["kernel", "prefix", "macro", "syntax", "extern", "notation"] {
        add(
            &format!("def f («{word}» : Nat) : Nat := «{word}»\n"),
            &empty_imports,
        );
        add(
            &format!("def f ({word} : Nat) : Nat := {word}\n"),
            &empty_imports,
        );
        add(&format!("{word} foo := 1\n"), &empty_imports);
    }
    for source in [
        "@[implemented_by f] def g := 1\n",
        "theorem t : True := sorry\n",
        "def x := Lean.ofReduceBool\n",
        "def x := Lean.«ofReduceBool»\n",
        "theorem z : False := «sorryAx» False true\n",
        "@[«implemented_by» g] def h := 1\n",
        "@[«extern» \"c\"] def h := 1\n",
        "def x := «Lean».«ofReduceNat»\n",
        "@[«macro» foo] def h := 1\n",
        "@[simp, «implemented_by» g] def h := 1\n",
        "def x := «a\ntheorem t : False := sorry\n",
        "def «» := 1\n",
        "def «a b» := 1\n",
        "def s := \"unclosed\ntheorem t : False := sorry\n",
        "/- unclosed\ntheorem t : False := sorry\n",
        "def «sorry» := 1\n",
        "def f («kernel» : Nat) := «kernel» + 1\n",
        "def c1 := '\"'\ntheorem t3 : False := sorry\ndef c2 := '\"'\n",
        "def s := r\"\\\"\ntheorem t : False := sorry\ndef u := r\"\\\"\n",
        "def s := r#\"a\"#\ntheorem t : False := sorry\n",
        "theorem t : False := sorry.1\n",
        "theorem t : False := 0sorry\n",
        "theorem t : False := sorry\u{3bb}x\n",
        "theorem t : False := sorry\u{e9}\n",
        "attribute [«implemented_by» g] h\n",
        "attribute [simp, «extern» c] h\n",
        "def s := s!\"{sorry}\"\n",
        "def c := 'ab'\ntheorem t : False := sorry\n",
        "def c := '\n",
        "def x := 1\u{0}theorem t : False := sorry\n",
        "/- /- nested -/ theorem t : False := sorry\n",
        "def x : Float := 1e10axiom bad : False\ntheorem t : False := bad\n",
        concat!("def x := 1.5e3un", "safe def y := 1\n"),
        "def x := 1e10#evalIO.println \"pwn2\"\n",
        "def x := 0b1sorry\n",
        "def x := 0o7sorry\n",
        "def x := 1e\n",
        "def x := 1e+\n",
        "def x := 0x\n",
        "def x := 1_0axiom bad : False\n",
        "def x := 0b1_0axiom bad : False\n",
        "def x := 0o7_7axiom bad : False\n",
        "def x := 0x1_Fsorry\n",
        "def x := 1_000e5axiom bad : False\n",
        "def x := 1.5_0axiom bad : False\n",
        "def x := 0_0axiom bad : False\n",
        "def x := 1__0axiom bad : False\n",
        "def x := 1.e5axiom bad : False\n",
        "def x := 1.5e+3axiom bad : False\n",
        "def x := 1.5E-3axiom bad : False\n",
        "def x := 1.axiom bad : False\n",
        "def x := 1_\n",
        "def x := 1_ + 1\n",
        "def x := 0x_\n",
        "def x := 1.foo\n",
        "def x := 1._5\n",
        "def x := p.1_0axiom bad : False\n",
        "def x := p.1e5axiom bad : False\n",
        "#evalIO.println \"pwned\"\n",
        "#evalpwned\n",
        "#printaxiom bad\n",
        "#eval 1\n",
        "def s := \"a -- b /- c «d \\\" 'e\"\n",
        "def s := r\"a -- b\"\ndef t := r#\"a\"b\"#\n",
        "def c := ['a', '\\n', '\\'', '\\x41', '\\u0041', '\"']\n",
        "def x' := x'' + f' 1\n",
        "/-- a doc comment: sorry -/ def f := 1\n/- nested /- sorry -/ sorry -/\n-- sorry\n",
        "def «sorry» := 1\ndef x := sorry_ + sorryAlt + «sorry».1\n",
        "def α₁ := β₂ + 0x1F + 1.5 + 2e3\n",
        "def f := fun x => x.1.2 + Nat.succ' 1\n",
        "def s := \"\\u00e9\"\n",
        "def x := [1e10, 1E-3, 1.5e+3, 0xFF, 0b101, 0o17, 1e10 + 2, 0b101e]\n",
        "def x := [1_000, 0b1_0, 0o7_7, 0x1_F, 1_000e5, 1.5_0, 0_0, 1.e5, 1., 1.5E-3]\n",
        "def x := [1..2, 1.0, 0.5, p.1.val, p.2.1, p.10.2 + 1]\n",
        "set_option debug.skipKernelTC true\n",
        "set_option maxRecDepth 100000\n",
        "import Mathlib\n",
        "import LexLeanTarget.TargetSyntax\n",
        "def x := 1\u{7f}\n",
        "def x := 1\u{85}\n",
        "def x := 1\u{9f}\n",
        "def x := 1\u{a0}\n",
        "def 𝒜 := 𝖟 + ℀ + ἀ + Ϛ + xᵢ\n",
    ] {
        add(source, &default_imports);
    }

    // 2. Number-lexing corpus of SP-09 (`numbers_are_read_as_lean_reads_them`).
    let bases = [
        "0", "1", "07", "10", "0x", "0X", "0x0", "0xF", "0x1_F", "0x_", "0b", "0B", "0b0", "0b1",
        "0b2", "0b1_0", "0b_", "0o", "0O", "0o0", "0o7", "0o8", "0o7_7", "0o_", "1.5", "0.5", "1.",
        "0.", "1e5", "1E5", "1e+5", "1E-5", "1.5e3", "1.e3", "p.1", "p.0", "p.12", "(p).1",
    ];
    let tails = [
        "", "_", "_0", "_1", "0_", "1_", "__0", "0__0", "_0_", ".", ".5", ".e5", ".E5", ".5e5",
        ".5e", ".5_0", ".5_", "._5", ".foo", "e5", "E5", "e-5", "E+5", "e+", "e", "e_5", "e5_0",
        "e5_", "_0e5", "_0.5", "_0.e5", "..", "..1", "...", "F", "f", "a", "x", "b1", "o7", "e5.",
        ".1", ".1.2", "_0.1",
    ];
    let mut cases: Vec<String> = Vec::new();
    for base in bases {
        for tail in tails {
            cases.push(format!("def x := {base}{tail}axiom bad : False\n"));
        }
    }
    cases.sort();
    cases.dedup();
    for case in &cases {
        add(case, &default_imports);
    }

    // 3. Shipped preservation library and calculus modules (first 2,048 bytes of each
    // so the evaluator completes quickly while exercising real generated/native Lean).
    let library = lexlean::production::preserve::library().expect("the shipped registry");
    let mut env_imports = default_imports.clone();
    env_imports.push("Init".to_owned());
    for module in &library.modules {
        env_imports.push(format!(
            "{}.{module}",
            lexlean::production::preserve::LIBRARY_PREFIX
        ));
    }
    for module in &library.modules {
        let full = lexlean::production::preserve::module_text(module).expect("module text");
        let snippet: String = full.chars().take(1_500).collect();
        add(&snippet, &env_imports);
    }

    (lex_runs, audit_runs)
}

fn token_corpus(root: &str, count: usize) -> Vec<Run> {
    let mut seeded = Seeded(0xB502_5EED_CAFE_F00D);
    let mut out = Vec::new();
    let atoms: &[&str] = &[
        "def",
        "theorem",
        "fun",
        "match",
        "with",
        "import",
        "set_option",
        "attribute",
        "sorry",
        "axiom",
        "unsafe",
        "partial",
        "kernel",
        "native_decide",
        "implemented_by",
        "extern",
        "ofReduceBool",
        "ofReduceNat",
        "sorryAx",
        "«sorry»",
        "«kernel»",
        "«implemented_by»",
        "«ofReduceBool»",
        "«»",
        "«a b»",
        "«a.b»",
        "«unclosed",
        "Lean.ofReduceBool",
        "Lean.«ofReduceNat»",
        "autoImplicit",
        "maxRecDepth",
        "maxHeartbeats",
        "synthInstance.maxHeartbeats",
        "linter.unusedVariables",
        "linter.unusedSimpArgs",
        "debug.skipKernelTC",
        "Init",
        "LexLeanTarget.TargetSyntax",
        "Mathlib",
        "x",
        "x'",
        "x₁",
        "α",
        "λ",
        "Π",
        "Σ",
        "𝒜",
        "0",
        "1",
        "42",
        "0x1F",
        "0b101",
        "0o77",
        "1_000",
        "1.5",
        "1.5e-3",
        "1e10",
        "1_",
        "0x",
        "1e",
        "1.foo",
        "p.1",
        "p.1.2",
        "p.1_0",
        "p.1e5",
        "1..2",
        "\"hello\"",
        "\"a\\\"b\\nc\"",
        "\"unclosed",
        "r\"raw\"",
        "r#\"raw\"quote\"#",
        "r#\"unclosed\"",
        "s!\"{x}\"",
        "'a'",
        "'\\n'",
        "'\\x41'",
        "'\\u0041'",
        "'ab'",
        "'",
        "-- line comment\n",
        "/- block comment -/",
        "/- nested /- inner -/ outer -/",
        "/- unclosed",
        "#eval",
        "#check",
        "#print",
        "@[",
        "]",
        "[",
        "(",
        ")",
        ":=",
        ":",
        ".",
        ",",
        "+",
        "\n",
        " ",
        "\t",
        "\r\n",
        "\u{0}",
        "\u{7f}",
        "\u{85}",
        "\u{a0}",
    ];
    let import_pool = [
        "Init",
        "LexLeanTarget.TargetSyntax",
        "LexLeanTarget.TargetSemantics",
        "LexLeanPreservation.Core",
    ];
    while out.len() < count {
        let pieces = 1 + seeded.below(8);
        let mut text_buf = String::new();
        for _ in 0..pieces {
            text_buf.push_str(seeded.pick(atoms));
            if seeded.chance(40) {
                text_buf.push(' ');
            }
        }
        let mut imports = Vec::new();
        for candidate in import_pool {
            if seeded.chance(60) {
                imports.push(candidate.to_owned());
            }
        }
        if root == "lex" {
            out.push(lex_run(&text_buf));
        } else {
            out.push(token_audit_run(&text_buf, &imports));
        }
    }
    out
}

const TOKEN_PLANTS: [Plant; 7] = [
    Plant {
        label: "a nested block comment does not increment depth",
        site: "depth := (dep + 1)",
        replacement: "depth := dep",
        root: "lex",
    },
    Plant {
        label: "a hexadecimal literal rejects uppercase A..F digits",
        site: "(inRange byte (65 : UInt8) (70 : UInt8))",
        replacement: "(inRange byte (65 : UInt8) (65 : UInt8))",
        root: "lex",
    },
    Plant {
        label: "an interpolated string is not refused",
        site: "(if (if (not (project firstSeg quoted)) then (if isBanged then isQuoteAfter else false) else false)",
        replacement: "(if false",
        root: "lex",
    },
    Plant {
        label: "a C1 control character (U+0080..U+009F) is not refused",
        site: "(byteIn subject (position + 1) (128 : UInt8) (159 : UInt8))",
        replacement: "(byteIn subject (position + 1) (128 : UInt8) (128 : UInt8))",
        root: "lex",
    },
    Plant {
        label: "a quoted forbidden attribute inside @[...] is not refused",
        site: "(let activeTok : Bool := (if (not (project onlySeg quoted)) then true else (0 <? attributeDepth));",
        replacement: "(let activeTok : Bool := (not (project onlySeg quoted));",
        root: "auditTokens",
    },
    Plant {
        label: "a qualified forbidden constant is not checked",
        site: "(let bad : Bool := (isForbiddenConstant (project seg text));",
        replacement: "(let bad : Bool := false;",
        root: "auditTokens",
    },
    Plant {
        label: "an import of an unlisted module is accepted",
        site: "(let okMod : Bool := (bytesInList modName imports);",
        replacement: "(let okMod : Bool := true;",
        root: "auditTokens",
    },
];

/// Run the case for one `BS` conformance ID, on a stack sized for the
/// interpreter, which recurses with the depth of the program it runs.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub fn run(id: &str) {
    let id = id.to_owned();
    if let Err(failure) = std::thread::Builder::new()
        .stack_size(1 << 30)
        .spawn(move || case(&id))
        .expect("thread")
        .join()
    {
        std::panic::resume_unwind(failure);
    }
}

#[allow(clippy::too_many_lines)]
fn case(id: &str) {
    match id {
        // §17.23: the first component, the axiom-output parser.
        "BS-01" => {
            let started = Instant::now();
            // `Text` is the standard library's own module, imported as a
            // source package (§10.5), not a copy.
            let project = P::bootstrap();
            assert!(
                !project.root.join("src/Text.lex.tex").as_std_path().exists(),
                "the project holds no copy of the standard library's Text module"
            );
            let checked = project.check_ok();
            assert!(
                checked.units.contains_key("LexLeanStd.Text"),
                "the standard library's Text module is linked as a module of the project"
            );
            let built = project.build_ok();
            let build_id = built.build_id.expect("built");
            let report: Json = serde_json::from_slice(
                &std::fs::read(
                    project
                        .build_dir(&build_id)
                        .join("production/LexLeanBoot/AxiomOutput.eligibility.json")
                        .as_std_path(),
                )
                .expect("the eligibility report"),
            )
            .expect("the report is JSON");
            let reported = report["roots"].as_array().expect("roots");
            assert_eq!(
                reported
                    .iter()
                    .map(|row| row["root"].as_str().expect("root").to_owned())
                    .collect::<Vec<_>>(),
                ROOTS
                    .iter()
                    .map(|(root, _)| format!("{MODULE}.{root}"))
                    .collect::<Vec<_>>(),
                "the two roots are the module's production roots, in order"
            );
            for (row, (root, effects)) in reported.iter().zip(ROOTS) {
                let declared: Vec<&str> = row["declared_effects"]
                    .as_array()
                    .expect("declared effects")
                    .iter()
                    .map(|effect| effect.as_str().expect("an effect"))
                    .collect();
                assert_eq!(declared, effects, "{root}: the declared effects");
                let targets = row["targets"].as_array().expect("targets");
                assert_eq!(targets.len(), 1, "{root}: one target");
                let target = &targets[0];
                assert_eq!(target["target"], "rust-std", "{root}: the target");
                assert_eq!(target["status"], "eligible", "{root}: eligible");
                let realized: Vec<&str> = target["effects"]
                    .as_array()
                    .expect("effects")
                    .iter()
                    .map(|effect| effect["effect"].as_str().expect("an effect"))
                    .collect();
                assert_eq!(
                    realized, effects,
                    "{root}: the declaration admits exactly the realized effects"
                );
            }
            // Every root lowers once. On the normative host the evaluator runs
            // the committed vectors and its seeded corpus; on every host the
            // rendered rust-std package compiled by the pinned rustc runs the
            // deterministic vectors (and on the normative host the full seeded
            // corpus as well).
            let on_normative_host = support::normative_host();
            let checked = support::checked_project(&project);
            let lowered: BTreeMap<&str, Lowered> = ROOTS
                .iter()
                .map(|(root, _)| (*root, lower(&checked, &project, MODULE, root)))
                .collect();
            let (golden_runs, accepted, rejected) = vectors();
            assert!(
                accepted > 0 && rejected > 0,
                "the vectors cover both outcomes (accepted {accepted}, rejected {rejected})"
            );
            let corpus = |root: &str, audits: usize, names: usize| {
                if root == "parseAuditOutput" {
                    audit_corpus(audits)
                } else {
                    name_corpus(names)
                }
            };
            if on_normative_host {
                let vector_count = golden_runs.len();
                let disagreements = evaluate(&lowered["parseAuditOutput"], &golden_runs, 5);
                assert!(
                    disagreements.is_empty(),
                    "the evaluator and verify::axiom disagree on the committed vectors: {disagreements:?}"
                );
                eprintln!(
                    "BS-01: {vector_count} committed vectors ({accepted} accepted, {rejected} rejected) agree under the evaluator"
                );
                for (root, _) in ROOTS {
                    let runs = corpus(root, EVALUATOR_AUDITS, EVALUATOR_NAMES);
                    let disagreements = evaluate(&lowered[root], &runs, 5);
                    assert!(
                        disagreements.is_empty(),
                        "{root}: the evaluator and the Rust oracle disagree: {disagreements:?}"
                    );
                    eprintln!(
                        "BS-01: {root}: {} evaluator runs agree ({:?})",
                        runs.len(),
                        started.elapsed()
                    );
                }
            }
            // The rendered rust-std package of both roots, compiled by the
            // pinned rustc, runs the deterministic vectors on every host and
            // the large seeded corpus on the normative host.
            let dir = tempfile::Builder::new()
                .prefix("lexlean-bootstrap-")
                .tempdir()
                .expect("tempdir");
            let mut crates = Vec::new();
            let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
            for (root, _) in ROOTS {
                let low = &lowered[root];
                let manifest = rust_packages::manifest(
                    &format!("boot_{}", snake(root)),
                    &low.program,
                    low.entry(),
                    Profile::Std,
                );
                let package =
                    target::package(&manifest.to_file_bytes().expect("canonical manifest"))
                        .unwrap_or_else(|failure| panic!("{root}: {failure:?}"));
                let runs = if on_normative_host {
                    corpus(root, RUSTC_AUDITS, RUSTC_NAMES)
                } else if root == "parseAuditOutput" {
                    audit_vectors()
                } else {
                    name_vectors()
                };
                let arguments: Vec<Vec<Value>> = runs
                    .iter()
                    .map(|(arguments, _)| arguments.clone())
                    .collect();
                let arity = low.program.functions[usize::try_from(low.entry()).expect("entry")]
                    .types
                    .len();
                let passing = vec![Passing::Own; arity];
                let harness_source = rust_harness::render_calls(
                    &low.program,
                    Profile::Std,
                    &manifest.name,
                    &[Calls {
                        entry: low.entry(),
                        function: "run",
                        passing: &passing,
                        arguments: &arguments,
                    }],
                )
                .expect("the differential harness");
                expected.insert(
                    manifest.name.clone(),
                    runs.into_iter().map(|(_, expected)| expected).collect(),
                );
                crates.push(Built {
                    name: manifest.name,
                    files: package.files,
                    harness: harness_source,
                });
            }
            workspace(dir.path(), &crates);
            let build = cargo_in(
                dir.path(),
                &["build", "--offline", "--workspace", "--quiet"],
            );
            assert!(
                build.status.success(),
                "a root's package does not build:\n{}",
                String::from_utf8_lossy(&build.stderr)
            );
            eprintln!("BS-01: rustc built both packages ({:?})", started.elapsed());
            for (name, outcomes) in &expected {
                let binary = dir
                    .path()
                    .join("target/debug")
                    .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
                let ran = std::process::Command::new(&binary)
                    .output()
                    .expect("the differential runs");
                assert!(
                    ran.status.success(),
                    "{name}: {}",
                    String::from_utf8_lossy(&ran.stderr)
                );
                let stdout = String::from_utf8(ran.stdout).expect("utf8");
                let printed: Vec<Json> = stdout
                    .lines()
                    .filter(|line| !line.starts_with("work "))
                    .map(|line| {
                        serde_json::from_str(line)
                            .unwrap_or_else(|error| panic!("{name}: {error}: {line}"))
                    })
                    .collect();
                assert_eq!(printed.len(), outcomes.len(), "{name}: one outcome per run");
                let disagreements: Vec<String> = printed
                    .iter()
                    .zip(outcomes)
                    .enumerate()
                    .filter(|(_, (printed, expected))| printed != expected)
                    .map(|(run, (printed, expected))| format!("run {run}: {printed} != {expected}"))
                    .collect();
                assert!(
                    disagreements.is_empty(),
                    "{name}: the rendering and the Rust oracle disagree on {} of {} runs: {:?}",
                    disagreements.len(),
                    outcomes.len(),
                    &disagreements[..disagreements.len().min(5)]
                );
                eprintln!(
                    "BS-01: {name}: {} rustc runs agree with the oracle ({:?})",
                    outcomes.len(),
                    started.elapsed()
                );
            }
            if !support::lean_backed("BS-01") {
                return;
            }
            // The gate is armed on the normative host: each defect planted in a
            // copy of the source is a disagreement with the oracle.
            for plant in &PLANTS {
                let planted = P::bootstrap();
                let source = planted.read_raw("src/AxiomOutput.lex.tex");
                assert_eq!(
                    source.matches(plant.site).count(),
                    1,
                    "{}: the plant site occurs once",
                    plant.label
                );
                planted.write(
                    "src/AxiomOutput.lex.tex",
                    &source.replacen(plant.site, plant.replacement, 1),
                );
                let checked = support::checked_project(&planted);
                let low = lower(&checked, &planted, MODULE, plant.root);
                let disagreements = evaluate(
                    &low,
                    &corpus(plant.root, EVALUATOR_AUDITS, EVALUATOR_NAMES),
                    1,
                );
                assert!(
                    !disagreements.is_empty(),
                    "{}: the planted defect is detected by the differential",
                    plant.label
                );
                eprintln!(
                    "BS-01: plant detected: {}: {}",
                    plant.label, disagreements[0]
                );
            }
            // Under the pinned Lean, the production export certifies both
            // roots; its packages are the committed ones and realize the
            // programs the differential ran.
            let exported = harness::exported("bootstrap");
            let bundle = exported.bundle();
            let packages = harness::tree(&bundle.join("packages"));
            let committed = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
            assert_eq!(
                packages.keys().collect::<Vec<_>>(),
                committed.keys().collect::<Vec<_>>(),
                "the exported package files are the committed ones"
            );
            for (relative, content) in &packages {
                assert!(
                    committed.get(relative) == Some(content),
                    "packages/{relative} differs from the committed package"
                );
            }
            let mut realized = 0usize;
            for root in &exported.output.result.roots {
                let Some(member) = root.root.strip_prefix(&format!("{MODULE}.")) else {
                    continue;
                };
                for output in &root.outputs {
                    let package: Json = serde_json::from_slice(
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
                        serde_json::to_value(&lowered[member].program).expect("a program is JSON"),
                        "{member}: the exported package realizes the program the differential ran"
                    );
                    realized += 1;
                }
            }
            assert_eq!(realized, ROOTS.len(), "one certified package per root");
            eprintln!("BS-01: passed in {:?}", started.elapsed());
        }
        // §17.23, §17.17: the second component, the preservation lexer and token audit.
        "BS-02" => {
            let started = Instant::now();
            let project = P::bootstrap();
            let checked = project.check_ok();
            assert!(
                checked.units.contains_key("TokenAudit")
                    && checked.units.contains_key("LexLeanStd.Text"),
                "TokenAudit and LexLeanStd.Text are linked modules of the project"
            );
            let built = project.build_ok();
            let build_id = built.build_id.expect("built");
            let report: Json = serde_json::from_slice(
                &std::fs::read(
                    project
                        .build_dir(&build_id)
                        .join("production/LexLeanBoot/TokenAudit.eligibility.json")
                        .as_std_path(),
                )
                .expect("the eligibility report"),
            )
            .expect("the report is JSON");
            let reported = report["roots"].as_array().expect("roots");
            assert_eq!(
                reported
                    .iter()
                    .map(|row| row["root"].as_str().expect("root").to_owned())
                    .collect::<Vec<_>>(),
                TOKEN_ROOTS
                    .iter()
                    .map(|(root, _)| format!("{TOKEN_MODULE}.{root}"))
                    .collect::<Vec<_>>(),
                "the two roots are TokenAudit's production roots, in order"
            );
            for (row, (root, effects)) in reported.iter().zip(TOKEN_ROOTS) {
                let declared: Vec<&str> = row["declared_effects"]
                    .as_array()
                    .expect("declared effects")
                    .iter()
                    .map(|effect| effect.as_str().expect("an effect"))
                    .collect();
                assert_eq!(declared, effects, "{root}: the declared effects");
                let targets = row["targets"].as_array().expect("targets");
                assert_eq!(targets.len(), 1, "{root}: one target");
                let target = &targets[0];
                assert_eq!(target["target"], "rust-std", "{root}: the target");
                assert_eq!(target["status"], "eligible", "{root}: eligible");
                let realized: Vec<&str> = target["effects"]
                    .as_array()
                    .expect("effects")
                    .iter()
                    .map(|effect| effect["effect"].as_str().expect("an effect"))
                    .collect();
                assert_eq!(
                    realized, effects,
                    "{root}: the declaration admits exactly the realized effects"
                );
            }

            let on_normative_host = support::normative_host();
            let checked = support::checked_project(&project);
            let lowered: BTreeMap<&str, Lowered> = TOKEN_ROOTS
                .iter()
                .map(|(root, _)| (*root, lower(&checked, &project, TOKEN_MODULE, root)))
                .collect();
            let (lex_vectors, audit_vectors) = token_vectors();
            if on_normative_host {
                let disagreements = evaluate(&lowered["lex"], &lex_vectors, 5);
                assert!(
                    disagreements.is_empty(),
                    "lex: the evaluator and production::preserve::lex disagree on the vectors: {disagreements:?}"
                );
                let disagreements = evaluate(&lowered["auditTokens"], &audit_vectors, 5);
                assert!(
                    disagreements.is_empty(),
                    "auditTokens: the evaluator and production::preserve::audit_tokens disagree on the vectors: {disagreements:?}"
                );
                eprintln!(
                    "BS-02: {} lex vectors and {} auditTokens vectors agree under the evaluator ({:?})",
                    lex_vectors.len(),
                    audit_vectors.len(),
                    started.elapsed()
                );
                for (root, _) in TOKEN_ROOTS {
                    let runs = token_corpus(root, EVALUATOR_TOKEN_TEXTS);
                    let disagreements = evaluate(&lowered[root], &runs, 5);
                    assert!(
                        disagreements.is_empty(),
                        "{root}: the evaluator and the Rust oracle disagree: {disagreements:?}"
                    );
                    eprintln!(
                        "BS-02: {root}: {} seeded evaluator runs agree ({:?})",
                        runs.len(),
                        started.elapsed()
                    );
                }
            }

            // Rendered rust-std packages under pinned rustc: the deterministic
            // vectors run on every host, and the seeded corpus on the normative
            // host.
            let dir = tempfile::Builder::new()
                .prefix("lexlean-bootstrap-tokens-")
                .tempdir()
                .expect("tempdir");
            let mut crates = Vec::new();
            let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
            for (root, _) in TOKEN_ROOTS {
                let low = &lowered[root];
                let manifest = rust_packages::manifest(
                    &format!("boot_{}", snake(root)),
                    &low.program,
                    low.entry(),
                    Profile::Std,
                );
                let package =
                    target::package(&manifest.to_file_bytes().expect("canonical manifest"))
                        .unwrap_or_else(|failure| panic!("{root}: {failure:?}"));
                let mut runs = if root == "lex" {
                    lex_vectors.clone()
                } else {
                    audit_vectors.clone()
                };
                if on_normative_host {
                    runs.extend(token_corpus(root, RUSTC_TOKEN_TEXTS));
                }
                let arguments: Vec<Vec<Value>> = runs
                    .iter()
                    .map(|(arguments, _)| arguments.clone())
                    .collect();
                let arity = low.program.functions[usize::try_from(low.entry()).expect("entry")]
                    .types
                    .len();
                let passing = vec![Passing::Own; arity];
                let harness_source = rust_harness::render_calls(
                    &low.program,
                    Profile::Std,
                    &manifest.name,
                    &[Calls {
                        entry: low.entry(),
                        function: "run",
                        passing: &passing,
                        arguments: &arguments,
                    }],
                )
                .expect("the differential harness");
                expected.insert(
                    manifest.name.clone(),
                    runs.into_iter().map(|(_, expected)| expected).collect(),
                );
                crates.push(Built {
                    name: manifest.name,
                    files: package.files,
                    harness: harness_source,
                });
            }
            workspace(dir.path(), &crates);
            let build = cargo_in(
                dir.path(),
                &["build", "--offline", "--workspace", "--quiet"],
            );
            assert!(
                build.status.success(),
                "a root's package does not build:\n{}",
                String::from_utf8_lossy(&build.stderr)
            );
            eprintln!("BS-02: rustc built both packages ({:?})", started.elapsed());
            for (name, outcomes) in &expected {
                let binary = dir
                    .path()
                    .join("target/debug")
                    .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
                let ran = std::process::Command::new(&binary)
                    .output()
                    .expect("the differential runs");
                assert!(
                    ran.status.success(),
                    "{name}: {}",
                    String::from_utf8_lossy(&ran.stderr)
                );
                let stdout = String::from_utf8(ran.stdout).expect("utf8");
                let printed: Vec<Json> = stdout
                    .lines()
                    .filter(|line| !line.starts_with("work "))
                    .map(|line| {
                        serde_json::from_str(line)
                            .unwrap_or_else(|error| panic!("{name}: {error}: {line}"))
                    })
                    .collect();
                assert_eq!(printed.len(), outcomes.len(), "{name}: one outcome per run");
                let disagreements: Vec<String> = printed
                    .iter()
                    .zip(outcomes)
                    .enumerate()
                    .filter(|(_, (printed, expected))| printed != expected)
                    .map(|(run, (printed, expected))| format!("run {run}: {printed} != {expected}"))
                    .collect();
                assert!(
                    disagreements.is_empty(),
                    "{name}: the rendering and the Rust oracle disagree on {} of {} runs: {:?}",
                    disagreements.len(),
                    outcomes.len(),
                    &disagreements[..disagreements.len().min(5)]
                );
                eprintln!(
                    "BS-02: {name}: {} rustc runs agree with the oracle ({:?})",
                    outcomes.len(),
                    started.elapsed()
                );
            }

            if !support::lean_backed("BS-02") {
                return;
            }
            // Planted defects in TokenAudit.lex.tex.
            for plant in &TOKEN_PLANTS {
                let planted = P::bootstrap();
                let source = planted.read_raw("src/TokenAudit.lex.tex");
                assert_eq!(
                    source.matches(plant.site).count(),
                    1,
                    "{}: the plant site occurs once",
                    plant.label
                );
                planted.write(
                    "src/TokenAudit.lex.tex",
                    &source.replacen(plant.site, plant.replacement, 1),
                );
                let checked = support::checked_project(&planted);
                let low = lower(&checked, &planted, TOKEN_MODULE, plant.root);
                let runs = if plant.root == "lex" {
                    &lex_vectors
                } else {
                    &audit_vectors
                };
                let disagreements = evaluate(&low, runs, 1);
                assert!(
                    !disagreements.is_empty(),
                    "{}: the planted defect is detected by the differential",
                    plant.label
                );
                eprintln!(
                    "BS-02: plant detected: {}: {}",
                    plant.label, disagreements[0]
                );
            }

            let exported = harness::exported("bootstrap");
            let bundle = exported.bundle();
            let packages = harness::tree(&bundle.join("packages"));
            let committed = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
            assert_eq!(
                packages.keys().collect::<Vec<_>>(),
                committed.keys().collect::<Vec<_>>(),
                "the exported package files are the committed ones"
            );
            for (relative, content) in &packages {
                assert!(
                    committed.get(relative) == Some(content),
                    "packages/{relative} differs from the committed package"
                );
            }
            let mut realized = 0usize;
            for root in &exported.output.result.roots {
                let Some(member) = root.root.strip_prefix(&format!("{TOKEN_MODULE}.")) else {
                    continue;
                };
                for output in &root.outputs {
                    let package: Json = serde_json::from_slice(
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
                        serde_json::to_value(&lowered[member].program).expect("a program is JSON"),
                        "{member}: the exported package realizes the program the differential ran"
                    );
                    realized += 1;
                }
            }
            assert_eq!(
                realized,
                TOKEN_ROOTS.len(),
                "one certified package per root"
            );
            eprintln!("BS-02: passed in {:?}", started.elapsed());
        }
        other => panic!("no bootstrap case is wired for {other}"),
    }
}
