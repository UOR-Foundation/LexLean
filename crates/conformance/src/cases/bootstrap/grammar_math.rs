//! `BS-42` (SPEC.md §17.23, §15.5, §14.1): the grammar of a mathematical island
//! as language-1.2 source, `LexLeanBoot.GrammarMath`, held to the byte to
//! `lexlean::grammar::math::parse_math`.
//!
//! The root parses an island's atom range by the oracle's Pratt parser, kept
//! as an explicit stack of frames under `iterate_until`, under every
//! segmentation the token lattice admits, with the oracle's worklist of
//! branch points. The glossary closure is the lexicon's: the host gives the
//! root its projection, the math-channel edges at each atom position with the
//! fields of each edge's form the parser reads (frame, precedence,
//! associativity), and the limits and counters of the parse budget, so every
//! limit failure falls where the oracle's does.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::grammar::chart::Budget;
use lexlean::grammar::math::{parse_math, LeafKind, MathAst};
use lexlean::lexicon::entry::{Associativity, Channel, Frame};
use lexlean::lexicon::package::LoadContext;
use lexlean::lexicon::resolve::{Closure, FormRef};
use lexlean::source::atom::{Atom, AtomClass};
use serde_json::Value as Json;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::scan::{atom_value, rejection_value};
use super::{list, nat_val, text, Run, Seeded};
use crate::production_export as harness;
use crate::support;

const MODULE: &str = "LexLeanBoot.GrammarMath";
const SOURCE: &str = "src/GrammarMath.lex.tex";
const ROOTS: [(&str, &[&str]); 1] = [("parseMath", &["allocation", "overflow"])];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// The path every vector is read under.
const PATH: &str = "src/Island.lex.tex";
/// The limits of the committed projects (`bootstrap/lexlean.toml`).
const MAX_EDGES: u64 = 4_000_000;
const MAX_STATES: u64 = 4_000_000;
const MAX_DEPTH: u64 = 1_024;
/// Islands of the committed sources: at most this many atoms, and how many
/// under the evaluator and under rustc.
const ISLAND_ATOMS: usize = 40;
const EVALUATOR_ISLANDS: usize = 300;
const RUSTC_ISLANDS: usize = 3_000;
/// The atoms after an island's range the root is also given.
const TAIL_ATOMS: usize = 6;
/// Seeded islands under the evaluator and under rustc.
const EVALUATOR_SEEDED: usize = 300;
const RUSTC_SEEDED: usize = 3_000;
/// The packages every parse sees.
const VISIBLE: [&str; 3] = ["lexlean.core", "lexlean.std.nat", "case.math"];

// ------------------------------------------------------------ the closure

