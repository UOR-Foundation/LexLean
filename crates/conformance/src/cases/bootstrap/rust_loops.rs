//! `BS-13`: a function whose self calls are all tail calls, rendered as a
//! loop, `tail::looped` of the Rust lowering, as language-1.2 source
//! (`LexLeanBoot.RustLoops`), held by composition to the Function items of
//! `rust::lower`.
//!
//! `looped` is private to the renderer, whose files the compiler-semantics
//! digest frames. In `lower` each function runs the terms, `succeed`, the
//! fold, and `looped`, in this order and on one counter of fresh names, so the
//! names of the next function's terms follow the names `looped` took. The case
//! composes the roots of `BS-11` and `BS-12` with `loopFunction` per function,
//! passing the counter from one root to the next, and then with `applyMoves`;
//! the result is compared with the Function items of the crate `rust::lower`
//! returns. The parameters and the result type of a function are inputs, as
//! they are to the rows before this one: a looping function's parameters are
//! read back from the oracle's item, where the loop renamed the ones its state
//! replaces.

use lexlean::calculus::rust::ast::{Expr as RustExpr, Ident, ItemDef, Pat, Type};
use lexlean::calculus::{Expr, Function, Prim, Program, Shape, Ty, Value};
use serde_json::Value as Json;

use super::calculus_check::{committed, nat, program_of};
use super::rust_bodies::{
    arm, drawn, function, lower_bodies, natural, programs, var, vector_in, Mode, Vector,
};
use super::rust_lower::flags;
use super::rust_print::{enc_item_def, enc_items, enc_params};
use super::rust_validate::enc_type;
use super::Run;
use crate::production_export as harness;
use crate::support;

const MODULE: &str = "LexLeanBoot.RustLoops";
const BODIES: &str = "LexLeanBoot.RustBodies";
const SUCCEED: &str = "LexLeanBoot.RustSucceed";
const MOVES: &str = "LexLeanBoot.RustMoves";

/// The most a part of a certificate of the root may be, as the row states.
const CERTIFICATE_BUDGET: u64 = 3_145_728;

/// The programs the evaluator is given: those of at most this many nodes.
const EVALUATOR_NODES: usize = 300;
const VECTORS: usize = 88;
const VECTORS_WITHIN: usize = 43;
const EVALUATOR_MUTANTS: usize = 107;

fn prim(operation: Prim, operands: Vec<Expr>) -> Expr {
    Expr::Prim {
        operation,
        operands,
    }
}

fn call(function: u64, operands: Vec<Expr>) -> Expr {
    Expr::Call { function, operands }
}

/// A function that counts `n` down to zero and loops. The rules below put it
/// beside the function they are about, so that each program is one that loops.
fn countdown() -> Function {
    function(
        vec![(0, Ty::Nat), (1, Ty::Nat)],
        Ty::Nat,
        Expr::Cond {
            condition: Box::new(prim(Prim::NatEq, vec![var(0), natural("0")])),
            then_branch: Box::new(var(1)),
            else_branch: Box::new(call(
                0,
                vec![
                    prim(Prim::NatSub, vec![var(0), natural("1")]),
                    prim(Prim::NatAdd, vec![var(1), natural("1")]),
                ],
            )),
        },
    )
}

fn predecessor() -> Expr {
    prim(Prim::NatSub, vec![var(0), natural("1")])
}

/// The programs of the rules the corpora do not reach: a function that has a
/// tail self call and one that is not in tail position, whether it is a
/// binding, a condition, or the scrutinee of a match, and a function with a
/// parameter nothing reads, which stays a call.
pub(super) fn rules() -> Vec<Program> {
    let beside = |other: Function| program_of(Vec::new(), vec![countdown(), other]);
    vec![
        beside(function(
            vec![(0, Ty::Nat)],
            Ty::Nat,
            Expr::Let {
                name: 1,
                ty: Ty::Nat,
                bound: Box::new(call(1, vec![predecessor()])),
                body: Box::new(Expr::Cond {
                    condition: Box::new(prim(Prim::NatEq, vec![var(0), natural("0")])),
                    then_branch: Box::new(var(1)),
                    else_branch: Box::new(call(1, vec![predecessor()])),
                }),
            },
        )),
        beside(function(
            vec![(0, Ty::Nat)],
            Ty::Nat,
            Expr::Cond {
                condition: Box::new(prim(Prim::NatEq, vec![call(1, vec![var(0)]), natural("0")])),
                then_branch: Box::new(call(1, vec![predecessor()])),
                else_branch: Box::new(var(0)),
            },
        )),
        beside(function(
            vec![(0, Ty::Nat)],
            Ty::Nat,
            Expr::Match {
                ty: Ty::Nat,
                scrutinee: Box::new(call(1, vec![var(0)])),
                arms: vec![
                    arm(Shape::Zero, Vec::new(), var(0)),
                    arm(Shape::Succ, vec![2], call(1, vec![var(2)])),
                ],
            },
        )),
        beside(function(
            vec![(0, Ty::Nat), (1, Ty::Nat)],
            Ty::Nat,
            Expr::Cond {
                condition: Box::new(prim(Prim::NatEq, vec![var(0), natural("0")])),
                then_branch: Box::new(natural("0")),
                else_branch: Box::new(call(1, vec![predecessor(), natural("0")])),
            },
        )),
    ]
}

