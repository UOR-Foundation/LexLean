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

/// The calculus and Rust machine modules a certificate imports, in
/// dependency order.
pub const TARGET_MODULES: [&str; 4] = [
    "LexLeanTarget.TargetSyntax",
    "LexLeanTarget.TargetSemantics",
    "LexLeanTarget.RustSyntax",
    "LexLeanTarget.RustSemantics",
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
pub const ALLOWED_OPTIONS: [&str; 6] = [
    "autoImplicit",
    "maxRecDepth",
    "maxHeartbeats",
    "synthInstance.maxHeartbeats",
    "linter.unusedVariables",
    "linter.unusedSimpArgs",
];

/// One component of a Lean name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    /// The component, without its quotation.
    pub text: String,
    /// Whether it was written `«…»`.
    pub quoted: bool,
}

/// What a lexeme of Lean source is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexemeKind {
    /// A name: its components, joined by `.`.
    Ident(Vec<Segment>),
    /// A numeric literal.
    Number,
    /// A string, raw string, or character literal, whose content is data.
    Literal,
    /// `#` and the identifier after it: a command.
    Command(String),
    /// Any other character.
    Symbol(char),
}

/// A lexeme and where it stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexeme {
    /// What it is.
    pub kind: LexemeKind,
    /// The line it is on, from 0.
    pub line: usize,
    /// Whether nothing but whitespace and comments precede it on its line.
    pub first_on_line: bool,
    /// Where it starts, in characters of the text.
    pub start: usize,
}

/// `c` is a letter-like character that Lean reads as part of an identifier
/// (`Lean.isLetterLike`): Greek but for `λ`, `Π`, `Σ`, Coptic, polytonic
/// Greek, the letterlike symbols, and the script and fraktur capitals.
fn letter_like(c: char) -> bool {
    let n = c as u32;
    (0x3b1..=0x3c9).contains(&n) && n != 0x3bb
        || (0x391..=0x3a9).contains(&n) && n != 0x3a0 && n != 0x3a3
        || (0x3ca..=0x3fb).contains(&n)
        || (0x1f00..=0x1ffe).contains(&n)
        || (0x2100..=0x214f).contains(&n)
        || (0x1d49c..=0x1d59f).contains(&n)
}

fn sub_script(c: char) -> bool {
    let n = c as u32;
    (0x2080..=0x2089).contains(&n)
        || (0x2090..=0x209c).contains(&n)
        || (0x1d62..=0x1d6a).contains(&n)
}

/// Lean's `isIdFirst`.
fn id_first(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || letter_like(c)
}

/// Lean's `isIdRest`.
fn id_rest(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || matches!(c, '_' | '\'' | '!' | '?')
        || letter_like(c)
        || sub_script(c)
}

