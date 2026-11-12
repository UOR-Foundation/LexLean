//! `BS-66` (SPEC.md §17.23, §17.4, §17.5, §17.9, §15.9): the proof and
//! declaration IR and their alpha-safe canonical JSON as language-1.2 source,
//! `LexLeanBoot.IrProof`, held to the byte to `lexlean::ir::proof::Proof::to_json`,
//! `lexlean::ir::declaration::Declaration::to_json`, and `AxiomPolicy::permits`.
//!
//! `proofText` and `declarationText` write a proof and a declaration in the
//! canonical JSON of the linked IR under the hashing renumbering of `BS-65`,
//! whose term JSON they reuse; `permits` enforces an axiom policy against an
//! observed axiom set. Every proof and declaration of every committed project
//! whose modules declare document environments is a run, and so are
//! hand-built and seeded ones.
//!
//! Each planted defect names the root and the one vector that catch it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::ir::declaration::{AxiomPolicy, DeclBody, DeclKind, Declaration};
use lexlean::ir::document::Block;
use lexlean::ir::proof::{CalculationStep, CaseProof, Proof, RewriteRule, RewriteTarget};
use lexlean::ir::term::{
    Binder, BinderMode, CoreRef, ExternalConstRef, GlobalRef, LocalId, Renumber, Term, Universe,
};
use lexlean::lexicon::lse::QualifiedId;
use lexlean::Sha256Digest;
use serde_json::Value as Json2;

use super::ir_term::{binder_chain, enc_global, enc_term, linked_modules, nat, term_chain};
use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, option, pair, text, texts, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.IrProof";
const SOURCE: &str = "bootstrap/src/IrProof.lex.tex";
const ROOTS: [(&str, &[&str]); 3] = [
    ("proofText", &["allocation", "overflow", "recursion"]),
    ("declarationText", &["allocation", "overflow", "recursion"]),
    ("permits", &["allocation", "overflow"]),
];

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
/// Seeded inputs of each root under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 100;
const RUSTC_SEEDED: usize = 500;

// ---------------------------------------------------------------- the sorts

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

fn proof_chain(items: &[Proof]) -> Value {
    items
        .iter()
        .rev()
        .fold(adt("Proof", "PNil", Vec::new()), |rest, item| {
            adt("Proof", "PItem", vec![enc_proof(item), rest])
        })
}

fn rule_chain(rules: &[RewriteRule]) -> Value {
    rules
        .iter()
        .rev()
        .fold(adt("Proof", "PNil", Vec::new()), |rest, rule| {
            adt(
                "Proof",
                "PRule",
                vec![
                    Value::Bool {
                        value: rule.reverse,
                    },
                    enc_term(&rule.term),
                    rest,
                ],
            )
        })
}

fn case_chain(cases: &[CaseProof]) -> Value {
    cases
        .iter()
        .rev()
        .fold(adt("Proof", "PNil", Vec::new()), |rest, case| {
            adt(
                "Proof",
                "PCase",
                vec![
                    text(&case.constructor.to_string()),
                    text(&case.lean_name),
                    list(
                        case.binders
                            .iter()
                            .map(|(local, spelling)| pair(nat(local.0), text(spelling)))
                            .collect(),
                    ),
                    enc_proof(&case.proof),
                    rest,
                ],
            )
        })
}

fn step_chain(steps: &[CalculationStep]) -> Value {
    steps
        .iter()
        .rev()
        .fold(adt("Proof", "PNil", Vec::new()), |rest, step| {
            adt(
                "Proof",
                "PStep",
                vec![enc_term(&step.term), enc_term(&step.proof), rest],
            )
        })
}

fn enc_target(target: &RewriteTarget) -> Value {
    option(match target {
        RewriteTarget::Goal => None,
        RewriteTarget::Hypothesis(local) => Some(nat(local.0)),
    })
}

