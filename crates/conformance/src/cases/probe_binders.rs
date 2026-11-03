//! VR-27: how the external-interface probe writes a signature (SPEC.md §18.8).
//!
//! The fixture is `tests/fixtures/probe-binders`: a project with entries
//! denoting `HXor.hXor`, which has an instance binder, and `UInt32.xor_self`,
//! whose signature has a numeral argument and a binder named `a`, as the type
//! parameter of `Eq` is. A defined glossary predicate binds the same name
//! around a numeral in an ordinary document declaration.

use crate::support::{self, P};

const PROJECT: &str = "fixtures/probe-binders/project";

/// VR-27: the instance binder is anonymous and the numeral is ascribed no
/// local, so pinned Lean elaborates the probe without output and verifies
/// the ordinary document that uses the defined value.
pub(super) fn probe() {
    let project = P::copy_tests_project(PROJECT);
    project.relock();
    let build = support::rendered(&project);
    let main = support::lean_text(&build, "Main");
    assert!(
        main.contains("(fun (x0 : Nat) => Eq x0 0)"),
        "the ordinary defined value leaves Eq's numeral type to inference:\n{main}"
    );
    assert!(
        !main.contains("(0 : x"),
        "the ordinary defined value never ascribes a numeral a local's type:\n{main}"
    );
    if !support::lean_backed("VR-27") {
        return;
    }
    // Ordinary verification reaches Nat and Nat.add here, not these two
    // otherwise unused signatures, so they keep their focused probe.
    let (probe, record) = support::probe_lean(
        &project,
        &["test.probe::h-xor.h-xor", "test.probe::xor-self"],
    );
    assert!(
        probe.text.contains(": {x0 : Type p0u} → {x1 : Type p0v} → {x2 : Type p0w} → [HXor.{p0u, p0v, p0w} x0 x1 x2] → "),
        "the instance binder is written without a name:\n{}",
        probe.text
    );
    assert!(
        !probe.text.contains("(0 : x"),
        "a numeral is never ascribed the type of a local:\n{}",
        probe.text
    );
    assert_eq!(
        record.exit_code, 0,
        "Lean elaborates the probe:\n{}\n{}",
        record.stdout, record.stderr
    );
    assert!(
        record.stdout.trim().is_empty() && record.stderr.trim().is_empty(),
        "and says nothing about it:\n{}\n{}",
        record.stdout,
        record.stderr
    );
    support::verify_ok(&project);
}
