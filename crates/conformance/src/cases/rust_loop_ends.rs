//! The ends of a fallible loop (SPEC.md §17.16, issue 279): `RB-21`.
//!
//! A function whose self calls are all tail calls is rendered as a loop
//! (`RB-14`): a tail self call continues it with `Ok` of the next state, and
//! every other tail value `e` ends it as `Err(e)`, so the loop breaks with
//! `e`. When the function is fallible and the loop is its value, `e` is the
//! value the function returns, so a call of another fallible function there
//! returns that function's result as it is, as it would in the tail of a
//! function that is not a loop. The project of the case holds the module of
//! issue 279 as it was filed and a variant whose callee is itself a loop.

use std::collections::BTreeMap;

use lexlean::api::production::ReplayStatus;
use lexlean::calculus::rust::ast::{
    Block, Callee, Crate, Ctor, Expr, Ident, ItemDef, Let, Pat, Type,
};
use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::{self, validate, Profile};
use lexlean::calculus::{self as target, interp, Value};
use lexlean::production::lower::{linked_modules, lower_root, roots, Lowered};
use serde_json::Value as Json;

use crate::production_consumer::{self, AppRoot, Application};
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support::{self, P};

/// The committed project, below `tests/`.
const PROJECT: &str = "rust-backend/loop-ends";

/// The roots and their crate names: the module of issue 279, whose loop
/// ends with a call of a function that is not a loop, and the variant whose
/// loop ends with a call of a function that is.
const ROOTS: [(&str, &str); 2] = [
    ("LoopEnds.Main.run", "loop_end_call"),
    ("LoopEnds.Looped.run", "loop_end_loop"),
];

const FUEL: u64 = 1_000_000;

fn nat(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

/// The inputs of a root: counts the loop runs, and for the variant the
/// accumulators that reach the callee's overflow and those that do not.
fn inputs(root: &str) -> Vec<Vec<Value>> {
    let counts = [0, 1, 2, 3, 10, 100];
    if root == ROOTS[0].0 {
        return counts.iter().map(|n| vec![nat(*n)]).collect();
    }
    let accumulators = [
        0,
        1,
        1 << 63,
        u64::MAX - 4,
        u64::MAX - 3,
        u64::MAX - 2,
        u64::MAX - 1,
        u64::MAX,
    ];
    let mut out = Vec::new();
    for n in counts {
        for a in accumulators {
            out.push(vec![nat(n), nat(a)]);
        }
    }
    out
}

fn lower(checked: &lexlean::link::CheckedProject, project: &P, root: &str) -> Lowered {
    let linked = linked_modules(checked);
    let found = roots(checked).expect("the production roots");
    let found = found
        .iter()
        .find(|candidate| candidate.report.root == root)
        .unwrap_or_else(|| panic!("`{root}` is a production root"));
    lower_root(
        &linked,
        &found.module,
        &found.name,
        found.report,
        &support::limits(project),
    )
    .expect("the root lowers")
}

/// The functions of a crate by number: whether each returns `R<T>`, and its
/// body.
fn functions(krate: &Crate) -> BTreeMap<u64, (bool, &Block)> {
    krate
        .items
        .iter()
        .filter_map(|item| match item {
            ItemDef::Function {
                name: Ident::Function(number),
                result,
                body,
                ..
            } => Some((*number, (matches!(result, Type::Fallible(_)), body))),
            _ => None,
        })
        .collect()
}

/// The loop that is the value of a function rendered as one.
fn loop_of(body: &Block) -> Option<&Block> {
    match &body.tail {
        Expr::Loop { body, .. } => Some(body),
        _ => None,
    }
}

/// The `Err` expressions at the tails of a loop body: the ends of the loop.
fn ends<'a>(block: &'a Block, out: &mut Vec<&'a Expr>) {
    match &block.tail {
        end @ Expr::Construct {
            ctor: Ctor::Err(..),
            ..
        } => out.push(end),
        Expr::If {
            then_branch,
            else_branch,
            ..
        } => {
            ends(then_branch, out);
            ends(else_branch, out);
        }
        Expr::Match { arms, .. } => arms.iter().for_each(|(_, body)| ends(body, out)),
        Expr::Block(inner) => ends(inner, out),
        _ => {}
    }
}

/// The function an end calls, when the end returns the result of a call of
/// another function as it is.
fn called(end: &Expr, me: u64) -> Option<u64> {
    let Expr::Construct { args, .. } = end else {
        return None;
    };
    match args.as_slice() {
        [Expr::Call {
            callee: Callee::Function(callee),
            propagate: false,
            ..
        }] if *callee != me => Some(*callee),
        _ => None,
    }
}

