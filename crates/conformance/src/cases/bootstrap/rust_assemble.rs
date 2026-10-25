//! `BS-14`: the crate around the bodies of the Rust lowering, as language-1.2
//! source (`LexLeanBoot.RustAssemble`), held by composition to the whole crate
//! `rust::lower` returns.
//!
//! What `Lowering::lower` does outside the passes of the earlier rows is the
//! order of the items, the parameters of each function (a name when the body
//! mentions it and its type is not unit), its result type (wrapped when the
//! function can overflow), and the body of a function with an uninhabited
//! parameter, which is the empty match on that parameter. `functionShells`
//! makes a Function item of each function with a placeholder body for a live
//! one, and `assembleItems` puts the items of `lowerTypes` (`BS-07`), the
//! shells, and what the loop pass returned for each live function into one
//! chain. The case composes that with the roots of `BS-11`, `BS-12`, and
//! `BS-13` function by function and with `applyMoves`, and compares the whole
//! chain of items, the items of the types too, with the crate. The profile is
//! the host's input.

use serde_json::Value as Json;

use super::calculus_check::{committed, program_of};
use super::rust_bodies::{drawn, function, programs, var, vector_in, Mode, Vector};
use super::rust_loops::{nth, rules, Roots as Passes};
use super::rust_lower::flags;
use super::rust_print::enc_items;
use super::Run;
use crate::production_export as harness;
use crate::support;
use lexlean::calculus::{Program, Ty};

/// The programs of the rules the corpora do not reach: a function whose
/// uninhabited parameter (a function type no closure inhabits) is not its first, beside
/// one whose only parameter is.
fn empty_rules() -> Vec<Program> {
    let empty = Ty::Fn {
        parameters: vec![Ty::Nat],
        result: Box::new(Ty::Nat),
    };
    vec![
        program_of(
            Vec::new(),
            vec![function(
                vec![(0, Ty::Nat), (1, empty.clone())],
                Ty::Nat,
                var(0),
            )],
        ),
        program_of(
            Vec::new(),
            vec![function(
                vec![(0, Ty::Bool), (1, Ty::Nat), (2, empty), (3, Ty::Nat)],
                Ty::Nat,
                var(3),
            )],
        ),
    ]
}

const MODULE: &str = "LexLeanBoot.RustAssemble";
const LOWER: &str = "LexLeanBoot.RustLower";
const MOVES: &str = "LexLeanBoot.RustMoves";

/// The most a part of a certificate of a root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;
const ROOTS: [&str; 2] = ["functionShells", "assembleItems"];

/// The programs the evaluator is given: those of at most this many nodes.
const EVALUATOR_NODES: usize = 300;
const VECTORS: usize = 324;
const VECTORS_WITHIN: usize = 276;
const EVALUATOR_MUTANTS: usize = 537;

/// What a host that did more than thread the counter would get wrong: the
/// case runs the composition with each of these and requires the comparison
/// with the crate to see it.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Fault {
    /// The honest host.
    None,
    /// The counter of the fresh names starts again at each function.
    Renumbers,
    /// The functions are lowered last to first, the counter threaded so.
    Reorders,
    /// The loop pass of the first live function is left out of the items.
    Drops,
}

const FAULTS: [(&str, Fault); 3] = [
    (
        "the counter starts again at each function",
        Fault::Renumbers,
    ),
    ("the functions are lowered last to first", Fault::Reorders),
    (
        "the items of the first live function are not put in",
        Fault::Drops,
    ),
];

struct Roots {
    passes: Passes,
    types: super::Lowered,
    shells: super::Lowered,
    assemble: super::Lowered,
}

/// What a vector gives the roots, and what they returned.
struct Made {
    shells: (Vec<super::Value>, Json),
    assembly: (Vec<super::Value>, Json),
}

impl Roots {
    fn of(project: &support::P) -> Self {
        let checked = support::checked_project(project);
        Self {
            passes: Passes::of(project),
            types: super::lower(&checked, project, LOWER, "lowerTypes"),
            shells: super::lower(&checked, project, MODULE, "functionShells"),
            assemble: super::lower(&checked, project, MODULE, "assembleItems"),
        }
    }

