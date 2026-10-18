//! CI evidence (SPEC.md §9.4, §22.12): what the performance telemetry may
//! never be part of, and the summary of what it is.
//!
//! A measurement is evidence about one run on one host. A verified artifact is
//! evidence about a project. The first may be read beside the second, never
//! inside it, so the check here is by name: any JSON key of the closed
//! telemetry vocabulary in a verified directory is a measurement that leaked.

use std::collections::BTreeSet;
use std::path::Path;

use lexlean::verify::telemetry::VOCABULARY;

/// Every JSON key of `value`, at any depth.
fn collect_keys(root: &serde_json::Value, into: &mut BTreeSet<String>) {
    let mut stack = vec![root];
    while let Some(value) = stack.pop() {
        match value {
            serde_json::Value::Object(map) => {
                for (key, inner) in map {
                    into.insert(key.clone());
                    stack.push(inner);
                }
            }
            serde_json::Value::Array(rows) => {
                stack.extend(rows);
            }
            _ => {}
        }
    }
}

/// The telemetry that a verified directory contains, as `path: key` rows:
/// a JSON key of the vocabulary in any `.json` file under it, or a file whose
/// name says it is telemetry.
///
/// # Errors
///
/// A file under `verified` that cannot be read, or a `.json` file that is not
/// JSON (a verified artifact that does not parse is a defect of its own, and
/// one this check cannot clear).
pub fn telemetry_in(verified: &Path) -> Result<Vec<String>, String> {
    let mut found = Vec::new();
    for entry in walkdir::WalkDir::new(verified) {
        let entry = entry.map_err(|error| format!("{}: {error}", verified.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(verified)
            .unwrap_or(entry.path())
            .to_string_lossy()
            .replace('\\', "/");
        if relative.to_ascii_lowercase().contains("telemetry") {
            found.push(format!("{relative}: a file named as telemetry"));
        }
        if !relative.ends_with(".json") {
            continue;
        }
        let bytes = std::fs::read(entry.path()).map_err(|error| format!("{relative}: {error}"))?;
        let keys = keys_in_json(&bytes, &relative)?;
        for key in VOCABULARY {
            if keys.contains(key) {
                found.push(format!("{relative}: the telemetry key `{key}`"));
            }
        }
    }
    Ok(found)
}

fn keys_in_json(bytes: &[u8], relative: &str) -> Result<BTreeSet<String>, String> {
    let stack = bytes.len().saturating_mul(512).clamp(16 << 20, 4 << 30);
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(stack)
            .spawn_scoped(scope, || {
                let mut reader = serde_json::Deserializer::from_slice(bytes);
                reader.disable_recursion_limit();
                let value: serde_json::Value = serde::Deserialize::deserialize(&mut reader)
                    .and_then(|value| reader.end().map(|()| value))
                    .map_err(|error| format!("{relative}: not JSON: {error}"))?;
                let mut keys = BTreeSet::new();
                collect_keys(&value, &mut keys);
                drop_deep(value);
                Ok(keys)
            })
            .map_err(|error| format!("{relative}: {error}"))?
            .join()
            .map_err(|_| format!("{relative}: JSON reader panicked"))?
    })
}

