//! The embedded text-laws library (SPEC.md §17.10, §17.12).
//!
//! The tokenizer's universal laws are LexLean source: one native
//! `lexlean/core-module/2` module of the embedded project
//! `language/text-1.2/library/`, whose Lean the compiler generates by the
//! ordinary rules, never reading it from a file it did not write. A module
//! that declares a text component states no law of its own — its
//! elaboration makes the generated module import this library, and the
//! definitions it elaborates to apply the library's functions — so no Lean
//! text is spliced for the laws and one sealed module serves every project.
//!
//! This file is the single place that names the library. `LM-13` checks the
//! name against the registry the compiler embeds, and the audit reads the
//! declarations and their exact axiom policies from the native data, so a
//! name that drifted from the data is a failure of the register rather than
//! a silently wrong import.

use std::collections::{BTreeMap, BTreeSet};
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

/// The import line a generated module carries for the laws module.
pub const IMPORT_LINE: &str = "public import LexLeanText.LexLeanText";

/// The Lean module every text declaration's elaboration imports, named once
/// so that swapping where the laws are proved touches this constant, its
/// registry row, and the emission site and nothing else.
#[must_use]
pub fn laws_module() -> &'static str {
    LAWS_MODULE
}

/// The declarations of the library a tokenizer's elaboration applies, by
/// their short names below [`PREFIX`] (SPEC.md §17.12, *Text components*).
/// Linking registers each function with its signature and names each lemma
/// in a generated proof; the loader refuses a native module that does not
/// declare every one of them.
pub mod names {
    /// `vocabSize : List String -> Nat`, the ids a tokenizer admits.
    pub const VOCAB_SIZE: &str = "vocabSize";
    /// `encodeWith : List String -> ByteArray -> List UInt32`, `longest_match`.
    pub const ENCODE_WITH: &str = "encodeWith";
    /// `encodeBytes : ByteArray -> List UInt32`, `byte_level`.
    pub const ENCODE_BYTES: &str = "encodeBytes";
    /// `decodeWith : List String -> List UInt32 -> Option ByteArray`.
    pub const DECODE_WITH: &str = "decodeWith";
    /// `encodeText : List String -> String -> List UInt32`, `longest_match`.
    pub const ENCODE_TEXT: &str = "encodeText";
    /// `encodeTextBytes : String -> List UInt32`, `byte_level`.
    pub const ENCODE_TEXT_BYTES: &str = "encodeTextBytes";
    /// `decodeText : List String -> List UInt32 -> Option String`.
    pub const DECODE_TEXT: &str = "decodeText";
    /// `allBelow : Nat -> List UInt32 -> Bool`, the `in_range` test.
    pub const ALL_BELOW: &str = "allBelow";
    /// The round-trip law under `longest_match`.
    pub const ROUNDTRIP: &str = "roundtrip";
    /// The round-trip law under `byte_level`.
    pub const ROUNDTRIP_BYTES: &str = "roundtrip_bytes";
    /// The range law under `longest_match`.
    pub const IN_RANGE: &str = "in_range";
    /// The range law under `byte_level`.
    pub const IN_RANGE_BYTES: &str = "in_range_bytes";
    /// The text round-trip law under `longest_match`.
    pub const TEXT_ROUNDTRIP: &str = "text_roundtrip";
    /// The text round-trip law under `byte_level`.
    pub const TEXT_ROUNDTRIP_BYTES: &str = "text_roundtrip_bytes";

    /// Every name above, each of which the native module must declare.
    pub const ALL: [&str; 14] = [
        VOCAB_SIZE,
        ENCODE_WITH,
        ENCODE_BYTES,
        DECODE_WITH,
        ENCODE_TEXT,
        ENCODE_TEXT_BYTES,
        DECODE_TEXT,
        ALL_BELOW,
        ROUNDTRIP,
        ROUNDTRIP_BYTES,
        IN_RANGE,
        IN_RANGE_BYTES,
        TEXT_ROUNDTRIP,
        TEXT_ROUNDTRIP_BYTES,
    ];
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

/// One library declaration and its exact axioms, read from the native data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryDeclaration {
    /// The module that declares it.
    pub module: String,
    /// Its full Lean name.
    pub name: String,
    /// Its exact axioms, sorted.
    pub axioms: Vec<String>,
}

/// One generated Lean file of the library, by its path below a source root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedFile {
    /// The path below the source root, `LexLeanText/LexLeanText.lean`.
    pub path: String,
    /// The Lean module name.
    pub module: String,
    /// The generated Lean.
    pub text: String,
}

