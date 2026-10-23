//! The constructs of the heap runtime (SPEC.md §17.16): `RB-19`.
//!
//! `RuntimeProbe` defines `probeCrate`, a `std` crate of `RuntimeSyntax` with
//! one use of each construct the module grew by. The Rust crate decodes,
//! validates, and prints it; the printed crate is compiled by the pinned
//! rustc with warnings denied and a harness checks every function against an
//! independent computation. The validator's refusals are planted on the
//! decoded crate and asserted by the rule they name.

use std::collections::BTreeMap;

use lexlean::calculus::rust::rt::{
    self, Block, Crate, Expr, Function, Generic, Item, Lit, Op, Pat, Profile, Sig, Type, Variant,
};
use lexlean::calculus::rust::{rt_decode, rt_validate};
use lexlean::calculus::IntKind;

use crate::rust_build::{cargo_in, workspace, Built};
use crate::support::repo_root;

/// The rules of the validator that this case plants and `RB-17`'s does not:
/// those of the constructs the module grew by (the eleventh new rule,
/// `hidden-allocation`, is planted by `RB-17`'s text plants and here on the probe). `RB-17` requires the rules in
/// the validator's source to be its own planted rules and these.
pub(super) const RULES: [&str; 15] = [
    "assign-immutable",
    "assign-place",
    "break-position",
    "char-literal",
    "derive",
    "identifier",
    "let-position",
    "param-duplicate",
    "pattern-duplicate",
    "pattern-fields",
    "struct-fields",
    "struct-head",
    "tuple-arity",
    "type-scope",
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
    assert_eq!(p::widen_signed(u64::MAX), u64::MAX as i128);
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
    compile_with(text, HARNESS)
}

