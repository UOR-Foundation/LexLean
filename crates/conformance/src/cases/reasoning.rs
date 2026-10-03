//! The `reasoning` suite: RS-01..RS-14, language-1.2 reasoning machines
//! (SPEC.md §17.12): logics, inference rules, verifiers, reasoners, their
//! elaboration, generated theorems, runtime boundary, and production.

use crate::support::{self, P};

const EXAMPLE: &str = "reasoning";

/// Run the case for one RS conformance ID.
///
/// # Panics
///
/// Panics when the case's assertion fails, and for an unwired ID.
pub fn run(id: &str) {
    match id {
        "RS-01" => rs_01(),
        "RS-02" | "RS-03" | "RS-04" | "RS-05" | "RS-06" | "RS-07" | "RS-08" | "RS-09"
        | "RS-10" | "RS-11" | "RS-12" | "RS-13" | "RS-14" => example_links(),
        _ => panic!("no reasoning case {id}"),
    }
}

/// §17.12: the reasoning constructs belong to the closed schemas.
fn rs_01() {
    let module = support::schema("semantic-module-v2");
    let text = module.to_string();
    for kind in [
        "\"logic\"",
        "\"inference_rule\"",
        "\"verifier\"",
        "\"reasoner\"",
        "\"reasoning_failure\"",
    ] {
        assert!(text.contains(kind), "the module schema closes over {kind}");
    }
}

/// The committed reasoning example links.
fn example_links() {
    let project = P::copy_example(EXAMPLE);
    project.check_ok();
}
