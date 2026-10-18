//! The classification-exhaustiveness audit (RP-15, R8).
//!
//! A `match` that names several variants of a closed enum is a classification:
//! each variant it names has been decided on, and a default arm decides every
//! other one silently, including a variant added tomorrow. The compiler's own
//! exhaustiveness check is the only reviewer a new variant is certain to meet,
//! and a catch-all arm turns it off. So a `match` that names at least
//! [`CLASSIFYING_ARMS`] variants of one audited enum must name them all: no
//! `_` arm and no bare binding arm (which matches everything exactly as `_`
//! does).
//!
//! Smaller matches are lookups ("is this a `Nat`?"), where an `_` arm states
//! that the answer is the same for everything else, and are not audited. The
//! threshold is a judgment, recorded here so the choice is visible: the
//! shortest classification this repository had when the audit was written
//! named five variants, and the longest lookup named three.
//!
//! The audit reads source text, so it is shared by `cargo xtask
//! validate-model` and the conformance case that plants a catch-all in each
//! consumer group and observes the audit fail. It reads only direct arm
//! patterns (`SemanticType::Nat`, or `Self::Nat` inside an `impl` of the
//! enum); a match on a tuple of enums is a pairing, not a classification of
//! either, and is not audited. Serialization and the snapshot are
//! `serde`-derived, so no hand-written `match` of theirs exists to audit.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::exhaustive::AUDITED_ENUMS;

/// A `match` naming at least this many variants of one audited enum is a
/// classification.
pub const CLASSIFYING_ARMS: usize = 5;

/// The source tree the audit reads, relative to the repository root.
pub const SOURCE_ROOT: &str = "crates/lexlean/src";

/// The directory whose enums are the production realization calculus and its
/// Rust AST; their names are common (`Expr`, `Type`), so they are audited
/// only beneath it.
pub const CALCULUS_ROOT: &str = "crates/lexlean/src/calculus/";

/// The calculus and Rust AST enums whose every variant a classification
/// names.
pub const CALCULUS_ENUMS: [&str; 11] = [
    "Ty", "Prim", "Value", "Shape", "Expr", "Type", "Lit", "Ctor", "Pat", "Callee", "ItemDef",
];

/// The sources that must each still hold a classification, and the consumers
/// of a new variant they stand for. A file that loses its last one has been
/// refactored out from under the audit, and the audit would pass over it
/// vacuously.
pub const CONSUMERS: [(&str, &str); 8] = [
    (
        "crates/lexlean/src/backend/semantic.rs",
        "the Lean and LaTeX backends",
    ),
    (
        "crates/lexlean/src/ir/semantic.rs",
        "validation, serialization checks, and charge",
    ),
    (
        "crates/lexlean/src/production/eligibility.rs",
        "production eligibility",
    ),
    (
        "crates/lexlean/src/calculus/check.rs",
        "the calculus checker",
    ),
    (
        "crates/lexlean/src/calculus/realization.rs",
        "the realization table",
    ),
    ("crates/lexlean/src/calculus/rust/ast.rs", "the Rust AST"),
    (
        "crates/lexlean/src/calculus/rust/lower.rs",
        "the Rust lowering",
    ),
    (
        "crates/lexlean/src/calculus/rust/validate.rs",
        "the Rust validator",
    ),
];

/// Sources that still hold a catch-all in a classification, because their
/// bytes are recorded in the compiler-semantics ID (`rust_renderer` in
/// `language/semantics-1.2.toml`): editing them moves that ID, which is a
/// decision of its own and is made in the change that next moves the record.
///
/// The entry expires by itself. A source listed here that holds no catch-all
/// fails the audit, so the entry is removed in the change that removes the
/// last one, and the exemption cannot outlive its need.
pub const IDENTITY_BOUND: [&str; 2] = [
    "crates/lexlean/src/calculus/rust/lower.rs",
    "crates/lexlean/src/calculus/rust/validate.rs",
];

