//! The environment of certificate A (SPEC.md §17.17): the hand-written
//! preservation library and the calculus modules every certificate imports,
//! shipped as language data, and the workspace in which one project's
//! certificates compile beside its generated modules.
//!
//! The library is data rather than generated text because the metatheory it
//! carries (fuel monotonicity of the mutual evaluator, the compatibility of
//! every construct, the collection templates) quantifies over the
//! evaluator itself, which LexLean's proof language cannot state. Its
//! declarations' axioms are pinned per declaration in `library.toml`, so a
//! drifted library is caught by an exact comparison, not trusted.

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use super::certificate::Certificate;
use crate::code;
use crate::diagnostic::Diagnostic;

/// The library registry, hashed into the language-1.2 compiler semantics.
pub const LIBRARY_PATH: &str = "language/preservation-1.2/library.toml";

/// The registry's tag.
pub const LIBRARY_SPEC: &str = "lexlean/preservation-library/1";

/// Where the library's modules live, one file per module.
pub const LIBRARY_DIR: &str = "language/preservation-1.2/library";

/// Where the shipped copies of the calculus modules live. They are
/// byte-equal to the compiler project's golden modules (§17.14).
pub const MODULES_DIR: &str = "language/preservation-1.2/modules";

/// The calculus modules a certificate imports, in dependency order.
pub const TARGET_MODULES: [&str; 2] = [
    "LexLeanTarget.TargetSyntax",
    "LexLeanTarget.TargetSemantics",
];

/// Module roots the preservation environment owns. A project whose module
/// prefix starts with one would shadow the environment its certificates
/// are checked in.
pub const RESERVED_ROOTS: [&str; 3] = ["LexLeanTarget", "LexLeanPreservation", "LexLeanPreserve"];

/// The exact axioms of every certificate's root theorem: the library's
/// stability and compatibility lemmas use all three, and a certificate adds
/// none.
pub const CERTIFICATE_AXIOMS: [&str; 3] = ["Classical.choice", "Quot.sound", "propext"];

/// The parsed library registry.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Library {
    pub spec: String,
    pub toolchain: String,
    /// The library's modules in dependency order.
    pub modules: Vec<String>,
    /// Every declaration of the library with its exact axioms.
    pub declaration: Vec<LibraryDeclaration>,
}

/// One library declaration and its exact axioms.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryDeclaration {
    pub module: String,
    pub name: String,
    pub axioms: Vec<String>,
}

/// One file of a staged workspace, by its path below the workspace root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedFile {
    pub path: String,
    pub module: String,
    pub text: String,
}

/// The files a project's certificates compile in, and their order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    /// Every module source, in compilation order: the calculus modules,
    /// the library, the project's generated modules, the certificates.
    pub files: Vec<StagedFile>,
    /// The certificate modules, each replayed separately.
    pub certificates: Vec<String>,
    /// The audit module: it imports every certificate and the library and
    /// prints the axioms of each library declaration and root theorem.
    pub audit: StagedFile,
}

fn internal(reason: impl std::fmt::Display) -> Diagnostic {
    Diagnostic::new(code!("LLI9001"), format!("phase preservation: {reason}"))
}

fn embedded_text(path: &str) -> Result<&'static str, Diagnostic> {
    crate::embedded::FILES
        .iter()
        .find(|(candidate, _)| *candidate == path)
        .and_then(|(_, bytes)| std::str::from_utf8(bytes).ok())
        .ok_or_else(|| internal(format!("embedded `{path}` is missing")))
}

/// The path of a module's source below a workspace or data directory.
#[must_use]
pub fn module_path(module: &str) -> String {
    format!("{}.lean", module.replace('.', "/"))
}

