//! `BS-11`: the lowering of the terms of a function body, `Lowering::expr`
//! with its operands, its conditionals, its matches and the fold of
//! `lower`, as language-1.2 source (`LexLeanBoot.RustBodies`), held by
//! composition to the Function items of `rust::lower`.
//!
//! `expr`, `fold`, and the rest are private to the renderer, whose files the
//! compiler-semantics digest frames, and the last pass of a body, the moves,
//! is `BS-08`'s root. So the root returns each live body as `lower` holds it
//! before the moves, and the case composes it with `applyMoves` and compares
//! the result with the Function items of the crate that `rust::lower`
//! returns. The programs are those the passes that follow do not touch: no
//! function of them can overflow (`succeed` is not ported) and none loops
//! (`tail::looped` is not ported, and it takes names from the counter the
//! terms take theirs from).
//!
//! Four roots are held: `useTrees` of `CalculusCheck`, the types the lowering
//! reads beside the tree of `siteTrees` (the operands of each primitive and
//! the value of each field), to `calculus::sites::uses`; and the three of
//! `RustBodies`, by their composition: `lowerFrame` (what the analysis of
//! `BS-07` knows that the terms read: the numbered function types, the boxes,
//! the inhabited types, the closures), `lowerBodies` (the terms, given the
//! frame), and `foldBodies` (the fold).

use lexlean::calculus::rust::ast::{Ident, ItemDef};
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::sites;
use lexlean::calculus::{Arm, Expr, Function, Program, Shape, Ty, Value};
use serde_json::Value as Json;

use super::calculus_check::{
    committed, literal_corpus, mutants, prim_corpus, program_arguments, program_of, program_size,
    shape_corpus, targeted, Source,
};
use super::rust_lower::{analysis, enc_site, flags, lowered};
use super::rust_print::{enc_item_def, enc_items};
use super::{Run, Seeded};
use crate::production_export as harness;
use crate::support;

const MODULE: &str = "LexLeanBoot.RustBodies";
/// The roots of the module in the order it declares them.
const ROOTS: [&str; 3] = ["lowerFrame", "lowerBodies", "foldBodies"];
const TYPING: &str = "LexLeanBoot.CalculusCheck";
const MOVES: &str = "LexLeanBoot.RustMoves";

/// The most a part of a certificate of these roots may be, as the row states:
/// 87.5% of `max_file_bytes`, above the 75% of the rule of #92 because the term
/// lowering is one recursion family. The source hands a binder of the matched
/// type through `keepE` and `keepP` until certificate B accepts it (#310).
const CERTIFICATE_BUDGET: u64 = 5_505_024;

const LITERALS: usize = 200;
const PRIM_ROUNDS: usize = 3;
/// The programs the evaluator is given: those of at most this many nodes.
const EVALUATOR_NODES: usize = 300;
const BASE_NODES: usize = 150;
/// The mutants drawn, of which the checker and the filters keep some.
const MUTANT_DRAWS: usize = 3_000;
const VECTORS: usize = 156;
const VECTORS_WITHIN: usize = 155;
const EVALUATOR_MUTANTS: usize = 320;

/// A program with what the roots are given and the oracle's items.
pub(super) struct Vector {
    pub(super) nodes: usize,
    /// The arguments of `useTrees` and the tree it returns.
    pub(super) uses: Run,
    /// The arguments of `lowerFrame`, and of `lowerBodies` before the frame.
    pub(super) frame: Vec<Value>,
    pub(super) bodies: Vec<Value>,
    /// The Function items of the crate `rust::lower` returns.
    pub(super) items: Vec<ItemDef>,
    /// The arguments of `succeedBodies` before the bodies: the result types
    /// and the functions that can overflow.
    pub(super) succeed: Vec<Value>,
    /// The functions the analysis finds dead, and those that loop.
    pub(super) dead: Vec<bool>,
    pub(super) loops: Vec<bool>,
}

/// Which programs a row holds: the passes that a program reaches decide it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Mode {
    /// No function can overflow and none loops.
    Plain,
    /// Some function can overflow and none loops.
    Overflowing,
    /// Some function loops.
    Looping,
}

/// A program is a vector when the oracle lowers it and no pass this row does
/// not carry changes its bodies.
fn vector(program: &Program) -> Option<Vector> {
    vector_in(program, Mode::Plain)
}