/// A lexicon package of the case's own, built from committed entries with
/// their frame, precedence, and surface changed, so that the vectors reach
/// what the committed lexicons never write: a postfix form, prefix and
/// operator candidates that disagree on their profile, a surface that is both
/// a prefix operator and a leaf, and leaves of two atoms, which are branch
/// points of the token lattice (one of them, `p !`, also parses as a postfix,
/// so that an island has two covers).
fn synthetic_files() -> Vec<(String, Vec<u8>)> {
    let prefix = |id: &str, surface: &str, precedence: u8| {
        format!(
            "spec = \"lexlean/entry/1\"\nid = \"{id}\"\ncategory = \"prefix-function\"\nsignature = \"(pi ((explicit p (sort prop))) (sort prop))\"\nsurface_arity = 1\nframe = \"prefix\"\nprecedence = {precedence}\n\n[denotation]\nkind = \"lean\"\nmodule = \"Init\"\nname = \"Not\"\n\n[[form]]\nid = \"{id}\"\nchannel = \"math\"\nsurface = \"{surface}\"\ncanonical_source = true\nfeatures = []\n\n[render]\nmath = \"(seq (token logical-not) (slot 0))\"\n"
        )
    };
    let postfix = "spec = \"lexlean/entry/1\"\nid = \"bang\"\ncategory = \"postfix-function\"\nsignature = \"(pi ((explicit p (sort prop))) (sort prop))\"\nsurface_arity = 1\nframe = \"postfix\"\nprecedence = 90\n\n[denotation]\nkind = \"lean\"\nmodule = \"Init\"\nname = \"Not\"\n\n[[form]]\nid = \"bang\"\nchannel = \"math\"\nsurface = \"!\"\ncanonical_source = true\nfeatures = []\n\n[render]\nmath = \"(seq (slot 0) (token logical-not))\"\n".to_owned();
    let infix = |id: &str, precedence: u8| {
        format!(
            "spec = \"lexlean/entry/1\"\nid = \"{id}\"\ncategory = \"infix-predicate\"\nsignature = \"(pi ((implicit a (sort (type u))) (explicit b (local a)) (explicit c (local a))) (sort prop))\"\nuniverses = [\"u\"]\nsurface_arity = 2\nframe = \"infix\"\nprecedence = {precedence}\nassociativity = \"none\"\n\n[denotation]\nkind = \"lean\"\nmodule = \"Init\"\nname = \"Eq\"\n\n[[form]]\nid = \"{id}\"\nchannel = \"math\"\nsurface = \"≈\"\ncanonical_source = true\nfeatures = []\n\n[render]\nmath = \"(seq (slot 0) (space) (token equals) (space) (slot 1))\"\n"
        )
    };
    let atom = |id: &str, surface: &str| {
        format!(
            "spec = \"lexlean/entry/1\"\nid = \"{id}\"\ncategory = \"proof-constant\"\nsignature = \"(pi ((implicit a (sort (type u))) (implicit x (local a))) (app (const lexlean.core::eq) (local x) (local x)))\"\nuniverses = [\"u\"]\nsurface_arity = 0\nframe = \"atom\"\n\n[denotation]\nkind = \"lean\"\nmodule = \"Init\"\nname = \"rfl\"\n\n[[form]]\nid = \"{id}\"\nchannel = \"math\"\nsurface = \"{surface}\"\ncanonical_source = true\nfeatures = []\n\n[render]\nmath = \"(operator-name rfl)\"\n"
        )
    };
    let mut files = vec![
        (
            "lexicon.toml".to_owned(),
            "spec = \"lexlean/lexicon/1\"\npackage = \"case.math\"\nversion = \"1.0.0\"\nlanguage = \"1.0\"\nimports = [\"lexlean.core@1.0.0\"]\n".to_owned(),
        ),
        ("entries/approx-high.toml".to_owned(), infix("approx-high", 55)),
        ("entries/approx-low.toml".to_owned(), infix("approx-low", 50)),
        ("entries/bang.toml".to_owned(), postfix),
        ("entries/both-leaf.toml".to_owned(), atom("both-leaf", "bb")),
        ("entries/both-prefix.toml".to_owned(), prefix("both-prefix", "bb", 40)),
        ("entries/pair-bang.toml".to_owned(), atom("pair-bang", "p !")),
        ("entries/pair-leaf.toml".to_owned(), atom("pair-leaf", "p q")),
        ("entries/tilde-high.toml".to_owned(), prefix("tilde-high", "tl", 45)),
        ("entries/tilde-low.toml".to_owned(), prefix("tilde-low", "tl", 40)),
    ];
    files.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    files
        .into_iter()
        .map(|(path, content)| (path, content.into_bytes()))
        .collect()
}

/// The built-in closure (`lexlean.core`, `lexlean.std.nat`) with the case's
/// package.
fn closure() -> &'static Closure {
    static CLOSURE: OnceLock<Closure> = OnceLock::new();
    CLOSURE.get_or_init(|| {
        let bootstrap = lexlean::lexicon::load_bootstrap().expect("bootstrap loads");
        let ctx = LoadContext {
            language: lexlean::LANGUAGE_VERSION,
            forbidden_controls: &bootstrap.structural.forbidden_controls,
            max_scope_depth: 1024,
        };
        let mut packages: Vec<_> = bootstrap
            .builtin_packages
            .iter()
            .map(|row| lexlean::lexicon::load_builtin_package(row, &ctx).expect("a package"))
            .collect();
        packages.push(
            lexlean::lexicon::package::load_package(
                "case:case.math",
                &synthetic_files(),
                None,
                &ctx,
            )
            .unwrap_or_else(|diagnostics| panic!("the case's lexicon package: {diagnostics:?}")),
        );
        let registry = lexlean::lexicon::load_token_registry().expect("the registry");
        Closure::build(packages, registry, bootstrap, 128)
            .unwrap_or_else(|diagnostics| panic!("the case's closure: {diagnostics:?}"))
    })
}

fn visible() -> BTreeSet<String> {
    VISIBLE
        .iter()
        .map(|package| (*package).to_owned())
        .collect()
}

// ------------------------------------------------------------- the encoders

fn frame_code(frame: Frame) -> usize {
    match frame {
        Frame::Atom => 0,
        Frame::Call => 1,
        Frame::Prefix => 2,
        Frame::Postfix => 3,
        Frame::Infix => 4,
        _ => 5,
    }
}

fn associativity_code(associativity: Option<Associativity>) -> usize {
    match associativity {
        None => 0,
        Some(Associativity::Left) => 1,
        Some(Associativity::Right) => 2,
        Some(Associativity::None) => 3,
    }
}

/// The forms of one run, numbered in the order the edges meet them.
#[derive(Default)]
struct Forms(Vec<FormRef>);

