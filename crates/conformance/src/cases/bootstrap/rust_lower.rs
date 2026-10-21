//! `BS-07` (SPEC.md §17.16, §17.23): the lowering of target programs to the
//! closed Rust AST as language-1.2 source (`LexLeanBoot.RustLower`), held to
//! the Rust lowering. This slice is its failure typing,
//! `calculus::rust::fallible_functions`.
//!
//! The root takes a program as calculus data, the six chains of `BS-04`, and
//! a program that `checkProgram` accepts: `Lowering::new` begins with
//! `check::check`, and the host composes the two roots, so a program the
//! oracle refuses is not a vector of this root.

use std::collections::BTreeMap;

use lexlean::calculus::rust::{self};
use lexlean::calculus::{
    Adt, Arm, Expr, Function, Prim, Program, Shape, Ty, Value, PROGRAM_SPEC,
};
use serde_json::Value as Json;

use super::calculus_check::{
    committed, constant, literal_corpus, mutants, prim_corpus, program_arguments, program_of,
    program_size, sample_tys, sample_values, shape_corpus, targeted, unit_expr, Source,
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
            function(
                Vec::new(),
                Ty::Nat,
                call(2, vec![closure(1, Vec::new())]),
            ),
            function(
                Vec::new(),
                fun.clone(),
                closure(0, Vec::new()),
            ),
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
            function(
                Vec::new(),
                Ty::Bool,
                call(5, vec![closure(6, Vec::new())]),
            ),
        ],
    ));
    // A closure that captures, applied through a pair.
    out.push(program_of(
        Vec::new(),
        vec![
            function(
                vec![Ty::Nat, Ty::Nat],
                Ty::Nat,
                add(var(0), var(1)),
            ),
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
    let _ = (constant, unit_expr, Adt { constructors: Vec::new() });
    let _ = PROGRAM_SPEC;
    out
}

/// The vectors every host runs: the committed programs, the programs of the
/// rules, the programs the other corpora of `BS-04` make, and the refusals
/// that the oracle's checker does not make.
fn vectors(sources: &[Source]) -> Vec<Run> {
    let mut seeded = Seeded(0x00B5_0007);
    let mut programs: Vec<Program> = analysis();
    programs.extend(targeted());
    programs.extend(literal_corpus(&mut seeded, 200));
    programs.extend(prim_corpus(&mut seeded, 3));
    programs.extend(shape_corpus(&mut seeded));
    programs.extend(sources.iter().map(|source| source.program.clone()));
    programs.iter().filter_map(run).collect()
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
    let sources = committed();
    let runs = vectors(&sources);
    eprintln!(
        "BS-07: {} vectors, {} functions ({:?})",
        runs.len(),
        runs.iter()
            .map(|(_, expected)| expected["items"].as_array().map_or(0, Vec::len))
            .sum::<usize>(),
        started.elapsed()
    );
    let checked = support::checked_project(&project);
    let root = super::lower(&checked, &project, MODULE, "fallibleFunctions");
    let disagreements = super::evaluate(&root, &runs, 5);
    assert!(disagreements.is_empty(), "{disagreements:?}");
    let _ = (
        BTreeMap::<u8, u8>::new(),
        Json::Null,
        program_size,
        mutants,
        sample_tys,
        sample_values,
        ROOTS,
    );
}