/// The looped fallible functions of a crate whose loop ends with a call of
/// another fallible function, with the function each calls.
fn tail_calls(krate: &Crate) -> Vec<(u64, u64)> {
    let all = functions(krate);
    let mut out = Vec::new();
    for (me, (fallible, body)) in &all {
        let Some(looped) = loop_of(body) else {
            continue;
        };
        let mut found = Vec::new();
        ends(looped, &mut found);
        for end in found {
            if let Some(callee) = called(end, *me) {
                assert!(
                    *fallible,
                    "f{me} ends a loop with a call but is not fallible"
                );
                assert!(
                    all[&callee].0,
                    "f{callee}, called at a loop's end, is fallible"
                );
                out.push((*me, callee));
            }
        }
    }
    out
}

/// Rewrite the first end of the loop of `me` that returns another
/// function's result with `rewrite`.
fn plant(krate: &Crate, me: u64, rewrite: &dyn Fn(Expr) -> Expr) -> Crate {
    fn walk(block: &mut Block, me: u64, rewrite: &dyn Fn(Expr) -> Expr) -> bool {
        match &mut block.tail {
            Expr::If {
                then_branch,
                else_branch,
                ..
            } => walk(then_branch, me, rewrite) || walk(else_branch, me, rewrite),
            Expr::Match { arms, .. } => arms.iter_mut().any(|(_, body)| walk(body, me, rewrite)),
            Expr::Block(inner) => walk(inner, me, rewrite),
            end => {
                if called(end, me).is_none() {
                    return false;
                }
                let taken = std::mem::replace(end, Expr::Lit(rust::ast::Lit::Unit, end_origin()));
                *end = rewrite(taken);
                true
            }
        }
    }
    let mut out = krate.clone();
    for item in &mut out.items {
        if let ItemDef::Function {
            name: Ident::Function(number),
            body,
            ..
        } = item
        {
            if *number == me {
                if let Expr::Loop { body: looped, .. } = &mut body.tail {
                    assert!(walk(looped, me, rewrite), "f{me}: the end is planted");
                    return out;
                }
            }
        }
    }
    panic!("f{me} is a loop of the crate");
}

fn end_origin() -> rust::ast::Origin {
    rust::ast::Origin::of("expr:call")
}

/// The operand of an `Err` end: the call it returns.
fn operand(end: Expr) -> Expr {
    match end {
        Expr::Construct { mut args, .. } => args.pop().expect("an end has an operand"),
        other => panic!("not an end: {other:?}"),
    }
}

