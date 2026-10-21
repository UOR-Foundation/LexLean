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

/// The modules that hold the theorems about pairs, one per first scalar: the
/// nine of a first scalar elaborate in about a gigabyte, the eighty-one
/// together past the host's memory.
fn key_names() -> Vec<String> {
    scalars()
        .iter()
        .map(|t| {
            let mut chars = t.chars();
            let first = chars.next().expect("a name").to_uppercase().collect::<String>();
            format!("RuntimeKeys{first}{}", chars.as_str())
        })
        .collect()
}

/// The texts of those modules.
pub(super) fn key_modules() -> Vec<String> {
    key_names()
        .iter()
        .map(|name| {
            let path = repo_root().join(format!("compiler/src/{name}.lex.tex"));
            std::fs::read_to_string(path.as_std_path()).unwrap_or_else(|e| panic!("{name}: {e}"))
        })
        .collect()
}

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

/// Each theorem of the modules with the text of its statement.
fn theorems(texts: &[&str]) -> Vec<(String, String)> {
    texts
        .iter()
        .flat_map(|text| {
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
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Whether the modules have exactly the theorems of the predicate, each
/// stating the run of the function it is named for against the primitive of
/// that name.
fn complete(texts: &[&str]) -> Result<(), String> {
    let stated = theorems(texts);
    let named: BTreeSet<String> = stated.iter().map(|(name, _)| name.clone()).collect();
    if named.len() != stated.len() {
        return Err("a value of the predicate has two theorems".to_owned());
    }
    let expected = expected();
    if named != expected {
        return Err(format!(
            "the predicate calls for {} theorems, the modules state {}: missing {:?}, unexpected {:?}",
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
    let keys = key_modules();
    let all = |support: &str, keys: &[String]| -> Result<(), String> {
        let texts: Vec<&str> = std::iter::once(support)
            .chain(keys.iter().map(String::as_str))
            .collect();
        complete(&texts)
    };
    all(&text, &keys).expect("each value of the predicate has its theorem");

    // A theorem for a value outside the predicate, a value without a theorem,
    // and a theorem that states another primitive are each refused.
    let outside = text.replacen("name: equal_bool", "name: equal_unit", 1);
    assert_ne!(outside, text, "the plant site exists");
    let error = all(&outside, &keys).expect_err("a value outside the predicate");
    assert!(error.contains("unexpected [\"equal_unit\"]"), "{error}");
    let key = &keys[1];
    let start = key
        .find("theorem{\n  name: compare_pair_u8_u16,")
        .or_else(|| key.find("theorem{\n  name: compare_pair_"))
        .expect("a pair theorem");
    let end = start + key[start..].find("\n}\n").expect("its end") + 3;
    let dropped = key[start..end]
        .split("name: ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .expect("its name")
        .to_owned();
    let mut without = keys.clone();
    without[1] = format!("{}{}", &key[..start], &key[end..]);
    let error = all(&text, &without).expect_err("a value without a theorem");
    assert!(error.contains(&format!("missing [\"{dropped}\"]")), "{error}");
    let block = &key[start..end];
    let mut other = keys.clone();
    other[1] = key.replacen(block, &block.replace("Prim.compare", "Prim.equal"), 1);
    assert_ne!(other[1], *key, "the plant site exists");
    let error = all(&text, &other).expect_err("a theorem that states another primitive");
    assert!(error.contains(&format!("`{dropped}` does not state")), "{error}");

    if !support::lean_backed("SP-24") {
        return;
    }
    // The pinned Lean elaborates every theorem with the axioms its policy
    // states, each module apart.
    let names = key_names();
    let mut modules: Vec<(&str, &str)> = vec![("RuntimeSupport", text.as_str())];
    modules.extend(names.iter().map(String::as_str).zip(keys.iter().map(String::as_str)));
    let mut imported = IMPORTED.to_vec();
    imported.extend(names.iter().map(String::as_str));
    let accepted = project_of(&modules, &imported);
    let _ = support::verify_ok(&accepted);

    // An impl that returns another result, and an impl that is missing for a
    // value of the predicate, each leave the theorem of its value unproved.
    // One theorem is kept: the others take minutes to elaborate.
    let key_of_u8 = "\"Key\") (ctor RuntimeSyntax::RtType.fixed (ctor TargetSyntax::IntKind.u8))";
    let reversed = text.replacen(
        "(ctor RuntimeSyntax::RtExpr.method (ctor RuntimeSyntax::RtExpr.var \"self\") \"cmp\" (cons (ctor RuntimeSyntax::RtExpr.var \"other\")",
        "(ctor RuntimeSyntax::RtExpr.method (ctor RuntimeSyntax::RtExpr.var \"other\") \"cmp\" (cons (ctor RuntimeSyntax::RtExpr.var \"self\")",
        1,
    );
    assert_ne!(reversed, text, "the plant site exists");
    let missing = text.replacen(key_of_u8, "\"Key\") (ctor RuntimeSyntax::RtType.bool)", 1);
    assert_ne!(missing, text, "the plant site exists");
    for (what, planted, kept) in [
        ("an impl that returns another result", reversed, "compare_bool"),
        ("an impl that is missing", missing, "compare_u8"),
    ] {
        let kept_text = keeping(&planted, kept);
        let refused = project_of(&[("RuntimeSupport", kept_text.as_str())], &IMPORTED);
        let error = refused.verify_fails_with("LLV7002");
        assert!(
            error.to_string().contains("unsolved goals"),
            "{what}: Lean leaves the theorem unproved: {error}"
        );
    }
}
