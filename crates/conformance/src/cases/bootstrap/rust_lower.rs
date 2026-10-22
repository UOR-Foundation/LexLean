//! `BS-07` (SPEC.md §17.16, §17.23): the lowering of target programs to the
//! closed Rust AST as language-1.2 source (`LexLeanBoot.RustLower`), held to
//! the Rust lowering. The first slice is its failure typing,
//! `calculus::rust::fallible_functions`, as three roots composed by the host:
//! `siteTrees` of `CalculusCheck` (the type of each scrutinee and each applied
//! target, from the module's own `walk`), `deadFunctions`, and
//! `fallibleFunctions`; the second is `lowerTypes`, the enums and dispatch of
//! the types of the crate, given what the first returns.
//!
//! The roots take a program as calculus data, the six chains of `BS-04`, and
//! a program that `checkProgram` accepts: `Lowering::new` begins with
//! `check::check`, and the host composes the roots, so a program the oracle
//! refuses is not a vector of these roots. The consumers take the tree the
//! typing root returns, never a type the host computed: each leg is held to
//! the oracle on the same vectors, so the host's composition of the roots
//! gives the oracle's result on every vector, leg by leg.

use lexlean::calculus::rust::ast::{Expr as RustExpr, Ident, ItemDef, Pat};
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::sites::{self, Site};
use lexlean::calculus::{Arm, Expr, Function, Prim, Program, Shape, Ty, Value};
use serde_json::Value as Json;

use super::calculus_check::{
    adt as constructor, committed, enc_ty, literal_corpus, mutants, prim_corpus, program_arguments,
    program_of, program_size, shape_corpus, targeted, ty_chain, Source,
};
use super::rust_moves;
use super::rust_print::enc_items;
use super::{bool_val as boolean, list, Run, Seeded};
use crate::support;

const MODULE: &str = "LexLeanBoot.RustLower";

/// The roots in the order the module declares them, each with the effects its
/// declaration admits, which are exactly the effects its closure realizes.
const ROOTS: [(&str, &[&str]); 3] = [
    ("deadFunctions", &["allocation", "overflow", "recursion"]),
    (
        "fallibleFunctions",
        &["allocation", "overflow", "recursion"],
    ),
    ("lowerTypes", &["allocation", "overflow", "recursion"]),
];

/// Three quarters of `max_file_bytes` (4,194,304), the most a part of a
/// certificate of these roots may be.
const CERTIFICATE_BUDGET: u64 = 3_145_728;

/// The module of the typing root, which is `CalculusCheck`'s own.
const TYPING: &str = "LexLeanBoot.CalculusCheck";

/// The module of the pass that moves last reads.
const MOVES: &str = "LexLeanBoot.RustMoves";

/// The mutants of the crates the moves are run on: under the evaluator and
/// under rustc.
/// The crates of at most this many expressions are the evaluator's share.
const MOVES_EVALUATOR_NODES: usize = 400;
/// The crates the moves are run on, and those within the evaluator's bound.
const MOVES_CRATES: usize = 352;
const MOVES_WITHIN: usize = 317;
const MOVES_EVALUATOR_MUTANTS: usize = 150;
const MOVES_RUSTC_MUTANTS: usize = 1_500;

/// A tree of typing sites as the `Ty` the typing root returns: a node is a
/// pair of its own type (nil when it reads none) and the chain of its
/// children, a leaf is nil.
fn enc_site(site: &Site) -> Value {
    match site {
        Site::Leaf => constructor(14, Vec::new()),
        Site::Node { own, kids } => constructor(
            11,
            vec![
                own.as_ref()
                    .map_or_else(|| constructor(14, Vec::new()), enc_ty),
                ty_chain(kids.iter().map(enc_site).collect()),
            ],
        ),
    }
}

/// What the oracle makes of a program, leg by leg, as each root returns it:
/// the tree of typing sites; the dead functions, given the tree; the Boolean
/// of each function, given the tree and the dead functions; the items of the
/// crate that declare its types (the enum of each ADT and of each function
/// type, and the dispatch of each function type), given all three. A program
/// the checker refuses is no vector.
struct Legs {
    sites: Run,
    dead: Run,
    fallible: Run,
    types: Run,
}

fn flags(flags: Vec<bool>) -> Value {
    list(flags.into_iter().map(boolean).collect())
}

