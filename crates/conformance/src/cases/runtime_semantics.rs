//! The meaning of the runtime's trait methods (SPEC.md §17.16): `SP-24`.
//!
//! `RuntimeSemantics` evaluates a `RuntimeSyntax` crate over the values of the
//! calculus, finding a trait method by the shape of its receiver. For each
//! value of the core profile's predicate, `RuntimeSupport` holds a theorem
//! that the evaluation of `equal` or `compare` in `supportCrate` is the
//! calculus primitive of the same name on the same values. The theorems are
//! listed here from the predicate itself, never from the module, so a theorem
//! the module lacks is a failure of this case.

use std::collections::BTreeSet;

use lexlean::calculus::IntKind;

use super::rust_refinement::{keeping, project_of};
use crate::support::{self, repo_root};

const SUPPORT: &str = "compiler/src/RuntimeSupport.lex.tex";
const SEMANTICS: &str = "compiler/src/RuntimeSemantics.lex.tex";

/// The modules the support module imports, transitively.
const IMPORTED: [&str; 5] = [
    "TargetSyntax",
    "TargetSemantics",
    "RuntimeSyntax",
    "RuntimeSemantics",
    "RuntimeSupport",
];

/// The scalars of the predicate: the Boolean and every width.
fn scalars() -> Vec<String> {
    std::iter::once("bool".to_owned())
        .chain(IntKind::ALL.iter().map(|kind| kind.name().to_owned()))
        .collect()
}

/// The theorems the predicate calls for: `equal` of every scalar and of
/// `Ordering`, `compare` of every scalar, and `compare` of every pair of two
/// scalars, each named for the function and the types it states.
fn expected() -> BTreeSet<String> {
    let scalars = scalars();
    let mut out: BTreeSet<String> = scalars.iter().map(|t| format!("equal_{t}")).collect();
    out.insert("equal_ordering".to_owned());
    out.extend(scalars.iter().map(|t| format!("compare_{t}")));
    for a in &scalars {
        for b in &scalars {
            out.insert(format!("compare_pair_{a}_{b}"));
        }
    }
    out
}

/// Each theorem of a module with the text of its statement.
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

/// Whether the module has exactly the theorems of the predicate, each stating
/// the run of the function it is named for against the primitive of that
/// name.
fn complete(text: &str) -> Result<(), String> {
    let stated = theorems(text);
    let named: BTreeSet<String> = stated.iter().map(|(name, _)| name.clone()).collect();
    if named.len() != stated.len() {
        return Err("a value of the predicate has two theorems".to_owned());
    }
    let expected = expected();
    if named != expected {
        return Err(format!(
            "the predicate calls for {} theorems, the module states {}: missing {:?}, unexpected {:?}",
            expected.len(),
            named.len(),
            expected.difference(&named).collect::<Vec<_>>(),
            named.difference(&expected).collect::<Vec<_>>(),
        ));
    }
    for (name, statement) in &stated {
        let function = if name.starts_with("equal_") { "equal" } else { "compare" };
        if !statement.contains(&format!("\"value\":\"{function}\""))
            || statement.matches(&format!("Prim.{function}")).count() != 1
        {
            return Err(format!(
                "`{name}` does not state the run of `{function}` against the primitive `{function}`"
            ));
        }
    }
    Ok(())
}

/// Run the case of `SP-24`. The module nests as deeply as its crate does, so
/// the case runs on a stack of its own.
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
    assert_eq!(id, "SP-24");
    let root = repo_root();
    let text = std::fs::read_to_string(root.join(SUPPORT).as_std_path()).expect(SUPPORT);
    assert!(
        std::fs::metadata(root.join(SEMANTICS).as_std_path()).is_ok(),
        "{SEMANTICS} exists"
    );
    complete(&text).expect("each value of the predicate has its theorem");

    // A theorem for a value outside the predicate, a value without a theorem,
    // and a theorem that states another primitive are each refused.
    let outside = text.replacen("name: equal_bool", "name: equal_unit", 1);
    assert_ne!(outside, text, "the plant site exists");
    let error = complete(&outside).expect_err("a value outside the predicate");
    assert!(error.contains("unexpected [\"equal_unit\"]"), "{error}");
    let start = text.find("theorem{\n  name: compare_u8,").expect("the theorem");
    let end = start + text[start..].find("\n}\n").expect("its end") + 3;
    let without = format!("{}{}", &text[..start], &text[end..]);
    let error = complete(&without).expect_err("a value without a theorem");
    assert!(error.contains("missing [\"compare_u8\"]"), "{error}");
    let block = &text[start..end];
    let other = text.replacen(block, &block.replace("Prim.compare", "Prim.equal"), 1);
    assert_ne!(other, text, "the plant site exists");
    let error = complete(&other).expect_err("a theorem that states another primitive");
    assert!(error.contains("`compare_u8` does not state"), "{error}");

    if !support::lean_backed("SP-24") {
        return;
    }
    // The pinned Lean elaborates every theorem with the axioms its policy
    // states.
    let accepted = project_of("RuntimeSupport", &IMPORTED, &text);
    let _ = support::verify_ok(&accepted);

    // An impl that returns another result, and an impl that is missing for a
    // value of the predicate, each leave the theorem of its value unproved.
    // One theorem is kept: the module's others take minutes to elaborate.
    let key_of_u8 = "\"Key\") (ctor RuntimeSyntax::RtType.fixed (ctor TargetSyntax::IntKind.u8))";
    let reversed = text.replacen(
        "(ctor RuntimeSyntax::RtExpr.method (ctor RuntimeSyntax::RtExpr.var \"self\") \"cmp\" (cons (ctor RuntimeSyntax::RtExpr.var \"other\")",
        "(ctor RuntimeSyntax::RtExpr.method (ctor RuntimeSyntax::RtExpr.var \"other\") \"cmp\" (cons (ctor RuntimeSyntax::RtExpr.var \"self\")",
        1,
    );
    assert_ne!(reversed, text, "the plant site exists");
    let missing = text.replacen(
        key_of_u8,
        "\"Key\") (ctor RuntimeSyntax::RtType.bool)",
        1,
    );
    assert_ne!(missing, text, "the plant site exists");
    for (what, planted, kept) in [
        ("an impl that returns another result", reversed, "compare_bool"),
        ("an impl that is missing", missing, "compare_u8"),
    ] {
        let refused = project_of("RuntimeSupport", &IMPORTED, &keeping(&planted, kept));
        let error = refused.verify_fails_with("LLV7002");
        assert!(
            error.to_string().contains("unsolved goals"),
            "{what}: Lean leaves the theorem unproved: {error}"
        );
    }
}
