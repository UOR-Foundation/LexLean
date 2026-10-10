//! The certificate-B rule-set audit (SPEC.md §17.17).
//!
//! Certificate B is a derivation of the library's correspondence `Corr`,
//! which the aligner writes rule by rule. The rule set is closed in three
//! places that nothing else ties together, so this audit compares their
//! texts: the aligner's declared rules are exactly `Corr`'s constructors,
//! each of them is emitted somewhere in the aligner and is a case of the
//! soundness theorem's induction, and the aligner's block judgment names
//! every calculus construct with no wildcard arm. A rule the library does
//! not prove, a constructor the aligner never writes, or a construct the
//! aligner meets only by default would each break that closure.

use std::collections::BTreeSet;
use std::fmt::Write as _;

/// The aligner.
pub const ALIGNER_SOURCE: &str = "crates/lexlean/src/production/rust_cert.rs";

/// The library module declaring the correspondence.
pub const CORRESPONDENCE_SOURCE: &str =
    "language/preservation-1.2/library/LexLeanPreservation/RustCorr.lean";

/// The library module proving it sound.
pub const SOUNDNESS_SOURCE: &str =
    "language/preservation-1.2/library/LexLeanPreservation/RustSound.lean";

/// The calculus, whose expression constructs the aligner must name.
pub const CALCULUS_SOURCE: &str = "crates/lexlean/src/calculus/mod.rs";

/// The constructors of `inductive Corr`, in declaration order.
///
/// # Errors
///
/// Returns a reason when the inductive is absent or declares no
/// constructor.
pub fn constructors(lean: &str) -> Result<Vec<String>, String> {
    let mut lines = lean
        .lines()
        .skip_while(|line| !line.starts_with("inductive Corr "));
    if lines.next().is_none() {
        return Err(format!(
            "{CORRESPONDENCE_SOURCE}: `inductive Corr` is absent"
        ));
    }
    let mut out = Vec::new();
    for line in lines {
        if let Some(rest) = line.strip_prefix("  | ") {
            out.push(
                rest.chars()
                    .take_while(|character| character.is_ascii_alphanumeric())
                    .collect(),
            );
        } else if !line.starts_with("    ") && !line.trim().is_empty() {
            break;
        }
    }
    if out.is_empty() {
        return Err(format!(
            "{CORRESPONDENCE_SOURCE}: `Corr` declares no constructor"
        ));
    }
    Ok(out)
}

/// The cases of the soundness theorem's induction on a derivation.
///
/// # Errors
///
/// Returns a reason when `theorem sound` is absent.
pub fn sound_cases(lean: &str) -> Result<Vec<String>, String> {
    let start = lean
        .find("theorem sound ")
        .ok_or_else(|| format!("{SOUNDNESS_SOURCE}: `theorem sound` is absent"))?;
    let body = &lean[start..];
    let body = &body[..body.find("\ntheorem ").unwrap_or(body.len())];
    let induction = body
        .find("induction h with")
        .ok_or_else(|| format!("{SOUNDNESS_SOURCE}: `sound` has no induction on the derivation"))?;
    Ok(body[induction..]
        .lines()
        .skip(1)
        .filter_map(|line| line.trim_start().strip_prefix("| "))
        .map(|rest| {
            rest.chars()
                .take_while(|character| character.is_ascii_alphanumeric())
                .collect()
        })
        .collect())
}

/// The rules the aligner declares in its `RULES` constant.
///
/// # Errors
///
/// Returns a reason when the constant is absent.
pub fn declared_rules(rust: &str) -> Result<Vec<String>, String> {
    let start = rust
        .find("pub const RULES: [&str; ")
        .ok_or_else(|| format!("{ALIGNER_SOURCE}: `pub const RULES` is absent"))?;
    let body = &rust[start..];
    let open = body.find("= [").ok_or("`RULES` is not a list")?;
    let close = body.find("];").ok_or("`RULES` is not closed")?;
    Ok(body[open + 3..close]
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| item.trim_matches('"').to_owned())
        .collect())
}

/// The string literals of Rust source outside its `RULES` constant and its
/// comments.
fn literals(rust: &str) -> BTreeSet<String> {
    let start = rust.find("pub const RULES: [&str; ").unwrap_or(rust.len());
    let end = rust[start..]
        .find("];")
        .map_or(rust.len(), |close| start + close);
    let code = format!("{}{}", &rust[..start], &rust[end.min(rust.len())..]);
    let mut out = BTreeSet::new();
    for line in code.lines() {
        let line = line.split("//").next().unwrap_or_default();
        let mut parts = line.split('"');
        let _ = parts.next();
        while let (Some(inside), Some(_)) = (parts.next(), parts.next()) {
            out.insert(inside.to_owned());
        }
    }
    out
}