fn drop_deep(root: serde_json::Value) {
    let mut stack = vec![root];
    while let Some(value) = stack.pop() {
        match value {
            serde_json::Value::Object(mut map) => {
                for (_, inner) in std::mem::take(&mut map) {
                    if matches!(
                        inner,
                        serde_json::Value::Object(_) | serde_json::Value::Array(_)
                    ) {
                        stack.push(inner);
                    }
                }
            }
            serde_json::Value::Array(mut rows) => {
                for inner in std::mem::take(&mut rows) {
                    if matches!(
                        inner,
                        serde_json::Value::Object(_) | serde_json::Value::Array(_)
                    ) {
                        stack.push(inner);
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::telemetry_in;

    #[test]
    fn a_planted_telemetry_key_is_found_wherever_it_is_nested() {
        let directory = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(directory.path().join("process")).expect("mkdir");
        std::fs::write(
            directory.path().join("attestation.json"),
            br#"{"status":"verified","width":3,"processes":[{"tool":"lean","exit_code":0}]}"#,
        )
        .expect("write");
        assert_eq!(
            telemetry_in(directory.path()).expect("scans"),
            Vec::<String>::new(),
            "`width` and `exit_code` are artifact vocabulary, not telemetry"
        );
        std::fs::write(
            directory.path().join("process/lean.json"),
            br#"{"tool":"lean","argv":[],"extra":[{"nested":{"wall_ms":12}}]}"#,
        )
        .expect("write");
        let found = telemetry_in(directory.path()).expect("scans");
        assert_eq!(found, ["process/lean.json: the telemetry key `wall_ms`"]);
        std::fs::write(directory.path().join("verify-telemetry.jsonl"), b"").expect("write");
        assert_eq!(telemetry_in(directory.path()).expect("scans").len(), 2);
    }

    #[test]
    fn an_artifact_that_does_not_parse_is_an_error_not_a_pass() {
        let directory = tempfile::tempdir().expect("tempdir");
        std::fs::write(directory.path().join("broken.json"), b"{").expect("write");
        assert!(telemetry_in(directory.path()).is_err());
    }
}

/// The line `just vv` prints when every recipe has passed; the end of the last
/// recipe's span.
pub const PASSED_LINE: &str = "echo \"vv: the acceptance gate passed\"";

/// The line a leg of the CI gate appends, stamped, when every recipe it ran
/// has passed (SPEC.md §9.4). A leg does not run `vv`, so it never prints
/// [`PASSED_LINE`]; its last recipe ends at the last stamped line, which
/// without this one is that recipe's own first command, and it would be
/// timed at nothing. `conformance_rp_14` requires `ci.yml` to append it.
pub const PART_PASSED_LINE: &str = "vv part: every recipe passed";

/// The wall time of one recipe of `just vv`, read from the gate log.
#[derive(Debug, Clone, PartialEq)]
pub struct RecipeSpan {
    /// The recipe.
    pub recipe: String,
    /// Seconds since the epoch at which its command started.
    pub start: f64,
    /// Seconds since the epoch at which the next recipe (or the end) started.
    pub end: f64,
}

impl RecipeSpan {
    /// The seconds the recipe took.
    #[must_use]
    pub fn seconds(&self) -> f64 {
        (self.end - self.start).max(0.0)
    }
}

/// The first command line of each recipe the `vv` recipe depends on, in order.
fn vv_commands(justfile: &str) -> Vec<(String, String)> {
    let mut dependencies: Vec<String> = Vec::new();
    let mut commands: std::collections::BTreeMap<String, String> =
        std::collections::BTreeMap::new();
    let mut current: Option<String> = None;
    for line in justfile.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            if let Some(name) = &current {
                commands
                    .entry(name.clone())
                    .or_insert_with(|| line.trim().trim_start_matches('@').to_owned());
            }
        } else if let Some((name, rest)) = line.split_once(':') {
            let name = name.trim().to_owned();
            if name == "vv" {
                dependencies = rest.split_whitespace().map(str::to_owned).collect();
            }
            current = Some(name);
        }
    }
    dependencies
        .into_iter()
        .filter_map(|name| commands.get(&name).map(|command| (name, command.clone())))
        .collect()
}

/// The wall time of each recipe of `vv`, from a log written by
/// `just --timestamp --timestamp-format '%s%.3f' vv`: `just` prints
/// `[<seconds since the epoch>] <command>` before each command it runs, so a
/// recipe lasts from its command's line to the next recipe's.
///
/// A recipe whose command never appears (the run failed before it, or a leg
/// of the CI gate does not run it) has no span. The last recipe ends at
/// [`PASSED_LINE`], or at the last stamped line of a log that never reached
/// it, which in a leg of the CI gate is [`PART_PASSED_LINE`].
#[must_use]
pub fn recipe_spans(justfile: &str, log: &str) -> Vec<RecipeSpan> {
    let commands = vv_commands(justfile);
    let mut stamped: Vec<(f64, &str)> = Vec::new();
    for line in log.lines() {
        let Some(rest) = line.strip_prefix('[') else {
            continue;
        };
        let Some((stamp, command)) = rest.split_once("] ") else {
            continue;
        };
        if let Ok(seconds) = stamp.parse::<f64>() {
            stamped.push((seconds, command));
        }
    }
    let last = stamped.last().map_or(0.0, |(seconds, _)| *seconds);
    let end_of_gate = stamped
        .iter()
        .find(|(_, command)| *command == PASSED_LINE)
        .map_or(last, |(seconds, _)| *seconds);
    let mut starts: Vec<(String, f64)> = Vec::new();
    for (recipe, command) in &commands {
        if let Some((seconds, _)) = stamped.iter().find(|(_, line)| line == command) {
            starts.push((recipe.clone(), *seconds));
        }
    }
    starts.sort_by(|a, b| a.1.total_cmp(&b.1));
    let mut spans = Vec::new();
    for (index, (recipe, start)) in starts.iter().enumerate() {
        let end = starts.get(index + 1).map_or(end_of_gate, |next| next.1);
        spans.push(RecipeSpan {
            recipe: recipe.clone(),
            start: *start,
            end,
        });
    }
    spans
}

/// One host sample: `<epoch seconds> <MemAvailable KiB> <swap used KiB>
/// <largest resident set KiB>` separated by tabs.
#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    /// Seconds since the epoch.
    pub epoch: f64,
    /// `MemAvailable`, in KiB.
    pub available_kib: u64,
    /// Swap in use, in KiB.
    pub swap_used_kib: u64,
    /// The resident set of the largest process, in KiB.
    pub top_rss_kib: u64,
}

/// Parse `resources.tsv`; rows that are not four numbers are dropped, because
/// a sampler killed mid-line leaves a torn last row and the rest is still
/// evidence.
#[must_use]
pub fn parse_samples(text: &str) -> Vec<Sample> {
    text.lines()
        .filter_map(|line| {
            let mut fields = line.split('\t');
            Some(Sample {
                epoch: fields.next()?.trim().parse().ok()?,
                available_kib: fields.next()?.trim().parse().ok()?,
                swap_used_kib: fields.next()?.trim().parse().ok()?,
                top_rss_kib: fields.next()?.trim().parse().ok()?,
            })
        })
        .collect()
}

fn mib(kib: u64) -> u64 {
    kib / 1024
}

/// The rows of any one table of the summary: the slowest few, never every
/// process or module of the run (those are in `verify-telemetry.jsonl`).
pub const TOP: usize = 15;

/// The variable GitHub Actions sets to the file that becomes the step's
/// summary on the run page.
pub const STEP_SUMMARY_VARIABLE: &str = "GITHUB_STEP_SUMMARY";

/// The most this report adds to a step summary. GitHub refuses a step summary
/// of more than 1 MiB and drops it whole, so the report stops at half of that
/// rather than lose every table to a run with more than the tables expect.
pub const STEP_SUMMARY_LIMIT: usize = 512 * 1024;

/// `text` cut at a line boundary to fit `limit` bytes, with a note that says
/// it was cut. Text that fits is returned as it is.
#[must_use]
pub fn bounded(text: &str, limit: usize) -> String {
    const NOTE: &str = "\n_The summary was cut here to fit the step summary; the whole of it is `summary.md` of the uploaded CI evidence._\n";
    if text.len() <= limit {
        return text.to_owned();
    }
    let mut end = limit.saturating_sub(NOTE.len()).min(text.len());
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let kept = text[..end].rfind('\n').map_or(0, |at| at + 1);
    format!("{}{NOTE}", &text[..kept])
}

/// Read the JSON lines of `file` under `directory`, dropping what is not JSON.
fn json_lines(directory: &Path, file: &str) -> Vec<serde_json::Value> {
    std::fs::read_to_string(directory.join(file))
        .map(|text| {
            text.lines()
                .filter_map(|line| serde_json::from_str(line).ok())
                .collect()
        })
        .unwrap_or_default()
}

/// What [`publish`] counted, for the line it ends with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    /// Recipes of `vv` found in the gate log.
    pub recipes: usize,
    /// Rows of host samples.
    pub samples: usize,
    /// Lines of verifier telemetry.
    pub verifier: usize,
    /// Examples timed.
    pub examples: usize,
    /// Conformance cases timed.
    pub cases: usize,
}

/// Summarize the evidence directory of a gate run (SPEC.md §9.4): the summary
/// goes to `stdout`, which in CI is the job log, because an uploaded artifact
/// cannot be read from the log; to the step summary named by the environment
/// variable [`STEP_SUMMARY_VARIABLE`], when it is set, bounded by
/// [`STEP_SUMMARY_LIMIT`]; and to `summary.md` in the directory, which stays
/// the complete record. It reads whatever the run left, so a gate that failed
/// early still reports the recipes it reached. The summary is evidence read
/// beside the gate: nothing here writes into a verified artifact.
///
/// A step summary that cannot be written is a warning in the log and not an
/// error: it is a second copy, and the first is already printed.
///
/// # Errors
///
/// A `stdout` that cannot be written, or a directory that cannot be created
/// or a `summary.md` that cannot be written in it.
pub fn publish(
    justfile: &str,
    directory: &Path,
    environment: &dyn Fn(&str) -> Option<std::ffi::OsString>,
    stdout: &mut dyn std::io::Write,
) -> std::io::Result<Counts> {
    use std::io::Write as _;
    let read = |file: &str| std::fs::read_to_string(directory.join(file)).unwrap_or_default();
    let spans = recipe_spans(justfile, &read("vv.txt"));
    let samples = parse_samples(&read("resources.tsv"));
    let verifier = json_lines(directory, lexlean::verify::telemetry::VERIFY_FILE);
    let examples = json_lines(directory, "examples-telemetry.jsonl");
    let cases = json_lines(directory, "cases-telemetry.jsonl");
    let mut summary = render_summary(&spans, &samples, &verifier, &examples);
    summary.push_str(&render_detail(&verifier, &cases));

    // The log first: it is what a reader of a failed run has.
    writeln!(stdout, "{summary}")?;
    stdout.flush()?;
    if let Some(step) = environment(STEP_SUMMARY_VARIABLE) {
        let appended = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&step)
            .and_then(|mut file| writeln!(file, "{}", bounded(&summary, STEP_SUMMARY_LIMIT)));
        if let Err(error) = appended {
            writeln!(
                stdout,
                "::warning::the step summary {} was not written: {error}",
                Path::new(&step).display()
            )?;
        }
    }
    std::fs::create_dir_all(directory)?;
    std::fs::write(directory.join("summary.md"), &summary)?;
    Ok(Counts {
        recipes: spans.len(),
        samples: samples.len(),
        verifier: verifier.len(),
        examples: examples.len(),
        cases: cases.len(),
    })
}

