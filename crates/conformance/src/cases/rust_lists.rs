//! The representation of a list (SPEC.md §17.16): `SP-27`.
//!
//! `RustLists` models a list as the cells in front of it and a run, the first
//! slots of a buffer in a store, and reads it back as the list of the
//! calculus. Its abstraction theorems are accepted by the pinned Lean. They
//! are listed here, from the laws, never from the module, so a law the module
//! lacks is a failure of this case, and each theorem is held to the functions
//! it states.

use std::collections::BTreeSet;

use super::rust_refinement::project_of;
use crate::support::{self, repo_root};

const MODULE: &str = "compiler/src/RustLists.lex.tex";

/// The modules the list module imports.
const IMPORTED: [&str; 2] = ["RustMemory", "TargetSyntax"];

/// Each law with the functions its statement must be about: a theorem of the
/// right name that states another function is refused.
const LAWS: [(&str, &[&str]); 26] = [
    ("run_cons_abs", &["absRun", "snoc", "takeN"]),
    ("run_uncons_abs", &["absRun", "unconsAbs", "get"]),
    ("walk_cons", &["runWalk", "consRun"]),
    ("walk_alloc", &["runWalk", "consRun"]),
    ("walk_uncons", &["runWalk", "unconsAbs", "headAt"]),
    ("walk_uncons_last", &["runWalk", "unconsAbs", "headAt"]),
    (
        "walk_frame",
        &["runWalk", "consRun", "gapPick", "gapOf", "cellsAt"],
    ),
    ("cons_cell_abs", &["absHandle", "consWorld"]),
    ("cons_shared_abs", &["absHandle", "consWorld"]),
    ("cons_alloc_abs", &["absHandle", "consWorld"]),
    ("cons_inplace_abs", &["absHandle", "consWorld"]),
    ("uncons_cell_abs", &["absHandle"]),
    ("uncons_cell_taken", &["unconsHandle"]),
    ("uncons_run_abs", &["absHandle", "unconsAbs", "headAt"]),
    ("uncons_run_taken", &["unconsHandle", "takenFrom", "headAt"]),
    ("uncons_last_abs", &["absHandle", "unconsAbs", "headAt"]),
    (
        "uncons_last_taken",
        &["unconsHandle", "takenFrom", "headAt"],
    ),
    (
        "inplace_frame",
        &["absHandle", "consRun", "gapPick", "gapOf"],
    ),
    ("same_is_no_gap", &["sameNat", "gapFlag", "gapOf"]),
    ("same_with_itself", &["sameNat"]),
    ("holder_on_own_buffer", &["holds"]),
    ("cons_steps_bounded", &["consSteps", "lessThan"]),
    ("uncons_steps_bounded", &["unconsSteps", "lessThan"]),
    ("release_ends", &["ends", "releaseLoop", "lengthOf"]),
    ("release_needs_fuel", &["releaseLoop"]),
    ("release_one_cell", &["releaseLoop"]),
];

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
/// functions of its law.
fn complete(text: &str) -> Result<(), String> {
    let stated = theorems(text);
    let named: BTreeSet<String> = stated.iter().map(|(name, _)| name.clone()).collect();
    if named.len() != stated.len() {
        return Err("a law has two theorems".to_owned());
    }
    let expected: BTreeSet<String> = LAWS.iter().map(|(name, _)| (*name).to_owned()).collect();
    if named != expected {
        return Err(format!(
            "the laws call for {} theorems, the module states {}: missing {:?}, unexpected {:?}",
            expected.len(),
            named.len(),
            expected.difference(&named).collect::<Vec<_>>(),
            named.difference(&expected).collect::<Vec<_>>(),
        ));
    }
    for (name, statement) in &stated {
        let law = LAWS
            .iter()
            .find(|(law, _)| law == name)
            .expect("a law of the list");
        for function in law.1 {
            if !statement.contains(&format!("\"{function}\"")) {
                return Err(format!("`{name}` does not state `{function}`"));
            }
        }
    }
    Ok(())
}

/// Run the case of `SP-27`. The module is small, but its reader is the one
/// of the compiler, so the case runs on a stack of its own.
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
    assert_eq!(id, "SP-27");
    let text = std::fs::read_to_string(repo_root().join(MODULE).as_std_path()).expect(MODULE);
    complete(&text).expect("each law has its theorem");

    // A theorem for another function, a law without a theorem, and a theorem
    // that states another function are each refused.
    let outside = text.replacen("name: walk_cons,", "name: walk_conses,", 1);
    assert_ne!(outside, text, "the plant site exists");
    let error = complete(&outside).expect_err("a theorem for another function");
    assert!(error.contains("unexpected [\"walk_conses\"]"), "{error}");
    assert!(error.contains("missing [\"walk_cons\"]"), "{error}");
    let start = text
        .find("theorem{\n  name: release_one_cell,")
        .expect("the last theorem");
    let end = start + text[start..].find("\n}\n").expect("its end") + 3;
    let without = format!("{}{}", &text[..start], &text[end..]);
    let error = complete(&without).expect_err("a law without a theorem");
    assert!(error.contains("missing [\"release_one_cell\"]"), "{error}");
    let block = &text[start..end];
    let other = text.replacen(block, &block.replace("releaseLoop", "lengthOf"), 1);
    assert_ne!(other, text, "the plant site exists");
    assert!(
        complete(&other).is_err(),
        "a theorem that states another function is refused"
    );

    if !support::lean_backed("SP-27") {
        return;
    }
    // The pinned Lean elaborates every theorem with the axioms its policy
    // states.
    let accepted = project_of(&[("RustLists", text.as_str())], &IMPORTED);
    let _ = support::verify_ok(&accepted);

    // A cons that writes the wrong slot, an uncons that reads the wrong slot,
    // a shared run written in place, a write that reaches the next buffer,
    // and a release given a step too few are each left unproved by Lean.
    for (what, from, to) in [
        (
            "a cons that writes the wrong slot",
            "      | Store.SCons c rest => (ctor Store.SCons (snoc (takeN c length) item) rest))",
            "      | Store.SCons c rest => (ctor Store.SCons (snoc c item) rest))",
        ),
        (
            "an uncons that reads the wrong slot",
            "      | Nat.zero => (RustMemory::get c slot)",
            "      | Nat.zero => (RustMemory::get c (slot + 1))",
        ),
        (
            "a shared run written in place",
            "        | RustMemory::Flag.no => (record World {store := store,",
            "        | RustMemory::Flag.no => (record World {store := (consRun store address (more + 1) item),",
        ),
        (
            "a write that reaches the next buffer",
            "(ctor Store.SCons c (consRun rest lower length item))",
            "(ctor Store.SCons (snoc c item) rest)",
        ),
        (
            "a release given a step too few",
            "((lengthOf rest) + 1)",
            "(lengthOf rest)",
        ),
    ] {
        assert_eq!(
            text.matches(from).count(),
            1,
            "{what}: the plant site occurs once"
        );
        let planted = text.replacen(from, to, 1);
        let refused = project_of(&[("RustLists", planted.as_str())], &IMPORTED);
        let error = refused.verify_fails_with("LLV7002").to_string();
        assert!(
            error.contains("unsolved goals") || error.contains("Type mismatch"),
            "{what}: Lean leaves a theorem unproved: {error}"
        );
    }
}