/// What the native library renders to and declares, read once.
struct Native {
    /// The generated Lean of each library module.
    texts: BTreeMap<String, String>,
    /// The declarations a Lean command can name, with their exact axioms.
    declarations: Vec<LibraryDeclaration>,
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

/// The path of a module's source below a source root.
#[must_use]
pub fn module_path(module: &str) -> String {
    format!("{}.lean", module.replace('.', "/"))
}

/// Whether a generated module imports the laws module, so that a
/// verification must compile the library before it.
#[must_use]
pub fn imports_laws(lean_text: &str) -> bool {
    lean_text.lines().any(|line| line == IMPORT_LINE)
}

/// The Lean the compiler generates from one library module's native
/// source.
///
/// # Errors
///
/// Returns `LLI9001` when the module is not shipped or its data is
/// malformed.
pub fn lean_text(module: &str) -> Result<&'static str, Diagnostic> {
    native()?
        .texts
        .get(module)
        .map(String::as_str)
        .ok_or_else(|| internal(format!("the text library has no module `{module}`")))
}

/// Every declaration of the library a Lean command can name, with its exact
/// axioms, read from the native modules.
///
/// # Errors
///
/// Returns `LLI9001` when a native module is missing or malformed.
pub fn declarations() -> Result<&'static [LibraryDeclaration], Diagnostic> {
    native().map(|native| native.declarations.as_slice())
}

/// The library's generated Lean files, in registry order: what a
/// verification compiles before the modules that import the laws.
///
/// # Errors
///
/// Returns `LLI9001` when the registry or a native module is malformed.
pub fn stage() -> Result<Vec<StagedFile>, Diagnostic> {
    let library = library()?;
    library
        .modules
        .iter()
        .map(|module| {
            Ok(StagedFile {
                path: module_path(module),
                module: module.clone(),
                text: lean_text(module)?.to_owned(),
            })
        })
        .collect()
}

/// Check a native library read from `read` instead of the embedded one: the
/// conformance gate plants defects in the sources this way. Returns the
/// number of declarations a Lean command can name.
///
/// # Errors
///
/// Returns the first reason the library is refused.
pub fn check_native_sources(
    read: &dyn Fn(&str) -> Result<String, String>,
) -> Result<usize, String> {
    let library = registry().and_then(checked)?;
    build_native(&library, read).map(|native| native.declarations.len())
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

/// Build the native library from the registry and the native sources it
/// names: every module parsed, its imports closed over `Init` and the
/// earlier library modules, its Lean generated, its declarations collected,
/// and every name the elaboration applies found among them.
fn build_native(
    library: &Library,
    read: &dyn Fn(&str) -> Result<String, String>,
) -> Result<Native, String> {
    let mut texts = BTreeMap::new();
    let mut declarations = Vec::new();
    let mut seen_modules: BTreeSet<String> = BTreeSet::new();
    let mut names = BTreeSet::new();
    for module in &library.modules {
        let path = native_source_path(module)
            .ok_or_else(|| format!("{LIBRARY_PATH}: `{module}` is outside `{PREFIX}`"))?;
        let source = read(&path)?;
        let core = crate::ir::core::CoreModule::from_source(&source, "1.2")
            .map_err(|reason| format!("{path}: {reason}"))?;
        for import in &core.imports {
            if import != "Init" && !seen_modules.contains(import) {
                return Err(format!(
                    "{path}: imports `{import}`, which is neither `Init` nor an earlier library module"
                ));
            }
        }
        for row in &core.declarations {
            if row.generated {
                continue;
            }
            if !names.insert(row.name.clone()) {
                return Err(format!("{path}: `{}` is declared twice", row.name));
            }
            if !matches!(row.policy, crate::ir::core::CoreAxiomPolicy::Exact(_)) {
                return Err(format!(
                    "{path}: `{}` does not pin its axioms exactly",
                    row.name
                ));
            }
            if crate::ir::core::is_closed_lean_name(&row.name) {
                let mut axioms = row.policy.axioms().to_vec();
                axioms.sort();
                declarations.push(LibraryDeclaration {
                    module: module.clone(),
                    name: row.name.clone(),
                    axioms,
                });
            }
        }
        let text = crate::backend::core::lean_text(&core)
            .map_err(|diagnostic| format!("{path}: {}", diagnostic.message))?;
        texts.insert(module.clone(), text);
        seen_modules.insert(module.clone());
    }
    for name in names::ALL {
        let qualified = format!("{PREFIX}.{name}");
        if !names.contains(&qualified) {
            return Err(format!(
                "the text library does not declare `{qualified}`, which a tokenizer's elaboration applies"
            ));
        }
    }
    Ok(Native {
        texts,
        declarations,
    })
}

fn load_native() -> Result<Native, String> {
    let library = registry().and_then(checked)?;
    build_native(&library, &|path| embedded_text(path).map(str::to_owned))
}

fn native() -> Result<&'static Native, Diagnostic> {
    static NATIVE: OnceLock<Result<Native, String>> = OnceLock::new();
    NATIVE.get_or_init(load_native).as_ref().map_err(internal)
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
