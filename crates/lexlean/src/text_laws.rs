//! The embedded text-laws library (SPEC.md §17.10, §17.12).
//!
//! The tokenizer's universal laws are LexLean source: one native
//! `lexlean/core-module/2` module of the embedded project
//! `language/text-1.2/library/`, whose Lean the compiler generates by the
//! ordinary rules, never reading it from a file it did not write. A module
//! that declares a text component states no law of its own — its
//! elaboration makes the generated module import this library — so no Lean
//! text is spliced for the laws and one sealed module serves every project.
//!
//! This file is the single place that names the library. `LM-13` checks the
//! name against the registry the compiler embeds, and the audit reads the
//! declarations and their exact axiom policies from the native data, so a
//! name that drifted from the data is a failure of the register rather than
//! a silently wrong import.

use std::sync::OnceLock;

use serde::Deserialize;

use crate::code;
use crate::diagnostic::Diagnostic;

/// The library's registry: its module list and toolchain pin, hashed into
/// the language-1.2 compiler semantics with the sources it names.
pub const LIBRARY_PATH: &str = "language/text-1.2/library.toml";

/// The registry's tag.
pub const LIBRARY_SPEC: &str = "lexlean/text-library/1";

/// The embedded project of the library; its native sources are in `src/`.
pub const LIBRARY_DIR: &str = "language/text-1.2/library";

/// The module prefix of the library project.
pub const PREFIX: &str = "LexLeanText";

/// The Lean module that states the text laws, and the sole import a text
/// declaration's elaboration adds to a generated module.
pub const LAWS_MODULE: &str = "LexLeanText.LexLeanText";

/// The Lean module every text declaration's elaboration imports, named once
/// so that swapping where the laws are proved touches this constant, its
/// registry row, and the emission site and nothing else.
#[must_use]
pub fn laws_module() -> &'static str {
    LAWS_MODULE
}

/// The parsed library registry.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Library {
    /// The registry's schema tag.
    pub spec: String,
    /// The pinned Lean toolchain the sources are proved under.
    pub toolchain: String,
    /// The library's modules in dependency order.
    pub modules: Vec<String>,
}

/// The library registry, read from the normative data the compiler embeds.
///
/// # Errors
///
/// Reports the embedded path, the schema tag, the toolchain pin, or a
/// registry that does not list the laws module.
pub fn library() -> Result<Library, Diagnostic> {
    static REGISTRY: OnceLock<Result<Library, String>> = OnceLock::new();
    REGISTRY
        .get_or_init(|| registry().and_then(checked))
        .clone()
        .map_err(internal)
}

/// The repository-relative path of one library module's native source.
#[must_use]
pub fn native_source_path(module: &str) -> Option<String> {
    let relative = module.strip_prefix(&format!("{PREFIX}."))?;
    Some(format!(
        "{LIBRARY_DIR}/src/{}.lex.tex",
        relative.replace('.', "/")
    ))
}

fn registry() -> Result<Library, String> {
    let text = embedded_text(LIBRARY_PATH)?;
    toml::from_str(text).map_err(|error| format!("{LIBRARY_PATH}: {error}"))
}

fn checked(library: Library) -> Result<Library, String> {
    if library.spec != LIBRARY_SPEC {
        return Err(format!(
            "{LIBRARY_PATH}: spec `{}` is not `{LIBRARY_SPEC}`",
            library.spec
        ));
    }
    if library.toolchain != crate::LEAN_TOOLCHAIN {
        return Err(format!(
            "{LIBRARY_PATH}: toolchain `{}` is not the pinned `{}`",
            library.toolchain,
            crate::LEAN_TOOLCHAIN
        ));
    }
    if !library.modules.iter().any(|module| module == LAWS_MODULE) {
        return Err(format!(
            "{LIBRARY_PATH}: the registry does not list `{LAWS_MODULE}`"
        ));
    }
    Ok(library)
}

fn embedded_text(path: &str) -> Result<&'static str, String> {
    crate::embedded::FILES
        .iter()
        .find(|(candidate, _)| *candidate == path)
        .and_then(|(_, bytes)| std::str::from_utf8(bytes).ok())
        .ok_or_else(|| format!("embedded `{path}` is missing"))
}

fn internal(reason: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(code!("LLI9001"), format!("phase text-laws: {reason}"))
}
