//! The std runtime as source (SPEC.md §17.17): `RB-20` and `SP-29`.
//!
//! The functions of `RustStd` are decoded and printed in place of the
//! handwritten functions of the same name, which are kept as the oracle at
//! `tests/rust-runtime-oracle/`. One crate holds the oracle as module `old`
//! and the generated runtime as module `new`; a harness calls both on lists of
//! every shape and compares the result and the counted work (`RB-20`). The
//! module's theorems are listed here, from the laws, never from the module, and
//! accepted by the pinned Lean (`SP-29`).

use std::collections::BTreeSet;

use lexlean::calculus::rust::ast::{Ident, ItemDef};
use lexlean::calculus::rust::{runtime, source};

use super::rust_refinement::project_of;
use super::rust_runtime::differ_with;
use crate::support::{self, repo_root};

const MODULE: &str = "compiler/src/RustStd.lex.tex";
const SHIPPED: &str = "language/rust-runtime-1.2/RustStd.lex.tex";

/// The modules the std module imports.
const IMPORTED: [&str; 4] = [
    "RustSemantics",
    "RustSyntax",
    "TargetSemantics",
    "TargetSyntax",
];

/// Each function the module defines with the theorem that refines it and the
/// names its statement must be about: a theorem of the right name that runs
/// another function or another item is refused.
const LAWS: [(&str, &str, &[&str]); 4] = [
    (
        "append_list",
        "refines_append_list",
        &["runExport", "runItem"],
    ),
    (
        "length_list",
        "refines_length_list",
        &["runExport", "runItem"],
    ),
    (
        "index_list",
        "refines_index_list",
        &["runExport", "runItem"],
    ),
    (
        "slice_list",
        "refines_slice_list",
        &["runExport", "runItem"],
    ),
];

/// The prelude of the harness: both runtimes, and a builder of a list of `n`
/// cells of one of four shapes, which returns the list and the handles that
/// keep the buffer it shares alive for the call.
const PRELUDE: &str = "use runtime_diff::{new, old};\nmacro_rules! build {\n    ($m:ident, $n:expr, $shape:expr) => {{\n        let mut list = $m::List::nil();\n        for i in 0..$n { list = $m::List::cons(i as u64, list); }\n        match $shape {\n            1 => match list.uncons() {\n                Some((head, tail)) => { drop(list); ($m::List::cons(head, tail), None) }\n                None => (list, None),\n            },\n            2 => { let shared = list.clone(); ($m::List::cons(7u64, shared), Some(list)) }\n            3 => { let shared = list.clone(); ($m::List::cons(1u64, $m::List::cons(2u64, shared)), Some(list)) }\n            _ => (list, None),\n        }\n    }};\n}\nmacro_rules! items {\n    ($l:expr) => {{\n        let mut out = Vec::new();\n        let mut cur = $l;\n        while let Some((head, tail)) = cur.uncons() { out.push(head); cur = tail; }\n        out\n    }};\n}\nfn main() {\n    let lengths: Vec<usize> = (0..=70).chain([1000, 4096, 200_000]).collect();\n";

/// The calls the differential makes of each function the module defines.
const CALLS: [(&str, &str); 4] = [
    (
        "append_list",
        "    for &n in &lengths {\n        for m in [0usize, 1, 5, 70, 1000] {\n            for shape in 0..4 {\n                let (a, _keep_a) = build!(old, n, shape);\n                let (b, _keep_b) = build!(new, n, shape);\n                let (c, _keep_c) = build!(old, m, 0);\n                let (d, _keep_d) = build!(new, m, 0);\n                let (w0, w1) = (old::work(), new::work());\n                let x = old::append_list(a, c);\n                let y = new::append_list(b, d);\n                assert_eq!(old::work() - w0, new::work() - w1, \"append_list work\");\n                assert_eq!(items!(x), items!(y), \"append_list result\");\n            }\n        }\n    }\n",
    ),
    (
        "length_list",
        "    for &n in &lengths {\n        for shape in 0..4 {\n            let (a, _keep_a) = build!(old, n, shape);\n            let (b, _keep_b) = build!(new, n, shape);\n            let (w0, w1) = (old::work(), new::work());\n            let x = old::length_list(a);\n            let y = new::length_list(b);\n            assert_eq!(x, y, \"length_list result\");\n            assert_eq!(old::work() - w0, new::work() - w1, \"length_list work\");\n        }\n    }\n",
    ),
    (
        "index_list",
        "    for &n in &lengths {\n        let n = n as u64;\n        for at in [0, 1, n / 2, n.saturating_sub(1), n, n + 1, u64::MAX] {\n            for shape in 0..4 {\n                let (a, _keep_a) = build!(old, n as usize, shape);\n                let (b, _keep_b) = build!(new, n as usize, shape);\n                let (w0, w1) = (old::work(), new::work());\n                let x = old::index_list(a, at);\n                let y = new::index_list(b, at);\n                assert_eq!(x, y, \"index_list result\");\n                assert_eq!(old::work() - w0, new::work() - w1, \"index_list work\");\n            }\n        }\n    }\n",
    ),
    (
        "slice_list",
        "    for &n in &lengths {\n        let n = n as u64;\n        for (start, count) in [(0, 0), (0, n), (1, n), (n / 2, n / 4), (n, 0), (n, 1), (n.saturating_sub(1), 1), (u64::MAX, u64::MAX), (1, u64::MAX)] {\n            for shape in 0..4 {\n                let (a, _keep_a) = build!(old, n as usize, shape);\n                let (b, _keep_b) = build!(new, n as usize, shape);\n                let (w0, w1) = (old::work(), new::work());\n                let x = old::slice_list(a, start, count);\n                let y = new::slice_list(b, start, count);\n                assert_eq!(old::work() - w0, new::work() - w1, \"slice_list work\");\n                assert_eq!(x.map(|l| items!(l)), y.map(|l| items!(l)), \"slice_list result\");\n            }\n        }\n    }\n",
    ),
];