pub(super) struct Roots {
    pub(super) frame: super::Lowered,
    pub(super) bodies: super::Lowered,
    pub(super) succeed: super::Lowered,
    pub(super) fold: super::Lowered,
    pub(super) looped: super::Lowered,
}

/// What the roots made of one function: the arguments `loopFunction` was
/// given and what it returned, and the parameters and body that go into the
/// item.
pub(super) struct Step {
    pub(super) run: Run,
    pub(super) parameters: Value,
    pub(super) body: Value,
}

/// The element `at` of a chain of expressions.
pub(super) fn nth(chain: &Json, at: usize) -> Value {
    let mut current: Value = serde_json::from_value(chain.clone()).expect("a value");
    for _ in 0..at {
        let Value::Adt { mut fields, .. } = current else {
            panic!("a chain");
        };
        current = fields.pop().expect("the rest");
    }
    let Value::Adt { mut fields, .. } = current else {
        panic!("a chain");
    };
    fields.swap_remove(0)
}

/// The parameters of a function as the lowering holds them before the loop
/// renames the ones its state replaces, and the result type: the item's, when
/// the function does not loop.
fn before_loop(item: &ItemDef, loops: bool) -> (Vec<(Pat, Type)>, Type) {
    let ItemDef::Function {
        parameters,
        result,
        body,
        ..
    } = item
    else {
        panic!("a function");
    };
    if !loops {
        return (parameters.clone(), result.clone());
    }
    let RustExpr::Loop { body: looping, .. } = &body.tail else {
        panic!("a loop");
    };
    let mut tuple = looping.lets[0].pat.clone();
    let changed: Vec<usize> = parameters
        .iter()
        .enumerate()
        .filter(|(_, (pat, _))| matches!(pat, Pat::Bind(Ident::Param(_))))
        .map(|(position, _)| position)
        .collect();
    let mut restored = parameters.clone();
    for (at, position) in changed.iter().enumerate() {
        if at + 1 < changed.len() {
            let Pat::Tuple(mut items) = tuple else {
                panic!("a tuple of the state");
            };
            let rest = items.pop().expect("the rest");
            restored[*position].0 = items.pop().expect("the first");
            tuple = rest;
        } else {
            restored[*position].0 = tuple.clone();
        }
    }
    (restored, result.clone())
}

impl Roots {
    pub(super) fn of(project: &support::P) -> Self {
        let checked = support::checked_project(project);
        Self {
            frame: super::lower(&checked, project, BODIES, "lowerFrame"),
            bodies: super::lower(&checked, project, BODIES, "lowerBodies"),
            succeed: super::lower(&checked, project, SUCCEED, "succeedBodies"),
            fold: super::lower(&checked, project, BODIES, "foldBodies"),
            looped: super::lower(&checked, project, MODULE, "loopFunction"),
        }
    }