fn enc_proof(proof: &Proof) -> Value {
    match proof {
        Proof::Sequence(steps) => adt("Proof", "PSeq", vec![proof_chain(steps)]),
        Proof::Intro(locals) => adt(
            "Proof",
            "PIntro",
            vec![list(locals.iter().map(|local| nat(local.0)).collect())],
        ),
        Proof::Exact(term) => adt("Proof", "PExact", vec![enc_term(term)]),
        Proof::ApplyOne(term) => adt("Proof", "PApplyOne", vec![enc_term(term)]),
        Proof::Apply { function, premises } => adt(
            "Proof",
            "PApply",
            vec![enc_term(function), proof_chain(premises)],
        ),
        Proof::Reflexivity => adt("Proof", "PRfl", Vec::new()),
        Proof::Witness(term) => adt("Proof", "PWitness", vec![enc_term(term)]),
        Proof::SelectLeft => adt("Proof", "PLeft", Vec::new()),
        Proof::SelectRight => adt("Proof", "PRight", Vec::new()),
        Proof::Have {
            local,
            proposition,
            proof,
        } => adt(
            "Proof",
            "PHave",
            vec![nat(local.0), enc_term(proposition), enc_proof(proof)],
        ),
        Proof::Rewrite { target, rules } => adt(
            "Proof",
            "PRewrite",
            vec![enc_target(target), rule_chain(rules)],
        ),
        Proof::SimplifyOnly { target, rules } => adt(
            "Proof",
            "PSimp",
            vec![enc_target(target), term_chain(rules)],
        ),
        Proof::Constructor(branches) => adt("Proof", "PConstructor", vec![proof_chain(branches)]),
        Proof::Cases { scrutinee, cases } => adt(
            "Proof",
            "PCases",
            vec![enc_term(scrutinee), case_chain(cases)],
        ),
        Proof::Induction { scrutinee, cases } => adt(
            "Proof",
            "PInduction",
            vec![enc_term(scrutinee), case_chain(cases)],
        ),
        Proof::Calculate {
            relation,
            start,
            steps,
        } => adt(
            "Proof",
            "PCalc",
            vec![enc_global(relation), enc_term(start), step_chain(steps)],
        ),
    }
}

fn enc_kind(kind: DeclKind) -> Value {
    let name = match kind {
        DeclKind::TypeDefinition => "KTypeDefinition",
        DeclKind::TermDefinition => "KTermDefinition",
        DeclKind::PredicateDefinition => "KPredicateDefinition",
        DeclKind::Theorem => "KTheorem",
        DeclKind::Lemma => "KLemma",
        DeclKind::Corollary => "KCorollary",
    };
    adt("DeclKind", name, Vec::new())
}

fn enc_policy(policy: &AxiomPolicy) -> Value {
    match policy {
        AxiomPolicy::None => adt("AxiomPolicy", "ANone", Vec::new()),
        AxiomPolicy::Allow(names) => adt("AxiomPolicy", "AAllow", vec![texts(names)]),
        AxiomPolicy::Exact(names) => adt("AxiomPolicy", "AExact", vec![texts(names)]),
    }
}

pub(super) fn enc_declaration(declaration: &Declaration) -> Value {
    let body = match &declaration.body {
        DeclBody::Definition { entry, ty, value } => adt(
            "DeclBody",
            "BDefinition",
            vec![text(&entry.to_string()), enc_term(ty), enc_term(value)],
        ),
        DeclBody::TheoremLike { statement, proof } => adt(
            "DeclBody",
            "BTheorem",
            vec![enc_term(statement), enc_proof(proof)],
        ),
    };
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(&declaration.component),
            text(&declaration.lean_name),
            enc_kind(declaration.kind),
            binder_chain(&declaration.params),
            body,
            enc_policy(&declaration.policy),
        ],
    }
}

// ---------------------------------------------------------------- the inputs

#[derive(Clone)]
enum Input {
    Proof(Proof),
    Declaration(Declaration),
    Permit(AxiomPolicy, Vec<String>),
}

impl Input {
    const fn root(&self) -> usize {
        match self {
            Self::Proof(_) => 0,
            Self::Declaration(_) => 1,
            Self::Permit(..) => 2,
        }
    }

