//! The core runtime's support items as source (SPEC.md §17.16): `RB-17`.
//!
//! `RuntimeSupport` defines `supportCrate`, a crate of `RuntimeSyntax`: the
//! uses, the counter, the traits `Same` and `Key` with their impls, and the
//! generic `equal` and `compare`. The Rust crate decodes, validates, and
//! prints it where the handwritten base stood; the handwritten runtime kept
//! at `tests/rust-runtime-oracle/` is the oracle the printed items are
//! compared with, in result and in counted work.

use lexlean::calculus::rust::{rt, rt_decode, rt_validate, runtime, source};

use super::rust_runtime::{differ_with, PRELUDE};
use crate::support::repo_root;

const MODULE: &str = "compiler/src/RuntimeSupport.lex.tex";
const SHIPPED: &str = "language/rust-runtime-1.2/RuntimeSupport.lex.tex";
/// The seeded rounds of nested keys, four comparisons each: two of depth two
/// and two of depth three.
pub const NESTED: usize = 500;
const WIDTHS: [&str; 8] = ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"];

/// The calls the differential makes: `equal` and `compare` at every scalar
/// type, `equal` of orderings, `compare` of pairs of scalars, `tick`, `work`.
fn harness() -> String {
    let mut out = String::from(PRELUDE);
    out.push_str("    use std::cmp::Ordering;\n    let orders = [Ordering::Less, Ordering::Equal, Ordering::Greater];\n");
    out.push_str("    for &a in &[false, true] { for &b in &[false, true] { same!(equal(a, b)); same!(compare(a, b)); } }\n");
    out.push_str("    for &a in &orders { for &b in &orders { same!(equal(a, b)); } }\n");
    for k in WIDTHS {
        out.push_str(&format!(
            "    {{ let v = [{k}::MIN, {k}::MIN.wrapping_add(1), (0 as {k}).wrapping_sub(1), 0, 1, 2, {k}::MAX / 2, {k}::MAX - 1, {k}::MAX];\n      for &a in &v {{ for &b in &v {{ same!(equal(a, b)); same!(compare(a, b)); same!(compare((a, true), (b, false))); same!(compare((false, a), (true, b))); same!(compare((a, b), (b, a))); same!(compare((a, 1u8), (b, 2u8))); }} }}\n      for _ in 0..200 {{ let (a, b) = (next() as {k}, next() as {k}); same!(equal(a, b)); same!(compare(a, b)); same!(compare((a, b), (b, a))); }} }}\n"
        ));
    }
    // Nested keys, which `SP-25` does not prove (each pair level spends
    // fuel of the declared semantics) and this differential validates: pairs
    // of depth two and three whose components come from a domain of three
    // values, so that the first components tie and the comparison falls
    // through to the next level.
    out.push_str(&format!(
        "    let mut tally = [0usize; 3];\n    for _ in 0..{NESTED} {{\n      let t = |n: u64| n % 3;\n      let (a, b, c, d) = (t(next()), t(next()), t(next()), t(next()));\n      let (e, f, g, h) = (t(next()), t(next()), t(next()), t(next()));\n      same!(compare(((a as u8, b as i64), c % 2 == 0), ((e as u8, f as i64), g % 2 == 0)));\n      same!(compare((a as u32, (b as i8, c as u64)), (e as u32, (f as i8, g as u64))));\n      same!(compare((((a as u8, b == 0), (c as i32, d as u64)), (h as i8, a as i16)), (((e as u8, f == 0), (g as i32, h as u64)), (h as i8, a as i16))));\n      same!(compare((((a as u8, b == 0), (c as i32, d as u64)), (h as i8, a as i16)), (((a as u8, b == 0), (c as i32, h as u64)), (e as i8, f as i16))));\n      for o in [new::compare((((a as u8, b == 0), (c as i32, d as u64)), (h as i8, a as i16)), (((e as u8, f == 0), (g as i32, h as u64)), (h as i8, a as i16))), new::compare((((a as u8, b == 0), (c as i32, d as u64)), (h as i8, a as i16)), (((a as u8, b == 0), (c as i32, h as u64)), (e as i8, f as i16)))] {{ tally[(o as i8 + 1) as usize] += 1; }}\n    }}\n    println!(\"nested {{tally:?}}\");\n"
    ));
    out.push_str("    same!(compare((1u8, -1i64), (1u8, 5i64)));\n    same!(compare((1u64, 2u8), (1u64, 2u8)));\n");
    out.push_str("    for &u in &[0u64, 1, 7, 1 << 40, u64::MAX / 2] { same!(tick(u)); same!(work()); }\n    println!(\"runtime\");\n}\n");
    out
}