/// The lexemes of Lean source, as Lean's tokenizer reads it: comments (nested
/// block comments, doc comments) dropped; string literals, raw strings
/// `r#"…"#`, and character literals read as data; names as `isIdFirst` and
/// `isIdRest` bound them, with `«…»` components and `.` joining a component
/// only to one that follows; a number as its own lexeme (`0sorry` is `0` and
/// `sorry`); and `sorry.1` or `sorryλ` as `sorry` and what follows.
///
/// # Errors
///
/// Fails closed: returns the reason when a comment, string, raw string,
/// character literal, or quoted name is not closed or is not what it should
/// be, when an interpolated string (whose braces hold code) is met, or when a
/// control character stands in the text.
#[allow(clippy::too_many_lines)]
pub fn lex(text: &str) -> Result<Vec<Lexeme>, String> {
    let chars: Vec<char> = text.chars().collect();
    let mut out: Vec<Lexeme> = Vec::new();
    let mut at = 0usize;
    let mut line = 0usize;
    let mut first_on_line = true;
    let at_char = |at: usize| chars.get(at).copied();
    while let Some(c) = at_char(at) {
        match c {
            '\n' => {
                line += 1;
                first_on_line = true;
                at += 1;
                continue;
            }
            ' ' | '\t' | '\r' => {
                at += 1;
                continue;
            }
            _ => {}
        }
        if c.is_control() {
            return Err(format!(
                "a control character U+{:04X} in the text",
                c as u32
            ));
        }
        // Comments.
        if c == '-' && at_char(at + 1) == Some('-') {
            while at_char(at).is_some_and(|d| d != '\n') {
                at += 1;
            }
            continue;
        }
        if c == '/' && at_char(at + 1) == Some('-') {
            let mut depth = 1usize;
            at += 2;
            while depth > 0 {
                match (at_char(at), at_char(at + 1)) {
                    (None, _) => return Err("a comment is not closed".to_owned()),
                    (Some('/'), Some('-')) => {
                        depth += 1;
                        at += 2;
                    }
                    (Some('-'), Some('/')) => {
                        depth -= 1;
                        at += 2;
                    }
                    (Some('\n'), _) => {
                        line += 1;
                        at += 1;
                    }
                    _ => at += 1,
                }
            }
            continue;
        }
        let here = Lexeme {
            kind: LexemeKind::Number,
            line,
            first_on_line,
            start: at,
        };
        first_on_line = false;
        // Strings.
        if c == '"' {
            at += 1;
            loop {
                match at_char(at) {
                    None => return Err("a string is not closed".to_owned()),
                    Some('\\') => at += 2,
                    Some('"') => {
                        at += 1;
                        break;
                    }
                    Some('\n') => {
                        line += 1;
                        at += 1;
                    }
                    Some(_) => at += 1,
                }
            }
            out.push(Lexeme {
                kind: LexemeKind::Literal,
                ..here
            });
            continue;
        }
        // Character literals.
        if c == '\'' {
            let mut end = at + 1;
            match at_char(end) {
                Some('\\') => {
                    end += 2;
                    match at_char(end - 1) {
                        Some('x') => end += 2,
                        Some('u') => end += 4,
                        _ => {}
                    }
                }
                Some(d) if d != '\'' && d != '\n' => end += 1,
                _ => return Err("a character literal is not closed".to_owned()),
            }
            if at_char(end) != Some('\'') {
                return Err("a character literal is not closed".to_owned());
            }
            at = end + 1;
            out.push(Lexeme {
                kind: LexemeKind::Literal,
                ..here
            });
            continue;
        }
        // Numbers, by Lean's rules: `0x`, `0b`, and `0o` take the digits of
        // their radix, a decimal takes a fraction and an exponent, and the
        // token ends where they do, so what follows is another token
        // (`1e10axiom` is a number and the keyword `axiom`).
        if c.is_ascii_digit() {
            let radix = if c == '0' {
                match at_char(at + 1) {
                    Some('x' | 'X') => 16,
                    Some('b' | 'B') => 2,
                    Some('o' | 'O') => 8,
                    _ => 10,
                }
            } else {
                10
            };
            if radix == 10 {
                while at_char(at).is_some_and(|d| d.is_ascii_digit()) {
                    at += 1;
                }
                if at_char(at) == Some('.') && at_char(at + 1).is_some_and(|d| d.is_ascii_digit()) {
                    at += 1;
                    while at_char(at).is_some_and(|d| d.is_ascii_digit()) {
                        at += 1;
                    }
                }
                if matches!(at_char(at), Some('e' | 'E')) {
                    let mut end = at + 1;
                    if matches!(at_char(end), Some('+' | '-')) {
                        end += 1;
                    }
                    if !at_char(end).is_some_and(|d| d.is_ascii_digit()) {
                        return Err("a scientific literal has no exponent digits".to_owned());
                    }
                    while at_char(end).is_some_and(|d| d.is_ascii_digit()) {
                        end += 1;
                    }
                    at = end;
                }
            } else {
                at += 2;
                let digits = at;
                while at_char(at).is_some_and(|d| d.is_digit(radix)) {
                    at += 1;
                }
                if at == digits {
                    return Err("a number has no digits after its radix prefix".to_owned());
                }
            }
            out.push(here);
            continue;
        }
        // Commands.
        if c == '#' && at_char(at + 1).is_some_and(id_first) {
            let mut end = at + 1;
            while at_char(end).is_some_and(id_rest) {
                end += 1;
            }
            out.push(Lexeme {
                kind: LexemeKind::Command(chars[at..end].iter().collect()),
                ..here
            });
            at = end;
            continue;
        }
        // Names.
        if id_first(c) || c == '\u{ab}' {
            let mut segments = Vec::new();
            loop {
                if at_char(at) == Some('\u{ab}') {
                    let mut name = String::new();
                    at += 1;
                    loop {
                        match at_char(at) {
                            None | Some('\n') => {
                                return Err("a quoted name is not closed".to_owned())
                            }
                            Some('\u{bb}') => {
                                at += 1;
                                break;
                            }
                            Some(d) => {
                                name.push(d);
                                at += 1;
                            }
                        }
                    }
                    if name.is_empty() || !name.chars().all(|d| id_rest(d) || d == '.') {
                        return Err(format!("the quoted name `{name}` is not a name"));
                    }
                    segments.push(Segment {
                        text: name,
                        quoted: true,
                    });
                } else {
                    let start = at;
                    at += 1;
                    while at_char(at).is_some_and(id_rest) {
                        at += 1;
                    }
                    segments.push(Segment {
                        text: chars[start..at].iter().collect(),
                        quoted: false,
                    });
                }
                if at_char(at) == Some('.')
                    && at_char(at + 1).is_some_and(|d| id_first(d) || d == '\u{ab}')
                {
                    at += 1;
                    continue;
                }
                break;
            }
            // `r"…"` and `r#"…"#` are raw strings; `s!"…"` interpolates code.
            if let [only] = segments.as_slice() {
                if !only.quoted && only.text == "r" {
                    let mut hashes = 0usize;
                    while at_char(at + hashes) == Some('#') {
                        hashes += 1;
                    }
                    if at_char(at + hashes) == Some('"') {
                        at += hashes + 1;
                        loop {
                            match at_char(at) {
                                None => return Err("a raw string is not closed".to_owned()),
                                Some('"')
                                    if (0..hashes).all(|k| at_char(at + 1 + k) == Some('#')) =>
                                {
                                    at += 1 + hashes;
                                    break;
                                }
                                Some('\n') => {
                                    line += 1;
                                    at += 1;
                                }
                                Some(_) => at += 1,
                            }
                        }
                        out.push(Lexeme {
                            kind: LexemeKind::Literal,
                            ..here
                        });
                        continue;
                    }
                }
                if !only.quoted && only.text.ends_with('!') && at_char(at) == Some('"') {
                    return Err("an interpolated string holds code".to_owned());
                }
            }
            out.push(Lexeme {
                kind: LexemeKind::Ident(segments),
                ..here
            });
            continue;
        }
        at += 1;
        out.push(Lexeme {
            kind: LexemeKind::Symbol(c),
            ..here
        });
    }
    Ok(out)
}