/// [`vector`] for the programs `mode` holds: a pass is a row of its own, and
/// a row needs programs that reach it.
pub(super) fn vector_in(program: &Program, mode: Mode) -> Option<Vector> {
    let krate = rust::lower(program, Profile::Std).ok()?;
    let fallible = rust::fallible_functions(program).ok()?;
    let loops: Vec<bool> = program
        .functions
        .iter()
        .enumerate()
        .map(|(position, function)| rust::loops(position as u64, function))
        .collect();
    let held = match mode {
        Mode::Plain => !fallible.contains(&true) && !loops.contains(&true),
        Mode::Overflowing => fallible.contains(&true) && !loops.contains(&true),
        Mode::Looping => loops.contains(&true),
    };
    if !held {
        return None;
    }
    let (dead, _) = lowered(program)?;
    let base = program_arguments(program);
    let tree = enc_site(&sites::trees(program).ok()?);
    let used = sites::uses(program).ok()?;
    let mut frame = base.clone();
    frame.extend([tree.clone(), flags(dead.clone())]);
    let mut bodies = base.clone();
    bodies.extend([
        tree,
        enc_site(&used),
        flags(dead.clone()),
        flags(fallible.clone()),
    ]);
    let succeed = vec![base[4].clone(), flags(fallible)];
    let items = krate
        .items
        .into_iter()
        .filter(|item| matches!(item, ItemDef::Function { .. }))
        .collect();
    Some(Vector {
        nodes: program_size(program),
        uses: (
            base,
            serde_json::to_value(enc_site(&used)).expect("a value is JSON"),
        ),
        frame,
        bodies,
        items,
        succeed,
        dead,
        loops,
    })
}

fn unit() -> Expr {
    Expr::Value {
        ty: Ty::Unit,
        value: Value::Unit,
    }
}

pub(super) fn natural(value: &str) -> Expr {
    Expr::Value {
        ty: Ty::Nat,
        value: Value::Nat {
            value: value.to_owned(),
        },
    }
}

pub(super) fn var(name: u64) -> Expr {
    Expr::Var { name }
}

pub(super) fn function(parameters: Vec<(u64, Ty)>, result: Ty, body: Expr) -> Function {
    Function {
        parameters: parameters.iter().map(|(name, _)| *name).collect(),
        types: parameters.into_iter().map(|(_, ty)| ty).collect(),
        result,
        body,
    }
}

pub(super) fn arm(shape: Shape, binders: Vec<u64>, body: Expr) -> Arm {
    Arm {
        shape,
        binders,
        body,
    }
}

/// The programs of the rules the corpora of the earlier rows do not reach: a
/// binding with a stated type that is returned, an operand of unit type that
/// a call computes, a field an arm loads from a box, and a conditional whose
/// then-branch is another that has no else.
fn rules() -> Vec<Program> {
    let list = |constructor| Shape::Adt { constructor };
    let tail = Expr::Match {
        ty: Ty::Nat,
        scrutinee: Box::new(var(2)),
        arms: vec![
            arm(list(0), Vec::new(), natural("0")),
            arm(list(1), vec![3, 4], var(3)),
        ],
    };
    vec![
        program_of(
            Vec::new(),
            vec![function(
                Vec::new(),
                Ty::Nat,
                Expr::Let {
                    name: 1,
                    ty: Ty::Nat,
                    bound: Box::new(natural("1")),
                    body: Box::new(var(1)),
                },
            )],
        ),
        program_of(
            Vec::new(),
            vec![
                function(vec![(0, Ty::Unit), (1, Ty::Nat)], Ty::Nat, var(1)),
                function(Vec::new(), Ty::Unit, unit()),
                function(
                    Vec::new(),
                    Ty::Nat,
                    Expr::Call {
                        function: 0,
                        operands: vec![
                            Expr::Call {
                                function: 1,
                                operands: Vec::new(),
                            },
                            natural("1"),
                        ],
                    },
                ),
            ],
        ),
        program_of(
            vec![vec![Vec::new(), vec![Ty::Nat, Ty::Adt { index: 0 }]]],
            vec![function(
                vec![(0, Ty::Adt { index: 0 })],
                Ty::Nat,
                Expr::Match {
                    ty: Ty::Nat,
                    scrutinee: Box::new(var(0)),
                    arms: vec![
                        arm(list(0), Vec::new(), natural("0")),
                        arm(list(1), vec![1, 2], tail),
                    ],
                },
            )],
        ),
        program_of(
            Vec::new(),
            vec![
                function(Vec::new(), Ty::Unit, unit()),
                function(
                    vec![(0, Ty::Bool), (1, Ty::Bool)],
                    Ty::Unit,
                    Expr::Cond {
                        condition: Box::new(var(0)),
                        then_branch: Box::new(Expr::Cond {
                            condition: Box::new(var(1)),
                            then_branch: Box::new(Expr::Call {
                                function: 0,
                                operands: Vec::new(),
                            }),
                            else_branch: Box::new(unit()),
                        }),
                        else_branch: Box::new(unit()),
                    },
                ),
            ],
        ),
    ]
}

