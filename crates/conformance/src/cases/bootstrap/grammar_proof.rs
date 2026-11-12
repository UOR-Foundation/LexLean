//! `BS-35` (SPEC.md §17.23, §15.5, §16.2): the text-channel tokens and the
//! simple proof sentences as language-1.2 source, `LexLeanBoot.GrammarProof`,
//! held to the byte to `lexlean::grammar::chart::text_tokens` and
//! `lexlean::grammar::proof::parse_proof_sentence`.
//!
//! `textTokens` drops white space and folds each mathematical island of an
//! atom range into one token, refusing a dollar delimiter, a nested or unclosed
//! island, and a closing control that closes none; `parseProofSentence` reads
//! the tokens of one sentence as one of the registered simple sentences, with
//! the keywords that cover it, or names the forbidden proof form it holds. Both
//! read the atoms `scanSource` of `BS-03` returns, so the host composes them.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::Value;
use lexlean::grammar::chart::{text_tokens, TextToken};
use lexlean::grammar::proof::{parse_proof_sentence, ProofSentence, SentenceAstKind};
use lexlean::grammar::proposition::TextParser;
use lexlean::lexicon::package::LoadContext;
use lexlean::lexicon::resolve::Closure;
use lexlean::source::atom::{Atom, AtomClass};
use serde_json::Value as Json;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::scan::{atom_value, rejection_value};
use super::{bool_val, list, nat_val, text, Run, Seeded};
use crate::production_export as harness;
use crate::support;

const MODULE: &str = "LexLeanBoot.GrammarProof";
const SOURCE: &str = "src/GrammarProof.lex.tex";
const ROOTS: [(&str, &[&str]); 2] = [
    ("textTokens", &["allocation", "overflow"]),
    ("parseProofSentence", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The atoms a range of a committed source holds at most, and the atoms after
/// it the root is also given, which the composition of a tactic word reads.
const RANGE_ATOMS: usize = 48;
const TAIL_ATOMS: usize = 8;
/// The committed sources whose ranges are drawn: those of at most this many
/// bytes.
const SOURCE_BYTES: usize = 400_000;
/// Ranges of committed sources under the evaluator and under rustc.
const EVALUATOR_RANGES: usize = 600;
const RUSTC_RANGES: usize = 4_000;
/// Seeded sentences under the evaluator and under rustc.
const EVALUATOR_SEEDED: usize = 400;
const RUSTC_SEEDED: usize = 3_000;
/// The path every vector is read under.
const PATH: &str = "src/Proof.lex.tex";

// ------------------------------------------------------------- the encoders

fn token_value(token: &TextToken) -> Value {
    match token {
        TextToken::Atom(index) => Value::Adt {
            constructor: 0,
            fields: vec![nat_val(*index)],
        },
        TextToken::Island {
            open,
            inner_start,
            inner_end,
            close,
            display,
        } => Value::Adt {
            constructor: 1,
            fields: vec![
                nat_val(*open),
                nat_val(*inner_start),
                nat_val(*inner_end),
                nat_val(*close),
                bool_val(*display),
            ],
        },
    }
}

fn tokens_value(tokens: &[TextToken]) -> Value {
    list(tokens.iter().map(token_value).collect())
}

fn kind_value(kind: &SentenceAstKind) -> Value {
    let (constructor, fields) = match kind {
        SentenceAstKind::Assume { islands } => (0, vec![tokens_value(islands)]),
        SentenceAstKind::Apply { term } => (1, vec![token_value(term)]),
        SentenceAstKind::CloseWith { term } => (2, vec![token_value(term)]),
        SentenceAstKind::CloseByReflexivity => (3, Vec::new()),
        SentenceAstKind::Witness { term } => (4, vec![token_value(term)]),
        SentenceAstKind::SelectLeft => (5, Vec::new()),
        SentenceAstKind::SelectRight => (6, Vec::new()),
    };
    Value::Adt {
        constructor,
        fields,
    }
}

fn sentence_value(sentence: &ProofSentence) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            kind_value(&sentence.kind),
            list(
                sentence
                    .keywords
                    .iter()
                    .map(|keyword| Value::Adt {
                        constructor: 0,
                        fields: vec![nat_val(keyword.atom), text(keyword.entry)],
                    })
                    .collect(),
            ),
        ],
    }
}

