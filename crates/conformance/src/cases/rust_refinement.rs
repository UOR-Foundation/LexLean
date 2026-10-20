//! The refinement of each core runtime function (SPEC.md §17.16): `SP-22`.
//!
//! The functions `RustRuntime` defines are listed by the same decoder the
//! compiler prints them with; the module's theorems are listed by name; the
//! two sets and the function each theorem states are compared, and the pinned
//! Lean elaborates the module with exactly the axioms each policy states.

use std::collections::BTreeSet;

use lexlean::calculus::rust::ast::{Ident, ItemDef};
use lexlean::calculus::rust::source;

use crate::support::{self, repo_root, P};

const MODULE: &str = "compiler/src/RustRuntime.lex.tex";

/// The modules the runtime module imports, transitively: the project that
/// elaborates it holds these and nothing else of the compiler project.
const IMPORTED: [&str; 5] = [
    "TargetSyntax",
    "TargetSemantics",
    "RustSyntax",
    "RustSemantics",
    "RustRuntime",
];

/// The names of the functions a module defines.
fn functions(text: &str) -> BTreeSet<String> {
    source::decode(text)
        .expect("the module decodes")
        .core
        .items
        .iter()
        .map(|item| match item {
            ItemDef::Function {
                name: Ident::Export(name),
                ..
            } => name.clone(),
            _ => panic!("the core runtime module defines an item that is not a named function"),
        })
        .collect()
}

/// Each refinement theorem of a module, by the function it names and the
/// function its statement runs.
fn theorems(text: &str) -> Vec<(String, String)> {
    let data = support::module_json(text);
    data["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .filter(|declaration| declaration["kind"] == "theorem")
        .filter_map(|declaration| {
            let name = declaration["name"].as_str()?.strip_prefix("refines_")?;
            let statement = declaration["statement"].to_string();
            // The function the statement runs: the one whose exported name is
            // a string literal of the statement.
            let run = functions(text)
                .into_iter()
                .find(|function| statement.contains(&format!("\"value\":\"{function}\"")))
                .unwrap_or_default();
            Some((name.to_owned(), run))
        })
        .collect()
}

/// Whether the module has exactly one refinement theorem per function it
/// defines, each stating the function it is named for.
fn complete(text: &str) -> Result<(), String> {
    let defined = functions(text);
    let stated = theorems(text);
    let named: BTreeSet<String> = stated.iter().map(|(name, _)| name.clone()).collect();
    if named.len() != stated.len() {
        return Err("a function has two refinement theorems".to_owned());
    }
    if named != defined {
        return Err(format!(
            "the module defines {defined:?}, its refinement theorems are for {named:?}"
        ));
    }
    for (name, run) in &stated {
        if name != run {
            return Err(format!(
                "`refines_{name}` states the run of `{run}`, not of `{name}`"
            ));
        }
    }
    Ok(())
}

/// A project holding the runtime module and what it imports.
fn project(text: &str) -> P {
    let project = P::compiler();
    for entry in std::fs::read_dir(project.root.join("src").as_std_path()).expect("src") {
        let path = entry.expect("entry").path();
        let stem = path
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.strip_suffix(".lex.tex"))
            .map(str::to_owned);
        if stem.is_some_and(|stem| !IMPORTED.contains(&stem.as_str())) {
            std::fs::remove_file(path).expect("remove a module");
        }
    }
    project.write_raw("src/RustRuntime.lex.tex", text);
    project.edit(
        "lexlean.toml",
        "entrypoints = [\"src/Main.lex.tex\"]",
        "entrypoints = [\"src/RustRuntime.lex.tex\"]",
    );
    project.relock();
    project
}

/// Run the case of `SP-22`.
pub fn run(id: &str) {
    assert_eq!(id, "SP-22");
    let text = std::fs::read_to_string(repo_root().join(MODULE).as_std_path()).expect(MODULE);
    assert!(
        !functions(&text).is_empty(),
        "a module that defines nothing states no refinement"
    );
    complete(&text).expect("each function has its theorem");

    // A theorem for another function, a function without one, and a theorem
    // that runs another function than its name says are each refused.
    let renamed = text.replacen("name: refines_nat_add", "name: refines_nat_sub", 1);
    assert_ne!(renamed, text, "the plant site exists");
    let error = complete(&renamed).expect_err("a theorem for another function");
    assert!(error.contains("refinement theorems are for"), "{error}");
    let start = text
        .find("theorem{\n  name: refines_nat_add")
        .expect("the theorem");
    let end = start + text[start..].find("\n}\n").expect("its end") + 3;
    let without = format!("{}{}", &text[..start], &text[end..]);
    let error = complete(&without).expect_err("a function without a theorem");
    assert!(error.contains("refinement theorems are for"), "{error}");
    let other = text.replacen(
        "(RustSemantics::runExport 12 (coreCrate) \"nat_add\"",
        "(RustSemantics::runExport 12 (coreCrate) \"nat_sub\"",
        1,
    );
    assert_ne!(other, text, "the plant site exists");
    let error = complete(&other).expect_err("a theorem that runs another function");
    assert!(error.contains("states the run of"), "{error}");

    if !support::lean_backed("SP-22") {
        return;
    }
    // The pinned Lean elaborates the module and each theorem's policy.
    let accepted = project(&text);
    let _ = support::verify_ok(&accepted);

    // A function whose body is changed to another result no longer refines
    // the item: the checked product for the checked sum.
    let changed = text.replacen("Machine.checkedNatAdd", "Machine.checkedNatMul", 1);
    assert_ne!(changed, text, "the plant site exists");
    let refused = project(&changed);
    let error = refused.verify_fails_with("LLV7002");
    assert!(
        error.to_string().contains("unsolved goals"),
        "Lean leaves the refinement unproved: {error}"
    );
}
