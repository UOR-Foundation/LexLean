//! The execution side of the gate (SPEC.md §9.2, §9.4, §22.12): the test
//! inventory, the digest of a verified directory, and the evidence summary.
//!
//! Nothing here changes what `just vv` accepts. These tasks check how the gate
//! is run, or report it; the reports go to a directory of CI evidence and are
//! never read back by a recipe.

use std::collections::BTreeSet;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use repo_conformance::{evidence, inventory};
use serde_json::Value;

use crate::Fail;

/// The first command of a recipe of the Justfile.
fn recipe_command(justfile: &str, name: &str) -> Result<String, Fail> {
    let mut in_recipe = false;
    for line in justfile.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            if in_recipe {
                return Ok(line.trim().trim_start_matches('@').to_owned());
            }
        } else {
            in_recipe = line
                .split_once(':')
                .is_some_and(|(recipe, _)| recipe.trim() == name);
        }
    }
    Err(format!("RP-13: the Justfile has no recipe `{name}` with a command").into())
}

/// Run `cargo test <arguments> -- --list` and return what it wrote, stdout
/// and stderr in the order they were written: `cargo` announces each target on
/// stderr and the harness lists its tests on stdout, so the two have to share
/// one file to be read together.
fn list_tests(root: &Path, arguments: &[String]) -> Result<inventory::Parsed, Fail> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut file = tempfile::tempfile()?;
    let status = Command::new(cargo)
        .current_dir(root)
        .env("CARGO_TERM_COLOR", "never")
        .arg("test")
        .args(arguments)
        .args(["--", "--list"])
        .stdin(Stdio::null())
        .stdout(Stdio::from(file.try_clone()?))
        .stderr(Stdio::from(file.try_clone()?))
        .status()?;
    file.seek(SeekFrom::Start(0))?;
    let mut output = String::new();
    file.read_to_string(&mut output)?;
    if !status.success() {
        let tail: Vec<&str> = output.lines().rev().take(20).collect();
        return Err(format!(
            "RP-13: `cargo test {} -- --list` failed:\n{}",
            arguments.join(" "),
            tail.into_iter().rev().collect::<Vec<_>>().join("\n")
        )
        .into());
    }
    inventory::parse(&output).map_err(Into::into)
}

/// The names of the binaries that carry the tests of `repo-conformance`:
/// its library and each of its test targets, from `cargo metadata`.
fn owned_crates(root: &Path) -> Result<BTreeSet<String>, Fail> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = Command::new(cargo)
        .current_dir(root)
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;
    if !output.status.success() {
        return Err("RP-13: `cargo metadata` failed".into());
    }
    let metadata: Value = serde_json::from_slice(&output.stdout)?;
    let mut crates = BTreeSet::new();
    for package in metadata["packages"].as_array().into_iter().flatten() {
        if package["name"] != inventory::OWNED_PACKAGE {
            continue;
        }
        for target in package["targets"].as_array().into_iter().flatten() {
            let kinds: Vec<&str> = target["kind"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect();
            let tested = kinds
                .iter()
                .any(|kind| matches!(*kind, "lib" | "rlib" | "bin" | "test"));
            if let (true, Some(name)) = (tested, target["name"].as_str()) {
                crates.insert(name.replace('-', "_"));
            }
        }
    }
    Ok(crates)
}

/// RP-13: `test` and `bdd` partition `cargo test --workspace --all-features`.
pub fn check_test_inventory(root: &Path) -> Result<(), Fail> {
    let justfile = std::fs::read_to_string(root.join("Justfile"))?;
    let ordinary_line = recipe_command(&justfile, "test")?;
    let owned_line = recipe_command(&justfile, "bdd")?;
    let reference: Vec<String> = inventory::REFERENCE
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    let workspace = list_tests(root, &reference)?;
    let ordinary = list_tests(root, &inventory::recipe_arguments(&ordinary_line)?)?;
    let owned = list_tests(root, &inventory::recipe_arguments(&owned_line)?)?;
    let summary = inventory::compare(&workspace, &ordinary, &owned, &owned_crates(root)?)?;
    println!(
        "check-test-inventory: the {} tests of `cargo test {}` are the {} of `test` and the {} of `bdd`, disjoint, none lost (RP-13)",
        summary.workspace,
        inventory::REFERENCE.join(" "),
        summary.ordinary,
        summary.owned
    );
    Ok(())
}

/// The digest of a directory: the SHA-256 of every relative path and the
/// bytes of every file under it, in path order. Two directories with the same
/// digest are byte for byte the same tree.
pub fn tree_digest(directory: &Path) -> Result<String, Fail> {
    let mut files: Vec<(String, PathBuf)> = Vec::new();
    for entry in walkdir::WalkDir::new(directory) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let relative = entry
                .path()
                .strip_prefix(directory)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .replace('\\', "/");
            files.push((relative, entry.path().to_path_buf()));
        }
    }
    files.sort();
    let mut rows = String::new();
    for (relative, path) in &files {
        let bytes = std::fs::read(path)?;
        rows.push_str(&format!(
            "{relative}\0{}\0{}\n",
            bytes.len(),
            lexlean::Sha256Digest::of(&bytes).to_hex()
        ));
    }
    Ok(lexlean::Sha256Digest::of(rows.as_bytes()).to_hex())
}

