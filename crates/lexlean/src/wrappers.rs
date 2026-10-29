//! Specialisation wrappers (SPEC.md §13.13): the host-library declarations
//! that let a class-polymorphic Lean declaration be denoted by a glossary
//! entry at a concrete type.
//!
//! A *request* (TOML, `lexlean/wrappers/1`) lists, for each wrapper, a Lean
//! declaration, its kind, a tag, and the explicit arguments of the
//! application `@declaration arguments` that specialises it. [`emit`] writes
//! the host module: one `public` declaration per wrapper whose type is
//! `type_of%` the application and whose value is the application.
//!
//! The emitter never runs Lean and never writes a signature. What the
//! wrapper's type is, Lean decides when the host library is built; whether
//! an entry's signature is that type, the verification probe decides
//! (SPEC.md §18.8). The only text that reaches the host module is a closed
//! term grammar printed with every name `_root_`-qualified, so a request
//! cannot smuggle a tactic, a command, or a comment into it.

use std::collections::BTreeSet;

use serde::Deserialize;

use crate::code;
use crate::diagnostic::Diagnostic;
use crate::lexicon::entry::is_lean_name;

/// The schema tag of a request file.
pub const REQUEST_SPEC: &str = "lexlean/wrappers/1";

/// The Lean namespace reserved for wrappers. The verification probe reads a
/// name under it as a specialisation (SPEC.md §18.8), so nothing else may
/// declare there.
pub const WRAPPER_ROOT: &str = "LexLeanSpec";

/// The deepest nesting of parentheses an argument may have. A request is
/// user input, so the parser is bounded rather than trusting the stack.
const MAX_TERM_DEPTH: usize = 64;

/// Words that are Lean syntax and not constants. `_root_` qualification
/// already makes them unparseable as the names they look like; they are
/// refused outright so that a request that spells a tactic block says so.
const TACTIC_WORDS: [&str; 22] = [
    "by", "fun", "do", "let", "have", "show", "from", "match", "with", "if", "then", "else",
    "calc", "sorry", "admit", "axiom", "theorem", "def", "open", "end", "at", "in",
];

/// Whether `lean_name` is a wrapper name: strictly under [`WRAPPER_ROOT`].
#[must_use]
pub fn is_wrapper_name(lean_name: &str) -> bool {
    lean_name
        .strip_prefix(WRAPPER_ROOT)
        .is_some_and(|rest| rest.starts_with('.') && rest.len() > 1)
}

/// How a wrapper is declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapperKind {
    /// `public theorem`: the declaration is a proof.
    Theorem,
    /// `public def`: the declaration is a definition.
    Def,
}

impl WrapperKind {
    fn keyword(self) -> &'static str {
        match self {
            Self::Theorem => "theorem",
            Self::Def => "def",
        }
    }
}

/// One explicit argument of the specialising application.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Term {
    /// `_`: left to unification or instance resolution.
    Hole,
    /// A constant, with `@` when its implicit arguments are written out.
    Name { explicit: bool, name: String },
    /// A decimal literal.
    Number(String),
    /// A parenthesised application of two or more terms, or one term.
    Group(Vec<Term>),
}

impl Term {
    fn print(&self, out: &mut String) {
        match self {
            Self::Hole => out.push('_'),
            Self::Name { explicit, name } => {
                if *explicit {
                    out.push('@');
                }
                out.push_str("_root_.");
                out.push_str(name);
            }
            Self::Number(digits) => out.push_str(digits),
            Self::Group(items) => {
                out.push('(');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(' ');
                    }
                    item.print(out);
                }
                out.push(')');
            }
        }
    }
}

/// One validated wrapper of a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wrapper {
    declaration: String,
    kind: WrapperKind,
    tag: String,
    arguments: Vec<Term>,
}

impl Wrapper {
    /// The wrapper's Lean name: `LexLeanSpec.<tag>.<declaration>`.
    #[must_use]
    pub fn lean_name(&self) -> String {
        format!("{WRAPPER_ROOT}.{}.{}", self.tag, self.declaration)
    }

    /// The specialised application, `@declaration arguments`, as printed
    /// into the host module.
    fn application(&self) -> String {
        let mut text = format!("@_root_.{}", self.declaration);
        for argument in &self.arguments {
            text.push(' ');
            argument.print(&mut text);
        }
        text
    }

