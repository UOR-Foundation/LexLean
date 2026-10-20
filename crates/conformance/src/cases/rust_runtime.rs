//! The core runtime as source (SPEC.md §17.16): `RB-16`.
//!
//! The functions of `RustRuntime` are decoded and printed in place of the
//! handwritten functions of the same name, which are kept as the oracle at
//! `tests/rust-runtime-oracle/`. One crate holds the oracle as module `old`
//! and the generated runtime as module `new`; a harness calls both on the
//! boundary and seeded values of each defined function and compares the
//! result and the counted work.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::rust::ast::{Ident, ItemDef};
use lexlean::calculus::rust::{runtime, source};
use lexlean::calculus::IntKind;

use crate::rust_build::{cargo_in, workspace, Built};
use crate::support::repo_root;

const MODULE: &str = "compiler/src/RustRuntime.lex.tex";
const SHIPPED: &str = "language/rust-runtime-1.2/RustRuntime.lex.tex";

/// The calls the differential makes of each defined function: the `same!`
/// of the harness runs the oracle's and the generated function on the same
/// arguments and compares what they return and the work they count.
const SCALARS: [(&str, &str); 18] = [
    ("nat_add",
        "for &a in &nats { for &b in &nats { same!(nat_add(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_add(a, b)); same!(nat_add(a >> 1, b >> 1)); }"),
    ("nat_sub",
        "for &a in &nats { for &b in &nats { same!(nat_sub(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_sub(a, b)); same!(nat_sub(a >> 1, b >> 1)); }"),
    ("nat_mul",
        "for &a in &nats { for &b in &nats { same!(nat_mul(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_mul(a, b)); same!(nat_mul(a >> 1, b >> 1)); }"),
    ("nat_quot",
        "for &a in &nats { for &b in &nats { for &z in &[0u64, 7, u64::MAX] { same!(nat_quot(a, b, z)); } } }\n    for _ in 0..1000 { let (a, b, z) = (next(), next(), next()); same!(nat_quot(a, b, z)); same!(nat_quot(a >> 40, b >> 50, z)); }"),
    ("nat_rem",
        "for &a in &nats { for &b in &nats { for &z in &[0u64, 7, u64::MAX] { same!(nat_rem(a, b, z)); } } }\n    for _ in 0..1000 { let (a, b, z) = (next(), next(), next()); same!(nat_rem(a, b, z)); same!(nat_rem(a >> 40, b >> 50, z)); }"),
    ("nat_eq",
        "for &a in &nats { for &b in &nats { same!(nat_eq(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_eq(a, b)); same!(nat_eq(a >> 1, b >> 1)); }"),
    ("nat_le",
        "for &a in &nats { for &b in &nats { same!(nat_le(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_le(a, b)); same!(nat_le(a >> 1, b >> 1)); }"),
    ("nat_lt",
        "for &a in &nats { for &b in &nats { same!(nat_lt(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next(), next()); same!(nat_lt(a, b)); same!(nat_lt(a >> 1, b >> 1)); }"),
    ("nat_succ",
        "for &a in &nats { same!(nat_succ(a)); }\n    for _ in 0..1000 { let a = next(); same!(nat_succ(a)); }"),
    ("int_add",
        "for &a in &ints { for &b in &ints { same!(int_add(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next() as i64, next() as i64); same!(int_add(a, b)); same!(int_add(a >> 1, b >> 1)); }"),
    ("int_sub",
        "for &a in &ints { for &b in &ints { same!(int_sub(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next() as i64, next() as i64); same!(int_sub(a, b)); same!(int_sub(a >> 1, b >> 1)); }"),
    ("int_mul",
        "for &a in &ints { for &b in &ints { same!(int_mul(a, b)); } }\n    for _ in 0..1000 { let (a, b) = (next() as i64, next() as i64); same!(int_mul(a, b)); same!(int_mul(a >> 1, b >> 1)); }"),
    ("int_neg",
        "for &a in &ints { same!(int_neg(a)); }\n    for _ in 0..1000 { let a = next() as i64; same!(int_neg(a)); }"),
    ("int_quot",
        "for &a in &ints { for &b in &ints { for &z in &[i64::MIN, -1, 0, 7, i64::MAX] { same!(int_quot(a, b, z)); } } }\n    for _ in 0..1000 { let (a, b, z) = (next() as i64, next() as i64, next() as i64); same!(int_quot(a, b, z)); same!(int_quot(a >> 40, b >> 50, z)); }"),
    ("int_rem",
        "for &a in &ints { for &b in &ints { for &z in &[i64::MIN, -1, 0, 7, i64::MAX] { same!(int_rem(a, b, z)); } } }\n    for _ in 0..1000 { let (a, b, z) = (next() as i64, next() as i64, next() as i64); same!(int_rem(a, b, z)); same!(int_rem(a >> 40, b >> 50, z)); }"),
    (
        "bool_not",
        "for &a in &[false, true] { same!(bool_not(a)); }",
    ),
    (
        "bool_and",
        "for &a in &[false, true] { for &b in &[false, true] { same!(bool_and(a, b)); } }",
    ),
    (
        "bool_or",
        "for &a in &[false, true] { for &b in &[false, true] { same!(bool_or(a, b)); } }",
    ),
];

