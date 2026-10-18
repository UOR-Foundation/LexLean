//! The third component (SPEC.md §17.23, §12.2): the primitive scanner,
//! `source::scan::{scan, reject_forbidden_atoms}`, as `LexLeanBoot.PrimitiveScan`.
//! Its oracle is the Rust scanner, which stays until the fixed point.

use std::collections::BTreeMap;
use std::time::Instant;

use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::Profile;
use lexlean::calculus::{self as target, Value};
use lexlean::diagnostic::{Diagnostic, Span};
use lexlean::production::lower::Lowered;
use lexlean::source::atom::{Atom, AtomClass};
use serde_json::Value as Json;

use super::{evaluate, list, lower, nat_val, option, snake, text, texts, Plant, Run, Seeded};
use crate::production_export as harness;
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support::{self, P};

const MODULE: &str = "LexLeanBoot.PrimitiveScan";

/// The production roots in the order the module declares them, each with the
/// effects its declaration admits, which are exactly the effects its closure
/// realizes.
const ROOTS: [(&str, &[&str]); 2] = [
    ("scanSource", &["allocation", "overflow", "recursion"]),
    ("rejectAtoms", &["allocation", "overflow", "recursion"]),
];

/// The window of every committed source the evaluator reads, in bytes (cut at
/// scalar boundaries): the evaluator interprets the lowered program, so the
/// gate reads a window of each source and leaves the whole sources to the
/// rendered package.
const EVALUATOR_WINDOW: usize = 128;
/// Seeded edge texts under the evaluator.
const EVALUATOR_SEEDED: usize = 80;
/// Seeded edge texts under rustc.
const RUSTC_SEEDED: usize = 2_500;
/// Of the seeded texts, those whose atoms `rejectAtoms` reads under rustc: its
/// argument is the whole atom list, which the harness spells out, and a
/// harness of a few hundred thousand atoms does not compile.
const RUSTC_SEEDED_REJECTS: usize = 500;
/// The prefix of every committed source `scanSource` reads under rustc.
const RUSTC_PREFIX: usize = 2_048;
/// The largest committed source run whole through `scanSource` under rustc.
const WHOLE_SCAN_CAP: usize = 8_192;
/// The largest committed source run whole through `rejectAtoms` under rustc,
/// and which of those: one in `REJECT_EVERY`, by a hash of the path.
const WHOLE_REJECT_CAP: usize = 1_024;
const REJECT_EVERY: u64 = 4;
/// One in this many committed sources contributes its window to `rejectAtoms`
/// under rustc.
const WINDOW_REJECT_EVERY: u64 = 6;

/// Paths a run is named by; the limit message carries the path.
const PATHS: &[&str] = &[
    "m",
    "src/Demo.lex.tex",
    "\u{fc}/\u{2115}.lex.tex",
    "a b",
    "",
    "x/y/z.lex.tex",
];

/// Whole atoms and fragments of atoms, one of each lexical shape.
const TOKENS: &[&str] = &[
    "a",
    "Zz",
    "abc",
    "word",
    "1",
    "0",
    "00",
    "007",
    "01",
    "10",
    "12345",
    "{",
    "}",
    "(",
    ")",
    "[",
    "]",
    "+",
    "-",
    "_",
    "'",
    "\"",
    "`",
    "#",
    "~",
    "!",
    "=",
    ",",
    ".",
    ":",
    "\\\\",
    "\\ ",
    "\\{",
    "\\}",
    "\\(",
    "\\)",
    "\\1",
    "\\a",
    "\\ab",
    "\\abc",
    "\\begin",
    "\\end",
    "\\def",
    "\\gdef",
    "\\verb",
    "\\verbatim",
    "\\input",
    "\\usepackage",
    "\\define",
    "\\semanticdata",
    "\\semanticdataX",
    "\\toolingdata",
    "\\coredata",
    "\\x1",
    "\u{e9}",
    "\u{2115}",
    "\u{65e5}",
    "\u{1f600}",
    "\u{3b1}",
    " ",
    "  ",
    "\n",
    "\n\n",
    " \n ",
    "\u{0}",
    "\u{1}",
    "\u{7}",
    "\u{1f}",
    "\u{7f}",
    "\t",
    "\r",
    "\u{80}",
    "\\\u{e9}",
    "\\\u{2115}",
    "\\\n",
    "\\\u{1}",
    "x01",
    "add-01",
    "x_01",
    "x'01",
    "(01)",
    "1.05",
    "n + 007",
    "-01",
    "_01",
    "'01",
    " 01",
    "a 01",
    "a\n01",
    "0a",
    "a0",
];

