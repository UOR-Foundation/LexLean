//! The constructs of the heap runtime (SPEC.md §17.16): `RB-19`.
//!
//! `RuntimeProbe` defines `probeCrate`, a `std` crate of `RuntimeSyntax` with
//! one use of each construct the module grew by. The Rust crate decodes,
//! validates, and prints it; the printed crate is compiled by the pinned
//! rustc with warnings denied and a harness checks every function against an
//! independent computation. The validator's refusals are planted on the
//! decoded crate and asserted by the rule they name.

use std::collections::BTreeMap;

use lexlean::calculus::rust::rt::{self, Block, Crate, Expr, Item, Lit, Pat, Profile, Type};
use lexlean::calculus::rust::{rt_decode, rt_validate};

use crate::rust_build::{cargo_in, workspace, Built};
use crate::support::repo_root;

/// The rules of the validator that this case plants and `RB-17`'s does not:
/// those of the constructs the module grew by. `RB-17` requires the rules in
/// the validator's source to be its own planted rules and these.
pub(super) const RULES: [&str; 10] = [
    "assign-immutable",
    "assign-place",
    "break-position",
    "let-position",
    "param-duplicate",
    "pattern-duplicate",
    "pattern-fields",
    "struct-fields",
    "tuple-arity",
    "variant-name",
];

const MODULE: &str = "compiler/src/RuntimeProbe.lex.tex";
const SHIPPED: &str = "language/rust-runtime-1.2/RuntimeProbe.lex.tex";

/// The harness: every function of the probe against an independent
/// computation.
const HARNESS: &str = r#"use runtime_probe as p;
fn main() {
    for n in [0u64, 1, 5, 100, 1000] { assert_eq!(p::sum_to(n), n * (n + 1) / 2, "sum_to {n}"); }
    assert_eq!(p::count_chars("héllo"), 5);
    assert_eq!(p::count_chars(""), 0);
    assert_eq!(p::first_or(&[], 7), 7);
    assert_eq!(p::first_or(&[3, 4], 7), 3);
    assert_eq!(p::middle(&[1, 2, 3, 4]), &[2, 3]);
    assert_eq!(p::middle(&[1]), &[1]);
    assert_eq!(p::shape_area(&p::Shape::Empty), 0);
    assert_eq!(p::shape_area(&p::Shape::Dot(5)), 1);
    assert_eq!(p::shape_area(&p::Shape::Box2 { w: 3, h: 4 }), 12);
    assert_eq!([p::classify(0), p::classify(4), p::classify(7)], [10, 20, 30]);
    assert_eq!([p::collatz_steps(1), p::collatz_steps(6), p::collatz_steps(27)], [0, 8, 111]);
    assert_eq!(p::swap_pair((1, 2)), (2, 1));
    assert_eq!(p::cell_clone_count(5), 2);
    assert_eq!(p::widen_mul(u64::MAX, u64::MAX), (u64::MAX as u128) * (u64::MAX as u128));
    assert_eq!(p::to_index(7), 7usize);
    assert!(p::digit('7') && !p::digit('x'));
    assert!(p::is_a('a') && !p::is_a('b'));
    assert_eq!(p::hello_len(), 5);
    let mut x = 3u64;
    assert_eq!(p::replace_with(&mut x, 8), 3);
    assert_eq!(x, 8);
    assert_eq!([p::parse_num("42"), p::parse_num("x"), p::parse_num("-1")], [Some(42), None, None]);
    assert_eq!(p::shared_len(vec![1, 2, 3]), 3);
    assert_eq!(p::build(4), vec![0, 1, 2, 3]);
    assert_eq!([p::first_even(&[1, 3, 4, 5]), p::first_even(&[1, 3])], [Some(4), None]);
    assert_eq!(p::drain_sum(vec![1, 2, 3, 4]), 10);
    assert_eq!(p::blocky(4), 10);
    assert!(!p::flip(true) && p::flip(false));
    for a in 0u64..12 {
        for b in 0u64..12 {
            let s = u64::from(a != b) + if a > b { 2 } else { 0 } + if a >= b && b <= a { 4 } else { 0 } + if a < b || a == b { 8 } else { 0 };
            assert_eq!(p::ops(a, b), s + (a * b) % 7, "ops {a} {b}");
        }
    }
    assert_eq!(p::twin_total(3, 4), 7);
    let before = p::drops();
    p::make_and_drop();
    p::make_and_drop();
    assert_eq!(p::drops() - before, 2);
    println!("probe");
}
"#;

