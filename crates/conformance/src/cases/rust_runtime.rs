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

use crate::rust_build::{cargo_in, workspace, Built};
use crate::support::repo_root;

const MODULE: &str = "compiler/src/RustRuntime.lex.tex";
const SHIPPED: &str = "language/rust-runtime-1.2/RustRuntime.lex.tex";

/// The calls the differential makes of each defined function: the `same!`
/// of the harness runs the oracle's and the generated function on the same
/// arguments and compares what they return and the work they count.
const DIFFERENTIAL: [(&str, &str); 1] = [(
    "nat_add",
    "let edge = [0u64, 1, 2, 3, u32::MAX as u64, u32::MAX as u64 + 1, u64::MAX / 2, u64::MAX - 2, u64::MAX - 1, u64::MAX];\n    for &a in &edge { for &b in &edge { same!(nat_add(a, b)); } }\n    for _ in 0..2000 { let (a, b) = (next(), next()); same!(nat_add(a, b)); same!(nat_add(a >> 1, b >> 1)); }",
)];

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
    let differential: BTreeSet<String> = DIFFERENTIAL
        .iter()
        .map(|(name, _)| (*name).to_owned())
        .collect();
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
        "use runtime_diff::{new, old};\nmacro_rules! same {\n    ($f:ident($($a:expr),*)) => {{\n        let (w0, w1) = (old::work(), new::work());\n        let x = old::$f($($a),*);\n        let y = new::$f($($a),*);\n        assert_eq!(format!(\"{x:?}\"), format!(\"{y:?}\"), \"{} result\", stringify!($f));\n        assert_eq!(old::work() - w0, new::work() - w1, \"{} work\", stringify!($f));\n    }};\n}\nfn main() {\n    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;\n    let mut next = move || { seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17; seed };\n",
    );
    for (_, calls) in DIFFERENTIAL {
        out.push_str("    {\n    ");
        out.push_str(calls);
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
        assert!(
            !runtime::CORE_BASE.contains(&format!("pub fn {name}(")),
            "the base still holds `{name}`"
        );
        assert!(
            oracle.contains(&format!("pub fn {name}(")),
            "the oracle lacks `{name}`"
        );
    }
    let generated = runtime::assemble(&decoded.core.items);
    let ran = differ(&oracle, &generated).expect("the generated runtime agrees with the oracle");
    assert_eq!(ran, "runtime");

    // A function whose body computes another result is caught by the
    // differential, not by the decoder: the checked product for the checked sum.
    let changed = shipped.replacen("Machine.checkedNatAdd", "Machine.checkedNatMul", 1);
    assert_ne!(changed, shipped, "the plant site exists");
    let items = source::decode(&changed)
        .expect("a changed body decodes")
        .core
        .items;
    let error = differ(&oracle, &runtime::assemble(&items))
        .expect_err("a changed body disagrees with the oracle");
    assert!(error.contains("nat_add result"), "{error}");

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
    let extra = shipped
        .replacen(block, &format!("{block}\n{copy}"), 1)
        .replacen(
            "(cons (natAddFunction) ",
            "(cons (natOtherFunction) (cons (natAddFunction) ",
            1,
        )
        .replacen(
            "(nil RustSyntax::ItemDef))})",
            "(nil RustSyntax::ItemDef)))})",
            1,
        );
    let items = source::decode(&extra)
        .expect("a module with a second function decodes")
        .core
        .items;
    assert_eq!(items.len(), 2, "the plant adds a function");
    let error = covered(&items).expect_err("a function without a differential");
    assert!(error.contains("nat_other"), "{error}");
}
