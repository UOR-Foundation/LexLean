//! A definition recurses in its generated Lean on the argument its source
//! declares (SPEC.md §17.12, §17.17): `SP-32`, #314.
//!
//! Certificate A reduces a structural definition on the declared argument, so
//! it holds only if Lean recursed on that argument. `paramsOf` of the fixture
//! is the case where Lean's own search would take another: it matches a list
//! written before the argument, and every recursive call decreases that list.
//! `countOf` has an earlier parameter it does not match, and `modelOf` is not
//! executable, so neither states an argument: their bytes, like those of every
//! committed definition, are what they were before the rule.

use crate::production_consumer::{self, AppRoot, Application};
use crate::support::{self, P};

/// The fixture's generated clause for `paramsOf`.
const STATED: &str = "termination_by structural _ types _ _ _ _ => types";

/// The generated Lean of the fixture's definition `name`: from its line to the
/// next declaration.
fn definition<'a>(lean: &'a str, name: &str) -> &'a str {
    let head = format!("public def {name} ");
    let start = lean
        .find(&head)
        .unwrap_or_else(|| panic!("`{name}` is generated"));
    let rest = &lean[start..];
    let end = rest[head.len()..]
        .find("public def ")
        .map_or(rest.len(), |next| next + head.len());
    &rest[..end]
}

/// `SP-32`.
///
/// # Panics
///
/// When the fixture's Lean or its export disagrees with the row.
pub fn run(id: &str) {
    assert_eq!(id, "SP-32");
    let project = P::copy_tests_project("fixtures/recursive-argument");
    project.relock();
    let lean = support::lean_text(&support::rendered(&project), "Main");
    assert!(
        definition(&lean, "paramsOf").contains(STATED),
        "`paramsOf` matches a parameter before its argument and states the argument:\n{}",
        definition(&lean, "paramsOf")
    );
    for control in ["countOf", "modelOf"] {
        assert!(
            !definition(&lean, control).contains("termination_by"),
            "`{control}` states no argument:\n{}",
            definition(&lean, control)
        );
    }
    if !support::lean_backed(id) {
        return;
    }
    let application = Application {
        roots: vec![AppRoot {
            root: "RecursiveArgument.Main.tailOf".to_owned(),
            name: "recursive_argument".to_owned(),
            version: "1.0.0".to_owned(),
            targets: vec!["rust-std".to_owned()],
        }],
    };
    let exported =
        production_consumer::export(&project.root.join("lexlean.toml"), &application, &[]);
    if let Err(error) = exported {
        panic!(
            "the kernel accepts every certificate of `tailOf`: {:?}",
            error.diagnostics
        );
    }
}
