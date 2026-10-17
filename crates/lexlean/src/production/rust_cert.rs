//! Certificate B (SPEC.md §17.17): the rendered crate of a lowered root
//! simulates the lowered program. The certificate is built as kernel-term
//! data by [`super::certificate::data_b`] and rendered by the one core
//! backend; this module is its interface to verification: the module names
//! and the certificate as the preservation workspace stages and audits it.

use crate::calculus::rust::ast::Crate;
use crate::calculus::Program;

pub use super::certificate::data_b::{RULES, SOUNDNESS_MODULE};

/// One certificate-B module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateB {
    /// The Lean module name.
    pub module: String,
    /// The module text, rendered from the data.
    pub text: String,
    /// The fully qualified name of the simulation theorem.
    pub theorem: String,
    /// The certificate as kernel-term data: a native core module.
    pub data: crate::ir::core::CoreModule,
    /// The record of the certificate: its authoring notation.
    pub source: String,
}

impl CertificateB {
    /// The certificate as the preservation workspace stages and audits it.
    #[must_use]
    pub fn into_certificate(self) -> super::certificate::Certificate {
        super::certificate::Certificate {
            module: self.module,
            denote: self.theorem.clone(),
            theorem: self.theorem,
            text: self.text,
            imports: Vec::new(),
            data: Some(self.data),
            source: Some(self.source),
        }
    }
}

/// The module of a root's certificate B in `target`, below its certificate
/// A module.
#[must_use]
pub fn module_for(certificate_a: &str, target: &str) -> Option<String> {
    let suffix = match target {
        "rust-core" => "RustCore",
        "rust-std" => "RustStd",
        _ => return None,
    };
    Some(format!("{certificate_a}.{suffix}"))
}

/// Certificate B of one root's program rendered as `krate`, as module
/// `module`.
///
/// The derivations are built under `max_bytes`, the largest file a
/// certificate may be: the work of a root whose certificate would exceed it
/// stops at about that size rather than at the size the root would have
/// taken.
///
/// # Errors
///
/// Returns the reason the aligner cannot relate the crate to the program:
/// a rendering no rule of the correspondence derives; or, prefixed with
/// [`super::lower::LIMIT`], that the certificate would exceed `max_bytes`.
pub fn certificate_b(
    program: &Program,
    krate: &Crate,
    module: &str,
    max_bytes: u64,
) -> Result<CertificateB, String> {
    let (data, text, source) =
        super::certificate::data_b::render(program, krate, module, max_bytes)?;
    Ok(CertificateB {
        module: module.to_owned(),
        theorem: format!("{module}.root"),
        text,
        data,
        source,
    })
}