/// The Markdown summary of one gate run: the wall time of each recipe with the
/// host's memory extremes while it ran, the verifier's stages, and its slowest
/// processes. Everything in it is a measurement of this run; none of it is an
/// input of any recipe (SPEC.md §9.4).
#[must_use]
pub fn render_summary(
    spans: &[RecipeSpan],
    samples: &[Sample],
    verifier: &[serde_json::Value],
    examples: &[serde_json::Value],
) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    out.push_str("## Wall time per `just vv` recipe\n\n| Recipe | Seconds | Minutes | Lowest free memory (MiB) | Highest swap in use (MiB) | Largest process (MiB) |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    let mut total = 0.0;
    for span in spans {
        let within: Vec<&Sample> = samples
            .iter()
            .filter(|sample| sample.epoch >= span.start && sample.epoch < span.end)
            .collect();
        let low = within.iter().map(|s| s.available_kib).min();
        let swap = within.iter().map(|s| s.swap_used_kib).max();
        let top = within.iter().map(|s| s.top_rss_kib).max();
        let cell =
            |value: Option<u64>| value.map_or_else(|| "n/a".to_owned(), |v| mib(v).to_string());
        total += span.seconds();
        let _ = writeln!(
            out,
            "| `{}` | {:.0} | {:.1} | {} | {} | {} |",
            span.recipe,
            span.seconds(),
            span.seconds() / 60.0,
            cell(low),
            cell(swap),
            cell(top)
        );
    }
    let _ = writeln!(out, "| total | {total:.0} | {:.1} | | | |", total / 60.0);

    let ms = |event: &serde_json::Value| event["wall_ms"].as_u64().unwrap_or(0);
    let mut stages: std::collections::BTreeMap<(String, String), (u64, u64)> =
        std::collections::BTreeMap::new();
    let mut projects: std::collections::BTreeMap<String, (u64, u64)> =
        std::collections::BTreeMap::new();
    for event in verifier {
        let project = event["project"].as_str().unwrap_or("?").to_owned();
        match event["event"].as_str() {
            Some("stage") => {
                let stage = event["stage"].as_str().unwrap_or("?").to_owned();
                let slot = stages.entry((project, stage)).or_insert((0, 0));
                slot.0 += 1;
                slot.1 += ms(event);
            }
            Some("verify_end") => {
                let slot = projects.entry(project).or_insert((0, 0));
                slot.0 += 1;
                slot.1 += ms(event);
            }
            _ => {}
        }
    }
    if !projects.is_empty() {
        out.push_str(
            "\n## Verifications\n\n| Project | Verifications | Seconds |\n| --- | ---: | ---: |\n",
        );
        let mut rows: Vec<_> = projects.iter().collect();
        rows.sort_by_key(|row| std::cmp::Reverse(row.1 .1));
        let all = rows.len();
        for (project, (count, wall)) in rows.into_iter().take(TOP) {
            let _ = writeln!(
                out,
                "| {project} | {count} | {:.1} |",
                *wall as f64 / 1000.0
            );
        }
        if all > TOP {
            let _ = writeln!(
                out,
                "\n{all} projects in all; the {TOP} slowest are listed."
            );
        }
        out.push_str(
            "\n## Verification stages (all verifications)\n\n| Stage | Seconds |\n| --- | ---: |\n",
        );
        let mut by_stage: std::collections::BTreeMap<&str, u64> = std::collections::BTreeMap::new();
        for ((_, stage), (_, wall)) in &stages {
            *by_stage.entry(stage).or_insert(0) += wall;
        }
        let mut rows: Vec<_> = by_stage.into_iter().collect();
        rows.sort_by_key(|row| std::cmp::Reverse(row.1));
        for (stage, wall) in rows {
            let _ = writeln!(out, "| {stage} | {:.1} |", wall as f64 / 1000.0);
        }
        let mut processes: Vec<&serde_json::Value> = verifier
            .iter()
            .filter(|event| event["event"] == "process")
            .collect();
        processes.sort_by_key(|event| std::cmp::Reverse(ms(event)));
        out.push_str("\n## Slowest verifier processes\n\n| Project | Stage | Tool | Module or audit member | Seconds | Peak RSS (MiB) | Peak swap (MiB) |\n| --- | --- | --- | --- | ---: | ---: | ---: |\n");
        for event in processes.into_iter().take(TOP) {
            let kib = |key: &str| {
                event[key]
                    .as_u64()
                    .map_or_else(|| "n/a".to_owned(), |v| mib(v).to_string())
            };
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {:.1} | {} | {} |",
                event["project"].as_str().unwrap_or("?"),
                event["stage"].as_str().unwrap_or("?"),
                event["tool"].as_str().unwrap_or("?"),
                event["module"].as_str().unwrap_or(""),
                ms(event) as f64 / 1000.0,
                kib("peak_rss_kib"),
                kib("peak_swap_kib")
            );
        }
    }
    if !examples.is_empty() {
        out.push_str(
            "\n## Examples\n\n| Example | Seconds | Verifier width | fmt | lock | check | build | verify |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n",
        );
        let mut slowest: Vec<&serde_json::Value> = examples.iter().collect();
        slowest.sort_by_key(|event| std::cmp::Reverse(ms(event)));
        for event in slowest.into_iter().take(TOP) {
            let step = |label: &str| {
                event["steps"][format!("{label}_ms")].as_u64().map_or_else(
                    || "n/a".to_owned(),
                    |ms| format!("{:.1}", ms as f64 / 1000.0),
                )
            };
            let _ = writeln!(
                out,
                "| {} | {:.1} | {} | {} | {} | {} | {} | {} |",
                event["example"].as_str().unwrap_or("?"),
                ms(event) as f64 / 1000.0,
                event["process_width"].as_u64().unwrap_or(0),
                step("fmt"),
                step("lock"),
                step("check"),
                step("build"),
                step("verify")
            );
        }
        if examples.len() > TOP {
            let _ = writeln!(
                out,
                "\n{} examples in all; the {TOP} slowest are listed.",
                examples.len()
            );
        }
    }
    out
}