impl Forms {
    fn index(&mut self, form: &FormRef) -> usize {
        if let Some(found) = self.0.iter().position(|known| known == form) {
            return found;
        }
        self.0.push(form.clone());
        self.0.len() - 1
    }

    fn known(&self, form: &FormRef) -> usize {
        self.0
            .iter()
            .position(|known| known == form)
            .expect("an output form is a form of the edges")
    }
}

/// The projection of the closure the root reads: the math-channel edges at
/// each atom position, each with the fields of its form the parser reads (a
/// form the closure cannot resolve is frame 5, which no rule takes, and is
/// still counted).
fn edges_value(atoms: &[Atom], forms: &mut Forms) -> Value {
    let closure = closure();
    let visible = visible();
    list(
        (0..atoms.len())
            .map(|position| {
                list(
                    closure
                        .matches_at(atoms, position, Channel::Math, &visible)
                        .iter()
                        .map(|(form, end)| {
                            let (frame, precedence, associativity) =
                                closure.form(form).map_or((5, 0, 0), |(entry, _)| {
                                    (
                                        frame_code(entry.frame),
                                        usize::from(entry.precedence.unwrap_or(0)),
                                        associativity_code(entry.associativity),
                                    )
                                });
                            Value::Adt {
                                constructor: 0,
                                fields: vec![
                                    nat_val(forms.index(form)),
                                    nat_val(*end),
                                    nat_val(frame),
                                    nat_val(precedence),
                                    nat_val(associativity),
                                ],
                            }
                        })
                        .collect(),
                )
            })
            .collect(),
    )
}

fn adt(constructor: u64, fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor,
        fields,
    }
}

fn forms_value(candidates: &[FormRef], forms: &Forms) -> Value {
    list(
        candidates
            .iter()
            .map(|form| nat_val(forms.known(form)))
            .collect(),
    )
}

fn math_value(ast: &MathAst, forms: &Forms) -> Value {
    match ast {
        MathAst::Numeral {
            text: digits,
            atoms,
        } => adt(0, vec![text(digits), nat_val(atoms.0), nat_val(atoms.1)]),
        MathAst::Leaf { kinds, atoms } => adt(
            1,
            vec![
                list(
                    kinds
                        .iter()
                        .map(|kind| match kind {
                            LeafKind::Form(form) => adt(0, vec![nat_val(forms.known(form))]),
                            LeafKind::Ident(spelled) => adt(1, vec![text(spelled)]),
                        })
                        .collect(),
                ),
                nat_val(atoms.0),
                nat_val(atoms.1),
            ],
        ),
        MathAst::Lexeme { qualified, atoms } => {
            adt(2, vec![text(qualified), nat_val(atoms.0), nat_val(atoms.1)])
        }
        MathAst::Reference {
            module,
            component,
            atoms,
        } => adt(
            3,
            vec![
                text(module),
                text(component),
                nat_val(atoms.0),
                nat_val(atoms.1),
            ],
        ),
        MathAst::Call {
            head,
            args,
            atoms,
            open,
            close,
            commas,
        } => adt(
            4,
            vec![
                math_value(head, forms),
                args.iter().rev().fold(adt(9, Vec::new()), |rest, arg| {
                    adt(10, vec![math_value(arg, forms), rest])
                }),
                nat_val(atoms.0),
                nat_val(atoms.1),
                nat_val(*open),
                nat_val(*close),
                list(commas.iter().map(|comma| nat_val(*comma)).collect()),
            ],
        ),
        MathAst::Prefix {
            candidates,
            op_atoms,
            arg,
        } => adt(
            5,
            vec![
                forms_value(candidates, forms),
                nat_val(op_atoms.0),
                nat_val(op_atoms.1),
                math_value(arg, forms),
            ],
        ),
        MathAst::Postfix {
            candidates,
            op_atoms,
            arg,
        } => adt(
            6,
            vec![
                forms_value(candidates, forms),
                nat_val(op_atoms.0),
                nat_val(op_atoms.1),
                math_value(arg, forms),
            ],
        ),
        MathAst::Infix {
            candidates,
            op_atoms,
            lhs,
            rhs,
        } => adt(
            7,
            vec![
                forms_value(candidates, forms),
                nat_val(op_atoms.0),
                nat_val(op_atoms.1),
                math_value(lhs, forms),
                math_value(rhs, forms),
            ],
        ),
        MathAst::Paren { inner, atoms } => adt(
            8,
            vec![math_value(inner, forms), nat_val(atoms.0), nat_val(atoms.1)],
        ),
    }
}

/// The limits and the counters the budget holds before the parse.
#[derive(Clone, Copy)]
struct Limits {
    edges: u64,
    states: u64,
    depth: u64,
    states_before: u64,
}