/// The environment and payload shapes `SemanticDataContext` tracks.
const ENVIRONMENTS: &[&str] = &[
    "\\begin{semanticmodule}",
    "\\begin{toolingmodule}",
    "\\begin{coremodule}",
    "\\begin {semanticmodule}",
    "\\begin{ semanticmodule}",
    "\\begin{semanticmodule }",
    "\\begin{semanticmodules}",
    "\\begin[semanticmodule]",
    "\\begin{semanticmodule}\n",
    "\\begin{semanticmodule}\n\n  ",
    "\\end{semanticmodule}",
    "\\end",
    "\\semanticdata{",
    "\\semanticdata",
    "\\semanticdata {",
    "\\semanticdata\n{",
    "\\toolingdata{{\"a\":\"007\"}}",
    "\\coredata{{\"a\":007}}",
    "\\semanticdata{{\"value\":\"007\"}}",
    "\\semanticdata{{\"value\":007}}",
];

const PAYLOADS: &[&str] = &[
    "{\"v\":\"007\"}",
    "\"",
    "\\\"",
    "\\\\",
    "\"} 007 {\"",
    "{{",
    "}}",
    "{",
    "}",
    "\"007\"",
    "\"unterminated 007",
    "\"esc\\\" 007 \"",
    "\"\\end\"",
    "\\end",
    "\"\\semanticdata{ 007\"",
    "\\semanticdata{ 007",
    " 007 ",
    "\n008\n",
    "x 009",
];

fn class_index(class: AtomClass) -> u64 {
    match class {
        AtomClass::Control => 0,
        AtomClass::Word => 1,
        AtomClass::Numeral => 2,
        AtomClass::Delimiter => 3,
        AtomClass::AsciiSymbol => 4,
        AtomClass::UnicodeSymbol => 5,
        AtomClass::Whitespace => 6,
    }
}

fn atom_value(atom: &Atom) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            Value::Adt {
                constructor: class_index(atom.class),
                fields: Vec::new(),
            },
            nat_val(atom.byte_start),
            nat_val(atom.byte_end),
            nat_val(atom.line_start),
            nat_val(atom.column_start),
            nat_val(atom.line_end),
            nat_val(atom.column_end),
            text(&atom.text),
        ],
    }
}

fn place_value(span: &Span) -> Value {
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

/// The oracle's diagnostic as the `Rejection` the roots return: its code,
/// message, primary span, and help. The scanner sets nothing else, which the
/// conversion asserts rather than drops.
fn rejection_value(diagnostic: &Diagnostic) -> Value {
    assert!(
        diagnostic.labels.is_empty()
            && diagnostic.notes.is_empty()
            && diagnostic.causes.is_empty()
            && diagnostic.help.len() <= 1,
        "the scanner sets a code, a message, a span, and at most one help"
    );
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(diagnostic.code.as_str()),
            text(&diagnostic.message),
            option(diagnostic.primary.as_ref().map(place_value)),
            option(diagnostic.help.first().map(|help| text(help))),
        ],
    }
}

/// What each corpus contains of each outcome, so that a differential which
/// never saw a rejection cannot pass for one that did (R2).
#[derive(Default)]
struct Tally(BTreeMap<String, usize>);

impl Tally {
    fn note(&mut self, outcome: &str) {
        *self.0.entry(outcome.to_owned()).or_insert(0) += 1;
    }
}

fn scan_run(tally: &mut Tally, path: &str, source: &str, max: u64) -> Run {
    let value = match lexlean::source::scan::scan(path, source, max) {
        Ok(atoms) => {
            tally.note("scan accepted");
            Value::Ok {
                value: Box::new(list(atoms.iter().map(atom_value).collect())),
            }
        }
        Err(diagnostic) => {
            tally.note(&format!("scan {}", diagnostic.code.as_str()));
            Value::Error {
                value: Box::new(rejection_value(&diagnostic)),
            }
        }
    };
    (
        vec![text(path), text(source), nat_val_u64(max)],
        serde_json::to_value(value).expect("a value is JSON"),
    )
}