/// `text` with the `n`th (from 0) occurrence of `from` replaced by `to`.
pub(super) fn replace_nth(text: &str, from: &str, to: &str, n: usize) -> String {
    let at = text
        .match_indices(from)
        .nth(n)
        .map(|(at, _)| at)
        .unwrap_or_else(|| panic!("the plant site `{from}` has no occurrence {n}"));
    format!("{}{to}{}", &text[..at], &text[at + from.len()..])
}

/// Run the case of `RB-17`.
pub fn run(id: &str) {
    assert_eq!(id, "RB-17");
    let root = repo_root();
    let read = |path: &str| {
        std::fs::read_to_string(root.join(path).as_std_path())
            .unwrap_or_else(|error| panic!("{path}: {error}"))
    };
    let module = read(MODULE);
    let shipped = read(SHIPPED);
    assert_eq!(module, shipped, "{SHIPPED} differs from {MODULE}");
    let oracle = read("tests/rust-runtime-oracle/core.rs.txt");
    let runtime_module = read("compiler/src/RustRuntime.lex.tex");

    let krate = rt_decode::decode(&shipped).expect("the module decodes");
    assert!(
        !krate.items.is_empty(),
        "an empty support crate defines nothing to compare"
    );
    rt_validate::validate(&krate).expect("the crate is valid");
    let support = runtime::support_from(&shipped).expect("the module prints");
    assert_eq!(
        runtime::support().expect("the embedded support"),
        support,
        "the embedded support is the committed one"
    );
    assert!(
        !support.contains("macro_rules")
            && !support.contains("alloc")
            && !support.contains("std::"),
        "the support holds no macro and names neither `alloc` nor `std`"
    );
    // The printed items are the oracle's: the runtime `RustRuntime` defines
    // follows them, and the pair agrees with the handwritten core.
    let generated = runtime::core_over(&support, &runtime_module).expect("the runtime assembles");
    let ran = differ_with(&oracle, &generated, harness())
        .expect("the printed support agrees with the oracle");
    assert_eq!(ran.lines().last(), Some("runtime"));
    // The nested-key corpus is measured: how many of its comparisons the
    // depth-three keys decide as less, equal, and greater.
    let nested = ran
        .lines()
        .find(|line| line.starts_with("nested"))
        .expect("the tally");
    eprintln!(
        "RB-17: {nested} over {} depth-three comparisons",
        2 * NESTED
    );
    let slots: Vec<usize> = nested
        .trim_start_matches("nested [")
        .trim_end_matches(']')
        .split(", ")
        .map(|slot| slot.parse().expect("a count"))
        .collect();
    assert_eq!(slots.len(), 3, "less, equal, greater: {nested}");
    assert!(
        slots.iter().all(|&slot| slot > 0) && slots.iter().sum::<usize>() == 2 * NESTED,
        "the corpus decides every order and every comparison: {nested}"
    );

    // A changed body is caught by the differential, not by the validator.
    for (what, from, to, nth, caught) in [
        (
            "the counter starts at one",
            "(ctor RuntimeSyntax::RtExpr.lit (ctor RuntimeSyntax::RtLit.nat 0))",
            "(ctor RuntimeSyntax::RtExpr.lit (ctor RuntimeSyntax::RtLit.nat 1))",
            0,
            "work result",
        ),
        (
            "compare keys the right value against the left",
            "(ctor RuntimeSyntax::RtExpr.method (ctor RuntimeSyntax::RtExpr.var \"a\") \"key\" (cons (ctor RuntimeSyntax::RtExpr.borrow (ctor RuntimeSyntax::RtExpr.var \"b\"))",
            "(ctor RuntimeSyntax::RtExpr.method (ctor RuntimeSyntax::RtExpr.var \"b\") \"key\" (cons (ctor RuntimeSyntax::RtExpr.borrow (ctor RuntimeSyntax::RtExpr.var \"a\"))",
            0,
            "compare result",
        ),
        (
            "the key of a scalar is reversed",
            "(ctor RuntimeSyntax::RtExpr.method (ctor RuntimeSyntax::RtExpr.var \"self\") \"cmp\" (cons (ctor RuntimeSyntax::RtExpr.var \"other\")",
            "(ctor RuntimeSyntax::RtExpr.method (ctor RuntimeSyntax::RtExpr.var \"other\") \"cmp\" (cons (ctor RuntimeSyntax::RtExpr.var \"self\")",
            0,
            "compare result",
        ),
    ] {
        let changed = replace_nth(&shipped, from, to, nth);
        let printed = runtime::support_from(&changed).unwrap_or_else(|error| panic!("{what}: {error}"));
        let generated = runtime::core_over(&printed, &runtime_module).expect("assembles");
        let error = differ_with(&oracle, &generated, harness()).expect_err(what);
        assert!(error.replace([' ', '\n'], "").contains(&caught.replace(' ', "")), "{what}: {error}");
    }

    // A crate outside the closed set or its tables is refused by the rule it
    // breaks, named.
    let mut planted: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for (what, from, to, nth, rule) in text_plants() {
        let changed = replace_nth(&shipped, from, to, nth);
        let error = runtime::support_from(&changed).expect_err(what);
        assert!(
            error.contains(&format!("runtime support: {rule}")),
            "{what}: {error}"
        );
        planted.insert(rule);
    }

    // The nine rules the text of the module cannot reach without a second
    // change are planted on the decoded crate.
    let decoded = rt_decode::decode(&shipped).expect("the module decodes");
    for (what, mutate, rule) in ast_plants() {
        let mut crate_ = decoded.clone();
        mutate(&mut crate_);
        let error = rt_validate::validate(&crate_).expect_err(what);
        assert!(
            error.contains(&format!("runtime support: {rule}")),
            "{what}: {error}"
        );
        planted.insert(rule);
    }
    // No rule of the validator is without a plant: the rules are read from
    // its source, so a rule added there is a failure here until it is planted.
    let source = read("crates/lexlean/src/calculus/rust/rt_validate.rs");
    let named = rules_named(&source);
    // The rules of the constructs `RB-19` grew the set by are planted there.
    planted.extend(super::rust_heap_syntax::RULES);
    let missing: Vec<&String> = named
        .iter()
        .filter(|name| !planted.contains(name.as_str()))
        .collect();
    assert!(missing.is_empty(), "rules without a plant: {missing:?}");
    assert_eq!(
        named.len(),
        planted.len(),
        "a plant names a rule the validator lacks"
    );

    // A module the decoder refuses, each for the rule it breaks.
    for (what, from, to, message) in [
        ("no support crate", "name: supportCrate", "name: otherCrate", "does not define `supportCrate`"),
        (
            "an expression constructor RuntimeSyntax lacks",
            "RtExpr.matchOn",
            "RtExpr.matchOff",
            "is not an expression",
        ),
        (
            "an item constructor RuntimeSyntax lacks",
            "RtItem.traitItem",
            "RtItem.traitItems",
            "is not an item",
        ),
        (
            "a constructor with another number of arguments",
            "(ctor RuntimeSyntax::RtExpr.borrow (ctor RuntimeSyntax::RtExpr.var \"b\"))",
            "(ctor RuntimeSyntax::RtExpr.borrow (ctor RuntimeSyntax::RtExpr.var \"b\") (nil RuntimeSyntax::RtExpr))",
            "takes 1 arguments",
        ),
        (
            "a constructor of the other module",
            "RuntimeSyntax::RtProfile.core",
            "RustSyntax::Profile.core",
            "is not a constructor of `RuntimeSyntax::RtProfile`",
        ),
    ] {
        let edited = shipped.replacen(from, to, 1);
        assert_ne!(edited, shipped, "{what}: the plant site exists");
        let error = rt_decode::decode(&edited).expect_err(what);
        assert!(error.contains(message), "{what}: {error}");
    }
    // `i128` is runtime-only: a crate that names it in a parameter, as
    // `convert` does, is accepted by the validator and refused by the
    // correspondence between a rendering and its program, which has no row for
    // `type:i128`. A row added for it fails here.
    {
        use lexlean::calculus::rust::ast::{
            Block as RustBlock, Crate as RustCrate, Expr as RustExpr, Ident, ItemDef,
            Lit as RustLit, Origin, Pat as RustPat, Type as RustType,
        };
        let wide = RustCrate {
            profile: lexlean::calculus::rust::Profile::Std,
            items: vec![ItemDef::Function {
                name: Ident::Function(0),
                parameters: vec![(RustPat::Bind(Ident::Local(0)), RustType::I128)],
                result: RustType::Overflow,
                body: RustBlock::of(RustExpr::Lit(RustLit::Overflow, Origin::of("function"))),
                at: Origin::of("function"),
            }],
        };
        let elements: std::collections::BTreeSet<String> = ["function".to_owned()].into();
        let error = lexlean::calculus::rust::validate::correspond(&wide, &elements)
            .expect_err("a lowered crate cannot name i128");
        assert!(
            error.contains("type:i128") && error.contains("no target-semantics correspondence"),
            "{error}"
        );
    }
    // The first module's reader still reads its own.
    source::decode(&runtime_module).expect("RustRuntime still decodes");
}

