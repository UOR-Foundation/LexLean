//! The boundary validators and the entry (SPEC.md §17.17).

use super::rows::Sig;
use super::Writer;
use crate::ir::semantic::SemanticType;

impl Writer<'_, '_> {
    /// The validators and entry of a root with invariant-carrying parameters.
    pub(super) fn boundary(
        &mut self,
        sig: &Sig,
        parameters: &[SemanticType],
    ) -> Result<(), String> {
        let _ = sig;
        self.gen.verify_boundary(parameters)?;
        if self.gen.checks().is_empty() && self.gen.entry_of().is_none() {
            return Ok(());
        }
        Err("boundary validators".to_owned())
    }
}