/// The differential over every function: the scalar functions, and the
/// functions of each width on its boundary values and 200 seeded pairs.
fn differential() -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = SCALARS
        .iter()
        .map(|(name, calls)| ((*name).to_owned(), (*calls).to_owned()))
        .collect();
    for kind in IntKind::ALL {
        let k = kind.name();
        let values = format!(
            "let v = [{k}::MIN, {k}::MIN.wrapping_add(1), (0 as {k}).wrapping_sub(1), 0, 1, 2, {k}::MAX / 2, {k}::MAX - 1, {k}::MAX];"
        );
        for name in [
            "checked_add",
            "checked_sub",
            "checked_mul",
            "checked_quot",
            "bit_and",
            "bit_or",
            "bit_xor",
        ] {
            let path = format!("fixed_{k}::{name}");
            out.push((
                path.clone(),
                format!(
                    "{values}\n    for &a in &v {{ for &b in &v {{ same!({path}(a, b)); }} }}\n    for _ in 0..200 {{ let (a, b) = (next() as {k}, next() as {k}); same!({path}(a, b)); }}"
                ),
            ));
        }
        let path = format!("fixed_{k}::bit_not");
        out.push((
            path.clone(),
            format!("{values}\n    for &a in &v {{ same!({path}(a)); }}"),
        ));
        for name in ["shift_left", "shift_right"] {
            let path = format!("fixed_{k}::{name}");
            out.push((
                path.clone(),
                format!(
                    "{values}\n    for &a in &v {{ for &s in &[0u32, 1, {k}::BITS - 1, {k}::BITS, {k}::BITS + 1, 200] {{ same!({path}(a, s)); }} }}"
                ),
            ));
        }
        let path = format!("fixed_{k}::convert");
        out.push((
            path.clone(),
            format!(
                "for &w in &[i128::MIN, {k}::MIN as i128 - 1, {k}::MIN as i128, -1i128, 0, 1, {k}::MAX as i128, {k}::MAX as i128 + 1, i128::MAX] {{ same!({path}(w)); }}\n    for _ in 0..200 {{ let w = (next() as i128) << (next() % 64); same!({path}(w)); }}"
            ),
        ));
        if kind.signed() {
            let path = format!("checked_neg_{k}");
            out.push((
                path.clone(),
                format!("{values}\n    for &a in &v {{ same!({path}(a)); }}"),
            ));
        }
    }
    out
}

/// The names of the functions a decoded module defines.
fn defined(items: &[ItemDef]) -> Result<BTreeSet<String>, String> {
    items
        .iter()
        .map(|item| match item {
            ItemDef::Function {
                name: Ident::Export(name),
                ..
            } => Ok(name.clone()),
            _ => Err(
                "the core runtime module defines an item that is not a named function".to_owned(),
            ),
        })
        .collect()
}

