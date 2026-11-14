//! `BS-43` (SPEC.md §17.23, §15.3, §15.4, §12.2): the grammar keywords, the
//! unknown-atom diagnosis, the binders, and the term phrases of the text
//! channel as language-1.2 source, `LexLeanBoot.PropositionTerms`, held to the
//! byte to `lexlean::grammar::proposition::keyword_entry` and the
//! `TextParser` methods `unknown_atom_diagnostic`, `binder`, and `term_phrase`.
//!
//! The roots read the tokens of `BS-35` (`GrammarProof`) and the atoms of
//! `BS-03`; the glossary closure is the lexicon's, so the host gives them its
//! projection: the text-channel edges at each atom position, each with its
//! form's category and whether the form is inflected (plural or
//! sentence-case), and the limits and counters of the parse budget. The term
//! phrase, whose arguments are term phrases, is an explicit stack of frames
//! that gathers every alternative in the oracle's order.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::grammar::chart::{text_tokens, Budget, TextToken};
use lexlean::grammar::proposition::{
    keyword_entry, ArticleRule, BinderAst, Keyword, TermPhraseAst, TextParser, TypePhraseAst,
};
use lexlean::lexicon::entry::{Category, Channel};
use lexlean::lexicon::package::LoadContext;
use lexlean::lexicon::resolve::{Closure, FormRef};
use lexlean::source::atom::{Atom, AtomClass};
use serde_json::Value as Json;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::scan::{atom_value, rejection_value};
use super::{bool_val, list, nat_val, option, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.PropositionTerms";
const SOURCE: &str = "src/PropositionTerms.lex.tex";
const ROOTS: [(&str, &[&str]); 4] = [
    ("keywordEntry", &["allocation"]),
    ("unknownAtom", &["allocation", "overflow"]),
    ("binderAlternatives", &["allocation", "overflow"]),
    ("termPhrase", &["allocation", "overflow"]),
];
/// The glossary package of the case's closure beside the built-in ones: the
/// committed package of the Peano example, whose noun-of and binary-noun-of
/// forms (`the successor of`, `the sum of … and …`) the term phrases read.
const GLOSSARY: &str = "examples/peano-arithmetic/glossary/peano.arith";
const VISIBLE: [&str; 3] = ["lexlean.core", "lexlean.std.nat", "peano.arith"];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
const PATH: &str = "src/Prose.lex.tex";
const MAX_EDGES: u64 = 4_000_000;
const MAX_STATES: u64 = 4_000_000;
const MAX_DEPTH: u64 = 1_024;
/// Sentences of the committed sources: at most this many tokens, and how many
/// under the evaluator and under rustc.
const SENTENCE_TOKENS: usize = 24;
const EVALUATOR_SENTENCES: usize = 150;
const RUSTC_SENTENCES: usize = 1_500;
const EVALUATOR_SEEDED: usize = 200;
const RUSTC_SEEDED: usize = 2_000;

// ------------------------------------------------------------ the closure

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
        let root = repo_root().join(GLOSSARY);
        let mut files: Vec<(String, Vec<u8>)> = walkdir::WalkDir::new(root.as_std_path())
            .into_iter()
            .flatten()
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| {
                let relative = entry
                    .path()
                    .strip_prefix(root.as_std_path())
                    .expect("under the package")
                    .to_string_lossy()
                    .replace('\\', "/");
                (
                    relative,
                    std::fs::read(entry.path()).expect("a glossary file"),
                )
            })
            .filter(|(relative, _)| relative == "lexicon.toml" || relative.starts_with("entries/"))
            .collect();
        files.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
        packages.push(
            lexlean::lexicon::package::load_package(GLOSSARY, &files, None, &ctx)
                .unwrap_or_else(|diagnostics| panic!("the Peano glossary: {diagnostics:?}")),
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

fn category_code(category: Category) -> usize {
    match category {
        Category::TypeNoun => 0,
        Category::NounFunction => 1,
        Category::BinaryNounFunction => 2,
        Category::AdjectivePredicate => 3,
        Category::IntransitivePredicate => 4,
        Category::TransitivePredicate => 5,
        _ => 8,
    }
}

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

/// The text-channel edges at each atom position, each with its form's
/// category (9 for a form the closure cannot resolve, which no rule takes and
/// which is still counted) and whether the form is plural or sentence-case.
fn edges_value(atoms: &[Atom], forms: &mut Forms) -> Value {
    let closure = closure();
    let visible = visible();
    list(
        (0..atoms.len())
            .map(|position| {
                list(
                    closure
                        .matches_at(atoms, position, Channel::Text, &visible)
                        .iter()
                        .map(|(form, end)| {
                            let (category, inflected) =
                                closure.form(form).map_or((9, false), |(entry, shape)| {
                                    (
                                        category_code(entry.category),
                                        shape.features.iter().any(|feature| {
                                            feature == "plural" || feature == "sentence-case"
                                        }),
                                    )
                                });
                            Value::Adt {
                                constructor: 0,
                                fields: vec![
                                    nat_val(forms.index(form)),
                                    nat_val(*end),
                                    nat_val(category),
                                    bool_val(inflected),
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

fn token_value(token: &TextToken) -> Value {
    match token {
        TextToken::Atom(index) => adt(0, vec![nat_val(*index)]),
        TextToken::Island {
            open,
            inner_start,
            inner_end,
            close,
            display,
        } => adt(
            1,
            vec![
                nat_val(*open),
                nat_val(*inner_start),
                nat_val(*inner_end),
                nat_val(*close),
                bool_val(*display),
            ],
        ),
    }
}

fn keywords_value(keywords: &[Keyword]) -> Value {
    list(
        keywords
            .iter()
            .map(|keyword| adt(0, vec![nat_val(keyword.atom), text(keyword.entry)]))
            .collect(),
    )
}

fn binder_value(binder: &BinderAst, forms: &Forms) -> Value {
    let phrase = match &binder.type_phrase {
        TypePhraseAst::Noun { candidates, atoms } => adt(
            0,
            vec![
                list(
                    candidates
                        .iter()
                        .map(|form| nat_val(forms.known(form)))
                        .collect(),
                ),
                nat_val(atoms.0),
                nat_val(atoms.1),
            ],
        ),
        TypePhraseAst::Math(token) => adt(1, vec![token_value(token)]),
    };
    adt(
        0,
        vec![
            phrase,
            text(&binder.spelling),
            token_value(&binder.island),
            nat_val(binder.ident_atoms.0),
            nat_val(binder.ident_atoms.1),
            keywords_value(&binder.keywords),
        ],
    )
}

fn term_value(term: &TermPhraseAst, forms: &Forms) -> Value {
    match term {
        TermPhraseAst::Island(token) => adt(0, vec![token_value(token)]),
        TermPhraseAst::NounOf {
            candidates,
            surface_atoms,
            args,
            keywords,
        } => adt(
            1,
            vec![
                list(
                    candidates
                        .iter()
                        .map(|form| nat_val(forms.known(form)))
                        .collect(),
                ),
                nat_val(surface_atoms.0),
                nat_val(surface_atoms.1),
                args.iter().rev().fold(adt(2, Vec::new()), |rest, arg| {
                    adt(3, vec![term_value(arg, forms), rest])
                }),
                keywords_value(keywords),
            ],
        ),
    }
}

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

/// What a root is asked.
#[derive(Clone, Copy)]
enum Ask {
    Unknown,
    Binder(usize, bool),
    Term(usize, u64),
}

impl Ask {
    fn root(self) -> usize {
        match self {
            Self::Unknown => 1,
            Self::Binder(..) => 2,
            Self::Term(..) => 3,
        }
    }
}

fn nat_u64(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

/// The run of a root on a sentence's atoms and tokens, and the oracle's code
/// and message when it refuses.
fn run_on(
    path: &str,
    atoms: &[Atom],
    tokens: &[TextToken],
    ask: Ask,
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
    let visible = visible();
    let parser = TextParser {
        path,
        atoms,
        tokens,
        closure: closure(),
        visible: &visible,
        sentence_initial: true,
    };
    let refused = |diagnostic: lexlean::diagnostic::Diagnostic| {
        (
            Value::Error {
                value: Box::new(rejection_value(&diagnostic)),
            },
            Some(format!(
                "{} {}",
                diagnostic.code.as_str(),
                diagnostic.message
            )),
        )
    };
    let (value, refusal) = match ask {
        Ask::Unknown => match parser.unknown_atom_diagnostic(&mut budget) {
            Ok(found) => {
                let message = found.as_ref().map(|diagnostic| {
                    format!("{} {}", diagnostic.code.as_str(), diagnostic.message)
                });
                (
                    Value::Ok {
                        value: Box::new(option(found.as_ref().map(rejection_value))),
                    },
                    message,
                )
            }
            Err(diagnostic) => refused(diagnostic),
        },
        Ask::Binder(position, required) => {
            let article = if required {
                ArticleRule::Required
            } else {
                ArticleRule::Forbidden
            };
            match parser.binder(position, &mut budget, article) {
                Ok(alternatives) => (
                    Value::Ok {
                        value: Box::new(list(
                            alternatives
                                .iter()
                                .map(|(end, binder)| {
                                    adt(0, vec![nat_val(*end), binder_value(binder, &forms)])
                                })
                                .collect(),
                        )),
                    },
                    None,
                ),
                Err(diagnostic) => refused(diagnostic),
            }
        }
        Ask::Term(position, depth) => match parser.term_phrase(position, &mut budget, depth) {
            Ok(alternatives) => (
                Value::Ok {
                    value: Box::new(list(
                        alternatives
                            .iter()
                            .map(|(end, term)| {
                                adt(0, vec![nat_val(*end), term_value(term, &forms)])
                            })
                            .collect(),
                    )),
                },
                None,
            ),
            Err(diagnostic) => refused(diagnostic),
        },
    };
    let mut arguments = vec![
        text(path),
        list(atoms.iter().map(atom_value).collect()),
        list(tokens.iter().map(token_value).collect()),
        edges,
        nat_u64(limits.edges),
        nat_u64(limits.states),
        nat_u64(limits.depth),
        nat_u64(limits.states_before),
        nat_val(0),
    ];
    match ask {
        Ask::Unknown => {}
        Ask::Binder(position, required) => {
            arguments.push(nat_val(position));
            arguments.push(bool_val(required));
        }
        Ask::Term(position, depth) => {
            arguments.push(nat_val(position));
            arguments.push(nat_u64(depth));
        }
    }
    (
        (
            arguments,
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

fn keyword_run(word: &str) -> Run {
    (
        vec![text(word)],
        serde_json::to_value(option(keyword_entry(word).map(text))).expect("a value is JSON"),
    )
}

fn sentence(source: &str) -> (Vec<Atom>, Vec<TextToken>) {
    let atoms = lexlean::source::scan::scan(PATH, source, u64::MAX)
        .unwrap_or_else(|failure| panic!("{source:?}: {failure:?}"));
    let tokens = text_tokens(PATH, &atoms, 0, atoms.len())
        .unwrap_or_else(|failure| panic!("{source:?}: {failure:?}"));
    (atoms, tokens)
}

// ------------------------------------------------------------------ vectors

struct Vector {
    label: &'static str,
    source: &'static str,
    ask: Ask,
    limits: Limits,
}

impl Vector {
    fn run(&self) -> (Run, Option<String>) {
        let (atoms, tokens) = sentence(self.source);
        run_on(PATH, &atoms, &tokens, self.ask, self.limits)
    }
}

const fn term(label: &'static str, source: &'static str, position: usize) -> Vector {
    Vector {
        label,
        source,
        ask: Ask::Term(position, 1),
        limits: COMMITTED,
    }
}

const fn binder(
    label: &'static str,
    source: &'static str,
    position: usize,
    required: bool,
) -> Vector {
    Vector {
        label,
        source,
        ask: Ask::Binder(position, required),
        limits: COMMITTED,
    }
}

const fn unknown(label: &'static str, source: &'static str) -> Vector {
    Vector {
        label,
        source,
        ask: Ask::Unknown,
        limits: COMMITTED,
    }
}

const fn limited(label: &'static str, source: &'static str, ask: Ask, limits: Limits) -> Vector {
    Vector {
        label,
        source,
        ask,
        limits,
    }
}

fn named() -> Vec<Vector> {
    vec![
        term("an island", "\\(n\\)", 0),
        term("a word", "zero", 0),
        term("the successor", "the successor of \\(n\\)", 0),
        term(
            "the successor of the successor",
            "the successor of the successor of \\(n\\)",
            0,
        ),
        term("the sum", "the sum of \\(a\\) and \\(b\\)", 0),
        term(
            "the sum of sums",
            "the sum of the sum of \\(a\\) and \\(b\\) and \\(c\\)",
            0,
        ),
        term(
            "the sum of a successor",
            "the sum of the successor of \\(a\\) and \\(b\\)",
            0,
        ),
        term("the sum without and", "the sum of \\(a\\) or \\(b\\)", 0),
        term("the sum of one argument", "the sum of \\(a\\)", 0),
        term("the successor without of", "the successor \\(n\\)", 0),
        term("the successor of nothing", "the successor of", 0),
        term("the without a noun", "the \\(n\\)", 0),
        term("a capital the", "The successor of \\(n\\)", 0),
        term("the at the end", "the", 0),
        term(
            "a phrase later in the sentence",
            "\\(m\\) is the successor of \\(n\\)",
            2,
        ),
        term("a position past the tokens", "\\(n\\)", 3),
        limited(
            "term phrases nested beyond the limit",
            "the successor of the successor of the successor of \\(n\\)",
            Ask::Term(0, 1),
            Limits {
                depth: 2,
                ..COMMITTED
            },
        ),
        limited(
            "a term phrase at a depth beyond the limit",
            "\\(n\\)",
            Ask::Term(3, 5),
            Limits {
                depth: 4,
                ..COMMITTED
            },
        ),
        limited(
            "term phrases beyond the states",
            "the sum of \\(a\\) and \\(b\\)",
            Ask::Term(0, 1),
            Limits {
                states: 3,
                ..COMMITTED
            },
        ),
        limited(
            "term phrases spending states before",
            "the successor of \\(n\\)",
            Ask::Term(0, 1),
            Limits {
                states: 4,
                states_before: 2,
                ..COMMITTED
            },
        ),
        limited(
            "term phrases beyond the edges",
            "the successor of the successor of \\(n\\)",
            Ask::Term(0, 1),
            Limits {
                edges: 1,
                ..COMMITTED
            },
        ),
        binder("a binder of a noun", "natural number \\(n\\)", 0, false),
        binder(
            "a binder of a noun of two words",
            "natural number list \\(xs\\)",
            0,
            false,
        ),
        binder("a binder of an island", "\\(T\\) \\(x\\)", 0, false),
        binder(
            "a binder with an article",
            "a natural number \\(n\\)",
            0,
            true,
        ),
        binder("a binder with an", "an natural number \\(n\\)", 0, true),
        binder(
            "a binder without its article",
            "natural number \\(n\\)",
            0,
            true,
        ),
        binder(
            "a binder with a forbidden article",
            "a natural number \\(n\\)",
            0,
            false,
        ),
        binder(
            "a binder of a plural noun",
            "natural number lists \\(xs\\)",
            0,
            false,
        ),
        binder(
            "a binder of two identifiers",
            "natural number \\(n m\\)",
            0,
            false,
        ),
        binder(
            "a binder of a composed identifier",
            "natural number \\( n_1' \\)",
            0,
            false,
        ),
        binder("a binder of a numeral", "natural number \\(1\\)", 0, false),
        binder("a binder of nothing", "natural number \\(\\)", 0, false),
        binder("a binder without an island", "natural number n", 0, false),
        binder(
            "a binder of a display island",
            "natural number \\[n\\]",
            0,
            false,
        ),
        binder(
            "a binder later in the sentence",
            "for every natural number \\(n\\)",
            2,
            false,
        ),
        limited(
            "a binder beyond the states",
            "natural number \\(n\\)",
            Ask::Binder(0, false),
            Limits {
                states: 0,
                ..COMMITTED
            },
        ),
        limited(
            "a binder beyond the edges",
            "natural number \\(n\\)",
            Ask::Binder(0, false),
            Limits {
                edges: 0,
                ..COMMITTED
            },
        ),
        unknown("no unknown atom", "the successor of \\(n\\) is positive"),
        unknown("an unknown word", "the frobnicator of \\(n\\)"),
        unknown("an unknown symbol", "\\(n\\) ¤ \\(m\\)"),
        unknown("a proof keyword is known", "Assume \\(n\\)"),
        unknown("a grammar keyword is known", "for every \\(n\\)"),
        unknown("a word inside a longer form", "natural number list"),
        unknown("islands only", "\\(n\\) \\(m\\)"),
        unknown(
            "the second unknown word is not reported",
            "frobnicate and twiddle",
        ),
        limited(
            "the unknown atom beyond the edges",
            "natural number frobnicate",
            Ask::Unknown,
            Limits {
                edges: 1,
                ..COMMITTED
            },
        ),
    ]
}

const KEYWORDS: &[&str] = &[
    "a", "A", "an", "An", "and", "as", "defined", "every", "exactly", "exists", "for", "For",
    "holds", "if", "If", "implies", "is", "not", "Not", "one", "only", "or", "such", "that", "the",
    "The", "then", "there", "There", "when", "of", "AND", "Every", "Of", "thE", "", "x", "ands",
    "of ", "Then",
];

const REFUSALS: &[&str] = &[
    "LLS8002 max_scope_depth exceeded in phase parse (proposition nesting)",
    "LLS8002 max_parse_states exceeded in phase parse",
    "LLS8002 max_token_lattice_edges exceeded in phase lexical resolution",
    "LLL1004 `frobnicator` is not a declared atom in any visible glossary",
    "LLL1004 `¤` is not a declared atom in any visible glossary",
];

// ------------------------------------------------------------------- seeded

const PIECES: &[&str] = &[
    "the",
    "The",
    "successor",
    "sum",
    "of",
    "and",
    "a",
    "an",
    "natural",
    "number",
    "list",
    "lists",
    "\\(n\\)",
    "\\(a b\\)",
    "\\(\\)",
    "is",
    "frobnicate",
    ";",
    "for",
    "every",
];

fn seeded_runs(count: usize, seed: u64) -> Vec<(usize, Run)> {
    let mut seeded = Seeded(seed);
    let mut out = Vec::with_capacity(count);
    while out.len() < count {
        let pieces = 1 + seeded.below(9);
        let source: Vec<&str> = (0..pieces).map(|_| *seeded.pick(PIECES)).collect();
        let source = source.join(" ");
        let Ok(atoms) = lexlean::source::scan::scan(PATH, &source, u64::MAX) else {
            continue;
        };
        let Ok(tokens) = text_tokens(PATH, &atoms, 0, atoms.len()) else {
            continue;
        };
        let position = seeded.below(tokens.len() + 1);
        let ask = match seeded.below(3) {
            0 => Ask::Unknown,
            1 => Ask::Binder(position, seeded.chance(50)),
            _ => Ask::Term(position, 1),
        };
        out.push((ask.root(), run_on(PATH, &atoms, &tokens, ask, COMMITTED).0));
    }
    out
}

/// Runs of the case, each with the index of the root it runs.
type RootRuns = Vec<(usize, Run)>;

/// The sentences of the committed sources (each run of atoms between two
/// periods that tokenizes, of at most [`SENTENCE_TOKENS`] tokens), each asked
/// for its unknown atom, and for a binder and a term phrase at every token.
fn sentence_runs(evaluator: usize, rustc: usize) -> (usize, RootRuns, RootRuns) {
    let mut found = 0usize;
    let mut first = Vec::new();
    let mut second = Vec::new();
    for (path, content) in super::scan::sources() {
        if content.len() > 400_000 || found >= rustc {
            continue;
        }
        let Ok(atoms) = lexlean::source::scan::scan(&path, &content, u64::MAX) else {
            continue;
        };
        let mut start = 0;
        while start < atoms.len() && found < rustc {
            let end = (start..atoms.len())
                .find(|index| {
                    atoms[*index].class == AtomClass::AsciiSymbol && atoms[*index].text == "."
                })
                .unwrap_or(atoms.len());
            let window = &atoms[start..end];
            start = end + 1;
            let Ok(tokens) = text_tokens(&path, window, 0, window.len()) else {
                continue;
            };
            if tokens.is_empty() || tokens.len() > SENTENCE_TOKENS {
                continue;
            }
            let mut asks = vec![Ask::Unknown];
            for position in 0..tokens.len() {
                asks.push(Ask::Term(position, 1));
                asks.push(Ask::Binder(position, position % 2 == 0));
            }
            found += 1;
            for ask in asks {
                let run = run_on(&path, window, &tokens, ask, COMMITTED).0;
                if first.len() < evaluator && found.is_multiple_of(7) {
                    first.push((ask.root(), run.clone()));
                }
                second.push((ask.root(), run));
            }
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
    /// For a plant of `keywordEntry`, the word its run asks about (its
    /// vector is then `keyword`).
    word: Option<&'static str>,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "the capital a is no keyword",
        site: "(if (if (same word (lit \"a\")) then true else (same word (lit \"A\"))) then",
        replacement: "(if (same word (lit \"a\")) then",
        vector: "keyword",
        word: Some("A"),
    },
    Plant {
        label: "of names the entry the",
        site: "(if (same word (lit \"of\")) then (ctor Option.some [Bytes] (lit \"of\"))",
        replacement: "(if (same word (lit \"of\")) then (ctor Option.some [Bytes] (lit \"the\"))",
        vector: "keyword",
        word: Some("of"),
    },
    Plant {
        label: "a form covers only its first atom",
        site: "(both (start <=? index) (index <? (project edge stop)))",
        replacement: "(both (start =? index) (index <? (project edge stop)))",
        vector: "a word inside a longer form",
        word: None,
    },
    Plant {
        label: "the scan for a cover starts at the atom",
        site: "(rangeStart, (record Scan {bud := (project state bud)",
        replacement: "(index, (record Scan {bud := (project state bud)",
        vector: "a word inside a longer form",
        word: None,
    },
    Plant {
        label: "every unknown word is reported last",
        site: "| Option.some hit => state\n    | Option.none => (match (project state error)",
        replacement: "| Option.some hit => (record Scan {bud := (project state bud), found := (ctor Option.none [PrimitiveScan::Rejection]), error := (project state error)})\n    | Option.none => (match (project state error)",
        vector: "the second unknown word is not reported",
        word: None,
    },
    Plant {
        label: "the article is optional",
        site: "(if required then (prim append",
        replacement: "(if false then (prim append",
        vector: "a binder with an article",
        word: None,
    },
    Plant {
        label: "an is no article",
        site: "(match (isWord ctx position (lit \"an\"))",
        replacement: "(match (isWord ctx position (lit \"a\"))",
        vector: "a binder with an",
        word: None,
    },
    Plant {
        label: "a plural noun binds",
        site: "(if (project found inflected) then out else",
        replacement: "(if false then out else",
        vector: "a binder of a plural noun",
        word: None,
    },
    Plant {
        label: "a binder identifier may be followed",
        site: "(if ((skipWs ctx innerEnd identEnd) =? innerEnd) then",
        replacement: "(if true then",
        vector: "a binder of two identifiers",
        word: None,
    },
    Plant {
        label: "a binder identifier may be a numeral",
        site: "(if (both ((PrimitiveScan::classCode atom) =? 1) (index <? (prim length (project ctx atoms) : Nat))) then",
        replacement: "(if (index <? (prim length (project ctx atoms) : Nat)) then",
        vector: "a binder of a numeral",
        word: None,
    },
    Plant {
        label: "a quote ends an identifier",
        site: "(if (same (project atom text) (lit \"_\")) then true else (same (project atom text) (lit \"'\")))",
        replacement: "(same (project atom text) (lit \"_\"))",
        vector: "a binder of a composed identifier",
        word: None,
    },
    Plant {
        label: "an island is no type phrase",
        site: "| Option.some token => (match (binderLocal ctx ((first start) + 1) (ctor TypePhrase.TMath token))",
        replacement: "| Option.some token => (match (ctor Option.none [BAlt])",
        vector: "a binder of an island",
        word: None,
    },
    Plant {
        label: "the article keyword is dropped",
        site: "(prim append keywords (project held keywords) : (List GrammarProof::Keyword))",
        replacement: "(project held keywords)",
        vector: "a binder with an article",
        word: None,
    },
    Plant {
        label: "a binder charges no state",
        site: "(let charge : (Bud * (Option PrimitiveScan::Rejection)) := (chargeState ctx (fresh statesBefore edgesBefore));",
        replacement: "(let charge : (Bud * (Option PrimitiveScan::Rejection)) := ((fresh statesBefore edgesBefore), (ctor Option.none [PrimitiveScan::Rejection]));",
        vector: "a binder beyond the states",
        word: None,
    },
    Plant {
        label: "a noun phrase needs no of",
        site: "(match (isWord ctx (project found upto) (lit \"of\"))",
        replacement: "(match (isWord ctx (project found upto) (lit \"the\"))",
        vector: "the successor",
        word: None,
    },
    Plant {
        label: "the argument is not nested",
        site: "(mk m (cons (ctor Frame.XStart ((project found upto) + 1) (depth + 1))",
        replacement: "(mk m (cons (ctor Frame.XStart ((project found upto) + 1) depth)",
        vector: "term phrases nested beyond the limit",
        word: None,
    },
    Plant {
        label: "a binary noun takes one argument",
        site: "(if ((project found category) =? 1) then (mk m",
        replacement: "(if true then (mk m",
        vector: "the sum",
        word: None,
    },
    Plant {
        label: "the second argument needs no and",
        site: "(match (isWord ctx (project held upto) (lit \"and\"))",
        replacement: "(match (isWord ctx (project held upto) (lit \"or\"))",
        vector: "the sum",
        word: None,
    },
    Plant {
        label: "a binary noun charges no state",
        site: "(let charge : (Bud * (Option PrimitiveScan::Rejection)) := (chargeState ctx (first state));",
        replacement: "(let charge : (Bud * (Option PrimitiveScan::Rejection)) := ((first state), (ctor Option.none [PrimitiveScan::Rejection]));",
        vector: "term phrases beyond the states",
        word: None,
    },
    Plant {
        label: "the island alternative is dropped",
        site: "| Option.some token => (cons (record TAlt {upto := (position + 1), term := (ctor Term.TIsland token)}) (nil TAlt))",
        replacement: "| Option.some token => (nil TAlt)",
        vector: "an island",
        word: None,
    },
    Plant {
        label: "the surface ends at the of",
        site: "(let range : (Nat * Nat) := (atomRange ctx (position + 1) (project found upto));",
        replacement: "(let range : (Nat * Nat) := (atomRange ctx (position + 1) ((project found upto) + 1));",
        vector: "the successor",
        word: None,
    },
    Plant {
        label: "edges are counted again at a known position",
        site: "(if (natIn position (project bud memo)) then (record Looked",
        replacement: "(if false then (record Looked",
        vector: "the unknown atom beyond the edges",
        word: None,
    },
    Plant {
        label: "the depth refusal has no place",
        site: "| Option.some token => (ctor Option.some [PrimitiveScan::Place] (placeAt ctx (GrammarProof::firstAtom token)))",
        replacement: "| Option.some token => (ctor Option.none [PrimitiveScan::Place])",
        vector: "term phrases nested beyond the limit",
        word: None,
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let (root, run) = match plant.word {
            Some(word) => (0, keyword_run(word)),
            None => {
                let vector = vectors
                    .iter()
                    .find(|vector| vector.label == plant.vector)
                    .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
                (vector.ask.root(), vector.run().0)
            }
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
        let low = super::lower(&checked, &planted, MODULE, ROOTS[root].0);
        match super::evaluate(&low, &[run], 1).first() {
            Some(first) => eprintln!(
                "BS-43: plant detected: {}: {}",
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
            "BS-43: {}: largest certificate part {largest} bytes",
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
        checked_units.units.contains_key("PropositionTerms"),
        "PropositionTerms is a linked module of the project"
    );
    eligibility_of(&project, "PropositionTerms", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let mut runs: [Vec<Run>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    runs[0] = KEYWORDS.iter().map(|word| keyword_run(word)).collect();
    let mut messages: Vec<String> = Vec::new();
    let mut shapes = BTreeSet::new();
    for vector in &vectors {
        let (run, refusal) = vector.run();
        messages.extend(refusal);
        if run.1["kind"] == "ok" {
            shapes.insert(match vector.ask {
                Ask::Unknown => "unknown",
                Ask::Binder(..) => "binder",
                Ask::Term(..) => "term",
            });
        }
        runs[vector.ask.root()].push(run);
    }
    for refusal in REFUSALS {
        assert!(
            messages.iter().any(|message| message.starts_with(refusal)),
            "no vector reaches the refusal `{refusal}`"
        );
    }
    assert_eq!(shapes.len(), 3, "a vector of each root is accepted");

    let (sentences, evaluator_sentences, rustc_sentences) =
        sentence_runs(EVALUATOR_SENTENCES, RUSTC_SENTENCES);
    assert!(sentences >= 50, "the committed sentences are {sentences}");
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_runs(EVALUATOR_SEEDED, 0x0B55_0043),
            seeded_runs(RUSTC_SEEDED, 0x0B55_1043),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-43: {} vectors, {} keywords, {} committed sentences ({} and {} runs), {} and {} seeded runs ({:?})",
        vectors.len(),
        KEYWORDS.len(),
        sentences,
        evaluator_sentences.len(),
        rustc_sentences.len(),
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
        for (index, (root, _)) in ROOTS.iter().enumerate() {
            let mut corpus: Vec<Run> = runs[index].clone();
            for (at, run) in evaluator_sentences.iter().chain(evaluator_seeded.iter()) {
                if *at == index {
                    corpus.push(run.clone());
                }
            }
            let disagreements = super::evaluate(&lowered[root], &corpus, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-43: {root}: the evaluator agrees on {} runs ({:?})",
                corpus.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-43: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled = runs;
    for (at, run) in rustc_sentences.into_iter().chain(rustc_seeded) {
        compiled[at].push(run);
    }
    let total = rustc_differential(
        "BS-43",
        &[
            (ROOTS[0].0, &lowered[ROOTS[0].0], compiled[0].as_slice()),
            (ROOTS[1].0, &lowered[ROOTS[1].0], compiled[1].as_slice()),
            (ROOTS[2].0, &lowered[ROOTS[2].0], compiled[2].as_slice()),
            (ROOTS[3].0, &lowered[ROOTS[3].0], compiled[3].as_slice()),
        ],
        "oracle",
    );
    eprintln!("BS-43: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-43") {
        return;
    }
    export(&lowered);
    eprintln!("BS-43: passed in {:?}", started.elapsed());
}