/// The embedded library registry, checked for internal consistency: its
/// tag and toolchain, each declaration in a listed module, no name twice,
/// and every axiom list sorted.
///
/// # Errors
///
/// Returns `LLI9001` when the shipped registry is missing or malformed.
pub fn library() -> Result<Library, Diagnostic> {
    let text = embedded_text(LIBRARY_PATH)?;
    let library: Library =
        toml::from_str(text).map_err(|error| internal(format!("{LIBRARY_PATH}: {error}")))?;
    if library.spec != LIBRARY_SPEC {
        return Err(internal(format!(
            "{LIBRARY_PATH}: spec `{}` is not `{LIBRARY_SPEC}`",
            library.spec
        )));
    }
    if library.toolchain != crate::LEAN_TOOLCHAIN {
        return Err(internal(format!(
            "{LIBRARY_PATH}: toolchain `{}` is not the pinned `{}`",
            library.toolchain,
            crate::LEAN_TOOLCHAIN
        )));
    }
    let modules: BTreeSet<&str> = library.modules.iter().map(String::as_str).collect();
    let mut names = BTreeSet::new();
    for declaration in &library.declaration {
        if !modules.contains(declaration.module.as_str()) {
            return Err(internal(format!(
                "{LIBRARY_PATH}: `{}` names the unlisted module `{}`",
                declaration.name, declaration.module
            )));
        }
        if !names.insert(declaration.name.as_str()) {
            return Err(internal(format!(
                "{LIBRARY_PATH}: `{}` is listed twice",
                declaration.name
            )));
        }
        let mut sorted = declaration.axioms.clone();
        sorted.sort();
        sorted.dedup();
        if sorted != declaration.axioms {
            return Err(internal(format!(
                "{LIBRARY_PATH}: the axioms of `{}` are not sorted and unique",
                declaration.name
            )));
        }
    }
    Ok(library)
}

/// The shipped source of a library or calculus module.
///
/// # Errors
///
/// Returns `LLI9001` when the module is not shipped.
pub fn module_text(module: &str) -> Result<&'static str, Diagnostic> {
    let directory = if TARGET_MODULES.contains(&module) {
        MODULES_DIR
    } else {
        LIBRARY_DIR
    };
    embedded_text(&format!("{directory}/{}", module_path(module)))
}

/// The modules a Lean source imports.
fn imports(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| {
            line.strip_prefix("public import ")
                .or_else(|| line.strip_prefix("import "))
        })
        .map(|module| module.trim().to_owned())
        .collect()
}

/// The project's generated modules in dependency order, ties by name, so
/// the order is a function of the modules alone.
fn ordered(user: &[(String, String)]) -> Result<Vec<(String, String)>, Diagnostic> {
    let texts: BTreeMap<&str, &str> = user
        .iter()
        .map(|(module, text)| (module.as_str(), text.as_str()))
        .collect();
    let mut pending: BTreeMap<&str, BTreeSet<String>> = texts
        .iter()
        .map(|(module, text)| {
            (
                *module,
                imports(text)
                    .into_iter()
                    .filter(|import| texts.contains_key(import.as_str()))
                    .collect(),
            )
        })
        .collect();
    let mut out = Vec::new();
    while !pending.is_empty() {
        let ready: Vec<&str> = pending
            .iter()
            .filter(|(_, dependencies)| dependencies.is_empty())
            .map(|(module, _)| *module)
            .collect();
        if ready.is_empty() {
            return Err(internal(
                "the generated modules import each other cyclically",
            ));
        }
        for module in ready {
            pending.remove(module);
            for dependencies in pending.values_mut() {
                dependencies.remove(module);
            }
            out.push((module.to_owned(), texts[module].to_owned()));
        }
    }
    Ok(out)
}

/// The text of the audit module: it prints the axioms of every library
/// declaration and every certificate's root theorem.
fn audit_text(library: &Library, certificates: &[Certificate]) -> String {
    let mut text = String::new();
    for module in &library.modules {
        text.push_str(&format!("import {module}\n"));
    }
    for certificate in certificates {
        text.push_str(&format!("import {}\n", certificate.module));
    }
    for declaration in &library.declaration {
        text.push_str(&format!("#print axioms {}\n", declaration.name));
    }
    for certificate in certificates {
        text.push_str(&format!("#print axioms {}\n", certificate.theorem));
    }
    text
}

/// The audit module's name.
pub const AUDIT_MODULE: &str = "LexLeanPreserve.Audit";