/// Whether the differential covers exactly the functions the module defines.
fn covered(items: &[ItemDef]) -> Result<(), String> {
    let defined = defined(items)?;
    let differential: BTreeSet<String> = differential().into_iter().map(|(name, _)| name).collect();
    if defined != differential {
        return Err(format!(
            "the module defines {defined:?}, the differential covers {differential:?}"
        ));
    }
    Ok(())
}

/// The harness: both runtimes, every function of the differential.
fn harness() -> String {
    let mut out = String::from(
        "use runtime_diff::{new, old};\nmacro_rules! same {\n    ($($f:ident)::+($($a:expr),*)) => {{\n        let (w0, w1) = (old::work(), new::work());\n        let x = old::$($f)::+($($a),*);\n        let y = new::$($f)::+($($a),*);\n        assert_eq!(format!(\"{x:?}\"), format!(\"{y:?}\"), \"{} result\", stringify!($($f)::+));\n        assert_eq!(old::work() - w0, new::work() - w1, \"{} work\", stringify!($($f)::+));\n    }};\n}\nfn main() {\n    let nats = [0u64, 1, 2, 3, 7, u32::MAX as u64, u32::MAX as u64 + 1, u64::MAX / 2, u64::MAX - 2, u64::MAX - 1, u64::MAX];\n    let ints = [i64::MIN, i64::MIN + 1, -3037000500, -2, -1, 0, 1, 2, 3037000500, i64::MAX - 1, i64::MAX];\n    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;\n    let mut next = move || { seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17; seed };\n",
    );
    for (_, calls) in differential() {
        out.push_str("    {\n    ");
        out.push_str(&calls);
        out.push_str("\n    }\n");
    }
    out.push_str("    println!(\"runtime\");\n}\n");
    out
}

