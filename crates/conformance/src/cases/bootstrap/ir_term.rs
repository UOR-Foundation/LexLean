//! `BS-65` (SPEC.md §17.23, §17.2, §17.3, §17.9): the linked term IR and its
//! alpha-safe canonical JSON as language-1.2 source, `LexLeanBoot.IrTerm`,
//! held to the byte to `lexlean::ir::term::Term::to_json`, `canonical_key`,
//! `eq_key`, `node_count`, and `depth`.
//!
//! `termText` writes a term's canonical JSON under the hashing renumbering,
//! which numbers binders and first-seen free locals densely in traversal
//! order; `canonicalKey` writes the same JSON without display spellings;
//! `eqKey` writes it under the comparison renumbering, which keeps free
//! locals verbatim; `nodeCount` and `termDepth` measure a term as the
//! resource limits count it. Every term of every committed project whose
//! modules declare document environments is a run, and so are hand-built and
//! seeded terms.
//!
//! Each planted defect names the root and the one vector that catch it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::ir::declaration::DeclBody;
use lexlean::ir::document::{Block, DocumentModule, Phrase, PhraseItem};
use lexlean::ir::proof::{Proof, RewriteTarget};
use lexlean::ir::term::{
    Binder, BinderMode, CoreRef, DefinedLexiconRef, DocumentDeclRef, ExternalConstRef, GlobalRef,
    ImplicitBinderId, LocalId, Renumber, Term, Universe,
};
use lexlean::{Selection, Sha256Digest};
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{bytes, list, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.IrTerm";
const SOURCE: &str = "bootstrap/src/IrTerm.lex.tex";
/// The module whose `Universe` the term IR reuses, as the oracle reuses
/// `lexicon::lse::Universe`.
const UNIVERSE_SOURCE: &str = "bootstrap/src/LexiconLse.lex.tex";
const ROOTS: [(&str, &[&str]); 5] = [
    ("termText", &["allocation", "overflow", "recursion"]),
    ("canonicalKey", &["allocation", "overflow", "recursion"]),
    ("eqKey", &["allocation", "overflow", "recursion"]),
    ("nodeCount", &["allocation", "overflow", "recursion"]),
    ("termDepth", &["allocation", "overflow", "recursion"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded terms under the evaluator, and under rustc; each is a run of every
/// root.
const EVALUATOR_SEEDED: usize = 120;
const RUSTC_SEEDED: usize = 600;
/// The committed projects whose linked IR carries terms.
const PROJECTS: &[&str] = &[
    "examples/list-induction",
    "examples/nat-add-zero",
    "examples/peano-arithmetic",
    "examples/propositional-logic",
];
/// The committed fixture projects are searched for more.
const FIXTURES: &str = "tests/fixtures";

// ---------------------------------------------------------------- the sorts

/// The constructors of an inductive of the module or of the module whose
/// `Universe` it reuses, in declaration order, read from their text, so that
/// the encoder names a constructor and never numbers it.
fn constructors(sort: &str) -> &'static Vec<String> {
    static TABLES: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    let tables = TABLES.get_or_init(|| {
        let mut out = BTreeMap::new();
        for file in [SOURCE, UNIVERSE_SOURCE] {
            let source = std::fs::read_to_string(repo_root().join(file).as_std_path())
                .expect("the module's source");
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
                if file == SOURCE || name == "Universe" {
                    out.insert(name, names);
                }
            }
        }
        out
    });
    tables
        .get(sort)
        .unwrap_or_else(|| panic!("no inductive `{sort}`"))
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

pub(super) fn nat(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

fn mode_nat(mode: BinderMode) -> Value {
    nat(match mode {
        BinderMode::Explicit => 0,
        BinderMode::Implicit => 1,
        BinderMode::Instance => 2,
    })
}

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
        Universe::Num(value) => adt("Universe", "UNum", vec![nat(*value)]),
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

fn enc_core(core: CoreRef) -> Value {
    let name = match core {
        CoreRef::Eq => "CEq",
        CoreRef::And => "CAnd",
        CoreRef::Or => "COr",
        CoreRef::Not => "CNot",
        CoreRef::Iff => "CIff",
        CoreRef::Exists => "CExists",
        CoreRef::ExistsUnique => "CExistsUnique",
    };
    adt("CoreRef", name, Vec::new())
}

pub(super) fn enc_global(global: &GlobalRef) -> Value {
    match global {
        GlobalRef::Core(core) => adt("GlobalRef", "GCore", vec![enc_core(*core)]),
        GlobalRef::External(external) => adt(
            "GlobalRef",
            "GExternal",
            vec![
                text(&external.package),
                text(&external.entry),
                text(&external.lean_module),
                text(&external.lean_name),
                bytes(&external.signature_hash.0),
            ],
        ),
        GlobalRef::Document(document) => adt(
            "GlobalRef",
            "GDocument",
            vec![
                text(&document.module),
                text(&document.component),
                text(&document.lean_name),
            ],
        ),
        GlobalRef::DefinedLexicon(defined) => adt(
            "GlobalRef",
            "GDefined",
            vec![
                text(&defined.package),
                text(&defined.entry),
                bytes(&defined.signature_hash.0),
            ],
        ),
    }
}

/// Terms as the module's chain of `TItem` cells.
pub(super) fn term_chain(items: &[Term]) -> Value {
    items
        .iter()
        .rev()
        .fold(adt("Term", "TNil", Vec::new()), |rest, item| {
            adt("Term", "TItem", vec![enc_term(item), rest])
        })
}

/// Binders as the module's chain of `TBind` cells.
pub(super) fn binder_chain(binders: &[Binder]) -> Value {
    binders
        .iter()
        .rev()
        .fold(adt("Term", "TNil", Vec::new()), |rest, binder| {
            adt(
                "Term",
                "TBind",
                vec![
                    nat(binder.id.0),
                    mode_nat(binder.mode),
                    text(&binder.spelling),
                    enc_term(&binder.ty),
                    rest,
                ],
            )
        })
}

pub(super) fn enc_term(term: &Term) -> Value {
    match term {
        Term::Sort(universe) => adt("Term", "TSort", vec![enc_universe(universe)]),
        Term::Local(local) => adt("Term", "TLocal", vec![nat(local.0)]),
        Term::Global(global, universes) => adt(
            "Term",
            "TGlobal",
            vec![enc_global(global), universe_chain(universes)],
        ),
        Term::App {
            function,
            explicit_args,
            omitted_implicit_binders,
        } => adt(
            "Term",
            "TApp",
            vec![
                enc_term(function),
                term_chain(explicit_args),
                list(
                    omitted_implicit_binders
                        .iter()
                        .map(|binder| nat(u64::from(binder.0)))
                        .collect(),
                ),
            ],
        ),
        Term::Pi { binders, body } => {
            adt("Term", "TPi", vec![binder_chain(binders), enc_term(body)])
        }
        Term::Lambda { binders, body } => {
            adt("Term", "TLam", vec![binder_chain(binders), enc_term(body)])
        }
        Term::Let {
            binder,
            value,
            body,
        } => adt(
            "Term",
            "TLet",
            vec![
                nat(binder.id.0),
                mode_nat(binder.mode),
                text(&binder.spelling),
                enc_term(&binder.ty),
                enc_term(value),
                enc_term(body),
            ],
        ),
        Term::NatLiteral {
            decimal,
            expected_type,
        } => adt("Term", "TNat", vec![text(decimal), enc_term(expected_type)]),
    }
}

/// The oracle's outcome of one root on one term.
fn oracle(root: &str, term: &Term) -> Value {
    match root {
        "termText" => text(&term.to_json(&mut Renumber::default()).to_canonical_string()),
        "canonicalKey" => text(&term.canonical_key()),
        "eqKey" => text(&term.eq_key()),
        "nodeCount" => nat(term.node_count()),
        "termDepth" => nat(term.depth()),
        other => panic!("no root `{other}`"),
    }
}

fn run(root: &str, term: &Term) -> Run {
    (
        vec![enc_term(term)],
        serde_json::to_value(oracle(root, term)).expect("a value is JSON"),
    )
}

// ---------------------------------------------------------------- the corpus

fn proof_terms(proof: &Proof, out: &mut Vec<Term>) {
    match proof {
        Proof::Sequence(steps) | Proof::Constructor(steps) => {
            for step in steps {
                proof_terms(step, out);
            }
        }
        Proof::Intro(_) | Proof::Reflexivity | Proof::SelectLeft | Proof::SelectRight => {}
        Proof::Exact(term) | Proof::ApplyOne(term) | Proof::Witness(term) => {
            out.push(term.clone());
        }
        Proof::Apply { function, premises } => {
            out.push(function.clone());
            for premise in premises {
                proof_terms(premise, out);
            }
        }
        Proof::Have {
            proposition, proof, ..
        } => {
            out.push(proposition.clone());
            proof_terms(proof, out);
        }
        Proof::Rewrite { target, rules } => {
            if let RewriteTarget::Hypothesis(local) = target {
                out.push(Term::Local(*local));
            }
            out.extend(rules.iter().map(|rule| rule.term.clone()));
        }
        Proof::SimplifyOnly { rules, .. } => out.extend(rules.iter().cloned()),
        Proof::Cases { scrutinee, cases } | Proof::Induction { scrutinee, cases } => {
            out.push(scrutinee.clone());
            for case in cases {
                proof_terms(&case.proof, out);
            }
        }
        Proof::Calculate {
            relation,
            start,
            steps,
        } => {
            out.push(Term::Global(relation.clone(), Vec::new()));
            out.push(start.clone());
            for step in steps {
                out.push(step.term.clone());
                out.push(step.proof.clone());
            }
        }
    }
}

fn phrase_terms(phrase: &Phrase, out: &mut Vec<Term>) {
    for item in &phrase.items {
        if let PhraseItem::Math(term) = item {
            out.push(term.clone());
        }
    }
}

fn block_terms(blocks: &[Block], out: &mut Vec<Term>) {
    for block in blocks {
        match block {
            Block::Section(section) => {
                phrase_terms(&section.heading, out);
                out.extend(section.params.iter().map(|binder| binder.ty.clone()));
                block_terms(&section.blocks, out);
            }
            Block::Declaration(declaration) => {
                out.extend(declaration.params.iter().map(|binder| binder.ty.clone()));
                match &declaration.body {
                    DeclBody::Definition { ty, value, .. } => {
                        out.push(ty.clone());
                        out.push(value.clone());
                    }
                    DeclBody::TheoremLike { statement, proof } => {
                        out.push(statement.clone());
                        proof_terms(proof, out);
                    }
                }
                // The whole declaration as one term: its parameters bound
                // over its statement or type, as the linker scopes it.
                let inner = match &declaration.body {
                    DeclBody::Definition { ty, .. } => ty.clone(),
                    DeclBody::TheoremLike { statement, .. } => statement.clone(),
                };
                if !declaration.params.is_empty() {
                    out.push(Term::Pi {
                        binders: declaration.params.clone(),
                        body: Box::new(inner),
                    });
                }
            }
        }
    }
}

/// The linked modules of one committed project, or none when it does not
/// check under its committed lock.
fn linked_of(dir: &camino::Utf8Path) -> Option<Vec<DocumentModule>> {
    let project = lexlean::project::Project::load(&dir.join("lexlean.toml")).ok()?;
    let (lock, packages) = lexlean::lock::read_current_lock(&project).ok()?;
    let checked =
        lexlean::link::check_project(&project, &Selection::Entrypoints, &lock, packages).ok()?;
    Some(
        checked
            .modules
            .into_values()
            .map(|module| module.document)
            .collect(),
    )
}

/// The linked modules of the committed projects that declare document
/// environments, and of the fixture projects that check.
pub(super) fn linked_modules() -> Vec<DocumentModule> {
    let root = repo_root();
    let mut dirs: Vec<camino::Utf8PathBuf> =
        PROJECTS.iter().map(|project| root.join(project)).collect();
    if let Ok(entries) = std::fs::read_dir(root.join(FIXTURES).as_std_path()) {
        let mut fixtures: Vec<_> = entries
            .flatten()
            .filter(|entry| entry.path().join("lexlean.toml").is_file())
            .map(|entry| camino::Utf8PathBuf::from_path_buf(entry.path()).expect("a UTF-8 path"))
            .collect();
        fixtures.sort();
        dirs.extend(fixtures);
    }
    let mut out = Vec::new();
    for dir in &dirs {
        let modules = linked_of(dir);
        if PROJECTS.iter().any(|project| dir.ends_with(project)) {
            assert!(
                modules.as_ref().is_some_and(|modules| !modules.is_empty()),
                "{dir}: the committed project checks"
            );
        }
        out.extend(modules.unwrap_or_default());
    }
    out
}

/// Every term of the committed projects that declare document environments,
/// each distinct one once.
fn committed() -> Vec<Term> {
    let mut terms = Vec::new();
    for module in linked_modules() {
        phrase_terms(&module.title, &mut terms);
        block_terms(&module.blocks, &mut terms);
    }
    let mut seen = BTreeSet::new();
    terms
        .into_iter()
        .filter(|term| seen.insert(term.to_json(&mut Renumber::default()).to_canonical_string()))
        .collect()
}

// ------------------------------------------------------------------ vectors

fn sha(byte: u8) -> Sha256Digest {
    let mut out = [0u8; 32];
    for (index, slot) in out.iter_mut().enumerate() {
        *slot = byte.wrapping_add(u8::try_from(index).expect("a byte"));
    }
    Sha256Digest(out)
}

fn local(id: u64) -> Term {
    Term::Local(LocalId(id))
}

fn sort(level: u64) -> Term {
    Term::Sort(Universe::Num(level))
}

fn binder(id: u64, mode: BinderMode, spelling: &str, ty: Term) -> Binder {
    Binder {
        id: LocalId(id),
        mode,
        spelling: spelling.to_owned(),
        ty,
    }
}

fn explicit(id: u64, spelling: &str, ty: Term) -> Binder {
    binder(id, BinderMode::Explicit, spelling, ty)
}

fn core(core: CoreRef) -> Term {
    Term::Global(GlobalRef::Core(core), Vec::new())
}

fn app(function: Term, args: Vec<Term>, omitted: &[u32]) -> Term {
    Term::App {
        function: Box::new(function),
        explicit_args: args,
        omitted_implicit_binders: omitted.iter().copied().map(ImplicitBinderId).collect(),
    }
}

fn external() -> GlobalRef {
    GlobalRef::External(ExternalConstRef {
        package: "lexlean.std.nat".to_owned(),
        entry: "lexlean.std.nat::add".to_owned(),
        lean_module: "Init.Prelude".to_owned(),
        lean_name: "Nat.add".to_owned(),
        signature_hash: sha(0x40),
    })
}

struct Vector {
    label: &'static str,
    term: Term,
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out = Vec::new();
    let mut add = |label: &'static str, term: Term| out.push(Vector { label, term });
    add("a sort", sort(0));
    add(
        "a bound local",
        Term::Lambda {
            binders: vec![explicit(5, "x", sort(1))],
            body: Box::new(local(5)),
        },
    );
    add(
        "free locals first",
        app(core(CoreRef::Eq), vec![local(7), local(3), local(7)], &[]),
    );
    add(
        "a rebound id",
        Term::Pi {
            binders: vec![explicit(4, "x", sort(0)), explicit(4, "y", sort(1))],
            body: Box::new(local(4)),
        },
    );
    add(
        "a let over free locals",
        Term::Let {
            binder: Box::new(explicit(2, "y", local(9))),
            value: Box::new(local(8)),
            body: Box::new(app(local(2), vec![local(8), local(9)], &[])),
        },
    );
    add(
        "a binder in its own type",
        Term::Pi {
            binders: vec![explicit(6, "z", local(11)), explicit(12, "w", local(6))],
            body: Box::new(local(11)),
        },
    );
    add(
        "binder modes",
        Term::Pi {
            binders: vec![
                binder(1, BinderMode::Implicit, "a", sort(1)),
                binder(2, BinderMode::Instance, "i", local(1)),
                binder(3, BinderMode::Explicit, "b", local(1)),
            ],
            body: Box::new(local(3)),
        },
    );
    add(
        "spelled binders",
        Term::Lambda {
            binders: vec![explicit(1, "alpha", sort(0))],
            body: Box::new(local(1)),
        },
    );
    add(
        "nested binder spellings",
        Term::Pi {
            binders: vec![explicit(
                1,
                "f",
                Term::Pi {
                    binders: vec![explicit(2, "inner", sort(0))],
                    body: Box::new(local(2)),
                },
            )],
            body: Box::new(Term::Lambda {
                binders: vec![explicit(3, "deep", local(1))],
                body: Box::new(local(3)),
            }),
        },
    );
    add(
        "omitted implicits",
        app(
            Term::Global(external(), Vec::new()),
            vec![local(1)],
            &[0, 2],
        ),
    );
    add(
        "a global with universes",
        Term::Global(
            external(),
            vec![
                Universe::Var("u".to_owned()),
                Universe::Succ(Box::new(Universe::Num(1))),
            ],
        ),
    );
    add("an external global", Term::Global(external(), Vec::new()));
    add(
        "a document global",
        Term::Global(
            GlobalRef::Document(DocumentDeclRef {
                module: "Main".to_owned(),
                component: "thm-one".to_owned(),
                lean_name: "Demo.Main.thm_one".to_owned(),
            }),
            Vec::new(),
        ),
    );
    add(
        "a defined global",
        Term::Global(
            GlobalRef::DefinedLexicon(DefinedLexiconRef {
                package: "demo.terms".to_owned(),
                entry: "demo.terms::two".to_owned(),
                signature_hash: sha(0x90),
            }),
            Vec::new(),
        ),
    );
    add(
        "every core constructor",
        app(
            core(CoreRef::Eq),
            vec![
                core(CoreRef::And),
                core(CoreRef::Or),
                core(CoreRef::Not),
                core(CoreRef::Iff),
                core(CoreRef::Exists),
                core(CoreRef::ExistsUnique),
            ],
            &[],
        ),
    );
    add(
        "max and imax",
        Term::Sort(Universe::Max(vec![
            Universe::IMax(
                Box::new(Universe::Var("u".to_owned())),
                Box::new(Universe::Num(0)),
            ),
            Universe::Num(3),
        ])),
    );
    add("a large universe", Term::Sort(Universe::Num(u64::MAX)));
    add(
        "a large free local",
        app(local(u64::MAX), vec![local(1 << 63)], &[]),
    );
    add(
        "a numeral",
        Term::NatLiteral {
            decimal: "18446744073709551616".to_owned(),
            expected_type: Box::new(Term::Global(external(), Vec::new())),
        },
    );
    add(
        "escapes in a spelling",
        Term::Lambda {
            binders: vec![explicit(1, "a \"b\"\\c\n\u{1}\u{e9}", sort(0))],
            body: Box::new(local(1)),
        },
    );
    add(
        "a wide application",
        app(local(1), vec![local(2), local(3), local(4)], &[]),
    );
    add(
        "a deep argument",
        app(
            local(1),
            vec![app(local(2), vec![app(local(3), vec![local(4)], &[])], &[])],
            &[],
        ),
    );
    out
}

// ------------------------------------------------------------------- seeded

const LOCALS: &[u64] = &[0, 1, 2, 3, 7, 42, 1 << 63, u64::MAX];
const PIECES: &[&str] = &["s", "x", "t", "\"", "\\", "\n", "\u{e9}", "_", "0"];
const NAMES: &[&str] = &["u", "v", "w.1"];

fn drawn_text(seeded: &mut Seeded) -> String {
    let count = seeded.below(4);
    (0..count).map(|_| *seeded.pick(PIECES)).collect()
}

fn drawn_sha(seeded: &mut Seeded) -> Sha256Digest {
    let mut out = [0u8; 32];
    for slot in &mut out {
        *slot = u8::try_from(seeded.below(256)).expect("a byte");
    }
    Sha256Digest(out)
}

fn drawn_universe(seeded: &mut Seeded, depth: usize) -> Universe {
    let kind = if depth == 0 {
        seeded.below(2)
    } else {
        seeded.below(5)
    };
    match kind {
        0 => Universe::Num(*seeded.pick(&[0u64, 1, 2, 9_223_372_036_854_775_808, u64::MAX])),
        1 => Universe::Var((*seeded.pick(NAMES)).to_owned()),
        2 => Universe::Succ(Box::new(drawn_universe(seeded, depth - 1))),
        3 => Universe::Max(
            (0..seeded.below(3))
                .map(|_| drawn_universe(seeded, depth - 1))
                .collect(),
        ),
        _ => Universe::IMax(
            Box::new(drawn_universe(seeded, depth - 1)),
            Box::new(drawn_universe(seeded, depth - 1)),
        ),
    }
}

fn drawn_global(seeded: &mut Seeded) -> GlobalRef {
    match seeded.below(4) {
        0 => GlobalRef::Core(*seeded.pick(&[
            CoreRef::Eq,
            CoreRef::And,
            CoreRef::Or,
            CoreRef::Not,
            CoreRef::Iff,
            CoreRef::Exists,
            CoreRef::ExistsUnique,
        ])),
        1 => GlobalRef::External(ExternalConstRef {
            package: drawn_text(seeded),
            entry: drawn_text(seeded),
            lean_module: drawn_text(seeded),
            lean_name: drawn_text(seeded),
            signature_hash: drawn_sha(seeded),
        }),
        2 => GlobalRef::Document(DocumentDeclRef {
            module: drawn_text(seeded),
            component: drawn_text(seeded),
            lean_name: drawn_text(seeded),
        }),
        _ => GlobalRef::DefinedLexicon(DefinedLexiconRef {
            package: drawn_text(seeded),
            entry: drawn_text(seeded),
            signature_hash: drawn_sha(seeded),
        }),
    }
}

fn drawn_binder(seeded: &mut Seeded, depth: usize) -> Binder {
    Binder {
        id: LocalId(*seeded.pick(LOCALS)),
        mode: *seeded.pick(&[
            BinderMode::Explicit,
            BinderMode::Implicit,
            BinderMode::Instance,
        ]),
        spelling: drawn_text(seeded),
        ty: drawn_term(seeded, depth),
    }
}

fn drawn_term(seeded: &mut Seeded, depth: usize) -> Term {
    let kind = if depth == 0 {
        seeded.below(3)
    } else {
        seeded.below(8)
    };
    let next = depth.saturating_sub(1);
    match kind {
        0 => Term::Sort(drawn_universe(seeded, 2)),
        1 => Term::Local(LocalId(*seeded.pick(LOCALS))),
        2 => Term::Global(
            drawn_global(seeded),
            (0..seeded.below(3))
                .map(|_| drawn_universe(seeded, 1))
                .collect(),
        ),
        3 => Term::App {
            function: Box::new(drawn_term(seeded, next)),
            explicit_args: (0..seeded.below(4))
                .map(|_| drawn_term(seeded, next))
                .collect(),
            omitted_implicit_binders: (0..seeded.below(3))
                .map(|_| ImplicitBinderId(*seeded.pick(&[0u32, 1, 2, u32::MAX])))
                .collect(),
        },
        4 => Term::Pi {
            binders: (0..seeded.below(3))
                .map(|_| drawn_binder(seeded, next))
                .collect(),
            body: Box::new(drawn_term(seeded, next)),
        },
        5 => Term::Lambda {
            binders: (0..seeded.below(3))
                .map(|_| drawn_binder(seeded, next))
                .collect(),
            body: Box::new(drawn_term(seeded, next)),
        },
        6 => Term::Let {
            binder: Box::new(drawn_binder(seeded, next)),
            value: Box::new(drawn_term(seeded, next)),
            body: Box::new(drawn_term(seeded, next)),
        },
        _ => Term::NatLiteral {
            decimal: (*seeded.pick(&["0", "7", "18446744073709551616"])).to_owned(),
            expected_type: Box::new(drawn_term(seeded, next)),
        },
    }
}

fn seeded_terms(count: usize, seed: u64) -> Vec<Term> {
    let mut seeded = Seeded(seed);
    (0..count)
        .map(|_| {
            let depth = seeded.below(5);
            drawn_term(&mut seeded, depth)
        })
        .collect()
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    root: usize,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "a let binds before its value",
        site: "(bind (second valueJson) localId)",
        replacement: "(bind (second typeJson) localId)",
        root: 0,
        vector: "a let over free locals",
    },
    Plant {
        label: "a binder binds before its type",
        site: "(bind (second typeJson) localId)",
        replacement: "(bind state localId)",
        root: 0,
        vector: "a binder in its own type",
    },
    Plant {
        label: "a known binder takes a fresh index",
        site: "| Option.some index => (index, state)",
        replacement: "| Option.some index => (fresh state localId)",
        root: 0,
        vector: "a rebound id",
    },
    Plant {
        label: "comparison renumbers free locals",
        site: "(termJson term (comparing))",
        replacement: "(termJson term (hashing))",
        root: 2,
        vector: "free locals first",
    },
    Plant {
        label: "hashing keeps free locals",
        site: "next := 0, keepFree := false})",
        replacement: "next := 0, keepFree := true})",
        root: 0,
        vector: "free locals first",
    },
    Plant {
        label: "the key keeps spellings",
        site: "(LexLeanStd.Json::writeJson (withoutSpellings (first (termJson term (hashing)))))",
        replacement: "(LexLeanStd.Json::writeJson (first (termJson term (hashing))))",
        root: 1,
        vector: "spelled binders",
    },
    Plant {
        label: "spellings after a stripped one are kept",
        site: "then (withoutSpellings rest) else",
        replacement: "then rest else",
        root: 1,
        vector: "nested binder spellings",
    },
    Plant {
        label: "omitted implicits are never written",
        site: "| List.cons head rest => (member \"i\" (ctor LexLeanStd.Json::Json.JArr (implicitJson implicits)) (ctor LexLeanStd.Json::Json.JNil))",
        replacement: "| List.cons head rest => (ctor LexLeanStd.Json::Json.JNil)",
        root: 0,
        vector: "omitted implicits",
    },
    Plant {
        label: "omitted implicits are written last first",
        site: "(LexLeanStd.Order::reverseList [Nat] implicits)",
        replacement: "implicits",
        root: 0,
        vector: "omitted implicits",
    },
    Plant {
        label: "universes are never written",
        site: "(if (noLevels levels) then",
        replacement: "(if true then",
        root: 0,
        vector: "a global with universes",
    },
    Plant {
        label: "a large number is not capped",
        site: "then number else 9223372036854775807",
        replacement: "then number else number",
        root: 0,
        vector: "a large universe",
    },
    Plant {
        label: "an external global names its package as its entry",
        site: "(member \"entry\" (ctor LexLeanStd.Json::Json.JStr entry) (member \"lean_module\"",
        replacement: "(member \"entry\" (ctor LexLeanStd.Json::Json.JStr package) (member \"lean_module\"",
        root: 0,
        vector: "an external global",
    },
    Plant {
        label: "a defined global loses its hash",
        site: "(jtext \"defined\") (member \"entry\" (ctor LexLeanStd.Json::Json.JStr entry) (member \"signature_sha256\" (ctor LexLeanStd.Json::Json.JStr (LexLeanStd.Numeral::hex signature)) (ctor LexLeanStd.Json::Json.JNil)))",
        replacement: "(jtext \"defined\") (member \"entry\" (ctor LexLeanStd.Json::Json.JStr entry) (ctor LexLeanStd.Json::Json.JNil))",
        root: 0,
        vector: "a defined global",
    },
    Plant {
        label: "a document global loses its component",
        site: "(member \"component\" (ctor LexLeanStd.Json::Json.JStr component) (member \"lean_name\" (ctor LexLeanStd.Json::Json.JStr leanName) (ctor LexLeanStd.Json::Json.JNil)))",
        replacement: "(member \"lean_name\" (ctor LexLeanStd.Json::Json.JStr leanName) (ctor LexLeanStd.Json::Json.JNil))",
        root: 0,
        vector: "a document global",
    },
    Plant {
        label: "unique existence is spelled existence",
        site: "| CoreRef.CExistsUnique => (lit \"exists-unique\")",
        replacement: "| CoreRef.CExistsUnique => (lit \"exists\")",
        root: 0,
        vector: "every core constructor",
    },
    Plant {
        label: "imax arguments are swapped",
        site: "(ctor LexLeanStd.Json::Json.JItem (universeJson left) (ctor LexLeanStd.Json::Json.JItem (universeJson right)",
        replacement: "(ctor LexLeanStd.Json::Json.JItem (universeJson right) (ctor LexLeanStd.Json::Json.JItem (universeJson left)",
        root: 0,
        vector: "max and imax",
    },
    Plant {
        label: "every binder is explicit",
        site: "(member \"m\" (modeName mode)",
        replacement: "(member \"m\" (modeName 0)",
        root: 0,
        vector: "binder modes",
    },
    Plant {
        label: "a lambda is tagged a pi",
        site: "(jtext \"lam\")",
        replacement: "(jtext \"pi\")",
        root: 0,
        vector: "a bound local",
    },
    Plant {
        label: "a numeral loses its digits",
        site: "(member \"d\" (ctor LexLeanStd.Json::Json.JStr decimal)",
        replacement: "(member \"d\" (jtext \"0\")",
        root: 0,
        vector: "a numeral",
    },
    Plant {
        label: "a let value is not counted",
        site: "(((1 + (nodeCount typed)) + (nodeCount bound)) + (nodeCount body))",
        replacement: "((1 + (nodeCount typed)) + (nodeCount body))",
        root: 3,
        vector: "a let over free locals",
    },
    Plant {
        label: "a binder type is not counted",
        site: "| Term.TBind localId mode spelling typed tail => ((nodeCount typed) + (nodeCount tail))",
        replacement: "| Term.TBind localId mode spelling typed tail => (nodeCount tail)",
        root: 3,
        vector: "a rebound id",
    },
    Plant {
        label: "the depth counts list cells",
        site: "| Term.TItem head tail => (larger (termDepth head) (termDepth tail))",
        replacement: "| Term.TItem head tail => (1 + (larger (termDepth head) (termDepth tail)))",
        root: 4,
        vector: "a wide application",
    },
    Plant {
        label: "the depth ignores the arguments",
        site: "| Term.TApp function arguments implicits => (1 + (larger (termDepth function) (termDepth arguments)))",
        replacement: "| Term.TApp function arguments implicits => (1 + (termDepth function))",
        root: 4,
        vector: "a deep argument",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let root = ROOTS[plant.root].0;
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/IrTerm.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/IrTerm.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, root);
        match super::evaluate(&low, &[run(root, &vector.term)], 1).first() {
            Some(first) => eprintln!(
                "BS-65: plant detected: {}: {}",
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
            "BS-65: {}: largest certificate part {largest} bytes",
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

fn runs_of(root: &str, terms: &[Term]) -> Vec<Run> {
    terms.iter().map(|term| run(root, term)).collect()
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("IrTerm"),
        "IrTerm is a linked module of the project"
    );
    eligibility_of(&project, "IrTerm", MODULE, &ROOTS);
    assert_eq!(
        constructors("Term").as_slice(),
        [
            "TSort", "TLocal", "TGlobal", "TApp", "TPi", "TLam", "TLet", "TNat", "TNil", "TItem",
            "TBind"
        ],
        "the constructors of Term"
    );
    assert_eq!(
        constructors("Universe").as_slice(),
        ["UNum", "UVar", "USucc", "UMax", "UIMax", "UNil", "UItem"],
        "the constructors of Universe"
    );

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let named_terms: Vec<Term> = vectors.iter().map(|vector| vector.term.clone()).collect();

    let corpus = committed();
    assert!(
        corpus.len() >= 40,
        "the committed projects carry {} distinct terms",
        corpus.len()
    );
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_terms(EVALUATOR_SEEDED, 0x0B55_0065),
            seeded_terms(RUSTC_SEEDED, 0x0B55_1065),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-65: {} vectors, {} committed terms, {} and {} seeded terms, each a run of {} roots ({:?})",
        vectors.len(),
        corpus.len(),
        evaluator_seeded.len(),
        rustc_seeded.len(),
        ROOTS.len(),
        started.elapsed()
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    if normative {
        for (root, _) in ROOTS {
            let mut terms = named_terms.clone();
            terms.extend(corpus.iter().cloned());
            terms.extend(evaluator_seeded.iter().cloned());
            let runs = runs_of(root, &terms);
            let disagreements = super::evaluate(&lowered[root], &runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-65: {root}: the evaluator agrees on {} runs ({:?})",
                runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-65: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled_terms = named_terms;
    compiled_terms.extend(corpus);
    compiled_terms.extend(rustc_seeded);
    let compiled: Vec<(&str, Vec<Run>)> = ROOTS
        .iter()
        .map(|(root, _)| (*root, runs_of(root, &compiled_terms)))
        .collect();
    let differential: Vec<(&str, &super::Lowered, &[Run])> = compiled
        .iter()
        .map(|(root, runs)| (*root, &lowered[root], runs.as_slice()))
        .collect();
    let total = rustc_differential("BS-65", &differential, "oracle");
    eprintln!("BS-65: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-65") {
        return;
    }
    export(&lowered);
    eprintln!("BS-65: passed in {:?}", started.elapsed());
}