    /// The crate's items as the roots make them, ready for `applyMoves`, and
    /// the arguments and results of the roots on the way.
    fn run(
        &self,
        vector: &Vector,
        fault: Fault,
    ) -> Result<(Made, Vec<super::Value>, Json), String> {
        let frame = super::evaluate_once(&self.passes.frame, &vector.frame);
        let fallible = vector.bodies[vector.bodies.len() - 1].clone();
        // The program: the names, the types, the results, and the bodies.
        let mut shell_arguments: Vec<super::Value> = vector.frame[0..=5].to_vec();
        shell_arguments.push(fallible.clone());
        shell_arguments.push(serde_json::from_value(frame.clone()).expect("a value"));
        let shells = super::evaluate_once(&self.shells, &shell_arguments);
        let mut type_arguments = vector.frame.clone();
        type_arguments.push(fallible);
        let types = super::evaluate_once(&self.types, &type_arguments);
        let mut start = 0u64;
        let mut outs = Vec::new();
        let mut order: Vec<usize> = (0..vector.dead.len()).collect();
        if fault == Fault::Reorders {
            order.reverse();
        }
        for position in order {
            if vector.dead[position] {
                continue;
            }
            let super::Value::Adt { mut fields, .. } = nth(&shells, position) else {
                return Err(format!("function {position}: a shell"));
            };
            let result = fields.swap_remove(2);
            let parameters = fields.swap_remove(1);
            let (step, after) =
                self.passes
                    .function(vector, &frame, position, start, (parameters, result))?;
            start = if fault == Fault::Renumbers { 0 } else { after };
            outs.push(serde_json::from_value::<super::Value>(step.run.1.clone()).expect("a value"));
        }
        if fault == Fault::Drops && !outs.is_empty() {
            outs.remove(0);
        }
        let assembly_arguments = vec![
            serde_json::from_value(types).expect("a value"),
            serde_json::from_value(shells.clone()).expect("a value"),
            super::list(outs),
            flags(vector.dead.clone()),
        ];
        let items = super::evaluate_once(&self.assemble, &assembly_arguments);
        let moved_input = vec![serde_json::from_value(items.clone()).expect("a value")];
        let expected =
            serde_json::to_value(enc_items(&vector.crate_items)).expect("a value is JSON");
        Ok((
            Made {
                shells: (shell_arguments, shells),
                assembly: (assembly_arguments, items),
            },
            moved_input,
            expected,
        ))
    }
}

