//! `BS-08`: the last-use moves of SPEC.md §17.16
//! (`calculus::rust::moves::apply`) as language-1.2 source
//! (`LexLeanBoot.RustMoves`), held to the oracle to the byte. It is the first
//! pass of the function bodies of the lowering, a case of its own.
//!
//! The root takes the items of a crate as the `Def` chain of `RustAst` and
//! returns them with every function body rewritten. `apply` is public, so
//! this pass is held to it directly on any block; the corpora are crates: the
//! hand-built crates of every rule of the pass, the hand-built crates of
//! `BS-05`, the crates the renderer lowers from the committed programs, and
//! seeded mutants of the smaller of those.

use lexlean::calculus::rust::ast::{
    Block, Callee, Crate, Expr, Ident, ItemDef, Let, Lit, Origin, Pat,
};
use lexlean::calculus::rust::moves;
use lexlean::calculus::rust::Profile;
use serde_json::Value as Json;

use super::rust_lower::{assert_eligible, eligibility_report};
use super::rust_print::enc_items;
use super::rust_validate::{
    branch_crates, committed, loop_crates, machine_crates, mutant, pool_of, rendered, small, weight,
};
use super::{Run, Seeded};
use crate::production_export as harness;
use crate::support;

const MODULE: &str = "LexLeanBoot.RustMoves";

/// Three quarters of `max_file_bytes` (4,194,304), the most a part of a
/// certificate of this root may be.
const CERTIFICATE_BUDGET: u64 = 3_145_728;

/// The crates of at most this many expressions are the evaluator's share.
const EVALUATOR_NODES: usize = 400;
/// The crates the moves are run on, and those within the evaluator's share.
const CRATES: usize = 362;
const WITHIN: usize = 318;
const EVALUATOR_MUTANTS: usize = 150;
const RUSTC_MUTANTS: usize = 1_500;

/// The crates a mutation draws on have at most this many expressions.
const BASE_NODES: usize = 80;

fn at() -> Origin {
    Origin::of("hand")
}

fn local(n: u64) -> Ident {
    Ident::Local(n)
}

fn cloned(n: u64) -> Expr {
    Expr::Clone(local(n), at())
}

fn unit() -> Expr {
    Expr::Lit(Lit::Unit, at())
}

fn block(lets: Vec<Let>, tail: Expr) -> Block {
    Block { lets, tail }
}

fn bind(n: u64, value: Expr) -> Let {
    Let {
        pat: Pat::Bind(local(n)),
        ty: None,
        value,
        at: at(),
    }
}

/// `let (h, _) = ...` or `let (_, h) = ...` of a pair, the projection the pass
/// moves out of a pair.
fn project(part: usize, n: u64, value: Expr) -> Let {
    let mut parts = vec![Pat::Wild, Pat::Wild];
    parts[part] = Pat::Bind(local(n));
    Let {
        pat: Pat::Tuple(parts),
        ty: None,
        value,
        at: at(),
    }
}

fn pair(left: Expr, right: Expr) -> Expr {
    Expr::Pair(Box::new(left), Box::new(right), at())
}

fn call(args: Vec<Expr>) -> Expr {
    Expr::Call {
        callee: Callee::Function(0),
        args,
        propagate: false,
        at: at(),
    }
}

fn function(body: Block) -> ItemDef {
    ItemDef::Function {
        name: Ident::Function(0),
        parameters: Vec::new(),
        result: lexlean::calculus::rust::ast::Type::Unit,
        body,
        at: at(),
    }
}

fn krate(body: Block) -> Crate {
    Crate {
        profile: Profile::Std,
        items: vec![function(body)],
    }
}

fn arm(pattern: Pat, body: Block) -> (Pat, Block) {
    (pattern, body)
}

