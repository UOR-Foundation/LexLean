//! The statement-vocabulary audit (SPEC.md §17.17).
//!
//! What certificates A, B, and E state is written in definitions of the
//! preservation library: the observations and their convergence (`Obs`,
//! `Rel`, `Conv`, `FunRel`, `RunConv`), the value typing the rendering
//! relies on (`WT`), how a rendering realizes an observation (`Realizes`,
//! `RealizesFn`), the simulation statement (`FunSem`), and the arguments E
//! quantifies over (`Representable`). A reader trusts a theorem only as far
//! as they trust those definitions, so the audit module of every
//! verification prints them, after the axioms, as the pinned Lean states them
//! (`#print` for a definition, `#check @` for a theorem), and SPEC.md quotes
//! that output in fenced blocks tagged `lean-library`. The library is native
//! kernel data, so there is no source text to quote: the output of Lean over
//! the generated library is the statement, and this audit requires the quoted
//! blocks, taken together, to equal it exactly. A definition changed in the
//! library but not in the specification, or the reverse, is refused.

/// The audit output of the committed `production` example: the file whose
/// statement-vocabulary section the specification quotes.
pub const AUDIT_OUTPUT: &str = "examples/production/expected/verify/preserve/audit.txt";

/// The line of the audit output that begins the statement vocabulary.
pub const MARKER: &str = "-- statement vocabulary";

/// The declarations the audit module prints, in order, each with the first
/// line its statement begins with.
pub const REQUIRED: [(&str, &str); 17] = [
    (
        "LexLeanPreservation.Obs",
        "inductive LexLeanPreservation.Obs :",
    ),
    ("LexLeanPreservation.Rel", "def LexLeanPreservation.Rel :"),
    ("LexLeanPreservation.Conv", "def LexLeanPreservation.Conv :"),
    (
        "LexLeanPreservation.FunRel",
        "def LexLeanPreservation.FunRel :",
    ),
    (
        "LexLeanPreservation.RunConv",
        "def LexLeanPreservation.RunConv :",
    ),
    (
        "LexLeanPreservation.Rust.WT",
        "def LexLeanPreservation.Rust.WT :",
    ),
    (
        "LexLeanPreservation.Rust.WTL",
        "def LexLeanPreservation.Rust.WTL :",
    ),
    (
        "LexLeanPreservation.Rust.fnIdent",
        "def LexLeanPreservation.Rust.fnIdent :",
    ),
    (
        "LexLeanPreservation.Rust.fnFallible",
        "def LexLeanPreservation.Rust.fnFallible :",
    ),
    (
        "LexLeanPreservation.Rust.RCI",
        "def LexLeanPreservation.Rust.RCI :",
    ),
    (
        "LexLeanPreservation.Rust.Realizes",
        "def LexLeanPreservation.Rust.Realizes :",
    ),
    (
        "LexLeanPreservation.Rust.RealizesFn",
        "def LexLeanPreservation.Rust.RealizesFn :",
    ),
    (
        "LexLeanPreservation.Rust.FunSem",
        "def LexLeanPreservation.Rust.FunSem :",
    ),
    (
        "LexLeanPreservation.someObs",
        "def LexLeanPreservation.someObs :",
    ),
    (
        "LexLeanPreservation.Rust.Representable",
        "def LexLeanPreservation.Rust.Representable :",
    ),
    (
        "LexLeanPreservation.Rust.RepresentableL",
        "def LexLeanPreservation.Rust.RepresentableL :",
    ),
    (
        "LexLeanPreservation.Rust.runItem_abort_infallible",
        "LexLeanPreservation.Rust.runItem_abort_infallible :",
    ),
];

/// The statement vocabulary an audit output holds, one statement per
/// required declaration, in order.
///
/// # Errors
///
/// Returns a reason when the output has no vocabulary section, or lacks the
/// statement of a required declaration, or states them out of order.
pub fn statements(audit_output: &str) -> Result<Vec<String>, String> {
    let at = audit_output
        .find(MARKER)
        .ok_or_else(|| format!("the audit output has no `{MARKER}` section"))?;
    let section = &audit_output[at + MARKER.len()..];
    let mut starts = Vec::with_capacity(REQUIRED.len());
    let mut from = 0;
    for (name, head) in REQUIRED {
        let found = section[from..]
            .match_indices(head)
            .map(|(offset, _)| from + offset)
            .find(|offset| *offset == 0 || section.as_bytes()[*offset - 1] == b'\n')
            .ok_or_else(|| {
                format!("the audit output does not state `{name}` after its predecessor")
            })?;
        starts.push(found);
        from = found + head.len();
    }
    starts.push(section.len());
    Ok(starts
        .windows(2)
        .map(|pair| section[pair[0]..pair[1]].trim().to_owned())
        .collect())
}

