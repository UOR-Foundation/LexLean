//! The counts the bootstrap cases assert (SPEC.md §17.23): what the committed
//! corpora hold and what the case derives from them, each a named constant of
//! the case, so a repository that grows fails the case instead of leaving a
//! stale statement behind.
//!
//! A landing moves several of them at once, and the cases check them in
//! stages, the cheapest first. Under `LEXLEAN_COUNTS` a case records every
//! count it checks instead of stopping at the first that moved, so one run
//! reports all of them; `cargo xtask counts` reads the report and rewrites the
//! constants and the statements that state them.
//!
//! - unset: a count that moved fails the case, as an `assert_eq!` would;
//! - `report`: every count is recorded and the case runs on, whole;
//! - `fast`: as `report`, and the case stops at [`stop_before_runs`], before
//!   the evaluator and rustc runs that take most of its time.
//!
//! The report is one JSON line per checked count, in
//! `target/count-report/<id>.jsonl`, rewritten by each run of the case.

use std::collections::BTreeSet;
use std::io::Write as _;
use std::sync::Mutex;

use camino::Utf8PathBuf;

/// One count a case checks: the constant that states it, the value the
/// constant holds, and the value the case observed.
#[derive(Debug, Clone, Copy)]
pub struct Count {
    /// The constant's name in the case's source.
    pub name: &'static str,
    /// The constant's value.
    pub expected: usize,
    /// What the case observed.
    pub observed: usize,
}

/// One recorded count of a report line.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Recorded {
    /// The case's conformance ID, `BS-05` say.
    pub id: String,
    /// The case's source file, relative to the repository root.
    pub file: String,
    /// What the checked group of counts is.
    pub what: String,
    /// The constant's name.
    pub name: String,
    /// The constant's value.
    pub expected: usize,
    /// What the case observed.
    pub observed: usize,
}

/// How the counts are checked, read from `LEXLEAN_COUNTS`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// A count that moved fails the case.
    Assert,
    /// Every count is recorded and the case runs whole.
    Report,
    /// Every count is recorded and the case stops before its runs.
    Fast,
}

/// The mode `LEXLEAN_COUNTS` names.
///
/// # Panics
///
/// When the variable holds anything but `report` or `fast`, so a misspelt
/// mode is not taken for the asserting one.
#[must_use]
pub fn mode() -> Mode {
    match std::env::var("LEXLEAN_COUNTS").as_deref() {
        Err(_) | Ok("") => Mode::Assert,
        Ok("report") => Mode::Report,
        Ok("fast") => Mode::Fast,
        Ok(other) => panic!("LEXLEAN_COUNTS is `report` or `fast`, not `{other}`"),
    }
}

/// The directory of the reports.
#[must_use]
pub fn report_dir() -> Utf8PathBuf {
    crate::support::repo_root().join("target/count-report")
}

/// The cases whose report this process has begun, each rewritten once.
static BEGUN: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());

/// Check a group of counts of the case `id` in `file`. Under the asserting
/// mode a moved count fails the case with both tuples; otherwise each count
/// is recorded and every moved one is said on stderr.
///
/// # Panics
///
/// Under the asserting mode, when a count moved; under the others, when the
/// report cannot be written.
pub fn check(id: &str, file: &str, what: &str, counts: &[Count]) {
    let mode = mode();
    if mode == Mode::Assert {
        let observed: Vec<usize> = counts.iter().map(|count| count.observed).collect();
        let expected: Vec<usize> = counts.iter().map(|count| count.expected).collect();
        assert!(
            observed == expected,
            "{id}: {what}\n  observed: {observed:?}\n  expected: {expected:?} ({})",
            counts
                .iter()
                .map(|count| count.name)
                .collect::<Vec<_>>()
                .join(", ")
        );
        return;
    }
    let dir = report_dir();
    std::fs::create_dir_all(&dir).expect("the count report's directory");
    let path = dir.join(format!("{id}.jsonl"));
    let fresh = BEGUN
        .lock()
        .expect("the begun reports")
        .insert(id.to_owned());
    let mut out = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(!fresh)
        .truncate(fresh)
        .open(&path)
        .expect("the count report");
    for count in counts {
        let line = serde_json::to_string(&Recorded {
            id: id.to_owned(),
            file: file.to_owned(),
            what: what.to_owned(),
            name: count.name.to_owned(),
            expected: count.expected,
            observed: count.observed,
        })
        .expect("a recorded count is JSON");
        writeln!(out, "{line}").expect("the count report");
        if count.expected != count.observed {
            eprintln!(
                "{id}: count moved: {} {} -> {} ({what})",
                count.name, count.expected, count.observed
            );
        }
    }
}

/// Whether the case is to stop here, before the runs that take most of its
/// time: under `LEXLEAN_COUNTS=fast`, once every count before them is
/// recorded. Says so on stderr when it is.
#[must_use]
pub fn stop_before_runs(id: &str) -> bool {
    let stop = mode() == Mode::Fast;
    if stop {
        eprintln!(
            "{id}: LEXLEAN_COUNTS=fast: the counts before the runs are in {}; stopping before the runs",
            report_dir().join(format!("{id}.jsonl"))
        );
    }
    stop
}

/// Every report line under `dir`, in file order.
///
/// # Errors
///
/// When a report cannot be read or a line is not a recorded count.
pub fn read_reports(dir: &std::path::Path) -> Result<Vec<Recorded>, String> {
    let mut paths: Vec<std::path::PathBuf> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "jsonl"))
            .collect(),
        Err(error) => return Err(format!("{}: {error}", dir.display())),
    };
    paths.sort();
    let mut out = Vec::new();
    for path in paths {
        let text = std::fs::read_to_string(&path)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        for (index, line) in text.lines().enumerate() {
            out.push(
                serde_json::from_str(line)
                    .map_err(|error| format!("{}:{}: {error}", path.display(), index + 1))?,
            );
        }
    }
    Ok(out)
}

/// Check the named counts of a bootstrap case: `counts!("BS-05", "what",
/// { CONSTANT: observed, ... })`. See [`check`].
#[macro_export]
macro_rules! counts {
    ($id:literal, $what:expr, { $( $name:ident : $observed:expr ),+ $(,)? }) => {
        $crate::counts::check(
            $id,
            file!(),
            &$what,
            &[ $( $crate::counts::Count {
                name: stringify!($name),
                expected: $name,
                observed: $observed,
            } ),+ ],
        )
    };
}