/// The variants of `pub enum Expr` of the calculus.
///
/// # Errors
///
/// Returns a reason when the enum is absent.
pub fn calculus_constructs(calculus: &str) -> Result<Vec<String>, String> {
    let mut lines = calculus
        .lines()
        .skip_while(|line| line.trim() != "pub enum Expr {");
    if lines.next().is_none() {
        return Err(format!("{CALCULUS_SOURCE}: `pub enum Expr` is absent"));
    }
    let mut out = Vec::new();
    for line in lines {
        if line == "}" {
            break;
        }
        if let Some(rest) = line.strip_prefix("    ") {
            if rest.starts_with(|character: char| character.is_ascii_uppercase()) {
                out.push(
                    rest.chars()
                        .take_while(|character| character.is_ascii_alphanumeric())
                        .collect(),
                );
            }
        }
    }
    Ok(out)
}

/// The text of the aligner's block judgment, `fn b(`, up to the next
/// method.
fn block_judgment(rust: &str) -> Result<&str, String> {
    let start = rust
        .find("    fn b(\n")
        .ok_or_else(|| format!("{ALIGNER_SOURCE}: the block judgment `fn b` is absent"))?;
    let body = &rust[start..];
    let end = body[1..]
        .find("\n    fn ")
        .map_or(body.len(), |next| next + 1);
    Ok(&body[..end])
}

/// Whether a line of a match is an arm that meets anything: a wildcard, or
/// a lone binding, either of which would let a construct the judgment never
/// names fall through to a rule written for another.
fn catch_all(line: &str) -> bool {
    let compact: String = line.split_whitespace().collect::<Vec<_>>().join(" ");
    let pattern = compact.trim_start_matches("| ");
    let Some(pattern) = pattern
        .split(" =>")
        .next()
        .filter(|_| pattern.contains(" =>"))
    else {
        return false;
    };
    let pattern = pattern.split(" if ").next().unwrap_or(pattern);
    let pattern = pattern
        .trim_start_matches("ref ")
        .trim_start_matches("mut ");
    pattern == ".."
        || (!pattern.is_empty()
            && pattern.starts_with(|first: char| first == '_' || first.is_ascii_lowercase())
            && pattern
                .chars()
                .all(|character| character == '_' || character.is_ascii_alphanumeric()))
}

