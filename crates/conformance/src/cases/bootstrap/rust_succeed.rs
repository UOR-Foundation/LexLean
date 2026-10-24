//! `BS-12`: the value a function that can overflow returns, `succeed` of the
//! Rust lowering, as language-1.2 source (`LexLeanBoot.RustSucceed`), held by
//! composition to the Function items of `rust::lower`.
//!
//! `succeed` is private to the renderer, whose files the compiler-semantics
//! digest frames, and it runs on a body between the terms and the fold. So the
//! case composes the roots of `BS-11` with `succeedBodies` between the bodies
//! and the fold, then with `applyMoves`, and compares the result with the
//! Function items of the crate `rust::lower` returns. The programs are those
//! in which some function can overflow and none loops: `tail::looped`, which
//! takes names from the counter the terms take theirs from, is not ported.

use serde_json::Value as Json;

use super::calculus_check::committed;
use super::rust_bodies::{composed, drawn, lower_bodies, programs, vector_in, Mode, Vector};
use super::Run;
use crate::production_export as harness;
use crate::support;

const MODULE: &str = "LexLeanBoot.RustSucceed";
const BODIES: &str = "LexLeanBoot.RustBodies";
const MOVES: &str = "LexLeanBoot.RustMoves";

/// The most a part of a certificate of the root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;

/// The programs the evaluator is given: those of at most this many nodes.
const EVALUATOR_NODES: usize = 300;
const VECTORS: usize = 76;
const VECTORS_WITHIN: usize = 75;
const EVALUATOR_MUTANTS: usize = 166;

/// The lowering up to the moves, as the roots compose it.
struct Roots {
    frame: super::Lowered,
    bodies: super::Lowered,
    succeed: super::Lowered,
    fold: super::Lowered,
}

impl Roots {
    fn of(project: &support::P) -> Self {
        let checked = support::checked_project(project);
        Self {
            frame: super::lower(&checked, project, BODIES, "lowerFrame"),
            bodies: super::lower(&checked, project, BODIES, "lowerBodies"),
            succeed: super::lower(&checked, project, MODULE, "succeedBodies"),
            fold: super::lower(&checked, project, BODIES, "foldBodies"),
        }
    }

    /// The bodies before `succeed`, after it, and after the fold.
    fn run(&self, vector: &Vector) -> (Json, Json, Json) {
        let frame = super::evaluate_once(&self.frame, &vector.frame);
        let (_, bodies, _) = lower_bodies(&self.bodies, &vector.bodies, &frame, 0);
        let mut arguments = vector.succeed.clone();
        arguments.push(serde_json::from_value(bodies.clone()).expect("a value"));
        let succeeded = super::evaluate_once(&self.succeed, &arguments);
        let folded = super::evaluate_once(
            &self.fold,
            &[serde_json::from_value(succeeded.clone()).expect("a value")],
        );
        (bodies, succeeded, folded)
    }
}

/// The runs of the evaluator on `vectors`: where the composed items differ
/// from the oracle's.
fn composition(
    roots: &Roots,
    moves: &super::Lowered,
    vectors: &[&Vector],
    limit: usize,
) -> Vec<String> {
    let mut out = Vec::new();
    for (at, vector) in vectors.iter().enumerate() {
        let (_, _, folded) = roots.run(vector);
        let (input, expected) = composed(vector, &folded);
        let disagreements = super::evaluate(moves, &[(input, expected)], 1);
        if let Some(first) = disagreements.into_iter().next() {
            out.push(format!("vector {at}: {first}"));
            if out.len() >= limit {
                break;
            }
        }
    }
    out
}

/// A defect planted in the source of `RustSucceed`, caught on the composition.
struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
}

const PLANTS: [Plant; 11] = [
    Plant {
        label: "a call in tail position still propagates",
        site: "(if ((mode =? 0) && propagate) then (ctor RustAst::Expr.ECall",
        replacement: "(if false then (ctor RustAst::Expr.ECall",
    },
    Plant {
        label: "an application in tail position still propagates",
        site: "(if ((mode =? 0) && propagate) then (ctor RustAst::Expr.EApply",
        replacement: "(if false then (ctor RustAst::Expr.EApply",
    },
    Plant {
        label: "a unit literal is wrapped like any other value",
        site: "(if ((mode =? 0) && (RustBodies::unitLit whole)) then",
        replacement: "(if false then",
    },
    Plant {
        label: "a computed unit is not bound before the success",
        site: "(if unit then (unitBound whole) else (succeeded whole))",
        replacement: "(succeeded whole)",
    },
    Plant {
        label: "the then-branch of a conditional is not succeeded",
        site: "(succeedExpr t t unit 1) (succeedExpr e e unit 1)",
        replacement: "t (succeedExpr e e unit 1)",
    },
    Plant {
        label: "the else-branch of a conditional is not succeeded",
        site: "(succeedExpr t t unit 1) (succeedExpr e e unit 1)",
        replacement: "(succeedExpr t t unit 1) e",
    },
    Plant {
        label: "the body of an arm is not succeeded",
        site: "(succeedExpr block block unit 1) (succeedExpr rest rest unit 2)",
        replacement: "block (succeedExpr rest rest unit 2)",
    },
    Plant {
        label: "the arms after the first are not succeeded",
        site: "(succeedExpr block block unit 1) (succeedExpr rest rest unit 2)",
        replacement: "(succeedExpr block block unit 1) rest",
    },
    Plant {
        label: "a function that cannot overflow is succeeded",
        site: "(if ((RustLower::flagAt fallible index) && ",
        replacement: "(if (true && ",
    },
    Plant {
        label: "the flags are read at the first function",
        site: "(succeedChain results fallible tail (index + 1))",
        replacement: "(succeedChain results fallible tail index)",
    },
    Plant {
        label: "the tail of a block is not succeeded",
        site: "(RustBodies::setTail (ctor RustAst::Expr.EBlock lets tail) (succeedExpr tail tail unit 0))",
        replacement: "(ctor RustAst::Expr.EBlock lets tail)",
    },
];

