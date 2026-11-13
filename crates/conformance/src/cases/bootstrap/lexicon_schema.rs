//! `BS-50` (SPEC.md §17.23, §13.1 to §13.6): the tables and predicates of the
//! lexicon entry schema as language-1.2 source, `LexLeanBoot.LexiconSchema`,
//! held to the byte to `lexlean::lexicon::entry` (the category, frame,
//! channel, associativity and feature tokens, `frame_permitted`,
//! `Frame::fixed_arity`, `Category::core_only`, `has_signature`,
//! `required_channels`, `needs_some_canonical`, `is_lean_name`,
//! `surface_safety` with `is_math_safe_scalar`) and to
//! `lexlean::lexicon::package::{toml_comment_at, entry_path}`.
//!
//! `surfaceSafety` reads the atoms of the primitive scanner of `BS-03`, so a
//! form's surface is scanned by the oracle's scanner and handed over as the
//! atoms `scanSource` returns. Each planted defect names the one vector that
//! catches it.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::Value;
use lexlean::lexicon::entry::{
    frame_permitted, is_lean_name, surface_safety, Associativity, Category, Channel, Frame,
    FEATURES,
};
use lexlean::lexicon::package::{entry_path, toml_comment_at};
use lexlean::source::atom::Atom;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::scan::atom_value;
use super::{list, option, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.LexiconSchema";
const ROOTS: [(&str, &[&str]); 6] = [
    ("tokenIndex", &["allocation", "overflow"]),
    ("shapeFacts", &["allocation", "overflow"]),
    ("surfaceSafety", &["allocation", "overflow"]),
    ("isLeanName", &["allocation", "overflow"]),
    ("tomlCommentAt", &["allocation", "overflow"]),
    ("entryPath", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded runs per root under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 300;
const RUSTC_SEEDED: usize = 2_000;

/// The schema tokens in the order of the oracle's enums.
const CATEGORIES: [&str; 17] = [
    "structural",
    "grammar",
    "label-word",
    "type-noun",
    "term-constant",
    "function",
    "prefix-function",
    "postfix-function",
    "infix-function",
    "noun-function",
    "binary-noun-function",
    "predicate-constant",
    "adjective-predicate",
    "intransitive-predicate",
    "transitive-predicate",
    "infix-predicate",
    "proof-constant",
];
const FRAMES: [&str; 10] = [
    "atom",
    "call",
    "prefix",
    "postfix",
    "infix",
    "noun-of",
    "binary-noun-of",
    "adjective",
    "intransitive",
    "transitive",
];
const CHANNELS: [&str; 3] = ["text", "math", "both"];
const ASSOCIATIVITIES: [&str; 3] = ["left", "right", "none"];

// ------------------------------------------------------------- the encoders

fn nat(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

fn boolean(value: bool) -> Value {
    Value::Bool { value }
}

fn index_of(table: &[&str], token: &str) -> u64 {
    u64::try_from(
        table
            .iter()
            .position(|row| *row == token)
            .unwrap_or_else(|| panic!("`{token}` is in its table")),
    )
    .expect("an index")
}

fn channel_of(index: u64) -> Channel {
    Channel::parse(CHANNELS[usize::try_from(index).expect("an index")]).expect("a channel")
}

fn channel_index(channel: Channel) -> u64 {
    match channel {
        Channel::Text => 0,
        Channel::Math => 1,
        Channel::Both => 2,
    }
}

/// What the oracle reads a token of table `which` as: its position in the
/// enum, or nothing.
fn token_value(which: u64, token: &str) -> Value {
    let found = match which {
        0 => Category::parse(token).map(|category| index_of(&CATEGORIES, category.as_str())),
        1 => Frame::parse(token).map(|frame| index_of(&FRAMES, frame.as_str())),
        2 => Channel::parse(token).map(channel_index),
        3 => Associativity::parse(token).map(|associativity| match associativity {
            Associativity::Left => 0,
            Associativity::Right => 1,
            Associativity::None => 2,
        }),
        _ => FEATURES
            .iter()
            .position(|feature| *feature == token)
            .map(|found| u64::try_from(found).expect("an index")),
    };
    option(found.map(nat))
}

fn token_run(which: u64, token: &str) -> Run {
    (
        vec![nat(which), text(token)],
        serde_json::to_value(token_value(which, token)).expect("a value is JSON"),
    )
}

/// What the oracle says of a category, a frame, and a surface arity, as the
/// module's `Shape`.
fn shape_run(category: u64, frame: u64, arity: u32) -> Run {
    let category_enum =
        Category::parse(CATEGORIES[usize::try_from(category).expect("an index")]).expect("a category");
    let frame_enum = Frame::parse(FRAMES[usize::try_from(frame).expect("an index")]).expect("a frame");
    let shape = Value::Adt {
        constructor: 0,
        fields: vec![
            boolean(frame_permitted(category_enum, frame_enum, arity)),
            option(frame_enum.fixed_arity().map(|fixed| nat(u64::from(fixed)))),
            boolean(category_enum.core_only()),
            boolean(category_enum.has_signature()),
            list(
                category_enum
                    .required_channels()
                    .iter()
                    .map(|channel| nat(channel_index(*channel)))
                    .collect(),
            ),
            boolean(category_enum.needs_some_canonical()),
        ],
    };
    (
        vec![nat(category), nat(frame), nat(u64::from(arity))],
        serde_json::to_value(shape).expect("a value is JSON"),
    )
}

/// The atoms of a surface, as the oracle's scanner reads it when a package
/// loads (`entry::parse_entry`).
fn atoms_of(surface: &str) -> Option<Vec<Atom>> {
    let scalars = u64::try_from(surface.chars().count()).expect("a count");
    lexlean::source::scan::scan("<form>", surface, scalars).ok()
}

fn safety_run(atoms: &[Atom], channel: u64) -> Run {
    let outcome = surface_safety(atoms, channel_of(channel)).err();
    (
        vec![list(atoms.iter().map(atom_value).collect()), nat(channel)],
        serde_json::to_value(option(outcome.as_deref().map(text))).expect("a value is JSON"),
    )
}

fn lean_run(name: &str) -> Run {
    (
        vec![text(name)],
        serde_json::to_value(boolean(is_lean_name(name))).expect("a value is JSON"),
    )
}

fn comment_run(source: &str) -> Run {
    let found = toml_comment_at(source).map(|at| nat(u64::try_from(at).expect("an offset")));
    (
        vec![text(source)],
        serde_json::to_value(option(found)).expect("a value is JSON"),
    )
}

fn path_run(local: &str) -> Run {
    (
        vec![text(local)],
        serde_json::to_value(text(&entry_path(local))).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ vectors

struct Vector {
    label: String,
    root: usize,
    run: Run,
}

fn scanned(surface: &str) -> Vec<Atom> {
    atoms_of(surface).unwrap_or_else(|| panic!("`{surface}` scans"))
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out: Vec<Vector> = Vec::new();
    let mut add = |label: String, root: usize, run: Run| {
        out.push(Vector { label, root, run });
    };

    // tokenIndex: every token of every table, and near misses.
    let tables: [(u64, Vec<&str>); 5] = [
        (0, CATEGORIES.to_vec()),
        (1, FRAMES.to_vec()),
        (2, CHANNELS.to_vec()),
        (3, ASSOCIATIVITIES.to_vec()),
        (4, FEATURES.to_vec()),
    ];
    for (which, table) in &tables {
        for token in table {
            add(format!("the token `{token}` of table {which}"), 0, token_run(*which, token));
        }
        for miss in ["", "Structural", "atom ", "noun_of", "both-", "article"] {
            add(format!("the near miss `{miss}` in table {which}"), 0, token_run(*which, miss));
        }
    }
    add("a frame is not a category".to_owned(), 0, token_run(0, "infix"));
    add("a category is not a frame".to_owned(), 0, token_run(1, "function"));

    // shapeFacts: every category with every frame, at arities 0, 1, 2, 3.
    for category in 0..17u64 {
        for frame in 0..10u64 {
            for arity in 0..4u32 {
                add(
                    format!(
                        "the shape of {} as {} at arity {arity}",
                        CATEGORIES[usize::try_from(category).expect("an index")],
                        FRAMES[usize::try_from(frame).expect("an index")]
                    ),
                    1,
                    shape_run(category, frame, arity),
                );
            }
        }
    }
    add(
        "the shape at the largest arity".to_owned(),
        1,
        shape_run(16, 1, u32::MAX),
    );

    // surfaceSafety.
    for (label, surface, channel) in [
        ("a text word", "natural number", 0),
        ("a text surface with every safe punctuation", "a: b-c, d. e; f'", 0),
        ("a text surface with a colon", "is: a", 0),
        ("a text surface with a parenthesis", "f(x)", 0),
        ("a text surface with a numeral", "x 2", 0),
        ("a text surface with a plus", "a + b", 0),
        ("a text surface with a Unicode symbol", "\u{2200}", 0),
        ("a text surface with a control", "\\N", 0),
        ("a math plus", "+", 1),
        ("a math surface with an underscore", "a_b", 1),
        ("a math surface with every unsafe symbol", "\\%$#&~^_{}\"`", 1),
        ("a math surface with brackets", "[a](b)", 1),
        ("a math surface with a brace", "{", 1),
        ("a math surface led by a numeral", "2x", 1),
        ("a math surface with an inner numeral", "x2", 1),
        ("a math surface with alpha", "\u{3b1}", 1),
        ("a math surface with capital omega", "\u{3a9}", 1),
        ("a math surface with final sigma", "\u{3c2}", 1),
        ("a math surface with not", "\u{ac}", 1),
        ("a math surface with times", "\u{d7}", 1),
        ("a math surface with division", "\u{f7}", 1),
        ("a math surface with a middle dot", "\u{b7}", 1),
        ("a math surface with plus-minus", "\u{b1}", 1),
        ("a math surface with an ellipsis", "\u{2026}", 1),
        ("a math surface with a prime", "\u{2032}", 1),
        ("a math surface with a triple prime", "\u{2034}", 1),
        ("a math surface with a quadruple prime", "\u{2057}", 1),
        ("a math surface with the naturals", "\u{2115}", 1),
        ("a math surface with an arrow", "\u{2192}", 1),
        ("a math surface with for all", "\u{2200}", 1),
        ("a math surface with the last operator", "\u{22ff}", 1),
        ("a math surface with a ceiling", "\u{2308}", 1),
        ("a math surface with a floor end", "\u{230b}", 1),
        ("a math surface with an angle bracket", "\u{27e8}", 1),
        ("a math surface with a white bracket", "\u{2983}", 1),
        ("a math surface with a circled plus operator", "\u{2a01}", 1),
        ("a math surface of a double-struck letter", "\u{1d538}", 1),
        ("a math surface with the last alphanumeric", "\u{1d7ff}", 1),
        ("a math surface with e acute", "\u{e9}", 1),
        ("a math surface with a euro sign", "\u{20ac}", 1),
        ("a math surface with an emoji", "\u{1f600}", 1),
        ("a math surface with a control", "\\alpha", 1),
        ("a both surface with a plus", "a + b", 2),
        ("a both surface of words", "the sum", 2),
        ("a both surface with a colon", "a:b", 2),
        ("a both surface with a Unicode symbol", "\u{2264}", 2),
        ("two unsafe atoms", "a % b $ c", 1),
        ("no atoms", "", 1),
    ] {
        let atoms = if surface.is_empty() {
            Vec::new()
        } else {
            scanned(surface)
        };
        add(label.to_owned(), 2, safety_run(&atoms, channel));
    }

    // isLeanName.
    for (label, name) in [
        ("a Lean name", "Nat.add"),
        ("a Lean name of one segment", "Nat"),
        ("a Lean name led by an underscore", "_private.x"),
        ("a Lean name with digits", "Nat.add_zero1"),
        ("the empty name", ""),
        ("a name of one dot", "."),
        ("a name with two dots", "Nat..add"),
        ("a name with a trailing dot", "Nat."),
        ("a name with a leading dot", ".Nat"),
        ("a name whose segment starts with a digit", "Nat.1add"),
        ("a name with a hyphen", "Nat.add-zero"),
        ("a name with a space", "Nat add"),
        ("a name with a non-ASCII letter", "Nat.\u{3b1}"),
        ("a name with a prime", "Nat.add'"),
    ] {
        add(label.to_owned(), 3, lean_run(name));
    }

    // tomlCommentAt.
    for (label, source) in [
        ("a comment line", "# c"),
        ("no comment", "a = 1\n"),
        ("a comment after a value", "a = 1 # c\n"),
        ("a hash in a basic string", "a = \"#\"\n"),
        ("an escaped quote before a hash", "a = \"x\\\"#\""),
        ("an escaped backslash before a hash", "a = \"x\\\\\"#"),
        ("a hash in a literal string", "a = '#'"),
        ("a literal string broken by a line", "a = 'x\n# c"),
        ("a basic string broken by a line", "a = \"x\n# c"),
        ("a hash in a multi-line string", "a = \"\"\"\n#\n\"\"\""),
        ("a quote inside a multi-line string", "a = \"\"\"x\"#\"\"\""),
        ("an escape inside a multi-line string", "a = \"\"\"\\\"\"\"#\"\"\""),
        ("a hash after a multi-line string", "a = \"\"\"x\"\"\" # c"),
        ("a hash in a multi-line literal string", "a = '''\n#'''"),
        ("a hash after a multi-line literal string", "a = '''x''' #"),
        ("an unclosed basic string", "a = \"#"),
        ("a backslash at the end", "a = \"\\"),
        ("a non-ASCII byte before a hash", "a = \"\u{e9}\" #"),
        ("an empty text", ""),
    ] {
        add(label.to_owned(), 4, comment_run(source));
    }

    // entryPath.
    for (label, local) in [
        ("a nested entry", "nat.add.zero"),
        ("an entry of one segment", "zero"),
        ("an entry with hyphens", "nat.add-comm"),
        ("the empty entry", ""),
        ("an entry of empty segments", ".."),
    ] {
        add(label.to_owned(), 5, path_run(local));
    }
    out
}

// ------------------------------------------------------------- the corpora

/// Every committed lexicon file and its parsed TOML, by repository path.
fn committed_files() -> Vec<(String, String)> {
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
        let shown = path.to_string_lossy().replace('\\', "/");
        if !entry.file_type().is_file() || !shown.ends_with(".toml") {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };
        let relative = path
            .strip_prefix(root.as_std_path())
            .expect("under the root")
            .to_string_lossy()
            .replace('\\', "/");
        out.push((relative, content));
    }
    out
}

fn is_entry_file(path: &str) -> bool {
    path.contains("/entries/")
}

fn is_lexicon_file(path: &str) -> bool {
    is_entry_file(path) || path.ends_with("/lexicon.toml")
}

/// The committed corpora: every form surface with its channel, every Lean
/// name of a denotation or an eliminator, every entry ID, and every TOML text.
struct Committed {
    surfaces: Vec<(String, u64)>,
    names: Vec<String>,
    ids: Vec<String>,
    lexicon_texts: Vec<String>,
    other_texts: Vec<String>,
}

fn committed() -> Committed {
    let mut surfaces = Vec::new();
    let mut names = Vec::new();
    let mut ids = Vec::new();
    let mut lexicon_texts = Vec::new();
    let mut other_texts = Vec::new();
    for (path, content) in committed_files() {
        if is_lexicon_file(&path) {
            lexicon_texts.push(content.clone());
        } else {
            other_texts.push(content.clone());
        }
        if !is_entry_file(&path) {
            continue;
        }
        let Ok(value) = content.parse::<toml::Table>() else {
            continue;
        };
        if let Some(local) = value.get("id").and_then(toml::Value::as_str) {
            ids.push(local.to_owned());
        }
        if let Some(denotation) = value.get("denotation").and_then(toml::Value::as_table) {
            for key in ["module", "name"] {
                if let Some(name) = denotation.get(key).and_then(toml::Value::as_str) {
                    names.push(name.to_owned());
                }
            }
        }
        if let Some(eliminator) = value.get("eliminator").and_then(toml::Value::as_table) {
            for key in ["cases_lean_name", "induction_lean_name"] {
                if let Some(name) = eliminator.get(key).and_then(toml::Value::as_str) {
                    names.push(name.to_owned());
                }
            }
            for constructor in eliminator
                .get("constructor")
                .and_then(toml::Value::as_array)
                .into_iter()
                .flatten()
            {
                if let Some(name) = constructor.get("lean_name").and_then(toml::Value::as_str) {
                    names.push(name.to_owned());
                }
            }
        }
        for form in value
            .get("form")
            .and_then(toml::Value::as_array)
            .into_iter()
            .flatten()
        {
            let (Some(surface), Some(channel)) = (
                form.get("surface").and_then(toml::Value::as_str),
                form.get("channel").and_then(toml::Value::as_str),
            ) else {
                continue;
            };
            if let Some(found) = CHANNELS.iter().position(|row| *row == channel) {
                surfaces.push((surface.to_owned(), u64::try_from(found).expect("an index")));
            }
        }
    }
    Committed {
        surfaces,
        names,
        ids,
        lexicon_texts,
        other_texts,
    }
}

// ------------------------------------------------------------------- seeded

const SURFACE_PIECES: &[&str] = &[
    "a", "nat", "x", "the", "of", "1", "20", " ", " ", ":", "-", ",", ".", ";", "'", "+", "=",
    "<", "*", "/", "|", "!", "?", "@", "%", "$", "#", "&", "~", "^", "_", "\"", "`", "(", ")",
    "[", "]", "{", "}", "\\N", "\\alpha", "\u{2200}", "\u{3b1}", "\u{3a9}", "\u{2115}",
    "\u{2192}", "\u{2264}", "\u{2282}", "\u{2308}", "\u{27e8}", "\u{2983}", "\u{2a01}",
    "\u{1d538}", "\u{e9}", "\u{20ac}", "\u{2026}", "\u{2032}", "\u{ac}", "\u{d7}",
];
const NAME_PIECES: &[&str] = &[
    "Nat", "add", "_", "1", ".", "x9", "-", "\u{3b1}", "'", "Zero", "", " ",
];
const TOML_PIECES: &[&str] = &[
    "\"", "'", "\"\"\"", "'''", "#", "\\", "\n", "a = ", "x", " ", "\u{e9}", "[t]",
];
const ID_PIECES: &[&str] = &["a", "b1", ".", "-", "nat", "add", "", "zero"];
const TOKEN_PIECES: &[&str] = &[
    "infix", "function", "-", "noun", "of", "both", "text", "article", "a", "n", "left",
    "plural", "case", "Atom", " ",
];

fn joined(seeded: &mut Seeded, pieces: &[&str], most: usize) -> String {
    (0..seeded.below(most + 1))
        .map(|_| *seeded.pick(pieces))
        .collect()
}

/// `count` seeded runs of each root that reads text: surfaces in a channel,
/// Lean names, TOML texts, entry IDs, and tokens.
fn seeded_runs(count: usize) -> [Vec<Run>; 6] {
    let mut seeded = Seeded(0x0B55_0050);
    let mut runs: [Vec<Run>; 6] = Default::default();
    while runs[2].len() < count {
        let surface = joined(&mut seeded, SURFACE_PIECES, 6);
        if let Some(atoms) = atoms_of(&surface) {
            let channel = u64::try_from(seeded.below(3)).expect("a channel");
            runs[2].push(safety_run(&atoms, channel));
        }
    }
    for _ in 0..count {
        let which = u64::try_from(seeded.below(5)).expect("a table");
        runs[0].push(token_run(which, &joined(&mut seeded, TOKEN_PIECES, 3)));
        runs[3].push(lean_run(&joined(&mut seeded, NAME_PIECES, 6)));
        runs[4].push(comment_run(&joined(&mut seeded, TOML_PIECES, 10)));
        runs[5].push(path_run(&joined(&mut seeded, ID_PIECES, 5)));
    }
    runs
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
        label: "a category is read as a frame",
        root: 0,
        site: "(if (which =? 0) then (categories) else",
        replacement: "(if (which =? 0) then (frames) else",
        vector: "the token `structural` of table 0",
    },
    Plant {
        label: "the last feature is lost",
        root: 0,
        site: "(cons (lit \"article-a\") (cons (lit \"article-an\") (nil Bytes)))",
        replacement: "(cons (lit \"article-a\") (nil Bytes))",
        vector: "the token `article-an` of table 4",
    },
    Plant {
        label: "the index counts from one",
        root: 0,
        site: "(0, (ctor Option.none [Nat])) table : (Nat * (Option Nat))",
        replacement: "(1, (ctor Option.none [Nat])) table : (Nat * (Option Nat))",
        vector: "the token `left` of table 3",
    },
    Plant {
        label: "a proof constant of arity one is an atom",
        root: 1,
        site: "(if (arity =? 0) then 0 else 1)",
        replacement: "(if (arity =? 0) then 0 else 0)",
        vector: "the shape of proof-constant as call at arity 1",
    },
    Plant {
        label: "a prefix frame fixes two",
        root: 1,
        site: "(if (frame =? 4) then true else (if (frame =? 6) then true else (frame =? 9)))",
        replacement: "(if (frame =? 4) then true else (if (frame =? 6) then true else (if (frame =? 9) then true else (frame =? 2))))",
        vector: "the shape of prefix-function as prefix at arity 1",
    },
    Plant {
        label: "a label word has a signature",
        root: 1,
        site: "hasSignature := (2 <? category)",
        replacement: "hasSignature := (1 <? category)",
        vector: "the shape of label-word as atom at arity 0",
    },
    Plant {
        label: "a term constant needs no canonical form",
        root: 1,
        site: "needsSome := (if (category =? 3) then true else (category =? 4))",
        replacement: "needsSome := (category =? 3)",
        vector: "the shape of term-constant as atom at arity 0",
    },
    Plant {
        label: "an infix predicate needs a text form",
        root: 1,
        site: "(cons 1 (cons 2 (cons 2 (nil Nat))))",
        replacement: "(cons 1 (cons 1 (cons 2 (nil Nat))))",
        vector: "the shape of infix-predicate as infix at arity 2",
    },
    Plant {
        label: "a colon is not text-safe",
        root: 2,
        site: "(cons (lit \":\") (cons (lit \"-\") (cons (lit \",\") (cons (lit \".\") (cons (lit \";\") (cons (lit \"'\") (nil Bytes)))))))",
        replacement: "(cons (lit \"-\") (cons (lit \",\") (cons (lit \".\") (cons (lit \";\") (cons (lit \"'\") (nil Bytes))))))",
        vector: "a text surface with a colon",
    },
    Plant {
        label: "an underscore is math-safe",
        root: 2,
        site: "(cons 0x5f (cons",
        replacement: "(cons 0x7b (cons",
        vector: "a math surface with an underscore",
    },
    Plant {
        label: "a leading numeral is math-safe",
        root: 2,
        site: "=> (0 <? place)",
        replacement: "=> true",
        vector: "a math surface led by a numeral",
    },
    Plant {
        label: "the mathematical alphanumerics end early",
        root: 2,
        site: "(inRange code 119808 120831)",
        replacement: "(inRange code 119808 119999)",
        vector: "a math surface of a double-struck letter",
    },
    Plant {
        label: "a two-byte scalar keeps five bits too few",
        root: 2,
        site: "(prim multiply (prim remainder lead 32 0 : Nat) 64 : Nat)",
        replacement: "(prim multiply (prim remainder lead 32 0 : Nat) 32 : Nat)",
        vector: "a math surface with alpha",
    },
    Plant {
        label: "a delimiter is named an ASCII symbol",
        root: 2,
        site: "| PrimitiveScan::AtomClass.Delimiter => (lit \"delimiter\")",
        replacement: "| PrimitiveScan::AtomClass.Delimiter => (lit \"ascii-symbol\")",
        vector: "a text surface with a parenthesis",
    },
    Plant {
        label: "both channels are the math channel",
        root: 2,
        site: "(if (textOk atom) then (mathOk atom place) else false)",
        replacement: "(mathOk atom place)",
        vector: "a both surface with a plus",
    },
    Plant {
        label: "the last unsafe atom is reported",
        root: 2,
        site: "| Option.some reason => state",
        replacement: "| Option.some reason => (if (atomSafe atom (first state) channel) then (((first state) + 1), (second state)) else (((first state) + 1), (ctor Option.some [Bytes] (unsafeMessage atom channel))))",
        vector: "two unsafe atoms",
    },
    Plant {
        label: "a segment may start with a digit",
        root: 3,
        site: "(if (isNameStart value) then",
        replacement: "(if (isNameRest value) then",
        vector: "a name whose segment starts with a digit",
    },
    Plant {
        label: "an empty segment is a segment",
        root: 3,
        site: "(if (value =? 46) then (if (segAt st) then (ctor Option.none [(Nat * Bool)])",
        replacement: "(if (value =? 46) then (if false then (ctor Option.none [(Nat * Bool)])",
        vector: "a name with two dots",
    },
    Plant {
        label: "a trailing dot ends a name",
        root: 3,
        site: "(if (segAt final) then false else true)",
        replacement: "true",
        vector: "a name with a trailing dot",
    },
    Plant {
        label: "an escape does not skip the next byte",
        root: 4,
        site: "(if (value =? 92) then (cmMove st 2 1)",
        replacement: "(if (value =? 92) then (cmMove st 1 1)",
        vector: "an escaped quote before a hash",
    },
    Plant {
        label: "a literal string ends at a quote only",
        root: 4,
        site: "(if (if (value =? 39) then true else (value =? 10)) then (cmMove st 1 0) else (cmMove st 1 2))",
        replacement: "(if (value =? 39) then (cmMove st 1 0) else (cmMove st 1 2))",
        vector: "a literal string broken by a line",
    },
    Plant {
        label: "one quote ends a multi-line string",
        root: 4,
        site: "(if (LexLeanStd.Text::matchAt text 0x222222 position) then (cmMove st 3 0) else (if (value =? 92) then (cmMove st 2 3)",
        replacement: "(if (value =? 34) then (cmMove st 1 0) else (if (value =? 92) then (cmMove st 2 3)",
        vector: "a quote inside a multi-line string",
    },
    Plant {
        label: "the comment is found one byte late",
        root: 4,
        site: "found := (ctor Option.some [Nat] position)",
        replacement: "found := (ctor Option.some [Nat] (position + 1))",
        vector: "a comment line",
    },
    Plant {
        label: "dots stay dots",
        root: 5,
        site: "(LexLeanStd.Text::splitOn localId (46 : UInt8))",
        replacement: "(LexLeanStd.Text::splitOn localId (47 : UInt8))",
        vector: "a nested entry",
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
            vector.root, plant.root,
            "{}: the vector is a run of the root planted",
            plant.label
        );
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/LexiconSchema.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/LexiconSchema.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, std::slice::from_ref(&vector.run), 1).first() {
            Some(first) => eprintln!(
                "BS-50: plant detected: {}: {}",
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
            "BS-50: {}: largest certificate part {largest} bytes",
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
        checked_units.units.contains_key("LexiconSchema")
            && checked_units.units.contains_key("PrimitiveScan")
            && checked_units.units.contains_key("LexiconLse"),
        "LexiconSchema, PrimitiveScan and LexiconLse are linked modules of the project"
    );
    eligibility_of(&project, "LexiconSchema", MODULE, &ROOTS);
    for token in CATEGORIES {
        assert_eq!(
            Category::parse(token).map(Category::as_str),
            Some(token),
            "the category table is the oracle's"
        );
    }
    for token in FRAMES {
        assert_eq!(
            Frame::parse(token).map(Frame::as_str),
            Some(token),
            "the frame table is the oracle's"
        );
    }

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut by_root: [Vec<Run>; 6] = Default::default();
    for vector in &vectors {
        by_root[vector.root].push(vector.run.clone());
    }
    let safe = serde_json::to_value(Value::None).expect("a value is JSON");
    let unsafe_named = vectors
        .iter()
        .filter(|vector| vector.root == 2 && vector.run.1 != safe)
        .count();
    assert!(unsafe_named >= 15, "the named surfaces reach refusals");

    // The committed corpora.
    let corpus = committed();
    assert!(
        corpus.surfaces.len() >= 300,
        "the committed entries carry {} forms",
        corpus.surfaces.len()
    );
    let mut committed_runs: [Vec<Run>; 6] = Default::default();
    for (surface, channel) in &corpus.surfaces {
        let atoms = scanned(surface);
        committed_runs[2].push(safety_run(&atoms, *channel));
        for other in (0..3).filter(|other| other != channel) {
            committed_runs[2].push(safety_run(&atoms, other));
        }
    }
    for name in &corpus.names {
        committed_runs[3].push(lean_run(name));
    }
    for local in &corpus.ids {
        committed_runs[5].push(path_run(local));
    }
    for source in &corpus.lexicon_texts {
        committed_runs[4].push(comment_run(source));
    }
    let other_comment_runs: Vec<Run> = corpus.other_texts.iter().map(|source| comment_run(source)).collect();
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (seeded_runs(EVALUATOR_SEEDED), seeded_runs(RUSTC_SEEDED))
    } else {
        (Default::default(), Default::default())
    };
    eprintln!(
        "BS-50: {} vectors, {} committed forms, {} Lean names, {} entry IDs, {} lexicon files and {} other TOML files ({:?})",
        vectors.len(),
        corpus.surfaces.len(),
        corpus.names.len(),
        corpus.ids.len(),
        corpus.lexicon_texts.len(),
        other_comment_runs.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        for (index, (root, _)) in ROOTS.iter().enumerate() {
            let mut corpus_runs: Vec<Run> = by_root[index].clone();
            corpus_runs.extend(committed_runs[index].iter().cloned());
            corpus_runs.extend(evaluator_seeded[index].iter().cloned());
            let disagreements = super::evaluate(&lowered[root], &corpus_runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-50: {root}: the evaluator agrees on {} runs ({:?})",
                corpus_runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-50: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: [Vec<Run>; 6] = Default::default();
    for (index, runs) in compiled.iter_mut().enumerate() {
        runs.append(&mut by_root[index]);
        runs.append(&mut committed_runs[index]);
        runs.extend(rustc_seeded[index].iter().cloned());
    }
    compiled[4].extend(other_comment_runs);
    let suites: Vec<(&str, &super::Lowered, &[Run])> = ROOTS
        .iter()
        .enumerate()
        .map(|(index, (root, _))| (*root, &lowered[root], compiled[index].as_slice()))
        .collect();
    let total = rustc_differential("BS-50", &suites, "oracle");
    eprintln!("BS-50: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-50") {
        return;
    }
    export(&lowered);
    eprintln!("BS-50: passed in {:?}", started.elapsed());
}
