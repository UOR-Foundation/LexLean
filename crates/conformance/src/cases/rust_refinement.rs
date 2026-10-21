//! The refinement of each core runtime function (SPEC.md §17.16): `SP-22`.
//!
//! The functions `RustRuntime` defines are listed by the same decoder the
//! compiler prints them with; the module's theorems are listed by name; the
//! two sets and the function each theorem states are compared, and the pinned
//! Lean elaborates the module with exactly the axioms each policy states.

use std::collections::BTreeSet;

use lexlean::calculus::rust::ast::{Ident, ItemDef};
use lexlean::calculus::rust::rt::Item as Support;
use lexlean::calculus::rust::runtime::Item;
use lexlean::calculus::rust::{rt_decode, source};

use crate::support::{self, repo_root, P};

const MODULE: &str = "compiler/src/RustRuntime.lex.tex";
const SUPPORT: &str = "compiler/src/RuntimeSupport.lex.tex";

/// The functions of the support crate that no refinement theorem of this row
/// states: the work counter's, which the counted-work gate (`RB-06`, `TC-07`)
/// holds, because their meaning is an effect that the erased-value semantics
/// does not model.
const WORK_GATE: [&str; 2] = ["tick", "work"];

/// The functions of the support crate whose meaning `SP-25` states as a
/// theorem per type of the core profile's predicate.
const TRAIT_GATE: [&str; 2] = ["equal", "compare"];

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
    let defined = functions(text);
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
            let run = defined
                .iter()
                .find(|function| statement.contains(&format!("\"value\":\"{function}\"")))
                .map(|function| function.replace("::", "_"))
                .unwrap_or_default();
            Some((name.to_owned(), run))
        })
        .collect()
}

/// Whether each authored item of the runtime is held: a function of
/// `RustRuntime` is held by its refinement theorem and must name a runtime
/// item; an item of `RuntimeSupport` is of a closed kind (the decoder admits
/// no other), and a function of it is held by the work gate or by `SP-25`'s
/// theorems. An authored item in neither set is refused by name.
fn held(runtime: &str, support: &str, keys: &[String]) -> Result<(), String> {
    let paths: BTreeSet<String> = Item::all().iter().map(|item| item.path()).collect();
    for function in functions(runtime) {
        if !paths.contains(&function) {
            return Err(format!(
                "`{function}` is an authored function that is not a runtime item"
            ));
        }
    }
    let krate = rt_decode::decode(support).map_err(|error| error.to_string())?;
    let theorems: Vec<String> = std::iter::once(support)
        .chain(keys.iter().map(String::as_str))
        .flat_map(|text| {
            support::module_json(text)["declarations"]
                .as_array()
                .expect("declarations")
                .iter()
                .filter(|declaration| declaration["kind"] == "theorem")
                .filter_map(|declaration| declaration["name"].as_str().map(str::to_owned))
                .collect::<Vec<_>>()
        })
        .collect();
    for item in &krate.items {
        if let Support::Fn(function) = item {
            let name = function.sig.name.as_str();
            if WORK_GATE.contains(&name) {
                continue;
            }
            if TRAIT_GATE.contains(&name)
                && theorems
                    .iter()
                    .any(|theorem| theorem.starts_with(&format!("{name}_")))
            {
                continue;
            }
            return Err(format!(
                "`{name}` is an authored item in neither the refined functions nor the closed support kinds"
            ));
        }
    }
    Ok(())
}