fn nat_val_u64(n: u64) -> Value {
    Value::Nat {
        value: n.to_string(),
    }
}

fn reject_run(tally: &mut Tally, path: &str, atoms: &[Atom], forbidden: &[String]) -> Run {
    let value = match lexlean::source::scan::reject_forbidden_atoms(path, atoms, forbidden) {
        Ok(()) => {
            tally.note("reject accepted");
            Value::None
        }
        Err(diagnostic) => {
            tally.note(&format!("reject {}", diagnostic.code.as_str()));
            Value::Some {
                value: Box::new(rejection_value(&diagnostic)),
            }
        }
    };
    (
        vec![
            text(path),
            list(atoms.iter().map(atom_value).collect()),
            texts(forbidden),
        ],
        serde_json::to_value(value).expect("a value is JSON"),
    )
}

/// The runs of one text: `scanSource` with the limit given, and `rejectAtoms`
/// on the atoms the oracle scans from it when it scans.
fn runs(
    tally: &mut Tally,
    path: &str,
    source: &str,
    max: u64,
    forbidden: &[String],
) -> (Run, Option<Run>) {
    let scan = scan_run(tally, path, source, max);
    let reject = lexlean::source::scan::scan(path, source, u64::MAX)
        .ok()
        .map(|atoms| reject_run(tally, path, &atoms, forbidden));
    (scan, reject)
}

/// Both roots' runs, one corpus each.
#[derive(Default)]
struct Corpus {
    scan: Vec<Run>,
    reject: Vec<Run>,
    /// The runs of `reject` that the rustc harness reads.
    reject_rustc: Vec<Run>,
}

impl Corpus {
    fn add(&mut self, tally: &mut Tally, path: &str, source: &str, max: u64, forbidden: &[String]) {
        let (scan, reject) = runs(tally, path, source, max, forbidden);
        self.scan.push(scan);
        self.reject.extend(reject.clone());
        self.reject_rustc.extend(reject);
    }

    fn of(&self, root: &str) -> &[Run] {
        if root == "scanSource" {
            &self.scan
        } else {
            &self.reject
        }
    }

    fn append(&mut self, other: Self) {
        self.scan.extend(other.scan);
        self.reject.extend(other.reject);
        self.reject_rustc.extend(other.reject_rustc);
    }
}

/// A run whose expected outcome is kept as text and parsed when it is
/// compared: a whole source has as many atoms as it has scalars, and a tree of
/// them for every source at once does not fit in memory.
struct Heavy {
    arguments: Vec<Value>,
    expected: String,
}

impl From<Run> for Heavy {
    fn from((arguments, expected): Run) -> Self {
        Self {
            arguments,
            expected: expected.to_string(),
        }
    }
}

/// The forbidden controls the compiler passes, from the language data.
fn forbidden_controls() -> Vec<String> {
    lexlean::lexicon::load_bootstrap()
        .expect("the bootstrap data")
        .structural
        .forbidden_controls
        .clone()
}

/// Every committed `.lex.tex` source of the checkout that §12.1 accepts, as
/// `link.rs` reads it (normalized), with its repository-relative path.
fn sources() -> Vec<(String, String)> {
    let root = support::repo_root();
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
        if !entry.file_type().is_file() || !path.to_string_lossy().ends_with(".lex.tex") {
            continue;
        }
        let relative = path
            .strip_prefix(root.as_std_path())
            .expect("under the root")
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = std::fs::read(path).expect("a source is readable");
        if let Ok(normalized) = lexlean::source::normalize::normalize(&relative, &bytes, false) {
            out.push((relative, normalized.text));
        }
    }
    out
}

/// The longest prefix of `source` of at most `bytes` bytes that ends on a
/// scalar boundary.
fn prefix(source: &str, bytes: usize) -> &str {
    let mut end = bytes.min(source.len());
    while !source.is_char_boundary(end) {
        end -= 1;
    }
    &source[..end]
}