/// One `match` that names at least [`CLASSIFYING_ARMS`] variants of an enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classification {
    /// The 1-based line of the `match` keyword.
    pub line: usize,
    /// The enum whose variants it names.
    pub enumeration: String,
    /// How many distinct variants it names.
    pub named: usize,
    /// The 1-based line and kind of its first catch-all arm, if it has one.
    pub catch_all: Option<(usize, &'static str)>,
}

/// What a clean audit read, so it can report that it was armed.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Summary {
    /// The number of source files read.
    pub files: usize,
    /// The number of classifications found, per source file that has any.
    pub per_file: BTreeMap<String, usize>,
    /// The catch-alls tolerated in [`IDENTITY_BOUND`] sources, per source.
    pub tolerated: BTreeMap<String, usize>,
}

impl Summary {
    /// The total number of classifications, all of them total.
    #[must_use]
    pub fn classifications(&self) -> usize {
        self.per_file.values().sum()
    }
}

/// `source` with comments blanked and string and character literals reduced to
/// their quotes, newlines kept, so a forbidden spelling in prose or data does
/// not count and one in code cannot hide behind a quote.
fn blank(source: &str) -> Vec<char> {
    let chars: Vec<char> = source.chars().collect();
    let mut out = Vec::with_capacity(chars.len());
    let mut index = 0;
    let keep = |c: char| if c == '\n' { '\n' } else { ' ' };
    while index < chars.len() {
        let c = chars[index];
        let next = chars.get(index + 1).copied();
        if c == '/' && next == Some('/') {
            while index < chars.len() && chars[index] != '\n' {
                out.push(' ');
                index += 1;
            }
        } else if c == '/' && next == Some('*') {
            while index < chars.len()
                && !(chars[index] == '*' && chars.get(index + 1) == Some(&'/'))
            {
                out.push(keep(chars[index]));
                index += 1;
            }
            out.extend([' ', ' ']);
            index += 2;
        } else if c == 'r'
            && (index == 0 || !is_ident(chars[index - 1]))
            && matches!(next, Some('"' | '#'))
            && raw_string_hashes(&chars, index + 1).is_some()
        {
            let hashes = raw_string_hashes(&chars, index + 1).unwrap_or(0);
            let close: Vec<char> = std::iter::once('"')
                .chain(std::iter::repeat_n('#', hashes))
                .collect();
            let mut end = index + 2 + hashes;
            while end < chars.len() && chars[end..].iter().take(close.len()).ne(close.iter()) {
                end += 1;
            }
            // Length-preserving, so an offset in the blanked text is an
            // offset in the source.
            let total = (end + close.len()).min(chars.len()) - index;
            for (position, &character) in chars.iter().enumerate().skip(index).take(total) {
                let quote =
                    position == index + 1 + hashes || position == index + total - 1 - hashes;
                out.push(match character {
                    '\n' => '\n',
                    '"' if quote => '"',
                    _ => ' ',
                });
            }
            index += total;
        } else if c == '"' {
            out.push('"');
            index += 1;
            while index < chars.len() && chars[index] != '"' {
                if chars[index] == '\\' {
                    out.push(' ');
                    index += 1;
                }
                if index < chars.len() {
                    out.push(keep(chars[index]));
                    index += 1;
                }
            }
            out.push('"');
            index += 1;
        } else if c == '\'' {
            // A character literal is blanked whole, so a quote inside it
            // cannot open a string; a lifetime has no closing quote.
            let literal_end = if next == Some('\\') {
                (index + 2..chars.len().min(index + 12)).find(|&at| chars[at] == '\'')
            } else if chars.get(index + 2) == Some(&'\'') {
                Some(index + 2)
            } else {
                None
            };
            match literal_end {
                Some(end) => {
                    out.extend(std::iter::repeat_n(' ', end + 1 - index));
                    index = end + 1;
                }
                None => {
                    out.push(c);
                    index += 1;
                }
            }
        } else {
            out.push(c);
            index += 1;
        }
    }
    out
}

