//! The gate behind SP-17 (SPEC.md §17.17, issue #89): no Lean source text
//! template remains in the production compiler.
//!
//! A template is a Rust string literal that is Lean syntax. The scan is a
//! lexer over the sources, not a search for keywords: it reads comments, string
//! literals (plain, raw, byte), character literals and lifetimes the way the Rust
//! lexer does, drops every item under `#[cfg(test)]`, and tests each remaining
//! literal with [`is_lean_syntax`]. Lean is written to a file by the backend
//! from typed data (`backend::lean::AuxiliaryModule` and the core backend), so a
//! literal that is Lean syntax anywhere else is a template that bypasses it.
//!
//! What counts as Lean syntax, by [`is_lean_syntax`], for the text of a literal
//! with its `\n` escapes read as line breaks:
//!
//! * a Lean symbol: `:=`, `=>`, `↦`, `→`, `∀`, `∃`, `⟨`, `⟩`, `(.`, a name
//!   quote `«`, an attribute `@[`, a `fun` binder (`fun` and a space, then a
//!   letter, `_` or `(`), a command `#print`, `#check`, or `#eval` and a space
//!   and more, or a parenthesized application of a capitalized name (`(Option
//!   {})`: a parenthesis, a name that begins with a capital, a space, and
//!   more);
//! * a command line: a line that begins with `import`, `namespace`,
//!   `set_option`, `universe`, or `open` and a space and more;
//! * a declaration: a line that begins with `theorem`, `def`, `example`,
//!   `instance`, `structure`, `inductive`, or `abbrev` and a space and more,
//!   in a literal that holds a `:`.
//!
//! A bare word (`"inductive"`, `"theorem "`) is a name or a token a scan
//! compares against, not a template, and English diagnostics that happen to
//! begin with one of the words carry no `:`. The allow-list ([`ALLOWED`]) is
//! short, and each entry says why: the printouts of kernel terms that
//! developers read, the quote of a name, and the closed commands the auxiliary
//! module renderer writes.

use std::path::Path;

/// One literal the scan reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The file, relative to the repository.
    pub file: String,
    /// The one-based line of the literal.
    pub line: usize,
    /// The literal, as written between its quotes.
    pub literal: String,
}

/// The literals that are Lean syntax and are not templates: the file they are
/// in (a suffix of its path), the beginning of the literal, and why.
pub const ALLOWED: [(&str, &str, &str); 9] = [
    (
        "production/certificate.rs",
        "«{segment}»",
        "the quote of one segment of a Lean name, which is name syntax; the runtime type of a document names its constructors by it",
    ),
    (
        "production/kernel/ext.rs",
        "∀",
        "the printout of a kernel binder, read by developers in diagnostics",
    ),
    (
        "production/kernel/ext.rs",
        "({keyword} {open}",
        "the printout of a kernel binder, read by developers in diagnostics",
    ),
    (
        "production/kernel/ext.rs",
        "(let {n} : {ty}",
        "the printout of a kernel let, read by developers in diagnostics",
    ),
    (
        "backend/lean.rs",
        "set_option autoImplicit false",
        "the renderer of a closed auxiliary command",
    ),
    (
        "backend/lean.rs",
        "example : ",
        "the renderer of a closed auxiliary command",
    ),
    (
        "backend/lean.rs",
        " := ",
        "the renderer of a closed auxiliary command",
    ),
    (
        "backend/lean.rs",
        "#print axioms ",
        "the renderer of a closed auxiliary command",
    ),
    (
        "backend/lean.rs",
        "#check @",
        "the renderer of a closed auxiliary command",
    ),
];

/// The line of `backend/lean.rs` that begins the part the scan covers: the
/// probe and audit modules, which are data the backend renders. The rest of
/// the file renders the lexicon terms of the program a user wrote, which is
/// what the backend is for.
pub const AUXILIARY_PART: &str = "Probe and audit modules";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Literal(usize, String),
    Symbol(char),
    Word(String),
}