    fn declaration_text(&self) -> String {
        let application = self.application();
        format!(
            "public {} {} : type_of% ({application}) := {application}",
            self.kind.keyword(),
            self.lean_name()
        )
    }
}

/// A validated specialisation request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    imports: Vec<String>,
    wrappers: Vec<Wrapper>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRequest {
    spec: String,
    imports: Vec<String>,
    #[serde(default)]
    wrapper: Vec<RawWrapper>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWrapper {
    declaration: String,
    kind: String,
    tag: String,
    arguments: Vec<String>,
}

fn refusal(message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code!("LLT4018"), message)
}

fn is_plain_name(text: &str) -> bool {
    is_lean_name(text) && text.split('.').all(|segment| segment != "_root_")
}

fn is_tag(text: &str) -> bool {
    is_plain_name(text) && !text.contains('.')
}

/// Parse and validate a request (SPEC.md §13.13).
///
/// # Errors
///
/// `LLT4018` for malformed TOML, an unknown field or schema, an invalid
/// import, name, tag, kind, or argument, a duplicate wrapper, or a request
/// with no wrapper.
pub fn parse_request(text: &str) -> Result<Request, Diagnostic> {
    let raw: RawRequest = toml::from_str(text)
        .map_err(|error| refusal(format!("the specialisation request is not valid: {error}")))?;
    if raw.spec != REQUEST_SPEC {
        return Err(refusal(format!(
            "the specialisation request has spec `{}`, expected `{REQUEST_SPEC}`",
            raw.spec
        )));
    }
    let mut imports = BTreeSet::new();
    for import in &raw.imports {
        if !is_plain_name(import) {
            return Err(refusal(format!(
                "import `{import}` is not a conservative ASCII Lean module name"
            )));
        }
        if !imports.insert(import.clone()) {
            return Err(refusal(format!("import `{import}` is listed twice")));
        }
    }
    if raw.imports.iter().collect::<Vec<_>>() != imports.iter().collect::<Vec<_>>() {
        return Err(refusal("the imports are not sorted"));
    }
    if raw.wrapper.is_empty() {
        return Err(refusal("the specialisation request has no wrapper"));
    }
    let mut seen = BTreeSet::new();
    let mut wrappers = Vec::with_capacity(raw.wrapper.len());
    for row in raw.wrapper {
        if !is_plain_name(&row.declaration) {
            return Err(refusal(format!(
                "declaration `{}` is not a conservative ASCII Lean name",
                row.declaration
            )));
        }
        if !is_tag(&row.tag) {
            return Err(refusal(format!(
                "tag `{}` is not an ASCII identifier",
                row.tag
            )));
        }
        let kind = match row.kind.as_str() {
            "theorem" => WrapperKind::Theorem,
            "def" => WrapperKind::Def,
            other => {
                return Err(refusal(format!(
                    "kind `{other}` is neither `theorem` nor `def`"
                )))
            }
        };
        let mut arguments = Vec::with_capacity(row.arguments.len());
        for argument in &row.arguments {
            arguments.push(parse_term(argument).map_err(|reason| {
                refusal(format!(
                    "argument `{argument}` of `{}` at `{}` is outside the term grammar: {reason}",
                    row.declaration, row.tag
                ))
            })?);
        }
        let wrapper = Wrapper {
            declaration: row.declaration,
            kind,
            tag: row.tag,
            arguments,
        };
        if !seen.insert(wrapper.lean_name()) {
            return Err(refusal(format!(
                "`{}` is requested twice",
                wrapper.lean_name()
            )));
        }
        wrappers.push(wrapper);
    }
    Ok(Request {
        imports: imports.into_iter().collect(),
        wrappers,
    })
}

/// Parse one argument: `term = "_" | name | "@" name | decimal | "(" term {term} ")"`.
fn parse_term(text: &str) -> Result<Term, String> {
    let mut parser = TermParser {
        bytes: text.as_bytes(),
        at: 0,
    };
    let term = parser.term(0)?;
    if parser.at != parser.bytes.len() {
        return Err(format!("unexpected `{}`", parser.rest()));
    }
    Ok(term)
}

