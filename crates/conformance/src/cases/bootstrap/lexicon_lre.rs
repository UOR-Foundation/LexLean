//! `BS-26` (SPEC.md §17.23, §13.9): the reader of LexLean Rendering Expressions
//! and the facts the entry validator reads from a render, as language-1.2
//! source, `LexLeanBoot.LexiconLre`, held to the byte to
//! `lexlean::lexicon::lre::parse` and the walkers of `Render`.
//!
//! `parseLre` tokenizes and reads a text with the reader of `BS-25` and
//! converts the tree with a work stack, so that it admits no `recursion`.
//! `renderFacts` walks a render once, pre-order over an explicit stack, and
//! returns what `slots`, `tokens`, `form_refs`, `self_form_refs`,
//! `operator_names`, `math_only_construct`, and `script_operand_error` return;
//! the host passes the number of nodes, which bounds the walk.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::lexicon::lre::{parse, Render};
use lexlean::lexicon::lse::{ParseError, QualifiedId};
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, option, pair, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.LexiconLre";
const SOURCE: &str = "bootstrap/src/LexiconLre.lex.tex";
const ROOTS: [(&str, &[&str]); 2] = [
    ("parseLre", &["allocation", "overflow"]),
    ("renderFacts", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The depth the committed renders and the seeded ones are read at.
const DEPTH: u64 = 64;
/// Seeded renders under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 300;
const RUSTC_SEEDED: usize = 1_500;

// ---------------------------------------------------------------- the sorts

/// The constructors of the module's `Render`, in declaration order, read from
/// its text.
fn constructors() -> &'static Vec<String> {
    static TABLE: OnceLock<Vec<String>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let source = std::fs::read_to_string(repo_root().join(SOURCE).as_std_path())
            .expect("the module's source");
        let block = source
            .split("inductive{")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("the first inductive");
        assert!(
            block.contains("name: Render,"),
            "the first inductive is Render"
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
    let found = constructors()
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`Render` has no constructor `{constructor}`"));
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

/// A list of renders as the module's chain of `RItem` cells.
fn chain(items: &[Render]) -> Value {
    items
        .iter()
        .rev()
        .fold(adt("RNil", Vec::new()), |rest, item| {
            adt("RItem", vec![enc(item), rest])
        })
}

/// A render as the module's `Render`.
fn enc(render: &Render) -> Value {
    match render {
        Render::Form { entry, form } => adt(
            "RForm",
            vec![text(&entry.package), text(&entry.entry), text(form)],
        ),
        Render::SelfForm(form) => adt("RSelfForm", vec![text(form)]),
        Render::Slot(index) => adt("RSlot", vec![nat_u64(u64::from(*index))]),
        Render::Seq(items) => adt("RSeq", vec![chain(items)]),
        Render::Space => adt("RSpace", Vec::new()),
        Render::Token(id) => adt("RToken", vec![text(id)]),
        Render::Group(inner) => adt("RGroup", vec![enc(inner)]),
        Render::Paren(inner) => adt("RParen", vec![enc(inner)]),
        Render::Bracket(inner) => adt("RBracket", vec![enc(inner)]),
        Render::Sub(a, b) => adt("RSub", vec![enc(a), enc(b)]),
        Render::Sup(a, b) => adt("RSup", vec![enc(a), enc(b)]),
        Render::Frac(a, b) => adt("RFrac", vec![enc(a), enc(b)]),
        Render::OperatorName(name) => adt("ROperatorName", vec![text(name)]),
    }
}

/// The number of nodes of a render, the bound the host passes to the walk.
fn nodes(render: &Render) -> usize {
    let mut count = 0;
    let mut stack = vec![render];
    while let Some(node) = stack.pop() {
        count += 1;
        match node {
            Render::Seq(items) => stack.extend(items.iter()),
            Render::Group(inner) | Render::Paren(inner) | Render::Bracket(inner) => {
                stack.push(inner);
            }
            Render::Sub(a, b) | Render::Sup(a, b) | Render::Frac(a, b) => {
                stack.push(a);
                stack.push(b);
            }
            _ => {}
        }
    }
    count
}

fn texts(items: &[String]) -> Value {
    list(items.iter().map(|item| text(item)).collect())
}

/// What the oracle's walkers say of a render, as the module's `Facts`.
fn facts(render: &Render) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            list(
                render
                    .slots()
                    .iter()
                    .map(|index| nat_u64(u64::from(*index)))
                    .collect(),
            ),
            texts(&render.tokens()),
            list(
                render
                    .form_refs()
                    .iter()
                    .map(|(entry, form): &(QualifiedId, String)| {
                        pair(text(&entry.package), pair(text(&entry.entry), text(form)))
                    })
                    .collect(),
            ),
            texts(&render.self_form_refs()),
            texts(&render.operator_names()),
            option(render.math_only_construct().map(text)),
            option(render.script_operand_error().as_deref().map(text)),
        ],
    }
}