/// The deterministic edge vectors: the oracle's own unit-test inputs, the
/// atom limit at N and N+1, each rejection, and the context shapes.
fn vectors(tally: &mut Tally, forbidden: &[String]) -> Corpus {
    let mut corpus = Corpus::default();
    let big = 2_000_000;
    let mut add = |tally: &mut Tally, path: &str, source: &str, max: u64| {
        corpus.add(tally, path, source, max, forbidden);
    };
    for source in [
        "",
        "\\begin{x} ab 12 + \u{2115}\n",
        "\\(x\\)",
        "\\\\\\ \\1",
        "\\ab1",
        "\\\u{2115}",
        "\\",
        "\\\n",
        "\\\u{1}",
        "a\u{0}b",
        "a\tb",
        "a\rb",
        "a\u{7f}b",
        "\u{1f}",
        "\u{2115}\n+",
        "\u{fc}\u{e9}\u{2115}\u{1f600}x\n\u{3b1}",
        "x1_2' y",
        "a \\def b",
        "a \\define b",
        "n + 007",
        "n + 0",
        "n + 70",
        "x01",
        "add-01",
        "x_01",
        "x'01",
        "f(0)",
        "(01)",
        "00",
        "1.05",
        "0",
        "01 02",
        "a  \n \n b",
        "\u{fc}\u{fc}\u{fc}\u{fc}\u{fc}\n\u{fc}",
        "\\input\\verb\\verbatim\\write",
    ] {
        add(tally, "m", source, big);
    }
    // The limit at N and N+1, and at the extremes.
    for source in ["a b c", "\\x \u{2115}12(", "", "  ", "a", "x01 y"] {
        let count = lexlean::source::scan::scan("m", source, u64::MAX)
            .map_or(0, |atoms| atoms.len() as u64);
        for max in [
            0,
            1,
            count.saturating_sub(1),
            count,
            count + 1,
            9,
            10,
            99,
            100,
            12_345,
            1_000_000,
            u64::MAX,
        ] {
            add(tally, "m", source, max);
        }
    }
    for path in PATHS {
        add(tally, path, "a b", 2);
    }
    // Payload strings, notation bodies, and the shapes of the context.
    for source in [
        "\\semanticdata{{\"value\":\"sha256:007abc\"}}",
        "\\semanticdata{{\"value\":007}}",
        "\\semanticdata{{\"value\":\"ok\"}} \"007\"",
        "sha256:007abc",
        "\\semanticdata{{\"value\":\"escaped \\\" and 007\"}}",
        "\\semanticdata \n {{\"value\":\"007\"}}",
        "\\semanticdata{{\"value\":\"escaped \\\\semanticdata{ 007\"}}",
        "\\semanticdata{{\"value\":\"} 007 {\"}}",
        "\\semanticdata{{\"value\":\"007\"}} \\semanticdata{{\"next\":\"008\"}}",
        "\\semanticdataOther{{\"value\":\"007\"}}",
        "\\\\semanticdata{{\"value\":\"007\"}}",
        "\\semanticdata{{\"value\":\"007\"}} 008",
        "\\semanticdata{{\"a\":\"b\"} \"007\" }",
        "\\semanticdata{{\"a\":\"b\"},\"k\":\"007\"}",
        "\\begin{semanticmodule}\n\\semanticdata{{\"v\":\"007\"}}\n\\end{semanticmodule}",
        "\\begin{semanticmodule}\n\\semanticdata{{\"v\":007}}\n\\end{semanticmodule}",
        "\\begin{semanticmodule}\nnotation \"007\" and \"\\end\" 008\n\\end{semanticmodule} 009",
        "\\begin{semanticmodule}\nnotation 007\n\\end{semanticmodule}",
        "\\begin{semanticmodule}\nx \"unterminated \\end{semanticmodule} 007",
        "\\begin{semanticmodule}\n  \n\\toolingdata{{\"v\":\"007\"}}\n\\end{semanticmodule} 007",
        "\\begin{toolingmodule}\n\\toolingdata{{\"v\":\"007\"}}\n\\end{toolingmodule}",
        "\\begin{coremodule}\n\\coredata{{\"v\":\"007\"}}\n\\end{coremodule}",
        "\\begin{coremodule}\n\\semanticdata{{\"v\":\"007\"}}",
        "\\begin{coremodule}\ndefinition \"x007\" 007",
        "\\begin{coremodule}\nnotation \"a 007 b\" 008\n\\end{coremodule}",
        "\\begin{toolingmodule}\nnotation \"a 007 b\" 008\n\\end{toolingmodule}",
        "\\begin{semanticmodule}\nnotation \"a 007 b\" 008\n\\end{semanticmodule}",
        "\\begin{semanticmodules}\n\\semanticdata{{\"v\":\"007\"}}",
        "\\begin {semanticmodule}\n\\semanticdata{{\"v\":\"007\"}}",
        "\\begin{semanticmodule}\n\\coredata{{\"a\":{\"b\":\"007\"}}}",
        "\\begin{semanticmodule}\n\\end{semanticmodule}\n\\semanticdata{{\"v\":\"007\"}}",
        "\\begin{semanticmodule}\n\\semanticdata{{\"v\":\"\\\\\"}} 007",
        "\\begin{semanticmodule}\n\\semanticdata{{\"v\":\"a\\\"}} 007",
        "\\begin{semanticmodule}\nx \"a\\\\\" 007\n\\end{semanticmodule}",
        "\\begin{semanticmodule}\nx \"a\\\"b\" 007\n\\end{semanticmodule}",
        "\\begin{semanticmodule}\n\\end 007",
        "\\semanticdata{}\\semanticdata{}007",
        "\\semanticdata{{{\"v\":\"007\"}}}\\def",
        "\\begin\n{semanticmodule}\n\"007\"\\end{semanticmodule}\n007",
        "\\begin{toolingmodule}\n\\semanticdata{007}",
        "\\begin{semanticmodule}\n\"\\end{semanticmodule}\n007",
    ] {
        add(tally, "m", source, big);
    }
    let gaps = gapped(tally, forbidden);
    corpus.reject.extend(gaps.iter().cloned());
    corpus.reject_rustc.extend(gaps);
    corpus
}

