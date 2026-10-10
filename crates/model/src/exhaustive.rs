//! The production-eligibility exhaustiveness audit (SPEC.md §17.13).
//!
//! Production eligibility is decided by mapping every semantic construct to
//! its registry row. A default branch there would admit a construct that
//! nobody classified, so the analysis source may name constructs only
//! explicitly: no wildcard arm, no rest pattern, no `if let`, `while let`,
//! `let`-`else`, or `matches!`, and every variant of every audited IR enum
//! named at least once. The audit reads source text, so it is shared by
//! `cargo xtask validate-model` and the conformance case that plants each
//! forbidden form and observes the audit fail.

use std::fmt::Write as _;

/// The analysis source the audit reads, relative to the repository root.
pub const ELIGIBILITY_SOURCE: &str = "crates/lexlean/src/production/eligibility.rs";

/// The IR source whose enums the analysis must cover.
pub const SEMANTIC_SOURCE: &str = "crates/lexlean/src/ir/semantic.rs";

/// The IR enums whose every variant the analysis names explicitly.
pub const AUDITED_ENUMS: [&str; 5] = [
    "SemanticType",
    "SemanticTerm",
    "SemanticPrimitive",
    "SemanticDeclaration",
    "SemanticInteger",
];

/// The variants of `pub enum <name>` in Rust source, in declaration order.
///
/// # Errors
///
/// Returns a reason when the enum is absent or declares no variant.
pub fn enum_variants(source: &str, name: &str) -> Result<Vec<String>, String> {
    let header = format!("pub enum {name} {{");
    let mut lines = source.lines().skip_while(|line| line.trim() != header);
    if lines.next().is_none() {
        return Err(format!("{SEMANTIC_SOURCE}: `pub enum {name}` is absent"));
    }
    let mut variants = Vec::new();
    for line in lines {
        if line == "}" {
            break;
        }
        // Variants sit at exactly one level of indentation; fields, doc
        // comments, and attributes do not start with an uppercase letter
        // there.
        let Some(rest) = line.strip_prefix("    ") else {
            continue;
        };
        if rest.starts_with(' ') {
            continue;
        }
        let identifier: String = rest
            .chars()
            .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
            .collect();
        if identifier
            .chars()
            .next()
            .is_some_and(|first| first.is_ascii_uppercase())
        {
            variants.push(identifier);
        }
    }
    if variants.is_empty() {
        return Err(format!(
            "{SEMANTIC_SOURCE}: `pub enum {name}` declares no variant"
        ));
    }
    Ok(variants)
}

/// The code of one line with string and character literals blanked and any
/// line comment removed, so a forbidden spelling in prose or data does not
/// count and a forbidden spelling in code cannot hide behind a quote.
fn code_of(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    let mut in_string = false;
    while let Some(character) = chars.next() {
        if in_string {
            match character {
                '\\' => {
                    chars.next();
                }
                '"' => {
                    in_string = false;
                    out.push('"');
                }
                _ => out.push(' '),
            }
            continue;
        }
        match character {
            '"' => {
                in_string = true;
                out.push('"');
            }
            // A character literal (`'"'`, `'\''`, `'x'`) is blanked whole,
            // so a quote inside it cannot open a string; a lifetime (`'a`)
            // has no closing quote and is kept.
            '\'' => {
                let rest: String = chars.clone().take(12).collect();
                let length = if rest.starts_with('\\') {
                    rest.chars()
                        .enumerate()
                        .skip(2)
                        .find(|(_, next)| *next == '\'')
                        .map(|(index, _)| index + 1)
                } else {
                    rest.chars().nth(1).filter(|next| *next == '\'').map(|_| 2)
                };
                match length {
                    Some(length) => {
                        out.push_str("' '");
                        for _ in 0..length {
                            chars.next();
                        }
                    }
                    None => out.push(character),
                }
            }
            '/' if chars.peek() == Some(&'/') => break,
            _ => out.push(character),
        }
    }
    out
}

/// Whether `text` is a bare binding pattern: an identifier that is not a
/// literal, optionally bound with `@` or guarded. Such an arm matches
/// everything its scrutinee can be, exactly like `_`.
fn is_binding(text: &str) -> bool {
    let text = text.trim();
    let head = text
        .split(" if ")
        .next()
        .unwrap_or_default()
        .split(" @ ")
        .next()
        .unwrap_or_default()
        .trim();
    let head = head
        .strip_prefix("ref mut ")
        .or_else(|| head.strip_prefix("ref "))
        .or_else(|| head.strip_prefix("mut "))
        .unwrap_or(head);
    head == "_"
        || (!head.is_empty()
            && head != "true"
            && head != "false"
            && head
                .chars()
                .next()
                .is_some_and(|first| first.is_ascii_lowercase() || first == '_')
            && head
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || character == '_'))
}