/// The refusals the text of the shipped module reaches: the description, the
/// text replaced, its replacement, which occurrence, and the rule that refuses.
pub(super) fn text_plants() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    usize,
    &'static str,
)> {
    let sig_same = "{name := \"same\", receiver := true";
    let first_use = "(cons \"core\" (cons \"cmp\"";
    vec![
        ("a use outside the profile", first_use, "(cons \"alloc\" (cons \"cmp\"", 0, "profile-root"),
        ("a std use in the core profile", first_use, "(cons \"std\" (cons \"cmp\"", 0, "profile-root"),
        (
            "a type that is neither declared nor in the table",
            "RtType.named \"AtomicU64\"",
            "RtType.named \"Vecc\"",
            0,
            "type-name",
        ),
        (
            "a heap type in the core profile",
            "RtType.named \"AtomicU64\"",
            "RtType.named \"Vec\"",
            0,
            "hidden-allocation",
        ),
        (
            "a type with another number of arguments",
            "RtType.named \"Overflow\" (nil RuntimeSyntax::RtType)",
            "RtType.named \"Overflow\" (cons (ctor RuntimeSyntax::RtType.bool) (nil RuntimeSyntax::RtType))",
            0,
            "type-arity",
        ),
        (
            "a bound on a trait that is not declared",
            "{name := \"T\", bounds := (cons \"Same\"",
            "{name := \"T\", bounds := (cons \"Sane\"",
            0,
            "bound-trait",
        ),
        (
            "a method the bound does not provide",
            "{name := \"T\", bounds := (cons \"Same\"",
            "{name := \"T\", bounds := (cons \"Key\"",
            0,
            "method-bound",
        ),
        (
            "an impl that lacks a method of its trait",
            sig_same,
            "{name := \"sane\", receiver := true",
            1,
            "impl-complete",
        ),
        (
            "an impl that states another signature",
            "{name := \"other\", mutable := false, type := (ctor RuntimeSyntax::RtType.ref (ctor RuntimeSyntax::RtType.selfType))}) (nil RuntimeSyntax::RtParam)), result := (ctor RuntimeSyntax::RtType.bool)}), generics := (nil RuntimeSyntax::RtGeneric), public := false",
            "{name := \"other\", mutable := false, type := (ctor RuntimeSyntax::RtType.ref (ctor RuntimeSyntax::RtType.selfType))}) (nil RuntimeSyntax::RtParam)), result := (ctor RuntimeSyntax::RtType.ordering)}), generics := (nil RuntimeSyntax::RtGeneric), public := false",
            0,
            "impl-signature",
        ),
        (
            "an impl of a trait that is not declared",
            "(ctor Option.some [String] \"Same\")",
            "(ctor Option.some [String] \"Sane\")",
            0,
            "impl-trait",
        ),
        (
            "a trait method made public in its impl",
            "generics := (nil RuntimeSyntax::RtGeneric), public := false",
            "generics := (nil RuntimeSyntax::RtGeneric), public := true",
            0,
            "impl-visibility",
        ),
        (
            "a free function that is not public",
            "generics := (nil RuntimeSyntax::RtGeneric), public := true",
            "generics := (nil RuntimeSyntax::RtGeneric), public := false",
            0,
            "function-visibility",
        ),
        (
            "a variable that is not in scope",
            "(ctor RuntimeSyntax::RtExpr.var \"units\")",
            "(ctor RuntimeSyntax::RtExpr.var \"unitz\")",
            0,
            "variable-scope",
        ),
        (
            "a method of no trait and not in the table",
            "\"fetch_add\"",
            "\"fetch_sub\"",
            0,
            "method-name",
        ),
        (
            "a path no use declares",
            "(cons \"Memory\" (cons \"Relaxed\"",
            "(cons \"Memori\" (cons \"Relaxed\"",
            0,
            "path-name",
        ),
        (
            "a call of a path outside the table",
            "(ctor RuntimeSyntax::RtExpr.call (cons \"AtomicU64\" (cons \"new\"",
            "(ctor RuntimeSyntax::RtExpr.call (cons \"AtomicU64\" (cons \"neu\"",
            0,
            "call-name",
        ),
        (
            "a type parameter beyond the generics in scope",
            "RtType.param 1",
            "RtType.param 5",
            0,
            "type-parameter",
        ),
    ]
}

