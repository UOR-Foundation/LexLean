//! The meaning of the runtime's buffer, share, and replace primitives
//! (SPEC.md §17.17): `SP-28`.
//!
//! `RustPrims` reads a buffer as the list of its items, gives the primitives
//! their erased meaning on that list, and states what the runtime does with a
//! buffer, a clone, and a replaced place against it. Its theorems are accepted
//! by the pinned Lean. They are listed here, from the laws, never from the
//! module, so a law the module lacks is a failure of this case, and each
//! theorem is held to the functions it states.

use std::collections::BTreeSet;

use super::rust_refinement::project_of;
use crate::support::{self, repo_root};

const MODULE: &str = "compiler/src/RustPrims.lex.tex";

/// The modules the primitives module imports.
const IMPORTED: [&str; 3] = ["RustLists", "RustMemory", "TargetSyntax"];

/// Each law with the functions its statement must be about: a theorem of the
/// right name that states another function is refused. The four laws of the
/// calculus primitives name no function, so they carry the token of the
/// primitive their statement applies (`"operation":"append"`), which a
/// content-free restatement would lack.
const LAWS: [(&str, &[&str]); 21] = [
    ("prim_append_nil", &["operation\":\"append"]),
    ("prim_append_cons", &["operation\":\"append"]),
    ("prim_length_nil", &["operation\":\"length"]),
    ("prim_length_cons", &["operation\":\"length"]),
    ("first_of_front", &["firstOf", "append", "size", "itemsOf"]),
    ("first_of_snoc", &["firstOf", "append", "size", "itemsOf"]),
    (
        "first_of_take",
        &[
            "firstOf",
            "append",
            "minNat",
            "size",
            "takeItems",
            "itemsOf",
        ],
    ),
    ("length_of_items", &["itemsOf", "size"]),
    ("less_than_itself", &["lessThan"]),
    ("append_nothing", &["append"]),
    ("put_into_blank", &["put", "append", "blank", "size"]),
    ("new_reads", &["readBuf", "bufNew"]),
    (
        "push_reads",
        &[
            "readBuf",
            "pushGrow",
            "erasedPush",
            "itemsOf",
            "append",
            "size",
        ],
    ),
    (
        "truncate_reads",
        &["readBuf", "bufTruncate", "erasedTruncate", "itemsOf"],
    ),
    ("length_reads", &["erasedLength", "readBuf"]),
    ("clone_reads", &["absHandle", "cloneHandle"]),
    ("clone_holds", &["holds", "cloneHandle"]),
    ("clone_blocks", &["aloneOn", "cloneHandle"]),
    ("alone_without_others", &["aloneOn"]),
    ("replace_old_reads", &["absHandle", "replaceHandle"]),
    ("replace_now_reads", &["absHandle", "replaceHandle"]),
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

/// Run the case of `SP-28`. The module is small, but its reader is the one
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
    assert_eq!(id, "SP-28");
    let text = std::fs::read_to_string(repo_root().join(MODULE).as_std_path()).expect(MODULE);
    complete(&text).expect("each law has its theorem");

    // A theorem for another function, a law without a theorem, and a theorem
    // that states another function are each refused.
    let outside = text.replacen("name: new_reads,", "name: new_readings,", 1);
    assert_ne!(outside, text, "the plant site exists");
    let error = complete(&outside).expect_err("a theorem for another function");
    assert!(error.contains("unexpected [\"new_readings\"]"), "{error}");
    assert!(error.contains("missing [\"new_reads\"]"), "{error}");
    let start = text
        .find("theorem{\n  name: replace_now_reads,")
        .expect("the last theorem");
    let end = start + text[start..].find("\n}\n").expect("its end") + 3;
    let without = format!("{}{}", &text[..start], &text[end..]);
    let error = complete(&without).expect_err("a law without a theorem");
    assert!(error.contains("missing [\"replace_now_reads\"]"), "{error}");
    let block = &text[start..end];
    let other = text.replacen(block, &block.replace("replaceHandle", "cloneHandle"), 1);
    assert_ne!(other, text, "the plant site exists");
    let error = complete(&other).expect_err("a theorem that states another function");
    assert!(
        error.contains("`replace_now_reads` does not state `replaceHandle`"),
        "{error}"
    );

    if !support::lean_backed("SP-28") {
        return;
    }
    // The pinned Lean elaborates every theorem with the axioms its policy
    // states.
    let accepted = project_of(&[("RustPrims", text.as_str())], &IMPORTED);
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
        let refused = project_of(&[("RustPrims", planted.as_str())], &IMPORTED);
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

type Plant = (
    &'static str,
    &'static [&'static str],
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);

/// The defects of the module: what is planted, the theorems Lean refuses, the
/// kind of its diagnostic, a fragment of the goal it leaves open, and the
/// text replaced.
const PLANTS: [Plant; 7] = [
    (
        "a push that puts the item in front",
        &["push_reads"],
        "unsolved goals",
        "⊢ LexLeanRuntime.append (itemsOf front) [item] = item :: itemsOf front",
        "body: (prim append items (cons item (nil TargetSyntax::Value)) : (List TargetSyntax::Value))",
        "body: (cons item items)",
    ),
    (
        "a full buffer that is not grown",
        &["push_reads"],
        "unsolved goals",
        "⊢ firstOf front (RustMemory.size front) = LexLeanRuntime.append (itemsOf front) [item]",
        "    | Option.none => (RustLists::keepOr (RustMemory::bufPush (RustLists::growBuf buf ((project buf len) + 1) item) item) buf))",
        "    | Option.none => buf)",
    ),
    (
        "a truncation that keeps one too many",
        &["truncate_reads"],
        "unsolved goals",
        "⊢ takeItems (itemsOf front) keep = takeItems (itemsOf front) (keep + 1)",
        "body: (takeItems items count)",
        "body: (takeItems items (count + 1))",
    ),
    (
        "a clone that does not hold",
        &["clone_reads", "clone_holds", "clone_blocks"],
        "unsolved goals",
        "RustLists.holds address { spine := spine, base := RustLists.Terminal.TNil }",
        "  body: handle\n}",
        "  body: (record RustLists::Handle {spine := (project handle spine), base := (ctor RustLists::Terminal.TNil)})\n}",
    ),
    (
        "a replace that returns the new value",
        &["replace_old_reads", "replace_now_reads"],
        "unsolved goals",
        "⊢ RustLists.absHandle store value = RustLists.absHandle store place",
        "(record Replaced {old := place, now := value})",
        "(record Replaced {old := value, now := place})",
    ),
    (
        "a length that counts one more",
        &["length_reads"],
        "unsolved goals",
        "⊢ RustMemory.size front + 1 = RustMemory.size front",
        "body: (prim length items : Nat)",
        "body: ((prim length items : Nat) + 1)",
    ),
    (
        "a new buffer that is not empty",
        &["new_reads"],
        "unsolved goals",
        "⊢ firstOf (RustMemory.blank capacity fill) capacity = []",
        "(RustMemory::blank capacity fill), len := 0})",
        "(RustMemory::blank capacity fill), len := capacity})",
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