    /// The function at `position`, lowered alone: the terms from the counter
    /// `start`, `succeed`, the fold, and the loop.
    pub(super) fn function(
        &self,
        vector: &Vector,
        frame: &Json,
        position: usize,
        start: u64,
        signature: (Value, Value),
    ) -> Result<(Step, u64), String> {
        let mask: Vec<bool> = (0..vector.dead.len()).map(|at| at != position).collect();
        let mut given = vector.bodies.clone();
        let dead_at = given.len() - 2;
        given[dead_at] = flags(mask);
        let (_, bodies, terms) = lower_bodies(&self.bodies, &given, frame, start);
        let mut arguments = vector.succeed.clone();
        arguments.push(serde_json::from_value(bodies).expect("a value"));
        let succeeded = super::evaluate_once(&self.succeed, &arguments);
        let folded = super::evaluate_once(
            &self.fold,
            &[serde_json::from_value(succeeded).expect("a value")],
        );
        let body = nth(&folded, position);
        let (parameters, result) = signature;
        let arguments = vec![
            nat(position as u64),
            parameters.clone(),
            result,
            body.clone(),
            nat(terms),
        ];
        let returned = super::evaluate_once(&self.looped, &arguments);
        let Value::Adt { mut fields, .. } =
            serde_json::from_value(returned.clone()).expect("a value")
        else {
            panic!("a structure");
        };
        let Some(Value::Nat { value }) = fields.pop() else {
            panic!("the counter");
        };
        let after: u64 = value.parse().expect("a counter");
        let looped_body = fields.pop().expect("the body");
        let looped_parameters = fields.pop().expect("the parameters");
        let Some(Value::Bool { value: found }) = fields.pop() else {
            panic!("the flag");
        };
        if found != vector.loops[position] {
            return Err(format!(
                "function {position}: loopFunction and the calculus disagree on whether it loops"
            ));
        }
        let step = if found {
            Step {
                run: (arguments, returned),
                parameters: looped_parameters,
                body: looped_body,
            }
        } else {
            Step {
                run: (arguments, returned),
                parameters,
                body,
            }
        };
        Ok((step, after))
    }