fn tokens(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let mut out = Vec::new();
    let mut line = 1;
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        if c == '\n' {
            line += 1;
            at += 1;
        } else if c.is_whitespace() {
            at += 1;
        } else if c == '/' && chars.get(at + 1) == Some(&'/') {
            while at < chars.len() && chars[at] != '\n' {
                at += 1;
            }
        } else if c == '/' && chars.get(at + 1) == Some(&'*') {
            let mut depth = 0;
            while at < chars.len() {
                if chars[at] == '/' && chars.get(at + 1) == Some(&'*') {
                    depth += 1;
                    at += 2;
                } else if chars[at] == '*' && chars.get(at + 1) == Some(&'/') {
                    depth -= 1;
                    at += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    if chars[at] == '\n' {
                        line += 1;
                    }
                    at += 1;
                }
            }
        } else if let Some((hashes, start)) = raw_string_start(&chars, at) {
            let begin = line;
            let mut text = String::new();
            let mut end = start;
            while end < chars.len() {
                if chars[end] == '"' && (0..hashes).all(|k| chars.get(end + 1 + k) == Some(&'#')) {
                    break;
                }
                if chars[end] == '\n' {
                    line += 1;
                }
                text.push(chars[end]);
                end += 1;
            }
            out.push(Token::Literal(begin, text));
            at = end + 1 + hashes;
        } else if c == '"' || (c == 'b' && chars.get(at + 1) == Some(&'"')) {
            if c == 'b' {
                at += 1;
            }
            let begin = line;
            let mut text = String::new();
            at += 1;
            while at < chars.len() && chars[at] != '"' {
                if chars[at] == '\\' {
                    text.push('\\');
                    at += 1;
                }
                if at < chars.len() {
                    if chars[at] == '\n' {
                        line += 1;
                    }
                    text.push(chars[at]);
                    at += 1;
                }
            }
            at += 1;
            out.push(Token::Literal(begin, text));
        } else if c == '\'' {
            // A character literal is `'x'` or `'\…'`; anything else is a
            // lifetime or a label.
            if chars.get(at + 1) == Some(&'\\') {
                at += 2;
                while at < chars.len() && chars[at] != '\'' {
                    at += 1;
                }
                at += 1;
            } else if chars.get(at + 2) == Some(&'\'') {
                at += 3;
            } else {
                at += 1;
            }
        } else if c.is_alphanumeric() || c == '_' {
            let mut word = String::new();
            while at < chars.len() && (chars[at].is_alphanumeric() || chars[at] == '_') {
                word.push(chars[at]);
                at += 1;
            }
            out.push(Token::Word(word));
        } else {
            out.push(Token::Symbol(c));
            at += 1;
        }
    }
    out
}

/// A raw string begins at `at`: the number of `#` and the index after the quote.
fn raw_string_start(chars: &[char], at: usize) -> Option<(usize, usize)> {
    if at > 0 && (chars[at - 1].is_alphanumeric() || chars[at - 1] == '_') {
        return None;
    }
    let mut cursor = at;
    if chars.get(cursor) == Some(&'b') {
        cursor += 1;
    }
    if chars.get(cursor) != Some(&'r') {
        return None;
    }
    cursor += 1;
    let mut hashes = 0;
    while chars.get(cursor) == Some(&'#') {
        hashes += 1;
        cursor += 1;
    }
    (chars.get(cursor) == Some(&'"')).then_some((hashes, cursor + 1))
}

/// The tokens with every item under `#[cfg(test)]` removed.
fn outside_tests(tokens: Vec<Token>) -> Vec<Token> {
    let attribute = [
        Token::Symbol('#'),
        Token::Symbol('['),
        Token::Word("cfg".to_owned()),
        Token::Symbol('('),
        Token::Word("test".to_owned()),
        Token::Symbol(')'),
        Token::Symbol(']'),
    ];
    let mut out = Vec::new();
    let mut at = 0;
    while at < tokens.len() {
        if tokens[at..].starts_with(&attribute) {
            at += attribute.len();
            while at < tokens.len()
                && tokens[at] != Token::Symbol('{')
                && tokens[at] != Token::Symbol(';')
            {
                at += 1;
            }
            if tokens.get(at) == Some(&Token::Symbol('{')) {
                let mut depth = 0;
                while at < tokens.len() {
                    match tokens[at] {
                        Token::Symbol('{') => depth += 1,
                        Token::Symbol('}') => depth -= 1,
                        _ => {}
                    }
                    at += 1;
                    if depth == 0 {
                        break;
                    }
                }
            } else {
                at += 1;
            }
        } else {
            out.push(tokens[at].clone());
            at += 1;
        }
    }
    out
}

