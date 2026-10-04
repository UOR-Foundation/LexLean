//! The statement-vocabulary audit (SPEC.md §17.17).
//!
//! What certificates A, B, and E state is written in definitions of the
//! preservation library that are hand-written, not generated: the
//! observations and their convergence (`Obs`, `Rel`, `Conv`, `FunRel`,
//! `RunConv`), the value typing the rendering relies on (`WT`), how a
//! rendering realizes an observation (`Realizes`, `RealizesFn`), the
//! simulation statement (`FunSem`), and the arguments E quantifies over
//! (`Representable`). A reader trusts a theorem only as far as they trust
//! those definitions, so SPEC.md quotes them, in fenced blocks tagged
//! `lean-library`, and this audit requires each quoted declaration to be
//! byte-equal to a declaration of the library file the block names. A
//! definition changed in the library but not in the specification, or the
//! reverse, is refused.

use std::collections::BTreeSet;

/// The directory of the library, relative to the repository root.
pub const LIBRARY_DIR: &str = "language/preservation-1.2/library";

/// The declarations the specification must quote, by library file and name.
pub const REQUIRED: [(&str, &str); 10] = [
    ("LexLeanPreservation/Core.lean", "Obs"),
    ("LexLeanPreservation/Core.lean", "Rel"),
    ("LexLeanPreservation/Core.lean", "Conv"),
    ("LexLeanPreservation/Core.lean", "FunRel"),
    ("LexLeanPreservation/RustBase.lean", "WT"),
    ("LexLeanPreservation/RustCorr.lean", "Realizes"),
    ("LexLeanPreservation/RustCorr.lean", "RealizesFn"),
    ("LexLeanPreservation/RustCorr.lean", "FunSem"),
    ("LexLeanPreservation/Compose.lean", "Representable"),
    (
        "LexLeanPreservation/Compose.lean",
        "runItem_abort_infallible",
    ),
];

/// The blank-line-separated declarations of a Lean source.
fn chunks(text: &str) -> Vec<&str> {
    text.split("\n\n")
        .map(str::trim)
        .filter(|chunk| !chunk.is_empty())
        .collect()
}

/// Whether a chunk declares `name`: some line of it starts a declaration of
/// that name.
fn declares(chunk: &str, name: &str) -> bool {
    chunk.lines().any(|line| {
        ["def ", "theorem ", "inductive ", "structure ", "abbrev "]
            .iter()
            .filter_map(|keyword| line.strip_prefix(keyword))
            .any(|rest| {
                rest.strip_prefix(name).is_some_and(|after| {
                    after.starts_with([' ', '\n', ':', '(']) || after.is_empty()
                })
            })
    })
}

/// The quoted blocks of the specification: for each `lean-library` fence,
/// its first line's file and its declarations.
fn blocks(spec: &str) -> Vec<(String, Vec<String>)> {
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
        let Some(first) = body.first() else { continue };
        let file = first.trim_start_matches("-- ").trim().to_owned();
        let text = body[1..].join("\n");
        out.push((file, chunks(&text).into_iter().map(str::to_owned).collect()));
    }
    out
}

/// Audit the specification's quotations against the library's files, read
/// by `read` from their path under [`LIBRARY_DIR`]. Returns the number of
/// declarations quoted.
///
/// # Errors
///
/// Returns every disagreement, one per line: a quoted declaration absent from
/// its file, a file the library does not have, or a required declaration the
/// specification does not quote.
pub fn audit(spec: &str, read: &dyn Fn(&str) -> Option<String>) -> Result<usize, String> {
    let mut problems = Vec::new();
    let mut quoted: BTreeSet<(String, String)> = BTreeSet::new();
    let mut count = 0;
    let blocks = blocks(spec);
    if blocks.is_empty() {
        problems.push(
            "SPEC.md quotes no `lean-library` block: the statement vocabulary is not specified"
                .to_owned(),
        );
    }
    for (file, declarations) in &blocks {
        let Some(source) = read(file) else {
            problems.push(format!(
                "SPEC.md quotes `{file}`, which the library does not have"
            ));
            continue;
        };
        let actual = chunks(&source);
        for declaration in declarations {
            count += 1;
            if !actual.contains(&declaration.as_str()) {
                let head = declaration.lines().next().unwrap_or_default();
                problems.push(format!(
                    "SPEC.md quotes a declaration of `{file}` that the file does not state byte for byte: `{head}`"
                ));
            }
            for (required_file, name) in REQUIRED {
                if required_file == file && declares(declaration, name) {
                    quoted.insert((file.clone(), name.to_owned()));
                }
            }
        }
    }
    for (file, name) in REQUIRED {
        if !quoted.contains(&(file.to_owned(), name.to_owned())) {
            problems.push(format!(
                "SPEC.md does not quote the declaration `{name}` of `{file}`"
            ));
        }
    }
    if problems.is_empty() {
        Ok(count)
    } else {
        Err(problems.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::{audit, REQUIRED};

    fn library(file: &str) -> Option<String> {
        let mut text = String::new();
        for (path, name) in REQUIRED {
            if path == file {
                text.push_str(&format!("def {name} : Nat :=\n  1\n\n"));
            }
        }
        (!text.is_empty()).then_some(text)
    }

    fn spec(edit: impl Fn(&str) -> String) -> String {
        let mut out = String::new();
        let files: std::collections::BTreeSet<&str> = REQUIRED.iter().map(|(f, _)| *f).collect();
        for file in files {
            out.push_str("```lean-library\n");
            out.push_str(&format!("-- {file}\n"));
            for (path, name) in REQUIRED {
                if path == file {
                    out.push_str(&edit(&format!("def {name} : Nat :=\n  1\n\n")));
                }
            }
            out.push_str("```\n\n");
        }
        out
    }

    /// The quotation of a library with every required definition passes,
    /// and each way it can drift is refused for what it is.
    #[test]
    fn drift_in_the_quotation_is_refused() {
        let good = spec(str::to_owned);
        assert_eq!(audit(&good, &library), Ok(REQUIRED.len()));
        let changed = spec(|text| text.replace("  1", "  2"));
        let report = audit(&changed, &library).expect_err("a changed definition");
        assert!(report.contains("byte for byte"), "{report}");
        let missing = good.replacen("def Obs : Nat :=\n  1\n\n", "", 1);
        let report = audit(&missing, &library).expect_err("a missing quotation");
        assert!(
            report.contains("does not quote the declaration `Obs`"),
            "{report}"
        );
        let foreign = good.replacen(
            "-- LexLeanPreservation/Core.lean",
            "-- LexLeanPreservation/Gone.lean",
            1,
        );
        let report = audit(&foreign, &library).expect_err("a missing file");
        assert!(report.contains("does not have"), "{report}");
        let report = audit("no block", &library).expect_err("no block");
        assert!(report.contains("quotes no"), "{report}");
    }
}
