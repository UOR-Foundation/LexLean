//! VR-25: how the external-interface probe writes a signature (SPEC.md §18.8).
//!
//! The fixture is `tests/fixtures/probe-binders`: a project whose two entries
//! denote `HXor.hXor`, which has an instance binder, and `UInt32.xor_self`,
//! whose signature has a numeral argument and a binder named `a`, as the type
//! parameter of `Eq` is.

use crate::support::{self, P};

const PROJECT: &str = "fixtures/probe-binders/project";

/// VR-25: the instance binder is anonymous and the numeral is ascribed no
/// local, so pinned Lean elaborates the probe without output.
pub(super) fn probe() {
    if !support::lean_backed("VR-25") {
        return;
    }
    let project = P::copy_tests_project(PROJECT);
    project.relock();
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
}
