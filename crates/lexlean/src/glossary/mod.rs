//! Generated external-Lean glossaries (SPEC.md §13.12): a lexicon package of
//! `kind = "lean"` entries derived from the declarations of a Lean
//! environment, with no hand-written entry.
//!
//! The work splits where the knowledge lives. A Lean metaprogram
//! ([`extract`]) reads the environment, because only Lean can say what a
//! declaration's type is, which binders are implicit, and which inductive
//! types have eliminators. The planner ([`plan`]) and the naming scheme
//! ([`naming`]) are Rust, so the package bytes, the diagnostics, and the
//! coverage report are fixed by this release and testable without Lean.

pub mod extract;
pub mod naming;
pub mod plan;

use std::collections::BTreeSet;

use camino::{Utf8Path, Utf8PathBuf};

use crate::code;
use crate::diagnostic::Diagnostic;
use crate::error::LexLeanError;
use crate::glossary::extract::{Extraction, Selection};
use crate::glossary::plan::{PackageSpec, Plan, Report};
use crate::lexicon::entry::is_lean_name;
use crate::lexicon::lse::is_package_id;

/// A `deps glossary` request.
#[derive(Debug, Clone)]
pub struct Request {
    /// The package identity the output carries.
    pub package: PackageSpec,
    /// The modules or declarations to translate.
    pub selection: Selection,
    /// The Lake workspace whose libraries the modules come from, or `None`
    /// for the toolchain's own (`Init`, `Std`, `Lean`).
    pub workspace: Option<Utf8PathBuf>,
    /// The package directory to create; it is absent or empty beforehand.
    pub out: Utf8PathBuf,
    /// Where to write the coverage report, if anywhere.
    pub report: Option<Utf8PathBuf>,
    /// Fail with the registered code of the first refusal instead of
    /// reporting it.
    pub strict: bool,
}

/// What a finished run produced.
#[derive(Debug, Clone)]
pub struct Generated {
    /// The coverage report.
    pub report: Report,
    /// The files written, relative to the working directory of the caller as
    /// given.
    pub artifacts: Vec<String>,
}

fn usage(message: impl Into<String>) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(code!("LLC0001"), message))
}

fn selection_error(message: impl Into<String>) -> LexLeanError {
    LexLeanError::from_diagnostic(Diagnostic::new(code!("LLC0002"), message))
}