/// The `#` count of a raw string whose opening `#`s start at `from`, if a `"`
/// follows them.
fn raw_string_hashes(chars: &[char], from: usize) -> Option<usize> {
    let hashes = chars[from..].iter().take_while(|&&c| c == '#').count();
    (chars.get(from + hashes) == Some(&'"')).then_some(hashes)
}

fn is_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn line_of(chars: &[char], at: usize) -> usize {
    1 + chars[..at.min(chars.len())]
        .iter()
        .filter(|&&c| c == '\n')
        .count()
}

/// The index of the `}` closing the `{` at `open`, if any.
fn close_of(chars: &[char], open: usize) -> Option<usize> {
    let mut depth = 0_usize;
    for (offset, &c) in chars[open..].iter().enumerate() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}

/// The index of the first `{` at bracket depth zero from `from`.
fn open_brace(chars: &[char], from: usize) -> Option<usize> {
    let mut depth = 0_isize;
    for (offset, &c) in chars[from..].iter().enumerate() {
        match c {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            '{' if depth == 0 => return Some(from + offset),
            ';' if depth == 0 => return None,
            _ => {}
        }
    }
    None
}

/// Every occurrence of the word `word` in code, as a start index.
fn words(chars: &[char], word: &str) -> Vec<usize> {
    let needle: Vec<char> = word.chars().collect();
    let mut found = Vec::new();
    let mut index = 0;
    while index + needle.len() <= chars.len() {
        if chars[index..index + needle.len()] == needle[..]
            && (index == 0 || !is_ident(chars[index - 1]))
            && chars
                .get(index + needle.len())
                .is_none_or(|&c| !is_ident(c))
        {
            found.push(index);
            index += needle.len();
        } else {
            index += 1;
        }
    }
    found
}

/// `text` less a leading generic parameter list (`<T: Ord>`).
fn strip_generics(text: &str) -> &str {
    if !text.starts_with('<') {
        return text;
    }
    let mut depth = 0_usize;
    for (at, c) in text.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return text[at + 1..].trim_start();
                }
            }
            _ => {}
        }
    }
    text
}

/// The enum a `Self::Variant` pattern names at each position: the type of the
/// innermost `impl` block holding it.
fn impl_types(chars: &[char]) -> Vec<(usize, usize, String)> {
    let mut blocks = Vec::new();
    for start in words(chars, "impl") {
        // Only an item (`impl` starting its line), never `-> impl Trait`.
        let line_start = chars[..start]
            .iter()
            .rposition(|&c| c == '\n')
            .map_or(0, |at| at + 1);
        if chars[line_start..start].iter().any(|c| !c.is_whitespace()) {
            continue;
        }
        let Some(open) = open_brace(chars, start) else {
            continue;
        };
        let Some(close) = close_of(chars, open) else {
            continue;
        };
        let header: String = chars[start + 4..open].iter().collect();
        let header = strip_generics(header.trim());
        let target = header.split(" for ").last().unwrap_or_default().trim();
        let target = strip_generics(target.trim_start_matches('&').trim_start());
        let name: String = target.chars().take_while(|&c| is_ident(c)).collect();
        if !name.is_empty() {
            blocks.push((open, close, name));
        }
    }
    blocks
}

