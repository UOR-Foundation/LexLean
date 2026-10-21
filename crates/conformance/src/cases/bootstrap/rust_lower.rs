//! `BS-07` (SPEC.md §17.16, §17.23): the lowering of target programs to the
//! closed Rust AST as language-1.2 source (`LexLeanBoot.RustLower`), held to
//! the Rust lowering. This slice is its failure typing,
//! `calculus::rust::fallible_functions`.
//!
//! The root takes a program as calculus data, the six chains of `BS-04`, and
//! a program that `checkProgram` accepts: `Lowering::new` begins with
//! `check::check`, and the host composes the two roots, so a program the
//! oracle refuses is not a vector of this root.

use lexlean::calculus::rust::{self};
use lexlean::calculus::{Arm, Expr, Function, Prim, Program, Shape, Ty, Value};
use serde_json::Value as Json;

use super::calculus_check::{
    committed, literal_corpus, mutants, prim_corpus, program_arguments, program_of, program_size,
    shape_corpus, targeted, Source,
};
use super::{bool_val as boolean, list, Run, Seeded};
use crate::support;

const MODULE: &str = "LexLeanBoot.RustLower";

/// The roots in the order the module declares them, each with the effects its
/// declaration admits, which are exactly the effects its closure realizes.
const ROOTS: [(&str, &[&str]); 1] = [(
    "fallibleFunctions",
    &["allocation", "overflow", "recursion"],
)];

/// What the oracle makes of a program, as the root returns it: the Boolean
/// of each function. A program the checker refuses is no vector.
fn run(program: &Program) -> Option<Run> {
    let flags = rust::fallible_functions(program).ok()?;
    Some((
        program_arguments(program),
        serde_json::to_value(list(flags.into_iter().map(boolean).collect()))
            .expect("a value is JSON"),
    ))
}

fn var(name: u64) -> Expr {
    Expr::Var { name }
}

fn nat(value: u64) -> Expr {
    Expr::Value {
        ty: Ty::Nat,
        value: Value::Nat {
            value: value.to_string(),
        },
    }
}

fn function(types: Vec<Ty>, result: Ty, body: Expr) -> Function {
    Function {
        parameters: (0..types.len() as u64).collect(),
        types,
        result,
        body,
    }
}

fn call(function: u64, operands: Vec<Expr>) -> Expr {
    Expr::Call { function, operands }
}

fn closure(function: u64, captures: Vec<Expr>) -> Expr {
    Expr::Closure { function, captures }
}

fn add(left: Expr, right: Expr) -> Expr {
    Expr::Prim {
        operation: Prim::NatAdd,
        operands: vec![left, right],
    }
}

fn function_type(parameters: Vec<Ty>, result: Ty) -> Ty {
    Ty::Fn {
        parameters,
        result: Box::new(result),
    }
}

fn adt(index: u64) -> Ty {
    Ty::Adt { index }
}