/// The top-level elements of a parenthesized tuple pattern.
fn tuple_elements(pattern: &str) -> Option<Vec<String>> {
    let inner = pattern.trim().strip_prefix('(')?.strip_suffix(')')?;
    let mut elements = Vec::new();
    let mut depth = 0_usize;
    let mut current = String::new();
    for character in inner.chars() {
        match character {
            '(' | '[' | '{' => {
                depth += 1;
                current.push(character);
            }
            ')' | ']' | '}' => {
                depth = depth.saturating_sub(1);
                current.push(character);
            }
            ',' if depth == 0 => elements.push(std::mem::take(&mut current)),
            other => current.push(other),
        }
    }
    if !current.trim().is_empty() {
        elements.push(current);
    }
    Some(elements)
}

/// The default a match arm's pattern takes, if any: the arm's pattern is the
/// text before `=>`, less a leading `|`. The audited source is `rustfmt`
/// output, which places each arm on its own line.
fn default_arm(code: &str) -> Option<&'static str> {
    let (pattern, _) = code.split_once("=>")?;
    let pattern = pattern.trim().trim_start_matches('|').trim();
    if pattern.is_empty() {
        return None;
    }
    for alternative in pattern.split(" | ") {
        if is_binding(alternative) {
            return Some("a binding or wildcard catch-all arm");
        }
        if let Some(elements) = tuple_elements(alternative) {
            if elements.iter().any(|element| is_binding(element)) {
                return Some("a tuple pattern with a binding or wildcard element");
            }
        }
    }
    None
}

/// The forbidden pattern forms, each with the reason it is forbidden.
fn forbidden(code: &str) -> Option<&'static str> {
    let compact: String = code.split_whitespace().collect::<Vec<_>>().join(" ");
    let tight: String = code
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();
    if tight.contains("_=>") && (compact.contains(" _ =>") || compact.starts_with("_ =>")) {
        return Some("a wildcard arm");
    }
    if let Some(reason) = default_arm(&compact) {
        return Some(reason);
    }
    // The IR derives `PartialEq`, so an equality chain over it classifies
    // by default in its final `else`.
    let ir_path = |operand: &str| operand.starts_with("Semantic") && operand.contains("::");
    let path_character = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == ':';
    for operator in ["==", "!="] {
        if compact.match_indices(operator).any(|(start, _)| {
            let after = compact[start + operator.len()..]
                .trim_start()
                .trim_start_matches(['&', '*']);
            let before = compact[..start].trim_end();
            let left = before
                .rsplit(|c: char| !path_character(c))
                .next()
                .unwrap_or_default();
            let right = after
                .split(|c: char| !path_character(c))
                .next()
                .unwrap_or_default();
            ir_path(left) || ir_path(right)
        }) {
            return Some("an equality test on an IR enum");
        }
    }
    if tight.contains("..}")
        || tight.contains("..)")
        || tight.contains("..]")
        || tight.contains(",..")
    {
        return Some("a rest pattern");
    }
    if compact.contains("if let ") {
        return Some("an `if let` with an implicit default");
    }
    if compact.contains("while let ") {
        return Some("a `while let` with an implicit default");
    }
    if compact.contains("matches!(") {
        return Some("a `matches!` with an implicit default");
    }
    let let_else = compact.match_indices("let ").any(|(start, _)| {
        let word_start = start == 0
            || !compact[..start].ends_with(|c: char| c.is_ascii_alphanumeric() || c == '_');
        let statement = compact[start..].split(';').next().unwrap_or_default();
        word_start && statement.contains(" else {")
    });
    if let_else {
        return Some("a `let`-`else` with an implicit default");
    }
    None
}

/// The preservation sources that lower and certify every runtime construct
/// (§17.17). A default branch there would realize or prove a construct
/// nobody considered, so they obey the eligibility analysis's rules.
pub const PRESERVATION_SOURCES: [&str; 4] = [
    "crates/lexlean/src/production/lower.rs",
    "crates/lexlean/src/production/certificate.rs",
    "crates/lexlean/src/production/source.rs",
    "crates/lexlean/src/production/rust_term.rs",
];

/// Audit the eligibility analysis source against the IR source.
///
/// # Errors
///
/// Returns every violation found, one per line.
pub fn audit_eligibility(eligibility: &str, semantic: &str) -> Result<(), String> {
    audit_source(ELIGIBILITY_SOURCE, eligibility, semantic, true)
}

/// Audit one preservation source: no forbidden form, and, for the lowering
/// and the certificate, every variant of every audited enum named.
///
/// # Errors
///
/// Returns every violation found, one per line.
pub fn audit_preservation(path: &str, text: &str, semantic: &str) -> Result<(), String> {
    audit_source(
        path,
        text,
        semantic,
        path == PRESERVATION_SOURCES[0] || path == PRESERVATION_SOURCES[1],
    )
}