/// Check everything about a request that needs no Lean.
///
/// # Errors
///
/// `LLC0002` for an empty or repeated selection, `LLC0001` for a malformed
/// package identity, language, module, name, or output directory.
pub fn validate(request: &Request) -> Result<(), LexLeanError> {
    let package = &request.package;
    if !is_package_id(&package.package) {
        return Err(usage(format!(
            "`{}` is not a valid package ID",
            package.package
        )));
    }
    if semver::Version::parse(&package.version).is_err() {
        return Err(usage(format!(
            "`{}` is not an exact version",
            package.version
        )));
    }
    if !crate::supports_language(&package.language) {
        return Err(usage(format!(
            "`{}` is not a language version this compiler supports",
            package.language
        )));
    }
    let (what, items) = match &request.selection {
        Selection::Modules(items) => ("module", items),
        Selection::Names(items) => ("declaration", items),
    };
    if items.is_empty() {
        return Err(selection_error(format!("no {what} is selected")));
    }
    let mut seen = BTreeSet::new();
    for item in items {
        if !is_lean_name(item) {
            return Err(usage(format!(
                "`{item}` is not a {what} name in the conservative ASCII Lean-name grammar"
            )));
        }
        if !seen.insert(item) {
            return Err(selection_error(format!(
                "{what} `{item}` is given more than once"
            )));
        }
    }
    match std::fs::read_dir(request.out.as_std_path()) {
        Ok(mut entries) => {
            if entries.next().is_some() {
                return Err(usage(format!(
                    "`{}` exists and is not empty; the output directory is absent or empty",
                    request.out
                )));
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(usage(format!(
                "`{}` is not a usable output directory: {error}",
                request.out
            )));
        }
    }
    if let Some(report) = &request.report {
        if report.as_std_path().exists() {
            return Err(usage(format!(
                "`{report}` exists; the report is not overwritten"
            )));
        }
    }
    Ok(())
}

/// Write a plan: the package directory, then the report. The package appears
/// in one rename, so a failure leaves no partial package.
fn write(
    plan: &Plan,
    out: &Utf8Path,
    report: Option<&Utf8Path>,
) -> Result<Vec<String>, Diagnostic> {
    let io = |what: &str, error: std::io::Error| {
        Diagnostic::new(code!("LLS8001"), format!("{what}: {error}"))
    };
    let parent = out
        .parent()
        .filter(|parent| !parent.as_str().is_empty())
        .unwrap_or_else(|| Utf8Path::new("."));
    std::fs::create_dir_all(parent.as_std_path())
        .map_err(|e| io("cannot create the parent directory", e))?;
    let staging = tempfile::Builder::new()
        .prefix(".lexlean-glossary-")
        .tempdir_in(parent.as_std_path())
        .map_err(|e| io("cannot create the staging directory", e))?;
    for (path, bytes) in &plan.files {
        let target = staging.path().join(path);
        if let Some(directory) = target.parent() {
            std::fs::create_dir_all(directory).map_err(|e| io("cannot create a directory", e))?;
        }
        std::fs::write(&target, bytes).map_err(|e| io("cannot write an entry", e))?;
    }
    let staged = staging.keep();
    if out.as_std_path().is_dir() {
        std::fs::remove_dir(out.as_std_path())
            .map_err(|e| io("cannot replace the empty output directory", e))?;
    }
    std::fs::rename(&staged, out.as_std_path()).map_err(|e| io("cannot publish the package", e))?;
    let mut artifacts = vec![out.to_string()];
    if let Some(report) = report {
        std::fs::write(report.as_std_path(), plan.report.to_bytes())
            .map_err(|e| io("cannot write the report", e))?;
        artifacts.push(report.to_string());
    }
    Ok(artifacts)
}

/// The diagnostics of a strict run: one per refused requested declaration,
/// each with its registered code.
fn refusal_diagnostics(report: &Report) -> Vec<Diagnostic> {
    report
        .refusals
        .iter()
        .filter(|refusal| refusal.requested)
        .map(|refusal| {
            let code = plan::code_of(&refusal.reason);
            let mut message = format!("`{}` is refused: {}", refusal.name, refusal.detail);
            if let (Some(root), Some(reason)) = (&refusal.root, &refusal.root_reason) {
                message.push_str(&format!(" (root `{root}`: {reason})"));
            }
            Diagnostic::new(code, message)
        })
        .collect()
}

/// Plan from an extraction and write the result. This is [`generate`] without
/// the Lean child, which the conformance cases and tests drive with committed
/// extractions.
///
/// # Errors
///
/// Request errors as [`validate`]; `LLS8001` when the output cannot be
/// written; the registered code of the first refusal when `strict`.
pub fn generate_from(
    request: &Request,
    extraction: &Extraction,
) -> Result<Generated, LexLeanError> {
    validate(request)?;
    let plan = plan::plan(extraction, &request.package).map_err(LexLeanError::from_diagnostics)?;
    if request.strict && plan.report.refused > 0 {
        return Err(
            LexLeanError::from_diagnostics(refusal_diagnostics(&plan.report))
                .bounded(256, "glossary"),
        );
    }
    let artifacts = write(&plan, &request.out, request.report.as_deref())
        .map_err(LexLeanError::from_diagnostic)?;
    Ok(Generated {
        report: plan.report,
        artifacts,
    })
}

/// Run the whole command: validate, extract under the pinned toolchain, plan,
/// and write.
///
/// # Errors
///
/// As [`generate_from`], and `LLV7001`, `LLV7002`, or `LLS8002` when the
/// toolchain, the extractor, or its limits fail.
pub fn generate(request: &Request) -> Result<Generated, LexLeanError> {
    generate_with(request, None)
}

/// [`generate`] with one more directory of compiled Lean modules on the
/// extractor's search path, for the repository's own cases.
///
/// # Errors
///
/// As [`generate`].
pub fn generate_with(
    request: &Request,
    lean_path: Option<&Utf8Path>,
) -> Result<Generated, LexLeanError> {
    validate(request)?;
    let extraction = extract::run_with(&request.selection, request.workspace.as_deref(), lean_path)
        .map_err(LexLeanError::from_diagnostic)?;
    generate_from(request, &extraction)
}
