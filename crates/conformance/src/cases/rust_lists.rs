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
const LAWS: [(&str, &[&str]); 43] = [
    (
        "push_run_abs",
        &["flagPick", "absRun", "put", "lessThan", "size"],
    ),
    ("run_uncons_abs", &["absRun", "unconsAbs", "get"]),
    ("run_last_abs", &["absRun", "unconsAbs", "get"]),
    ("append_run_abs", &["absRun", "append", "size"]),
    ("grow_run_abs", &["absRun", "growBuf", "append", "size"]),
    (
        "room_after_growth",
        &["lessThan", "size", "append", "blank"],
    ),
    ("grown_has_room", &["lessThan", "growBuf", "size"]),
    ("truncate_keeps_run", &["minNat", "plus"]),
    (
        "walk_cons",
        &[
            "runWalk", "consIn", "bufAt", "flagPick", "put", "minNat", "lessThan", "size",
        ],
    ),
    (
        "cons_frame",
        &["runWalk", "consIn", "bufAt", "sameNat", "flagPick"],
    ),
    (
        "full_is_capacity",
        &["isFull", "flipFlag", "lessThan", "minNat", "size"],
    ),
    (
        "walk_alloc",
        &["runWalk", "storeSnoc", "storeLen", "allocBuf"],
    ),
    (
        "alloc_frame",
        &["runWalk", "storeSnoc", "storeLen", "lessThan", "bufAt"],
    ),
    ("walk_uncons", &["runWalk", "unconsAbs", "headAt"]),
    ("walk_uncons_last", &["runWalk", "unconsAbs", "headAt"]),
    ("cons_cell_view", &["view", "consWorld", "absHandle"]),
    ("cons_cell_store", &["storeOf", "consWorld"]),
    ("cons_alloc_view", &["view", "consWorld", "absHandle"]),
    (
        "cons_alloc_store",
        &["storeOf", "consWorld", "storeSnoc", "allocBuf"],
    ),
    (
        "cons_view",
        &["view", "consWorld", "aloneOn", "inPlace", "flagPick"],
    ),
    ("cons_shared_view", &["view", "sharedCons", "absHandle"]),
    ("cons_shared_store", &["storeOf", "sharedCons"]),
    (
        "inplace_view",
        &[
            "view",
            "inPlace",
            "isFull",
            "consIn",
            "runWalk",
            "bufTruncate",
        ],
    ),
    (
        "inplace_frame",
        &["absHandle", "consIn", "holds", "bufAt", "absRun"],
    ),
    ("uncons_cell_abs", &["absHandle"]),
    ("uncons_cell_taken", &["unconsHandle"]),
    ("uncons_run_abs", &["absHandle", "unconsAbs", "headAt"]),
    ("uncons_run_taken", &["unconsHandle", "takenFrom", "headAt"]),
    ("uncons_last_abs", &["absHandle", "unconsAbs", "headAt"]),
    (
        "uncons_last_taken",
        &["unconsHandle", "takenFrom", "headAt"],
    ),
    ("same_with_itself", &["sameNat"]),
    ("holder_on_own_buffer", &["holds"]),
    ("holder_without_run", &["holds"]),
    ("own_blocks", &["notAlone", "sameNat"]),
    ("others_allow_above", &["notAlone", "sameNat", "plus"]),
    ("others_allow_below", &["notAlone", "sameNat", "plus"]),
    ("holder_blocks", &["aloneOn"]),
    ("holder_elsewhere_above", &["aloneOn", "plus"]),
    ("holder_elsewhere_below", &["aloneOn", "plus"]),
    ("release_ends", &["ends", "releaseLoop", "lengthOf"]),
    ("release_one_short", &["releaseLoop", "unshared"]),
    ("release_needs_fuel", &["releaseLoop", "unshared"]),
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
    let error = complete(&other).expect_err("a theorem that states another function");
    assert!(
        error.contains("`release_one_cell` does not state `releaseLoop`"),
        "{error}"
    );

    if !support::lean_backed("SP-27") {
        return;
    }
    // The pinned Lean elaborates every theorem with the axioms its policy
    // states.
    let accepted = project_of(&[("RustLists", text.as_str())], &IMPORTED);
    let _ = support::verify_ok(&accepted);

    // Each defect is applied alone to the finished module, and Lean must
    // refuse exactly the theorems it breaks, with the kind of diagnostic and,
    // for an unproved goal, the goal that stays open.
    for (what, theorems, kind, goal, from, to) in PLANTS {
        assert_eq!(
            text.matches(from).count(),
            1,
            "{what}: the plant site occurs once"
        );
        let planted = text.replacen(from, to, 1);
        let refused = project_of(&[("RustLists", planted.as_str())], &IMPORTED);
        let shown = format!("{:?}", refused.verify_fails_with("LLV7002"));
        assert!(
            shown.contains(kind) && shown.contains(goal),
            "{what}: Lean reports `{kind}` with `{goal}`: {shown}"
        );
        assert_eq!(
            refused_theorems(&shown, &planted),
            theorems.iter().map(|name| (*name).to_owned()).collect(),
            "{what}: the theorems Lean refuses: {shown}"
        );
    }
}