/// The detail of a run for finding where its time and memory went: the slowest
/// conformance cases, every project's stages, and, for the longest single
/// verification, its slowest frontiers, its stages with the host's lowest free
/// memory and highest swap, and the processes that were slowest, largest, or
/// swapped. A project's name separates verifications of different projects; two
/// verifications of one project that overlap in time share their rows.
#[must_use]
pub fn render_detail(verifier: &[serde_json::Value], cases: &[serde_json::Value]) -> String {
    use std::fmt::Write as _;
    let ms = |event: &serde_json::Value| event["wall_ms"].as_u64().unwrap_or(0);
    let secs = |value: u64| value as f64 / 1000.0;
    let mib = |event: &serde_json::Value, key: &str| {
        event[key]
            .as_u64()
            .map_or_else(|| "n/a".to_owned(), |kib| (kib / 1024).to_string())
    };
    let mut out = String::new();
    if !cases.is_empty() {
        let mut rows: Vec<&serde_json::Value> = cases.iter().collect();
        rows.sort_by_key(|event| std::cmp::Reverse(ms(event)));
        let total: u64 = cases.iter().map(ms).sum();
        let _ = write!(
            out,
            "\n## Slowest conformance cases\n\n{} cases, {:.0} s in all (summed over the test threads)\n\n| Case | Seconds | Passed |\n| --- | ---: | --- |\n",
            cases.len(),
            secs(total)
        );
        for event in rows.into_iter().take(TOP) {
            let _ = writeln!(
                out,
                "| {} | {:.1} | {} |",
                event["case"].as_str().unwrap_or("?"),
                secs(ms(event)),
                event["ok"]
            );
        }
    }
    // Projects and their stages.
    let mut by_project: std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, u64>,
    > = std::collections::BTreeMap::new();
    let mut longest: Option<(String, u64)> = None;
    for event in verifier {
        let project = event["project"].as_str().unwrap_or("?").to_owned();
        match event["event"].as_str() {
            Some("stage") => {
                *by_project
                    .entry(project)
                    .or_default()
                    .entry(event["stage"].as_str().unwrap_or("?").to_owned())
                    .or_insert(0) += ms(event);
            }
            Some("verify_end") if longest.as_ref().is_none_or(|(_, wall)| ms(event) > *wall) => {
                longest = Some((project, ms(event)));
            }
            _ => {}
        }
    }
    if !by_project.is_empty() {
        const COLUMNS: [&str; 7] = [
            "probe",
            "elaboration",
            "replay",
            "audit",
            "extraction",
            "certificates",
            "attestation",
        ];
        out.push_str("\n## Stages of each project (seconds, all its verifications)\n\n| Project |");
        for column in COLUMNS {
            let _ = write!(out, " {column} |");
        }
        out.push_str(" other |\n| --- |");
        for _ in 0..=COLUMNS.len() {
            out.push_str(" ---: |");
        }
        out.push('\n');
        let mut rows: Vec<_> = by_project.iter().collect();
        rows.sort_by_key(|(_, stages)| std::cmp::Reverse(stages.values().sum::<u64>()));
        for (project, stages) in rows.into_iter().take(12) {
            let _ = write!(out, "| {project} |");
            let mut named = 0;
            for column in COLUMNS {
                let value = stages.get(column).copied().unwrap_or(0);
                named += value;
                let _ = write!(out, " {:.1} |", secs(value));
            }
            let other = stages.values().sum::<u64>().saturating_sub(named);
            let _ = writeln!(out, " {:.1} |", secs(other));
        }
    }
    let Some((project, wall)) = longest else {
        return out;
    };
    let mine: Vec<&serde_json::Value> = verifier
        .iter()
        .filter(|event| event["project"].as_str() == Some(project.as_str()))
        .collect();
    let runs = mine
        .iter()
        .filter(|event| event["event"] == "verify_end")
        .count();
    let _ = write!(
        out,
        "\n## The longest verification: {project} ({:.0} s; {})\n\n### Stages\n\n| Stage | Seconds | Lowest free memory (MiB) | Highest swap in use (MiB) | Verifier's own peak (MiB) |\n| --- | ---: | ---: | ---: | ---: |\n",
        secs(wall),
        match runs {
            1 => "its only verification".to_owned(),
            count => format!("rows are summed over its {count} verifications"),
        }
    );
    let mut stages: std::collections::BTreeMap<String, (u64, Option<u64>, u64, u64)> =
        std::collections::BTreeMap::new();
    for event in mine.iter().filter(|event| event["event"] == "stage") {
        let slot = stages
            .entry(event["stage"].as_str().unwrap_or("?").to_owned())
            .or_insert((0, None, 0, 0));
        slot.0 += ms(event);
        if let Some(available) = event["mem_available_kib"].as_u64() {
            slot.1 = Some(slot.1.map_or(available, |low| low.min(available)));
        }
        slot.2 = slot.2.max(event["swap_used_kib"].as_u64().unwrap_or(0));
        slot.3 = slot.3.max(event["peak_rss_kib"].as_u64().unwrap_or(0));
    }
    let mut rows: Vec<_> = stages.into_iter().collect();
    rows.sort_by_key(|(_, slot)| std::cmp::Reverse(slot.0));
    for (stage, (wall, low, swap, own)) in rows {
        let _ = writeln!(
            out,
            "| {stage} | {:.1} | {} | {} | {} |",
            secs(wall),
            low.map_or_else(|| "n/a".to_owned(), |kib| (kib / 1024).to_string()),
            swap / 1024,
            own / 1024
        );
    }
    let processes: Vec<&&serde_json::Value> = mine
        .iter()
        .filter(|event| event["event"] == "process")
        .collect();
    let table = |out: &mut String, title: &str, mut rows: Vec<&&serde_json::Value>, key: &str| {
        let _ = write!(
            out,
            "\n### {title}\n\n| Stage | Tool | Module or audit member | Seconds | Peak RSS (MiB) | Peak swap (MiB) |\n| --- | --- | --- | ---: | ---: | ---: |\n"
        );
        rows.sort_by_key(|event| {
            std::cmp::Reverse(if key == "wall_ms" {
                ms(event)
            } else {
                event[key].as_u64().unwrap_or(0)
            })
        });
        for event in rows.into_iter().take(TOP) {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {:.1} | {} | {} |",
                event["stage"].as_str().unwrap_or("?"),
                event["tool"].as_str().unwrap_or("?"),
                event["module"].as_str().unwrap_or(""),
                secs(ms(event)),
                mib(event, "peak_rss_kib"),
                mib(event, "peak_swap_kib")
            );
        }
    };
    table(&mut out, "Slowest processes", processes.clone(), "wall_ms");
    table(
        &mut out,
        "Largest processes",
        processes.clone(),
        "peak_rss_kib",
    );
    let swapped: Vec<&&serde_json::Value> = processes
        .iter()
        .copied()
        .filter(|event| event["peak_swap_kib"].as_u64().unwrap_or(0) > 0)
        .collect();
    if swapped.is_empty() {
        out.push_str("\n### Processes that swapped\n\nnone\n");
    } else {
        table(&mut out, "Processes that swapped", swapped, "peak_swap_kib");
    }
    let mut frontiers: Vec<&&serde_json::Value> = mine
        .iter()
        .filter(|event| event["event"] == "frontier")
        .collect();
    frontiers.sort_by_key(|event| std::cmp::Reverse(ms(event)));
    let _ = write!(
        out,
        "\n### Slowest frontiers of module elaboration ({} in all)\n\n| Frontier | Members | Seconds |\n| ---: | ---: | ---: |\n",
        frontiers.len()
    );
    for event in frontiers.into_iter().take(10) {
        let _ = writeln!(
            out,
            "| {} | {} | {:.1} |",
            event["frontier"],
            event["members"],
            secs(ms(event))
        );
    }
    out
}