fn compile_with(text: &str, harness: &str) -> Result<String, String> {
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
            harness: harness.to_owned(),
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

/// A crate of the std profile holding `items`.
fn tiny(items: Vec<Item>) -> Crate {
    Crate {
        profile: Profile::Std,
        items,
    }
}

/// `pub fn name(params) { statements }`, with the generics given.
fn function(
    name: &str,
    params: Vec<rt::Param>,
    generics: Vec<Generic>,
    statements: Vec<Expr>,
) -> Item {
    Item::Fn(Function {
        sig: Sig {
            name: name.to_owned(),
            receiver: false,
            exclusive: false,
            params,
            result: Type::Unit,
        },
        generics,
        public: true,
        body: Block {
            statements,
            tail: Expr::Lit(Lit::Unit),
        },
    })
}

fn param(name: &str, ty: Type) -> rt::Param {
    rt::Param {
        name: name.to_owned(),
        mutable: false,
        ty,
    }
}

fn strings(list: &[&str]) -> Vec<String> {
    list.iter().map(|name| (*name).to_owned()).collect()
}

fn var(name: &str) -> Box<Expr> {
    Box::new(Expr::Var(name.to_owned()))
}

/// The constructors of `RuntimeSyntax` that the heap runtime added, each
/// with the enum it belongs to.
const GROWN: [&str; 53] = [
    "RtItem.tupleStruct",
    "RtItem.recordStruct",
    "RtItem.enumItem",
    "RtVariant.unitV",
    "RtVariant.tupleV",
    "RtVariant.structV",
    "RtType.wide",
    "RtType.size",
    "RtType.char",
    "RtType.str",
    "RtType.slice",
    "RtType.refMut",
    "RtLit.char",
    "RtLit.str",
    "RtPat.bindMut",
    "RtPat.tuple",
    "RtPat.tupleStruct",
    "RtPat.structPat",
    "RtPat.lit",
    "RtExpr.letE",
    "RtExpr.assign",
    "RtExpr.ifElse",
    "RtExpr.whileLoop",
    "RtExpr.whileLet",
    "RtExpr.loopE",
    "RtExpr.forIn",
    "RtExpr.breakE",
    "RtExpr.returnE",
    "RtExpr.tuple",
    "RtExpr.structLit",
    "RtExpr.binary",
    "RtExpr.notE",
    "RtExpr.cast",
    "RtExpr.index",
    "RtExpr.rangeSlice",
    "RtExpr.deref",
    "RtExpr.borrowMut",
    "RtExpr.methodOf",
    "RtExpr.blockE",
    "RtExpr.fieldNamed",
    "RtArm.guarded",
    "RtOp.add",
    "RtOp.sub",
    "RtOp.mul",
    "RtOp.div",
    "RtOp.rem",
    "RtOp.andAnd",
    "RtOp.orOr",
    "RtOp.ne",
    "RtOp.lt",
    "RtOp.le",
    "RtOp.gt",
    "RtOp.ge",
];

/// The grown constructors that `module` never applies.
fn constructors_absent(module: &str) -> Vec<String> {
    GROWN
        .iter()
        .filter(|name| {
            !module.match_indices(**name).any(|(at, _)| {
                !module[at + name.len()..]
                    .starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
            })
        })
        .map(|name| (*name).to_owned())
        .collect()
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
    // harness's independent computation is evidence for it. The constructors
    // are named with their enum, as the module spells them: the debug form of
    // the decoded crate does not tell `RtPat.tuple` from `RtExpr.tuple`.
    let absent = constructors_absent(&shipped);
    assert!(absent.is_empty(), "the probe has no {absent:?}");
    for (constructor, text) in [
        ("RtPat.lit", shipped.replace("RtPat.lit", "RtPat.wild")),
        (
            "RtExpr.tuple",
            shipped.replace("RtExpr.tuple", "RtExpr.lit"),
        ),
        (
            "RtVariant.tupleV",
            shipped.replace("RtVariant.tupleV", "RtVariant.unitV"),
        ),
        ("RtLit.str", shipped.replace("RtLit.str", "RtLit.nat")),
    ] {
        assert_eq!(
            constructors_absent(&text),
            vec![constructor.to_owned()],
            "a probe without `{constructor}` is caught"
        );
    }
    assert!(
        format!("{krate:?}").contains("guard: Some"),
        "the probe has no guarded arm"
    );

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

    // The sites of the rules, each planted alone: a rule is refused where the
    // construct is, not only by name. A name the printer writes is an
    // identifier, a derive is in the closed table, a character literal is one
    // scalar value, and the head of a condition holds no struct literal.
    let unit_item = |item: Item| tiny(vec![item]);
    let generic = |name: &str| Generic {
        name: name.to_owned(),
        bounds: vec![],
    };
    let flip = |statement: Expr| with_statement(&krate, "flip", statement);
    let match_arm = |pattern: Pat| {
        Expr::Match(
            Box::new(unit.clone()),
            vec![rt::Arm {
                pattern,
                guard: None,
                body: unit.clone(),
            }],
        )
    };
    let hostile = "x }; pub fn g() -> u8 { 0 }";
    let literal = Expr::StructLit(vec!["Cell2".to_owned()], vec![], vec![]);
    let empty_block = || {
        Box::new(Block {
            statements: vec![],
            tail: unit.clone(),
        })
    };
    let generic_body = |statement: Expr| {
        tiny(vec![function(
            "f",
            vec![],
            vec![generic("T")],
            vec![statement],
        )])
    };
    let mut sites: Vec<(&str, Crate, &str, &str)> = vec![
        // identifiers, one site each
        (
            "a unit struct name",
            unit_item(Item::UnitStruct(hostile.to_owned(), vec![])),
            "identifier",
            "a unit struct name",
        ),
        (
            "a tuple struct name",
            unit_item(Item::TupleStruct(
                hostile.to_owned(),
                vec![],
                vec![],
                vec![],
            )),
            "identifier",
            "a tuple struct name",
        ),
        (
            "a record struct name",
            unit_item(Item::RecordStruct(
                hostile.to_owned(),
                vec![],
                vec![],
                vec![],
                vec![],
            )),
            "identifier",
            "a record struct name",
        ),
        (
            "an enum name",
            unit_item(Item::Enum(hostile.to_owned(), vec![], vec![], vec![])),
            "identifier",
            "an enum name",
        ),
        (
            "an alias name",
            unit_item(Item::Alias(hostile.to_owned(), vec![], Type::Unit)),
            "identifier",
            "an alias name",
        ),
        (
            "a static name",
            unit_item(Item::Static(
                hostile.to_owned(),
                Type::Nat,
                Expr::Lit(Lit::Nat(0)),
            )),
            "identifier",
            "a static name",
        ),
        (
            "a trait name",
            unit_item(Item::Trait(hostile.to_owned(), vec![])),
            "identifier",
            "a trait name",
        ),
        (
            "a function name",
            unit_item(function(hostile, vec![], vec![], vec![])),
            "identifier",
            "a function name",
        ),
        (
            "a generic of a tuple struct",
            unit_item(Item::TupleStruct(
                "T".to_owned(),
                strings(&["P>"]),
                vec![],
                vec![],
            )),
            "identifier",
            "a tuple struct generic",
        ),
        (
            "a generic of a record struct",
            unit_item(Item::RecordStruct(
                "R".to_owned(),
                strings(&["P>"]),
                vec![],
                vec![],
                vec![],
            )),
            "identifier",
            "a record struct generic",
        ),
        (
            "a generic of an enum",
            unit_item(Item::Enum("E".to_owned(), strings(&["P>"]), vec![], vec![])),
            "identifier",
            "an enum generic",
        ),
        (
            "a parameter of an alias",
            unit_item(Item::Alias("A".to_owned(), strings(&["P>"]), Type::Unit)),
            "identifier",
            "an alias parameter",
        ),
        (
            "a generic of a function",
            unit_item(function("f", vec![], vec![generic("P>")], vec![])),
            "identifier",
            "a generic parameter",
        ),
        (
            "a record field name",
            unit_item(Item::RecordStruct(
                "R".to_owned(),
                vec![],
                vec![],
                strings(&["a: u8, pub b"]),
                vec![Type::Nat],
            )),
            "identifier",
            "a record field name",
        ),
        (
            "a variant name",
            unit_item(Item::Enum(
                "E".to_owned(),
                vec![],
                vec![],
                vec![Variant::Unit(hostile.to_owned())],
            )),
            "identifier",
            "a variant name",
        ),
        (
            "a record variant field name",
            unit_item(Item::Enum(
                "E".to_owned(),
                vec![],
                vec![],
                vec![Variant::Record(
                    "V".to_owned(),
                    strings(&["x y"]),
                    vec![Type::Nat],
                )],
            )),
            "identifier",
            "a record variant field name",
        ),
        (
            "a method name of a trait",
            unit_item(Item::Trait(
                "Tr".to_owned(),
                vec![Sig {
                    name: hostile.to_owned(),
                    receiver: true,
                    exclusive: false,
                    params: vec![],
                    result: Type::Unit,
                }],
            )),
            "identifier",
            "a method name",
        ),
        (
            "a parameter name",
            unit_item(function(
                "f",
                vec![param(hostile, Type::Nat)],
                vec![],
                vec![],
            )),
            "identifier",
            "a parameter name",
        ),
        (
            "a `use` segment",
            unit_item(Item::Use(strings(&["core", "a b"]), None)),
            "identifier",
            "a `use` segment",
        ),
        (
            "a `use` alias",
            unit_item(Item::Use(
                strings(&["core", "cmp", "Ordering"]),
                Some(hostile.to_owned()),
            )),
            "identifier",
            "a `use` alias",
        ),
        (
            "a binding",
            flip(Expr::Let(
                Pat::Bind("a b".to_owned()),
                None,
                Box::new(unit.clone()),
            )),
            "identifier",
            "a pattern binding",
        ),
        (
            "a mutable binding",
            flip(Expr::Let(
                Pat::BindMut("a b".to_owned()),
                None,
                Box::new(unit.clone()),
            )),
            "identifier",
            "a pattern binding",
        ),
        (
            "a field by name",
            flip(Expr::FieldNamed(
                var("b"),
                "0 } pub fn g() -> u8 { 0".to_owned(),
            )),
            "identifier",
            "a field access",
        ),
        (
            "a path tail after a struct head",
            flip(Expr::Path(strings(&["Cell2", "a b"]))),
            "identifier",
            "a path segment",
        ),
        (
            "a path tail after an integer head",
            flip(Expr::Path(strings(&["u8", "MAX; fn"]))),
            "identifier",
            "a path segment",
        ),
        (
            "a path tail after an enum variant",
            flip(Expr::Path(strings(&["Shape", "Dot", "x y"]))),
            "identifier",
            "a path segment",
        ),
        (
            "a path tail after an imported head",
            flip(Expr::Path(strings(&["Rc", "a b"]))),
            "identifier",
            "a path segment",
        ),
        (
            "a field name of a struct pattern",
            flip(match_arm(Pat::Struct(
                strings(&["Rc"]),
                strings(&["a b"]),
                vec![Pat::Wild],
            ))),
            "identifier",
            "a pattern field name",
        ),
        // derives, one site each
        (
            "a derive of a unit struct",
            unit_item(Item::UnitStruct(
                "S".to_owned(),
                strings(&["Clone", "Frob"]),
            )),
            "derive",
            "a unit struct",
        ),
        (
            "a derive of a tuple struct",
            unit_item(Item::TupleStruct(
                "S".to_owned(),
                vec![],
                strings(&["Frob"]),
                vec![],
            )),
            "derive",
            "a tuple struct",
        ),
        (
            "a derive of a record struct",
            unit_item(Item::RecordStruct(
                "S".to_owned(),
                vec![],
                strings(&["Frob"]),
                vec![],
                vec![],
            )),
            "derive",
            "a record struct",
        ),
        (
            "a derive of an enum",
            unit_item(Item::Enum(
                "S".to_owned(),
                vec![],
                strings(&["Frob"]),
                vec![],
            )),
            "derive",
            "an enum",
        ),
        // character literals
        (
            "a character literal of two characters",
            flip(Expr::Lit(Lit::Char("ab".to_owned()))),
            "char-literal",
            "2 scalar values",
        ),
        (
            "a character literal of none",
            flip(Expr::Lit(Lit::Char(String::new()))),
            "char-literal",
            "0 scalar values",
        ),
        (
            "a character literal in a pattern",
            flip(match_arm(Pat::Lit(Lit::Char("ab".to_owned())))),
            "char-literal",
            "2 scalar values",
        ),
        // a struct literal in the head of a condition, a loop, or a match
        (
            "a struct literal as the condition of an if",
            flip(Expr::If(
                Box::new(literal.clone()),
                empty_block(),
                empty_block(),
            )),
            "struct-head",
            "outside parentheses",
        ),
        (
            "a struct literal as the condition of a while",
            flip(Expr::While(Box::new(literal.clone()), empty_block())),
            "struct-head",
            "outside parentheses",
        ),
        (
            "a struct literal as the value of a while let",
            flip(Expr::WhileLet(
                Pat::Wild,
                Box::new(literal.clone()),
                empty_block(),
            )),
            "struct-head",
            "outside parentheses",
        ),
        (
            "a struct literal as the iterator of a for",
            flip(Expr::For(
                Pat::Wild,
                Box::new(literal.clone()),
                empty_block(),
            )),
            "struct-head",
            "outside parentheses",
        ),
        (
            "a struct literal as the scrutinee of a match",
            flip(Expr::Match(
                Box::new(literal.clone()),
                vec![rt::Arm {
                    pattern: Pat::Wild,
                    guard: None,
                    body: unit.clone(),
                }],
            )),
            "struct-head",
            "outside parentheses",
        ),
        (
            "a struct literal at the left of a comparison in a condition",
            flip(Expr::If(
                Box::new(Expr::Equals(
                    Box::new(literal.clone()),
                    Box::new(unit.clone()),
                )),
                empty_block(),
                empty_block(),
            )),
            "struct-head",
            "outside parentheses",
        ),
        (
            "a struct literal behind a field in a condition",
            flip(Expr::If(
                Box::new(Expr::Field(Box::new(literal.clone()), 0)),
                empty_block(),
                empty_block(),
            )),
            "struct-head",
            "outside parentheses",
        ),
        // a generic parameter in a type the printer writes without the generics
        (
            "a generic parameter in a let type",
            generic_body(Expr::Let(
                Pat::Wild,
                Some(Type::Param(0)),
                Box::new(unit.clone()),
            )),
            "type-scope",
            "a `let` type",
        ),
        (
            "a generic parameter in a cast",
            generic_body(Expr::Cast(Box::new(Expr::Lit(Lit::Nat(0))), Type::Param(0))),
            "type-scope",
            "a cast type",
        ),
        (
            "a generic parameter in a type argument",
            generic_body(Expr::MethodOf(
                Box::new(Expr::Lit(Lit::Str("1".to_owned()))),
                "parse".to_owned(),
                vec![Type::Ref(Box::new(Type::Param(0)))],
                vec![],
            )),
            "type-scope",
            "a type argument",
        ),
        // each site of a rule is planted, not only the rule
        (
            "a variant declared twice",
            unit_item(Item::Enum(
                "E".to_owned(),
                vec![],
                vec![],
                vec![Variant::Unit("A".to_owned()), Variant::Unit("A".to_owned())],
            )),
            "variant-name",
            "declares `A` twice",
        ),
        (
            "a record struct with unequal names and types",
            unit_item(Item::RecordStruct(
                "R".to_owned(),
                vec![],
                vec![],
                strings(&["a", "b"]),
                vec![Type::Nat],
            )),
            "struct-fields",
            "2 field names and 1 types",
        ),
        (
            "a struct literal of a path that is no record",
            flip(Expr::StructLit(strings(&["Shape", "Dot"]), vec![], vec![])),
            "struct-fields",
            "no record struct or record variant",
        ),
        (
            "a tuple variant matched with the wrong number of patterns",
            flip(match_arm(Pat::TupleStruct(
                strings(&["Shape", "Dot"]),
                vec![],
            ))),
            "pattern-fields",
            "matched with 0 patterns",
        ),
        (
            "a struct pattern with unequal names and patterns",
            flip(match_arm(Pat::Struct(
                strings(&["Shape", "Box2"]),
                strings(&["w", "h"]),
                vec![Pat::Wild],
            ))),
            "pattern-fields",
            "unequal names and patterns",
        ),
    ];
    // The five sites of the refusal of a name that allocates, in the core
    // profile.
    let core = |item: Item| Crate {
        profile: Profile::Core,
        items: vec![item],
    };
    let heap_sites = [
        (
            "a heap type",
            function(
                "f",
                vec![param("a", Type::Named("Vec".to_owned(), vec![Type::Unit]))],
                vec![],
                vec![],
            ),
            "`Vec`",
        ),
        (
            "the string type",
            function(
                "f",
                vec![param("a", Type::Named("String".to_owned(), vec![]))],
                vec![],
                vec![],
            ),
            "`String`",
        ),
        (
            "a call of a heap function",
            function(
                "f",
                vec![],
                vec![],
                vec![Expr::Call(strings(&["Vec", "new"]), vec![])],
            ),
            "`Vec::new`",
        ),
        (
            "a method that allocates",
            function(
                "f",
                vec![param("a", Type::Nat)],
                vec![],
                vec![Expr::Method(
                    var("a"),
                    "push".to_owned(),
                    vec![unit.clone()],
                )],
            ),
            "`push`",
        ),
        (
            "a method with type arguments that allocates",
            function(
                "f",
                vec![param("a", Type::Nat)],
                vec![],
                vec![Expr::MethodOf(
                    var("a"),
                    "collect".to_owned(),
                    vec![Type::Unit],
                    vec![],
                )],
            ),
            "`collect`",
        ),
    ];
    for (what, item, name) in heap_sites {
        let error = rt_validate::validate(&core(item)).expect_err(what);
        assert!(
            error.contains("runtime support: hidden-allocation") && error.contains(name),
            "{what}: {error}"
        );
    }
    sites.retain(|_| true);
    for (what, planted, rule, site) in sites {
        let error = rt_validate::validate(&planted).expect_err(what);
        assert!(
            error.contains(&format!("runtime support: {rule}")) && error.contains(site),
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

    // The printer writes a comparison whose left operand ends in a cast with
    // parentheses, whatever the operators between: `x + y as u8 < z` is read
    // by `rustc` as the opening of the arguments of a type. The printed crate
    // compiles and computes, and the same text without the parentheses is
    // refused.
    let compare = tiny(vec![Item::Fn(Function {
        sig: Sig {
            name: "less".to_owned(),
            receiver: false,
            exclusive: false,
            params: vec![
                param("x", Type::Fixed(IntKind::U8)),
                param("y", Type::Nat),
                param("z", Type::Fixed(IntKind::U8)),
            ],
            result: Type::Bool,
        },
        generics: vec![],
        public: true,
        body: Block {
            statements: vec![],
            tail: Expr::Binary(
                Op::Lt,
                Box::new(Expr::Binary(
                    Op::Add,
                    var("x"),
                    Box::new(Expr::Cast(var("y"), Type::Fixed(IntKind::U8))),
                )),
                var("z"),
            ),
        },
    })]);
    rt_validate::validate(&compare).expect("the comparison is valid");
    let printed = rt::print(&compare);
    assert!(
        printed.contains("(x + y as u8) < z"),
        "the cast is closed by a parenthesis: {printed}"
    );
    let harness = "use runtime_probe as p;\nfn main() {\n    assert!(p::less(1, 2, 9));\n    assert!(!p::less(1, 2, 3));\n    println!(\"less\");\n}\n";
    assert_eq!(
        compile_with(&printed, harness).expect("the printed comparison compiles"),
        "less"
    );
    let unclosed = printed.replace("(x + y as u8) < z", "x + y as u8 < z");
    assert_ne!(unclosed, printed, "the plant site exists");
    let error = compile_with(&unclosed, harness).expect_err("an unclosed cast is refused");
    assert!(error.contains("generic arguments"), "{error}");

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