/// The harness: both runtimes, every function of the differential.
fn harness() -> String {
    let mut out = String::from(PRELUDE);
    for (_, calls) in CALLS {
        out.push_str(calls);
    }
    out.push_str("    println!(\"runtime\");\n}\n");
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
                "the std runtime module defines an item that is not a named function".to_owned(),
            ),
        })
        .collect()
}

/// Whether the differential covers exactly the functions the module defines.
fn covered(items: &[ItemDef]) -> Result<(), String> {
    let defined = defined(items)?;
    let differential: BTreeSet<String> = CALLS.iter().map(|(name, _)| (*name).to_owned()).collect();
    if defined != differential {
        return Err(format!(
            "the module defines {defined:?}, the differential covers {differential:?}"
        ));
    }
    Ok(())
}

/// The first function of the oracle that `generated` does not define.
fn lost(oracle: &str, generated: &str) -> Option<String> {
    oracle
        .lines()
        .filter(|line| line.starts_with("pub fn "))
        .map(|line| {
            line["pub fn ".len()..]
                .split(['<', '('])
                .next()
                .expect("a name")
                .to_owned()
        })
        .find(|name| {
            !generated.contains(&format!("pub fn {name}<"))
                && !generated.contains(&format!("pub fn {name}("))
        })
}

/// The theorem a Lean diagnostic points at: the diagnostic carries the span
/// of the declaration, whose first line is `theorem{` and whose next line
/// names it.
fn refused_theorem(source: &str, shown: &str) -> String {
    let line: usize = shown
        .split("line_start: ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .and_then(|number| number.trim().parse().ok())
        .expect("the diagnostic has a line");
    let lines: Vec<&str> = source.lines().collect();
    assert_eq!(lines[line - 1], "theorem{", "the span opens a theorem");
    lines[line]
        .trim()
        .trim_start_matches("name: ")
        .trim_end_matches(',')
        .to_owned()
}

/// Run the case of `RB-20` or `SP-29`. The module is read by the reader of
/// the compiler, so the case runs on a stack of its own.
pub fn run(id: &str) {
    let id = id.to_owned();
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(lexlean::READER_STACK)
            .spawn_scoped(scope, || match id.as_str() {
                "RB-20" => differential(),
                "SP-29" => refinement(),
                other => panic!("`{other}` is not a case of the std runtime"),
            })
            .expect("a thread for the case")
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic));
    });
}