pub(super) type Mutation = fn(&mut rt::Crate);

/// The rules planted on the decoded crate: the construct that breaks each,
/// added to or changed in the crate the module defines.
pub(super) fn ast_plants() -> Vec<(&'static str, Mutation, &'static str)> {
    use rt::{Expr, Item, Type};
    fn function<'a>(krate: &'a mut rt::Crate, name: &str) -> &'a mut rt::Function {
        krate
            .items
            .iter_mut()
            .find_map(|item| match item {
                Item::Fn(function) if function.sig.name == name => Some(function),
                _ => None,
            })
            .expect("the function")
    }
    vec![
        (
            "a call with another number of arguments",
            |krate| {
                function(krate, "tick").body.statements.insert(
                    0,
                    Expr::Call(vec!["AtomicU64".to_owned(), "new".to_owned()], vec![]),
                );
            },
            "call-arity",
        ),
        (
            "a name declared twice",
            |krate| {
                let again = krate
                    .items
                    .iter()
                    .find(|item| matches!(item, Item::Fn(function) if function.sig.name == "work"))
                    .cloned()
                    .expect("work");
                krate.items.push(again);
            },
            "duplicate-name",
        ),
        (
            "a free function that takes `&self`",
            |krate| function(krate, "work").sig.receiver = true,
            "function-receiver",
        ),
        (
            "an impl that states a method its trait does not",
            |krate| {
                for item in &mut krate.items {
                    if let Item::Impl(_, Some(tr), _, functions) = item {
                        if tr == "Same" {
                            let mut extra = functions[0].clone();
                            extra.sig.name = "extra".to_owned();
                            functions.push(extra);
                            return;
                        }
                    }
                }
            },
            "impl-extra",
        ),
        (
            "a match with no arm",
            |krate| {
                for item in &mut krate.items {
                    if let Item::Impl(_, Some(tr), _, functions) = item {
                        if tr == "Key" {
                            if let Expr::Match(_, arms) =
                                &mut functions.last_mut().expect("a method").body.tail
                            {
                                arms.clear();
                                return;
                            }
                        }
                    }
                }
            },
            "match-arms",
        ),
        (
            "a method with another number of arguments",
            |krate| {
                function(krate, "tick").body.statements.insert(
                    0,
                    Expr::Method(
                        Box::new(Expr::Var("units".to_owned())),
                        "cmp".to_owned(),
                        vec![],
                    ),
                );
            },
            "method-arity",
        ),
        (
            "`Self` outside a trait or an impl",
            |krate| function(krate, "tick").sig.params[0].ty = Type::SelfType,
            "self-type",
        ),
        (
            "a trait that declares a method twice",
            |krate| {
                for item in &mut krate.items {
                    if let Item::Trait(_, sigs) = item {
                        let again = sigs[0].clone();
                        sigs.push(again);
                        return;
                    }
                }
            },
            "trait-method",
        ),
        (
            "a use with no path",
            |krate| krate.items.insert(0, Item::Use(vec![], None)),
            "use-path",
        ),
    ]
}

/// The names of the rules a validator source refuses by: the first string
/// literal after each `rule(`.
pub(super) fn rules_named(source: &str) -> std::collections::BTreeSet<String> {
    let mut out = std::collections::BTreeSet::new();
    let mut rest = source;
    while let Some(at) = rest.find("rule(") {
        let after = rest[at + 5..].trim_start();
        if let Some(quoted) = after.strip_prefix('"') {
            if let Some(end) = quoted.find('"') {
                out.insert(quoted[..end].to_owned());
            }
        }
        rest = &rest[at + 5..];
    }
    out.remove("");
    out
}
