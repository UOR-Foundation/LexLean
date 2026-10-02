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

/// Tokens no library module or certificate may contain: each either
/// admits a statement without proof, trusts code the kernel does not
/// check, or changes how the source is read.
pub const FORBIDDEN_TOKENS: [&str; 27] = [
    "sorry",
    "admit",
    "axiom",
    "opaque",
    "unsafe",
    "partial",
    "native_decide",
    "kernel",
    "ofReduceBool",
    "ofReduceNat",
    "implemented_by",
    "extern",
    "macro",
    "macro_rules",
    "syntax",
    "elab",
    "elab_rules",
    "notation",
    "infix",
    "infixl",
    "infixr",
    "prefix",
    "postfix",
    "#eval",
    "run_cmd",
    "run_tac",
    "sorryAx",
];

/// The forbidden constants, rejected in any qualified spelling; the other
/// forbidden tokens are keywords and attributes, rejected as written, so a
/// user declaration that happens to share a keyword's spelling inside its
/// module's namespace is not one.
const FORBIDDEN_CONSTANTS: [&str; 3] = ["ofReduceBool", "ofReduceNat", "sorryAx"];

/// The options a library module or certificate may set: none weakens
/// what the kernel checks.
pub const ALLOWED_OPTIONS: [&str; 5] = [
    "autoImplicit",
    "maxRecDepth",
    "maxHeartbeats",
    "linter.unusedVariables",
    "linter.unusedSimpArgs",
];

/// `text` without its comments and string literals, so prose and data
/// never read as tokens.
fn code_only(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    let mut depth = 0usize;
    while let Some(character) = chars.next() {
        if depth > 0 {
            if character == '/' && chars.peek() == Some(&'-') {
                chars.next();
                depth += 1;
            } else if character == '-' && chars.peek() == Some(&'/') {
                chars.next();
                depth -= 1;
            }
            continue;
        }
        match character {
            '/' if chars.peek() == Some(&'-') => {
                chars.next();
                depth = 1;
                out.push(' ');
            }
            '-' if chars.peek() == Some(&'-') => {
                for skipped in chars.by_ref() {
                    if skipped == '\n' {
                        out.push('\n');
                        break;
                    }
                }
            }
            '"' => {
                let mut escaped = false;
                for skipped in chars.by_ref() {
                    if escaped {
                        escaped = false;
                    } else if skipped == '\\' {
                        escaped = true;
                    } else if skipped == '"' {
                        break;
                    }
                }
                out.push_str(" \"\" ");
            }
            other => out.push(other),
        }
    }
    out
}