/// Audit the rule set across the aligner, the correspondence, and its
/// soundness theorem.
///
/// # Errors
///
/// Returns every violation, one per line.
pub fn audit(
    rust: &str,
    correspondence: &str,
    soundness: &str,
    calculus: &str,
) -> Result<(), String> {
    let mut report = String::new();
    let constructors = constructors(correspondence)?;
    let declared = declared_rules(rust)?;
    let cases = sound_cases(soundness)?;
    let constructor_set: BTreeSet<&String> = constructors.iter().collect();
    let declared_set: BTreeSet<&String> = declared.iter().collect();
    let case_set: BTreeSet<&String> = cases.iter().collect();
    if constructor_set.len() != constructors.len() {
        let _ = writeln!(
            report,
            "{CORRESPONDENCE_SOURCE}: a constructor is declared twice"
        );
    }
    if declared_set.len() != declared.len() {
        let _ = writeln!(report, "{ALIGNER_SOURCE}: a rule is declared twice");
    }
    for name in constructor_set.difference(&declared_set) {
        let _ = writeln!(
            report,
            "{ALIGNER_SOURCE}: the correspondence's rule `{name}` is not among the aligner's rules"
        );
    }
    for name in declared_set.difference(&constructor_set) {
        let _ = writeln!(
            report,
            "{ALIGNER_SOURCE}: the rule `{name}` is not a constructor of the correspondence"
        );
    }
    for name in constructor_set.difference(&case_set) {
        let _ = writeln!(
            report,
            "{SOUNDNESS_SOURCE}: the rule `{name}` is not a case of the soundness theorem"
        );
    }
    for name in case_set.difference(&constructor_set) {
        let _ = writeln!(
            report,
            "{SOUNDNESS_SOURCE}: the soundness case `{name}` is no rule of the correspondence"
        );
    }
    let written = literals(rust);
    for name in &declared {
        if !written.contains(name) {
            let _ = writeln!(
                report,
                "{ALIGNER_SOURCE}: the rule `{name}` is never written"
            );
        }
    }
    let judgment = block_judgment(rust)?;
    for construct in calculus_constructs(calculus)? {
        let path = format!("Term::{construct}");
        let named = judgment.match_indices(path.as_str()).any(|(at, _)| {
            !judgment[at + path.len()..]
                .chars()
                .next()
                .is_some_and(|next| next.is_ascii_alphanumeric() || next == '_')
        });
        if !named {
            let _ = writeln!(
                report,
                "{ALIGNER_SOURCE}: the block judgment does not name the construct `{path}`"
            );
        }
    }
    if judgment.lines().any(catch_all) {
        let _ = writeln!(
            report,
            "{ALIGNER_SOURCE}: the block judgment has a wildcard arm, which would meet a construct by default"
        );
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

    const CORR: &str = "inductive Corr (p : P) : J → Prop where\n  | var {x} : A →\n      B\n  | lNil : C\n\nend X\n";
    const SOUND: &str = "theorem sound :\n  induction h with\n        | var hx => x\n        | lNil => y\n\ntheorem other : 1 = 1 := rfl\n";
    const CALC: &str = "pub enum Expr {\n    Var {\n        name: u64,\n    },\n    Nil,\n}\n";
    const RUST: &str = "pub const RULES: [&str; 2] = [\"var\", \"lNil\"];\nimpl A {\n    fn b(\n        x: u8,\n    ) {\n        match term {\n            Term::Var { .. } => rule(\"var\"),\n            Term::Nil => rule(\"lNil\"),\n        }\n    }\n    fn c() {}\n}\n";

    #[test]
    fn a_closed_rule_set_passes() {
        audit(RUST, CORR, SOUND, CALC).unwrap();
        assert_eq!(constructors(CORR).unwrap(), ["var", "lNil"]);
        assert_eq!(sound_cases(SOUND).unwrap(), ["var", "lNil"]);
    }

    #[test]
    fn every_break_of_the_closure_fails() {
        let plant = |rust: &str, corr: &str, sound: &str, calc: &str, expected: &str| {
            let report = audit(rust, corr, sound, calc).unwrap_err();
            assert!(report.contains(expected), "{expected}: {report}");
        };
        plant(
            &RUST.replace("\"var\", ", ""),
            CORR,
            SOUND,
            CALC,
            "`var` is not among the aligner's rules",
        );
        plant(
            RUST,
            &CORR.replace("  | lNil : C\n", "  | lNil : C\n  | extra : D\n"),
            SOUND,
            CALC,
            "`extra` is not among the aligner's rules",
        );
        plant(
            &RUST.replace(
                "[&str; 2] = [\"var\", \"lNil\"]",
                "[&str; 3] = [\"var\", \"lNil\", \"var\"]",
            ),
            CORR,
            SOUND,
            CALC,
            "declared twice",
        );
        plant(
            RUST,
            CORR,
            &SOUND.replace("        | lNil => y\n", ""),
            CALC,
            "`lNil` is not a case of the soundness",
        );
        plant(
            RUST,
            CORR,
            &SOUND.replace(
                "        | lNil => y\n",
                "        | lNil => y\n        | stray => z\n",
            ),
            CALC,
            "`stray` is no rule",
        );
        plant(
            &RUST.replace("rule(\"lNil\")", "rule(x)"),
            CORR,
            SOUND,
            CALC,
            "`lNil` is never written",
        );
        plant(
            &RUST.replace(
                "            Term::Nil => rule(\"lNil\"),\n",
                "            _ => rule(\"lNil\"),\n",
            ),
            CORR,
            SOUND,
            CALC,
            "wildcard arm",
        );
        plant(
            &RUST.replace(
                "            Term::Nil => rule(\"lNil\"),\n",
                "            Term::Nil => rule(\"lNil\"),\n            other => rule(\"lNil\"),\n",
            ),
            CORR,
            SOUND,
            CALC,
            "wildcard arm",
        );
        plant(
            RUST,
            CORR,
            SOUND,
            &CALC.replace("    Nil,\n", "    Nil,\n    Cons,\n"),
            "`Term::Cons`",
        );
        plant(
            &RUST.replace("pub const RULES", "const RULES"),
            CORR,
            SOUND,
            CALC,
            "`pub const RULES` is absent",
        );
    }
}