fn compile_and_run(text: &str) -> Result<String, String> {
    let directory = tempfile::tempdir().expect("a directory");
    let files: BTreeMap<String, Vec<u8>> = BTreeMap::from([
        (
            "Cargo.toml".to_owned(),
            b"[package]\nname = \"runtime_probe\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[lints.rust]\nwarnings = \"deny\"\n".to_vec(),
        ),
        ("src/lib.rs".to_owned(), text.as_bytes().to_vec()),
    ]);
    workspace(
        directory.path(),
        &[Built {
            name: "runtime_probe".to_owned(),
            files,
            harness: HARNESS.to_owned(),
        }],
    );
    let output = cargo_in(
        directory.path(),
        &["run", "--quiet", "--offline", "-p", "run_runtime_probe"],
    );
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        Err(format!(
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// `krate` with the body of its function `name` given one more statement at
/// the front.
fn with_statement(krate: &Crate, name: &str, statement: Expr) -> Crate {
    let mut out = krate.clone();
    for item in &mut out.items {
        if let Item::Fn(function) = item {
            if function.sig.name == name {
                function.body.statements.insert(0, statement.clone());
            }
        }
    }
    out
}

/// `krate` with the function `name` given one more parameter.
fn with_param(krate: &Crate, name: &str, param: rt::Param) -> Crate {
    let mut out = krate.clone();
    for item in &mut out.items {
        if let Item::Fn(function) = item {
            if function.sig.name == name {
                function.sig.params.push(param.clone());
            }
        }
    }
    out
}

fn var(name: &str) -> Box<Expr> {
    Box::new(Expr::Var(name.to_owned()))
}

/// Run the case of `RB-19`.
pub fn run(id: &str) {
    assert_eq!(id, "RB-19");
    let root = repo_root();
    let read = |path: &str| {
        std::fs::read_to_string(root.join(path).as_std_path())
            .unwrap_or_else(|error| panic!("{path}: {error}"))
    };
    let module = read(MODULE);
    let shipped = read(SHIPPED);
    assert_eq!(module, shipped, "{SHIPPED} differs from {MODULE}");

    let krate = rt_decode::decode_named(&shipped, "probeCrate").expect("the probe decodes");
    assert_eq!(krate.profile, Profile::Std);
    assert!(
        krate.items.len() >= 30,
        "the probe holds an item of each construct"
    );
    rt_validate::validate(&krate).expect("the probe is valid");
    let text = rt::print(&krate);
    let ran = compile_and_run(&text).expect("the printed probe computes what the harness states");
    assert_eq!(ran, "probe");

    // Each construct the module grew by occurs in the probe, so that the
    // harness's independent computation is evidence for it: the identifiers
    // of the decoded crate's debug form, each with whether an argument list
    // follows (`Char(` the literal, `Char` the type).
    let seen = format!("{krate:?}");
    let mut tokens: std::collections::BTreeSet<(String, bool)> = std::collections::BTreeSet::new();
    let bytes = seen.as_bytes();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at].is_ascii_uppercase() && (at == 0 || !bytes[at - 1].is_ascii_alphanumeric()) {
            let end = (at..bytes.len())
                .find(|&to| !bytes[to].is_ascii_alphanumeric())
                .unwrap_or(bytes.len());
            tokens.insert((seen[at..end].to_owned(), bytes.get(end) == Some(&b'(')));
            at = end;
        } else {
            at += 1;
        }
    }
    for (name, arguments) in [
        // items and variants
        ("TupleStruct", true),
        ("RecordStruct", true),
        ("Enum", true),
        ("Unit", true),
        ("Record", true),
        // types
        ("Wide", true),
        ("Size", false),
        ("Char", false),
        ("Str", false),
        ("Slice", true),
        ("RefMut", true),
        // literals
        ("Char", true),
        ("Str", true),
        // patterns
        ("BindMut", true),
        ("Tuple", true),
        ("TupleStruct", true),
        ("Struct", true),
        ("Lit", true),
        // expressions
        ("Let", true),
        ("Assign", true),
        ("If", true),
        ("While", true),
        ("WhileLet", true),
        ("Loop", true),
        ("For", true),
        ("Break", false),
        ("Return", true),
        ("StructLit", true),
        ("Binary", true),
        ("Not", true),
        ("Cast", true),
        ("Index", true),
        ("RangeSlice", true),
        ("Deref", true),
        ("BorrowMut", true),
        ("MethodOf", true),
        ("Block", true),
        ("FieldNamed", true),
    ] {
        assert!(
            tokens.contains(&(name.to_owned(), arguments)),
            "the probe has no `{name}`"
        );
    }
    for op in [
        "Add", "Sub", "Mul", "Div", "Rem", "AndAnd", "OrOr", "Ne", "Lt", "Le", "Gt", "Ge",
    ] {
        assert!(
            tokens.contains(&(op.to_owned(), false)),
            "the probe has no operator `{op}`"
        );
    }
    assert!(seen.contains("guard: Some"), "the probe has no guarded arm");

    // The closed table of the standard library is the validator's, as SPEC.md
    // §17.16 lists it, and the register states its size.
    let spec = read("SPEC.md").replace('\n', " ");
    let names = |from: &str, to: &str| -> Vec<String> {
        let start = spec
            .find(from)
            .unwrap_or_else(|| panic!("SPEC.md lacks `{from}`"))
            + from.len();
        let end = start
            + spec[start..]
                .find(to)
                .unwrap_or_else(|| panic!("SPEC.md lacks `{to}`"));
        let mut out: Vec<String> = spec[start..end]
            .split('`')
            .enumerate()
            .filter(|(at, _)| at % 2 == 1)
            .map(|(_, name)| name.to_owned())
            .collect();
        out.sort();
        out
    };
    let mut table = rt_validate::table();
    table.sort();
    assert_eq!(
        names(
            "The closed table of the standard library (51 names): ",
            ". A call"
        ),
        table,
        "SPEC.md §17.16 lists the validator's table"
    );
    assert_eq!(table.len(), 51, "the register states 51 names");
    let mut allocating = rt_validate::allocating();
    allocating.sort();
    assert_eq!(
        names(
            "The names of the table that allocate (",
            ") are refused in the core profile"
        ),
        allocating,
        "SPEC.md §17.16 lists the names that allocate"
    );
    let register = read("model/ids.toml");
    assert!(
        register.contains("(51 names, which SPEC.md §17.16 lists"),
        "the register states the size"
    );

    // The core profile refuses every allocating name of the table as the
    // hidden allocation the validator refuses in every `rust-core` crate.
    let mut core = krate.clone();
    core.profile = Profile::Core;
    core.items
        .retain(|item| !matches!(item, Item::Use(path, _) if path[0] == "std"));
    let error = rt_validate::validate(&core).expect_err("the probe allocates");
    assert!(error.contains("hidden-allocation"), "{error}");

    // Each refusal of the validator, planted on the decoded crate and
    // asserted by the rule it names.
    let unit = Expr::Lit(Lit::Unit);
    let mut planted_rules: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for (what, planted, rule) in [
        (
            "a break outside a loop",
            with_statement(&krate, "flip", Expr::Break),
            "break-position",
        ),
        (
            "a variable not in scope",
            with_statement(&krate, "flip", Expr::Var("nope".to_owned())),
            "variable-scope",
        ),
        (
            "a call with another number of arguments",
            with_statement(
                &krate,
                "flip",
                Expr::Call(vec!["Vec".to_owned(), "new".to_owned()], vec![unit.clone()]),
            ),
            "call-arity",
        ),
        (
            "a call that is neither declared nor in the table",
            with_statement(&krate, "flip", Expr::Call(vec!["frob".to_owned()], vec![])),
            "call-name",
        ),
        (
            "a method of no impl and not in the table",
            with_statement(
                &krate,
                "flip",
                Expr::Method(var("b"), "frobnicate".to_owned(), vec![]),
            ),
            "method-name",
        ),
        (
            "a method with another number of arguments",
            with_statement(
                &krate,
                "flip",
                Expr::Method(var("b"), "len".to_owned(), vec![unit.clone()]),
            ),
            "method-arity",
        ),
        (
            "a struct literal that names a field the struct lacks",
            with_statement(
                &krate,
                "flip",
                Expr::StructLit(
                    vec!["Cell2".to_owned()],
                    vec!["head".to_owned(), "weight".to_owned()],
                    vec![unit.clone(), unit.clone()],
                ),
            ),
            "struct-fields",
        ),
        (
            "a tuple of one",
            with_statement(&krate, "flip", Expr::Tuple(vec![unit.clone()])),
            "tuple-arity",
        ),
        (
            "a pattern that binds a name twice",
            with_statement(
                &krate,
                "flip",
                Expr::Let(
                    Pat::Tuple(vec![Pat::Bind("a".to_owned()), Pat::Bind("a".to_owned())]),
                    None,
                    Box::new(Expr::Tuple(vec![unit.clone(), unit.clone()])),
                ),
            ),
            "pattern-duplicate",
        ),
        (
            "an assignment to a name that is not mutable",
            with_statement(
                &krate,
                "sum_to",
                Expr::Assign(var("n"), Box::new(Expr::Lit(Lit::Nat(0)))),
            ),
            "assign-immutable",
        ),
        (
            "a let that is not a statement",
            with_statement(
                &krate,
                "flip",
                Expr::If(
                    Box::new(Expr::Let(Pat::Wild, None, Box::new(unit.clone()))),
                    Box::new(Block {
                        statements: vec![],
                        tail: unit.clone(),
                    }),
                    Box::new(Block {
                        statements: vec![],
                        tail: unit.clone(),
                    }),
                ),
            ),
            "let-position",
        ),
        (
            "an assignment to a value that is no place",
            with_statement(
                &krate,
                "flip",
                Expr::Assign(Box::new(unit.clone()), Box::new(unit.clone())),
            ),
            "assign-place",
        ),
        (
            "two parameters of one name",
            with_param(
                &krate,
                "flip",
                rt::Param {
                    name: "b".to_owned(),
                    mutable: false,
                    ty: Type::Bool,
                },
            ),
            "param-duplicate",
        ),
        (
            "a struct pattern that names a field the variant lacks",
            with_statement(
                &krate,
                "flip",
                Expr::Match(
                    Box::new(unit.clone()),
                    vec![rt::Arm {
                        pattern: Pat::Struct(
                            vec!["Shape".to_owned(), "Box2".to_owned()],
                            vec!["zzz".to_owned()],
                            vec![Pat::Wild],
                        ),
                        guard: None,
                        body: unit.clone(),
                    }],
                ),
            ),
            "pattern-fields",
        ),
        (
            "a variant the enum lacks",
            with_statement(
                &krate,
                "flip",
                Expr::Path(vec!["Shape".to_owned(), "Round".to_owned()]),
            ),
            "variant-name",
        ),
        (
            "a type that is neither declared nor in the table",
            with_statement(
                &krate,
                "flip",
                Expr::Let(
                    Pat::Wild,
                    Some(Type::Named("Frob".to_owned(), vec![])),
                    Box::new(unit.clone()),
                ),
            ),
            "type-name",
        ),
    ] {
        let error = rt_validate::validate(&planted).expect_err(what);
        assert!(
            error.contains(&format!("runtime support: {rule}")),
            "{what}: {error}"
        );
        planted_rules.insert(rule);
    }
    for rule in RULES {
        assert!(
            planted_rules.contains(rule),
            "the rule `{rule}` has no plant"
        );
    }

    // A module the decoder refuses, each for the rule it breaks.
    for (what, from, to, message) in [
        (
            "an expression constructor RuntimeSyntax lacks",
            "RtExpr.whileLoop",
            "RtExpr.whileLoops",
            "is not an expression",
        ),
        (
            "a pattern constructor RuntimeSyntax lacks",
            "RtPat.bindMut",
            "RtPat.bindMutt",
            "is not a pattern",
        ),
        (
            "an operator RuntimeSyntax lacks",
            "RtOp.rem",
            "RtOp.remm",
            "is not an operator",
        ),
        (
            "a constructor with another number of arguments",
            "(ctor RuntimeSyntax::RtExpr.breakE)",
            "(ctor RuntimeSyntax::RtExpr.breakE (nil RuntimeSyntax::RtExpr))",
            "takes 0 arguments",
        ),
    ] {
        let edited = shipped.replacen(from, to, 1);
        assert_ne!(edited, shipped, "{what}: the plant site exists");
        let error = rt_decode::decode_named(&edited, "probeCrate").expect_err(what);
        assert!(error.contains(message), "{what}: {error}");
    }

    // The module is accepted by the pinned Lean: its term is well typed
    // against `RuntimeSyntax`.
    if crate::support::lean_backed("RB-19") {
        let project = super::rust_refinement::project_of(
            &[("RuntimeProbe", module.as_str())],
            &["TargetSyntax", "RuntimeSyntax", "RuntimeProbe"],
        );
        let _ = crate::support::verify_ok(&project);
    }
}