/// The programs of each rule of the analysis: a failing primitive and the
/// calls that reach it, a successor, the application of a function type one
/// of whose closures fails, a function type that nothing inhabits, a function
/// with an argument no value inhabits whose closures do not count, and types
/// that hold themselves or a function type.
#[allow(clippy::too_many_lines)]
fn analysis() -> Vec<Program> {
    let fun = function_type(vec![Ty::Nat], Ty::Nat);
    let mut out = Vec::new();
    // A chain of calls to a failing primitive, and a function that calls none.
    out.push(program_of(
        Vec::new(),
        vec![
            function(Vec::new(), Ty::Nat, call(1, Vec::new())),
            function(Vec::new(), Ty::Nat, call(2, Vec::new())),
            function(Vec::new(), Ty::Nat, add(nat(1), nat(2))),
            function(Vec::new(), Ty::Nat, nat(0)),
            function(Vec::new(), Ty::Nat, call(3, Vec::new())),
        ],
    ));
    // Every failing primitive of the table, one function each, and the
    // primitives that do not fail.
    for operation in [
        Prim::NatAdd,
        Prim::NatSub,
        Prim::NatMul,
        Prim::NatQuot,
        Prim::NatRem,
    ] {
        out.push(program_of(
            Vec::new(),
            vec![function(
                Vec::new(),
                Ty::Nat,
                Expr::Prim {
                    operation,
                    operands: vec![nat(1), nat(2)],
                },
            )],
        ));
    }
    // A successor fails.
    out.push(program_of(
        Vec::new(),
        vec![
            function(
                vec![Ty::Nat],
                Ty::Nat,
                Expr::Build {
                    shape: Shape::Succ,
                    ty: Ty::Nat,
                    operands: vec![var(0)],
                },
            ),
            function(Vec::new(), Ty::Nat, call(0, vec![nat(1)])),
            function(Vec::new(), Ty::Nat, nat(0)),
        ],
    ));
    // An application fails when some closure of its function type does, and
    // only then; two function types, one with a failing closure.
    let other = function_type(vec![Ty::Nat], Ty::Bool);
    out.push(program_of(
        Vec::new(),
        vec![
            function(vec![Ty::Nat], Ty::Nat, add(var(0), var(0))),
            function(vec![Ty::Nat], Ty::Nat, var(0)),
            function(
                vec![fun.clone()],
                Ty::Nat,
                Expr::Apply {
                    target: Box::new(var(0)),
                    operands: vec![nat(1)],
                },
            ),
            function(Vec::new(), Ty::Nat, call(2, vec![closure(1, Vec::new())])),
            function(Vec::new(), fun.clone(), closure(0, Vec::new())),
            function(
                vec![other.clone()],
                Ty::Bool,
                Expr::Apply {
                    target: Box::new(var(0)),
                    operands: vec![nat(1)],
                },
            ),
            function(
                vec![Ty::Nat],
                Ty::Bool,
                Expr::Value {
                    ty: Ty::Bool,
                    value: Value::Bool { value: true },
                },
            ),
            function(Vec::new(), Ty::Bool, call(5, vec![closure(6, Vec::new())])),
        ],
    ));
    // A closure that captures, applied through a pair.
    out.push(program_of(
        Vec::new(),
        vec![
            function(vec![Ty::Nat, Ty::Nat], Ty::Nat, add(var(0), var(1))),
            function(
                vec![Ty::Pair {
                    left: Box::new(fun.clone()),
                    right: Box::new(Ty::Nat),
                }],
                Ty::Nat,
                Expr::Apply {
                    target: Box::new(Expr::First {
                        value: Box::new(var(0)),
                    }),
                    operands: vec![nat(2)],
                },
            ),
            function(
                Vec::new(),
                Ty::Nat,
                call(
                    1,
                    vec![Expr::Build {
                        shape: Shape::Pair,
                        ty: Ty::Pair {
                            left: Box::new(fun.clone()),
                            right: Box::new(Ty::Nat),
                        },
                        operands: vec![closure(0, vec![nat(1)]), nat(0)],
                    }],
                ),
            ),
        ],
    ));
    // A type with no constructor, a function of an argument of it whose
    // closure would make an application fail, and a closure that counts.
    let empty = vec![Vec::<Vec<Ty>>::new()];
    out.push(program_of(
        empty.clone(),
        vec![
            function(vec![adt(0)], fun.clone(), closure(1, Vec::new())),
            function(vec![Ty::Nat], Ty::Nat, add(var(0), var(0))),
            function(
                vec![fun.clone()],
                Ty::Nat,
                Expr::Apply {
                    target: Box::new(var(0)),
                    operands: vec![nat(1)],
                },
            ),
            function(Vec::new(), Ty::Nat, nat(0)),
        ],
    ));
    out.push(program_of(
        empty,
        vec![
            function(vec![adt(0)], fun.clone(), closure(1, Vec::new())),
            function(vec![Ty::Nat], Ty::Nat, add(var(0), var(0))),
            function(
                vec![fun.clone()],
                Ty::Nat,
                Expr::Apply {
                    target: Box::new(var(0)),
                    operands: vec![nat(1)],
                },
            ),
            function(Vec::new(), fun.clone(), closure(1, Vec::new())),
        ],
    ));
    // Types that hold themselves, through a pair, a result, and a function
    // type; and an argument of a type that is inhabited only by boxing.
    let list_like = vec![vec![Vec::new(), vec![adt(0)]]];
    let only_self = vec![vec![vec![adt(0)]]];
    let paired = vec![
        vec![vec![Ty::Pair {
            left: Box::new(adt(0)),
            right: Box::new(adt(1)),
        }]],
        vec![Vec::new()],
    ];
    let hollow = vec![
        vec![vec![Ty::Pair {
            left: Box::new(Ty::Nat),
            right: Box::new(adt(1)),
        }]],
        Vec::<Vec<Ty>>::new(),
    ];
    let through_result = vec![
        vec![vec![Ty::Result {
            ok: Box::new(adt(1)),
            error: Box::new(Ty::Nat),
        }]],
        Vec::<Vec<Ty>>::new(),
    ];
    for declared in [list_like, only_self, paired, hollow, through_result] {
        for argument in 0..declared.len() as u64 {
            out.push(program_of(
                declared
                    .iter()
                    .map(|constructors| constructors.clone())
                    .collect::<Vec<_>>(),
                vec![
                    function(vec![adt(argument)], fun.clone(), closure(1, Vec::new())),
                    function(vec![Ty::Nat], Ty::Nat, add(var(0), var(0))),
                    function(
                        vec![fun.clone()],
                        Ty::Nat,
                        Expr::Apply {
                            target: Box::new(var(0)),
                            operands: vec![nat(1)],
                        },
                    ),
                ],
            ));
        }
    }
    // A function type stored in a type, and a type that stores itself behind
    // it; the function type's closure captures a value of the type.
    let stores = vec![vec![vec![fun.clone()]]];
    out.push(program_of(
        stores,
        vec![
            function(vec![Ty::Nat], Ty::Nat, add(var(0), var(0))),
            function(
                vec![adt(0)],
                Ty::Nat,
                Expr::Apply {
                    target: Box::new(Expr::Field {
                        value: Box::new(var(0)),
                        index: 0,
                    }),
                    operands: vec![nat(3)],
                },
            ),
            function(
                Vec::new(),
                Ty::Nat,
                call(
                    1,
                    vec![Expr::Build {
                        shape: Shape::Adt { constructor: 0 },
                        ty: adt(0),
                        operands: vec![closure(0, Vec::new())],
                    }],
                ),
            ),
        ],
    ));
    let selfish = vec![vec![vec![function_type(vec![adt(0)], Ty::Nat)]]];
    out.push(program_of(
        selfish,
        vec![
            function(vec![adt(0), Ty::Nat], Ty::Nat, add(var(1), var(1))),
            function(
                vec![adt(0)],
                Ty::Nat,
                Expr::Apply {
                    target: Box::new(Expr::Field {
                        value: Box::new(var(0)),
                        index: 0,
                    }),
                    operands: vec![var(0)],
                },
            ),
        ],
    ));
    // A match whose arm binders carry a function type, and a let that binds one.
    out.push(program_of(
        Vec::new(),
        vec![
            function(vec![Ty::Nat], Ty::Nat, add(var(0), var(0))),
            function(
                vec![Ty::Option {
                    value: Box::new(fun.clone()),
                }],
                Ty::Nat,
                Expr::Match {
                    ty: Ty::Nat,
                    scrutinee: Box::new(var(0)),
                    arms: vec![
                        Arm {
                            shape: Shape::None,
                            binders: Vec::new(),
                            body: nat(0),
                        },
                        Arm {
                            shape: Shape::Some,
                            binders: vec![5],
                            body: Expr::Let {
                                name: 6,
                                ty: fun.clone(),
                                bound: Box::new(var(5)),
                                body: Box::new(Expr::Apply {
                                    target: Box::new(var(6)),
                                    operands: vec![nat(1)],
                                }),
                            },
                        },
                    ],
                },
            ),
            function(
                Vec::new(),
                Ty::Nat,
                call(
                    1,
                    vec![Expr::Build {
                        shape: Shape::Some,
                        ty: Ty::Option {
                            value: Box::new(fun.clone()),
                        },
                        operands: vec![closure(0, Vec::new())],
                    }],
                ),
            ),
        ],
    ));
    out
}