    /// Every live function in order, with the counter passed on: the steps,
    /// and the items put together for `applyMoves`, with what the oracle holds.
    fn run(&self, vector: &Vector) -> Result<(Vec<Step>, Vec<Value>, Json), String> {
        let frame = super::evaluate_once(&self.frame, &vector.frame);
        let mut start = 0u64;
        let mut steps = Vec::new();
        let mut items = Vec::new();
        for (position, item) in vector.items.iter().enumerate() {
            let mut encoded = enc_item_def(item);
            if !vector.dead[position] {
                let (parameters, result) = before_loop(item, vector.loops[position]);
                let signature = (enc_params(&parameters), enc_type(&result));
                let (step, after) = self.function(vector, &frame, position, start, signature)?;
                start = after;
                if let Value::Adt { fields, .. } = &mut encoded {
                    fields[1] = step.parameters.clone();
                    fields[3] = step.body.clone();
                }
                steps.push(step);
            }
            items.push(encoded);
        }
        let input = items
            .into_iter()
            .rev()
            .fold(super::calculus_check::adt(3, Vec::new()), |tail, item| {
                super::calculus_check::adt(4, vec![item, tail])
            });
        Ok((
            steps,
            vec![input],
            serde_json::to_value(enc_items(&vector.items)).expect("a value is JSON"),
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
) -> Vec<String> {
    let mut out = Vec::new();
    for (at, vector) in vectors.iter().enumerate() {
        let (_, input, expected) = match roots.run(vector) {
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

/// A defect planted in the source of `RustLoops`, caught on the composition.
struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
}

const PLANTS: [Plant; 12] = [
    Plant {
        label: "a call to the function itself is not found by name",
        site: "(if (called =? me) then true else (occurs args me))",
        replacement: "(occurs args me)",
    },
    Plant {
        label: "a binding that calls the function does not stop the loop",
        site: "((not (occurs lets me)) && (project last ok))",
        replacement: "(project last ok)",
    },
    Plant {
        label: "a parameter passed on as a clone is a change",
        site: "| RustValidate::ReadKind.RkClone => (RustBodies::idEq read name)",
        replacement: "| RustValidate::ReadKind.RkClone => false",
    },
    Plant {
        label: "the type of the state is nested to the left",
        site: "(ctor RustValidate::BinaryKind.BkPair) ty (project more ty)",
        replacement: "(ctor RustValidate::BinaryKind.BkPair) (project more ty) ty",
    },
    Plant {
        label: "the pattern of the state is nested to the left",
        site: "(ctor RustAst::Pat.PTuple (ctor RustAst::Pat.PCons whole (ctor RustAst::Pat.PCons (project more tuple) (ctor RustAst::Pat.PNil))))",
        replacement: "(ctor RustAst::Pat.PTuple (ctor RustAst::Pat.PCons (project more tuple) (ctor RustAst::Pat.PCons whole (ctor RustAst::Pat.PNil))))",
    },
    Plant {
        label: "a renamed parameter takes the name of the holder",
        site: "(start + (2 + j))",
        replacement: "(start + (1 + j))",
    },
    Plant {
        label: "the counter after the loop is one short",
        site: "next := ((start + 1) + (project made count))",
        replacement: "next := (start + (project made count))",
    },
    Plant {
        label: "the else-branch of a conditional is not retargeted",
        site: "(retarget e e plan 1)",
        replacement: "e",
    },
    Plant {
        label: "the arms after the first are not retargeted",
        site: "(retarget rest rest plan 2)",
        replacement: "rest",
    },
    Plant {
        label: "the next state is read from the second argument",
        site: "(nextState (project plan params) (project plan calls) args 0 at)",
        replacement: "(nextState (project plan params) (project plan calls) args 1 at)",
    },
    Plant {
        label: "a parameter every call passes on is part of the state",
        site: "(not (passesAll calls position name))",
        replacement: "true",
    },
    Plant {
        label: "a function with a parameter no pattern binds still loops",
        site: "((not (project made ok)) || ((project made count) =? 0))",
        replacement: "((project made count) =? 0)",
    },
];

fn plants(moves: &super::Lowered, detection: &[&Vector]) {
    let mut missed = Vec::new();
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/RustLoops.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/RustLoops.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let roots = Roots::of(&planted);
        let disagreements = composition(&roots, moves, detection, 1);
        match disagreements.first() {
            Some(first) => eprintln!(
                "BS-13: plant detected: {}: {}",
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
        checked_units.units.contains_key("RustLoops"),
        "RustLoops is a linked module of the project"
    );
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report = super::rust_lower::eligibility_report(&project, &build_id, MODULE);
    let rows = report["roots"].as_array().expect("roots");
    assert_eq!(rows.len(), 1, "the root of RustLoops");
    assert_eq!(rows[0]["root"], format!("{MODULE}.loopFunction"));
    super::rust_lower::assert_eligible(
        &rows[0],
        "loopFunction",
        &["allocation", "overflow", "recursion"],
    );
    let sources = committed();
    let looping = |program: &Program| vector_in(program, Mode::Looping);
    let mut corpus = programs(&sources);
    corpus.extend(rules());
    let all: Vec<Vector> = corpus.iter().filter_map(looping).collect();
    let within: Vec<&Vector> = all
        .iter()
        .filter(|vector| vector.nodes <= EVALUATOR_NODES)
        .collect();
    let mutated: Vec<Vector> = drawn(&sources, 0x00B5_0113)
        .iter()
        .filter_map(looping)
        .collect();
    eprintln!(
        "BS-13: {} vectors ({} within {EVALUATOR_NODES} nodes), {} mutants ({:?})",
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
        eprintln!("BS-13: platform-bound host: the evaluator, the composition, and the planted defects run on the normative host");
        return;
    }
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
        "BS-13: the composition agrees on {} vectors and {} mutants ({:?})",
        within.len(),
        mutated.len(),
        started.elapsed()
    );
    let detection: Vec<&Vector> = within.iter().copied().chain(mutated_refs).collect();
    plants(&moves, &detection);
    // The package is run last: its stack bound is a record of the export.
    let loop_runs: Vec<Run> = within
        .iter()
        .flat_map(|vector| roots.run(vector).expect("the roots run").0)
        .map(|step| step.run)
        .collect();
    super::rust_validate::rustc_differential(
        "BS-13",
        &[("loopFunction", &roots.looped, &loop_runs)],
        "evaluator",
    );
    if !support::lean_backed("BS-13") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let root = exported
        .output
        .result
        .roots
        .iter()
        .find(|exported_root| exported_root.root == format!("{MODULE}.loopFunction"))
        .expect("the root is exported");
    let largest = root
        .outputs
        .iter()
        .flat_map(|output| &output.certification.certificates)
        .map(|certificate| certificate.file.byte_length)
        .max()
        .expect("a certificate");
    eprintln!("BS-13: loopFunction: largest certificate part {largest} bytes");
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
            serde_json::to_value(&roots.looped.program).expect("a program is JSON"),
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
    eprintln!("BS-13: passed in {:?}", started.elapsed());
}