fn plants(moves: &super::Lowered, detection: &[&Vector]) {
    let mut missed = Vec::new();
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/RustSucceed.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/RustSucceed.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let roots = Roots::of(&planted);
        let disagreements = composition(&roots, moves, detection, 1);
        match disagreements.first() {
            Some(first) => eprintln!(
                "BS-12: plant detected: {}: {}",
                plant.label,
                &first[..first.len().min(160)]
            ),
            None => missed.push(plant.label),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects the composition does not detect: {missed:?}"
    );
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("RustSucceed"),
        "RustSucceed is a linked module of the project"
    );
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report = super::rust_lower::eligibility_report(&project, &build_id, MODULE);
    let rows = report["roots"].as_array().expect("roots");
    assert_eq!(rows.len(), 1, "the root of RustSucceed");
    assert_eq!(rows[0]["root"], format!("{MODULE}.succeedBodies"));
    super::rust_lower::assert_eligible(
        &rows[0],
        "succeedBodies",
        &["allocation", "overflow", "recursion"],
    );
    let sources = committed();
    let overflowing = |program: &lexlean::calculus::Program| vector_in(program, Mode::Overflowing);
    let all: Vec<Vector> = programs(&sources).iter().filter_map(overflowing).collect();
    let within: Vec<&Vector> = all
        .iter()
        .filter(|vector| vector.nodes <= EVALUATOR_NODES)
        .collect();
    let mutated: Vec<Vector> = drawn(&sources, 0x00B5_0112)
        .iter()
        .filter_map(overflowing)
        .collect();
    eprintln!(
        "BS-12: {} vectors ({} within {EVALUATOR_NODES} nodes), {} mutants ({:?})",
        all.len(),
        within.len(),
        mutated.len(),
        started.elapsed()
    );
    assert_eq!(
        (all.len(), within.len(), mutated.len()),
        (VECTORS, VECTORS_WITHIN, EVALUATOR_MUTANTS),
        "the vectors, those the evaluator is given, and the mutants"
    );
    let checked = support::checked_project(&project);
    let moves = super::lower(&checked, &project, MOVES, "applyMoves");
    let roots = Roots::of(&project);
    if !support::normative_host() {
        eprintln!("BS-12: platform-bound host: the evaluator, the composition, and the planted defects run on the normative host");
        return;
    }
    let runs: Vec<(Json, Json, Json)> = within.iter().map(|vector| roots.run(vector)).collect();
    let disagreements = composition(&roots, &moves, &within, 5);
    assert!(
        disagreements.is_empty(),
        "the roots composed with applyMoves and the oracle disagree on the vectors: {disagreements:?}"
    );
    let mutated_refs: Vec<&Vector> = mutated.iter().collect();
    let disagreements = composition(&roots, &moves, &mutated_refs, 5);
    assert!(
        disagreements.is_empty(),
        "the roots composed with applyMoves and the oracle disagree on the mutants: {disagreements:?}"
    );
    eprintln!(
        "BS-12: the composition agrees on {} vectors and {} mutants ({:?})",
        within.len(),
        mutated.len(),
        started.elapsed()
    );
    let detection: Vec<&Vector> = within.iter().copied().chain(mutated_refs).collect();
    plants(&moves, &detection);
    // The package is run last: its stack bound is a record of the export.
    let succeed_runs: Vec<Run> = within
        .iter()
        .zip(&runs)
        .map(|(vector, (bodies, succeeded, _))| {
            let mut arguments = vector.succeed.clone();
            arguments.push(serde_json::from_value(bodies.clone()).expect("a value"));
            (arguments, succeeded.clone())
        })
        .collect();
    super::rust_validate::rustc_differential(
        "BS-12",
        &[("succeedBodies", &roots.succeed, &succeed_runs)],
        "evaluator",
    );
    if !support::lean_backed("BS-12") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let root = exported
        .output
        .result
        .roots
        .iter()
        .find(|exported_root| exported_root.root == format!("{MODULE}.succeedBodies"))
        .expect("the root is exported");
    let largest = root
        .outputs
        .iter()
        .flat_map(|output| &output.certification.certificates)
        .map(|certificate| certificate.file.byte_length)
        .max()
        .expect("a certificate");
    eprintln!("BS-12: succeedBodies: largest certificate part {largest} bytes");
    assert!(
        largest <= CERTIFICATE_BUDGET,
        "a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}"
    );
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
            serde_json::to_value(&roots.succeed.program).expect("a program is JSON"),
            "the exported package realizes the program the differential ran"
        );
    }
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
    eprintln!("BS-12: passed in {:?}", started.elapsed());
}