/// The arms of the `match` body `body`, each as the byte offset (within
/// `body`) of its pattern and the pattern's text, guard included.
fn arms(body: &[char]) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    let mut index = 0;
    let mut pattern = String::new();
    let mut pattern_start = 0;
    let mut in_pattern = true;
    let mut depth = 0_isize;
    while index < body.len() {
        let c = body[index];
        if in_pattern {
            if depth == 0 && c == '=' && body.get(index + 1) == Some(&'>') {
                found.push((pattern_start, std::mem::take(&mut pattern)));
                in_pattern = false;
                index += 2;
                continue;
            }
            match c {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth -= 1,
                _ => {}
            }
            if pattern.trim().is_empty() {
                pattern_start = index;
            }
            pattern.push(c);
            index += 1;
            continue;
        }
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' => depth -= 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    // A block body ends the arm unless the expression goes on
                    // (`{ .. }.len()`, `{ .. }?`).
                    let rest = &body[index + 1..];
                    let next = rest.iter().find(|c| !c.is_whitespace());
                    match next {
                        Some(',') => {
                            in_pattern = true;
                            index += 1 + rest.iter().take_while(|c| **c != ',').count() + 1;
                            pattern.clear();
                            continue;
                        }
                        Some('.' | '?') => {}
                        _ => {
                            in_pattern = true;
                            pattern.clear();
                        }
                    }
                }
            }
            ',' if depth == 0 => {
                in_pattern = true;
                pattern.clear();
            }
            _ => {}
        }
        index += 1;
    }
    found
}

/// The top-level `|` alternatives of an arm's pattern, less its guard.
fn alternatives(pattern: &str) -> (Vec<String>, bool) {
    let mut depth = 0_isize;
    let mut parts = vec![String::new()];
    let mut guarded = false;
    let chars: Vec<char> = pattern.chars().collect();
    for (index, &c) in chars.iter().enumerate() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            _ => {}
        }
        if depth == 0
            && c.is_whitespace()
            && chars.get(index + 1) == Some(&'i')
            && chars.get(index + 2) == Some(&'f')
            && chars
                .get(index + 3)
                .is_some_and(|next| next.is_whitespace())
        {
            guarded = true;
            break;
        }
        if depth == 0 && c == '|' {
            parts.push(String::new());
        } else if let Some(last) = parts.last_mut() {
            last.push(c);
        }
    }
    let parts = parts
        .into_iter()
        .map(|part| part.trim().to_owned())
        .filter(|part| !part.is_empty())
        .collect();
    (parts, guarded)
}

/// Whether `text` is a bare binding: an identifier that is not a literal,
/// which matches everything its scrutinee can be, exactly like `_`.
fn is_binding(text: &str) -> bool {
    let head = text.trim();
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
            && head.chars().all(is_ident))
}

/// The enum and variant a direct arm pattern names.
fn direct_variant(
    alternative: &str,
    enums: &BTreeSet<&str>,
    in_impl: Option<&str>,
) -> Option<(String, String)> {
    let text = alternative
        .trim()
        .trim_start_matches('&')
        .trim_start()
        .trim_start_matches("ref ")
        .trim_start_matches("mut ");
    let path: String = text
        .chars()
        .take_while(|&c| is_ident(c) || c == ':')
        .collect();
    let segments: Vec<&str> = path.split("::").collect();
    let [.., owner, variant] = segments.as_slice() else {
        return None;
    };
    let owner = if *owner == "Self" { in_impl? } else { owner };
    (enums.contains(owner)
        && variant
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_uppercase()))
    .then(|| (owner.to_owned(), (*variant).to_owned()))
}

/// A classification and where its `match` lies in the source.
struct Found {
    classification: Classification,
    keyword: usize,
    close: usize,
}

fn scan(chars: &[char], enums: &[&str]) -> Vec<Found> {
    let enums: BTreeSet<&str> = enums.iter().copied().collect();
    let impls = impl_types(chars);
    let mut found = Vec::new();
    for keyword in words(chars, "match") {
        let Some(open) = open_brace(chars, keyword + 5) else {
            continue;
        };
        let Some(close) = close_of(chars, open) else {
            continue;
        };
        let in_impl = impls
            .iter()
            .filter(|(start, end, _)| *start < keyword && keyword < *end)
            .min_by_key(|(start, end, _)| end - start)
            .map(|(_, _, name)| name.as_str());
        let body = &chars[open + 1..close];
        let mut variants: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut catch_all = None;
        for (offset, pattern) in arms(body) {
            let (alternatives, guarded) = alternatives(&pattern);
            for alternative in &alternatives {
                if let Some((owner, variant)) = direct_variant(alternative, &enums, in_impl) {
                    variants.entry(owner).or_default().insert(variant);
                }
            }
            // A guarded arm never makes a match exhaustive, so the compiler
            // still refuses a variant that only it would take.
            if catch_all.is_none() && !guarded && alternatives.iter().any(|a| is_binding(a)) {
                let kind = if alternatives.iter().any(|a| a.trim() == "_") {
                    "a wildcard arm"
                } else {
                    "a binding catch-all arm"
                };
                catch_all = Some((line_of(chars, open + 1 + offset), kind));
            }
        }
        let Some((enumeration, named)) = variants
            .into_iter()
            .map(|(owner, set)| (owner, set.len()))
            .max_by_key(|(_, count)| *count)
        else {
            continue;
        };
        if named >= CLASSIFYING_ARMS {
            found.push(Found {
                classification: Classification {
                    line: line_of(chars, keyword),
                    enumeration,
                    named,
                    catch_all,
                },
                keyword,
                close,
            });
        }
    }
    found
}