    fn run(&self) -> Run {
        let (arguments, outcome) = match self {
            Self::Proof(proof) => (
                vec![enc_proof(proof)],
                text(
                    &proof
                        .to_json(&mut Renumber::default())
                        .to_canonical_string(),
                ),
            ),
            Self::Declaration(declaration) => (
                vec![enc_declaration(declaration)],
                text(
                    &declaration
                        .to_json(&mut Renumber::default())
                        .to_canonical_string(),
                ),
            ),
            Self::Permit(policy, observed) => (
                vec![enc_policy(policy), texts(observed)],
                Value::Bool {
                    value: policy.permits(observed),
                },
            ),
        };
        (
            arguments,
            serde_json::to_value(outcome).expect("a value is JSON"),
        )
    }
}

// ---------------------------------------------------------------- the corpus

fn proofs_in(proof: &Proof, out: &mut Vec<Proof>) {
    out.push(proof.clone());
    match proof {
        Proof::Sequence(steps) | Proof::Constructor(steps) => {
            for step in steps {
                proofs_in(step, out);
            }
        }
        Proof::Apply { premises, .. } => {
            for premise in premises {
                proofs_in(premise, out);
            }
        }
        Proof::Have { proof, .. } => proofs_in(proof, out),
        Proof::Cases { cases, .. } | Proof::Induction { cases, .. } => {
            for case in cases {
                proofs_in(&case.proof, out);
            }
        }
        _ => {}
    }
}

fn declarations_in(blocks: &[Block], out: &mut Vec<Declaration>) {
    for block in blocks {
        match block {
            Block::Section(section) => declarations_in(&section.blocks, out),
            Block::Declaration(declaration) => out.push((**declaration).clone()),
        }
    }
}

/// Every proof and declaration of the committed projects that declare
/// document environments, each distinct one once, and every policy with its
/// own axioms and with none.
fn committed() -> (Vec<Input>, usize, usize) {
    let mut declarations = Vec::new();
    for module in linked_modules() {
        declarations_in(&module.blocks, &mut declarations);
    }
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    let mut proofs = 0usize;
    for declaration in &declarations {
        let key = declaration
            .to_json(&mut Renumber::default())
            .to_canonical_string();
        if !seen.insert(key) {
            continue;
        }
        out.push(Input::Declaration(declaration.clone()));
        out.push(Input::Permit(
            declaration.policy.clone(),
            declaration.policy.axioms().to_vec(),
        ));
        out.push(Input::Permit(declaration.policy.clone(), Vec::new()));
        if let DeclBody::TheoremLike { proof, .. } = &declaration.body {
            let mut nested = Vec::new();
            proofs_in(proof, &mut nested);
            for inner in nested {
                if seen.insert(
                    inner
                        .to_json(&mut Renumber::default())
                        .to_canonical_string(),
                ) {
                    out.push(Input::Proof(inner));
                    proofs += 1;
                }
            }
        }
    }
    let count = seen.len() - proofs;
    (out, count, proofs)
}

// ------------------------------------------------------------------ vectors

fn local(id: u64) -> Term {
    Term::Local(LocalId(id))
}

fn locals(ids: &[u64]) -> Vec<LocalId> {
    ids.iter().copied().map(LocalId).collect()
}

fn qualified(package: &str, entry: &str) -> QualifiedId {
    QualifiedId {
        package: package.to_owned(),
        entry: entry.to_owned(),
    }
}