/// A program with its run: the oracle's Booleans, the arguments of the root.
struct Vector {
    nodes: usize,
    run: Run,
}

fn vector(program: &Program) -> Option<Vector> {
    run(program).map(|run| Vector {
        nodes: program_size(program),
        run,
    })
}

/// The vectors every host runs: the programs of each rule, the programs
/// `BS-04` makes (the refusals of `check` are no vector of this root), and the
/// committed programs.
fn vectors(sources: &[Source]) -> Vec<Vector> {
    let mut seeded = Seeded(0x00B5_0007);
    let mut programs: Vec<Program> = analysis();
    programs.extend(targeted());
    programs.extend(literal_corpus(&mut seeded, LITERALS));
    programs.extend(prim_corpus(&mut seeded, PRIM_ROUNDS));
    programs.extend(shape_corpus(&mut seeded));
    programs.extend(sources.iter().map(|source| source.program.clone()));
    programs.iter().filter_map(vector).collect()
}

/// `count` seeded mutants of the smaller committed programs that the oracle's
/// checker still accepts.
fn seeded_mutants(
    sources: &[Source],
    seeded: &mut Seeded,
    count: usize,
    limit: usize,
) -> Vec<Vector> {
    let mut out = Vec::new();
    while out.len() < count {
        let batch = mutants(sources, seeded, count - out.len(), limit);
        out.extend(batch.iter().filter_map(vector));
    }
    out
}

