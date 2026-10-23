//! The std runtime as source (SPEC.md §17.17): `RB-20` and `SP-29`.
//!
//! The types and helpers of `RuntimeStd` and the functions of `RustStd` are
//! decoded and printed where the handwritten std runtime stood. A harness
//! calls the printed runtime on lists of every shape, on byte strings, texts,
//! and numerals, and records what each call returned and the work it counted;
//! the digests are the golden transcript at `tests/rust-runtime-golden/transcript.txt`,
//! recorded from the handwritten runtime at the last pass of the differential
//! (`RB-20`). The module's theorems are listed here, from the laws, never from the module, and
//! accepted by the pinned Lean (`SP-29`).

use std::collections::BTreeSet;

use lexlean::calculus::rust::ast::{Ident, ItemDef};
use lexlean::calculus::rust::{rt_decode, runtime, source};

use super::rust_refinement::project_of;
use super::rust_runtime::{golden, observe, run_harness, RECORDER};
use crate::support::{self, repo_root};

const MODULE: &str = "compiler/src/RustStd.lex.tex";
const SHIPPED: &str = "language/rust-runtime-1.2/RustStd.lex.tex";
const TYPES: &str = "compiler/src/RuntimeStd.lex.tex";
const TYPES_SHIPPED: &str = "language/rust-runtime-1.2/RuntimeStd.lex.tex";

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
const LAWS: [(&str, &str, &[&str]); 32] = [
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
    (
        "append_bytes",
        "refines_append_bytes",
        &["runExport", "runItem"],
    ),
    (
        "length_bytes",
        "refines_length_bytes",
        &["runExport", "runItem"],
    ),
    (
        "length_string",
        "refines_length_string",
        &["runExport", "runItem"],
    ),
    (
        "index_bytes",
        "refines_index_bytes",
        &["runExport", "runItem"],
    ),
    (
        "slice_bytes",
        "refines_slice_bytes",
        &["runExport", "runItem"],
    ),
    (
        "utf8_encode",
        "refines_utf8_encode",
        &["runExport", "runItem"],
    ),
    (
        "utf8_decode",
        "refines_utf8_decode",
        &["runExport", "runItem"],
    ),
    (
        "compare_bytes",
        "refines_compare_bytes",
        &["runExport", "runItem"],
    ),
    (
        "split_exact",
        "refines_split_exact",
        &["runExport", "runItem"],
    ),
    ("join", "refines_join", &["runExport", "runItem"]),
    ("format_u8", "refines_format_u8", &["runExport", "runItem"]),
    ("parse_u8", "refines_parse_u8", &["runExport", "runItem"]),
    ("format_u16", "refines_format_u16", &["runExport", "runItem"]),
    ("parse_u16", "refines_parse_u16", &["runExport", "runItem"]),
    ("format_u32", "refines_format_u32", &["runExport", "runItem"]),
    ("parse_u32", "refines_parse_u32", &["runExport", "runItem"]),
    ("format_u64", "refines_format_u64", &["runExport", "runItem"]),
    ("parse_u64", "refines_parse_u64", &["runExport", "runItem"]),
    ("format_i8", "refines_format_i8", &["runExport", "runItem"]),
    ("parse_i8", "refines_parse_i8", &["runExport", "runItem"]),
    ("format_i16", "refines_format_i16", &["runExport", "runItem"]),
    ("parse_i16", "refines_parse_i16", &["runExport", "runItem"]),
    ("format_i32", "refines_format_i32", &["runExport", "runItem"]),
    ("parse_i32", "refines_parse_i32", &["runExport", "runItem"]),
    ("format_i64", "refines_format_i64", &["runExport", "runItem"]),
    ("parse_i64", "refines_parse_i64", &["runExport", "runItem"]),
    ("parse_int", "refines_parse_int", &["runExport", "runItem"]),
    ("format_int", "refines_format_int", &["runExport", "runItem"]),
];

/// The harness: a prelude that builds lists of four shapes, then one section per
/// function of the module and per group of the support and the number text.
/// Every call goes through the recorder of the harnesses, so the digest under
/// a label is what the calls under it returned and counted.
const HARNESS: &str = include_str!("rust_std_harness.txt");

/// The golden transcript of the harness on the std runtime.
const GOLDEN: &str = "std";

/// The sections of the harness, each with its name.
fn sections() -> Vec<(&'static str, &'static str)> {
    let mut out = Vec::new();
    let mut rest = HARNESS;
    while let Some(at) = rest.find("//@ ") {
        let after = &rest[at + 4..];
        let (name, body) = after.split_once('\n').expect("a section name line");
        let end = body.find("//@ ").unwrap_or(body.len());
        out.push((name, &body[..end]));
        rest = &body[end..];
    }
    out
}