/// The crates of the rules of the pass: a read that is the last, one that is
/// not, the projection of a pair component and of both, alternatives, a match
/// with and without arms, a loop, an application, and the blocks in between.
fn rule_crates() -> Vec<Crate> {
    let conditional = |condition: Expr, then_branch: Block, else_branch: Block| Expr::If {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
        at: at(),
    };
    let matched = |scrutinee: Expr, arms: Vec<(Pat, Block)>| Expr::Match {
        scrutinee: Box::new(scrutinee),
        arms,
        at: at(),
    };
    vec![
        // The only read.
        krate(block(Vec::new(), cloned(0))),
        // Read, then read again: only the second can move.
        krate(block(vec![bind(1, cloned(0))], pair(cloned(0), cloned(1)))),
        // The same read twice in one call.
        krate(block(Vec::new(), call(vec![cloned(0), cloned(0)]))),
        // A bound value read once, and one read once and then bound again.
        krate(block(
            vec![bind(1, cloned(0)), bind(0, cloned(1))],
            cloned(0),
        )),
        // The projection of the first component, alone.
        krate(block(vec![project(0, 1, cloned(0))], cloned(1))),
        // The projection of the second component, with the pair read whole
        // afterwards.
        krate(block(
            vec![project(1, 1, cloned(0))],
            pair(cloned(1), cloned(0)),
        )),
        // Both components projected, each once.
        krate(block(
            vec![project(0, 1, cloned(0)), project(1, 2, cloned(0))],
            pair(cloned(1), cloned(2)),
        )),
        // A component projected twice.
        krate(block(
            vec![project(0, 1, cloned(0)), project(0, 2, cloned(0))],
            pair(cloned(1), cloned(2)),
        )),
        // A component projected while another is read twice after it: the
        // pair cannot lose a component that the later reads of the other
        // would need whole.
        krate(block(
            vec![
                project(0, 1, cloned(0)),
                project(1, 2, cloned(0)),
                project(1, 3, cloned(0)),
            ],
            pair(pair(cloned(1), cloned(2)), cloned(3)),
        )),
        // A component read once on each of two alternatives is read once, so
        // the other component may be projected before them.
        krate(block(
            vec![project(1, 1, cloned(0))],
            conditional(
                cloned(5),
                block(vec![project(0, 2, cloned(0))], cloned(2)),
                block(vec![project(0, 3, cloned(0))], cloned(3)),
            ),
        )),
        // A projection of a pattern that binds both, which is not one.
        krate(block(
            vec![Let {
                pat: Pat::Tuple(vec![Pat::Bind(local(1)), Pat::Bind(local(2))]),
                ty: None,
                value: cloned(0),
                at: at(),
            }],
            pair(cloned(1), cloned(2)),
        )),
        // A projection followed by a pattern that binds both components: the
        // second reads the pair whole, so the first cannot move it.
        krate(block(
            vec![
                project(1, 1, cloned(0)),
                Let {
                    pat: Pat::Tuple(vec![Pat::Bind(local(2)), Pat::Bind(local(3))]),
                    ty: None,
                    value: cloned(0),
                    at: at(),
                },
            ],
            pair(pair(cloned(1), cloned(2)), cloned(3)),
        )),
        // Alternatives: a read in one branch only, in both, and after.
        krate(block(
            Vec::new(),
            conditional(
                cloned(3),
                block(Vec::new(), cloned(0)),
                block(Vec::new(), unit()),
            ),
        )),
        krate(block(
            Vec::new(),
            conditional(
                cloned(3),
                block(Vec::new(), cloned(0)),
                block(Vec::new(), cloned(0)),
            ),
        )),
        krate(block(
            vec![bind(
                1,
                conditional(
                    cloned(3),
                    block(Vec::new(), cloned(0)),
                    block(Vec::new(), unit()),
                ),
            )],
            pair(cloned(1), cloned(0)),
        )),
        // The condition read again in a branch.
        krate(block(
            Vec::new(),
            conditional(
                cloned(0),
                block(Vec::new(), cloned(0)),
                block(Vec::new(), unit()),
            ),
        )),
        // A match: the scrutinee, an arm that binds, an arm that reads
        // outside, and a pattern that rebinds a name read after.
        krate(block(
            Vec::new(),
            matched(
                cloned(0),
                vec![
                    arm(
                        Pat::Some(Box::new(Pat::Bind(local(1)))),
                        block(Vec::new(), cloned(1)),
                    ),
                    arm(Pat::None, block(Vec::new(), cloned(2))),
                ],
            ),
        )),
        krate(block(
            vec![bind(
                3,
                matched(
                    cloned(0),
                    vec![
                        arm(Pat::Bind(local(0)), block(Vec::new(), cloned(0))),
                        arm(Pat::Wild, block(Vec::new(), cloned(4))),
                    ],
                ),
            )],
            pair(cloned(3), cloned(0)),
        )),
        // A match without arms.
        krate(block(Vec::new(), matched(cloned(0), Vec::new()))),
        // An application: the holder is read before the operands.
        krate(block(
            Vec::new(),
            Expr::Apply {
                holder: Ident::Callee(0),
                args: vec![cloned(1), cloned(0)],
                propagate: false,
                at: at(),
            },
        )),
        krate(block(
            vec![bind(1, cloned(2))],
            Expr::Apply {
                holder: local(2),
                args: vec![cloned(1)],
                propagate: true,
                at: at(),
            },
        )),
        // The closure applied is also an operand: it is borrowed for the
        // whole application, so the operand cannot move it.
        krate(block(
            Vec::new(),
            Expr::Apply {
                holder: local(0),
                args: vec![cloned(0)],
                propagate: false,
                at: at(),
            },
        )),
        // Loops: a read of the state, of another binding, and of both.
        krate(block(
            Vec::new(),
            Expr::Loop {
                state: local(0),
                body: Box::new(block(vec![bind(1, cloned(0))], cloned(2))),
            },
        )),
        krate(block(
            vec![bind(5, cloned(2))],
            pair(
                Expr::Loop {
                    state: local(0),
                    body: Box::new(block(Vec::new(), pair(cloned(0), cloned(2)))),
                },
                cloned(2),
            ),
        )),
        // A read of the state before the loop that rebinds it: the loop reads
        // it whole, so the earlier read cannot move it.
        krate(block(
            vec![bind(1, cloned(0))],
            Expr::Loop {
                state: local(0),
                body: Box::new(block(Vec::new(), cloned(1))),
            },
        )),
        // Blocks as expressions, and wrappers.
        krate(block(
            Vec::new(),
            Expr::Block(Box::new(block(vec![bind(1, cloned(0))], cloned(1)))),
        )),
        krate(block(
            Vec::new(),
            Expr::Not(
                Box::new(Expr::Box(Box::new(pair(cloned(0), cloned(0))), at())),
                at(),
            ),
        )),
        // Reads that are not clones.
        krate(block(
            vec![bind(1, Expr::Copy(local(0), at()))],
            pair(cloned(0), Expr::Deref(local(1), at())),
        )),
    ]
}