fn read(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path).as_std_path())
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// `RB-20`: the generated std runtime against the oracle.
fn differential() {
    let module = read(MODULE);
    let shipped = read(SHIPPED);
    // The shipped copy is the compiler project's module byte for byte: the
    // proofs are about one, the Rust crate reads the other.
    assert_eq!(module, shipped, "{SHIPPED} differs from {MODULE}");
    let core_oracle = read("tests/rust-runtime-oracle/core.rs.txt");
    let std_oracle = read("tests/rust-runtime-oracle/std.rs.txt");
    let oracle = format!("{core_oracle}{std_oracle}");
    let krate = source::decode_std(&shipped).expect("the module decodes");
    assert!(
        !krate.items.is_empty(),
        "an empty std module defines nothing the differential could compare"
    );
    covered(&krate.items).expect("the differential covers the module");
    let core = runtime::core().expect("the core runtime");
    let generated_std = runtime::std().expect("the embedded std module");
    assert_eq!(
        generated_std,
        runtime::std_from(&shipped).expect("the std runtime is valid"),
        "std() is std_from of the embedded module"
    );
    // The generated functions replace the oracle's: the base holds none of
    // their names, so no function is declared twice.
    for name in defined(&krate.items).expect("named functions") {
        assert!(
            !runtime::STD_BASE.contains(&format!("pub fn {name}<")),
            "the base still holds `{name}`"
        );
        assert!(
            std_oracle.contains(&format!("pub fn {name}<")),
            "the oracle lacks `{name}`"
        );
    }
    // And none of the oracle's functions is lost: each is in the generated
    // text, whether the base keeps it or the module defines it. One function
    // taken out of the generated text is the plant.
    assert_eq!(lost(&std_oracle, generated_std), None);
    let without = generated_std.replacen("pub fn length_bytes(", "pub fn length_byte(", 1);
    assert_ne!(without, generated_std, "the plant site exists");
    assert_eq!(
        lost(&std_oracle, &without),
        Some("length_bytes".to_owned()),
        "a function of the oracle that the generated runtime lacks is refused"
    );
    let ran = differ_with(&oracle, &format!("{core}{generated_std}"), harness())
        .expect("the generated runtime agrees with the oracle");
    assert_eq!(ran, "runtime");

    // A miscounting walk, an extra tick of the work counter, and a signature
    // without its type parameter are each found: the first two by the
    // differential, the last by the compiler.
    for (what, from, to, caught) in [
        (
            "a walk that counts two for a cell",
            "count += 1; true",
            "count += 2; true",
            "length_list result",
        ),
        (
            "a walk that ticks an extra time for the whole list",
            "pub fn length(&self) -> u64 {\n        let mut count = 0;",
            "pub fn length(&self) -> u64 {\n        tick(1);\n        let mut count = 0;",
            "length_list work",
        ),
        (
            "an append that puts the lists the other way",
            "List::onto(a.items(), b)",
            "List::onto(b.items(), a)",
            "append_list",
        ),
        (
            "an index that finds the next position",
            "if position == i {",
            "if position == i + 1 {",
            "index_list",
        ),
        (
            "a slice that is short by one cell and not refused",
            "if position < end { None }",
            "if position + 1 < end { None }",
            "slice_list",
        ),
        (
            "a signature without its type parameter",
            "pub fn length_list<T: Clone>(",
            "pub fn length_list<>(",
            "cannot find type `T`",
        ),
    ] {
        assert_eq!(
            generated_std.matches(from).count(),
            1,
            "{what}: the plant site occurs once"
        );
        let planted = generated_std.replacen(from, to, 1);
        let error = differ_with(&oracle, &format!("{core}{planted}"), harness())
            .expect_err("a changed runtime disagrees with the oracle");
        assert!(error.contains(caught), "{what}: {error}");
    }

    // The crate is checked by the validator that checks every rendering: a
    // machine call with another number of operands is refused.
    let wrong = shipped.replacen(
        "(ctor RustSyntax::Machine.listLength)) (cons (ctor RustSyntax::Expr.move (ctor RustSyntax::Ident.generated (ctor RustSyntax::IdentKind.param) 0)) (nil RustSyntax::Expr))",
        "(ctor RustSyntax::Machine.listLength)) (cons (ctor RustSyntax::Expr.move (ctor RustSyntax::Ident.generated (ctor RustSyntax::IdentKind.param) 0)) (cons (ctor RustSyntax::Expr.lit (ctor RustSyntax::Lit.nat 0)) (nil RustSyntax::Expr)))",
        1,
    );
    assert_ne!(wrong, shipped, "the plant site exists");
    let error = runtime::std_from(&wrong).expect_err("two operands of a one-operand primitive");
    assert!(
        error.contains("the std runtime is not a valid crate") && error.contains("`list_length`"),
        "{error}"
    );

    // A module the decoder refuses, each for the rule it breaks.
    for (what, from, to, message) in [
        (
            "no std crate",
            "name: stdCrate",
            "name: otherCrate",
            "does not define `stdCrate`",
        ),
        (
            "a std crate of another profile",
            "(ctor RustSyntax::Profile.std)",
            "(ctor RustSyntax::Profile.core)",
            "is not a std crate",
        ),
        (
            "a machine primitive the machine lacks",
            "Machine.listLength",
            "Machine.listLengths",
            "is not a machine primitive",
        ),
        (
            "a type the syntax lacks",
            "RustType.param",
            "RustType.parameter",
            "is not a type",
        ),
    ] {
        let edited = shipped.replacen(from, to, 1);
        assert_ne!(edited, shipped, "{what}: the plant site exists");
        let error = source::decode_std(&edited).expect_err(what);
        assert!(error.contains(message), "{what}: {error}");
    }

    // A function the differential does not cover is refused by the case,
    // whatever the function is.
    let mut items = krate.items.clone();
    if let ItemDef::Function { name, .. } = &mut items[0] {
        *name = Ident::Export("length_other".to_owned());
    }
    let error = covered(&items).expect_err("a function without a differential");
    assert!(error.contains("length_other"), "{error}");
}