/// The dead functions as the lowering shows them: a function with an argument
/// no value inhabits renders as the empty match on that argument, and no live
/// body is one. The oracle needs no entry of its own for them.
fn lowered(program: &Program) -> Option<(Vec<bool>, Value)> {
    let krate = rust::lower(program, Profile::Std).ok()?;
    let mut dead = vec![false; program.functions.len()];
    for item in &krate.items {
        if let ItemDef::Function {
            name: Ident::Function(at),
            parameters,
            body,
            ..
        } = item
        {
            // The renderer's rule: the match on a parameter that is bound
            // and moved, with no arm.
            let empty = match &body.tail {
                RustExpr::Match {
                    scrutinee, arms, ..
                } if arms.is_empty() => matches!(
                    scrutinee.as_ref(),
                    RustExpr::Move(Ident::Local(moved), _)
                        if parameters.iter().any(|(pattern, _)| {
                            matches!(pattern, Pat::Bind(Ident::Local(bound)) if bound == moved)
                        })
                ),
                _ => false,
            };
            *dead.get_mut(usize::try_from(*at).ok()?)? = body.lets.is_empty() && empty;
        }
    }
    let declared = krate
        .items
        .iter()
        .position(|item| matches!(item, ItemDef::Function { .. }))
        .unwrap_or(krate.items.len());
    Some((dead, enc_items(&krate.items[..declared])))
}