fn audit_source(
    path: &str,
    eligibility: &str,
    semantic: &str,
    covering: bool,
) -> Result<(), String> {
    let mut report = String::new();
    for (index, line) in eligibility.lines().enumerate() {
        let code = code_of(line);
        if let Some(reason) = forbidden(&code) {
            let _ = writeln!(
                report,
                "{path}:{}: {reason} would classify constructs by default",
                index + 1
            );
        }
    }
    // A variant counts as classified only where a match arm names it: a
    // pattern line starts with the path (after any `|`) and either carries
    // its `=>`, opens a multi-line struct pattern, or continues into another
    // alternative. A mention in a list or an expression classifies nothing.
    let lines: Vec<String> = eligibility.lines().map(code_of).collect();
    let pattern_line = |index: usize, path: &str| {
        let line = lines[index].trim();
        let line = line.strip_prefix('|').map_or(line, str::trim_start);
        let continues = lines
            .iter()
            .skip(index + 1)
            .find(|next| !next.trim().is_empty())
            .is_some_and(|next| next.trim_start().starts_with('|'));
        line.strip_prefix(path).is_some_and(|rest| {
            !rest
                .chars()
                .next()
                .is_some_and(|next| next.is_ascii_alphanumeric() || next == '_')
        }) && (line.contains("=>") || line.ends_with('{') || line.ends_with('|') || continues)
    };
    for name in AUDITED_ENUMS.iter().filter(|_| covering) {
        for variant in enum_variants(semantic, name)? {
            let variant_path = format!("{name}::{variant}");
            if !(0..lines.len()).any(|index| pattern_line(index, &variant_path)) {
                let _ = writeln!(
                    report,
                    "{path}: `{variant_path}` has no explicit production disposition"
                );
            }
        }
    }
    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const IR: &str = "pub enum SemanticType {\n    Nat,\n    List { element: Box<Self> },\n}\npub enum SemanticTerm {\n    Var { name: String },\n}\npub enum SemanticPrimitive {\n    Length,\n}\npub enum SemanticDeclaration {\n    Theorem {\n        name: String,\n    },\n}\npub enum SemanticInteger {\n    Int,\n}\n";

    const CLEAN: &str = "fn f(t: &SemanticType) {\n match t {\n SemanticType::Nat => {}\n SemanticType::List { element: _ } => {}\n }\n}\n// _ => .. }\nconst K: [&str; 1] = [\"_ => .. }\"];\nfn g(t: &SemanticTerm, p: SemanticPrimitive, d: &SemanticDeclaration, i: SemanticInteger) {\n match t {\n SemanticTerm::Var { name: _ } => {}\n }\n match p {\n SemanticPrimitive::Length => {}\n }\n match d {\n SemanticDeclaration::Theorem {\n name: _,\n } => {}\n }\n match i {\n SemanticInteger::Int => {}\n }\n}\n";

    #[test]
    fn variants_are_read_at_one_indentation_level() {
        assert_eq!(enum_variants(IR, "SemanticType").unwrap(), ["Nat", "List"]);
        assert_eq!(
            enum_variants(IR, "SemanticDeclaration").unwrap(),
            ["Theorem"]
        );
    }

    #[test]
    fn explicit_source_passes_and_comments_or_strings_do_not_count() {
        audit_eligibility(CLEAN, IR).unwrap();
    }

    #[test]
    fn every_default_form_and_every_unnamed_variant_fails() {
        for planted in [
            "fn h(t: &SemanticType) { match t { _ => {} } }",
            "fn h(t: &SemanticTerm) { match t { SemanticTerm::Var { .. } => {} } }",
            "fn h(t: &SemanticType) { if let SemanticType::Nat = t {} }",
            "fn h(t: &SemanticType) -> bool { matches!(t, SemanticType::Nat) }",
            "fn h(t: Option<u8>) { while let Some(_) = t {} }",
            "fn h(t: Option<u8>) { let Some(_) = t else { return }; }",
            "fn h(t: &SemanticType) { match t { SemanticType::Nat => {}\n _other => {} } }",
            "fn h(t: &SemanticType) { match t { SemanticType::Nat => {}\n other => {} } }",
            "fn h(t: &SemanticType) { match t { SemanticType::Nat => {}\n ty if ty.is_nat() => {} } }",
            "fn h(t: &SemanticType, u: &SemanticType) { match (t, u) {\n (SemanticType::Nat, SemanticType::Nat) => {}\n (_, _) => {} } }",
            "fn h(t: &SemanticType, u: &SemanticType) { match (t, u) {\n (SemanticType::Nat, other) => {} } }",
            "fn h(t: &SemanticType) -> bool { *t == SemanticType::Nat }",
            "fn h(t: &SemanticType) -> bool { SemanticType::Nat != *t }",
            "fn h() -> char { '\"' } fn k(t: &SemanticType) { match t { _ => {} } }",
        ] {
            let source = format!("{CLEAN}{planted}\n");
            assert!(audit_eligibility(&source, IR).is_err(), "{planted}");
        }
        let missing = CLEAN.replace(" SemanticInteger::Int => {}\n", "");
        let error = audit_eligibility(&missing, IR).unwrap_err();
        assert!(error.contains("SemanticInteger::Int"), "{error}");
        // A mention outside a match arm classifies nothing.
        let mentioned =
            format!("{missing}const L: [SemanticInteger; 1] = [\n SemanticInteger::Int,\n];\n");
        let error = audit_eligibility(&mentioned, IR).unwrap_err();
        assert!(error.contains("SemanticInteger::Int"), "{error}");
    }
}