/// The names of the sections that call one function of the module each.
fn called() -> BTreeSet<String> {
    sections()
        .into_iter()
        .map(|(name, _)| name)
        .filter(|name| !matches!(*name, "prelude" | "support"))
        .map(str::to_owned)
        .collect()
}

/// The harness: every function of the module, then the support and the number text.
pub(super) fn harness() -> String {
    let mut out = String::from(RECORDER);
    for (_, body) in sections() {
        out.push_str(body);
    }
    out.push_str("    report();\n    println!(\"runtime\");\n}\n");
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
    let differential = called();
    if defined != differential {
        return Err(format!(
            "the module defines {defined:?}, the differential covers {differential:?}"
        ));
    }
    Ok(())
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

/// `RB-20`: the generated std runtime against the golden transcript.
fn differential() {
    let module = read(MODULE);
    let shipped = read(SHIPPED);
    // The shipped copy is the compiler project's module byte for byte: the
    // proofs are about one, the Rust crate reads the other.
    assert_eq!(module, shipped, "{SHIPPED} differs from {MODULE}");
    let krate = source::decode_std(&shipped).expect("the module decodes");
    assert!(
        !krate.items.is_empty(),
        "an empty std module defines nothing the differential could compare"
    );
    covered(&krate.items).expect("the differential covers the module");
    let types = read(TYPES);
    let types_shipped = read(TYPES_SHIPPED);
    assert_eq!(types, types_shipped, "{TYPES_SHIPPED} differs from {TYPES}");
    let support = rt_decode::decode_std(&types_shipped).expect("RuntimeStd decodes");
    assert!(
        !support.items.is_empty(),
        "an empty RuntimeStd defines nothing the differential could compare"
    );
    let core = runtime::core().expect("the core runtime");
    let generated_std = runtime::std().expect("the embedded std module");
    assert_eq!(
        generated_std,
        runtime::std_from(&shipped).expect("the std runtime is valid"),
        "std() is std_from of the embedded module"
    );
    let printed = runtime::std_types_from(&types_shipped).expect("RuntimeStd is valid");
    assert!(
        generated_std.starts_with(&printed),
        "the std runtime begins with the printed crate of RuntimeStd"
    );
    // The functions of the module are the runtime's free functions: the base
    // holds none of their names, so no function is declared twice, and the
    // golden transcript has a label for each.
    let recorded = golden(GOLDEN);
    for name in defined(&krate.items).expect("named functions") {
        // A free function: a method of the base may carry the name.
        let text = format!("\n{}", runtime::std_base().expect("the base"));
        assert!(
            !text.contains(&format!("\npub fn {name}<"))
                && !text.contains(&format!("\npub fn {name}(")),
            "the base still holds `{name}`"
        );
        // A function the oracle had under another name is held to that label.
        let label = super::rust_runtime::HELD_AS
            .iter()
            .find(|(held, _)| *held == name)
            .map_or(name.as_str(), |(_, to)| to);
        assert!(
            recorded.iter().any(|(recorded, _)| recorded == label),
            "the golden transcript lacks `{label}`"
        );
    }
    // A function taken out of the generated text leaves a harness that does
    // not compile: it is the plant.
    let without = generated_std.replacen("pub fn length_bytes(", "pub fn length_byte(", 1);
    assert_ne!(without, generated_std, "the plant site exists");
    let error = run_harness(&format!("{core}{without}"), harness())
        .expect_err("a function the generated runtime lacks is refused");
    assert!(error.contains("length_bytes"), "{error}");
    let ran = observe(&format!("{core}{generated_std}"), harness(), GOLDEN)
        .expect("the generated runtime has the golden transcript");
    assert_eq!(ran.lines().last(), Some("runtime"));

    // A miscounting walk, an extra tick of the work counter, and a signature
    // without its type parameter are each found: the first two by the
    // differential, the last by the compiler.
    for (what, from, to, caught) in [
        (
            "a walk that counts two for a cell",
            "tick(1); count += 1; cursor = &cell.tail;",
            "tick(1); count += 2; cursor = &cell.tail;",
            "length_list",
        ),
        (
            "a walk that ticks an extra time for the whole list",
            "pub fn length(&self) -> u64 { let mut count = 0;",
            "pub fn length(&self) -> u64 { tick(1); let mut count = 0;",
            "length_list",
        ),
        (
            "an append that puts the lists the other way",
            "List::onto(a.items(), b)",
            "List::onto(b.items(), a)",
            "append_list",
        ),
        (
            "an index that finds the next position",
            "if position == i { return Some(cell.head.clone()); }",
            "if position == i + 1 { return Some(cell.head.clone()); }",
            "index_list",
        ),
        (
            "a slice that is short by one cell and not refused",
            "if position < end { None } else {",
            "if position + 1 < end { None } else {",
            "slice_list",
        ),
        (
            "a comparison that does not count the first unit of work",
            "tick(1 + self.0.len().min(other.0.len()) as u64); self.0.cmp(&other.0)",
            "tick(self.0.len().min(other.0.len()) as u64); self.0.cmp(&other.0)",
            "compare_bytes",
        ),
        (
            "a join that does not count its delimiters",
            "tick(1); if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }; first = false; chars(&cell.head.0);",
            "tick(1); if !first { out.push_str(&delimiter.0); }; first = false; chars(&cell.head.0);",
            "join",
        ),
        (
            "a split that refuses the maximum itself",
            "if fields.len() as u128 > u128::from(maximum) { return None; }",
            "if fields.len() as u128 >= u128::from(maximum) { return None; }",
            "split_exact",
        ),
        (
            "a decoding that does not count its bytes",
            "pub fn utf8_decode(&self) -> Option<Str> { tick(self.0.len() as u64);",
            "pub fn utf8_decode(&self) -> Option<Str> { ",
            "utf8_decode",
        ),
        (
            "a slice that counts one more",
            "tick(count); Some(Bytes::lit(",
            "tick(count + 1); Some(Bytes::lit(",
            "slice_bytes",
        ),
        (
            "a changed byte in the number text",
            "strip_prefix('-')",
            "strip_prefix('+')",
            "parse_",
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
        let error = observe(&format!("{core}{planted}"), harness(), GOLDEN)
            .expect_err(&format!("{what}: a changed runtime has another transcript"));
        eprintln!(
            "RB-20: {what}: {}",
            error.lines().find(|line| !line.trim().is_empty()).unwrap_or_default()
        );
        assert!(error.contains(caught), "{what}: {error}");
    }

    // `format_int` is held to the transcript's `format_i64` (`HELD_AS`): one
    // that formats another value keeps its own label, which the golden
    // transcript lacks.
    let start = generated_std
        .find("pub fn format_int(")
        .expect("the generated runtime has format_int");
    let end = start + generated_std[start..].find("\n}\n").expect("its end");
    let body = &generated_std[start..end];
    assert_eq!(body.matches("Str::format_i64(").count(), 1, "format_int calls format_i64 once");
    let planted = format!(
        "{}{}{}",
        &generated_std[..start],
        body.replacen("Str::format_i64(", "Str::format_i64(-1 - ", 1),
        &generated_std[end..]
    );
    let error = observe(&format!("{core}{planted}"), harness(), GOLDEN)
        .expect_err("a format_int that formats another value has another transcript");
    eprintln!(
        "RB-20: a format_int that formats another value: {}",
        error.lines().find(|line| !line.trim().is_empty()).unwrap_or_default()
    );
    assert!(
        error.contains("the transcript differs in: format_int"),
        "a format_int that formats another value: {error}"
    );

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

    // The module of the types is refused for each rule it breaks: by the
    // decoder, and by the validator that checks it with the support items.
    for (what, from, to, message) in [
        (
            "no std support crate",
            "name: stdSupportCrate",
            "name: otherCrate",
            "does not define `stdSupportCrate`",
        ),
        (
            "a std support crate of another profile",
            "(ctor RuntimeSyntax::RtProfile.std)",
            "(ctor RuntimeSyntax::RtProfile.core)",
            "is not a std crate",
        ),
        (
            "an expression constructor RuntimeSyntax lacks",
            "RtExpr.loopE",
            "RtExpr.loopEs",
            "is not an expression",
        ),
        (
            "a std path the closed table lacks",
            "(cons \"str\" (cons \"from_utf8\"",
            "(cons \"str\" (cons \"from_utf9\"",
            "call-name",
        ),
        (
            "an associated function the impl does not state",
            "(cons \"Str\" (cons \"lit\" (nil String)))",
            "(cons \"Str\" (cons \"lit2\" (nil String)))",
            "call-name",
        ),
    ] {
        let edited = types_shipped.replace(from, to);
        assert_ne!(edited, types_shipped, "{what}: the plant site exists");
        let error = runtime::std_types_from(&edited).expect_err(what);
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

    // The module of the types is accepted by the pinned Lean: its term is
    // well typed against `RuntimeSyntax`.
    if support::lean_backed("RB-20") {
        let project = project_of(
            &[("RuntimeStd", types.as_str())],
            &["TargetSyntax", "RuntimeSyntax", "RuntimeStd"],
        );
        let _ = support::verify_ok(&project);
    }
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
            "= RustSemantics.ROutcome.stuck",
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