/// The oracle's move of every function body of a crate, as the root takes and
/// returns the items.
pub(super) fn run(krate: &Crate) -> Run {
    let input = enc_items(&krate.items);
    let mut after = krate.clone();
    for item in &mut after.items {
        if let ItemDef::Function { body, .. } = item {
            moves::apply(body);
        }
    }
    (
        vec![input],
        serde_json::to_value(enc_items(&after.items)).expect("a value is JSON"),
    )
}

/// The crates the pass is run on: the rule crates, the hand-built crates of
/// `BS-05`, and the crates the renderer lowers from the committed programs.
pub(super) fn base() -> Vec<Crate> {
    let mut out = rule_crates();
    out.extend(branch_crates());
    out.extend(machine_crates());
    out.extend(loop_crates());
    let (sources, ..) = committed();
    out.extend(rendered(&sources).into_iter().map(|one| one.krate));
    out
}

/// `count` seeded mutants of the crates the renderer lowers from the
/// committed programs.
pub(super) fn mutants(count: usize, seed: u64) -> Vec<Crate> {
    let (sources, ..) = committed();
    let lowered = rendered(&sources);
    // The mutants of the compiler's own crates are too large to hold by the
    // thousand: the bases are the crates of at most `BASE_NODES` expressions,
    // as `BS-05` draws its mutants.
    let bases = small(&lowered, BASE_NODES);
    let mut seeded = Seeded(seed);
    bases
        .iter()
        .cycle()
        .take(count)
        .map(|one| mutant(&one.krate, &mut seeded, &pool_of(&one.krate)))
        .collect()
}

/// A defect planted in the source of `RustMoves`, caught on the runs of the
/// moves.
struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
}

const PLANTS: [Plant; 8] = [
    Plant {
        label: "a last read never moves",
        site: "    | Option.some found => (((project found whole) =? 0) && ((prim length (project found parts) : Nat) =? 0))\n    | Option.none => true)",
        replacement: "    | Option.some found => false\n    | Option.none => false)",
    },
    Plant {
        label: "a component moves though another is read twice",
        site: "(atMostOne (project found parts))",
        replacement: "true",
    },
    Plant {
        label: "the reads of a component on two alternatives are added",
        site: "(natMax (second head) (partCount left (first head)))",
        replacement: "((second head) + (partCount left (first head)))",
    },
    Plant {
        label: "the holder of an application is not read before its operands",
        site: "(let held : (List (RustValidate::Id * Later)) := (readWhole live holder); (ctor RustAst::Expr.EApply holder (moveE args held) propagate o))",
        replacement: "(ctor RustAst::Expr.EApply holder (moveE args live) propagate o)",
    },
    Plant {
        label: "the left of a pair is read before its right",
        site: "(let after : (List (RustValidate::Id * Later)) := (readsE right live 0); (ctor RustAst::Expr.EPair (moveE left after) (moveE right live) o))",
        replacement: "(let after : (List (RustValidate::Id * Later)) := (readsE left live 0); (ctor RustAst::Expr.EPair (moveE left live) (moveE right after) o))",
    },
    Plant {
        label: "a pattern does not unbind its names",
        site: "| RustAst::Pat.PBind ident => (liveRemove live ident)",
        replacement: "| RustAst::Pat.PBind ident => live",
    },
    Plant {
        label: "a pattern that binds both components is a projection",
        site: "(((boundCount parts) =? 1) && (1 <? (partsLength parts)))",
        replacement: "(1 <? (partsLength parts))",
    },
    Plant {
        label: "the state of a loop is not read",
        site: "(readWhole ran state)",
        replacement: "ran",
    },
];