#[cfg(test)]
mod summary_tests {
    use super::{bounded, parse_samples, publish, recipe_spans, render_summary};

    const JUSTFILE: &str = "\
vv: a b c
    @echo \"vv: the acceptance gate passed\"

a:
    cargo a

b:
    cargo b --flag

c:
    cargo c
";

    #[test]
    fn a_recipe_lasts_from_its_command_to_the_next_recipes() {
        let log = "\
[100.000] cargo a
noise
[130.500] cargo b --flag
[190.000] cargo c
[200.250] echo \"vv: the acceptance gate passed\"
vv: the acceptance gate passed
";
        let spans = recipe_spans(JUSTFILE, log);
        let seconds: Vec<(&str, f64)> = spans
            .iter()
            .map(|span| (span.recipe.as_str(), span.seconds()))
            .collect();
        assert_eq!(seconds, [("a", 30.5), ("b", 59.5), ("c", 10.25)]);
    }

    /// A leg of the CI gate runs some of the recipes and ends with its own
    /// stamped line: its recipes are timed, the last one to that line, and the
    /// recipes of the other leg have no span.
    #[test]
    fn a_leg_of_the_gate_ends_its_last_recipe_at_the_leg_s_own_line() {
        let log = format!(
            "[100.000] cargo b --flag\n[190.000] cargo c\n[200.250] {}\n",
            super::PART_PASSED_LINE
        );
        let log = log.as_str();
        let spans = recipe_spans(JUSTFILE, log);
        let seconds: Vec<(&str, f64)> = spans
            .iter()
            .map(|span| (span.recipe.as_str(), span.seconds()))
            .collect();
        assert_eq!(seconds, [("b", 90.0), ("c", 10.25)]);
    }