/// The single unquoted word a lexeme is, if it is one.
fn word(lexeme: &Lexeme) -> Option<&str> {
    match &lexeme.kind {
        LexemeKind::Ident(segments) => match segments.as_slice() {
            [only] if !only.quoted => Some(only.text.as_str()),
            _ => None,
        },
        LexemeKind::Number
        | LexemeKind::Literal
        | LexemeKind::Command(_)
        | LexemeKind::Symbol(_) => None,
    }
}

/// The dotted name a lexeme is, if it is a name.
fn dotted(lexeme: Option<&Lexeme>) -> Option<String> {
    match &lexeme?.kind {
        LexemeKind::Ident(segments) => Some(
            segments
                .iter()
                .map(|segment| segment.text.as_str())
                .collect::<Vec<_>>()
                .join("."),
        ),
        LexemeKind::Number
        | LexemeKind::Literal
        | LexemeKind::Command(_)
        | LexemeKind::Symbol(_) => None,
    }
}

/// Check one library module or certificate: no forbidden token, only
/// allowed options, and imports only of `imports`. The text is read as Lean
/// reads it ([`lex`]): a forbidden constant is refused whether or not its
/// components are quoted, a forbidden keyword unless it is a quoted name
/// outside an attribute list (where it is a name, not a keyword), and text
/// that cannot be classified is refused.
///
/// # Errors
///
/// Returns the first violation.
pub fn audit_tokens(text: &str, imports: &BTreeSet<String>) -> Result<(), String> {
    let lexemes = lex(text)?;
    // The bracket depth inside `@[ … ]` and `attribute [ … ]`.
    let mut attribute = 0usize;
    // `@` or `attribute` has been read, and `[` opens the list.
    let mut pending = false;
    for (index, lexeme) in lexemes.iter().enumerate() {
        let next = lexemes.get(index + 1);
        let opens = matches!(next.map(|n| &n.kind), Some(LexemeKind::Symbol('[')));
        match &lexeme.kind {
            LexemeKind::Ident(segments) => {
                for segment in segments {
                    if FORBIDDEN_CONSTANTS.contains(&segment.text.as_str()) {
                        return Err(format!(
                            "the forbidden constant `{}` (in `{}`)",
                            segment.text,
                            dotted(Some(lexeme)).unwrap_or_default()
                        ));
                    }
                }
                if let [only] = segments.as_slice() {
                    if FORBIDDEN_TOKENS.contains(&only.text.as_str())
                        && (!only.quoted || attribute > 0)
                    {
                        return Err(format!("the forbidden token `{}`", only.text));
                    }
                }
                match word(lexeme) {
                    Some("attribute") if opens => pending = true,
                    Some("set_option") => {
                        let option = dotted(next).unwrap_or_default();
                        if !ALLOWED_OPTIONS.contains(&option.as_str()) {
                            return Err(format!("the option `{option}` is not allowed"));
                        }
                    }
                    Some("import") => {
                        let module = dotted(next).unwrap_or_default();
                        if !imports.contains(&module) {
                            return Err(format!("the import of `{module}` is not allowed"));
                        }
                    }
                    Some(_) | None => {}
                }
            }
            // Neither a library module nor a certificate holds a command that
            // prints or runs anything. Lean reads the longest token it knows
            // from a `#`, so `#evalIO` is `#eval` and a name, and `#printaxiom`
            // is `#print` and a keyword: the word after the `#` is not read
            // here, and every such command is refused.
            LexemeKind::Command(command) => {
                return Err(format!("the command `{command}`"));
            }
            LexemeKind::Symbol('@') if opens => pending = true,
            LexemeKind::Symbol('[') if pending => {
                pending = false;
                attribute = 1;
            }
            LexemeKind::Symbol('[') if attribute > 0 => attribute += 1,
            LexemeKind::Symbol(']') if attribute > 0 => attribute -= 1,
            LexemeKind::Number | LexemeKind::Literal | LexemeKind::Symbol(_) => {}
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
        environment.insert(certificate.module.clone());
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
        // A later certificate may import an earlier one: certificate E
        // imports the certificates A and B it composes.
        imports.insert(certificate.module.clone());
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
/// differ from the registry is environment drift (`LLV7014`); a theorem
/// whose axioms are not exactly [`CERTIFICATE_AXIOMS`] is a rejected
/// certificate A (`LLV7013`), B (`LLV7015`), or E (`LLV7016`).
///
/// # Errors
///
/// Returns the first disagreement.
pub fn classify_audit(
    output: &str,
    certificates: &[Certificate],
    renderings: &[Certificate],
    composed: &[Certificate],
) -> Result<(), Diagnostic> {
    let environment = |reason: String| {
        if reason.starts_with("the library declaration") {
            Some(Diagnostic::new(
                code!("LLV7014"),
                format!("preservation environment: {reason}"),
            ))
        } else {
            None
        }
    };
    audit(output, certificates).map_err(|reason| {
        environment(reason.clone()).unwrap_or_else(|| {
            Diagnostic::new(code!("LLV7013"), format!("certificate A: {reason}"))
        })
    })?;
    audit(output, renderings).map_err(|reason| {
        environment(reason.clone()).unwrap_or_else(|| {
            Diagnostic::new(code!("LLV7015"), format!("certificate B: {reason}"))
        })
    })?;
    audit(output, composed).map_err(|reason| {
        environment(reason.clone()).unwrap_or_else(|| {
            Diagnostic::new(code!("LLV7016"), format!("certificate E: {reason}"))
        })
    })
}

/// Certificate B of one rendering of a root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertifiedRendering {
    /// The target the crate realizes.
    pub target: String,
    /// The certificate, whose theorem is the simulation of every function.
    pub certificate: Certificate,
    /// Certificate E: certificates A and B composed, whose theorem is that
    /// the crate's root realizes the encoded source result.
    pub composed: Certificate,
    /// The rendered crate's text, which certificate B is about.
    pub crate_text: String,
    /// The function of the program a caller of this rendering invokes: the
    /// root's boundary entry when a parameter carries an invariant, else the
    /// root, function 0. It is the function certificate E states of the
    /// crate (`Rust.fnIdent`), and its Rust symbol is `f<function>`.
    pub entry: u64,
}

/// One certified root as `preservation.json` records it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertifiedRoot {
    pub root: String,
    pub targets: Vec<String>,
    pub certificate: Certificate,
    /// The canonical file bytes of the lowered program the certificates are
    /// about.
    pub program: Vec<u8>,
    /// Certificate B of the root's crate in each of its targets.
    pub renderings: Vec<CertifiedRendering>,
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
                                "program",
                                Json::object(vec![
                                    ("byte_length", Json::from_usize(root.program.len())),
                                    (
                                        "sha256",
                                        Json::Str(Sha256Digest::of(&root.program).to_hex()),
                                    ),
                                ]),
                            ),
                            (
                                "sha256",
                                Json::Str(
                                    Sha256Digest::of(root.certificate.text.as_bytes()).to_hex(),
                                ),
                            ),
                            (
                                "renderings",
                                Json::Arr(
                                    root.renderings
                                        .iter()
                                        .map(|rendering| {
                                            let certificate = &rendering.certificate;
                                            let composed = &rendering.composed;
                                            Json::object(vec![
                                                ("target", Json::Str(rendering.target.clone())),
                                                ("module", Json::Str(certificate.module.clone())),
                                                (
                                                    "entry",
                                                    Json::object(vec![
                                                        (
                                                            "function",
                                                            Json::from_usize(
                                                                usize::try_from(rendering.entry)
                                                                    .unwrap_or(usize::MAX),
                                                            ),
                                                        ),
                                                        (
                                                            "symbol",
                                                            Json::Str(format!(
                                                                "f{}",
                                                                rendering.entry
                                                            )),
                                                        ),
                                                    ]),
                                                ),
                                                (
                                                    "crate",
                                                    Json::object(vec![
                                                        (
                                                            "byte_length",
                                                            Json::from_usize(
                                                                rendering.crate_text.len(),
                                                            ),
                                                        ),
                                                        (
                                                            "sha256",
                                                            Json::Str(
                                                                Sha256Digest::of(
                                                                    rendering.crate_text.as_bytes(),
                                                                )
                                                                .to_hex(),
                                                            ),
                                                        ),
                                                    ]),
                                                ),
                                                ("theorem", Json::Str(certificate.theorem.clone())),
                                                (
                                                    "byte_length",
                                                    Json::from_usize(certificate.text.len()),
                                                ),
                                                (
                                                    "sha256",
                                                    Json::Str(
                                                        Sha256Digest::of(
                                                            certificate.text.as_bytes(),
                                                        )
                                                        .to_hex(),
                                                    ),
                                                ),
                                                (
                                                    "composed",
                                                    Json::object(vec![
                                                        (
                                                            "module",
                                                            Json::Str(composed.module.clone()),
                                                        ),
                                                        (
                                                            "theorem",
                                                            Json::Str(composed.theorem.clone()),
                                                        ),
                                                        (
                                                            "byte_length",
                                                            Json::from_usize(composed.text.len()),
                                                        ),
                                                        (
                                                            "sha256",
                                                            Json::Str(
                                                                Sha256Digest::of(
                                                                    composed.text.as_bytes(),
                                                                )
                                                                .to_hex(),
                                                            ),
                                                        ),
                                                    ]),
                                                ),
                                            ])
                                        })
                                        .collect(),
                                ),
                            ),
                        ])
                    })
                    .collect(),
            ),
        ),
    ])
}