struct TermParser<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl TermParser<'_> {
    fn rest(&self) -> String {
        String::from_utf8_lossy(&self.bytes[self.at..]).into_owned()
    }

    fn skip_spaces(&mut self) {
        while self.bytes.get(self.at) == Some(&b' ') {
            self.at += 1;
        }
    }

    fn term(&mut self, depth: usize) -> Result<Term, String> {
        if depth > MAX_TERM_DEPTH {
            return Err(format!("nesting is deeper than {MAX_TERM_DEPTH}"));
        }
        self.skip_spaces();
        match self.bytes.get(self.at) {
            None => Err("a term is missing".to_owned()),
            Some(b'(') => {
                self.at += 1;
                let mut items = Vec::new();
                loop {
                    self.skip_spaces();
                    match self.bytes.get(self.at) {
                        None => return Err("a parenthesis is not closed".to_owned()),
                        Some(b')') => {
                            self.at += 1;
                            break;
                        }
                        Some(_) => items.push(self.term(depth + 1)?),
                    }
                }
                if items.is_empty() {
                    return Err("a group is empty".to_owned());
                }
                Ok(Term::Group(items))
            }
            Some(_) => {
                let start = self.at;
                while self
                    .bytes
                    .get(self.at)
                    .is_some_and(|b| !matches!(b, b' ' | b'(' | b')'))
                {
                    self.at += 1;
                }
                let atom = std::str::from_utf8(&self.bytes[start..self.at])
                    .map_err(|_| "a term is not ASCII".to_owned())?;
                Self::atom(atom)
            }
        }
    }

    fn atom(atom: &str) -> Result<Term, String> {
        if atom == "_" {
            return Ok(Term::Hole);
        }
        if !atom.is_empty() && atom.bytes().all(|b| b.is_ascii_digit()) {
            return Ok(Term::Number(atom.to_owned()));
        }
        let (explicit, name) = match atom.strip_prefix('@') {
            Some(name) => (true, name),
            None => (false, atom),
        };
        if TACTIC_WORDS.contains(&name) {
            Err(format!(
                "`{name}` introduces a tactic, a binder, or a hole in a proof"
            ))
        } else if is_plain_name(name) {
            Ok(Term::Name {
                explicit,
                name: name.to_owned(),
            })
        } else {
            Err(format!("`{atom}` is not `_`, a name, or a number"))
        }
    }
}

/// One wrapper as emitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmittedWrapper {
    /// The wrapper's Lean name.
    pub lean_name: String,
    /// The specialised declaration.
    pub declaration: String,
    /// The request's tag.
    pub tag: String,
    /// The one-based line of the declaration in the host module.
    pub line: usize,
}

impl EmittedWrapper {
    /// The `[denotation]` table of an entry that denotes this wrapper
    /// (SPEC.md §13.6), for the host module named `lean_module`.
    #[must_use]
    pub fn denotation(&self, lean_module: &str) -> String {
        format!(
            "[denotation]\nkind = \"lean\"\nmodule = \"{lean_module}\"\nname = \"{}\"\n",
            self.lean_name
        )
    }
}

/// The host module and its wrappers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emitted {
    /// The Lean source of the host module, ending in a line feed.
    pub module_text: String,
    /// The wrappers in the order of their Lean names.
    pub wrappers: Vec<EmittedWrapper>,
}