const COMMITTED: Limits = Limits {
    edges: MAX_EDGES,
    states: MAX_STATES,
    depth: MAX_DEPTH,
    states_before: 0,
};

/// The run of the root on an atom window and a range of it, and the
/// oracle's code and message when it refuses.
fn run_on(
    path: &str,
    atoms: &[Atom],
    range: (usize, usize),
    limits: Limits,
) -> (Run, Option<String>) {
    let mut forms = Forms::default();
    let edges = edges_value(atoms, &mut forms);
    let mut budget = Budget::new(limits.edges, limits.states, limits.depth, u64::MAX);
    for _ in 0..limits.states_before {
        budget
            .state()
            .expect("the states before are within the limit");
    }
    let (value, refusal) = match parse_math(path, atoms, range, closure(), &visible(), &mut budget)
    {
        Ok(covers) => (
            Value::Ok {
                value: Box::new(list(
                    covers.iter().map(|ast| math_value(ast, &forms)).collect(),
                )),
            },
            None,
        ),
        Err(diagnostic) => (
            Value::Error {
                value: Box::new(rejection_value(&diagnostic)),
            },
            Some(format!(
                "{} {}",
                diagnostic.code.as_str(),
                diagnostic.message
            )),
        ),
    };
    (
        (
            vec![
                text(path),
                list(atoms.iter().map(atom_value).collect()),
                nat_val(range.0),
                nat_val(range.1),
                edges,
                Value::Nat {
                    value: limits.edges.to_string(),
                },
                Value::Nat {
                    value: limits.states.to_string(),
                },
                Value::Nat {
                    value: limits.depth.to_string(),
                },
                Value::Nat {
                    value: limits.states_before.to_string(),
                },
                nat_val(0),
            ],
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

/// One named vector: an island's text, the atoms at its end left out of the
/// range (still given to the root), and the budget.
struct Vector {
    label: &'static str,
    source: &'static str,
    trim: usize,
    limits: Limits,
}

impl Vector {
    fn run(&self) -> (Run, Option<String>) {
        let atoms = scanned(self.source);
        let stop = atoms.len() - self.trim;
        run_on(PATH, &atoms, (0, stop), self.limits)
    }
}

const fn v(label: &'static str, source: &'static str) -> Vector {
    Vector {
        label,
        source,
        trim: 0,
        limits: COMMITTED,
    }
}

const fn limited(label: &'static str, source: &'static str, limits: Limits) -> Vector {
    Vector {
        label,
        source,
        trim: 0,
        limits,
    }
}

const fn trimmed(label: &'static str, source: &'static str, trim: usize) -> Vector {
    Vector {
        label,
        source,
        trim,
        limits: COMMITTED,
    }
}

fn named() -> Vec<Vector> {
    vec![
        v("a numeral", "42"),
        v("a variable", "x"),
        v("a composed identifier", "x_1'"),
        v("white space around", " x "),
        v("white space only", " "),
        v("a sum", "a + b"),
        v("a left chain", "a + b + c"),
        v("an equation", "a = b"),
        v("a nonassociative chain", "a = b = c"),
        v("a right chain", "a → b → c"),
        v("a conjunction", "a ∧ b ∧ c"),
        v("precedence over equality", "a + b = c"),
        v("equality under a sum", "a = b + c"),
        v("a negation", "¬ a"),
        v("a double negation", "¬ ¬ a"),
        v("a negation of an equation", "¬ a = b"),
        v("a negation in a conjunction", "a ∧ ¬ b"),
        v("a postfix", "a !"),
        v("a double postfix", "a ! !"),
        v("a postfix before a sum", "a ! + b"),
        v("a postfix after a sum", "a + b !"),
        v("a postfix of a call", "f(a) !"),
        v("a grouping", "(a + b)"),
        v("a nested grouping", "((a))"),
        v("a grouping and an operator", "(a + b) = c"),
        v("an unclosed grouping", "(a + b"),
        v("a grouping closed by a brace", "(a}"),
        v("an open brace", "{a}"),
        v("a close brace", "}"),
        v("a call", "f(a, b)"),
        v("a call of one argument", "f(a)"),
        v("a call of three arguments", "f(a, b, c)"),
        v("a call of a call", "f(a)(b)"),
        v("a call with nested parentheses", "f((a), g(b, c))"),
        v("a call missing a comma", "f(a b)"),
        v("an unclosed call", "f(a"),
        v("a call of nothing", "f()"),
        v("a call after a grouping", "(f)(a)"),
        v("a lexeme", "\\lexeme{lexlean.core::eq}"),
        v(
            "a lexeme with white space",
            "\\lexeme{ lexlean.core :: eq }",
        ),
        v("a lexeme without a brace", "\\lexeme x"),
        v("an unclosed lexeme", "\\lexeme{abc"),
        v("a reference", "\\reference{M::c}"),
        v("a reference of two separators", "\\reference{M::c::d}"),
        v("a reference without a separator", "\\reference{Mc}"),
        v("a reference called", "\\reference{M::f}(a)"),
        v("an operator alone", "+"),
        v("an operator at the end", "a +"),
        v("two leaves", "a b"),
        v("a comma after the expression", "a ,"),
        v("an unknown symbol after the expression", "a $"),
        v("an unknown symbol first", "$"),
        v("a prefix whose candidates disagree", "tl a"),
        v("operators that disagree", "a ≈ b"),
        v("a prefix that is also a leaf", "bb a"),
        v("a leaf of two atoms", "p q"),
        v("a leaf of two atoms in a sum", "p q + p"),
        v("a leaf of two atoms twice", "p q = p q"),
        v("a leaf of two atoms called", "p q(a)"),
        v("a leaf or a postfix", "p !"),
        trimmed("a sum before the range ends", "a + b c", 2),
        trimmed("an identifier running past the range", "x_y", 2),
        trimmed("a leaf of two atoms past the range", "p q", 2),
        trimmed("an operator past the range", "a + b", 2),
        limited(
            "nesting beyond the limit",
            "((((a))))",
            Limits {
                depth: 3,
                ..COMMITTED
            },
        ),
        limited(
            "negations beyond the limit",
            "¬ ¬ ¬ a",
            Limits {
                depth: 3,
                ..COMMITTED
            },
        ),
        limited(
            "edges at the limit at a branch",
            "p q",
            Limits {
                edges: 1,
                ..COMMITTED
            },
        ),
        limited(
            "nesting at the limit",
            "((a))",
            Limits {
                depth: 3,
                ..COMMITTED
            },
        ),
        limited(
            "a call nested beyond the limit",
            "f(g(h(a)))",
            Limits {
                depth: 3,
                ..COMMITTED
            },
        ),
        limited(
            "states beyond the limit",
            "a + b + c",
            Limits {
                states: 4,
                ..COMMITTED
            },
        ),
        limited(
            "states spent before",
            "a + b",
            Limits {
                states: 6,
                states_before: 3,
                ..COMMITTED
            },
        ),
        limited(
            "edges beyond the limit",
            "a + b + c",
            Limits {
                edges: 1,
                ..COMMITTED
            },
        ),
        limited(
            "edges beyond the limit at a branch",
            "p q + p q",
            Limits {
                edges: 2,
                ..COMMITTED
            },
        ),
        limited(
            "states beyond the limit at a branch",
            "p q",
            Limits {
                states: 3,
                ..COMMITTED
            },
        ),
    ]
}

/// The refusals a vector must reach, by code and message.
const REFUSALS: &[&str] = &[
    "LLP2001 expected a mathematical expression",
    "LLP2004 unclosed grouping parenthesis",
    "LLP2004 braces do not group mathematical terms",
    "LLP2003 expected `{`",
    "LLP2003 unclosed `{` argument",
    "LLR3005 `Mc` is not a `Module::component` reference",
    "LLP2002 prefix candidates disagree on their profile",
    "LLP2002 this surface is both a prefix operator and a leaf",
    "LLP2002 operator candidates at this surface disagree",
    "LLL1004 unknown atom",
    "LLP2004 expected `,` or `)` in a call argument list",
    "LLP2004 a nonassociative chain requires explicit parentheses",
    "LLP2004 unexpected `b` after the expression",
    "LLS8002 max_scope_depth exceeded in phase parse (mathematical grouping)",
    "LLS8002 max_parse_states exceeded in phase parse",
    "LLS8002 max_token_lattice_edges exceeded in phase lexical resolution",
];

// ------------------------------------------------------------------- seeded

const PIECES: &[&str] = &[
    "a",
    "b",
    "x_1",
    "1",
    "+",
    "=",
    "¬",
    "→",
    "∧",
    "(",
    ")",
    ",",
    "!",
    "tl",
    "≈",
    "bb",
    "p q",
    "p",
    "q",
    "f(",
    "\\lexeme{x}",
    "\\reference{M::c}",
    "{",
    "$",
];

fn seeded_source(seeded: &mut Seeded) -> String {
    let count = 1 + seeded.below(9);
    let mut out = String::new();
    for index in 0..count {
        if index > 0 && seeded.chance(80) {
            out.push(' ');
        }
        out.push_str(seeded.pick(PIECES));
    }
    out
}

fn seeded_runs(count: usize, seed: u64) -> Vec<Run> {
    let mut seeded = Seeded(seed);
    let mut out = Vec::with_capacity(count);
    while out.len() < count {
        let source = seeded_source(&mut seeded);
        let Ok(atoms) = lexlean::source::scan::scan(PATH, &source, u64::MAX) else {
            continue;
        };
        if atoms.is_empty() {
            continue;
        }
        out.push(run_on(PATH, &atoms, (0, atoms.len()), COMMITTED).0);
    }
    out
}

/// The islands of the committed sources: the inner range of each `\(…\)` and
/// `\[…\]` of at most [`ISLAND_ATOMS`] atoms, one source scanned at a time,
/// with [`TAIL_ATOMS`] atoms after the range.
fn island_runs(evaluator: usize, rustc: usize) -> (usize, Vec<Run>, Vec<Run>) {
    let mut found = 0usize;
    let mut first = Vec::new();
    let mut second = Vec::new();
    for (path, content) in super::scan::sources() {
        if content.len() > 400_000 || second.len() >= rustc {
            continue;
        }
        let Ok(atoms) = lexlean::source::scan::scan(&path, &content, u64::MAX) else {
            continue;
        };
        let mut index = 0;
        while index < atoms.len() && second.len() < rustc {
            let atom = &atoms[index];
            let closer = match (atom.class, atom.text.as_str()) {
                (AtomClass::Control, "\\(") => "\\)",
                (AtomClass::Control, "\\[") => "\\]",
                _ => {
                    index += 1;
                    continue;
                }
            };
            let Some(close) = (index + 1..atoms.len().min(index + 2 + ISLAND_ATOMS))
                .find(|at| atoms[*at].class == AtomClass::Control && atoms[*at].text == closer)
            else {
                index += 1;
                continue;
            };
            if close > index + 1 {
                let window = &atoms[index + 1..(close + TAIL_ATOMS).min(atoms.len())];
                let run = run_on(&path, window, (0, close - index - 1), COMMITTED).0;
                found += 1;
                if first.len() < evaluator && found.is_multiple_of(3) {
                    first.push(run.clone());
                }
                second.push(run);
            }
            index = close + 1;
        }
    }
    (found, first, second)
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "left associativity binds to the right",
        site: "(let next : Nat := (if (associativity =? 2) then precedence else (precedence + 1));",
        replacement: "(let next : Nat := precedence;",
        vector: "a left chain",
    },
    Plant {
        label: "right associativity binds to the left",
        site: "(let next : Nat := (if (associativity =? 2) then precedence else (precedence + 1));",
        replacement: "(let next : Nat := (precedence + 1);",
        vector: "a right chain",
    },
    Plant {
        label: "a nonassociative chain is accepted",
        site: "(match last\n        | Option.some held => (held =? precedence)",
        replacement: "(match (ctor Option.none [Nat])\n        | Option.some held => (held =? precedence)",
        vector: "a nonassociative chain",
    },
    Plant {
        label: "a lower precedence still binds",
        site: "(if (precedence <? minBp) then charged else",
        replacement: "(if false then charged else",
        vector: "precedence over equality",
    },
    Plant {
        label: "a postfix is an infix",
        site: "(if ((project head frame) =? 3) then (mk charged",
        replacement: "(if ((project head frame) =? 9) then (mk charged",
        vector: "a postfix",
    },
    Plant {
        label: "operator profiles are not compared",
        site: "(if (profileAgrees head operators true) then",
        replacement: "(if true then",
        vector: "operators that disagree",
    },
    Plant {
        label: "prefix profiles are not compared",
        site: "(if (profileAgrees head prefixes false) then",
        replacement: "(if true then",
        vector: "a prefix whose candidates disagree",
    },
    Plant {
        label: "a prefix that is a leaf is a prefix",
        site: "(if ((prim length leaves : Nat) =? 0) then (mk charged",
        replacement: "(if true then (mk charged",
        vector: "a prefix that is also a leaf",
    },
    Plant {
        label: "the prefix operand is one level too shallow",
        site: "(cons (ctor Frame.FExpr (project head precedence) (depth + 1))",
        replacement: "(cons (ctor Frame.FExpr (project head precedence) depth)",
        vector: "negations beyond the limit",
    },
    Plant {
        label: "a grouping is not nested",
        site: "(cons (ctor Frame.FExpr 0 (depth + 1)) (cons (ctor Frame.FParen position depth) rest))",
        replacement: "(cons (ctor Frame.FExpr 0 depth) (cons (ctor Frame.FParen position depth) rest))",
        vector: "nesting beyond the limit",
    },
    Plant {
        label: "the depth limit admits one more",
        site: "(if ((project ctx maxDepth) <? depth) then",
        replacement: "(if (((project ctx maxDepth) + 1) <? depth) then",
        vector: "nesting beyond the limit",
    },
    Plant {
        label: "a primary charges no state",
        site: "(let charge : (Bud * (Option PrimitiveScan::Rejection)) := (chargeState ctx (project m bud)); (let counted : Mach := (withBud m (first charge)); (match (second charge)\n    | Option.some problem => (failed counted problem)\n    | Option.none => (if ((project ctx maxDepth)",
        replacement: "(let charge : (Bud * (Option PrimitiveScan::Rejection)) := ((project m bud), (ctor Option.none [PrimitiveScan::Rejection])); (let counted : Mach := (withBud m (first charge)); (match (second charge)\n    | Option.some problem => (failed counted problem)\n    | Option.none => (if ((project ctx maxDepth)",
        vector: "states beyond the limit",
    },
    Plant {
        label: "edges are counted again at a known position",
        site: "(if (natIn position (project bud memo)) then",
        replacement: "(if false then",
        vector: "edges at the limit at a branch",
    },
    Plant {
        label: "the shortest extent is not the first pass",
        site: "(let upto : Nat := (match (chosenAt ctx position)\n      | Option.some picked => (if (natIn picked ends) then picked else shortest)\n      | Option.none => shortest);",
        replacement: "(let upto : Nat := (match (chosenAt ctx position)\n      | Option.some picked => (if (natIn picked ends) then picked else shortest)\n      | Option.none => (match (prim index ends 1 : (Option Nat))\n        | Option.some longer => longer\n        | Option.none => shortest));",
        vector: "a leaf or a postfix",
    },
    Plant {
        label: "a branch point is not recorded",
        site: "(let met : (List Met) := (if (1 <? (prim length ends : Nat)) then",
        replacement: "(let met : (List Met) := (if false then",
        vector: "a leaf of two atoms",
    },
    Plant {
        label: "the last failure stands",
        site: "| Option.some earlier => (project state failure)",
        replacement: "| Option.some earlier => (ctor Option.some [PrimitiveScan::Rejection] problem)",
        vector: "states beyond the limit at a branch",
    },
    Plant {
        label: "an identifier may run past the range",
        site: "(if ((project composed next) <=? stop) then (ctor Option.some [Composed] composed)",
        replacement: "(if true then (ctor Option.some [Composed] composed)",
        vector: "an identifier running past the range",
    },
    Plant {
        label: "a quote does not compose",
        site: "(if (same (project atom text) (lit \"_\")) then true else (same (project atom text) (lit \"'\")))",
        replacement: "(same (project atom text) (lit \"_\"))",
        vector: "a composed identifier",
    },
    Plant {
        label: "an operator may run past the range",
        site: "(prim list_fold (fun [high, low, stop] (out : (List Edge)) (edge : Edge) => (prim append out (if (both ((project edge stop) <=? stop)",
        replacement: "(prim list_fold (fun [high, low] (out : (List Edge)) (edge : Edge) => (prim append out (if (both true",
        vector: "a leaf of two atoms past the range",
    },
    Plant {
        label: "the call's close is the open",
        site: "(callVal head (reverseVals gathered) (position + 1) opening position (reverseNats commas))",
        replacement: "(callVal head (reverseVals gathered) (position + 1) opening opening (reverseNats commas))",
        vector: "a call",
    },
    Plant {
        label: "the commas are kept in reverse",
        site: "(callVal head (reverseVals gathered) (position + 1) opening position (reverseNats commas))",
        replacement: "(callVal head (reverseVals gathered) (position + 1) opening position commas)",
        vector: "a call of three arguments",
    },
    Plant {
        label: "a call is not continued",
        site: "(if (both (position <? (project ctx stop)) (isAtom (atomOr ctx position) 3 (lit \"(\"))) then (mk m (position + 1)",
        replacement: "(if false then (mk m (position + 1)",
        vector: "a call of a call",
    },
    Plant {
        label: "white space in a lexeme is kept",
        site: "(if ((PrimitiveScan::classCode atom) =? 6) then (ctor Option.some [Braced] (record Braced {cursor := (position + 1), spelled := (project state spelled), closed := false}))",
        replacement: "(if false then (ctor Option.some [Braced] (record Braced {cursor := (position + 1), spelled := (project state spelled), closed := false}))",
        vector: "a lexeme with white space",
    },
    Plant {
        label: "a reference splits at its end",
        site: "(let component : Bytes := (LexLeanStd.Text::sliceOr spelled (split + 2)",
        replacement: "(let component : Bytes := (LexLeanStd.Text::sliceOr spelled (split + 1)",
        vector: "a reference",
    },
    Plant {
        label: "braces group",
        site: "(if (if (isAtom atom 3 (lit \"{\")) then true else (isAtom atom 3 (lit \"}\"))) then",
        replacement: "(if (isAtom atom 3 (lit \"}\")) then",
        vector: "an open brace",
    },
    Plant {
        label: "the end of the range is the last atom",
        site: "(placeAt ctx (prim subtract (project ctx stop) 1 : Nat))",
        replacement: "(placeAt ctx (project ctx stop))",
        vector: "an operator at the end",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
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
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, &[vector.run().0], 1).first() {
            Some(first) => eprintln!(
                "BS-42: plant detected: {}: {}",
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
            "BS-42: {}: largest certificate part {largest} bytes",
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

/// The kinds of node a cover holds.
fn shapes(ast: &MathAst, into: &mut BTreeSet<&'static str>) {
    match ast {
        MathAst::Numeral { .. } => {
            into.insert("numeral");
        }
        MathAst::Leaf { kinds, .. } => {
            into.insert("leaf");
            for kind in kinds {
                into.insert(match kind {
                    LeafKind::Form(_) => "form leaf",
                    LeafKind::Ident(_) => "identifier leaf",
                });
            }
        }
        MathAst::Lexeme { .. } => {
            into.insert("lexeme");
        }
        MathAst::Reference { .. } => {
            into.insert("reference");
        }
        MathAst::Call { head, args, .. } => {
            into.insert("call");
            shapes(head, into);
            for arg in args {
                shapes(arg, into);
            }
        }
        MathAst::Prefix { arg, .. } => {
            into.insert("prefix");
            shapes(arg, into);
        }
        MathAst::Postfix { arg, .. } => {
            into.insert("postfix");
            shapes(arg, into);
        }
        MathAst::Infix { lhs, rhs, .. } => {
            into.insert("infix");
            shapes(lhs, into);
            shapes(rhs, into);
        }
        MathAst::Paren { inner, .. } => {
            into.insert("paren");
            shapes(inner, into);
        }
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("GrammarMath"),
        "GrammarMath is a linked module of the project"
    );
    eligibility_of(&project, "GrammarMath", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut runs: Vec<Run> = Vec::new();
    let mut messages: Vec<String> = Vec::new();
    let mut nodes = BTreeSet::new();
    let mut several = false;
    for vector in &vectors {
        let atoms = scanned(vector.source);
        let stop = atoms.len() - vector.trim;
        let mut budget = Budget::new(
            vector.limits.edges,
            vector.limits.states,
            vector.limits.depth,
            u64::MAX,
        );
        for _ in 0..vector.limits.states_before {
            budget.state().expect("within the limit");
        }
        if let Ok(covers) = parse_math(PATH, &atoms, (0, stop), closure(), &visible(), &mut budget)
        {
            several |= covers.len() > 1;
            for cover in &covers {
                shapes(cover, &mut nodes);
            }
        }
        let (run, refusal) = vector.run();
        messages.extend(refusal);
        runs.push(run);
    }
    for refusal in REFUSALS {
        assert!(
            messages.iter().any(|message| message.starts_with(refusal)),
            "no vector reaches the refusal `{refusal}`"
        );
    }
    for shape in [
        "numeral",
        "leaf",
        "form leaf",
        "identifier leaf",
        "lexeme",
        "reference",
        "call",
        "prefix",
        "postfix",
        "infix",
        "paren",
    ] {
        assert!(nodes.contains(shape), "no vector's cover holds a {shape}");
    }
    assert!(several, "a vector has more than one cover");

    let (islands, evaluator_islands, rustc_islands) = island_runs(EVALUATOR_ISLANDS, RUSTC_ISLANDS);
    assert!(islands >= 100, "the committed islands are {islands}");
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, 0x0B55_0042),
            seeded_runs(RUSTC_SEEDED, 0x0B55_1042),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-42: {} vectors, {} committed islands ({} and {} run), {} and {} seeded islands ({:?})",
        vectors.len(),
        islands,
        evaluator_islands.len(),
        rustc_islands.len(),
        evaluator_seeded.len(),
        rustc_seeded.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        let mut corpus: Vec<Run> = runs.clone();
        corpus.extend(evaluator_islands);
        corpus.extend(evaluator_seeded);
        let disagreements = super::evaluate(&lowered[ROOTS[0].0], &corpus, 5);
        assert!(
            disagreements.is_empty(),
            "parseMath and the Rust oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-42: parseMath: the evaluator agrees on {} runs ({:?})",
            corpus.len(),
            started.elapsed()
        );
        plants(&vectors);
    } else {
        eprintln!("BS-42: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled = runs;
    compiled.extend(rustc_islands);
    compiled.extend(rustc_seeded);
    let total = rustc_differential(
        "BS-42",
        &[(ROOTS[0].0, &lowered[ROOTS[0].0], compiled.as_slice())],
        "oracle",
    );
    eprintln!("BS-42: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-42") {
        return;
    }
    export(&lowered);
    eprintln!("BS-42: passed in {:?}", started.elapsed());
}