/// The run of `parseLre` on a text at a depth, and the oracle's message when
/// it refuses.
fn parse_run(source: &str, depth: u64) -> (Run, Option<String>) {
    let (value, refusal) = match parse(source, depth) {
        Ok(render) => (
            Value::Ok {
                value: Box::new(enc(&render)),
            },
            None,
        ),
        Err(error) => {
            let kind = match error {
                ParseError::Syntax(_) => 0,
                ParseError::DepthExceeded(_) => 1,
            };
            let message = error.to_string();
            (
                Value::Error {
                    value: Box::new(Value::Adt {
                        constructor: 0,
                        fields: vec![nat_u64(kind), text(&message)],
                    }),
                },
                Some(message),
            )
        }
    };
    (
        (
            vec![text(source), nat_u64(depth)],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

/// The run of `renderFacts` on a render.
fn facts_run(render: &Render) -> Run {
    (
        vec![
            enc(render),
            nat_u64(u64::try_from(nodes(render)).expect("a count")),
        ],
        serde_json::to_value(facts(render)).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ vectors

enum Subject {
    Parse(String, u64),
    Facts(Render),
}

struct Vector {
    label: String,
    subject: Subject,
}

impl Vector {
    fn walking(&self) -> bool {
        matches!(self.subject, Subject::Facts(_))
    }

    fn run(&self) -> (Run, Option<String>) {
        match &self.subject {
            Subject::Parse(source, depth) => parse_run(source, *depth),
            Subject::Facts(render) => (facts_run(render), None),
        }
    }
}

fn parsed(source: &str) -> Render {
    parse(source, DEPTH).unwrap_or_else(|error| panic!("`{source}` parses: {error}"))
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
    let reading = |source: &str| Subject::Parse(source.to_owned(), DEPTH);

    // Reading: what is accepted.
    for (label, source) in [
        ("a form", "(form lexlean.core::the the-l)"),
        ("a form of a nested entry", "(form a.b::c.d e-f9)"),
        ("a self form", "(self-form x)"),
        ("a self form with digits and hyphens", "(self-form x0-y)"),
        ("a slot", "(slot 0)"),
        ("a slot of many digits", "(slot 4294967295)"),
        ("a space", "(space)"),
        ("a token", "(token plus)"),
        ("a token with digits and hyphens", "(token comma-1)"),
        ("a group", "(group (slot 0))"),
        ("a paren", "(paren (slot 0))"),
        ("a bracket", "(bracket (slot 0))"),
        ("a sub", "(sub (slot 0) (slot 1))"),
        ("a sup", "(sup (slot 0) (slot 1))"),
        ("a frac", "(frac (slot 0) (slot 1))"),
        ("an operator name", "(operator-name succ_1)"),
        ("an operator name of capitals", "(operator-name Succ)"),
        ("a seq of one", "(seq (slot 0))"),
        ("a seq", "(seq (slot 0) (space) (token plus) (space) (slot 1))"),
        (
            "every production",
            "(seq (form lexlean.core::the the-l) (space) (self-form x) (slot 0) (token plus) (group (slot 1)) (paren (slot 2)) (bracket (slot 3)) (sub (slot 4) (slot 5)) (sup (slot 6) (slot 7)) (frac (slot 8) (slot 9)) (operator-name succ_1))",
        ),
        ("white space of both kinds", "(seq\n(slot 0)  (space)\n)"),
        ("atoms touching parentheses", "(seq(slot 0)(space))"),
        (
            "the oracle's own render",
            "(seq (operator-name mulassoc) (paren (seq (slot 0) (token comma) (space) (slot 1) (token comma) (space) (slot 2))))",
        ),
    ] {
        add(label, reading(source));
    }

    // Reading: the limit on nesting.
    add(
        "nesting at the depth",
        Subject::Parse("(group (slot 0))".to_owned(), 2),
    );
    add(
        "nesting beyond the depth",
        Subject::Parse("(group (slot 0))".to_owned(), 1),
    );

    // Reading: what is refused.
    for (label, source) in [
        ("the empty text", ""),
        ("an unbalanced close", ")"),
        ("an unclosed open", "(seq (slot 0)"),
        ("trailing tokens", "(slot 0) (slot 1)"),
        ("a tab", "(slot\t0)"),
        ("a bare atom", "x"),
        ("an empty list", "()"),
        ("a list headed by a list", "((slot 0))"),
        ("an unknown form", "(raw \\relax)"),
        ("a form of one", "(form a::b)"),
        ("a form of three", "(form a::b c d)"),
        ("a form whose entry is a list", "(form (a::b) c)"),
        ("a form with a bad entry", "(form nat the-l)"),
        ("a form whose form is a list", "(form a::b (c))"),
        ("a form with a bad form ID", "(form a::b Bad)"),
        ("a self form of nothing", "(self-form)"),
        ("a self form that is a list", "(self-form (x))"),
        ("a self form with a bad ID", "(self-form Bad)"),
        ("a slot of nothing", "(slot)"),
        ("a slot that is a list", "(slot (0))"),
        ("a slot with a leading zero", "(slot 01)"),
        ("a slot with a sign", "(slot +1)"),
        ("a slot of a word", "(slot x)"),
        ("a slot beyond 32 bits", "(slot 4294967296)"),
        ("a slot beyond 64 bits", "(slot 18446744073709551616)"),
        ("a space with an operand", "(space x)"),
        ("a token of nothing", "(token)"),
        ("a token that is a list", "(token (plus))"),
        ("a token with a bad ID", "(token \\relax)"),
        ("a token of capitals", "(token Plus)"),
        ("an operator name of nothing", "(operator-name)"),
        ("an operator name that is a list", "(operator-name (x))"),
        ("an operator name with a hyphen", "(operator-name a-b)"),
        ("an operator name of a digit first", "(operator-name 1a)"),
        ("an empty seq", "(seq)"),
        ("a group of two", "(group (slot 0) (slot 1))"),
        ("a paren of nothing", "(paren)"),
        ("a bracket of two", "(bracket (slot 0) (slot 1))"),
        ("a sub of one", "(sub (slot 0))"),
        ("a sup of three", "(sup (slot 0) (slot 1) (slot 2))"),
        ("a frac of nothing", "(frac)"),
        ("a bad item inside a seq", "(seq (slot 0) x)"),
        (
            "the first operand's error comes before the second's",
            "(sub (foo) (bar))",
        ),
        ("a bad item inside a group", "(group (bar))"),
    ] {
        add(label, reading(source));
    }

    // The facts of renders.
    for (label, source) in [
        ("the facts of every production", "(seq (form lexlean.core::the the-l) (space) (self-form x) (slot 0) (token plus) (group (slot 1)) (paren (slot 2)) (bracket (slot 3)) (sub (slot 4) (slot 5)) (sup (slot 6) (slot 7)) (frac (slot 8) (slot 9)) (operator-name succ_1))"),
        ("the facts of a slot", "(slot 3)"),
        ("the facts of a space", "(space)"),
        ("slots in order", "(seq (slot 2) (group (seq (slot 0) (slot 1))) (slot 2))"),
        ("tokens and forms in order", "(seq (token b) (form p::e f) (self-form g) (token a) (form q::r s))"),
        ("a sub is math only", "(seq (slot 0) (sub (slot 1) (slot 2)))"),
        ("a frac before an operator name", "(seq (frac (slot 0) (slot 1)) (operator-name f))"),
        ("an operator name before a sup", "(seq (operator-name f) (sup (slot 0) (slot 1)))"),
        ("a sub whose script is a space", "(sub (slot 0) (space))"),
        ("a sup whose base renders nothing", "(sup (seq (space) (space)) (slot 0))"),
        ("a frac whose operands render", "(frac (seq (space) (slot 0)) (slot 1))"),
        ("a sup whose script is a sub", "(sup (slot 0) (sub (slot 1) (slot 2)))"),
        ("a sub whose base is a sup", "(sub (sup (slot 0) (slot 1)) (slot 2))"),
        ("a grouped nested script", "(sup (slot 0) (group (sub (slot 1) (slot 2))))"),
        ("a frac of scripts", "(frac (sub (slot 0) (slot 1)) (sup (slot 2) (slot 3)))"),
        ("the first problem of two", "(seq (sub (slot 0) (sub (slot 1) (slot 2))) (sup (slot 3) (space)))"),
        ("a sub whose operands fail differently", "(sub (space) (sub (slot 1) (slot 2)))"),
        ("a problem inside a group", "(group (seq (slot 0) (sub (space) (slot 1))))"),
        ("a nested seq of spaces", "(frac (seq (seq (space)) (space)) (slot 0))"),
    ] {
        add(label, Subject::Facts(parsed(source)));
    }
    out
}

// ------------------------------------------------------------------- seeded

const TOKENS: &[&str] = &["plus", "comma", "times-1", "eq"];
const FORMS: &[(&str, &str)] = &[("lexlean.core", "the"), ("p.q-r", "s.t-u")];
const FORM_IDS: &[&str] = &["the-l", "x", "f0"];
const OPERATORS: &[&str] = &["succ_1", "mulassoc", "Op"];

fn drawn(seeded: &mut Seeded, depth: usize) -> Render {
    let kind = if depth == 0 {
        seeded.below(6)
    } else {
        seeded.below(13)
    };
    match kind {
        0 => {
            let (package, entry) = *seeded.pick(FORMS);
            Render::Form {
                entry: QualifiedId::parse(&format!("{package}::{entry}")).expect("an ID"),
                form: (*seeded.pick(FORM_IDS)).to_owned(),
            }
        }
        1 => Render::SelfForm((*seeded.pick(FORM_IDS)).to_owned()),
        2 => Render::Slot(u32::try_from(seeded.below(4)).expect("a slot")),
        3 => Render::Space,
        4 => Render::Token((*seeded.pick(TOKENS)).to_owned()),
        5 => Render::OperatorName((*seeded.pick(OPERATORS)).to_owned()),
        6 | 7 => {
            let count = 1 + seeded.below(4);
            Render::Seq((0..count).map(|_| drawn(seeded, depth - 1)).collect())
        }
        8 => Render::Group(Box::new(drawn(seeded, depth - 1))),
        9 => Render::Paren(Box::new(drawn(seeded, depth - 1))),
        10 => Render::Bracket(Box::new(drawn(seeded, depth - 1))),
        11 => Render::Sub(
            Box::new(drawn(seeded, depth - 1)),
            Box::new(drawn(seeded, depth - 1)),
        ),
        _ => {
            if seeded.chance(50) {
                Render::Sup(
                    Box::new(drawn(seeded, depth - 1)),
                    Box::new(drawn(seeded, depth - 1)),
                )
            } else {
                Render::Frac(
                    Box::new(drawn(seeded, depth - 1)),
                    Box::new(drawn(seeded, depth - 1)),
                )
            }
        }
    }
}

/// The text of a render, as the entries write them.
fn written(render: &Render) -> String {
    match render {
        Render::Form { entry, form } => format!("(form {entry} {form})"),
        Render::SelfForm(form) => format!("(self-form {form})"),
        Render::Slot(index) => format!("(slot {index})"),
        Render::Seq(items) => format!(
            "(seq {})",
            items.iter().map(written).collect::<Vec<_>>().join(" ")
        ),
        Render::Space => "(space)".to_owned(),
        Render::Token(id) => format!("(token {id})"),
        Render::Group(inner) => format!("(group {})", written(inner)),
        Render::Paren(inner) => format!("(paren {})", written(inner)),
        Render::Bracket(inner) => format!("(bracket {})", written(inner)),
        Render::Sub(a, b) => format!("(sub {} {})", written(a), written(b)),
        Render::Sup(a, b) => format!("(sup {} {})", written(a), written(b)),
        Render::Frac(a, b) => format!("(frac {} {})", written(a), written(b)),
        Render::OperatorName(name) => format!("(operator-name {name})"),
    }
}

const PIECES: &[&str] = &[
    "(",
    ")",
    " ",
    "\n",
    "seq",
    "slot",
    "space",
    "token",
    "group",
    "sub",
    "frac",
    "form",
    "x",
    "0",
    "01",
    "::",
    "-",
    "(slot 0)",
    "(space)",
    "(sub (slot 0) (slot 1))",
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

/// `count` seeded renders: the runs of `renderFacts` on them and of `parseLre`
/// on their texts and on edits of those.
fn seeded_runs(count: usize, bases: &[String]) -> (Vec<Run>, Vec<Run>) {
    let mut seeded = Seeded(0x0B55_0026);
    let mut walking = Vec::with_capacity(count);
    let mut reading = Vec::with_capacity(2 * count);
    while walking.len() < count {
        let depth = 1 + seeded.below(4);
        let render = drawn(&mut seeded, depth);
        let source = written(&render);
        walking.push(facts_run(&render));
        reading.push(parse_run(&source, DEPTH).0);
        let base = if seeded.chance(30) && !bases.is_empty() {
            seeded.pick(bases).clone()
        } else {
            source
        };
        reading.push(parse_run(&edited(&mut seeded, &base), DEPTH).0);
    }
    (walking, reading)
}

/// Every render a committed lexicon entry carries: the `math` field of the
/// `[render]` table of every `entries/*.toml`, with its path.
fn committed() -> Vec<(String, String)> {
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
        if !entry.file_type().is_file() || !shown.ends_with(".toml") || !shown.contains("/entries/")
        {
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
        for line in content.lines() {
            for field in ["math = \"", "text = \""] {
                if let Some(rest) = line.strip_prefix(field) {
                    if let Some(inner) = rest.strip_suffix('"') {
                        if inner.starts_with('(') {
                            out.push((relative.clone(), inner.to_owned()));
                        }
                    }
                }
            }
        }
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
        label: "a form takes three",
        root: 0,
        site: "(if ((prim length arguments : Nat) =? 2) then (match (LexiconLse::atomOf (LexiconLse::nodeAt arguments 0) (LexiconLse::lit \"a qualified ID\") fuel)",
        replacement: "(if (2 <=? (prim length arguments : Nat)) then (match (LexiconLse::atomOf (LexiconLse::nodeAt arguments 0) (LexiconLse::lit \"a qualified ID\") fuel)",
        vector: "a form of three",
    },
    Plant {
        label: "a form ID need not be an ID",
        root: 0,
        site: "| Result.ok form => (if (isFormId form) then (rvPush st rest (ctor Render.RForm (first parts) (second parts) form))",
        replacement: "| Result.ok form => (if true then (rvPush st rest (ctor Render.RForm (first parts) (second parts) form))",
        vector: "a form with a bad form ID",
    },
    Plant {
        label: "a token ID admits a capital",
        root: 0,
        site: "(if (LexiconLse::isLowerNum (LexiconLse::byteNum text 0)) then ((first (prim iterate_until (fun [n, text] (position : Nat) => (scanTokenId text n position)) n 1 : (Nat * Bool))) =? n) else false)",
        replacement: "(if (LexiconLse::isAlphaNum (LexiconLse::byteNum text 0)) then ((first (prim iterate_until (fun [n, text] (position : Nat) => (scanTokenId text n position)) n 1 : (Nat * Bool))) =? n) else false)",
        vector: "a token of capitals",
    },
    Plant {
        label: "an operator name admits a hyphen",
        root: 0,
        site: "(if (if (LexiconLse::isAlnumNum value) then true else (value =? 95)) then",
        replacement: "(if (if (LexiconLse::isAlnumNum value) then true else (if (value =? 95) then true else (value =? 45))) then",
        vector: "an operator name with a hyphen",
    },
    Plant {
        label: "a slot index may exceed 32 bits",
        root: 0,
        site: "(if (4294967295 <? value) then",
        replacement: "(if (18446744073709551615 <? value) then",
        vector: "a slot beyond 32 bits",
    },
    Plant {
        label: "a slot index may have a leading zero",
        root: 0,
        site: "| Option.some fault => (rvFail st (LexiconLse::failed (LexiconLse::cat (LexiconLse::quoted digits) (LexiconLse::lit \" is not a slot index\"))))",
        replacement: "| Option.some fault => (rvPush st rest (ctor Render.RSlot (LexiconLse::decimalValue digits)))",
        vector: "a slot with a leading zero",
    },
    Plant {
        label: "a space takes an operand",
        root: 0,
        site: "(if (argc =? 0) then (rvPush st rest (ctor Render.RSpace))",
        replacement: "(if (argc <=? 1) then (rvPush st rest (ctor Render.RSpace))",
        vector: "a space with an operand",
    },
    Plant {
        label: "an empty seq is a seq",
        root: 0,
        site: "(if (argc =? 0) then (rvFail st (LexiconLse::failed (LexiconLse::lit \"`seq` has at least one item\")))",
        replacement: "(if false then (rvFail st (LexiconLse::failed (LexiconLse::lit \"`seq` has at least one item\")))",
        vector: "an empty seq",
    },
    Plant {
        label: "a group takes two",
        root: 0,
        site: "(if (argc =? 1) then (record Rv {work := (cons (ctor RWork.RVisit (LexiconLse::nodeAt arguments 0)) (cons (ctor RWork.RBuild head 1) rest))",
        replacement: "(if (1 <=? argc) then (record Rv {work := (cons (ctor RWork.RVisit (LexiconLse::nodeAt arguments 0)) (cons (ctor RWork.RBuild head 1) rest))",
        vector: "a group of two",
    },
    Plant {
        label: "a sub takes one",
        root: 0,
        site: "(if (argc =? 2) then (record Rv {work := (cons (ctor RWork.RVisit (LexiconLse::nodeAt arguments 0)) (cons (ctor RWork.RVisit (LexiconLse::nodeAt arguments 1))",
        replacement: "(if (1 <=? argc) then (record Rv {work := (cons (ctor RWork.RVisit (LexiconLse::nodeAt arguments 0)) (cons (ctor RWork.RVisit (LexiconLse::nodeAt arguments 1))",
        vector: "a sub of one",
    },
    Plant {
        label: "a sup is a sub",
        root: 0,
        site: "(if (prim equal head (LexiconLse::lit \"sub\") : Bool) then (ctor Render.RSub left right) else",
        replacement: "(if (isBinary head) then (ctor Render.RSub left right) else",
        vector: "a sup",
    },
    Plant {
        label: "a paren is a group",
        root: 0,
        site: "(if (prim equal head (LexiconLse::lit \"paren\") : Bool) then (ctor Render.RParen inner) else",
        replacement: "(if false then (ctor Render.RParen inner) else",
        vector: "a paren",
    },
    Plant {
        label: "the second operand is visited first",
        root: 0,
        site: "work := (cons (ctor RWork.RVisit (LexiconLse::nodeAt arguments 0)) (cons (ctor RWork.RVisit (LexiconLse::nodeAt arguments 1)) (cons (ctor RWork.RBuild head 2) rest))",
        replacement: "work := (cons (ctor RWork.RVisit (LexiconLse::nodeAt arguments 1)) (cons (ctor RWork.RVisit (LexiconLse::nodeAt arguments 0)) (cons (ctor RWork.RBuild head 2) rest))",
        vector: "the first operand's error comes before the second's",
    },
    Plant {
        label: "the slots come last first",
        root: 1,
        site: "slots := (LexLeanStd.Order::reverseList [Nat] (project facts slots))",
        replacement: "slots := (project facts slots)",
        vector: "slots in order",
    },
    Plant {
        label: "the tokens come last first",
        root: 1,
        site: "tokens := (LexLeanStd.Order::reverseList [Bytes] (project facts tokens))",
        replacement: "tokens := (project facts tokens)",
        vector: "tokens and forms in order",
    },
    Plant {
        label: "a frac is not math only",
        root: 1,
        site: "| Render.RFrac a b => (withFacts (cons a (cons b rest)) (binaryFacts facts (LexiconLse::lit \"frac\") a b fuel))",
        replacement: "| Render.RFrac a b => (withFacts (cons a (cons b rest)) (record Facts {slots := (project facts slots), tokens := (project facts tokens), formRefs := (project facts formRefs), selfForms := (project facts selfForms), operatorNames := (project facts operatorNames), mathOnly := (project facts mathOnly), scriptError := (firstProblem (project facts scriptError) (scriptProblem (LexiconLse::lit \"frac\") a b fuel))}))",
        vector: "a frac before an operator name",
    },
    Plant {
        label: "the last math construct wins",
        root: 1,
        site: "  body: (match current\n    | Option.some kept => current\n    | Option.none => (ctor Option.some [Bytes] candidate)),",
        replacement: "  body: (ctor Option.some [Bytes] candidate),",
        vector: "an operator name before a sup",
    },
    Plant {
        label: "a seq of spaces renders something",
        root: 1,
        site: "| Render.RSeq items => (ctor Option.some [((List Render) * Nat)] ((prim list_fold (fun (acc : (List Render)) (item : Render) => (cons item acc)) rest (renderItems items fuel) : (List Render)), 0))",
        replacement: "| Render.RSeq items => (ctor Option.some [((List Render) * Nat)] (rest, 1))",
        vector: "a sup whose base renders nothing",
    },
    Plant {
        label: "a frac's script may be a script",
        root: 1,
        site: "(if (if (prim equal name (LexiconLse::lit \"frac\") : Bool) then false else (isScript operand)) then",
        replacement: "(if false then",
        vector: "a sup whose script is a sub",
    },
    Plant {
        label: "the second operand is checked first",
        root: 1,
        site: "  body: (match (operandProblem name left fuel)\n    | Option.some problem => (ctor Option.some [Bytes] problem)\n    | Option.none => (operandProblem name right fuel)),",
        replacement: "  body: (match (operandProblem name right fuel)\n    | Option.some problem => (ctor Option.some [Bytes] problem)\n    | Option.none => (operandProblem name left fuel)),",
        vector: "a sub whose operands fail differently",
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
            vector.walking(),
            plant.root == 1,
            "{}: the vector is a run of the root planted",
            plant.label
        );
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/LexiconLre.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/LexiconLre.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, &[vector.run().0], 1).first() {
            Some(first) => eprintln!(
                "BS-26: plant detected: {}: {}",
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
            "BS-26: {}: largest certificate part {largest} bytes",
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

/// Every error `parse` can state, by a part of its message (R2).
const REFUSALS: &[&str] = &[
    "whitespace is one or more ASCII spaces or LF",
    "max_scope_depth exceeded: configured",
    "trailing tokens after the expression",
    "unbalanced `)`",
    "unexpected end of expression",
    "expected `(`, found",
    "expected an LRE form",
    "a form reference is `(form qualified-id form-id)`",
    "expected a qualified ID, found",
    "is not a qualified `package::entry` ID",
    "expected a form ID, found",
    "is not a form ID",
    "a self form is `(self-form form-id)`",
    "a slot is `(slot decimal)`",
    "expected a slot index, found",
    "is not a slot index",
    "`(space)` takes no operand",
    "a token is `(token renderer-token-id)`",
    "expected a renderer-token ID, found",
    "is not a renderer-token ID",
    "an operator name is `(operator-name identifier)`",
    "expected an operator name, found",
    "is not an operator name",
    "`seq` has at least one item",
    "takes exactly one operand",
    "takes exactly two operands",
    "is not an LRE form",
];

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("LexiconLre")
            && checked_units.units.contains_key("LexiconLse"),
        "LexiconLre and LexiconLse are linked modules of the project"
    );
    eligibility_of(&project, "LexiconLre", MODULE, &ROOTS);
    assert_eq!(
        constructors().as_slice(),
        [
            "RForm",
            "RSelfForm",
            "RSlot",
            "RSeq",
            "RSpace",
            "RToken",
            "RGroup",
            "RParen",
            "RBracket",
            "RSub",
            "RSup",
            "RFrac",
            "ROperatorName",
            "RNil",
            "RItem"
        ],
        "the constructors of Render"
    );

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label.as_str()).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut messages = Vec::new();
    let mut reading: Vec<Run> = Vec::new();
    let mut walking: Vec<Run> = Vec::new();
    for vector in &vectors {
        let (run, refusal) = vector.run();
        messages.extend(refusal);
        if vector.walking() {
            walking.push(run);
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

    // The committed renders: read, and walked.
    let texts = committed();
    assert!(
        texts.len() >= 200,
        "the committed lexicon entries carry {} renders",
        texts.len()
    );
    let mut committed_reading: Vec<Run> = Vec::new();
    let mut committed_walking: Vec<Run> = Vec::new();
    let mut bases: Vec<String> = Vec::new();
    for (path, source) in &texts {
        let (run, refusal) = parse_run(source, DEPTH);
        assert!(
            refusal.is_none(),
            "{path}: `{source}` is refused: {refusal:?}"
        );
        committed_reading.push(run);
        committed_walking.push(facts_run(&parsed(source)));
        bases.push(source.clone());
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
        "BS-26: {} vectors ({} read, {} walked), {} committed renders, {} and {} seeded renders under the evaluator and rustc ({:?})",
        vectors.len(),
        reading.len(),
        walking.len(),
        texts.len(),
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
        let mut read_corpus: Vec<Run> = reading.clone();
        read_corpus.extend(committed_reading.iter().cloned());
        read_corpus.extend(evaluator_seeded.1);
        let mut walk_corpus: Vec<Run> = walking.clone();
        walk_corpus.extend(committed_walking.iter().cloned());
        walk_corpus.extend(evaluator_seeded.0);
        for (root, corpus) in [("parseLre", &read_corpus), ("renderFacts", &walk_corpus)] {
            let disagreements = super::evaluate(&lowered[root], corpus, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-26: {root}: the evaluator agrees on {} runs ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-26: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled_reading = reading;
    compiled_reading.append(&mut committed_reading);
    compiled_reading.extend(rustc_seeded.1);
    let mut compiled_walking = walking;
    compiled_walking.append(&mut committed_walking);
    compiled_walking.extend(rustc_seeded.0);
    let total = rustc_differential(
        "BS-26",
        &[
            (
                "parseLre",
                &lowered["parseLre"],
                compiled_reading.as_slice(),
            ),
            (
                "renderFacts",
                &lowered["renderFacts"],
                compiled_walking.as_slice(),
            ),
        ],
        "oracle",
    );
    eprintln!("BS-26: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-26") {
        return;
    }
    export(&lowered);
    eprintln!("BS-26: passed in {:?}", started.elapsed());
}