fn runs(vectors: &[&Vector]) -> Vec<Run> {
    vectors.iter().map(|vector| vector.run.clone()).collect()
}

// The counts SPEC.md §17.23, the register statement of BS-07, its scenario,
// and VERIFICATION.md state: the case fails when the tree no longer gives them.
const LITERALS: usize = 200;
const PRIM_ROUNDS: usize = 3;
/// The programs the evaluator is given in full: those of at most this many nodes.
const EVALUATOR_NODES: usize = 300;
/// Programs a mutation draws on, at most this many nodes.
const BASE_NODES: usize = 150;
const EVALUATOR_MUTANTS: usize = 150;
const RUSTC_MUTANTS: usize = 1_500;

/// A defect planted in the source and the corpus it is caught on.
const PLANTS: [super::Plant; 10] = [
    super::Plant {
        label: "an addition of naturals does not fail",
        site: "| CalculusCheck::Prim.NatAdd => true",
        replacement: "| CalculusCheck::Prim.NatAdd => false",
        root: "fallibleFunctions",
    },
    super::Plant {
        label: "a successor does not fail",
        site: "{field: own, value: true},",
        replacement: "{field: own, value: false},",
        root: "fallibleFunctions",
    },
    super::Plant {
        label: "a parse of an integer does not fail",
        site: "((CalculusCheck::tyTag target) =? 3)",
        replacement: "false",
        root: "fallibleFunctions",
    },
    super::Plant {
        label: "a call is not an edge",
        site: "(if (natMember (project inner calls) function) then (project inner calls) else (cons function (project inner calls)))",
        replacement: "(project inner calls)",
        root: "fallibleFunctions",
    },
    super::Plant {
        label: "an application is not an edge",
        site: "(tyAddNew (project two applies) (inferTy program target scope))",
        replacement: "(project two applies)",
        root: "fallibleFunctions",
    },
    super::Plant {
        label: "the closures of a dead function count",
        site: "} bodies 0 dead) closures)",
        replacement: "} bodies 0 (nil Bool)) closures)",
        root: "fallibleFunctions",
    },
    super::Plant {
        label: "the failures are followed one round",
        site: "(1 + count) (falses count) : ((List Bool) * Bool)",
        replacement: "1 (falses count) : ((List Bool) * Bool)",
        root: "fallibleFunctions",
    },
    super::Plant {
        label: "a closure of any function type makes an application fail",
        site: "(if (CalculusCheck::tyEq known fnTy) then (if (flagAt flags member)",
        replacement: "(if true then (if (flagAt flags member)",
        root: "fallibleFunctions",
    },
    super::Plant {
        label: "no type holds itself",
        site: "(anyHeld owner edges (inPlace known (ctor CalculusCheck::Ty.TNil) ty))",
        replacement: "false",
        root: "fallibleFunctions",
    },
    super::Plant {
        label: "every function type is inhabited",
        site: "| CalculusCheck::Ty.TFn parameters result => (tyMember inhabited ty)",
        replacement: "| CalculusCheck::Ty.TFn parameters result => true",
        root: "fallibleFunctions",
    },
];

fn eligibility(project: &support::P) {
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanBoot/RustLower.eligibility.json")
                .as_std_path(),
        )
        .expect("the eligibility report"),
    )
    .expect("the report is JSON");
    let reported = report["roots"].as_array().expect("roots");
    assert_eq!(
        reported
            .iter()
            .map(|row| row["root"].as_str().expect("root").to_owned())
            .collect::<Vec<_>>(),
        ROOTS
            .iter()
            .map(|(root, _)| format!("{MODULE}.{root}"))
            .collect::<Vec<_>>(),
        "the roots are the module's production roots, in order"
    );
    for (row, (root, effects)) in reported.iter().zip(ROOTS) {
        let declared: Vec<&str> = row["declared_effects"]
            .as_array()
            .expect("declared effects")
            .iter()
            .map(|effect| effect.as_str().expect("an effect"))
            .collect();
        assert_eq!(declared, effects, "{root}: the declared effects");
        let targets = row["targets"].as_array().expect("targets");
        assert_eq!(targets.len(), 1, "{root}: one target");
        let target = &targets[0];
        assert_eq!(target["target"], "rust-std", "{root}: the target");
        assert_eq!(target["status"], "eligible", "{root}: eligible");
        let realized: Vec<&str> = target["effects"]
            .as_array()
            .expect("effects")
            .iter()
            .map(|effect| effect["effect"].as_str().expect("an effect"))
            .collect();
        assert_eq!(
            realized, effects,
            "{root}: the declaration admits exactly the realized effects"
        );
    }
}

pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("RustLower")
            && checked_units.units.contains_key("CalculusCheck"),
        "RustLower and CalculusCheck are linked modules of the project"
    );
    eligibility(&project);
    let sources = committed();
    let all = vectors(&sources);
    let evaluator: Vec<&Vector> = all
        .iter()
        .filter(|vector| vector.nodes <= EVALUATOR_NODES)
        .collect();
    let mut seeded = Seeded(0x00B5_0107);
    let evaluator_mutants = seeded_mutants(&sources, &mut seeded, EVALUATOR_MUTANTS, BASE_NODES);
    let mut large = Seeded(0x00B5_0207);
    let rustc_mutants = seeded_mutants(&sources, &mut large, RUSTC_MUTANTS, BASE_NODES);
    eprintln!(
        "BS-07: {} vectors ({} within {EVALUATOR_NODES} nodes), {} + {} mutants ({:?})",
        all.len(),
        evaluator.len(),
        evaluator_mutants.len(),
        rustc_mutants.len(),
        started.elapsed()
    );
    let checked = support::checked_project(&project);
    let root = super::lower(&checked, &project, MODULE, "fallibleFunctions");
    let every: Vec<&Vector> = all.iter().collect();
    // On every host: the rendered rust-std package, compiled by that host's
    // pinned rustc, agrees with the oracle on the vectors.
    super::rust_validate::rustc_differential(
        "BS-07",
        &[("fallibleFunctions", &root, &runs(&every))],
        "vectors",
    );
    if !support::normative_host() {
        eprintln!("BS-07: platform-bound host: the evaluator, the seeded corpora, and the planted defects run on the normative host");
        return;
    }
    let evaluated = runs(&evaluator);
    let disagreements = super::evaluate(&root, &evaluated, 5);
    assert!(
        disagreements.is_empty(),
        "the evaluator and the Rust analysis disagree on the vectors: {disagreements:?}"
    );
    eprintln!(
        "BS-07: the evaluator agrees on {} vectors ({:?})",
        evaluated.len(),
        started.elapsed()
    );
    let mutated: Vec<&Vector> = evaluator_mutants.iter().collect();
    let mutated_runs = runs(&mutated);
    let disagreements = super::evaluate(&root, &mutated_runs, 5);
    assert!(
        disagreements.is_empty(),
        "the evaluator and the Rust analysis disagree on the seeded mutants: {disagreements:?}"
    );
    eprintln!(
        "BS-07: the evaluator agrees on {} seeded mutants ({:?})",
        mutated_runs.len(),
        started.elapsed()
    );
    let large: Vec<&Vector> = rustc_mutants.iter().collect();
    super::rust_validate::rustc_differential(
        "BS-07",
        &[("fallibleFunctions", &root, &runs(&large))],
        "seeded",
    );
    // The gate is armed: each defect planted in a copy of the source is a
    // disagreement with the oracle.
    let detection: Vec<Run> = evaluated.iter().chain(&mutated_runs).cloned().collect();
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/RustLower.lex.tex");
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            "src/RustLower.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, plant.root);
        let disagreements = super::evaluate(&low, &detection, 1);
        assert!(
            !disagreements.is_empty(),
            "{}: the planted defect is detected by the differential",
            plant.label
        );
        eprintln!(
            "BS-07: plant detected: {}: {}",
            plant.label, disagreements[0]
        );
    }
    // Under the pinned Lean, the production export certifies the root; its
    // package is the committed one and realizes the program the differential ran.
    if !support::lean_backed("BS-07") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
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
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some(member) = exported_root.root.strip_prefix(&format!("{MODULE}.")) else {
            continue;
        };
        assert_eq!(member, "fallibleFunctions");
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
                serde_json::to_value(&root.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    eprintln!("BS-07: passed in {:?}", started.elapsed());
}

use crate::production_export as harness;