/// The quoted blocks of the specification: the body of each `lean-library`
/// fence, trimmed.
fn blocks(spec: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut lines = spec.lines();
    while let Some(line) = lines.next() {
        if line.trim() != "```lean-library" {
            continue;
        }
        let mut body = Vec::new();
        for line in lines.by_ref() {
            if line.trim() == "```" {
                break;
            }
            body.push(line);
        }
        out.push(body.join("\n").trim().to_owned());
    }
    out
}

/// Audit the specification's quotations against an audit output. Returns the
/// number of declarations quoted.
///
/// # Errors
///
/// Returns every disagreement, one per line: no quoted block, a statement
/// the output does not hold, or text between the blocks that is no
/// statement.
pub fn audit(spec: &str, audit_output: &str) -> Result<usize, String> {
    let blocks = blocks(spec);
    if blocks.is_empty() {
        return Err(
            "SPEC.md quotes no `lean-library` block: the statement vocabulary is not specified"
                .to_owned(),
        );
    }
    let statements = statements(audit_output)?;
    let expected = statements.join("\n");
    let quoted = blocks.join("\n");
    if quoted == expected {
        return Ok(statements.len());
    }
    let mut problems = Vec::new();
    for (statement, (name, _)) in statements.iter().zip(REQUIRED) {
        if !quoted.contains(statement.as_str()) {
            problems.push(format!(
                "SPEC.md does not quote the statement of `{name}` as Lean prints it"
            ));
        }
    }
    if problems.is_empty() {
        problems.push(
            "SPEC.md quotes the statement vocabulary out of order, or with text Lean did not print"
                .to_owned(),
        );
    }
    Err(problems.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::{audit, statements, MARKER, REQUIRED};

    fn output(edit: impl Fn(&str) -> String) -> String {
        let mut out = format!("'x' depends on axioms: [propext]\n{MARKER}\n");
        for (name, head) in REQUIRED {
            out.push_str(&edit(&format!("{head} Nat :=\n  1 -- {name}\n")));
        }
        out
    }

    fn spec(output: &str, split: usize) -> String {
        let all = statements(output).expect("statements");
        let mut text = String::from("prose\n\n");
        for part in all.chunks(split) {
            text.push_str("```lean-library\n");
            text.push_str(&part.join("\n"));
            text.push_str("\n```\n\nmore prose\n\n");
        }
        text
    }

    #[test]
    fn the_quotation_equals_the_printed_vocabulary() {
        let printed = output(str::to_owned);
        for split in [1, 4, REQUIRED.len()] {
            assert_eq!(audit(&spec(&printed, split), &printed), Ok(REQUIRED.len()));
        }
    }

    #[test]
    fn every_drift_between_quotation_and_library_is_refused() {
        let printed = output(str::to_owned);
        let good = spec(&printed, 4);
        // The library changed and the specification did not.
        let changed = output(|text| text.replace("  1", "  2"));
        let report = audit(&good, &changed).expect_err("a changed definition");
        assert!(report.contains("does not quote the statement"), "{report}");
        // The specification lost a statement.
        let missing = good.replacen(&statements(&printed).expect("statements")[0], "", 1);
        let report = audit(&missing, &printed).expect_err("a missing quotation");
        assert!(report.contains("`LexLeanPreservation.Obs`"), "{report}");
        // The specification states one out of order.
        let all = statements(&printed).expect("statements");
        let mut swapped = all.clone();
        swapped.swap(0, 1);
        let reordered = format!("```lean-library\n{}\n```\n", swapped.join("\n"));
        let report = audit(&reordered, &printed).expect_err("an order drift");
        assert!(report.contains("out of order"), "{report}");
        // No block, and no vocabulary section.
        let report = audit("no block", &printed).expect_err("no block");
        assert!(report.contains("quotes no"), "{report}");
        let report = audit(&good, "no section").expect_err("no section");
        assert!(
            report.contains("no `-- statement vocabulary` section"),
            "{report}"
        );
        // A statement the audit module no longer prints.
        let dropped = printed.replace("def LexLeanPreservation.Rel :", "def Elsewhere :");
        let report = audit(&good, &dropped).expect_err("an unprinted statement");
        assert!(report.contains("LexLeanPreservation.Rel"), "{report}");
    }
}