/// The top-level declaration of a certificate's `text` that the first error
/// of Lean's `output` lies in: the relation, derivation, or composition a
/// rejection concerns.
#[must_use]
pub fn failing_declaration(text: &str, output: &str) -> Option<String> {
    let line: usize = output.lines().find_map(|line| {
        let (_, rest) = line.split_once(".lean:")?;
        let (number, rest) = rest.split_once(':')?;
        let (_, rest) = rest.split_once(": ")?;
        rest.starts_with("error").then(|| number.parse().ok())?
    })?;
    text.lines()
        .take(line)
        .filter(|line| line.starts_with("theorem ") || line.starts_with("def "))
        .last()
        .and_then(|line| line.split_whitespace().nth(1))
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::audit_tokens;

    fn audited(text: &str) -> Result<(), String> {
        audit_tokens(text, &BTreeSet::new())
    }

    /// A user's name spelled like a forbidden keyword is an identifier when
    /// the generator quotes it, as it does, and the audit still refuses the
    /// keyword itself, in a command, an attribute, or a binder.
    #[test]
    fn a_quoted_name_is_data_and_a_keyword_is_refused() {
        for word in ["kernel", "prefix", "macro", "syntax", "extern", "notation"] {
            assert!(
                audited(&format!("def f («{word}» : Nat) : Nat := «{word}»\n")).is_ok(),
                "{word} quoted"
            );
            assert!(
                audited(&format!("def f ({word} : Nat) : Nat := {word}\n")).is_err(),
                "{word} bare"
            );
            assert!(
                audited(&format!("{word} foo := 1\n")).is_err(),
                "{word} as a command"
            );
        }
        assert!(audited("@[implemented_by f] def g := 1\n").is_err());
        assert!(audited("theorem t : True := sorry\n").is_err());
        assert!(audited("def x := Lean.ofReduceBool\n").is_err());
    }

    /// Quoting a name does not change which name it is: a quoted forbidden
    /// constant, attribute, or namespace is still the forbidden one, and text
    /// that would hide the rest of the file from the audit is refused.
    #[test]
    fn a_quoted_forbidden_name_is_still_forbidden() {
        for text in [
            "def x := Lean.«ofReduceBool»\n",
            "theorem z : False := «sorryAx» False true\n",
            "@[«implemented_by» g] def h := 1\n",
            "@[«extern» \"c\"] def h := 1\n",
            "def x := «Lean».«ofReduceNat»\n",
            "@[«macro» foo] def h := 1\n",
            "@[simp, «implemented_by» g] def h := 1\n",
            "def x := «a\ntheorem t : False := sorry\n",
            "def «» := 1\n",
            "def «a b» := 1\n",
            "def s := \"unclosed\ntheorem t : False := sorry\n",
            "/- unclosed\ntheorem t : False := sorry\n",
        ] {
            assert!(audited(text).is_err(), "{text}");
        }
        assert!(audited("def «sorry» := 1\n").is_ok());
        assert!(audited("def f («kernel» : Nat) := «kernel» + 1\n").is_ok());
    }

    /// The text is read as Lean reads it: what a different reading would hide
    /// from the audit is read, and what cannot be classified is refused.
    #[test]
    fn the_text_is_read_as_lean_reads_it() {
        for text in [
            // A character literal holding a quote opens no string.
            "def c1 := '\"'\ntheorem t3 : False := sorry\ndef c2 := '\"'\n",
            // A raw string ends at its quote, whatever precedes it.
            "def s := r\"\\\"\ntheorem t : False := sorry\ndef u := r\"\\\"\n",
            "def s := r#\"a\"#\ntheorem t : False := sorry\n",
            // A name is a name where Lean ends it.
            "theorem t : False := sorry.1\n",
            "theorem t : False := 0sorry\n",
            "theorem t : False := sorry\u{3bb}x\n",
            "theorem t : False := sorry\u{e9}\n",
            "attribute [«implemented_by» g] h\n",
            "attribute [simp, «extern» c] h\n",
            // Code in an interpolated string, an unclosed literal, a control
            // character: refused rather than guessed.
            "def s := s!\"{sorry}\"\n",
            "def c := 'ab'\ntheorem t : False := sorry\n",
            "def c := '\n",
            "def x := 1\u{0}theorem t : False := sorry\n",
            "/- /- nested -/ theorem t : False := sorry\n",
            // A number ends where Lean ends it: the keyword or command that
            // follows is read as one.
            "def x : Float := 1e10axiom bad : False\ntheorem t : False := bad\n",
            // (the keyword in halves: the shipped crate's audit reads this file)
            concat!("def x := 1.5e3un", "safe def y := 1\n"),
            "def x := 1e10#evalIO.println \"pwn2\"\n",
            "def x := 0b1sorry\n",
            "def x := 0o7sorry\n",
            "def x := 1e\n",
            "def x := 1e+\n",
            "def x := 0x\n",
            // A command is read by its longest known token, so the rest of
            // the word may be a name or a keyword: every command is refused.
            "#evalIO.println \"pwned\"\n",
            "#evalpwned\n",
            "#printaxiom bad\n",
            "#eval 1\n",
        ] {
            assert!(audited(text).is_err(), "{text:?}");
        }
        for text in [
            "def s := \"a -- b /- c «d \\\" 'e\"\n",
            "def s := r\"a -- b\"\ndef t := r#\"a\"b\"#\n",
            "def c := ['a', '\\n', '\\'', '\\x41', '\\u0041', '\"']\n",
            "def x' := x'' + f' 1\n",
            "/-- a doc comment: sorry -/ def f := 1\n/- nested /- sorry -/ sorry -/\n-- sorry\n",
            "def «sorry» := 1\ndef x := sorry_ + sorryAlt + «sorry».1\n",
            "def α₁ := β₂ + 0x1F + 1.5 + 2e3\n",
            "def f := fun x => x.1.2 + Nat.succ' 1\n",
            "def s := \"\\u00e9\"\n",
            "def x := [1e10, 1E-3, 1.5e+3, 0xFF, 0b101, 0o17, 1e10 + 2, 0b101e]\n",
        ] {
            assert!(audited(text).is_ok(), "{text:?}: {:?}", audited(text));
        }
    }

    /// Every module the library ships is read, and audited, by the same
    /// lexer: the lexer follows Lean closely enough for the library's own
    /// source.
    #[test]
    fn the_shipped_library_audits() {
        super::workspace(&[], &[]).expect("the shipped environment audits");
    }
}