/// The programs of the corpora: the rules of `BS-07`, the programs of
/// `BS-04`, the committed programs, and seeded mutants of the smaller ones.
pub(super) fn programs(sources: &[Source]) -> Vec<Program> {
    let mut seeded = Seeded(0x00B5_0011);
    let mut out: Vec<Program> = analysis();
    out.extend(rules());
    out.extend(targeted());
    out.extend(literal_corpus(&mut seeded, LITERALS));
    out.extend(prim_corpus(&mut seeded, PRIM_ROUNDS));
    out.extend(shape_corpus(&mut seeded));
    out.extend(sources.iter().map(|source| source.program.clone()));
    out
}

pub(super) fn drawn(sources: &[Source], seed: u64) -> Vec<Program> {
    mutants(sources, &mut Seeded(seed), MUTANT_DRAWS, BASE_NODES)
}

/// The bodies a run of `lowerBodies` returned, put into the Function items
/// of the oracle in place of its own, as the chain `applyMoves` takes, and
/// the items the oracle holds.
pub(super) fn composed(vector: &Vector, bodies: &Json) -> (Vec<Value>, Json) {
    let mut chain = Vec::new();
    let mut current: Value = serde_json::from_value(bodies.clone()).expect("a value");
    while let Value::Adt {
        constructor: 13,
        mut fields,
    } = current
    {
        let rest = fields.pop().expect("the rest");
        let head = fields.pop().expect("the head");
        chain.push(head);
        current = rest;
    }
    let mut position = 0usize;
    let mut items = Vec::new();
    for item in &vector.items {
        let mut encoded = enc_item_def(item);
        if let ItemDef::Function {
            name: Ident::Function(at),
            ..
        } = item
        {
            let body = chain.get(usize::try_from(*at).expect("an index"));
            if let (Some(body), Value::Adt { fields, .. }) = (body, &mut encoded) {
                // A dead function is returned as the empty chain: its body is
                // the oracle's own.
                if !matches!(
                    body,
                    Value::Adt {
                        constructor: 12,
                        ..
                    }
                ) {
                    fields[3] = body.clone();
                }
            }
            position += 1;
        }
        items.push(encoded);
    }
    let _ = position;
    let input = items
        .into_iter()
        .rev()
        .fold(super::calculus_check::adt(3, Vec::new()), |tail, item| {
            super::calculus_check::adt(4, vec![item, tail])
        });
    (
        vec![input],
        serde_json::to_value(enc_items(&vector.items)).expect("a value is JSON"),
    )
}

/// The roots of the pass as lowered programs.
struct Roots {
    frame: super::Lowered,
    bodies: super::Lowered,
    fold: super::Lowered,
}

impl Roots {
    fn of(project: &support::P) -> Self {
        let checked = support::checked_project(project);
        Self {
            frame: super::lower(&checked, project, MODULE, "lowerFrame"),
            bodies: super::lower(&checked, project, MODULE, "lowerBodies"),
            fold: super::lower(&checked, project, MODULE, "foldBodies"),
        }
    }

    /// The frame the analysis returns, what `lowerBodies` returns, the bodies
    /// in it, and the bodies after the fold.
    fn run(&self, vector: &Vector) -> (Json, Json, Json, Json) {
        let frame = super::evaluate_once(&self.frame, &vector.frame);
        let (lowered, bodies, _) = lower_bodies(&self.bodies, &vector.bodies, &frame, 0);
        let folded = super::evaluate_once(
            &self.fold,
            &[serde_json::from_value(bodies.clone()).expect("a value")],
        );
        (frame, lowered, bodies, folded)
    }
}