/// Check one library module or certificate: no forbidden token, only
/// allowed options, and imports only of `imports`.
///
/// # Errors
///
/// Returns the first violation.
pub fn audit_tokens(text: &str, imports: &BTreeSet<String>) -> Result<(), String> {
    let code = code_only(text);
    let tokens: Vec<&str> = code
        .split(|character: char| {
            !(character.is_alphanumeric()
                || matches!(character, '_' | '\'' | '.' | '#' | '!' | '?'))
        })
        .filter(|token| !token.is_empty())
        .collect();
    for (index, token) in tokens.iter().enumerate() {
        if FORBIDDEN_TOKENS.contains(token) {
            return Err(format!("the forbidden token `{token}`"));
        }
        for segment in token.split('.') {
            if FORBIDDEN_CONSTANTS.contains(&segment) {
                return Err(format!("the forbidden constant `{segment}` (in `{token}`)"));
            }
        }
        if *token == "set_option" {
            let option = tokens.get(index + 1).copied().unwrap_or_default();
            if !ALLOWED_OPTIONS.contains(&option) {
                return Err(format!("the option `{option}` is not allowed"));
            }
        }
        if *token == "import" {
            let module = tokens.get(index + 1).copied().unwrap_or_default();
            if !imports.contains(module) {
                return Err(format!("the import of `{module}` is not allowed"));
            }
        }
    }
    Ok(())
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
    let mut environment: BTreeSet<String> = TARGET_MODULES
        .iter()
        .map(|module| (*module).to_owned())
        .collect();
    let mut files = Vec::new();
    for module in TARGET_MODULES
        .iter()
        .map(|module| (*module).to_owned())
        .chain(library.modules.clone())
    {
        let text = module_text(&module)?;
        if library.modules.contains(&module) {
            audit_tokens(text, &environment)
                .map_err(|reason| internal(format!("the library module `{module}`: {reason}")))?;
        }
        environment.insert(module.clone());
        files.push(StagedFile {
            path: module_path(&module),
            text: text.to_owned(),
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
    environment.extend(user.iter().map(|(module, _)| module.clone()));
    for certificate in certificates {
        audit_tokens(&certificate.text, &environment).map_err(|reason| {
            internal(format!(
                "the certificate `{}`: {reason}",
                certificate.module
            ))
        })?;
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
    while let Some(start) = rest.find('\'') {
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

/// The modules verification compiles to check a project's certificates, in
/// order: the calculus modules and the library (the environment), then the
/// certificates, then the audit module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage {
    pub environment: Vec<StagedFile>,
    pub certificates: Vec<StagedFile>,
    pub audit: StagedFile,
}

/// Stage a project's certificates for verification (§17.17): the shipped
/// environment, audited token by token, then each certificate, audited
/// against the environment and the project's generated `modules`.
///
/// # Errors
///
/// Returns `LLV7014` when the shipped environment fails its audit, and
/// `LLI9001` when a generated certificate does.
pub fn stage(modules: &[String], certificates: &[Certificate]) -> Result<Stage, Diagnostic> {
    let drift = |reason: String| {
        Diagnostic::new(
            code!("LLV7014"),
            format!("preservation environment: {reason}"),
        )
    };
    let library = library().map_err(|diagnostic| drift(diagnostic.message))?;
    let mut imports: BTreeSet<String> = TARGET_MODULES
        .iter()
        .map(|module| (*module).to_owned())
        .collect();
    let mut environment = Vec::new();
    for module in TARGET_MODULES
        .iter()
        .map(|module| (*module).to_owned())
        .chain(library.modules.clone())
    {
        let text = module_text(&module).map_err(|diagnostic| drift(diagnostic.message))?;
        if library.modules.contains(&module) {
            audit_tokens(text, &imports)
                .map_err(|reason| drift(format!("the library module `{module}`: {reason}")))?;
        }
        imports.insert(module.clone());
        environment.push(StagedFile {
            path: module_path(&module),
            text: text.to_owned(),
            module,
        });
    }
    imports.extend(modules.iter().cloned());
    let mut staged = Vec::new();
    for certificate in certificates {
        audit_tokens(&certificate.text, &imports).map_err(|reason| {
            internal(format!(
                "the certificate `{}`: {reason}",
                certificate.module
            ))
        })?;
        staged.push(StagedFile {
            path: module_path(&certificate.module),
            module: certificate.module.clone(),
            text: certificate.text.clone(),
        });
    }
    Ok(Stage {
        environment,
        certificates: staged,
        audit: StagedFile {
            path: module_path(AUDIT_MODULE),
            module: AUDIT_MODULE.to_owned(),
            text: audit_text(&library, certificates),
        },
    })
}

/// Classify the audit module's output: a library declaration whose axioms
/// differ from the registry is environment drift (`LLV7014`); a root
/// theorem whose axioms are not exactly [`CERTIFICATE_AXIOMS`] is a rejected
/// certificate (`LLV7013`).
///
/// # Errors
///
/// Returns the first disagreement.
pub fn classify_audit(output: &str, certificates: &[Certificate]) -> Result<(), Diagnostic> {
    audit(output, certificates).map_err(|reason| {
        if reason.starts_with("the library declaration") {
            Diagnostic::new(
                code!("LLV7014"),
                format!("preservation environment: {reason}"),
            )
        } else {
            Diagnostic::new(code!("LLV7013"), format!("certificate A: {reason}"))
        }
    })
}

/// One certified root as `preservation.json` records it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertifiedRoot {
    pub root: String,
    pub targets: Vec<String>,
    pub certificate: Certificate,
}

/// The tag of `preservation.json`.
pub const PRESERVATION_SPEC: &str = "lexlean/preservation/1";

/// `preservation.json`: the registry identity, and per root its targets,
/// certificate module, theorem, and certificate source identity.
#[must_use]
pub fn record(roots: &[CertifiedRoot]) -> crate::artifact::canonical_json::Json {
    use crate::artifact::canonical_json::Json;
    use crate::artifact::content_id::Sha256Digest;
    let registry = embedded_text(LIBRARY_PATH).unwrap_or_default();
    Json::object(vec![
        ("spec", Json::Str(PRESERVATION_SPEC.to_owned())),
        (
            "library",
            Json::object(vec![
                (
                    "registry_sha256",
                    Json::Str(Sha256Digest::of(registry.as_bytes()).to_hex()),
                ),
                (
                    "axioms",
                    Json::Arr(
                        CERTIFICATE_AXIOMS
                            .iter()
                            .map(|axiom| Json::Str((*axiom).to_owned()))
                            .collect(),
                    ),
                ),
            ]),
        ),
        (
            "roots",
            Json::Arr(
                roots
                    .iter()
                    .map(|root| {
                        Json::object(vec![
                            ("root", Json::Str(root.root.clone())),
                            (
                                "targets",
                                Json::Arr(root.targets.iter().cloned().map(Json::Str).collect()),
                            ),
                            ("module", Json::Str(root.certificate.module.clone())),
                            ("theorem", Json::Str(root.certificate.theorem.clone())),
                            ("byte_length", Json::from_usize(root.certificate.text.len())),
                            (
                                "sha256",
                                Json::Str(
                                    Sha256Digest::of(root.certificate.text.as_bytes()).to_hex(),
                                ),
                            ),
                        ])
                    })
                    .collect(),
            ),
        ),
    ])
}