/// Whether the module has exactly one refinement theorem per function it
/// defines, each stating the function it is named for.
fn complete(text: &str) -> Result<(), String> {
    // A function in a module is named by its path; its theorem by the path
    // with `_` for `::`.
    let defined: BTreeSet<String> = functions(text)
        .into_iter()
        .map(|function| function.replace("::", "_"))
        .collect();
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

/// The module with every theorem but `name` left out.
pub(super) fn keeping(text: &str, name: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(at) = rest.find("theorem{\n  name: ") {
        out.push_str(&rest[..at]);
        let block_end = rest[at..]
            .find("\n}\n")
            .map_or(rest.len(), |end| at + end + 3);
        if rest[at..block_end].starts_with(&format!("theorem{{\n  name: {name},")) {
            out.push_str(&rest[at..block_end]);
        }
        rest = rest[block_end..].trim_start_matches('\n');
    }
    out.push_str(rest);
    out
}

/// A project holding the runtime module and what it imports.
fn project(text: &str) -> P {
    project_of(&[("RustRuntime", text)], &IMPORTED)
}

/// A project holding the modules `modules`, each with its text, and the
/// modules `imported` of the compiler project, and nothing else of it.
pub(super) fn project_of(modules: &[(&str, &str)], imported: &[&str]) -> P {
    let project = P::compiler();
    for entry in std::fs::read_dir(project.root.join("src").as_std_path()).expect("src") {
        let path = entry.expect("entry").path();
        let stem = path
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.strip_suffix(".lex.tex"))
            .map(str::to_owned);
        if stem.is_some_and(|stem| !imported.contains(&stem.as_str())) {
            std::fs::remove_file(path).expect("remove a module");
        }
    }
    for (module, text) in modules {
        project.write_raw(&format!("src/{module}.lex.tex"), text);
    }
    // The project requires its entrypoints unique and sorted.
    let mut entrypoints: Vec<String> = modules
        .iter()
        .map(|(module, _)| format!("\"src/{module}.lex.tex\""))
        .collect();
    entrypoints.sort();
    project.edit(
        "lexlean.toml",
        "entrypoints = [\"src/Main.lex.tex\"]",
        &format!("entrypoints = [{}]", entrypoints.join(", ")),
    );
    // Each theorem is a proof search over the whole crate: the module of every
    // function elaborates in minutes, longer than the five a project allows.
    project.edit(
        "lexlean.toml",
        "child_timeout_ms = 300000",
        "child_timeout_ms = 3600000",
    );
    project.relock();
    project
}

/// Run the case of `SP-22`. The module of every function nests as deeply as
/// its list, and the readers it passes through recurse with it, so the case
/// runs on a stack of its own rather than the two megabytes of a test thread.
pub fn run(id: &str) {
    let id = id.to_owned();
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(lexlean::READER_STACK)
            .spawn_scoped(scope, || check(&id))
            .expect("a thread for the case")
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic));
    });
}

fn check(id: &str) {
    assert_eq!(id, "SP-22");
    let text = std::fs::read_to_string(repo_root().join(MODULE).as_std_path()).expect(MODULE);
    assert!(
        !functions(&text).is_empty(),
        "a module that defines nothing states no refinement"
    );
    complete(&text).expect("each function has its theorem");
    let support_text =
        std::fs::read_to_string(repo_root().join(SUPPORT).as_std_path()).expect(SUPPORT);
    let keys: Vec<String> = super::runtime_semantics::key_modules();
    held(&text, &support_text, &keys).expect("each authored item is held");
    // An authored function that names no runtime item, and a support function
    // that is neither the counter's nor a trait function with theorems, are
    // refused by name.
    let stray = text.replacen("\"nat_add\"", "\"nat_addd\"", 1);
    assert_ne!(stray, text, "the plant site exists");
    let error = held(&stray, &support_text, &keys).expect_err("a function that is no item");
    assert!(error.contains("`nat_addd` is an authored function"), "{error}");
    let alien = support_text.replacen("name := \"tick\"", "name := \"frobnicate\"", 1);
    assert_ne!(alien, support_text, "the plant site exists");
    let error = held(&text, &alien, &keys).expect_err("a support function in neither set");
    assert!(error.contains("`frobnicate` is an authored item in neither"), "{error}");
    let bare = support_text.replace("name: equal_", "name: sameness_");
    let error = held(&text, &bare, &keys).expect_err("a trait function without theorems");
    assert!(error.contains("`equal` is an authored item in neither"), "{error}");

    // A theorem for another function, a function without one, and a theorem
    // that runs another function than its name says are each refused.
    let renamed = text.replacen("name: refines_nat_add", "name: refines_nat_other", 1);
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
    // The plant needs one theorem to fail, so the others are left out: the
    // module of every function takes minutes to elaborate.
    let refused = project(&keeping(&changed, "refines_nat_add"));
    let error = refused.verify_fails_with("LLV7002");
    assert!(
        error.to_string().contains("unsolved goals"),
        "Lean leaves the refinement unproved: {error}"
    );
}
