//! `cargo xtask spec`: edit the chapter modules of `spec/` by item (`NS-07`).
//!
//! A thin command over `repo_model::spec_edit`, which does the editing through
//! the authoring SDK and refuses unless the regenerated SPEC.md is the intended
//! text. This writes the changed modules and then the projections, as
//! `model-write` does, so a person has one command and the tree is consistent
//! when it ends.

use std::path::Path;

use repo_model::spec_edit::{self, InsertKind, Outcome};

use crate::Fail;

const USAGE: &str = "cargo xtask spec <operation> [--dry-run]\n\
    \n\
    replace-block --chapter ChapterNN --first-line <text> --from <file>\n\
        replace the paragraph, table or fence that begins with <text> by the lines of <file>\n\
    insert-after --chapter ChapterNN --first-line <text> --kind para|table --from <file>\n\
        insert the lines of <file> as a block after the block that begins with <text>\n\
    port --merged <SPEC.md>\n\
        apply each hunk of a merged SPEC.md that is outside the derived regions\n\
    \n\
    --dry-run  verify the edit on a scratch copy and write nothing";

fn option<'a>(arguments: &'a [String], name: &str) -> Result<&'a str, Fail> {
    arguments
        .iter()
        .position(|argument| argument == name)
        .and_then(|at| arguments.get(at + 1))
        .map(String::as_str)
        .ok_or_else(|| format!("missing {name}\n\n{USAGE}").into())
}

fn lines_of_file(path: &str) -> Result<Vec<String>, Fail> {
    let text = std::fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    Ok(text
        .strip_suffix('\n')
        .unwrap_or(&text)
        .split('\n')
        .map(str::to_owned)
        .collect())
}

/// Run the command on the arguments after `spec`.
pub fn run(root: &Path, arguments: &[String]) -> Result<(), Fail> {
    let dry_run = arguments.iter().any(|argument| argument == "--dry-run");
    let Some(operation) = arguments.first() else {
        return Err(USAGE.to_owned().into());
    };
    let outcome: Outcome = match operation.as_str() {
        "replace-block" => spec_edit::replace_block(
            root,
            option(arguments, "--chapter")?,
            option(arguments, "--first-line")?,
            &lines_of_file(option(arguments, "--from")?)?,
        )?,
        "insert-after" => {
            let kind = match option(arguments, "--kind")? {
                "para" => InsertKind::Paragraph,
                "table" => InsertKind::Table,
                other => return Err(format!("--kind is para or table, not `{other}`").into()),
            };
            spec_edit::insert_after(
                root,
                option(arguments, "--chapter")?,
                option(arguments, "--first-line")?,
                kind,
                &lines_of_file(option(arguments, "--from")?)?,
            )?
        }
        "port" => {
            let merged = std::fs::read_to_string(option(arguments, "--merged")?)?;
            spec_edit::port(root, &merged)?
        }
        _ => return Err(USAGE.to_owned().into()),
    };
    for line in &outcome.report {
        println!("spec: {line}");
    }
    if dry_run {
        println!(
            "spec: verified on a scratch copy ({} module(s) would change); nothing written",
            outcome.changes.len()
        );
        return Ok(());
    }
    spec_edit::apply(root, &outcome)?;
    crate::codegen::check_model(root, true)?;
    println!(
        "spec: wrote {} module(s) and regenerated SPEC.md and the model files (NS-07)",
        outcome.changes.len()
    );
    Ok(())
}