fn run(program: &Program) -> Option<Legs> {
    let base = program_arguments(program);
    let tree = enc_site(&sites::trees(program).ok()?);
    let (dead, types) = lowered(program)?;
    let fallible = rust::fallible_functions(program).ok()?;
    let with_tree: Vec<Value> = base.iter().cloned().chain([tree.clone()]).collect();
    let with_dead: Vec<Value> = with_tree
        .iter()
        .cloned()
        .chain([flags(dead.clone())])
        .collect();
    let with_fallible: Vec<Value> = with_dead
        .iter()
        .cloned()
        .chain([flags(fallible.clone())])
        .collect();
    Some(Legs {
        sites: (base, serde_json::to_value(&tree).expect("a value is JSON")),
        dead: (
            with_tree,
            serde_json::to_value(flags(dead)).expect("a value is JSON"),
        ),
        fallible: (
            with_dead,
            serde_json::to_value(flags(fallible)).expect("a value is JSON"),
        ),
        types: (
            with_fallible,
            serde_json::to_value(types).expect("a value is JSON"),
        ),
    })
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
                    operands: match operation {
                        Prim::NatQuot | Prim::NatRem => vec![nat(1), nat(2), nat(3)],
                        _ => vec![nat(1), nat(2)],
                    },
                    operation,
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
    // A function type no closure inhabits (the checker refuses a type with no
    // constructor), a function of an argument of it whose closure would make
    // an application fail, and a closure that counts: the dead function's
    // closure is the only one of its type, so the application's function
    // becomes dead in a second round, and nothing fails.
    let unfilled = function_type(vec![Ty::Bool], Ty::Nat);
    let empty = Vec::<Vec<Vec<Ty>>>::new();
    out.push(program_of(
        empty.clone(),
        vec![
            function(vec![unfilled.clone()], fun.clone(), closure(1, Vec::new())),
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
            function(vec![unfilled.clone()], fun.clone(), closure(1, Vec::new())),
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
        vec![vec![adt(1)]],
    ];
    let through_result = vec![
        vec![vec![Ty::Result {
            ok: Box::new(adt(1)),
            error: Box::new(Ty::Nat),
        }]],
        vec![vec![adt(1)]],
    ];
    for declared in [list_like, only_self, paired, hollow, through_result] {
        for argument in 0..declared.len() as u64 {
            out.push(program_of(
                declared.clone(),
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
    legs: Legs,
}

fn vector(program: &Program) -> Option<Vector> {
    run(program).map(|legs| Vector {
        nodes: program_size(program),
        legs,
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

/// One leg of each vector: the runs of a root.
fn runs(vectors: &[&Vector], leg: Leg) -> Vec<Run> {
    vectors
        .iter()
        .map(|vector| leg(&vector.legs).clone())
        .collect()
}

fn sites_leg(legs: &Legs) -> &Run {
    &legs.sites
}

fn dead_leg(legs: &Legs) -> &Run {
    &legs.dead
}

fn fallible_leg(legs: &Legs) -> &Run {
    &legs.fallible
}

fn types_leg(legs: &Legs) -> &Run {
    &legs.types
}

// The counts SPEC.md §17.23, the register statement of BS-07, its scenario,
// and VERIFICATION.md state: the case fails when the tree no longer gives them.
/// The programs of the rules of the analysis, and the vectors the corpora make
/// of them and of the committed programs, as the statement of the register row
/// and SPEC.md §17.23 count them.
const RULE_PROGRAMS: usize = 22;
const VECTORS: usize = 311;
const VECTORS_WITHIN: usize = 272;
const LITERALS: usize = 200;
const PRIM_ROUNDS: usize = 3;
/// The programs the evaluator is given in full: those of at most this many nodes.
const EVALUATOR_NODES: usize = 300;
/// Programs a mutation draws on, at most this many nodes.
const BASE_NODES: usize = 150;
const EVALUATOR_MUTANTS: usize = 150;
const RUSTC_MUTANTS: usize = 1_500;

/// The part of a vector a root is run on.
type Leg = fn(&Legs) -> &Run;

/// A defect planted in a module's source, the root whose differential must
/// catch it, and the leg of the vectors it is caught on.
struct Planted {
    label: &'static str,
    file: &'static str,
    site: &'static str,
    replacement: &'static str,
    module: &'static str,
    root: &'static str,
    leg: Leg,
}

const PLANTS: [Planted; 20] = [
    Planted {
        label: "an addition of naturals does not fail",
        file: "src/RustLower.lex.tex",
        site: "| CalculusCheck::Prim.NatAdd => true",
        replacement: "| CalculusCheck::Prim.NatAdd => false",
        module: MODULE,
        root: "fallibleFunctions",
        leg: fallible_leg,
    },
    Planted {
        label: "a successor does not fail",
        file: "src/RustLower.lex.tex",
        site: "{field: own, value: true},",
        replacement: "{field: own, value: false},",
        module: MODULE,
        root: "fallibleFunctions",
        leg: fallible_leg,
    },
    Planted {
        label: "a parse of an integer does not fail",
        file: "src/RustLower.lex.tex",
        site: "((CalculusCheck::tyTag target) =? 3)",
        replacement: "false",
        module: MODULE,
        root: "fallibleFunctions",
        leg: fallible_leg,
    },
    Planted {
        label: "a call is not an edge",
        file: "src/RustLower.lex.tex",
        site: "(if (natMember (project inner calls) function) then (project inner calls) else (cons function (project inner calls)))",
        replacement: "(project inner calls)",
        module: MODULE,
        root: "fallibleFunctions",
        leg: fallible_leg,
    },
    Planted {
        label: "an application is not an edge",
        file: "src/RustLower.lex.tex",
        site: "(tyAddNew (project two applies) (siteOwn sites))",
        replacement: "(project two applies)",
        module: MODULE,
        root: "fallibleFunctions",
        leg: fallible_leg,
    },
    Planted {
        label: "the closures of a dead function count",
        file: "src/RustLower.lex.tex",
        site: "} bodies 0 dead sites) closures)",
        replacement: "} bodies 0 (nil Bool) sites) closures)",
        module: MODULE,
        root: "deadFunctions",
        leg: dead_leg,
    },
    Planted {
        label: "the failures are followed one round",
        file: "src/RustLower.lex.tex",
        site: "(1 + count) (falses count) : ((List Bool) * Bool)",
        replacement: "1 (falses count) : ((List Bool) * Bool)",
        module: MODULE,
        root: "fallibleFunctions",
        leg: fallible_leg,
    },
    Planted {
        label: "a closure of any function type makes an application fail",
        file: "src/RustLower.lex.tex",
        site: "(if (CalculusCheck::tyEq known fnTy) then (if (flagAt flags member)",
        replacement: "(if true then (if (flagAt flags member)",
        module: MODULE,
        root: "fallibleFunctions",
        leg: fallible_leg,
    },
    Planted {
        label: "no type holds itself",
        file: "src/RustLower.lex.tex",
        site: "(anyHeld owner edges (inPlace known (ctor CalculusCheck::Ty.TNil) ty))",
        replacement: "false",
        module: MODULE,
        root: "deadFunctions",
        leg: dead_leg,
    },
    Planted {
        label: "every function type is inhabited",
        file: "src/RustLower.lex.tex",
        site: "| CalculusCheck::Ty.TFn parameters result => (tyMember inhabited (ctor CalculusCheck::Ty.TFn parameters result))",
        replacement: "| CalculusCheck::Ty.TFn parameters result => true",
        module: MODULE,
        root: "deadFunctions",
        leg: dead_leg,
    },
    Planted {
        label: "an applied target reads as no type",
        file: "src/CalculusCheck.lex.tex",
        site: "(ctor Ty.TPair (inferTy program target scope) ",
        replacement: "(ctor Ty.TPair (ctor Ty.TNil) ",
        module: TYPING,
        root: "siteTrees",
        leg: sites_leg,
    },
    Planted {
        label: "the body of an arm is typed without its binders",
        file: "src/CalculusCheck.lex.tex",
        site: "(siteTree program body (bindAll binders (armFields program shape subject) scope) (ctor Ty.TNil) 0)",
        replacement: "(siteTree program body scope (ctor Ty.TNil) 0)",
        module: TYPING,
        root: "siteTrees",
        leg: sites_leg,
    },
    Planted {
        label: "the body of a let is typed without its binding",
        file: "src/CalculusCheck.lex.tex",
        site: "(siteTree program body (extendScope name ty scope) (ctor Ty.TNil) 0)",
        replacement: "(siteTree program body scope (ctor Ty.TNil) 0)",
        module: TYPING,
        root: "siteTrees",
        leg: sites_leg,
    },
    Planted {
        label: "a field that holds its owner is not boxed",
        file: "src/RustLower.lex.tex",
        site: "body: (if (cyclic known edges owner ty) then (ctor RustValidate::Ty.TyUnary (ctor RustValidate::UnaryKind.UkRc) (vty known ty)) else (vty known ty)),",
        replacement: "body: (vty known ty),",
        module: MODULE,
        root: "lowerTypes",
        leg: types_leg,
    },
    Planted {
        label: "a boxed capture is cloned",
        file: "src/RustLower.lex.tex",
        site: "(if (cyclic known edges owner ty) then (ctor RustValidate::Cap.CapUnbox) else",
        replacement: "(if false then (ctor RustValidate::Cap.CapUnbox) else",
        module: MODULE,
        root: "lowerTypes",
        leg: types_leg,
    },
    Planted {
        label: "an application of a function type is never fallible",
        file: "src/RustLower.lex.tex",
        site: "(if (flagAt fallible member) then true else (applyFails target fallible rest))",
        replacement: "(applyFails target fallible rest)",
        module: MODULE,
        root: "lowerTypes",
        leg: types_leg,
    },
    Planted {
        label: "the closures of a function type are listed newest first",
        file: "src/RustLower.lex.tex",
        site: "(revClos (project live closures) (ctor Clos.ClNil))",
        replacement: "(project live closures)",
        module: MODULE,
        root: "lowerTypes",
        leg: types_leg,
    },
    Planted {
        label: "a function type with closures is written as the stated type",
        file: "src/RustLower.lex.tex",
        site: "body: (if closed then",
        replacement: "body: (if false then",
        module: MODULE,
        root: "lowerTypes",
        leg: types_leg,
    },
    Planted {
        label: "function types are numbered newest first",
        file: "src/RustLower.lex.tex",
        site: "(tyAppend chain (ctor CalculusCheck::Ty.TCons item (ctor CalculusCheck::Ty.TNil)))",
        replacement: "(ctor CalculusCheck::Ty.TCons item chain)",
        module: MODULE,
        root: "lowerTypes",
        leg: types_leg,
    },
    Planted {
        label: "a string capture is copied",
        file: "src/RustLower.lex.tex",
        site: "| CalculusCheck::Ty.TFixed w => true\n    | CalculusCheck::Ty.TString => false",
        replacement: "| CalculusCheck::Ty.TFixed w => true\n    | CalculusCheck::Ty.TString => true",
        module: MODULE,
        root: "lowerTypes",
        leg: types_leg,
    },
];

/// The row of a root in the eligibility report of its module: eligible for
/// `rust-std`, with its declared effects exactly the effects it realizes.
fn assert_eligible(row: &Json, root: &str, effects: &[&str]) {
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

/// A defect planted in the source of `RustMoves`, caught on the runs of the
/// moves.
struct MovePlant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
}

const MOVE_PLANTS: [MovePlant; 8] = [
    MovePlant {
        label: "a last read never moves",
        site: "    | Option.some found => (((project found whole) =? 0) && ((prim length (project found parts) : Nat) =? 0))\n    | Option.none => true)",
        replacement: "    | Option.some found => false\n    | Option.none => false)",
    },
    MovePlant {
        label: "a component moves though another is read twice",
        site: "(atMostOne (project found parts))",
        replacement: "true",
    },
    MovePlant {
        label: "the reads of a component on two alternatives are added",
        site: "(natMax (second head) (partCount left (first head)))",
        replacement: "((second head) + (partCount left (first head)))",
    },
    MovePlant {
        label: "the holder of an application is not read before its operands",
        site: "(let held : (List (RustValidate::Id * Later)) := (readWhole live holder); (ctor RustAst::Expr.EApply holder (moveE args held) propagate o))",
        replacement: "(ctor RustAst::Expr.EApply holder (moveE args live) propagate o)",
    },
    MovePlant {
        label: "the left of a pair is read before its right",
        site: "(let after : (List (RustValidate::Id * Later)) := (readsE right live 0); (ctor RustAst::Expr.EPair (moveE left after) (moveE right live) o))",
        replacement: "(let after : (List (RustValidate::Id * Later)) := (readsE left live 0); (ctor RustAst::Expr.EPair (moveE left live) (moveE right after) o))",
    },
    MovePlant {
        label: "a pattern does not unbind its names",
        site: "| RustAst::Pat.PBind ident => (liveRemove live ident)",
        replacement: "| RustAst::Pat.PBind ident => live",
    },
    MovePlant {
        label: "a pattern that binds both components is a projection",
        site: "(((boundCount parts) =? 1) && (1 <? (partsLength parts)))",
        replacement: "(1 <? (partsLength parts))",
    },
    MovePlant {
        label: "the state of a loop is not read",
        site: "(readWhole ran state)",
        replacement: "ran",
    },
];

fn eligibility_report(
    project: &support::P,
    build_id: &lexlean::Sha256Digest,
    module: &str,
) -> Json {
    let file = module.replace('.', "/");
    serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(build_id)
                .join(format!("production/{file}.eligibility.json"))
                .as_std_path(),
        )
        .expect("the eligibility report"),
    )
    .expect("the report is JSON")
}

fn eligibility(project: &support::P) {
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report = eligibility_report(project, &build_id, MODULE);
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
        assert_eligible(row, root, effects);
    }
    // The typing root is a root of `CalculusCheck`, whose report `BS-04`
    // checks for the others: this row is the one this case composes.
    let typing = eligibility_report(project, &build_id, TYPING);
    let row = typing["roots"]
        .as_array()
        .expect("roots")
        .iter()
        .find(|row| row["root"] == format!("{TYPING}.siteTrees"))
        .expect("siteTrees is a production root of CalculusCheck");
    assert_eligible(row, "siteTrees", &["allocation", "overflow", "recursion"]);
    // The pass that moves last reads is a module of its own.
    let moving = eligibility_report(project, &build_id, MOVES);
    let rows = moving["roots"].as_array().expect("roots");
    assert_eq!(rows.len(), 1, "applyMoves is the one root of RustMoves");
    assert_eq!(rows[0]["root"], format!("{MOVES}.applyMoves"));
    assert_eligible(
        &rows[0],
        "applyMoves",
        &["allocation", "overflow", "recursion"],
    );
}

/// Each defect planted in a copy of `RustMoves` is a disagreement with the
/// oracle on the runs of the moves.
fn move_plants(detection: &[Run]) {
    for plant in &MOVE_PLANTS {
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
        let low = super::lower(&checked, &planted, MOVES, "applyMoves");
        let disagreements = super::evaluate(&low, detection, 1);
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
    // The programs of the rules are the claim's evidence: a program the
    // checker refuses is no vector, so none may be refused, and the rule of
    // the dead functions must have a program with a dead function in it.
    let rules = analysis();
    for (at, program) in rules.iter().enumerate() {
        if let Err(reason) = rust::fallible_functions(program) {
            panic!("rule program {at} is refused by the checker: {reason}");
        }
    }
    assert!(
        rules
            .iter()
            .any(|program| lowered(program).is_some_and(|(dead, _)| dead.contains(&true))),
        "some rule program has a dead function"
    );
    let all = vectors(&sources);
    let evaluator: Vec<&Vector> = all
        .iter()
        .filter(|vector| vector.nodes <= EVALUATOR_NODES)
        .collect();
    let mut seeded = Seeded(0x00B5_0107);
    let evaluator_mutants = seeded_mutants(&sources, &mut seeded, EVALUATOR_MUTANTS, BASE_NODES);
    let mut large = Seeded(0x00B5_0207);
    let rustc_mutants = seeded_mutants(&sources, &mut large, RUSTC_MUTANTS, BASE_NODES);
    assert_eq!(
        (
            rules.len(),
            all.len(),
            evaluator.len(),
            evaluator_mutants.len(),
            rustc_mutants.len()
        ),
        (
            RULE_PROGRAMS,
            VECTORS,
            VECTORS_WITHIN,
            EVALUATOR_MUTANTS,
            RUSTC_MUTANTS
        ),
        "the programs of the rules, the vectors, those the evaluator is given, and the mutants"
    );
    eprintln!(
        "BS-07: {} vectors ({} within {EVALUATOR_NODES} nodes), {} + {} mutants ({:?})",
        all.len(),
        evaluator.len(),
        evaluator_mutants.len(),
        rustc_mutants.len(),
        started.elapsed()
    );
    let checked = support::checked_project(&project);
    let sites = super::lower(&checked, &project, TYPING, "siteTrees");
    let dead = super::lower(&checked, &project, MODULE, "deadFunctions");
    let fallible = super::lower(&checked, &project, MODULE, "fallibleFunctions");
    let types = super::lower(&checked, &project, MODULE, "lowerTypes");
    let moving = super::lower(&checked, &project, MOVES, "applyMoves");
    let move_base = rust_moves::base();
    let move_runs: Vec<Run> = move_base.iter().map(rust_moves::run).collect();
    let move_small: Vec<Run> = move_base
        .iter()
        .zip(&move_runs)
        .filter(|(krate, _)| super::rust_validate::weight(krate) <= MOVES_EVALUATOR_NODES)
        .map(|(_, run)| run.clone())
        .collect();
    let move_evaluator_mutants: Vec<Run> =
        rust_moves::mutants(MOVES_EVALUATOR_MUTANTS, 0x0B57_0103)
            .iter()
            .map(rust_moves::run)
            .collect();
    let move_rustc_mutants: Vec<Run> = rust_moves::mutants(MOVES_RUSTC_MUTANTS, 0x0B57_0203)
        .iter()
        .map(rust_moves::run)
        .collect();
    assert_eq!(
        (
            move_base.len(),
            move_small.len(),
            move_evaluator_mutants.len(),
            move_rustc_mutants.len()
        ),
        (
            MOVES_CRATES,
            MOVES_WITHIN,
            MOVES_EVALUATOR_MUTANTS,
            MOVES_RUSTC_MUTANTS
        ),
        "the crates the moves are run on, those within the evaluator's bound, and the mutants"
    );
    let every: Vec<&Vector> = all.iter().collect();
    // On every host: the rendered rust-std packages, compiled by that host's
    // pinned rustc, agree with the oracle on the vectors, leg by leg.
    super::rust_validate::rustc_differential(
        "BS-07",
        &[
            ("siteTrees", &sites, &runs(&every, sites_leg)),
            ("deadFunctions", &dead, &runs(&every, dead_leg)),
            ("fallibleFunctions", &fallible, &runs(&every, fallible_leg)),
            ("lowerTypes", &types, &runs(&every, types_leg)),
            ("applyMoves", &moving, &move_runs),
        ],
        "vectors",
    );
    if !support::normative_host() {
        eprintln!("BS-07: platform-bound host: the evaluator, the seeded corpora, and the planted defects run on the normative host");
        return;
    }
    let mutated: Vec<&Vector> = evaluator_mutants.iter().collect();
    // The evaluator on the vectors and on the seeded mutants, for each root:
    // `detection` is what the planted defects are caught on.
    let mut detection: Vec<(Leg, Vec<Run>)> = Vec::new();
    for (name, low, leg) in [
        ("siteTrees", &sites, sites_leg as Leg),
        ("deadFunctions", &dead, dead_leg),
        ("fallibleFunctions", &fallible, fallible_leg),
        ("lowerTypes", &types, types_leg),
    ] {
        let evaluated = runs(&evaluator, leg);
        let disagreements = super::evaluate(low, &evaluated, 5);
        assert!(
            disagreements.is_empty(),
            "{name}: the evaluator and the oracle disagree on the vectors: {disagreements:?}"
        );
        let mutated_runs = runs(&mutated, leg);
        let disagreements = super::evaluate(low, &mutated_runs, 5);
        assert!(
            disagreements.is_empty(),
            "{name}: the evaluator and the oracle disagree on the seeded mutants: {disagreements:?}"
        );
        eprintln!(
            "BS-07: {name}: the evaluator agrees on {} vectors and {} seeded mutants ({:?})",
            evaluated.len(),
            mutated_runs.len(),
            started.elapsed()
        );
        detection.push((leg, evaluated.into_iter().chain(mutated_runs).collect()));
    }
    let moves_agree = |label: &str, runs: &[Run]| {
        let disagreements = super::evaluate(&moving, runs, 5);
        assert!(
            disagreements.is_empty(),
            "applyMoves: the evaluator and the oracle disagree on the {label}: {disagreements:?}"
        );
    };
    moves_agree("crates", &move_small);
    moves_agree("seeded mutants", &move_evaluator_mutants);
    eprintln!(
        "BS-07: applyMoves: the evaluator agrees on {} crates and {} seeded mutants ({:?})",
        move_small.len(),
        move_evaluator_mutants.len(),
        started.elapsed()
    );
    let move_detection: Vec<Run> = move_small
        .iter()
        .chain(&move_evaluator_mutants)
        .cloned()
        .collect();
    let large: Vec<&Vector> = rustc_mutants.iter().collect();
    super::rust_validate::rustc_differential(
        "BS-07",
        &[
            ("siteTrees", &sites, &runs(&large, sites_leg)),
            ("deadFunctions", &dead, &runs(&large, dead_leg)),
            ("fallibleFunctions", &fallible, &runs(&large, fallible_leg)),
            ("lowerTypes", &types, &runs(&large, types_leg)),
            ("applyMoves", &moving, &move_rustc_mutants),
        ],
        "seeded",
    );
    // The gate is armed: each defect planted in a copy of the source is a
    // disagreement with the oracle on its root.
    for plant in &PLANTS {
        let planted = support::P::bootstrap();
        let source = planted.read_raw(plant.file);
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(
            plant.file,
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, plant.module, plant.root);
        let runs = &detection
            .iter()
            .find(|(leg, _)| std::ptr::fn_addr_eq(*leg, plant.leg))
            .expect("the leg of the plant")
            .1;
        let disagreements = super::evaluate(&low, runs, 1);
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
    move_plants(&move_detection);
    // Under the pinned Lean, the production export certifies the root; its
    // package is the committed one and realizes the program the differential ran.
    if !support::lean_backed("BS-07") {
        return;
    }
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    // The shape of the roots is decided by the measured size of their
    // certificates, not by the cap: a part of a certificate beyond three
    // quarters of `max_file_bytes` leaves the next slice no room.
    for exported_root in &exported.output.result.roots {
        if !exported_root.root.starts_with(&format!("{MODULE}."))
            && !exported_root.root.starts_with(&format!("{MOVES}."))
            && exported_root.root != format!("{TYPING}.siteTrees")
        {
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
            "BS-07: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}, three quarters of max_file_bytes",
            exported_root.root
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
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let (low, member) =
            if let Some(member) = exported_root.root.strip_prefix(&format!("{MODULE}.")) {
                (
                    match member {
                        "deadFunctions" => &dead,
                        "fallibleFunctions" => &fallible,
                        "lowerTypes" => &types,
                        other => panic!("{other} is not a root of {MODULE}"),
                    },
                    member,
                )
            } else if exported_root.root == format!("{TYPING}.siteTrees") {
                (&sites, "siteTrees")
            } else if exported_root.root == format!("{MOVES}.applyMoves") {
                (&moving, "applyMoves")
            } else {
                continue;
            };
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
    assert_eq!(
        realized,
        ROOTS.len() + 2,
        "one certified package per root, the typing root, and the moves"
    );
    eprintln!("BS-07: passed in {:?}", started.elapsed());
}

use crate::production_export as harness;