/// `lowerBodies` on `arguments` given the frame the analysis returned and the
/// counter the fresh names continue from: what it returns, the bodies in it,
/// and the counter after them.
pub(super) fn lower_bodies(
    root: &super::Lowered,
    arguments: &[Value],
    frame: &Json,
    start: u64,
) -> (Json, Json, u64) {
    let mut given = arguments.to_vec();
    given.push(serde_json::from_value(frame.clone()).expect("a value"));
    given.push(super::calculus_check::nat(start));
    let lowered = super::evaluate_once(root, &given);
    let Value::Adt { mut fields, .. } = serde_json::from_value(lowered.clone()).expect("a value")
    else {
        panic!("lowerBodies returns a structure");
    };
    let Some(Value::Nat { value }) = fields.pop() else {
        panic!("the counter after the bodies");
    };
    let bodies = fields.pop().expect("the bodies");
    (
        lowered,
        serde_json::to_value(bodies).expect("a value is JSON"),
        value.parse().expect("a counter"),
    )
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
        let (_, _, _, folded) = roots.run(vector);
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

/// A defect planted in the source of `RustBodies`, caught on the composition.
struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
}

const PLANTS: [Plant; 13] = [
    Plant {
        label: "a variable of a copy type is cloned",
        site: "(if (RustLower::copyTy ty) then (ctor RustValidate::ReadKind.RkCopy) else (ctor RustValidate::ReadKind.RkClone))",
        replacement: "(ctor RustValidate::ReadKind.RkClone)",
    },
    Plant {
        label: "a binding with a stated type is folded into the value it returns",
        site: "(if typed then false else ",
        replacement: "(if false then false else ",
    },
    Plant {
        label: "a conditional on true and false is not the condition",
        site: "(((literalCode thenB) =? 1) && ((literalCode elseBranch) =? 2))",
        replacement: "false",
    },
    Plant {
        label: "a conditional on false and true is not the negated condition",
        site: "(((literalCode thenB) =? 2) && ((literalCode elseBranch) =? 1))",
        replacement: "false",
    },
    Plant {
        label: "a computed unit operand is not bound first",
        site: "(if (anyComputedUnit rendered types) then",
        replacement: "(if false then",
    },
    Plant {
        label: "a match whose arms rebuild what they matched is kept",
        site: "(if ((not (isNilE arms)) && (allRebuild arms)) then scrutinee",
        replacement: "(if false then scrutinee",
    },
    Plant {
        label: "the predecessor of a successor arm is not bound",
        site: "(if (isWildPat predecessor) then succBlock else",
        replacement: "(if true then succBlock else",
    },
    Plant {
        label: "a boxed field of an arm is not loaded",
        site: "(RustLower::cyclic known edges (ctor CalculusCheck::Ty.TAdt adt) (consHead types))",
        replacement: "false",
    },
    Plant {
        label: "a closure's function type keeps the captured parameters",
        site: "(RustLower::tyDrop (calleeTypes ctx function) count)",
        replacement: "(calleeTypes ctx function)",
    },
    Plant {
        label: "a conversion's operand is not widened",
        site: "(if convert then",
        replacement: "(if false then",
    },
    Plant {
        label: "two branches of one code are never the same code",
        site: "(if (tokensEq (project one toks) (project two toks)) then",
        replacement: "(if false then",
    },
    Plant {
        label: "the condition of an inner conditional is not held",
        site: "(if ((emptyBlock elseBranch) && ((isNilE (blockLets thenBranch)) && (thenHoldsIf (blockTail thenBranch)))) then",
        replacement: "(if false then",
    },
    Plant {
        label: "a literal condition is not bound first",
        site: "(if (0 <? (boolCode condition)) then",
        replacement: "(if false then",
    },
];

