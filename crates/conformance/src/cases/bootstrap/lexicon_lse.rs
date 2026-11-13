//! `BS-25` (SPEC.md §17.23, §13.8): the reader and the canonical printer of
//! LexLean Semantic Expressions as language-1.2 source, `LexLeanBoot.LexiconLse`,
//! held to the byte to `lexlean::lexicon::lse::parse` and `Lse::print`.
//!
//! `parseLse` tokenizes a text, reads the tokens into one S-expression tree
//! with an explicit stack of open lists, bounded by the configured depth, and
//! converts the tree into an expression with a work stack, so that it admits no
//! `recursion`; it returns the expression or the kind and message of the first
//! error, in the order the oracle finds them. `printLse` prints an expression
//! canonically, renaming binders `x0`, `x1`, ... in printing order when asked,
//! by the structural recursion the oracle's work stack unrolls, so it admits
//! `recursion`.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::lexicon::lse::{parse, BinderMode, Lse, LseBinder, ParseError, Universe};
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.LexiconLse";
const SOURCE: &str = "bootstrap/src/LexiconLse.lex.tex";
const ROOTS: [(&str, &[&str]); 2] = [
    ("parseLse", &["allocation", "overflow"]),
    ("printLse", &["allocation", "overflow", "recursion"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The depth the committed expressions and the seeded ones are read at; the
/// vectors that reach the limit name their own.
const DEPTH: u64 = 64;
/// Seeded expressions under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 300;
const RUSTC_SEEDED: usize = 1_500;

// ---------------------------------------------------------------- the sorts

/// The constructors of an inductive of the module, in declaration order, read
/// from its text, so that the encoder names a constructor and never numbers
/// it, and one moved in the module fails here.
fn constructors(sort: &str) -> &'static Vec<String> {
    static TABLES: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    let tables = TABLES.get_or_init(|| {
        let source = std::fs::read_to_string(repo_root().join(SOURCE).as_std_path())
            .expect("the module's source");
        let mut out = BTreeMap::new();
        for block in source.split("inductive{").skip(1) {
            let block = block.split("\n}\n").next().expect("a block");
            let name = block
                .split("name: ")
                .nth(1)
                .and_then(|rest| rest.split(',').next())
                .expect("the inductive's name")
                .trim()
                .to_owned();
            let names: Vec<String> = block
                .split("{name: ")
                .skip(1)
                .map(|rest| {
                    rest.split(',')
                        .next()
                        .expect("a constructor")
                        .trim()
                        .to_owned()
                })
                .collect();
            out.insert(name, names);
        }
        out
    });
    tables
        .get(sort)
        .unwrap_or_else(|| panic!("the module has no inductive `{sort}`"))
}

fn adt(sort: &str, constructor: &str, fields: Vec<Value>) -> Value {
    let found = constructors(sort)
        .iter()
        .position(|name| name == constructor)
        .unwrap_or_else(|| panic!("`{sort}` has no constructor `{constructor}`"));
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

fn mode_nat(mode: BinderMode) -> Value {
    nat_u64(match mode {
        BinderMode::Explicit => 0,
        BinderMode::Implicit => 1,
        BinderMode::Instance => 2,
    })
}

/// A list of universes as the module's chain of `UItem` cells.
fn universe_chain(items: &[Universe]) -> Value {
    items
        .iter()
        .rev()
        .fold(adt("Universe", "UNil", Vec::new()), |rest, item| {
            adt("Universe", "UItem", vec![enc_universe(item), rest])
        })
}

fn enc_universe(universe: &Universe) -> Value {
    match universe {
        Universe::Num(value) => adt("Universe", "UNum", vec![nat_u64(*value)]),
        Universe::Var(name) => adt("Universe", "UVar", vec![text(name)]),
        Universe::Succ(inner) => adt("Universe", "USucc", vec![enc_universe(inner)]),
        Universe::Max(items) => adt("Universe", "UMax", vec![universe_chain(items)]),
        Universe::IMax(left, right) => adt(
            "Universe",
            "UIMax",
            vec![enc_universe(left), enc_universe(right)],
        ),
    }
}

/// Arguments as the module's chain of `LArg` cells.
fn arg_chain(items: &[Lse]) -> Value {
    items
        .iter()
        .rev()
        .fold(adt("Lse", "LNil", Vec::new()), |rest, item| {
            adt("Lse", "LArg", vec![enc_lse(item), rest])
        })
}

/// Binders as the module's chain of `LBinder` cells.
fn binder_chain(binders: &[LseBinder]) -> Value {
    binders
        .iter()
        .rev()
        .fold(adt("Lse", "LNil", Vec::new()), |rest, binder| {
            adt(
                "Lse",
                "LBinder",
                vec![
                    mode_nat(binder.mode),
                    text(&binder.name),
                    enc_lse(&binder.ty),
                    rest,
                ],
            )
        })
}

/// An expression as the module's `Lse`.
pub(super) fn enc_lse(expression: &Lse) -> Value {
    match expression {
        Lse::SortProp => adt("Lse", "LSortProp", Vec::new()),
        Lse::SortType(universe) => adt("Lse", "LSortType", vec![enc_universe(universe)]),
        Lse::Const(id, universes) => adt(
            "Lse",
            "LConst",
            vec![
                text(&id.package),
                text(&id.entry),
                universe_chain(universes),
            ],
        ),
        Lse::Local(name) => adt("Lse", "LLocal", vec![text(name)]),
        Lse::App(function, arguments) => {
            adt("Lse", "LApp", vec![enc_lse(function), arg_chain(arguments)])
        }
        Lse::Pi(binders, body) => adt("Lse", "LPi", vec![binder_chain(binders), enc_lse(body)]),
        Lse::Lam(binders, body) => adt("Lse", "LLam", vec![binder_chain(binders), enc_lse(body)]),
        Lse::Let {
            name,
            ty,
            value,
            body,
        } => adt(
            "Lse",
            "LLet",
            vec![text(name), enc_lse(ty), enc_lse(value), enc_lse(body)],
        ),
        Lse::Nat(digits) => adt("Lse", "LNat", vec![text(digits)]),
    }
}

/// The run of `parseLse` on a text at a depth, and the oracle's message when
/// it refuses.
fn parse_run(source: &str, depth: u64) -> (Run, Option<String>) {
    let (value, refusal) = match parse(source, depth) {
        Ok(expression) => (
            Value::Ok {
                value: Box::new(enc_lse(&expression)),
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

/// The run of `printLse` on an expression.
fn print_run(expression: &Lse, rename: bool) -> Run {
    (
        vec![enc_lse(expression), Value::Bool { value: rename }],
        serde_json::to_value(text(&expression.print(rename))).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ vectors

/// What a vector is a run of.
enum Subject {
    Parse(String, u64),
    Print(Lse, bool),
}

/// One named vector.
struct Vector {
    label: String,
    subject: Subject,
}

impl Vector {
    fn printing(&self) -> bool {
        matches!(self.subject, Subject::Print(..))
    }

    fn run(&self) -> (Run, Option<String>) {
        match &self.subject {
            Subject::Parse(source, depth) => parse_run(source, *depth),
            Subject::Print(expression, rename) => (print_run(expression, *rename), None),
        }
    }
}

/// The oracle's reading of a text the vectors print.
fn parsed(source: &str) -> Lse {
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
        ("a sort prop", "(sort prop)"),
        ("a sort type of a number", "(sort (type 0))"),
        ("a sort type of a variable", "(sort (type u))"),
        ("a sort type of succ", "(sort (type (succ u)))"),
        ("a sort type of max", "(sort (type (max u v 1)))"),
        ("a sort type of imax", "(sort (type (imax u v)))"),
        (
            "a sort type of nested universes",
            "(sort (type (max (succ u) (imax 1 v) (succ (succ 0)))))",
        ),
        ("a const", "(const lexlean.std.nat::nat)"),
        ("a const with universes", "(const pkg::entry (u 1))"),
        ("a const with one universe", "(const pkg::entry ((succ u)))"),
        (
            "a const of a package with segments and hyphens",
            "(const a.b2.c-d::e.f-g9)",
        ),
        ("a const with a nested entry", "(const pkg::a.b)"),
        ("a local", "(local x)"),
        ("a local with digits", "(local x1)"),
        ("a local of capitals", "(local Xy)"),
        ("an app", "(app (local f) (local a))"),
        ("an app of three", "(app (local f) (local a) (local b))"),
        (
            "an app of apps",
            "(app (app (local f) (local a)) (app (local g) (local b)))",
        ),
        ("a pi", "(pi ((explicit a (sort prop))) (local a))"),
        (
            "a pi of three binders",
            "(pi ((explicit a (sort prop)) (implicit b (local a)) (instance c (local b))) (app (local b) (local c)))",
        ),
        ("a lam", "(lam ((explicit x (const p::t))) (local x))"),
        (
            "nested binders",
            "(pi ((explicit a (sort prop))) (lam ((explicit b (local a))) (local b)))",
        ),
        ("a let", "(let x (sort prop) (local y) (local x))"),
        (
            "a let in a let",
            "(let x (sort prop) (sort prop) (let y (local x) (local x) (local y)))",
        ),
        ("a nat", "(nat 0)"),
        ("a large nat", "(nat 18446744073709551615)"),
        ("a nat of many digits", "(nat 123456789)"),
        ("white space of both kinds", "(app\n(local f)  (local a)\n)"),
        ("leading and trailing white space", "  (sort prop)\n"),
        ("atoms touching parentheses", "(app(local f)(local a))"),
        (
            "the oracle's own signature",
            "(pi ((explicit a (const lexlean.std.nat::nat)) (explicit b (const lexlean.std.nat::nat))) (const lexlean.std.nat::nat))",
        ),
    ] {
        add(label, reading(source));
    }

    // Reading: the limit on nesting.
    add(
        "nesting at the depth",
        Subject::Parse("(app (local f) (local a))".to_owned(), 2),
    );
    add(
        "nesting beyond the depth",
        Subject::Parse("(app (local f) (local a))".to_owned(), 1),
    );
    add(
        "a parenthesis at depth zero",
        Subject::Parse("(sort prop)".to_owned(), 0),
    );
    add("an atom at depth zero", Subject::Parse("x".to_owned(), 0));

    // Reading: what is refused.
    for (label, source) in [
        ("the empty text", ""),
        ("white space only", " \n"),
        ("an unbalanced close", ")"),
        ("an unclosed open", "("),
        ("an unclosed inner list", "(app (local f)"),
        ("trailing tokens", "(sort prop) x"),
        ("two expressions", "(sort prop) (sort prop)"),
        ("a tab", "(sort\tprop)"),
        ("a carriage return", "(sort prop)\r"),
        ("a bare atom", "x"),
        ("an empty list", "()"),
        ("a list headed by a list", "((sort prop))"),
        ("an unknown form", "(foo)"),
        ("a sort of nothing", "(sort)"),
        ("a sort of a word", "(sort set)"),
        ("a sort type without a universe", "(sort (type))"),
        ("a sort type of two universes", "(sort (type u v))"),
        ("a sort of a list that is not type", "(sort (kind u))"),
        ("a sort type of a bad universe", "(sort (type 01))"),
        ("a universe that is a word", "(sort (type u-v))"),
        ("a universe form unknown", "(sort (type (plus u v)))"),
        ("a succ of two", "(sort (type (succ u v)))"),
        ("a max of one", "(sort (type (max u)))"),
        ("an imax of three", "(sort (type (imax u v w)))"),
        ("a universe list without a head", "(sort (type ((u))))"),
        ("a universe too large", "(sort (type 18446744073709551616))"),
        ("a const of nothing", "(const)"),
        ("a const with empty universes", "(const p::e ())"),
        ("a const of two atoms", "(const p::e u)"),
        ("a const without a scope", "(const nat)"),
        ("a const with a bad package", "(const Pkg::e)"),
        (
            "a const with a hyphen in the first package segment",
            "(const a-b::e)",
        ),
        ("a const with a bad entry", "(const p::E)"),
        ("a const with an empty entry", "(const p::)"),
        ("a const with an empty package", "(const ::e)"),
        ("a const with a bad universe", "(const p::e (01))"),
        ("a const with a double scope", "(const p::e::f)"),
        ("a local of nothing", "(local)"),
        ("a local of two", "(local x y)"),
        ("a local that is a list", "(local (x))"),
        ("a local with a bad name", "(local 1x)"),
        ("a local with an underscore", "(local x_y)"),
        ("a nat of nothing", "(nat)"),
        ("a nat of a word", "(nat x)"),
        ("a nat with a leading zero", "(nat 01)"),
        ("a nat too large", "(nat 18446744073709551616)"),
        ("a nat of twenty-one digits", "(nat 100000000000000000000)"),
        ("a nat that is a list", "(nat (1))"),
        ("an app of one", "(app (local f))"),
        ("an app of nothing", "(app)"),
        ("an app with a bad argument", "(app (local f) x)"),
        ("an app with a bad function", "(app x (local a))"),
        ("a pi of one", "(pi ((explicit a (sort prop))))"),
        (
            "a lam of three",
            "(lam ((explicit a (sort prop))) (local a) (local a))",
        ),
        ("a pi whose binders are an atom", "(pi x (local x))"),
        ("a pi of no binders", "(pi () (sort prop))"),
        ("a binder that is an atom", "(pi (x) (local x))"),
        ("a binder of two", "(pi ((explicit a)) (local a))"),
        (
            "a binder with a bad mode",
            "(pi ((strict a (sort prop))) (local a))",
        ),
        (
            "a binder whose mode is a list",
            "(pi (((explicit) a (sort prop))) (local a))",
        ),
        (
            "a binder whose name is a list",
            "(pi ((explicit (a) (sort prop))) (local a))",
        ),
        (
            "a binder with a bad name",
            "(pi ((explicit 1a (sort prop))) (local a))",
        ),
        (
            "the last binder is checked first",
            "(pi ((bad a (sort prop)) (explicit 1b (sort prop))) (sort prop))",
        ),
        (
            "a binder with a bad type",
            "(pi ((explicit a x)) (local a))",
        ),
        ("a let of three", "(let x (sort prop) (local y))"),
        (
            "a let whose name is a list",
            "(let (x) (sort prop) (sort prop) (sort prop))",
        ),
        (
            "a let with a bad name",
            "(let 1x (sort prop) (sort prop) (sort prop))",
        ),
        (
            "the function's error comes before the argument's",
            "(app (foo) (bar))",
        ),
        (
            "a binder type's error comes before the body's",
            "(pi ((explicit a (foo))) (bar))",
        ),
        (
            "a leaf error inside a deep form",
            "(app (local f) (app (local g) (nat 01)))",
        ),
    ] {
        add(label, reading(source));
    }

    // Printing: every form, as read and renamed.
    for (label, source) in [
        ("a sort prop printed", "(sort prop)"),
        ("a sort type printed", "(sort (type (max (succ u) (imax 1 v))))"),
        ("a const printed", "(const lexlean.std.nat::nat)"),
        ("a const with universes printed", "(const pkg::entry (u (succ 1) (max u v)))"),
        ("a local printed", "(local x)"),
        ("an app printed", "(app (local f) (local a) (local b))"),
        ("a nat printed", "(nat 42)"),
        (
            "white space canonicalized",
            "(app\n(local f)  (local a)\n)",
        ),
        (
            "a pi renamed",
            "(pi ((explicit a (sort prop)) (explicit b (local a))) (app (local a) (local b)))",
        ),
        (
            "a lam of every mode renamed",
            "(lam ((explicit a (sort prop)) (implicit b (local a)) (instance c (local b))) (local c))",
        ),
        (
            "shadowing",
            "(pi ((explicit a (sort prop))) (lam ((explicit a (local a))) (local a)))",
        ),
        ("an unbound local renamed stays", "(app (local f) (local z))"),
        ("a let renamed", "(let x (local x) (local x) (local x))"),
        (
            "a let name shadows a binder",
            "(pi ((explicit x (sort prop))) (let x (local x) (local x) (local x)))",
        ),
        (
            "the counter runs through siblings",
            "(app (pi ((explicit a (sort prop))) (local a)) (lam ((explicit b (sort prop))) (local b)))",
        ),
        (
            "a binder whose type names itself",
            "(pi ((explicit a (local a))) (sort prop))",
        ),
        (
            "the oracle's own signature printed",
            "(pi ((explicit a (const lexlean.std.nat::nat)) (explicit b (const lexlean.std.nat::nat))) (const lexlean.std.nat::nat))",
        ),
    ] {
        add(label, Subject::Print(parsed(source), false));
        add(
            &format!("{label}, renamed"),
            Subject::Print(parsed(source), true),
        );
    }
    out
}

// ------------------------------------------------------------------- seeded

const NAMES: &[&str] = &["a", "b", "x", "y", "f", "g", "u1", "Zed"];
const UNIVERSES: &[&str] = &["u", "v", "w1"];
const CONSTS: &[&str] = &[
    "lexlean.std.nat::nat",
    "lexlean.std.nat::add",
    "core::eq",
    "p.q-r::s.t-u",
];

fn drawn_universe(seeded: &mut Seeded, depth: usize) -> Universe {
    let kind = if depth == 0 {
        seeded.below(2)
    } else {
        seeded.below(5)
    };
    match kind {
        0 => Universe::Num(*seeded.pick(&[0u64, 1, 2, 7, 18_446_744_073_709_551_615])),
        1 => Universe::Var((*seeded.pick(UNIVERSES)).to_owned()),
        2 => Universe::Succ(Box::new(drawn_universe(seeded, depth - 1))),
        3 => {
            let count = 2 + seeded.below(2);
            Universe::Max(
                (0..count)
                    .map(|_| drawn_universe(seeded, depth - 1))
                    .collect(),
            )
        }
        _ => Universe::IMax(
            Box::new(drawn_universe(seeded, depth - 1)),
            Box::new(drawn_universe(seeded, depth - 1)),
        ),
    }
}

fn drawn_binders(seeded: &mut Seeded, depth: usize) -> Vec<LseBinder> {
    let count = 1 + seeded.below(3);
    (0..count)
        .map(|_| LseBinder {
            mode: *seeded.pick(&[
                BinderMode::Explicit,
                BinderMode::Explicit,
                BinderMode::Implicit,
                BinderMode::Instance,
            ]),
            name: (*seeded.pick(NAMES)).to_owned(),
            ty: drawn_lse(seeded, depth - 1),
        })
        .collect()
}

/// An expression of at most `depth` levels.
pub(super) fn drawn_lse(seeded: &mut Seeded, depth: usize) -> Lse {
    let kind = if depth == 0 {
        seeded.below(5)
    } else {
        seeded.below(9)
    };
    match kind {
        0 => Lse::SortProp,
        1 => Lse::SortType(drawn_universe(seeded, 2)),
        2 => {
            let id = lexlean::lexicon::lse::QualifiedId::parse(seeded.pick(CONSTS))
                .expect("a qualified ID");
            let universes = if seeded.chance(40) {
                (0..=seeded.below(2))
                    .map(|_| drawn_universe(seeded, 1))
                    .collect()
            } else {
                Vec::new()
            };
            Lse::Const(id, universes)
        }
        3 => Lse::Local((*seeded.pick(NAMES)).to_owned()),
        4 => Lse::Nat((*seeded.pick(&["0", "1", "42", "18446744073709551615"])).to_owned()),
        5 => {
            let count = 1 + seeded.below(3);
            Lse::App(
                Box::new(drawn_lse(seeded, depth - 1)),
                (0..count).map(|_| drawn_lse(seeded, depth - 1)).collect(),
            )
        }
        6 => Lse::Pi(
            drawn_binders(seeded, depth),
            Box::new(drawn_lse(seeded, depth - 1)),
        ),
        7 => Lse::Lam(
            drawn_binders(seeded, depth),
            Box::new(drawn_lse(seeded, depth - 1)),
        ),
        _ => Lse::Let {
            name: (*seeded.pick(NAMES)).to_owned(),
            ty: Box::new(drawn_lse(seeded, depth - 1)),
            value: Box::new(drawn_lse(seeded, depth - 1)),
            body: Box::new(drawn_lse(seeded, depth - 1)),
        },
    }
}

/// Pieces an edit inserts or substitutes.
const PIECES: &[&str] = &[
    "(",
    ")",
    " ",
    "\n",
    "\t",
    "sort",
    "prop",
    "type",
    "app",
    "pi",
    "lam",
    "let",
    "local",
    "const",
    "nat",
    "explicit",
    "x",
    "0",
    "01",
    "::",
    "-",
    ".",
    "(local x)",
    "(sort prop)",
    "(explicit a (sort prop))",
];

/// One seeded edit of a text at a byte boundary, up to three of them.
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

/// `count` seeded expressions, with the runs of `printLse` on them, and the
/// runs of `parseLse` on their printed texts and on edits of those.
fn seeded_runs(count: usize, bases: &[String]) -> (Vec<Run>, Vec<Run>) {
    let mut seeded = Seeded(0x0B55_0025);
    let mut printing = Vec::with_capacity(count);
    let mut reading = Vec::with_capacity(2 * count);
    while printing.len() < count {
        let depth = 1 + seeded.below(4);
        let expression = drawn_lse(&mut seeded, depth);
        let rename = seeded.chance(50);
        let printed = expression.print(false);
        printing.push(print_run(&expression, rename));
        reading.push(parse_run(&printed, DEPTH).0);
        let base = if seeded.chance(30) && !bases.is_empty() {
            seeded.pick(bases).clone()
        } else {
            printed
        };
        reading.push(parse_run(&edited(&mut seeded, &base), DEPTH).0);
    }
    (printing, reading)
}

/// Every LSE text a committed lexicon entry carries: the `signature` and
/// `value` fields of every `entries/*.toml`, with its path.
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
            for field in ["signature = \"", "value = \""] {
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
        label: "the depth admits one level more",
        root: 0,
        site: "(prim length (project st stack) : Nat) <? maxDepth",
        replacement: "(prim length (project st stack) : Nat) <? (maxDepth + 1)",
        vector: "nesting beyond the depth",
    },
    Plant {
        label: "a tab is white space",
        root: 0,
        site: "(if (value <? 32) then true else (value =? 127))",
        replacement: "(if (value <? 9) then true else (value =? 127))",
        vector: "a tab",
    },
    Plant {
        label: "an LF is not white space",
        root: 0,
        site: "(if (value =? 32) then true else (value =? 10))",
        replacement: "(if (value =? 32) then true else (value =? 11))",
        vector: "white space of both kinds",
    },
    Plant {
        label: "a close with nothing open is the root",
        root: 0,
        site: "| List.nil => (rtFail st (failed (lit \"unbalanced `)`\")))",
        replacement: "| List.nil => (rtPlace st (ctor Node.NNil) (nil (List Node)))",
        vector: "an unbalanced close",
    },
    Plant {
        label: "trailing tokens are ignored",
        root: 0,
        site: "| Option.some found => (ctor Option.some [Rt] (rtFail st (failed (lit \"trailing tokens after the expression\"))))",
        replacement: "| Option.some found => (ctor Option.some [Rt] (record Rt {pos := (project st pos), stack := (project st stack), root := (project st root), error := (ctor Option.none [Failure]), done := true}))",
        vector: "trailing tokens",
    },
    Plant {
        label: "the end of the text closes an open list",
        root: 0,
        site: "| List.cons top rest => (ctor Option.some [Rt] (rtFail st (failed (lit \"unexpected end of expression\"))))",
        replacement: "| List.cons top rest => (ctor Option.some [Rt] (rtToken maxDepth st (ctor Tok.TClose)))",
        vector: "an unclosed open",
    },
    Plant {
        label: "app admits one argument fewer",
        root: 0,
        site: "(if ((prim length arguments : Nat) <? 2) then",
        replacement: "(if ((prim length arguments : Nat) <? 1) then",
        vector: "an app of one",
    },
    Plant {
        label: "pi admits no binder",
        root: 0,
        site: "| List.nil => (cvFail st (failed (cat (quoted head) (lit \" has at least one binder\"))))",
        replacement: "| List.nil => (record Cv {work := (cons (ctor Work.WVisit (nodeAt arguments 1)) (cons (ctor Work.WBuild node) rest)), values := (project st values), error := (ctor Option.none [Failure])})",
        vector: "a pi of no binders",
    },
    Plant {
        label: "binders are checked first to last",
        root: 0,
        site: "(LexLeanStd.Order::reverseList [Node] binders) : (Result (List Work) Failure))",
        replacement: "binders : (Result (List Work) Failure))",
        vector: "the last binder is checked first",
    },
    Plant {
        label: "a binder's name need not be a name",
        root: 0,
        site: "(if (isLseIdentifier name) then (ctor Result.ok [Header, Failure]",
        replacement: "(if true then (ctor Result.ok [Header, Failure]",
        vector: "a binder with a bad name",
    },
    Plant {
        label: "a let's name need not be a name",
        root: 0,
        site: "(if (isLseIdentifier name) then (record Cv",
        replacement: "(if true then (record Cv",
        vector: "a let with a bad name",
    },
    Plant {
        label: "a local need not be a name",
        root: 0,
        site: "(if (isLseIdentifier name) then (ctor Result.ok [Lse, Failure] (ctor Lse.LLocal name))",
        replacement: "(if true then (ctor Result.ok [Lse, Failure] (ctor Lse.LLocal name))",
        vector: "a local with a bad name",
    },
    Plant {
        label: "a package's first segment admits a hyphen",
        root: 0,
        site: "(idScan text false)",
        replacement: "(idScan text true)",
        vector: "a const with a hyphen in the first package segment",
    },
    Plant {
        label: "an empty segment ends an ID",
        root: 0,
        site: "(if (project final segStart) then false else true)",
        replacement: "(if (project final segStart) then true else true)",
        vector: "a const with an empty entry",
    },
    Plant {
        label: "a leading zero is a digit",
        root: 0,
        site: "(if (if (1 <? n) then ((byteNum text 0) =? 48) else false) then",
        replacement: "(if (if (1 <? n) then ((byteNum text 0) =? 47) else false) then",
        vector: "a nat with a leading zero",
    },
    Plant {
        label: "the 64-bit bound is one digit wider",
        root: 0,
        site: "(if (if (20 <? n) then true else (if (n =? 20) then",
        replacement: "(if (if (21 <? n) then true else (if (n =? 21) then",
        vector: "a nat of twenty-one digits",
    },
    Plant {
        label: "the largest number is beyond the bound",
        root: 0,
        site: "(prim compare_bytes 0x01 0x : Ordering) : Bool) else false))",
        replacement: "(prim compare_bytes 0x 0x : Ordering) : Bool) else false))",
        vector: "a large nat",
    },
    Plant {
        label: "succ takes two",
        root: 0,
        site: "(if (prim equal head (lit \"succ\") : Bool) then (argc =? 1) else",
        replacement: "(if (prim equal head (lit \"succ\") : Bool) then (argc <=? 2) else",
        vector: "a succ of two",
    },
    Plant {
        label: "max takes one",
        root: 0,
        site: "(2 <=? argc)",
        replacement: "(1 <=? argc)",
        vector: "a max of one",
    },
    Plant {
        label: "any word is a universe",
        root: 0,
        site: "else (if (isLseIdentifier text) then (record Uv {work := rest, values := (cons (ctor Universe.UVar text)",
        replacement: "else (if true then (record Uv {work := rest, values := (cons (ctor Universe.UVar text)",
        vector: "a universe that is a word",
    },
    Plant {
        label: "a sort type takes two universes",
        root: 0,
        site: "(if ((prim length items : Nat) =? 2) then",
        replacement: "(if (2 <=? (prim length items : Nat)) then",
        vector: "a sort type of two universes",
    },
    Plant {
        label: "a const's empty universes are none",
        root: 0,
        site: "| List.nil => (ctor Result.error [Lse, Failure] (constError))",
        replacement: "| List.nil => (match (qualifiedId spelled)\n          | Result.ok parts => (ctor Result.ok [Lse, Failure] (ctor Lse.LConst (first parts) (second parts) (ctor Universe.UNil)))\n          | Result.error failure => (ctor Result.error [Lse, Failure] failure))",
        vector: "a const with empty universes",
    },
    Plant {
        label: "the renamed binders count from one",
        root: 1,
        site: "((cat (lit \"x\") (LexLeanStd.Numeral::decimal counter)), (counter + 1))",
        replacement: "((cat (lit \"x\") (LexLeanStd.Numeral::decimal (counter + 1))), (counter + 1))",
        vector: "a pi renamed, renamed",
    },
    Plant {
        label: "a local is renamed by its outermost binder",
        root: 1,
        site: "(ctor Option.none [Bytes]) scope : (Option Bytes))",
        replacement: "(ctor Option.none [Bytes]) (LexLeanStd.Order::reverseList [(Bytes * Bytes)] scope) : (Option Bytes))",
        vector: "shadowing, renamed",
    },
    Plant {
        label: "a let's value is outside its scope",
        root: 1,
        site: "(showExpr bound rename inner (second shownType))",
        replacement: "(showExpr bound rename scope (second shownType))",
        vector: "a let renamed, renamed",
    },
    Plant {
        label: "a binder's type is in its own scope",
        root: 1,
        site: "(showExpr ty rename scope (second shownName))",
        replacement: "(showExpr ty rename (cons (name, (first shownName)) scope) (second shownName))",
        vector: "a binder whose type names itself, renamed",
    },
    Plant {
        label: "arguments print without a space",
        root: 1,
        site: "((cat (cat (lit \" \") (first shownHead)) (first shownTail)), (second shownTail))",
        replacement: "((cat (first shownHead) (first shownTail)), (second shownTail))",
        vector: "an app printed",
    },
    Plant {
        label: "a const's universes print without their parentheses",
        root: 1,
        site: "(cat (cat (cat heading (lit \" (\")) (LexLeanStd.Text::sliceOr listed 1",
        replacement: "(cat (cat (cat heading (lit \" \")) (LexLeanStd.Text::sliceOr listed 1",
        vector: "a const with universes printed",
    },
    Plant {
        label: "implicit prints as explicit",
        root: 1,
        site: "(if (mode =? 1) then (lit \"implicit\") else",
        replacement: "(if (mode =? 3) then (lit \"implicit\") else",
        vector: "a lam of every mode renamed",
    },
    Plant {
        label: "the binders of a pi print without their space",
        root: 1,
        site: "(((cat (cat piece (lit \" \")) (first (first shownRest)))",
        replacement: "(((cat piece (first (first shownRest)))",
        vector: "a pi renamed",
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
        let source = planted.read_raw("src/LexiconLse.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/LexiconLse.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, &[vector.run().0], 1).first() {
            Some(first) => eprintln!(
                "BS-25: plant detected: {}: {}",
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
            "BS-25: {}: largest certificate part {largest} bytes",
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
    "whitespace is one or more ASCII spaces or LF",
    "max_scope_depth exceeded: configured",
    "trailing tokens after the expression",
    "unbalanced `)`",
    "unexpected end of expression",
    "expected `(`, found",
    "expected an LSE form",
    "`app` has at least a function and one argument",
    "`pi` is `(pi (binders...) body)`",
    "`lam` is `(lam (binders...) body)`",
    "expected a binder list",
    "has at least one binder",
    "expected a binder, found",
    "a binder is `(mode name type)`",
    "expected a binder mode, found",
    "is not a binder mode",
    "expected a local ID, found",
    "is not a valid local ID",
    "`let` is `(let name type value body)`",
    "is not an LSE form",
    "a sort is `(sort prop)` or `(sort (type u))`",
    "is not a universe form",
    "is not a universe",
    "has the wrong arity",
    "expected a universe form",
    "a const is `(const qualified-id [universes])`",
    "is not a qualified `package::entry` ID",
    "is not a valid package ID",
    "is not a valid local entry ID",
    "a local is `(local id)`",
    "a nat is `(nat decimal)`",
    "is not a decimal",
    "has a redundant leading zero",
    "does not fit 64 bits",
];

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("LexiconLse"),
        "LexiconLse is a linked module of the project"
    );
    eligibility_of(&project, "LexiconLse", MODULE, &ROOTS);
    assert_eq!(
        constructors("Lse").as_slice(),
        [
            "LSortProp",
            "LSortType",
            "LConst",
            "LLocal",
            "LApp",
            "LPi",
            "LLam",
            "LLet",
            "LNat",
            "LNil",
            "LArg",
            "LBinder"
        ],
        "the constructors of Lse"
    );
    assert_eq!(
        constructors("Universe").as_slice(),
        ["UNum", "UVar", "USucc", "UMax", "UIMax", "UNil", "UItem"],
        "the constructors of Universe"
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

    // The committed expressions: read, printed both ways, and read back.
    let texts = committed();
    assert!(
        texts.len() >= 400,
        "the committed lexicon entries carry {} LSE texts",
        texts.len()
    );
    let mut committed_reading: Vec<Run> = Vec::new();
    let mut committed_printing: Vec<Run> = Vec::new();
    let mut bases: Vec<String> = Vec::new();
    for (path, source) in &texts {
        let (run, refusal) = parse_run(source, DEPTH);
        assert!(
            refusal.is_none(),
            "{path}: `{source}` is refused: {refusal:?}"
        );
        committed_reading.push(run);
        let expression = parsed(source);
        committed_printing.push(print_run(&expression, false));
        committed_printing.push(print_run(&expression, true));
        let canonical = expression.print(false);
        committed_reading.push(parse_run(&canonical, DEPTH).0);
        bases.push(canonical);
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
        "BS-25: {} vectors ({} read, {} printed), {} committed expressions ({} read and {} printed), {} and {} seeded expressions under the evaluator and rustc ({:?})",
        vectors.len(),
        reading.len(),
        printing.len(),
        texts.len(),
        committed_reading.len(),
        committed_printing.len(),
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
        let mut print_corpus: Vec<Run> = printing.clone();
        print_corpus.extend(committed_printing.iter().cloned());
        print_corpus.extend(evaluator_seeded.0);
        for (root, corpus) in [("parseLse", &read_corpus), ("printLse", &print_corpus)] {
            let disagreements = super::evaluate(&lowered[root], corpus, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-25: {root}: the evaluator agrees on {} runs ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-25: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled_reading = reading;
    compiled_reading.append(&mut committed_reading);
    compiled_reading.extend(rustc_seeded.1);
    let mut compiled_printing = printing;
    compiled_printing.append(&mut committed_printing);
    compiled_printing.extend(rustc_seeded.0);
    let total = rustc_differential(
        "BS-25",
        &[
            (
                "parseLse",
                &lowered["parseLse"],
                compiled_reading.as_slice(),
            ),
            (
                "printLse",
                &lowered["printLse"],
                compiled_printing.as_slice(),
            ),
        ],
        "oracle",
    );
    eprintln!("BS-25: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-25") {
        return;
    }
    export(&lowered);
    eprintln!("BS-25: passed in {:?}", started.elapsed());
}
