//! The memory a runtime buffer lives in (SPEC.md §17.16): `SP-26`.
//!
//! `RustMemory` models a buffer as index-addressed slots, whose number is its
//! capacity, and a length. The laws of its writes, its guard, and its
//! truncation are theorems the pinned Lean accepts. The theorems are listed
//! here, from the laws, never from the module, so a law the module lacks is a
//! failure of this case, and each theorem is held to the functions it states.

use std::collections::BTreeSet;

use super::rust_refinement::project_of;
use crate::support::{self, repo_root};

const MODULE: &str = "compiler/src/RustMemory.lex.tex";

/// The modules the memory module imports.
const IMPORTED: [&str; 1] = ["TargetSyntax"];

/// Each law with the functions its statement must be about: a theorem of the
/// right name that states another function is refused.
const LAWS: [(&str, &[&str]); 16] = [
    ("size_put", &["size", "put"]),
    (
        "get_put_same",
        &["get", "put", "guardGet", "lessThan", "size"],
    ),
    ("get_put_below", &["get", "put", "plus"]),
    ("get_put_above", &["get", "put", "plus"]),
    ("size_blank", &["size", "blank"]),
    ("get_blank", &["get", "blank", "guardGet", "lessThan"]),
    ("no_room_at_capacity", &["lessThan", "plus", "size"]),
    ("room_before_slot", &["lessThan", "size", "append"]),
    ("push_lands_in_slot", &["get", "put", "size", "append"]),
    ("below_truncated", &["lessThan", "minNat", "both"]),
    ("plus_assoc", &["plus"]),
    (
        "push_at_capacity",
        &["bufPush", "none", "lessThan", "plus", "size"],
    ),
    ("put_in_slot", &["put", "size", "append"]),
    (
        "push_with_room",
        &["bufPush", "some", "put", "size", "append"],
    ),
    (
        "get_truncated",
        &[
            "bufGet",
            "bufTruncate",
            "guardGet",
            "lessThan",
            "minNat",
            "get",
        ],
    ),
    ("window_of_window", &["subWindow", "plus"]),
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

/// Run the case of `SP-26`. The module is small, but its reader is the one
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
    assert_eq!(id, "SP-26");
    let text = std::fs::read_to_string(repo_root().join(MODULE).as_std_path()).expect(MODULE);
    complete(&text).expect("each law has its theorem");

    // A theorem for another function, a law without a theorem, and a theorem
    // that states another function are each refused.
    let outside = text.replacen("name: size_blank,", "name: size_blanks,", 1);
    assert_ne!(outside, text, "the plant site exists");
    let error = complete(&outside).expect_err("a theorem for another function");
    assert!(error.contains("unexpected [\"size_blanks\"]"), "{error}");
    assert!(error.contains("missing [\"size_blank\"]"), "{error}");
    let start = text
        .find("theorem{\n  name: plus_assoc,")
        .expect("the last theorem");
    let end = start + text[start..].find("\n}\n").expect("its end") + 3;
    let without = format!("{}{}", &text[..start], &text[end..]);
    let error = complete(&without).expect_err("a law without a theorem");
    assert!(error.contains("missing [\"plus_assoc\"]"), "{error}");
    let block = &text[start..end];
    let other = text.replacen(block, &block.replace("(plus ", "(minNat "), 1);
    assert_ne!(other, text, "the plant site exists");
    let error = complete(&other).expect_err("a theorem that states another function");
    assert!(
        error.contains("`plus_assoc` does not state `plus`"),
        "{error}"
    );

    if !support::lean_backed("SP-26") {
        return;
    }
    // The pinned Lean elaborates every theorem with the axioms its policy
    // states.
    let accepted = project_of(&[("RustMemory", text.as_str())], &IMPORTED);
    let _ = support::verify_ok(&accepted);

    // A write that lands elsewhere, a write that drops a slot, an order of
    // indices that is reversed, and a truncation that keeps one too many are
    // each left unproved by Lean.
    for (what, goal, from, to) in [
        (
            "a write that lands elsewhere",
            "⊢ some head = some item",
            "      | Nat.zero => (ctor Cells.CCons item tail)\n",
            "      | Nat.zero => (ctor Cells.CCons head tail)\n",
        ),
        (
            "a write that drops a slot",
            "⊢ size tail = size tail + 1",
            "(ctor Cells.CCons head (put tail lower item))",
            "(put tail lower item)",
        ),
        (
            "an order of indices that is reversed",
            "⊢ some item = none",
            "      | Nat.zero => (ctor Flag.yes)\n      | Nat.succ lower => (lessThan lower more)",
            "      | Nat.zero => (ctor Flag.no)\n      | Nat.succ lower => (lessThan lower more)",
        ),
        (
            "a push that adds two to the length",
            "⊢ some { slots := append front (Cells.CCons item back), len := size front + 1 + 1 } =",
            "len := ((project buf len) + 1)}))\n    | Flag.no",
            "len := (((project buf len) + 1) + 1)}))\n    | Flag.no",
        ),
        (
            "a truncation that keeps one too many",
            "⊢ lessThan lower (minNat less oldLess + 1)",
            "((minNat more less) + 1)",
            "(((minNat more less) + 1) + 1)",
        ),
    ] {
        assert_eq!(
            text.matches(from).count(),
            1,
            "{what}: the plant site occurs once"
        );
        let planted = text.replacen(from, to, 1);
        let refused = project_of(&[("RustMemory", planted.as_str())], &IMPORTED);
        let error = refused.verify_fails_with("LLV7002");
        let shown = format!("{error:?}");
        assert!(
            shown.contains("unsolved goals") && shown.contains(goal),
            "{what}: Lean leaves the goal `{goal}` unproved: {shown}"
        );
    }
}
