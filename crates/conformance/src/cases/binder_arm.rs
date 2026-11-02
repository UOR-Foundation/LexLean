//! A match arm whose whole body is a binder of a boxed field (SPEC.md §17.16,
//! §17.17): `SP-32`, #310.
//!
//! The arm loads the field and returns the load, which `fold` folds to the
//! read of the box. Certificate B's `mCons` states an arm through `foldB`, so
//! a rendering that kept the binding could not be certified at all. The case
//! holds the rendering without Lean, on a hand-built program, and the kernel's
//! acceptance on the fixture: `itemTail` and `itemHead` return either field,
//! and `bumpedTail` returns one in a function that can overflow, whose arm
//! is not folded because `Ok` wraps its value.

use lexlean::calculus::rust::ast::{Block, Expr as Rust, Ident, ItemDef};
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{Adt, Arm, Expr, Function, Program, Shape, Ty, PROGRAM_SPEC};

use crate::production_consumer::{self, AppRoot, Application};
use crate::support::{self, P};

/// The rendering of the arm `ECons a b => b`, over a type with an empty
/// constructor and a pair of itself.
fn rendered_arm() -> Block {
    let expr = Ty::Adt { index: 0 };
    let body = Expr::Match {
        ty: expr.clone(),
        scrutinee: Box::new(Expr::Var { name: 0 }),
        arms: vec![
            Arm {
                shape: Shape::Adt { constructor: 0 },
                binders: Vec::new(),
                body: Expr::Build {
                    shape: Shape::Adt { constructor: 0 },
                    ty: expr.clone(),
                    operands: Vec::new(),
                },
            },
            Arm {
                shape: Shape::Adt { constructor: 1 },
                binders: vec![1, 2],
                body: Expr::Var { name: 2 },
            },
        ],
    };
    let program = Program {
        spec: PROGRAM_SPEC.to_owned(),
        adts: vec![Adt {
            constructors: vec![Vec::new(), vec![expr.clone(), expr.clone()]],
        }],
        functions: vec![Function {
            parameters: vec![0],
            types: vec![expr.clone()],
            result: expr,
            body,
        }],
    };
    let krate = rust::lower(&program, Profile::Std).expect("the program lowers");
    let body = krate
        .items
        .into_iter()
        .find_map(|item| match item {
            ItemDef::Function { body, .. } => Some(body),
            ItemDef::Enum { .. } | ItemDef::Apply { .. } => None,
        })
        .expect("the function is rendered");
    let Rust::Match { arms, .. } = body.tail else {
        panic!("the body is a match")
    };
    let (_, arm) = arms.into_iter().nth(1).expect("the arm with binders");
    arm
}

/// `SP-32`.
///
/// # Panics
///
/// When the rendering or the fixture's export disagrees with the row.
pub fn run(id: &str) {
    assert_eq!(id, "SP-32");
    let arm = rendered_arm();
    assert!(
        arm.lets.is_empty() && matches!(arm.tail, Rust::Unbox(Ident::Boxed(_), _)),
        "the arm reads the box and binds nothing: {arm:?}"
    );
    if !support::lean_backed(id) {
        return;
    }
    let project = P::copy_tests_project("fixtures/binder-arm");
    project.relock();
    let application = Application {
        roots: vec![AppRoot {
            root: "BinderArm.Main.tailOf".to_owned(),
            name: "binder_arm".to_owned(),
            version: "1.0.0".to_owned(),
            targets: vec!["rust-std".to_owned()],
        }],
    };
    let exported =
        production_consumer::export(&project.root.join("lexlean.toml"), &application, &[]);
    if let Err(error) = exported {
        panic!(
            "the kernel accepts every certificate of `tailOf`: {:?}",
            error.diagnostics
        );
    }
}
