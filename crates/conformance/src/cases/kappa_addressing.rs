//! κ-addressing (SPEC.md §17.22, decision D7 of #110, #127): the `kappa`
//! project, whose label and canonical JSON are held to uor-addr as an oracle.

use crate::support::repo_root;

/// Run the case for one `KA` conformance ID.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub fn run(id: &str) {
    assert!(
        repo_root()
            .join("kappa/lexlean.toml")
            .as_std_path()
            .exists(),
        "{id}: the `kappa` project is not committed"
    );
}