/// The runs of the evaluator on `vectors`: where the composed items differ
/// from the oracle's.
fn composition(
    roots: &Roots,
    moves: &super::Lowered,
    vectors: &[&Vector],
    limit: usize,
    fault: Fault,
) -> Vec<String> {
    let mut out = Vec::new();
    for (at, vector) in vectors.iter().enumerate() {
        let (_, input, expected) = match roots.run(vector, fault) {
            Ok(composed) => composed,
            Err(reason) => {
                out.push(format!("vector {at}: {reason}"));
                if out.len() >= limit {
                    break;
                }
                continue;
            }
        };
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

/// A defect planted in the source of `RustAssemble`, caught on the composition.
struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
}

const PLANTS: [Plant; 11] = [
    Plant {
        label: "a name for a parameter the body does not mention",
        site: "(RustBodies::mentions body name)",
        replacement: "true",
    },
    Plant {
        label: "a parameter of unit type is named",
        site: "(not (CalculusCheck::tyEq type (ctor CalculusCheck::Ty.TUnit)))",
        replacement: "true",
    },
    Plant {
        label: "a result that can overflow is not wrapped",
        site: "(if failing then",
        replacement: "(if false then",
    },
    Plant {
        label: "no parameter is found uninhabited",
        site:
            "(if (RustLower::visible head inhabited) then (firstEmpty tail inhabited (index + 1))",
        replacement: "(if true then (firstEmpty tail inhabited (index + 1))",
    },
    Plant {
        label: "the uninhabited parameter is not bound",
        site: "((position + 1) =? empty)",
        replacement: "false",
    },
    Plant {
        label: "the empty match is on the first parameter",
        site: "(emptyBody (nameAt names empty) (typeAt types empty))",
        replacement: "(emptyBody (nameAt names 0) (typeAt types empty))",
    },
    Plant {
        label: "a function type is named an ADT in an origin",
        site: "(prim utf8_encode \"type:fn\" : Bytes)",
        replacement: "(prim utf8_encode \"type:adt\" : Bytes)",
    },
    Plant {
        label: "a dead function takes a loop result",
        site: "(if (RustLower::flagAt dead position)",
        replacement: "(if false",
    },
    Plant {
        label: "the items of the types come after the functions",
        site: "(appendDefs types (placeAll shells outs dead 0))",
        replacement: "(appendDefs (placeAll shells outs dead 0) types)",
    },
    Plant {
        label: "a loop's renamed parameters are not put in",
        site:
            "(ctor RustAst::Def.DFunction name (project out params) result (project out body) at)",
        replacement: "(ctor RustAst::Def.DFunction name params result (project out body) at)",
    },
    Plant {
        label: "the position of a function is not advanced",
        site: "fallible known inhabited (position + 1))",
        replacement: "fallible known inhabited position)",
    },
];

fn plants(moves: &super::Lowered, detection: &[&Vector]) {
    let mut missed = Vec::new();
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/RustAssemble.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/RustAssemble.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let roots = Roots::of(&planted);
        let disagreements = composition(&roots, moves, detection, 1, Fault::None);
        match disagreements.first() {
            Some(first) => eprintln!(
                "BS-14: plant detected: {}: {}",
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
        checked_units.units.contains_key("RustAssemble"),
        "RustAssemble is a linked module of the project"
    );
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report = super::rust_lower::eligibility_report(&project, &build_id, MODULE);
    let rows = report["roots"].as_array().expect("roots");
    assert_eq!(rows.len(), ROOTS.len(), "the roots of RustAssemble");
    for (name, row) in ROOTS.iter().zip(rows) {
        assert_eq!(row["root"], format!("{MODULE}.{name}"));
        super::rust_lower::assert_eligible(row, name, &["allocation", "overflow", "recursion"]);
    }
    let sources = committed();
    let mut corpus = programs(&sources);
    corpus.extend(rules());
    corpus.extend(empty_rules());
    let every = |program: &lexlean::calculus::Program| vector_in(program, Mode::Any);
    let all: Vec<Vector> = corpus.iter().filter_map(every).collect();
    let within: Vec<&Vector> = all
        .iter()
        .filter(|vector| vector.nodes <= EVALUATOR_NODES)
        .collect();
    let mutated: Vec<Vector> = drawn(&sources, 0x00B5_0114)
        .iter()
        .filter_map(every)
        .collect();
    eprintln!(
        "BS-14: {} vectors ({} within {EVALUATOR_NODES} nodes), {} mutants ({:?})",
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
        eprintln!("BS-14: platform-bound host: the evaluator, the composition, and the planted defects run on the normative host");
        return;
    }
    let disagreements = composition(&roots, &moves, &within, 5, Fault::None);
    assert!(
        disagreements.is_empty(),
        "the roots composed with applyMoves and the oracle disagree on the vectors: {disagreements:?}"
    );
    let mutated_refs: Vec<&Vector> = mutated.iter().collect();
    let disagreements = composition(&roots, &moves, &mutated_refs, 5, Fault::None);
    assert!(
        disagreements.is_empty(),
        "the roots composed with applyMoves and the oracle disagree on the mutants: {disagreements:?}"
    );
    eprintln!(
        "BS-14: the composition agrees on {} vectors and {} mutants ({:?})",
        within.len(),
        mutated.len(),
        started.elapsed()
    );
    let detection: Vec<&Vector> = within.iter().copied().chain(mutated_refs).collect();
    plants(&moves, &detection);
    for (label, fault) in FAULTS {
        let seen = composition(&roots, &moves, &detection, 1, fault);
        match seen.first() {
            Some(first) => eprintln!(
                "BS-14: host fault detected: {label}: {}",
                &first[..first.len().min(160)]
            ),
            None => panic!("a host that does this is not detected: {label}"),
        }
    }
    // The packages are run last: their stack bounds are records of the export.
    let made: Vec<Made> = within
        .iter()
        .map(|vector| roots.run(vector, Fault::None).expect("the roots run").0)
        .collect();
    let shell_runs: Vec<Run> = made.iter().map(|made| made.shells.clone()).collect();
    let assembly_runs: Vec<Run> = made.iter().map(|made| made.assembly.clone()).collect();
    super::rust_validate::rustc_differential(
        "BS-14",
        &[
            ("functionShells", &roots.shells, &shell_runs),
            ("assembleItems", &roots.assemble, &assembly_runs),
        ],
        "evaluator",
    );
    if !support::lean_backed("BS-14") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let low = match exported_root.root.strip_prefix(&format!("{MODULE}.")) {
            Some("functionShells") => &roots.shells,
            Some("assembleItems") => &roots.assemble,
            _ => continue,
        };
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-14: {}: largest certificate part {largest} bytes",
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
                "{}: the exported package realizes the program the differential ran",
                exported_root.root
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
    eprintln!("BS-14: passed in {:?}", started.elapsed());
}