    #[test]
    fn a_log_cut_short_ends_at_its_last_stamp_and_misses_the_recipes_not_reached() {
        let spans = recipe_spans(
            JUSTFILE,
            "[10.0] cargo a\n[25.0] cargo b --flag\n[40.0] other\n",
        );
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[1].seconds(), 15.0);
    }

    #[test]
    fn the_summary_carries_the_memory_extremes_of_each_window() {
        let log = "[100.0] cargo a\n[160.0] cargo b --flag\n[220.0] echo \"vv: the acceptance gate passed\"\n";
        let samples = parse_samples(
            "100.5\t8000000\t0\t2000000\n130\t3000000\t1024000\t4000000\nbad row\n170\t7000000\t0\t1000000\n",
        );
        assert_eq!(samples.len(), 3);
        let summary = render_summary(&recipe_spans(JUSTFILE, log), &samples, &[], &[]);
        assert!(
            summary.contains("| `a` | 60 | 1.0 | 2929 | 1000 | 3906 |"),
            "{summary}"
        );
        assert!(
            summary.contains("| `b` | 60 | 1.0 | 6835 | 0 | 976 |"),
            "{summary}"
        );
    }

    #[test]
    fn the_detail_names_the_longest_verification_its_stages_and_its_swapping_processes() {
        let events: Vec<serde_json::Value> = [
            r#"{"event":"stage","project":"big","stage":"elaboration","wall_ms":9000,"mem_available_kib":2097152,"swap_used_kib":524288,"peak_rss_kib":102400}"#,
            r#"{"event":"process","project":"big","stage":"elaboration","tool":"lean","module":"Big.A","wall_ms":5000,"peak_rss_kib":8388608,"peak_swap_kib":1048576}"#,
            r#"{"event":"process","project":"big","stage":"replay","tool":"leanchecker","module":"Big.B","wall_ms":100,"peak_rss_kib":1024,"peak_swap_kib":0}"#,
            r#"{"event":"frontier","project":"big","stage":"elaboration","frontier":0,"members":2,"wall_ms":5000}"#,
            r#"{"event":"verify_end","project":"big","wall_ms":10000}"#,
            r#"{"event":"verify_end","project":"small","wall_ms":50}"#,
        ]
        .iter()
        .map(|line| serde_json::from_str(line).expect("json"))
        .collect();
        let cases: Vec<serde_json::Value> =
            [r#"{"event":"case","case":"VR-21","wall_ms":170000,"ok":true}"#]
                .iter()
                .map(|line| serde_json::from_str(line).expect("json"))
                .collect();
        let detail = super::render_detail(&events, &cases);
        assert!(detail.contains("| VR-21 | 170.0 | true |"), "{detail}");
        assert!(
            detail.contains("## The longest verification: big (10 s; its only verification)"),
            "{detail}"
        );
        assert!(
            detail.contains("| elaboration | 9.0 | 2048 | 512 | 100 |"),
            "{detail}"
        );
        assert!(
            detail.contains("| elaboration | lean | Big.A | 5.0 | 8192 | 1024 |"),
            "{detail}"
        );
        assert!(
            detail.contains("### Slowest frontiers of module elaboration (1 in all)"),
            "{detail}"
        );
        assert!(
            !detail.contains("none\n"),
            "the swapped process is listed: {detail}"
        );
    }

    /// A gate that failed in its second recipe: a log cut short, a verifier
    /// that never ran, no host samples. What the run did leave is the evidence.
    fn failed_run() -> tempfile::TempDir {
        let directory = tempfile::tempdir().expect("a directory");
        std::fs::write(
            directory.path().join("vv.txt"),
            "[100.0] cargo a\n[160.0] cargo b --flag\nerror: b failed\n",
        )
        .expect("a log");
        directory
    }

    fn published(
        directory: &std::path::Path,
        step: Option<&std::path::Path>,
    ) -> (String, super::Counts) {
        let mut printed = Vec::new();
        let counts = publish(
            JUSTFILE,
            directory,
            &|name| {
                (name == super::STEP_SUMMARY_VARIABLE)
                    .then(|| step.map(|path| path.as_os_str().to_owned()))
                    .flatten()
            },
            &mut printed,
        )
        .expect("publishes");
        (String::from_utf8(printed).expect("UTF-8"), counts)
    }

    #[test]
    fn the_summary_is_printed_to_the_log_even_when_the_gate_failed() {
        let run = failed_run();
        let (printed, counts) = published(run.path(), None);
        assert!(
            printed.contains("## Wall time per `just vv` recipe") && printed.contains("| `a` |"),
            "the recipes the run reached are in the log: {printed}"
        );
        assert_eq!(counts.recipes, 2);
        assert_eq!(
            format!(
                "{}\n",
                std::fs::read_to_string(run.path().join("summary.md")).expect("the file")
            ),
            printed,
            "the log and the file carry the same summary"
        );
    }

    #[test]
    fn the_step_summary_is_appended_to_when_the_variable_names_one_and_only_then() {
        let run = failed_run();
        let step = run.path().join("step-summary.md");
        std::fs::write(&step, "an earlier step\n").expect("an earlier step");
        let (printed, _) = published(run.path(), Some(&step));
        let text = std::fs::read_to_string(&step).expect("the step summary");
        assert!(
            text.starts_with("an earlier step\n") && text.contains("| `a` |"),
            "appended after what was there: {text}"
        );
        assert!(printed.contains("| `a` |"), "and still printed");

        let other = failed_run();
        let untouched = other.path().join("never-named.md");
        published(other.path(), None);
        assert!(!untouched.exists());
        assert_eq!(
            std::fs::read_dir(other.path()).expect("listed").count(),
            2,
            "without the variable only `vv.txt` and `summary.md` exist"
        );
    }

    #[test]
    fn a_step_summary_that_cannot_be_written_is_a_warning_and_not_a_failure() {
        let run = failed_run();
        let missing = run.path().join("no-such-directory").join("step.md");
        let (printed, _) = published(run.path(), Some(&missing));
        assert!(printed.contains("::warning::the step summary"), "{printed}");
        assert!(run.path().join("summary.md").is_file());
    }

    #[test]
    fn what_goes_into_the_step_summary_is_bounded_and_cut_at_a_line() {
        let long = "a line of the summary, 38 bytes long...\n".repeat(100);
        assert_eq!(bounded(&long, long.len()), long);
        for limit in [600, 1000, 1001, 2048] {
            let cut = bounded(&long, limit);
            assert!(cut.len() <= limit, "{limit}: {}", cut.len());
            assert!(cut.contains("was cut here"), "{cut}");
            assert!(
                cut.starts_with("a line of the summary, 38 bytes long...\n"),
                "{cut}"
            );
        }
        // A multi-byte character at the cut is not split.
        let wide = "\u{2014}".repeat(1000);
        assert!(bounded(&wide, 700).len() <= 700);
    }

    #[test]
    fn a_run_with_thousands_of_projects_and_examples_lists_only_the_slowest() {
        let verifier: Vec<serde_json::Value> = (0..5000)
            .map(|n| serde_json::json!({"event": "verify_end", "project": format!("p{n}"), "wall_ms": n}))
            .collect();
        let examples: Vec<serde_json::Value> = (0..5000)
            .map(|n| serde_json::json!({"example": format!("e{n}"), "wall_ms": n, "steps": {}}))
            .collect();
        let summary = render_summary(&[], &[], &verifier, &examples);
        assert!(
            summary.len() < 16 * 1024,
            "the summary does not grow with the run: {} bytes",
            summary.len()
        );
        assert!(summary.contains("| p4999 |") && !summary.contains("| p0 |"));
        assert!(summary.contains("5000 projects in all; the 15 slowest are listed."));
        assert!(summary.contains("5000 examples in all; the 15 slowest are listed."));
    }
}