/// `rejectAtoms` on atom lists the scanner does not produce: the atoms of a
/// text without its white space, so that neighbours do not touch. The scanner
/// never leaves a gap, and a check of adjacency that always holds would pass
/// every scanned text.
fn gapped(tally: &mut Tally, forbidden: &[String]) -> Vec<Run> {
    let mut out = Vec::new();
    for source in [
        "x 01",
        "add - 01",
        "x_ 01 y",
        "a 007 b",
        "n + 007",
        "' 01",
        "_ 01",
        "x\n01",
        "\\def 01 \\input",
        "\\semanticdata {\"v\" : 007}",
        "a 01 b 02",
    ] {
        let source = source.replace("\\n", "\n");
        let atoms: Vec<Atom> = lexlean::source::scan::scan("m", &source, u64::MAX)
            .expect("scans")
            .into_iter()
            .filter(|atom| atom.class != AtomClass::Whitespace)
            .collect();
        out.push(reject_run(tally, "m", &atoms, forbidden));
    }
    out
}

/// A seeded edge text: a run of atoms, environments, and payload shapes.
fn edge_text(seeded: &mut Seeded) -> String {
    let mut out = String::new();
    let pieces = 1 + seeded.below(16);
    for _ in 0..pieces {
        match seeded.below(10) {
            0 | 1 => out.push_str(seeded.pick(ENVIRONMENTS)),
            2 | 3 => out.push_str(seeded.pick(PAYLOADS)),
            _ => out.push_str(seeded.pick(TOKENS)),
        }
        if seeded.chance(30) {
            out.push_str(seeded.pick(&[" ", "\n", "  ", "\n\n"]));
        }
    }
    out
}

/// The seeded corpus: edge texts, and committed sources cut and spliced.
fn seeded(
    tally: &mut Tally,
    count: usize,
    sources: &[(String, String)],
    forbidden: &[String],
) -> Corpus {
    let mut corpus = Corpus::default();
    let mut generator = Seeded(0x5CA9_0003_9D2F_77A1);
    for _ in 0..count {
        let mut source = edge_text(&mut generator);
        if generator.chance(25) {
            let (_, whole) = generator.pick(sources);
            let cut = prefix(whole, generator.below(whole.len().min(600) + 1));
            source = if generator.chance(50) {
                format!("{cut}{source}")
            } else {
                format!("{source}{cut}")
            };
        }
        let path = *generator.pick(PATHS);
        let max = if generator.chance(70) {
            2_000_000
        } else {
            let atoms = lexlean::source::scan::scan(path, &source, u64::MAX)
                .map_or(3, |atoms| atoms.len() as u64);
            let near = [atoms.saturating_sub(1), atoms, atoms + 1, 0, 1, 5];
            near[generator.below(near.len())]
        };
        let mut chosen = forbidden.to_vec();
        match generator.below(5) {
            0 => chosen.clear(),
            1 => chosen.retain(|_| generator.chance(50)),
            2 => chosen.push("\\semanticdata".to_owned()),
            _ => {}
        }
        corpus.add(tally, path, &source, max, &chosen);
    }
    corpus
}