/// Each theorem of the module with the text of its statement.
fn theorems(text: &str) -> Vec<(String, String)> {
    support::module_json(text)["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .filter(|declaration| declaration["kind"] == "theorem")
        .map(|declaration| {
            (
                declaration["name"].as_str().expect("a name").to_owned(),
                declaration["statement"].to_string(),
            )
        })
        .collect()
}

/// Whether the module has exactly the theorems of the laws, each stating the
/// function it refines and the names of its law.
fn complete(text: &str) -> Result<(), String> {
    let stated = theorems(text);
    let named: BTreeSet<String> = stated.iter().map(|(name, _)| name.clone()).collect();
    if named.len() != stated.len() {
        return Err("a function has two theorems".to_owned());
    }
    let expected: BTreeSet<String> = LAWS
        .iter()
        .map(|(_, theorem, _)| (*theorem).to_owned())
        .collect();
    if named != expected {
        return Err(format!(
            "the functions call for {} theorems, the module states {}: missing {:?}, unexpected {:?}",
            expected.len(),
            named.len(),
            expected.difference(&named).collect::<Vec<_>>(),
            named.difference(&expected).collect::<Vec<_>>(),
        ));
    }
    for (name, statement) in &stated {
        let (function, _, names) = LAWS
            .iter()
            .find(|(_, theorem, _)| theorem == name)
            .expect("a law of the list");
        if !statement.contains(&format!("\"{function}\"")) {
            return Err(format!("`{name}` does not run `{function}`"));
        }
        for item in *names {
            if !statement.contains(&format!("\"{item}\"")) {
                return Err(format!("`{name}` does not state `{item}`"));
            }
        }
    }
    Ok(())
}

/// `SP-29`: the refinement theorem of the function the module defines.
fn refinement() {
    let text = read(MODULE);
    complete(&text).expect("each function has its theorem");

    // A theorem for another function, a function without a theorem, and a
    // theorem that runs another function are each refused.
    let outside = text.replacen(
        "name: refines_length_list,",
        "name: refines_length_lists,",
        1,
    );
    assert_ne!(outside, text, "the plant site exists");
    let error = complete(&outside).expect_err("a theorem for another function");
    assert!(
        error.contains("unexpected [\"refines_length_lists\"]"),
        "{error}"
    );
    assert!(
        error.contains("missing [\"refines_length_list\"]"),
        "{error}"
    );
    let start = text
        .find("theorem{\n  name: refines_length_list,")
        .expect("the theorem");
    let end = start + text[start..].find("\n}\n").expect("its end") + 3;
    let without = format!("{}{}", &text[..start], &text[end..]);
    let error = complete(&without).expect_err("a function without a theorem");
    assert!(
        error.contains("missing [\"refines_length_list\"]"),
        "{error}"
    );
    let block = &text[start..end];
    let other = text.replacen(
        block,
        &block.replace("\"length_list\"", "\"length_lists\""),
        1,
    );
    assert_ne!(other, text, "the plant site exists");
    let error = complete(&other).expect_err("a theorem that runs another function");
    assert!(
        error.contains("`refines_length_list` does not run `length_list`"),
        "{error}"
    );

    if !support::lean_backed("SP-29") {
        return;
    }
    // The pinned Lean elaborates every theorem with the axioms its policy
    // states.
    let accepted = project_of(&[("RustStd", text.as_str())], &IMPORTED);
    let _ = support::verify_ok(&accepted);

    // A machine call whose primitive is another, and a theorem that states
    // another runtime item, are each left unproved by Lean.
    for (what, from, to, goal) in [
        (
            "a machine call whose primitive is another",
            "(ctor RustSyntax::Machine.listLength)",
            "(ctor RustSyntax::Machine.natEq)",
            "TargetSyntax.Prim.natEq",
        ),
        (
            "a theorem that states another runtime item",
            "(ctor RustSyntax::Item.lengthList)",
            "(ctor RustSyntax::Item.lengthBytes)",
            "RustSemantics.isBytes",
        ),
    ] {
        assert_eq!(
            text.matches(from).count(),
            1,
            "{what}: the plant site occurs once"
        );
        let planted = text.replacen(from, to, 1);
        let refused = project_of(&[("RustStd", planted.as_str())], &IMPORTED);
        let shown = format!("{:?}", refused.verify_fails_with("LLV7002"));
        assert!(
            shown.contains("unsolved goals") && shown.contains(goal),
            "{what}: Lean leaves the goal `{goal}` open: {shown}"
        );
        assert_eq!(
            refused_theorem(&planted, &shown),
            "refines_length_list",
            "{what}: the theorem Lean refuses"
        );
    }
}