/// Stage the workspace of a project's certificates.
///
/// # Errors
///
/// Returns `LLI9001` when the shipped environment is incomplete, the
/// generated modules import each other cyclically, or a certificate's
/// module lies outside the reserved `LexLeanPreserve` root.
pub fn workspace(
    user: &[(String, String)],
    certificates: &[Certificate],
) -> Result<Workspace, Diagnostic> {
    let library = library()?;
    let mut files = Vec::new();
    for module in TARGET_MODULES
        .iter()
        .map(|module| (*module).to_owned())
        .chain(library.modules.clone())
    {
        files.push(StagedFile {
            path: module_path(&module),
            text: module_text(&module)?.to_owned(),
            module,
        });
    }
    for (module, text) in ordered(user)? {
        if RESERVED_ROOTS
            .iter()
            .any(|root| module == *root || module.starts_with(&format!("{root}.")))
        {
            return Err(internal(format!(
                "the generated module `{module}` lies under a reserved root"
            )));
        }
        files.push(StagedFile {
            path: module_path(&module),
            module,
            text,
        });
    }
    for certificate in certificates {
        if !certificate.module.starts_with("LexLeanPreserve.") {
            return Err(internal(format!(
                "the certificate `{}` lies outside `LexLeanPreserve`",
                certificate.module
            )));
        }
        files.push(StagedFile {
            path: module_path(&certificate.module),
            module: certificate.module.clone(),
            text: certificate.text.clone(),
        });
    }
    Ok(Workspace {
        files,
        certificates: certificates
            .iter()
            .map(|certificate| certificate.module.clone())
            .collect(),
        audit: StagedFile {
            path: module_path(AUDIT_MODULE),
            module: AUDIT_MODULE.to_owned(),
            text: audit_text(&library, certificates),
        },
    })
}

/// The axioms Lean printed for each name, read from `#print axioms`
/// output.
fn printed_axioms(output: &str) -> BTreeMap<String, Vec<String>> {
    let mut found = BTreeMap::new();
    let mut rest = output;
    loop {
        let start = match rest.find('\'') {
            Some(start) => start,
            None => break,
        };
        let after = &rest[start + 1..];
        let end = match after.find('\'') {
            Some(end) => end,
            None => break,
        };
        let name = &after[..end];
        let tail = &after[end + 1..];
        if let Some(listed) = tail.strip_prefix(" depends on axioms: [") {
            let close = listed.find(']').unwrap_or(listed.len());
            let mut axioms: Vec<String> = listed[..close]
                .split(',')
                .map(|axiom| axiom.trim().to_owned())
                .filter(|axiom| !axiom.is_empty())
                .collect();
            axioms.sort();
            found.insert(name.to_owned(), axioms);
            rest = &listed[close..];
        } else if let Some(after_none) = tail.strip_prefix(" does not depend on any axioms") {
            found.insert(name.to_owned(), Vec::new());
            rest = after_none;
        } else {
            rest = tail;
        }
    }
    found
}

/// Compare the audit module's output with the registry and the
/// certificate axioms: every library declaration's axioms exactly as
/// registered, every root theorem's exactly [`CERTIFICATE_AXIOMS`].
///
/// # Errors
///
/// Returns the first disagreement.
pub fn audit(output: &str, certificates: &[Certificate]) -> Result<(), String> {
    let library = library().map_err(|diagnostic| diagnostic.message.clone())?;
    let printed = printed_axioms(output);
    for declaration in &library.declaration {
        match printed.get(&declaration.name) {
            Some(axioms) if *axioms == declaration.axioms => {}
            Some(axioms) => {
                return Err(format!(
                    "the library declaration `{}` depends on {axioms:?}, registered {:?}",
                    declaration.name, declaration.axioms
                ));
            }
            None => {
                return Err(format!(
                    "the audit printed no axioms for the library declaration `{}`",
                    declaration.name
                ));
            }
        }
    }
    let expected: Vec<String> = CERTIFICATE_AXIOMS
        .iter()
        .map(|axiom| (*axiom).to_owned())
        .collect();
    for certificate in certificates {
        match printed.get(&certificate.theorem) {
            Some(axioms) if *axioms == expected => {}
            Some(axioms) => {
                return Err(format!(
                    "the certificate theorem `{}` depends on {axioms:?}, not exactly {expected:?}",
                    certificate.theorem
                ));
            }
            None => {
                return Err(format!(
                    "the audit printed no axioms for the certificate theorem `{}`",
                    certificate.theorem
                ));
            }
        }
    }
    Ok(())
}