/// A hash of a path (FNV-1a), which fixes where a window lies and which
/// sources the rustc harness for `rejectAtoms` reads.
fn path_hash(path: &str) -> u64 {
    path.bytes().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// A window of `bytes` bytes of each committed source, from a place that
/// depends on the path alone (the first for `at == 0`), so that the windows
/// reach the bodies of the sources and not only their headers.
fn windows(
    tally: &mut Tally,
    sources: &[(String, String)],
    bytes: usize,
    spread: bool,
    forbidden: &[String],
) -> Corpus {
    let mut corpus = Corpus::default();
    for (path, source) in sources {
        let at = if spread && source.len() > bytes {
            usize::try_from(
                path_hash(path) % u64::try_from(source.len() - bytes).expect("a length"),
            )
            .expect("an offset")
        } else {
            0
        };
        let mut start = at.min(source.len());
        while !source.is_char_boundary(start) {
            start -= 1;
        }
        let window = prefix(&source[start..], bytes);
        let (scan, reject) = runs(tally, path, window, 2_000_000, forbidden);
        corpus.scan.push(scan);
        if let Some(reject) = reject {
            if path_hash(path) % WINDOW_REJECT_EVERY == 0 {
                corpus.reject_rustc.push(reject.clone());
            }
            corpus.reject.push(reject);
        }
    }
    corpus
}

/// `scanSource` on a prefix of each committed source: the text is the whole
/// argument, so a prefix costs the harness what it weighs.
fn prefix_scans(tally: &mut Tally, sources: &[(String, String)]) -> Vec<Heavy> {
    sources
        .iter()
        .map(|(path, source)| {
            Heavy::from(scan_run(
                tally,
                path,
                prefix(source, RUSTC_PREFIX),
                2_000_000,
            ))
        })
        .collect()
}

/// The committed sources whole, up to the caps, as the heavy runs of both
/// roots.
fn wholes(
    tally: &mut Tally,
    sources: &[(String, String)],
    forbidden: &[String],
) -> (Vec<Heavy>, Vec<Heavy>) {
    let (mut scans, mut rejects) = (Vec::new(), Vec::new());
    for (path, source) in sources {
        if source.len() > WHOLE_SCAN_CAP {
            continue;
        }
        let (scan, reject) = runs(tally, path, source, 2_000_000, forbidden);
        scans.push(Heavy::from(scan));
        if source.len() <= WHOLE_REJECT_CAP && path_hash(path) % REJECT_EVERY == 0 {
            rejects.extend(reject.map(Heavy::from));
        }
    }
    (scans, rejects)
}

/// Plants: a defect in the source, the root whose differential must catch it.
const PLANTS: [Plant; 10] = [
    Plant {
        label: "a run of white space stops at a line feed",
        site: "(if space then true else feed)",
        replacement: "space",
        root: "scanSource",
    },
    Plant {
        label: "a backslash before any scalar below U+0100 is a control",
        site: "(let nbAscii : Bool := (inRange nb (0 : UInt8) (127 : UInt8))",
        replacement: "(let nbAscii : Bool := (inRange nb (0 : UInt8) (255 : UInt8))",
        root: "scanSource",
    },
    Plant {
        label: "a column counts the bytes of a scalar",
        site: "(record Cursor {position := (pos + width), line := (project cur line), column := ((project cur column) + 1)})",
        replacement: "(record Cursor {position := (pos + width), line := (project cur line), column := ((project cur column) + width)})",
        root: "scanSource",
    },
    Plant {
        label: "the atom limit admits one atom more",
        site: "(if (maxAtoms <? count)",
        replacement: "(if (maxAtoms <? (prim subtract count 1 : Nat))",
        root: "scanSource",
    },
    Plant {
        label: "a lone zero has a redundant leading zero",
        site: "(if (code =? 2) then (if longer then zeroLead else false) else false)",
        replacement: "(if (code =? 2) then zeroLead else false)",
        root: "rejectAtoms",
    },
    Plant {
        label: "a numeral after identifier material need not touch it",
        site: "((project before byteEnd) =? (project atom byteStart))",
        replacement: "true",
        root: "rejectAtoms",
    },
    Plant {
        label: "a leading zero inside a payload string is rejected",
        site: "(if continues then (ctor Option.none [Rejection]) else (if (project ctx quoted) then (ctor Option.none [Rejection]) else",
        replacement: "(if continues then (ctor Option.none [Rejection]) else (if false then (ctor Option.none [Rejection]) else",
        root: "rejectAtoms",
    },
    Plant {
        label: "a coremodule environment is not a payload",
        site: "0x636f72656d6f64756c65",
        replacement: "0x636f72656d6f64756c66",
        root: "rejectAtoms",
    },
    Plant {
        label: "an escaped backslash escapes the next scalar",
        site: "quoted := true, escaped := true})) else (if isQuote",
        replacement: "quoted := true, escaped := false})) else (if isQuote",
        root: "rejectAtoms",
    },
    Plant {
        label: "a brace opens no level of a payload",
        site: "depth := (depth + 1), quoted := false",
        replacement: "depth := depth, quoted := false",
        root: "rejectAtoms",
    },
];

/// The rendered `rust-std` package of each root, compiled by the pinned
/// rustc, run on its corpus; every outcome is the oracle's.
fn rustc_differential(
    lowered: &BTreeMap<&str, Lowered>,
    corpora: &[(&str, Vec<Heavy>)],
    started: Instant,
) {
    let dir = tempfile::Builder::new()
        .prefix("lexlean-bootstrap-scan-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (root, runs) in corpora {
        let low = &lowered[root];
        let manifest = rust_packages::manifest(
            &format!("boot_{}", snake(root)),
            &low.program,
            low.entry(),
            Profile::Std,
        );
        let package = target::package(&manifest.to_file_bytes().expect("canonical manifest"))
            .unwrap_or_else(|failure| panic!("{root}: {failure:?}"));
        let arguments: Vec<Vec<Value>> = runs.iter().map(|run| run.arguments.clone()).collect();
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
            runs.iter().map(|run| run.expected.clone()).collect(),
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
    eprintln!("BS-03: rustc built both packages ({:?})", started.elapsed());
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
        let mut disagreements: Vec<String> = Vec::new();
        let mut printed = 0usize;
        for (run, line) in stdout
            .lines()
            .filter(|line| !line.starts_with("work "))
            .enumerate()
        {
            let observed: Json =
                serde_json::from_str(line).unwrap_or_else(|error| panic!("{name}: {error}"));
            let expected: Json =
                serde_json::from_str(&outcomes[run]).expect("the expected outcome");
            if observed != expected {
                let shown = |value: &Json| value.to_string().chars().take(300).collect::<String>();
                disagreements.push(format!(
                    "run {run}: {} != {}",
                    shown(&observed),
                    shown(&expected)
                ));
            }
            printed += 1;
        }
        assert_eq!(printed, outcomes.len(), "{name}: one outcome per run");
        assert!(
            disagreements.is_empty(),
            "{name}: the rendering and the Rust oracle disagree on {} of {} runs: {:?}",
            disagreements.len(),
            outcomes.len(),
            &disagreements[..disagreements.len().min(5)]
        );
        eprintln!(
            "BS-03: {name}: {} rustc runs agree with the oracle ({:?})",
            outcomes.len(),
            started.elapsed()
        );
    }
}

/// The eligibility of the module's roots, read from the build's report.
fn assert_eligible(project: &P) {
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanBoot/PrimitiveScan.eligibility.json")
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
        "the two roots are PrimitiveScan's production roots, in order"
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
}

/// The outcomes an honest differential has seen: every rejection the scanner
/// has, and both verdicts of the check.
fn assert_covers(label: &str, tally: &Tally) {
    for outcome in [
        "scan accepted",
        "scan LLL1001",
        "scan LLL1004",
        "scan LLS8002",
        "reject accepted",
        "reject LLL1002",
        "reject LLL1003",
    ] {
        assert!(
            tally.0.get(outcome).copied().unwrap_or(0) > 0,
            "{label}: the corpus holds no `{outcome}` outcome: {:?}",
            tally.0
        );
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = Instant::now();
    let project = P::bootstrap();
    let checked = project.check_ok();
    assert!(
        checked.units.contains_key("PrimitiveScan")
            && checked.units.contains_key("LexLeanStd.Text"),
        "PrimitiveScan and LexLeanStd.Text are linked modules of the project"
    );
    assert_eligible(&project);
    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, lower(&checked, &project, MODULE, root)))
        .collect();

    let forbidden = forbidden_controls();
    let sources = sources();
    eprintln!("BS-03: {} committed sources", sources.len());
    let mut tally = Tally::default();
    let mut deterministic = vectors(&mut tally, &forbidden);
    deterministic.append(windows(
        &mut tally,
        &sources,
        EVALUATOR_WINDOW,
        true,
        &forbidden,
    ));
    assert_covers("deterministic", &tally);
    let normative = support::normative_host();
    let mut seeded_tally = Tally::default();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded(&mut seeded_tally, EVALUATOR_SEEDED, &sources, &forbidden),
            seeded(&mut seeded_tally, RUSTC_SEEDED, &sources, &forbidden),
        )
    } else {
        (Corpus::default(), Corpus::default())
    };
    if normative {
        assert_covers("seeded", &seeded_tally);
    }

    // Every host: the rendered packages, compiled by this host's rustc, on
    // the deterministic vectors, prefixes and whole sources; the normative
    // host adds the seeded corpus.
    let heavies = |runs: &[Run]| runs.iter().cloned().map(Heavy::from).collect::<Vec<_>>();
    let mut rustc_scan = heavies(&deterministic.scan);
    let mut rustc_reject = heavies(&deterministic.reject_rustc);
    rustc_scan.extend(prefix_scans(&mut tally, &sources));
    let (whole_scan, whole_reject) = wholes(&mut tally, &sources, &forbidden);
    eprintln!(
        "BS-03: deterministic ({}, {}) runs, prefixes {}, whole sources ({}, {}), outcomes {:?} ({:?})",
        deterministic.scan.len(),
        deterministic.reject.len(),
        sources.len(),
        whole_scan.len(),
        whole_reject.len(),
        tally.0,
        started.elapsed()
    );
    rustc_scan.extend(whole_scan);
    rustc_reject.extend(whole_reject);
    rustc_scan.extend(heavies(&rustc_seeded.scan));
    rustc_reject.extend(heavies(
        &rustc_seeded.reject[..rustc_seeded.reject.len().min(RUSTC_SEEDED_REJECTS)],
    ));
    rustc_differential(
        &lowered,
        &[("scanSource", rustc_scan), ("rejectAtoms", rustc_reject)],
        started,
    );

    if !normative {
        eprintln!(
            "BS-03: platform-bound host: the evaluator differential, the seeded corpora, the planted defects, and the export ran on the normative host only"
        );
        return;
    }

    // The lowered programs under the calculus evaluator.
    for (root, _) in ROOTS {
        for (label, corpus) in [
            ("vectors", deterministic.of(root)),
            ("seeded", evaluator_seeded.of(root)),
        ] {
            let disagreements = evaluate(&lowered[root], corpus, 5);
            assert!(
                disagreements.is_empty(),
                "{root}: the evaluator and the Rust oracle disagree on the {label}: {disagreements:?}"
            );
            eprintln!(
                "BS-03: {root}: {} {label} evaluator runs agree ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
    }

    // The gate is armed: each defect planted in a copy of the source is a
    // disagreement with the oracle on the evaluator's corpus.
    for plant in &PLANTS {
        let planted = P::bootstrap();
        let source = planted.read_raw("src/PrimitiveScan.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/PrimitiveScan.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = lower(&checked, &planted, MODULE, plant.root);
        let mut corpus = deterministic.of(plant.root).to_vec();
        corpus.extend(evaluator_seeded.of(plant.root).iter().cloned());
        let disagreements = evaluate(&low, &corpus, 1);
        assert!(
            !disagreements.is_empty(),
            "{}: the planted defect is detected by the differential",
            plant.label
        );
        eprintln!(
            "BS-03: plant detected: {}: {}",
            plant.label,
            disagreements[0].chars().take(240).collect::<String>()
        );
    }

    if !support::lean_backed("BS-03") {
        return;
    }
    // Under the pinned Lean, the production export certifies the roots; its
    // packages are the committed ones and realize the programs the
    // differential ran.
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
    eprintln!("BS-03: passed in {:?}", started.elapsed());
}