fn option(arguments: &[String], name: &str) -> Option<String> {
    arguments
        .iter()
        .position(|argument| argument == name)
        .and_then(|at| arguments.get(at + 1))
        .cloned()
}

/// Summarize the evidence directory of a gate run (SPEC.md §9.4): print the
/// summary, append it to the step summary when GitHub provides one, and keep
/// `summary.md`. The work is `repo_conformance::evidence::publish`, which has
/// the tests; this reads the Justfile and the process environment for it.
pub fn ci_summary(root: &Path, arguments: &[String]) -> Result<(), Fail> {
    let directory =
        PathBuf::from(option(arguments, "--evidence").unwrap_or_else(|| "ci-evidence".to_owned()));
    let justfile = std::fs::read_to_string(root.join("Justfile"))?;
    let counts = evidence::publish(
        &justfile,
        &directory,
        &|name| std::env::var_os(name),
        &mut std::io::stdout().lock(),
    )?;
    println!(
        "ci-summary: {} recipes, {} host samples, {} verifier events, {} examples, {} conformance cases",
        counts.recipes, counts.samples, counts.verifier, counts.examples, counts.cases
    );
    Ok(())
}

/// The scheduling hints of SPEC.md §22.11, derived from a verifier telemetry
/// file (§22.12): one entry per module of an elaboration or replay process,
/// keyed by its full Lean module name, holding the largest `wall_ms` and
/// `peak_rss_kib` observed under that name in any project or stage. The
/// largest is the conservative choice: a module name that two examples share
/// is scheduled as the heavier of the two. Keys are sorted, so the same
/// telemetry always gives the same bytes.
pub fn hints_from_telemetry(telemetry: &str) -> Result<String, Fail> {
    let mut hints: std::collections::BTreeMap<String, (u64, u64)> = Default::default();
    for (number, line) in telemetry.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let event: Value = serde_json::from_str(line)
            .map_err(|error| format!("telemetry line {}: {error}", number + 1))?;
        if event["event"] != "process"
            || !matches!(event["stage"].as_str(), Some("elaboration" | "replay"))
        {
            continue;
        }
        let (Some(module), Some(wall_ms)) = (event["module"].as_str(), event["wall_ms"].as_u64())
        else {
            continue;
        };
        let peak = event["peak_rss_kib"].as_u64().unwrap_or(0);
        let entry = hints.entry(module.to_owned()).or_default();
        *entry = (entry.0.max(wall_ms), entry.1.max(peak));
    }
    let object: serde_json::Map<String, Value> = hints
        .into_iter()
        .map(|(module, (wall_ms, peak_rss_kib))| {
            (
                module,
                serde_json::json!({ "peak_rss_kib": peak_rss_kib, "wall_ms": wall_ms }),
            )
        })
        .collect();
    Ok(serde_json::to_string_pretty(&Value::Object(object))? + "\n")
}

/// Write the scheduling hints that `--from` (a `verify-telemetry.jsonl`)
/// gives to `--out` (SPEC.md §22.11). The file is a schedule input of the
/// examples leg and nothing else: a stale hint reorders, it never refuses.
pub fn ci_hints(arguments: &[String]) -> Result<(), Fail> {
    let from = option(arguments, "--from").ok_or("ci-hints: --from <verify-telemetry.jsonl>")?;
    let out = option(arguments, "--out").ok_or("ci-hints: --out <hints.json>")?;
    let hints = hints_from_telemetry(&std::fs::read_to_string(&from)?)?;
    lexlean::verify::operational::Operational::parse_hints(&hints).map_err(|diagnostic| {
        format!("ci-hints: the derived hints do not parse: {diagnostic:?}")
    })?;
    std::fs::write(&out, &hints)?;
    println!(
        "ci-hints: {} modules from {from} written to {out}",
        hints.matches("\"wall_ms\"").count()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::hints_from_telemetry;

    #[test]
    fn hints_keep_the_heaviest_observation_of_each_module() {
        let telemetry = [
            r#"{"event":"process","module":"A.M","peak_rss_kib":10,"stage":"elaboration","wall_ms":5}"#,
            r#"{"event":"process","module":"A.M","peak_rss_kib":30,"stage":"replay","wall_ms":2}"#,
            r#"{"event":"process","module":"B.M","peak_rss_kib":7,"stage":"elaboration","wall_ms":9}"#,
            r#"{"event":"process","module":"A.M","peak_rss_kib":99,"stage":"probe","wall_ms":99}"#,
            r#"{"event":"stage","stage":"elaboration","wall_ms":100}"#,
        ]
        .join("\n");
        let hints = hints_from_telemetry(&telemetry).expect("hints");
        let parsed =
            lexlean::verify::operational::Operational::parse_hints(&hints).expect("parses");
        assert_eq!(parsed.len(), 2, "{hints}");
        assert_eq!(
            (parsed["A.M"].wall_ms, parsed["A.M"].peak_rss_kib),
            (5, Some(30))
        );
        assert_eq!(
            (parsed["B.M"].wall_ms, parsed["B.M"].peak_rss_kib),
            (9, Some(7))
        );
        assert_eq!(
            hints,
            hints_from_telemetry(&telemetry).expect("hints"),
            "deterministic"
        );
    }
}
