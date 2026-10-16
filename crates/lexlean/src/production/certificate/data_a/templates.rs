//! Collection templates and key comparisons (SPEC.md §17.17.3).

use super::{Pf, Scope, Writer};
use crate::calculus::Expr;
use crate::ir::semantic::{SemanticTerm, SemanticType};

impl<'a> Writer<'_, 'a> {
    /// A template call.
    pub(super) fn template_call(
        &mut self,
        template: crate::calculus::library::Template,
        instance: &[SemanticType],
        arguments: &[SemanticTerm],
        types: &[SemanticType],
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        let _ = (template, instance, arguments, types, expr, scope);
        Err("collection templates".to_owned())
    }

    /// A key comparison.
    pub(super) fn less_than(
        &mut self,
        types: &[SemanticType],
        arguments: &[SemanticTerm],
        expr: &'a Expr,
        scope: &Scope,
    ) -> Result<Pf, String> {
        let _ = (types, arguments, expr, scope);
        Err("key comparisons".to_owned())
    }
}