/// Whether the text of a literal is Lean syntax (see the module's definition).
#[must_use]
pub fn is_lean_syntax(literal: &str) -> bool {
    const SYMBOLS: [&str; 10] = [":=", "=>", "↦", "→", "∀", "∃", "⟨", "⟩", "(.", "«"];
    if SYMBOLS.iter().any(|symbol| literal.contains(symbol)) || literal.contains("@[") {
        return true;
    }
    let text = literal.replace("\\n", "\n");
    let characters: Vec<char> = text.chars().collect();
    for (at, character) in characters.iter().enumerate() {
        if *character == '(' && characters.get(at + 1).is_some_and(char::is_ascii_uppercase) {
            let name = characters[at + 1..]
                .iter()
                .take_while(|c| c.is_alphanumeric() || **c == '.' || **c == '_')
                .count();
            if characters.get(at + 1 + name) == Some(&' ')
                && characters
                    .get(at + 2 + name)
                    .is_some_and(|next| !next.is_whitespace())
            {
                return true;
            }
        }
    }
    if text.contains("fun ")
        && text
            .split("fun ")
            .skip(1)
            .any(|rest| rest.starts_with(|c: char| c.is_alphabetic() || c == '_' || c == '('))
    {
        return true;
    }
    let begins = |line: &str, words: &[&str]| {
        words.iter().any(|word| {
            line.trim_start()
                .strip_prefix(word)
                .and_then(|rest| rest.strip_prefix(' '))
                .is_some_and(|rest| !rest.trim().is_empty())
        })
    };
    text.lines().any(|line| {
        begins(
            line,
            &[
                "import",
                "namespace",
                "set_option",
                "universe",
                "open",
                "#print",
                "#check",
                "#eval",
            ],
        ) || (text.contains(':')
            && begins(
                line,
                &[
                    "theorem",
                    "def",
                    "example",
                    "instance",
                    "structure",
                    "inductive",
                    "abbrev",
                ],
            ))
    })
}

/// The literals of `source` that are Lean syntax and are not allowed, in the
/// file named `file`, from line `from` on.
#[must_use]
pub fn scan_source(file: &str, source: &str, from: usize) -> Vec<Finding> {
    outside_tests(tokens(source))
        .into_iter()
        .filter_map(|token| match token {
            Token::Literal(line, literal)
                if line >= from
                    && is_lean_syntax(&literal)
                    && !ALLOWED.iter().any(|(suffix, beginning, _)| {
                        file.ends_with(suffix) && literal.starts_with(beginning)
                    }) =>
            {
                Some(Finding {
                    file: file.to_owned(),
                    line,
                    literal,
                })
            }
            _ => None,
        })
        .collect()
}

/// The first line of `backend/lean.rs` the scan covers.
#[must_use]
pub fn auxiliary_part_from(source: &str) -> usize {
    source
        .lines()
        .position(|line| line.contains(AUXILIARY_PART))
        .map_or(1, |index| index + 1)
}

/// The findings of the repository at `root`: every source under
/// `crates/lexlean/src/production/`, and `backend/lean.rs` from its auxiliary
/// module part.
///
/// # Panics
///
/// Panics when a source cannot be read.
#[must_use]
pub fn scan_repository(root: &Path) -> Vec<Finding> {
    let base = root.join("crates/lexlean/src");
    let mut findings = Vec::new();
    for entry in walkdir::WalkDir::new(base.join("production"))
        .sort_by_file_name()
        .into_iter()
        .flatten()
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
    {
        let source = std::fs::read_to_string(entry.path()).expect("a source reads");
        let file = entry
            .path()
            .strip_prefix(&base)
            .expect("under the sources")
            .to_string_lossy()
            .replace('\\', "/");
        findings.extend(scan_source(&file, &source, 1));
    }
    let backend = std::fs::read_to_string(base.join("backend/lean.rs")).expect("the backend reads");
    findings.extend(scan_source(
        "backend/lean.rs",
        &backend,
        auxiliary_part_from(&backend),
    ));
    findings
}