/// Compile the oracle and `generated` as modules of one crate and run the
/// differential: the output of the harness, or the reason it failed.
fn differ(oracle: &str, generated: &str) -> Result<String, String> {
    let directory = tempfile::tempdir().expect("a directory");
    let lib = format!("pub mod old {{\n{oracle}\n}}\npub mod new {{\n{generated}\n}}\n");
    let files: BTreeMap<String, Vec<u8>> = BTreeMap::from([
        (
            "Cargo.toml".to_owned(),
            b"[package]\nname = \"runtime_diff\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[lints.rust]\nwarnings = \"deny\"\n".to_vec(),
        ),
        ("src/lib.rs".to_owned(), lib.into_bytes()),
    ]);
    workspace(
        directory.path(),
        &[Built {
            name: "runtime_diff".to_owned(),
            files,
            harness: harness(),
        }],
    );
    let output = cargo_in(
        directory.path(),
        &["run", "--quiet", "--offline", "-p", "run_runtime_diff"],
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

/// Run the case of `RB-16`.
pub fn run(id: &str) {
    assert_eq!(id, "RB-16");
    let root = repo_root();
    let read = |path: &str| {
        std::fs::read_to_string(root.join(path).as_std_path())
            .unwrap_or_else(|error| panic!("{path}: {error}"))
    };
    let module = read(MODULE);
    let shipped = read(SHIPPED);
    // The shipped copy is the compiler project's module byte for byte: the
    // proofs are about one, the Rust crate reads the other.
    assert_eq!(module, shipped, "{SHIPPED} differs from {MODULE}");
    let oracle = read("tests/rust-runtime-oracle/core.rs.txt");
    // The handwritten text of the `rust-std` runtime is kept beside it and is
    // still the text the compiler prints: no function of it is defined by the
    // module yet, so the oracle is that text exactly.
    assert_eq!(
        read("tests/rust-runtime-oracle/std.rs.txt"),
        runtime::STD,
        "the std oracle is the std runtime"
    );
    let decoded = source::decode(&shipped).expect("the module decodes");
    assert!(
        !decoded.core.items.is_empty(),
        "an empty runtime module defines nothing the differential could compare"
    );
    covered(&decoded.core.items).expect("the differential covers the module");
    assert_eq!(
        runtime::core().expect("the embedded module"),
        runtime::assemble(&decoded.core.items),
        "the embedded module is the committed one"
    );
    // The generated functions replace the oracle's: the base holds none of
    // their names, so no function is declared twice.
    for name in defined(&decoded.core.items).expect("named functions") {
        let (module, last) = name
            .rsplit_once("::")
            .map_or((None, name.as_str()), |(m, l)| (Some(m), l));
        if let Some(module) = module {
            assert!(
                !runtime::CORE_BASE.contains(&format!("pub mod {module} {{")),
                "the base still holds the module `{module}`"
            );
            assert!(
                oracle.contains(&format!("fixed!({module}, ")),
                "the oracle lacks the module `{module}`"
            );
        } else {
            assert!(
                !runtime::CORE_BASE.contains(&format!("pub fn {last}(")),
                "the base still holds `{name}`"
            );
            assert!(
                oracle.contains(&format!("pub fn {last}(")),
                "the oracle lacks `{name}`"
            );
        }
    }
    let generated = runtime::assemble(&decoded.core.items);
    let ran = differ(&oracle, &generated).expect("the generated runtime agrees with the oracle");
    assert_eq!(ran, "runtime");

    // A function whose body computes another result is caught by the
    // differential, not by the decoder: the checked product for the checked
    // sum, the checked difference for the checked sum of integers, the strict
    // order for the loose one, the remainder for the quotient.
    for (from, to, caught) in [
        (
            "Machine.checkedNatAdd",
            "Machine.checkedNatMul",
            "nat_add result",
        ),
        (
            "Machine.checkedIntAdd",
            "Machine.checkedIntSub",
            "int_add result",
        ),
        ("Machine.natLe", "Machine.natLt", "nat_le result"),
        ("Machine.natQuot", "Machine.natRem", "nat_quot result"),
        (
            "Machine.checkedAdd (ctor TargetSyntax::IntKind.u8)",
            "Machine.checkedSub (ctor TargetSyntax::IntKind.u8)",
            "fixed_u8::checked_addresult",
        ),
        (
            "Machine.checkedShl (ctor TargetSyntax::IntKind.i32)",
            "Machine.checkedShr (ctor TargetSyntax::IntKind.i32)",
            "fixed_i32::shift_leftresult",
        ),
        (
            "Machine.bitXor (ctor TargetSyntax::IntKind.u16)",
            "Machine.bitOr (ctor TargetSyntax::IntKind.u16)",
            "fixed_u16::bit_xorresult",
        ),
    ] {
        let changed = shipped.replacen(from, to, 1);
        assert_ne!(changed, shipped, "the plant site exists");
        let items = source::decode(&changed)
            .expect("a changed body decodes")
            .core
            .items;
        let error = differ(&oracle, &runtime::assemble(&items))
            .expect_err("a changed body disagrees with the oracle");
        assert!(
            error
                .replace([' ', '\n'], "")
                .contains(&caught.replace(' ', "")),
            "{error}"
        );
    }

    // The runtime crate is checked by the validator that checks every
    // rendering, with the runtime's reading of its arithmetic rule: its
    // functions fail only with `Err(Overflow)`, and a core function
    // allocates nothing.
    lexlean::calculus::rust::validate::validate(&decoded.core).expect("the runtime is valid");
    for (what, from, to, message) in [
        (
            "an Err of a value that is not the overflow value",
            "(ctor RustSyntax::Expr.lit (ctor RustSyntax::Lit.overflow))",
            "(ctor RustSyntax::Expr.lit (ctor RustSyntax::Lit.nat 0))",
            "is not a fallible value",
        ),
        (
            "an Err whose error type is not the runtime's",
            "(ctor RustSyntax::RustType.nat) (ctor RustSyntax::RustType.overflow)",
            "(ctor RustSyntax::RustType.nat) (ctor RustSyntax::RustType.nat)",
            "is not a fallible value",
        ),
        (
            "a heap type in the core runtime",
            "type := (ctor RustSyntax::RustType.nat)",
            "type := (ctor RustSyntax::RustType.str)",
            "hidden allocation",
        ),
    ] {
        let edited = shipped.replacen(from, to, 1);
        assert_ne!(edited, shipped, "{what}: the plant site exists");
        let krate = source::decode(&edited).expect(what).core;
        let error = lexlean::calculus::rust::validate::validate(&krate).expect_err(what);
        assert!(error.contains(message), "{what}: {error}");
    }

    // A module the decoder refuses, each for the rule it breaks.
    for (what, from, to, message) in [
        (
            "no core crate",
            "name: coreCrate",
            "name: otherCrate",
            "does not define `coreCrate`",
        ),
        (
            "a definition with a parameter",
            "name: natAddFunction,\n  parameters: []",
            "name: natAddFunction,\n  parameters: [extra : Nat]",
            "takes parameters",
        ),
        (
            "a reference to a definition the module lacks",
            "(cons (natAddFunction) ",
            "(cons (natAddFunctions) ",
            "which the module does not define",
        ),
        (
            "a machine primitive the machine lacks",
            "Machine.checkedNatAdd",
            "Machine.checkedNatAddd",
            "is not a machine primitive",
        ),
        (
            "a runtime function the runtime lacks",
            "ctor RustSyntax::Expr.lit (ctor RustSyntax::Lit.overflow)",
            "ctor RustSyntax::Expr.call (ctor RustSyntax::Callee.runtime (ctor RustSyntax::Item.natAddd)) (nil RustSyntax::Expr) false",
            "is not a runtime function",
        ),
        (
            "a constructor with another number of arguments",
            "(ctor RustSyntax::Expr.succeed (ctor RustSyntax::Expr.copy (ctor RustSyntax::Ident.generated (ctor RustSyntax::IdentKind.binding) 0)))",
            "(ctor RustSyntax::Expr.succeed (ctor RustSyntax::Expr.copy (ctor RustSyntax::Ident.generated (ctor RustSyntax::IdentKind.binding) 0)) (nil RustSyntax::Expr))",
            "takes 1 arguments",
        ),
    ] {
        let edited = shipped.replacen(from, to, 1);
        assert_ne!(edited, shipped, "{what}: the plant site exists");
        let error = source::decode(&edited).expect_err(what);
        assert!(error.contains(message), "{what}: {error}");
    }

    // A module that defines a function the differential does not cover is
    // refused by the case, whatever the function is.
    let start = shipped
        .find("definition{\n  name: natAddFunction,")
        .expect("the function's definition");
    let end = start + shipped[start..].find("\n}\n").expect("its end") + 3;
    let block = &shipped[start..end];
    let copy = block
        .replace("natAddFunction", "natOtherFunction")
        .replace("\"nat_add\"", "\"nat_other\"");
    let at = shipped.find("name: coreCrate").expect("the core crate");
    let (before, crate_text) = shipped.split_at(at);
    let crate_text = crate_text
        .replacen(
            "items := (cons (natAddFunction) ",
            "items := (cons (natOtherFunction) (cons (natAddFunction) ",
            1,
        )
        .replacen("})\n}\n", ")})\n}\n", 1);
    let extra = format!("{before}{crate_text}").replacen(block, &format!("{block}\n{copy}"), 1);
    let items = source::decode(&extra)
        .expect("a module with a second function decodes")
        .core
        .items;
    assert_eq!(
        items.len(),
        decoded.core.items.len() + 1,
        "the plant adds a function"
    );
    let error = covered(&items).expect_err("a function without a differential");
    assert!(error.contains("nat_other"), "{error}");
}