fn atoms_value(atoms: &[Atom]) -> Value {
    list(atoms.iter().map(atom_value).collect())
}

/// The built-in closure (`lexlean.core`, `lexlean.std.nat`), which the
/// sentence parser is given and does not read.
fn closure() -> &'static Closure {
    static CLOSURE: std::sync::OnceLock<Closure> = std::sync::OnceLock::new();
    CLOSURE.get_or_init(|| {
        let bootstrap = lexlean::lexicon::load_bootstrap().expect("bootstrap loads");
        let ctx = LoadContext {
            language: lexlean::LANGUAGE_VERSION,
            forbidden_controls: &bootstrap.structural.forbidden_controls,
            max_scope_depth: 1024,
        };
        let packages = bootstrap
            .builtin_packages
            .iter()
            .map(|row| lexlean::lexicon::load_builtin_package(row, &ctx).expect("a package"))
            .collect();
        let registry = lexlean::lexicon::load_token_registry().expect("the registry");
        Closure::build(packages, registry, bootstrap, 128).expect("the closure")
    })
}

/// What the roots say of an atom range: the run of `textTokens`, its
/// tokens when it accepts, and the oracle's message when it refuses.
fn tokens_run(
    path: &str,
    atoms: &[Atom],
    start: usize,
    end: usize,
) -> (Run, Option<Vec<TextToken>>, Option<String>) {
    let (value, tokens, refusal) = match text_tokens(path, atoms, start, end) {
        Ok(tokens) => (
            Value::Ok {
                value: Box::new(tokens_value(&tokens)),
            },
            Some(tokens),
            None,
        ),
        Err(diagnostic) => (
            Value::Error {
                value: Box::new(rejection_value(&diagnostic)),
            },
            None,
            Some(diagnostic.message.clone()),
        ),
    };
    (
        (
            vec![text(path), atoms_value(atoms), nat_val(start), nat_val(end)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        tokens,
        refusal,
    )
}

fn sentence_run(path: &str, atoms: &[Atom], tokens: &[TextToken]) -> (Run, Option<String>) {
    let visible = BTreeSet::new();
    let parser = TextParser {
        path,
        atoms,
        tokens,
        closure: closure(),
        visible: &visible,
        sentence_initial: true,
    };
    let (value, refusal) = match parse_proof_sentence(&parser) {
        Ok(sentence) => (
            Value::Ok {
                value: Box::new(sentence_value(&sentence)),
            },
            None,
        ),
        Err(diagnostic) => (
            Value::Error {
                value: Box::new(rejection_value(&diagnostic)),
            },
            Some(diagnostic.message.clone()),
        ),
    };
    (
        (
            vec![text(path), atoms_value(atoms), tokens_value(tokens)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

fn scanned(source: &str) -> Vec<Atom> {
    lexlean::source::scan::scan(PATH, source, u64::MAX)
        .unwrap_or_else(|failure| panic!("{source:?}: {failure:?}"))
}

// ------------------------------------------------------------------ vectors

/// One named vector: a text whose atoms are tokenized whole, and, when they
/// tokenize, read as a sentence.
struct Vector {
    label: String,
    source: String,
    /// The first atom of the range; the range runs to the last atom.
    start: usize,
    /// The atoms without their white space, which the scanner never returns:
    /// neighbours that do not touch hold the composition to bytes.
    gapped: bool,
}

/// The two runs of a vector: the tokens, and the sentence when there are
/// tokens; with the oracle's messages.
fn vector_runs(source: &str, start: usize, gapped: bool) -> (Run, Option<Run>, Vec<String>) {
    let atoms: Vec<Atom> = scanned(source)
        .into_iter()
        .filter(|atom| !gapped || atom.class != AtomClass::Whitespace)
        .collect();
    let (run, tokens, refusal) = tokens_run(PATH, &atoms, start, atoms.len());
    let mut messages: Vec<String> = refusal.into_iter().collect();
    let sentence = tokens.map(|tokens| {
        let (run, refusal) = sentence_run(PATH, &atoms, &tokens);
        messages.extend(refusal);
        run
    });
    (run, sentence, messages)
}

/// The forbidden words of §16.12, each of which a vector holds.
const FORBIDDEN: &[&str] = &[
    "aesop",
    "admit",
    "cases",
    "constructor",
    "induction",
    "all_goals",
    "any_goals",
    "apply?",
    "assumption",
    "case",
    "decide",
    "exact",
    "exact?",
    "first",
    "intro",
    "linarith",
    "native_decide",
    "next",
    "omega",
    "refine",
    "repeat",
    "rfl",
    "rw",
    "rwa",
    "simp",
    "simp_all",
    "simpa",
    "sorry",
    "tauto",
    "trivial",
    "try",
];

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out: Vec<Vector> = Vec::new();
    let mut add = |label: &str, source: &str| {
        out.push(Vector {
            label: label.to_owned(),
            source: source.to_owned(),
            start: 0,
            gapped: false,
        });
    };
    // The tokens.
    add("an empty range", "");
    add("white space only", "  \n ");
    add("words and symbols", "Close the goal, by x.");
    add("an inline island", "Apply \\(f x\\)");
    add("a display island", "Apply \\[f x\\]");
    add("two islands", "\\(a\\) and \\[b\\]");
    add("a dollar delimiter", "Apply $x$");
    add("a dollar after an island", "\\(x\\) $");
    add("a nested inline island", "\\(a \\(b\\) c\\)");
    add("a display island in an inline one", "\\(a \\[b\\]\\)");
    add("an unclosed inline island", "Apply \\(x");
    add("an unclosed display island", "Apply \\[x");
    add("an inline island closed as display", "\\(x\\]");
    add("a display island closed as inline", "\\[x\\) y");
    add("a stray inline close", "x \\) y");
    add("a stray display close", "x \\] y");
    add("an empty island", "\\(\\)");
    add("an island at the end", "Use \\(w\\)");
    // The sentences.
    add("assume one", "Assume \\(x\\)");
    add("assume two", "Assume \\(x\\), \\(y\\)");
    add("assume three", "Assume \\(x\\), \\(y\\), \\[z\\]");
    add("assume without an island", "Assume x");
    add("assume with a trailing comma", "Assume \\(x\\),");
    add("assume with a comma and a word", "Assume \\(x\\), y");
    add("assume with a word after", "Assume \\(x\\) y");
    add("assume with two commas", "Assume \\(x\\),, \\(y\\)");
    add("assume alone", "Assume");
    add("apply", "Apply \\(h\\)");
    add("apply with a word after", "Apply \\(h\\) now");
    add("apply without an island", "Apply h");
    add("close with", "Close the goal with \\(h\\)");
    add("close by reflexivity", "Close the goal by reflexivity");
    add("close with nothing after", "Close the goal with");
    add("close with a word after", "Close the goal with \\(h\\) x");
    add(
        "close by reflexivity and more",
        "Close the goal by reflexivity x",
    );
    add("close by something else", "Close the goal by induction");
    add("close without the goal", "Close the aim with \\(h\\)");
    add("close without the", "Close goal with \\(h\\)");
    add("close and nothing", "Close the goal");
    add("close with a word", "Close the goal with h");
    add("use as the witness", "Use \\(w\\) as the witness");
    add("use without as", "Use \\(w\\) for the witness");
    add("use without the witness", "Use \\(w\\) as the");
    add("use with more", "Use \\(w\\) as the witness x");
    add("use without an island", "Use w as the witness");
    add("select left", "Select the left alternative");
    add("select right", "Select the right alternative");
    add("select neither", "Select the middle alternative");
    add("select without alternative", "Select the left");
    add("select with more", "Select the right alternative x");
    add("select without the", "Select left alternative");
    add("a lower-case verb", "apply \\(h\\)");
    add("a verb as a symbol", "\\(h\\) Apply");
    add("an island first", "\\(h\\)");
    add("a numeral first", "3 Apply \\(h\\)");
    add("a tactic in the middle", "Close the goal by simp");
    add("a tactic at the end", "Apply \\(h\\) omega");
    add("a composed tactic", "Close the goal with exact?");
    add("a composed underscore tactic", "native_decide");
    add("an underscore word first", "foo_bar the goal");
    add("an underscore word later", "Close the foo_bar");
    add("an underscore word after a space", "Close foo _bar");
    add("a combinator", "Apply \\(h\\) <;> simp");
    add("a combinator alone", "<;>");
    add("a combinator split", "< ;> x");
    add("a focusing dot", "· Apply \\(h\\)");
    add("a tactic broken by a comma", "simp,all");
    add("a tactic broken by a space", "exact ?");
    add("a tactic word joined to a numeral", "rw2 x");
    add("a tactic word in an island", "Apply \\(simp\\)");
    add("a tactic word after a period", "Apply \\(h\\). simp");
    add("a word with an apostrophe", "Apply' \\(h\\)");
    add("use as the something", "Use \\(w\\) as the proof");
    for word in FORBIDDEN {
        let source = format!("Close the goal by {word}");
        out.push(Vector {
            label: format!("the forbidden word {word}"),
            source,
            start: 0,
            gapped: false,
        });
    }
    for (label, source, start) in [
        ("a range after a dollar", "$ Apply \\(h\\)", 1),
        ("a range inside an island", "\\(a b\\)", 1),
        ("a range at its last atom", "x y", 2),
        ("a range past its last atom", "x", 1),
    ] {
        out.push(Vector {
            label: label.to_owned(),
            source: source.to_owned(),
            start,
            gapped: false,
        });
    }
    for (label, source) in [
        ("a tactic split by a dropped space", "exact ?"),
        (
            "an island and words without white space",
            "Use \\(w\\) as the witness",
        ),
    ] {
        out.push(Vector {
            label: label.to_owned(),
            source: source.to_owned(),
            start: 0,
            gapped: true,
        });
    }
    out
}

/// The refusals a vector must reach, each by its message.
const REFUSALS: &[&str] = &[
    "dollar math delimiters are forbidden",
    "mathematical islands do not nest",
    "unclosed mathematical island; expected `\\)`",
    "unclosed mathematical island; expected `\\]`",
    "`\\)` closes no mathematical island",
    "`\\]` closes no mathematical island",
    "is a forbidden proof form",
    "not a registered proof sentence",
];

// ------------------------------------------------------------------- seeded

const PIECES: &[&str] = &[
    "Assume",
    "Apply",
    "Close",
    "Use",
    "Select",
    "the",
    "goal",
    "with",
    "by",
    "reflexivity",
    "as",
    "witness",
    "left",
    "right",
    "alternative",
    ",",
    ",",
    "\\(x\\)",
    "\\[y\\]",
    "\\(",
    "\\)",
    "\\[",
    "\\]",
    "$",
    "simp",
    "exact",
    "?",
    "_",
    "native",
    "<;>",
    "<",
    ";",
    ">",
    "·",
    "x",
    "3",
    ".",
    "(",
    ")",
    "'",
    "rfl",
    "first",
    "foo_bar",
];

fn sentence(seeded: &mut Seeded) -> String {
    let count = 1 + seeded.below(7);
    let mut out = String::new();
    for index in 0..count {
        if index > 0 && seeded.chance(70) {
            out.push(' ');
        }
        out.push_str(seeded.pick(PIECES));
    }
    out
}

/// A registered sentence with one edit, or a sentence drawn from the pieces.
fn seeded_source(seeded: &mut Seeded, bases: &[String]) -> String {
    if seeded.chance(50) {
        let base = seeded.pick(bases).clone();
        let mut words: Vec<String> = base.split(' ').map(str::to_owned).collect();
        let at = seeded.below(words.len() + 1);
        match seeded.below(3) {
            0 if at < words.len() => {
                words.remove(at);
            }
            1 => words.insert(at, (*seeded.pick(PIECES)).to_owned()),
            _ if at < words.len() => words[at] = (*seeded.pick(PIECES)).to_owned(),
            _ => words.push((*seeded.pick(PIECES)).to_owned()),
        }
        words.join(" ")
    } else {
        sentence(seeded)
    }
}

fn seeded_runs(count: usize, bases: &[String], seed: u64) -> (Vec<Run>, Vec<Run>) {
    let mut seeded = Seeded(seed);
    let mut tokens = Vec::with_capacity(count);
    let mut sentences = Vec::with_capacity(count);
    while tokens.len() < count {
        let source = seeded_source(&mut seeded, bases);
        let (run, sentence, _) = vector_runs(&source, 0, false);
        tokens.push(run);
        sentences.extend(sentence);
    }
    (tokens, sentences)
}

/// The runs of `textTokens` and of `parseProofSentence` on drawn ranges.
type RangeRuns = (Vec<Run>, Vec<Run>);

/// The ranges of one scanned source: up to [`RANGE_ATOMS`] atoms from an atom
/// where a range may start (a period ends the one before it, as the elaborator
/// cuts sentences), with [`TAIL_ATOMS`] atoms after it.
fn ranges_of(seeded: &mut Seeded, path: &str, atoms: &[Atom], count: usize, out: &mut RangeRuns) {
    if atoms.is_empty() {
        return;
    }
    for _ in 0..count {
        let from = seeded.below(atoms.len());
        let start = (from..atoms.len())
            .find(|index| atoms[*index].text == "." || *index == atoms.len() - 1)
            .map_or(from, |index| (index + 1).min(atoms.len() - 1));
        let span = 1 + seeded.below(RANGE_ATOMS);
        let end = (start + span).min(atoms.len());
        let end = (start..end)
            .find(|index| atoms[*index].text == ".")
            .unwrap_or(end);
        let window = &atoms[start..(end + TAIL_ATOMS).min(atoms.len())];
        let (run, found, _) = tokens_run(path, window, 0, end - start);
        out.0.push(run);
        if let Some(found) = found {
            out.1.push(sentence_run(path, window, &found).0);
        }
    }
}

/// Seeded atom ranges of every committed source of at most [`SOURCE_BYTES`]
/// bytes, each source scanned once and dropped, the same number from each,
/// for the evaluator and for rustc: the scanned atoms of the whole checkout
/// do not fit in memory at once.
fn source_runs() -> (usize, RangeRuns, RangeRuns) {
    let sources: Vec<(String, String)> = super::scan::sources()
        .into_iter()
        .filter(|(_, content)| content.len() <= SOURCE_BYTES)
        .collect();
    let mut evaluator = (Vec::new(), Vec::new());
    let mut compiled = (Vec::new(), Vec::new());
    let mut evaluator_seeded = Seeded(0x0B55_0035);
    let mut rustc_seeded = Seeded(0x0B55_1035);
    let evaluator_each = EVALUATOR_RANGES.div_ceil(sources.len().max(1));
    let rustc_each = RUSTC_RANGES.div_ceil(sources.len().max(1));
    for (path, content) in &sources {
        let Ok(atoms) = lexlean::source::scan::scan(path, content, u64::MAX) else {
            continue;
        };
        if evaluator.0.len() < EVALUATOR_RANGES {
            ranges_of(
                &mut evaluator_seeded,
                path,
                &atoms,
                evaluator_each,
                &mut evaluator,
            );
        }
        if compiled.0.len() < RUSTC_RANGES {
            ranges_of(&mut rustc_seeded, path, &atoms, rustc_each, &mut compiled);
        }
    }
    (sources.len(), evaluator, compiled)
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
        label: "white space is a token",
        root: 0,
        site: "(if (code =? 6) then",
        replacement: "(if false then",
        vector: "words and symbols",
    },
    Plant {
        label: "a dollar is a word",
        root: 0,
        site: "(lit \"$\")",
        replacement: "(lit \"$$\")",
        vector: "a dollar delimiter",
    },
    Plant {
        label: "a display island is inline",
        root: 0,
        site: "(let shown : Bool := (isControl atom (lit \"\\\\[\"));",
        replacement: "(let shown : Bool := false;",
        vector: "a display island",
    },
    Plant {
        label: "islands nest",
        root: 0,
        site: "(if (opens atom) then (ctor Option.none [Nat]) else (ctor Option.some [Nat] (position + 1)))",
        replacement: "(ctor Option.some [Nat] (position + 1))",
        vector: "a nested inline island",
    },
    Plant {
        label: "the scan resumes at the close",
        root: 0,
        site: "(pushed scan (reached + 1) (ctor TextToken.TokenIsland",
        replacement: "(pushed scan reached (ctor TextToken.TokenIsland",
        vector: "two islands",
    },
    Plant {
        label: "the inner range begins at the open",
        root: 0,
        site: "(ctor TextToken.TokenIsland position (position + 1) reached reached shown)",
        replacement: "(ctor TextToken.TokenIsland position position reached reached shown)",
        vector: "an inline island",
    },
    Plant {
        label: "an unclosed island names its opener",
        root: 0,
        site: "(lit \"unclosed mathematical island; expected `\") closer)",
        replacement: "(lit \"unclosed mathematical island; expected `\") (project atom text))",
        vector: "an unclosed inline island",
    },
    Plant {
        label: "a stray display close is a word",
        root: 0,
        site: "(if (if (isControl atom (lit \"\\\\)\")) then true else (isControl atom (lit \"\\\\]\"))) then",
        replacement: "(if (isControl atom (lit \"\\\\)\")) then",
        vector: "a stray display close",
    },
    Plant {
        label: "the range starts at its first atom",
        root: 0,
        site: "(record Scan {cursor := start, tokens := (nil TextToken)",
        replacement: "(record Scan {cursor := 0, tokens := (nil TextToken)",
        vector: "a range after a dollar",
    },
    Plant {
        label: "assume needs no island",
        root: 1,
        site: "(if (islandIs tokens 1) then (let final : Commas",
        replacement: "(if true then (let final : Commas",
        vector: "assume without an island",
    },
    Plant {
        label: "assume keeps the first island only",
        root: 1,
        site: "islands := (cons (tokenOr tokens (position + 1)) (project state islands))",
        replacement: "islands := (project state islands)",
        vector: "assume two",
    },
    Plant {
        label: "the islands of assume in reverse",
        root: 1,
        site: "(ctor SentenceKind.Assume (reverseTokens (project final islands)))",
        replacement: "(ctor SentenceKind.Assume (project final islands))",
        vector: "assume three",
    },
    Plant {
        label: "apply takes what follows",
        root: 1,
        site: "(if (both (islandIs tokens 1) (count =? 2)) then",
        replacement: "(if (islandIs tokens 1) then",
        vector: "apply with a word after",
    },
    Plant {
        label: "close with takes what follows",
        root: 1,
        site: "(if (both (islandIs tokens 4) (count =? 5)) then (accepted (ctor SentenceKind.CloseWith",
        replacement: "(if (islandIs tokens 4) then (accepted (ctor SentenceKind.CloseWith",
        vector: "close with a word after",
    },
    Plant {
        label: "close needs no goal",
        root: 1,
        site: "(if (both (wordIs atoms tokens 1 (lit \"the\")) (wordIs atoms tokens 2 (lit \"goal\"))) then (let opening",
        replacement: "(if (wordIs atoms tokens 1 (lit \"the\")) then (let opening",
        vector: "close without the goal",
    },
    Plant {
        label: "the witness word is not read",
        root: 1,
        site: "(both (wordIs atoms tokens 4 (lit \"witness\")) (count =? 5))",
        replacement: "(count =? 5)",
        vector: "use as the something",
    },
    Plant {
        label: "select right is select left",
        root: 1,
        site: "(if left then (ctor SentenceKind.SelectLeft) else (ctor SentenceKind.SelectRight))",
        replacement: "(ctor SentenceKind.SelectLeft)",
        vector: "select right",
    },
    Plant {
        label: "the keyword of apply",
        root: 1,
        site: "(lit \"apply-verb\")",
        replacement: "(lit \"apply\")",
        vector: "apply",
    },
    Plant {
        label: "simp is not forbidden",
        root: 1,
        site: "(lit \"simp\")",
        replacement: "(lit \"simq\")",
        vector: "the forbidden word simp",
    },
    Plant {
        label: "atoms that do not touch compose",
        root: 1,
        site: "(both ((project atom byteStart) =? (project state reach))",
        replacement: "(both true",
        vector: "a tactic split by a dropped space",
    },
    Plant {
        label: "no atoms compose",
        root: 1,
        site: "(project (first (prim iterate_until (fun [atoms] (state : Compose) => (composeStep atoms state)) ((prim length atoms : Nat) + 1)",
        replacement: "(project (first (prim iterate_until (fun [atoms] (state : Compose) => (composeStep atoms state)) 0",
        vector: "a composed tactic",
    },
    Plant {
        label: "a comma joins a composition",
        root: 1,
        site: "(lit \",\") (cons (lit \".\")",
        replacement: "(lit \"!\") (cons (lit \".\")",
        vector: "a tactic broken by a comma",
    },
    Plant {
        label: "an underscore anywhere is a tactic",
        root: 1,
        site: "(both (position =? 0) (match (LexLeanStd.Text::find composed (lit \"_\"))",
        replacement: "(both true (match (LexLeanStd.Text::find composed (lit \"_\"))",
        vector: "an underscore word later",
    },
    Plant {
        label: "an empty sentence has a place",
        root: 1,
        site: "columnEnd := 1})), help := (ctor Option.none [Bytes])}",
        replacement: "columnEnd := 2})), help := (ctor Option.none [Bytes])}",
        vector: "an empty range",
    },
    Plant {
        label: "the refusal is at the last token",
        root: 1,
        site: "(refusal path (lit \"LLF5005\") message (atomOr atoms (firstAtom token)))",
        replacement: "(refusal path (lit \"LLF5005\") message (atomOr atoms ((firstAtom token) + 1)))",
        vector: "a lower-case verb",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let (tokens, sentence, _) = vector_runs(&vector.source, vector.start, vector.gapped);
        let run = if plant.root == 0 {
            tokens
        } else {
            sentence.unwrap_or_else(|| panic!("{}: the vector has tokens", plant.label))
        };
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
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, &[run], 1).first() {
            Some(first) => eprintln!(
                "BS-35: plant detected: {}: {}",
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
            "BS-35: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
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
        checked_units.units.contains_key("GrammarProof"),
        "GrammarProof is a linked module of the project"
    );
    eligibility_of(&project, "GrammarProof", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut messages = Vec::new();
    let mut named_runs: [Vec<Run>; 2] = [Vec::new(), Vec::new()];
    let mut kinds = BTreeSet::new();
    for vector in &vectors {
        let (tokens, sentence, refusals) = vector_runs(&vector.source, vector.start, vector.gapped);
        named_runs[0].push(tokens);
        named_runs[1].extend(sentence);
        messages.extend(refusals);
        let atoms: Vec<Atom> = scanned(&vector.source)
            .into_iter()
            .filter(|atom| !vector.gapped || atom.class != AtomClass::Whitespace)
            .collect();
        if let Ok(tokens) = text_tokens(PATH, &atoms, vector.start, atoms.len()) {
            let visible = BTreeSet::new();
            let parser = TextParser {
                path: PATH,
                atoms: &atoms,
                tokens: &tokens,
                closure: closure(),
                visible: &visible,
                sentence_initial: true,
            };
            if let Ok(sentence) = parse_proof_sentence(&parser) {
                kinds.insert(format!("{:?}", std::mem::discriminant(&sentence.kind)));
            }
        }
    }
    assert_eq!(kinds.len(), 7, "a vector reads as each registered sentence");
    for refusal in REFUSALS {
        assert!(
            messages.iter().any(|message| message.contains(refusal)),
            "no vector reaches the refusal `{refusal}`"
        );
    }
    for word in FORBIDDEN {
        assert!(
            messages
                .iter()
                .any(|message| message.starts_with(&format!("`{word}` is a forbidden"))),
            "no vector names the forbidden word `{word}`"
        );
    }
    let bases: Vec<String> = vectors
        .iter()
        .filter(|vector| {
            matches!(
                vector.label.as_str(),
                "assume two"
                    | "apply"
                    | "close with"
                    | "close by reflexivity"
                    | "use as the witness"
                    | "select left"
                    | "select right"
            )
        })
        .map(|vector| vector.source.clone())
        .collect();
    assert_eq!(bases.len(), 7, "a base of each registered sentence");

    let (source_count, evaluator_ranges, rustc_ranges) = source_runs();
    assert!(
        source_count >= 100,
        "the committed sources are {source_count}"
    );
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, &bases, 0x0B55_2035),
            seeded_runs(RUSTC_SEEDED, &bases, 0x0B55_3035),
        )
    } else {
        ((Vec::new(), Vec::new()), (Vec::new(), Vec::new()))
    };
    eprintln!(
        "BS-35: {} vectors ({} tokenized, {} read as sentences), {} committed sources, {} and {} ranges, {} and {} seeded sentences ({:?})",
        vectors.len(),
        named_runs[0].len(),
        named_runs[1].len(),
        source_count,
        evaluator_ranges.0.len(),
        rustc_ranges.0.len(),
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
        for (index, (root, _)) in ROOTS.iter().enumerate() {
            let mut corpus: Vec<Run> = named_runs[index].clone();
            if index == 0 {
                corpus.extend(evaluator_ranges.0.iter().cloned());
                corpus.extend(evaluator_seeded.0.iter().cloned());
            } else {
                corpus.extend(evaluator_ranges.1.iter().cloned());
                corpus.extend(evaluator_seeded.1.iter().cloned());
            }
            let disagreements = super::evaluate(&lowered[root], &corpus, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-35: {root}: the evaluator agrees on {} runs ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-35: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: [Vec<Run>; 2] = [named_runs[0].clone(), named_runs[1].clone()];
    compiled[0].extend(rustc_ranges.0);
    compiled[1].extend(rustc_ranges.1);
    compiled[0].extend(rustc_seeded.0);
    compiled[1].extend(rustc_seeded.1);
    let total = rustc_differential(
        "BS-35",
        &[
            (ROOTS[0].0, &lowered[ROOTS[0].0], compiled[0].as_slice()),
            (ROOTS[1].0, &lowered[ROOTS[1].0], compiled[1].as_slice()),
        ],
        "oracle",
    );
    eprintln!("BS-35: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-35") {
        return;
    }
    export(&lowered);
    eprintln!("BS-35: passed in {:?}", started.elapsed());
}