/// The defects of the module: what is planted, the theorems Lean refuses, the
/// kind of its diagnostic, a fragment of the goal it leaves open, and the
/// text replaced.
const PLANTS: [(&str, &[&str], &str, &str, &str, &str); 9] = [
    (
        "a cons that writes the wrong slot, the buffer not truncated to the run",
        &["walk_cons"],
        "unsolved goals",
        "⊢ absRun",
        "(RustMemory::bufPush (RustMemory::bufTruncate b length) item) b) rest)",
        "(RustMemory::bufPush b item) b) rest)",
    ),
    (
        "an uncons that reads the wrong slot",
        &["walk_uncons", "walk_uncons_last"],
        "Type mismatch",
        "headAt",
        "      | Nat.zero => (RustMemory::get (project b slots) slot)",
        "      | Nat.zero => (RustMemory::get (project b slots) (slot + 1))",
    ),
    (
        "a shared run written in place",
        &["cons_view"],
        "unsolved goals",
        "match inPlace store address (more + 1) item with",
        "        | RustMemory::Flag.no => (sharedCons store address more item)))",
        "        | RustMemory::Flag.no => (inPlace store address (more + 1) item)))",
    ),
    (
        "a write that reaches the next buffer",
        &["cons_frame"],
        "unsolved goals",
        "keepOr",
        "      | Nat.succ lower => (ctor Store.SCons b (consIn rest lower length item)))),",
        "      | Nat.succ lower => (ctor Store.SCons (keepOr (RustMemory::bufPush (RustMemory::bufTruncate b length) item) b) (consIn rest lower length item)))),",
    ),
    (
        "a release given a step too few, a chain counted one short",
        &["release_ends"],
        "unsolved goals",
        "releaseLoop (lengthOf rest)",
        "((lengthOf rest) + 1)",
        "(lengthOf rest)",
    ),
    (
        "a growth that drops the slots it grows",
        &["grow_run_abs", "grown_has_room"],
        "Type mismatch",
        "growBuf",
        "(RustMemory::append (project buf slots) (RustMemory::blank extra fill))",
        "(RustMemory::blank extra fill)",
    ),
    (
        "a fresh buffer that is not at the length of the store",
        &["cons_alloc_view"],
        "Type mismatch",
        "storeLen store",
        "(ctor Terminal.TRun (storeLen store) 0)",
        "(ctor Terminal.TRun 0 0)",
    ),
    (
        "a refusal that is the opposite of the capacity",
        &["full_is_capacity"],
        "unsolved goals",
        "⊢ RustMemory.Flag.yes = RustMemory.Flag.no",
        "    | Option.none => (ctor RustMemory::Flag.yes)\n    | Option.some got => (ctor RustMemory::Flag.no))",
        "    | Option.none => (ctor RustMemory::Flag.no)\n    | Option.some got => (ctor RustMemory::Flag.yes))",
    ),
    (
        "a holder that does not block the write",
        &["own_blocks"],
        "unsolved goals",
        "⊢ otherwise = RustMemory.Flag.no",
        "    | RustMemory::Flag.yes => (ctor RustMemory::Flag.no)\n    | RustMemory::Flag.no => otherwise)",
        "    | RustMemory::Flag.yes => otherwise\n    | RustMemory::Flag.no => otherwise)",
    ),
];

/// The theorems a diagnostic names, by the line of the module it points at:
/// each `line_start: LINE` of a span falls on the block of one theorem.
fn refused_theorems(shown: &str, module: &str) -> BTreeSet<String> {
    let lines: Vec<&str> = module.lines().collect();
    let mut refused = BTreeSet::new();
    for piece in shown.split("line_start: ").skip(1) {
        let digits: String = piece.chars().take_while(char::is_ascii_digit).collect();
        let line: usize = digits.parse().expect("a line number");
        let name = lines[line - 1..]
            .iter()
            .find_map(|text| text.strip_prefix("  name: "))
            .expect("a name after the block start");
        refused.insert(name.trim_end_matches(',').to_owned());
    }
    refused
}