/// Each defect planted in a copy of `RustMoves` is a disagreement with the
/// oracle on the runs of the moves.
fn plants(detection: &[Run]) {
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/RustMoves.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/RustMoves.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, "applyMoves");
        let disagreements = super::evaluate(&low, detection, 1);
        assert!(
            !disagreements.is_empty(),
            "{}: the planted defect is detected by the differential",
            plant.label
        );
        eprintln!(
            "BS-08: plant detected: {}: {}",
            plant.label, disagreements[0]
        );
    }
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("RustMoves"),
        "RustMoves is a linked module of the project"
    );
    // The one root of the module: eligible for rust-std with exactly its
    // declared effects.
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report = eligibility_report(&project, &build_id, MODULE);
    let rows = report["roots"].as_array().expect("roots");
    assert_eq!(rows.len(), 1, "applyMoves is the one root of RustMoves");
    assert_eq!(rows[0]["root"], format!("{MODULE}.applyMoves"));
    assert_eligible(
        &rows[0],
        "applyMoves",
        &["allocation", "overflow", "recursion"],
    );
    let base = base();
    let runs: Vec<Run> = base.iter().map(run).collect();
    let small_runs: Vec<Run> = base
        .iter()
        .zip(&runs)
        .filter(|(krate, _)| weight(krate) <= EVALUATOR_NODES)
        .map(|(_, run)| run.clone())
        .collect();
    let evaluator_mutants: Vec<Run> = mutants(EVALUATOR_MUTANTS, 0x0B58_0103)
        .iter()
        .map(run)
        .collect();
    let rustc_mutants: Vec<Run> = mutants(RUSTC_MUTANTS, 0x0B58_0203)
        .iter()
        .map(run)
        .collect();
    assert_eq!(
        (
            base.len(),
            small_runs.len(),
            evaluator_mutants.len(),
            rustc_mutants.len()
        ),
        (CRATES, WITHIN, EVALUATOR_MUTANTS, RUSTC_MUTANTS),
        "the crates the moves are run on, those within the evaluator's bound, and the mutants"
    );
    eprintln!(
        "BS-08: {} crates ({} within {EVALUATOR_NODES} expressions), {} + {} mutants ({:?})",
        base.len(),
        small_runs.len(),
        evaluator_mutants.len(),
        rustc_mutants.len(),
        started.elapsed()
    );
    let checked = support::checked_project(&project);
    let moving = super::lower(&checked, &project, MODULE, "applyMoves");
    // On every host: the rendered rust-std package, compiled by that host's
    // pinned rustc, agrees with the oracle on the crates.
    super::rust_validate::rustc_differential("BS-08", &[("applyMoves", &moving, &runs)], "crates");
    if !support::normative_host() {
        eprintln!("BS-08: platform-bound host: the evaluator, the seeded corpus, and the planted defects run on the normative host");
        return;
    }
    for (label, runs) in [
        ("crates", &small_runs),
        ("seeded mutants", &evaluator_mutants),
    ] {
        let disagreements = super::evaluate(&moving, runs, 5);
        assert!(
            disagreements.is_empty(),
            "the evaluator and the oracle disagree on the {label}: {disagreements:?}"
        );
    }
    eprintln!(
        "BS-08: the evaluator agrees on {} crates and {} seeded mutants ({:?})",
        small_runs.len(),
        evaluator_mutants.len(),
        started.elapsed()
    );
    super::rust_validate::rustc_differential(
        "BS-08",
        &[("applyMoves", &moving, &rustc_mutants)],
        "seeded",
    );
    let detection: Vec<Run> = small_runs
        .iter()
        .chain(&evaluator_mutants)
        .cloned()
        .collect();
    plants(&detection);
    // Under the pinned Lean, the production export certifies the root; its
    // package is the committed one and realizes the program the differential
    // ran.
    if !support::lean_backed("BS-08") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        if exported_root.root != format!("{MODULE}.applyMoves") {
            continue;
        }
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-08: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}, three quarters of max_file_bytes",
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
                serde_json::to_value(&moving.program).expect("a program is JSON"),
                "applyMoves: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, 1, "one certified package for the root");
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
    eprintln!("BS-08: passed in {:?}", started.elapsed());
}