/// Write the host module of a request (SPEC.md §13.13). The wrappers are in
/// the order of their Lean names, so the bytes do not depend on the order of
/// the request.
#[must_use]
pub fn emit(request: &Request) -> Emitted {
    let mut text = String::from("module\n");
    for import in &request.imports {
        text.push_str("public import ");
        text.push_str(import);
        text.push('\n');
    }
    text.push_str("set_option autoImplicit false\n");
    let first_line = 3 + request.imports.len();
    let mut ordered: Vec<&Wrapper> = request.wrappers.iter().collect();
    ordered.sort_by_key(|wrapper| wrapper.lean_name());
    let mut wrappers = Vec::with_capacity(ordered.len());
    for (offset, wrapper) in ordered.into_iter().enumerate() {
        text.push_str(&wrapper.declaration_text());
        text.push('\n');
        wrappers.push(EmittedWrapper {
            lean_name: wrapper.lean_name(),
            declaration: wrapper.declaration.clone(),
            tag: wrapper.tag.clone(),
            line: first_line + offset,
        });
    }
    Emitted {
        module_text: text,
        wrappers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REQUEST: &str = r#"spec = "lexlean/wrappers/1"
imports = ["Init", "Std"]

[[wrapper]]
declaration = "Std.Commutative.comm"
kind = "theorem"
tag = "UInt32"
arguments = ["UInt32", "(@HAdd.hAdd UInt32 UInt32 UInt32 _)", "_"]

[[wrapper]]
declaration = "Std.Commutative.comm"
kind = "theorem"
tag = "Nat"
arguments = ["Nat", "(@HAdd.hAdd Nat Nat Nat _)", "_"]
"#;

    #[test]
    fn a_wrapper_is_a_term_in_sorted_order() {
        let emitted = emit(&parse_request(REQUEST).expect("valid"));
        assert_eq!(
            emitted.module_text,
            "module\npublic import Init\npublic import Std\nset_option autoImplicit false\n\
public theorem LexLeanSpec.Nat.Std.Commutative.comm : type_of% (@_root_.Std.Commutative.comm _root_.Nat (@_root_.HAdd.hAdd _root_.Nat _root_.Nat _root_.Nat _) _) := @_root_.Std.Commutative.comm _root_.Nat (@_root_.HAdd.hAdd _root_.Nat _root_.Nat _root_.Nat _) _\n\
public theorem LexLeanSpec.UInt32.Std.Commutative.comm : type_of% (@_root_.Std.Commutative.comm _root_.UInt32 (@_root_.HAdd.hAdd _root_.UInt32 _root_.UInt32 _root_.UInt32 _) _) := @_root_.Std.Commutative.comm _root_.UInt32 (@_root_.HAdd.hAdd _root_.UInt32 _root_.UInt32 _root_.UInt32 _) _\n"
        );
        assert_eq!(emitted.wrappers[0].line, 5);
        assert!(is_wrapper_name(&emitted.wrappers[0].lean_name));
    }

    #[test]
    fn refused_requests_are_llt4018() {
        let with = |wrapper: &str| {
            format!("spec = \"lexlean/wrappers/1\"\nimports = [\"Init\"]\n[[wrapper]]\n{wrapper}\n")
        };
        for bad in [
            with("declaration = \"f\"\nkind = \"theorem\"\ntag = \"T\"\narguments = [\"(by simp)\"]"),
            with("declaration = \"f\"\nkind = \"theorem\"\ntag = \"T\"\narguments = [\"\\\"s\\\"\"]"),
            with("declaration = \"f\"\nkind = \"theorem\"\ntag = \"T\"\narguments = [\"(Nat\"]"),
            with("declaration = \"f g\"\nkind = \"theorem\"\ntag = \"T\"\narguments = []"),
            with("declaration = \"f\"\nkind = \"axiom\"\ntag = \"T\"\narguments = []"),
            with("declaration = \"f\"\nkind = \"def\"\ntag = \"T.U\"\narguments = []"),
            with("declaration = \"_root_.f\"\nkind = \"def\"\ntag = \"T\"\narguments = []"),
            with("declaration = \"f\"\nkind = \"def\"\ntag = \"T\"\narguments = []\ntactic = \"simp\""),
            "spec = \"lexlean/wrappers/2\"\nimports = []\n".to_owned(),
            "spec = \"lexlean/wrappers/1\"\nimports = []\n".to_owned(),
            "spec = \"lexlean/wrappers/1\"\nimports = [\"Std\", \"Init\"]\n".to_owned(),
        ] {
            let error = parse_request(&bad).expect_err(&bad);
            assert_eq!(error.code.as_str(), "LLT4018", "{bad}");
        }
        let duplicate = with("declaration = \"f\"\nkind = \"def\"\ntag = \"T\"\narguments = []")
            + "[[wrapper]]\ndeclaration = \"f\"\nkind = \"def\"\ntag = \"T\"\narguments = []\n";
        assert!(parse_request(&duplicate).is_err());
    }

    #[test]
    fn nesting_is_bounded() {
        let deep = format!("{}Nat{}", "(".repeat(100), ")".repeat(100));
        assert!(parse_term(&deep).is_err());
    }
}