/// Every `match` of `source` that names at least [`CLASSIFYING_ARMS`]
/// variants of one of `enums`.
#[must_use]
pub fn classifications(source: &str, enums: &[&str]) -> Vec<Classification> {
    scan(&blank(source), enums)
        .into_iter()
        .map(|found| found.classification)
        .collect()
}

/// `source` with a wildcard arm added to the first classification inside the
/// function `function` that has none, or `None` if there is no such one.
///
/// This exists for the conformance case that plants a catch-all in each
/// consumer and observes the audit fail; the result is text to audit, not
/// necessarily source that compiles.
#[must_use]
pub fn plant_catch_all(source: &str, enums: &[&str], function: &str) -> Option<String> {
    let chars: Vec<char> = source.chars().collect();
    let blanked = blank(source);
    let start = words(&blanked, "fn").into_iter().find(|&at| {
        blanked[at + 2..]
            .iter()
            .skip_while(|c| c.is_whitespace())
            .take_while(|&&c| is_ident(c))
            .collect::<String>()
            == function
    })?;
    let open = open_brace(&blanked, start)?;
    let close = close_of(&blanked, open)?;
    let target = scan(&blanked, enums).into_iter().find(|found| {
        open < found.keyword && found.keyword < close && found.classification.catch_all.is_none()
    })?;
    let mut planted: String = chars[..target.close].iter().collect();
    planted.push_str("\n_ => Default::default(),\n");
    planted.extend(&chars[target.close..]);
    Some(planted)
}

/// The enums audited in the source at `path`.
fn enums_for(path: &str) -> Vec<&'static str> {
    let mut enums: Vec<&'static str> = AUDITED_ENUMS.to_vec();
    if path.starts_with(CALCULUS_ROOT) {
        enums.extend(CALCULUS_ENUMS);
    }
    enums
}