fn plants(moves: &super::Lowered, detection: &[&Vector]) {
    let mut missed = Vec::new();
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/RustBodies.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/RustBodies.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let roots = Roots::of(&planted);
        let disagreements = composition(&roots, moves, detection, 1);
        match disagreements.first() {
            Some(first) => eprintln!(
                "BS-11: plant detected: {}: {}",
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
        checked_units.units.contains_key("RustBodies"),
        "RustBodies is a linked module of the project"
    );
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report = super::rust_lower::eligibility_report(&project, &build_id, MODULE);
    let rows = report["roots"].as_array().expect("roots");
    assert_eq!(rows.len(), ROOTS.len(), "the roots of RustBodies");
    for (name, row) in ROOTS.iter().zip(rows) {
        assert_eq!(row["root"], format!("{MODULE}.{name}"));
        // The fold does no arithmetic, so it realizes no overflow.
        let effects: &[&str] = if *name == "foldBodies" {
            &["allocation", "recursion"]
        } else {
            &["allocation", "overflow", "recursion"]
        };
        super::rust_lower::assert_eligible(row, name, effects);
    }
    let sources = committed();
    let all: Vec<Vector> = programs(&sources).iter().filter_map(vector).collect();
    let within: Vec<&Vector> = all
        .iter()
        .filter(|vector| vector.nodes <= EVALUATOR_NODES)
        .collect();
    let mutated: Vec<Vector> = drawn(&sources, 0x00B5_0111)
        .iter()
        .filter_map(vector)
        .collect();
    eprintln!(
        "BS-11: {} vectors ({} within {EVALUATOR_NODES} nodes), {} of {MUTANT_DRAWS} mutants ({:?})",
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
    let uses = super::lower(&checked, &project, TYPING, "useTrees");
    let moves = super::lower(&checked, &project, MOVES, "applyMoves");
    let roots = Roots::of(&project);
    let every: Vec<&Vector> = all.iter().collect();
    let use_runs: Vec<Run> = every.iter().map(|vector| vector.uses.clone()).collect();
    // On every host: the rendered package of the typing root agrees with the
    // oracle, whose tree the other roots are given.
    super::rust_validate::rustc_differential("BS-11", &[("useTrees", &uses, &use_runs)], "vectors");
    if !support::normative_host() {
        eprintln!("BS-11: platform-bound host: the evaluator, the composition, and the planted defects run on the normative host");
        return;
    }
    let disagreements = super::evaluate(&uses, &use_runs, 5);
    assert!(
        disagreements.is_empty(),
        "useTrees: the evaluator and the oracle disagree: {disagreements:?}"
    );
    // The rendered packages of the three roots agree with the evaluator on
    // the vectors: the realizations of one program.
    let runs: Vec<(Json, Json, Json, Json)> =
        within.iter().map(|vector| roots.run(vector)).collect();
    let as_run = |arguments: Vec<Value>, expected: &Json| -> Run { (arguments, expected.clone()) };
    let frame_runs: Vec<Run> = within
        .iter()
        .zip(&runs)
        .map(|(vector, (frame, _, _, _))| as_run(vector.frame.clone(), frame))
        .collect();
    let body_runs: Vec<Run> = within
        .iter()
        .zip(&runs)
        .map(|(vector, (frame, lowered, _, _))| {
            let mut arguments = vector.bodies.clone();
            arguments.push(serde_json::from_value(frame.clone()).expect("a value"));
            arguments.push(super::calculus_check::nat(0));
            as_run(arguments, lowered)
        })
        .collect();
    let fold_runs: Vec<Run> = runs
        .iter()
        .map(|(_, _, bodies, folded)| {
            as_run(
                vec![serde_json::from_value(bodies.clone()).expect("a value")],
                folded,
            )
        })
        .collect();
    super::rust_validate::rustc_differential(
        "BS-11",
        &[
            ("lowerFrame", &roots.frame, &frame_runs),
            ("lowerBodies", &roots.bodies, &body_runs),
            ("foldBodies", &roots.fold, &fold_runs),
        ],
        "evaluator",
    );
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
        "BS-11: the composition agrees on {} vectors and {} mutants ({:?})",
        within.len(),
        mutated.len(),
        started.elapsed()
    );
    let detection: Vec<&Vector> = within.iter().copied().chain(mutated_refs).collect();
    plants(&moves, &detection);
    if !support::lean_backed("BS-11") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let low = match exported_root.root.strip_prefix(&format!("{MODULE}.")) {
            Some("lowerFrame") => &roots.frame,
            Some("lowerBodies") => &roots.bodies,
            Some("foldBodies") => &roots.fold,
            _ if exported_root.root == format!("{TYPING}.useTrees") => &uses,
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
            "BS-11: {}: largest certificate part {largest} bytes",
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
    assert_eq!(realized, ROOTS.len() + 1, "one certified package per root");
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
    eprintln!("BS-11: passed in {:?}", started.elapsed());
}