/// The case.
///
/// # Panics
///
/// When an assertion of `RB-21` fails.
#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    assert_eq!(id, "RB-21");
    let project = P::copy_tests_project(PROJECT);
    let checked = support::checked_project(&project);
    let lowered: Vec<(&str, &str, Lowered)> = ROOTS
        .iter()
        .map(|(root, name)| (*root, *name, lower(&checked, &project, root)))
        .collect();
    for (root, _, low) in &lowered {
        // The rendering exists and its loop ends with the callee's result.
        let krate = rust::lower(&low.program, Profile::Std)
            .unwrap_or_else(|reason| panic!("{root}: no rendering in rust-std: {reason}"));
        validate::validate(&krate).expect("a rendering is valid");
        let calls = tail_calls(&krate);
        assert!(
            !calls.is_empty(),
            "{root}: a looped fallible function ends its loop with the result of another fallible call"
        );
        let all = functions(&krate);
        let looped_callee = calls
            .iter()
            .any(|(_, callee)| loop_of(all[callee].1).is_some());
        assert_eq!(
            looped_callee,
            *root == ROOTS[1].0,
            "{root}: the callee is itself a loop exactly in the variant"
        );
        // The validator still refuses the same call where it does not
        // return: bound before the end, or left as the body's value with the
        // end dropped.
        for (me, callee) in &calls {
            let refusal = format!(
                "arithmetic: the fallible call of f{callee} does not propagate its overflow"
            );
            let bound = plant(&krate, *me, &|end| {
                let held = Ident::Holder(1_000_000);
                let call = operand(end.clone());
                let Expr::Construct { ctor, at, .. } = end else {
                    unreachable!("an end is a construct")
                };
                Expr::Construct {
                    ctor,
                    args: vec![Expr::Block(Box::new(Block {
                        lets: vec![Let {
                            pat: Pat::Bind(held.clone()),
                            ty: None,
                            value: call,
                            at: end_origin(),
                        }],
                        tail: Expr::Succeed(
                            Box::new(Expr::Move(held, end_origin())),
                            rust::ast::Origin::of("overflow"),
                        ),
                    }))],
                    at,
                }
            });
            let error = validate::validate(&bound)
                .expect_err("a call bound before the loop's end does not propagate");
            assert!(error.contains(&refusal), "{root}: {error}");
            let dropped = plant(&krate, *me, &operand);
            let error = validate::validate(&dropped)
                .expect_err("a call left as the loop body's value does not end the loop");
            assert!(error.contains(&refusal), "{root}: {error}");
        }
    }
    // The rendered packages, compiled by the pinned rustc, agree with the
    // calculus evaluator on every input, an overflow included.
    let dir = tempfile::Builder::new()
        .prefix("lexlean-loop-ends-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    let mut overflows = 0usize;
    for (root, name, low) in &lowered {
        let manifest = rust_packages::manifest(name, &low.program, low.entry(), Profile::Std);
        let package = target::package(&manifest.to_file_bytes().expect("canonical manifest"))
            .unwrap_or_else(|failure| panic!("{root}: {failure:?}"));
        let arguments = inputs(root);
        let outcomes: Vec<Json> = arguments
            .iter()
            .map(|arguments| {
                let outcome = interp::run(&low.program, FUEL, low.entry(), arguments);
                rust::observable(&outcome)
                    .unwrap_or_else(|| panic!("{root} on {arguments:?}: {outcome:?}"))
            })
            .collect();
        overflows += outcomes
            .iter()
            .filter(|outcome| outcome["kind"] == "overflow")
            .count();
        let passing = vec![Passing::Own; arguments[0].len()];
        let harness = rust_harness::render_calls(
            &low.program,
            Profile::Std,
            &manifest.name,
            &[Calls {
                entry: low.entry(),
                function: "run",
                passing: &passing,
                arguments: &arguments,
            }],
        )
        .expect("the differential harness");
        expected.insert(manifest.name.clone(), outcomes);
        crates.push(Built {
            name: manifest.name,
            files: package.files,
            harness,
        });
    }
    assert!(overflows > 0, "some input reaches the callee's overflow");
    workspace(dir.path(), &crates);
    let build = cargo_in(
        dir.path(),
        &["build", "--offline", "--workspace", "--quiet"],
    );
    assert!(
        build.status.success(),
        "a package does not build:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    for (name, outcomes) in &expected {
        let binary = dir
            .path()
            .join("target/debug")
            .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
        let ran = std::process::Command::new(&binary)
            .output()
            .expect("the differential runs");
        assert!(
            ran.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        let stdout = String::from_utf8(ran.stdout).expect("utf8");
        let printed: Vec<Json> = stdout
            .lines()
            .filter(|line| !line.starts_with("work "))
            .map(|line| {
                serde_json::from_str(line).unwrap_or_else(|error| panic!("{name}: {error}: {line}"))
            })
            .collect();
        assert_eq!(
            &printed, outcomes,
            "{name}: the rendering and the evaluator agree"
        );
    }
    // Under the pinned Lean, the production export certifies both roots
    // and the published bundle replays; its packages realize the programs the
    // differential ran.
    if !support::lean_backed(id) {
        return;
    }
    let application = Application {
        roots: ROOTS
            .iter()
            .map(|(root, name)| AppRoot {
                root: (*root).to_owned(),
                name: (*name).to_owned(),
                version: "1.0.0".to_owned(),
                targets: vec!["rust-std".to_owned()],
            })
            .collect(),
    };
    let (output, report) =
        production_consumer::export(&project.root.join("lexlean.toml"), &application, &[])
            .unwrap_or_else(|error| panic!("the roots export: {:?}", error.diagnostics));
    assert_eq!(
        (report.status, report.outputs),
        (ReplayStatus::BindingsVerified, ROOTS.len() as u64),
        "the published bundle replays both packages"
    );
    let bundle = project.root.join(&output.bundle);
    for root in &output.result.roots {
        let low = &lowered
            .iter()
            .find(|(name, _, _)| *name == root.root)
            .unwrap_or_else(|| panic!("`{}` is a root of the case", root.root))
            .2;
        for exported in &root.outputs {
            let package: Json = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&exported.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("package JSON");
            assert_eq!(
                package["program"],
                serde_json::to_value(&low.program).expect("program JSON"),
                "{}: the exported package realizes the program the differential ran",
                root.root
            );
        }
    }
    assert_eq!(output.result.roots.len(), ROOTS.len(), "both roots export");
}