/// Audit every source of `sources`, given as `(repository-relative path,
/// text)`.
///
/// # Errors
///
/// Returns every catch-all in a classification, one per line, and every
/// consumer in [`CONSUMERS`] that no longer holds a classification (or is
/// absent), because an audit that read nothing there would pass without
/// having checked it.
pub fn audit(sources: &[(String, String)]) -> Result<Summary, String> {
    let mut summary = Summary::default();
    let mut failures = String::new();
    for (path, text) in sources {
        summary.files += 1;
        let found = classifications(text, &enums_for(path));
        if !found.is_empty() {
            summary.per_file.insert(path.clone(), found.len());
        }
        for classification in found {
            if let Some((line, kind)) = classification.catch_all {
                if IDENTITY_BOUND.contains(&path.as_str()) {
                    *summary.tolerated.entry(path.clone()).or_default() += 1;
                    continue;
                }
                let _ = writeln!(
                    failures,
                    "R8: {path}:{}: a `match` naming {} variants of `{}` has {kind} at line {line}; \
                     name every variant, so that a variant added later is a compile error here and \
                     not a silent default (RP-15)",
                    classification.line, classification.named, classification.enumeration
                );
            }
        }
    }
    for path in IDENTITY_BOUND {
        if !summary.tolerated.contains_key(path) {
            let _ = writeln!(
                failures,
                "R8: {path} holds no catch-all in a classification, or is not read; remove it from \
                 IDENTITY_BOUND, which exists only for sources that still hold one (RP-15)"
            );
        }
    }
    for (path, role) in CONSUMERS {
        if !summary.per_file.contains_key(path) {
            let _ = writeln!(
                failures,
                "R8: {path} holds no `match` naming {CLASSIFYING_ARMS} or more variants of an \
                 audited enum, or is not read; the classification audit would pass over {role} \
                 without checking it (RP-15)"
            );
        }
    }
    if failures.is_empty() {
        Ok(summary)
    } else {
        Err(failures.trim_end().to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENUMS: [&str; 1] = ["Shape"];

    fn flagged(source: &str) -> Vec<(usize, Option<&'static str>)> {
        classifications(source, &ENUMS)
            .into_iter()
            .map(|c| (c.named, c.catch_all.map(|(_, kind)| kind)))
            .collect()
    }

    const TOTAL: &str = "fn f(s: Shape) -> u8 {\n    match s {\n        Shape::A => 1,\n        Shape::B => 2,\n        Shape::C => 3,\n        Shape::D => 4,\n        Shape::E => 5,\n    }\n}\n";

    #[test]
    fn a_total_classification_is_clean() {
        assert_eq!(flagged(TOTAL), [(5, None)]);
    }

    #[test]
    fn a_wildcard_in_a_classification_is_found() {
        let planted = TOTAL.replace("Shape::E => 5,", "_ => 5,");
        assert_eq!(flagged(&planted), [], "four named is a lookup");
        let planted = TOTAL.replace(
            "        Shape::E => 5,\n",
            "        Shape::E => 5,\n        _ => 0,\n",
        );
        assert_eq!(flagged(&planted), [(5, Some("a wildcard arm"))]);
    }

    #[test]
    fn a_binding_arm_is_a_catch_all() {
        let planted = TOTAL.replace(
            "        Shape::E => 5,\n",
            "        Shape::E => 5,\n        other => other.len(),\n",
        );
        assert_eq!(flagged(&planted), [(5, Some("a binding catch-all arm"))]);
    }

    #[test]
    fn a_guarded_wildcard_is_not_a_catch_all() {
        // The exhaustiveness check ignores guards, so a variant added later
        // is still a compile error.
        let planted = TOTAL.replace(
            "        Shape::E => 5,\n",
            "        Shape::E => 5,\n        _ if true => 0,\n",
        );
        assert_eq!(flagged(&planted), [(5, None)]);
    }

    #[test]
    fn a_lookup_may_default() {
        let lookup = "fn g(s: Shape) -> bool {\n    match s {\n        Shape::A | Shape::B => true,\n        _ => false,\n    }\n}\n";
        assert_eq!(flagged(lookup), []);
    }

    #[test]
    fn self_paths_count_inside_an_impl_of_the_enum() {
        let source = "impl Shape {\n    fn f(self) -> u8 {\n        match self {\n            Self::A => 1,\n            Self::B => 2,\n            Self::C => 3,\n            Self::D => 4,\n            Self::E => 5,\n            _ => 0,\n        }\n    }\n}\n";
        assert_eq!(flagged(source), [(5, Some("a wildcard arm"))]);
        let elsewhere = source.replace("impl Shape", "impl Other");
        assert_eq!(flagged(&elsewhere), []);
    }

    #[test]
    fn prose_and_literals_do_not_count() {
        let source = TOTAL.replace(
            "        Shape::E => 5,\n",
            "        Shape::E => 5, // _ => 0,\n        // other => 1,\n",
        );
        assert_eq!(flagged(&source), [(5, None)]);
        let in_string = TOTAL.replace("1", "\"_ => 0, other =>\"");
        assert_eq!(flagged(&in_string), [(5, None)]);
    }

    #[test]
    fn block_bodies_and_nested_matches_are_read_per_match() {
        let source = "fn f(s: Shape, t: u8) -> u8 {\n    match s {\n        Shape::A => {\n            match t {\n                0 => 1,\n                _ => 2,\n            }\n        }\n        Shape::B => 2,\n        Shape::C => 3,\n        Shape::D => 4,\n        Shape::E => { 5 }\n    }\n}\n";
        assert_eq!(flagged(source), [(5, None)]);
    }

    #[test]
    fn a_tuple_pairing_is_not_a_classification() {
        let source = "fn f(s: Shape, t: Shape) -> u8 {\n    match (s, t) {\n        (Shape::A, _) => 1,\n        (Shape::B, _) => 2,\n        (Shape::C, _) => 3,\n        (Shape::D, _) => 4,\n        (Shape::E, _) => 5,\n        _ => 0,\n    }\n}\n";
        assert_eq!(flagged(source), []);
    }

    #[test]
    fn blanking_keeps_every_offset() {
        let source = "let a = r#\"x \" y\"#; // c\nlet b = \"s\\\"t\"; /* d\ne */ let c = '\\'';\n";
        assert_eq!(blank(source).len(), source.chars().count());
    }

    #[test]
    fn a_catch_all_is_planted_in_the_named_function() {
        let source = format!("fn other() {{}}\n{TOTAL}");
        let planted = plant_catch_all(&source, &ENUMS, "f").expect("a classification in f");
        assert_eq!(flagged(&planted), [(5, Some("a wildcard arm"))]);
        assert_eq!(plant_catch_all(&source, &ENUMS, "other"), None);
        assert_eq!(plant_catch_all(&source, &ENUMS, "absent"), None);
    }

    /// A total classification for every consumer, and a catch-all in each
    /// identity-bound one.
    fn consumers() -> Vec<(String, String)> {
        CONSUMERS
            .iter()
            .map(|(path, _)| {
                let enumeration = if path.contains("/calculus/") {
                    "Shape"
                } else {
                    "SemanticType"
                };
                let mut text = TOTAL.replace("Shape", enumeration);
                if IDENTITY_BOUND.contains(path) {
                    text = text.replace(
                        &format!("{enumeration}::E => 5,"),
                        &format!("{enumeration}::E => 5,\n        _ => 0,"),
                    );
                }
                ((*path).to_owned(), text)
            })
            .collect()
    }

    #[test]
    fn the_audit_names_the_file_and_arms_the_consumers() {
        let sources = consumers();
        let summary = audit(&sources).expect("a total classification in every consumer");
        assert_eq!(summary.classifications(), CONSUMERS.len());
        assert_eq!(summary.tolerated.len(), IDENTITY_BOUND.len());
        let mut planted = sources.clone();
        planted[0].1 = planted[0].1.replace(
            "SemanticType::E => 5,",
            "SemanticType::E => 5,\n        _ => 6,",
        );
        let error = audit(&planted).expect_err("a wildcard is found");
        assert!(error.contains(CONSUMERS[0].0), "{error}");
        let mut vacated = sources;
        vacated[1].1 = "fn nothing() {}\n".to_owned();
        let error = audit(&vacated).expect_err("a consumer with no classification fails");
        assert!(error.contains(CONSUMERS[1].0), "{error}");
        assert!(error.contains("would pass over"), "{error}");
    }

    #[test]
    fn an_identity_bound_source_is_tolerated_only_while_it_needs_to_be() {
        let mut sources = consumers();
        let at = sources
            .iter()
            .position(|(path, _)| path == IDENTITY_BOUND[0])
            .expect("a consumer");
        sources[at].1 = TOTAL.replace("Shape", "Ty");
        let error = audit(&sources).expect_err("a clean source is removed from the list");
        assert!(error.contains(IDENTITY_BOUND[0]), "{error}");
        assert!(error.contains("remove it from IDENTITY_BOUND"), "{error}");
    }
}