fn names(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

fn app(function: Term, args: Vec<Term>) -> Term {
    Term::App {
        function: Box::new(function),
        explicit_args: args,
        omitted_implicit_binders: Vec::new(),
    }
}

fn external() -> Term {
    Term::Global(
        GlobalRef::External(ExternalConstRef {
            package: "lexlean.std.nat".to_owned(),
            entry: "lexlean.std.nat::nat".to_owned(),
            lean_module: "Init.Prelude".to_owned(),
            lean_name: "Nat".to_owned(),
            signature_hash: Sha256Digest([7; 32]),
        }),
        Vec::new(),
    )
}

fn branch(constructor: &str, lean: &str, binders: &[(u64, &str)], proof: Proof) -> CaseProof {
    CaseProof {
        constructor: qualified("demo", constructor),
        lean_name: lean.to_owned(),
        binders: binders
            .iter()
            .map(|(id, spelling)| (LocalId(*id), (*spelling).to_owned()))
            .collect(),
        proof,
    }
}

fn two_cases() -> Vec<CaseProof> {
    vec![
        branch("zero", "Nat.zero", &[], Proof::Reflexivity),
        branch(
            "succ",
            "Nat.succ",
            &[(8, "n"), (9, "ih")],
            Proof::Exact(local(9)),
        ),
    ]
}

fn declaration(
    kind: DeclKind,
    params: Vec<Binder>,
    body: DeclBody,
    policy: AxiomPolicy,
) -> Declaration {
    Declaration {
        component: "thm-one".to_owned(),
        lean_name: "thm_one".to_owned(),
        kind,
        params,
        body,
        policy,
    }
}

fn binder(id: u64, spelling: &str, ty: Term) -> Binder {
    Binder {
        id: LocalId(id),
        mode: BinderMode::Explicit,
        spelling: spelling.to_owned(),
        ty,
    }
}

struct Vector {
    label: &'static str,
    input: Input,
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let mut out = Vec::new();
    let mut add = |label: &'static str, input: Input| out.push(Vector { label, input });
    let theorem = |proof: Proof| Input::Proof(proof);
    add(
        "a sequence",
        theorem(Proof::Sequence(vec![
            Proof::Intro(locals(&[1, 2])),
            Proof::Exact(local(2)),
        ])),
    );
    add(
        "intro order",
        theorem(Proof::Sequence(vec![
            Proof::Intro(locals(&[5, 3, 6, 5])),
            Proof::Exact(app(local(3), vec![local(5), local(6)])),
        ])),
    );
    add(
        "a have",
        theorem(Proof::Have {
            local: LocalId(4),
            proposition: local(9),
            proof: Box::new(Proof::Exact(local(4))),
        }),
    );
    add(
        "rewrite at a hypothesis",
        theorem(Proof::Rewrite {
            target: RewriteTarget::Hypothesis(LocalId(7)),
            rules: vec![
                RewriteRule {
                    reverse: false,
                    term: local(1),
                },
                RewriteRule {
                    reverse: true,
                    term: local(2),
                },
            ],
        }),
    );
    add(
        "rewrite at a bound hypothesis",
        theorem(Proof::Sequence(vec![
            Proof::Intro(locals(&[3, 7])),
            Proof::Rewrite {
                target: RewriteTarget::Hypothesis(LocalId(7)),
                rules: vec![RewriteRule {
                    reverse: true,
                    term: local(3),
                }],
            },
        ])),
    );
    add(
        "simplify at the goal",
        theorem(Proof::SimplifyOnly {
            target: RewriteTarget::Goal,
            rules: vec![local(1), local(2)],
        }),
    );
    add(
        "cases",
        theorem(Proof::Cases {
            scrutinee: local(3),
            cases: two_cases(),
        }),
    );
    add(
        "induction",
        theorem(Proof::Induction {
            scrutinee: local(3),
            cases: two_cases(),
        }),
    );
    add(
        "a calculation",
        theorem(Proof::Calculate {
            relation: GlobalRef::Core(CoreRef::Eq),
            start: local(1),
            steps: vec![
                CalculationStep {
                    term: local(2),
                    proof: local(10),
                },
                CalculationStep {
                    term: local(3),
                    proof: local(11),
                },
            ],
        }),
    );
    add(
        "apply with premises",
        theorem(Proof::Apply {
            function: local(1),
            premises: vec![Proof::Exact(local(2)), Proof::Reflexivity],
        }),
    );
    add(
        "the small steps",
        theorem(Proof::Sequence(vec![
            Proof::ApplyOne(local(1)),
            Proof::Witness(local(2)),
            Proof::SelectLeft,
            Proof::SelectRight,
            Proof::Constructor(vec![Proof::Reflexivity, Proof::Exact(local(3))]),
        ])),
    );
    add(
        "a theorem with parameters",
        Input::Declaration(declaration(
            DeclKind::Lemma,
            vec![
                binder(1, "x", Term::Sort(Universe::Num(1))),
                binder(2, "h", local(1)),
            ],
            DeclBody::TheoremLike {
                statement: app(local(2), vec![local(1)]),
                proof: Proof::Exact(local(2)),
            },
            AxiomPolicy::Exact(names(&["Quot.sound", "propext"])),
        )),
    );
    add(
        "a definition",
        Input::Declaration(declaration(
            DeclKind::TermDefinition,
            Vec::new(),
            DeclBody::Definition {
                entry: qualified("demo", "two"),
                ty: external(),
                value: Term::NatLiteral {
                    decimal: "2".to_owned(),
                    expected_type: Box::new(external()),
                },
            },
            AxiomPolicy::None,
        )),
    );
    add(
        "a corollary",
        Input::Declaration(declaration(
            DeclKind::Corollary,
            Vec::new(),
            DeclBody::TheoremLike {
                statement: local(5),
                proof: Proof::Reflexivity,
            },
            AxiomPolicy::Allow(names(&["propext", "Classical.choice"])),
        )),
    );
    add(
        "none permits nothing",
        Input::Permit(AxiomPolicy::None, Vec::new()),
    );
    add(
        "none refuses one",
        Input::Permit(AxiomPolicy::None, names(&["propext"])),
    );
    add(
        "allow a subset",
        Input::Permit(AxiomPolicy::Allow(names(&["a", "b"])), names(&["b"])),
    );
    add(
        "allow refuses",
        Input::Permit(AxiomPolicy::Allow(names(&["a"])), names(&["c", "a"])),
    );
    add(
        "exact sorted and unique",
        Input::Permit(
            AxiomPolicy::Exact(names(&["a", "b"])),
            names(&["b", "a", "b"]),
        ),
    );
    add(
        "exact missing one",
        Input::Permit(AxiomPolicy::Exact(names(&["a", "b"])), names(&["a"])),
    );
    add(
        "exact with one more",
        Input::Permit(AxiomPolicy::Exact(names(&["a"])), names(&["a", "b"])),
    );
    add(
        "an exact policy out of order",
        Input::Permit(AxiomPolicy::Exact(names(&["b", "a"])), names(&["a", "b"])),
    );
    out
}

// ------------------------------------------------------------------- seeded

const LOCALS: &[u64] = &[0, 1, 2, 3, 7, 1 << 63];
const AXIOMS: &[&str] = &["propext", "Quot.sound", "Classical.choice", "a", ""];

fn drawn_local(seeded: &mut Seeded) -> LocalId {
    LocalId(*seeded.pick(LOCALS))
}

fn drawn_term(seeded: &mut Seeded) -> Term {
    match seeded.below(3) {
        0 => Term::Local(drawn_local(seeded)),
        1 => app(
            Term::Local(drawn_local(seeded)),
            (0..seeded.below(3))
                .map(|_| Term::Local(drawn_local(seeded)))
                .collect(),
        ),
        _ => Term::Pi {
            binders: vec![binder(
                *seeded.pick(LOCALS),
                "x",
                Term::Local(drawn_local(seeded)),
            )],
            body: Box::new(Term::Local(drawn_local(seeded))),
        },
    }
}

fn drawn_proofs(seeded: &mut Seeded, depth: usize) -> Vec<Proof> {
    (0..seeded.below(3))
        .map(|_| drawn_proof(seeded, depth))
        .collect()
}

fn drawn_cases(seeded: &mut Seeded, depth: usize) -> Vec<CaseProof> {
    (0..seeded.below(3))
        .map(|_| CaseProof {
            constructor: qualified("demo", *seeded.pick(&["zero", "succ", "node"])),
            lean_name: (*seeded.pick(&["Nat.zero", "Nat.succ", "T.node"])).to_owned(),
            binders: (0..seeded.below(3))
                .map(|_| (drawn_local(seeded), (*seeded.pick(&["n", "ih"])).to_owned()))
                .collect(),
            proof: drawn_proof(seeded, depth),
        })
        .collect()
}

fn drawn_target(seeded: &mut Seeded) -> RewriteTarget {
    if seeded.chance(40) {
        RewriteTarget::Goal
    } else {
        RewriteTarget::Hypothesis(drawn_local(seeded))
    }
}

fn drawn_proof(seeded: &mut Seeded, depth: usize) -> Proof {
    let kind = if depth == 0 {
        seeded.below(7)
    } else {
        seeded.below(16)
    };
    let next = depth.saturating_sub(1);
    match kind {
        0 => Proof::Intro((0..seeded.below(4)).map(|_| drawn_local(seeded)).collect()),
        1 => Proof::Exact(drawn_term(seeded)),
        2 => Proof::ApplyOne(drawn_term(seeded)),
        3 => Proof::Reflexivity,
        4 => Proof::Witness(drawn_term(seeded)),
        5 => seeded
            .pick(&[Proof::SelectLeft, Proof::SelectRight])
            .clone(),
        6 => Proof::SimplifyOnly {
            target: drawn_target(seeded),
            rules: (0..seeded.below(3)).map(|_| drawn_term(seeded)).collect(),
        },
        7 => Proof::Sequence(drawn_proofs(seeded, next)),
        8 => Proof::Apply {
            function: drawn_term(seeded),
            premises: drawn_proofs(seeded, next),
        },
        9 => Proof::Have {
            local: drawn_local(seeded),
            proposition: drawn_term(seeded),
            proof: Box::new(drawn_proof(seeded, next)),
        },
        10 => Proof::Rewrite {
            target: drawn_target(seeded),
            rules: (0..seeded.below(3))
                .map(|_| RewriteRule {
                    reverse: seeded.chance(50),
                    term: drawn_term(seeded),
                })
                .collect(),
        },
        11 => Proof::Constructor(drawn_proofs(seeded, next)),
        12 => Proof::Cases {
            scrutinee: drawn_term(seeded),
            cases: drawn_cases(seeded, next),
        },
        13 => Proof::Induction {
            scrutinee: drawn_term(seeded),
            cases: drawn_cases(seeded, next),
        },
        _ => Proof::Calculate {
            relation: GlobalRef::Core(*seeded.pick(&[CoreRef::Eq, CoreRef::Iff])),
            start: drawn_term(seeded),
            steps: (0..seeded.below(3))
                .map(|_| CalculationStep {
                    term: drawn_term(seeded),
                    proof: drawn_term(seeded),
                })
                .collect(),
        },
    }
}

fn drawn_names(seeded: &mut Seeded) -> Vec<String> {
    (0..seeded.below(4))
        .map(|_| (*seeded.pick(AXIOMS)).to_owned())
        .collect()
}

fn drawn_policy(seeded: &mut Seeded) -> AxiomPolicy {
    match seeded.below(3) {
        0 => AxiomPolicy::None,
        1 => AxiomPolicy::Allow(drawn_names(seeded)),
        _ => {
            let mut exact = drawn_names(seeded);
            if seeded.chance(70) {
                exact.sort();
                exact.dedup();
            }
            AxiomPolicy::Exact(exact)
        }
    }
}

fn drawn_declaration(seeded: &mut Seeded) -> Declaration {
    let theorem_like = seeded.chance(60);
    let kind = if theorem_like {
        *seeded.pick(&[DeclKind::Theorem, DeclKind::Lemma, DeclKind::Corollary])
    } else {
        *seeded.pick(&[
            DeclKind::TypeDefinition,
            DeclKind::TermDefinition,
            DeclKind::PredicateDefinition,
        ])
    };
    let params = (0..seeded.below(3))
        .map(|_| binder(*seeded.pick(LOCALS), "p", drawn_term(seeded)))
        .collect();
    let body = if theorem_like {
        DeclBody::TheoremLike {
            statement: drawn_term(seeded),
            proof: drawn_proof(seeded, 2),
        }
    } else {
        DeclBody::Definition {
            entry: qualified("demo", "defined"),
            ty: drawn_term(seeded),
            value: drawn_term(seeded),
        }
    };
    Declaration {
        component: (*seeded.pick(&["a-b", "x", "\"q\""])).to_owned(),
        lean_name: (*seeded.pick(&["a_b", "x"])).to_owned(),
        kind,
        params,
        body,
        policy: drawn_policy(seeded),
    }
}

fn seeded_inputs(count: usize, seed: u64) -> Vec<Input> {
    let mut seeded = Seeded(seed);
    (0..count)
        .flat_map(|_| {
            let depth = seeded.below(4);
            let proof = drawn_proof(&mut seeded, depth);
            let declaration = drawn_declaration(&mut seeded);
            let policy = drawn_policy(&mut seeded);
            let observed = drawn_names(&mut seeded);
            [
                Input::Proof(proof),
                Input::Declaration(declaration),
                Input::Permit(policy, observed),
            ]
        })
        .collect()
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
        label: "a have binds before its proposition",
        site: "(IrTerm::bind (second stated) localId)",
        replacement: "(IrTerm::bind state localId)",
        vector: "a have",
    },
    Plant {
        label: "an intro writes its indices last first",
        site: "(jnil) (first folded) : LexLeanStd.Json::Json)",
        replacement: "(jnil) (LexLeanStd.Order::reverseList [Nat] (first folded)) : LexLeanStd.Json::Json)",
        vector: "intro order",
    },
    Plant {
        label: "the goal is written as a hypothesis",
        site: "(IrTerm::jtext \"goal\")",
        replacement: "(IrTerm::jtext \"hypothesis\")",
        vector: "simplify at the goal",
    },
    Plant {
        label: "every rewrite is forward",
        site: "(ctor LexLeanStd.Json::Json.JBool backward)",
        replacement: "(ctor LexLeanStd.Json::Json.JBool false)",
        vector: "rewrite at a hypothesis",
    },
    Plant {
        label: "a simplification is tagged simp",
        site: "(tagged \"simp-only\"",
        replacement: "(tagged \"simp\"",
        vector: "simplify at the goal",
    },
    Plant {
        label: "a case proof does not see its binders",
        site: "(proofJson branch (second bound))",
        replacement: "(proofJson branch state)",
        vector: "cases",
    },
    Plant {
        label: "an induction is tagged cases",
        site: "(tagged \"induction\"",
        replacement: "(tagged \"cases\"",
        vector: "induction",
    },
    Plant {
        label: "a case writes its constructor as its Lean name",
        site: "(IrTerm::member \"lean\" (ctor LexLeanStd.Json::Json.JStr leanName)",
        replacement: "(IrTerm::member \"lean\" (ctor LexLeanStd.Json::Json.JStr ctorName)",
        vector: "cases",
    },
    Plant {
        label: "a calculation step loses its justification",
        site: "(jobj (IrTerm::member \"t\" (first written) (IrTerm::member \"p\" (first justified) (jnil))))",
        replacement: "(jobj (IrTerm::member \"t\" (first written) (jnil)))",
        vector: "a calculation",
    },
    Plant {
        label: "an apply loses its premises",
        site: "(tagged \"apply\" (IrTerm::member \"f\" (first head) (IrTerm::member \"p\" (jarr (first inner)) (jnil))))",
        replacement: "(tagged \"apply\" (IrTerm::member \"f\" (first head) (IrTerm::member \"p\" (jarr (jnil)) (jnil))))",
        vector: "apply with premises",
    },
    Plant {
        label: "left is written right",
        site: "| Proof.PLeft => ((tagged \"left\" (jnil)), state)",
        replacement: "| Proof.PLeft => ((tagged \"right\" (jnil)), state)",
        vector: "the small steps",
    },
    Plant {
        label: "the parameters are not in scope of the body",
        site: "(bodyJson (project declaration body) (second bound))",
        replacement: "(bodyJson (project declaration body) state)",
        vector: "a theorem with parameters",
    },
    Plant {
        label: "a definition's value is written as its type",
        site: "(IrTerm::member \"v\" (first valued) (jnil))",
        replacement: "(IrTerm::member \"v\" (first written) (jnil))",
        vector: "a definition",
    },
    Plant {
        label: "a corollary is a lemma",
        site: "| DeclKind.KCorollary => (IrTerm::jtext \"corollary\")",
        replacement: "| DeclKind.KCorollary => (IrTerm::jtext \"lemma\")",
        vector: "a corollary",
    },
    Plant {
        label: "the policy names are written last first",
        site: "(LexLeanStd.Order::reverseList [Bytes] names) : LexLeanStd.Json::Json)",
        replacement: "names : LexLeanStd.Json::Json)",
        vector: "a corollary",
    },
    Plant {
        label: "no policy permits anything",
        site: "| AxiomPolicy.ANone => (match observed | List.nil => true | List.cons head rest => false)",
        replacement: "| AxiomPolicy.ANone => (match observed | List.nil => true | List.cons head rest => true)",
        vector: "none refuses one",
    },
    Plant {
        label: "an allowed set forgets a refusal",
        site: "(if allSoFar then (listed allowed named) else false)",
        replacement: "(if allSoFar then (listed allowed named) else true)",
        vector: "allow refuses",
    },
    Plant {
        label: "an exact set keeps duplicates",
        site: "(if (prim equal head (first entry) : Bool) then kept else",
        replacement: "(if false then kept else",
        vector: "exact sorted and unique",
    },
    Plant {
        label: "an exact set is not sorted",
        site: "(LexLeanStd.Order::sortEntries (prim list_fold",
        replacement: "(LexLeanStd.Order::reverseList [(Bytes * Bytes)] (prim list_fold",
        vector: "exact sorted and unique",
    },
    Plant {
        label: "a prefix is the same",
        site: "(if (project walked same) then (match (project walked remaining)",
        replacement: "(if (project walked same) then (match (nil Bytes)",
        vector: "exact missing one",
    },
    Plant {
        label: "a longer list is the same",
        site: "| List.nil => (record Walk {remaining := (nil Bytes), same := false})",
        replacement: "| List.nil => (record Walk {remaining := (nil Bytes), same := true})",
        vector: "exact with one more",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let root = ROOTS[vector.input.root()].0;
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/IrProof.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/IrProof.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, root);
        match super::evaluate(&low, &[vector.input.run()], 1).first() {
            Some(first) => eprintln!(
                "BS-66: plant detected: {}: {}",
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
            "BS-66: {}: largest certificate part {largest} bytes",
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

fn split(inputs: &[Input]) -> [Vec<Run>; 3] {
    let mut out = [Vec::new(), Vec::new(), Vec::new()];
    for input in inputs {
        out[input.root()].push(input.run());
    }
    out
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("IrProof"),
        "IrProof is a linked module of the project"
    );
    eligibility_of(&project, "IrProof", MODULE, &ROOTS);
    assert_eq!(
        constructors("DeclKind").as_slice(),
        [
            "KTypeDefinition",
            "KTermDefinition",
            "KPredicateDefinition",
            "KTheorem",
            "KLemma",
            "KCorollary"
        ],
        "the constructors of DeclKind"
    );

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let named_inputs: Vec<Input> = vectors.iter().map(|vector| vector.input.clone()).collect();

    let (corpus, declarations, proofs) = committed();
    assert!(
        declarations >= 10 && proofs >= 10,
        "the committed projects carry {declarations} distinct declarations and {proofs} distinct proofs"
    );
    let normative = support::normative_host();
    let (evaluator_seeded, rustc_seeded) = if normative {
        (
            seeded_inputs(EVALUATOR_SEEDED, 0x0B55_0066),
            seeded_inputs(RUSTC_SEEDED, 0x0B55_1066),
        )
    } else {
        (Vec::new(), Vec::new())
    };
    eprintln!(
        "BS-66: {} vectors, {declarations} committed declarations and {proofs} proofs, {} and {} seeded inputs of each root ({:?})",
        vectors.len(),
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
        let mut inputs = named_inputs.clone();
        inputs.extend(corpus.iter().cloned());
        inputs.extend(evaluator_seeded.iter().cloned());
        for (index, runs) in split(&inputs).iter().enumerate() {
            let root = ROOTS[index].0;
            let disagreements = super::evaluate(&lowered[root], runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the Rust oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-66: {root}: the evaluator agrees on {} runs ({:?})",
                runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-66: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled = named_inputs;
    compiled.extend(corpus);
    compiled.extend(rustc_seeded);
    let runs = split(&compiled);
    let differential: Vec<(&str, &super::Lowered, &[Run])> = ROOTS
        .iter()
        .enumerate()
        .map(|(index, (root, _))| (*root, &lowered[root], runs[index].as_slice()))
        .collect();
    let total = rustc_differential("BS-66", &differential, "oracle");
    eprintln!("BS-66: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-66") {
        return;
    }
    export(&lowered);
    eprintln!("BS-66: passed in {:?}", started.elapsed());
}
